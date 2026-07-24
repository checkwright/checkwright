#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §scratch-run — echo-then-exec runner for scratch scripts
# usage: scratch-run.sh <script-inside-scratch-dir> [args…]
set -uo pipefail

SCRATCH_DIR="${GATE_SDK_TMP_DIR:-.tmp}"

TARGET="${1:-}"
[[ -n "$TARGET" ]] || { echo "scratch-run: usage: scratch-run.sh <script> [args…]" >&2; exit 2; }
shift

if ! scratch_abs="$(cd "$SCRATCH_DIR" 2>/dev/null && pwd -P)"; then
    echo "scratch-run: no scratch dir at $SCRATCH_DIR (GATE_SDK_TMP_DIR)" >&2
    exit 2
fi
if [[ ! -f "$TARGET" ]]; then
    echo "scratch-run: no such script: $TARGET" >&2
    exit 2
fi
target_abs="$(cd "$(dirname "$TARGET")" && pwd -P)/$(basename "$TARGET")"

case "$target_abs" in
    "$scratch_abs"/*) ;;
    *)
        echo "scratch-run: refusing $TARGET — outside the scratch dir $scratch_abs" >&2
        exit 2
        ;;
esac

echo "=== scratch-run: $TARGET ==="
cat "$TARGET"
echo "=== scratch-run: executing $TARGET ==="
bash "$TARGET" "$@"
exit $?
