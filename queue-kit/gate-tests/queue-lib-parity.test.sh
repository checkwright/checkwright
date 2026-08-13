#!/usr/bin/env bash
# Cross-implementation parity for the one derivation queue-kit holds twice: the
# shell library queue-kit/lib/queue.sh and its compiled counterpart in
# native/src/queue.rs. Seven live consumers still source the library, so the
# duplication is permanent and gets criterion 6's machine-held disposition rather
# than the deletion disposition a ported primitive takes
# (gate-sdk/SPEC.md §The port-candidate criteria, criterion 6).
#
# The two sides share no representation — the shell derives ERE strings and an
# array, the crate derives a Sections value and free predicates — so what is
# compared is *classification* over one canned corpus, never the derived literals
# (queue-kit/SPEC.md §The queue format). A against B directly, with no committed
# expected file: a maintained golden would be a third copy to drift, and the
# failure this exists to catch is one side edited without the other.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$GATE_SDK_TEST_LIB_DIR/gate.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a declaration is not a dispatch: a
# consumer on an uncovered platform vendors the shell library with no artifact behind it, and
# a parity assertion there would be vacuous rather than true. The skip is declared on the
# clean line, in the shape the port's omitted-member roster uses, so a reader can tell "no
# binary here" from "parity holds". A binary that is *present* and refuses the arm is a stale
# binary rather than an absent one, so it fails here and check-gate-binary-fresh names it.
BIN="$(gate_native_bin)"
if [[ ! -x "$BIN" ]]; then
    echo "queue-lib-parity.test: ok (0 assertions; skipped — no gate binary at $BIN, so nothing dispatches to the compiled twin)"
    exit 0
fi

# The corpus lives here rather than in a directory beside this runner because
# run-gate-tests.sh globs <tests-dir>/*/ as fixture *pairs*: a corpus directory
# there is a harness error, not a fixture. It exercises every branch each side
# has — a bold lead-in in each task section, a lessons line, a bare-slug done
# bullet, and a bullet in a section that is none of these — and it is driven
# twice, with the icebox configured and unconfigured, because the icebox
# matchers degrade to the empty string rather than to "every section".
cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: parity-corpus

## New Features

- **feat-a** — a bold lead-in in the first active section.
  - **sub-a** — an indented sub-task carries the same grammar.

## Technical Debt

- **debt-a** — a bold lead-in in the second active section.

## Deferred

- **defer-a** — a bold lead-in in the deferred section.

## Chill

- **chill-a** — a bold lead-in in the section the icebox knob names.

## Done

- done-a
- **done-b** — a Done entry wearing an active entry's shape.

## Lessons Learned

- **l1** [attend] — the lessons heading is fixed spelling, no knob.

## Appendix

- **appendix-a** — a bullet in a section that is none of the above.
EOF

cat >"$SANDBOX/icebox-config.sh" <<'EOF'
QUEUE_KIT_ICEBOX_SECTION="Chill"
EOF
: >"$SANDBOX/no-icebox-config.sh"

# The shell side's answer is what a consumer gets when it applies the library's
# exported globals — the same awk the bin/ scripts run, including the explicit
# `iceboxre != ""` guard an unset icebox knob requires (queue-kit/SPEC.md
# §The icebox tier). An unguarded `$0 ~ ""` matches every line in awk, which is
# exactly the degradation-to-"every section" the guard exists to stop.
cat >"$SANDBOX/classify.awk" <<'EOF'
function add(v, w) { return v (v == "" ? "" : ",") w }
{
    v = ""
    if ($0 ~ sectre)                     v = add(v, "section")
    if ($0 ~ activere)                   v = add(v, "active")
    if ($0 ~ deferredre)                 v = add(v, "deferred")
    if (iceboxre != "" && $0 ~ iceboxre) v = add(v, "icebox")
    if ($0 ~ taskre)                     v = add(v, "task")
    if ($0 ~ lessonsre)                  v = add(v, "lessons")
    b = "-"
    if ($0 ~ /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/) {
        match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
        b = substr($0, RSTART + 2, RLENGTH - 4)
    }
    if (v == "" && b == "-") next
    printf "line\t%d\t%s\t%s\n", NR, (v == "" ? "-" : v), b
}
EOF

shell_side() {   # $1=config file  $2=queue file
    (
        export QUEUE_KIT_CONFIG_FILE="$1"
        # shellcheck source=../lib/queue.sh
        source "$DIR/lib/queue.sh"
        printf 'task-sections'
        printf '\t%s' "${QUEUE_TASK_SECTIONS[@]}"
        printf '\n'
        awk -v sectre="$QUEUE_SECTION_RE" -v activere="$QUEUE_ACTIVE_RE" \
            -v deferredre="$QUEUE_DEFERRED_RE" -v iceboxre="$QUEUE_ICEBOX_RE" \
            -v taskre="$QUEUE_TASK_RE" -v lessonsre="$QUEUE_LESSONS_RE" \
            -f "$SANDBOX/classify.awk" "$2"
        queue_live_slugs "$2" | awk '{ print "live\t" $0 }'
    )
}

