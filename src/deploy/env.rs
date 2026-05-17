//! `.env` rendering — first-deploy clobber vs redeploy merge.
//!
//! The original learnloopllc-glow-api ops learned a hard lesson here
//! (`feedback_deploy_workflows.md`): clobbering `.env` on redeploy loses
//! `SECRET_KEY` / `DEPLOYMENT_TOKEN` / other first-deploy-only secrets,
//! breaking Keycloak the next time auth tokens need verifying. We
//! replicate the same write-once-then-merge pattern here.
//!
//! Keys we OWN (overwritten on every redeploy):
//!   - API_VERSION, ORIGIN, CLIENT_ORIGINS, ACTIVE_ENV, ACTIVE_KC_ENV
//!   - COMPOSE_PROJECT_NAME (derived from instance name)
//!   - DB_BACKUP, SEED_SETUP
//!   - GRACE_PERIOD_MINUTES, APP_PREFIX
//!
//! Keys we PRESERVE (written once on first deploy):
//!   - SECRET_KEY, DEPLOYMENT_TOKEN
//!   - DB_PASSWORD, KEYCLOAK_ADMIN_PASSWORD
//!   - AUTH_CLIENT_SECRET (derived from SECRET_KEY)
//!   - Any user-added vars
//!
//! Client `.env` rendering follows the same pattern but with a
//! different set of owned keys (see `render_client`).

use anyhow::{Context, Result};
use base64::Engine;
use pbkdf2::pbkdf2_hmac_array;
use rand::distributions::{Alphanumeric, DistString};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Keys the CLI mutates on every redeploy of the api stack.
/// Everything else in the api .env is left alone (preserves
/// first-deploy secrets).
const OWNED_KEYS: &[&str] = &[
    "API_VERSION",
    "ORIGIN",
    "CLIENT_ORIGINS",
    "ACTIVE_ENV",
    "ACTIVE_KC_ENV",
    "COMPOSE_PROJECT_NAME",
    "DB_BACKUP",
    "SEED_SETUP",
    "GRACE_PERIOD_MINUTES",
    "APP_PREFIX",
    "GLOW_NETWORK",
];

/// Keys the CLI mutates on every redeploy of the client stack.
/// The client stack has no persistent secrets of its own — it
/// reuses the api's SECRET_KEY / AUTH_CLIENT_SECRET via these env
/// vars — so every key in the client .env is owned.
const CLIENT_OWNED_KEYS: &[&str] = &[
    "CLIENT_VERSION",
    "ACTIVE_CLIENT_ENV",
    "COMPOSE_PROJECT_NAME",
    "DOMAIN",
    "NEXT_PUBLIC_API_URL",
    "INTERNAL_API_BASE",
    "NEXT_PUBLIC_API_BASE",
    "AUTH_SECRET",
    "AUTH_KEYCLOAK_ID",
    "AUTH_KEYCLOAK_SECRET",
    "KEYCLOAK_PUBLIC_URL",
    "NEXTAUTH_URL",
    "AUTH_TRUST_HOST",
    "MCP_BACKEND",
    "GLOW_NETWORK",
    "CLIENT_HTTP_PORT",
];

/// Inputs the CLI computes from the deploy config + state for the api
/// stack. Caller fills these in; we serialize and merge with existing
/// .env on disk.
pub struct EnvInputs {
    pub api_version: String,
    pub origin: String,
    /// CORS allowlist — comma-separated list of client origins the api
    /// will accept OIDC redirects from. Empty in api_only mode.
    pub client_origins: String,
    pub active_env: String,
    pub active_kc_env: String,
    pub project_name: String,
    pub seed_setup: Option<String>,
    pub db_backup: Option<String>,
    pub grace_period_minutes: u32,
    pub app_prefix: String,
    /// Name of the shared docker network the api + client stacks join.
    pub glow_network: String,
}

