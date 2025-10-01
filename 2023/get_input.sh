#!/bin/bash

this_dir=$(dirname "$(readlink -f "$0")")
[ -f "$this_dir/.env" ] && source "$this_dir/.env"

[ -z "$COOKIE_SESSION" ] && {
    echo "Error: Environment variable COOKIE_SESSION is empty." >&2
    exit 1
}

for day in $(seq 25); do
    day_str=$(printf '%02d' "$day")
    echo "[$(date +%F\ %T)] Downloading input for day-$day_str" >&2

    curl "https://adventofcode.com/2023/day/$day/input" \
        --compressed \
        -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0' \
        -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8' \
        -H 'Accept-Language: en-US,en;q=0.5' \
        -H 'Accept-Encoding: gzip, deflate, br, zstd' \
        -H "Referer: https://adventofcode.com/2023/day/$day" \
        -H 'DNT: 1' \
        -H 'Connection: keep-alive' \
        -H "Cookie: session=$COOKIE_SESSION" \
        -H 'Upgrade-Insecure-Requests: 1' \
        -H 'Sec-Fetch-Dest: document' \
        -H 'Sec-Fetch-Mode: navigate' \
        -H 'Sec-Fetch-Site: same-origin' \
        -H 'Sec-Fetch-User: ?1' \
        -H 'Priority: u=0, i' \
        -H 'Pragma: no-cache' \
        -H 'Cache-Control: no-cache' \
        -H 'TE: trailers' > "input/day-$day_str.txt"
done