# The compiled side is reached through the resolved command rather than a path,
# and every knob it reads crosses the config bridge as a resolved value exactly
# as a dispatched gate's does (gate-sdk/SPEC.md §lib/gate.sh): the values are
# taken from the shell library that computed them, so no default is duplicated
# and the seam this arm crosses is the one every gate already crosses.
native_side() {  # $1=config file  $2=queue file
    (
        export QUEUE_KIT_CONFIG_FILE="$1"
        # shellcheck source=../lib/queue.sh
        source "$DIR/lib/queue.sh"
        local IFS=$'\t'
        env "GATE_SDK_KNOB_QUEUE_KIT_ACTIVE_SECTIONS=${QUEUE_KIT_ACTIVE_SECTIONS[*]}" \
            "GATE_SDK_KNOB_QUEUE_KIT_DEFERRED_SECTION=$QUEUE_KIT_DEFERRED_SECTION" \
            "GATE_SDK_KNOB_QUEUE_KIT_ICEBOX_SECTION=$QUEUE_KIT_ICEBOX_SECTION" \
            "$BIN" --queue-parity "$2"
    )
}

compare() {  # $1=label  $2=config file
    local a b arc brc
    checks=$((checks + 1))
    a="$(shell_side "$2" "$SANDBOX/TASK-QUEUE.md")"; arc=$?
    b="$(native_side "$2" "$SANDBOX/TASK-QUEUE.md")"; brc=$?
    if [[ "$arc" -ne 0 || "$brc" -ne 0 ]]; then
        echo "  FAIL [$1]: a side could not report (shell exit $arc, binary exit $brc)"
        fails=$((fails + 1))
        return
    fi
    if [[ -z "$a" ]]; then
        echo "  FAIL [$1]: the shell side classified nothing — a vacuous agreement, not a parity hold"
        fails=$((fails + 1))
        return
    fi
    if [[ "$a" != "$b" ]]; then
        echo "  FAIL [$1]: the two implementations disagree about the same queue file:"
        diff <(printf '%s\n' "$a") <(printf '%s\n' "$b") | sed 's/^/    /'
        fails=$((fails + 1))
    fi
}

compare "icebox-configured"   "$SANDBOX/icebox-config.sh"
compare "icebox-unconfigured" "$SANDBOX/no-icebox-config.sh"

# The corpus must actually reach the branches the comparison is bought for: an
# agreement over a corpus that classifies nothing is the vacuity this unit exists
# to end, arriving one layer up. Each of these is read off the shell side, the
# one that owns the globals under obligation.
corpus="$(shell_side "$SANDBOX/icebox-config.sh" "$SANDBOX/TASK-QUEUE.md")"
T=$'\t'
have() {   # $1=label $2=grep -E pattern
    checks=$((checks + 1))
    grep -qE "$2" <<<"$corpus" || {
        echo "  FAIL [$1]: the corpus no longer exercises this branch"
        fails=$((fails + 1))
    }
}
have "corpus-active"   "^line${T}[0-9]+${T}section,active,task${T}-$"
have "corpus-deferred" "^line${T}[0-9]+${T}section,deferred,task${T}-$"
have "corpus-icebox"   "^line${T}[0-9]+${T}section,icebox,task${T}-$"
have "corpus-lessons"  "^line${T}[0-9]+${T}section,lessons${T}-$"
have "corpus-plain"    "^line${T}[0-9]+${T}section${T}-$"
have "corpus-bullet"   "^line${T}[0-9]+${T}-${T}feat-a$"
have "corpus-done"     "^line${T}[0-9]+${T}-${T}done-b$"
have "corpus-live"     "^live${T}chill-a$"

# The discriminating case for the icebox degradation: unconfigured, the icebox
# section must classify as a plain section on both sides — never as every line.
corpus_off="$(shell_side "$SANDBOX/no-icebox-config.sh" "$SANDBOX/TASK-QUEUE.md")"
lack() {   # $1=label $2=grep -E pattern $3=what the hit would mean
    checks=$((checks + 1))
    if grep -qE "$2" <<<"$corpus_off"; then
        echo "  FAIL [$1]: $3"
        fails=$((fails + 1))
    fi
}
lack "icebox-degrades-class" "^line${T}[0-9]+${T}[a-z,]*icebox" \
     "an unconfigured icebox still classified a line as icebox"
lack "icebox-degrades-task"  "^live${T}chill-a$" \
     "an unconfigured icebox left its section a live task section"

if [[ "$fails" -gt 0 ]]; then
    echo "queue-lib-parity.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "queue-lib-parity.test: ok ($checks assertions; one helper and seven globals held to the compiled twin over one corpus, icebox configured and not)"
exit 0
