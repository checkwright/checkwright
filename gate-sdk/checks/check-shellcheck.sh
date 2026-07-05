#!/usr/bin/env bash
# graph: couples=scripts/*.sh,gate-sdk/*.sh,lifecycle-kit/*.sh,queue-kit/*.sh,spec-kit/*.sh,friction-kit/*.sh,delegation-kit/*.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-shellcheck — ShellCheck lint of the gate family at -S warning (the self-lint contract)
#
# usage: check-shellcheck.sh [dir...]
#   Lints *.sh directly under each given dir. Default: the consumer gates dir
#   plus each vendored kit's lib/, bin/, and checks/ — the family lints itself
#   by the standard it enforces.

set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

if [[ $# -gt 0 ]]; then
    DIRS=("$@")
else
    DIRS=("$(gate_sdk_gates_dir)")
    while IFS= read -r k; do
        DIRS+=("$k/lib" "$k/bin" "$k/checks")
    done < <(gate_kit_roots)
fi

if ! command -v shellcheck >/dev/null 2>&1; then
    echo "check-shellcheck: shellcheck not found on PATH — the gate cannot run." >&2
    echo "  A gate that cannot run is not clean (fail-closed)." >&2
    echo "  help: install ShellCheck (e.g. 'apt install shellcheck' / 'brew install shellcheck')." >&2
    exit 2
fi

targets=()
shopt -s nullglob
for d in "${DIRS[@]}"; do
    targets+=("$d"/*.sh)
done
shopt -u nullglob

if [[ ${#targets[@]} -eq 0 ]]; then
    echo "check-shellcheck: no *.sh found under: ${DIRS[*]} — nothing to lint." >&2
    exit 2
fi

if output="$(shellcheck -S warning "${targets[@]}" 2>&1)"; then
    echo "SHELLCHECK: clean (${#targets[@]} scripts)"
    exit 0
fi

printf '%s\n' "$output"
echo
echo "help: ShellCheck flagged the script(s) above (-S warning). Fix each finding,"
echo "      or silence a genuine false positive inline with '# shellcheck"
echo "      disable=SCxxxx' PLUS a justifying comment (no blanket .shellcheckrc)."
exit 1
