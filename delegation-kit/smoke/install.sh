#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — consumer-smoke install (the executable
# form of README.md §Install): registers the tamper gate, regenerates the
# coupling artifacts, and drives one crafted snapshot through usage-gate — the
# other tools are invoked not gated, so coverage is the gate plus a live verdict
# on the extracted tool. cwd = scratch-consumer root; SMOKE_KIT_ROOT = the
# vendored delegation-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# delegation-kit
check-gate-tamper
EOF

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

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
