#!/bin/bash

# Makes sure the inputs for every solution in solutions_dir is downloaded to inputs_dir.  Assumes solution source
# files are named like day{day_number}.rs

year=2022
inputs_dir=input
solutions_dir=src/bin

if [ -z "$AOC_SESSION" ]; then
    echo "Set AOC_SESSION to your AOC session id.  You can find this in a browser cookie called 'session'"
    exit 1
fi

set -e

if ! [ -d "$inputs_dir" ]; then
    mkdir "$inputs_dir"
fi

# enable extended glob patterns
shopt -s extglob

for src_file in "$solutions_dir"/day+([0-9]).rs; do
    base=$(basename "$src_file" .rs)
    day=${base#day}
    input_file="$inputs_dir/$base-input.txt"
    if ! [ -f "$input_file" ]; then
        echo "Fetching $input_file"
        curl -b session="$AOC_SESSION" "https://adventofcode.com/$year/day/$day/input" > "$input_file"
    fi
done