/// Inputs for the client stack. Everything is owned (re-derived per
/// redeploy from the api's .env + the topology); the client has no
/// persistent secrets of its own.
pub struct ClientEnvInputs {
    pub client_version: String,
    pub active_client_env: String,
    pub project_name: String,
    /// Name of the shared docker network the api stack joins. Used as
    /// `${GLOW_NETWORK}` in the client compose's external-network
    /// declaration.
    pub glow_network: String,
    /// Public port the client nginx listens on (typically 80 for a
    /// fronting reverse proxy to forward to).
    pub client_http_port: u16,
    /// The client's public domain (no scheme), used by nginx server_name.
    pub domain: String,
    /// Public URL the browser uses to call the api. Per topology:
    ///   airgapped  → client_origin (api proxied via client nginx)
    ///   exposed    → api_origin (browser hits api directly)
    ///   api_only   → unused
    pub public_api_url: String,
    /// Server-side URL the client's Next.js server uses to call the api
    /// from inside the docker network. Usually the api's internal nginx
    /// alias (e.g. `http://glow-api-nginx:80`).
    pub internal_api_base: String,
    /// AUTH_SECRET — must match the api's SECRET_KEY (NextAuth uses it
    /// to verify JWTs minted by the api).
    pub auth_secret: String,
    /// Keycloak OIDC client id. Defaults to `glow-client`.
    pub auth_keycloak_id: String,
    /// Keycloak OIDC client secret — same value as the api's
    /// AUTH_CLIENT_SECRET (PBKDF2-derived from SECRET_KEY).
    pub auth_keycloak_secret: String,
    /// Public Keycloak URL. Per topology:
    ///   airgapped → ${client_origin}/auth   (proxied by client nginx)
    ///   exposed   → ${api_origin}/auth      (Keycloak is on api domain)
    pub keycloak_public_url: String,
    /// NEXTAUTH_URL — public base URL of the client itself, used for
    /// constructing OAuth callback URLs.
    pub nextauth_url: String,
    /// Docker network address of the api's nginx, used by the client
    /// nginx to proxy /auth/, /api/, /mcp/ etc. when airgapped.
    pub mcp_backend: String,
}

/// Render or merge `.env`. If `path` doesn't exist, this is treated as a
/// first deploy: we generate all the persistent secrets (SECRET_KEY,
/// DB_PASSWORD, etc.) and clobber. If it exists, we keep secrets intact
/// and only update the keys in OWNED_KEYS.
pub fn render(path: &Path, inputs: &EnvInputs) -> Result<()> {
    let mut env = if path.exists() {
        parse(
            &fs::read_to_string(path)
                .with_context(|| format!("read existing .env: {}", path.display()))?,
        )
    } else {
        // First deploy — generate the persistent secrets.
        let mut seed = BTreeMap::new();
        let secret_key = random_hex(32);
        seed.insert("SECRET_KEY".into(), secret_key.clone());
        seed.insert("DEPLOYMENT_TOKEN".into(), random_alnum(48));
        seed.insert("DB_USER".into(), "glow".into());
        seed.insert("DB_PASSWORD".into(), random_alnum(32));
        seed.insert("DB_NAME".into(), "glowapi".into());
        seed.insert("KEYCLOAK_ADMIN".into(), "admin".into());
        seed.insert("KEYCLOAK_ADMIN_PASSWORD".into(), random_alnum(32));
        // Derive AUTH_CLIENT_SECRET via the same PBKDF2 scheme deploy.yml
        // uses (`feedback_destroy_and_logs_bugs.md` shows the salt).
        seed.insert(
            "AUTH_CLIENT_SECRET".into(),
            derive_auth_client_secret(&secret_key),
        );
        seed
    };

    // Apply owned-key overrides.
    upsert(&mut env, "API_VERSION", &inputs.api_version);
    upsert(&mut env, "ORIGIN", &inputs.origin);
    upsert(&mut env, "CLIENT_ORIGINS", &inputs.client_origins);
    upsert(&mut env, "ACTIVE_ENV", &inputs.active_env);
    upsert(&mut env, "ACTIVE_KC_ENV", &inputs.active_kc_env);
    upsert(&mut env, "COMPOSE_PROJECT_NAME", &inputs.project_name);
    upsert(&mut env, "APP_PREFIX", &inputs.app_prefix);
    upsert(&mut env, "GLOW_NETWORK", &inputs.glow_network);
    upsert(
        &mut env,
        "GRACE_PERIOD_MINUTES",
        &inputs.grace_period_minutes.to_string(),
    );
    if let Some(s) = &inputs.seed_setup {
        upsert(&mut env, "SEED_SETUP", s);
    } else {
        env.remove("SEED_SETUP");
    }
    if let Some(b) = &inputs.db_backup {
        upsert(&mut env, "DB_BACKUP", b);
    } else {
        // Don't carry stale DB_BACKUP between redeploys; that would re-trigger
        // the restore entrypoint every time.
        env.remove("DB_BACKUP");
    }

    write(path, &env)
}

