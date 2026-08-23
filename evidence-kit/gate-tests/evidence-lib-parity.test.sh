#!/usr/bin/env bash
# Cross-implementation parity for the two derivations evidence-kit holds twice after
# shell-gate-tail-port: `ek_lock_read` and `ek_pid_alive` in evidence-kit/lib/evidence.sh, and
# their compiled counterparts in native/src/evidence.rs. bin/run-validate.sh still calls both
# (`:32`, `:48`, `:52`), so the caller set does not empty and the duplication is permanent — which
# is criterion 6's *unless* clause and its machine-held disposition rather than the deletion one
# (gate-sdk/SPEC.md §The port-candidate criteria, criterion 6).
#
# What is compared is *classification* over one canned corpus, never a derived literal: the shell
# side answers in exit codes and one stdout line, the crate in an enum. A against B directly, with
# no committed expected file — a maintained golden would be a third copy to drift, and the failure
# this exists to catch is one side edited without the other.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$GATE_SDK_TEST_LIB_DIR/gate.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
SANDBOX="$(mktemp -d)"
live=""
cleanup() { [[ -n "$live" ]] && kill "$live" 2>/dev/null; rm -rf "$SANDBOX"; }
trap cleanup EXIT

fails=0
checks=0

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a declaration is not a dispatch: a consumer on
# an uncovered platform vendors the shell library with no artifact behind it, and a parity
# assertion there would be vacuous rather than true. The skip is declared on the clean line, in the
# shape the port's omitted-member roster uses, so a reader can tell "no binary here" from "parity
# holds". A binary that is present and refuses the arm is a stale binary, so it fails here.
BIN="$(gate_native_bin)"
if [[ ! -x "$BIN" ]]; then
    echo "evidence-lib-parity.test: ok (0 assertions; skipped — no gate binary at $BIN, so nothing dispatches to the compiled twin)"
    exit 0
fi

sleep 300 &
live=$!

# The lock corpus reaches every branch of the record grammar plus the two non-record readings the
# helper must keep apart: an absent file is the *free* reading and an unreadable line is
# corruption, and folding either into the other is what §The producer-liveness lock forbids.
mkdir -p "$SANDBOX/locks"
printf 'pid=1234 run=alpha\n'        > "$SANDBOX/locks/wellformed"
printf 'pid=1234 run=alpha'          > "$SANDBOX/locks/no-trailing-newline"
: >                                    "$SANDBOX/locks/empty"
printf 'garbage\n'                   > "$SANDBOX/locks/garbage"
printf 'pid=0 run=alpha\n'           > "$SANDBOX/locks/zero-pid"
printf 'pid=0123 run=alpha\n'        > "$SANDBOX/locks/leading-zero"
printf 'pid=1234  run=alpha\n'       > "$SANDBOX/locks/two-spaces"
printf 'pid=1234\trun=alpha\n'       > "$SANDBOX/locks/tab-separated"
printf 'pid=1234 alpha\n'            > "$SANDBOX/locks/no-run-field"
printf 'pid=1234 run=\n'             > "$SANDBOX/locks/empty-run-key"
printf 'pid=1234 run=alpha \n'       > "$SANDBOX/locks/trailing-space"
printf 'pid=1234 run=alpha\r\n'      > "$SANDBOX/locks/carriage-return"
printf 'pid=1234 run=a/b-c.d\n'      > "$SANDBOX/locks/punctuated-run-key"
printf 'pid=1234 run=alpha\nsecond line\n' > "$SANDBOX/locks/two-lines"
mkdir -p "$SANDBOX/locks/a-directory"

LOCKS=(
    "$SANDBOX/locks/wellformed"
    "$SANDBOX/locks/no-trailing-newline"
    "$SANDBOX/locks/empty"
    "$SANDBOX/locks/garbage"
    "$SANDBOX/locks/zero-pid"
    "$SANDBOX/locks/leading-zero"
    "$SANDBOX/locks/two-spaces"
    "$SANDBOX/locks/tab-separated"
    "$SANDBOX/locks/no-run-field"
    "$SANDBOX/locks/empty-run-key"
    "$SANDBOX/locks/trailing-space"
    "$SANDBOX/locks/carriage-return"
    "$SANDBOX/locks/punctuated-run-key"
    "$SANDBOX/locks/two-lines"
    "$SANDBOX/locks/a-directory"
    "$SANDBOX/locks/absent"
)

