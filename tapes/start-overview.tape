Output demo-output/start-overview.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 30ms

Sleep 800ms

# Zero to running is four commands — install, scaffold, deploy, verify:
Type "echo '1) brew install glow-academic/tap/glow   # install the CLI'"
Enter
Sleep 900ms
Type "echo '2) glow init my-school                   # scaffold glow-deploy.yaml'"
Enter
Sleep 900ms
Type "echo '3) glow deploy --name my-school          # blue/green bring-up'"
Enter
Sleep 900ms
Type "echo '4) glow status --name my-school          # confirm all green'"
Enter
Sleep 1500ms

# Steps 1–3 each have their own demo; here's step 4 against a live stack —
# every service healthy.
Type "glow status --name vhsdemo"
Enter
Sleep 4000ms
