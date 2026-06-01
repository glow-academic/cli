Output demo-output/data-layer-bypass.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

# Auth + instance URL come from scripts/render-tapes.sh (bypass token).

Sleep 800ms

# A normal read is served straight from Redis (x-cache-hit: 1)...
Type "glow system groups --page-size 3 --show-headers | grep -i x-cache-hit"
Enter
Sleep 2800ms

Type "clear"
Enter
Sleep 400ms

# ...but sending X-Bypass-Cache: 1 forces a fresh read past the cache
# (x-cache-hit: 0) — see the new row without waiting for the refresh cycle.
Type `curl -s -D - -o /dev/null -X POST "$GLOW_INSTANCE_URL/system/groups" -H "Authorization: Bearer $GLOW_TOKEN" -H "X-Bypass-Cache: 1" -H "Content-Type: application/json" -d '{"page_size":3}' | grep -i x-cache-hit`
Enter
Sleep 3000ms
