#!/usr/bin/env bash
# Cross-implementation parity for the three primitives guard-kit holds twice after the
# scan-prompts cut: `guard_split_compound`, `guard_skeleton` and `_guard_redirect_pairs` in
# guard-kit/lib/guard.sh, and their compiled counterparts in native/src/guard.rs. The library
# is permanently shell on two grounds that reach neither the knob-resolution nor the
# consumer-surface question for these three, and its shell caller set cannot empty — the rules
# that call them are functions in the same file — so the duplication is permanent and this is
# criterion 6's *unless* clause rather than its deletion clause (gate-sdk/SPEC.md §The
# port-candidate criteria, criterion 6).
#
# What is compared is *classification* over one canned corpus, A against B directly with no
# committed expected file: a maintained golden would be a third copy to drift, and the failure
# this exists to catch is one side edited without the other.
#
# The corpus is scoped to the shapes a **friction-log line** can carry, and that scope is a
# property of the input rather than a convenience: `guard_log_fallthrough` flattens every `\n`
# and `\t` to a space before the append, so a logged line is newline-free by construction and
# the compiled `skeleton` implements exactly the reachable subset. The newline case is asserted
# separately below, as an out-of-contract refusal rather than as a compared classification.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$GATE_SDK_TEST_LIB_DIR/gate.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # guard-kit/

fails=0
checks=0

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a declaration is not a dispatch: a consumer on
# an uncovered platform vendors the shell library with no artifact behind it, and a parity
# assertion there would be vacuous rather than true. A binary that is present and refuses the arm
# is a stale binary, so it fails here.
BIN="$(gate_native_bin)"
if [[ ! -x "$BIN" ]]; then
    echo "guard-lib-parity.test: ok (0 assertions; skipped — no gate binary at $BIN, so nothing dispatches to the compiled twins)"
    exit 0
