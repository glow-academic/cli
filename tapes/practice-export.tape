Output demo-output/practice-export.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token) — these
# are REAL calls against the local instance, not --help.

Sleep 800ms

# Export the practice view — a profile-level, denormalized ZIP of every
# practice attempt. Same shape as the Home export, but without the
# official-training certificate gating, so it just builds and returns.
Type `glow attempts export --body '{"view":"practice"}'`
Enter
Sleep 3500ms
