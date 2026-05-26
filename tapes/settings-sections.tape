Output demo-output/settings-sections.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

Sleep 800ms

# The platform settings resolve as one composed object — every section in a
# single get.
Type "glow settings get --json | jq 'keys'"
Enter
Sleep 3500ms

Type "clear"
Enter
Sleep 400ms

# Drill into one section — the colors palette.
Type "glow settings get --json | jq '.colors'"
Enter
Sleep 3000ms
