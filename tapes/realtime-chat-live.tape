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
# Resolve real ids off-camera (the library reseeds, so don't hardcode them).
# Walk the attempts until we find one that actually carries a chat instance —
# a lobby attempt has current_chat_id=null and would 422 the socket. From that
# one `attempts get` payload we read everything:
# - AC = the messageable attempt_chat id (`current_chat_id`) — what
#   attempt.chat_message FKs to (attempt_message_entry.chat_id → attempt_chat).
# - UP = the learner persona (attempt.user_persona_id).
# - ASST = a NON-learner instance persona. resources.personas[].entry_id is the
#   per-attempt personas_entry id (the one attempt_content_entry.persona_id FKs
#   to); the template persona_ids from `chat get` are NOT in personas_entry and
#   would 500 the assistant reply on the FK.
Type `for a in $(glow attempts search --page-size 10 --json | jq -r '.data[].attempt_id'); do J=$(glow attempts get --body "{\"attempt_id\":\"$a\"}" --json); AC=$(echo "$J" | jq -r '.current_chat_id // empty'); if [ -n "$AC" ]; then UP=$(echo "$J" | jq -r .attempt.user_persona_id); ASST=$(echo "$J" | jq -r --arg up "$UP" '[.resources.personas[]|.entry_id|select(. != null and . != $up)][0]'); break; fi; done`
Enter
Type `( sleep 9; glow attempts chat message --body "{\"chat_id\":\"$AC\",\"text\":\"I'm right here — take your time and tell me what happened.\",\"persona_id\":\"$ASST\"}" >/dev/null 2>&1; sleep 9; glow attempts chat message --body "{\"chat_id\":\"$AC\",\"text\":\"That sounds really frustrating. Let's look at it together.\",\"persona_id\":\"$ASST\"}" >/dev/null 2>&1 ) &`
Enter
Sleep 400ms
Type "clear"
Enter
Show

Sleep 800ms

# Interactive terminal REPL over socket.io. Each stdin line emits
# attempt.chat_message; inbound events stream back between prompts.
Type `glow attempts chat live $AC --persona $UP`
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
