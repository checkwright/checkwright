#!/usr/bin/env bash
# Behavioral test of check-pipefail-readers and its fix arm over what the good/+bad/ pair
# cannot reach: an empty configuration, operands, the exit-2 and exit-3 classes, the
# whole-file knob, the prune set, and the arm rewriting a tree in place.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
CASES="$DIR/gate-tests/check-pipefail-readers"
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

# --- no path configured: clean, not applicable.
mkdir -p "$SANDBOX/empty"
out="$( cd "$SANDBOX/empty" && gate_run check-pipefail-readers "$CHECKS" 2>&1 )"; rc=$?
expect not-applicable 0 'not applicable: no shell path configured' "$rc" "$out"

# --- an operand naming nothing on disk is refused at 2.
out="$( cd "$SANDBOX/empty" && gate_run check-pipefail-readers "$CHECKS" gone.sh 2>&1 )"; rc=$?
expect absent-operand 2 'no shell file or directory at gone.sh' "$rc" "$out"

# --- an operand narrows the corpus: only its records, and no stale row for a file outside it.
out="$( cd "$CASES/bad" && gate_env GATE_SDK_PIPEFAIL_BASELINE=baseline.tsv \
    && gate_run check-pipefail-readers "$CHECKS" tree/over.sh 2>&1 )"; rc=$?
expect operand 1 'tree/over.sh:5: [PIPEFAIL-EARLY-EXIT] grep -q [auto-fix]' "$rc" "$out"
refute operand-readers 'tree/readers.sh' "$out"
refute operand-stale 'BASELINE-STALE' "$out"

# --- the whole-file knob lints a file that never sets pipefail.
out="$( cd "$CASES/good" && gate_env GATE_SDK_PIPEFAIL_PATHS=tree GATE_SDK_PIPEFAIL_BASELINE=baseline.tsv GATE_SDK_PIPEFAIL_ALL=1 \
    && gate_run check-pipefail-readers "$CHECKS" 2>&1 )"; rc=$?
expect all-files 1 'tree/no-pipefail.sh:3: [PIPEFAIL-EARLY-EXIT] head [auto-fix]' "$rc" "$out"

# --- a value outside 0|1 is refused at 2.
out="$( cd "$SANDBOX/empty" && gate_env GATE_SDK_PIPEFAIL_ALL=yes \
    && gate_run check-pipefail-readers "$CHECKS" 2>&1 )"; rc=$?
expect all-value 2 'GATE_SDK_PIPEFAIL_ALL must be 0|1' "$rc" "$out"

# --- the waiver floor is the consumer's: a five-character reason the default passes reds under
# a floor of 10, and a value that is no positive integer is refused at 2.
mkdir -p "$SANDBOX/floor"
printf '#!/usr/bin/env bash\nset -o pipefail\nls | head -1 # pipefail-ok: short\n' > "$SANDBOX/floor/waived.sh"
out="$( cd "$SANDBOX/floor" && gate_run check-pipefail-readers "$CHECKS" waived.sh 2>&1 )"; rc=$?
expect floor-default 0 'clean (1 shell file(s) read' "$rc" "$out"
out="$( cd "$SANDBOX/floor" && gate_env GATE_SDK_PIPEFAIL_WAIVER_MIN=10 \
    && gate_run check-pipefail-readers "$CHECKS" waived.sh 2>&1 )"; rc=$?
expect floor-raised 1 'waived.sh:3: [PIPEFAIL-EARLY-EXIT] waiver reason under 10 characters' "$rc" "$out"
out="$( cd "$SANDBOX/floor" && gate_env GATE_SDK_PIPEFAIL_WAIVER_MIN=0 \
    && gate_run check-pipefail-readers "$CHECKS" waived.sh 2>&1 )"; rc=$?
expect floor-value 2 'GATE_SDK_PIPEFAIL_WAIVER_MIN must be a positive integer' "$rc" "$out"

