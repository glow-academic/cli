//! `glow record` — drive the demo recorders against a running instance.
//!
//! Two surfaces, one verb, two very different asset homes:
//!
//!   * `cli`    — the VHS tapes are **embedded in this binary**
//!     (`include_dir!` below), so the cli surface is fully native: no
//!     checkout, the tapes ship with the release. We extract the tape to
//!     a temp dir and drive host `vhs`.
//!
//!   * `client` — the Playwright spec/config files ride **inside the
//!     deployed client docker image** (a few KB of text added to the
//!     image; no Playwright/Chromium baked in). We `docker cp` the specs
//!     out of the running client container — so they always match the
//!     deployed version — and run them with the **host's** Playwright
//!     against the live client URL.
//!
//! The principle in both cases: the *what to test* travels with the
//! versioned artifact (binary / image); the *how to run it* (the heavy,
//! generic engine — vhs, Playwright+Chromium, ffmpeg) lives on the host,
//! installed once. We never bundle the heavy engine; we fail fast with an
//! install hint (mirroring `deploy::preflight`).
//!
//! The polish step (gradient/shadow treatment) is embedded here as the
//! single source of truth shared by both surfaces — the web client's
//! duplicate copy is deleted in favor of it.

use anyhow::{anyhow, bail, Context, Result};
use colored::Colorize;
use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use which::which;

use crate::deploy::config::{DeployConfig, Topology};
use crate::deploy::instance::{DeployState, Instance};

/// VHS tapes, embedded at build time from the repo's `tapes/` dir.
static TAPES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tapes");

/// The polish script, embedded so it travels with the binary and is the
/// single source of truth for the gradient/shadow treatment.
const POLISH_SCRIPT: &str = include_str!("../scripts/polish-video.mjs");

// ── Client surface ──────────────────────────────────────────────────

pub struct ClientArgs {
    pub workflow: String,
    pub base_url: Option<String>,
    pub name: String,
    pub raw: bool,
    pub out: Option<String>,
}

pub fn client(args: ClientArgs, _cfg: &crate::config::Config) -> Result<()> {
    crate::deploy::runtime::assert_docker_available()?;

    // The client lives in docker — resolve the running container we
    // deployed for this instance, guarding the api_only case (no client).
    let inst = Instance::open(&args.name)?;
    let dc = DeployConfig::load(&inst.deploy_yaml()).ok();
    if let Some(dc) = &dc {
        if dc.topology == Topology::ApiOnly {
            bail!(
                "instance `{}` is api_only topology — there's no client to record.\n  use `glow record cli <tape>` for a terminal demo.",
                args.name
            );
        }
    }
    let state = DeployState::load(&inst.state_file())?;
    let color = state.active_client_env.as_deref().ok_or_else(|| {
        anyhow!(
            "no running client for instance `{}` — deploy one first (`glow deploy`).",
            args.name
        )
    })?;
    let container = running_client_container(&inst, color)?;

    let base_url = match args.base_url.as_deref() {
        Some(u) => with_scheme(u),
        None => client_url(dc.as_ref()),
    };

    // Host runtime: a Playwright runner provisioned once on the box. We
    // overlay the versioned specs from the image into it and run there so
    // the spec imports (@playwright/test, @next/env, helpers) resolve.
    let home = recorder_home()?;
    let pw = home.join("node_modules/.bin/playwright");
    if !pw.exists() {
        bail!(
            "host Playwright runner not provisioned at {home}.\n  set it up once (the generic engine — specs come from the image):\n    mkdir -p {home} && cd {home} \\\n      && npm init -y && npm i -D @playwright/test @next/env \\\n      && npx playwright install chromium",
            home = home.display()
        );
    }
    which("node").map_err(|_| {
        anyhow!("`node` not found in PATH — Playwright runs on Node. Install Node 20+ first.")
    })?;

    // Pull the versioned specs out of the running container into the
    // runner. Replace any prior copy so stale specs can't linger.
    let _ = std::fs::remove_dir_all(home.join("e2e"));
    docker_cp(&container, "/app/e2e", &home.join("e2e"))?;
    docker_cp(
        &container,
        "/app/playwright.config.ts",
        &home.join("playwright.config.ts"),
    )?;

    let rel = format!("e2e/demos/{}.spec.ts", args.workflow);
    if !home.join(&rel).exists() {
        bail!(
            "no demo spec `{rel}` in the deployed client image (container {container}).\n  list available workflows with `glow record list`"
        );
    }

    println!(
        "{} recording client demo {} against {}",
        "▶".cyan(),
        args.workflow.bold(),
        base_url.bold()
    );

    let status = Command::new(&pw)
        .current_dir(&home)
        .arg("test")
        .arg(&rel)
        .env("PLAYWRIGHT_BASE_URL", &base_url)
        .env("PLAYWRIGHT_NO_SERVER", "1")
        .env("PLAYWRIGHT_DEMO", "1")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("spawn playwright")?;
    if !status.success() {
        bail!("playwright exited {}", status.code().unwrap_or(-1));
    }

    let raw_video = home.join(format!("demo-output/{}.webm", args.workflow));
    if !raw_video.exists() {
        bail!(
            "recording finished but no video at {}\n  does the spec call saveDemoVideo(page, \"{}\")?",
            raw_video.display(),
            args.workflow
        );
    }

    finish(&raw_video, args.raw, args.out.as_deref())
}

