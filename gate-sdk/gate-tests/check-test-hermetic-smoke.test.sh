#!/usr/bin/env bash
# Direct unit test of check-test-hermetic.sh assertion (B) via the --smoke mode.
# The good/bad fixture pair stays on assertion (A) (the gate-tests bootstrap
# scan), so the smoke credential-pin rule needs its own test: a
# credential-managing smoke script whose own-kit bin call carries no *_CRED_FILE
# pin reds; the same call pinned clears; a smoke script that manages no
# credentials is never triggered; and the # hermetic-exempt: valve opts a file
# out. Sourcing lib/test-hermetic.sh isolates the test from ambient config.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-test-hermetic.sh"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

# a credential-managing smoke script with the own-kit bin call PINNED — compliant
mkdir -p "$tmp/good"
cat > "$tmp/good/install.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
DEMO_KIT_CRED_FILE="$pp/absent.json" bash "$SMOKE_KIT_ROOT/bin/verdict.sh" "$snap"
EOF

# a credential-managing smoke script whose own-kit bin call is UNPINNED — leaks
mkdir -p "$tmp/bad"
cat > "$tmp/bad/install.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
DEMO_KIT_CRED_FILE="$pp/creds.json" bash "$SMOKE_KIT_ROOT/bin/poller.sh"
bash "$SMOKE_KIT_ROOT/bin/verdict.sh" "$snap"
EOF

# a smoke script that manages no credentials — the trigger never fires
mkdir -p "$tmp/neutral"
cat > "$tmp/neutral/install.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
bash "$SMOKE_KIT_ROOT/bin/install-thing.sh"
EOF

# the same leaky shape as bad/, but opted out with the hermetic-exempt valve
mkdir -p "$tmp/exempt"
cat > "$tmp/exempt/install.sh" <<'EOF'
#!/usr/bin/env bash
# hermetic-exempt: constructs its own HOME sandbox before any bin call.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
DEMO_KIT_CRED_FILE="$pp/creds.json" bash "$SMOKE_KIT_ROOT/bin/poller.sh"
bash "$SMOKE_KIT_ROOT/bin/verdict.sh" "$snap"
EOF

run() { out="$("$GATE" --smoke "$1" 2>&1)"; rc=$?; }

run "$tmp/good"
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: pinned own-kit bin call expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

run "$tmp/bad"
[[ "$rc" -eq 1 ]] \
    || { echo "  FAIL: unpinned own-kit bin call expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- 'with no *_CRED_FILE= pin on the line' <<<"$out" \
    || { echo "  FAIL: red output does not name the ambient-credential leak: $out"; fails=$((fails + 1)); }

run "$tmp/neutral"
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: credential-free smoke script must not trigger, got $rc: $out"; fails=$((fails + 1)); }

run "$tmp/exempt"
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: hermetic-exempt smoke script expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-test-hermetic-smoke.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-test-hermetic-smoke.test: ok (unpinned cred bin reds; pinned clears; no-cred untriggered; exempt opts out)"
exit 0
