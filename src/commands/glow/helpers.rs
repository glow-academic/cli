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

/// `glow attempts start <simulation_id> [--persona <id>] [--cohort <id>]`
/// → POST /attempt/start with body { simulation_id, persona_id?, cohort_id? }
pub fn cmd_attempt_start(
    client: &GlowClient,
    simulation_id: &str,
    persona_id: Option<&str>,
    cohort_id: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let mut body = json!({ "simulation_id": simulation_id });
    if let Some(p) = persona_id {
        body["persona_id"] = json!(p);
    }
    if let Some(c) = cohort_id {
        body["cohort_id"] = json!(c);
    }
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "attempt", "start", Some(&body_str), mode)
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
                .or_else(|| {
                    resp.get("resource_id").and_then(|v| v.as_str())
                });
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

/// `glow attempts grade <chat_id> [--score N]`
/// → POST /attempt/chat_grade with { chat_id, score? }
pub fn cmd_attempt_grade(
    client: &GlowClient,
    chat_id: &str,
    score: Option<u32>,
    mode: OutputMode,
) -> Result<()> {
    let mut body = json!({ "chat_id": chat_id });
    if let Some(s) = score {
        body["score"] = json!(s);
    }
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "attempt", "chat_grade", Some(&body_str), mode)
}

// ── scenarios ───────────────────────────────────────────────────

/// `glow scenarios generate <modality> <title> [--instructions <text>]`
/// → POST /scenario/generate with { modality, title, instructions? }
///
/// Matches the Scenario_Generate tool's args contract — modality is one
/// of image/video; title becomes the asset's display name; instructions
/// drives the generation prompt + becomes the asset description.
pub fn cmd_scenario_generate(
    client: &GlowClient,
    modality: &str,
    title: &str,
    instructions: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let mut body = json!({ "modality": modality, "title": title });
    if let Some(i) = instructions {
        body["instructions"] = json!(i);
    }
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "scenario", "generate", Some(&body_str), mode)
}

// ── personas ────────────────────────────────────────────────────

/// `glow personas search [--name <pattern>] [--page-size N] [--page-offset N]`
/// → POST /persona/search with { name?, page_size?, page_offset? }
pub fn cmd_personas_search(
    client: &GlowClient,
    name: Option<&str>,
    page_size: Option<u32>,
    page_offset: Option<u32>,
    mode: OutputMode,
) -> Result<()> {
    let mut body = json!({});
    if let Some(n) = name {
        body["name"] = json!(n);
    }
    if let Some(sz) = page_size {
        body["page_size"] = json!(sz);
    }
    if let Some(off) = page_offset {
        body["page_offset"] = json!(off);
    }
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "persona", "search", Some(&body_str), mode)
}

// ── simulations ─────────────────────────────────────────────────

/// `glow simulations list [--cohort <id>] [--page-size N]` — sugar for search.
/// → POST /simulation/search with { cohort_ids?, page_size? }
pub fn cmd_simulations_list(
    client: &GlowClient,
    cohort: Option<&str>,
    page_size: Option<u32>,
    mode: OutputMode,
) -> Result<()> {
    let mut body = json!({});
    if let Some(c) = cohort {
        body["cohort_ids"] = json!([c]);
    }
    if let Some(sz) = page_size {
        body["page_size"] = json!(sz);
    }
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "simulation", "search", Some(&body_str), mode)
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
    use std::io::{BufRead, Write};
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

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        // Drain any pending inbound events first (non-blocking) so
        // the user sees responses to the previous message before
        // their next prompt.
        while let Some((name, payload)) =
            crate::glow::ws::try_recv_with_timeout(&sock.events, Duration::from_millis(50))
        {
            println!(
                "{}: {}",
                name.green().bold(),
                serde_json::to_string(&payload).unwrap_or_default().dimmed(),
            );
        }

        // Prompt for the next user message.
        print!("{} ", ">".bold());
        stdout.flush().ok();
        let mut line = String::new();
        let n = stdin
            .lock()
            .read_line(&mut line)
            .context("Failed to read stdin")?;
        if n == 0 {
            // EOF — clean disconnect.
            break;
        }
        let text = line.trim().to_string();
        if text.is_empty() {
            continue;
        }
        if text == ":quit" || text == ":q" {
            break;
        }

        // Build the chat_message payload. Plural ``audios_id`` is
        // intentionally absent — voice is on the deferred Phase 5
        // sub-task. ``persona_id`` is forwarded when supplied so
        // the user can attribute messages on multi-persona chats.
        let mut payload = json!({ "chat_id": chat_id, "text": text });
        if let Some(p) = persona_id {
            payload["persona_id"] = json!(p);
        }

        if let Err(e) = sock.emit("attempt.chat_message", payload) {
            eprintln!("{} emit failed: {}", "·".red(), e);
        }
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
    let _ = client; // base_url has the bearer-aware path; keep client param for symmetry
    let bearer = crate::auth::get_token(base_url).ok().map(|t| t.access_token);
    let url = format!("{}/mcp/", base_url.trim_end_matches('/'));
    let mut req = reqwest::blocking::Client::new()
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .json(&envelope);
    if let Some(t) = bearer {
        req = req.header("Authorization", format!("Bearer {}", t));
    }
    let resp = req
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
    let body = if text.trim_start().starts_with("event:")
        || text.trim_start().starts_with("data:")
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

pub fn cmd_mcp_list_tools(
    client: &GlowClient,
    base_url: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;
    let resp = mcp_jsonrpc(client, base_url, "tools/list", json!({}))?;
    let tools = resp
        .pointer("/result/tools")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    match mode {
        OutputMode::Json => {
            println!("{}", serde_json::to_string_pretty(&tools).unwrap_or_default());
        }
        OutputMode::Human => {
            if let Some(arr) = tools.as_array() {
                println!("{} {} MCP tools:", "·".dimmed(), arr.len());
                for t in arr {
                    let name = t.get("name").and_then(|v| v.as_str()).unwrap_or("(unnamed)");
                    let desc = t
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let first_line = desc.lines().next().unwrap_or("");
                    println!("  {:30} {}", name.bold(), first_line.dimmed());
                }
            } else {
                // Unexpected shape — surface the whole response.
                println!("{}", serde_json::to_string_pretty(&resp).unwrap_or_default());
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
            println!("{}", serde_json::to_string_pretty(&result).unwrap_or_default());
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
                println!("{}", serde_json::to_string_pretty(&result).unwrap_or_default());
            }
        }
    }
    Ok(())
}

/// Placeholder for the voice REPL. Prints a clear deferral message
/// with the suggested next-step so the user knows the WS scaffold
/// is ready and what remains gated on their go-ahead.
pub fn cmd_chats_voice_placeholder(chat_id: &str) -> Result<()> {
    use colored::Colorize;
    eprintln!(
        "{} ``glow chats voice {}`` is deferred.",
        "·".yellow().bold(),
        chat_id,
    );
    eprintln!("  The Phase-5 voice REPL needs ``cpal`` (mic capture) +");
    eprintln!("  ``rodio`` (playback) — both pull native deps (CoreAudio /");
    eprintln!("  ALSA / WASAPI). The goal command explicitly gates these on");
    eprintln!("  user confirmation, so the WS layer is in place but this");
    eprintln!("  command is a no-op until that gate clears.");
    eprintln!();
    eprintln!("  Path forward: confirm the deps, then add the mic→upload→");
    eprintln!("  emit→playback loop on top of ``glow::ws::GlowSocket`` —");
    eprintln!("  the wire-up is straightforward once the deps land.");
    Ok(())
}
