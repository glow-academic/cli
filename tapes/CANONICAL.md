# Canonical VHS clips — plan & status

Goal: every VHS tape should show the **real flow** (live commands against a local
instance), not `glow … --help`. These clips are embedded one-to-one in the docs
(`../glow-academic-docs`, `<DemoVideo topic="<name>" kind="vhs" />` →
`public/demos/<name>.mp4`). There are **53** `kind="vhs"` slots.

## Foundation (done)
- All tapes record at **1600×900** (FontSize 24) → `scripts/polish-video.mjs`
  wraps them on the gradient background → **1856×1156** (same framing as the
  web/Playwright clips).
- `scripts/render-tapes.sh` exports the bypass-auth env
  (`GLOW_INSTANCE_URL` / `GLOW_TOKEN` / `GLOW_E2E_PROFILE_ID`) so tapes inherit
  it — **no per-tape `Env` block**; canonical tapes just run real commands.
- Transfer to docs is **manual**: when a clip is final,
  `cp demo-output/<topic>.polished.mp4 ../glow-academic-docs/public/demos/<topic>.mp4`,
  then in docs `node scripts/gen-demo-manifest.mjs` + commit. (No cross-repo script.)
- **Mutation cleanup discipline:** use VHS `Hide` / `Show` to run setup +
  teardown off-camera — `Hide` → create a test entity → `Show` → demo it →
  `Hide` → delete it (`<resource> delete --body '{"<x>_ids":["…"]}'`). Only the
  real flow is recorded.

## Tier 1 — read/export-safe, DONE ✅ (recorded, polished, copied to docs)
| topic | command |
|---|---|
| `permissions-context` | `glow profiles context --json \| jq .caller_permissions` |
| `settings-sections`   | `glow settings get --json \| jq 'keys'` then `… \| jq '.colors'` |
| `health-refresh`      | `glow system refresh` → `glow system health` |
| `activity-export`     | `glow system export --body '{"view":"activity"}'` → `glow system refresh` |
| `home-export`         | `glow attempts search` → `glow attempts export --body '{"attempt_id":"…","view":"single"}'` |
| `attempts-start`      | `glow attempts home` (a card's `home_id`) → `glow attempts start --body '{"home_id":"…"}'` (mints the attempt; no model calls) |

This is the full safe set — every other clip needs an unblock (below). Valid
`system export` views: `activity`, `pricing`. `attempts export` needs an
`attempt_id` + `view:"single"` (uses an existing seed attempt; read-only — both
exports just generate a server-side ZIP and return its name/row_count).

## Tier 2 — mutations (need a target + Hide/Show cleanup)
- `agents-generate`, `generation-trigger`, `generation-wait` → `glow agents generate --body '{…}'`
  (AI gen: slow, nondeterministic, costs tokens; clean up the produced draft).
- `scenarios-generate` → `glow scenarios generate --body '{…}'`.
- `evals-run` → `glow evals run --body '{…}'`.
- `voice-upload` → audio upload via attempts; `documents-upload` → `glow documents …` upload (need a fixture file).
- `tests-start` / `tests-stop` / `tests-grade` / `tests-decrypt` → `glow tests <action>`: `tests start` needs an `eval_id` (runs an eval = model calls/token cost); `grade`/`decrypt` need an `invocation_id` (+ `key_id`) from a run. DEFERRED with the generate clips (token cost).
- **Attempts lifecycle — UNBLOCKED via `glow attempts home`:** the CLI
  dispatches any action to `POST /attempt/<action>`, so `glow attempts home`
  (and `practice`) work even though they're not in the "common" list. `home`'s
  `.items[]` carry `home_id`; `attempt/start` needs only `{"home_id":…}` (profile
  comes from the session). Cleanup = `glow attempts archive --body
  '{"attempt_ids":[…],"archived":true}'` (no `attempt/delete` — that 404s; an
  extra attempt is legitimate data anyway, archive just tidies).
  - `attempts-start` ✅ DONE (mint only — no model calls).
  - `attempts-complete` → `glow attempts complete --body '{"attempt_id":…}'` runs
    the **rubric grader** (LLM grading = token cost) → DEFERRED with generate.
  - `attempts-stop` → truncate a **streaming** chat reply mid-response (needs a
    live AI response in flight, i.e. an active `generate`) → DEFERRED with generate.

## Tier 3 — live streaming/realtime (gated on `generate` → deferred with it)
`chat-live`, `realtime-chat-live` (`glow attempts chat live`), `streaming-cli`/`-connect`/`-overview`/`-terminal`, `realtime-connect` (`glow attempts watch <run_id>`), `invocation-trace`, `tools-invocation`. `attempts watch` streams a run's SSE *until a terminal event* — only meaningful while a **`generate` is in flight** (otherwise it just emits the terminal frame). So these need an active AI run = same bucket as generate. `attempts-stop` (truncate a streaming reply) is here too.

## Tier 4 — MCP (blocked on auth)
`mcp-overview`, `mcp-list-tools`, `mcp-call` — `glow mcp list-tools`/`call` return
**HTTP 401**; `/mcp/` uses different auth than the resource endpoints. Unblock the
MCP auth (or use a real session token), then: `list-tools`, then `call <tool> --args '{…}'`.

## Tier 5 — deploy / instance ops (need Docker / real infra; some destructive)
`start-deploy`, `start-init` (interactive wizard), `topology-airgapped`/`-api-only`/`-exposed`,
`backup-create`. Consider a throwaway instance, or keep conceptual. `start-overview`
is legitimately an overview (`glow --help` + `glow status`) — fine to keep help-style.

## Tier 6 — interactive auth (device/OAuth flow, hard to script)
`authentication-login`, `start-login` (`glow login`), `authentication-service-account`.

## Tier 7 — need HTTP headers / SQL, not a `glow` command
- `data-layer-cache` — two identical `/system/groups` calls showing `X-Cache-Hit: 0` then `1` (curl with `-i`, or a CLI flag that surfaces headers).
- `data-layer-bypass` — mutate, then re-read with `X-Bypass-Cache: 1` header.
- `data-layer-mvs` — a `pg_matviews` SQL `SELECT` (psql), e.g. the `personas_mv` definition.
- `group-media-downloads` — binary audio via `POST /system/audio_download` (range request) + media viewer.
- `group-lean-vs-detail` — caption wants lean vs `include_detail:true`, but
  `glow system group` returns the same `runs` either way here — the lean/detail
  distinction isn't exposed by the CLI param. Investigate the right param/flag.
- `documents-upload`, `voice-upload`, `media-chunked` — web upload flows; no CLI
  upload command (`documents create` only takes JSON). Treat as web-only.

## Tapes that don't exist yet (5)
`authentication-service-accounts`, `backup-overview`, `backup-storage`, `data-layer-mvs`, `media-chunked`.
