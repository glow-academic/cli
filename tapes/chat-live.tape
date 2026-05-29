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
Type `CE=019e73be-7ce4-76e2-ae90-172d6671b2c8; UP=019e73be-7c45-750f-9fe2-3dbecf549950; ASST=019e73be-7ce2-78c6-a2f3-57110e0ed9b7`
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
