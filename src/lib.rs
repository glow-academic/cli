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
mod resource;
mod serve;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use output::OutputMode;

#[derive(Parser)]
#[command(name = "glow", about = "Glow CLI — manage your platform", version)]
struct Cli {
    /// LearnLoop API URL (central platform)
    #[arg(long, env = "GLOW_API_URL")]
    api_url: Option<String>,

    /// Glow instance URL
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
    /// Authenticate with Glow instance (OAuth)
    Login,

    /// Remove stored authentication token for Glow instance
    Logout,

    /// Check Glow instance health
    Health,

    /// Show current user context and identity
    Context,

    /// Emulate another user profile
    Emulate {
        /// Target profile ID to emulate
        target_profile_id: String,
        /// TTL in minutes (default: 120 = 2 hours)
        #[arg(long, default_value = "120")]
        ttl: u32,
    },

    /// Stop emulating and return to your own profile
    Unemulate,

    /// Generate content for a group
    Generate {
        /// Group ID to generate for
        group_id: String,
        /// JSON body for additional options
        #[arg(long)]
        body: Option<String>,
    },

    /// Stream events via SSE (Server-Sent Events)
    Stream {
        /// Artifact type to stream
        #[arg(long)]
        artifact: String,
        /// Operation to stream (e.g. create, update, delete)
        #[arg(long)]
        operation: String,
        /// Filter by entity ID
        #[arg(long)]
        entity_id: Option<String>,
        /// Cursor for resuming from a position
        #[arg(long)]
        cursor: Option<String>,
        /// Event type filter (comma-separated)
        #[arg(long)]
        types: Option<String>,
        /// Max events per batch (1-200)
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Create a stream session (returns session ID)
    Connect,

    /// Destroy a stream session
    Disconnect {
        /// Session ID to destroy
        sid: String,
    },

    /// Report a problem
    Problem {
        /// Problem type
        #[arg(long, alias = "type")]
        kind: String,
        /// Problem description
        #[arg(long)]
        message: String,
    },

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
            .map(|r| json!({ "slug": r.slug(), "about": r.about() }))
            .collect();
        obj.insert("resources".into(), json!(resources));

        let media_types: Vec<&str> = resource::MediaType::all_slugs().to_vec();
        obj.insert("media_types".into(), json!(media_types));

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
        Commands::Login => {
            println!("TODO: instance login");
        }
        Commands::Logout => {
            println!("TODO: instance logout");
        }
        Commands::Health => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_health(&client, mode)?
        }
        Commands::Context => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_context(&client, mode)?
        }
        Commands::Emulate {
            target_profile_id,
            ttl,
        } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_emulate(&client, &target_profile_id, ttl, mode)?
        }
        Commands::Unemulate => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_unemulate(&client, mode)?
        }
        Commands::Generate { group_id, body } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_generate(&client, &group_id, body.as_deref(), mode)?
        }
        Commands::Stream {
            artifact,
            operation,
            entity_id,
            cursor,
            types,
            limit,
        } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_stream(
                &client,
                &artifact,
                &operation,
                entity_id.as_deref(),
                cursor.as_deref(),
                types.as_deref(),
                limit,
                mode,
            )?
        }
        Commands::Connect => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_connect(&client, mode)?
        }
        Commands::Disconnect { sid } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_disconnect(&client, &sid, mode)?
        }
        Commands::Problem { kind, message } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_problem(&client, &kind, &message, mode)?
        }

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

    if args.len() < 2 {
        anyhow::bail!(
            "Expected an action for resource '{}'. Example: glow {} search",
            resource.slug(),
            resource.slug(),
        );
    }

    // Check if second arg is a media type (file, image, text, audio, video)
    if let Some(media) = resource::MediaType::from_str(&args[1]) {
        return dispatch_media(client, resource.slug(), media, &args[2..], mode);
    }

    let action = &args[1];

    // Show help for resource action
    if args[2..].iter().any(|a| a == "--help" || a == "-h") {
        use colored::Colorize;
        println!(
            "{}\n",
            format!("glow {} {}", resource.slug(), action).bold()
        );
        println!("  {} /{}/{}\n", "POST".dimmed(), resource.slug(), action);
        println!("  Pass --key value pairs as needed. The API will validate parameters.\n");
        println!("{}:", "Options".bold());
        println!(
            "  {:<30} Raw JSON body (can combine with flags)",
            "--body <json>".green()
        );
        println!("  {:<30} Output as JSON", "--json".green());
        return Ok(());
    }

    let body = build_resource_body(&args[2..])?;

    commands::glow::cmd_resource_action(client, resource.slug(), action, body.as_deref(), mode)
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
