//! Deploy orchestration — the public entry-points called by clap.
//!
//! Each function here corresponds to one CLI subcommand and ties together
//! the lower-level modules (`compose`, `env`, `runtime`, `bluegreen`,
//! `healthcheck`, `migrations`). The orchestration logic is intentionally
//! linear and explicit — readability wins over abstraction for ops code
//! that has to be debuggable at 2am.
//!
//! Module map:
//!
//! ```text
//!   instance     paths + on-disk state (DeployState)
//!   config       glow-deploy.yaml parsing
//!   env          .env render + first-deploy secret generation
//!   compose      bundled docker-compose.yml writer
//!   runtime      docker compose shellouts
//!   healthcheck  poll containers for health
//!   bluegreen    detect → bring up → swap → monitor → rollback
//!   migrations   make migrate-docker invocations
//! ```

pub mod bluegreen;
pub mod compat;
pub mod compose;
pub mod config;
pub mod env;
pub mod healthcheck;
pub mod instance;
pub mod migrations;
pub mod preflight;
pub mod proxy_hints;
pub mod runtime;
pub mod wizard;

use anyhow::{anyhow, Context, Result};
use std::time::Duration;

use crate::deploy::config::DeployConfig;
use crate::deploy::env::EnvInputs;
use crate::deploy::instance::{DeployState, Instance};

/// Args common to deploy + redeploy. Coming from clap.
pub struct DeployArgs {
    pub name: String,
    pub api_version: String,
    /// Pinned client image tag. Required unless topology=api_only;
    /// in api_only mode the CLI never brings up the client stack.
    pub client_version: Option<String>,
    /// Optional override path; defaults to `<instance-dir>/glow-deploy.yaml`.
    pub config_path: Option<std::path::PathBuf>,
    /// Optional seed setup override (`fresh` / `university` / …).
    /// On first deploy, drives the initial DB content. On redeploy,
    /// ignored unless `--reseed` is also passed (destructive).
    pub seed_setup: Option<String>,
    /// Optional backup filename inside the instance's `backups/` dir to
    /// restore from on deploy. Mutually exclusive with seed_setup —
    /// caller asserts.
    pub db_backup: Option<String>,
    /// Grace period to monitor the new color before committing the
    /// swap. Default 2 min — matches the spec's GRACE_PERIOD_MINUTES.
    pub grace_minutes: u32,
}

impl Default for DeployArgs {
    fn default() -> Self {
        Self {
            name: instance::DEFAULT_NAME.into(),
            api_version: "v1.0.0".into(),
            client_version: Some("v1.0.0".into()),
            config_path: None,
            seed_setup: None,
            db_backup: None,
            grace_minutes: 2,
        }
    }
}

// ── DEPLOY (first time OR re-deploy — they share the engine) ───────