# --- a span the reader cannot read is LINT-PARSE at 3, and the fix arm writes nothing to it.
mkdir -p "$SANDBOX/parse"
printf '#!/usr/bin/env bash\nset -o pipefail\necho "open\nls | head -1\n' > "$SANDBOX/parse/broken.sh"
cp "$SANDBOX/parse/broken.sh" "$SANDBOX/parse.orig"
out="$( cd "$SANDBOX/parse" && gate_run check-pipefail-readers "$CHECKS" broken.sh 2>&1 )"; rc=$?
expect lint-parse 3 'broken.sh:3: [LINT-PARSE]' "$rc" "$out"
out="$( cd "$SANDBOX/parse" && gate_arm_run --emit pipefail-fix broken.sh 2>&1 )"; rc=$?
expect fix-unparsed 3 'could not rewrite' "$rc" "$out"
cmp -s "$SANDBOX/parse/broken.sh" "$SANDBOX/parse.orig" \
    || { echo "  FAIL [fix-unparsed]: the arm wrote to an unparseable file"; fails=$((fails + 1)); }

# --- a baseline: absent is refused at 2, a malformed row is exit 3 naming its line.
mkdir -p "$SANDBOX/base"
printf '#!/usr/bin/env bash\nset -o pipefail\nls | head -1\n' > "$SANDBOX/base/one.sh"
out="$( cd "$SANDBOX/base" && gate_env GATE_SDK_PIPEFAIL_PATHS=. GATE_SDK_PIPEFAIL_BASELINE=missing.tsv \
    && gate_run check-pipefail-readers "$CHECKS" 2>&1 )"; rc=$?
expect baseline-absent 2 'missing.tsv' "$rc" "$out"
printf '# rows\none\tone.sh\n' > "$SANDBOX/base/bad.tsv"
out="$( cd "$SANDBOX/base" && gate_env GATE_SDK_PIPEFAIL_PATHS=. GATE_SDK_PIPEFAIL_BASELINE=bad.tsv \
    && gate_run check-pipefail-readers "$CHECKS" 2>&1 )"; rc=$?
expect baseline-malformed 3 'bad.tsv:2' "$rc" "$out"

# --- a directory in the prune set is never walked.
mkdir -p "$SANDBOX/prune/node_modules/pkg"
cp "$SANDBOX/base/one.sh" "$SANDBOX/prune/node_modules/pkg/one.sh"
out="$( cd "$SANDBOX/prune" && gate_env GATE_SDK_PIPEFAIL_PATHS=. \
    && gate_run check-pipefail-readers "$CHECKS" 2>&1 )"; rc=$?
expect prune 0 'clean (0 shell file(s) read' "$rc" "$out"

# --- the fix arm rewrites every auto-fix site in place, keeps the mode, and leaves the hand fixes.
mkdir -p "$SANDBOX/fix"
cp "$CASES/bad/tree/readers.sh" "$CASES/bad/tree/over.sh" "$SANDBOX/fix/"
chmod 755 "$SANDBOX/fix/readers.sh"
out="$( cd "$SANDBOX/fix" && gate_arm_run --emit pipefail-fix . 2>&1 )"; rc=$?
expect fix-run 0 'readers.sh:5: fixed' "$rc" "$out"
expect fix-other 0 'over.sh:4: fixed' "$rc" "$out"
[[ -x "$SANDBOX/fix/readers.sh" ]] \
    || { echo "  FAIL [fix-mode]: the rewrite dropped the file's mode"; fails=$((fails + 1)); }
grep -qxF "ls | grep x | sed -n '1,1p'" "$SANDBOX/fix/readers.sh" \
    || { echo "  FAIL [fix-text]: grep -m 1 was not rewritten to a sed range"; fails=$((fails + 1)); }
out="$( cd "$SANDBOX/fix" && gate_env GATE_SDK_PIPEFAIL_PATHS=. \
    && gate_run check-pipefail-readers "$CHECKS" 2>&1 )"; rc=$?
expect fix-leaves 1 'readers.sh:11: [PIPEFAIL-EARLY-EXIT] while-read break' "$rc" "$out"
refute fix-leaves '[auto-fix]' "$out"

# --- with no path configured the arm has nothing to rewrite.
out="$( cd "$SANDBOX/empty" && gate_arm_run --emit pipefail-fix 2>&1 )"; rc=$?
expect fix-empty 0 'nothing to rewrite' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-pipefail-readers.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-pipefail-readers.test.sh: clean (configuration, operands, exit classes, the whole-file knob, the waiver floor, pruning and the fix arm — $cases cases)"
exit 0
