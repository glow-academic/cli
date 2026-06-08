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
//!   - DB_BACKUP, SEED_SETUP, FORCE_RESEED
//!   - GRACE_PERIOD_MINUTES, APP_PREFIX
//!
//! FORCE_RESEED is special: it is TRANSIENT — rendered (`=1`) only on a
//! redeploy invoked with an explicit `--reseed`, and explicitly REMOVED
//! on every other render. It must never persist, or every future plain
//! redeploy would force the api to drop+reseed and wipe data.
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
    "FORCE_RESEED",
    "GRACE_PERIOD_MINUTES",
    "APP_PREFIX",
    "GLOW_NETWORK",
];

/// The one persistent secret the client stack owns: the AES key Next.js
/// 15 uses to encrypt Server Action closure args. It must (a) be the SAME
/// across the blue and green app containers and all their Node workers —
/// otherwise an arg encrypted by one instance can't be decrypted by the
/// one nginx routes the follow-up to, yielding HTTP 500 `Invalid Server
/// Actions request` — and (b) be STABLE across redeploys, so in-flight
/// sessions survive. We therefore generate it ONCE on first client deploy
/// and preserve it on every redeploy. Both compose colors read it from the
/// single shared .env, so blue == green is structural. It is deliberately
/// NOT in CLIENT_OWNED_KEYS (those are re-derived/clobbered each redeploy).
const SERVER_ACTIONS_ENC_KEY: &str = "NEXT_SERVER_ACTIONS_ENCRYPTION_KEY";

