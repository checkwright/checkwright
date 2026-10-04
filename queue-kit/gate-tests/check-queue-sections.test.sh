#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §Layout and configuration — a section knob naming a heading that ends in `:` is refused at exit 2, one case per section knob, which the good/bad pair cannot hold: a refusal is exit 2, and a `bad/` case exits 1.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CHECKS="$ROOT/queue-kit/checks"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
mkdir -p "$SANDBOX/scripts"

fails=0

# The heading each case's knob names is present with a value after the colon, so a prefix match
# of the derived name would read the floor as met.
cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: seeded

## New Features

## Now: this week

## Technical Debt

## Deferred

## Backlog: later

## Icebox: cold

## Done

## Done: archive

## Lessons Learned
EOF

# $1 = the case name, $2 = the knob file body, $3 = the knob the refusal names
refused() {
    printf '%s\n' "$2" >"$SANDBOX/scripts/queue-config.knobs"
    local out rc
    out="$( cd "$SANDBOX" && gate_env "QUEUE_KIT_KNOB_FILE=$SANDBOX/scripts/queue-config.knobs" \
        && gate_run check-queue-sections "$CHECKS" 2>&1 )"; rc=$?
    if [[ "$rc" -ne 2 ]] || [[ "$out" != *"$3 names '"*"', ending in ':'"* ]]; then
        echo "  FAIL [$1]: a section knob ending in ':' must refuse at exit 2 naming $3 (rc=$rc): $out"
        fails=$((fails + 1))
    fi
}

refused active "QUEUE_KIT_ACTIVE_SECTIONS[] = New Features
QUEUE_KIT_ACTIVE_SECTIONS[] = Now:" QUEUE_KIT_ACTIVE_SECTIONS
refused deferred "QUEUE_KIT_DEFERRED_SECTION = Backlog:" QUEUE_KIT_DEFERRED_SECTION
refused icebox "QUEUE_KIT_ICEBOX_SECTION = Icebox:" QUEUE_KIT_ICEBOX_SECTION
refused done "QUEUE_KIT_DONE_SECTION = Done:" QUEUE_KIT_DONE_SECTION

if [[ "$fails" -gt 0 ]]; then
    echo "check-queue-sections.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-queue-sections.test: ok (an active, deferred, icebox and done section knob ending in ':' each refuse at exit 2)"
exit 0
