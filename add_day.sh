#!/bin/sh

if [ $# -ne 1 ]; then
    echo "Usage: $0 <day>" >&2
    exit
fi

day="$1"
if [ -e "src/solutions/day${day}.rs" ]; then
    echo "Day already exists, aborting." >&2
    exit
fi

set -ex

# Create day file.
sed -e "s/DAY/$day/g" src/solutions/template.rs.in > src/solutions/day${day}.rs

# Add input data.
mkdir -p res/day$day
touch res/day${day}/sample.txt
touch res/day${day}/input.txt

# Change latest day.
echo "Files created, please adjust latest day in src/solutions/mod.rs."

set +ex
