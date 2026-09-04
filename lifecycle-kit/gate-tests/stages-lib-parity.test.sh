#!/usr/bin/env bash
# Cross-implementation parity for the six derivations lifecycle-kit holds twice after the
# enter-stage cut: `lifecycle_header`/`lifecycle_header_iter`, `lifecycle_current_stage`,
# `lifecycle_stage_known`, `lifecycle_stage_journal`, `lifecycle_stage_journal_written` and the
# opening-line mark in lifecycle-kit/lib/stages.sh, against their compiled counterparts in
# native/src/stages.rs and native/src/emit/enter_stage.rs.
#
# The cut is what makes this owed rather than merely absent. `lib/stages.sh` stays shell by declared
# cause — it is the config bridge's sole resolver for the LIFECYCLE_KIT_* family — while the port
# moved every in-tree *caller* of these six readers onto the crate's side of a seam with no
# comparator across it. The opening-line shape is the sharpest case: §bin/enter-stage.sh rules that
# writer and reader "share one spelling of that line's shape in lib/stages.sh so writer and reader
# cannot drift", and after this cut the writer is compiled and the reader is not.
#
# What is compared is *classification* over one canned corpus, never a derived literal: A against B
# directly, with no committed expected file — a maintained golden would be a third copy to drift,
# and the failure this exists to catch is one side edited without the other.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$GATE_SDK_TEST_LIB_DIR/gate.sh"
# shellcheck source=../lib/stages.sh
source "$(dirname "${BASH_SOURCE[0]}")/../lib/stages.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a declaration is not a dispatch: a consumer on
# an uncovered platform vendors the shell library with no artifact behind it, and a parity assertion
# there would be vacuous rather than true. A binary that is present and refuses the arm is a stale
# binary, so it fails here.
BIN="$(gate_native_bin)"
if [[ ! -x "$BIN" ]]; then
    echo "stages-lib-parity.test: ok (0 assertions; skipped — no gate binary at $BIN, so nothing dispatches to the compiled twin)"
    exit 0
fi

# The corpus reaches every branch each reader has: a plain header, a residual pre-upgrade [stage:]
# field, the unnamed placeholder, a header with padding, a state file with a cursor, one with no
# data line, one with no separator at all, and a one-field data line.
mkdir -p "$SANDBOX/q" "$SANDBOX/s" "$SANDBOX/j"
printf '# q\n\n## Iteration: plain-name\n\n## Done\n'                 >"$SANDBOX/q/plain"
printf '# q\n\n## Iteration:   padded-name  [stage: build]\n'         >"$SANDBOX/q/residual"
printf '# q\n\n## Iteration: —\n'                                     >"$SANDBOX/q/unnamed"
printf '# q\n\nno header at all\n'                                    >"$SANDBOX/q/headerless"
printf '# c\n\n---\n\nit scope aaaa 2026-06-01 none\nit build bbbb 2026-06-02 none\n' >"$SANDBOX/s/cursor"
printf '# c\n\n---\n\n'                                               >"$SANDBOX/s/nodata"
printf '# c\nit scope aaaa 2026-06-01 none\n'                         >"$SANDBOX/s/noseparator"
printf '# c\n\n---\n\nonefield\n'                                     >"$SANDBOX/s/onefield"
: >"$SANDBOX/j/empty"
printf '\n   \n'                                                      >"$SANDBOX/j/blanks"
printf '# stage-journal build — it aaaa 2026-06-01 none\n'            >"$SANDBOX/j/skeleton"
printf '# stage-journal build — it aaaa 2026-06-01 none\n\nthe session wrote this\n' >"$SANDBOX/j/written"
printf 'a session wrote this and no opener ever ran\n'                >"$SANDBOX/j/bare"

