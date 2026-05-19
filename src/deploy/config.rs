//! `glow-deploy.yaml` schema + loader.
//!
//! This is the *source-of-truth* config the user owns. Everything else
//! (`.env`, compose project, blue/green state) is derived from it.
//!
//! We intentionally accept a permissive shape — only the fields we read
//! are typed; arbitrary extra keys ride along untouched so the user can
//! evolve their yaml without breaking the CLI on every change.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Top-level glow-deploy.yaml shape. Fields are all optional at parse
/// time so an in-progress wizard can write partial configs without the
/// load failing; the deploy step does its own readiness checks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeployConfig {
    /// Seed template: "fresh" | "university" | "organization".
    /// Drives initial DB content on first deploy. Ignored on redeploy
    /// unless the user explicitly opts back in.
    #[serde(default)]
    pub setup: Option<String>,

    /// Deployment topology — picks which stacks to bring up and how
    /// `ORIGIN` / `CLIENT_ORIGINS` / `AUTH_ISSUER` are wired.
    /// Defaults to `airgapped` (api + client on one domain).
    #[serde(default)]
    pub topology: Topology,

    /// Public api domain. Required for `exposed` and `api_only`.
    /// For `airgapped`, this is unused — api is internal-only and
    /// `client_origin` is the single public domain.
    #[serde(default)]
    pub api_origin: Option<String>,

    /// Public client domain. Required for `airgapped` and `exposed`.
    /// Unused in `api_only`.
    #[serde(default)]
    pub client_origin: Option<String>,

    /// Pinned api image tag (e.g. `v1.0.0`).
    #[serde(default)]
    pub api_version: Option<String>,

    /// Pinned client image tag. Required unless topology is `api_only`.
    #[serde(default)]
    pub client_version: Option<String>,

    /// Host port (or `<ip>:<port>` spec) the client nginx publishes on.
    /// Default when unset: `127.0.0.1:18080` — localhost-bound, high
    /// port, safe to coexist with anything else on the host. Override
    /// examples:
    ///   "18080"               → 0.0.0.0:18080 (reachable from other hosts)
    ///   "0.0.0.0:18080"       → same, explicit
    ///   "10.0.0.5:18080"      → bind to a specific interface
    ///   "80"                  → take the privileged port (needs root)
    /// The right value usually matches what your reverse proxy expects.
    /// Unused in `api_only` topology.
    #[serde(default)]
    pub client_http_port: Option<String>,

    /// Legacy single-domain field. Read-only fallback for old configs:
    /// if set and `client_origin`/`api_origin` are not, it maps to
    /// whichever public domain the topology expects.
    #[serde(default)]
    pub origin: Option<String>,

    /// AI provider catalog. Mirrors the deploy-yaml shape we ported to
    /// the api repo's templates.
    #[serde(default)]
    pub ai: AiConfig,

    /// Auth provider catalog (OIDC clients to register with Keycloak).
    #[serde(default)]
    pub auth: AuthConfig,

    /// Catch-all so unknown keys round-trip.
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

/// Deployment topology — three supported shapes mirroring the
/// LearnLoop control plane's instance modes. The mode is purely a
/// provisioning + env-var + routing choice; the api and client code
/// themselves are mode-agnostic.
///
/// - `Airgapped`: api + client on one public domain. Api reachable
///   only via the client nginx. Single TLS cert.
/// - `Exposed`: api + client on two public domains. External callers
///   (mobile apps, scripts) can hit the api directly.
/// - `ApiOnly`: api on its own public domain, no client. For users
///   building their own UI against the api.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Topology {
    #[default]
    Airgapped,
    Exposed,
    ApiOnly,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiConfig {
    #[serde(default)]
    pub providers: Vec<AiProvider>,
    /// Role → model name. Keys are the canonical glow-api role names
    /// (`text`, `grader`, `image`, `video`, `audio`, `transcribe`,
    /// `realtime`) per the rename we shipped in learnloopllc-api 2.24.9.
    #[serde(default)]
    pub roles: BTreeMap<String, String>,
    #[serde(default)]
    pub models: Vec<serde_yaml::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiProvider {
    pub name: String,
    #[serde(default)]
    pub endpoint: Option<String>,
    /// Set when the provider needs a key. CLI stores via `glow init`
    /// password prompt; never echoed in logs.
    #[serde(default)]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(default)]
    pub providers: Vec<serde_yaml::Value>,
}

impl DeployConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let body = fs::read_to_string(path)
            .with_context(|| format!("read deploy config: {}", path.display()))?;
        let cfg: Self = serde_yaml::from_str(&body)
            .with_context(|| format!("parse deploy config: {}", path.display()))?;
        Ok(cfg)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let body = serde_yaml::to_string(self)?;
        fs::write(path, body)
            .with_context(|| format!("write deploy config: {}", path.display()))?;
        Ok(())
    }

    /// Pre-deploy validation — surfaces missing fields with a friendly
    /// message before we touch docker. Add to this as schema evolves.
    pub fn validate(&self) -> Result<()> {
        // Topology-driven origin requirements.
        let api_origin = self.effective_api_origin();
        let client_origin = self.effective_client_origin();
        match self.topology {
            Topology::Airgapped => {
                if client_origin.is_empty() {
                    return Err(anyhow!(
                        "`client_origin` is required for airgapped topology \
                         (set it to the single public domain you want, e.g. \
                         https://glow.example.com)"
                    ));
                }
            }
            Topology::Exposed => {
                if api_origin.is_empty() || client_origin.is_empty() {
                    return Err(anyhow!(
                        "`api_origin` and `client_origin` are both required \
                         for exposed topology (two public domains)"
                    ));
                }
            }
            Topology::ApiOnly => {
                if api_origin.is_empty() {
                    return Err(anyhow!("`api_origin` is required for api_only topology"));
                }
            }
        }
        if self.ai.providers.is_empty() {
            return Err(anyhow!(
                "at least one entry in `ai.providers` is required \
                 (e.g. openai with an api key)"
            ));
        }
        if self.ai.roles.is_empty() {
            return Err(anyhow!(
                "at least one `ai.roles.<role>` mapping is required \
                 (minimum: `text: <model-name>`)"
            ));
        }
        Ok(())
    }

    /// Resolve api origin honoring the legacy `origin` field as a
    /// fallback for old configs. For airgapped this returns "" since
    /// api isn't publicly addressed.
    pub fn effective_api_origin(&self) -> String {
        if let Some(o) = self.api_origin.as_deref().filter(|s| !s.is_empty()) {
            return o.into();
        }
        match self.topology {
            Topology::Airgapped => String::new(),
            Topology::Exposed | Topology::ApiOnly => self.origin.clone().unwrap_or_default(),
        }
    }

    /// Resolve client origin honoring the legacy `origin` field for
    /// airgapped configs. In api_only this is always empty.
    pub fn effective_client_origin(&self) -> String {
        if let Some(o) = self.client_origin.as_deref().filter(|s| !s.is_empty()) {
            return o.into();
        }
        match self.topology {
            Topology::Airgapped => self.origin.clone().unwrap_or_default(),
            Topology::Exposed => String::new(),
            Topology::ApiOnly => String::new(),
        }
    }
}