// ── CLI surface ─────────────────────────────────────────────────────

pub struct CliArgs {
    pub workflow: String,
    pub instance_url: Option<String>,
    pub raw: bool,
    pub out: Option<String>,
}

pub fn cli_surface(args: CliArgs, cfg: &crate::config::Config) -> Result<()> {
    let tape_name = format!("{}.tape", args.workflow);
    let tape = TAPES.get_file(&tape_name).ok_or_else(|| {
        anyhow!(
            "no embedded tape `{tape_name}`\n  list available workflows with `glow record list`"
        )
    })?;

    which("vhs").map_err(|_| {
        anyhow!("`vhs` not found in PATH — install it first: https://github.com/charmbracelet/vhs")
    })?;

    // The api/instance URL the recorded `glow` commands hit.
    let url = args
        .instance_url
        .clone()
        .or_else(|| cfg.active_instance_url().map(str::to_string))
        .or_else(|| cfg.glow_url.clone())
        .unwrap_or_else(|| "http://localhost:8000".to_string());

    // Materialize the embedded tape into a temp workdir. vhs writes its
    // `Output demo-output/<name>.mp4` relative to cwd, so we run there.
    let work = std::env::temp_dir().join("glow-record-cli");
    std::fs::create_dir_all(&work).context("create cli record workdir")?;
    let tape_path = work.join(&tape_name);
    std::fs::write(&tape_path, tape.contents()).context("write embedded tape")?;

    // Make the *running* glow resolve as a bare `glow` inside vhs's shell
    // so recorded tapes drive this exact binary.
    let mut path = String::new();
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(Path::to_path_buf))
    {
        path.push_str(&dir.to_string_lossy());
        path.push(':');
    }
    path.push_str(&std::env::var("PATH").unwrap_or_default());

    println!(
        "{} recording cli demo {} against {}",
        "▶".cyan(),
        args.workflow.bold(),
        url.bold()
    );

    let mut cmd = Command::new("vhs");
    cmd.current_dir(&work)
        .arg(&tape_name)
        .env("PATH", path)
        .env("GLOW_INSTANCE_URL", &url);
    // An explicit `$GLOW_TOKEN` wins (inherited); otherwise seed from the
    // stored login so canonical tapes can run real resource commands.
    if std::env::var("GLOW_TOKEN").is_err() {
        if let Ok(tok) = crate::auth::get_token(&url) {
            cmd.env("GLOW_TOKEN", tok.access_token);
        }
    }
    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("spawn vhs")?;
    if !status.success() {
        bail!("vhs exited {}", status.code().unwrap_or(-1));
    }

    let raw_video = work.join(format!("demo-output/{}.mp4", args.workflow));
    if !raw_video.exists() {
        bail!(
            "vhs finished but no video at {}\n  check the tape's `Output` directive",
            raw_video.display()
        );
    }

    finish(&raw_video, args.raw, args.out.as_deref())
}

// ── List ────────────────────────────────────────────────────────────

pub fn list(name: &str) -> Result<()> {
    println!("{}", "cli workflows (embedded tapes):".bold());
    let mut tapes: Vec<String> = TAPES
        .files()
        .filter_map(|f| {
            f.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(str::to_string)
        })
        .filter(|n| n.ends_with(".tape") && !n.starts_with('_'))
        .map(|n| n.trim_end_matches(".tape").to_string())
        .collect();
    tapes.sort();
    for t in tapes {
        println!("  {t}");
    }

    // Client workflows live in the deployed image — best-effort list them
    // out of the running container, since that's where they ship.
    if let Some(demos) = client_demo_names(name) {
        println!("{}", "client workflows (from running image):".bold());
        for d in demos {
            println!("  {d}");
        }
    }
    Ok(())
}

