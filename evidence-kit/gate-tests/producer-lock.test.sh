#!/usr/bin/env bash
# Behavioral test of the producer-liveness lock — the halves no gate fixture
# pair can hold. The pair covers the reader's two static verdicts; everything
# below needs a process the test itself owns (a live PID) or a run of the
# writer (refusal, bounded reclaim, conditional release).
#
# spec: evidence-kit/SPEC.md §check-producer-liveness — the reader is reached through `gate_run`
# and named as a *gate*, never as a script path: the member is `.gate`-dispatched since
# shell-gate-tail-port, and a path would have pinned the substrate this test is indifferent to.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
CHECKS="$DIR/checks"
# spec: evidence-kit/SPEC.md §bin/run-validate.sh — the writer is driven through the front end
# rather than by a path: it is the bridged `--run-validate` arm, and the front end is what
# resolves its declared knob roster. The binary needs no pin here — the preamble above already
# exports GATE_SDK_NATIVE_BIN absolute, which is what survives the front end's cd to each
# scratch tree's own toplevel, where the knob's repo-relative default would name nothing.
FE="$(cd "$DIR/../gate-sdk/bin" && pwd)/run-gates.sh"

fails=0
tmp="$(mktemp -d)"
live=""
cleanup() { [[ -n "$live" ]] && kill "$live" 2>/dev/null; rm -rf "$tmp"; }
trap cleanup EXIT

sleep 60 &
live=$!

# spec: evidence-kit/SPEC.md §bin/run-validate.sh — each scratch tree is its own git toplevel,
# because the front end refuses outside a repository and resolves every relative knob against the
# toplevel it lands on: nested inside one, a run would read the enclosing tree and still exit 0.
mk_tree() {
    rm -rf "$1"; mkdir -p "$1/.workflow" "$1/scripts" "$1/.tmp"
    ( cd "$1" && git init -q . ) >/dev/null 2>&1
    printf '# baseline\ngreen green pass\n' >"$1/.workflow/validate-baseline.txt"
    printf '# contract: evidence-manifest v1\n' >"$1/.workflow/validate-evidence.txt"
    printf "EVIDENCE_KIT_SUITES=(green)\nEVIDENCE_KIT_PARSER=exit-code\nEVIDENCE_KIT_RUN_ID=lock-test\nEVIDENCE_KIT_RUN_green='%s'\n" \
        "${2:-true}" >"$1/scripts/evidence-config.sh"
}
# test-hermetic pins EVIDENCE_KIT_CONFIG_FILE to a shared empty file, so each
# scratch tree names its own config rather than relying on the cwd lookup.
_rv() { ( cd "$1" && EVIDENCE_KIT_CONFIG_FILE=scripts/evidence-config.sh bash "$FE" --run-validate 2>&1 ); }

# A — the reader reds on a PID the test owns. The fixture pair's bad case can
#     only reach for PID 1, so the live-PID verdict is pinned here against a
#     process whose liveness this test controls end to end.
printf 'pid=%s run=owned-run\n' "$live" >"$tmp/a.lock"
out="$(gate_run check-producer-liveness "$CHECKS" "$tmp/a.lock" 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]] || [[ "$out" != *"still running (pid $live)"* ]]; then
    echo "  FAIL: the reader did not red on a live PID the test owns (rc=$rc): $out"; fails=$((fails + 1))
fi

# B — a lock that does not parse is exit 2, never a free reading. The claim
#     publishes the record whole, so an unparseable lock is corruption.
printf 'garbage\n' >"$tmp/b.lock"
out="$(gate_run check-producer-liveness "$CHECKS" "$tmp/b.lock" 2>&1)"; rc=$?
if [[ "$rc" -ne 2 ]]; then
    echo "  FAIL: an unparseable lock must be exit 2, got rc=$rc: $out"; fails=$((fails + 1))
fi

# B2 — set mode's four verdicts. The gate's fixture pair holds exactly one good
#      and one bad case, so the quantifier's aggregation rule cannot live there;
#      three of these four also need a multi-record directory the pair has no
#      shape for, and one needs a live PID the pair can only reach as init.
set_dir="$tmp/set"
mkdir -p "$set_dir"
out="$(gate_run check-producer-liveness "$CHECKS" "$set_dir" 2>&1)"; rc=$?
if [[ "$rc" -ne 0 ]] || [[ "$out" != *"no '*.run' record"* ]]; then
    echo "  FAIL: an empty directory must be green (rc=$rc): $out"; fails=$((fails + 1))
fi

printf 'pid=2147483646 run=dead-a\n' >"$set_dir/dead-a.run"
printf 'pid=2147483645 run=dead-b\n' >"$set_dir/dead-b.run"
out="$(gate_run check-producer-liveness "$CHECKS" "$set_dir" 2>&1)"; rc=$?
if [[ "$rc" -ne 0 ]] || [[ "$out" != *"none naming a live pid"* ]]; then
    echo "  FAIL: a directory of dead records must be green (rc=$rc): $out"; fails=$((fails + 1))
fi

