// glow / glw — Glow OSS command-line interface.
//
// Two responsibilities:
//   1. Own the local deploy lifecycle (init / deploy / redeploy / stop /
//      start / destroy / status / backup) — talks to docker compose,
//      stores per-instance state under `~/.glow/instances/<name>/`.
//   2. Manage a running instance (auth / context / generate / resource
//      CRUD / streaming) — talks to the Glow HTTP API on the configured
//      instance URL.
//
// v1.0.0 is single-instance: one machine, one deployment. Multi-instance
// switching is intentionally out of scope.

mod api_common;
mod auth;
mod backup;
mod commands;
mod config;
mod deploy;
mod glow;
mod output;
mod render;
mod resource;
mod serve;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use output::OutputMode;

#[derive(Parser)]
#[command(name = "glow", about = "Glow CLI — manage your platform", version)]
struct Cli {
    /// Glow instance URL (overrides config + ``$GLOW_INSTANCE_URL``).
    #[arg(long, env = "GLOW_INSTANCE_URL")]
    instance_url: Option<String>,

    /// OAuth client ID
    #[arg(long, env = "GLOW_CLIENT_ID")]
    client_id: Option<String>,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Skip confirmation prompts for destructive actions
    #[arg(long, short = 'y', global = true)]
    yes: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with Glow instance.
    ///
    /// Default: opens a browser for the OAuth redirect flow.
    /// With ``--token``: skip the redirect and store a JWT directly
    /// (service-account / programmatic / CI use cases).
    Login {
        /// Pre-issued JWT to store as the access token. When set, the
        /// OAuth redirect flow is skipped and the token is persisted
        /// keyed on the instance URL. No refresh_token is recorded —
        /// the server enforces expiry via the token's own ``exp`` claim.
        #[arg(long)]
        token: Option<String>,

        /// OAuth client ID. Defaults to the value from
        /// ``--client-id`` / ``$GLOW_CLIENT_ID`` / config / "glow-cli".
        /// Ignored when ``--token`` is set.
        #[arg(long = "client-id")]
        client_id: Option<String>,
    },

    /// Sign out: fire ``GET /logout`` server-side (writes a
    /// ``logouts_entry`` row so the next request mints a fresh
    /// session) and then clear the local stored token. The
    /// server-side call is best-effort — local clear always runs.
    Logout,

    // ``Health`` removed in Cleanup E — there is no top-level /health
    // route. The old command hit GET / (a public liveness ping that
    // returns {service, version, status}), which is misleadingly named.
    // The real health artifact is POST /system/health — reach via
    // ``glow system health`` through generic dispatch.

    // ``Context`` removed — no top-level /context route. Every artifact
    // has its own POST /<art>/context (returns a ComposedContextResponse).
    // Reach via ``glow profiles context`` (or any other artifact) through
    // generic dispatch.
    //
    // ``Emulate`` / ``Unemulate`` removed — these only exist on the
    // profile artifact (POST /profile/emulate, POST /profile/unemulate).
    // Reach via ``glow profiles emulate <profile_id> [--ttl-minutes N]``
    // and ``glow profiles unemulate`` — ergonomic helpers preserve the
    // positional + flag UX (see dispatch_resource_helper).

    // ``Generate``, ``Stream``, ``Connect``, ``Disconnect`` removed —
    // they all hit non-existent top-level routes on the API
    // (/generate, /stream, /connect, /disconnect). The actual API:
    //   * Generate is per-artifact: POST /<artifact>/generate.
    //     Reach via ``glow <art> generate [--wait]`` — see the
    //     ``generate`` intercept in dispatch_resource_helper for the
    //     --wait flag plumbing.
    //   * Streaming is per-artifact: ``glow <art> watch <run_id>``
    //     opens the SSE filtered to a run.
    //   * Connect/disconnect for session-scoped streams didn't exist
    //     on the API and had no live callers.
    /// MCP — talk to the Glow instance's Model Context Protocol
    /// endpoint at ``/mcp/``. Currently a JSON-RPC client thin enough
    /// to list + call tools without dragging in a full MCP crate.
    Mcp {
        #[command(subcommand)]
        command: McpCommands,
    },

    // ``Problem`` removed — no top-level /problem route. Every artifact
    // has its own POST /<art>/problem with an artifact-scoped response.
    // Reach via ``glow <art> problem --body '{"type":"...","message":"..."}'``
    // through generic dispatch.
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },

    /// Start the CLI dev server (serves CLI spec + exec endpoint)
    Serve {
        /// Port to listen on
        #[arg(long, default_value = "9000", env = "GLOW_CLI_PORT")]
        port: u16,
    },

    /// Interactive wizard: scaffold a `glow-deploy.yaml` under
    /// `~/.glow/instances/<name>/`. Walks you through origin URL, AI
    /// provider + key, seed template, and optional OIDC. Idempotent —
    /// re-running overwrites the yaml.
    Init {
        #[arg(long, default_value = "default")]
        name: String,
    },

    /// Tail container logs (`docker compose logs [-f]`).
    Logs {
        #[arg(long, default_value = "default")]
        name: String,
        /// Follow log output.
        #[arg(short, long)]
        follow: bool,
        /// Optional service name (e.g. `server-blue`, `database`).
        service: Option<String>,
    },

    /// Deploy a Glow instance to the local machine (first-time setup).
    /// Reads `~/.glow/instances/<name>/glow-deploy.yaml`. Run `glow init`
    /// first to scaffold one interactively.
    Deploy {
        /// Instance name (defaults to "default").
        #[arg(long, default_value = "default")]
        name: String,
        /// API image version, e.g. `v1.0.0`.
        #[arg(long, default_value = "v1.0.0")]
        api_version: String,
        /// Client image version (required unless topology=api_only).
        #[arg(long, default_value = "v1.0.0")]
        client_version: String,
        /// Seed template for first deploy: fresh | university | organization.
        #[arg(long)]
        seed_setup: Option<String>,
        /// Grace period (minutes) to watch the new color after traffic swap.
        #[arg(long, default_value = "2")]
        grace_minutes: u32,
    },

    /// Roll out a new version while keeping the database. Auto-pre-backs
    /// up before doing anything destructive, blue/green swaps with
    /// healthcheck monitoring + auto-rollback if the new color flaps.
    Redeploy {
        #[arg(long, default_value = "default")]
        name: String,
        #[arg(long)]
        api_version: Option<String>,
        /// Client image tag — falls back to the last-deployed version
        /// recorded in state when omitted.
        #[arg(long)]
        client_version: Option<String>,
        /// Restore from a named backup as part of the redeploy.
        #[arg(long, conflicts_with = "reseed")]
        from_backup: Option<String>,
        /// DESTRUCTIVE: re-run seed-gen and wipe the DB. Use with care.
        #[arg(long, conflicts_with = "from_backup")]
        reseed: Option<String>,
        #[arg(long, default_value = "2")]
        grace_minutes: u32,
    },

    /// Stop containers; volumes + network intact. `glow start` resumes.
    Stop {
        #[arg(long, default_value = "default")]
        name: String,
    },

    /// Resume previously-stopped containers.
    Start {
        #[arg(long, default_value = "default")]
        name: String,
    },

    /// Tear down containers + volumes (DESTROYS DATA — confirm twice).
    /// Config + backups under `~/.glow/instances/<name>/` are preserved.
    Destroy {
        #[arg(long, default_value = "default")]
        name: String,
    },

    /// Show instance state + per-container health.
    Status {
        #[arg(long, default_value = "default")]
        name: String,
    },

    /// Manage local pg_dump backups.
    Backup {
        #[command(subcommand)]
        action: BackupCommands,
    },

    /// Interact with a resource on the Glow instance (e.g. glow personas search)
    #[command(external_subcommand)]
    Resource(Vec<String>),
}

