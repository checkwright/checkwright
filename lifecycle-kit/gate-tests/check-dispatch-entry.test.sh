#!/usr/bin/env bash
# Direct unit test of check-dispatch-entry — the cases one good/bad pair cannot
# carry: the gap-inbox skip, an empty marker, the parallel two-line marker, and
# the no-arg, missing-file and wrong-arity edges.
# The gate is named, never its substrate: gate_run resolves the declaration path.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

hdr=$'# contract: x\n---\n'
printf '%s\nit scope s1 2026-01-01 1111111\n' "$hdr" > "$tmp/head.txt"
printf '%s\nit scope s1 2026-01-01 1111111\nit build s2 2026-01-02 2222222\n' "$hdr" > "$tmp/stamped.txt"
printf 'feat(lifecycle-kit): work\n' > "$tmp/msg.txt"
printf 'build\n' > "$tmp/one.txt"
printf 'build\nbuild\n' > "$tmp/two.txt"
: > "$tmp/empty.txt"
printf '.workflow/gap-inbox.md\n' > "$tmp/inbox-only.txt"
printf '.workflow/gap-inbox.md\nTASK-QUEUE.md\n' > "$tmp/inbox-plus.txt"
printf 'native/src/x.rs\n' > "$tmp/work.txt"

run() {  # $1=marker $2=staged $3=paths
    (gate_run check-dispatch-entry "$DIR/checks" "$tmp/msg.txt" "$1" "$2" "$tmp/head.txt" "$3" >/dev/null 2>&1)
    echo $?
}
expect() {  # $1=want $2=got $3=what
    [[ "$2" -eq "$1" ]] || { echo "  FAIL: $3 — expected exit $1, got $2"; fails=$((fails + 1)); }
}

expect 0 "$(run "$tmp/one.txt" "$tmp/head.txt" "$tmp/inbox-only.txt")" \
    "a commit staging the gap inbox alone is the lead's sanctioned commit"
expect 1 "$(run "$tmp/one.txt" "$tmp/head.txt" "$tmp/inbox-plus.txt")" \
    "the gap inbox beside another path is no longer the sanctioned commit"
expect 0 "$(run "$tmp/empty.txt" "$tmp/head.txt" "$tmp/work.txt")" \
    "an empty marker declares nothing"
expect 1 "$(run "$tmp/two.txt" "$tmp/head.txt" "$tmp/work.txt")" \
    "a parallel two-line marker reds a work commit adding no stamp"
expect 0 "$(run "$tmp/two.txt" "$tmp/stamped.txt" "$tmp/work.txt")" \
    "a commit adding a stamp clears a two-line marker"

(gate_run check-dispatch-entry "$DIR/checks" >/dev/null 2>&1); rc=$?
expect 0 "$rc" "no-arg run clean-skips"
(gate_run check-dispatch-entry "$DIR/checks" "$tmp/does-not-exist.txt" >/dev/null 2>&1); rc=$?
expect 2 "$rc" "a missing message file fails closed"
(gate_run check-dispatch-entry "$DIR/checks" "$tmp/msg.txt" "$tmp/one.txt" >/dev/null 2>&1); rc=$?
expect 2 "$rc" "two arguments is a usage error"
(gate_run check-dispatch-entry "$DIR/checks" "$tmp/msg.txt" "$tmp/nope.txt" "$tmp/head.txt" "$tmp/head.txt" "$tmp/work.txt" >/dev/null 2>&1); rc=$?
expect 2 "$rc" "an unreadable marker fails closed"

if [[ "$fails" -gt 0 ]]; then
    echo "check-dispatch-entry.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-dispatch-entry.test: ok (gap-inbox skip; empty marker; parallel lines; edges)"
exit 0