/// Full deploy/redeploy flow. Auto-detects first-vs-redeploy from the
/// presence of `.env`, branches on the few differences (env merge vs
/// clobber, pre-deploy backup, etc.) inline.
///
/// Sequence:
///   1. Validate env (docker compose v2 present, yaml parses, etc).
///   2. Materialize instance dir + compose file.
///   3. Render `.env` (first deploy = generate secrets; redeploy = merge).
///   4. Pre-deploy backup if redeploying with a DB up.
///   5. Pull api image.
///   6. Start DB + redis + nginx if cold start (idempotent — compose
///      no-ops if already running).
///   7. Wait DB healthy.
///   8. Apply add migrations.
///   9. Bring up the OTHER color (bluegreen target).
///  10. Wait target color healthy.
///  11. Swap traffic.
///  12. Monitor grace period — rollback if it flaps.
///  13. Apply remove migrations.
///  14. Commit + persist state.
pub fn deploy(args: DeployArgs) -> Result<()> {
    use crate::deploy::config::Topology;

    let instance = Instance::ensure(&args.name)?;
    instance.mark_active()?;

    // Load config — file must exist by now. If user is wizard-driving,
    // `init` writes the initial yaml; this engine just consumes.
    let cfg_path = args
        .config_path
        .clone()
        .unwrap_or_else(|| instance.deploy_yaml());
    if !cfg_path.exists() {
        return Err(anyhow!(
            "no glow-deploy.yaml at {} — run `glow init` to create one",
            cfg_path.display()
        ));
    }
    let cfg = DeployConfig::load(&cfg_path)?;
    cfg.validate()?;

    let deploy_client = !matches!(cfg.topology, Topology::ApiOnly);
    if deploy_client && args.client_version.is_none() {
        return Err(anyhow!(
            "topology={:?} requires a --client-version (or set client_version in glow-deploy.yaml)",
            cfg.topology
        ));
    }

    // Write both stacks' files. Idempotent — picks up any template
    // changes that ship with a CLI upgrade.
    compose::write_api_stack(&instance.dir, cfg.topology)?;
    if deploy_client {
        compose::write_client_stack(&instance.dir)?;
    }

    // Pre-flight checks AFTER we have origin to inspect, BEFORE we touch
    // docker / network state.
    println!("· pre-flight checks");
    preflight::check_all(&instance.dir, Some(&cfg.effective_client_origin()))?;

    // Cross-repo compat: refuse early if this CLI is older than the api
    // version's published floor. Network failure is a warn (see compat.rs).
    println!("· compat check (api {} ↔ cli)", args.api_version);
    compat::check_api(&args.api_version)?;
    if deploy_client {
        let cv = args.client_version.as_deref().unwrap();
        println!(
            "· compat check (client {cv} ↔ cli + api {})",
            args.api_version
        );
        compat::check_client(cv, &args.api_version)?;
    }

    // Shared docker network — both stacks join it for cross-stack
    // routing (client nginx → api nginx).
    runtime::ensure_network(&instance.shared_network())?;

    // State BEFORE this run — drives blue/green next-color and the
    // first-vs-redeploy fork.
    let mut state = DeployState::load(&instance.state_file())?;
    let is_first_deploy = !instance.api_env_file().exists();

    // Pre-deploy backup on redeploy if DB is running — best-effort.
    if !is_first_deploy {
        if let Err(e) = pre_deploy_backup(&instance) {
            eprintln!("  ! pre-deploy backup failed (continuing): {e}");
        }
    }

    let plan = bluegreen::plan_swap(&state);
    let target_env = plan.to_env.clone();
    let prev_env = if is_first_deploy {
        target_env.clone()
    } else {
        plan.from_env.clone()
    };

    let (api_origin, api_client_origins) = env::derive_api_origins(&cfg);

    println!(
        "→ deploy `{}` ({:?}) — api={} client={} (target color: {target_env})",
        args.name,
        cfg.topology,
        if api_origin.is_empty() {
            "(internal)"
        } else {
            &api_origin
        },
        if cfg.effective_client_origin().is_empty() {
            "(none)".to_string()
        } else {
            cfg.effective_client_origin()
        },
    );

    // ── Render api .env (first deploy seeds secrets; redeploy merges).
    let env_inputs = EnvInputs {
        api_version: args.api_version.clone(),
        origin: api_origin,
        client_origins: api_client_origins,
        active_env: prev_env.clone(),
        active_kc_env: prev_env.clone(),
        project_name: instance.api_project_name(),
        seed_setup: resolve_seed_setup(
            is_first_deploy,
            args.seed_setup.as_deref(),
            cfg.setup.as_deref(),
            &state,
        ),
        // TRANSIENT force-reseed: set FORCE_RESEED=1 iff an explicit
        // `--reseed <setup>` was passed THIS invocation. Derived from the
        // arg (NOT the persisted seed_setup state), so a plain redeploy
        // renders no FORCE_RESEED and clears any stale one.
        force_reseed: args.seed_setup.is_some(),
        db_backup: args.db_backup.clone(),
        grace_period_minutes: args.grace_minutes,
        app_prefix: String::new(),
        glow_network: instance.shared_network(),
    };
    env::render(&instance.api_env_file(), &env_inputs)?;

    // ── API stack: pull → base → wait DB → add migrations → target color.
    runtime::pull(
        &instance.api_dir(),
        &instance.api_project_name(),
        &["server-blue", "server-green"],
    )?;
    println!("  · bringing up shared base (database + redis + nginx)");
    runtime::up(
        &instance.api_dir(),
        &instance.api_project_name(),
        &["database", "redis", "nginx"],
    )?;
    healthcheck::wait_healthy(
        &instance.api_project_name(),
        "database",
        "database",
        Duration::from_secs(180),
    )?;
    // Run the one-shot seed runner against the live DB. Idempotent —
    // exits 0 immediately if the target DB already has public tables,
    // so re-deploys don't re-seed. First deploy runs the full seed,
    // which can take a few minutes depending on setup.
    println!("  · running db-init (first-deploy seed, no-op on redeploy)");
    runtime::up(
        &instance.api_dir(),
        &instance.api_project_name(),
        &["db-init"],
    )?;
    if !is_first_deploy {
        let server = format!("server-{prev_env}");
        if let Err(e) =
            migrations::run_add(&instance.api_dir(), &instance.api_project_name(), &server)
        {
            eprintln!("  ! add migrations failed (continuing): {e}");
        }
    }
    bluegreen::bring_up_api_target(
        &instance.api_dir(),
        &instance.api_project_name(),
        &target_env,
    )?;

    // ── Client stack (skipped in api_only topology).
    let client_target = if deploy_client {
        let ct = state.next_client_env().to_string();
        let cv = args.client_version.clone().unwrap();
        let (public_api_url, _kc_public_url) = env::derive_client_public_urls(&cfg);
        let client_origin = cfg.effective_client_origin();
        // Strip scheme for DOMAIN (used as nginx server_name).
        let domain = client_origin
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/')
            .to_string();
        // Reuse the api's SECRET_KEY + AUTH_CLIENT_SECRET so NextAuth
        // can verify api-minted JWTs and the OIDC handshake works.
        let (auth_secret, auth_kc_secret) = env::read_api_shared_secrets(&instance.api_env_file())?;
        // The academic client's `glow` OIDC provider issuer is the api's
        // ORIGIN-derived URL — public-api URL for both airgapped (api
        // proxied via client nginx) and exposed (api on its own domain).
        // Falls back to client_origin when somehow empty.
        let auth_issuer = if public_api_url.is_empty() {
            client_origin.clone()
        } else {
            public_api_url.clone()
        };
        let client_inputs = env::ClientEnvInputs {
            client_version: cv.clone(),
            active_client_env: state
                .active_client_env
                .clone()
                .unwrap_or_else(|| ct.clone()),
            project_name: instance.client_project_name(),
            glow_network: instance.shared_network(),
            client_http_port: cfg
                .client_http_port
                .clone()
                .unwrap_or_else(|| "127.0.0.1:18080".into()),
            domain,
            public_api_url,
            internal_api_base: format!("http://{}-nginx:80", instance.api_project_name()),
            auth_secret,
            auth_issuer,
            // Container-reachable api URL — same alias the client nginx
            // already uses for MCP/api/auth proxying. Works in local
            // (single host) AND prod (the shared docker net is private
            // either way).
            auth_issuer_internal: format!("http://{}-nginx:80", instance.api_project_name()),
            auth_client_id: "glow-client".into(),
            auth_client_secret: auth_kc_secret,
            nextauth_url: client_origin.clone(),
            mcp_backend: format!("{}-nginx:80", instance.api_project_name()),
        };
        env::render_client(&instance.client_env_file(), &client_inputs)?;

        runtime::pull(
            &instance.client_dir(),
            &instance.client_project_name(),
            &["app-blue", "app-green"],
        )?;
        // Bring up the client's nginx so docker-gen has somewhere to
        // signal, then bring up the target app color.
        runtime::up(
            &instance.client_dir(),
            &instance.client_project_name(),
            &["nginx", "docker-gen"],
        )?;
        bluegreen::bring_up_client_target(
            &instance.client_dir(),
            &instance.client_project_name(),
            &ct,
        )?;
        Some(ct)
    } else {
        None
    };

    // ── First-deploy fast-path: no traffic to swap.
    if is_first_deploy {
        state.active_env = Some(target_env.clone());
        state.active_kc_env = Some(target_env.clone());
        state.api_version = Some(args.api_version.clone());
        if let Some(ct) = &client_target {
            state.active_client_env = Some(ct.clone());
            state.client_version = args.client_version.clone();
        }
        let now = chrono_iso8601_now();
        state.first_deployed_at = Some(now.clone());
        state.last_deployed_at = Some(now);
        state.initial_seed_setup = env_inputs.seed_setup.clone();
        // Remember the effective setup so future plain redeploys render a
        // matching SEED_SETUP (db-init idempotent skip, data preserved).
        state.seed_setup = env_inputs.seed_setup.clone();
        state.save(&instance.state_file())?;

        // Point each nginx at its freshly-promoted color.
        bluegreen::switch_traffic(
            &instance.api_env_file(),
            &instance.api_project_name(),
            &target_env,
        )?;
        if let Some(ct) = &client_target {
            bluegreen::switch_client_traffic(
                &instance.client_env_file(),
                &instance.client_project_name(),
                ct,
            )?;
        }

        println!("\n✓ first deploy complete — {} is live", args.name);
        let (public_url, published) = published_endpoint(&cfg);
        if let Some(ct) = &client_target {
            println!("  api color: {target_env}   client color: {ct}");
        } else {
            println!("  api color: {target_env}");
        }
        if !public_url.is_empty() {
            println!("  open: {public_url}");
        }
        println!("  bound: {published}");
        proxy_hints::print_proxy_hints(&public_url, &published);
        return Ok(());
    }

    // ── Redeploy path: swap api, monitor, rollback-or-commit.
    bluegreen::switch_traffic(
        &instance.api_env_file(),
        &instance.api_project_name(),
        &target_env,
    )?;
    let grace = Duration::from_secs(args.grace_minutes as u64 * 60);
    bluegreen::monitor_then_commit_or_rollback(
        &instance,
        &mut state,
        &prev_env,
        &target_env,
        grace,
    )?;

    // Then client swap (only if there's a client to swap).
    if let Some(ct) = &client_target {
        let prev_client = state
            .active_client_env
            .clone()
            .unwrap_or_else(|| ct.clone());
        bluegreen::switch_client_traffic(
            &instance.client_env_file(),
            &instance.client_project_name(),
            ct,
        )?;
        bluegreen::monitor_client_then_commit_or_rollback(
            &instance,
            &mut state,
            &prev_client,
            ct,
            grace,
        )?;
        state.client_version = args.client_version.clone();
    }

    // Post-promotion destructive migrations (drops, etc.).
    let new_server = format!("server-{target_env}");
    if let Err(e) = migrations::run_remove(
        &instance.api_dir(),
        &instance.api_project_name(),
        &new_server,
    ) {
        eprintln!("  ! remove migrations failed (non-fatal, continuing): {e}");
    }

    state.api_version = Some(args.api_version);
    // A `--reseed <setup>` on redeploy re-seeds the DB, so update the
    // remembered effective setup. A plain redeploy (no override) leaves it
    // untouched — it already drove this run's SEED_SETUP.
    if let Some(s) = &env_inputs.seed_setup {
        if args.seed_setup.is_some() {
            state.seed_setup = Some(s.clone());
        }
    }
    state.last_deployed_at = Some(chrono_iso8601_now());
    state.save(&instance.state_file())?;

    println!(
        "\n✓ redeploy complete — {} is live on {target_env}",
        args.name
    );
    let (public_url, _published) = published_endpoint(&cfg);
    if !public_url.is_empty() {
        println!("  open: {public_url}");
    }
    Ok(())
}

