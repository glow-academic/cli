Output demo-output/backup-storage.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# Backups are plain gzipped pg_dumps on disk under the instance dir —
# manual snapshots (manual-<label>-<ts>.sql.gz) sit right next to the
# auto pre-deploy snapshots a redeploy writes.
Type "ls -lh ~/.glow/instances/vhsdemo/backups/"
Enter
Sleep 3500ms
