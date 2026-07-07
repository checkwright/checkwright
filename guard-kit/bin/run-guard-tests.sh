#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §Testing — decision-table runner for the guard
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GUARD="$KIT/templates/bash-guard.sh"
LIB="$KIT/lib/guard.sh"
CASES="${1:-$KIT/guard-tests/cases.tsv}"

for f in "$GUARD" "$LIB" "$CASES"; do
    [[ -f "$f" ]] || { echo "run-guard-tests: missing $f" >&2; exit 2; }
done
command -v jq >/dev/null 2>&1 || { echo "run-guard-tests: jq not found on PATH" >&2; exit 2; }

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
git -C "$SANDBOX" init -q
printf 'scratch.txt\n.tmp/\nfriction.log\n' >"$SANDBOX/.gitignore"

mkdir -p "$SANDBOX/.claude"
printf '%s\n' '{ "permissions": { "allow": ["Bash(git status)", "Bash(ls)", "Bash(printf:*)"] } }' \
    >"$SANDBOX/.claude/settings.json"

LOG="$SANDBOX/friction.log"

classify() {
    local rc="$1" out="$2"
    if [[ "$rc" -eq 2 ]]; then echo block; return; fi
    if [[ "$rc" -ne 0 ]]; then echo "exit$rc"; return; fi
    if grep -q '"updatedInput"' <<<"$out"; then echo rewrite; return; fi
    if grep -q '"additionalContext"' <<<"$out"; then echo advise; return; fi
    if grep -q '"permissionDecision":"allow"' <<<"$out"; then echo allow; return; fi
    [[ -z "$out" ]] && { echo fallthrough; return; }
    echo "unknown"
}

fails=0
ran=0
while IFS=$'\t' read -r want cmd; do
    [[ -z "${want// }" ]] && continue
    [[ "$want" == \#* ]] && continue
    cmd="${cmd//@ROOT@/$SANDBOX}"
    json="$(jq -nc --arg c "$cmd" '{tool_input:{command:$c}}')"
    out="$(cd "$SANDBOX" && printf '%s' "$json" \
        | GUARD_KIT_LIB="$LIB" GUARD_KIT_LOG="$LOG" bash "$GUARD" 2>/dev/null)"
    rc=$?
    got="$(classify "$rc" "$out")"
    ran=$((ran + 1))
    if [[ "$got" != "$want" ]]; then
        echo "  FAIL: want '$want', got '$got' -- $cmd"
        fails=$((fails + 1))
    fi
done <"$CASES"

if [[ "$ran" -eq 0 ]]; then
    echo "run-guard-tests: no cases parsed from $CASES" >&2
    exit 2
fi
if [[ "$fails" -gt 0 ]]; then
    echo "run-guard-tests: $fails/$ran case(s) failed"
    exit 1
fi
echo "run-guard-tests: ok ($ran cases across the generic ruleset)"
exit 0