/// (public_url, published-host-port) for the current deploy. `public_url`
/// is what the user typed in glow-deploy.yaml; `published` is what the
/// stack is actually bound to (env-override aware).
fn published_endpoint(cfg: &DeployConfig) -> (String, String) {
    use crate::deploy::config::Topology;
    match cfg.topology {
        Topology::Airgapped | Topology::Exposed => {
            let port = cfg
                .client_http_port
                .clone()
                .unwrap_or_else(|| "127.0.0.1:18080".into());
            (cfg.effective_client_origin(), port)
        }
        Topology::ApiOnly => {
            // No client stack. api_only uses API_HTTP_PORT env (compose
            // template default `127.0.0.1:18081`) — we don't model that
            // in the config struct, so report the default and let the
            // user override via .env if they need to.
            (cfg.effective_api_origin(), "127.0.0.1:18081".into())
        }
    }
}

// ── STOP / START / DESTROY ─────────────────────────────────────────

pub fn stop(name: &str) -> Result<()> {
    runtime::assert_docker_available()?;
    let i = Instance::open(name)?;
    runtime::stop(&i.dir, &i.project_name())?;
    println!("✓ stopped — volumes intact, `glow start` to resume");
    Ok(())
}

pub fn start(name: &str) -> Result<()> {
    runtime::assert_docker_available()?;
    let i = Instance::open(name)?;
    runtime::start(&i.dir, &i.project_name())?;
    println!("✓ started");
    Ok(())
}

