#!/bin/bash
set -e -o pipefail

die() {
    [ $# -gt 0 ] && echo "Fatal: $*" >&2
    exit 1
}

force=
while [ "$#" -gt 0 ]; do
    case "$1" in
        -f | --force) force=1 ;;
        *) die "Unknown argument $1" ;;
    esac
    shift
done

year=$(basename "$(pwd)")
echo "$year" | grep -q '^20[0-9]\{2\}$' || die "Unknown year. Check current directory"
days=25
[ "$year" -ge 2025 ] && days=12

load_dotenv() {
    local dir
    dir=$(pwd)
    while true; do
        if [ -f "$dir/.env" ] && [ -O "$dir/.env" ]; then
            source "$dir/.env"
            return
        fi

        [ "$dir" = "/" ] && return
        dir=$(dirname "$dir")
    done
}
load_dotenv

[ -n "$COOKIE_SESSION" ] || die "Environment variable COOKIE_SESSION is empty."

mkdir -pv input
for day in $(seq $days); do
    day_str=$(printf '%02d' "$day")
    fn="input/day-$day_str.txt"

    if [ -s "$fn" ] && [ -z "$force" ]; then
        continue
    fi

    echo "[$(date +%F\ %T)] Downloading input for AoC $year day $day" >&2
    curl -sSf "https://adventofcode.com/$year/day/$day/input" \
        -H 'Accept: text/plain' \
        -H "Referer: https://adventofcode.com/$year/day/$day" \
        -H "Cookie: session=$COOKIE_SESSION" \
        --compressed -o "$fn" \
        || {
            rm -fv "$fn"
            exit 1
        }
done
