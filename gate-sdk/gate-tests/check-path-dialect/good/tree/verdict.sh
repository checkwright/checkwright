#!/usr/bin/env bash
set -uo pipefail

# spec: gate-sdk/SPEC.md §The path-dialect contract — recorded verdict: this answer is compared
# only against another answer from the same producer, so the two spellings are symmetric.
GITDIR="$(git rev-parse --git-dir 2>/dev/null)"
echo "$GITDIR"
