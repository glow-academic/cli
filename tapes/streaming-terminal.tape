Output demo-output/streaming-terminal.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# A run that fails: `generate --wait` fires the trigger, attaches to
# /persona/watch, and the very first event is the terminal
# `persona.generate.failed` envelope — the provider rejected the call. The CLI
# prints the failure payload and exits with code 1, so a script's exit status
# tells you the run failed without polling a status endpoint.
Type `glow personas generate --wait --body '{"instructions":["A persona whose run will fail"]}'`
Enter
Sleep 6s

# The exit code reflects the terminal event: 1 on .failed.
Type `echo "exit code: $?"`
Enter
Sleep 2s
