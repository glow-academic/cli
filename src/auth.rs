// auth.rs — OAuth/OIDC login flow with multi-server token storage
//
// Implements the Authorization Code flow with a localhost callback:
//   1. Discover OIDC endpoints from the server
//   2. Start a local HTTP server for the callback
//   3. Open the browser to the authorize URL
//   4. Exchange the authorization code for tokens
//   5. Store tokens locally, keyed by server URL
//
// Supports N concurrent sessions — one per server URL:
//   - 1 LearnLoop API token (central platform)
//   - N Glow instance tokens (one per instance)
//
// In Python terms: like authlib or oauthlib
// In TS terms:     like openid-client

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;

// ── Types ─────────────────────────────────────────────────────

/// OIDC Discovery document (subset of fields we use)
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct OidcDiscovery {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: Option<String>,
}

/// Token response from the token endpoint
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
    token_type: String,
    #[serde(default)]
    expires_in: Option<u64>,
    #[serde(default)]
    refresh_token: Option<String>,
}

/// A single server's stored credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub access_token: String,
    pub id_token: Option<String>,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub issued_at: u64,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub token_endpoint: Option<String>,
}

/// All tokens, keyed by server URL
/// e.g. { "https://api.learn-loop.org": { ... }, "http://localhost:8000": { ... } }
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TokenStore {
    #[serde(flatten)]
    pub tokens: HashMap<String, StoredToken>,
}

// ── Token storage ─────────────────────────────────────────────

fn tokens_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("glow")
        .join("tokens.json")
}

/// Load the full token store from disk
pub fn load_token_store() -> Result<TokenStore> {
    let path = tokens_path();
    if !path.exists() {
        return Ok(TokenStore::default());
    }
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read token file: {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse token file: {}", path.display()))
}

/// Get a token for a specific server URL, auto-refreshing if expired.
pub fn get_token(server_url: &str) -> Result<StoredToken> {
    let store = load_token_store()?;
    let normalized = normalize_url(server_url);
    let token = store.tokens.get(&normalized).cloned().ok_or_else(|| {
        anyhow::anyhow!(
            "Not logged in to {}. Run 'glow login' to authenticate.",
            server_url
        )
    })?;

    // Check if token is expired (with 60s buffer)
    if let Some(expires_in) = token.expires_in {
        let age = now_epoch().saturating_sub(token.issued_at);
        if age + 60 >= expires_in {
            // Try to refresh
            if let (Some(ref rt), Some(ref endpoint)) =
                (&token.refresh_token, &token.token_endpoint)
            {
                if let Ok(refreshed) = refresh_access_token(endpoint, rt, server_url) {
                    return Ok(refreshed);
                }
            }
        }
    }

    Ok(token)
}

/// Exchange a refresh token for a new access token
fn refresh_access_token(
    token_endpoint: &str,
    refresh_token: &str,
    server_url: &str,
) -> Result<StoredToken> {
    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = http
        .post(token_endpoint)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .context("Failed to refresh token")?;

    if !resp.status().is_success() {
        anyhow::bail!("Token refresh failed (HTTP {})", resp.status());
    }

    let token_resp: TokenResponse = resp.json().context("Failed to parse refresh response")?;

    let stored = StoredToken {
        access_token: token_resp.access_token,
        id_token: token_resp.id_token,
        token_type: token_resp.token_type,
        expires_in: token_resp.expires_in,
        issued_at: now_epoch(),
        refresh_token: token_resp
            .refresh_token
            .or_else(|| Some(refresh_token.to_string())),
        token_endpoint: Some(token_endpoint.to_string()),
    };
    save_token(server_url, stored.clone())?;

    Ok(stored)
}

/// Save a token for a specific server URL
fn save_token(server_url: &str, token: StoredToken) -> Result<()> {
    let path = tokens_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
    }

    let mut store = load_token_store().unwrap_or_default();
    let normalized = normalize_url(server_url);
    store.tokens.insert(normalized, token);

    let json = serde_json::to_string_pretty(&store)?;
    std::fs::write(&path, json)
        .with_context(|| format!("Failed to write token file: {}", path.display()))?;
    Ok(())
}

