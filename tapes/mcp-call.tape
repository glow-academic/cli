Output demo-output/mcp-call.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance's MCP endpoint, not --help.

Sleep 800ms

# Call an MCP tool by name. --args is a JSON object, validated against the
# tool's inputSchema before dispatch (JSON-RPC tools/call over /mcp/).
Type `glow mcp call search_content --args '{"artifact":"persona","page_size":3}'`
Enter
Sleep 4000ms
