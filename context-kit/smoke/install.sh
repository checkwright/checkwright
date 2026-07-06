#!/usr/bin/env bash
# context-kit consumer-smoke install — the executable form of README.md §Install.
# Registers check-brevity, copies the config + hook templates into the gates dir,
# wires the SessionStart hook, seeds a minimal always-loaded surface with a clean
# budgeted section, then self-verifies: the hook runs end-to-end (exit 0, emits
# the queue index, no fallback) and always-loaded --update-baseline writes the
# baseline file.
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored context-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

# 1. Register the gate (resolves from the vendored context-kit/checks/).
cat >> scripts/gates.list <<'EOF'
# context-kit
check-brevity
EOF

# 2. Config + hook templates into the gates dir (config unedited -> defaults).
cp "$SMOKE_KIT_ROOT/templates/context-config.sh"  scripts/context-config.sh
cp "$SMOKE_KIT_ROOT/templates/session-context.sh" scripts/session-context.sh

# 3. SessionStart wiring merged into .claude/settings.json (drop the note key).
mkdir -p .claude
if [[ -f .claude/settings.json ]]; then
    jq -s '.[0] * .[1] | del(.["//"])' \
        .claude/settings.json "$SMOKE_KIT_ROOT/templates/settings-sessionstart.json" \
        > .claude/settings.json.new
    mv .claude/settings.json.new .claude/settings.json
else
    jq 'del(.["//"])' "$SMOKE_KIT_ROOT/templates/settings-sessionstart.json" > .claude/settings.json
fi

# 4. A minimal always-loaded surface with a clean budgeted section (default
#    config governs CLAUDE.md §"## Shared conventions").
cat > CLAUDE.md <<'EOF'
# Smoke consumer

## Shared conventions

- **Terse:** one clean line, well within the four-line budget.
EOF

# 5. A queue file so the hook's queue-index step emits. context-kit installs
#    before queue-kit alphabetically, so reuse queue-kit's shipped template.
[[ -f TASK-QUEUE.md ]] || cp "$SMOKE_KIT_ROOT/../queue-kit/templates/TASK-QUEUE.md" TASK-QUEUE.md

# 6. Regenerate the coupling artifacts check-graph asserts fresh.
bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html

# 7. Self-verify: the hook runs end-to-end and actually emits the queue index.
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

# 8. Self-verify: the meter writes its baseline.
bash "$SMOKE_KIT_ROOT/bin/always-loaded.sh" --update-baseline >/dev/null
if [[ ! -f .workflow/always-loaded-baseline.txt ]]; then
    echo "context-kit/smoke/install.sh: always-loaded --update-baseline wrote no baseline" >&2
    exit 1
fi
