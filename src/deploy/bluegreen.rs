//! Blue/green orchestration — the "swap traffic, monitor, rollback on
//! failure" pattern lifted from `deploy.yml` / `scripts/monitor.sh`.
//!
//! Sequence (the bit that has to be bulletproof — direct port from the
//! workflow spec):
//!
//! 1. Detect the currently-active color (from state file, fallback "blue").
//! 2. Compute the target color (the other one).
//! 3. Pull + bring up the target color's `server-<target>` container.
//! 4. Wait for it to report healthy.
//! 5. Update `ACTIVE_ENV` in `.env` + restart nginx so traffic flips.
//! 6. Monitor the new color for a grace period (30s warmup + N min watch).
//! 7. If healthy throughout → commit the swap to state.json.
//!    If unhealthy → flip `ACTIVE_ENV` back, restart nginx, bail.
//!
//! Keycloak's blue/green pair is orchestrated the same way via the
//! `ACTIVE_KC_ENV` variable. We keep them in lockstep for v1.0.0 —
//! they swap together — because decoupling adds tons of state without
//! buying anything for single-instance OSS.

use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;

use crate::deploy::healthcheck;
use crate::deploy::instance::{DeployState, Instance};
use crate::deploy::runtime;

/// Plan returned by `plan_swap` — the orchestrator-level "what's about
/// to happen" so the caller can print it before doing the work.
pub struct SwapPlan {
    /// The color currently serving traffic (from state file).
    pub from_env: String,
    /// The color we're about to bring up + promote.
    pub to_env: String,
}

/// Pure function — figure out the swap plan from the current state.
/// Doesn't touch docker or filesystem.
pub fn plan_swap(state: &DeployState) -> SwapPlan {
    let from = state.active_env.clone().unwrap_or_else(|| "none".into());
    let to = state.next_env().to_string();
    SwapPlan {
        from_env: from,
        to_env: to,
    }
}

/// Bring up the api server + keycloak pair for `target_env` and wait
/// for both to be healthy. Caller is responsible for the upstream
/// orchestration (`docker compose up` of base services like the DB,
/// which both colors share, must happen first).
///
/// Service-name convention: we expect the compose file to declare
/// `server-blue` / `server-green` and `keycloak-blue` / `keycloak-green`.
/// If we ever rename, change here and the compose template in lockstep.
pub fn bring_up_api_target(project_dir: &Path, project_name: &str, target_env: &str) -> Result<()> {
    let server = format!("server-{target_env}");
    let keycloak = format!("keycloak-{target_env}");

    println!("  · bringing up {server} + {keycloak} + docker-gen");
    // docker-gen watches the docker socket and regenerates the nginx
    // upstreams.conf when server-* / keycloak-* containers appear.
    // Without it, nginx is stuck pointing at "127.0.0.1:1 down" from
    // the seed file and proxies return 502.
    runtime::up(project_dir, project_name, &[&server, &keycloak, "docker-gen"])?;

    println!("  · waiting for {keycloak} to become healthy (up to 3min)");
    healthcheck::wait_healthy(project_name, &keycloak, &keycloak, Duration::from_secs(180))?;

    // First-boot servers retry against keycloak (which itself may still
    // be warming up) for ~60-90s before becoming healthy. Give 4min on
    // first deploy; subsequent deploys are usually <30s.
    println!("  · waiting for {server} to become healthy (up to 4min)");
    healthcheck::wait_healthy(project_name, &server, &server, Duration::from_secs(240))?;

    Ok(())
}

/// Bring up the client app target color + wait for health. Mirror of
/// `bring_up_api_target` but for the single `app-<env>` service that
/// makes up a client stack color.
pub fn bring_up_client_target(
    project_dir: &Path,
    project_name: &str,
    target_env: &str,
) -> Result<()> {
    let app = format!("app-{target_env}");
    println!("  · bringing up {app}");
    runtime::up(project_dir, project_name, &[&app])?;
    println!("  · waiting for {app} to become healthy (up to 2min)");
    healthcheck::wait_healthy(project_name, &app, &app, Duration::from_secs(120))?;
    Ok(())
}

