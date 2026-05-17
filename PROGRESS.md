# Glow Academic CLI — port progress

Sibling working tree to `glow-academic-api/`. Bootstrapped from
`learnloopllc-glow-cli` (Rust + clap), with new deploy + backup modules
that port the workflow logic captured in the deep-dive ops spec.

## State as of this session

| Area | Status |
|---|---|
| Crate bootstrap (renamed `glow-cli`→`cli`, version → 1.0.0, dropped Cargo.lock) | ✅ |
| `Cargo.toml` deps for deploy (dialoguer, indicatif, serde_yaml, pbkdf2, etc.) | ✅ |
| Deploy module skeleton (`src/deploy/`) — 9 files | ✅ |
| Backup module (`src/backup/mod.rs`) | ✅ |
| Bundled compose template (`src/deploy/templates/docker-compose.yml`) | ⚠ copied from `glow-academic-api/` — may need trimming for the project |
| Clap subcommands: `init` / `deploy` / `redeploy` / `stop` / `start` / `destroy` / `status` / `logs` / `backup {list,create,restore,delete}` | ✅ |
| `cargo check` / `cargo build` | ✅ clean (3 dead-code warnings on intentional latent helpers) |
| End-to-end smoke test | ⏳ untested against real docker — code paths exercised but no live deploy run yet |

## Architecture (mirrors the ops spec from `deploy.yml`)

```
src/
  bin/glow.rs                # thin shim → glow::run()
  bin/glw.rs                 # second bin (kept per design call)
  lib.rs                     # clap tree + dispatch
  deploy/
    mod.rs                   # orchestrator: deploy / stop / start / destroy / status
    bluegreen.rs             # detect → bring up → swap → monitor → rollback
    compose.rs               # writes the bundled docker-compose.yml
    config.rs                # glow-deploy.yaml schema + validate
    env.rs                   # write-once-then-merge .env + secret gen + AUTH_CLIENT_SECRET derivation
    healthcheck.rs           # docker inspect polling
    instance.rs              # paths + DeployState bookkeeping
    migrations.rs            # make migrate-docker invocations (add pre-promotion, remove post-stable)
    runtime.rs               # `docker compose` shellouts (v2 only)
    wizard.rs                # interactive `glow init` (dialoguer-based)
    templates/
      docker-compose.yml     # bundled via include_str!
  backup/
    mod.rs                   # list / create (pg_dump | gzip) / restore (drop+recreate+psql) / delete
```

### Per-instance on-disk layout

```
~/.glow/instances/<name>/
  glow-deploy.yaml           # source of truth (written by wizard, hand-editable)
  .env                       # generated; mode 0600; secrets written ONCE, owned-keys merged on redeploy
  docker-compose.yml         # rendered from bundled template (rewritten every deploy)
  .deploy-state.json         # active env, api_version, timestamps
  backups/*.sql.gz           # local backup files
```

`~/.config/glow/active-instance` holds the currently active name.

### Ops sequence (what `glow deploy` actually does)

Direct port of `deploy.yml`'s step-by-step:

