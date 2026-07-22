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

# spec: delegation-kit/SPEC.md §Testing — the table encodes the kit defaults, so strip ambient DELEGATION_KIT_* at every gate invocation; the poison export proves the strip each run (a leak fails the table loudly)
export DELEGATION_KIT_PAUSE_PCT=0
DK_UNSET=()
while IFS= read -r name; do DK_UNSET+=(-u "$name"); done < <(env | grep -o '^DELEGATION_KIT_[A-Za-z0-9_]*')

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
    out="$( cd "$SANDBOX" && env "${DK_UNSET[@]}" DELEGATION_KIT_USAGE_HISTORY="$HIST" bash "$GATE" "$USAGE" "$CRED" 2>&1 )"; rc=$?
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
wout="$( cd "$SANDBOX" && env "${DK_UNSET[@]}" DELEGATION_KIT_FAN_WIDTH=7 bash "$GATE" "$USAGE" "$CRED" 2>&1 )"
ran=$((ran + 1))
if ! grep -qF -- "width=7 " <<<"$wout"; then
    echo "  FAIL [fan-width knob override]: width field did not track DELEGATION_KIT_FAN_WIDTH=7: $wout"
    fails=$((fails + 1))
fi

# spec: delegation-kit/SPEC.md §usage-verdict — demand-driven refresh: armed/fail-soft/short-circuit, each proved through a stub REFRESH_CMD (a real poll would need the network)
STUB="$SANDBOX/refresh-stub.sh"
STAMP="$SANDBOX/refresh-ran"

seed_snapshot() {
    {
        printf 'five_hour_used_pct=%s\n' "$1"
        printf 'five_hour_resets_at=%s\n' "$(( now + 3600 ))"
        printf 'updated_at=%s\n' "$(( now - $2 ))"
    } > "$USAGE"
}

rm -f "$CRED"

cat > "$STUB" <<STUBEOF
#!/usr/bin/env bash
touch "$STAMP"
{
    printf 'five_hour_used_pct=95\n'
    printf 'five_hour_resets_at=$(( now + 3600 ))\n'
    printf 'updated_at=$now\n'
} > "$USAGE.tmp" && mv "$USAGE.tmp" "$USAGE"
STUBEOF

seed_snapshot 40 1200
rm -f "$STAMP"
rout="$( cd "$SANDBOX" && env "${DK_UNSET[@]}" DELEGATION_KIT_REFRESH_CMD="bash $STUB" bash "$GATE" "$USAGE" "$CRED" 2>&1 )"
ran=$((ran + 1))
if [[ ! -f "$STAMP" ]]; then
    echo "  FAIL [refresh-armed-stale]: a stale snapshot did not invoke DELEGATION_KIT_REFRESH_CMD"
    fails=$((fails + 1))
elif ! grep -qF -- "used=95%" <<<"$rout"; then
    echo "  FAIL [refresh-armed-stale]: verdict read the cached pct, not the refreshed one: $rout"
    fails=$((fails + 1))
fi

cat > "$STUB" <<'STUBEOF'
#!/usr/bin/env bash
echo "usage-poller: fetch failed" >&2
exit 1
STUBEOF

seed_snapshot 40 1200
before="$(cat "$USAGE")"
fout="$( cd "$SANDBOX" && env "${DK_UNSET[@]}" DELEGATION_KIT_REFRESH_CMD="bash $STUB" bash "$GATE" "$USAGE" "$CRED" 2>&1 )"; frc=$?
ran=$((ran + 1))
if [[ "$(cat "$USAGE")" != "$before" ]]; then
    echo "  FAIL [refresh-fail-soft]: a failed refresh mutated the snapshot"
    fails=$((fails + 1))
elif [[ "$frc" -ne 2 ]] || ! grep -qF -- "-> STALE" <<<"$fout"; then
    echo "  FAIL [refresh-fail-soft]: want the cached snapshot judged STALE by the existing staleness machinery, got exit $frc -- $fout"
    fails=$((fails + 1))
elif grep -qF -- "fetch failed" <<<"$fout"; then
    echo "  FAIL [refresh-fail-soft]: refresh diagnostics leaked into the verdict output (callers relay this line verbatim): $fout"
    fails=$((fails + 1))
fi

