#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-amendment-queue — the absent-queue clean and the wrong-kind refusal,
# arms the pair cannot spell: its good/ holds a queue, and exit 2 has no pair spelling
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=queue path
    local label="$1" want="$2" sub="$3" queue="$4"
    local out rc
    out="$(cd "$SANDBOX" && gate_run check-amendment-queue "$DIR/checks" "$queue" "$SANDBOX" 2>&1)"
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

ln -s "$SANDBOX/gone.md" "$SANDBOX/danglingqueue" 2>/dev/null
if [[ -L "$SANDBOX/danglingqueue" ]]; then
    check_case "dangling-link-queue-fails-closed" 2 "not a file" "$SANDBOX/danglingqueue"
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-amendment-queue.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-amendment-queue.test.sh: clean (an absent queue is not applicable; a queue path naming a directory or a dangling link refuses)"
exit 0
