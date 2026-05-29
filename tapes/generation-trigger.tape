Output demo-output/generation-trigger.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# POST /persona/generate — fire a single instructions string. The call returns
# in ~200ms with a run_id + group_id while the model generates in the background.
Type `glow personas generate --body '{"instructions":["A calm, experienced ICU charge nurse who mentors students"]}'`
Enter
Sleep 4000ms
