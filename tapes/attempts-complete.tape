Output demo-output/attempts-complete.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

Sleep 800ms

# Start an attempt and capture its id...
Type `ATT=$(glow attempts start --body '{"home_id":"019e485b-3526-70e1-bf97-df0d45d308ad"}' --json | jq -r .attempt_id); echo "started: $ATT"`
Enter
Sleep 2600ms

# ...then complete it — the grader runs across its chats and finalizes the attempt.
Type `glow attempts complete --body "{\"attempt_id\":\"$ATT\"}" --json`
Enter
Sleep 3500ms

# Tidy up off-camera: archive the demo attempt (legitimate data; archive keeps the list clean).
Hide
Type `glow attempts archive --body "{\"attempt_ids\":[\"$ATT\"],\"archived\":true}" >/dev/null 2>&1`
Enter
Show
