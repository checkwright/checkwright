#!/usr/bin/env bash
# Behavioral test of check-script-preflight over what the good/+bad/ pair cannot reach: an
# empty configuration, operands, the rules file's exit-2 and exit-3 classes, an unparseable
# script, and the prune set over the tracked tree.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
CASES="$DIR/gate-tests/check-script-preflight"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
cases=0

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    cases=$((cases + 1))
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

refute() {  # refute <label> <substring> <output>
    if grep -qF -- "$2" <<<"$3"; then
        echo "  FAIL [$1]: output holds '$2': $3"; fails=$((fails + 1))
    fi
}

# --- no script configured: clean, not applicable, and no rules file read.
mkdir -p "$SANDBOX/empty"
out="$( cd "$SANDBOX/empty" && gate_run check-script-preflight "$CHECKS" 2>&1 )"; rc=$?
expect not-applicable 0 'not applicable: no script configured' "$rc" "$out"

# --- an operand naming nothing on disk is refused at 2.
out="$( cd "$SANDBOX/empty" && gate_run check-script-preflight "$CHECKS" gone.sh 2>&1 )"; rc=$?
expect absent-operand 2 'no script at gone.sh' "$rc" "$out"

# --- an operand is read tracked or not, and narrows the corpus to itself.
mkdir -p "$SANDBOX/operand"
cp "$CASES/bad/rules.list" "$SANDBOX/operand/"
printf '#!/usr/bin/env bash\npkill -f job\n' > "$SANDBOX/operand/one.sh"
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=rules.list \
    && gate_run check-script-preflight "$CHECKS" one.sh 2>&1 )"; rc=$?
expect operand 1 'one.sh:2: [KILL-BROAD] pkill' "$rc" "$out"

# --- the rules file: absent while a corpus is configured is refused at 2, unreadable is 3, and a
# malformed one is 2 naming its file and line.
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=missing.list \
    && gate_run check-script-preflight "$CHECKS" one.sh 2>&1 )"; rc=$?
expect rules-absent 2 'missing.list' "$rc" "$out"
printf 'rule KILL command\n  cmd pkill\n\xff\n' > "$SANDBOX/operand/binary.list"
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=binary.list \
    && gate_run check-script-preflight "$CHECKS" one.sh 2>&1 )"; rc=$?
expect rules-unreadable 3 'binary.list' "$rc" "$out"
printf 'waiver-min 2\nrule KILL command\n  cmd pkill\n  allow (\n' > "$SANDBOX/operand/ere.list"
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=ere.list \
    && gate_run check-script-preflight "$CHECKS" one.sh 2>&1 )"; rc=$?
expect rules-ere 2 'ere.list:4: allow:' "$rc" "$out"
printf 'rule kill command\n  cmd pkill\n' > "$SANDBOX/operand/id.list"
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=id.list \
    && gate_run check-script-preflight "$CHECKS" one.sh 2>&1 )"; rc=$?
expect rules-id 2 'id.list:1: rule id' "$rc" "$out"
printf 'rule KILL command\n  allow x\n' > "$SANDBOX/operand/missing-key.list"
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=missing-key.list \
    && gate_run check-script-preflight "$CHECKS" one.sh 2>&1 )"; rc=$?
expect rules-missing-key 2 "missing-key.list:1: rule KILL lacks its 'cmd' key" "$rc" "$out"

# --- a span the reader cannot read is LINT-PARSE at 3; a script not UTF-8 is 3.
printf '#!/usr/bin/env bash\necho "open\n' > "$SANDBOX/operand/broken.sh"
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=rules.list \
    && gate_run check-script-preflight "$CHECKS" broken.sh 2>&1 )"; rc=$?
expect lint-parse 3 'broken.sh:2: [LINT-PARSE] unparseable span' "$rc" "$out"
printf 'echo \xff\n' > "$SANDBOX/operand/bytes.sh"
out="$( cd "$SANDBOX/operand" && gate_env GATE_SDK_SCRIPT_RULES=rules.list \
    && gate_run check-script-preflight "$CHECKS" bytes.sh 2>&1 )"; rc=$?
expect not-utf8 3 'bytes.sh' "$rc" "$out"

# --- the knob's pathspecs select tracked files only, and the prune set is never read.
mkdir -p "$SANDBOX/tree/target" "$SANDBOX/tree/lib"
cp "$CASES/bad/rules.list" "$SANDBOX/tree/"
printf 'pkill -P "$pid"\n' > "$SANDBOX/tree/lib/ok.sh"
printf 'pkill -f job\n' > "$SANDBOX/tree/target/built.sh"
printf 'pkill -f job\n' > "$SANDBOX/tree/untracked.sh"
git -C "$SANDBOX/tree" init -q
git -C "$SANDBOX/tree" add lib/ok.sh target/built.sh
out="$( cd "$SANDBOX/tree" && gate_env 'GATE_SDK_SCRIPT_FILES=*.sh' GATE_SDK_SCRIPT_RULES=rules.list \
    && gate_run check-script-preflight "$CHECKS" 2>&1 )"; rc=$?
expect tracked-pruned 0 'clean (1 script(s) read against 4 rule(s)' "$rc" "$out"
refute tracked-only 'untracked.sh' "$out"

# --- outside a work tree the tracked listing fails, and the gate reads that as exit 3, never clean.
out="$( cd "$SANDBOX/empty" && gate_env 'GATE_SDK_SCRIPT_FILES=*.sh' GATE_SDK_SCRIPT_RULES=rules.list \
    && gate_run check-script-preflight "$CHECKS" 2>&1 )"; rc=$?
expect no-work-tree 3 'check-script-preflight' "$rc" "$out"

# --- an option the gate does not take is a usage error.
out="$( cd "$SANDBOX/empty" && gate_run check-script-preflight "$CHECKS" --all 2>&1 )"; rc=$?
expect usage 2 'unknown option --all' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-script-preflight.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-script-preflight.test.sh: clean (configuration, operands, the rules file's exit classes, LINT-PARSE and the tracked corpus — $cases cases)"
exit 0
