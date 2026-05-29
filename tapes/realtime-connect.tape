Output demo-output/realtime-connect.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Setup off-camera: start an attempt to get a real chat_id.
Hide
Type `CID=$(glow attempts start --body '{"home_id":"019e485b-3526-70e1-bf97-df0d45d308ad"}' --json | jq -r .chat_id)`
Enter
Sleep 2500ms
Type "clear"
Enter
Sleep 300ms
Show

Sleep 600ms

# socket.io v4 handshake: the bearer token rides in the connect auth payload
# (not an Authorization header). The server resolves identity once, at connect.
Type `glow attempts chat live "$CID"`
Enter
Sleep 4500ms
Ctrl+D
Sleep 1500ms