# The pid corpus: a process this test owns, one it cannot own but every platform has, one past the
# system maximum, and the malformed spellings the grammar rejects before it probes anything.
PIDS=("$live" 1 "$$" 2147483646 0 01 12x " 12" abc -1 "" 99999999999999999999)

shell_side() {
    (
        # shellcheck source=../lib/evidence.sh
        source "$DIR/lib/evidence.sh"
        local f holder hs p
        for f in "${LOCKS[@]}"; do
            holder="$(ek_lock_read "$f")"; hs=$?
            case "$hs" in
                0) printf 'lock\t%s\theld\t%s\t%s\n' "$f" "${holder%% *}" "${holder#* }" ;;
                2) printf 'lock\t%s\tcorrupt\n' "$f" ;;
                *) printf 'lock\t%s\tabsent\n' "$f" ;;
            esac
        done
        for p in "${PIDS[@]}"; do
            if ek_pid_alive "$p"; then
                printf 'pid\t%s\talive\n' "$p"
            else
                printf 'pid\t%s\tdead\n' "$p"
            fi
        done
    )
}

# The compiled side is reached through the same binary a dispatched gate reaches, and the arm
# reports classification rather than an internal representation — `--queue-parity`'s own rule.
native_side() {
    "$BIN" --evidence-lib-parity lock "${LOCKS[@]}" || return $?
    "$BIN" --evidence-lib-parity pid "${PIDS[@]}"
}

checks=$((checks + 1))
a="$(shell_side)"; arc=$?
b="$(native_side)"; brc=$?
if [[ "$arc" -ne 0 || "$brc" -ne 0 ]]; then
    echo "  FAIL: a side could not report (shell exit $arc, binary exit $brc)"
    fails=$((fails + 1))
elif [[ -z "$a" ]]; then
    echo "  FAIL: the shell side classified nothing — a vacuous agreement, not a parity hold"
    fails=$((fails + 1))
elif [[ "$a" != "$b" ]]; then
    echo "  FAIL: the two implementations disagree about the same corpus:"
    diff <(printf '%s\n' "$a") <(printf '%s\n' "$b") | sed 's/^/    /'
    fails=$((fails + 1))
fi

# The corpus must actually reach the branches the comparison is bought for: an agreement over a
# corpus that classifies nothing is the vacuity this lane exists to end, arriving one layer up.
# Each is read off the shell side, the one that owns the helpers under obligation.
T=$'\t'
have() {   # $1=label $2=grep -E pattern
    checks=$((checks + 1))
    grep -qE "$2" <<<"$a" || {
        echo "  FAIL [$1]: the corpus no longer exercises this branch"
        fails=$((fails + 1))
    }
}
have "lock-held"     "^lock${T}.*${T}held${T}1234${T}alpha$"
have "lock-corrupt"  "^lock${T}.*/garbage${T}corrupt$"
have "lock-absent"   "^lock${T}.*/absent${T}absent$"
have "lock-dir"      "^lock${T}.*/a-directory${T}absent$"
have "pid-alive"     "^pid${T}$live${T}alive$"
have "pid-dead"      "^pid${T}2147483646${T}dead$"
have "pid-malformed" "^pid${T}12x${T}dead$"

# The discriminating pair for the fallback leg, which is the whole reason `ps` is a declared
# dependency: PID 1 is the one PID this test can assert the liveness of on every platform and
# cannot signal, so `kill -0` alone reads it as dead and only `ps -p` recovers it. If this line
# ever reads `dead`, the fallback has been dropped on one side or the other.
checks=$((checks + 1))
grep -qE "^pid${T}1${T}alive$" <<<"$a" || {
    echo "  FAIL [pid-fallback]: init read as dead, so the ps fallback is gone from the shell side"
    fails=$((fails + 1))
}

if [[ "$fails" -gt 0 ]]; then
    echo "evidence-lib-parity.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "evidence-lib-parity.test: ok ($checks assertions; ek_lock_read and ek_pid_alive held to their compiled twins over ${#LOCKS[@]} records and ${#PIDS[@]} pids)"
exit 0
