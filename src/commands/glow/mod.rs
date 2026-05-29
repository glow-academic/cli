use crate::glow::GlowClient;
use crate::output::{self, OutputMode};
use anyhow::Result;

pub(crate) mod helpers;

// ── Generic resource action ──────────────────────────────────

pub(crate) fn cmd_resource_action(
    client: &GlowClient,
    resource: &str,
    action: &str,
    body_str: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    cmd_resource_action_ext(client, resource, action, body_str, mode, None, false, &[])
}

/// Extended dispatch — adds ``--file`` (multipart upload, with body
/// args folded into form fields) and ``--show-headers`` (print response
/// headers before the JSON body). The simpler ``cmd_resource_action``
/// remains the default entry point; this exists so ``lib.rs`` can route
/// based on the presence of those flags without churning every helper.
// Dispatch shim that mirrors the wire call's parameters (resource, action,
// body, file, headers, form fields); grouping them into a struct would add
// indirection without clarity, so allow the arg count here.
#[allow(clippy::too_many_arguments)]
pub(crate) fn cmd_resource_action_ext(
    client: &GlowClient,
    resource: &str,
    action: &str,
    body_str: Option<&str>,
    mode: OutputMode,
    file_path: Option<&str>,
    show_headers: bool,
    extra_form_fields: &[(String, String)],
) -> Result<()> {
    let body = match body_str {
        Some(s) => Some(
            serde_json::from_str::<serde_json::Value>(s)
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --body: {}", e))?,
        ),
        None => None,
    };

    if let Some(path) = file_path {
        // Multipart upload — body args (if any) are folded into form fields
        // alongside the file. Useful for soft/accept idempotency_key params
        // on upload endpoints, e.g.
        // ``glow attempts audio_upload --file rec.webm --soft true``.
        let response =
            client.resource_action_multipart(resource, action, path, extra_form_fields)?;
        output::print_result(mode, &response, |resp| {
            println!(
                "{}",
                serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
            );
        });
        return Ok(());
    }

    if show_headers {
        let (headers, response) = client.resource_action_with_headers(resource, action, body)?;
        // Headers first so the cache demo can call out X-Cache-Hit /
        // X-Bypass-Cache before the response body scrolls past.
        for (k, v) in headers.iter() {
            let key = k.as_str();
            if key.starts_with("x-")
                || key == "cache-control"
                || key == "content-type"
                || key == "content-length"
            {
                if let Ok(val) = v.to_str() {
                    println!("{}: {}", key, val);
                }
            }
        }
        println!();
        output::print_result(mode, &response, |resp| {
            println!(
                "{}",
                serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
            );
        });
        return Ok(());
    }

    let response = client.resource_action(resource, action, body)?;
    output::print_result(mode, &response, |resp| {
        // Known shapes (search tables, detail views, write/ack summaries)
        // get pretty output; everything else (and any unexpected/error
        // payload) falls back to the raw JSON.
        if !crate::render::render(resource, action, resp) {
            println!(
                "{}",
                serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
            );
        }
    });
    Ok(())
}

// ── Generate (per-artifact, with --wait) ──────────────────────
//
// ``cmd_context`` / ``cmd_emulate`` / ``cmd_unemulate`` removed
// in Cleanup D — context lives at POST /<art>/context on every
// artifact; emulate/unemulate exist only on /profile. Reach via
// generic dispatch (``glow profiles context``, ``glow profiles
// emulate <id>``, ``glow profiles unemulate``). Ergonomic shape
// for emulate lives in helpers::cmd_profile_emulate.

