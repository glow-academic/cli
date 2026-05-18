// glow/mod.rs — HTTP client for Glow instance APIs
//
// Each Glow instance is a separate deployment with its own URL.
// Auth: Bearer token (OAuth) — auto-loaded from stored auth.

pub mod api;
pub mod types;
pub mod ws;

use anyhow::{Context, Result};
use reqwest::blocking;
use serde_json::{json, Value};
use std::io::BufRead;

use crate::api_common::{api_request, api_request_raw, Auth};

pub struct GlowClient {
    base_url: String,
    http: blocking::Client,
    token: Option<String>,
}

impl GlowClient {
    pub fn new(base_url: &str) -> Self {
        // Auto-load OAuth token from stored auth for this Glow instance
        let token = crate::auth::get_token(base_url)
            .ok()
            .map(|t| t.access_token);

        GlowClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: blocking::Client::new(),
            token,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Public accessor for the base URL — the WS layer needs this to
    /// build the socket.io connect URL without going through the HTTP
    /// path helpers.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Public accessor for the stored bearer token (if any). Mirrors
    /// what ``self.auth()`` does internally; exposed so the WS layer
    /// can forward the same JWT through socket.io's auth handshake.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    fn auth(&self) -> Auth<'_> {
        match &self.token {
            Some(t) => Auth::Bearer(t),
            None => Auth::None,
        }
    }

    /// Build an authenticated request (for custom requests like uploads)
    fn authed_request(&self, method: reqwest::Method, url: &str) -> blocking::RequestBuilder {
        let mut req = self.http.request(method, url);
        if let Some(ref t) = self.token {
            req = req.header("Authorization", format!("Bearer {}", t));
        }
        req
    }

    // ── Server-side logout ────────────────────────────────────
    //
    // The API exposes ``GET /logout`` as an OIDC end_session_endpoint
    // — it writes a ``logouts_entry`` row when the caller includes a
    // bearer token, then redirects to Keycloak's logout for the
    // browser. For CLI use we fire the request best-effort (don't
    // follow the redirect, don't fail the local clear if this fails)
    // so the server-side session state matches the client's intent.

    /// Best-effort server-side logout. Returns ``Ok(())`` on any
    /// outcome (including network failure) so the caller can always
    /// proceed to clear the local token store.
    pub fn logout_server_side(&self) -> Result<()> {
        let url = self.url("/logout");
        // Build a request that does NOT follow redirects — we don't
        // want to chase Keycloak's logout redirect target from a CLI
        // context. The 302 is treated as success here.
        let client = blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap_or_else(|_| blocking::Client::new());
        let mut req = client.get(&url);
        if let Some(ref t) = self.token {
            req = req.header("Authorization", format!("Bearer {}", t));
        }
        let _ = req.send(); // swallow errors — best-effort
        Ok(())
    }

    // ``health`` removed in Cleanup E — the method hit GET / (root
    // liveness), not a real /health route. The health artifact lives
    // at POST /system/health and is reached via
    // ``resource_action("system", "health", ...)``.

    // ── Generic resource CRUD (v5 routes) ────────────────────
    //
    // The new URL pattern: POST /{resource}/{action}

    pub fn resource_action(
        &self,
        resource: &str,
        action: &str,
        body: Option<Value>,
    ) -> Result<Value> {
        let url = self.url(&format!("/{}/{}", resource, action));
        api_request(
            &self.http,
            reqwest::Method::POST,
            &url,
            Some(body.unwrap_or_else(|| json!({}))),
            self.auth(),
        )
    }

    // ``context`` / ``emulate`` / ``unemulate`` / ``problem`` removed
    // in Cleanup D — these aren't top-level routes:
    //   * /context → POST /<artifact>/context on every artifact
    //   * /problem → POST /<artifact>/problem on every artifact
    //   * /emulate, /unemulate → POST /profile/{emulate,unemulate}
    //     (profile artifact only)
    // Callers use ``resource_action(api_path, action, body)`` instead.

    // ``generate`` removed — no top-level /generate route on the API.
    // Generation is per-artifact: POST /{artifact}/generate. Callers
    // should use ``resource_action(api_path, "generate", body)``.

    // ── Per-resource media operations ──────────────────────────
    //
    // URL patterns:
    //   POST /{resource}/{media}/upload          — multipart upload
    //   GET  /{resource}/{media}/discover         — discover types
    //   GET  /{resource}/{media}/discover/{id}    — discover specific
    //   POST /{resource}/{media}/create           — TUS initiation
    //   GET  /{resource}/{media}/{id}/status      — TUS status
    //   PATCH /{resource}/{media}/{id}/chunk      — TUS chunk
    //   POST /{resource}/{media}/{id}/finalize    — TUS finalize
    //   GET  /{resource}/{media}/{id}/download    — download
    //   GET  /{resource}/{media}/{id}/preview     — preview

