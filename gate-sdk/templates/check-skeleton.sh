#!/usr/bin/env bash
# graph: couples=<surfaces> dir=one valve=none tier=precommit
# spec: <your SPEC> §check-<area> — <one-line invariant>
# no-port: gate-sdk/SPEC.md §The harness-template port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this file is angle-bracket fill-ins in a runnable frame, and gate-sdk/SPEC.md §Layout and configuration calls a consumer's own check-*.sh gates copy-edits of it while gate-sdk/SPEC.md §templates/check-skeleton.sh rules it a template, never a registry member. A vendoring adopter with no crate births a shell gate from this file, so porting it deletes the only thing gate-sdk hands that adopter. Structural, not a sizing judgment. DELETE THIS LINE when you copy the file into a gate of your own: the disposition ruled here is this template's, never your gate's.
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
