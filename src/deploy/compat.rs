//! Cross-repo compatibility check.
//!
//! Every api release ships a `compat.json` asset of the shape:
//!
//! ```json
//! {
//!   "component": "api",
//!   "version": "1.0.0",
//!   "requires": {
//!     "cli":    { "min_version": "1.0.0" },
//!     "client": { "min_version": "1.0.0" },
//!     "docs":   { "min_version": "1.0.0" }
//!   }
//! }
//! ```
//!
//! Before a `glow deploy --api-version vX.Y.Z` runs, this module fetches
//! the matching tag's compat.json and refuses if the running CLI is
//! older than the api's `requires.cli.min_version`. That way a stale
//! `glow` binary can't accidentally render an out-of-date compose
//! template against a much newer api image and silently misbehave.
//!
//! The fetch is HTTP-only (no GH token required for public releases);
//! private releases need a `GITHUB_TOKEN` env var the user provides.
//! On network failure we DON'T block the deploy — we warn and proceed,
//! because being unable to reach github at deploy time is a worse UX
//! than missing a compat check.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::time::Duration;

/// CLI's own version, embedded at build time from Cargo.toml.
const SELF_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Owner/repo of the api repo. Hardcoded for v1.0.0 — when we
/// eventually let users override the source (forks, private registries),
/// thread this through `DeployArgs`.
const API_REPO: &str = "glow-academic/api";
const CLIENT_REPO: &str = "glow-academic/client";

#[derive(Debug, Deserialize)]
struct Compat {
    #[allow(dead_code)]
    component: String,
    version: String,
    #[serde(default)]
    requires: BTreeMap<String, Requirement>,
}

#[derive(Debug, Deserialize)]
struct Requirement {
    min_version: String,
}

/// Verify the running CLI can deploy `api_version`. Returns Ok on
/// compatibility OR when the manifest can't be fetched (warn + proceed,
/// per docstring rationale). Returns Err when we definitively know
/// the combo is incompatible.
pub fn check_api(api_version: &str) -> Result<()> {
    match fetch(API_REPO, api_version) {
        Ok(compat) => verify(&compat, "cli", "api", "cli"),
        Err(e) => {
            eprintln!(
                "  ⚠ couldn't fetch compat manifest for api {api_version} ({e}). \
                 Proceeding without compatibility check — set GITHUB_TOKEN if the \
                 release is private."
            );
            Ok(())
        }
    }
}

/// Verify CLI ↔ client AND client ↔ api compatibility. The client's
/// compat.json declares its required cli floor + api floor; we check
/// both. Like `check_api`, network failure is a warn-and-proceed.
pub fn check_client(client_version: &str, api_version: &str) -> Result<()> {
    let compat = match fetch(CLIENT_REPO, client_version) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "  ⚠ couldn't fetch compat manifest for client {client_version} ({e}). \
                 Proceeding without client compatibility check."
            );
            return Ok(());
        }
    };
    // CLI version floor.
    verify(&compat, "cli", "client", "cli")?;
    // Api version floor — refuse if the chosen api is older than the
    // client's published min.
    if let Some(req) = compat.requires.get("api") {
        if version_lt(api_version, &req.min_version) {
            return Err(anyhow!(
                "client {} requires api >= {} but you're deploying api {}. \
                 Bump --api-version or pin an older client.",
                compat.version,
                req.min_version,
                api_version,
            ));
        }
    }
    Ok(())
}

/// Generic verify against a single requires-key. The 3 string params
/// are (component-name-being-checked, manifest-component, name-in-error).
fn verify(compat: &Compat, key: &str, manifest: &str, label: &str) -> Result<()> {
    let Some(req) = compat.requires.get(key) else {
        // No floor declared — nothing to check.
        return Ok(());
    };
    if version_lt(SELF_VERSION, &req.min_version) {
        return Err(anyhow!(
            "this CLI ({}) is older than the minimum required by {} {} \
             (needs {} >= {}). Upgrade with `brew upgrade glow` or grab a \
             newer release from github.com/glow-academic/cli/releases.",
            SELF_VERSION,
            manifest,
            compat.version,
            label,
            req.min_version,
        ));
    }
    Ok(())
}

fn fetch(repo: &str, version: &str) -> Result<Compat> {
    // Normalize: accept "v1.0.0" or "1.0.0" — GitHub uses the leading-v.
    let tag = if version.starts_with('v') {
        version.to_string()
    } else {
        format!("v{version}")
    };
    let url = format!(
        "https://github.com/{repo}/releases/download/{tag}/compat.json"
    );

    let mut req = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(8))
        .user_agent(format!("glow-cli/{SELF_VERSION}"))
        .build()?
        .get(&url);
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        req = req.bearer_auth(token);
    }
    let resp = req.send().with_context(|| format!("GET {url}"))?;
    if !resp.status().is_success() {
        return Err(anyhow!("HTTP {} from {url}", resp.status().as_u16()));
    }
    let compat: Compat = resp.json().context("parse compat.json")?;
    Ok(compat)
}

/// Compares two dotted semver-ish strings. Strips a leading `v` and any
/// `-prerelease` suffix. We don't need full semver semantics for a
/// min-version floor check — just numeric major.minor.patch order.
fn version_lt(a: &str, b: &str) -> bool {
    fn parts(v: &str) -> [u32; 3] {
        let core = v.trim_start_matches('v').split('-').next().unwrap_or("0");
        let mut iter = core.split('.').map(|s| s.parse::<u32>().unwrap_or(0));
        [
            iter.next().unwrap_or(0),
            iter.next().unwrap_or(0),
            iter.next().unwrap_or(0),
        ]
    }
    parts(a) < parts(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_lt_works() {
        assert!(version_lt("1.0.0", "1.0.1"));
        assert!(version_lt("1.0.0", "2.0.0"));
        assert!(!version_lt("1.0.1", "1.0.0"));
        assert!(!version_lt("1.0.0", "1.0.0"));
        // Leading v + prerelease tolerated.
        assert!(version_lt("v1.0.0-beta", "v1.0.0"));
        assert!(!version_lt("v2.0.0", "v1.9.9"));
    }
}
