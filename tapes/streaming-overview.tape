Output demo-output/streaming-overview.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Off-camera: fire an async persona generate (wait_for_complete:false returns a
# run_id immediately while the model works in the background) and capture its
# run_id so the watch below attaches to a still-live run.
Hide
Type `RID=$(glow personas generate --json --body '{"instructions":["A patient algebra tutor who never gives away the answer"],"wait_for_complete":false}' | jq -r .run_id)`
Enter
Sleep 600ms
Show

Sleep 600ms

# `watch <run_id>` opens GET /persona/watch?run_id=… — a Server-Sent Events
# stream scoped to that one run. Live envelopes stream in; the terminal frame
# closes the stream and the CLI exits 0.
Type `glow personas watch $RID`
Enter
Sleep 14s
