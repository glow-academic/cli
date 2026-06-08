# Invocation trace — POST /test/invocation_trace opens a trace entry for a
# test invocation: the bundle config + run binding that anchors replay and
# audit. Real flow against the local instance, token-free (no LLM re-run —
# materialize a cell, then open its trace).

Output demo-output/invocation-trace.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Hide
Type `EVAL=$(glow tests benchmark --json | jq -r '.evals[0].eval_id')`
Enter
Type `glow tests start --body '{"eval_id": "'$EVAL'"}' --json > /tmp/t.json`
Enter
Type `TID=$(jq -r .test_id /tmp/t.json); INV=$(jq -r .invocation_id /tmp/t.json)`
Enter
Type "clear"
Enter
Show

# Materialize a cell from the benchmark template — the test invocation we'll trace.
Type `INVID=$(glow tests invocation create --body '{"test_id": "'$TID'", "invocation_id": "'$INV'"}' --json | jq -r .invocation_id) && echo "test_invocation: $INVID"`
Enter
Sleep 3s

# Open its trace — pins the invocation's bundle config + run binding into a
# trace entry. The returned id is the replay/audit anchor for this cell.
Type `glow tests invocation trace --body '{"test_id": "'$TID'", "test_invocation_id": "'$INVID'"}' --json`
Enter
Sleep 4s
