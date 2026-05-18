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

if [[ $# -gt 0 && -n "${1:-}" ]]; then
  tape="tapes/${1}.tape"
  if [[ ! -f "$tape" ]]; then
    echo "Tape not found: $tape"
    exit 1
  fi
  vhs "$tape"
  exit 0
fi

shopt -s nullglob
tapes=(tapes/*.tape)
if [[ ${#tapes[@]} -eq 0 ]]; then
  echo "No VHS tapes found in tapes/."
  exit 1
fi

for tape in "${tapes[@]}"; do
  vhs "$tape"
done