cat > "$STUB" <<STUBEOF
#!/usr/bin/env bash
touch "$STAMP"
STUBEOF

seed_snapshot 40 0
rm -f "$STAMP"
sout="$( cd "$SANDBOX" && env "${DK_UNSET[@]}" DELEGATION_KIT_REFRESH_CMD="bash $STUB" bash "$GATE" "$USAGE" "$CRED" 2>&1 )"
ran=$((ran + 1))
if [[ -f "$STAMP" ]]; then
    echo "  FAIL [refresh-skip-fresh]: a snapshot under REFRESH_MIN_AGE (default 60s) still invoked the refresh — the render path would hammer the source: $sout"
    fails=$((fails + 1))
fi

# spec: delegation-kit/SPEC.md §usage-verdict — the roll witnesses need a pre-seeded history tail the table's columns cannot express, so they are asserted beside it: both witnesses disarm the login reroute, either alone does not, and an unusable tail falls open to it
roll_case() {
    local desc="$1" tail_line="$2" want_rc="$3" want_verdict="$4"
    {
        printf 'five_hour_used_pct=3\n'
        printf 'five_hour_resets_at=%s\n' "$(( now + 18000 ))"
        printf 'updated_at=%s\n' "$now"
    } > "$USAGE"
    : > "$CRED"
    touch -d "@$(( now - 60 ))" "$CRED"
    rm -f "$HIST"
    [[ -n "$tail_line" ]] && printf '%s\n' "$tail_line" > "$HIST"
    local out rc
    out="$( cd "$SANDBOX" && env "${DK_UNSET[@]}" DELEGATION_KIT_USAGE_HISTORY="$HIST" bash "$GATE" "$USAGE" "$CRED" 2>&1 )"; rc=$?
    ran=$((ran + 1))
    if [[ "$rc" -ne "$want_rc" ]] || ! grep -qF -- "-> $want_verdict" <<<"$out"; then
        echo "  FAIL [$desc]: want exit $want_rc and '-> $want_verdict', got exit $rc -- $out"
        fails=$((fails + 1))
    fi
}

prev_boundary_crossed=$(( now - 3600 ))

roll_case "roll-witnesses-disarm-reroute" \
    "updated_at=$(( now - 7200 )) pct=86.0 resets_at=${prev_boundary_crossed} verdict=PAUSE login_at=0" \
    0 OK
roll_case "roll-witness-boundary-unmoved" \
    "updated_at=$(( now - 60 )) pct=86.0 resets_at=$(( now + 18000 )) verdict=PAUSE login_at=0" \
    2 STALE
roll_case "roll-witness-uncrossed-boundary" \
    "updated_at=$(( now - 60 )) pct=86.0 resets_at=$(( now + 900 )) verdict=PAUSE login_at=0" \
    2 STALE
roll_case "roll-witness-malformed-tail" \
    "updated_at=$(( now - 7200 )) pct=86.0 resets_at=notanepoch verdict=PAUSE login_at=0" \
    2 STALE
roll_case "roll-witness-absent-history" "" 2 STALE

# spec: delegation-kit/SPEC.md §usage-verdict — the witness reads the *previous* sample, never the one this run appends: a first-ever sample cannot disarm its own reroute
rm -f "$HIST"
roll_case "roll-witness-not-self-witnessing" "" 2 STALE
[[ "$(grep -c '' "$HIST" 2>/dev/null || echo 0)" -eq 1 ]] || {
    echo "  FAIL [roll-witness-not-self-witnessing]: the run did not leave exactly its own sample behind"
    fails=$((fails + 1))
}

if [[ "$ran" -eq 0 ]]; then
    echo "run-usage-tests: no cases parsed from $CASES" >&2
    exit 2
fi
if [[ "$fails" -gt 0 ]]; then
    echo "run-usage-tests: $fails/$ran case(s) failed"
    exit 1
fi
echo "run-usage-tests: ok ($ran cases across the OK/PAUSE/STALE/RESET-OK verdict table, both pause axes and their at-or-over boundaries, the login reroute in both directions (it suppresses an OK, never a PAUSE), the fan-width field, the roll witnesses that disarm it, the demand-driven refresh (armed/fail-soft/short-circuit), and the sample-append discipline)"
exit 0
