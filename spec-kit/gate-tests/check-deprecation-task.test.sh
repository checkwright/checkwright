#!/usr/bin/env bash
# Behavioral test of the config-driven paths the one-pair good/bad harness
# cannot hold with stock defaults. The deprecation-marker roster ships empty,
# so the default path is a clean skip — a fixture can never trip it, and only
# a config that names a marker vocabulary exercises resolution at all. The
# good/bad pair covers bound-live (clean) and a bad file carrying all three
# failure classes at once (stale done slug, absent slug, unbound marker); this
# unit pins the paths that pair cannot: the empty-roster clean skip, that each
# failure class is reported under its own heading with the right slug, and that
# an unreadable queue is fail-closed.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # spec-kit/
GATE="$DIR/checks/check-deprecation-task.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md — sandbox

## Iteration: fixture  [stage: build]

---

## New Features

- **live-teardown** — a live decommission task.

## Technical Debt

## Deferred

## Done

- done-teardown

## Lessons Learned
EOF

cat >"$SANDBOX/src.sh" <<'EOF'
#!/usr/bin/env bash
# @deprecated task: live-teardown bound to a live task — resolves
# @deprecated task: done-teardown bound to a finished task — stale
# @deprecated task: ghost-teardown bound to a slug the queue never had
# @deprecated carrying no decommission binding at all
echo hi
EOF

cat >"$SANDBOX/cfg.sh" <<'EOF'
SPEC_KIT_DEPRECATION_MARKERS=('@deprecated')
SPEC_KIT_COMMENT_SURFACE=("*.sh")
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4..=env assignments
    local label="$1" want="$2" sub="$3"; shift 3
    local out rc
    out="$(cd "$SANDBOX" && env "$@" "$GATE" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# Empty roster (stock default): the gate clean-skips — the marker vocabulary is
# consumer config, so a repo that sets none is never reddened by this gate.
check_case "empty-roster-skip" 0 "no SPEC_KIT_DEPRECATION_MARKERS configured"

# Configured roster: the live-bound marker resolves while all three failure
# classes trip, each named with its offending slug.
check_case "stale-done-slug"  1 "task: done-teardown is done"       SPEC_KIT_CONFIG_FILE="$SANDBOX/cfg.sh"
check_case "absent-slug"      1 "no live task 'ghost-teardown'"     SPEC_KIT_CONFIG_FILE="$SANDBOX/cfg.sh"
check_case "unbound-marker"   1 "no 'task: <slug>' binding"         SPEC_KIT_CONFIG_FILE="$SANDBOX/cfg.sh"

# The live-bound marker never appears in the findings — a resolved slug is clean.
out="$(cd "$SANDBOX" && env SPEC_KIT_CONFIG_FILE="$SANDBOX/cfg.sh" "$GATE" 2>&1)"
if grep -qF -- "live-teardown" <<<"$out"; then
    echo "  FAIL [live-bound-clean]: a resolved marker was flagged:"; printf '    %s\n' "$out"
    fails=$((fails + 1))
fi

# An unreadable queue is a fail-closed harness error (exit 2), never a silent pass.
check_case "queue-missing-fail-closed" 2 "queue file not found" \
    SPEC_KIT_CONFIG_FILE="$SANDBOX/cfg.sh" SPEC_KIT_QUEUE_FILE="$SANDBOX/nope.md"

if [[ "$fails" -gt 0 ]]; then
    echo "check-deprecation-task.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-deprecation-task.test.sh: clean (empty-roster skip + stale/absent/unbound classes + live-bound clean + missing-queue fail-closed)"
exit 0
