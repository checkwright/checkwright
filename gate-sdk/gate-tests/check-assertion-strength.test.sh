#!/usr/bin/env bash
# Behavioral test of GATE_SDK_ASSERTION_STRENGTH_WINDOW, which the one good/bad
# pair cannot hold: a pair runs one config per case dir, and the window's reach
# needs the same guard read at two values plus the refusal of a malformed one.
# The guard below names PAUSE ten lines under its call, outside the default
# window and inside a wider one.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
mkdir -p "$SANDBOX/kit/bin" "$SANDBOX/kit/smoke"

cat >"$SANDBOX/kit/bin/verdict.sh" <<'EOF'
#!/usr/bin/env bash
# usage: verdict.sh [snapshot]
#   exit: 0 OK, 1 PAUSE, 2 STALE or unreadable (fail-closed)
exit 0
EOF
{ printf '#!/usr/bin/env bash\n'
  printf 'if bash "$SMOKE_KIT_ROOT/bin/verdict.sh" "$snap" >/dev/null 2>&1; then\n'
  printf '    : step\n%.0s' {1..9}
  printf '    echo "smoke: verdict did not PAUSE on a live reading" >&2\n'
  printf '    exit 1\nfi\n'
} >"$SANDBOX/kit/smoke/install.sh"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  [$4=window]
    local label="$1" want="$2" sub="$3" out rc
    out="$(cd "$SANDBOX" && gate_env ${4:+GATE_SDK_ASSERTION_STRENGTH_WINDOW="$4"} \
        && gate_run check-assertion-strength "$DIR/checks" "$SANDBOX/kit/smoke" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

check_case "window-default-misses-line-10" 0 "1 call(s) to a script with a declared exit contract"
check_case "window-12-reaches-line-10" 1 "message names PAUSE (exit 1 of verdict.sh)" 12
check_case "window-zero-exits-2" 2 "GATE_SDK_ASSERTION_STRENGTH_WINDOW must be a positive integer" 0
check_case "window-non-integer-exits-2" 2 "GATE_SDK_ASSERTION_STRENGTH_WINDOW must be a positive integer" wide

if [[ "$fails" -gt 0 ]]; then
    echo "check-assertion-strength.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-assertion-strength.test.sh: clean (the window misses a message past it, reaches it when widened, and refuses a zero or non-integer value)"
exit 0
