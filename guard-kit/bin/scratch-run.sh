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

# spec: guard-kit/SPEC.md §scratch-run — the bash-only rule's runner-side half, read off the file's own shebang rather than a roster; placed before the echo so a refusal still prints no body
first_line="$(head -n 1 "$TARGET" 2>/dev/null)"
if [[ "$first_line" == '#!'* ]]; then
    read -ra shebang <<<"${first_line#\#!}"
    interp="${shebang[0]:-}"
    [[ "${interp##*/}" == env ]] && interp="${shebang[1]:-}"
    case "${interp##*/}" in
        bash | sh | '') ;;
        *)
            echo "scratch-run: refusing $TARGET — scratch execution is bash-only (guard-kit/SPEC.md §scratch-run) and its shebang names '${interp##*/}'. Rewrite the body as a shell script, or do the work in a language the control covers." >&2
            exit 2
            ;;
    esac
fi

echo "=== scratch-run: $TARGET ==="
cat "$TARGET"
echo "=== scratch-run: executing $TARGET ==="
# spec: guard-kit/SPEC.md §scratch-run — the hardcoded interpreter is the bash-only rule, not an unexamined default: widening it would convert the committed grant for this path from "run bash on a reviewed body" into "run anything on a reviewed body" with no settings edit
bash "$TARGET" "$@"
exit $?
