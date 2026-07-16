#!/usr/bin/env bash
# Behavioral test of bin/queue-index.sh's attention block — the [attend] Lessons
# lead lines it appends (both default and --collapse-deferred output), capped at
# QUEUE_KIT_ATTEND_CAP with an overflow note. queue-index is a tool, not a gate,
# so it has no good/bad pair; this drives it directly. Config is isolated via
# QUEUE_KIT_CONFIG_FILE=/dev/null so the repo's queue-config.sh does not leak in.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
IDX="$DIR/bin/queue-index.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
: >"$SANDBOX/empty-config.sh"
export QUEUE_KIT_CONFIG_FILE="$SANDBOX/empty-config.sh"

fails=0

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo  [stage: build]

## New Features

- **feat-a** — do a thing.

## Technical Debt

## Deferred

## Done

## Lessons Learned

- **l1** [attend] — first attention point
- **l2** [attend] — second attention point
- **l3** [attend] — third attention point
- **l4** — not an attention point
EOF

want() {   # $1=label $2=output $3=substring-that-must-be-present
    grep -qF -- "$3" <<<"$2" || { echo "  FAIL [$1]: output lacks '$3':"; printf '    %s\n' "$2"; fails=$((fails + 1)); }
}
absent() { # $1=label $2=output $3=substring-that-must-be-absent
    grep -qF -- "$3" <<<"$2" && { echo "  FAIL [$1]: output should not contain '$3':"; printf '    %s\n' "$2"; fails=$((fails + 1)); } || true
}

# cap=2: two lead lines shown, third folded into the overflow note.
out="$(QUEUE_KIT_ATTEND_CAP=2 bash "$IDX" "$SANDBOX/TASK-QUEUE.md")"
want "cap2-header"   "$out" "Attention (Lessons [attend], this iteration):"
want "cap2-l1"       "$out" "first attention point"
want "cap2-l2"       "$out" "second attention point"
absent "cap2-l3"     "$out" "third attention point"
want "cap2-overflow" "$out" "(+1 more [attend])"
absent "cap2-l4"     "$out" "not an attention point"

# default cap (3): all three shown, no overflow note.
out="$(bash "$IDX" "$SANDBOX/TASK-QUEUE.md")"
want "cap3-l3"        "$out" "third attention point"
absent "cap3-overflow" "$out" "more [attend])"

# --collapse-deferred still appends the block.
out="$(QUEUE_KIT_ATTEND_CAP=2 bash "$IDX" --collapse-deferred "$SANDBOX/TASK-QUEUE.md")"
want "collapse-block" "$out" "Attention (Lessons [attend], this iteration):"
want "collapse-l1"    "$out" "first attention point"

# [drain-exempt: <reason>] re-echoed on the active line (tags otherwise stripped).
cat >"$SANDBOX/drain.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo  [stage: validate]

## New Features

- **feat-b** [drain-exempt: validate-half pending] — spanning feature.

## Technical Debt

## Deferred

## Done

## Lessons Learned
EOF
out="$(bash "$IDX" "$SANDBOX/drain.md")"
want "drain-echo"  "$out" "[drain-exempt: validate-half pending]"
want "drain-ready" "$out" "• feat-b"

# no [attend] entries -> no attention block.
cat >"$SANDBOX/none.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo  [stage: build]

## New Features

## Technical Debt

## Deferred

## Done

## Lessons Learned

- **only** — an untagged lesson
EOF
out="$(bash "$IDX" "$SANDBOX/none.md")"
absent "none-block" "$out" "Attention (Lessons"

if [[ "$fails" -gt 0 ]]; then
    echo "queue-index.test.sh: $fails case(s) failed"
    exit 1
fi
echo "queue-index.test.sh: clean (attend block: cap+overflow, default cap, --collapse-deferred, empty; drain-exempt echo; 14 checks)"
exit 0
