#!/usr/bin/env bash
set -uo pipefail

gate_path_rooted() {  # <path> — the predicate's own body, cleared by name
    case "$1" in
        /* | \\*) return 0 ;;
    esac
    [[ "$1" == /* ]] && return 0
    return 1
}

gate_path_rooted "$1" || set -- "$PWD/$1"

# path-dialect-exempt: a URL path, not a filesystem location
[[ "$2" == /* ]] && echo "route $2"

for d in a b; do
    case "$1" in
        */*) echo nested ;;
        /dev/*) echo device ;;
        "$d"/*) echo "under $d" ;;
    esac
done