/// Render the client stack's `.env`. All keys are owned (no
/// persistent secrets here — the client reuses the api's), so a
/// redeploy clobbers them all and leaves any user-added vars alone.
pub fn render_client(path: &Path, inputs: &ClientEnvInputs) -> Result<()> {
    let mut env = if path.exists() {
        parse(
            &fs::read_to_string(path)
                .with_context(|| format!("read existing client .env: {}", path.display()))?,
        )
    } else {
        BTreeMap::new()
    };

    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "CLIENT_VERSION",
        &inputs.client_version,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "ACTIVE_CLIENT_ENV",
        &inputs.active_client_env,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "COMPOSE_PROJECT_NAME",
        &inputs.project_name,
    );
    upsert_in(&mut env, CLIENT_OWNED_KEYS, "DOMAIN", &inputs.domain);
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "NEXT_PUBLIC_API_URL",
        &inputs.public_api_url,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "INTERNAL_API_BASE",
        &inputs.internal_api_base,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "NEXT_PUBLIC_API_BASE",
        &inputs.public_api_url,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "AUTH_SECRET",
        &inputs.auth_secret,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "AUTH_KEYCLOAK_ID",
        &inputs.auth_keycloak_id,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "AUTH_KEYCLOAK_SECRET",
        &inputs.auth_keycloak_secret,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "KEYCLOAK_PUBLIC_URL",
        &inputs.keycloak_public_url,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "NEXTAUTH_URL",
        &inputs.nextauth_url,
    );
    upsert_in(&mut env, CLIENT_OWNED_KEYS, "AUTH_TRUST_HOST", "true");
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "MCP_BACKEND",
        &inputs.mcp_backend,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "GLOW_NETWORK",
        &inputs.glow_network,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "CLIENT_HTTP_PORT",
        &inputs.client_http_port.to_string(),
    );

    write(path, &env)
}

/// Read SECRET_KEY + AUTH_CLIENT_SECRET out of an api `.env`. The
/// client stack reuses both — AUTH_SECRET = SECRET_KEY (so NextAuth
/// can verify api-minted JWTs) and AUTH_KEYCLOAK_SECRET =
/// AUTH_CLIENT_SECRET (so the OIDC handshake succeeds).
pub fn read_api_shared_secrets(path: &Path) -> Result<(String, String)> {
    let env = parse(
        &fs::read_to_string(path)
            .with_context(|| format!("read api .env for shared secrets: {}", path.display()))?,
    );
    let secret = env
        .get("SECRET_KEY")
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("SECRET_KEY missing from api .env"))?;
    let kc = env
        .get("AUTH_CLIENT_SECRET")
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("AUTH_CLIENT_SECRET missing from api .env"))?;
    Ok((secret, kc))
}

