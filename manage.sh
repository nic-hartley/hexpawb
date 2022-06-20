#!/bin/sh

projects() {
    for child in $(ls); do
        if ! [ -d "$child" ]; then
            continue
        fi
        if ! [ -f "$child"/Cargo.toml ]; then
            continue
        fi
        echo $child
    done
}

get_version() {
    version=""
    for proj in $(projects); do
        if [ -z "$version" ]; then
            version="$(sed -rn '/^\s*version\s*=\s*"/p' "$proj"/Cargo.toml | cut -d'"' -f2)"
        else
            new_ver="$(sed -rn '/^\s*version\s*=\s*"/p' "$proj"/Cargo.toml | cut -d'"' -f2)"
            if [ "$new_ver" != "$version" ]; then
                echo "Versions mismatch"
                exit 1
            fi
        fi
    done
}

bump_version() {
    major="$(cut -d. -f1 <<<"$version")"
    minor="$(cut -d. -f2 <<<"$version")"
    patch="$(cut -d. -f3 <<<"$version")"
    case "$1" in
    major)
        major=$(($major+1))
        minor=0
        patch=0
        ;;
    minor)
        minor=$(($minor+1))
        patch=0
        ;;
    patch)
        patch=$(($patch+1))
        ;;
    *)
        echo "Unknown/missing version part $1"
        exit 1
        ;;
    esac
    version="$major.$minor.$patch"
}

set -e

case "$1" in
ver)
    if ! git diff --exit-code >/dev/null; then
        echo 'Not everything is checked in!'
        exit 1
    fi
    get_version
    bump_version "$2"
    for proj in $(projects); do
        sed -i -E '
            s/^version.*$/version = "'"$version"'"/
            s/^hexpawb\s*=\s*".*$/hexpawb = "'"$version"'"/
            s/^(hexpawb\s*=\s*\{.*version\s*=\s*)"[^"]+"/\1"'"$version"'"/
        ' "$proj"/Cargo.toml
    done
    echo "Bumped version to $version"
    ;;
check)
    get_version
    echo "Versions match"
    ;;
publish)
    for proj in $(projects); do
        cargo publish -p "$proj" &
    done
    wait
    ;;
*)
    echo "Unknown/missing command $cmd"
    ;;
esac
