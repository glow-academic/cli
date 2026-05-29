Output demo-output/topology-api-only.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# api_only topology: a single API-side container stack — database, redis,
# keycloak, and the blue/green api servers behind one nginx. There is NO
# client/UI container; you drive everything from the CLI or your own backend.
Type "glow status --name vhsdemo"
Enter
Sleep 4000ms
