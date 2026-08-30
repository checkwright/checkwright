#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — context-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored context-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30, which reaches this file by its GROUND rather than by its scope: that ruling stated-contract cut covers the recipes answering to §Consumer smoke and this one answers to context-kit/SPEC.md §Testing, but both legs hold of it identically. Leg 2: an executable install recipe by stated contract whose body check-install-disposition assertion B reads as text, this kit shipping check-brevity zero-config, so a crate table ADDS violations rather than removing them. Leg 3: it vendors with the kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers are this repo own validate suites. Structural, not a sizing judgment.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# context-kit
check-brevity
check-memory-off
check-settings-pins
check-settings-paths
EOF

cp "$SMOKE_KIT_ROOT/templates/context-config.sh"  scripts/context-config.sh
cp "$SMOKE_KIT_ROOT/templates/session-context.sh" scripts/session-context.sh

mkdir -p .claude
if [[ -f .claude/settings.json ]]; then
    jq -s '.[0] * .[1] | del(.["//"])' \
        .claude/settings.json "$SMOKE_KIT_ROOT/templates/settings-sessionstart.json" \
        > .claude/settings.json.new
    mv .claude/settings.json.new .claude/settings.json
else
    jq 'del(.["//"])' "$SMOKE_KIT_ROOT/templates/settings-sessionstart.json" > .claude/settings.json
fi

cat > CLAUDE.md <<'EOF'
# Smoke consumer

## Shared conventions

- **Terse:** one clean line, well within the four-line budget.
EOF

qtpl="$SMOKE_KIT_ROOT/../queue-kit/templates/TASK-QUEUE.md"
[[ -f TASK-QUEUE.md || ! -f "$qtpl" ]] || cp "$qtpl" TASK-QUEUE.md

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html

hook_out="$(bash scripts/session-context.sh 2>/dev/null)"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "context-kit/smoke/install.sh: session-context hook exited $rc (want 0)" >&2
    exit 1
fi
if ! grep -q 'Session context' <<<"$hook_out"; then
    echo "context-kit/smoke/install.sh: hook produced no session-context brief" >&2
    exit 1
fi
# spec: context-kit/SPEC.md §The session-context hook — the queue-index assertion is predicated on the front-end resolving, not on a tool file existing: the index is reached through `run-gates.sh --emit queue-index`, so what the hook needs present is the front-end (gate-sdk/SPEC.md §The non-gate arm)
if bash gate-sdk/bin/run-gates.sh --emit queue-index >/dev/null 2>&1 \
    && { grep -q 'queue-index unavailable' <<<"$hook_out" || ! grep -q 'Iteration:' <<<"$hook_out"; }; then
    echo "context-kit/smoke/install.sh: hook did not emit the queue index" >&2
    printf '%s\n' "$hook_out" >&2
    exit 1
fi

bash "$SMOKE_KIT_ROOT/bin/always-loaded.sh" --update-baseline >/dev/null
if [[ ! -f .workflow/always-loaded-baseline.txt ]]; then
    echo "context-kit/smoke/install.sh: always-loaded --update-baseline wrote no baseline" >&2
    exit 1
fi
