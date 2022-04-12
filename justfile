set shell := ["sh", "-cu"]
set dotenv-load
set positional-arguments

year := "2021"
name := "aoc_{{year}}"

alias di := download_input
alias gen := generate
alias t := test
alias wt := watch_test
alias wr := watch_run

# -> list
@default: list

# List just targets
@list:
    just --list --unsorted --list-heading "$(printf 'Targets for {{name}}::\n\r')"

@update_core:
    git -C aoc_core pull origin master

test day='all':
    #!/usr/bin/env sh
    if [[ "{{day}}" = "all" ]]; then
        cargo test
    else
        cargo test --test day_"$(printf '%02d' {{day}})"
    fi

generate day:
    #!/usr/bin/env sh
    DAY="$(printf '%02d' {{day}})"
    IMPL_FILE="{{justfile_directory()}}/src/day/day_$DAY.rs"
    TEST_FILE="{{justfile_directory()}}/tests/day_$DAY.rs"
    IMPL_TEMPLATE="$(cat "{{justfile_directory()}}/templates/day.rs")"
    TEST_TEMPLATE="$(cat "{{justfile_directory()}}/templates/test.rs")"
    check_file() {
        if [ -f "$2" ]; then
            echo "$1 for {{year}} day {{day}} exists at $2"
            exit 1
        fi
    }
    check_file Implementation "$IMPL_FILE"
    check_file Tests "$TEST_FILE"
    echo "${IMPL_TEMPLATE//@DAY@/$DAY}" > "$IMPL_FILE"
    echo "${TEST_TEMPLATE//@DAY@/$DAY}" > "$TEST_FILE"
    echo "Implementation for day {{day}} generated at $IMPL_FILE"
    echo "Tests for day {{day}} generated at $TEST_FILE"

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

# Watch for code changes, running against input
@watch_run day:
    cargo watch -x 'run -- run -d {{day}}'

# Watch for code changes, running tests for day
@watch_test day:
    cargo watch -x "test --test day_$(printf '%02d' {{day}})"