#[derive(Subcommand)]
enum McpCommands {
    /// List the MCP tools the Glow instance exposes.
    /// → POST /mcp/ with JSON-RPC ``tools/list``.
    #[command(name = "list-tools")]
    ListTools,
    /// Call an MCP tool by name with a JSON arguments object.
    /// → POST /mcp/ with JSON-RPC ``tools/call``.
    Call {
        /// Tool name (as it appears in ``list-tools`` output).
        tool_name: String,
        /// JSON arguments object (defaults to ``{}``).
        #[arg(long, default_value = "{}")]
        args: String,
    },
}

#[derive(Subcommand)]
enum BackupCommands {
    /// List local backups for an instance.
    #[command(alias = "ls")]
    List {
        #[arg(long, default_value = "default")]
        name: String,
    },
    /// Create a new pg_dump backup.
    Create {
        #[arg(long, default_value = "default")]
        name: String,
        /// Optional label tag; filename becomes `manual-<label>-<ts>.sql.gz`.
        #[arg(long)]
        label: Option<String>,
    },
    /// Restore an existing backup (destructive — drops + recreates DB).
    Restore {
        #[arg(long, default_value = "default")]
        name: String,
        /// Backup filename inside the instance's `backups/` dir.
        file: String,
    },
    /// Delete a backup file.
    #[command(alias = "rm")]
    Delete {
        #[arg(long, default_value = "default")]
        name: String,
        file: String,
    },
}

// ── CLI spec generation ───────────────────────────────────────

fn dump_command_spec(cmd: &clap::Command) -> serde_json::Value {
    use serde_json::json;
    let mut obj = serde_json::Map::new();
    obj.insert("name".into(), json!(cmd.get_name()));
    if let Some(about) = cmd.get_about() {
        obj.insert("about".into(), json!(about.to_string()));
    }

    let aliases: Vec<&str> = cmd.get_visible_aliases().collect();
    if !aliases.is_empty() {
        obj.insert("aliases".into(), json!(aliases));
    }

    let args: Vec<serde_json::Value> = cmd
        .get_arguments()
        .filter(|a| !["help", "version"].contains(&a.get_id().as_str()))
        .map(|a| {
            let mut arg = serde_json::Map::new();
            arg.insert("name".into(), json!(a.get_id().as_str()));
            if let Some(long) = a.get_long() {
                arg.insert("long".into(), json!(format!("--{}", long)));
            }
            if let Some(short) = a.get_short() {
                arg.insert("short".into(), json!(format!("-{}", short)));
            }
            if let Some(env) = a.get_env() {
                arg.insert("env".into(), json!(env.to_string_lossy()));
            }
            if let Some(help) = a.get_help() {
                arg.insert("help".into(), json!(help.to_string()));
            }
            arg.insert("required".into(), json!(a.is_required_set()));
            if a.is_global_set() {
                arg.insert("global".into(), json!(true));
            }
            serde_json::Value::Object(arg)
        })
        .collect();
    if !args.is_empty() {
        obj.insert("args".into(), json!(args));
    }

    let subs: Vec<serde_json::Value> = cmd
        .get_subcommands()
        .filter(|s| !s.is_hide_set())
        .map(dump_command_spec)
        .collect();
    if !subs.is_empty() {
        obj.insert("subcommands".into(), json!(subs));
    }

    serde_json::Value::Object(obj)
}

/// Build the full CLI spec JSON (used by both --dump-cli-spec and the serve endpoint).
pub fn build_cli_spec() -> serde_json::Value {
    use serde_json::json;

    let cmd = Cli::command();
    let mut spec = dump_command_spec(&cmd);

    if let Some(obj) = spec.as_object_mut() {
        obj.insert("version".into(), json!(env!("CARGO_PKG_VERSION")));

        let resources: Vec<serde_json::Value> = resource::Resource::all()
            .iter()
            // Spec consumers (the CLI-dev server) want both the
            // user-typed name and the actual wire path so the help
            // surface matches what the user just typed AND what hits
            // the API. ``slug`` is kept as an alias for ``cli_name``
            // to avoid breaking any consumer that's still on the old
            // single-field shape.
            .map(|r| {
                json!({
                    "slug": r.cli_name(),
                    "cli_name": r.cli_name(),
                    "api_path": r.api_path(),
                    "about": r.about(),
                })
            })
            .collect();
        obj.insert("resources".into(), json!(resources));

        let media_types: Vec<&str> = resource::MediaType::all_slugs().to_vec();
        obj.insert("media_types".into(), json!(media_types));

        // Nested view-artifact namespaces. Docs gen + IDE tooling use
        // this to render `glow attempts chat get` style commands with
        // proper parent/view/action structure.
        let namespaces: Vec<serde_json::Value> = resource::NAMESPACES
            .iter()
            .map(|n| {
                json!({
                    "name": n.name,
                    "own_actions": n.own_actions,
                    "views": n.views,
                })
            })
            .collect();
        obj.insert("namespaces".into(), json!(namespaces));

        obj.insert("media_actions".into(), json!([
            { "name": "upload", "about": "Upload a file via multipart form", "args": [
                { "name": "file", "long": "--file", "required": true, "help": "Path to file to upload" }
            ]},
            { "name": "download", "about": "Download a media file", "args": [
                { "name": "id", "long": "--id", "required": true, "help": "Upload ID" },
                { "name": "output", "long": "--output", "required": false, "help": "Output file path (stdout if omitted)" }
            ]},
            { "name": "create", "about": "Initiate a TUS resumable upload", "args": [
                { "name": "filename", "long": "--filename", "required": true, "help": "Filename for the upload" },
                { "name": "size", "long": "--size", "required": false, "help": "Total file size in bytes" }
            ]},
            { "name": "chunk", "about": "Upload a chunk for a TUS upload", "args": [
                { "name": "id", "long": "--id", "required": true, "help": "Upload ID" },
                { "name": "file", "long": "--file", "required": true, "help": "Path to chunk data" },
                { "name": "offset", "long": "--offset", "required": false, "help": "Byte offset (default: 0)" }
            ]},
            { "name": "status", "about": "Check TUS upload status", "args": [
                { "name": "id", "long": "--id", "required": true, "help": "Upload ID" }
            ]},
            { "name": "finalize", "about": "Finalize a TUS upload", "args": [
                { "name": "id", "long": "--id", "required": true, "help": "Upload ID" },
                { "name": "body", "long": "--body", "required": false, "help": "Optional JSON body" }
            ]},
            { "name": "discover", "about": "Discover available upload types", "args": [
                { "name": "id", "long": "--id", "required": false, "help": "Optional upload ID" }
            ]},
            { "name": "preview", "about": "Preview a media file", "args": [
                { "name": "id", "long": "--id", "required": true, "help": "Upload ID" }
            ]}
        ]));
    }

    spec
}

