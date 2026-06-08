Output demo-output/realtime-chat-live.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Off-camera: ids + a background "assistant" posting in-character replies timed
# to land after each learner line (manual orchestration — real chat_messages
# over socket.io, no LLM). `set +m` suppresses job-control "Done" notices.
Hide
Type `set +m`
Enter
# Resolve real ids off-camera (the library reseeds, so don't hardcode them):
# the first searched attempt's chat_entry_id (CE) + its learner persona (UP),
# and an assistant persona (ASST = any chat persona other than the learner).
Type `ATT=$(glow attempts search --page-size 5 --json | jq -r '[.data[]|select(.attempt_id)][0].attempt_id')`
Enter
Type `CE=$(glow attempts get --body "{\"attempt_id\":\"$ATT\"}" --json | jq -r .chat_entry_id)`
Enter
Type `UP=$(glow attempts get --body "{\"attempt_id\":\"$ATT\"}" --json | jq -r .attempt.user_persona_id)`
Enter
Type `ASST=$(glow attempts chat get --body "{\"chat_id\":\"$CE\"}" --json | jq -r --arg up "$UP" '[.personas[]|select(.persona_id!=$up)][0].persona_id')`
Enter
Type `( sleep 9; glow attempts chat message --body "{\"chat_id\":\"$CE\",\"text\":\"I'm right here — take your time and tell me what happened.\",\"persona_id\":\"$ASST\"}" >/dev/null 2>&1; sleep 9; glow attempts chat message --body "{\"chat_id\":\"$CE\",\"text\":\"That sounds really frustrating. Let's look at it together.\",\"persona_id\":\"$ASST\"}" >/dev/null 2>&1 ) &`
Enter
Sleep 400ms
Type "clear"
Enter
Show

Sleep 800ms

# Interactive terminal REPL over socket.io. Each stdin line emits
# attempt.chat_message; inbound events stream back between prompts.
Type `glow attempts chat live $CE --persona $UP`
Enter
Sleep 3s

Type `Hi — can I talk to someone about my grade?`
Enter
Sleep 8s

Type `I worked really hard and still got a C.`
Enter
Sleep 8s

Ctrl+D
Sleep 1500ms
