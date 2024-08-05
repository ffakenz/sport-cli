## Install
cargo install --path ./app --bin sport-cli

### Usage
```sh
# query 
#   - from Premier League's season 23/24
#   - the 10 players who scored the most, and
API_KEY=$API_KEY sport-cli analytics \
    --sport football \
    --event "Premier's League"  \
    --location England  \
    --season-start 2024-08-17 \
    --season-end 2025-05-25 \
    --dimension player \
    --metric score \
    --gender male \
    --sort desc \
    --limit 10 \
    --timeout 2000

# query 
#   - from Premier League's season 23/24
#   - the 10 players who assisted the most.
API_KEY=$API_KEY sport-cli analytics \
    --sport football \
    --event "Premier's League"  \
    --location England  \
    --season-start 2024-08-17 \
    --season-end 2025-05-25 \
    --dimension player \
    --metric assist \
    --gender male \
    --sort desc \
    --limit 10 \
    --timeout 2000
```