// ── Helpers ──────────────────────────────────────────────────

/// Resolve glow instance URL: --instance-url > active instance > glow_url config > default
fn resolve_glow_url(cli_url: Option<&str>, cfg: &config::Config) -> String {
    cli_url
        .or_else(|| cfg.active_instance_url())
        .or(cfg.glow_url.as_deref())
        .unwrap_or("http://localhost:8000")
        .to_string()
}

// ── Main ───────────────────────────────────────────────────────

pub fn run() -> Result<()> {
    // Hidden: dump CLI spec as JSON for docs generation
    if std::env::args().any(|a| a == "--dump-cli-spec") {
        let spec = build_cli_spec();
        println!("{}", serde_json::to_string_pretty(&spec)?);
        return Ok(());
    }

    let cli = Cli::parse();
    let cfg = config::Config::load()?;
    let mode = OutputMode::resolve(cli.json);

    use commands::glow as glow_cmd;

    match cli.command {
        // ── Top-level Glow instance commands ─────────────────
        Commands::Login { token, client_id } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            if let Some(t) = token {
                // Service-account / programmatic flow: bypass the
                // OAuth redirect, store the JWT directly. The server
                // enforces expiry via the token's own ``exp`` claim;
                // we can't refresh without a refresh_token, so the
                // caller is responsible for rotation.
                let stored = auth::StoredToken {
                    access_token: t,
                    id_token: None,
                    token_type: "Bearer".to_string(),
                    expires_in: None, // unknown — server enforces via JWT exp
                    issued_at: 0,     // unknown — server enforces via JWT exp
                    refresh_token: None,
                    token_endpoint: None,
                };
                auth::save_token(&glow_url, stored)?;
                use colored::Colorize;
                println!("{} Token stored for {}", "✓".green(), glow_url.bold());
            } else {
                // OAuth redirect flow.
                let cid = client_id
                    .or(cli.client_id.clone())
                    .unwrap_or_else(|| "glow-cli".to_string());
                let _ = auth::login(&glow_url, &cid)?;
                use colored::Colorize;
                println!("{} Signed in to {}", "✓".green(), glow_url.bold());
            }
        }
        Commands::Logout => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            // Best-effort server-side hit first so the API gets a
            // logouts_entry row before the local token disappears.
            // GlowClient loads the bearer from the local store at
            // construction, so this fires with the right header.
            let client = glow::GlowClient::new(&glow_url);
            let _ = client.logout_server_side();
            let removed = auth::remove_token(&glow_url)?;
            use colored::Colorize;
            if removed {
                println!("{} Signed out of {}", "✓".green(), glow_url.bold());
            } else {
                println!(
                    "{} No stored token for {} — local store unchanged",
                    "·".dimmed(),
                    glow_url.bold(),
                );
            }
        }
        // ``Commands::Health`` removed in Cleanup E — see comment on
        // the enum variant. Use ``glow system health`` for the real
        // health artifact (POST /system/health).

        // ``Commands::Context`` / ``Emulate`` / ``Unemulate`` removed —
        // context lives at POST /<artifact>/context on every artifact;
        // emulate/unemulate only exist on /profile. Reach them via
        // generic dispatch (``glow profiles context``,
        // ``glow profiles emulate <profile_id>``, ``glow profiles
        // unemulate``) — see the Profiles intercept in
        // dispatch_resource_helper for the ergonomic emulate shape.

        // ``Commands::Generate`` removed — was a wrapper that hit a
        // non-existent /generate top-level route. Generate is per-
        // artifact (POST /<artifact>/generate). Reach via
        // ``glow <art> generate [--wait] [--body '{...}']`` — the
        // ``--wait`` flag is intercepted in dispatch_resource (see
        // the generate intercept).
        // ``Commands::Groups`` removed in Cleanup B. Now reachable as
        // ``glow system groups`` via the generic dispatch.
        Commands::Mcp { command } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            match command {
                McpCommands::ListTools => {
                    glow_cmd::helpers::cmd_mcp_list_tools(&client, &glow_url, mode)?
                }
                McpCommands::Call { tool_name, args } => {
                    glow_cmd::helpers::cmd_mcp_call(&client, &glow_url, &tool_name, &args, mode)?
                }
            }
        }
        // ``Commands::Chats`` removed in Cleanup B. Live REPL +
        // voice placeholder now live under the chat namespace at
        // ``glow attempts chat live <chat_id>`` /
        // ``glow attempts chat voice <chat_id>`` via the sub-op
        // namespace intercept in dispatch_resource.
        // ``Commands::Stream`` / ``Connect`` / ``Disconnect`` removed —
        // hit non-existent /stream, /connect, /disconnect top-level
        // routes. Streaming is per-artifact: ``glow <art> watch <run_id>``.
        // ``Commands::Problem`` removed — POST /<artifact>/problem is
        // the real surface; reach via generic dispatch on any artifact.

        // ── Shell completions ────────────────────────────────
        Commands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "glow", &mut std::io::stdout());
        }

        Commands::Serve { port } => {
            tokio::runtime::Runtime::new()?.block_on(serve::run(port))?;
        }

        // ── Local deploy lifecycle ────────────────────────────
        Commands::Init { name } => {
            deploy::wizard::run(&name)?;
        }
        Commands::Logs {
            name,
            follow,
            service,
        } => {
            let i = deploy::instance::Instance::open(&name)?;
            deploy::runtime::logs(&i.dir, &i.project_name(), follow, service.as_deref())?;
        }
        Commands::Deploy {
            name,
            api_version,
            client_version,
            seed_setup,
            grace_minutes,
        } => {
            deploy::deploy(deploy::DeployArgs {
                name,
                api_version,
                client_version: Some(client_version),
                config_path: None,
                seed_setup,
                db_backup: None,
                grace_minutes,
            })?;
        }
        Commands::Redeploy {
            name,
            api_version,
            client_version,
            from_backup,
            reseed,
            grace_minutes,
        } => {
            // Resolve image versions from state when not passed.
            let i = deploy::instance::Instance::open(&name)?;
            let state = deploy::instance::DeployState::load(&i.state_file())?;
            let resolved_api = match api_version {
                Some(v) => v,
                None => state.api_version.clone().ok_or_else(|| {
                    anyhow::anyhow!("no pinned api_version on disk and --api-version not passed")
                })?,
            };
            let resolved_client = client_version.or(state.client_version.clone());
            deploy::deploy(deploy::DeployArgs {
                name,
                api_version: resolved_api,
                client_version: resolved_client,
                config_path: None,
                seed_setup: reseed,
                db_backup: from_backup,
                grace_minutes,
            })?;
        }
        Commands::Stop { name } => deploy::stop(&name)?,
        Commands::Start { name } => deploy::start(&name)?,
        Commands::Destroy { name } => {
            // Double-confirm unless --yes — destruction is volume-removing.
            if !cli.yes {
                use std::io::Write;
                print!("This destroys ALL containers + volumes for `{name}`. Type the name to confirm: ");
                std::io::stdout().flush().ok();
                let mut answer = String::new();
                std::io::stdin().read_line(&mut answer)?;
                if answer.trim() != name {
                    anyhow::bail!("aborted (name didn't match)");
                }
            }
            deploy::destroy(&name)?;
        }
        Commands::Status { name } => deploy::status(&name)?,
        Commands::Backup { action } => match action {
            BackupCommands::List { name } => {
                for b in backup::list(&name)? {
                    println!("{}\t{} bytes\t{:?}", b.name, b.size_bytes, b.path);
                }
            }
            BackupCommands::Create { name, label } => {
                backup::create(&name, label.as_deref())?;
            }
            BackupCommands::Restore { name, file } => {
                if !cli.yes {
                    use std::io::Write;
                    print!("RESTORE will DROP the existing database for `{name}`. Type 'restore' to confirm: ");
                    std::io::stdout().flush().ok();
                    let mut answer = String::new();
                    std::io::stdin().read_line(&mut answer)?;
                    if answer.trim() != "restore" {
                        anyhow::bail!("aborted");
                    }
                }
                backup::restore(&name, &file)?;
            }
            BackupCommands::Delete { name, file } => {
                backup::delete(&name, &file)?;
            }
        },

        // ── Generic resource dispatch ────────────────────────
        Commands::Resource(args) => {
            // ``--json`` is a global flag, but ``Resource`` is an
            // ``external_subcommand`` so clap captures every trailing arg
            // raw — a late ``glow personas search --json`` never reaches
            // the global parser, leaving ``cli.json`` false. Honor it here:
            // detect ``--json`` anywhere in the captured args, force JSON
            // mode, and strip it so it can't leak into the --key/value body
            // parsing (where a bare flag would error or pollute the body).
            let mode = OutputMode::resolve(cli.json || args.iter().any(|a| a == "--json"));
            let args: Vec<String> = args.into_iter().filter(|a| a != "--json").collect();
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            dispatch_resource(&client, &args, mode)?
        }
    }

    Ok(())
}