/// Remove a token for a specific server URL
#[allow(dead_code)]
pub fn remove_token(server_url: &str) -> Result<bool> {
    let mut store = load_token_store().unwrap_or_default();
    let normalized = normalize_url(server_url);
    let existed = store.tokens.remove(&normalized).is_some();

    if existed {
        let path = tokens_path();
        let json = serde_json::to_string_pretty(&store)?;
        std::fs::write(&path, json)?;
    }
    Ok(existed)
}

/// Normalize a URL for use as a token store key (strip trailing slash)
fn normalize_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}

/// Current Unix timestamp
fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── OIDC Discovery ────────────────────────────────────────────

#[allow(dead_code)]
pub fn discover(server_url: &str) -> Result<OidcDiscovery> {
    let url = format!(
        "{}/.well-known/openid-configuration",
        server_url.trim_end_matches('/')
    );
    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = http.get(&url).send().with_context(|| {
        format!(
            "Failed to reach OIDC discovery at {}. Check the URL and your network.",
            url
        )
    })?;

    if !resp.status().is_success() {
        anyhow::bail!(
            "OIDC discovery failed (HTTP {}). The server at {} may not support OAuth.",
            resp.status(),
            server_url
        );
    }

    resp.json::<OidcDiscovery>()
        .context("Failed to parse OIDC discovery document.")
}

// ── OAuth flow internals ──────────────────────────────────────

/// Generate a random state parameter for CSRF protection
#[allow(dead_code)]
fn generate_state() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    let mut hasher = DefaultHasher::new();
    nanos.hash(&mut hasher);
    pid.hash(&mut hasher);
    format!("{:016x}{:016x}", hasher.finish(), nanos as u64)
}

/// Percent-encode a URL for use in a query parameter
#[allow(dead_code)]
fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => {
                out.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    out
}

/// Start a temporary HTTP server on a random port
#[allow(dead_code)]
fn start_callback_server() -> Result<(TcpListener, String)> {
    let listener =
        TcpListener::bind("127.0.0.1:0").context("Failed to bind callback server on localhost.")?;
    let port = listener.local_addr()?.port();
    let redirect_uri = format!("http://127.0.0.1:{}/callback", port);
    Ok((listener, redirect_uri))
}

/// Wait for the OAuth callback. Returns the authorization code.
#[allow(dead_code)]
fn wait_for_callback(listener: &TcpListener, expected_state: &str) -> Result<String> {
    let (mut stream, _) = listener
        .accept()
        .context("Failed to receive OAuth callback. Did you complete login in the browser?")?;

    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf[..n]);

    // Parse "GET /callback?code=xxx&state=yyy HTTP/1.1"
    let first_line = request.lines().next().unwrap_or_default();
    let path = first_line.split_whitespace().nth(1).unwrap_or_default();
    let query = path.split('?').nth(1).unwrap_or_default();
    let params: HashMap<&str, &str> = query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            Some((parts.next()?, parts.next()?))
        })
        .collect();

    // Check for OAuth errors
    if let Some(error) = params.get("error") {
        let desc = params.get("error_description").unwrap_or(&"");
        send_html(
            &mut stream,
            &format!(
                "<h2>Login Failed</h2><p>{}: {}</p><p>You can close this tab.</p>",
                error, desc
            ),
        );
        anyhow::bail!("OAuth error: {} — {}", error, desc);
    }

    // Verify state (CSRF protection)
    let state = params.get("state").unwrap_or(&"");
    if *state != expected_state {
        send_html(
            &mut stream,
            "<h2>Login Failed</h2><p>State mismatch. Please try again.</p>",
        );
        anyhow::bail!("OAuth state mismatch — possible CSRF attack. Try logging in again.");
    }

    // Extract authorization code
    let code = params.get("code").ok_or_else(|| {
        send_html(
            &mut stream,
            "<h2>Login Failed</h2><p>No authorization code received.</p>",
        );
        anyhow::anyhow!("No authorization code in callback.")
    })?;

    send_html(
        &mut stream,
        "<h2>Login Successful</h2><p>You can close this tab and return to the terminal.</p>",
    );

    Ok(code.to_string())
}