/// `glow <art> generate --wait [--body '{...}']` — trigger the
/// per-artifact generate (POST /<api_path>/generate) then block on
/// the watch SSE filtered to the returned run_id. Replaces the
/// removed top-level ``glow generate`` (which hit a non-existent
/// /generate route).
pub(crate) fn cmd_generate_and_wait_dispatch(
    client: &GlowClient,
    artifact_api_path: &str,
    body_str: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let mut body = match body_str {
        Some(s) => serde_json::from_str::<serde_json::Value>(s)
            .map_err(|e| anyhow::anyhow!("Invalid JSON for --body: {}", e))?,
        None => serde_json::json!({}),
    };

    // Force an ASYNC trigger. ``--wait`` blocks via the watch stream, so the
    // POST must return immediately (run_id while the run executes in the
    // background) — otherwise the trigger blocks server-side until the run
    // finishes (the default ``wait_for_complete: true``), and by the time we
    // attach to /watch the live, no-replay event hub has already gone quiet:
    // zero frames, and the watch hangs forever. Overriding to false is correct
    // for --wait regardless of what the caller put in --body.
    if let serde_json::Value::Object(ref mut map) = body {
        map.insert(
            "wait_for_complete".to_string(),
            serde_json::Value::Bool(false),
        );
    }

    // Trigger via the same generic POST the body-only path uses.
    let response = client.resource_action(artifact_api_path, "generate", Some(body))?;

    // Surface the trigger response first (run_id, group_id, etc.) so
    // the user has the handles even if the watch fails midway.
    let run_id = response
        .get("run_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let group_id = response
        .get("group_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    output::print_result(mode, &response, |resp| {
        if let Some(rid) = resp.get("run_id").and_then(|v| v.as_str()) {
            println!("{} run_id: {}", "OK".green().bold(), rid.dimmed());
        }
    });

    let Some(rid) = run_id else {
        anyhow::bail!("generate response had no ``run_id`` to watch");
    };
    eprintln!(
        "{} Watching /{}/watch run_id={} …",
        "·".dimmed(),
        artifact_api_path,
        rid
    );
    cmd_watch_run(client, artifact_api_path, &rid, group_id.as_deref(), mode)
}