1. **Assert docker compose v2** present.
2. **Materialize instance dir** + write bundled compose file.
3. **Load + validate `glow-deploy.yaml`** (origin, at least one ai provider + role).
4. **Load `DeployState`** + plan blue/green swap (next color = opposite of active, or "blue" on first run).
5. **First-deploy fork**: detect via missing `.env`; if redeploy, take a pre-deploy `pg_dump` to `backups/backup-deploy-<ts>.sql.gz` (rotates to last 7).
6. **Render `.env`**: first deploy = generate `SECRET_KEY`, `DEPLOYMENT_TOKEN`, `DB_PASSWORD`, `KEYCLOAK_ADMIN_PASSWORD`, derive `AUTH_CLIENT_SECRET` via PBKDF2-HMAC-SHA256 (100k iter, salt `glow-auth-secret-v1`, base64) — matches the deploy.yml derivation byte-for-byte. Redeploy = merge: only mutates owned keys (`API_VERSION`, `ORIGIN`, `ACTIVE_ENV`, `ACTIVE_KC_ENV`, `COMPOSE_PROJECT_NAME`, `DB_BACKUP`, `SEED_SETUP`, `GRACE_PERIOD_MINUTES`, `APP_PREFIX`).
7. **`docker compose pull` server-blue + server-green** — fail fast if image missing.
8. **`docker compose up -d database redis nginx`** — idempotent base.
9. **Wait database healthy** (180s timeout).
10. **Apply add migrations** (`make migrate-docker TYPE=add` inside the OLD color, on redeploy) — idempotent, runner skips already-applied.
11. **Bring up target color** (`server-<target>` + `keycloak-<target>`) and wait healthy.
12. **First-deploy fast-path**: commit state, switch nginx to the (only) running color, done.
13. **Redeploy path**: `bluegreen::switch_traffic` flips `ACTIVE_ENV` / `ACTIVE_KC_ENV` in `.env` and restarts nginx so upstream re-resolves.
14. **`bluegreen::monitor_then_commit_or_rollback`**: 30s warmup + N-min poll; if 3 consecutive unhealthy → flip ACTIVE_ENV back + restart nginx + bail with descriptive error.
15. **Apply remove migrations** (`make migrate-docker TYPE=remove`) — post-stable destructive drops.
16. **Persist `DeployState`** (active_env, api_version, last_deployed_at).

## Open work for next session

