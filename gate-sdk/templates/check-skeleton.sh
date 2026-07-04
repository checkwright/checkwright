#!/usr/bin/env bash
# graph: couples=<surfaces> dir=one valve=none tier=precommit
# spec: <your SPEC> §check-<area> — <one-line invariant>
#
# The reference skeleton (gate-sdk/SPEC.md §The gate model): a new gate is a
# copy-edit of this file, shipping with its good/+bad/ fixture pair under
# <tests-dir>/check-<area>/. Copy it into your gates dir; it is a template,
# never registered in gates.list itself.
set -uo pipefail

# Resolve the kit from the consumer's default layout (gates dir at repo root,
# kit at gate-sdk/); override with GATE_SDK_ROOT when yours differs.
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