fi
[[ "$BIN" == /* ]] || BIN="$(cd "$(dirname "$BIN")" && pwd)/$(basename "$BIN")"

# Every shape a friction-log line can carry that any of the three primitives decides on:
# single- and double-quoted spans (including a backslash escape inside a double-quoted one and
# both unterminated forms), a bare backslash escape and a trailing one, `<<<` here-strings,
# `<<`/`<<-` heredoc openers with bare, single-quoted and double-quoted delimiters and with an
# opener that is not one at all, every statement separator in and out of quotes and in trailing
# position, write redirects with and without a descriptor in both operators, read redirects, and
# fd-dups including the closing form.
CORPUS=(
    "echo hello"
    ""
    "echo 'a;b' && ls"
    'echo "a && b" | wc -l'
    'grep -oE "a\"b" file'
    "echo 'x || y ; z' ; true"
    'printf %s\ x'
    'echo trailing backslash \'
    'cat <<<"here string"'
    "cat <<<'here string'"
    'cat <<EOF'
    'cat <<-EOF'
    "cat <<'EOF'"
    'cat <<"EOF"'
    'x <<  SPACED'
    'x << "Q S"'
    "x <<- 'Q'"
    'x <<9BAD'
    'x <<'
    'a;b&&c||d|e'
    'echo trailing;'
    'echo "unterminated'
    "echo 'unterminated"
    'sort -rn > out.txt'
    'sort -rn >> out.txt'
    'cmd 2> err.log'
    'cmd 1>>log 2>&-'
    'cmd 2>&1'
    'cmd >&2'
    'wc -l < in.txt'
    'jq . < a.json > b.json'
    'git commit -m "x; y" && git push'
)

# The inert-class lists the holder takes. `sq,dq,hd` is `scan-prompts`' own call; it is here
# rather than alone because `hd` is read only inside the branch a newline-free command never
# reaches, so agreeing with `sq,dq` over the same corpus is what shows the class inert.
WANTS=(- sq dq sq,dq sq,dq,hd)

shell_side() {
    (
        # shellcheck source=../lib/guard.sh
        source "$DIR/lib/guard.sh"
        local c w seg p i
        local -a wa
        for c in "${CORPUS[@]}"; do
            i=0
            while IFS= read -r seg; do
                printf 'split\t%s\t%s\t%s\n' "$c" "$i" "$seg"
                i=$((i + 1))
            done < <(guard_split_compound "$c")
        done
        for w in "${WANTS[@]}"; do
            wa=()
            [[ "$w" != - ]] && IFS=, read -r -a wa <<<"$w"
            for c in "${CORPUS[@]}"; do
                printf 'skeleton\t%s\t%s\t%s\n' "$w" "$c" "$(guard_skeleton "$c" ${wa[@]+"${wa[@]}"})"
            done
        done
        for c in "${CORPUS[@]}"; do
            i=0
            while IFS= read -r p; do
                printf 'redirect\t%s\t%s\t%s\n' "$c" "$i" "$p"
                i=$((i + 1))
            done < <(_guard_redirect_pairs "$c")
        done
    )
}

# The compiled side is reached through the same binary a dispatched gate reaches, and the arm
# reports classification rather than an internal representation — `--queue-parity`'s own rule.
native_side() {
    local w
    "$BIN" --guard-lib-parity split "${CORPUS[@]}" || return $?
    for w in "${WANTS[@]}"; do
        "$BIN" --guard-lib-parity skeleton "$w" "${CORPUS[@]}" || return $?
    done
    "$BIN" --guard-lib-parity redirect "${CORPUS[@]}"
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
# Each is read off the shell side, the one that owns the predicate under obligation.
T=$'\t'
have() {   # $1=label $2=grep -E pattern
    checks=$((checks + 1))
    grep -qE "$2" <<<"$a" || {
        echo "  FAIL [$1]: the corpus no longer exercises this branch"
        fails=$((fails + 1))
    }
}
have "split-longest-separator" "^split${T}a;b&&c\|\|d\|e${T}4${T}e\$"
have "split-empty-tail"        "^split${T}echo trailing;${T}1${T}\$"
have "split-inside-quotes"     "^split${T}echo 'a;b' && ls${T}1${T}b' \$"
have "skeleton-sq"             "^skeleton${T}sq,dq${T}echo 'a;b' && ls${T}echo SQ && ls\$"
have "skeleton-dq"             "^skeleton${T}sq,dq${T}echo \"a && b\" \| wc -l${T}echo DQ \| wc -l\$"
have "skeleton-no-class"       "^skeleton${T}-${T}echo 'a;b' && ls${T}echo 'a;b' && ls\$"
have "skeleton-dq-escape"      "^skeleton${T}sq,dq${T}grep -oE \"a.\"b\" file${T}grep -oE DQ file\$"
have "skeleton-herestring"     "^skeleton${T}sq,dq${T}cat <<<\"here string\"${T}cat <<<DQ\$"
have "skeleton-heredoc-opener" "^skeleton${T}sq,dq,hd${T}cat <<'EOF'${T}cat <<'EOF'\$"
have "skeleton-unterminated"   "^skeleton${T}sq,dq${T}echo 'unterminated${T}echo 'unterminated\$"
have "redirect-descriptorless" "^redirect${T}sort -rn >> out.txt${T}0${T}>> out.txt\$"
have "redirect-descriptor"     "^redirect${T}cmd 1>>log 2>&-${T}1${T}2>&-\$"
have "redirect-fd-dup"         "^redirect${T}cmd 2>&1${T}0${T}2>&1\$"

# `hd` is inert on a newline-free command, and this is what says so rather than the comment
# above: the same corpus under `sq,dq` and under `sq,dq,hd` classifies identically, so the
# branch the compiled twin omits is one no friction-log line can reach.
checks=$((checks + 1))
if ! diff -q \
    <(grep -F "${T}sq,dq${T}" <<<"$a" | cut -f3-) \
    <(grep -F "${T}sq,dq,hd${T}" <<<"$a" | cut -f3-) >/dev/null; then
    echo "  FAIL [hd-inert]: adding 'hd' changed the shell holder's classification of a newline-free"
    echo "         corpus, so the branch the compiled twin omits is reachable after all and the"
    echo "         omission is a defect rather than the reachable subset"
    fails=$((fails + 1))
fi

# The precondition delta (5) rests on, carried in the code and checked here: a newline-bearing
# command is out of the twin's contract, refused at exit 2 rather than normalized by a branch this
# holder does not carry. The shell holder answers it; the two are not compared on it, and that
# asymmetry is the point rather than a gap.
checks=$((checks + 1))
nl_out="$("$BIN" --guard-lib-parity skeleton sq,dq "$(printf 'cat <<EOF\nbody\nEOF')" 2>&1)"
nl_rc=$?
if [[ "$nl_rc" -ne 2 ]]; then
    echo "  FAIL [newline-out-of-contract]: the compiled twin took a newline-bearing command (exit $nl_rc)"
    echo "         it omits the heredoc-body machinery, so normalizing one silently is the landmine"
    echo "         the refusal exists to prevent: $nl_out"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "guard-lib-parity.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "guard-lib-parity.test: ok ($checks assertions; guard_split_compound, guard_skeleton and _guard_redirect_pairs held to their compiled twins over ${#CORPUS[@]} log-line shapes and ${#WANTS[@]} inert-class lists)"
exit 0
