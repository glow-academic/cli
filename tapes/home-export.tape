Output demo-output/home-export.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

Sleep 800ms

# Find a completed attempt in the library.
Type "glow attempts search --page-size 3"
Enter
Sleep 3000ms

Type "clear"
Enter
Sleep 400ms

# Export it — a denormalized ZIP with the certificate of completion baked in.
Type `glow attempts export --body '{"attempt_id":"019e51c8-cb24-7b2b-9d6b-fbf1e903aa8a","view":"single"}'`
Enter
Sleep 3000ms
