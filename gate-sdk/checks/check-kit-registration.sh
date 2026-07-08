#!/usr/bin/env bash
# graph: couples=README.md,CLAUDE.md,kit:gate-tests/* dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-kit-registration — every gate_kit_roots kit is registered in the human-facing docs: a registry-doc row for each root, and a fixture-runner line for each root that ships gate-tests
#
# usage: check-kit-registration.sh [registry-doc [runner-doc]]
#   docs resolve relative to the git toplevel; defaults README.md / CLAUDE.md,
#   overridable via GATE_SDK_REGISTRY_DOC / GATE_SDK_RUNNER_DOC.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
    || { echo "check-kit-registration: not a git repository — cannot test tracked kit files" >&2; exit 2; }

REGISTRY_DOC="${1:-${GATE_SDK_REGISTRY_DOC:-README.md}}"
RUNNER_DOC="${2:-${GATE_SDK_RUNNER_DOC:-CLAUDE.md}}"
[[ "$REGISTRY_DOC" == /* ]] || REGISTRY_DOC="$REPO_ROOT/$REGISTRY_DOC"
[[ "$RUNNER_DOC" == /* ]]   || RUNNER_DOC="$REPO_ROOT/$RUNNER_DOC"
[[ -f "$REGISTRY_DOC" ]] || { echo "check-kit-registration: registry doc not found: $REGISTRY_DOC" >&2; exit 2; }
[[ -f "$RUNNER_DOC" ]]   || { echo "check-kit-registration: runner doc not found: $RUNNER_DOC" >&2; exit 2; }

mapfile -t KIT_ROOTS < <(gate_kit_roots_rel)
[[ ${#KIT_ROOTS[@]} -gt 0 ]] || { echo "check-kit-registration: no kit roots enumerated" >&2; exit 2; }

root_ships_gate_tests() {
    local root="$1" out st
    out="$(git -C "$REPO_ROOT" ls-files -- "${root%/}/gate-tests/" 2>/dev/null)"; st=$?
    fail_closed "$st" check-kit-registration "git ls-files"
    [[ -n "$out" ]]
}

missing_row=()
missing_runner=()
runner_owed=0
for r in "${KIT_ROOTS[@]}"; do
    r="${r%/}"
    # assertion A: registry row — a '](<kit>/)' link for the root in the registry doc
    grep -qF -- "]($r/)" "$REGISTRY_DOC" || missing_row+=("$r")
    # assertion B: fixture-runner line — a '<kit>/gate-tests' line for each root shipping gate-tests
    if root_ships_gate_tests "$r"; then
        runner_owed=$((runner_owed + 1))
        grep -qF -- "$r/gate-tests" "$RUNNER_DOC" || missing_runner+=("$r")
    fi
done

if [[ ${#missing_row[@]} -gt 0 || ${#missing_runner[@]} -gt 0 ]]; then
    if [[ ${#missing_row[@]} -gt 0 ]]; then
        echo "check-kit-registration: kit root(s) not registered in the registry doc"
        echo "($REGISTRY_DOC has no '](<kit>/)' link row):"
        for r in "${missing_row[@]}"; do echo "  $r"; done
    fi
    if [[ ${#missing_runner[@]} -gt 0 ]]; then
        [[ ${#missing_row[@]} -gt 0 ]] && echo ""
        echo "check-kit-registration: kit root(s) shipping gate-tests but absent from the"
        echo "fixture-runner battery ($RUNNER_DOC names no '<kit>/gate-tests' line):"
        for r in "${missing_runner[@]}"; do echo "  $r"; done
    fi
    echo "  help: add the kit's registry row to $REGISTRY_DOC (a '](<kit>/)' link) and,"
    echo "        for a kit that ships gate-tests, its 'run-gate-tests.sh <kit>/gate-tests'"
    echo "        line to $RUNNER_DOC, so a landed kit cannot fall out of the docs."
    exit 1
fi

echo "KIT-REGISTRATION: clean (${#KIT_ROOTS[@]} kit root(s) each carry a registry row; ${runner_owed} shipping gate-tests each name a fixture-runner line)"
exit 0
