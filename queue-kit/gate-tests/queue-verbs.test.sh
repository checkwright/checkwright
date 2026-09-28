#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §The queue verbs — the verbs over a seeded history, the properties a crate
# unit test cannot reach because they are properties of a git history and of a spawned post-check:
# (1) `promote` and `icebox` build the history a later `demote` and `thaw` read back, restoring the
# position, the board tags and the body; (2) `--emit queue-history` prints the same history's rows,
# oldest first; (3) the exit contract — 1 on a planted post-check red with the move written, 2 on a
# refusal with the file byte-for-byte unchanged.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

git -C "$SANDBOX" init -q
git -C "$SANDBOX" config user.email seed@example.invalid
git -C "$SANDBOX" config user.name Seed

# The post-check's registry: one stub gate coupled to the queue file, red while a marker exists.
mkdir -p "$SANDBOX/gates"
{
    printf '#!/usr/bin/env bash\n'
    printf '# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit\n'
    printf '[[ -e "%s/RED" ]] && { echo "g_queue: planted red"; exit 1; }\n' "$SANDBOX"
    printf 'echo "G-QUEUE: clean"\n'
} >"$SANDBOX/gates/g_queue.sh"
chmod +x "$SANDBOX/gates/g_queue.sh"
echo g_queue >"$SANDBOX/gates/gates.list"

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: seeded

## New Features

## Technical Debt

## Deferred

### def-a

[cost: event/low] [surface: gates]

the first entry.

### def-b

[cost: iteration/high] [surface: gates]

the promoted entry.

### def-c

[cost: once/low] [surface: gates]

the evicted entry, with a body the icebox drops.

a second paragraph.

### def-d

[cost: event/low] [surface: gates]

the last entry.

## Icebox

## Done

## Lessons Learned
EOF

commit() {
    git -C "$SANDBOX" add TASK-QUEUE.md
    GIT_AUTHOR_DATE="$2T12:00:00" GIT_COMMITTER_DATE="$2T12:00:00" git -C "$SANDBOX" commit -q -m "$1"
}
sha() { git -C "$SANDBOX" rev-parse --short=8 "$1"; }

verb() {
    ( cd "$SANDBOX" && gate_env GATE_SDK_GATES_DIR="$SANDBOX/gates" GATE_SDK_KIT_DIRS="$SANDBOX/gates" \
        GATE_SDK_TMP_DIR="$SANDBOX/.tmp" QUEUE_KIT_ICEBOX_SECTION=Icebox && gate_arm_run --queue "$@" 2>&1 )
}
history() {
    ( cd "$SANDBOX" && gate_env QUEUE_KIT_ICEBOX_SECTION=Icebox && gate_arm_run --emit-queue-history "$@" 2>&1 )
}

# --- the seeded history, built by the verbs themselves ----------------------------------
commit "file four entries" 2026-01-01
FILED="$(sha HEAD)"

checks=$((checks + 1))
out="$(verb promote def-b "New Features" --spec SPEC-b.md)"; rc=$?
[[ "$rc" -eq 0 ]] || note promote-exit "promote did not exit 0 under a green post-check (exit $rc): $out"
grep -qF 'promote def-b: Deferred -> New Features; dropped [cost: iteration/high] [surface: gates]; added [spec: SPEC-b.md]' <<<"$out" \
    || note promote-act "the act line is not the one the move made: $out"
grep -qF 'G-QUEUE: clean' <<<"$out" || grep -qF 'All 1 gates passed' <<<"$out" \
    || note promote-postcheck "the post-check's output is absent: $out"
commit "promote def-b" 2026-02-01
PROMOTED="$(sha HEAD)"

checks=$((checks + 1))
out="$(verb icebox def-c "dormant c.")"; rc=$?
[[ "$rc" -eq 0 ]] || note icebox-exit "icebox did not exit 0 (exit $rc): $out"
grep -qF 'a second paragraph.' "$SANDBOX/TASK-QUEUE.md" && note icebox-body "the evicted body is still in the file"
commit "evict def-c" 2026-03-01
EVICTED="$(sha HEAD)"

