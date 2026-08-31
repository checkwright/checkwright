#!/usr/bin/env bash
# Behavioral test of this repo's wiring of the SubagentStop hook: that the reader the
# --hook subagent-stop-liveness arm defaults to actually answers, and that its verdict reaches
# the exit code. delegation-kit's own test drives the hook with a stub reader, so it holds every
# verdict arm and cannot see whether the CONFIGURED reader resolves — which is how a port that
# deleted the reader's target left the hook logging verdict=unavailable on every firing with a
# green battery over it. Enforcement lands on exactly that seam, so both lanes move together.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the preamble above pins every kit's config file at
# an empty one so a KIT test runs on kit defaults; this is a CONSUMER-wiring test and since the
# port the reader's default lives in this repo's consumer config, so that file IS the subject here
export DELEGATION_KIT_CONFIG_FILE="$ROOT/scripts/delegation-config.sh"
# spec: gate-sdk/SPEC.md §The non-gate arm — the hook is a binary arm now, dispatched through the
# front-end; the subject of arms A-C is unchanged and only its substrate moved
HOOK_CMD=(bash "$ROOT/gate-sdk/bin/run-gates.sh" --hook subagent-stop-liveness)

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

PAYLOAD='{"session_id":"s-1","hook_event_name":"SubagentStop"}'

# fire <name> <run-dir> <want-rc> — one firing against a scratch run dir, returning the logged
# line. The knob is deliberately NOT set where the default is the subject: since the port that
# default lives in scripts/delegation-config.sh, so these arms are also what proves it resolves.
fire() {
    local name="$1" rundir="$2" want_rc="$3" log rc
    shift 3
    log="$tmp/$name.log"
    printf '%s' "$PAYLOAD" | env "$@" DELEGATION_KIT_STOP_LOG="$log" GATE_SDK_TMP_DIR="$rundir" \
        "${HOOK_CMD[@]}" >/dev/null 2>"$tmp/$name.err"
    rc=$?
    [[ "$rc" -eq "$want_rc" ]] || { echo "  FAIL: $name exited $rc, want $want_rc"; fails=$((fails + 1)); }
    cat "$log" 2>/dev/null
}

want() {  # $1=name $2=line $3.. = substrings the line must carry
    local name="$1" line="$2"; shift 2
    local s
    for s in "$@"; do
        grep -qF -- "$s" <<<"$line" || { echo "  FAIL: $name line lacks '$s': $line"; fails=$((fails + 1)); }
    done
}

cd "$ROOT" || { echo "  FAIL: cannot reach the repo root"; exit 1; }

# A — no records: the reader runs and reports a clean scratch dir, and the turn end is allowed.
#     verdict=unavailable here is the whole defect this test exists for, so it is asserted
#     against by name — and under enforcement it would also silently allow every turn end.
empty="$tmp/empty"; mkdir -p "$empty"
line="$(fire clean "$empty" 0)"
want clean "$line" "verdict=green" "live=no" "records=0" "decision=allow"

# B — one record naming a PID that is always alive: the reader reaches the gate's own liveness
#     predicate rather than merely resolving, so a reader that resolved and answered nothing
#     would still fail here. Through the configured reader that verdict now refuses, so the
#     exit code and the stderr reason are asserted beside the logged line.
live="$tmp/live"; mkdir -p "$live"
printf 'pid=1 run=k\n' >"$live/k.run"
line="$(fire live "$live" 2)"
want live "$line" "verdict=red" "live=yes" "records=1" "decision=refuse"
want live-message "$(cat "$tmp/live.err")" "turn-end refused" "delete the record once the producer has exited"

# C1 — the hook's OWN binary is absent. Before the port this arm broke only the reader's dispatch,
#      because the hook was a script; the hook and the reader now share one binary and one knob, so
#      GATE_SDK_NATIVE_BIN can no longer break one without the other. What it exercises instead is
#      the rule that replaced the old expectation: gate-sdk/SPEC.md §The non-gate arm fails a
#      harness-integration arm OPEN when it cannot run at all, so an absent binary allows the turn
#      end and writes no record, rather than refusing every turn end in a tree that never built.
absent="$tmp/absent"; mkdir -p "$absent"
line="$(fire no-binary "$absent" 0 GATE_SDK_NATIVE_BIN=/nonexistent)"
[[ -z "$line" ]] || { echo "  FAIL: an absent binary still wrote a record: $line"; fails=$((fails + 1)); }
want no-binary-message "$(cat "$tmp/no-binary.err")" "absent or not" "build-native.sh"

