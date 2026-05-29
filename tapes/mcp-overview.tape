Output demo-output/mcp-overview.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance's MCP endpoint, not --help.

Sleep 800ms

# List the tools the instance exposes over MCP...
Type "glow mcp list-tools"
Enter
Sleep 3500ms

Type "clear"
Enter
Sleep 400ms

# ...then invoke one — the same JSON-RPC envelope a real MCP client would send.
Type `glow mcp call search_content --args '{"artifact":"persona","page_size":3}'`
Enter
Sleep 4000ms
