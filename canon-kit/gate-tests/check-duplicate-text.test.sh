#!/usr/bin/env bash
# Behavioral test of the arms the one-pair good/bad harness cannot hold: the
# within-file switch turned on, the exclude knob dropping a file from the
# comparison, and the exit-3 refusal of an unreadable declared file.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'chmod -R u+rwx "$SANDBOX" 2>/dev/null; rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/a.md" <<'EOF'
# A

The battery runs on every commit before it lands.

- The battery runs on every commit before it lands.
EOF
cat >"$SANDBOX/b.md" <<'EOF'
# B

A page of its own, with nothing in common.
EOF
cat >"$SANDBOX/gates.knobs" <<'EOF'
CANON_KIT_DUPLICATE_TEXT_GLOBS[] = *.md
CANON_KIT_DUPLICATE_TEXT_MIN_WORDS = 6
EOF

run() {  # $1..=extra env assignments; prints output, returns rc
    (cd "$SANDBOX" && gate_env CANON_KIT_KNOB_FILE="$SANDBOX/gates.knobs" "$@" \
        && gate_run check-duplicate-text "$DIR/checks" 2>&1)
}

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=output  $5=rc
    if [[ "$5" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $5 -- $4"; fails=$((fails + 1)); return
    fi
    if [[ -n "$3" ]] && ! grep -qF -- "$3" <<<"$4"; then
        echo "  FAIL [$1]: exit $5 OK but output lacks '$3':"; printf '    %s\n' "$4"
        fails=$((fails + 1))
    fi
}

out="$(run)"; rc=$?
check_case "within-file-off" 0 "no run of 6 words or more stated in two files" "$out" "$rc"

out="$(run CANON_KIT_DUPLICATE_TEXT_WITHIN_FILE=on)"; rc=$?
check_case "within-file-on" 1 "a.md:5: a run of 9 words first stated at a.md:3" "$out" "$rc"

cp "$SANDBOX/a.md" "$SANDBOX/c.md"
out="$(run)"; rc=$?
check_case "copy-compared" 1 "c.md:3: a run of 9 words first stated at a.md:3" "$out" "$rc"

printf '%s\n' 'CANON_KIT_DUPLICATE_TEXT_EXCLUDE[] = c.md' >>"$SANDBOX/gates.knobs"
out="$(run)"; rc=$?
check_case "exclude-drops-the-copy" 0 "DUPLICATE-TEXT: clean (2 file(s)" "$out" "$rc"

# A tree where the sandbox owner can read what it just made unreadable proves
# nothing, so the refusal is probed before it is asserted.
chmod 000 "$SANDBOX/b.md"
if cat "$SANDBOX/b.md" >/dev/null 2>&1; then
    echo "  SKIP [unreadable-file-fails-closed]: this user reads a 0000 file"
else
    out="$(run)"; rc=$?
    check_case "unreadable-file-fails-closed" 3 "b.md" "$out" "$rc"
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-duplicate-text.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-duplicate-text.test.sh: clean (within-file off + on + a compared copy + the exclude knob + an unreadable file)"
exit 0
