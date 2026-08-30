#!/usr/bin/env bash
# The two-line idiom: the producer is the cd's direct argument, and the site binds a root, so it
# reads the cd back with pwd -P.
set -uo pipefail

cd "$(git rev-parse --show-toplevel 2>/dev/null)" || exit 1
ROOT="$(pwd -P)"
echo "$ROOT/sub"
