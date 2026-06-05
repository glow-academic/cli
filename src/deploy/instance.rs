//! Single-instance deploy state on disk.
//!
//! Every deployment owns one directory under `~/.glow/instances/<name>/`:
//!
//! ```text
//! ~/.glow/instances/<name>/
//!   glow-deploy.yaml     # user-edited source of truth
//!   .env                 # generated, mode 0600
//!   docker-compose.yml   # rendered from embedded template
//!   .deploy-state.json   # bookkeeping (active env, version, timestamps)
//!   backups/*.sql.gz     # local pg dumps
//! ```
//!
//! `~/.config/glow/active-instance` records the currently selected name so
//! every subcommand can default to it. v1.0.0 supports exactly one instance
//! at a time per user (per the design call) — multi-instance switching is
//! deferred.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Default instance name when the user doesn't pass `--name`.
pub const DEFAULT_NAME: &str = "default";

/// Marker file that holds the active-instance name. Lives under the user's
/// config dir, not the instance dir, so `glow status` / `glow stop` work
/// without `--name`.
fn active_instance_marker() -> Result<PathBuf> {
    let cfg = dirs::config_dir().ok_or_else(|| anyhow!("no config dir"))?;
    Ok(cfg.join("glow").join("active-instance"))
}

/// Root of all instance directories: `~/.glow/instances/`.
fn instances_root() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("no home dir"))?;
    Ok(home.join(".glow").join("instances"))
}

/// Resolved instance handle — pure path math + bookkeeping. Doesn't read
/// or write docker / compose state; that belongs to higher-level modules.
#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub dir: PathBuf,
}

impl Instance {
    /// Materialize the instance dir if missing. Idempotent.
    pub fn ensure(name: &str) -> Result<Self> {
        let dir = instances_root()?.join(name);
        fs::create_dir_all(dir.join("backups"))
            .with_context(|| format!("create instance dir: {}", dir.display()))?;
        Ok(Self {
            name: name.to_string(),
            dir,
        })
    }

    /// Open an existing instance (errors if missing — used by stop/destroy
    /// where we don't want to silently materialize a fresh dir).
    pub fn open(name: &str) -> Result<Self> {
        let dir = instances_root()?.join(name);
        if !dir.is_dir() {
            return Err(anyhow!(
                "no instance named `{name}` at {} — run `glow deploy` first",
                dir.display()
            ));
        }
        Ok(Self {
            name: name.to_string(),
            dir,
        })
    }

    /// Mark this as the active instance.
    pub fn mark_active(&self) -> Result<()> {
        let marker = active_instance_marker()?;
        if let Some(parent) = marker.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(&marker, &self.name)
            .with_context(|| format!("write active-instance marker: {}", marker.display()))?;
        Ok(())
    }

    // ── Conventional paths ──────────────────────────────────────────

    pub fn deploy_yaml(&self) -> PathBuf {
        self.dir.join("glow-deploy.yaml")
    }

    pub fn state_file(&self) -> PathBuf {
        self.dir.join(".deploy-state.json")
    }
    pub fn backups_dir(&self) -> PathBuf {
        self.dir.join("backups")
    }

    // ── Two-stack layout ────────────────────────────────────────────
    // Each stack gets its own subdir + .env so compose state is clean
    // and the two stacks can be touched independently.

    pub fn api_dir(&self) -> PathBuf {
        self.dir.join("api")
    }
    pub fn client_dir(&self) -> PathBuf {
        self.dir.join("client")
    }
    pub fn api_env_file(&self) -> PathBuf {
        self.api_dir().join(".env")
    }
    pub fn client_env_file(&self) -> PathBuf {
        self.client_dir().join(".env")
    }

    /// Compose project name — used as container prefix. We derive from the
    /// instance name so two instances on the same host don't collide.
    pub fn project_name(&self) -> String {
        // Compose requires lowercase + restricted charset; sanitize.
        let mut s = String::with_capacity(self.name.len() + 5);
        s.push_str("glow-");
        for c in self.name.chars() {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                s.push(c.to_ascii_lowercase());
            } else {
                s.push('-');
            }
        }
        s
    }

    /// Compose project name for the api stack — suffixed so the api and
    /// client stacks don't collide on container/network names.
    pub fn api_project_name(&self) -> String {
        format!("{}-api", self.project_name())
    }

    /// Compose project name for the client stack.
    pub fn client_project_name(&self) -> String {
        format!("{}-client", self.project_name())
    }

    /// Shared docker network the api + client stacks join so client
    /// nginx can resolve api nginx by container name (`<project>-nginx`).
    pub fn shared_network(&self) -> String {
        format!("{}-net", self.project_name())
    }
}

