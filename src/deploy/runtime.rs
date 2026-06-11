//! Thin wrapper over `docker compose` (v2 plugin only — design call).
//!
//! Every public function returns a `Result<()>` that propagates compose's
//! stderr in the error so failures are diagnosable without re-reading
//! console output. We deliberately don't try to parse compose's stdout —
//! it's logs-for-humans, not a stable interface.

use anyhow::{anyhow, bail, Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use which::which;

/// Asserted at the top of every command path so we error early with a
/// helpful message instead of failing with "command not found" 12 calls
/// deep into the orchestrator.
pub fn assert_docker_available() -> Result<()> {
    which("docker").map_err(|_| {
        anyhow!("`docker` not found in PATH — install Docker Desktop or the docker engine first")
    })?;
    // Sanity-check we have compose v2. `docker compose version` exits 0
    // on the plugin; v1 (`docker-compose`) wouldn't respond here.
    let out = Command::new("docker")
        .args(["compose", "version", "--short"])
        .output()
        .context("spawn `docker compose version`")?;
    if !out.status.success() {
        bail!(
            "`docker compose` v2 not available — install the compose plugin \
             (the standalone `docker-compose` v1 binary isn't supported)"
        );
    }
    Ok(())
}

/// The set of `--file` arguments compose should be invoked with, in
/// merge order. Always the base `docker-compose.yml`; plus the
/// cli-generated `docker-compose.override.yml` when it's present.
///
/// Passing `--file` explicitly disables compose's auto-discovery of
/// `docker-compose.override.yml`, so we MUST list it ourselves — otherwise
/// the override (db-init local-overlay mount + public-port binding written
/// by `assemble_api_override`) is silently dropped. Compose deep-merges
/// `--file`s in order: the override layers onto the base, with list-valued
/// keys like `volumes`/`ports` appending.
fn compose_files(project_dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = vec![project_dir.join("docker-compose.yml")];
    let override_file = project_dir.join("docker-compose.override.yml");
    if override_file.exists() {
        files.push(override_file);
    }
    files
}

/// Build a `docker compose` Command pre-configured with the right
/// project dir, project name, and compose files. Caller appends the
/// subcommand + args.
pub fn compose(project_dir: &Path, project_name: &str) -> Command {
    let mut cmd = Command::new("docker");
    cmd.arg("compose")
        .arg("--project-directory")
        .arg(project_dir)
        .arg("--project-name")
        .arg(project_name);
    for file in compose_files(project_dir) {
        cmd.arg("--file").arg(file);
    }
    // .env is auto-loaded from project_dir by compose; no need to pass.
    cmd
}

/// `docker compose pull <services…>`. Streams progress to the user's
/// terminal so big image pulls show their actual progress bars.
pub fn pull(project_dir: &Path, project_name: &str, services: &[&str]) -> Result<()> {
    let mut cmd = compose(project_dir, project_name);
    cmd.arg("pull");
    cmd.args(services);
    run(cmd, "docker compose pull")
}

/// `docker compose up -d <services…>`. Streams output. Pass `&[]` to
/// bring up every service in the compose file.
pub fn up(project_dir: &Path, project_name: &str, services: &[&str]) -> Result<()> {
    let mut cmd = compose(project_dir, project_name);
    cmd.args(["up", "-d"]);
    cmd.args(services);
    run(cmd, "docker compose up -d")
}

/// `docker compose stop` — preserves volumes + network.
pub fn stop(project_dir: &Path, project_name: &str) -> Result<()> {
    let cmd = {
        let mut c = compose(project_dir, project_name);
        c.arg("stop");
        c
    };
    run(cmd, "docker compose stop")
}

/// `docker compose start` — restart previously-stopped containers.
pub fn start(project_dir: &Path, project_name: &str) -> Result<()> {
    let cmd = {
        let mut c = compose(project_dir, project_name);
        c.arg("start");
        c
    };
    run(cmd, "docker compose start")
}

/// `docker compose restart <services…>` — restart containers in place.
/// Used to clear a Keycloak node's stale in-memory Infinispan cache after a
/// blue/green color-swap: the restart re-reads the shared DB so the
/// newly-active node sees clients the other node created.
pub fn restart(project_dir: &Path, project_name: &str, services: &[&str]) -> Result<()> {
    let mut cmd = compose(project_dir, project_name);
    cmd.arg("restart");
    cmd.args(services);
    run(cmd, "docker compose restart")
}

/// `docker compose down -v --remove-orphans` — destroy everything.
pub fn down_destroy(project_dir: &Path, project_name: &str) -> Result<()> {
    let cmd = {
        let mut c = compose(project_dir, project_name);
        c.args(["down", "-v", "--remove-orphans"]);
        c
    };
    run(cmd, "docker compose down -v")
}

/// `docker compose logs [-f] [service]`. Streams to terminal.
pub fn logs(
    project_dir: &Path,
    project_name: &str,
    follow: bool,
    service: Option<&str>,
) -> Result<()> {
    let mut cmd = compose(project_dir, project_name);
    cmd.arg("logs");
    if follow {
        cmd.arg("-f");
    }
    if let Some(s) = service {
        cmd.arg(s);
    }
    run(cmd, "docker compose logs")
}

/// `docker compose exec -T <service> <cmd…>` — non-interactive, returns
/// stdout. Used for in-container ops (pg_dump, migration runner, etc.).
pub fn exec_capture(
    project_dir: &Path,
    project_name: &str,
    service: &str,
    argv: &[&str],
) -> Result<Vec<u8>> {
    let mut cmd = compose(project_dir, project_name);
    cmd.args(["exec", "-T", service]);
    cmd.args(argv);
    let out = cmd
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("spawn docker compose exec {service}"))?;
    if !out.status.success() {
        bail!(
            "docker compose exec {service} {:?} failed (exit {}): {}",
            argv,
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(out.stdout)
}

/// `docker compose exec -T <service> <cmd…>` returning the raw exit
/// outcome WITHOUT bailing on non-zero. Unlike `exec_capture`, a non-zero
/// exit is a normal, expected result the caller interprets (used by the
/// readiness probe, where exit-code 0/1 IS the signal). Returns
/// `(success, stdout, stderr)`. Only `Err`s if the process couldn't be
/// spawned at all (e.g. the container isn't running yet).
pub fn exec_status(
    project_dir: &Path,
    project_name: &str,
    service: &str,
    argv: &[&str],
) -> Result<(bool, String, String)> {
    let mut cmd = compose(project_dir, project_name);
    cmd.args(["exec", "-T", service]);
    cmd.args(argv);
    let out = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("spawn docker compose exec {service}"))?;
    Ok((
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).trim().to_string(),
        String::from_utf8_lossy(&out.stderr).trim().to_string(),
    ))
}

