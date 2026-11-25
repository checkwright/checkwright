#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-todo-task-liveness — the absent-queue clean, with a marker on the
# surface, and the wrong-kind refusal: arms the pair cannot spell, since its good/ holds a queue
# and exit 2 has no pair spelling
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

printf '#!/usr/bin/env bash\n# TODO(task: ghost-task) a marker no queue resolves\necho hi\n' >"$SANDBOX/src.sh"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=queue path
    local label="$1" want="$2" sub="$3" queue="$4"
    local out rc
    out="$(cd "$SANDBOX" && gate_run check-todo-task-liveness "$DIR/checks" "$SANDBOX" "$queue" 2>&1)"
    rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

check_case "absent-queue-is-not-applicable" 0 "clean (not applicable: no queue file at" "$SANDBOX/TASK-QUEUE.md"

mkdir -p "$SANDBOX/dirqueue"
check_case "non-file-queue-fails-closed" 2 "not a file" "$SANDBOX/dirqueue"

if [[ "$fails" -gt 0 ]]; then
    echo "check-todo-task-liveness.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-todo-task-liveness.test.sh: clean (an absent queue is not applicable with a marker on the surface; a queue path naming a directory refuses)"
exit 0
