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

// ── groups (view, system-namespaced) ────────────────────────────

/// `glow groups history [--limit N] [--date-from ISO] [--date-to ISO]`
/// → POST /system/groups with { limit?, date_from?, date_to? }
///
/// Lives outside the Resource enum (groups is a system view, not a
/// CRUD artifact) so it gets its own top-level Clap subcommand.
pub fn cmd_groups_history(
    client: &GlowClient,
    limit: Option<u32>,
    date_from: Option<&str>,
    date_to: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let mut body = json!({});
    if let Some(n) = limit {
        body["page_size"] = json!(n);
    }
    if let Some(d) = date_from {
        body["date_from"] = json!(d);
    }
    if let Some(d) = date_to {
        body["date_to"] = json!(d);
    }
    let body_str = serde_json::to_string(&body)?;
    cmd_resource_action(client, "system", "groups", Some(&body_str), mode)
}

// Silence the unused-import warning when only a subset compiles.
#[allow(dead_code)]
fn _output_unused_marker(_m: OutputMode, _r: &serde_json::Value) {
    output::print_result(_m, _r, |_| {});
}
