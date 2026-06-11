// commands/glow/helpers.rs
//
// Per-artifact ergonomic helpers — positional + flag args that build
// the JSON body the API expects, so users don't have to remember the
// exact field names or hand-write ``--body '{...}'`` for the common
// flows.
//
// Each helper dispatches through the same ``cmd_resource_action``
// generic POST so output formatting / auth stay consistent. The
// helpers are intercepted in ``dispatch_resource`` (lib.rs) when the
// (resource, action) pair matches a known shortcut; anything else
// falls through to the generic ``--body``-driven dispatch.

use crate::glow::GlowClient;
use crate::output::{self, OutputMode};
use anyhow::{Context, Result};
use serde_json::json;

use super::cmd_resource_action;

// ── Common helpers ──────────────────────────────────────────────

/// Strip a leading "--" prefix and convert ``--foo-bar`` → ``foo_bar``
/// so flag-style keys can drop into snake_case JSON without churn.
#[allow(dead_code)]
fn flag_to_field(flag: &str) -> String {
    flag.trim_start_matches("--").replace('-', "_")
}

// ── attempts ────────────────────────────────────────────────────

/// `glow attempts start (--home <id> | --practice <id>) [--infinite-mode]`
/// → POST /attempt/start with body { home_id | practice_id, infinite_mode }
///
/// An attempt is always created from a *home* entry (an assigned simulation)
/// or a *practice* entry (a practice-available simulation) — the API's
/// ``AttemptStartRequest`` accepts ONLY ``home_id`` / ``practice_id`` (plus
/// ``infinite_mode``) and resolves the simulation, persona and chats from that
/// parent entry itself. The previous shape (``simulation_id`` / ``persona_id``
/// / ``cohort_id``) was wrong: every field was silently dropped by the
/// ``extra="ignore"`` model, leaving both required ids None → 400 "Either
/// home_id or practice_id is required." The home/practice ids are obtained via
/// ``glow homes search`` / ``glow practices search`` (same as the client). One
/// of ``--home`` / ``--practice`` is required.
pub fn cmd_attempt_start(
    client: &GlowClient,
    home_id: Option<&str>,
    practice_id: Option<&str>,
    infinite_mode: bool,
    mode: OutputMode,
) -> Result<()> {
    let body = build_attempt_start_body(home_id, practice_id, infinite_mode)?;
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "attempt", "start", Some(&body_str), mode)
}

/// Build the `/attempt/start` body, enforcing exactly one of home/practice.
/// ``infinite_mode`` is only honoured server-side for practice attempts but is
/// harmless on home starts, so we always include it (mirrors the web client).
fn build_attempt_start_body(
    home_id: Option<&str>,
    practice_id: Option<&str>,
    infinite_mode: bool,
) -> Result<serde_json::Value> {
    match (home_id, practice_id) {
        (Some(_), Some(_)) => anyhow::bail!(
            "Pass only one of --home / --practice — an attempt starts from a single home or practice entry."
        ),
        (None, None) => anyhow::bail!(
            "`glow attempts start` requires --home <home_id> or --practice <practice_id>. \
             Find ids via `glow homes search` / `glow practices search`."
        ),
        (Some(h), None) => Ok(json!({ "home_id": h, "infinite_mode": infinite_mode })),
        (None, Some(p)) => Ok(json!({ "practice_id": p, "infinite_mode": infinite_mode })),
    }
}

