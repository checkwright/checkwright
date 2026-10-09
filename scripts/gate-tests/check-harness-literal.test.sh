#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §check-harness-literal — the arms the good/+bad/ pair cannot reach: a case sets both knobs and ships readable text inside a repository, so the disabled cleans, the binary skip and the non-repository fail-close have no case form
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # scripts/, the gates dir declaring the member
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

run() {  # run <literals> <paths>, in the current directory
    gate_env GATE_SDK_HARNESS_LITERALS="$1" GATE_SDK_HARNESS_LITERAL_PATHS="$2" \
        && gate_run check-harness-literal "$DIR" 2>&1
}

mkdir -p "$SANDBOX/repo" "$SANDBOX/bare"
git -C "$SANDBOX/repo" init -q
printf 'Anchor at DEMO_HARNESS_DIR.\n' > "$SANDBOX/repo/prose.md"
git -C "$SANDBOX/repo" add prose.md

out="$( cd "$SANDBOX/repo" && run DEMO_HARNESS_DIR '' )"; rc=$?
expect empty-corpus 0 'no corpus configured; the gate is disabled' "$rc" "$out"
out="$( cd "$SANDBOX/repo" && run '' . )"; rc=$?
expect empty-literals 0 'no harness literal configured; the gate is disabled' "$rc" "$out"
out="$( cd "$SANDBOX/repo" && run '' '' )"; rc=$?
expect both-empty 0 'no harness literal and no corpus configured' "$rc" "$out"
if grep -qF 'file(s) scanned' <<<"$out"; then
    echo "  FAIL [both-empty]: the disabled verdict must not read as a nothing-found one -- $out"
    fails=$((fails + 1))
fi

out="$( cd "$SANDBOX/repo" && run DEMO_HARNESS_DIR . )"; rc=$?
expect knobs-live 1 'prose.md:1:' "$rc" "$out"

out="$( cd "$SANDBOX/repo" && run 'DEMO_HARNESS_.*' . )"; rc=$?
expect fixed-string 0 'none names one of the 1 harness literal(s)' "$rc" "$out"

printf 'ELF\x00\x01 DEMO_HARNESS_DIR\x00more\n' > "$SANDBOX/repo/artifact"
git -C "$SANDBOX/repo" add artifact
out="$( cd "$SANDBOX/repo" && run DEMO_HARNESS_DIR artifact )"; rc=$?
expect binary-skipped 0 '1 binary member(s) skipped' "$rc" "$out"

quoted="$(printf 'na\303\257ve.md')"
mkdir -p "$SANDBOX/repo/quoted"
printf 'Anchor at DEMO_HARNESS_DIR.\n' > "$SANDBOX/repo/quoted/$quoted"
git -C "$SANDBOX/repo" add quoted
out="$( cd "$SANDBOX/repo" && run DEMO_HARNESS_DIR quoted )"; rc=$?
expect quoted-name 1 "quoted/$quoted:1:" "$rc" "$out"

out="$( cd "$SANDBOX/bare" && GIT_CEILING_DIRECTORIES="$SANDBOX" run DEMO_HARNESS_DIR . )"; rc=$?
expect non-repository 2 'git-ls-files' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-harness-literal.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-harness-literal.test.sh: clean (the arms a case dir cannot reach: each empty knob disabling the gate in its own sentence, the same knobs finding the literal once set, a literal read as a fixed string, a binary member skipped-and-counted, a member whose name git quotes read under its real spelling, and a non-repository working directory failing closed — 9 assertions over 8 cases)"
exit 0
