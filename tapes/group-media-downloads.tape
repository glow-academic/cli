Output demo-output/group-media-downloads.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token).

# Setup off-camera: upload an audio clip and capture its audio_id. (The
# command substitution blocks until the upload completes, so $AUD is set
# before the on-camera download.)
Hide
Type `AUD=$(glow attempts audio_upload --file tapes/fixtures/clip.wav --json | jq -r .audio_id)`
Enter
Sleep 3000ms
Show

Sleep 600ms

# Pull a stored audio back by id — POST /system/audio_download streams the
# bytes straight to disk.
Type `glow system audio download --id $AUD --output /tmp/clip-out.wav`
Enter
Sleep 4000ms
