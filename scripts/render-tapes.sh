#!/usr/bin/env bash
set -euo pipefail

if ! command -v vhs >/dev/null 2>&1; then
  echo "VHS is required to record CLI demos."
  echo "Install it locally first: https://github.com/charmbracelet/vhs"
  exit 1
fi

mkdir -p demo-output
cargo build

# Make tapes/bin/glow + glw resolvable as bare `glow` / `glw` in the
# recorded shell — so the demo shows `glow --help` instead of
# `./target/debug/glow --help`. The wrappers exec the freshly-built
# debug binary; VHS inherits this PATH when it spawns its shell.
export PATH="$(pwd)/tapes/bin:$PATH"

# Bypass-auth env so canonical tapes can run REAL resource commands against the
# local instance (mirrors Playwright's authAs / the _smoke tape) — no per-tape
# Env block needed; VHS inherits the exported env when it spawns its shell.
# Override by exporting any of these before running render-tapes.sh.
: "${GLOW_INSTANCE_URL:=http://localhost:8000}"
: "${GLOW_TOKEN:=90e4f963cb1ffd0853553e97f851fc6c1180690426b9c4c6a3ec332ee14a65c9}"
: "${GLOW_E2E_PROFILE_ID:=102ea140-ca00-5c6a-9133-68e18a675a0e}"
export GLOW_INSTANCE_URL GLOW_TOKEN GLOW_E2E_PROFILE_ID

# Polish each rendered tape into a "Screen Studio"-style video on a soft
# gradient background — same treatment and final dimensions as the web client's
# Playwright demos (1600x900 content -> 1856x1156 framed). Skip with NO_POLISH=1.
# The polished file sits beside the raw one as <name>.polished.mp4.
polish() {
  local tape="$1"
  [[ "${NO_POLISH:-0}" == "1" ]] && return 0
  if ! command -v node >/dev/null 2>&1; then
    echo "  (node not found — skipping polish for $tape)"
    return 0
  fi
  # The output path is whatever the tape's `Output` directive names.
  local out
  out="$(grep -m1 -E '^Output[[:space:]]' "$tape" | awk '{print $2}')"
  if [[ -z "$out" || ! -f "$out" ]]; then
    echo "  (no rendered output for $tape — skipping polish)"
    return 0
  fi
  node scripts/polish-video.mjs "$out"
}

render() {
  local tape="$1"
  vhs "$tape"
  polish "$tape"
}

if [[ $# -gt 0 && -n "${1:-}" ]]; then
  tape="tapes/${1}.tape"
  if [[ ! -f "$tape" ]]; then
    echo "Tape not found: $tape"
    exit 1
  fi
  render "$tape"
  exit 0
fi

shopt -s nullglob
tapes=(tapes/*.tape)
if [[ ${#tapes[@]} -eq 0 ]]; then
  echo "No VHS tapes found in tapes/."
  exit 1
fi

for tape in "${tapes[@]}"; do
  render "$tape"
done