// ── Deploy-state bookkeeping ────────────────────────────────────────

/// Persistent bookkeeping written by deploy/redeploy. Read by status +
/// the blue/green orchestrator to know which color is live without
/// shelling out to docker every time.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeployState {
    /// Current active color for the api server pair: "blue" | "green".
    /// Updated after a successful traffic switch.
    pub active_env: Option<String>,
    /// Same idea but for the Keycloak blue/green pair.
    pub active_kc_env: Option<String>,
    /// Current active color for the client app pair: "blue" | "green".
    /// `None` in api_only topology where there's no client stack.
    #[serde(default)]
    pub active_client_env: Option<String>,
    /// Pinned api image version (e.g. "v1.0.0").
    pub api_version: Option<String>,
    /// Pinned client image version. `None` in api_only topology.
    #[serde(default)]
    pub client_version: Option<String>,
    /// Seed setup used on the *initial* deploy. Future redeploys preserve
    /// the DB unless the user explicitly opts back into reseeding.
    pub initial_seed_setup: Option<String>,
    /// The instance's EFFECTIVE seed setup — what the live DB was actually
    /// seeded with (e.g. `university`). Set on first deploy and updated on
    /// any `--reseed <setup>`. A plain redeploy renders `SEED_SETUP` from
    /// this so the api's `database.init` guard (#240) sees a *matching*
    /// setup → idempotent skip → data preserved, no `--reseed` needed.
    ///
    /// Pre-existing state files predate this field and carry only
    /// `initial_seed_setup`; `effective_seed_setup()` falls back to that.
    #[serde(default)]
    pub seed_setup: Option<String>,
    /// ISO-8601 timestamps for the audit trail.
    pub first_deployed_at: Option<String>,
    pub last_deployed_at: Option<String>,
}

impl DeployState {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let body = fs::read_to_string(path)
            .with_context(|| format!("read deploy state: {}", path.display()))?;
        Ok(serde_json::from_str(&body).unwrap_or_default())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let body = serde_json::to_string_pretty(self)?;
        fs::write(path, body).with_context(|| format!("write deploy state: {}", path.display()))?;
        Ok(())
    }

    /// Swap "blue" <-> "green" — used by the bluegreen module to compute
    /// the next deploy color for the api pair.
    pub fn next_env(&self) -> &'static str {
        match self.active_env.as_deref() {
            Some("blue") => "green",
            _ => "blue",
        }
    }

    /// Same idea for the client app pair.
    pub fn next_client_env(&self) -> &'static str {
        match self.active_client_env.as_deref() {
            Some("blue") => "green",
            _ => "blue",
        }
    }

    /// The instance's effective seed setup, with migration fallback.
    ///
    /// Newer state files carry `seed_setup`; older ones (written before the
    /// field existed) carry only `initial_seed_setup`. Prefer the new field,
    /// fall back to the legacy one — so a plain redeploy of a pre-existing
    /// instance still recovers the right setup.
    pub fn effective_seed_setup(&self) -> Option<String> {
        self.seed_setup
            .clone()
            .or_else(|| self.initial_seed_setup.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_seed_setup_prefers_new_field() {
        let s = DeployState {
            seed_setup: Some("university".into()),
            initial_seed_setup: Some("fresh".into()),
            ..Default::default()
        };
        assert_eq!(s.effective_seed_setup(), Some("university".into()));
    }

    #[test]
    fn effective_seed_setup_falls_back_to_initial() {
        // Pre-existing instance: only the legacy field is populated.
        let s = DeployState {
            seed_setup: None,
            initial_seed_setup: Some("university".into()),
            ..Default::default()
        };
        assert_eq!(s.effective_seed_setup(), Some("university".into()));
    }

    #[test]
    fn legacy_state_json_deserializes_with_default_seed_setup() {
        // A state file written before `seed_setup` existed must still parse
        // (the field is `#[serde(default)]`) and recover via the fallback.
        let legacy = r#"{
            "active_env": "blue",
            "api_version": "v1.0.0",
            "initial_seed_setup": "university",
            "first_deployed_at": "2026-01-01T00:00:00Z",
            "last_deployed_at": "2026-01-01T00:00:00Z"
        }"#;
        let s: DeployState = serde_json::from_str(legacy).unwrap();
        assert_eq!(s.seed_setup, None);
        assert_eq!(s.effective_seed_setup(), Some("university".into()));
    }
}
