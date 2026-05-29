Output demo-output/data-layer-cache.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token).

Sleep 800ms

# Two identical /system/groups reads, back to back. First is a cache MISS —
# the server computes it and writes Redis (x-cache-hit: 0).
Type "glow system groups --page-size 3 --show-headers | grep -i x-cache"
Enter
Sleep 3000ms

Type "clear"
Enter
Sleep 400ms

# Second is a cache HIT — served straight from Redis (x-cache-hit: 1).
Type "glow system groups --page-size 3 --show-headers | grep -i x-cache"
Enter
Sleep 3000ms
