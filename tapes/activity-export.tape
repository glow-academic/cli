Output demo-output/activity-export.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

Sleep 800ms

# Export the activity view — the server builds a denormalized ZIP for archival
# and returns its file name + row count.
Type `glow system export --body '{"view":"activity"}'`
Enter
Sleep 3500ms

Type "clear"
Enter
Sleep 400ms

# Refresh the materialized views so the next page-load picks up new sessions.
Type "glow system refresh"
Enter
Sleep 3000ms
