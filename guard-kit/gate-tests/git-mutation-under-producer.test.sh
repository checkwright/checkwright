#!/usr/bin/env bash
# Direct unit test of lib/guard.sh's rule 14 — a tracked-tree mutation blocked
# while a recorded producer is still alive. The decision table cannot hold this
# rule's firing arm: its second conjunct is a *live* PID, and a sandbox carrying
# one would turn every other git row in that table into a block. The table keeps
# the decline arm (a dead record present); everything below needs a process the
# test itself owns.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # guard-kit/
# shellcheck source=../lib/guard.sh
source "$DIR/lib/guard.sh"

fails=0
checks=0
tmp="$(mktemp -d)"
live=""
cleanup() { [[ -n "$live" ]] && kill "$live" 2>/dev/null; rm -rf "$tmp"; }
trap cleanup EXIT

sleep 60 &
live=$!

mkdir -p "$tmp/scratch"
GUARD_KIT_SCRATCH_DIRS=("$tmp/scratch")

# guard_block exits 2, so each verdict is taken in a subshell: 2 = blocked,
# 0 = declined and fell through to the rules after it.
verdict() { ( guard_rule_git_mutation_under_producer "$1" ) >/dev/null 2>&1; echo $?; }

want() {  # $1=label $2=command $3=want-rc
    checks=$((checks + 1))
    local got; got="$(verdict "$2")"
    [[ "$got" == "$3" ]] || { echo "  FAIL [$1]: '$2' gave rc=$got, want $3"; fails=$((fails + 1)); }
}

# --- no record at all: the rule is inert, whatever the command
want "empty-dir-commit" "git commit -m done" 0

# --- a record naming a dead pid is not a live producer
printf 'pid=2147483646 run=dead-run\n' >"$tmp/scratch/dead-run.run"
want "dead-record-commit" "git commit -m done" 0
want "dead-record-add"    "git add -A" 0

# --- a live record: the whole named write set blocks
printf 'pid=%s run=validate-batch\n' "$live" >"$tmp/scratch/validate-batch.run"
for sub in add commit rm mv restore checkout switch reset stash merge rebase \
    cherry-pick revert apply am clean; do
    want "live-blocks-$sub" "git $sub" 2
done

# The corrective must name the blocking run so the reader can tell 'wait for
# that' from 'reclaim a record whose owner is gone'.
out="$( ( guard_rule_git_mutation_under_producer "git commit -m done" ) 2>&1 )"
checks=$((checks + 1))
if [[ "$out" != *"validate-batch"* || "$out" != *"$live"* ]]; then
    echo "  FAIL [names-the-run]: the block did not name the blocking run and pid: $out"
    fails=$((fails + 1))
fi

# --- git's global options are walked, so a decorated invocation is still reached
want "global-C"        "git -C . commit -m done" 2
want "global-c"        "git -c user.name=x commit -m done" 2
want "global-no-pager" "git --no-pager stash" 2
want "global-glued"    "git --git-dir=.git commit -m done" 2

# --- and a compound fires on the mutating segment wherever it sits
want "compound-tail" "make build; git commit -m done" 2

# --- read-only git passes: the harm is the mutation, not the producer's liveness
for sub in status log diff show rev-parse ls-files branch remote; do
    want "readonly-$sub" "git $sub" 0
done
want "readonly-args" "git log --oneline -3" 0

# --- conservative in this ruleset's established directions
want "non-git"          "make build" 0
want "unknown-subcmd"   "git frobnicate --hard" 0
want "unknown-global"   "git --frobnicate commit -m done" 0
want "expansion"        "git commit -m \$MSG" 0
want "substitution"     "git commit -m \$(date)" 0
want "backtick"         'git commit -m `date`' 0
# A mutating verb inside a quoted span is not a command: the skeleton view is
# what makes that true, and a rule reading the raw text would false-block here.
want "quoted-mention"   "printf 'git commit -m x' >> notes.txt" 0

# --- a record that does not parse declines rather than blocks: a guard is not
#     where a corruption verdict is taken (check-producer-liveness exits 2 on one)
rm -f "$tmp/scratch/validate-batch.run" "$tmp/scratch/dead-run.run"
printf 'garbage\n' >"$tmp/scratch/broken.run"
want "unparseable-record" "git commit -m done" 0

# --- and a file without the '.run' suffix is not a record at all
printf 'pid=%s run=not-a-record\n' "$live" >"$tmp/scratch/notes.txt"
want "suffix-bound" "git commit -m done" 0

if [[ "$fails" -gt 0 ]]; then
    echo "git-mutation-under-producer.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "git-mutation-under-producer.test: ok ($checks assertions; the write set blocks under a live record, read-only git and every conservative direction decline)"
exit 0
