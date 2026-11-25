#!/usr/bin/env bash
# Behavioral test of check-binary-strings over the arms the good/+bad/ pair cannot
# reach: a case names its outputs in its knob file and both of its outputs read,
# so a named output that is absent, an ELF header cut short, a GNU escape in a
# shape list and the record order across files live here. Each expect.txt line is
# an independent substring assertion, so record *order* is assertable only here.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
CASES="$DIR/gate-tests/check-binary-strings"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

mkdir -p "$SANDBOX/repo"
git -C "$SANDBOX/repo" init -q

# --- a named output that is not there refuses at 2: the knob names it exactly, so
# its absence is a layout the consumer must fix, never an output with no strings.
out="$( cd "$SANDBOX/repo" && gate_env GATE_SDK_STRINGS_FILES=absent.bin \
    && gate_run check-binary-strings "$CHECKS" 2>&1 )"; rc=$?
expect absent-output 2 'no build output at absent.bin' "$rc" "$out"

# --- an ELF64 little-endian file whose header is cut short is unparseable input at 3.
printf '\177ELF\002\001 cut short' > "$SANDBOX/repo/short.elf"
out="$( cd "$SANDBOX/repo" && gate_env GATE_SDK_STRINGS_FILES=short.elf \
    && gate_run check-binary-strings "$CHECKS" 2>&1 )"; rc=$?
expect truncated-elf 3 'ELF header truncated' "$rc" "$out"

# --- a GNU escape in a shape list is refused at 2, never read as a clean output.
printf 'plain bytes here\n' > "$SANDBOX/repo/plain.bin"
printf '\\bplain\\b\n' > "$SANDBOX/repo/gnu.list"
out="$( cd "$SANDBOX/repo" && gate_env GATE_SDK_STRINGS_FILES=plain.bin GATE_SDK_STRINGS_SHAPES=gnu.list \
    && gate_run check-binary-strings "$CHECKS" 2>&1 )"; rc=$?
expect gnu-escape 2 'shape-compile' "$rc" "$out"

# --- record order over the bad pair's outputs, named in reverse of their path order:
# files in knob order, then offset within a file, whichever list the hit came from.
out="$( cd "$CASES/bad" \
    && gate_env GATE_SDK_STRINGS_FILES='out/notes.bin out/app.elf' GATE_SDK_STRINGS_SHAPES=shapes.list \
        GATE_SDK_DENY_TERMS=terms.list \
    && gate_run check-binary-strings "$CHECKS" 2>&1 )"; rc=$?
got="$(grep -E '^out/' <<<"$out")"
want="$(printf '%s\n' \
    'out/notes.bin@11: [CODENAME] term 1' \
    'out/app.elf@83: [HOME-PATH] shape 1' \
    'out/app.elf@115: [CODENAME] term 1')"
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [record-order]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
elif [[ "$got" != "$want" ]]; then
    echo "  FAIL [record-order]: record sequence is not knob-then-offset order:"
    printf '    %s\n' "$got"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-binary-strings.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-binary-strings.test.sh: clean (the arms a case dir cannot reach: an absent named output, a truncated ELF header, a GNU escape in a shape list, and the record order across outputs — 4 cases)"
exit 0
