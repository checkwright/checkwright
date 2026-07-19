#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored delegation-kit copy.
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
# spec: delegation-kit/SPEC.md §usage-verdict — hermetic cred pin: point CRED_FILE at an absent path so login_at=0 skips the login-window reroute; the ambient ~/.claude cred (or a fresh stub, mtime ~now) falls inside LOGIN_WINDOW and would reroute this call to STALE, making the smoke flaky by wall-clock
if DELEGATION_KIT_CRED_FILE="$snap.nocred" bash "$SMOKE_KIT_ROOT/bin/usage-verdict.sh" "$snap" >/dev/null 2>&1; then
    echo "delegation-kit/smoke: usage-verdict did not PAUSE on a live 95% reading" >&2
    rm -f "$snap"; exit 1
fi
rm -f "$snap"

# spec: delegation-kit/SPEC.md §The usage.txt contract — the poll producer through its file:// stub seam: happy path writes a contract-valid snapshot, fetch failure leaves a pre-seeded one byte-identical
pp="$PWD/.tmp/poller-smoke"
rm -rf "$pp"; mkdir -p "$pp"
printf '{"claudeAiOauth":{"accessToken":"smoke-stub-token","subscriptionType":"stub"}}\n' > "$pp/creds.json"
now="$(date +%s)"
printf '{"five_hour":{"utilization":12.5,"resets_at":%s},"seven_day":{"utilization":7,"resets_at":%s}}\n' \
    "$(( now + 3600 ))" "$(( now + 86400 ))" > "$pp/stub.json"
poller() {
    DELEGATION_KIT_USAGE_FILE="$pp/usage.txt" \
    DELEGATION_KIT_CRED_FILE="$pp/creds.json" \
    DELEGATION_KIT_ACCOUNT_CONFIG="$pp/absent.json" \
    DELEGATION_KIT_USAGE_ENDPOINT="$1" \
    bash "$SMOKE_KIT_ROOT/templates/usage-poller.sh"
}
poller "file://$pp/stub.json" || { echo "delegation-kit/smoke: poller happy path failed" >&2; exit 1; }
# spec: delegation-kit/SPEC.md §usage-verdict — same hermetic cred pin as the 95% check above; the absent path keeps this OK verdict from rerouting to STALE when the ambient cred rotated inside LOGIN_WINDOW
DELEGATION_KIT_CRED_FILE="$pp/absent.json" bash "$SMOKE_KIT_ROOT/bin/usage-verdict.sh" "$pp/usage.txt" >/dev/null || {
    echo "delegation-kit/smoke: poller snapshot did not verdict OK" >&2; exit 1; }
cp "$pp/usage.txt" "$pp/usage.before"
if poller "file://$pp/nonexistent.json" 2>/dev/null; then
    echo "delegation-kit/smoke: poller fetch failure exited zero" >&2; exit 1
fi
cmp -s "$pp/usage.before" "$pp/usage.txt" || {
    echo "delegation-kit/smoke: poller fetch failure touched the snapshot" >&2; exit 1; }
rm -rf "$pp"

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
