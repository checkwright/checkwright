#!/usr/bin/env bash
# Direct test of the shell-guard member's rule `git_mutation_under_producer` — a tracked-tree mutation blocked
# while a recorded producer is still alive. The decision table cannot hold this
# rule's firing arm: its second conjunct is a *live* PID, and a sandbox carrying
# one would turn every other git row in that table into a block. The table keeps
# the decline arm (a dead record present); everything below needs a process the
# test itself owns. Each case runs the whole ruleset, so a case another rule
# decides first asserts that the block is not this rule's.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

BIN="$GATE_SDK_NATIVE_BIN"
[[ -x "$BIN" ]] || { echo "git-mutation-under-producer.test: the gate binary $BIN is absent — build it first"; exit 2; }
BIN="$(cd "$(dirname "$BIN")" && pwd -P)/$(basename "$BIN")"

fails=0
checks=0
tmp="$(cd "$(mktemp -d)" && pwd -P)"
live=""
cleanup() { [[ -n "$live" ]] && kill "$live" 2>/dev/null; rm -rf "$tmp"; }
trap cleanup EXIT

sleep 60 &
live=$!

mkdir -p "$tmp/scratch" "$tmp/cwd"
git -C "$tmp/cwd" init -q
printf 'notes.txt\n' >"$tmp/cwd/.gitignore"
printf 'GUARD_KIT_SCRATCH_DIRS[] = %s/scratch\n' "$tmp" >"$tmp/guard.knobs"

payload() {
    local c="$1"
    c="${c//\\/\\\\}"
    c="${c//\"/\\\"}"
    printf '{"tool_name":"Bash","tool_input":{"command":"%s"}}' "$c"
}

# 2 = blocked, 0 = not blocked; the block text lands in $tmp/err.
verdict() {
    (cd "$tmp/cwd" && payload "$1" | GUARD_KIT_KNOB_FILE="$tmp/guard.knobs" GUARD_KIT_LOG="$tmp/friction.log" "$BIN" --hook shell-guard >/dev/null 2>"$tmp/err")
    echo $?
}

want() {  # $1=label $2=command $3=want-rc [$4=text the block must not carry]
    checks=$((checks + 1))
    local got; got="$(verdict "$2")"
    if [[ "$got" != "$3" ]]; then
        echo "  FAIL [$1]: '$2' gave rc=$got, want $3 — $(cat "$tmp/err")"
        fails=$((fails + 1))
    elif [[ -n "${4:-}" ]] && grep -qF -- "$4" "$tmp/err"; then
        echo "  FAIL [$1]: '$2' was blocked by the producer rule, which should have declined — $(cat "$tmp/err")"
        fails=$((fails + 1))
    fi
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
verdict "git commit -m done" >/dev/null
checks=$((checks + 1))
if ! grep -qF "validate-batch" "$tmp/err" || ! grep -qF "$live" "$tmp/err"; then
    echo "  FAIL [names-the-run]: the block did not name the blocking run and pid: $(cat "$tmp/err")"
    fails=$((fails + 1))
fi

# --- git's global options are walked, so a decorated invocation is still reached;
#     a '-c' override is refused by rule `git_c_root` before this rule reads it
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

# --- conservative in this ruleset's established directions: an expansion or a
#     substitution is refused by rule `expansion` first, and never by this rule
want "non-git"          "make build" 0
want "unknown-subcmd"   "git frobnicate --hard" 0
want "unknown-global"   "git --frobnicate commit -m done" 0
want "expansion"        "git commit -m \$MSG" 2 "validate-batch"
want "substitution"     "git commit -m \$(date)" 2 "validate-batch"
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