/// Keys the CLI mutates on every redeploy of the client stack. These are
/// re-derived from the api's .env + topology each redeploy. The client's
/// one persistent secret (SERVER_ACTIONS_ENC_KEY) is intentionally absent
/// here — it is written once and preserved.
const CLIENT_OWNED_KEYS: &[&str] = &[
    "CLIENT_VERSION",
    "ACTIVE_CLIENT_ENV",
    "COMPOSE_PROJECT_NAME",
    "DOMAIN",
    "NEXT_PUBLIC_API_URL",
    "INTERNAL_API_BASE",
    "NEXT_PUBLIC_API_BASE",
    "AUTH_SECRET",
    // The academic client uses a custom `glow` OIDC provider that reads
    // AUTH_ISSUER / AUTH_CLIENT_ID / AUTH_CLIENT_SECRET (not the NextAuth
    // built-in Keycloak provider's AUTH_KEYCLOAK_* names).
    "AUTH_ISSUER",
    "AUTH_ISSUER_INTERNAL",
    "AUTH_CLIENT_ID",
    "AUTH_CLIENT_SECRET",
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
    /// TRANSIENT force-reseed flag. True ONLY when this redeploy was
    /// invoked with an explicit `--reseed` (i.e. `seed_arg.is_some()`),
    /// NOT derived from the persisted seed_setup state. When true we
    /// render `FORCE_RESEED=1` so the api's db-init drops+reseeds even
    /// when the DB already has tables and SEED_SETUP is unchanged; when
    /// false we REMOVE any stale FORCE_RESEED so a plain redeploy never
    /// wipes data.
    pub force_reseed: bool,
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
    pub client_http_port: String,
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
    /// OIDC issuer URL the academic client's `glow` provider validates
    /// tokens against. Must equal the `iss` claim the api mints, which
    /// is ORIGIN-derived. Per topology:
    ///   airgapped → ${client_origin}   (api proxied via client nginx)
    ///   exposed   → ${api_origin}      (api on its own domain)
    /// Caveat: in local airgapped on a single host, the client container
    /// can't fetch /.well-known/openid-configuration from a localhost
    /// URL (localhost = container itself). Override AUTH_ISSUER in .env
    /// to a container-reachable URL for local-only testing, accepting
    /// that the iss-claim check will then fail in NextAuth.
    pub auth_issuer: String,
    /// Container-reachable URL the Next.js server uses for token /
    /// userinfo / jwks calls — bypasses the browser-facing `issuer`
    /// for endpoints the client backend hits directly. Typically the
    /// api-nginx alias on the shared docker network (same value as
    /// MCP_BACKEND with an `http://` prefix). Defaults to AUTH_ISSUER
    /// on the client side, so production with real DNS needs no
    /// override.
    pub auth_issuer_internal: String,
    /// OIDC client id. Defaults to `glow-client`.
    pub auth_client_id: String,
    /// OIDC client secret — same value as the api's AUTH_CLIENT_SECRET
    /// (PBKDF2-derived from SECRET_KEY).
    pub auth_client_secret: String,
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
    // TRANSIENT: render FORCE_RESEED=1 ONLY when this run was invoked with
    // an explicit `--reseed`; otherwise REMOVE it so a stale flag from a
    // prior reseed never causes a future plain redeploy to wipe data.
    if inputs.force_reseed {
        upsert(&mut env, "FORCE_RESEED", "1");
    } else {
        env.remove("FORCE_RESEED");
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

    // Persistent client secret: generate once, preserve thereafter.
    // Regenerating on every redeploy would change the AES key out from
    // under in-flight Server Action sessions; an absent/blank entry means
    // each Next instance invents its own ephemeral key and nginx-balanced
    // requests across blue/green fail to decrypt each other's args.
    if env
        .get(SERVER_ACTIONS_ENC_KEY)
        .map(|v| v.is_empty())
        .unwrap_or(true)
    {
        env.insert(SERVER_ACTIONS_ENC_KEY.into(), random_base64(32));
    }

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
        "AUTH_ISSUER",
        &inputs.auth_issuer,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "AUTH_ISSUER_INTERNAL",
        &inputs.auth_issuer_internal,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "AUTH_CLIENT_ID",
        &inputs.auth_client_id,
    );
    upsert_in(
        &mut env,
        CLIENT_OWNED_KEYS,
        "AUTH_CLIENT_SECRET",
        &inputs.auth_client_secret,
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
        &inputs.client_http_port,
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

/// `n_bytes` of CSPRNG randomness, standard-base64-encoded — the exact
/// shape `openssl rand -base64 <n_bytes>` produces. Next.js 15 expects
/// NEXT_SERVER_ACTIONS_ENCRYPTION_KEY to be a base64-encoded 32-byte
/// (256-bit) AES key.
fn random_base64(n_bytes: usize) -> String {
    use rand::RngCore;
    let mut buf = vec![0u8; n_bytes];
    rand::thread_rng().fill_bytes(&mut buf);
    base64::engine::general_purpose::STANDARD.encode(buf)
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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    fn sample_api_inputs() -> EnvInputs {
        EnvInputs {
            api_version: "v1.0.93".into(),
            origin: "https://demo.example.com".into(),
            client_origins: "https://demo.example.com".into(),
            active_env: "blue".into(),
            active_kc_env: "blue".into(),
            project_name: "glow-test-api".into(),
            seed_setup: Some("university".into()),
            force_reseed: false,
            db_backup: None,
            grace_period_minutes: 2,
            app_prefix: String::new(),
            glow_network: "glow-test-net".into(),
        }
    }

    /// A redeploy invoked with `--reseed university` (force_reseed=true,
    /// seed_setup=Some) must render BOTH `FORCE_RESEED=1` and the matching
    /// `SEED_SETUP=university`, so the api's db-init drops+reseeds even when
    /// the DB already has tables and SEED_SETUP is unchanged.
    #[test]
    fn reseed_renders_force_reseed_and_seed_setup() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".env");
        // Pre-existing .env (a redeploy, not a first deploy).
        fs::write(&path, "SECRET_KEY=deadbeef\nSEED_SETUP=university\n").unwrap();

        let mut inputs = sample_api_inputs();
        inputs.force_reseed = true;
        inputs.seed_setup = Some("university".into());
        render(&path, &inputs).unwrap();

        let env = parse(&fs::read_to_string(&path).unwrap());
        assert_eq!(env.get("FORCE_RESEED").map(String::as_str), Some("1"));
        assert_eq!(
            env.get("SEED_SETUP").map(String::as_str),
            Some("university")
        );
        // Preserved secret untouched.
        assert_eq!(env.get("SECRET_KEY").map(String::as_str), Some("deadbeef"));
    }

    /// A PLAIN redeploy (force_reseed=false) must render NO `FORCE_RESEED`,
    /// and must REMOVE any stale `FORCE_RESEED=1` left in the .env by a prior
    /// `--reseed` run — otherwise every future redeploy would force a
    /// destructive drop+reseed and wipe data.
    #[test]
    fn plain_redeploy_removes_stale_force_reseed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".env");
        // Simulate a prior `--reseed` having left FORCE_RESEED=1 behind.
        fs::write(
            &path,
            "SECRET_KEY=deadbeef\nSEED_SETUP=university\nFORCE_RESEED=1\n",
        )
        .unwrap();

        let mut inputs = sample_api_inputs();
        inputs.force_reseed = false; // plain redeploy
        inputs.seed_setup = Some("university".into()); // persisted setup re-rendered
        render(&path, &inputs).unwrap();

        let env = parse(&fs::read_to_string(&path).unwrap());
        assert!(
            !env.contains_key("FORCE_RESEED"),
            "stale FORCE_RESEED must be removed on a plain redeploy"
        );
        // Persisted setup is still rendered (data preserved, no reseed).
        assert_eq!(
            env.get("SEED_SETUP").map(String::as_str),
            Some("university")
        );
    }

    /// A first deploy (no existing .env) with no `--reseed` generates secrets
    /// and renders NO `FORCE_RESEED` — the empty DB is seeded regardless.
    #[test]
    fn first_deploy_renders_no_force_reseed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".env");

        let mut inputs = sample_api_inputs();
        inputs.force_reseed = false;
        render(&path, &inputs).unwrap();

        let env = parse(&fs::read_to_string(&path).unwrap());
        assert!(
            !env.contains_key("FORCE_RESEED"),
            "first deploy must not render FORCE_RESEED"
        );
        // First-deploy secret generation is unaffected.
        assert!(env.contains_key("SECRET_KEY"));
        assert!(env.contains_key("DEPLOYMENT_TOKEN"));
    }

    fn sample_client_inputs() -> ClientEnvInputs {
        ClientEnvInputs {
            client_version: "v1.0.20".into(),
            active_client_env: "blue".into(),
            project_name: "glow-test-client".into(),
            glow_network: "glow-test-net".into(),
            client_http_port: "127.0.0.1:18080".into(),
            domain: "demo.example.com".into(),
            public_api_url: "https://demo.example.com".into(),
            internal_api_base: "http://glow-test-nginx:80".into(),
            auth_secret: "deadbeef".into(),
            auth_issuer: "https://demo.example.com".into(),
            auth_issuer_internal: "http://glow-test-nginx:80".into(),
            auth_client_id: "glow-client".into(),
            auth_client_secret: "kc-secret".into(),
            nextauth_url: "https://demo.example.com".into(),
            mcp_backend: "glow-test-nginx:80".into(),
        }
    }

    fn render_to_tmp(inputs: &ClientEnvInputs) -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".env");
        render_client(&path, inputs).unwrap();
        (dir, path)
    }

    /// First client deploy renders a non-empty Server Actions key that is a
    /// valid base64-encoded 32-byte (256-bit) AES key, exactly the shape
    /// Next.js 15 expects. Because both compose colors read this single
    /// .env value via `${NEXT_SERVER_ACTIONS_ENCRYPTION_KEY}`, blue == green
    /// is guaranteed structurally.
    #[test]
    fn renders_base64_32_server_actions_key() {
        let (_dir, path) = render_to_tmp(&sample_client_inputs());
        let env = parse(&fs::read_to_string(&path).unwrap());
        let key = env
            .get(SERVER_ACTIONS_ENC_KEY)
            .expect("NEXT_SERVER_ACTIONS_ENCRYPTION_KEY must be rendered into the client .env");
        assert!(!key.is_empty(), "key must be non-empty");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(key)
            .expect("key must be valid standard base64");
        assert_eq!(
            decoded.len(),
            32,
            "key must decode to 32 bytes (256-bit AES)"
        );
    }

    /// Redeploys must REUSE the existing key (idempotent) — regenerating it
    /// would change the AES key out from under in-flight Server Action
    /// sessions. Owned keys around it are still re-derived.
    #[test]
    fn server_actions_key_is_stable_across_render_passes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".env");

        render_client(&path, &sample_client_inputs()).unwrap();
        let first = parse(&fs::read_to_string(&path).unwrap())
            .get(SERVER_ACTIONS_ENC_KEY)
            .cloned()
            .unwrap();

        // Second pass with different owned inputs (simulating a redeploy at
        // a new client version): the persistent secret must not change.
        let mut next = sample_client_inputs();
        next.client_version = "v1.0.21".into();
        render_client(&path, &next).unwrap();
        let second_env = parse(&fs::read_to_string(&path).unwrap());

        assert_eq!(
            second_env.get(SERVER_ACTIONS_ENC_KEY),
            Some(&first),
            "Server Actions key must be preserved (not regenerated) on redeploy"
        );
        // Sanity: owned keys still get re-derived.
        assert_eq!(
            second_env.get("CLIENT_VERSION").map(String::as_str),
            Some("v1.0.21")
        );
    }

    /// A pre-existing instance whose .env was written before this fix (key
    /// absent) gets a freshly generated key — but one already present is
    /// kept verbatim.
    #[test]
    fn preexisting_key_is_kept_blank_or_missing_is_filled() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".env");

        // Seed an .env that already pins a specific key.
        let pinned = base64::engine::general_purpose::STANDARD.encode([7u8; 32]);
        fs::write(&path, format!("{SERVER_ACTIONS_ENC_KEY}={pinned}\n")).unwrap();
        render_client(&path, &sample_client_inputs()).unwrap();
        assert_eq!(
            parse(&fs::read_to_string(&path).unwrap()).get(SERVER_ACTIONS_ENC_KEY),
            Some(&pinned),
            "an existing key must be preserved verbatim"
        );

        // A blank value is treated as absent and filled.
        let path2 = dir.path().join(".env2");
        fs::write(&path2, format!("{SERVER_ACTIONS_ENC_KEY}=\n")).unwrap();
        render_client(&path2, &sample_client_inputs()).unwrap();
        let filled = parse(&fs::read_to_string(&path2).unwrap())
            .get(SERVER_ACTIONS_ENC_KEY)
            .cloned()
            .unwrap();
        assert!(!filled.is_empty(), "a blank key must be regenerated");
    }
}
