#!/usr/bin/env bash
# Behavioral test of checks/check-lifecycle-registration.sh — the scenarios the
# one-pair good/bad harness cannot hold. The good/bad fixture pair covers the
# byte-lockstep-clean case and the stale-block case; this drives the
# block-absent finding, the unpaired-marker fail-closed, and the
# agent-file-missing fail-closed.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-lifecycle-registration.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=sandbox-dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && "$GATE" AGENT.md 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# --- block-absent: an agent file with no marker block at all ---
a="$SANDBOX/absent"; mkdir -p "$a"
cat >"$a/AGENT.md" <<'EOF'
# Agent (fixture)

The registration block was never installed — no markers.
EOF
check_case "block-absent" "$a" 1 "no lifecycle-kit registration block"

# --- unpaired-marker: a begin marker with no matching end (fail-closed) ---
u="$SANDBOX/unpaired"; mkdir -p "$u"
cat >"$u/AGENT.md" <<'EOF'
# Agent (fixture)

<!-- lifecycle-kit:begin -->
The block opened but the end marker is gone.
EOF
check_case "unpaired-marker-failclosed" "$u" 2 "end marker missing"

# --- agent-file-missing: the configured target does not exist (fail-closed) ---
m="$SANDBOX/missing"; mkdir -p "$m"
check_case "agent-file-missing-failclosed" "$m" 2 "agent file not found"

if [[ "$fails" -gt 0 ]]; then
    echo "check-lifecycle-registration.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-lifecycle-registration.test.sh: clean (block-absent + unpaired-marker + agent-file-missing, 3 cases)"
exit 0
