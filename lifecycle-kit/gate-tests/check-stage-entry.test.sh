#!/usr/bin/env bash
# Behavioral test of checks/check-stage-entry.sh assertions B and C — the
# scenarios the one-pair good/bad harness cannot hold. The good/bad fixture
# pair (run-gate-tests.sh) covers assertion A (prerequisite-stamp ordering: a
# [stage: close] header with no validate stamp); the harness admits only one
# bad/ dir, so assertion B drives a [stage: validate] header with a non-empty
# ## New Features (exit 1), and assertion C drives four cross-component
# build-entry scenarios (2-dir amendments ±waiver, single-amendment cross-
# component body, single-component amendment).
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-stage-entry.sh"
SANDBOX="$(mktemp -d)"
mkdir -p "$SANDBOX/.workflow"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration  [stage: validate]

---

## New Features

- **unfinished-feature** `[spec: SPEC-demo.md]` — still in the build queue

## Technical Debt

## Done
EOF

cat >"$SANDBOX/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01
demo-iteration build bbbbbbbb 2026-06-02
EOF

out="$(cd "$SANDBOX" && "$GATE" 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [validate-non-empty-queue]: want exit 1, got $rc -- $out"
    fails=$((fails + 1))
elif ! grep -qF 'active queue is non-empty' <<<"$out"; then
    echo "  FAIL [validate-non-empty-queue]: exit 1 OK but output lacks 'active queue is non-empty':"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi

# --- assertion C: a cross-component build entry demands an align (or waiver) stamp ---

check_case() {  # $1=label  $2=sandbox-dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && "$GATE" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

build_queue() {  # writes a [stage: build] TASK-QUEUE.md into $1
    mkdir -p "$1"
    cat >"$1/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration  [stage: build]

---

## New Features

## Technical Debt

## Done
EOF
}

# C1 (bad): amendments in two component dirs, no align stamp, no waiver — signal 1.
c1="$SANDBOX/c1"
build_queue "$c1"
mkdir -p "$c1/.workflow" "$c1/widget-service" "$c1/panel-facade"
: >"$c1/widget-service/SPEC.md"; : >"$c1/widget-service/SPEC-foo.md"
: >"$c1/panel-facade/SPEC.md"; : >"$c1/panel-facade/SPEC-bar.md"
cat >"$c1/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01
demo-iteration build bbbbbbbb 2026-06-02
EOF
check_case "C1 two-dir-no-align" "$c1" 1 "cross-component amendment signal"

# C2 (good): same two-dir amendments, with an explicit align-waived waiver.
c2="$SANDBOX/c2"
cp -r "$c1" "$c2"
cat >"$c2/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01
demo-iteration align-waived cccccccc 2026-06-03
demo-iteration build bbbbbbbb 2026-06-02
EOF
check_case "C2 two-dir-waiver" "$c2" 0 "STAGE-ENTRY: clean"

# C3 (bad): a single amendment whose body names a second component — signal 2.
c3="$SANDBOX/c3"
build_queue "$c3"
mkdir -p "$c3/.workflow" "$c3/widget-service" "$c3/panel-facade"
: >"$c3/widget-service/SPEC.md"; : >"$c3/panel-facade/SPEC.md"
printf 'delta folds the wire change into panel-facade/SPEC.md\n' >"$c3/widget-service/SPEC-foo.md"
cat >"$c3/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01
demo-iteration build bbbbbbbb 2026-06-02
EOF
check_case "C3 single-amendment-cross-component" "$c3" 1 "cross-component amendment signal"

# C4 (good): a single-component amendment (own dir only) — no signal.
c4="$SANDBOX/c4"
build_queue "$c4"
mkdir -p "$c4/.workflow" "$c4/widget-service"
: >"$c4/widget-service/SPEC.md"
printf 'delta stays within widget-service/SPEC.md\n' >"$c4/widget-service/SPEC-foo.md"
cat >"$c4/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01
demo-iteration build bbbbbbbb 2026-06-02
EOF
check_case "C4 single-component" "$c4" 0 "STAGE-ENTRY: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-stage-entry.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-stage-entry.test.sh: clean (assertion B queue-empty + assertion C cross-component align/waiver, 4 cases)"
exit 0
