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
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::deploy::runtime;

/// The readiness route the probe hits. `/readyz` is UNAUTHENTICATED and runs
/// the real dependency checks a request needs — a live `SELECT 1` on the DB
/// pool and a Redis `PING` — returning **200** when both are up and **503**
/// when either is down (fail-closed).
///
/// DEP-A (report-18 D1): we previously hit the authed `/system/health` with
/// no token and accepted the **401** the auth middleware raises BEFORE the
/// pool/Redis are ever touched. That "proves" only that the router mounted —
/// a color with a broken DB_HOST / Redis-down / Keycloak-unreachable config
/// boots, serves that 401, passes the gate, swaps to live, then 500s every
/// authenticated request (and the grace monitor, using the same blind probe,
/// never rolls back). `/readyz` closes that gap: it fails (503) exactly when
/// the color can't serve real traffic, so the broken color is caught before
/// the swap.
const READINESS_PATH: &str = "/readyz";

/// What a single readiness probe attempt concluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Readiness {
    /// `/readyz` returned 200 — DB + Redis are both up. The color can serve
    /// real requests.
    Ready,
    /// The color answered, but not in a way that proves it's serving
    /// (5xx, or an unexpected status). Keep waiting / fail the gate.
    NotReady,
    /// No HTTP answer at all (connection refused / probe couldn't run).
    /// Still booting, or hard-down. Keep waiting / fail the gate.
    Unreachable,
}

/// Pure classification of an HTTP status code from the readiness probe
/// into a verdict. Split out so the gate logic is unit-testable without
/// docker. `status == 0` is our sentinel for "no HTTP response at all"
/// (connection refused / timeout) emitted by the in-container probe.
///
/// `/readyz` returns **200** iff DB + Redis are both up, **503** when either
/// is down (fail-closed). So 200 is the ONLY pass; 503 (broken dependency),
/// any other 5xx (gunicorn down / app raised), a 401/404 (wrong route / not
/// the readiness endpoint), or `0` (no answer) are NOT a pass. Deliberately
/// strict on the failure side and lenient on time (the caller retries through
/// a generous grace window), so a slow-but-healthy boot is never failed for
/// being slow — only a genuinely-broken color is.
pub fn classify_readiness(status: u16) -> Readiness {
    match status {
        0 => Readiness::Unreachable,
        200 => Readiness::Ready,
        _ => Readiness::NotReady,
    }
}

/// In-container readiness probe: GET the unauthenticated `/readyz` and
/// return the HTTP status code (`200` ready / `503` not-ready, or `0` if the
/// connection itself failed). Runs via `docker compose exec` against the
/// color's `server-<env>` container — same `python3` + `urllib` the compose
/// HEALTHCHECK already relies on, so it needs no extra tooling, no host port
/// mapping, no public URL, and no secrets/token.
fn probe_readiness_status(project_dir: &Path, project_name: &str, service: &str) -> Result<u16> {
    // GET /readyz — no token, no body. `getcode()` gives 200 when ready; a
    // 503 (or other status) comes back via HTTPError.code; a raw connection
    // failure prints 0. One-liner so it's a single, quoted exec arg.
    let script = format!(
        "import urllib.request as u\n\
         try:\n    r=u.urlopen('http://localhost:8000{READINESS_PATH}',timeout=5);print(r.getcode())\n\
         except u.HTTPError as e:\n    print(e.code)\n\
         except Exception:\n    print(0)\n"
    );
    let (_ok, stdout, _stderr) = runtime::exec_status(
        project_dir,
        project_name,
        service,
        &["python3", "-c", &script],
    )?;
    // The probe prints exactly one integer line; tolerate trailing noise.
    let code = stdout
        .lines()
        .rev()
        .find_map(|l| l.trim().parse::<u16>().ok())
        .unwrap_or(0);
    Ok(code)
}

