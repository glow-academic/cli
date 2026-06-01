# Smoke tape — proves the bypass-auth path works end-to-end through
# VHS, and shows off the human-readable table rendering (src/render.rs).
# Hits a real ``personas search`` / ``attempts search`` against the local
# uvicorn on :8000 using GLOW_TOKEN. Mirror of what
# Playwright's authAs() does on the browser side.

Output demo-output/_smoke-personas-search.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 40ms

# Use the freshly-built dev binary, not the stale 1.0.5 on PATH.
Env PATH "/Users/ashoksaravanan/Coding/glow-academic-cli/target/release:/opt/homebrew/bin:/usr/bin:/bin"
Env GLOW_INSTANCE_URL "http://localhost:8000"
Env GLOW_TOKEN "90e4f963cb1ffd0853553e97f851fc6c1180690426b9c4c6a3ec332ee14a65c9"

# Pretty table — names tinted with each persona's own color, AI/MCP/draft
# badges in the FLAGS column, a dimmed "N of M" footer.
Type "glow personas search"
Enter
Sleep 4s

# Attempts use a different envelope (data[]); progress + colored score.
Type "glow attempts search --page-size 6"
Enter
Sleep 4s

# Detail view: ``get <id>`` renders the selected values as a key-value
# panel (name tinted by its color, a color swatch, departments, …). The id
# below is from the fixed local seed ("Aggressive (High)"); swap it if you
# reseed. ``--json`` from a search row gives you the id to paste here.
Type "glow personas get f55438bb-2ae6-54f5-b184-2f3d4ec5d945"
Enter
Sleep 4s

# --json still prints the raw response untouched (works in any position).
Type "glow personas search --page-size 2 --json | head -20"
Enter
Sleep 3s
