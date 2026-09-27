#!/usr/bin/env bash
# Resolve the directory of the feature a branch names.
# Accepts an absolute path or one relative to the repository root.

set -e

feature_dir() {
    local target="$1"
    # An absolute path is used as given.
    case "$target" in
        /*) printf '%s\n' "$target" ;;
        *) printf '%s/specs/%s\n' "$(git rev-parse --show-toplevel)" "$target" ;;
    esac
}

feature_dir "$@"
