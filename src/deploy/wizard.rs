//! Interactive `glow init` wizard — collapses the web wizard's 3 steps
//! (Setup → AI/Auth Config → Review) into a linear `dialoguer` flow.
//!
//! v1.0.0 scope: cover the 80% case — name, origin, AI provider + key,
//! one model role, optional auth providers. Power-user fields (custom
//! domains, model overrides per role, version pinning) are left for
//! direct yaml editing — the wizard's output is a real
//! `glow-deploy.yaml` the user can hand-edit afterward.

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};
use std::collections::BTreeMap;

use crate::deploy::config::{AiConfig, AiProvider, AuthConfig, DeployConfig};
use crate::deploy::instance::Instance;

/// Run the wizard against an instance, returning the populated config
/// (already written to disk).
pub fn run(name: &str) -> Result<DeployConfig> {
    let theme = ColorfulTheme::default();
    let instance = Instance::ensure(name)?;
    instance.mark_active()?;

    println!("\nLet's configure `{name}`. Press Ctrl-C anytime to abort.\n");

    let origin: String = Input::with_theme(&theme)
        .with_prompt("Public origin URL (where clients connect)")
        .default("http://localhost:6000".into())
        .interact_text()
        .context("origin prompt")?;

    // ── AI provider ─────────────────────────────────────────
    let provider_idx = Select::with_theme(&theme)
        .with_prompt("Primary AI provider")
        .items(&["openai", "anthropic", "gemini", "grok", "custom"])
        .default(0)
        .interact()?;
    let provider_name = ["openai", "anthropic", "gemini", "grok", "custom"][provider_idx];

    let endpoint = if provider_name == "custom" {
        Some(
            Input::<String>::with_theme(&theme)
                .with_prompt("Custom AI endpoint URL")
                .interact_text()?,
        )
    } else {
        None // Provider's default is wired in by the api image.
    };

    let key = Password::with_theme(&theme)
        .with_prompt(format!("{provider_name} API key (input hidden)"))
        .allow_empty_password(true)
        .interact()?;

    let model_for_text: String = Input::with_theme(&theme)
        .with_prompt("Model name for the `text` role (chat / generation)")
        .default(default_text_model(provider_name).into())
        .interact_text()?;

    // ── Seed setup ──────────────────────────────────────────
    let setup_idx = Select::with_theme(&theme)
        .with_prompt("Seed template (first-deploy DB content)")
        .items(&[
            "fresh — empty DB, you'll create everything yourself",
            "university — sample personas + scenarios for a university",
            "organization — sample personas + scenarios for a generic org",
        ])
        .default(0)
        .interact()?;
    let setup = ["fresh", "university", "organization"][setup_idx];

    // ── Optional Google OIDC ────────────────────────────────
    let mut auth_providers: Vec<serde_yaml::Value> = Vec::new();
    if Confirm::with_theme(&theme)
        .with_prompt("Add Google OIDC for login?")
        .default(false)
        .interact()?
    {
        let cid: String = Input::with_theme(&theme)
            .with_prompt("Google client_id")
            .interact_text()?;
        let csec = Password::with_theme(&theme)
            .with_prompt("Google client_secret")
            .interact()?;
        auth_providers.push(serde_yaml::to_value(serde_json::json!({
            "name": "google",
            "protocol": "oidc",
            "client_id": cid,
            "client_secret": csec,
        }))?);
    }

    // ── Assemble + persist ──────────────────────────────────
    let mut roles = BTreeMap::new();
    roles.insert("text".to_string(), model_for_text);

    let cfg = DeployConfig {
        setup: Some(setup.into()),
        topology: Default::default(),
        api_origin: None,
        client_origin: Some(origin.clone()),
        api_version: None,
        client_version: None,
        client_http_port: None,
        origin: Some(origin),
        ai: AiConfig {
            providers: vec![AiProvider {
                name: provider_name.into(),
                endpoint,
                key: if key.is_empty() { None } else { Some(key) },
            }],
            roles,
            models: vec![],
        },
        auth: AuthConfig {
            providers: auth_providers,
        },
        extra: BTreeMap::new(),
    };

    cfg.save(&instance.deploy_yaml())?;
    println!("\n✓ wrote {}", instance.deploy_yaml().display());
    println!("  edit by hand or re-run `glow init --name {name}` to overwrite.\n");
    println!("Next: `glow deploy --name {name}` to bring it up.");

    Ok(cfg)
}

fn default_text_model(provider: &str) -> &'static str {
    match provider {
        "openai" => "gpt-4.1",
        "anthropic" => "claude-sonnet-4-6",
        "gemini" => "gemini-3.1-pro",
        "grok" => "grok-4.3",
        _ => "",
    }
}
