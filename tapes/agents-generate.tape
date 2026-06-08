Output demo-output/agents-generate.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Setup off-camera: open an agent draft and capture its id (generation targets
# a draft, proving the model/tool/rubric wiring before going live).
Hide
Type `DID=$(glow agents draft --body '{"instructions":["A deadline-checking study coach"]}' --json | jq -r .draft_id)`
Enter
Sleep 2500ms
Type "clear"
Enter
Sleep 300ms
Show

Sleep 600ms

# POST /agent/generate against the draft — returns a run_id + group_id while
# the model generates the agent config in the background.
Type `glow agents generate --body "{\"draft_id\":\"$DID\"}"`
Enter
# Fire-and-return: the POST returns a run_id + group_id quickly, but give the
# round-trip headroom on a live DGX-backed instance.
Sleep 6s
