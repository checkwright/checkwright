#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the iteration-boundary gap-inbox check end-to-end through a sandboxed enter-stage: one detector, two dispositions. The close-skipped branch refuses with the drain recovery and writes nothing; the post-close branch stamps, carries the bullets and leaves the inbox intact; the never-named and no-cursor edges take the post-close branch; and --simulate reports each branch with its recovery relayed.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
ENTER="$DIR/bin/enter-stage.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir  $2=header iteration  $3=state data lines (may be empty)
    local sb="$SANDBOX/$1"
    mkdir -p "$sb/.workflow" "$sb/scratch"
    cat >"$sb/TASK-QUEUE.md" <<EOF
# TASK-QUEUE.md

## Iteration: $2

---

## New Features

## Technical Debt

## Done
EOF
    {
        printf '# contract: lifecycle-kit/SPEC.md §check-stage-evidence\n\n---\n\n'
        printf '%s' "$3"
    } >"$sb/.workflow/WORKFLOW-STATE.txt"
    printf '# contract: lifecycle-kit/SPEC.md §The committed gap inbox\n- 2026-06-05 — the first untriaged gap\n- 2026-06-06 — the second\n' \
        >"$sb/.workflow/gap-inbox.md"
    echo "$sb"
}

run_enter() {  # $1=sandbox subdir, rest=argv
    ( cd "$1" && env GATE_SDK_TMP_DIR=scratch \
                     LIFECYCLE_KIT_SESSION_ID=deadbeef03 \
                     bash "$ENTER" "${@:2}" 2>&1 )
}

CLOSED='demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration close dddddddd 2026-06-04 none
'
SKIPPED='demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
'

# --- close-skipped: refuse with the drain recovery, write nothing ---
sk="$(seed skipped demo-iteration "$SKIPPED")"
cp "$sk/.workflow/WORKFLOW-STATE.txt" "$sk/s.before"
cp "$sk/TASK-QUEUE.md" "$sk/q.before"
out="$(run_enter "$sk" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note skipped-refuses "want exit 1 on a close-skipped boundary, got $rc -- $out"
grep -qF 'refused' <<<"$out"                        || note skipped-verdict "the refusal does not say it refused: $out"
grep -qF "run the closing stage's gap-drain step" <<<"$out" \
    || note skipped-recovery "the close-skipped refusal withheld the drain recovery: $out"
grep -qF 'disposition them here' <<<"$out"          && note skipped-wrong-recovery "the close-skipped refusal offered the post-close recovery: $out"
cmp -s "$sk/s.before" "$sk/.workflow/WORKFLOW-STATE.txt" || note skipped-state "a refused entry wrote the state file"
cmp -s "$sk/q.before" "$sk/TASK-QUEUE.md"               || note skipped-queue "a refused entry wrote the queue"

# --- close-skipped under --simulate: the branch and its recovery both relay ---
out="$(run_enter "$sk" --simulate scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note skipped-sim-rc "want exit 1 from --simulate on a close-skipped boundary, got $rc -- $out"
grep -qF 'would be refused' <<<"$out" || note skipped-sim-verdict "--simulate did not report a would-be refusal: $out"
grep -qF "enter-stage (simulate):   help:" <<<"$out" \
    || note skipped-sim-help "--simulate relayed the refusal without its recovery: $out"
cmp -s "$sk/s.before" "$sk/.workflow/WORKFLOW-STATE.txt" || note skipped-sim-write "--simulate wrote the state file"

# --- post-close: admit, stamp, carry the bullets, leave the inbox alone ---
pc="$(seed postclose demo-iteration "$CLOSED")"
out="$(run_enter "$pc" scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note postclose-admits "want exit 0 on a post-close boundary, got $rc -- $out"
grep -qF 'intake' <<<"$out"       || note postclose-advisory "the admission printed no intake advisory: $out"
grep -qF '2026-06-05' <<<"$out"   || note postclose-names "the advisory did not name the carried bullet: $out"
grep -qF '2026-06-06' <<<"$out"   || note postclose-names-all "the advisory did not name every carried bullet: $out"
grep -qF 'refused' <<<"$out"      && note postclose-verdict "the admission reported a refusal: $out"
grep -q '^— scope deadbeef 2026' "$pc/.workflow/WORKFLOW-STATE.txt" \
    || note postclose-stamp "the admitted entry did not stamp"
grep -q '^## Iteration: —$' "$pc/TASK-QUEUE.md" || note postclose-reset "the admitted entry did not reset the header"
[[ "$(grep -c '^- ' "$pc/.workflow/gap-inbox.md")" -eq 2 ]] \
    || note postclose-inbox "the admitted entry consumed or reshaped the carried bullets — the entering session dispositions them"

# --- post-close under --simulate: reports the branch it would take, and writes nothing ---
sm="$(seed postclose-sim demo-iteration "$CLOSED")"
cp "$sm/.workflow/WORKFLOW-STATE.txt" "$sm/s.before"
out="$(run_enter "$sm" --simulate scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note postclose-sim-rc "want exit 0 from --simulate on a post-close boundary, got $rc -- $out"
grep -qF 'would not refuse' <<<"$out" || note postclose-sim-branch "--simulate did not report the admitting branch: $out"
grep -qF 'would carry 2 bullet(s)' <<<"$out" \
    || note postclose-sim-count "--simulate did not name how many bullets it would carry: $out"
cmp -s "$sm/s.before" "$sm/.workflow/WORKFLOW-STATE.txt" || note postclose-sim-write "--simulate wrote the state file"

# --- the never-named closing iteration takes the post-close branch ---
un="$(seed unnamed '—' '— scope aaaaaaaa 2026-06-01 none
')"
out="$(run_enter "$un" scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note unnamed-admits "a never-named closing iteration should not refuse, got $rc -- $out"
grep -qF 'intake' <<<"$out" || note unnamed-advisory "the never-named edge printed no intake advisory: $out"

# --- no cursor at all (a fresh consumer's first boundary) takes it too ---
nc="$(seed nocursor fresh-iteration '')"
out="$(run_enter "$nc" scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note nocursor-admits "a boundary with no cursor should not refuse, got $rc -- $out"
grep -qF 'intake' <<<"$out" || note nocursor-advisory "the no-cursor edge printed no intake advisory: $out"

# --- an empty inbox is silent on both branches ---
ei_n=0
for st in "$CLOSED" "$SKIPPED"; do
    ei_n=$((ei_n + 1))
    ei="$(seed "empty-$ei_n" demo-iteration "$st")"
    printf '# contract: lifecycle-kit/SPEC.md §The committed gap inbox\n' >"$ei/.workflow/gap-inbox.md"
    out="$(run_enter "$ei" scope)"; rc=$?
    [[ "$rc" -eq 0 ]] || note empty-inbox "a header-only inbox should not refuse either branch, got $rc -- $out"
    grep -qF 'gap-inbox.md holds' <<<"$out" && note empty-advisory "a header-only inbox printed a carried-bullet advisory: $out"
done

[[ "$fails" -eq 0 ]] || { echo "gap-inbox-route.test: $fails assertion(s) failed"; exit 1; }
echo "gap-inbox-route.test: clean (close-skipped refuses with the drain recovery and writes nothing; post-close stamps and carries; both edges take the post-close branch; --simulate reports each branch with its recovery)"
exit 0
