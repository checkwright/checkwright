#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — context-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored context-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30, which reaches this file by its GROUND rather than by its scope: that ruling stated-contract cut covers the recipes answering to §Consumer smoke and this one answers to context-kit/SPEC.md §Testing, but both legs hold of it identically. Leg 2: an executable install recipe by stated contract whose body check-install-disposition assertion B reads as text, this kit shipping check-brevity zero-config, so a crate table ADDS violations rather than removing them. Leg 3: it vendors with the kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers are this repo own validate suites. Structural, not a sizing judgment.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

# spec: gate-sdk/SPEC.md §lib/gate.sh — the ratchet is driven through gate_command, the way a consumer's own battery does, because it is exercised without being registered (below)
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

kit_gate() {   # $1=gate-name  $2.. = gate args — dispatch a vendored context-kit gate by name
    local g="$1"; shift
    local -a argv=()
    mapfile -t argv < <(gate_command "$g" "$SMOKE_KIT_ROOT/checks")
    [[ ${#argv[@]} -gt 0 ]] || return 2
    "${argv[@]}" "$@"
}

cat >> scripts/gates.list <<'EOF'
# context-kit
check-brevity
check-memory-off
check-settings-pins
check-settings-paths
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion I's own declaration grammar,
# a sibling of the ratchet's own unregistered-and-exercised comment below (§The install disposition)
# unregistered: check-footprint-fresh — byte-compares docs/footprint.md against its emitter; the scratch consumer vendors no docs/ tree
# unregistered: check-surface-ratchet — exercised unregistered by this script's own ratchet() calls below: a ceiling stamped and registered here would be asserted against a half-installed consumer every co-vendored kit installing after context-kit still grows
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

bash "$SDK/bin/run-gates.sh" --emit always-loaded --update-baseline >/dev/null
if [[ ! -f .workflow/always-loaded-baseline.txt ]]; then
    echo "context-kit/smoke/install.sh: always-loaded --update-baseline wrote no baseline" >&2
    exit 1
fi

# spec: context-kit/SPEC.md §Testing — the ratchet's three states in order: armed by `--ceiling` it is clean, a grown surface reds it, and a second `--ceiling` makes that growth the new floor. It is exercised unregistered and disarmed again at the end, because a ceiling left standing here would be stamped against a half-installed consumer that every co-vendored kit installing after context-kit still grows.
bash "$SDK/bin/run-gates.sh" --emit always-loaded --ceiling >/dev/null
ratchet() {  # $1=want-rc  $2=label
    local rc=0
    kit_gate check-surface-ratchet >/dev/null 2>&1 || rc=$?
    if [[ "$rc" -ne "$1" ]]; then
        echo "context-kit/smoke/install.sh: check-surface-ratchet $2: want exit $1, got $rc" >&2
        exit 1
    fi
}
ratchet 0 "on a freshly stamped ceiling"
printf '\n- **Grown:** one more resident line nobody priced.\n' >> CLAUDE.md
ratchet 1 "on a surface grown past its row"
bash "$SDK/bin/run-gates.sh" --emit always-loaded --ceiling >/dev/null
ratchet 0 "after the growth was deliberately re-stamped"
rm -f .workflow/surface-ceiling.txt
