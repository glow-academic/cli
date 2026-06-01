# Grading a test row — start a test, materialize a cell, then grade it with
# `full: true` to anchor the score to the rubric ceiling (the server resolves
# the rubric and assigns score = total_points). Real flow, token-free: no LLM
# re-run, just the grade write. Mirrors the Test Grade agent's Test_Grade call.

Output demo-output/tests-grade.mp4

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
# Off-camera setup: start a test + materialize one cell to grade.
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

# An invocation row, not yet graded.
Type `glow tests get --body '{"test_id": "'$TID'"}' --json | jq '.entries.invocations[] | {invocation_id, grade_id, grade_score}'`
Enter
Sleep 3s

# Grade with full=true — server resolves the rubric and scores to the ceiling.
Type `glow tests grade --body '{"invocation_id": "'$INVID'", "full": true}' --json`
Enter
Sleep 3500ms

# The grade landed on the row — score + pass flag, ready for per-standard feedback.
Type `glow tests refresh >/dev/null && glow tests get --body '{"test_id": "'$TID'"}' --json | jq '.entries.grades[] | {grade_id, score, passed}'`
Enter
Sleep 4s