printf 'pid=%s run=owned-set-run\n' "$live" >"$set_dir/owned-set-run.run"
out="$(gate_run check-producer-liveness "$CHECKS" "$set_dir" 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]] || [[ "$out" != *"run key 'owned-set-run' is still running (pid $live)"* ]]; then
    echo "  FAIL: one live record among dead ones must red and name it (rc=$rc): $out"; fails=$((fails + 1))
fi

# The aggregation rule: exit 2 wins over red, so a corrupt record is never
# averaged away by the clean ones beside it — nor by the live one still red here.
printf 'garbage\n' >"$set_dir/broken.run"
out="$(gate_run check-producer-liveness "$CHECKS" "$set_dir" 2>&1)"; rc=$?
if [[ "$rc" -ne 2 ]] || [[ "$out" != *"broken.run carries no readable"* ]]; then
    echo "  FAIL: an unparseable record must win over red with exit 2 (rc=$rc): $out"; fails=$((fails + 1))
fi

# A non-'.run' file in the same directory is not a record and is not read: the
# suffix is what makes the set derivable, so a stray file must not be corruption.
rm -f "$set_dir/broken.run"
printf 'garbage\n' >"$set_dir/notes.txt"
out="$(gate_run check-producer-liveness "$CHECKS" "$set_dir" 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL: a non-'.run' file must be invisible to set mode (rc=$rc): $out"; fails=$((fails + 1))
fi

# C — delta 6's headline: the writer refuses to start against a *live* PID,
#     names the blocking run key, and attempts no reclaim — the lock it found
#     is still there, untouched, when it exits.
mk_tree "$tmp/c"
printf 'pid=%s run=blocking-run\n' "$live" >"$tmp/c/.tmp/run-validate.lock"
out="$(_rv "$tmp/c")"; rc=$?
if [[ "$rc" -eq 0 ]] || [[ "$out" != *"blocking-run"* ]]; then
    echo "  FAIL: the writer did not refuse a live lock naming its run key (rc=$rc): $out"; fails=$((fails + 1))
fi
if [[ "$(cat "$tmp/c/.tmp/run-validate.lock" 2>/dev/null)" != "pid=$live run=blocking-run" ]]; then
    echo "  FAIL: the refusal disturbed the live holder's lock"; fails=$((fails + 1))
fi
if grep -q '^lock-test ' "$tmp/c/.workflow/validate-evidence.txt"; then
    echo "  FAIL: a refused run still recorded evidence"; fails=$((fails + 1))
fi

# D — a dead-PID lock is reclaimed, the run proceeds, and the trap releases the
#     lock it then owned.
mk_tree "$tmp/d"
printf 'pid=2147483646 run=stale-run\n' >"$tmp/d/.tmp/run-validate.lock"
out="$(_rv "$tmp/d")"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -q '^lock-test green ' "$tmp/d/.workflow/validate-evidence.txt"; then
    echo "  FAIL: a dead-PID lock was not reclaimed (rc=$rc): $out"; fails=$((fails + 1))
fi
if [[ -e "$tmp/d/.tmp/run-validate.lock" ]]; then
    echo "  FAIL: the EXIT trap did not release the run's own lock"; fails=$((fails + 1))
fi

# E — the reclaim is bounded. A claim that keeps failing with no live holder
#     must refuse on the second attempt rather than loop. An unclaimable lock
#     path is the deterministic stand-in: losing the reclaim race to another
#     producer reaches the same branch but cannot be scheduled reliably, and its
#     more common outcome is the live-holder refusal arm C already pins.
mk_tree "$tmp/e"
printf "EVIDENCE_KIT_LOCK_FILE='no-such-dir/run-validate.lock'\n" >>"$tmp/e/scripts/evidence-config.sh"
out="$(timeout 30 bash -c "cd '$tmp/e' && EVIDENCE_KIT_CONFIG_FILE=scripts/evidence-config.sh bash '$FE' --run-validate 2>&1")"; rc=$?
if [[ "$rc" -eq 124 ]] || [[ "$out" != *"refusing to start rather than retrying"* ]]; then
    echo "  FAIL: the reclaim is not bounded to one retry (rc=$rc): $out"; fails=$((fails + 1))
fi

# F — the release is conditional. The suite command removes this run's lock out
#     of band and a second producer claims the freed slot; the exiting run must
#     leave that live record alone. An unconditional rm -f deletes it, which is
#     this unit's own defect reproduced inside its own mechanism.
mk_tree "$tmp/f" "bash scripts/steal-lock.sh"
printf '#!/usr/bin/env bash\nrm -f .tmp/run-validate.lock\nprintf "pid=%%s run=second-producer\\n" "%s" > .tmp/run-validate.lock\nexit 0\n' \
    "$live" >"$tmp/f/scripts/steal-lock.sh"
out="$(_rv "$tmp/f")"; rc=$?
if [[ "$(cat "$tmp/f/.tmp/run-validate.lock" 2>/dev/null)" != "pid=$live run=second-producer" ]]; then
    echo "  FAIL: the EXIT trap removed a lock that was no longer ours (rc=$rc): $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "producer-lock.test: $fails assertion(s) failed"
    exit 1
fi
echo "producer-lock.test: ok (live-PID red, unparseable lock exit 2, set mode's four verdicts and its suffix bound, writer refusal on a live lock, bounded reclaim of a dead one, conditional release)"
exit 0