/// Best-effort: `ls` the demos dir inside the running client container.
/// Returns None when there's no instance / no running client.
fn client_demo_names(name: &str) -> Option<Vec<String>> {
    let inst = Instance::open(name).ok()?;
    let state = DeployState::load(&inst.state_file()).ok()?;
    let color = state.active_client_env.as_deref()?;
    let container = running_client_container(&inst, color).ok()?;
    let out = Command::new("docker")
        .args(["exec", &container, "ls", "/app/e2e/demos"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let mut names: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| l.ends_with(".spec.ts"))
        .map(|l| l.trim_end_matches(".spec.ts").to_string())
        .collect();
    names.sort();
    Some(names)
}

// ── docker helpers ──────────────────────────────────────────────────

/// Resolve the running client app container name for a color, trying the
/// indexed (`-1`) and bare compose name forms (matches runtime.rs).
fn running_client_container(inst: &Instance, color: &str) -> Result<String> {
    let project = inst.client_project_name();
    for c in [
        format!("{project}-app-{color}-1"),
        format!("{project}-app-{color}"),
    ] {
        let out = Command::new("docker")
            .args(["inspect", "-f", "{{.State.Running}}", &c])
            .output();
        if let Ok(o) = out {
            if o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == "true" {
                return Ok(c);
            }
        }
    }
    bail!(
        "no running client container for instance `{}` (looked for {project}-app-{color}).\n  is it deployed and started?",
        inst.name
    )
}

/// `docker cp <container>:<src> <dst>` — copies an artifact out of the
/// container. Streams errors so a missing path is diagnosable.
fn docker_cp(container: &str, src: &str, dst: &Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let status = Command::new("docker")
        .arg("cp")
        .arg(format!("{container}:{src}"))
        .arg(dst)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status()
        .context("spawn docker cp")?;
    if !status.success() {
        bail!("docker cp {container}:{src} failed — is the spec shipped in the image?");
    }
    Ok(())
}

// ── shared helpers ──────────────────────────────────────────────────

/// Host directory holding the provisioned Playwright runner (the generic
/// engine; versioned specs are overlaid into it per-record).
fn recorder_home() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("no home dir"))?;
    Ok(home.join(".glow").join("recorder").join("client"))
}

/// Polish (unless `raw`), report where the artifact landed, and move it
/// to `out` when requested.
fn finish(raw_video: &Path, raw: bool, out: Option<&str>) -> Result<()> {
    let produced = if raw {
        raw_video.to_path_buf()
    } else {
        polish(raw_video)?
    };
    let dest = match out {
        Some(o) => {
            let dest = PathBuf::from(o);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            std::fs::rename(&produced, &dest)
                .or_else(|_| std::fs::copy(&produced, &dest).map(|_| ()))
                .with_context(|| format!("move artifact to {}", dest.display()))?;
            dest
        }
        None => produced,
    };
    println!(
        "{} saved {}",
        "✓".green(),
        dest.display().to_string().bold()
    );
    Ok(())
}

/// Run the embedded polish-video.mjs over a raw recording → polished mp4.
fn polish(input: &Path) -> Result<PathBuf> {
    which("node").map_err(|_| {
        anyhow!("`node` not found in PATH — polishing needs Node. Re-run with --raw to skip it.")
    })?;
    which("ffmpeg").map_err(|_| {
        anyhow!(
            "`ffmpeg` not found in PATH — polishing needs ffmpeg. Re-run with --raw to skip it."
        )
    })?;
    let script = std::env::temp_dir().join("glow-polish-video.mjs");
    std::fs::write(&script, POLISH_SCRIPT).context("write embedded polish script")?;

    let out = input.with_extension("polished.mp4");
    println!("{} polishing → {}", "·".dimmed(), out.display());
    let status = Command::new("node")
        .arg(&script)
        .arg(input)
        .arg("--out")
        .arg(&out)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("spawn node polish-video.mjs")?;
    if !status.success() {
        bail!("polish step exited {}", status.code().unwrap_or(-1));
    }
    Ok(out)
}

fn with_scheme(origin: &str) -> String {
    if origin.contains("://") {
        origin.to_string()
    } else {
        format!("https://{origin}")
    }
}

/// Resolve the loopback URL the box reaches the deployed client on. We
/// prefer the locally-published port (`client_http_port`, default
/// 127.0.0.1:18080) over the public origin — recording happens on the box.
fn client_url(dc: Option<&DeployConfig>) -> String {
    match dc {
        Some(dc) => {
            let spec = dc.client_http_port.as_deref().unwrap_or("127.0.0.1:18080");
            local_url_from_port_spec(spec)
        }
        None => "http://localhost:3000".to_string(),
    }
}

/// Turn a `client_http_port` spec ("18080", "0.0.0.0:18080",
/// "127.0.0.1:18080") into a loopback URL Playwright can reach.
fn local_url_from_port_spec(spec: &str) -> String {
    let port = spec.rsplit(':').next().unwrap_or(spec);
    format!("http://127.0.0.1:{port}")
}