pub fn destroy(name: &str) -> Result<()> {
    runtime::assert_docker_available()?;
    let i = Instance::open(name)?;

    // Two-stack layout: the instance has no top-level docker-compose.yml — it's
    // `api/docker-compose.yml` + `client/docker-compose.yml`, each its own
    // compose project. The old single `down_destroy(&i.dir, …)` looked for a
    // compose file that doesn't exist → "no such file or directory", aborting
    // and leaving every container + volume + the shared network orphaned.
    //
    // Tear down the client stack first (its containers join the shared network),
    // then the api stack (which owns the network and removes it on the way out).
    // Skip a stack whose compose file is absent (e.g. an api-only topology).
    let mut errors: Vec<String> = Vec::new();
    for (dir, project, label) in [
        (i.client_dir(), i.client_project_name(), "client"),
        (i.api_dir(), i.api_project_name(), "api"),
    ] {
        if !dir.join("docker-compose.yml").exists() {
            continue;
        }
        if let Err(e) = runtime::down_destroy(&dir, &project) {
            errors.push(format!("{label} stack: {e}"));
        }
    }
    if !errors.is_empty() {
        anyhow::bail!(
            "destroy incomplete — some stacks failed:\n  {}",
            errors.join("\n  ")
        );
    }

    // We deliberately don't `rm -rf` the instance dir here — the user's
    // glow-deploy.yaml + backups are theirs. They can rm by hand if they
    // really want a clean slate.
    println!("✓ destroyed containers + volumes for `{name}`");
    println!("  (config + backups preserved at {})", i.dir.display());
    Ok(())
}

