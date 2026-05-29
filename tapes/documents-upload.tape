Output demo-output/documents-upload.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token).

Sleep 800ms

# Upload a PDF to the document store (multipart POST /document/file_upload).
# The server writes the upload chain and returns the file_id to attach.
Type "glow documents file_upload --file tapes/fixtures/course-syllabus.pdf"
Enter
Sleep 3500ms
