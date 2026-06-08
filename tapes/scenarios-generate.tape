Output demo-output/scenarios-generate.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# Hand the generator a brief and it spins up draft scenarios — each pre-wired
# with personas + objectives. The trigger returns a run_id; generation
# continues in the background.
Type `glow scenarios generate --body '{"prompt":"Three office-hours variants escalating in intensity"}'`
Enter
# Fire-and-return: the POST returns a run_id quickly, but give the round-trip
# headroom on a live DGX-backed instance.
Sleep 6s
