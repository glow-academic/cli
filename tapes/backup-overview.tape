Output demo-output/backup-overview.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# `glow backup list` is the CLI's inventory view of an instance's
# snapshots — name + size, newest first.
Type "glow backup list --name vhsdemo"
Enter
Sleep 2800ms

# Take a fresh labeled snapshot — pg_dump | gzip into the instance's
# backups/ dir.
Type "glow backup create --name vhsdemo --label nightly -y"
Enter
Sleep 3500ms

# It shows up immediately in the inventory.
Type "glow backup list --name vhsdemo"
Enter
Sleep 3000ms