// ── STATUS ─────────────────────────────────────────────────────────

pub fn status(name: &str) -> Result<()> {
    runtime::assert_docker_available()?;
    let i = Instance::open(name)?;
    let state = DeployState::load(&i.state_file())?;

    println!("instance:    {}", i.name);
    println!("dir:         {}", i.dir.display());
    println!("project:     {}", i.project_name());
    println!(
        "active env:  {}",
        state.active_env.as_deref().unwrap_or("none")
    );
    println!(
        "api version: {}",
        state.api_version.as_deref().unwrap_or("unknown")
    );
    println!(
        "first deploy:{}",
        state.first_deployed_at.as_deref().unwrap_or("never")
    );
    println!(
        "last deploy: {}",
        state.last_deployed_at.as_deref().unwrap_or("never")
    );
    println!();

    // Health snapshot for the key services.
    println!("container health:");
    for svc in [
        "database",
        "redis",
        "nginx",
        "keycloak-blue",
        "keycloak-green",
        "server-blue",
        "server-green",
    ] {
        // These services live in the API stack — project `glow-<name>-api`
        // (container `glow-<name>-api-<svc>-1`). Using the bare project name
        // misses every container, so health always read "starting".
        let h = runtime::container_health(&i.api_project_name(), svc)
            .unwrap_or_else(|_| "missing".into());
        println!("  {svc:18}  {h}");
    }
    Ok(())
}

/// Resolve the `SEED_SETUP` value to render into the api `.env` for this run.
///
/// - First deploy: the explicit `--seed-setup` override, else the config's
///   `setup`, else `None` (compose falls back to `fresh`).
/// - Redeploy with `--reseed <setup>` (i.e. an explicit `seed_arg`): that
///   override — it re-seeds destructively.
/// - Plain redeploy (no override): the instance's PERSISTED effective setup
///   (`seed_setup`, falling back to the legacy `initial_seed_setup`). This is
///   the fix for the post-#240 fragility: without it, redeploy rendered no
///   `SEED_SETUP`, compose defaulted to `fresh`, and the api's `database.init`
///   guard failed-loud on the fresh-over-`university` mismatch — blocking
///   every plain redeploy. Rendering the persisted setup makes db-init see a
///   matching setup → idempotent skip → data preserved, no `--reseed` needed.
fn resolve_seed_setup(
    is_first_deploy: bool,
    seed_arg: Option<&str>,
    cfg_setup: Option<&str>,
    state: &DeployState,
) -> Option<String> {
    if is_first_deploy {
        seed_arg.or(cfg_setup).map(str::to_string)
    } else if let Some(s) = seed_arg {
        // `--reseed <setup>` — explicit destructive override.
        Some(s.to_string())
    } else {
        // Plain redeploy — default to the instance's remembered setup.
        state.effective_seed_setup()
    }
}

// ── helpers ────────────────────────────────────────────────────────

