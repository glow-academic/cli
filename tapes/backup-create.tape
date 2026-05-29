Output demo-output/backup-create.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# A pg_dump runs inside the database container, gzipped onto the host.
Type "glow backup create --name vhsdemo --label pre-migration"
Enter
Sleep 6000ms

Type "clear"
Enter
Sleep 400ms

# The backup is now on the host, ready for restore.
Type "glow backup list --name vhsdemo"
Enter
Sleep 3000ms