/// Send a minimal HTML response back to the browser
#[allow(dead_code)]
fn send_html(stream: &mut std::net::TcpStream, body: &str) {
    let html = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Glow</title>\
         <style>body{{font-family:system-ui;max-width:480px;margin:80px auto;text-align:center}}</style>\
         </head><body>{}</body></html>",
        body
    );
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        html.len(),
        html
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

/// Exchange an authorization code for tokens
#[allow(dead_code)]
fn exchange_code(
    token_endpoint: &str,
    code: &str,
    redirect_uri: &str,
    client_id: &str,
    client_secret: Option<&str>,
) -> Result<TokenResponse> {
    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let mut params = vec![
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
    ];
    // Glow instances require client_secret; LearnLoop API does not
    if let Some(secret) = client_secret {
        params.push(("client_secret", secret));
    }

    let resp = http
        .post(token_endpoint)
        .form(&params)
        .send()
        .with_context(|| format!("Failed to connect to token endpoint: {}", token_endpoint))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        anyhow::bail!("Token exchange failed (HTTP {}): {}", status, text);
    }

    resp.json::<TokenResponse>()
        .context("Failed to parse token response.")
}

// ── Public API ────────────────────────────────────────────────

/// Run the full OAuth login flow for a given server.
/// Works for both the LearnLoop API and any Glow instance.
#[allow(dead_code)]
pub fn login(server_url: &str, client_id: &str) -> Result<StoredToken> {
    login_with_secret(server_url, client_id, None)
}

/// Run the full OAuth login flow, optionally with a client secret.
#[allow(dead_code)]
pub fn login_with_secret(
    server_url: &str,
    client_id: &str,
    client_secret: Option<&str>,
) -> Result<StoredToken> {
    // 1. Discover OIDC endpoints
    let discovery = discover(server_url)?;

    // 2. Start callback server
    let (listener, redirect_uri) = start_callback_server()?;

    // 3. Build authorize URL
    let state = generate_state();
    let authorize_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
        discovery.authorization_endpoint,
        percent_encode(client_id),
        percent_encode(&redirect_uri),
        percent_encode("openid profile email"),
        &state,
    );

    // 4. Open browser
    eprintln!("Opening browser for login...");
    eprintln!("If the browser doesn't open, visit:");
    eprintln!("  {}", authorize_url);

    if let Err(e) = open::that(&authorize_url) {
        eprintln!("Warning: could not open browser ({})", e);
    }

    // 5. Wait for callback
    eprintln!("Waiting for login to complete...");
    let code = wait_for_callback(&listener, &state)?;

    // 6. Exchange code for tokens
    let token_resp = exchange_code(
        &discovery.token_endpoint,
        &code,
        &redirect_uri,
        client_id,
        client_secret,
    )?;

    // 7. Store token keyed by server URL
    let stored = StoredToken {
        access_token: token_resp.access_token,
        id_token: token_resp.id_token,
        token_type: token_resp.token_type,
        expires_in: token_resp.expires_in,
        issued_at: now_epoch(),
        refresh_token: token_resp.refresh_token,
        token_endpoint: Some(discovery.token_endpoint.clone()),
    };
    save_token(server_url, stored.clone())?;

    Ok(stored)
}

