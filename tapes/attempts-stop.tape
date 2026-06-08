Output demo-output/attempts-stop.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Off-camera: post a learner message, fire the in-character AI turn (real
# streaming agent), and arm a background Stop that fires ~6s in. `set +m`
# suppresses job-control notices.
Hide
Type `set +m`
Enter
# Resolve real ids off-camera (the library reseeds, so don't hardcode them):
# the first searched attempt with a chat, its chat_entry_id, and its learner
# (user) persona.
Type `ATT=$(glow attempts search --page-size 5 --json | jq -r '[.data[]|select(.attempt_id)][0].attempt_id')`
Enter
Type `CE=$(glow attempts get --body "{\"attempt_id\":\"$ATT\"}" --json | jq -r .chat_entry_id)`
Enter
Type `UP=$(glow attempts get --body "{\"attempt_id\":\"$ATT\"}" --json | jq -r .attempt.user_persona_id)`
Enter
Type `glow attempts chat message --body "{\"chat_id\":\"$CE\",\"text\":\"Can you explain the whole rubric to me in detail?\",\"persona_id\":\"$UP\"}" >/dev/null 2>&1`
Enter
Type `GEN=$(glow attempts generate --json --body "{\"instructions\":[\"Respond to the user's latest message in character.\"],\"config\":{\"operations\":[\"chat_message\",\"get\"],\"params\":{\"attempt_id\":\"$ATT\",\"chat_id\":\"$CE\"}},\"wait_for_complete\":false}"); RID=$(echo "$GEN" | jq -r .run_id); GID=$(echo "$GEN" | jq -r .group_id)`
Enter
Type `( sleep 6; glow attempts stop --body "{\"group_id\":\"$GID\"}" >/dev/null 2>&1 ) &`
Enter
Sleep 400ms
Type "clear"
Enter
Show

Sleep 600ms

# Watch the run live: the assistant's reply streams in token by token, then
# Stop fires — the stream truncates cleanly and the watch returns control.
Type `glow attempts watch $RID --group-id $GID`
Enter
Sleep 11s
