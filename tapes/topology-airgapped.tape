Output demo-output/topology-airgapped.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# Airgapped topology: one public domain, one cert. The deploy config picks it,
# and the API never gets its own origin — it lives behind the client.
Type "grep -E 'topology|origin' ~/.glow/instances/my-school/glow-deploy.yaml"
Enter
Sleep 3000ms

Type "clear"
Enter
Sleep 400ms

# The generated client nginx proxies the auth + API endpoints through to the
# internal API container — Browser -> client nginx -> (proxied) api nginx.
Type "grep -B1 -A2 -iE 'airgapped|OIDC proxy' ~/.glow/instances/my-school/client/default.conf.template"
Enter
Sleep 4500ms
