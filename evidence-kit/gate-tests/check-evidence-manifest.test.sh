#!/usr/bin/env bash
# Behavioral test of check-evidence-manifest — the lifecycle-coupled
# assertions the one good/bad pair (grammar) cannot hold: (C) a validate stamp
# demands ≥1 evidence line, disarmed while the cursor is still at validate and
# re-armed past it; (A) a close-entry cursor demands the full green block over
# every configured suite, imported lines included; both no-cursor disarms; and
# (B) per major: each v2 field and line kind, a line under the other major's
# header, a green v1 file and an unknown major.
#
# The cursor is the state file's last data line, so every case
# below drives the stage axis from <state-body> and the header carries the name
# axis alone — a header-sourced stage would make these cases prove nothing.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
CHECKS="$DIR/checks"
HASH=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# The v2 suite line's four appended fields, for a line otherwise spelled at v1's eight.
COMMIT=0123456789abcdef0123456789abcdef01234567
TAIL=" commit=$COMMIT tree=clean cmd=$HASH origin=local"
MAJOR=v2

# case_run <name> <manifest-body> <header> <state-body> <suites> <want> <expect>
# (the manifest header's major is $MAJOR)
case_run() {
    local name="$1" man="$2" hdr="$3" state="$4" suites="$5" want="$6" expect="$7"
    local d="$tmp/$name" out rc; mkdir -p "$d/scripts"
    printf '# contract: evidence-manifest %s\n%b' "$MAJOR" "$man" >"$d/man.txt"
    printf '%s\n' "$hdr" >"$d/queue.md"
    printf '%b' "$state" >"$d/state.txt"
    # shellcheck disable=SC2086  # one element line per space-separated suite
    [[ -n "$suites" ]] && printf 'EVIDENCE_KIT_SUITES[] = %s\n' $suites >"$d/scripts/evidence-config.knobs"
    out="$( cd "$d" && unset EVIDENCE_KIT_KNOB_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-evidence-manifest "$CHECKS" man.txt queue.md state.txt 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

# A — validate stamp present, cursor past validate (last stamp is close), no
#     evidence line: stamp-coupling (C) fires — validate ran and recorded nothing.
case_run "C-recorded-nothing" \
    '' '## Iteration: it' \
    'h\n---\nit scope s1 2026-07-01\nit validate s2 2026-07-05\nit close s3 2026-07-06\n' '' \
    1 "recorded nothing"

# B — same validate stamp, but the cursor still AT validate: C is disarmed (the
#     entry stamp legitimately precedes the suites), so a bare manifest is CLEAN.
case_run "C-disarmed-at-validate" \
    '' '## Iteration: it' \
    'h\n---\nit scope s1 2026-07-01\nit validate s2 2026-07-05\n' '' \
    0 "clean"

# C — close entry with one of two configured suites missing its clean line:
#     assertion A fires for the missing suite.
case_run "A-missing-suite" \
    "it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-06$TAIL\n" \
    '## Iteration: it' \
    'h\n---\nit validate s2 2026-07-05\nit close s3 2026-07-06\n' 'unit extra' \
    1 "no clean evidence line"

# D — close entry with both suites clean and dated after the validate stamp is
#     CLEAN (the full green block).
case_run "A-green-block" \
    "it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-06$TAIL\nit extra sha256=$HASH pass=2 fail=0 ignore=0 verdict=clean 2026-07-06$TAIL\n" \
    '## Iteration: it' \
    'h\n---\nit validate s2 2026-07-05\nit close s3 2026-07-06\n' 'unit extra' \
    0 "clean"

# E — a clean line dated BEFORE the earliest validate stamp is stale evidence.
case_run "A-stale-date" \
    "it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-01$TAIL\n" \
    '## Iteration: it' \
    'h\n---\nit validate s2 2026-07-05\nit close s3 2026-07-06\n' 'unit' \
    1 "before the earliest validate stamp"

# E2 — an imported clean line satisfies the close-entry block like a local one.
case_run "A-imported-line" \
    "it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-06${TAIL/origin=local/origin=imported}\n" \
    '## Iteration: it' \
    'h\n---\nit validate s2 2026-07-05\nit close s3 2026-07-06\n' 'unit' \
    0 "clean"

# B — the grammar of each major. A v2 header reds an unknown third field and each
# malformed appended field; the stage cursor is disarmed so only (B) speaks.
V1LINE="it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-06"
case_run "B-unknown-kind" "it unit mutant=x pass=1 fail=0 ignore=0 verdict=clean 2026-07-06$TAIL\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "unknown line kind"
case_run "B-v1-line-under-v2" "$V1LINE\n" '## Iteration: it' 'h\n---\n' '' 1 "malformed line"
case_run "B-bad-commit" "$V1LINE${TAIL/commit=$COMMIT/commit=ABC}\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "bad commit field"
case_run "B-bad-tree" "$V1LINE${TAIL/tree=clean/tree=-}\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "bad tree field"
case_run "B-bad-cmd" "$V1LINE${TAIL/cmd=$HASH/cmd=xyz}\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "bad cmd field"
case_run "B-bad-origin" "$V1LINE${TAIL/origin=local/origin=ci}\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "bad origin field"
case_run "B-unborn" "$V1LINE commit=- tree=- cmd=$HASH origin=local\n" \
    '## Iteration: it' 'h\n---\n' '' 0 "clean"
# A control line is v2's second kind: green under its suite line, red with none above it, red
# on a witness outside red|green|absent, and red at eight fields, its assign= dropped.
CTLLINE="it unit control=c scenario=t witnessed=red 2026-07-06 commit=$COMMIT cmd=$HASH assign=$HASH"
case_run "B-control-under-suite" "$V1LINE$TAIL\n$CTLLINE\n" '## Iteration: it' 'h\n---\n' '' 0 "clean"
case_run "B-control-no-suite" "$CTLLINE\n$V1LINE$TAIL\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "control line with no 'it unit' suite line above it"
case_run "B-control-bad-witness" "$V1LINE$TAIL\n${CTLLINE/witnessed=red/witnessed=pink}\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "bad witnessed field"
case_run "B-control-eight-fields" "$V1LINE$TAIL\n${CTLLINE% assign=*}\n" \
    '## Iteration: it' 'h\n---\n' '' 1 "malformed control line"
# A v1 file whose lines are v1 stays green; a v2 line under a v1 header is red.
MAJOR=v1
case_run "B-v1-file" "$V1LINE\n" '## Iteration: it' 'h\n---\n' '' 0 "clean"
case_run "B-v2-line-under-v1" "$V1LINE$TAIL\n" '## Iteration: it' 'h\n---\n' '' 1 "malformed line"
MAJOR=v3
case_run "B-unknown-major" "" '## Iteration: it' 'h\n---\n' '' 1 "not a versioned wire-format header"
MAJOR=v2

# F — the no-cursor window /scope leaves at the iteration boundary: the state
#     file exists but is truncated to its preamble + '---', so there is no data
#     line to read a stage from. A and C must disarm at the gate's DECLARED
#     early-out (the "no lifecycle state" exit) — asserting the message, not
#     merely exit 0, is what distinguishes a declared disarm from an empty stage
#     slipping past two live assertions. Suites are configured and a validate
#     stamp is absent by construction, so a header-sourced cursor would have
#     made this case fire A.
case_run "no-cursor-window" \
    '' '## Iteration: it' \
    'h\n---\n' 'unit extra' \
    0 "close-entry/stamp-coupling disarmed"

# G — no lifecycle state file at all: the second no-cursor shape, same early-out.
_no_state() {
    local d="$tmp/nostate"; mkdir -p "$d"
    printf '# contract: evidence-manifest v2\n' >"$d/man.txt"
    printf '## Iteration: it\n' >"$d/queue.md"
    gate_run check-evidence-manifest "$CHECKS" "$d/man.txt" "$d/queue.md" "$d/absent-state.txt" 2>&1
}
if ! out="$(_no_state)" || ! grep -qF "close-entry/stamp-coupling disarmed" <<<"$out"; then
    echo "  FAIL: no-lifecycle disarm not reported clean: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-evidence-manifest.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-evidence-manifest.test: ok (C recorded-nothing + A missing-suite + A stale-date + each v2 field and kind + a cross-major line + an unknown major rejected; C disarmed-at-validate + A green-block + A imported + a v1 file + an unborn line + both no-cursor shapes accepted)"
exit 0
