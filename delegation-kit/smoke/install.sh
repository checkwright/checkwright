#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored delegation-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30, which reaches this file by its GROUND rather than by its scope: that ruling stated-contract cut covers the recipes answering to §Consumer smoke and this one answers to delegation-kit/SPEC.md §Testing, but both legs hold of it identically. Leg 2: an executable install recipe by stated contract whose body check-install-disposition assertion B reads as text, this kit shipping check-gate-tamper zero-config, so a crate table ADDS violations rather than removing them. Leg 3: it vendors with the kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers are this repo own validate suites. Structural, not a sizing judgment.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

# spec: delegation-kit/SPEC.md §Testing — one hermetic env prelude for the whole file: strip the whole DELEGATION_KIT_* namespace, then pin the knobs with no per-call home, so no assertion below inherits the host's config or writes a sample through it
while IFS= read -r knob; do unset "$knob"; done < <(env | grep -o '^DELEGATION_KIT_[A-Za-z0-9_]*' || true)
export DELEGATION_KIT_USAGE_HISTORY=""
export DELEGATION_KIT_PAUSE_PCT=80
export DELEGATION_KIT_PAUSE_PCT_7D=95

cat >> scripts/gates.list <<'EOF'
# delegation-kit
check-gate-tamper
check-agent-tier-explicit
check-rule-citation
EOF

snap="$(mktemp)"
now="$(date +%s)"
{
    printf 'five_hour_used_pct=95\n'
    printf 'five_hour_resets_at=%s\n' "$(( now + 3600 ))"
    printf 'updated_at=%s\n' "$now"
} > "$snap"
# spec: delegation-kit/SPEC.md §Testing — the cred pin stays at the invocation (gate-sdk/SPEC.md §check-test-hermetic wants line-local evidence): an absent path zeroes login_at so no ambient auth event reroutes this verdict
DELEGATION_KIT_CRED_FILE="$snap.nocred" bash "$SMOKE_KIT_ROOT/bin/usage-verdict.sh" "$snap" >/dev/null 2>&1 && vrc=0 || vrc=$?
if [[ "$vrc" -ne 1 ]]; then
    echo "delegation-kit/smoke: usage-verdict on a live 95% reading: want exit 1 (PAUSE), got $vrc" >&2
    rm -f "$snap"; exit 1
fi
rm -f "$snap"

# spec: delegation-kit/SPEC.md §The usage.txt contract — the --usage-poll arm through its file:// stub seam: happy path writes a contract-valid snapshot, fetch failure leaves a pre-seeded one byte-identical
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
    bash "$SDK/bin/run-gates.sh" --usage-poll
}
poller "file://$pp/stub.json" || { echo "delegation-kit/smoke: poller happy path failed" >&2; exit 1; }
# spec: delegation-kit/SPEC.md §Testing — same line-local cred pin as the 95% check above
DELEGATION_KIT_CRED_FILE="$pp/absent.json" bash "$SMOKE_KIT_ROOT/bin/usage-verdict.sh" "$pp/usage.txt" >/dev/null || {
    echo "delegation-kit/smoke: poller snapshot did not verdict OK" >&2; exit 1; }
cp "$pp/usage.txt" "$pp/usage.before"
if poller "file://$pp/nonexistent.json" 2>/dev/null; then
    echo "delegation-kit/smoke: poller fetch failure exited zero" >&2; exit 1
fi
cmp -s "$pp/usage.before" "$pp/usage.txt" || {
    echo "delegation-kit/smoke: poller fetch failure touched the snapshot" >&2; exit 1; }
rm -rf "$pp"

# spec: delegation-kit/SPEC.md §The turn-end liveness hook — the --hook subagent-stop-liveness arm is exercised with a crafted payload on its allowing arm: the knob is emptied, so the firing holds no reading (verdict=unavailable) whatever the run dir carries, and it must exit 0 and append exactly one grammar-conformant line
sp="$PWD/.tmp/stop-probe-smoke"
rm -rf "$sp"; mkdir -p "$sp"
printf 'pid=1 run=smoke\n' > "$sp/smoke.run"
printf '{"session_id":"smoke","hook_event_name":"SubagentStop"}' | \
    DELEGATION_KIT_STOP_LOG="$sp/probe.log" \
    DELEGATION_KIT_LIVENESS_CMD="" \
    GATE_SDK_TMP_DIR="$sp" \
    bash "$SDK/bin/run-gates.sh" --hook subagent-stop-liveness || {
    echo "delegation-kit/smoke: the SubagentStop hook did not exit 0 on its unavailable arm" >&2; exit 1; }
probe_line="$(cat "$sp/probe.log")"
case "$probe_line" in
    *"event=SubagentStop"*"session=smoke"*"live=no"*"verdict=unavailable"*"records=1"*"runs=smoke"*"decision=allow"*"keys="*) ;;
    *) echo "delegation-kit/smoke: hook line off grammar: $probe_line" >&2; exit 1 ;;
esac
if [[ "$(grep -c . "$sp/probe.log")" -ne 1 ]]; then
    echo "delegation-kit/smoke: the hook wrote more than one line for one firing" >&2; exit 1
fi
rm -rf "$sp"

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html