/// Poll the readiness probe on `server-<env>` until it reports `Ready`, or
/// the timeout elapses. This is the **readiness gate** that fronts the
/// blue/green swap: a color must pass `/readyz` (200 → DB + Redis both up) —
/// not merely have a running container — before we commit traffic to it.
///
/// The grace window here is generous on purpose: the box is load-sensitive
/// and a healthy color can take a while to warm up. We never fail a color
/// for being *slow* — only for never becoming ready within the window.
///
/// `Unreachable` early on is normal (the app is still booting); we keep
/// polling. We only give up on `timeout`.
pub fn wait_ready(
    project_dir: &Path,
    project_name: &str,
    service: &str,
    label: &str,
    overall_timeout: Duration,
) -> Result<()> {
    let started = Instant::now();
    let poll_interval = Duration::from_secs(3);

    loop {
        let elapsed = started.elapsed();
        if elapsed > overall_timeout {
            bail!(
                "timed out waiting for {label} to become READY (pass /readyz) \
                 after {}s — liveness may be up but the app can't serve real traffic \
                 (broken DB/Redis/Keycloak config or a startup error). Refusing to swap.",
                overall_timeout.as_secs()
            );
        }

        // A spawn error (container not up yet) is treated as Unreachable, not
        // a hard failure — keep polling until the timeout.
        let verdict = match probe_readiness_status(project_dir, project_name, service) {
            Ok(code) => classify_readiness(code),
            Err(_) => Readiness::Unreachable,
        };

        match verdict {
            Readiness::Ready => {
                println!(
                    "    ✓ {label} READY — DB + Redis up (/readyz 200) ({}s)",
                    elapsed.as_secs()
                );
                return Ok(());
            }
            Readiness::NotReady | Readiness::Unreachable => {}
        }

        sleep(poll_interval);
    }
}

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
///
/// `readiness_dir` opts into the DEP-A readiness probe: when `Some`, the
/// color must additionally keep passing `/readyz` (DB + Redis up) through the
/// window (used for the api server, which exposes `/readyz` + in-container
/// `python3`). Pass `None` to keep the liveness-only behavior for stacks
/// without that probe surface (the client app color).
pub fn monitor_after_switch(
    readiness_dir: Option<&Path>,
    project_name: &str,
    service: &str,
    label: &str,
    grace: Duration,
) -> Result<()> {
    let started = Instant::now();
    let poll_interval = Duration::from_secs(5);
    let mut consecutive_unhealthy = 0;
    let mut consecutive_not_ready = 0;

    // Per the spec: wait 30s before the first check (lets the new color
    // warm up under real traffic before we start scoring it).
    sleep(Duration::from_secs(30).min(grace));

    while started.elapsed() < grace {
        // Liveness (docker healthcheck) — container-level flapping.
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

        // READINESS (DEP1) — the now-live color must keep serving real
        // authed requests, not just answer the static liveness probe. A
        // color that boots, passes liveness, gets promoted, then starts
        // 5xx-ing on real traffic (e.g. its DB pool drops) would have
        // sailed through the old liveness-only monitor; here it trips the
        // rollback. `Unreachable` is tolerated transiently (restart/blip)
        // — only a sustained NOT-ready/unreachable streak rolls back.
        if let Some(dir) = readiness_dir {
            let verdict = match probe_readiness_status(dir, project_name, service) {
                Ok(code) => classify_readiness(code),
                Err(_) => Readiness::Unreachable,
            };
            match verdict {
                Readiness::Ready => consecutive_not_ready = 0,
                Readiness::NotReady | Readiness::Unreachable => {
                    consecutive_not_ready += 1;
                    if consecutive_not_ready >= 3 {
                        bail!(
                            "{label} failed the readiness probe 3 polls in a row during the \
                             grace period (live container but can't serve authed requests) — \
                             caller should roll back"
                        );
                    }
                }
            }
        }

        sleep(poll_interval);
    }

    let what = if readiness_dir.is_some() {
        "healthy + ready"
    } else {
        "healthy"
    };
    println!(
        "    ✓ {label} stayed {what} through {}s grace window",
        grace.as_secs()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── classify_readiness: the core gate decision ──────────────────

    #[test]
    fn only_200_is_ready() {
        // /readyz returns 200 iff DB + Redis are both up. This is the ONLY
        // pass condition.
        assert_eq!(classify_readiness(200), Readiness::Ready);
    }

    #[test]
    fn server_errors_are_not_ready() {
        // The exact DEP-A failure modes: a color whose DB/Redis is down
        // answers 503 from /readyz (fail-closed); a crash-looping app
        // answers another 5xx / bad-gateway. None must pass the gate.
        for code in [500u16, 502, 503, 504] {
            assert_eq!(
                classify_readiness(code),
                Readiness::NotReady,
                "HTTP {code} must not pass readiness"
            );
        }
    }

    #[test]
    fn connection_refused_is_unreachable() {
        // Sentinel 0 = no HTTP answer at all (still booting, or hard-down).
        assert_eq!(classify_readiness(0), Readiness::Unreachable);
    }

    #[test]
    fn auth_or_notfound_is_not_ready() {
        // The OLD bug: accepting a 401 (the auth middleware's pre-dependency
        // reject) as "ready". /readyz is unauthenticated, so a 401 now means
        // the wrong route answered — NOT a pass. 404 likewise.
        assert_eq!(classify_readiness(401), Readiness::NotReady);
        assert_eq!(classify_readiness(404), Readiness::NotReady);
    }

    // ── gate behavior simulated over a poll sequence ────────────────
    //
    // These exercise the SAME pass/fail decision the real loops use
    // (`Ready` => commit/continue; a sustained non-Ready streak => the
    // gate never passes / the monitor rolls back), without shelling to
    // docker. We model the `wait_ready` loop: it returns Ok only when a
    // poll classifies Ready, and otherwise keeps polling until timeout
    // (= gate failure / no swap).

    /// Faithful re-implementation of the wait_ready PASS condition over a
    /// finite sequence of probe statuses. `true` = gate passed (swap
    /// allowed); `false` = exhausted the sequence without ever becoming
    /// ready (gate fails → swap NOT committed → old color keeps serving).
    fn gate_passes(statuses: &[u16]) -> bool {
        statuses
            .iter()
            .any(|&s| classify_readiness(s) == Readiness::Ready)
    }

    #[test]
    fn liveness_up_but_readiness_fails_does_not_pass_gate() {
        // The DEP-A scenario: container is live (would be "healthy" to the
        // old liveness gate) but DB/Redis is broken, so /readyz 503s every
        // poll. The gate never sees a 200 → never passes → the swap is NOT
        // committed and auto-give-up fires.
        assert!(!gate_passes(&[502, 503, 503, 503, 500]));
        // Connection-refused throughout (hard-down) → also no pass.
        assert!(!gate_passes(&[0, 0, 0, 0]));
    }

    #[test]
    fn readiness_passes_commits_swap() {
        // A healthy color: slow to warm up (boot 5xx / refused / DB still
        // connecting → 503 for a while) but eventually 200s → gate passes →
        // swap commits. Proves a slow-but-healthy boot is NOT failed.
        assert!(gate_passes(&[0, 0, 502, 503, 200]));
        assert!(gate_passes(&[200]));
    }

    /// Faithful model of the grace-monitor's readiness rollback decision:
    /// a streak of >=3 consecutive non-Ready polls rolls back; a Ready
    /// poll resets the streak. Returns `true` if it would roll back.
    fn grace_rolls_back(statuses: &[u16]) -> bool {
        let mut streak = 0;
        for &s in statuses {
            match classify_readiness(s) {
                Readiness::Ready => streak = 0,
                _ => {
                    streak += 1;
                    if streak >= 3 {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[test]
    fn grace_monitor_rolls_back_on_sustained_not_ready() {
        // Promoted color goes bad mid-grace (e.g. DB pool drops → /readyz
        // 503s) → 3 in a row → rollback.
        assert!(grace_rolls_back(&[200, 200, 503, 503, 503]));
    }

    #[test]
    fn grace_monitor_tolerates_a_transient_blip() {
        // A single/double blip that recovers (back to 200) does NOT roll back
        // — avoids false rollbacks on a momentary restart/GC pause.
        assert!(!grace_rolls_back(&[200, 0, 200, 502, 200]));
        assert!(!grace_rolls_back(&[200, 503, 503, 200, 503]));
    }
}