/// `glow attempts message <chat_id> <text> [--audio <file>] [--persona <id>]`
/// → optional audio upload via /attempt/audio/upload (multipart), then
///   POST /attempt/chat_message with { chat_id, text, persona_id?, audios_id? }.
///
/// The ``audios_id`` field uses the canonical plural form — server side
/// is what we standardized on in the recent voice-pipeline work; the
/// audio_uploads/audios_resource promotion runs inside ``media_upload``.
pub fn cmd_attempt_message(
    client: &GlowClient,
    chat_id: &str,
    text: &str,
    audio_file: Option<&str>,
    persona_id: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let audios_id = match audio_file {
        Some(path) => {
            // Upload via the canonical media-upload pipeline (multipart
            // POST → uploads_entry → audios_resource → entry → links).
            // The response carries ``audios_id`` (plural = resource
            // handle) on success.
            let resp = client
                .media_upload("attempt", "audio", path)
                .with_context(|| format!("Failed to upload audio file: {}", path))?;
            // Defensive: the response shape may surface either
            // ``audios_id`` (canonical) or ``audio_id`` (legacy) — try
            // both before giving up so we don't break on older API
            // builds.
            let id = resp
                .get("audios_id")
                .and_then(|v| v.as_str())
                .or_else(|| resp.get("audio_id").and_then(|v| v.as_str()))
                .or_else(|| resp.get("resource_id").and_then(|v| v.as_str()));
            match id {
                Some(s) => Some(s.to_string()),
                None => anyhow::bail!(
                    "Audio upload succeeded but the response had no audios_id / audio_id / resource_id field. Raw response: {}",
                    serde_json::to_string(&resp).unwrap_or_default()
                ),
            }
        }
        None => None,
    };

    let mut body = json!({ "chat_id": chat_id, "text": text });
    if let Some(p) = persona_id {
        body["persona_id"] = json!(p);
    }
    if let Some(a) = audios_id {
        body["audios_id"] = json!(a);
    }
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "attempt", "chat_message", Some(&body_str), mode)
}

/// `glow attempts grade <chat_id> --score <N>`
/// → POST /attempt/chat_grade with { chat_id, score }
///
/// ``GradeAttemptApiRequest.score`` is REQUIRED (``int = Field(...)``); sending
/// the body without it yields an opaque 422. ``--score`` is therefore required
/// at the CLI layer (enforced in the dispatcher) so the user gets a clear
/// message instead of a server validation error.
pub fn cmd_attempt_grade(
    client: &GlowClient,
    chat_id: &str,
    score: u32,
    mode: OutputMode,
) -> Result<()> {
    let body = json!({ "chat_id": chat_id, "score": score });
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "attempt", "chat_grade", Some(&body_str), mode)
}

// ── scenarios ───────────────────────────────────────────────────

/// `glow scenarios generate <modality> [--instructions <text>]`
/// → POST /scenario/generate with { modalities: [modality], instructions?: [text] }
///
/// Matches ``ArtifactGenerateRequest``: ``modalities`` is a LIST of output
/// modalities (e.g. ["image"], ["video"], ["text"]) and ``instructions`` is a
/// LIST of free-text prompt lines. There is NO ``title`` field — the asset name
/// is derived server-side — so the old singular ``modality``/``title`` shape was
/// broken (``modality`` was dropped as a server-internal alias, ``title`` was
/// ignored, and a string ``instructions`` failed list validation).
pub fn cmd_scenario_generate(
    client: &GlowClient,
    modality: &str,
    instructions: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let body = build_scenario_generate_body(modality, instructions);
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "scenario", "generate", Some(&body_str), mode)
}

/// Build the `/scenario/generate` body with list-typed `modalities`/`instructions`.
fn build_scenario_generate_body(modality: &str, instructions: Option<&str>) -> serde_json::Value {
    let mut body = json!({ "modalities": [modality] });
    if let Some(i) = instructions {
        body["instructions"] = json!([i]);
    }
    body
}

// ── personas ────────────────────────────────────────────────────

/// `glow personas search [--name <pattern>] [--page-size N] [--page-offset N]`
/// → POST /persona/search with { search?, page_size?, page_offset? }
///
/// ``SearchPersonaApiRequest`` names the full-text field ``search`` (there is no
/// ``name`` field), so the CLI's user-facing ``--name`` flag maps to the
/// ``search`` body key. Sending ``name`` was silently dropped by ``extra="ignore"``.
pub fn cmd_personas_search(
    client: &GlowClient,
    name: Option<&str>,
    page_size: Option<u32>,
    page_offset: Option<u32>,
    mode: OutputMode,
) -> Result<()> {
    let body = build_personas_search_body(name, page_size, page_offset);
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "persona", "search", Some(&body_str), mode)
}

