Output demo-output/tools-invocation.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Off-camera: post a learner question, then fire the in-character AI turn. The
# agent's first move is always a tool call (Attempt_Get) — that's the
# invocation we're watching. `set +m` suppresses job-control notices.
Hide
Type `set +m`
Enter
Type `ATT=019e73be-7c48-7fa8-a771-4139b972d3f9; CE=019e73be-7ce4-76e2-ae90-172d6671b2c8; UP=019e73be-7c45-750f-9fe2-3dbecf549950`
Enter
Type `glow attempts chat message --body "{\"chat_id\":\"$CE\",\"text\":\"What's the late penalty on the midterm?\",\"persona_id\":\"$UP\"}" >/dev/null 2>&1`
Enter
Type `GEN=$(glow attempts generate --json --body "{\"instructions\":[\"Respond in character.\"],\"config\":{\"operations\":[\"chat_message\",\"get\"],\"params\":{\"attempt_id\":\"$ATT\",\"chat_id\":\"$CE\"}},\"wait_for_complete\":false}"); RID=$(echo "$GEN" | jq -r .run_id); GID=$(echo "$GEN" | jq -r .group_id)`
Enter
Sleep 400ms
Type "clear"
Enter
Show

Sleep 600ms

# Watch the run: each tool the agent invokes surfaces as a call.start frame
# carrying the tool name + arguments, then call.complete with the result —
# and the assistant's reply streams after, fed by what the tools returned.
Type `glow attempts watch $RID --group-id $GID`
Enter
Sleep 15s
