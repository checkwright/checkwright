#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §check-door-binding — assertion C's two fail-closed exits. The one-pair
# fixture harness cannot spell either: both are exit 2 on a corpus the gate could not read, and a
# pair asserts exit 0 against exit 1.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

CHECKS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../checks" && pwd)"

fails=0
checks=0
tmp="$(mktemp -d)"
trap 'chmod -R u+rwX "$tmp" 2>/dev/null; rm -rf "$tmp"' EXIT

# A sandbox repository with one kit root, so assertions A and B have a corpus
# they pass on and only C's configured entry decides the exit.
mkdir -p "$tmp/alpha-kit"
printf '# alpha-kit\n\nNothing here names a door.\n' >"$tmp/alpha-kit/README.md"
printf 'GATE_SDK_KIT_DIRS = alpha-kit\n' >"$tmp/gate-sdk.knobs"
git -C "$tmp" init -q
git -C "$tmp" add -A
git -C "$tmp" -c user.email=t@t -c user.name=t commit -qm seed

want() {  # $1=label $2=roots-entry $3=want-rc $4=want-substring
    checks=$((checks + 1))
    local out rc
    printf 'GUARD_KIT_DOOR_ROOTS[] = %s\n' "$2" >"$tmp/guard.knobs"
    out="$(
        cd "$tmp" || exit 2
        gate_env "GUARD_KIT_KNOB_FILE=$tmp/guard.knobs" "GATE_SDK_KNOB_FILE=$tmp/gate-sdk.knobs"
        gate_run check-door-binding "$CHECKS_DIR" . 2>&1
    )"
    rc=$?
    if [[ "$rc" != "$3" ]]; then
        echo "  FAIL [$1]: entry '$2' gave rc=$rc, want $3 — $out"
        fails=$((fails + 1))
    elif [[ "$out" != *"$4"* ]]; then
        echo "  FAIL [$1]: the output did not name '$4': $out"
        fails=$((fails + 1))
    fi
}

# --- the control: a tracked file entry resolves and the gate runs, which is what
# makes the two refusals below about the entry rather than about the file form
want "tracked-file-entry" "alpha-kit/README.md" 0 "configured surface(s) swept"

# --- a configured entry that is neither a tracked file nor a directory
want "absent-entry" "no/such/page.md" 2 "resolves to neither a tracked file nor a directory"

# --- present on disk but untracked is the same refusal: the corpus is the tracked
# tree, so an untracked entry is a misconfiguration and never a vacuous clean
printf 'untracked\n' >"$tmp/loose.md"
want "untracked-entry" "loose.md" 2 "resolves to neither a tracked file nor a directory"

# --- an unreadable configured surface: exit 2 rather than a clean over a corpus
# the gate could not read. Skipped for a caller that reads a mode-000 file anyway.
mkdir -p "$tmp/locked"
printf 'bash gate-sdk/bin/run-gates.sh --run-demo\n' >"$tmp/locked/page.md"
git -C "$tmp" add -A
git -C "$tmp" -c user.email=t@t -c user.name=t commit -qm locked
chmod 000 "$tmp/locked/page.md"
if head -c1 "$tmp/locked/page.md" >/dev/null 2>&1; then
    echo "  note: skipping the unreadable-surface case — this caller reads a mode-000 file"
else
    want "unreadable-surface" "locked" 2 "unreadable corpus file"
fi
chmod 644 "$tmp/locked/page.md"

echo "door-binding-corpus: $checks check(s), $fails failure(s)"
[[ "$fails" -eq 0 ]]