    /// Upload a file via multipart form
    pub fn media_upload(&self, resource: &str, media_type: &str, file_path: &str) -> Result<Value> {
        let path = std::path::Path::new(file_path);
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();
        let data = std::fs::read(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path))?;

        let part = blocking::multipart::Part::bytes(data).file_name(filename);
        let form = blocking::multipart::Form::new().part("file", part);

        let url = self.url(&format!("/{}/{}/upload", resource, media_type));
        let resp = self
            .authed_request(reqwest::Method::POST, &url)
            .multipart(form)
            .send()
            .with_context(|| format!("Failed to upload to {}", url))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Upload failed (HTTP {}): {}", status, text);
        }

        resp.json::<Value>()
            .context("Failed to parse upload response")
    }

    /// Discover available upload types for a resource media
    pub fn media_discover(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: Option<&str>,
    ) -> Result<Value> {
        let url = match upload_id {
            Some(id) => self.url(&format!("/{}/{}/discover/{}", resource, media_type, id)),
            None => self.url(&format!("/{}/{}/discover", resource, media_type)),
        };
        api_request(&self.http, reqwest::Method::GET, &url, None, self.auth())
    }

    /// TUS create (initiate resumable upload)
    pub fn media_create(
        &self,
        resource: &str,
        media_type: &str,
        filename: &str,
        size: Option<u64>,
    ) -> Result<Value> {
        let mut body = json!({ "filename": filename });
        if let Some(s) = size {
            body["size"] = json!(s);
        }
        let url = self.url(&format!("/{}/{}/create", resource, media_type));
        api_request(
            &self.http,
            reqwest::Method::POST,
            &url,
            Some(body),
            self.auth(),
        )
    }

    /// TUS chunk upload
    pub fn media_chunk(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
        data: Vec<u8>,
        offset: u64,
    ) -> Result<Value> {
        let url = self.url(&format!("/{}/{}/{}/chunk", resource, media_type, upload_id));
        let resp = self
            .authed_request(reqwest::Method::PATCH, &url)
            .header("Content-Type", "application/offset+octet-stream")
            .header("Upload-Offset", offset.to_string())
            .body(data)
            .send()
            .with_context(|| format!("Failed to upload chunk to {}", url))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Chunk upload failed (HTTP {}): {}", status, text);
        }

        let new_offset = resp
            .headers()
            .get("Upload-Offset")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(json!({ "offset": new_offset }))
    }

    /// TUS status check
    pub fn media_status(&self, resource: &str, media_type: &str, upload_id: &str) -> Result<Value> {
        let url = self.url(&format!(
            "/{}/{}/{}/status",
            resource, media_type, upload_id
        ));
        api_request(&self.http, reqwest::Method::GET, &url, None, self.auth())
    }

    /// TUS finalize
    pub fn media_finalize(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
        body: Option<Value>,
    ) -> Result<Value> {
        let url = self.url(&format!(
            "/{}/{}/{}/finalize",
            resource, media_type, upload_id
        ));
        api_request(
            &self.http,
            reqwest::Method::POST,
            &url,
            Some(body.unwrap_or_else(|| json!({}))),
            self.auth(),
        )
    }

    /// Download a media file
    pub fn media_download(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
    ) -> Result<Vec<u8>> {
        let url = self.url(&format!(
            "/{}/{}/{}/download",
            resource, media_type, upload_id
        ));
        let resp = api_request_raw(&self.http, reqwest::Method::GET, &url, None, self.auth())?;
        let bytes = resp.bytes().context("Failed to read download response")?;
        Ok(bytes.to_vec())
    }

    /// Preview a media file
    pub fn media_preview(
        &self,
        resource: &str,
        media_type: &str,
        upload_id: &str,
    ) -> Result<Value> {
        let url = self.url(&format!(
            "/{}/{}/{}/preview",
            resource, media_type, upload_id
        ));
        api_request(&self.http, reqwest::Method::GET, &url, None, self.auth())
    }

    // ── Watch (per-artifact, run-scoped SSE) ─────────────────
    //
    // The API exposes ``GET /{artifact}/watch?run_id=...&group_id=...``
    // as a per-artifact SSE stream filtered to a specific run. The
    // resolver inside the route reads ``run_id`` from the event
    // envelope and only emits matching frames. Used by the watch
    // helper to block until the run reaches a terminal lifecycle
    // state.

    /// Open the per-artifact watch SSE stream filtered to ``run_id``.
    /// Returns the raw Response so the caller can read events
    /// line-by-line. ``api_path`` should be the singular artifact
    /// path (``attempt``, ``scenario``, ...).
    pub fn watch_run(
        &self,
        api_path: &str,
        run_id: &str,
        group_id: Option<&str>,
    ) -> Result<blocking::Response> {
        let mut params = vec![format!("run_id={}", run_id)];
        if let Some(g) = group_id {
            params.push(format!("group_id={}", g));
        }
        let url = format!(
            "{}?{}",
            self.url(&format!("/{}/watch", api_path)),
            params.join("&"),
        );
        let resp = self
            .authed_request(reqwest::Method::GET, &url)
            .header("Accept", "text/event-stream")
            .send()
            .with_context(|| format!("Failed to connect to watch stream at {}", url))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            anyhow::bail!("Watch stream error (HTTP {}): {}", status, text);
        }
        Ok(resp)
    }

    // ``stream`` removed — no top-level /stream route on the API.
    // Per-artifact streaming is GET /{artifact}/watch?run_id=...,
    // exposed via ``watch_run`` above.
}

