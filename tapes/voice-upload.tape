Output demo-output/voice-upload.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token).

Sleep 800ms

# Upload audio (multipart). The server writes the audio chain and returns the
# canonical audios_id — the same handle the pre-reserved + promote shapes hit.
Type "glow attempts audio_upload --file tapes/fixtures/clip.wav"
Enter
Sleep 3500ms
