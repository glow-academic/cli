# Glow — CLI

Glow is source-available for academic, research, educational, and other noncommercial use under the [PolyForm Noncommercial License 1.0.0](./LICENSE).

Commercial use requires a separate written license from Purdue Research Foundation / Purdue University. Contact: ashok@learn-loop.org.

This repository contains the **CLI** — the canonical tool for deploying and managing a Glow instance. It is one of four components in the Glow platform:

| Component | What it is |
|---|---|
| [api](https://github.com/glow-academic/api) | FastAPI backend |
| [client](https://github.com/glow-academic/client) | Next.js frontend |
| **cli** (this repo) | Rust CLI — what end users actually install |
| [docs](https://github.com/glow-academic/docs) | Nextra docs site |

## Install

```bash
brew tap glow-academic/tap
brew install glow             # stable
# or:
brew install glow-beta        # pre-release channel
```

For non-Homebrew installs, download a prebuilt binary from [releases](https://github.com/glow-academic/cli/releases) for your platform (`darwin-arm64`, `darwin-x64`, `linux-arm64`, `linux-x64`).

## Quick start

```bash
glow init       # interactive wizard writes ~/.glow/instances/default/glow-deploy.yaml
glow deploy     # pulls api + client images, brings up the stack
glow status     # show container health + active blue/green colors
glow redeploy   # update to a new api/client version with zero-downtime swap
glow stop / start / destroy
glow backup list / create / restore
```

See the [docs](https://glow-academic.github.io/docs/) for the full deployment guide, including topology modes (`airgapped`, `exposed`, `api_only`), TLS setup, and configuration reference.

## Post-deploy CRUD

Once an instance is up, the CLI talks to its API directly. The user-facing command names are **plural** (`personas`, `scenarios`, `simulations`, ...) for natural reading; the wire path is whatever the frozen API exposes (mostly singular). The mapping happens inside `resource.rs`.

```bash
# Auth
glow login                    # OAuth redirect flow
glow login --token <jwt>      # Service-account / programmatic
glow logout                   # Fires POST /logout server-side, then clears local token
glow context                  # Current identity

# Generic dispatch — works for any (resource, action) pair
glow <resource> <action> [--body '{"key":"value"}'] [--flag value ...]

# Ergonomic helpers — positional + named flags for the 80% case
glow attempts start <simulation_id> [--persona <id>] [--cohort <id>]
glow attempts message <chat_id> <text> [--audio <file>] [--persona <id>]
glow attempts grade <chat_id> [--score N]
glow scenarios generate <modality> <title> [--instructions <text>]
glow personas search [--name <pattern>]
glow simulations list [--cohort <id>]

# View-namespace commands (groups is a system view, not a top-level artifact)
glow groups history [--limit N] [--date-from ISO] [--date-to ISO]

# Watch a run: blocks until terminal (.completed / .failed) lifecycle event
glow attempts watch <run_id> [--group-id <id>]
glow generate <group_id> --wait [--artifact attempt]

# Streaming
glow stream --artifact attempt --operation generate --entity-id <id>

# Chat live REPL over socket.io (untested — see PROGRESS.md)
glow chats live <chat_id> [--persona <id>]

# MCP — JSON-RPC against the Glow instance's /mcp/ endpoint
glow mcp list-tools
glow mcp call <tool_name> [--args '{"key":"value"}']
```

### Resources

Plural CLI name → singular API path mapping (full list lives in `src/resource.rs`):

| CLI | API path | About |
|---|---|---|
| personas | persona | AI personas |
| scenarios | scenario | Simulation scenarios |
| simulations | simulation | Simulation configurations |
| documents | document | Document management |
| agents | agent | AI agents |
| models | model | AI model configurations |
| providers | provider | AI provider integrations |
| parameters | parameter | Configuration parameters |
| fields | field | Custom fields |
| tools | tool | Tool integrations |
| departments | department | Organizational departments |
| cohorts | cohort | User cohorts |
| evals | eval | Evaluation configurations |
| rubrics | rubric | Grading rubrics |
| profiles | profile | User profiles |
| auths | auth | Authentication records |
| settings | setting | Instance settings |
| attempts | attempt | Simulation attempts |
| tests | test | Test sessions |

### OpenAPI type regen

```bash
make sync-types                       # against $GLOW_API_URL (default http://localhost:8000)
# or with a local openapi.json file:
python3.11 scripts/generate-rust-types.py --file /path/to/openapi.json --output src/glow/api/latest.rs
```

## Local development

```bash
cargo build
cargo run -- --help
cargo test
```

## License

This project is licensed under the [PolyForm Noncommercial License 1.0.0](./LICENSE).

This is not an OSI-approved open-source license. It is intended to support academic and research dissemination while preserving separate commercial licensing rights.