fn dispatch_resource(client: &glow::GlowClient, args: &[String], mode: OutputMode) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("Expected a resource name. Run 'glow --help' for usage.");
    }

    let resource_slug = &args[0];
    let resource = resource::Resource::from_slug(resource_slug)
        .ok_or_else(|| anyhow::anyhow!("{}", resource::unknown_resource_error(resource_slug)))?;

    // ``glow <resource> --help`` lands here (the catch-all
    // ``Resource(Vec<String>)`` variant means Clap doesn't render a
    // built-in help screen). Render a per-resource help inline so
    // ``glow personas --help`` / ``glow system --help`` etc. don't
    // fall through to a bogus POST /<res>/--help.
    if args.len() < 2 || matches!(args[1].as_str(), "--help" | "-h") {
        use colored::Colorize;
        println!(
            "{}\n",
            format!("glow {} <action> [args]", resource.cli_name()).bold()
        );
        println!(
            "  Dispatches to {} /{}/<action>\n",
            "POST".dimmed(),
            resource.api_path()
        );
        println!("  {}", resource.about());
        println!();
        println!("{}:", "Common actions".bold());
        println!("  search / get / create / draft / drafts / archive / refresh");
        println!("  generate / export / csv / problem / title / name / watch");
        println!();
        // Surface namespaced sub-ops the user can drill into.
        match resource {
            resource::Resource::Attempts => {
                println!("{}:", "Sub-namespaces".bold());
                println!(
                    "  {:14} {}",
                    "chat".bold(),
                    "Chat sub-ops (chat_get, chat_message, chat_grade, ...) + live REPL".dimmed()
                );
                println!(
                    "  {:14} {}",
                    "dashboard / home / practice / leaderboard / report".bold(),
                    "View ops".dimmed()
                );
                println!(
                    "  {:14} {}",
                    "watch".bold(),
                    "Block on a run_id over SSE".dimmed()
                );
                println!();
            }
            resource::Resource::Tests => {
                println!("{}:", "Sub-namespaces".bold());
                println!(
                    "  {:14} {}",
                    "invocation".bold(),
                    "Invocation sub-ops (invocation_get, invocation_run, ...)".dimmed()
                );
                println!(
                    "  {:14} {}",
                    "benchmark".bold(),
                    "Benchmark view op".dimmed()
                );
                println!();
            }
            resource::Resource::System => {
                println!("{}:", "Common system ops".bold());
                println!("  activity / health / sessions / groups / pricing");
                println!("  audio_download / image_download / file_download / video_download");
                println!("  text_download / call_download / file_preview");
                println!("  generate / generations / refresh / export / watch / resolve / title / context / problem");
                println!();
            }
            resource::Resource::Profiles => {
                println!("{}:", "Profile-only ops".bold());
                println!(
                    "  {:14} {}",
                    "emulate".bold(),
                    "glow profiles emulate <profile_id> [--ttl-minutes N]".dimmed(),
                );
                println!(
                    "  {:14} {}",
                    "unemulate".bold(),
                    "glow profiles unemulate".dimmed(),
                );
                println!();
            }
            _ => {}
        }
        println!("{}:", "Options".bold());
        println!(
            "  {:<30} Raw JSON body for the action",
            "--body <json>".green()
        );
        println!("  {:<30} Output as JSON", "--json".green());
        return Ok(());
    }

    // Check if second arg is a media type (file, image, text, audio, video).
    // Media routes also hit the singular ``api_path`` on the wire.
    if let Some(media) = resource::MediaType::from_str(&args[1]) {
        return dispatch_media(client, resource.api_path(), media, &args[2..], mode);
    }

    let action = &args[1];

    // ── Watch intercept ─────────────────────────────────────
    // ``glow <res> watch <run_id> [--group-id <id>]`` is a per-
    // artifact SSE GET, not a JSON POST — needs its own dispatch
    // path. ``watch`` is reserved at the action slot; nothing else
    // is named that across the 19 CRUD resources currently in scope.
    if action == "watch" {
        if args[2..].iter().any(|a| a == "--help" || a == "-h") {
            use colored::Colorize;
            println!("{}\n", format!("glow {} watch", resource.cli_name()).bold());
            println!(
                "  {} /{}/watch?run_id=<id>&group_id=<id>\n",
                "GET".dimmed(),
                resource.api_path()
            );
            println!("  Stream the per-artifact watch SSE filtered to a run_id.");
            println!("  Blocks until a terminal lifecycle event (.completed / .failed)\n");
            println!("{}:", "Args".bold());
            println!("  <run_id>             {}", "Required positional".dimmed());
            println!("\n{}:", "Options".bold());
            println!(
                "  {:<30} Scope to a group (optional, server resolves if omitted)",
                "--group-id <id>".green()
            );
            println!("  {:<30} Output frames as JSON lines", "--json".green());
            return Ok(());
        }
        if args.len() < 3 {
            anyhow::bail!(
                "Expected a run_id: glow {} watch <run_id> [--group-id <id>]",
                resource.cli_name(),
            );
        }
        let run_id = &args[2];
        // --group-id (or --group_id) is optional. The per-artifact
        // watch route accepts it as a query param to scope the
        // subscriber room; omit it and the server-side resolver
        // derives the group from run_id.
        let group_id =
            parse_flag(&args[3..], "--group-id")?.or(parse_flag(&args[3..], "--group_id")?);
        return commands::glow::cmd_watch_run(
            client,
            resource.api_path(),
            run_id,
            group_id.as_deref(),
            mode,
        );
    }

    // ── Sub-op namespace intercept ──────────────────────────
    // The API namespaces some operation families under a parent
    // artifact's URL with underscore-joined names — chat ops on
    // attempt (``/attempt/chat_get``, ``/attempt/chat_message``, ...),
    // invocation ops on test (``/test/invocation_get``, ...). For
    // discoverability the CLI surfaces these as ``glow attempts chat
    // <op>`` / ``glow tests invocation <op>`` (3-segment), then
    // re-dispatches as the joined action. Also handles the
    // ``glow attempts chat live <chat_id>`` WS REPL and the
    // ``glow attempts chat voice <chat_id>`` deferral message.
    if dispatch_subop_namespace(client, resource, action, &args[2..], mode)?.is_some() {
        return Ok(());
    }

    // Show help for resource action. UX note: the heading uses the
    // user-facing plural ``cli_name`` ("glow personas search") so the
    // copy-paste hint matches what they just typed; the URL line shows
    // the actual singular API path ("POST /persona/search") so the user
    // can verify the wire call against the API docs without confusion.
    if args[2..].iter().any(|a| a == "--help" || a == "-h") {
        use colored::Colorize;
        println!(
            "{}\n",
            format!("glow {} {}", resource.cli_name(), action).bold()
        );
        println!(
            "  {} /{}/{}\n",
            "POST".dimmed(),
            resource.api_path(),
            action
        );
        println!("  Pass --key value pairs as needed. The API will validate parameters.\n");
        println!("{}:", "Options".bold());
        println!(
            "  {:<30} Raw JSON body (can combine with flags)",
            "--body <json>".green()
        );
        println!("  {:<30} Output as JSON", "--json".green());
        return Ok(());
    }

    // ── Ergonomic helper intercept ─────────────────────────
    // For high-traffic (resource, action) combos, route through a
    // hand-written helper that takes positional args + named flags
    // instead of forcing the user to construct ``--body '{...}'``.
    // Everything not matched falls through to the generic body-driven
    // dispatch below — the helpers are pure sugar.
    if dispatch_resource_helper(client, resource, action, &args[2..], mode)?.is_some() {
        return Ok(());
    }

    let rest = &args[2..];
    let body = build_resource_body(rest)?;
    let file_path = parse_flag(rest, "--file")?;
    let show_headers = rest.iter().any(|a| a == "--show-headers");

    // When uploading, fold ``--key value`` params into multipart form
    // fields alongside the file (so e.g. ``--soft true --idempotency_key
    // <uuid>`` works for soft-accept upload endpoints).
    let extra_form_fields: Vec<(String, String)> = if file_path.is_some() {
        parse_params(rest)?.into_iter().collect()
    } else {
        Vec::new()
    };

    // Wire the singular ``api_path`` here — ``cli_name`` is plural for
    // the user, ``api_path`` is what the frozen API actually exposes.
    commands::glow::cmd_resource_action_ext(
        client,
        resource.api_path(),
        action,
        body.as_deref(),
        mode,
        file_path.as_deref(),
        show_headers,
        &extra_form_fields,
    )
}

