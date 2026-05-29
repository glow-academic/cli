Output demo-output/streaming-connect.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Off-camera: fire an async persona generate and capture its run_id so the curl
# below attaches while the run is still emitting. `clear` (also hidden) wipes
# the trigger line out of the scrollback so `Show` resumes on a clean prompt.
Hide
Type `RID=$(glow personas generate --json --body '{"instructions":["A nurturing kindergarten teacher"],"wait_for_complete":false}' | jq -r .run_id)`
Enter
Sleep 600ms
Type "clear"
Enter
Show

Sleep 500ms

# Raw HTTP: curl --no-buffer (-N) against /persona/watch. The response is
# Content-Type: text/event-stream — one `data:` line per envelope, streamed as
# the generator emits them. The server closes the stream on the terminal frame.
Type `curl -sN "$GLOW_INSTANCE_URL/persona/watch?run_id=$RID" -H "Authorization: Bearer $GLOW_TOKEN" -H "X-E2E-Profile-Id: $GLOW_E2E_PROFILE_ID" -H "Accept: text/event-stream"`
Enter
Sleep 14s
