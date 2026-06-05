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
    pub instance_url: Option<String>,
    pub raw: bool,
    pub out: Option<String>,
    /// Local mode: source the runner's specs from this local client
    /// checkout (containing `e2e/` + `playwright.config.ts`) instead of
    /// `docker cp`ing them out of the deployed image. Its presence
    /// switches the whole flow to docker-free local mode, so a remote
    /// demo can be recorded from a dev machine without the deploy box.
    pub specs_dir: Option<String>,
    /// Override the base URL the e2e helpers use for their **direct REST
    /// calls** (`INTERNAL_API_BASE` — session adoption, attemptDemo's
    /// `/attempt/search`, …). In docker mode this defaults to the
    /// CLI's instance/api URL (the loopback API). In local mode it
    /// defaults to `--base-url` (the public demo origin) — but the box
    /// only publicly proxies /socket.io, /mcp and OIDC, *not* REST, so a
    /// remote recording typically needs this pointed at a reachable API
    /// (e.g. an SSH-tunnel `http://localhost:18080`).
    pub internal_api_base: Option<String>,
}

pub fn client(args: ClientArgs, cfg: &crate::config::Config) -> Result<()> {
    // Local mode is gated purely on `--specs-dir`: when set, we skip
    // docker entirely (no daemon, no Instance/DeployState, no container)
    // and overlay the specs from the given local checkout. The default
    // (docker) path below is unchanged.
    let local_specs = args.specs_dir.as_deref().map(PathBuf::from);

    let mut container: Option<String> = None;
    let base_url;

    if let Some(specs) = &local_specs {
        if !specs.join("e2e").is_dir() || !specs.join("playwright.config.ts").is_file() {
            bail!(
                "--specs-dir `{}` is not a client checkout (need `e2e/` and `playwright.config.ts`).",
                specs.display()
            );
        }
        // No deploy config to derive a loopback port from in local mode,
        // so require `--base-url` or fall back to the dev default.
        base_url = local_base_url(args.base_url.as_deref());
    } else {
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
        container = Some(running_client_container(&inst, color)?);

        base_url = match args.base_url.as_deref() {
            Some(u) => with_scheme(u),
            None => client_url(dc.as_ref()),
        };
    }

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

    // Mirror the client's ESM module setup so its (ES module)
    // playwright.config.ts loads in the runner — the heavy deps are
    // provisioned once; these small config files are ours to write.
    ensure_recorder_scaffold(&home)?;

    // Auth: record drives the UI as the CLI's logged-in identity. The
    // browser session is adopted from this token (the client's
    // /api/session/adopt route), and the e2e helpers use it for their
    // direct API calls (INTERNAL_API_BASE). No static bypass token.
    let token_url = args
        .instance_url
        .clone()
        .or_else(|| cfg.active_instance_url().map(str::to_string))
        .or_else(|| cfg.glow_url.clone())
        .unwrap_or_else(|| "http://localhost:8000".to_string());
    let token = match std::env::var("GLOW_RECORD_TOKEN") {
        Ok(t) if !t.is_empty() => t,
        _ => crate::auth::get_token(&token_url)
            .map_err(|e| {
                anyhow!(
                    "not logged in ({e}).\n  run `glow login` first — recording authenticates as your CLI identity."
                )
            })?
            .access_token,
    };

    // Source the versioned specs into the runner. Replace any prior copy
    // so stale specs can't linger. In local mode they come from the given
    // checkout; otherwise they're `docker cp`d out of the running image.
    let _ = std::fs::remove_dir_all(home.join("e2e"));
    if let Some(specs) = &local_specs {
        copy_local_specs(specs, &home)?;
    } else {
        let container = container.as_deref().expect("container set in docker mode");
        docker_cp(container, "/app/e2e", &home.join("e2e"))?;
        docker_cp(
            container,
            "/app/playwright.config.ts",
            &home.join("playwright.config.ts"),
        )?;
    }

    let rel = format!("e2e/demos/{}.spec.ts", args.workflow);
    if !home.join(&rel).exists() {
        let src = match (&local_specs, &container) {
            (Some(specs), _) => format!("--specs-dir `{}`", specs.display()),
            (_, Some(c)) => format!("the deployed client image (container {c})"),
            _ => "the client specs".to_string(),
        };
        bail!("no demo spec `{rel}` in {src}.\n  list available workflows with `glow record list`");
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
        .env("GLOW_RECORD_TOKEN", &token)
        .env(
            "INTERNAL_API_BASE",
            internal_api_base(
                args.internal_api_base.as_deref(),
                local_specs.is_some(),
                &base_url,
                &token_url,
            ),
        )
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

// ── local-mode helpers ──────────────────────────────────────────────

/// Resolve the client origin for local mode. There's no deploy config to
/// derive a loopback port from, so honor an explicit `--base-url` (adding
/// a scheme if bare) and otherwise fall back to the dev default.
fn local_base_url(base_url: Option<&str>) -> String {
    match base_url {
        Some(u) => with_scheme(u),
        None => "http://localhost:3000".to_string(),
    }
}

/// Resolve `INTERNAL_API_BASE` — the origin the e2e helpers use for their
/// direct REST calls (session adoption, `/attempt/search`, …).
///
///   * An explicit `--internal-api-base` always wins (adding a scheme if
///     bare), so an operator can point it at a reachable API — e.g. an
///     SSH-tunnel `http://localhost:18080` when recording a remote demo
///     (the box only publicly proxies /socket.io, /mcp and OIDC, not REST).
///   * In local mode it defaults to `base_url` (the public demo origin),
///     which is the right answer when REST *is* reachable at that origin.
///   * In docker mode it defaults to `token_url` (the CLI's loopback API),
///     preserving the existing behavior exactly.
fn internal_api_base(
    explicit: Option<&str>,
    local_mode: bool,
    base_url: &str,
    token_url: &str,
) -> String {
    match explicit {
        Some(u) => with_scheme(u),
        None if local_mode => base_url.to_string(),
        None => token_url.to_string(),
    }
}

/// Mirror the docker_cp layout from a local client checkout: copy its
/// `e2e/` tree and `playwright.config.ts` into the recorder home.
fn copy_local_specs(specs: &Path, home: &Path) -> Result<()> {
    copy_dir_all(&specs.join("e2e"), &home.join("e2e"))
        .with_context(|| format!("copy specs from {}", specs.display()))?;
    std::fs::copy(
        specs.join("playwright.config.ts"),
        home.join("playwright.config.ts"),
    )
    .context("copy playwright.config.ts from --specs-dir")?;
    Ok(())
}

/// Recursively copy a directory tree (like `cp -r src dst`).
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
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

/// Ensure the recorder dir mirrors the client's ESM module setup so its
/// `playwright.config.ts` (an ES module importing CommonJS `@next/env`) loads.
/// The client is `"type": "module"`; without that here, Playwright treats the
/// config as CJS and the `@next/env` default import resolves to `undefined`.
/// The heavy deps are provisioned once; these two small config files are ours.
fn ensure_recorder_scaffold(home: &Path) -> Result<()> {
    let pkg_path = home.join("package.json");
    let mut pkg: serde_json::Value = std::fs::read_to_string(&pkg_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| serde_json::json!({ "name": "glow-recorder", "private": true }));
    if pkg.get("type").and_then(|t| t.as_str()) != Some("module") {
        pkg["type"] = serde_json::Value::String("module".into());
        std::fs::write(&pkg_path, serde_json::to_string_pretty(&pkg)?)
            .context("write recorder package.json")?;
    }
    let ts_path = home.join("tsconfig.json");
    if !ts_path.exists() {
        std::fs::write(
            &ts_path,
            "{\n  \"compilerOptions\": {\n    \"esModuleInterop\": true,\n    \"allowSyntheticDefaultImports\": true,\n    \"module\": \"commonjs\",\n    \"moduleResolution\": \"node\",\n    \"target\": \"es2020\",\n    \"skipLibCheck\": true\n  }\n}\n",
        )
        .context("write recorder tsconfig.json")?;
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Local mode defaults base_url to the dev origin when `--base-url`
    /// is absent, and otherwise honors it (adding a scheme if bare) —
    /// without ever touching the deploy config or docker.
    #[test]
    fn local_base_url_defaults_and_honors_flag() {
        assert_eq!(local_base_url(None), "http://localhost:3000");
        assert_eq!(
            local_base_url(Some("https://glow.ashoksaravanan.com")),
            "https://glow.ashoksaravanan.com"
        );
        // bare origin gets a scheme, mirroring the docker path's with_scheme.
        assert_eq!(
            local_base_url(Some("glow.ashoksaravanan.com")),
            "https://glow.ashoksaravanan.com"
        );
    }

    /// INTERNAL_API_BASE resolution: explicit `--internal-api-base` always
    /// wins (with a scheme added if bare); otherwise local mode defaults to
    /// the base-url (public demo origin) while docker mode preserves the
    /// existing default of the loopback api/token url.
    #[test]
    fn internal_api_base_resolution() {
        let base = "https://glow.ashoksaravanan.com";
        let token = "http://localhost:8000";

        // docker mode (local_mode = false): unchanged — defaults to token url.
        assert_eq!(internal_api_base(None, false, base, token), token);

        // local mode without an override: defaults to the base-url.
        assert_eq!(internal_api_base(None, true, base, token), base);

        // explicit override wins in either mode.
        assert_eq!(
            internal_api_base(Some("http://localhost:18080"), true, base, token),
            "http://localhost:18080"
        );
        assert_eq!(
            internal_api_base(Some("http://localhost:18080"), false, base, token),
            "http://localhost:18080"
        );

        // a bare host gets a scheme, mirroring the base-url handling.
        assert_eq!(
            internal_api_base(Some("api.internal"), true, base, token),
            "https://api.internal"
        );
    }

    /// Local mode sources specs from `--specs-dir` (no docker): the e2e
    /// tree and playwright.config.ts land in the recorder home exactly as
    /// the docker_cp path would produce them.
    #[test]
    fn copy_local_specs_mirrors_checkout() {
        let tmp = std::env::temp_dir().join(format!("glow-record-test-{}", std::process::id()));
        let specs = tmp.join("checkout");
        let home = tmp.join("home");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(specs.join("e2e/demos")).unwrap();
        std::fs::write(specs.join("e2e/demos/foo.spec.ts"), "// spec").unwrap();
        std::fs::write(specs.join("e2e/helpers.ts"), "// helper").unwrap();
        std::fs::write(specs.join("playwright.config.ts"), "// config").unwrap();
        std::fs::create_dir_all(&home).unwrap();

        copy_local_specs(&specs, &home).unwrap();

        assert!(home.join("e2e/demos/foo.spec.ts").is_file());
        assert!(home.join("e2e/helpers.ts").is_file());
        assert!(home.join("playwright.config.ts").is_file());
        assert_eq!(
            std::fs::read_to_string(home.join("playwright.config.ts")).unwrap(),
            "// config"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
