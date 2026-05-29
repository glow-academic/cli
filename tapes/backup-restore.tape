Output demo-output/backup-restore.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Setup off-camera: resolve the newest snapshot's filename into $BK so the
# on-camera restore reads naturally.
Hide
Type `BK=$(ls -t ~/.glow/instances/vhsdemo/backups/ | head -1)`
Enter
Sleep 800ms
Show

Sleep 600ms

# Roll the database back to a snapshot. Restore terminates the API servers'
# live sessions, drops + recreates the DB (FORCE), re-streams the dump, then
# bounces the server pools so they reconnect to the fresh database.
Type `glow backup restore "$BK" --name vhsdemo -y`
Enter
Sleep 7000ms
