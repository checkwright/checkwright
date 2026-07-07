#!/usr/bin/env bash
# lifecycle-kit consumer-smoke install — the executable form of README.md §Install
# steps 1-2, plus the first /scope stamp the README notes is required for green
# (the bare header leaves the battery red at check-stage-evidence by design).
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored lifecycle-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

# Register the stage gates (gate-sdk seeded scripts/gates.list).
cat >> scripts/gates.list <<'EOF'
# lifecycle-kit
check-stage-evidence
check-stage-entry
EOF

# The queue file carries the iteration header; create the minimal skeleton if no
# earlier kit did (same content queue-kit's install would write).
if [[ ! -f TASK-QUEUE.md ]]; then
    cat > TASK-QUEUE.md <<'EOF'
# TASK-QUEUE.md — smoke consumer work queue

## Iteration: —  [stage: scope]

---

## New Features

## Technical Debt

## Deferred

## Done
EOF
fi

# The evidence file: skeleton plus the bootstrap /scope stamp (an unnamed
# iteration at its first stage — the stamp check-stage-evidence requires).
mkdir -p .workflow
cat > .workflow/WORKFLOW-STATE.txt <<EOF
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

— scope smoke001 $(date +%F)
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html

# --- exercise bin/enter-stage.sh end-to-end (advisory tool; SPEC §Testing) ---
# All under .tmp (gitignored) with its own queue/state/sessions, so the tool is
# driven for real without perturbing the committed baseline the zero-config
# battery then checks. A fake transcript gives session-id.sh a stable id.
es="$PWD/.tmp/enter-stage-smoke"
rm -rf "$es"; mkdir -p "$es/.workflow" "$es/sessions" "$es/tmp"
: > "$es/sessions/aabbccdd-0000-0000-0000-000000000000.jsonl"
cat > "$es/TASK-QUEUE.md" <<'EOF'
# smoke queue
## Iteration: prev-iter  [stage: close]

---

## New Features
## Technical Debt
## Deferred
## Done
EOF
cat > "$es/.workflow/WORKFLOW-STATE.txt" <<'EOF'
# contract

---

prev-iter close ffffffff 2020-01-01
EOF

es_run() {
    LIFECYCLE_QUEUE_FILE="$es/TASK-QUEUE.md" \
    LIFECYCLE_STATE_FILE="$es/.workflow/WORKFLOW-STATE.txt" \
    LIFECYCLE_SESSIONS_DIR="$es/sessions" \
    GATE_SDK_TMP_DIR="$es/tmp" \
    bash "$SMOKE_KIT_ROOT/bin/enter-stage.sh" "$@"
}
esq="$es/TASK-QUEUE.md"; ess="$es/.workflow/WORKFLOW-STATE.txt"; d="$(date +%F)"

# 1. first stage (scope): reset + stamp, header → unnamed, prior stamp dropped.
es_run scope >/dev/null
grep -q "^## Iteration: —  \[stage: scope\]\$" "$esq" || { echo "smoke(enter-stage): scope header wrong" >&2; exit 1; }
grep -q "^— scope aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): scope stamp missing" >&2; exit 1; }
if grep -q 'prev-iter' "$ess"; then echo "smoke(enter-stage): state not truncated on reset" >&2; exit 1; fi

# 2. second stage (align): append + flip, the scope stamp retained.
es_run align >/dev/null
grep -q "^## Iteration: —  \[stage: align\]\$" "$esq" || { echo "smoke(enter-stage): align header wrong" >&2; exit 1; }
grep -q "^— align aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): align stamp missing" >&2; exit 1; }
grep -q "^— scope aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): scope stamp lost on append" >&2; exit 1; }

# 3. idempotent re-entry: same session id ⇒ no duplicate stamp, exit 0.
es_run align >/dev/null
if [[ "$(grep -c "^— align aabbccdd " "$ess")" -ne 1 ]]; then
    echo "smoke(enter-stage): idempotent re-entry duplicated the align stamp" >&2; exit 1
fi

# 4. header absent: the pre-flight refuses and writes nothing.
grep -v '^## Iteration:' "$esq" > "$es/q.headerless"; cp "$es/q.headerless" "$esq"
cp "$esq" "$es/q.before"; cp "$ess" "$es/s.before"
if es_run build >/dev/null 2>&1; then echo "smoke(enter-stage): should refuse a headerless queue" >&2; exit 1; fi
cmp -s "$es/s.before" "$ess" || { echo "smoke(enter-stage): wrote state on a refusal" >&2; exit 1; }
cmp -s "$es/q.before" "$esq" || { echo "smoke(enter-stage): wrote queue on a refusal" >&2; exit 1; }

rm -rf "$es"
