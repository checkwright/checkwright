#!/usr/bin/env bash
# Behavioral test of templates/subagent-stop-liveness.sh — the SubagentStop probe.
# The two properties nothing else can assert: the exit is 0 on every path including the
# failure ones, and stdout stays empty (a byte on stdout would be hook JSON, which is the
# blocking variant by accident). The reader is driven by stub scripts, one per exit class,
# so every verdict arm is reached without a real producer.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # delegation-kit/
HOOK="$DIR/templates/subagent-stop-liveness.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

PAYLOAD='{"session_id":"s-1","transcript_path":"/x/y.jsonl","hook_event_name":"SubagentStop","stop_hook_active":false}'

stub() {  # $1=name $2=exit-code
    printf '#!/usr/bin/env bash\nexit %s\n' "$2" >"$tmp/$1"
    printf '%s\n' "$tmp/$1"
}

# fire <name> <log> <reader> <run-dir> <payload> — runs the probe, asserting exit 0 and silent stdout
fire() {
    local name="$1" log="$2" reader="$3" rundir="$4" payload="$5" out rc
    out="$( printf '%s' "$payload" | DELEGATION_KIT_STOP_LOG="$log" \
        DELEGATION_KIT_LIVENESS_CMD="$reader" GATE_SDK_TMP_DIR="$rundir" bash "$HOOK" 2>/dev/null )"
    rc=$?
    [[ "$rc" -eq 0 ]] || { echo "  FAIL: $name exited $rc, not 0"; fails=$((fails + 1)); }
    [[ -z "$out" ]] || { echo "  FAIL: $name wrote '$out' to stdout; the probe emits no hook JSON"; fails=$((fails + 1)); }
}

want() {  # $1=name $2=line $3.. = substrings the line must carry
    local name="$1" line="$2"; shift 2
    local s
    for s in "$@"; do
        grep -qF -- "$s" <<<"$line" || { echo "  FAIL: $name line lacks '$s': $line"; fails=$((fails + 1)); }
    done
}

runs="$tmp/runs"; mkdir -p "$runs"

# A — no reader at all (the knob emptied): the probe still logs, and says so rather than
#     reporting a clean tree it never asked about.
log="$tmp/a.log"
fire no-reader "$log" "" "$runs" "$PAYLOAD"
want no-reader "$(cat "$log")" "event=SubagentStop" "session=s-1" "live=no" "verdict=unavailable" "records=0" "keys=session_id,transcript_path,hook_event_name,stop_hook_active"

# B/C/D — one firing per reader exit class, each with one record on disk so `records` is
#     non-zero and a `live=no` is informative rather than vacuous.
printf 'pid=1 run=k\n' >"$runs/k.run"
for case_spec in "green 0 live=no" "red 1 live=yes" "corrupt 2 live=no"; do
    read -r verdict code liveness <<<"$case_spec"
    log="$tmp/$verdict.log"
    fire "reader-$verdict" "$log" "$(stub "reader-$verdict" "$code")" "$runs" "$PAYLOAD"
    want "reader-$verdict" "$(cat "$log")" "verdict=$verdict" "$liveness" "records=1"
done

# E — a reader that cannot be run at all (an unresolvable path) is the vendoring prerequisite
#     failing, and it degrades to `unavailable` rather than to a wedged turn.
log="$tmp/e.log"
fire absent-reader "$log" "$tmp/nowhere/check.sh" "$runs" "$PAYLOAD"
want absent-reader "$(cat "$log")" "verdict=unavailable" "live=no"

# F — an unwritable log: the line cannot be recorded anywhere, so it is dropped silently and
#     the turn still ends. This is the path where "record the failure where it can be" has
#     nowhere left to record it.
printf 'not a directory\n' >"$tmp/blocker"
fire unwritable-log "$tmp/blocker/probe.log" "" "$runs" "$PAYLOAD"

# G — a payload whose values carry whitespace must not split the space-delimited line: the
#     field count is the assertion, because a split is invisible in a substring match.
log="$tmp/g.log"
fire spacey-payload "$log" "" "$runs" '{"session_id":"a b\tc","hook_event_name":"Subagent Stop"}'
nf="$(awk '{print NF}' "$log")"
[[ "$nf" == "7" ]] || { echo "  FAIL: spacey-payload produced $nf fields, want 7: $(cat "$log")"; fails=$((fails + 1)); }

# H — an empty payload (no stdin at all): the payload-derived fields degrade to '-' and the
#     liveness half of the line is still bought.
log="$tmp/h.log"
fire empty-payload "$log" "$(stub reader-h 1)" "$runs" ""
want empty-payload "$(cat "$log")" "event=-" "session=-" "keys=-" "live=yes"

# I — one firing appends exactly one line, so the log is a firing count as well as a record.
log="$tmp/i.log"
fire append-1 "$log" "" "$runs" "$PAYLOAD"
fire append-2 "$log" "" "$runs" "$PAYLOAD"
lines="$(grep -c . "$log")"
[[ "$lines" == "2" ]] || { echo "  FAIL: two firings wrote $lines line(s), want 2"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "subagent-stop-liveness.test: $fails assertion(s) failed"
    exit 1
fi
echo "subagent-stop-liveness.test: ok (exit 0 and silent stdout on every path; verdict green/red/corrupt/unavailable mapped; whitespace in a payload cannot split the line; one firing, one line)"
exit 0
