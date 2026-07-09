#!/usr/bin/env bash
# graph: couples=<surfaces> dir=one valve=none tier=precommit
# spec: <your SPEC> §check-<area> — <one-line invariant>
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

FILE="${1:-README.md}"
[[ -f "$FILE" ]] || { echo "check-skeleton: not found: $FILE" >&2; exit 2; }  # exit 2: harness/usage error

out="$(awk '/never-matching-placeholder/ { print FILENAME ":" FNR ": finding" }' "$FILE")"; st=$?
fail_closed "$st" check-skeleton awk

if [[ -n "$out" ]]; then
    echo "check-skeleton: <what is wrong>:"   # one line per finding (location + problem)
    echo "$out"
    echo "  help: <the remedy — how to fix it>"
    exit 1                                    # exit 1: violation
fi
echo "SKELETON: clean (<what was checked>)"   # exit 0: the one machine-keyable success line
exit 0