/// Per-(resource, action) ergonomic helpers. Returns ``Ok(Some(()))``
/// when the helper handled the call, ``Ok(None)`` to fall through to
/// the generic dispatch. Each helper parses its own positional args +
/// flags so the user can write ``glow attempts start <sim_id> --persona
/// <id>`` instead of ``glow attempts start --body '{...}'``.
fn dispatch_resource_helper(
    client: &glow::GlowClient,
    resource: resource::Resource,
    action: &str,
    rest: &[String],
    mode: OutputMode,
) -> Result<Option<()>> {
    use commands::glow::helpers;
    use resource::Resource::*;

    match (resource, action) {
        // glow attempts start <simulation_id> [--persona <id>] [--cohort <id>]
        (Attempts, "start") if !rest.is_empty() && !rest[0].starts_with("--") => {
            let sim_id = &rest[0];
            let persona = parse_flag(&rest[1..], "--persona")?;
            let cohort = parse_flag(&rest[1..], "--cohort")?;
            helpers::cmd_attempt_start(
                client,
                sim_id,
                persona.as_deref(),
                cohort.as_deref(),
                mode,
            )?;
            Ok(Some(()))
        }
        // glow attempts message <chat_id> <text> [--audio <file>] [--persona <id>]
        // — also matches the namespaced form ``glow attempts chat message ...``
        //   via the chat-subop intercept (which re-dispatches as
        //   action="chat_message").
        (Attempts, "message") | (Attempts, "chat_message")
            if rest.len() >= 2 && !rest[0].starts_with("--") && !rest[1].starts_with("--") =>
        {
            let chat_id = &rest[0];
            let text = &rest[1];
            let audio = parse_flag(&rest[2..], "--audio")?;
            let persona = parse_flag(&rest[2..], "--persona")?;
            helpers::cmd_attempt_message(
                client,
                chat_id,
                text,
                audio.as_deref(),
                persona.as_deref(),
                mode,
            )?;
            Ok(Some(()))
        }
        // glow attempts grade <chat_id> [--score N] — also matches
        // ``glow attempts chat grade ...`` via the chat-subop
        // intercept (which re-dispatches as action="chat_grade").
        (Attempts, "grade") | (Attempts, "chat_grade")
            if !rest.is_empty() && !rest[0].starts_with("--") =>
        {
            let chat_id = &rest[0];
            let score = parse_flag(&rest[1..], "--score")?
                .map(|s| s.parse::<u32>())
                .transpose()
                .context("--score must be an integer")?;
            helpers::cmd_attempt_grade(client, chat_id, score, mode)?;
            Ok(Some(()))
        }
        // glow scenarios generate <modality> <title> [--instructions <text>]
        (Scenarios, "generate")
            if rest.len() >= 2 && !rest[0].starts_with("--") && !rest[1].starts_with("--") =>
        {
            let modality = &rest[0];
            let title = &rest[1];
            let instructions = parse_flag(&rest[2..], "--instructions")?;
            helpers::cmd_scenario_generate(client, modality, title, instructions.as_deref(), mode)?;
            Ok(Some(()))
        }
        // glow personas search [--name <pat>] [--page-size N] [--page-offset N]
        (Personas, "search")
            if rest.iter().all(|a| {
                a.starts_with("--")
                    || rest
                        .iter()
                        .take_while(|x| !x.starts_with("--"))
                        .all(|_| false)
            }) =>
        {
            let name = parse_flag(rest, "--name")?;
            let page_size = parse_flag(rest, "--page-size")?
                .map(|s| s.parse::<u32>())
                .transpose()
                .context("--page-size must be an integer")?;
            let page_offset = parse_flag(rest, "--page-offset")?
                .map(|s| s.parse::<u32>())
                .transpose()
                .context("--page-offset must be an integer")?;
            // If user passed --body, defer to generic dispatch
            // (helpers don't merge custom JSON bodies).
            if parse_flag(rest, "--body")?.is_some() {
                return Ok(None);
            }
            helpers::cmd_personas_search(client, name.as_deref(), page_size, page_offset, mode)?;
            Ok(Some(()))
        }
        // glow <artifact> get <id> — positional id sugar for the detail
        // endpoints (which take the id in the body). ``--id`` can't be used
        // because it's reserved for media ops, so without this the user had
        // to hand-write ``--body '{"id":"…"}'``. Every artifact keys the id as
        // plain ``id`` except attempts (``attempt_id``) and tests (``test_id``).
        (_, "get") if !rest.is_empty() && !rest[0].starts_with("--") => {
            // Defer to generic dispatch if the user hand-wrote a body.
            if parse_flag(rest, "--body")?.is_some() {
                return Ok(None);
            }
            let id_field = match resource {
                Attempts => "attempt_id",
                Tests => "test_id",
                _ => "id",
            };
            helpers::cmd_resource_get(client, resource.api_path(), id_field, &rest[0], mode)?;
            Ok(Some(()))
        }
        // glow profiles emulate <profile_id> [--ttl-minutes N]
        //   → POST /profile/emulate {target_profile_id, ttl_minutes}
        // ``emulate`` and ``unemulate`` live only on the profile
        // artifact on the API; this helper preserves the old
        // top-level ``glow emulate <id>`` UX (positional + flag)
        // now that the top-level wrapper is gone.
        (Profiles, "emulate") if !rest.is_empty() && !rest[0].starts_with("--") => {
            // Defer to generic dispatch if user passed --body.
            if parse_flag(rest, "--body")?.is_some() {
                return Ok(None);
            }
            let target_profile_id = &rest[0];
            let ttl_minutes = parse_flag(&rest[1..], "--ttl-minutes")?
                .map(|s| s.parse::<u32>())
                .transpose()
                .context("--ttl-minutes must be an integer")?
                .unwrap_or(120);
            helpers::cmd_profile_emulate(client, target_profile_id, ttl_minutes, mode)?;
            Ok(Some(()))
        }
        // glow profiles unemulate → POST /profile/unemulate {}
        // (Trivial helper, but symmetric with emulate so the prompt
        // line + colored success message match.)
        (Profiles, "unemulate") if rest.iter().all(|a| a.starts_with("--")) => {
            if parse_flag(rest, "--body")?.is_some() {
                return Ok(None);
            }
            helpers::cmd_profile_unemulate(client, mode)?;
            Ok(Some(()))
        }
        // glow simulations list [--cohort <id>] [--page-size N]
        (Simulations, "list") => {
            let cohort = parse_flag(rest, "--cohort")?;
            let page_size = parse_flag(rest, "--page-size")?
                .map(|s| s.parse::<u32>())
                .transpose()
                .context("--page-size must be an integer")?;
            helpers::cmd_simulations_list(client, cohort.as_deref(), page_size, mode)?;
            Ok(Some(()))
        }
        // ── generate --wait (any resource) ───────────────────
        // Per-artifact ``POST /<art>/generate`` works via generic
        // dispatch already; the ``--wait`` flag here turns the
        // fire-and-forget trigger into a blocking call that watches
        // the returned run_id until terminal. Body is built from the
        // rest of the args (--body and/or --key value pairs) exactly
        // like the non-wait path.
        (_, "generate") if rest.iter().any(|a| a == "--wait") => {
            // Strip --wait from the body-building args so it doesn't
            // get folded into the JSON payload.
            let scrubbed: Vec<String> = rest
                .iter()
                .filter(|a| a.as_str() != "--wait")
                .cloned()
                .collect();
            let body = build_resource_body(&scrubbed)?;
            commands::glow::cmd_generate_and_wait_dispatch(
                client,
                resource.api_path(),
                body.as_deref(),
                mode,
            )?;
            Ok(Some(()))
        }
        // Anything else — let generic dispatch handle it.
        _ => Ok(None),
    }
}

