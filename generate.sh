#!/bin/bash

SCRIPTDIR=$(dirname "$(realpath $0)")
WSCARGO="$SCRIPTDIR/Cargo.toml"

printf "[workspace]\n" > "$WSCARGO"

{
    printf "resolver = \"2\"\n"
    printf "members = [\n\t\"common\","
} >> "$WSCARGO"

# Create Cargo.toml for Workspace
for i in $(seq 25); do
    printf "\n\t\"day%s\"," "$i"
done >> "$WSCARGO"

printf "\n]\n" >> "$WSCARGO"

# Common files are created by hand, but the rest can be generated

for i in $(seq 25); do
    if [ ! -d "$SCRIPTDIR/day$i" ]; then
        cp -r "$SCRIPTDIR/template-day" "$SCRIPTDIR/day$i"
        sed -i 's/template-day/day'"$i"'/g' "$SCRIPTDIR/day$i/Cargo.toml"
    else
        echo "$SCRIPTDIR/day$i exists, skipping..."
    fi
done
