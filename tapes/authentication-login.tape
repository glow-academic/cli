# Interactive login — the real PKCE/OAuth redirect round-trip, end to end.
#
# `glow login` spawns a localhost callback listener, opens the browser to the
# instance's /authorize, and blocks on the callback. The browser half (the
# Keycloak "Sign in to GLOW" form) is driven off-camera by a headless
# Playwright bridge (scripts/demo-login-bridge.mjs) so the recording is
# deterministic — VHS captures the terminal narrative; the bridge is the
# stand-in for the human at the browser. See also start-login.tape.
#
# Secrets (confidential-client GLOW_CLIENT_SECRET + the dev Keycloak creds)
# come from the untracked scripts/.demo-secrets.sh — never hardcoded here.

Output demo-output/authentication-login.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# --- off-camera: dev binary on PATH, instance + secret env, arm the bridge ---
Hide
Env PATH "/Users/ashoksaravanan/Coding/glow-academic-cli/target/release:/opt/homebrew/bin:/usr/bin:/bin"
Env GLOW_INSTANCE_URL "http://localhost:8000"
Type "source ./scripts/.demo-secrets.sh"
Enter
Type "rm -f /tmp/glow-login.log; glow logout >/dev/null 2>&1"
Enter
Sleep 500ms
# Disable job-control notifications so the backgrounded watcher's "[1]+ Done"
# line never leaks onto the recorded terminal.
Type "set +m"
Enter
# Watcher: as soon as `glow login` prints its /authorize URL into the log,
# hand it to the headless Playwright bridge, which fills the Keycloak form
# and lets the redirect chain land on the CLI's localhost callback.
Type `( for i in $(seq 1 150); do U=$(grep -oE "http://localhost:8000/authorize[^ ]*" /tmp/glow-login.log 2>/dev/null | head -1); [ -n "$U" ] && { node scripts/demo-login-bridge.mjs "$U" >/dev/null 2>&1; break; }; sleep 0.1; done ) &`
Enter
Type "clear"
Enter
Show

# --- on camera: the interactive login. stderr is tee'd to the log the
#     watcher scrapes; the user still sees the live handshake. ---
Type "glow login 2>&1 | tee /tmp/glow-login.log"
Sleep 800ms
Enter
Sleep 7s

# Token landed in ~/.config/glow/tokens.json — prove we're authenticated by
# making a real authenticated call with the freshly-minted bearer.
Type "glow personas search --page-size 4"
Enter
Sleep 4s
