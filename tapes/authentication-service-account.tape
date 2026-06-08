Output demo-output/authentication-service-account.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Capture the recorder-supplied service token, then strip GLOW_TOKEN from the
# env off-camera so the STORED token is genuinely what authenticates the call
# below — i.e. the real `glow login --token` path, not the env shortcut. The
# token is the pre-issued service credential the recorder injects (never
# hardcoded here).
Hide
Type `SA_TOKEN="$GLOW_TOKEN"; unset GLOW_TOKEN`
Enter
Sleep 400ms
Type "clear"
Enter
Sleep 300ms
Show

Sleep 800ms

# Headless CI flow: a service account authenticates with a pre-issued token —
# no browser, no OAuth redirect. The runner just stores the JWT.
Type `glow login --token "$SA_TOKEN"`
Enter
Sleep 2200ms

# Immediately authenticated — list personas straight away, no further setup.
Type "glow personas search --page-size 3"
Enter
Sleep 3500ms
