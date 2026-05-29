Output demo-output/topology-exposed.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# Exposed topology: the API and the client each get their OWN domain + cert
# (api.example.edu and glow.example.edu), unlike airgapped where the API has
# no public origin.
Type "grep -E 'topology|api_origin|client_origin' ~/.glow/instances/exposed-demo/glow-deploy.yaml"
Enter
Sleep 3200ms

Type "clear"
Enter
Sleep 400ms

# Two origins means the browser makes cross-origin calls — CORS is what keeps
# that handshake honest. The API's generated env allows exactly the client's
# origin (CLIENT_ORIGINS), with the API on its own ORIGIN.
Type "grep -E 'CLIENT_ORIGINS|^ORIGIN=' ~/.glow/instances/exposed-demo/api/.env"
Enter
Sleep 4500ms
