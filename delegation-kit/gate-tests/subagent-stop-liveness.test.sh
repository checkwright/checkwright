#!/usr/bin/env bash
# Behavioral test of templates/subagent-stop-liveness.sh — the SubagentStop hook.
# The properties nothing else can assert: the exit code per verdict arm (2 on red, corrupt and
# unresolved, 0 on green, unavailable and error), the decision= column that goes with it, a
# non-empty stderr on each refusing arm because that stderr IS the blocking reason, and stdout
# empty on every path (a byte on stdout would be hook JSON, which is a decision this hook never
# emits). The reader is driven by stub scripts, one per exit class, so every verdict arm is
# reached without a real producer. Reader exit 2 is driven TWICE — over a non-empty record set
# and over an empty one — because that one exit carries two verdicts, and the messages are
# asserted apart so a two-way branch cannot regrow behind a three-way spec.
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

# fire <name> <log> <reader> <run-dir> <payload> <want-rc> — runs the hook, asserting the exit
# code, silent stdout, and stderr non-empty exactly when the exit refuses.
fire() {
    local name="$1" log="$2" reader="$3" rundir="$4" payload="$5" want_rc="$6" out rc
    out="$( printf '%s' "$payload" | DELEGATION_KIT_STOP_LOG="$log" \
        DELEGATION_KIT_LIVENESS_CMD="$reader" GATE_SDK_TMP_DIR="$rundir" \
        bash "$HOOK" 2>"$tmp/$name.err" )"
    rc=$?
    [[ "$rc" -eq "$want_rc" ]] || { echo "  FAIL: $name exited $rc, want $want_rc"; fails=$((fails + 1)); }
    [[ -z "$out" ]] || { echo "  FAIL: $name wrote '$out' to stdout; the hook emits no hook JSON"; fails=$((fails + 1)); }
    if [[ "$want_rc" -eq 2 ]]; then
        [[ -s "$tmp/$name.err" ]] || { echo "  FAIL: $name refused with empty stderr; the stderr is the blocking reason"; fails=$((fails + 1)); }
    else
        [[ ! -s "$tmp/$name.err" ]] || { echo "  FAIL: $name allowed but wrote stderr: $(cat "$tmp/$name.err")"; fails=$((fails + 1)); }
    fi
}

want() {  # $1=name $2=line $3.. = substrings the line must carry
    local name="$1" line="$2"; shift 2
    local s
    for s in "$@"; do
        grep -qF -- "$s" <<<"$line" || { echo "  FAIL: $name line lacks '$s': $line"; fails=$((fails + 1)); }
    done
}

runs="$tmp/runs"; mkdir -p "$runs"

# A — no reader at all (the knob emptied): the hook holds no reading, so it logs, says so rather
#     than reporting a clean tree it never asked about, and allows.
log="$tmp/a.log"
fire no-reader "$log" "" "$runs" "$PAYLOAD" 0
want no-reader "$(cat "$log")" "event=SubagentStop" "session=s-1" "live=no" "verdict=unavailable" "records=0" "decision=allow" "keys=session_id,transcript_path,hook_event_name,stop_hook_active"

# B/C/D/E — one firing per reader exit class, each with one record on disk so `records` is
#     non-zero and a `live=no` is informative rather than vacuous. The exit code is the
#     predicate: red and corrupt refuse, green and an unmapped code allow. `corrupt` carries
#     `live=no decision=refuse`, which is why decision cannot be derived from live.
printf 'pid=1 run=k\n' >"$runs/k.run"
for case_spec in "green 0 live=no allow 0" "red 1 live=yes refuse 2" "corrupt 2 live=no refuse 2" "error 77 live=no allow 0"; do
    read -r verdict code liveness decision rc <<<"$case_spec"
    log="$tmp/$verdict.log"
    fire "reader-$verdict" "$log" "$(stub "reader-$verdict" "$code")" "$runs" "$PAYLOAD" "$rc"
    want "reader-$verdict" "$(cat "$log")" "verdict=$verdict" "$liveness" "records=1" "decision=$decision"
done

# F — the refusal message names the finding and both lawful exits, as every block message must,
#     and it names the reader command so the session can see the record set for itself.
want red-message "$(cat "$tmp/reader-red.err")" "turn-end refused" "wait for the producer on its own artifact" "delete the record once the producer has exited" "$runs"
want corrupt-message "$(cat "$tmp/reader-corrupt.err")" "does not parse" "which record is malformed"