# The shell side is driven into the crate's printed shape rather than the crate into the shell's,
# because the crate's is the one a table can be read off; the values compared are the functions'.
shell_side() {
    local sub="$1"; shift
    local x
    case "$sub" in
        iter)    for x in "$@"; do printf 'iter\t%s\t%s\n'    "$x" "$(lifecycle_header_iter "$(lifecycle_header "$x")")"; done ;;
        cursor)  for x in "$@"; do printf 'cursor\t%s\t%s\n'  "$x" "$(lifecycle_current_stage "$x")"; done ;;
        known)   for x in "$@"; do if lifecycle_stage_known "$x"; then printf 'known\t%s\ttrue\n' "$x"; else printf 'known\t%s\tfalse\n' "$x"; fi; done ;;
        journal) for x in "$@"; do printf 'journal\t%s\t%s\n' "$x" "$(lifecycle_stage_journal "$x")"; done ;;
        written) for x in "$@"; do if lifecycle_stage_journal_written "$x"; then printf 'written\t%s\ttrue\n' "$x"; else printf 'written\t%s\tfalse\n' "$x"; fi; done ;;
        mark)    printf 'mark\t%s\n' "$LIFECYCLE_STAGE_JOURNAL_MARK" ;;
    esac
}

# --stages-lib-parity is a top-level flag like its three sibling parity arms rather than a bridged
# one, so the two knobs its subcommands read are resolved out of the sourced library and exported
# directly — which is also what keeps this harness comparing the library against the crate rather
# than the config bridge against itself.
export GATE_SDK_KNOB_LIFECYCLE_KIT_STAGES GATE_SDK_KNOB_LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN
GATE_SDK_KNOB_LIFECYCLE_KIT_STAGES="$(printf '%s\t' "${LIFECYCLE_KIT_STAGES[@]}")"
GATE_SDK_KNOB_LIFECYCLE_KIT_STAGES="${GATE_SDK_KNOB_LIFECYCLE_KIT_STAGES%$'\t'}"
GATE_SDK_KNOB_LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN="$LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN"

compare() {  # $1=label  $2=subcommand  $3..=corpus
    local label="$1"; shift
    local a b
    checks=$((checks + 1))
    a="$(shell_side "$@" 2>&1)"
    b="$("$BIN" --stages-lib-parity "$@" 2>&1)"
    [[ "$a" == "$b" ]] || note "$label" "the shell library and the compiled twin disagree:
    shell: $a
    crate: $b"
}

compare header-iter iter "$SANDBOX/q/plain" "$SANDBOX/q/residual" "$SANDBOX/q/unnamed" "$SANDBOX/q/headerless"
compare cursor      cursor "$SANDBOX/s/cursor" "$SANDBOX/s/nodata" "$SANDBOX/s/noseparator" "$SANDBOX/s/onefield"
compare membership  known "${LIFECYCLE_KIT_STAGES[0]}" "${LIFECYCLE_KIT_STAGES[-1]}" notastage "" scope-ish
compare journal-path journal "${LIFECYCLE_KIT_STAGES[@]}"
compare written     written "$SANDBOX/j/empty" "$SANDBOX/j/blanks" "$SANDBOX/j/skeleton" "$SANDBOX/j/written" "$SANDBOX/j/bare"

# The opening-line shape, the case delta (14) names sharpest: the writer is compiled and the reader
# is shell, so the one spelling they share is asserted directly rather than only through the
# predicate that consumes it.
compare opening-mark mark

# And the two halves closed over each other: a journal the compiled writer opens must read as
# *unwritten* to the shell predicate, and as written once a session appends — which is the whole
# discrimination the entry assertion rests on, across the new seam.
OPENED="$SANDBOX/j/opened.md"
checks=$((checks + 1))
"$BIN" --stages-lib-parity open "$OPENED" \
    || note opener-run "the compiled opener refused to open a journal"
[[ -s "$OPENED" ]] || note opener-empty "the compiled opener wrote no skeleton"
lifecycle_stage_journal_written "$OPENED" \
    && note opener-vs-reader "the shell predicate read the compiled writer's own skeleton as written"
printf 'the session wrote this\n' >>"$OPENED"
lifecycle_stage_journal_written "$OPENED" \
    || note opener-vs-reader-append "the shell predicate did not see a session's append"

[[ "$fails" -eq 0 ]] || { echo "stages-lib-parity.test: $fails of $checks comparison(s) failed"; exit 1; }
echo "stages-lib-parity.test: clean ($checks comparisons; the header/iteration, cursor, membership, journal-path, journal-written and opening-mark derivations agree across the shell library and the compiled twin, and the compiled opener's own bytes read as unwritten to the shell predicate)"
exit 0
