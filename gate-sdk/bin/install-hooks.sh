#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §install-hooks — wire core.hooksPath for this clone (opt-in, per clone)

set -euo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

cd "$(git rev-parse --show-toplevel 2>/dev/null)"
# shellcheck disable=SC2034  # pwd -P is the dialect crossing itself (gate-sdk/SPEC.md §The path-dialect contract); re-derived even though nothing here reads it further
REPO_ROOT="$(pwd -P)"

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
# spec: gate-sdk/SPEC.md §install-hooks — the rung reaches the gate through gate_command
# rather than by interpreting the resolved declaration path: a .gate descriptor is a data
# file of comment lines, and `bash` on it exits 0, so the rung would pass silently
identity_rc=0
mapfile -t _check_dirs < <(gate_check_dirs)
_identity_argv_out=""
_identity_resolve_rc=0
_identity_argv_out="$(gate_command check-identity "${_check_dirs[@]}")" || _identity_resolve_rc=$?
if [[ "$_identity_resolve_rc" -eq 0 ]]; then
    mapfile -t _identity_argv <<<"$_identity_argv_out"
    echo ""
    echo "Verifying git identity (check-identity)…"
    "${_identity_argv[@]}" || identity_rc=$?
elif [[ "$_identity_resolve_rc" -ne 1 ]]; then
    # spec: gate-sdk/SPEC.md §install-hooks — status 1 is "no such member here", which a
    # consumer without the gate is entitled to; anything else is a dispatch that could not
    # be built, already named on stderr, and it fails the opt-in rather than skipping it
    identity_rc="$_identity_resolve_rc"
fi

echo "Active hooks:"
ls -1 "$HOOKS_DIR" | sed 's/^/  /'
echo ""
echo "Disable with:  git config --unset core.hooksPath"

exit "$identity_rc"
