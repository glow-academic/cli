//! Health-check polling — single source of truth for "is X up?".
//!
//! The spec we ported from `deploy.yml` calls out specific timeouts per
//! service (DB: 75s + 60s start period, Keycloak: 90s + 45s, Server:
//! 50s + 30s). We don't try to mirror those exactly per service — we
//! lean on the HEALTHCHECK directives baked into the compose file and
//! just poll docker inspect at our own cadence.
//!
//! See `runtime::container_health` for the underlying inspect call.

use anyhow::{bail, Result};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::deploy::runtime;

/// Wait until `service` reports "healthy" (or "none" — i.e. no healthcheck
/// is defined and the container is up). Returns Err on timeout or if the
/// container goes "unhealthy" and exhausts its retry budget.
///
/// `label` is just for the progress line — pass a friendly description
/// (`"database"`, `"server-blue"`, …).
pub fn wait_healthy(
    project_name: &str,
    service: &str,
    label: &str,
    overall_timeout: Duration,
) -> Result<()> {
    let started = Instant::now();
    let poll_interval = Duration::from_secs(2);
    let mut consecutive_unhealthy = 0;

    loop {
        let elapsed = started.elapsed();
        if elapsed > overall_timeout {
            bail!(
                "timed out waiting for {label} to become healthy after {}s",
                overall_timeout.as_secs()
            );
        }

        match runtime::container_health(project_name, service)?.as_str() {
            "healthy" | "none" => {
                // "none" = no HEALTHCHECK defined; if compose started the
                // container at all, treat that as good enough.
                println!("    ✓ {label} healthy ({}s)", elapsed.as_secs());
                return Ok(());
            }
            "unhealthy" => {
                consecutive_unhealthy += 1;
                // Per the spec: 3 consecutive failures = bail out so the
                // caller can roll back rather than wait the full timeout.
                if consecutive_unhealthy >= 3 {
                    bail!("{label} reported unhealthy 3 times in a row — giving up");
                }
            }
            // "starting" or any unknown state — keep polling. Don't reset
            // the unhealthy counter; transient flapping is still a failure.
            _ => {}
        }

        sleep(poll_interval);
    }
}

/// Post-traffic-switch monitor: poll the *just-promoted* color for a
/// grace period. If it stays healthy, traffic switch is committed. If it
/// flaps to unhealthy, caller is expected to roll back to the previous
/// color (see `bluegreen::monitor_then_promote`).
///
/// Returns Ok(()) if the service stayed healthy through the window;
/// Err if it failed 3 consecutive polls.
pub fn monitor_after_switch(
    project_name: &str,
    service: &str,
    label: &str,
    grace: Duration,
) -> Result<()> {
    let started = Instant::now();
    let poll_interval = Duration::from_secs(5);
    let mut consecutive_unhealthy = 0;

    // Per the spec: wait 30s before the first check (lets the new color
    // warm up under real traffic before we start scoring it).
    sleep(Duration::from_secs(30).min(grace));

    while started.elapsed() < grace {
        match runtime::container_health(project_name, service)?.as_str() {
            "healthy" | "none" => {
                consecutive_unhealthy = 0;
            }
            "unhealthy" => {
                consecutive_unhealthy += 1;
                if consecutive_unhealthy >= 3 {
                    bail!(
                        "{label} unhealthy 3 polls in a row during grace period — \
                         caller should roll back"
                    );
                }
            }
            _ => {}
        }
        sleep(poll_interval);
    }

    println!(
        "    ✓ {label} stayed healthy through {}s grace window",
        grace.as_secs()
    );
    Ok(())
}
