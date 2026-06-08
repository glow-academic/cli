Output demo-output/chat-live.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Off-camera: ids for a seeded attempt chat (entry id + the learner's
# user_persona + the assistant persona), and a background "assistant" that
# posts two in-character replies timed to land just after each learner line.
# This is manual orchestration — the replies are real chat_messages over the
# socket, no LLM in the loop — so the REPL shows them stream back inline.
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
Type `( sleep 9; glow attempts chat message --body "{\"chat_id\":\"$CE\",\"text\":\"Of course — which question's score felt off to you?\",\"persona_id\":\"$ASST\"}" >/dev/null 2>&1; sleep 9; glow attempts chat message --body "{\"chat_id\":\"$CE\",\"text\":\"Let's walk through the Q3 rubric together, step by step.\",\"persona_id\":\"$ASST\"}" >/dev/null 2>&1 ) &`
Enter
Sleep 400ms
Type "clear"
Enter
Show

Sleep 800ms

# `chat live` opens a socket.io REPL: each line you type is emitted as
# attempt.chat_message; inbound events stream back inline between prompts.
Type `glow attempts chat live $CE --persona $UP`
Enter
Sleep 3s

Type `Can we review my midterm grade?`
Enter
Sleep 8s

Type `Yes — I think Q3 was marked unfairly.`
Enter
Sleep 8s

Ctrl+D
Sleep 1500ms