/// Build the `/persona/search` body, mapping the `--name` flag to `search`.
fn build_personas_search_body(
    name: Option<&str>,
    page_size: Option<u32>,
    page_offset: Option<u32>,
) -> serde_json::Value {
    let mut body = json!({});
    if let Some(n) = name {
        body["search"] = json!(n);
    }
    if let Some(sz) = page_size {
        body["page_size"] = json!(sz);
    }
    if let Some(off) = page_offset {
        body["page_offset"] = json!(off);
    }
    body
}

// ── generic get (positional id) ─────────────────────────────────

/// `glow <artifact> get <id>` → POST /<art>/get with `{ <id_field>: id }`.
///
/// The detail endpoints take the id in the body, but the generic dispatch
/// can't carry it: ``--id`` is reserved for media ops (it's stripped from the
/// body in ``parse_params``), so without this helper users had to hand-write
/// ``--body '{"id":"…"}'``. ``id_field`` is ``attempt_id`` for attempts and
/// plain ``id`` for the content artifacts.
pub fn cmd_resource_get(
    client: &GlowClient,
    api_path: &str,
    id_field: &str,
    id: &str,
    mode: OutputMode,
) -> Result<()> {
    let body = json!({ id_field: id });
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, api_path, "get", Some(&body_str), mode)
}

// ── simulations ─────────────────────────────────────────────────

/// `glow simulations list [--cohort <id>] [--page-size N]` — sugar for search.
/// → POST /simulation/search with { filter_cohort_ids?, page_size? }
///
/// ``SearchSimulationApiRequest`` names the cohort filter ``filter_cohort_ids``
/// (a list); the CLI previously sent ``cohort_ids``, which ``extra="ignore"``
/// silently dropped → the ``--cohort`` filter was a no-op.
pub fn cmd_simulations_list(
    client: &GlowClient,
    cohort: Option<&str>,
    page_size: Option<u32>,
    mode: OutputMode,
) -> Result<()> {
    let body = build_simulations_list_body(cohort, page_size);
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "simulation", "search", Some(&body_str), mode)
}

/// Build the `/simulation/search` body, mapping `--cohort` to `filter_cohort_ids`.
fn build_simulations_list_body(cohort: Option<&str>, page_size: Option<u32>) -> serde_json::Value {
    let mut body = json!({});
    if let Some(c) = cohort {
        body["filter_cohort_ids"] = json!([c]);
    }
    if let Some(sz) = page_size {
        body["page_size"] = json!(sz);
    }
    body
}

// ── profiles emulate / unemulate ────────────────────────────────

/// `glow profiles emulate <profile_id> [--ttl-minutes N]`
/// → POST /profile/emulate { target_profile_id, ttl_minutes }
///
/// Emulation only exists on the profile artifact on the API; this is
/// the ergonomic shape that replaced the (broken) top-level
/// ``glow emulate`` command in Cleanup D.
pub fn cmd_profile_emulate(
    client: &GlowClient,
    target_profile_id: &str,
    ttl_minutes: u32,
    mode: OutputMode,
) -> Result<()> {
    let body = json!({
        "target_profile_id": target_profile_id,
        "ttl_minutes": ttl_minutes,
    });
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "profile", "emulate", Some(&body_str), mode)
}

/// `glow profiles unemulate` → POST /profile/unemulate {}
pub fn cmd_profile_unemulate(client: &GlowClient, mode: OutputMode) -> Result<()> {
    cmd_resource_action(client, "profile", "unemulate", Some("{}"), mode)
}

// ``cmd_groups_history`` removed in Cleanup B — now reachable as
// ``glow system groups [--body '{"page_size":N,"date_from":...}']``
// via the generic dispatch since System is back in the resource
// macro. If a sugary positional shape is wanted later, add it as
// a (Resource::System, "groups") helper intercept.