/// Sub-op namespace dispatch — handles 3-segment commands that map
/// to underscore-joined ops on a parent artifact:
///
///   ``glow attempts chat <op> [args]`` → POST /attempt/chat_<op>
///   ``glow tests invocation <op> [args]`` → POST /test/invocation_<op>
///
/// Plus the two WS-only special cases under attempts.chat:
///   ``glow attempts chat live <chat_id>``  → socket.io REPL
///   ``glow attempts chat voice <chat_id>`` → deferral message
///
/// Returns ``Some(())`` if the call was handled (success or
/// surfaced error), ``None`` to fall through to the generic
/// dispatch.
fn dispatch_subop_namespace(
    client: &glow::GlowClient,
    resource: resource::Resource,
    action: &str,
    rest: &[String],
    mode: OutputMode,
) -> Result<Option<()>> {
    use commands::glow::helpers;
    use resource::Resource::*;

    // attempts chat — chat ops live as chat_<op> on /attempt/.
    if resource == Attempts && action == "chat" {
        if rest.is_empty() || rest.iter().any(|a| a == "--help" || a == "-h") {
            print_chat_namespace_help();
            return Ok(Some(()));
        }
        let subop = &rest[0];
        let subop_args = &rest[1..];

        // WS-only: live REPL + voice placeholder. Pulled into the
        // chat namespace from the (now removed) top-level Chats
        // subcommand so all chat operations live in one place.
        if subop == "live" {
            if subop_args.is_empty() {
                anyhow::bail!(
                    "Expected a chat_id: glow attempts chat live <chat_id> [--persona <id>]"
                );
            }
            let chat_id = &subop_args[0];
            let persona = parse_flag(&subop_args[1..], "--persona")?;
            return helpers::cmd_chats_live(
                client,
                client.base_url(),
                client.token(),
                chat_id,
                persona.as_deref(),
                mode,
            )
            .map(|_| Some(()));
        }
        if subop == "voice" {
            let placeholder = String::new();
            let chat_id = subop_args.first().unwrap_or(&placeholder);
            return helpers::cmd_chats_voice_placeholder(chat_id).map(|_| Some(()));
        }

        // General case: re-dispatch as action=chat_<subop>. Try the
        // helper intercept first so ``glow attempts chat message``
        // and ``glow attempts chat grade`` get the friendly arg
        // path; everything else falls through to the generic
        // body-driven dispatch.
        let joined_action = format!("chat_{}", subop);
        if dispatch_resource_helper(client, resource, &joined_action, subop_args, mode)?.is_some() {
            return Ok(Some(()));
        }
        // Generic --body-driven dispatch as the fallback.
        let body = build_resource_body(subop_args)?;
        commands::glow::cmd_resource_action(
            client,
            resource.api_path(),
            &joined_action,
            body.as_deref(),
            mode,
        )?;
        return Ok(Some(()));
    }

    // tests invocation — invocation ops live as invocation_<op> on /test/.
    if resource == Tests && action == "invocation" {
        if rest.is_empty() || rest.iter().any(|a| a == "--help" || a == "-h") {
            print_invocation_namespace_help();
            return Ok(Some(()));
        }
        let subop = &rest[0];
        let subop_args = &rest[1..];
        let joined_action = format!("invocation_{}", subop);
        if dispatch_resource_helper(client, resource, &joined_action, subop_args, mode)?.is_some() {
            return Ok(Some(()));
        }
        let body = build_resource_body(subop_args)?;
        commands::glow::cmd_resource_action(
            client,
            resource.api_path(),
            &joined_action,
            body.as_deref(),
            mode,
        )?;
        return Ok(Some(()));
    }

    Ok(None)
}