// ── SSE helper ────────────────────────────────────────────────

/// Read SSE events from a response and call the handler for each data line.
///
/// Kept (with ``#[allow(dead_code)]``) as the simple variant alongside
/// ``read_sse_events_with_names`` — useful when callers don't care about
/// event-name terminal detection.
#[allow(dead_code)]
pub fn read_sse_events(response: blocking::Response, mut handler: impl FnMut(&str)) -> Result<()> {
    let reader = std::io::BufReader::new(response);
    for line in reader.lines() {
        let line = line.context("Error reading SSE stream")?;
        if let Some(data) = line.strip_prefix("data: ") {
            handler(data);
        }
        // Silently skip event:, id:, retry:, and blank lines
    }
    Ok(())
}

/// Read SSE events with both event names and payloads. Handler receives
/// ``(event_name, data_json_str)`` per frame. ``event_name`` is empty
/// when the SSE frame omits the ``event:`` line (rare for our API but
/// permitted by the SSE spec). Used by the watch helper to detect
/// terminal lifecycle events (``.completed`` / ``.failed``) without
/// having to parse the data payload.
pub fn read_sse_events_with_names(
    response: blocking::Response,
    mut handler: impl FnMut(&str, &str) -> std::ops::ControlFlow<()>,
) -> Result<()> {
    let reader = std::io::BufReader::new(response);
    let mut current_event = String::new();
    for line in reader.lines() {
        let line = line.context("Error reading SSE stream")?;
        if let Some(name) = line.strip_prefix("event: ") {
            current_event = name.to_string();
            continue;
        }
        if let Some(data) = line.strip_prefix("data: ") {
            if let std::ops::ControlFlow::Break(()) = handler(&current_event, data) {
                return Ok(());
            }
            // Per SSE spec, event: applies only to the next data: frame;
            // reset after dispatching.
            current_event.clear();
            continue;
        }
        if line.is_empty() {
            // Blank line = end of message; reset any pending event name.
            current_event.clear();
        }
        // Silently skip id:, retry:, and other lines.
    }
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // (Removed test_health_success — GET / liveness is no longer
    // wrapped; the real health artifact is exercised through
    // resource_action.)

    // ── Resource action (v5 routes) ────────────────────────

    #[test]
    fn test_resource_action_search() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/personas/search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"entries": [], "total_count": 0}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client.resource_action("personas", "search", None).unwrap();
        assert_eq!(result["total_count"], 0);
        mock.assert();
    }

    #[test]
    fn test_resource_action_with_body() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/scenarios/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"scenario_id": "s-1", "name": "Test"}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client
            .resource_action("scenarios", "get", Some(json!({"scenario_id": "s-1"})))
            .unwrap();
        assert_eq!(result["name"], "Test");
        mock.assert();
    }

    #[test]
    fn test_resource_action_attempt_start() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/attempt/start")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"attempt_id": "a-1"}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client
            .resource_action("attempt", "start", Some(json!({"scenario_id": "s-1"})))
            .unwrap();
        assert_eq!(result["attempt_id"], "a-1");
        mock.assert();
    }

    // (Removed test_connect / test_disconnect / test_context /
    // test_emulate / test_unemulate / test_problem — none of /connect,
    // /disconnect, /context, /emulate, /unemulate, /problem are real
    // top-level API routes. Per-artifact equivalents are exercised
    // via ``resource_action(api_path, action, ...)``.)

    // (Removed test_generate / test_generate_with_body /
    // test_stream_url_construction — top-level /generate and /stream
    // aren't real API routes. Per-artifact generate is exercised via
    // ``resource_action(api_path, "generate", ...)`` and per-artifact
    // watch is exercised via ``watch_run``.)

    // ── Per-resource media ────────────────────────────────────

    #[test]
    fn test_media_upload() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/documents/file/upload")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"upload_id": "up-1", "filename": "test.txt"}"#)
            .create();

        // Create a temp file
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, b"hello").unwrap();

        let client = GlowClient::new(&server.url());
        let result = client
            .media_upload("documents", "file", file_path.to_str().unwrap())
            .unwrap();
        assert_eq!(result["upload_id"], "up-1");
        mock.assert();
    }

    #[test]
    fn test_media_create_tus() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/documents/file/create")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"upload_id": "up-2", "upload_url": "/documents/file/up-2"}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client
            .media_create("documents", "file", "report.pdf", Some(1024))
            .unwrap();
        assert_eq!(result["upload_id"], "up-2");
        mock.assert();
    }

    #[test]
    fn test_media_discover() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/scenarios/video/discover")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"types": ["mp4", "webm"]}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client.media_discover("scenarios", "video", None).unwrap();
        assert_eq!(result["types"][0], "mp4");
        mock.assert();
    }

    #[test]
    fn test_media_status() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/documents/file/up-1/status")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"offset": 512, "length": 1024}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client.media_status("documents", "file", "up-1").unwrap();
        assert_eq!(result["offset"], 512);
        mock.assert();
    }

    #[test]
    fn test_media_finalize() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("POST", "/documents/file/up-1/finalize")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"finalized": true}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client
            .media_finalize("documents", "file", "up-1", None)
            .unwrap();
        assert_eq!(result["finalized"], true);
        mock.assert();
    }

    #[test]
    fn test_media_preview() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/documents/file/up-1/preview")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"columns": ["name"], "rows": 3}"#)
            .create();

        let client = GlowClient::new(&server.url());
        let result = client.media_preview("documents", "file", "up-1").unwrap();
        assert_eq!(result["rows"], 3);
        mock.assert();
    }

    // ── Error handling ───────────────────────────────────────

    #[test]
    fn test_401_returns_auth_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/personas/search")
            .with_status(401)
            .create();

        let client = GlowClient::new(&server.url());
        let err = client
            .resource_action("personas", "search", None)
            .unwrap_err();
        assert!(err.to_string().contains("Authentication failed"));
    }

    #[test]
    fn test_403_returns_permission_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/personas/search")
            .with_status(403)
            .create();

        let client = GlowClient::new(&server.url());
        let err = client
            .resource_action("personas", "search", None)
            .unwrap_err();
        assert!(err.to_string().contains("Permission denied"));
    }

    #[test]
    fn test_404_returns_not_found_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/personas/get")
            .with_status(404)
            .with_body("persona not found")
            .create();

        let client = GlowClient::new(&server.url());
        let err = client
            .resource_action("personas", "get", Some(json!({"persona_id": "x"})))
            .unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_500_returns_api_error() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("POST", "/personas/search")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let client = GlowClient::new(&server.url());
        let err = client
            .resource_action("personas", "search", None)
            .unwrap_err();
        assert!(err.to_string().contains("API error"));
    }

    #[test]
    fn test_connection_refused_returns_helpful_error() {
        let client = GlowClient::new("http://127.0.0.1:1");
        let err = client
            .resource_action("personas", "search", None)
            .unwrap_err();
        assert!(err.to_string().contains("Failed to connect"));
    }

    // ── SSE helper ───────────────────────────────────────────

    #[test]
    fn test_read_sse_events() {
        let sse_data = "event: message\ndata: hello\n\nevent: message\ndata: world\n\n";
        let cursor = std::io::Cursor::new(sse_data);

        // Simulate a Response-like reader
        let mut events = vec![];
        let reader = std::io::BufReader::new(cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("data: ") {
                events.push(line[6..].to_string());
            }
        }
        assert_eq!(events, vec!["hello", "world"]);
    }

    // ── Auth ──────────────────────────────────────────────────

    #[test]
    fn test_auth_none_when_no_credentials() {
        let client = GlowClient {
            base_url: "http://localhost".into(),
            http: blocking::Client::new(),
            token: None,
        };
        matches!(client.auth(), Auth::None);
    }

    #[test]
    fn test_auth_bearer() {
        let client = GlowClient {
            base_url: "http://localhost".into(),
            http: blocking::Client::new(),
            token: Some("tok".into()),
        };
        matches!(client.auth(), Auth::Bearer(_));
    }
}
