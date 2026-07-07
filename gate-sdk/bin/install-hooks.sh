#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §install-hooks — wire core.hooksPath for this clone (opt-in, per clone)

set -euo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

GATES_DIR="$(gate_sdk_gates_dir)"
HOOKS_DIR="${GATE_SDK_HOOKS_DIR:-$GATES_DIR/git-hooks}"

[[ -d "$HOOKS_DIR" ]] || {
    echo "install-hooks: no hooks dir at $HOOKS_DIR — generate the pre-commit hook first:" >&2
    echo "  bash gate-sdk/bin/gen-pre-commit.sh --write" >&2
    exit 2
}

chmod +x "$HOOKS_DIR"/* 2>/dev/null || true

git config core.hooksPath "$HOOKS_DIR"
echo "Installed: core.hooksPath = $HOOKS_DIR"

if [[ -f .git-blame-ignore-revs ]]; then
    git config blame.ignoreRevsFile ".git-blame-ignore-revs"
    echo "Installed: blame.ignoreRevsFile = .git-blame-ignore-revs"
fi

# spec: gate-sdk/SPEC.md §install-hooks — apply-and-verify rung: run check-identity
# once at opt-in so a fresh clone learns of a wrong-identity/wrong-remote mapping
# before its first commit; the gate's exit status surfaces through this script's.
identity_rc=0
mapfile -t _check_dirs < <(gate_check_dirs)
if identity_gate="$(gate_resolve check-identity "${_check_dirs[@]}")"; then
    echo ""
    echo "Verifying git identity (check-identity)…"
    bash "$identity_gate" || identity_rc=$?
fi

echo "Active hooks:"
ls -1 "$HOOKS_DIR" | sed 's/^/  /'
echo ""
echo "Disable with:  git config --unset core.hooksPath"

exit "$identity_rc"
