Output demo-output/mcp-list-tools.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance's MCP endpoint, not --help.

Sleep 800ms

# List the MCP tools the instance exposes (JSON-RPC tools/list over /mcp/).
Type "glow mcp list-tools"
Enter
Sleep 4000ms
