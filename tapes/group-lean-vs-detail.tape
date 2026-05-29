Output demo-output/group-lean-vs-detail.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# Lean: the table view — one row per run group: id, run count, cost, time span.
# Cheap to scan for audit linking + spend.
Type `glow system groups --body '{"page_size":6}'`
Enter
Sleep 3500ms

Type "clear"
Enter
Sleep 400ms

# Detail: the full JSON row behind a single table line — group id, run count,
# exact first/last run timestamps, and session — the complete record.
Type `glow system groups --body '{"page_size":1}' --json | jq '.data[0] | {group_id, run_count, first_run_at, last_run_at, session_id}'`
Enter
Sleep 4s
