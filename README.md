# how to run
- cargo build
- cargo install
- run cli binary
- usage:
```sh
# query 
#   - from Premier League's season 23/24
#   - the 10 players who scored the most, and
sport-cli analytics \
    --sport football \
    --event "Premier's League"  \
    --date-from "2024-08-17"  \
    --date-to "2025-05-25"  \
    --dimension "player" \
    --metric "score" \
    --sort "desc" \
    --limit 10 \
    --timeout 2000

# query 
#   - from Premier League's season 23/24
#   - the 10 players who assisted the most.
sport-cli analytics \
    --sport football \
    --event "Premier's League"  \
    --date-from "2024-08-17"  \
    --date-to "2025-05-25"  \
    --dimension "player" \
    --metric "assist" \
    --sort "desc" \
    --limit 10 \
    --timeout 2000
```