#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §check-template-copy-parity — the root-default derivation, which the
# good/bad pair structurally cannot reach: every committed case passes an explicit root, so the
# `git rev-parse --show-toplevel` branch and its refusal are exercised by nothing. This drives
# both: a sandbox git repo entered with no argument, and the same tree with git unable to answer.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
sb="$(mktemp -d)"
trap 'rm -rf "$sb"' EXIT
fails=0

mkdir -p "$sb/repo/widget-kit/templates" "$sb/repo/scripts" "$sb/norepo/scripts"
cat > "$sb/repo/widget-kit/templates/thing.sh" <<'EOF'
# spec: widget-kit/SPEC.md §thing — the template
helper_one() { :; }
: "${WIDGET_KIT_A:-1}"
EOF
cat > "$sb/repo/scripts/thing.sh" <<'EOF'
# spec: widget-kit/SPEC.md §thing — the vendored copy, glossed differently on purpose
helper_one() { :; }
: "${WIDGET_KIT_A:-1}"
EOF
git -C "$sb/repo" init -q
git -C "$sb/repo" config user.email t@example.invalid
git -C "$sb/repo" config user.name t

# --- the default root: entered from a SUBDIRECTORY, so a cwd-relative default would find
# --- no pair at all and pass vacuously where the toplevel derivation finds one.
out="$( cd "$sb/repo/scripts" && gate_run check-template-copy-parity "$DIR/checks" 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [toplevel-default]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF '1 template<->copy pair(s)' <<<"$out"; then
    echo "  FAIL [toplevel-default-reached]: the derivation resolved a root carrying no pair, so"
    echo "        a clean verdict here would be vacuous: $out"
    fails=$((fails + 1))
fi

# --- git cannot answer: the refusal, which is the other half of the same branch.
out="$( cd "$sb/norepo" && gate_env GIT_CEILING_DIRECTORIES="$sb" \
    && gate_run check-template-copy-parity "$DIR/checks" 2>&1 )"; rc=$?
if [[ "$rc" -ne 2 ]]; then
    echo "  FAIL [no-repo-refuses]: want exit 2, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF 'not a git repository and no root given' <<<"$out"; then
    echo "  FAIL [no-repo-message]: the refusal does not name its cause: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-template-copy-parity.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-template-copy-parity.test.sh: clean (git-toplevel root default reached from a subdirectory + the no-repository refusal)"
exit 0