// Silence the unused-import warning when only a subset compiles.
#[allow(dead_code)]
fn _output_unused_marker(_m: OutputMode, _r: &serde_json::Value) {
    output::print_result(_m, _r, |_| {});
}

// ── chats live (socket.io REPL) ─────────────────────────────────

/// `glow chats live <chat_id> [--persona <id>]` — opens a socket.io
/// connection and streams a chat REPL over it. Reads stdin lines on
/// the main thread, emits ``attempt.chat_message`` per line. Inbound
/// events are forwarded to a channel by rust_socketio's handler
/// thread; we drain the channel between user inputs and print frames.
///
/// **Untested against a live server** — see comment in src/glow/ws.rs.
/// The WS layer compiles, the REPL loop is structurally sound, but no
/// smoke-test against a running glow-academic-api has been run.
pub fn cmd_chats_live(
    client: &GlowClient,
    base_url: &str,
    bearer: Option<&str>,
    chat_id: &str,
    persona_id: Option<&str>,
    _mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;
    use std::io::Write;
    use std::time::Duration;

    // ``client`` parameter is reserved for future fall-back HTTP
    // dispatch (e.g. surface chat history before opening the live
    // stream); we accept it now so the dispatcher signature doesn't
    // churn later.
    let _ = client;

    eprintln!("{} connecting socket.io to {} …", "·".dimmed(), base_url);
    let sock = crate::glow::ws::GlowSocket::connect(base_url, bearer)
        .with_context(|| format!("Failed to open socket.io to {}", base_url))?;

    eprintln!(
        "{} chat REPL: type a message + Enter to send. Ctrl-D / Ctrl-C to quit.",
        "·".dimmed()
    );

    let mut stdout = std::io::stdout();

    // Read stdin on a dedicated thread and forward lines over a channel.
    // The previous design blocked the main loop on ``read_line``, so inbound
    // events (the assistant's reply) only printed AFTER the next Enter — they
    // never "streamed back inline". Decoupling stdin lets the main loop drain
    // and print inbound events continuously while waiting for input. ``None``
    // is the EOF/quit sentinel.
    let (line_tx, line_rx) = std::sync::mpsc::channel::<Option<String>>();
    std::thread::spawn(move || {
        use std::io::BufRead as _;
        let stdin = std::io::stdin();
        loop {
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(0) => {
                    let _ = line_tx.send(None);
                    break;
                }
                Ok(_) => {
                    if line_tx.send(Some(line)).is_err() {
                        break;
                    }
                }
                Err(_) => {
                    let _ = line_tx.send(None);
                    break;
                }
            }
        }
    });

    print!("{} ", ">".bold());
    stdout.flush().ok();

    loop {
        // Drain inbound events continuously (not just between prompts) so the
        // assistant's reply streams back inline as it arrives.
        let mut printed_event = false;
        while let Some((name, payload)) =
            crate::glow::ws::try_recv_with_timeout(&sock.events, Duration::from_millis(10))
        {
            // Keep the live view readable: surface the message ``text`` when
            // present (the chat bubble), else a compact one-line summary —
            // never the full tool-definition blob the raw payload carries.
            let text = payload.get("text").and_then(|t| t.as_str());
            match text {
                Some(t) => println!("\n{}  {}", name.green().bold(), t),
                None => println!("\n{}", name.green().bold()),
            }
            printed_event = true;
        }
        if printed_event {
            print!("{} ", ">".bold());
            stdout.flush().ok();
        }

        // Non-blocking check for a typed line.
        match line_rx.try_recv() {
            Ok(None) => break, // EOF — clean disconnect.
            Ok(Some(line)) => {
                let text = line.trim().to_string();
                if text == ":quit" || text == ":q" {
                    break;
                }
                if !text.is_empty() {
                    // Text-only REPL; ``persona_id`` is forwarded when supplied
                    // so messages can be attributed on multi-persona chats.
                    let mut payload = json!({ "chat_id": chat_id, "text": text });
                    if let Some(p) = persona_id {
                        payload["persona_id"] = json!(p);
                    }
                    if let Err(e) = sock.emit("attempt.chat_message", payload) {
                        eprintln!("{} emit failed: {}", "·".red(), e);
                    }
                }
                print!("{} ", ">".bold());
                stdout.flush().ok();
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
        }

        std::thread::sleep(Duration::from_millis(40));
    }

    eprintln!("{} closing chat REPL", "·".dimmed());
    sock.disconnect().ok();
    Ok(())
}

