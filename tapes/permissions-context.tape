Output demo-output/permissions-context.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL are exported by scripts/render-tapes.sh (bypass token),
# so these are REAL calls against the local instance — not --help.

Sleep 800ms

# Resolve the live permission context for the current caller: the resource
# description plus exactly what this caller may do against it.
Type "glow profiles context"
Enter
Sleep 3500ms

Type "clear"
Enter
Sleep 400ms

# Zoom in on the capability flags the server computed for this caller.
Type "glow profiles context --json | jq .caller_permissions"
Enter
Sleep 3500ms
