Output demo-output/start-init.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# `glow init` is the interactive wizard — it scaffolds an instance's
# glow-deploy.yaml by walking through origin, AI provider + key, the text
# model, a seed template, and optional OIDC.
Type "glow init --name my-school"
Enter
Sleep 1800ms

# Public origin URL (where clients connect).
Type "https://glow.my-school.edu"
Enter
Sleep 1400ms

# Primary AI provider — accept the default (openai).
Enter
Sleep 1400ms

# Provider API key (input hidden).
Type "sk-demo-Ab12Cd34Ef56Gh78"
Enter
Sleep 1400ms

# Model for the text role — accept the default (gpt-4.1).
Enter
Sleep 1400ms

# Seed template — accept the default (fresh, empty DB).
Enter
Sleep 1400ms

# Add Google OIDC? — accept the default (no).
Enter
Sleep 3000ms