/// Stream the per-artifact watch endpoint scoped to a single run,
/// printing event payloads and blocking until a terminal lifecycle
/// event (``.completed`` or ``.failed``) arrives — or the stream
/// closes naturally.
///
/// ``artifact_api_path`` is the singular API path (``attempt``,
/// ``scenario``, ...). Terminal detection runs on the SSE ``event:``
/// name suffix so we don't need to deserialize the payload.
pub(crate) fn cmd_watch_run(
    client: &GlowClient,
    artifact_api_path: &str,
    run_id: &str,
    group_id: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let response = client.watch_run(artifact_api_path, run_id, group_id)?;

    // Track whether we saw a terminal event so the exit code can
    // reflect it: 0 on success/completed, 1 on failed. Stream-closed
    // without a terminal event is also success (the API may end the
    // stream when its dedup/age window expires).
    let mut terminal_kind: Option<TerminalKind> = None;

    crate::glow::read_sse_events_with_names(response, |event_name, data| {
        // Print per-frame output. JSON mode: one JSON object per line
        // (event name + parsed payload). Pretty mode: a colored
        // header + indented data preview.
        match mode {
            OutputMode::Json => {
                let parsed: serde_json::Value = serde_json::from_str(data)
                    .unwrap_or(serde_json::Value::String(data.to_string()));
                let frame = serde_json::json!({
                    "event": event_name,
                    "data": parsed,
                });
                println!("{}", serde_json::to_string(&frame).unwrap_or_default());
            }
            OutputMode::Human => {
                let label = if event_name.is_empty() {
                    "(event)".dimmed().to_string()
                } else {
                    event_name.bold().to_string()
                };
                println!("{}", label);
                // Indent the data payload for readability without
                // pretty-printing huge nested JSON (one line is plenty
                // for live tail).
                println!("  {}", data.dimmed());
            }
        }

        // Terminal detection keys on the envelope's ``event_type`` field
        // inside the payload — NOT the SSE ``event:`` name, which our
        // server never sets (every frame rides the default ``message``
        // channel; ``event_type`` lives in the JSON so the client can
        // wildcard-match — see the API's infra/stream/sse.py).
        //
        // The set mirrors the server's run-scoped terminal (``_is_run_terminal``
        // in sse.py): the run's FINAL frame only. We deliberately do NOT match
        // the bare ``.complete`` suffix — the generation lifecycle emits mid-run
        // ``call.complete`` / ``text.complete`` frames, and matching those would
        // declare the run done after the first tool call.
        let et = serde_json::from_str::<serde_json::Value>(data)
            .ok()
            .and_then(|v| {
                v.get("event_type")
                    .and_then(|e| e.as_str())
                    .map(String::from)
            })
            .unwrap_or_default();
        // Suffix match, NOT equality: event_type is fully-qualified
        // (``persona.generate.agent_completed``), so compare the tail.
        if et.ends_with(".failed") || et.ends_with(".error") || et.ends_with("media_error") {
            terminal_kind = Some(TerminalKind::Failed);
            return std::ops::ControlFlow::Break(());
        }
        if et.ends_with(".completed")
            || et.ends_with("agent_completed")
            || et.ends_with("media_complete")
        {
            terminal_kind = Some(TerminalKind::Completed);
            return std::ops::ControlFlow::Break(());
        }
        std::ops::ControlFlow::Continue(())
    })?;

    match terminal_kind {
        Some(TerminalKind::Completed) => {
            println!("{} Run completed.", "✓".green());
            Ok(())
        }
        Some(TerminalKind::Failed) => {
            anyhow::bail!("Run failed (see the last event payload above).");
        }
        None => {
            println!(
                "{} Stream closed without a terminal lifecycle event.",
                "·".dimmed()
            );
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TerminalKind {
    Completed,
    Failed,
}

// ``cmd_connect`` / ``cmd_disconnect`` removed — hit non-existent
// /connect and /disconnect top-level routes. There's no session-scoped
// stream concept on the API; per-artifact watch is the canonical
// "give me events for this run" surface.

// ``cmd_problem`` removed in Cleanup D — POST /problem isn't a real
// route. Problem reporting lives per-artifact at POST /<art>/problem;
// reach via generic dispatch on any artifact, e.g.
// ``glow system problem --body '{"type":"bug","message":"..."}'``.

// ``cmd_stream`` removed — hit a non-existent /stream top-level route.
// Per-artifact streaming is ``glow <art> watch <run_id>``, which
// uses ``cmd_watch_run`` above and hits the real ``/<art>/watch`` SSE
// endpoint.

// ── Per-resource media operations ────────────────────────────

pub(crate) fn cmd_media_upload(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    file_path: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let response = client.media_upload(resource, media_type, file_path)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{} Uploaded to {}/{}",
            "OK".green().bold(),
            resource,
            media_type
        );
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
    Ok(())
}

pub(crate) fn cmd_media_download(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    upload_id: &str,
    output_path: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let bytes = client.media_download(resource, media_type, upload_id)?;
    match output_path {
        Some(path) => {
            std::fs::write(path, &bytes)
                .map_err(|e| anyhow::anyhow!("Failed to write to {}: {}", path, e))?;
            if mode == OutputMode::Human {
                println!(
                    "{} Downloaded {} bytes to {}",
                    "OK".green().bold(),
                    bytes.len(),
                    path,
                );
            }
        }
        None => {
            use std::io::Write;
            std::io::stdout().write_all(&bytes)?;
        }
    }
    Ok(())
}

pub(crate) fn cmd_media_discover(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    upload_id: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let response = client.media_discover(resource, media_type, upload_id)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
    Ok(())
}

pub(crate) fn cmd_media_preview(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    upload_id: &str,
    mode: OutputMode,
) -> Result<()> {
    let response = client.media_preview(resource, media_type, upload_id)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
    Ok(())
}

// ``cmd_health`` removed in Cleanup E — the top-level command hit
// GET / (root liveness), not /health. The real health artifact is
// reached via generic dispatch on ``glow system health``.
