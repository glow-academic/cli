// api_common.rs — Shared HTTP helpers for both API clients
//
// Both AdminClient and GlowClient use these.

use anyhow::{Context, Result};
use reqwest::blocking;

// ── Version compatibility check ─────────────────────────────

/// Compare server version against pinned version.
/// Prints a warning/error to stderr if versions are incompatible.
pub fn check_api_version(server_version: &str, pinned_version: &str, service_name: &str) {
    let server_parts: Vec<&str> = server_version.split('.').collect();
    let pinned_parts: Vec<&str> = pinned_version.split('.').collect();

    let server_major = server_parts
        .first()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let server_minor = server_parts
        .get(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let pinned_major = pinned_parts
        .first()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let pinned_minor = pinned_parts
        .get(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    if server_major != pinned_major {
        eprintln!(
            "\u{26a0}\u{fe0f}  {} version mismatch: server={}, cli expects={}",
            service_name, server_version, pinned_version
        );
        eprintln!(
            "   Major version differs — some features may not work. Update with: brew upgrade glow"
        );
    } else if server_minor > pinned_minor {
        eprintln!(
            "\u{2139}\u{fe0f}  {} v{} is newer than CLI expects (v{}). Consider updating: brew upgrade glow",
            service_name, server_version, pinned_version
        );
    } else if server_minor < pinned_minor {
        eprintln!(
            "\u{26a0}\u{fe0f}  {} v{} is older than CLI expects (v{}). Some features may not be available.",
            service_name, server_version, pinned_version
        );
    }
}

// ── Auth modes for the shared helper ─────────────────────────

pub(crate) enum Auth<'a> {
    None,
    Bearer(&'a str),
}

// ── Shared request helpers ───────────────────────────────────

fn apply_auth(req: blocking::RequestBuilder, auth: Auth) -> blocking::RequestBuilder {
    match auth {
        Auth::None => req,
        Auth::Bearer(token) => req.header("Authorization", format!("Bearer {}", token)),
    }
}

fn handle_error_status(status: reqwest::StatusCode, text: &str) -> anyhow::Error {
    match status.as_u16() {
        401 => {
            anyhow::anyhow!("Authentication failed (HTTP 401). Run 'glow login' to authenticate.")
        }
        403 => anyhow::anyhow!(
            "Permission denied (HTTP 403). Your account may not have access to this resource."
        ),
        404 => anyhow::anyhow!("Resource not found (HTTP 404): {}", text),
        _ => anyhow::anyhow!("API error (HTTP {}): {}", status, text),
    }
}

pub(crate) fn api_request<T: serde::de::DeserializeOwned>(
    http: &blocking::Client,
    method: reqwest::Method,
    url: &str,
    body: Option<serde_json::Value>,
    auth: Auth,
) -> Result<T> {
    let mut req = apply_auth(http.request(method, url), auth);

    if let Some(body) = body {
        req = req.json(&body);
    }

    let response = req.send().with_context(|| {
        format!(
            "Failed to connect to {}. Check your network or run 'glow network' to diagnose.",
            url
        )
    })?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return Err(handle_error_status(status, &text));
    }

    response.json::<T>().context(
        "Failed to parse API response. This might indicate a version mismatch between the CLI and server.",
    )
}

/// Like api_request but returns the raw Response (for SSE streaming, binary downloads, etc.)
pub(crate) fn api_request_raw(
    http: &blocking::Client,
    method: reqwest::Method,
    url: &str,
    body: Option<serde_json::Value>,
    auth: Auth,
) -> Result<blocking::Response> {
    let mut req = apply_auth(http.request(method, url), auth);

    if let Some(body) = body {
        req = req.json(&body);
    }

    let response = req.send().with_context(|| {
        format!(
            "Failed to connect to {}. Check your network or run 'glow network' to diagnose.",
            url
        )
    })?;

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        return Err(handle_error_status(status, &text));
    }

    Ok(response)
}