// ── MCP (JSON-RPC over POST /mcp/) ──────────────────────────────

/// Minimal MCP JSON-RPC call. Sends a single request to ``/mcp/``,
/// returns the parsed JSON-RPC response. The API mounts FastMCP at
/// ``/mcp/`` — POST takes a JSON-RPC 2.0 envelope, bearer header
/// flows through the existing auth.
///
/// This is intentionally NOT a full MCP client (no session handshake,
/// no SSE for server→client streaming, no capability negotiation)
/// because the common use cases (``tools/list``, ``tools/call``) work
/// fine as standalone POSTs against FastMCP. If we ever need
/// notifications / progress / subscribe, drop in a real MCP crate.
fn mcp_jsonrpc(
    client: &GlowClient,
    base_url: &str,
    method: &str,
    params: serde_json::Value,
) -> Result<serde_json::Value> {
    let envelope = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params,
    });
    // Route through the client's env-aware auth so the bypass token
    // (GLOW_TOKEN) + X-E2E-Profile-Id reach /mcp/ exactly as they do for
    // resource endpoints — not the stored-login token from `glow login`.
    let url = format!("{}/mcp/", base_url.trim_end_matches('/'));
    let resp = client
        .authed_request(reqwest::Method::POST, &url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .json(&envelope)
        .send()
        .with_context(|| format!("Failed to POST {}", url))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        anyhow::bail!("MCP request failed (HTTP {}): {}", status, text);
    }
    let text = resp.text().context("Failed to read MCP response body")?;
    // FastMCP may respond with either JSON or SSE-framed JSON depending
    // on the Accept header. When the body starts with ``event:`` /
    // ``data:`` we strip the SSE framing and parse the first data
    // frame; otherwise we parse the body directly.
    let body = if text.trim_start().starts_with("event:") || text.trim_start().starts_with("data:")
    {
        let mut payload = String::new();
        for line in text.lines() {
            if let Some(d) = line.strip_prefix("data: ") {
                payload.push_str(d);
                break;
            }
        }
        payload
    } else {
        text
    };
    serde_json::from_str::<serde_json::Value>(&body)
        .with_context(|| format!("MCP response was not JSON: {}", body))
}

pub fn cmd_mcp_list_tools(client: &GlowClient, base_url: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    let resp = mcp_jsonrpc(client, base_url, "tools/list", json!({}))?;
    let tools = resp
        .pointer("/result/tools")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    match mode {
        OutputMode::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&tools).unwrap_or_default()
            );
        }
        OutputMode::Human => {
            if let Some(arr) = tools.as_array() {
                println!("{} {} MCP tools:", "·".dimmed(), arr.len());
                for t in arr {
                    let name = t
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("(unnamed)");
                    let desc = t.get("description").and_then(|v| v.as_str()).unwrap_or("");
                    let first_line = desc.lines().next().unwrap_or("");
                    println!("  {:30} {}", name.bold(), first_line.dimmed());
                }
            } else {
                // Unexpected shape — surface the whole response.
                println!(
                    "{}",
                    serde_json::to_string_pretty(&resp).unwrap_or_default()
                );
            }
        }
    }
    Ok(())
}

