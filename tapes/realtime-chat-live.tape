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
Type `CE=019e73be-7ce4-76e2-ae90-172d6671b2c8; UP=019e73be-7c45-750f-9fe2-3dbecf549950; ASST=019e73be-7ce2-78c6-a2f3-57110e0ed9b7`
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
