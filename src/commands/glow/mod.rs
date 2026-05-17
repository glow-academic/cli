use crate::glow::GlowClient;
use crate::output::{self, OutputMode};
use anyhow::Result;

// ── Generic resource action ──────────────────────────────────

pub(crate) fn cmd_resource_action(
    client: &GlowClient,
    resource: &str,
    action: &str,
    body_str: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let body = match body_str {
        Some(s) => Some(
            serde_json::from_str::<serde_json::Value>(s)
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --body: {}", e))?,
        ),
        None => None,
    };

    let response = client.resource_action(resource, action, body)?;

    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
    Ok(())
}

// ── Context / Emulate / Generate ──────────────────────────────

pub(crate) fn cmd_context(client: &GlowClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.context()?;
    output::print_result(mode, &response, |resp| {
        println!("{}", "Context".bold());
        if let Some(name) = resp.get("name").and_then(|v| v.as_str()) {
            println!("  Name:       {}", name.bold());
        }
        if let Some(id) = resp.get("profile_id").and_then(|v| v.as_str()) {
            println!("  Profile ID: {}", id.dimmed());
        }
        if let Some(role) = resp.get("role").and_then(|v| v.as_str()) {
            println!("  Role:       {}", role);
        }
        if let Some(true) = resp.get("emulating").and_then(|v| v.as_bool()) {
            println!("  {}", "Currently emulating another user".yellow());
        }
    });
    Ok(())
}

pub(crate) fn cmd_emulate(
    client: &GlowClient,
    target_profile_id: &str,
    ttl: u32,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let response = client.emulate(target_profile_id, ttl)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{} Now emulating profile {}",
            "OK".green().bold(),
            target_profile_id.dimmed(),
        );
        if let Some(ttl) = resp.get("ttl").and_then(|v| v.as_u64()) {
            println!("  TTL: {}s", ttl);
        }
    });
    Ok(())
}

pub(crate) fn cmd_unemulate(client: &GlowClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.unemulate()?;
    output::print_result(mode, &response, |resp| {
        let _ = resp; // used for JSON mode
        println!("{} Stopped emulating", "OK".green().bold());
    });
    Ok(())
}

pub(crate) fn cmd_generate(
    client: &GlowClient,
    group_id: &str,
    body_str: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let body = match body_str {
        Some(s) => Some(
            serde_json::from_str::<serde_json::Value>(s)
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --body: {}", e))?,
        ),
        None => None,
    };

    let response = client.generate(group_id, body)?;
    output::print_result(mode, &response, |resp| {
        if let Some(job_id) = resp.get("job_id").and_then(|v| v.as_str()) {
            println!(
                "{} Generation queued: {}",
                "OK".green().bold(),
                job_id.dimmed()
            );
        }
        if let Some(status) = resp.get("status").and_then(|v| v.as_str()) {
            println!("  Status: {}", status);
        }
    });
    Ok(())
}

pub(crate) fn cmd_connect(client: &GlowClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.connect()?;
    output::print_result(mode, &response, |resp| {
        if let Some(sid) = resp.get("sid").and_then(|v| v.as_str()) {
            println!("{} Session created: {}", "OK".green().bold(), sid.bold());
            println!("  Use with: glow stream --artifact <type> --operation <op>");
            println!("  Destroy with: glow disconnect {}", sid);
        }
    });
    Ok(())
}

pub(crate) fn cmd_disconnect(client: &GlowClient, sid: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.disconnect(sid)?;
    output::print_result(mode, &response, |_| {
        println!(
            "{} Session destroyed: {}",
            "OK".green().bold(),
            sid.dimmed()
        );
    });
    Ok(())
}

pub(crate) fn cmd_problem(
    client: &GlowClient,
    problem_type: &str,
    message: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let response = client.problem(problem_type, message)?;
    output::print_result(mode, &response, |resp| {
        if let Some(id) = resp.get("problem_id").and_then(|v| v.as_str()) {
            println!("{} Problem reported: {}", "OK".green().bold(), id.dimmed());
        } else {
            println!("{} Problem reported", "OK".green().bold());
        }
    });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn cmd_stream(
    client: &GlowClient,
    artifact: &str,
    operation: &str,
    entity_id: Option<&str>,
    cursor: Option<&str>,
    types: Option<&str>,
    limit: Option<u32>,
    mode: OutputMode,
) -> Result<()> {
    let response = client.stream(artifact, operation, entity_id, cursor, types, limit)?;

    match mode {
        OutputMode::Json => {
            crate::glow::read_sse_events(response, |data| {
                println!("{}", data);
            })?;
        }
        OutputMode::Human => {
            use colored::Colorize;
            println!(
                "{} Streaming {} {} events...\n",
                "●".green(),
                artifact.bold(),
                operation,
            );
            crate::glow::read_sse_events(response, |data| {
                println!("{}", data);
            })?;
        }
    }
    Ok(())
}

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

pub(crate) fn cmd_media_create(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    filename: &str,
    size: Option<u64>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let response = client.media_create(resource, media_type, filename, size)?;
    output::print_result(mode, &response, |resp| {
        println!("{} TUS upload initiated", "OK".green().bold());
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
    Ok(())
}

pub(crate) fn cmd_media_chunk(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    upload_id: &str,
    file_path: &str,
    offset: u64,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let data = std::fs::read(file_path)
        .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file_path, e))?;
    let response = client.media_chunk(resource, media_type, upload_id, data, offset)?;
    output::print_result(mode, &response, |resp| {
        if let Some(new_offset) = resp.get("offset").and_then(|v| v.as_u64()) {
            println!(
                "{} Chunk uploaded, offset now {}",
                "OK".green().bold(),
                new_offset
            );
        }
    });
    Ok(())
}

pub(crate) fn cmd_media_status(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    upload_id: &str,
    mode: OutputMode,
) -> Result<()> {
    let response = client.media_status(resource, media_type, upload_id)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
    Ok(())
}

pub(crate) fn cmd_media_finalize(
    client: &GlowClient,
    resource: &str,
    media_type: &str,
    upload_id: &str,
    body_str: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let body = match body_str {
        Some(s) => Some(
            serde_json::from_str::<serde_json::Value>(s)
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --body: {}", e))?,
        ),
        None => None,
    };
    let response = client.media_finalize(resource, media_type, upload_id, body)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{} Finalized upload {}",
            "OK".green().bold(),
            upload_id.dimmed()
        );
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
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

// ── Health ────────────────────────────────────────────────────

pub(crate) fn cmd_health(client: &GlowClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.health()?;

    // Check API version compatibility
    if let Some(ref server_version) = response.version {
        crate::api_common::check_api_version(
            server_version,
            crate::glow::types::PINNED_API_VERSION,
            "Glow API",
        );
    }

    output::print_result(mode, &response, |resp| {
        let indicator = if resp.status == "ok" {
            "●".green()
        } else {
            "●".red()
        };
        println!("{} Glow instance: {}", indicator, resp.status.bold());
        if let Some(ref svc) = resp.service {
            println!("  Service: {}", svc);
        }
        if let Some(ref v) = resp.version {
            println!("  Version: {}", v.dimmed());
        }
    });
    Ok(())
}
