#!/bin/sh

for child in $(ls); do
    if ! [ -d "$child" ]; then
        continue
    fi
    if ! [ -f "$child"/Cargo.toml ]; then
        continue
    fi
    cargo publish -p "$child" &
done
wait
