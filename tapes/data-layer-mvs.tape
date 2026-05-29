Output demo-output/data-layer-mvs.mp4

Set Shell "bash"
Set Width 1600
Set Height 900
Set FontSize 24
Set Padding 20
Set TypingSpeed 35ms

Sleep 800ms

# The data layer's materialized read models live in Postgres — list them
# straight from pg_matviews.
Type `psql -h localhost -p 5432 -d glowapi -tAc "SELECT matviewname FROM pg_matviews ORDER BY 1 LIMIT 8"`
Enter
Sleep 2600ms

Type "clear"
Enter
Sleep 400ms

# Pull personas_mv's definition live from pg_matviews — the materialized read
# model the API serves persona lists from.
Type `psql -h localhost -p 5432 -d glowapi -tAc "SELECT definition FROM pg_matviews WHERE matviewname = 'personas_mv'"`
Enter
Sleep 3500ms
