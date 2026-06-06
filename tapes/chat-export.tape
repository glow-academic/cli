Output demo-output/chat-export.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

# Setup off-camera: resolve a real attempt_id (the library reseeds, so don't
# hardcode one — take the first row from a search), then pull its
# chat_entry_id. The command substitutions block, so $CEID is set before the
# on-camera export.
Hide
Type `ATTEMPT=$(glow attempts search --page-size 1 --json | jq -r '.data[0].attempt_id')`
Enter
Sleep 2000ms
Type `CEID=$(glow attempts get --body "{\"attempt_id\":\"$ATTEMPT\"}" --json | jq -r '.chat_entry_id')`
Enter
Sleep 2500ms
Show

Sleep 600ms

# Export one chat — a denormalized CSV row of every resolved resource
# (rubric, scenarios, personas, prompts, parameters, …) ready to drop
# into a review packet.
Type `glow attempts chat export --body "{\"chat_entry_id\":\"$CEID\"}"`
Enter
Sleep 3500ms