/// Pre-deploy snapshot — pg_dump → gzip → backups/backup-deploy-<ts>.sql.gz.
/// We retain the 7 most recent (rolling window per the workflow spec).
fn pre_deploy_backup(instance: &Instance) -> Result<()> {
    println!("  · pre-deploy backup");
    let ts = chrono_compact_now();
    let filename = format!("backup-deploy-{ts}.sql.gz");
    let target = instance.backups_dir().join(&filename);

    // Run pg_dump inside the DB container, gzip in pipe, write to host.
    // We shell out using docker compose exec which the runtime module
    // proxies. Capture stdout to file.
    //
    // The `database` service lives in the API stack — compose project
    // `glow-<name>-api`, files under `<instance>/api/` — NOT the bare
    // instance root (which has no docker-compose.yml at all in the
    // two-stack layout). Using the root dir/project here meant compose
    // ran without the api stack's `.env`, so `$POSTGRES_USER` fell
    // through to the `${DB_USER:-myuser}` default instead of the real
    // role `glow`, and the dump failed with `role "myuser" does not
    // exist`. Mirror `backup::create()` and target the api stack. (#123)
    let project_dir = instance.api_dir();
    let project_name = instance.api_project_name();
    let dump = runtime::exec_capture(
        &project_dir,
        &project_name,
        "database",
        &[
            "sh",
            "-c",
            "pg_dump --exclude-schema=keycloak -U $POSTGRES_USER $POSTGRES_DB | gzip",
        ],
    )
    .context("pg_dump inside database container")?;
    std::fs::write(&target, dump).with_context(|| format!("write backup: {}", target.display()))?;

    // Retention: keep newest 7.
    rotate_backups(&instance.backups_dir(), "backup-deploy-", 7).ok();

    println!("    ✓ {}", target.display());
    Ok(())
}

/// Drop backups beyond `keep` newest matching `prefix`. Best-effort —
/// failures here are non-fatal.
fn rotate_backups(dir: &std::path::Path, prefix: &str, keep: usize) -> Result<()> {
    let mut entries: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with(prefix))
        .collect();
    entries.sort_by_key(|e| std::cmp::Reverse(e.metadata().and_then(|m| m.modified()).ok()));
    for e in entries.into_iter().skip(keep) {
        let _ = std::fs::remove_file(e.path());
    }
    Ok(())
}

/// "2026-05-17T15:30:45Z" — pure rust, no chrono dep needed.
fn chrono_iso8601_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let secs_in_day = 86400;
    let days = now / secs_in_day;
    let rem = now % secs_in_day;
    let h = rem / 3600;
    let m = (rem % 3600) / 60;
    let s = rem % 60;
    let (y, mo, d) = days_to_ymd(days as i64);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

/// Compact timestamp for filenames: "20260517T153045Z".
fn chrono_compact_now() -> String {
    chrono_iso8601_now().replace(['-', ':'], "")
}

/// Days-since-epoch → (year, month, day) per the algorithm in Howard
/// Hinnant's date.h. ~30 lines but stable; avoids the chrono dep.
fn days_to_ymd(days: i64) -> (i64, u32, u32) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y_adj = if m <= 2 { y + 1 } else { y };
    (y_adj, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_with(seed: Option<&str>, initial: Option<&str>) -> DeployState {
        DeployState {
            seed_setup: seed.map(str::to_string),
            initial_seed_setup: initial.map(str::to_string),
            ..Default::default()
        }
    }

    #[test]
    fn first_deploy_prefers_arg_then_cfg_then_none() {
        let s = DeployState::default();
        // explicit --seed-setup wins.
        assert_eq!(
            resolve_seed_setup(true, Some("university"), Some("fresh"), &s),
            Some("university".into())
        );
        // falls back to cfg.setup.
        assert_eq!(
            resolve_seed_setup(true, None, Some("university"), &s),
            Some("university".into())
        );
        // nothing specified → None (compose defaults to `fresh`).
        assert_eq!(resolve_seed_setup(true, None, None, &s), None);
    }

    #[test]
    fn plain_redeploy_uses_persisted_setup() {
        // The fix: no --reseed → render the instance's remembered setup,
        // NOT None. cfg.setup is ignored on redeploy.
        let s = state_with(Some("university"), Some("university"));
        assert_eq!(
            resolve_seed_setup(false, None, Some("fresh"), &s),
            Some("university".into())
        );
    }

    #[test]
    fn plain_redeploy_falls_back_to_initial_when_seed_setup_absent() {
        // Migration case: a pre-existing state file has only
        // `initial_seed_setup` (the `seed_setup` field defaults to None).
        let s = state_with(None, Some("university"));
        assert_eq!(
            resolve_seed_setup(false, None, None, &s),
            Some("university".into())
        );
    }

    #[test]
    fn reseed_overrides_persisted_setup_on_redeploy() {
        let s = state_with(Some("university"), Some("university"));
        assert_eq!(
            resolve_seed_setup(false, Some("fresh"), None, &s),
            Some("fresh".into())
        );
    }
}