fn print_chat_namespace_help() {
    use colored::Colorize;
    println!("{}\n", "glow attempts chat <op>".bold());
    println!("  {} /attempt/chat_<op>\n", "POST".dimmed());
    println!("  Chat operations live as ``chat_<op>`` on the attempt artifact's URL.\n");
    println!("{}:", "Common subops".bold());
    for (op, hint) in [
        ("get", "Fetch a chat by id"),
        ("create", "Create a new chat in an attempt"),
        (
            "message",
            "Send a message (helper: positional <chat_id> <text>)",
        ),
        (
            "grade",
            "Grade the chat (helper: positional <chat_id> [--score N])",
        ),
        ("complete", "Mark a chat completed"),
        ("response", "Submit a question response"),
        ("hints", "Fetch hints"),
        ("feedback", "Per-standard feedback"),
        ("strengths", "Per-message strengths"),
        ("improvements", "Per-message improvements"),
        ("analyses", "Overall analyses"),
        ("audio", "Attach an audio resource to a chat message"),
        (
            "voice",
            "[deferred] WS voice REPL — needs cpal/rodio go-ahead",
        ),
        ("silence", "Silence the voice session"),
        ("speak", "Push a speak frame into the voice session"),
        (
            "live",
            "Open the socket.io REPL — glow attempts chat live <chat_id>",
        ),
    ] {
        println!("  {:14} {}", op.bold(), hint.dimmed());
    }
    println!("\n{}:", "Options".bold());
    println!(
        "  {:<30} Raw JSON body (for ops without a helper)",
        "--body <json>".green()
    );
    println!("  {:<30} Output as JSON", "--json".green());
}

