Output demo-output/start-deploy.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# After `glow deploy`, the instance is live — status reports each container's
# health (api stack: database + servers + keycloak, blue/green aware).
Type "glow status --name vhsdemo"
Enter
Sleep 4000ms
