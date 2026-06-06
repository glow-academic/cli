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

# Resolve a real attempt_id off-camera (the library reseeds, so don't
# hardcode one — capture the first row from the same search).
Hide
Type `ATTEMPT=$(glow attempts search --page-size 1 --json | jq -r '.data[0].attempt_id')`
Enter
Sleep 2000ms
Show

# Export it — a denormalized ZIP with the certificate of completion baked in.
Type `glow attempts export --body "{\"attempt_id\":\"$ATTEMPT\",\"view\":\"single\"}"`
Enter
Sleep 3000ms
