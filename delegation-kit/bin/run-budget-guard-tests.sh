#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — decision-table runner for the Agent budget guard
#
#   usage: run-budget-guard-tests.sh [cases.tsv]
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GUARD="$KIT/templates/agent-budget-guard.sh"
LIB="$KIT/../guard-kit/lib/guard.sh"
VERDICT="$KIT/bin/usage-verdict.sh"
CASES="${1:-$KIT/usage-tests/budget-guard-cases.tsv}"

for f in "$GUARD" "$LIB" "$VERDICT" "$CASES"; do
    [[ -f "$f" ]] || { echo "run-budget-guard-tests: missing $f" >&2; exit 2; }
done
command -v jq >/dev/null 2>&1 || { echo "run-budget-guard-tests: jq not found on PATH" >&2; exit 2; }

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
USAGE="$SANDBOX/usage.txt"
CRED="$SANDBOX/.credentials.json"
ERR="$SANDBOX/err"
JSON='{"tool_name":"Agent","tool_input":{"prompt":"audit"}}'

classify() {
    local rc="$1" out="$2"
    if [[ "$rc" -eq 2 ]]; then echo block; return; fi
    if [[ "$rc" -ne 0 ]]; then echo "exit$rc"; return; fi
    if grep -q '"additionalContext"' <<<"$out"; then echo advise; return; fi
    echo unknown
}

fails=0
ran=0
now="$(date +%s)"

while IFS=$'\t' read -r want pct age_off reset_off cred_age desc; do
    [[ -z "${want// }" ]] && continue
    [[ "$want" == \#* ]] && continue

    rm -f "$USAGE" "$CRED"
    if [[ "$pct" != "UNREADABLE" ]]; then
        {
            printf 'five_hour_used_pct=%s\n' "$pct"
            printf 'five_hour_resets_at=%s\n' "$(( now + reset_off ))"
            printf 'updated_at=%s\n' "$(( now - age_off ))"
        } > "$USAGE"
    fi
    if [[ "$cred_age" != "-" ]]; then
        : > "$CRED"
        touch -d "@$(( now - cred_age ))" "$CRED"
    fi

    out="$( cd "$SANDBOX" && printf '%s' "$JSON" \
        | GUARD_KIT_LIB="$LIB" DELEGATION_KIT_VERDICT_BIN="$VERDICT" \
          DELEGATION_KIT_USAGE_FILE="$USAGE" DELEGATION_KIT_CRED_FILE="$CRED" \
          bash "$GUARD" 2>"$ERR" )"
    rc=$?
    err="$(cat "$ERR")"
    got="$(classify "$rc" "$out")"
    ran=$((ran + 1))

    if [[ "$got" != "$want" ]]; then
        echo "  FAIL [$desc]: want '$want', got '$got' (rc=$rc) -- out=$out err=$err"
        fails=$((fails + 1))
        continue
    fi
    case "$want" in
        block)  grep -qF -- '->' <<<"$err" || { echo "  FAIL [$desc]: block carries no live verdict text -- $err"; fails=$((fails + 1)); } ;;
        advise) grep -qF -- '->' <<<"$out" || { echo "  FAIL [$desc]: advise carries no live verdict text -- $out"; fails=$((fails + 1)); } ;;
    esac
done < "$CASES"

if [[ "$ran" -eq 0 ]]; then
    echo "run-budget-guard-tests: no cases parsed from $CASES" >&2
    exit 2
fi
if [[ "$fails" -gt 0 ]]; then
    echo "run-budget-guard-tests: $fails/$ran case(s) failed"
    exit 1
fi
echo "run-budget-guard-tests: ok ($ran cases across the block/advise routing)"
exit 0