/// Flip `ACTIVE_ENV` / `ACTIVE_KC_ENV` in the api stack's env file and
/// restart its nginx so it re-resolves the upstream. This is the
/// actual "traffic switch" step — until this returns Ok, the old api
/// color is still serving.
///
/// We rewrite `.env` directly (it's a simple key=value file) rather than
/// calling back into `env::render` because at this point we only want
/// to flip two keys without disturbing anything else.
pub fn switch_traffic(env_path: &Path, project_name: &str, target_env: &str) -> Result<()> {
    println!("  · flipping ACTIVE_ENV → {target_env} in api .env");
    flip_env_key(env_path, "ACTIVE_ENV", target_env)?;
    flip_env_key(env_path, "ACTIVE_KC_ENV", target_env)?;

    println!("  · restarting api nginx so the new upstream takes effect");
    let project_dir = env_path.parent().context("env_path has no parent")?;
    runtime::up(project_dir, project_name, &["nginx"])?;

    Ok(())
}

/// Same as `switch_traffic` but for the client stack — flips
/// `ACTIVE_CLIENT_ENV` and restarts the client nginx.
pub fn switch_client_traffic(env_path: &Path, project_name: &str, target_env: &str) -> Result<()> {
    println!("  · flipping ACTIVE_CLIENT_ENV → {target_env} in client .env");
    flip_env_key(env_path, "ACTIVE_CLIENT_ENV", target_env)?;
    println!("  · restarting client nginx");
    let project_dir = env_path.parent().context("env_path has no parent")?;
    runtime::up(project_dir, project_name, &["nginx"])?;
    Ok(())
}

/// After the swap, watch the new color for `grace`. If it stays healthy,
/// the swap is "committed" and `state.active_env` should be updated by
/// the caller. If it fails, we automatically flip ACTIVE_ENV back to
/// `prev_env` and restart nginx so traffic returns to the old color.
pub fn monitor_then_commit_or_rollback(
    instance: &Instance,
    state: &mut DeployState,
    prev_env: &str,
    new_env: &str,
    grace: Duration,
) -> Result<()> {
    let server = format!("server-{new_env}");
    let project = instance.api_project_name();

    match healthcheck::monitor_after_switch(&project, &server, &server, grace) {
        Ok(()) => {
            state.active_env = Some(new_env.into());
            state.active_kc_env = Some(new_env.into());
            state.save(&instance.state_file())?;
            println!("  ✓ api swap committed: {prev_env} → {new_env}");
            Ok(())
        }
        Err(monitor_err) => {
            eprintln!("  ! grace period failed: {monitor_err}");
            eprintln!("  · rolling back: flipping ACTIVE_ENV back to {prev_env}");

            if let Err(rb_err) = switch_traffic(&instance.api_env_file(), &project, prev_env) {
                anyhow::bail!(
                    "monitor failed AND rollback failed — manual intervention needed.\n  monitor: {monitor_err}\n  rollback: {rb_err}"
                );
            }

            anyhow::bail!(
                "deploy rolled back automatically — api {new_env} stayed unhealthy. \
                 The old color ({prev_env}) is still serving traffic."
            );
        }
    }
}

/// Same grace-window-or-rollback semantics for the client stack.
pub fn monitor_client_then_commit_or_rollback(
    instance: &Instance,
    state: &mut DeployState,
    prev_env: &str,
    new_env: &str,
    grace: Duration,
) -> Result<()> {
    let app = format!("app-{new_env}");
    let project = instance.client_project_name();

    match healthcheck::monitor_after_switch(&project, &app, &app, grace) {
        Ok(()) => {
            state.active_client_env = Some(new_env.into());
            state.save(&instance.state_file())?;
            println!("  ✓ client swap committed: {prev_env} → {new_env}");
            Ok(())
        }
        Err(monitor_err) => {
            eprintln!("  ! grace period failed: {monitor_err}");
            eprintln!("  · rolling back: flipping ACTIVE_CLIENT_ENV back to {prev_env}");
            if let Err(rb_err) =
                switch_client_traffic(&instance.client_env_file(), &project, prev_env)
            {
                anyhow::bail!(
                    "client monitor failed AND rollback failed — manual intervention needed.\n  monitor: {monitor_err}\n  rollback: {rb_err}"
                );
            }
            anyhow::bail!(
                "client deploy rolled back automatically — {new_env} stayed unhealthy. \
                 The old color ({prev_env}) is still serving traffic."
            );
        }
    }
}

/// In-place mutation of a single key in a .env-style file. Keeps every
/// other line untouched (comments, ordering, secrets).
fn flip_env_key(path: &Path, key: &str, value: &str) -> Result<()> {
    let body = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let mut out = String::with_capacity(body.len());
    let mut found = false;
    for line in body.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(&format!("{key}=")) {
            let _ = rest; // not interested in old value
            out.push_str(key);
            out.push('=');
            out.push_str(value);
            out.push('\n');
            found = true;
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    if !found {
        // First time setting this key — append.
        out.push_str(&format!("{key}={value}\n"));
    }
    std::fs::write(path, out).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
