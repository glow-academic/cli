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

# Resolve a real home_id off-camera (the library reseeds, so don't hardcode one).
Hide
Type `HOME=$(glow attempts home --json | jq -r '.items[0].home_id')`
Enter
Sleep 2000ms
Show

# Start an attempt and capture its id...
Type `ATT=$(glow attempts start --body "{\"home_id\":\"$HOME\"}" --json | jq -r .attempt_id); echo "started: $ATT"`
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
