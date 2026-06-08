# Smoke tape — proves the auth path works end-to-end through VHS, and shows
# off the human-readable table rendering (src/render.rs). Hits a real
# ``personas search`` / ``attempts search`` against the recorder-supplied
# instance using the recorder-supplied GLOW_TOKEN (set by `glow record`, not
# hardcoded here). Mirror of what Playwright's authAs() does on the browser side.

Output demo-output/_smoke-personas-search.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 40ms

# Pretty table — names tinted with each persona's own color, AI/MCP/draft
# badges in the FLAGS column, a dimmed "N of M" footer.
Type "glow personas search"
Enter
Sleep 4s

# Attempts use a different envelope (data[]); progress + colored score.
Type "glow attempts search --page-size 6"
Enter
Sleep 4s

# Detail view: ``get <id>`` renders the selected values as a key-value panel
# (name tinted by its color, a color swatch, departments, …). Resolve a real
# persona id off-camera from the search above (the library reseeds, so don't
# hardcode one) and feed it to `get`.
Hide
Type `PID=$(glow personas search --page-size 1 --json | jq -r '.personas[0].id // .personas[0].persona_id')`
Enter
Sleep 2000ms
Show
Type "glow personas get $PID"
Enter
Sleep 4s

# --json still prints the raw response untouched (works in any position).
Type "glow personas search --page-size 2 --json | head -20"
Enter
Sleep 3s