# F2 — the SAME reader exit over an EMPTY record set is a different reading: there is no record
#     for it to be about, so it is `unresolved` rather than `corrupt`. It still refuses — the
#     count names the diagnosis and decides nothing — and its message must not reuse the corrupt
#     arm's "does not parse" wording over a case that holds no record to parse.
emptyruns="$tmp/emptyruns"; mkdir -p "$emptyruns"
log="$tmp/unresolved.log"
fire reader-unresolved "$log" "$(stub reader-unresolved 2)" "$emptyruns" "$PAYLOAD" 2
want reader-unresolved "$(cat "$log")" "verdict=unresolved" "live=no" "records=0" "decision=refuse"
want unresolved-message "$(cat "$tmp/reader-unresolved.err")" "turn-end refused" "produced no reading at all" "the reader's own reason"
if grep -qF -- "does not parse" "$tmp/reader-unresolved.err"; then
    echo "  FAIL: the unresolved arm reused the corrupt arm's 'does not parse' wording over a case holding no record"
    fails=$((fails + 1))
fi

# F3 — the record glob is taken AFTER the reader has run, and this is the assertion that holds
#     the order: a stub that writes a record and then exits 2 must read as `corrupt` over
#     records=1. Move the glob back above the reader and this case flips to unresolved/records=0,
#     which is the in-flight record being missed rather than counted.
raceruns="$tmp/raceruns"; mkdir -p "$raceruns"
printf '#!/usr/bin/env bash\nprintf "pid=1 run=r\\n" >"%s/r.run"\nexit 2\n' "$raceruns" >"$tmp/reader-race"
log="$tmp/race.log"
fire reader-race "$log" "$tmp/reader-race" "$raceruns" "$PAYLOAD" 2
want reader-race "$(cat "$log")" "verdict=corrupt" "records=1" "decision=refuse"

# G — a reader that cannot be run at all (an unresolvable path) is the vendoring prerequisite
#     failing: the hook holds no reading, so it degrades to `unavailable` and allows rather
#     than refusing every turn end in a tree that never configured enforcement.
log="$tmp/g.log"
fire absent-reader "$log" "$tmp/nowhere/check.sh" "$runs" "$PAYLOAD" 0
want absent-reader "$(cat "$log")" "verdict=unavailable" "live=no" "decision=allow"

# H — an unwritable log: the line cannot be recorded anywhere, so it is dropped silently. The
#     reading still decides, so this firing allows on `unavailable` rather than on the log.
printf 'not a directory\n' >"$tmp/blocker"
fire unwritable-log "$tmp/blocker/probe.log" "" "$runs" "$PAYLOAD" 0

# I — a payload whose values carry whitespace must not split the space-delimited line: the
#     field count is the assertion, because a split is invisible in a substring match.
log="$tmp/i.log"
fire spacey-payload "$log" "" "$runs" '{"session_id":"a b\tc","hook_event_name":"Subagent Stop"}' 0
nf="$(awk '{print NF}' "$log")"
[[ "$nf" == "8" ]] || { echo "  FAIL: spacey-payload produced $nf fields, want 8: $(cat "$log")"; fails=$((fails + 1)); }

# J — an empty payload (no stdin at all): the payload-derived fields degrade to '-' and the
#     refusal is exact, because the decision reads the liveness reader and no payload field.
log="$tmp/j.log"
fire empty-payload "$log" "$(stub reader-j 1)" "$runs" "" 2
want empty-payload "$(cat "$log")" "event=-" "session=-" "keys=-" "live=yes" "decision=refuse"

# K — one firing appends exactly one line, so the log is a firing count as well as a record.
log="$tmp/k.log"
fire append-1 "$log" "" "$runs" "$PAYLOAD" 0
fire append-2 "$log" "" "$runs" "$PAYLOAD" 0
lines="$(grep -c . "$log")"
[[ "$lines" == "2" ]] || { echo "  FAIL: two firings wrote $lines line(s), want 2"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "subagent-stop-liveness.test: $fails assertion(s) failed"
    exit 1
fi
echo "subagent-stop-liveness.test: ok (exit 2 on red/corrupt/unresolved with a reason on stderr, 0 on green/unavailable/error with none; reader exit 2 splits by record count into corrupt and unresolved with distinct wording; the glob is taken after the reader; decision= matches the exit; stdout silent on every path; whitespace in a payload cannot split the line; one firing, one line)"
exit 0
