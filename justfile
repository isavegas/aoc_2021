set shell := ["sh", "-cu"]
set dotenv-load
set positional-arguments

year := "2021"
name := "aoc_{{year}}"

alias di := download_input

# -> list
@default: list

# List just targets
@list:
    just --list --unsorted --list-heading "$(printf 'Targets for {{name}}::\n\r')"

download_input day:
    #!/usr/bin/env sh
    PADDED_DAY="$(printf '%02d' {{day}})"
    OUT_FILE="{{justfile_directory()}}/src/input/day_$PADDED_DAY.txt"
    if [ -f "$OUT_FILE" ]; then
        echo "Input for {{year}} day {{day}} exists at $OUT_FILE"
        exit 1
    fi
    source "{{justfile_directory()}}/.envrc"
    if [ -z "$AOC_SESSION" ]; then
        echo AOC_SESSION not defined
        exit 1
    fi
    curl -sf -H "Cookie: session=$AOC_SESSION" "https://adventofcode.com/{{year}}/day/{{day}}/input" -o "$OUT_FILE"
    if [ -f "$OUT_FILE" ]; then
        echo "Downloaded {{year}} day {{day}} input to $OUT_FILE"
    else
        echo "Unable to download input!"
    fi

# Platform info
@info:
    echo {{name}} :: {{os()}} {{arch()}}

