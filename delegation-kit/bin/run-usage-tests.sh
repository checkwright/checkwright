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
HIST="$SANDBOX/history.log"

fails=0
ran=0
now="$(date +%s)"

while IFS=$'\t' read -r verdict want pct age_off reset_off cred_age pct_7d reset7d_off append axis desc; do
    [[ -z "${verdict// }" ]] && continue
    [[ "$verdict" == \#* ]] && continue

    {
        printf 'five_hour_used_pct=%s\n' "$pct"
        printf 'five_hour_resets_at=%s\n' "$(( now + reset_off ))"
        printf 'updated_at=%s\n' "$(( now - age_off ))"
        if [[ "$pct_7d" != "-" ]]; then
            printf 'seven_day_used_pct=%s\n' "$pct_7d"
            printf 'seven_day_resets_at=%s\n' "$(( now + reset7d_off ))"
        fi
    } > "$USAGE"

    rm -f "$CRED"
    if [[ "$cred_age" != "-" ]]; then
        : > "$CRED"
        touch -d "@$(( now - cred_age ))" "$CRED"
    fi

    rm -f "$HIST"
    out="$( cd "$SANDBOX" && DELEGATION_KIT_USAGE_HISTORY="$HIST" bash "$GATE" "$USAGE" "$CRED" 2>&1 )"; rc=$?
    ran=$((ran + 1))

    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$desc]: want exit $want, got $rc -- $out"
        fails=$((fails + 1)); continue
    fi
    if ! grep -qF -- "-> $verdict" <<<"$out"; then
        echo "  FAIL [$desc]: output missing verdict '-> $verdict': $out"
        fails=$((fails + 1)); continue
    fi
    if ! grep -qF -- "width=2 " <<<"$out"; then
        echo "  FAIL [$desc]: verdict line dropped the fan-width field (default 2): $out"
        fails=$((fails + 1)); continue
    fi
    if [[ "$axis" != "-" ]]; then
        case "$axis" in
            5h) grep -qF -- "5h window" <<<"$out"     || { echo "  FAIL [$desc]: PAUSE did not name the 5h axis: $out"; fails=$((fails + 1)); continue; } ;;
            7d) grep -qF -- "7-day window" <<<"$out"   || { echo "  FAIL [$desc]: PAUSE did not name the 7-day axis: $out"; fails=$((fails + 1)); continue; } ;;
        esac
    fi

    got_lines=0
    [[ -f "$HIST" ]] && got_lines="$(grep -c '' "$HIST")"
    if [[ "$got_lines" -ne "$append" ]]; then
        echo "  FAIL [$desc]: want $append appended sample(s), got $got_lines"
        fails=$((fails + 1)); continue
    fi
    if [[ "$append" -eq 1 ]]; then
        if [[ "$pct_7d" != "-" ]]; then
            grep -qF -- "pct_7d=" "$HIST" || { echo "  FAIL [$desc]: sample line dropped the passed-through pct_7d: $(cat "$HIST")"; fails=$((fails + 1)); continue; }
        else
            grep -qF -- "pct_7d=" "$HIST" && { echo "  FAIL [$desc]: sample line invented a pct_7d from a three-line snapshot: $(cat "$HIST")"; fails=$((fails + 1)); continue; }
        fi
    fi
done < "$CASES"

# spec: delegation-kit/SPEC.md §usage-verdict — the width field tracks DELEGATION_KIT_FAN_WIDTH, not a literal
{
    printf 'five_hour_used_pct=40\n'
    printf 'five_hour_resets_at=%s\n' "$(( now + 3600 ))"
    printf 'updated_at=%s\n' "$now"
} > "$USAGE"
rm -f "$CRED"
wout="$( cd "$SANDBOX" && DELEGATION_KIT_FAN_WIDTH=7 bash "$GATE" "$USAGE" "$CRED" 2>&1 )"
ran=$((ran + 1))
if ! grep -qF -- "width=7 " <<<"$wout"; then
    echo "  FAIL [fan-width knob override]: width field did not track DELEGATION_KIT_FAN_WIDTH=7: $wout"
    fails=$((fails + 1))
fi

if [[ "$ran" -eq 0 ]]; then
    echo "run-usage-tests: no cases parsed from $CASES" >&2
    exit 2
fi
if [[ "$fails" -gt 0 ]]; then
    echo "run-usage-tests: $fails/$ran case(s) failed"
    exit 1
fi
echo "run-usage-tests: ok ($ran cases across the OK/PAUSE/STALE/RESET-OK verdict table, both pause axes, the fan-width field, and the sample-append discipline)"
exit 0
