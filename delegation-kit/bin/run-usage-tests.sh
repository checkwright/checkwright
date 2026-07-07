#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — decision-table runner for usage-verdict
#
#   usage: run-usage-tests.sh [cases.tsv]
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GATE="$KIT/bin/usage-verdict.sh"
CASES="${1:-$KIT/usage-tests/cases.tsv}"

[[ -x "$GATE" ]]  || { echo "run-usage-tests: missing or non-executable $GATE" >&2; exit 2; }
[[ -f "$CASES" ]] || { echo "run-usage-tests: cases file not found: $CASES" >&2; exit 2; }

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
USAGE="$SANDBOX/usage.txt"
CRED="$SANDBOX/.credentials.json"

fails=0
ran=0
now="$(date +%s)"

while IFS=$'\t' read -r verdict want pct age_off reset_off cred_age desc; do
    [[ -z "${verdict// }" ]] && continue
    [[ "$verdict" == \#* ]] && continue

    {
        printf 'five_hour_used_pct=%s\n' "$pct"
        printf 'five_hour_resets_at=%s\n' "$(( now + reset_off ))"
        printf 'updated_at=%s\n' "$(( now - age_off ))"
    } > "$USAGE"

    rm -f "$CRED"
    if [[ "$cred_age" != "-" ]]; then
        : > "$CRED"
        touch -d "@$(( now - cred_age ))" "$CRED"
    fi

    out="$( cd "$SANDBOX" && bash "$GATE" "$USAGE" "$CRED" 2>&1 )"; rc=$?
    ran=$((ran + 1))

    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$desc]: want exit $want, got $rc -- $out"
        fails=$((fails + 1))
    elif ! grep -qF -- "-> $verdict" <<<"$out"; then
        echo "  FAIL [$desc]: output missing verdict '-> $verdict': $out"
        fails=$((fails + 1))
    fi
done < "$CASES"

if [[ "$ran" -eq 0 ]]; then
    echo "run-usage-tests: no cases parsed from $CASES" >&2
    exit 2
fi
if [[ "$fails" -gt 0 ]]; then
    echo "run-usage-tests: $fails/$ran case(s) failed"
    exit 1
fi
echo "run-usage-tests: ok ($ran cases across the OK/PAUSE/STALE/RESET-OK verdict table)"
exit 0
