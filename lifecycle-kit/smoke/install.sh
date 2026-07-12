#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — lifecycle-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored lifecycle-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# lifecycle-kit
check-stage-evidence
check-stage-entry
EOF

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

mkdir -p .workflow
cat > .workflow/WORKFLOW-STATE.txt <<EOF
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

— scope smoke001 $(date +%F)
EOF

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — exercise enter-stage end-to-end under .tmp (advisory tool, no fixture pair)
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
    LIFECYCLE_KIT_QUEUE_FILE="$es/TASK-QUEUE.md" \
    LIFECYCLE_KIT_STATE_FILE="$es/.workflow/WORKFLOW-STATE.txt" \
    LIFECYCLE_KIT_SESSIONS_DIR="$es/sessions" \
    GATE_SDK_TMP_DIR="$es/tmp" \
    bash "$SMOKE_KIT_ROOT/bin/enter-stage.sh" "$@"
}
esq="$es/TASK-QUEUE.md"; ess="$es/.workflow/WORKFLOW-STATE.txt"; d="$(date +%F)"

es_run scope >/dev/null
grep -q "^## Iteration: —  \[stage: scope\]\$" "$esq" || { echo "smoke(enter-stage): scope header wrong" >&2; exit 1; }
grep -q "^— scope aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): scope stamp missing" >&2; exit 1; }
if grep -q 'prev-iter' "$ess"; then echo "smoke(enter-stage): state not truncated on reset" >&2; exit 1; fi

es_run align >/dev/null
grep -q "^## Iteration: —  \[stage: align\]\$" "$esq" || { echo "smoke(enter-stage): align header wrong" >&2; exit 1; }
grep -q "^— align aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): align stamp missing" >&2; exit 1; }
grep -q "^— scope aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): scope stamp lost on append" >&2; exit 1; }

es_run align >/dev/null
if [[ "$(grep -c "^— align aabbccdd " "$ess")" -ne 1 ]]; then
    echo "smoke(enter-stage): idempotent re-entry duplicated the align stamp" >&2; exit 1
fi

grep -v '^## Iteration:' "$esq" > "$es/q.headerless"; cp "$es/q.headerless" "$esq"
cp "$esq" "$es/q.before"; cp "$ess" "$es/s.before"
if es_run build >/dev/null 2>&1; then echo "smoke(enter-stage): should refuse a headerless queue" >&2; exit 1; fi
cmp -s "$es/s.before" "$ess" || { echo "smoke(enter-stage): wrote state on a refusal" >&2; exit 1; }
cmp -s "$es/q.before" "$esq" || { echo "smoke(enter-stage): wrote queue on a refusal" >&2; exit 1; }

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_ENTRY_PREFLIGHT: a red entry command refuses the flip (no writes), a green one lets it through
cat > "$esq" <<'EOF'
# smoke queue
## Iteration: pf-iter  [stage: build]

---

## New Features
## Technical Debt
## Deferred
## Done
EOF
cat > "$ess" <<EOF
# contract

---

pf-iter build aabbccdd $d
EOF
cat > "$es/preflight-stub.sh" <<'STUB'
#!/usr/bin/env bash
[[ -f "$(dirname "$0")/preflight-ok" ]]
STUB
chmod +x "$es/preflight-stub.sh"
cat > "$es/stages.sh" <<STAGES
# shellcheck shell=bash
LIFECYCLE_KIT_ENTRY_PREFLIGHT=('validate=$es/preflight-stub.sh')
STAGES

es_pf_run() {
    LIFECYCLE_KIT_QUEUE_FILE="$esq" \
    LIFECYCLE_KIT_STATE_FILE="$ess" \
    LIFECYCLE_KIT_SESSIONS_DIR="$es/sessions" \
    GATE_SDK_TMP_DIR="$es/tmp" \
    LIFECYCLE_KIT_CONFIG_FILE="$es/stages.sh" \
    bash "$SMOKE_KIT_ROOT/bin/enter-stage.sh" "$@"
}

cp "$esq" "$es/q.before"; cp "$ess" "$es/s.before"
if es_pf_run validate >/dev/null 2>&1; then echo "smoke(enter-stage): red preflight should refuse the flip" >&2; exit 1; fi
cmp -s "$es/s.before" "$ess" || { echo "smoke(enter-stage): wrote state on a preflight refusal" >&2; exit 1; }
cmp -s "$es/q.before" "$esq" || { echo "smoke(enter-stage): wrote queue on a preflight refusal" >&2; exit 1; }

touch "$es/preflight-ok"
es_pf_run validate >/dev/null || { echo "smoke(enter-stage): green preflight should let the flip through" >&2; exit 1; }
grep -q "^## Iteration: pf-iter  \[stage: validate\]\$" "$esq" || { echo "smoke(enter-stage): green preflight did not flip to validate" >&2; exit 1; }
grep -q "^pf-iter validate aabbccdd $d\$" "$ess" || { echo "smoke(enter-stage): green preflight did not stamp" >&2; exit 1; }

# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — exercise the injector + check-lifecycle-registration end-to-end under .tmp (advisory tool, no fixture pair)
il="$es/agent"; mkdir -p "$il"
cat > "$il/CLAUDE.md" <<'EOF'
# Scratch agent file

Resident context the consumer keeps.
EOF
il_run() { LIFECYCLE_KIT_AGENT_FILE="$il/CLAUDE.md" bash "$SMOKE_KIT_ROOT/bin/install-lifecycle.sh" "$@"; }
il_gate() { LIFECYCLE_KIT_AGENT_FILE="$il/CLAUDE.md" bash "$SMOKE_KIT_ROOT/checks/check-lifecycle-registration.sh" "$@"; }

il_run >/dev/null
grep -q "<!-- lifecycle-kit:begin -->" "$il/CLAUDE.md" || { echo "smoke(install-lifecycle): block not injected" >&2; exit 1; }
il_gate >/dev/null || { echo "smoke(install-lifecycle): a freshly installed block should pass the parity gate" >&2; exit 1; }

cp "$il/CLAUDE.md" "$il/before"
il_run >/dev/null
cmp -s "$il/before" "$il/CLAUDE.md" || { echo "smoke(install-lifecycle): re-run was not idempotent" >&2; exit 1; }

sed -i 's#`/close`##' "$il/CLAUDE.md"
if il_gate >/dev/null 2>&1; then echo "smoke(install-lifecycle): a staled block should redden the parity gate" >&2; exit 1; fi

rm -rf "$es"
