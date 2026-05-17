//! Pre-deploy environment checks — fail fast and friendly before we
//! touch docker, so a misconfigured machine doesn't half-create
//! containers / volumes / networks that the operator then has to GC.
//!
//! Each check returns `Result<()>` with an actionable error message
//! when it fails. Caller runs them in order and bails on the first
//! red flag. Use `check_all()` as the single entry-point.

use anyhow::{anyhow, Context, Result};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;

/// Run every blocking pre-flight in the right order. The order matters:
/// docker first (no point checking ports if we can't deploy at all),
/// then ports, then perms, then disk.
pub fn check_all(instance_dir: &Path, origin: Option<&str>) -> Result<()> {
    docker_available()?;
    instance_dir_writable(instance_dir)?;
    disk_space(instance_dir, 5)?;
    if let Some(o) = origin {
        ports_free_for_origin(o)?;
    }
    Ok(())
}

/// `docker compose v2` reachable via the same probe `runtime` uses.
/// Kept as a thin shim so we can preflight without importing runtime
/// from places that don't need it.
pub fn docker_available() -> Result<()> {
    crate::deploy::runtime::assert_docker_available()
}

/// Make sure we can create + write inside the instance dir. Catches
/// the "wrong user owns ~/.glow" footgun before we try to render
/// `docker-compose.yml` and get a confusing IO error.
pub fn instance_dir_writable(dir: &Path) -> Result<()> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("cannot create instance dir: {}", dir.display()))?;
    let probe = dir.join(".preflight-write-probe");
    std::fs::write(&probe, b"ok")
        .with_context(|| format!("cannot write inside {}", dir.display()))?;
    let _ = std::fs::remove_file(&probe);
    Ok(())
}

/// Best-effort disk-space check — warns if the partition has less than
/// `min_free_gb` available. Doesn't hard-fail (some filesystems lie or
/// can't be queried), just surfaces a heads-up so a deploy doesn't OOM
/// the docker daemon mid-pull.
pub fn disk_space(dir: &Path, min_free_gb: u64) -> Result<()> {
    // `df -k <dir>` portable enough across macOS + Linux. Parse the
    // 4th column (available KB) of the second line.
    let out = Command::new("df")
        .args(["-k", "--", dir.to_str().unwrap_or(".")])
        .output();
    let Ok(out) = out else {
        return Ok(());
    }; // df not present — skip
    if !out.status.success() {
        return Ok(());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let avail_kb: Option<u64> = text
        .lines()
        .nth(1)
        .and_then(|line| line.split_whitespace().nth(3))
        .and_then(|s| s.parse().ok());
    if let Some(kb) = avail_kb {
        let gb = kb / 1024 / 1024;
        if gb < min_free_gb {
            eprintln!(
                "  ⚠ pre-flight: only {gb} GB free on {}; >= {min_free_gb} GB recommended \
                 (image pulls + DB volume can easily eat several GB)",
                dir.display()
            );
        }
    }
    Ok(())
}

/// If the origin is on localhost, make sure its port isn't in use by
/// something else (a stray dev server, a previous Glow that didn't
/// clean up, etc.). Skips remote origins — nothing to check.
pub fn ports_free_for_origin(origin: &str) -> Result<()> {
    let Some(port) = port_from_origin(origin) else {
        return Ok(());
    };
    // Quick TCP bind-check on 127.0.0.1 — if it fails, something's
    // already listening. Don't bother checking other interfaces;
    // docker maps to all by default and the conflict will surface.
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => {
            drop(listener);
            Ok(())
        }
        Err(e) => Err(anyhow!(
            "port {port} (from origin {origin}) is already in use: {e}.\n\
             Run `lsof -i :{port}` (macOS/Linux) to find the culprit, \
             stop it, then retry `glow deploy`."
        )),
    }
}

/// Extract port from origin URL. Returns None for non-localhost / no
/// port specified (the host is presumably a real domain on 80/443
/// behind some load balancer — not our problem to check).
fn port_from_origin(origin: &str) -> Option<u16> {
    // Crude but sufficient: pull host:port from after the scheme.
    let after_scheme = origin.split_once("://").map(|x| x.1).unwrap_or(origin);
    let host_port = after_scheme.split('/').next().unwrap_or("");
    let mut parts = host_port.splitn(2, ':');
    let host = parts.next()?;
    if host != "localhost" && host != "127.0.0.1" && host != "0.0.0.0" {
        return None; // Only preflight localhost — remote ports aren't ours.
    }
    parts.next().and_then(|p| p.parse().ok())
}