### Done in this session (after the initial bootstrap)
- ✅ Dropped `Instances` / `Use` subcommands + `InstanceCommands` enum + `dispatch_instances` fn (single-instance model committed)
- ✅ Rewrote `lib.rs` header comment (no more LearnLoop references)
- ✅ Trimmed the bundled compose template — image namespace now `ghcr.io/glow-academic/api`, default project name `glow-default`
- ✅ Added `src/deploy/preflight.rs` with docker-available + port-free + dir-writable + disk-space checks; wired into `deploy()` before any docker calls
- ✅ Stripped the 4 lifecycle workflows + `glow-deploy.yaml` from `glow-academic-api/`
- ✅ Updated `glow-academic-api/.github/workflows/pipeline.yml`: `ubuntu-latest` runners (was self-hosted), `ghcr.io/glow-academic/api` image namespace, simplified compat.json gen, dropped `v1` branch logic and version-band validation
- ✅ Set `AI_BASE_URL` default to empty in compose so deploys fail loud if user didn't configure an AI provider
- ✅ Cross-repo compat check restored end-to-end:
  - `glow-academic-api/api-versions.json` re-added with canonical shape (`cli`/`client`/`docs` → `min_version`)
  - `pipeline.yml` compat.json step reads `api-versions.json` and bakes `{component, version, requires}` into each release as `compat.json`
  - New `src/deploy/compat.rs` in CLI fetches the release's `compat.json` over HTTP, compares CLI's own `CARGO_PKG_VERSION` against `requires.cli.min_version`, and aborts with a clear upgrade message on mismatch. Network failure → warn + proceed (so a github outage doesn't block a deploy)
  - Wired into `deploy()` immediately after pre-flight checks, before `runtime::pull`, so we bail before touching docker state
- ✅ Two-stack (api + client) orchestration with 3 topology modes:
  - `airgapped` (default): both stacks on one public domain. Client nginx is the only public-facing service; api is reachable only via the client nginx's `/auth/`, `/api/`, `/mcp/`, `/socket.io/` proxies. Single TLS cert. Mirrors LearnLoop's `is_airgapped=True` model.
  - `exposed`: api + client on two domains. External callers (mobile, scripts, integrations) hit the api directly.
  - `api_only`: api on its own domain, no client stack. For users building their own UI.
  - `DeployConfig::topology` enum + `effective_api_origin()` / `effective_client_origin()` resolvers in `config.rs`
  - `env::derive_api_origins()` + `env::derive_client_public_urls()` implement the 3-mode decision table directly from the LearnLoop control-plane logic (`learnloopllc-api/core/app/routes/instances.py`)
  - Per-instance shared docker network created by `runtime::ensure_network` before either stack starts
  - Two compose templates bundled into the CLI: `api-compose.yml` (postgres + redis + nginx + blue/green server/keycloak) and `client-compose.yml` (app-blue/green + nginx + docker-gen)
  - Compose project names suffixed `-api` / `-client` so the stacks don't collide
  - Stack lifecycle: pull → base → wait DB → add migrations → bring up api target color → (skip if api_only) pull client → bring up client target color → swap api traffic + monitor → swap client traffic + monitor → remove migrations → persist state
  - `DeployState` tracks `active_client_env` + `client_version`; `Redeploy` falls back to last-deployed versions when omitted
  - `compat::check_client` fetches `glow-academic/client`'s `compat.json` and refuses if CLI < client.min_cli OR api < client.min_api

### TLS / fronting reverse proxy (v1 expectations)

Glow stacks listen on port 80 (HTTP). Users are responsible for terminating TLS *in front of* the CLI-managed compose stacks. Recommended patterns:

- **Caddy** (simplest, auto-Let's-Encrypt): one-line Caddyfile, `glow.example.com { reverse_proxy localhost:80 }`. Caddy handles ACME, cert renewal, HTTP→HTTPS upgrade. Works out of the box for airgapped (single domain) and api_only.
- **Traefik** with a `letsencrypt` resolver: heavier but more flexible if you're already running it for other services.
- **Cloudflare Tunnel**: zero exposed ports. `cloudflared tunnel` points at `http://localhost:80`; TLS terminates at Cloudflare's edge.
- **Cloud LB / managed ingress**: AWS ALB, GCP HTTPS LB, Fly.io edge — point at the host's port 80.

For `exposed` topology with two domains, the fronting proxy needs two routes:
- `glow.example.com` → `http://localhost:80` (client nginx)
- `api.example.com` → `http://localhost:81` (api nginx — set `CLIENT_HTTP_PORT=81` in api .env to avoid the port collision)

**Why we don't bundle Caddy in v1**: every fronting choice adds a supported install surface. v1.1 is a good time to add `--with-caddy` once we have user feedback on what they're actually fronting with.

### Still deferred (require real work — not just a clean rename)
1. **Smoke-test against real docker**: nothing run end-to-end yet. Needs `ghcr.io/glow-academic/api:v1.0.0` published or a locally-built image. Easiest path: tag the `glow-academic-api/` tree, push to glow-academic org, build via the new pipeline.yml, then run `glow init && glow deploy` against the published image.
2. **CI for the CLI repo itself** — `cargo build --release`, Homebrew bottle, copy the working pipeline.yml shape from glow-academic-api but for Rust.
3. **Wizard polish**: multi-AI-provider, per-role model assignment, Microsoft OIDC, GitHub OIDC, Custom OIDC. Currently covers the 80% case (one provider, one role, optional Google).
4. **Custom-domain + CNAME validation** — the web wizard had a polished version that probed DNS before allowing submit. Optional in v1.0.
5. **`docker compose --profile`** support — could let users opt out of nginx + keycloak (run just the api + db for embedded/dev use). Nice-to-have, not blocker.
6. **Drop the second binary `glw`**? Or keep — minor.
7. **`glow upgrade --to vX.Y.Z`** sugar wrapper around redeploy.

## Files I added (recap)

```
src/deploy/mod.rs
src/deploy/bluegreen.rs
src/deploy/compose.rs
src/deploy/config.rs
src/deploy/env.rs
src/deploy/healthcheck.rs
src/deploy/instance.rs
src/deploy/migrations.rs
src/deploy/runtime.rs
src/deploy/wizard.rs
src/deploy/templates/docker-compose.yml    # seeded from glow-academic-api/
src/backup/mod.rs
PROGRESS.md                                  # this file
```

Modified:
```
Cargo.toml          # renamed pkg, bumped to 1.0.0, added deploy deps
src/lib.rs          # +Init/Deploy/Redeploy/Stop/Start/Destroy/Status/Logs/Backup subcommands + dispatch
```

Untouched (intentional, port over from the old CLI as-is):
```
src/auth.rs         # OAuth + token store — reused as-is
src/config.rs       # ~/.config/glow/config.toml — TODO rename paths, defer
src/output.rs       # color/json formatting + confirm
src/api_common.rs   # HTTP helpers
src/resource.rs     # macro-generated resource enum
src/commands/glow/  # post-deploy CRUD commands
```