# --- queue-history prints the same history, oldest first ---------------------------------
checks=$((checks + 1))
h="$(history def-b)"; rc=$?
[[ "$rc" -eq 0 ]] || note history-exit "queue-history did not exit 0 (exit $rc): $h"
want="def-b
  2026-01-01 $FILED (absent) -> Deferred  file four entries
  2026-02-01 $PROMOTED Deferred -> New Features  promote def-b"
[[ "$h" == "$want" ]] || note history-rows "the rows are not the seeded history's:
$h"
checks=$((checks + 1))
h="$(history def-c)"
grep -qF "  2026-03-01 $EVICTED Deferred -> Icebox  evict def-c" <<<"$h" \
    || note history-evict "the eviction row is absent: $h"
none="$(history no-such-slug)"; rc=$?
[[ "$rc" -eq 2 ]] || note history-refusal "a slug in no set exited $rc rather than 2: $none"

# --- demote restores the board tags and the place after its old predecessor ---------------
checks=$((checks + 1))
out="$(verb demote def-b)"; rc=$?
[[ "$rc" -eq 0 ]] || note demote-exit "demote did not exit 0 (exit $rc): $out"
grep -qF 'the first entry.

### def-b

[cost: iteration/high] [surface: gates]

the promoted entry.' "$SANDBOX/TASK-QUEUE.md" \
    || note demote-restore "def-b is not back after def-a with its board tags: $(cat "$SANDBOX/TASK-QUEUE.md")"

# --- thaw restores the whole body at its place and stamps the recurrence ------------------
checks=$((checks + 1))
out="$(verb thaw def-c --date 2026-04-01)"; rc=$?
[[ "$rc" -eq 0 ]] || note thaw-exit "thaw did not exit 0 (exit $rc): $out"
grep -qF 'the first entry.

### def-c

[cost: once/low] [surface: gates] [recurrence: 2026-04-01]

the evicted entry, with a body the icebox drops.

a second paragraph.' "$SANDBOX/TASK-QUEUE.md" \
    || note thaw-restore "def-c's body is not restored after def-a: $(cat "$SANDBOX/TASK-QUEUE.md")"
grep -qF 'dormant c.' "$SANDBOX/TASK-QUEUE.md" && note thaw-sentence "the icebox sentence survived the thaw"

# --- the exit contract: 1 written under a planted red, 2 with the file unchanged ----------
checks=$((checks + 1))
touch "$SANDBOX/RED"
cp "$SANDBOX/TASK-QUEUE.md" "$SANDBOX/before.md"
out="$(verb recur def-d 2026-05-01)"; rc=$?
[[ "$rc" -eq 1 ]] || note red-exit "a planted post-check red exited $rc rather than 1: $out"
grep -qF 'g_queue: planted red' <<<"$out" || note red-findings "the post-check's findings are not printed: $out"
cmp -s "$SANDBOX/before.md" "$SANDBOX/TASK-QUEUE.md" && note red-written "the move was not written under the red post-check"
rm -f "$SANDBOX/RED"

checks=$((checks + 1))
cp "$SANDBOX/TASK-QUEUE.md" "$SANDBOX/before.md"
for refusal in "promote no-such-slug New Features" "recur def-d 2026-04-30" "icebox def-a" "done"; do
    # shellcheck disable=SC2086 # each refusal is a word list on purpose
    out="$(verb $refusal)"; rc=$?
    [[ "$rc" -eq 2 ]] || note refusal-exit "'$refusal' exited $rc rather than 2: $out"
    cmp -s "$SANDBOX/before.md" "$SANDBOX/TASK-QUEUE.md" || note refusal-unchanged "'$refusal' changed the file"
done

if [[ "$fails" -gt 0 ]]; then
    echo "queue-verbs.test.sh: $fails case(s) failed"
    exit 1
fi
echo "queue-verbs.test.sh: clean (promote and icebox build a history that demote and thaw read back — position, board tags and body restored — queue-history prints its rows oldest first and refuses a slug in no set, a planted post-check red exits 1 with the move written, and a refusal exits 2 with the file unchanged; $checks checks)"
exit 0
