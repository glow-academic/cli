Output demo-output/attempts-start.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

Sleep 800ms

# A learner's home page — a simulation card they can start.
Type `glow attempts home --json | jq -c '.items[0] | {simulation_name, home_id, status}'`
Enter
Sleep 2800ms

# Resolve a real home_id off-camera (the library reseeds, so don't hardcode
# one — take the first card from the same home call).
Hide
Type `HOME=$(glow attempts home --json | jq -r '.items[0].home_id')`
Enter
Sleep 2000ms
Type "clear"
Enter
Sleep 400ms
Show

# Start it from the home entry — the attempt is minted and the first chat opens.
Type `glow attempts start --body "{\"home_id\":\"$HOME\"}" --json | jq '{attempt_id, chat_id}'`
Enter
Sleep 3500ms