# C2 — the reader is configured and resolvable but produces no reading: over an EMPTY run dir that
#      is `unresolved` and it refuses. It must NOT be `unavailable` — that would misreport a wired
#      reader as a tree that never configured one, and it is the arm the stub lane cannot reach.
#      Driven by a reader that exits 2 rather than by the shared binary knob, which C1 now owns.
mute="$tmp/mute-reader.sh"; printf 'exit 2\n' > "$mute"
line="$(fire unresolved "$absent" 2 DELEGATION_KIT_LIVENESS_CMD="$mute")"
want unresolved "$line" "verdict=unresolved" "live=no" "records=0" "decision=refuse"
want unresolved-message "$(cat "$tmp/unresolved.err")" "turn-end refused" "produced no reading at all"

# D — the reader run from inside a REAL linked worktree with no build output of its own. This is
#     the only executable statement of the worktree resolution: a fresh checkout carries no
#     native/target, so before the resolution the reader failed closed here and the hook's
#     `unresolved` refusal displaced the child's report. The worktree is genuine (git worktree
#     add) but the script under test is copied in from the working tree, so the arm holds the
#     code being edited rather than whatever HEAD happens to carry.
#     GATE_SDK_NATIVE_BIN IS UNSET FOR THESE RUNS AND THE ARM IS VACUOUS WITHOUT IT: the hermetic
#     preamble exports an ABSOLUTE path into the main checkout, which is the one shape that
#     resolves inside a worktree anyway and the one shape the resolution deliberately skips. A
#     dispatched child inherits no such variable, so unsetting it is what makes this arm the
#     dispatch's environment rather than the harness's. Verified by reverting the reader: with the
#     export in place the pre-fix reader passed this arm.
wt="$tmp/wt"
if ! git worktree add --detach -q "$wt" HEAD 2>/dev/null; then
    echo "  FAIL: could not create a linked worktree — the worktree arm asserted nothing"; fails=$((fails + 1))
else
    cp "$ROOT/scripts/producer-liveness-reader.sh" "$wt/scripts/producer-liveness-reader.sh"
    [[ ! -e "$wt/native/target" ]] \
        || { echo "  FAIL: the linked worktree carries build output — the arm's premise is gone"; fails=$((fails + 1)); }
    mkdir -p "$wt/.tmp"
    out="$( cd "$wt" && env -u GATE_SDK_NATIVE_BIN bash scripts/producer-liveness-reader.sh .tmp 2>&1 )"; rc=$?
    [[ "$rc" -eq 0 ]] \
        || { echo "  FAIL: the reader exited $rc from a linked worktree, want 0: $out"; fails=$((fails + 1)); }

    # The resolution must remove the TRIGGER, not the signal: the same worktree with a live
    # record still answers red rather than answering green because it could not look.
    printf 'pid=1 run=k\n' >"$wt/.tmp/k.run"
    out="$( cd "$wt" && env -u GATE_SDK_NATIVE_BIN bash scripts/producer-liveness-reader.sh .tmp 2>&1 )"; rc=$?
    [[ "$rc" -eq 1 ]] \
        || { echo "  FAIL: a live record inside a linked worktree read $rc, want 1: $out"; fails=$((fails + 1)); }
    git worktree remove --force "$wt" 2>/dev/null || rm -rf "$wt"
    git worktree prune 2>/dev/null
fi

# E — the refusal still fires where it should: a main checkout whose configured binary is broken
#     is unresolvable for a reason the worktree resolution must not paper over. D removed a
#     trigger; this is the assertion that it removed no signal.
mkdir -p "$tmp/main-rd"
out="$(GATE_SDK_NATIVE_BIN=native/target/release/nope bash "$ROOT/scripts/producer-liveness-reader.sh" "$tmp/main-rd" 2>&1)"; rc=$?
[[ "$rc" -eq 2 ]] \
    || { echo "  FAIL: a broken binary in a MAIN checkout exited $rc, want 2: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "subagent-stop-reader.test: $fails assertion(s) failed"
    exit 1
fi
echo "subagent-stop-reader.test: ok (the configured reader resolves and answers, and its verdict reaches the exit: clean scratch dir green and allowed, live producer red and refused with a reason, an absent dispatch binary declining open with no record, a reader that produces no reading unresolved and refused rather than reported unavailable; a linked worktree resolves through the main checkout and still reads its records, while a main checkout with a broken binary still refuses)"
exit 0
