Output demo-output/health-refresh.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

Sleep 800ms

# Rebuild the hourly materialized views; the response lists the cache tags it
# invalidated.
Type "glow system refresh"
Enter
Sleep 3500ms

Type "clear"
Enter
Sleep 400ms

# Re-read health — now reflecting the freshly-rebuilt views.
Type "glow system health"
Enter
Sleep 3000ms
