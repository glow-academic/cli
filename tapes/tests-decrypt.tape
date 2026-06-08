# Decryption surface — POST /test/decrypt reveals the plaintext of a key
# scoped to an invocation. The key_id must belong to the invocation's key_ids
# (the authorization boundary) else 403. Real flow against the local instance.
#
# Setup (off-camera, pre-wired): a throwaway dummy key ("Demo Decrypt Key",
# plaintext "glow-demo-key-DO-NOT-USE-…") is attached to a seeded benchmark
# invocation via ../glow-academic-api/scripts/setup-decrypt-demo-key.py.
# Re-run that script after a reseed to refresh the ids below. The plaintext is
# a deliberate non-secret so it is safe to reveal on camera.

Output demo-output/tests-decrypt.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Hide
Type "INV=019e485a-e5fa-71a2-8c6f-5abb9c4cd33b; KEY=019e748d-ef25-709b-b04c-418ec2fa05cd"
Enter
Type "clear"
Enter
Show

# A test invocation references a key only by id — the value is encrypted at
# rest. /test/decrypt verifies the key belongs to this invocation, then
# returns the plaintext (scoped, audited).
Type `glow tests decrypt --body '{"invocation_id": "'$INV'", "key_id": "'$KEY'"}' --json`
Enter
Sleep 4s

# The authorization boundary: a key_id NOT in the invocation's key_ids is
# rejected with 403 — knowing a key_id alone is not enough.
Type `glow tests decrypt --body '{"invocation_id": "'$INV'", "key_id": "00000000-0000-0000-0000-000000000000"}' --json 2>&1 | head -2`
Enter
Sleep 3s