/// List all servers the user is logged into
#[allow(dead_code)]
pub fn list_sessions() -> Result<Vec<(String, StoredToken)>> {
    let store = load_token_store()?;
    let mut sessions: Vec<_> = store.tokens.into_iter().collect();
    sessions.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(sessions)
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── URL normalization ────────────────────────────────

    #[test]
    fn test_normalize_url_strips_trailing_slash() {
        assert_eq!(
            normalize_url("http://localhost:8000/"),
            "http://localhost:8000"
        );
    }

    #[test]
    fn test_normalize_url_no_trailing_slash() {
        assert_eq!(
            normalize_url("http://localhost:8000"),
            "http://localhost:8000"
        );
    }

    #[test]
    fn test_normalize_url_with_path() {
        assert_eq!(
            normalize_url("https://api.example.com/v1/"),
            "https://api.example.com/v1"
        );
    }

    // ── Percent encoding ─────────────────────────────────

    #[test]
    fn test_percent_encode_simple() {
        assert_eq!(percent_encode("hello"), "hello");
    }

    #[test]
    fn test_percent_encode_url() {
        assert_eq!(
            percent_encode("http://127.0.0.1:9999/callback"),
            "http%3A%2F%2F127.0.0.1%3A9999%2Fcallback"
        );
    }

    #[test]
    fn test_percent_encode_spaces() {
        assert_eq!(
            percent_encode("openid profile email"),
            "openid%20profile%20email"
        );
    }

    // ── State generation ─────────────────────────────────

    #[test]
    fn test_generate_state_length() {
        let state = generate_state();
        assert_eq!(state.len(), 32);
    }

    #[test]
    fn test_generate_state_is_hex() {
        let state = generate_state();
        assert!(state.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_state_varies() {
        // Sleep a tiny bit to get different nanos
        let s1 = generate_state();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let s2 = generate_state();
        assert_ne!(s1, s2);
    }

    // ── Token store (file I/O) ───────────────────────────

    #[test]
    fn test_token_store_roundtrip() {
        let mut store = TokenStore::default();
        store.tokens.insert(
            "http://localhost:8000".into(),
            StoredToken {
                access_token: "tok-glow".into(),
                id_token: None,
                token_type: "Bearer".into(),
                expires_in: Some(3600),
                issued_at: 1000,
                refresh_token: None,
                token_endpoint: None,
            },
        );
        store.tokens.insert(
            "https://api.learn-loop.org".into(),
            StoredToken {
                access_token: "tok-ll".into(),
                id_token: Some("id-tok".into()),
                token_type: "Bearer".into(),
                expires_in: Some(3600),
                issued_at: 2000,
                refresh_token: None,
                token_endpoint: None,
            },
        );

        let json = serde_json::to_string_pretty(&store).unwrap();
        let parsed: TokenStore = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.tokens.len(), 2);
        assert_eq!(
            parsed.tokens["http://localhost:8000"].access_token,
            "tok-glow"
        );
        assert_eq!(
            parsed.tokens["https://api.learn-loop.org"].access_token,
            "tok-ll"
        );
    }

    #[test]
    fn test_token_store_empty_roundtrip() {
        let store = TokenStore::default();
        let json = serde_json::to_string(&store).unwrap();
        let parsed: TokenStore = serde_json::from_str(&json).unwrap();
        assert!(parsed.tokens.is_empty());
    }

    #[test]
    fn test_token_store_overwrite() {
        let mut store = TokenStore::default();
        store.tokens.insert(
            "http://localhost:8000".into(),
            StoredToken {
                access_token: "old".into(),
                id_token: None,
                token_type: "Bearer".into(),
                expires_in: None,
                issued_at: 1000,
                refresh_token: None,
                token_endpoint: None,
            },
        );
        store.tokens.insert(
            "http://localhost:8000".into(),
            StoredToken {
                access_token: "new".into(),
                id_token: None,
                token_type: "Bearer".into(),
                expires_in: None,
                issued_at: 2000,
                refresh_token: None,
                token_endpoint: None,
            },
        );
        assert_eq!(store.tokens["http://localhost:8000"].access_token, "new");
    }

    // ── Callback server ──────────────────────────────────

    #[test]
    fn test_callback_server_binds() {
        let (listener, redirect_uri) = start_callback_server().unwrap();
        assert!(redirect_uri.starts_with("http://127.0.0.1:"));
        assert!(redirect_uri.ends_with("/callback"));
        drop(listener);
    }

    // ── Callback parsing (simulated) ─────────────────────

    #[test]
    fn test_callback_success() {
        let (listener, _redirect_uri) = start_callback_server().unwrap();
        let port = listener.local_addr().unwrap().port();
        let state = "test-state-123";

        // Simulate browser callback in a thread
        let state_clone = state.to_string();
        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
            let request = format!(
                "GET /callback?code=auth-code-xyz&state={} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
                state_clone
            );
            stream.write_all(request.as_bytes()).unwrap();
            // Read response
            let mut buf = [0u8; 2048];
            let _ = stream.read(&mut buf);
        });

        let code = wait_for_callback(&listener, state).unwrap();
        assert_eq!(code, "auth-code-xyz");
        handle.join().unwrap();
    }

    #[test]
    fn test_callback_state_mismatch() {
        let (listener, _redirect_uri) = start_callback_server().unwrap();
        let port = listener.local_addr().unwrap().port();

        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
            let request = "GET /callback?code=abc&state=wrong HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();
            let mut buf = [0u8; 2048];
            let _ = stream.read(&mut buf);
        });

        let err = wait_for_callback(&listener, "expected-state").unwrap_err();
        assert!(err.to_string().contains("state mismatch"));
        handle.join().unwrap();
    }

    #[test]
    fn test_callback_oauth_error() {
        let (listener, _redirect_uri) = start_callback_server().unwrap();
        let port = listener.local_addr().unwrap().port();

        let handle = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
            let request = "GET /callback?error=access_denied&error_description=User+cancelled HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();
            let mut buf = [0u8; 2048];
            let _ = stream.read(&mut buf);
        });

        let err = wait_for_callback(&listener, "any-state").unwrap_err();
        assert!(err.to_string().contains("access_denied"));
        handle.join().unwrap();
    }

    // ── OIDC discovery (mock) ────────────────────────────

    #[test]
    fn test_discover_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/.well-known/openid-configuration")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "authorization_endpoint": "https://auth.example.com/authorize",
                    "token_endpoint": "https://auth.example.com/token",
                    "userinfo_endpoint": "https://auth.example.com/userinfo"
                }"#,
            )
            .create();

        let disc = discover(&server.url()).unwrap();
        assert_eq!(
            disc.authorization_endpoint,
            "https://auth.example.com/authorize"
        );
        assert_eq!(disc.token_endpoint, "https://auth.example.com/token");
        mock.assert();
    }

    #[test]
    fn test_discover_server_down() {
        let err = discover("http://127.0.0.1:1").unwrap_err();
        assert!(err.to_string().contains("Failed to reach OIDC discovery"));
    }

    #[test]
    fn test_discover_not_found() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/.well-known/openid-configuration")
            .with_status(404)
            .create();

        let err = discover(&server.url()).unwrap_err();
        assert!(err.to_string().contains("OIDC discovery failed"));
    }

    // ── Token exchange (mock) ────────────────────────────

    #[test]
    fn test_exchange_code_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "access_token": "eyJ-access",
                    "id_token": "eyJ-id",
                    "token_type": "Bearer",
                    "expires_in": 3600
                }"#,
            )
            .create();

        let resp = exchange_code(
            &format!("{}/token", server.url()),
            "auth-code",
            "http://127.0.0.1:9999/callback",
            "api-client",
            None,
        )
        .unwrap();

        assert_eq!(resp.access_token, "eyJ-access");
        assert_eq!(resp.id_token, Some("eyJ-id".into()));
        assert_eq!(resp.token_type, "Bearer");
        assert_eq!(resp.expires_in, Some(3600));
        mock.assert();
    }

    #[test]
    fn test_exchange_code_failure() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/token")
            .with_status(400)
            .with_body("invalid_grant")
            .create();

        let err = exchange_code(
            &format!("{}/token", server.url()),
            "bad-code",
            "http://127.0.0.1:9999/callback",
            "api-client",
            None,
        )
        .unwrap_err();

        assert!(err.to_string().contains("Token exchange failed"));
    }
}
