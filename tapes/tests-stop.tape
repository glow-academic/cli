# Stopping an in-flight run — `tests stop` halts queued/in-flight invocations
# attached to a test. It's the cancel surface (does NOT finalize the test
# record — that's `complete`). Real flow against the local instance. Takes an
# invocation_id (the per-cell run), returns success + a message.

Output demo-output/tests-stop.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Hide
# Off-camera: start a test + materialize a cell so there's a run to stop.
Type `EVAL=$(glow tests benchmark --json | jq -r '.evals[0].eval_id')`
Enter
Type `glow tests start --body '{"eval_id": "'$EVAL'"}' --json > /tmp/start.json`
Enter
Type `TID=$(jq -r .test_id /tmp/start.json); INV=$(jq -r .invocation_id /tmp/start.json)`
Enter
Type `INVID=$(glow tests invocation create --body '{"test_id": "'$TID'", "invocation_id": "'$INV'"}' --json | jq -r .invocation_id)`
Enter
Type "clear"
Enter
Show

# A materialized, in-flight invocation on the test.
Type `glow tests get --body '{"test_id": "'$TID'"}' --json | jq '.entries.invocations[] | {invocation_id, completed: .invocation_completed}'`
Enter
Sleep 3s

# Stop it — queued/in-flight work is cancelled (the test record itself is not
# finalized; use `complete` for that).
Type `glow tests stop --body '{"invocation_id": "'$INVID'"}' --json`
Enter
Sleep 4s
