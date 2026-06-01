# Starting a test — pick a benchmark-backed eval, fan out into per-cell
# invocations, and watch the rows materialize. Real flow against the local
# instance via the bypass-auth env (GLOW_TOKEN), mirror
# of Playwright's authAs(). Token-free: materializes invocations directly with
# `tests invocation create` (no LLM generate).

Output demo-output/tests-start.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Hide
Env PATH "/Users/ashoksaravanan/Coding/glow-academic-cli/target/release:/opt/homebrew/bin:/usr/bin:/bin"
Env GLOW_INSTANCE_URL "http://localhost:8000"
Env GLOW_TOKEN "90e4f963cb1ffd0853553e97f851fc6c1180690426b9c4c6a3ec332ee14a65c9"
Type "clear"
Enter
Show

# A benchmark is the read-side of an eval — the grid of cells to run. Pick one.
Type `glow tests benchmark --json | jq -r '.evals[0] | "\(.eval_name)   \(.eval_id)"'`
Enter
Sleep 2500ms
Hide
Type `EVAL=$(glow tests benchmark --json | jq -r '.evals[0].eval_id')`
Enter
Show

# Start the test — materializes the wrapper and returns the parent benchmark,
# the first template card, and the test id every other op targets.
Type `glow tests start --body '{"eval_id": "'$EVAL'"}' --json | tee /tmp/start.json`
Enter
Sleep 3s
Hide
Type `TID=$(jq -r .test_id /tmp/start.json); INV=$(jq -r .invocation_id /tmp/start.json)`
Enter
Show

# Materialize a cell from the benchmark template (test_id + template id is a
# complete call — the server resolves models→agents, rubrics, and the bundle).
Type `glow tests invocation create --body '{"test_id": "'$TID'", "invocation_id": "'$INV'"}' --json`
Enter
Sleep 3s

# The per-cell invocation rows have landed — each with its resolved agent.
Type `glow tests refresh >/dev/null && glow tests get --body '{"test_id": "'$TID'"}' --json | jq '.entries.invocations[] | {invocation_id, agent_ids, completed: .invocation_completed}'`
Enter
Sleep 4s