pub fn cmd_mcp_call(
    client: &GlowClient,
    base_url: &str,
    tool_name: &str,
    args_json: &str,
    mode: OutputMode,
) -> Result<()> {
    let arguments: serde_json::Value = serde_json::from_str(args_json)
        .with_context(|| format!("--args must be valid JSON. Got: {}", args_json))?;
    let resp = mcp_jsonrpc(
        client,
        base_url,
        "tools/call",
        json!({ "name": tool_name, "arguments": arguments }),
    )?;
    let result = resp
        .pointer("/result")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    match mode {
        OutputMode::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&result).unwrap_or_default()
            );
        }
        OutputMode::Human => {
            // MCP tool results have shape ``{ content: [{type, text}, ...], isError? }``.
            if let Some(content) = result.get("content").and_then(|v| v.as_array()) {
                for block in content {
                    if let Some(t) = block.get("text").and_then(|v| v.as_str()) {
                        println!("{}", t);
                    } else {
                        println!("{}", serde_json::to_string(block).unwrap_or_default());
                    }
                }
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_default()
                );
            }
        }
    }
    Ok(())
}

// ── request-serialization contract tests ────────────────────────
//
// These assert the EXACT field names the helpers serialize, pinning them
// to the API's Pydantic request models. The API uses ``extra="ignore"``,
// so a drifted field name is silently dropped (no error) — only a test
// that inspects the wire body can catch it. Field references:
//   attempt/start    → AttemptStartRequest (home_id | practice_id, infinite_mode)
//   persona/search   → SearchPersonaApiRequest.search
//   simulation/search→ SearchSimulationApiRequest.filter_cohort_ids
//   scenario/generate→ ArtifactGenerateRequest.modalities/.instructions (lists)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_start_home_body_uses_home_id() {
        let body = build_attempt_start_body(Some("h-1"), None, false).unwrap();
        assert_eq!(body["home_id"], json!("h-1"));
        assert_eq!(body["infinite_mode"], json!(false));
        assert!(body.get("practice_id").is_none());
        // The old drifted keys must not appear.
        assert!(body.get("simulation_id").is_none());
        assert!(body.get("persona_id").is_none());
        assert!(body.get("cohort_id").is_none());
    }

    #[test]
    fn attempt_start_practice_body_uses_practice_id_and_infinite() {
        let body = build_attempt_start_body(None, Some("p-1"), true).unwrap();
        assert_eq!(body["practice_id"], json!("p-1"));
        assert_eq!(body["infinite_mode"], json!(true));
        assert!(body.get("home_id").is_none());
    }

    #[test]
    fn attempt_start_requires_exactly_one_parent() {
        assert!(build_attempt_start_body(None, None, false).is_err());
        assert!(build_attempt_start_body(Some("h"), Some("p"), false).is_err());
    }

    #[test]
    fn personas_search_maps_name_to_search() {
        let body = build_personas_search_body(Some("ada"), Some(5), Some(10));
        assert_eq!(body["search"], json!("ada"));
        assert_eq!(body["page_size"], json!(5));
        assert_eq!(body["page_offset"], json!(10));
        // Drifted ``name`` key must not be present.
        assert!(body.get("name").is_none());
    }

    #[test]
    fn simulations_list_maps_cohort_to_filter_cohort_ids() {
        let body = build_simulations_list_body(Some("c-1"), Some(3));
        assert_eq!(body["filter_cohort_ids"], json!(["c-1"]));
        assert_eq!(body["page_size"], json!(3));
        // Drifted ``cohort_ids`` key must not be present.
        assert!(body.get("cohort_ids").is_none());
    }

    #[test]
    fn scenario_generate_uses_list_modalities_and_instructions() {
        let body = build_scenario_generate_body("image", Some("a cat"));
        assert_eq!(body["modalities"], json!(["image"]));
        assert_eq!(body["instructions"], json!(["a cat"]));
        // No singular ``modality`` and no ``title`` field on the model.
        assert!(body.get("modality").is_none());
        assert!(body.get("title").is_none());
    }

    #[test]
    fn scenario_generate_omits_instructions_when_absent() {
        let body = build_scenario_generate_body("video", None);
        assert_eq!(body["modalities"], json!(["video"]));
        assert!(body.get("instructions").is_none());
    }
}