// ── internals ──────────────────────────────────────────────────────

fn upsert(env: &mut BTreeMap<String, String>, key: &str, value: &str) {
    upsert_in(env, OWNED_KEYS, key, value);
}

fn upsert_in(env: &mut BTreeMap<String, String>, owned: &[&str], key: &str, value: &str) {
    debug_assert!(
        owned.contains(&key),
        "upsert should only touch the owned-key list — `{key}` isn't in it"
    );
    env.insert(key.into(), value.into());
}

fn parse(body: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            out.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    out
}

fn write(path: &Path, env: &BTreeMap<String, String>) -> Result<()> {
    let body: String = env.iter().map(|(k, v)| format!("{k}={v}\n")).collect();
    fs::write(path, body).with_context(|| format!("write .env: {}", path.display()))?;
    // Tighten perms to 0600 — these are secrets.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perm = fs::metadata(path)?.permissions();
        perm.set_mode(0o600);
        fs::set_permissions(path, perm).ok();
    }
    Ok(())
}

fn random_alnum(n: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), n)
}

fn random_hex(n_bytes: usize) -> String {
    use rand::RngCore;
    let mut buf = vec![0u8; n_bytes];
    rand::thread_rng().fill_bytes(&mut buf);
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

// ── topology-driven env derivation ─────────────────────────────────

/// Derive the api stack's public `ORIGIN` and `CLIENT_ORIGINS`
/// from the topology + config. This is the heart of the 3-mode
/// decision table:
///
/// | mode      | ORIGIN          | CLIENT_ORIGINS  |
/// |-----------|-----------------|-----------------|
/// | airgapped | client_origin   | client_origin   |
/// | exposed   | api_origin      | client_origin   |
/// | api_only  | api_origin      | (empty)         |
///
/// Airgapped sets the api's ORIGIN to the *client* domain because
/// OIDC callbacks resolve through the client nginx — Keycloak issuer
/// URLs need to match the URL the browser actually sees.
pub fn derive_api_origins(cfg: &crate::deploy::config::DeployConfig) -> (String, String) {
    use crate::deploy::config::Topology;
    let api = cfg.effective_api_origin();
    let client = cfg.effective_client_origin();
    match cfg.topology {
        Topology::Airgapped => (client.clone(), client),
        Topology::Exposed => (api, client),
        Topology::ApiOnly => (api, String::new()),
    }
}

/// Derive the client stack's `NEXT_PUBLIC_API_URL` and
/// `KEYCLOAK_PUBLIC_URL` from the topology. In airgapped mode the
/// browser only ever talks to the client domain (api is proxied);
/// in exposed mode the browser hits the api domain directly.
pub fn derive_client_public_urls(cfg: &crate::deploy::config::DeployConfig) -> (String, String) {
    use crate::deploy::config::Topology;
    let api = cfg.effective_api_origin();
    let client = cfg.effective_client_origin();
    match cfg.topology {
        Topology::Airgapped => {
            // Browser → client nginx for everything, including /auth/*.
            (client.clone(), format!("{client}/auth"))
        }
        Topology::Exposed => {
            // Browser → api directly; Keycloak is on the api domain.
            (api.clone(), format!("{api}/auth"))
        }
        Topology::ApiOnly => (String::new(), String::new()),
    }
}

/// PBKDF2-HMAC-SHA256, 100k iterations, salt "glow-auth-secret-v1",
/// base64-encoded. Matches the derivation deploy.yml uses (so a CLI
/// deploy ends up with the same AUTH_CLIENT_SECRET shape as the
/// LearnLoop control-plane deploys produced).
fn derive_auth_client_secret(secret_key: &str) -> String {
    let derived: [u8; 32] =
        pbkdf2_hmac_array::<Sha256, 32>(secret_key.as_bytes(), b"glow-auth-secret-v1", 100_000);
    base64::engine::general_purpose::STANDARD.encode(derived)
}
