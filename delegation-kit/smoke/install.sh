#!/usr/bin/env bash
# delegation-kit consumer-smoke install — the executable form of README.md
# §Install. Registers the tamper gate and regenerates the coupling artifacts.
# The kit's other tools (usage-gate, the templates) need no registration — they
# are invoked, not gated — so install coverage is the gate plus a live
# usage-gate verdict on a crafted snapshot (self-verifying the extracted tool).
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored delegation-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# delegation-kit
check-gate-tamper
EOF

# Drive one crafted snapshot through usage-gate under zero config: a live,
# over-threshold reading must PAUSE (exit 1). This self-verifies the extracted
# tool inside the scratch consumer, not just the gate registration.
snap="$(mktemp)"
now="$(date +%s)"
{
    printf 'five_hour_used_pct=95\n'
    printf 'five_hour_resets_at=%s\n' "$(( now + 3600 ))"
    printf 'updated_at=%s\n' "$now"
} > "$snap"
if bash "$SMOKE_KIT_ROOT/bin/usage-gate.sh" "$snap" >/dev/null 2>&1; then
    echo "delegation-kit/smoke: usage-gate did not PAUSE on a live 95% reading" >&2
    rm -f "$snap"; exit 1
fi
rm -f "$snap"

# Regenerate the coupling artifacts check-graph asserts fresh (README steps).
bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
