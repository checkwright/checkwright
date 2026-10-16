#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The iteration boundary — the set record end-to-end through a sandboxed enter-stage: the boundary entry drops it and keeps the preamble above it, its report names the drop, and --rename leaves it in place
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the second sanctioned caller: this harness
# drives the arm from a non-git sandbox cwd, which bin/run-gates.sh refuses by design (it cds to
# the git toplevel and a `mktemp -d` is no repository), so it resolves the binary through
# gate_arm_run rather than through that front-end.
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir  $2=last stamp's stage
    local sb="$1"
    mkdir -p "$sb/.workflow" "$sb/scratch"
    cat >"$sb/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

  A consumer preamble that stays.

  **Set record**

  **Push need (2026-06-01, inside the budget):** the closing push spends the one run.

  **Direction (2026-06-01, operator direction):** the set lands as one unit.

---

## New Features

## Technical Debt

## Done
EOF
    cat >"$sb/.workflow/WORKFLOW-STATE.txt" <<EOF
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration $2 dddddddd 2026-06-04 none
EOF
}

run_enter() {  # $1=sandbox subdir, rest = argv after the tool name
    local sb="$1"; shift
    ( cd "$sb" && gate_env GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
        && gate_arm_run --enter-stage "$@" 2>&1 )
}

# --- the boundary entry drops the set record and keeps the preamble ---
bnd="$SANDBOX/boundary"
seed "$bnd" close
out="$(run_enter "$bnd" scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note boundary-entry "want exit 0, got $rc -- $out"
grep -qF 'Set record' "$bnd/TASK-QUEUE.md"  && note drop-opening "the set record's opening line survived the boundary"
grep -qF 'Push need' "$bnd/TASK-QUEUE.md"   && note drop-record "a set-record paragraph survived the boundary"
grep -qF 'A consumer preamble that stays.' "$bnd/TASK-QUEUE.md" || note keep-preamble "the preamble above the set record was dropped"
grep -qxF '## Iteration: —' "$bnd/TASK-QUEUE.md" || note header "the header was not restarted at the unnamed form"
want=$'## Iteration: —\n\n  A consumer preamble that stays.\n\n---'
[[ "$(cat "$bnd/TASK-QUEUE.md")" == *"$want"* ]] || note shape "the header region is not the preamble then the rule: $(cat "$bnd/TASK-QUEUE.md")"
grep -qF 'dropped the closing iteration' <<<"$out" || note report "the boundary report names no drop: $out"

# --- --rename leaves the set record in place ---
ren="$SANDBOX/rename"
seed "$ren" build
cp "$ren/TASK-QUEUE.md" "$ren/queue.before"
out="$(run_enter "$ren" --rename new-name)"; rc=$?
[[ "$rc" -eq 0 ]] || note rename "want exit 0, got $rc -- $out"
diff <(sed 's/^## Iteration: .*$/## Iteration:/' "$ren/queue.before") \
     <(sed 's/^## Iteration: .*$/## Iteration:/' "$ren/TASK-QUEUE.md") >/dev/null \
    || note rename-keep "the rename changed the queue beyond its header line"
grep -qxF '## Iteration: new-name' "$ren/TASK-QUEUE.md" || note rename-header "the rename did not rewrite the header"

[[ "$fails" -eq 0 ]] || { echo "boundary-set-record.test: $fails assertion(s) failed"; exit 1; }
echo "boundary-set-record.test: clean (the boundary drops the set record, keeps the preamble above it and names the drop; --rename leaves it in place)"
exit 0
