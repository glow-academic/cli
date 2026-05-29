# Running an eval — the benchmark dashboard is the batch of evals; kick one
# off, materialize a cell from its template, and score the run against the
# rubric (full=true → the rubric ceiling). Real flow against the local
# instance, token-free (no LLM re-run — the grade is the scored payoff).

Output demo-output/evals-run.mp4

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
Env GLOW_E2E_PROFILE_ID "102ea140-ca00-5c6a-9133-68e18a675a0e"
Type "clear"
Enter
Show

# The benchmark surface is the eval-run dashboard — every eval and its status.
Type `glow tests benchmark --json | jq -r '.evals[:6][] | "  \(.status)  \(.eval_name)"'`
Enter
Sleep 3s
Hide
Type `EVAL=$(glow tests benchmark --json | jq -r '.evals[0].eval_id')`
Enter
Show

# Kick off a run on the first eval — returns the parent benchmark + the first
# template card to score.
Type `glow tests start --body '{"eval_id": "'$EVAL'"}' --json | tee /tmp/run.json`
Enter
Sleep 3s
Hide
Type `TID=$(jq -r .test_id /tmp/run.json); INV=$(jq -r .invocation_id /tmp/run.json)`
Enter
Type `INVID=$(glow tests invocation create --body '{"test_id": "'$TID'", "invocation_id": "'$INV'"}' --json | jq -r .invocation_id)`
Enter
Show

# Score the run against the eval's rubric — full=true anchors to the rubric
# ceiling (the evaluator model's max). Returns the grade + pass flag.
Type `glow tests grade --body '{"invocation_id": "'$INVID'", "full": true}' --json`
Enter
Sleep 4s