fn print_invocation_namespace_help() {
    use colored::Colorize;
    println!("{}\n", "glow tests invocation <op>".bold());
    println!("  {} /test/invocation_<op>\n", "POST".dimmed());
    println!("  Invocation operations live as ``invocation_<op>`` on the test artifact's URL.\n");
    println!("{}:", "Common subops".bold());
    for (op, hint) in [
        ("get", "Fetch an invocation"),
        ("create", "Create a new test invocation"),
        ("run", "Run an invocation"),
        ("complete", "Mark an invocation complete"),
        ("terminate", "Terminate an in-flight invocation"),
        ("trace", "Get the invocation trace"),
        ("draft", "Patch an invocation draft"),
        ("drafts", "List invocation drafts"),
    ] {
        println!("  {:14} {}", op.bold(), hint.dimmed());
    }
    println!("\n{}:", "Options".bold());
    println!("  {:<30} Raw JSON body", "--body <json>".green());
    println!("  {:<30} Output as JSON", "--json".green());
}

/// Dispatch per-resource media operations: glow <resource> <media> <action> [flags]
fn dispatch_media(
    client: &glow::GlowClient,
    resource: &str,
    media: resource::MediaType,
    args: &[String],
    mode: OutputMode,
) -> Result<()> {
    use commands::glow as glow_cmd;

    if args.is_empty() {
        anyhow::bail!(
            "Expected a media action. Available: upload, download, create, chunk, status, finalize, discover, preview"
        );
    }

    let action = args[0].as_str();
    let rest = &args[1..];

    match action {
        "upload" => {
            let file = parse_flag(rest, "--file")?
                .ok_or_else(|| anyhow::anyhow!("--file <path> is required for upload"))?;
            glow_cmd::cmd_media_upload(client, resource, media.slug(), &file, mode)
        }
        "download" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for download"))?;
            let output = parse_flag(rest, "--output")?;
            glow_cmd::cmd_media_download(client, resource, media.slug(), &id, output.as_deref(), mode)
        }
        "create" => {
            let filename = parse_flag(rest, "--filename")?
                .ok_or_else(|| anyhow::anyhow!("--filename is required for TUS create"))?;
            let size = parse_flag(rest, "--size")?
                .map(|s| s.parse::<u64>())
                .transpose()
                .map_err(|_| anyhow::anyhow!("--size must be a number"))?;
            glow_cmd::cmd_media_create(client, resource, media.slug(), &filename, size, mode)
        }
        "chunk" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for chunk"))?;
            let file = parse_flag(rest, "--file")?
                .ok_or_else(|| anyhow::anyhow!("--file <path> is required for chunk"))?;
            let offset = parse_flag(rest, "--offset")?
                .map(|s| s.parse::<u64>())
                .transpose()
                .map_err(|_| anyhow::anyhow!("--offset must be a number"))?
                .unwrap_or(0);
            glow_cmd::cmd_media_chunk(client, resource, media.slug(), &id, &file, offset, mode)
        }
        "status" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for status"))?;
            glow_cmd::cmd_media_status(client, resource, media.slug(), &id, mode)
        }
        "finalize" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for finalize"))?;
            let body = parse_flag(rest, "--body")?;
            glow_cmd::cmd_media_finalize(client, resource, media.slug(), &id, body.as_deref(), mode)
        }
        "discover" => {
            let id = parse_flag(rest, "--id")?;
            glow_cmd::cmd_media_discover(client, resource, media.slug(), id.as_deref(), mode)
        }
        "preview" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for preview"))?;
            glow_cmd::cmd_media_preview(client, resource, media.slug(), &id, mode)
        }
        other => anyhow::bail!(
            "Unknown media action '{}'. Available: upload, download, create, chunk, status, finalize, discover, preview",
            other,
        ),
    }
}

/// Build a JSON body string from extra args.
///
/// Supports two styles (can be combined):
///   --body '{"key": "value"}'         explicit JSON (merged first)
///   --key value --flag true           individual params folded into body
///
/// Values are auto-coerced: "true"/"false" → bool, integers/floats → number,
/// "null" → null, everything else → string.
fn build_resource_body(args: &[String]) -> Result<Option<String>> {
    use serde_json::{json, Map, Value};

    let explicit = parse_flag(args, "--body")?;
    let params = parse_params(args)?;

    if explicit.is_none() && params.is_empty() {
        return Ok(None);
    }

    // Start with explicit --body JSON if provided
    let mut obj: Map<String, Value> = match &explicit {
        Some(s) => {
            let v: Value = serde_json::from_str(s)
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --body: {}", e))?;
            match v {
                Value::Object(m) => m,
                _ => anyhow::bail!("--body must be a JSON object"),
            }
        }
        None => Map::new(),
    };

    // Merge --key value params (individual flags override --body keys)
    for (k, v) in params {
        obj.insert(k, coerce_value(&v));
    }

    Ok(Some(json!(obj).to_string()))
}

/// Parse --key value pairs from args, skipping --body and known media flags.
fn parse_params(args: &[String]) -> Result<Vec<(String, String)>> {
    let skip = [
        "--body",
        "--file",
        "--id",
        "--output",
        "--filename",
        "--size",
        "--offset",
        "--show-headers",
    ];
    let mut pairs = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if let Some(key) = args[i].strip_prefix("--") {
            if skip.contains(&args[i].as_str()) {
                i += 2; // skip flag + value
                continue;
            }
            if i + 1 >= args.len() {
                anyhow::bail!("--{} requires a value", key);
            }
            pairs.push((key.replace('-', "_"), args[i + 1].clone()));
            i += 2;
        } else {
            i += 1;
        }
    }
    Ok(pairs)
}

/// Auto-coerce a string value to the most appropriate JSON type.
fn coerce_value(s: &str) -> serde_json::Value {
    use serde_json::Value;
    match s {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "null" => Value::Null,
        _ => {
            if let Ok(n) = s.parse::<i64>() {
                Value::Number(n.into())
            } else if let Ok(f) = s.parse::<f64>() {
                serde_json::Number::from_f64(f)
                    .map(Value::Number)
                    .unwrap_or_else(|| Value::String(s.to_string()))
            } else {
                Value::String(s.to_string())
            }
        }
    }
}

/// Parse a --flag <value> pair from a slice of extra args
fn parse_flag(args: &[String], flag: &str) -> Result<Option<String>> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            if i + 1 < args.len() {
                return Ok(Some(args[i + 1].clone()));
            } else {
                anyhow::bail!("{} requires a value", flag);
            }
        }
        i += 1;
    }
    Ok(None)
}
