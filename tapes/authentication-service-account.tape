Output demo-output/authentication-service-account.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Strip the inherited bypass env (render-tapes.sh exports GLOW_TOKEN) off-camera
# so the STORED token is genuinely what authenticates the call below — i.e. the
# real `glow login --token` path, not the env shortcut.
Hide
Type `unset GLOW_TOKEN`
Enter
Sleep 400ms
Type "clear"
Enter
Sleep 300ms
Show

Sleep 800ms

# Headless CI flow: a service account authenticates with a pre-issued token —
# no browser, no OAuth redirect. The runner just stores the JWT.
Type `glow login --token 90e4f963cb1ffd0853553e97f851fc6c1180690426b9c4c6a3ec332ee14a65c9`
Enter
Sleep 2200ms

# Immediately authenticated — list personas straight away, no further setup.
Type "glow personas search --page-size 3"
Enter
Sleep 3500ms
