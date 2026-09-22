#!/usr/bin/env bash
# Direct unit test of check-stamp-subject — the cases one good/bad pair cannot
# carry: a boundary reset (the truncation leaves one added line), a commit adding
# no stamp, the rename subject, a merge whose stamps a parent carries, and the
# no-arg, missing-file and wrong-arity edges.
# The gate is named, never its substrate: gate_run resolves the declaration path.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

hdr=$'# contract: x\n---\n'
printf '%s\nold scope s1 2026-01-01 1111111\nold close s2 2026-01-02 2222222\n' "$hdr" > "$tmp/closed.txt"
printf '%s\nnext scope s3 2026-01-03 3333333\n' "$hdr" > "$tmp/reset.txt"
printf '%s\nnamed scope s3 2026-01-03 3333333\n' "$hdr" > "$tmp/renamed.txt"
printf '%s\nnext scope s3 2026-01-03 3333333\nnext build s4 2026-01-04 4444444\n' "$hdr" > "$tmp/side.txt"

run() {  # $1=subject $2=staged $3=head [$4...=merge parents]
    printf '%s\n' "$1" > "$tmp/msg.txt"
    (gate_run check-stamp-subject "$DIR/checks" "$tmp/msg.txt" "${@:2}" >/dev/null 2>&1)
    echo $?
}
expect() {  # $1=want $2=got $3=what
    [[ "$2" -eq "$1" ]] || { echo "  FAIL: $3 — expected exit $1, got $2"; fails=$((fails + 1)); }
}

expect 0 "$(run 'chore(scope): stamp the scope stage entry at the iteration boundary' "$tmp/reset.txt" "$tmp/closed.txt")" \
    "a boundary reset scoped to the first stage"
expect 1 "$(run 'chore(close): stamp the scope stage entry' "$tmp/reset.txt" "$tmp/closed.txt")" \
    "a boundary reset scoped to the prior iteration's last stage"
expect 0 "$(run 'chore(scope): name the iteration named' "$tmp/renamed.txt" "$tmp/reset.txt")" \
    "a rename scoped to the rewritten stamp's stage"
expect 0 "$(run 'feat(lifecycle-kit): unit work' "$tmp/closed.txt" "$tmp/closed.txt")" \
    "a commit adding no stamp keeps its component scope"
expect 1 "$(run 'chore: stamp the scope stage entry' "$tmp/reset.txt" "$tmp/closed.txt")" \
    "an unscoped subject on a stamp commit"
expect 0 "$(run "Merge branch 'side'" "$tmp/side.txt" "$tmp/reset.txt" "$tmp/side.txt")" \
    "a merge whose added stamp a merge parent carries"
expect 1 "$(run "Merge branch 'side'" "$tmp/side.txt" "$tmp/reset.txt" "$tmp/reset.txt")" \
    "a merge adding a stamp no parent carries"

(gate_run check-stamp-subject "$DIR/checks" >/dev/null 2>&1); rc=$?
expect 0 "$rc" "no-arg run clean-skips"
(gate_run check-stamp-subject "$DIR/checks" "$tmp/does-not-exist.txt" >/dev/null 2>&1); rc=$?
expect 2 "$rc" "a missing message file fails closed"
(gate_run check-stamp-subject "$DIR/checks" "$tmp/msg.txt" "$tmp/reset.txt" >/dev/null 2>&1); rc=$?
expect 2 "$rc" "two arguments is a usage error"

if [[ "$fails" -gt 0 ]]; then
    echo "check-stamp-subject.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-stamp-subject.test: ok (boundary reset; rename; no stamp; unscoped; merge parents; edges)"
exit 0
