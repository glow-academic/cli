Output demo-output/authentication-service-accounts.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Strip the inherited bypass env off-camera so the stored token is genuinely
# what authenticates (render-tapes.sh exports GLOW_TOKEN, which would win).
Hide
Type `unset GLOW_TOKEN`
Enter
Sleep 400ms
Type "clear"
Enter
Sleep 300ms
Show

Sleep 800ms

# In CI the service-account JWT comes from a secret store, not a prompt —
# here it lands in an env var the runner already has.
Type `export GLOW_SA_TOKEN=90e4f963cb1ffd0853553e97f851fc6c1180690426b9c4c6a3ec332ee14a65c9`
Enter
Sleep 700ms

# The runner authenticates non-interactively — no browser, no redirect.
Type `glow login --token "$GLOW_SA_TOKEN"`
Enter
Sleep 1800ms

# Then it just scripts against the API, fully authenticated.
Type "glow personas search --page-size 3"
Enter
Sleep 3500ms
