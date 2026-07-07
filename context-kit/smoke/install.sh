#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — context-kit consumer-smoke install, the
# executable form of README.md §Install: registers check-brevity, copies the
# config + hook templates into the gates dir, wires the SessionStart hook, seeds
# a minimal always-loaded surface with a clean budgeted section, then
# self-verifies (hook runs end-to-end emitting the queue index; always-loaded
# --update-baseline writes the baseline).
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored context-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# context-kit
check-brevity
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

[[ -f TASK-QUEUE.md ]] || cp "$SMOKE_KIT_ROOT/../queue-kit/templates/TASK-QUEUE.md" TASK-QUEUE.md  # context-kit installs before queue-kit, so reuse its queue template for the hook's queue-index step

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html

hook_out="$(bash scripts/session-context.sh 2>/dev/null)"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "context-kit/smoke/install.sh: session-context hook exited $rc (want 0)" >&2
    exit 1
fi
if ! grep -q 'Session context' <<<"$hook_out"; then
    echo "context-kit/smoke/install.sh: hook produced no session-context brief" >&2
    exit 1
fi
if grep -q 'queue-index unavailable' <<<"$hook_out" || ! grep -q 'Iteration:' <<<"$hook_out"; then
    echo "context-kit/smoke/install.sh: hook did not emit the queue index" >&2
    printf '%s\n' "$hook_out" >&2
    exit 1
fi

bash "$SMOKE_KIT_ROOT/bin/always-loaded.sh" --update-baseline >/dev/null
if [[ ! -f .workflow/always-loaded-baseline.txt ]]; then
    echo "context-kit/smoke/install.sh: always-loaded --update-baseline wrote no baseline" >&2
    exit 1
fi