/// Idempotent `docker network create`. The two stacks (api + client)
/// share an external network so they can resolve each other by
/// container name; compose declares it `external: true` and requires
/// it to exist first.
pub fn ensure_network(name: &str) -> Result<()> {
    // Already exists? bail with success.
    let out = Command::new("docker")
        .args(["network", "inspect", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("spawn docker network inspect")?;
    if out.success() {
        return Ok(());
    }
    let status = Command::new("docker")
        .args(["network", "create", name])
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status()
        .context("spawn docker network create")?;
    if !status.success() {
        bail!("docker network create {name} failed");
    }
    Ok(())
}

/// Container-health inspection — returns the state of a single container
/// (`"healthy"` | `"starting"` | `"unhealthy"` | `"none"` | other). Used
/// by the bluegreen monitor's grace-period polling.
pub fn container_health(project_name: &str, service: &str) -> Result<String> {
    // Default compose container names are `<project>-<service>-1`, but a
    // service with an explicit `container_name:` (e.g. nginx) drops the `-1`
    // index — try both forms.
    for container in [
        format!("{project_name}-{service}-1"),
        format!("{project_name}-{service}"),
    ] {
        let out = Command::new("docker")
            .args([
                "inspect",
                "--format",
                // Guard `.State.Health`: a container with no HEALTHCHECK has a
                // nil `.State.Health`, and `{{.State.Health.Status}}` then
                // errors the template — which the old code misread as "missing"
                // and reported "starting". Map the no-health case to "none"
                // (callers, incl. wait_healthy, treat "none" as "up").
                "{{if .State.Health}}{{.State.Health.Status}}{{else}}none{{end}}",
                &container,
            ])
            .output()
            .context("spawn docker inspect")?;
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            return Ok(if s.is_empty() { "none".into() } else { s });
        }
    }
    // No container under either name — doesn't exist yet (or an inactive
    // blue/green standby). Treat as "starting" so polling loops keep waiting.
    Ok("starting".into())
}

// ── private ────────────────────────────────────────────────────────

/// Inherit stdio so the user sees compose's live output (image pulls,
/// container creation, etc.). Returns Err with the descriptive label on
/// non-zero exit.
fn run(mut cmd: Command, label: &str) -> Result<()> {
    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("spawn `{label}`"))?;
    if !status.success() {
        bail!("`{label}` exited {}", status.code().unwrap_or(-1));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn compose_files_base_only_when_no_override() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("docker-compose.yml"), "services: {}\n").unwrap();
        let files = compose_files(dir.path());
        assert_eq!(files, vec![dir.path().join("docker-compose.yml")]);
    }

    #[test]
    fn compose_files_includes_override_when_present() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("docker-compose.yml"), "services: {}\n").unwrap();
        fs::write(
            dir.path().join("docker-compose.override.yml"),
            "services: {}\n",
        )
        .unwrap();
        let files = compose_files(dir.path());
        // Base first (merge order matters: override deep-merges onto base).
        assert_eq!(
            files,
            vec![
                dir.path().join("docker-compose.yml"),
                dir.path().join("docker-compose.override.yml"),
            ]
        );
    }

    /// End-to-end on the built Command: the override path must appear as a
    /// `--file` arg iff the file exists in project_dir.
    #[test]
    fn compose_command_passes_override_file_when_present() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("docker-compose.yml"), "services: {}\n").unwrap();

        let cmd = compose(dir.path(), "proj");
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        let override_arg = dir
            .path()
            .join("docker-compose.override.yml")
            .to_string_lossy()
            .into_owned();
        assert!(
            !args.contains(&override_arg),
            "override must not be passed when absent: {args:?}"
        );

        fs::write(
            dir.path().join("docker-compose.override.yml"),
            "services: {}\n",
        )
        .unwrap();
        let cmd = compose(dir.path(), "proj");
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().into_owned())
            .collect();
        assert!(
            args.contains(&override_arg),
            "override must be passed when present: {args:?}"
        );
    }
}
