# Onboarding login — `glow login` opening the browser, the Keycloak
# round-trip, and the token landing in the local config store. Same real
# OAuth flow as authentication-login.tape (headless Playwright bridge drives
# the Keycloak form off-camera); the payoff here is the stored token itself.
#
# The token file holds bearers for *every* instance you've logged into, so
# the payoff is scoped to the localhost entry and the JWTs are truncated —
# never dump the whole file on camera.

Output demo-output/start-login.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# --- off-camera: dev binary, instance + secret env, arm the bridge ---
Hide
Env PATH "/Users/ashoksaravanan/Coding/glow-academic-cli/target/release:/opt/homebrew/bin:/usr/bin:/bin"
Env GLOW_INSTANCE_URL "http://localhost:8000"
Type "source ./scripts/.demo-secrets.sh"
Enter
Type "rm -f /tmp/glow-login.log; glow logout >/dev/null 2>&1"
Enter
Type "set +m"
Enter
Type `TOKENS="$HOME/Library/Application Support/glow/tokens.json"`
Enter
Type `( for i in $(seq 1 150); do U=$(grep -oE "http://localhost:8000/authorize[^ ]*" /tmp/glow-login.log 2>/dev/null | head -1); [ -n "$U" ] && { node scripts/demo-login-bridge.mjs "$U" >/dev/null 2>&1; break; }; sleep 0.1; done ) &`
Enter
Type "clear"
Enter
Show

# --- on camera: the interactive login ---
Type "glow login 2>&1 | tee /tmp/glow-login.log"
Sleep 800ms
Enter
Sleep 7s

# The bearer + refresh metadata landed in the local config store, keyed by
# instance URL. Scoped to localhost, JWTs truncated.
Type `jq '."http://localhost:8000" | {token_type, expires_in, access_token: (.access_token[:20] + "…"), token_endpoint}' "$TOKENS"`
Enter
Sleep 4s
