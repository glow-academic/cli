Output demo-output/chat-export.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

# Setup off-camera: pull a real chat_entry_id from a completed attempt.
# The command substitution blocks until the lookup completes, so $CEID is
# set before the on-camera export.
Hide
Type `CEID=$(glow attempts get --body '{"attempt_id":"019e51c8-cb24-7b2b-9d6b-fbf1e903aa8a"}' --json | jq -r '.chat_entry_id')`
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
