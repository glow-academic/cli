Output demo-output/streaming-cli.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Human mode — off-camera trigger, then watch the live run. Each frame prints
# its event name + payload; the terminal frame breaks the loop and exits 0.
Hide
Type `RID=$(glow personas generate --json --body '{"instructions":["A strict but fair debate coach"],"wait_for_complete":false}' | jq -r .run_id)`
Enter
Sleep 600ms
Show
Sleep 500ms
Type `glow personas watch $RID`
Enter
Sleep 12s

# Same surface, machine-readable — one JSON object per frame on stdout, ready
# to pipe into jq. Same run lifecycle, two renderings.
Hide
Type `RID2=$(glow personas generate --json --body '{"instructions":["A cheerful intro-biology TA"],"wait_for_complete":false}' | jq -r .run_id)`
Enter
Sleep 600ms
Show
Sleep 500ms
Type `glow personas watch $RID2 --json`
Enter
Sleep 12s
