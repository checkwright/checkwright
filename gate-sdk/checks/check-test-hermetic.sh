#!/usr/bin/env bash
# graph: couples=kit:gate-tests/*.test.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-test-hermetic — every bespoke gate-tests/*.test.sh sources lib/test-hermetic.sh or carries a `# hermetic-exempt:` marker
#
# usage: check-test-hermetic.sh [dir...]   (default: each kit's gate-tests/)
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

if [[ $# -gt 0 ]]; then
    SCAN_DIRS=("$@")
else
    SCAN_DIRS=()
    while IFS= read -r k; do SCAN_DIRS+=("$k/gate-tests"); done < <(gate_kit_roots)
fi

files=()
shopt -s nullglob
for d in "${SCAN_DIRS[@]}"; do
    files+=("$d"/*.test.sh)
done
shopt -u nullglob
[[ ${#files[@]} -gt 0 ]] || { echo "check-test-hermetic: no *.test.sh under: ${SCAN_DIRS[*]}" >&2; exit 2; }

leaky=()
total=0
for f in "${files[@]}"; do
    total=$((total + 1))
    grep -q 'lib/test-hermetic\.sh' "$f" && continue
    grep -qE '^#[[:space:]]*hermetic-exempt:' "$f" && continue
    leaky+=("$f")
done

if [[ ${#leaky[@]} -gt 0 ]]; then
    echo "check-test-hermetic: bespoke test(s) neither source lib/test-hermetic.sh nor"
    echo "carry a '# hermetic-exempt:' marker (gate-sdk/SPEC.md §check-test-hermetic — a"
    echo "test on the invoker's cwd config can green wrongly on the consumer's posture):"
    for f in "${leaky[@]}"; do echo "  $f"; done
    echo "  help: source the bootstrap as the test's first act —"
    echo "        source \"\$(dirname \"\${BASH_SOURCE[0]}\")/../../gate-sdk/lib/test-hermetic.sh\""
    echo "  (per-case config overrides after the source still win by ordering), OR add a"
    echo "  '# hermetic-exempt: <reason>' line for a test that proves hermeticity otherwise."
    exit 1
fi

echo "TEST-HERMETIC: clean ($total bespoke test(s) pinned to kit defaults)"
exit 0
