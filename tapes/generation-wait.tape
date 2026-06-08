Output demo-output/generation-wait.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# `generate --wait` fuses trigger + watch into one blocking call: it fires
# POST /persona/generate (returning a run_id immediately), then attaches to
# GET /persona/watch?run_id=… and streams every live SSE frame until the
# terminal event — exiting 0 on the green "Run completed." line.
Type `glow personas generate --wait --body '{"instructions":["A calm, experienced ICU charge nurse who mentors students"]}'`
Enter
# Live DGX generation takes ~13-19s; wait past the terminal "Run completed." line.
Sleep 22s
