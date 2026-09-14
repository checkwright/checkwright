#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --open-lead-journal end-to-end through a sandboxed enter-stage: the heading keyed on fields 2 through the last of the last stamp, creation with its directory, append after an undisposed segment, a disposed segment dropped, the idempotent no-op, --simulate writing nothing, the surplus refusal, the none key on a stampless state file, and no stamp ever appended
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the second sanctioned caller: this harness
# drives the arm from a non-git sandbox cwd, which bin/run-gates.sh refuses by design (it cds to
# the git toplevel and a `mktemp -d` is no repository), so it resolves the binary through
# gate_arm_run rather than through that front-end.
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

KEY='build bbbbbbbb 2026-06-02 none'

seed() {  # $1=sandbox subdir  $2=stamps|stampless
    local sb="$1"
    mkdir -p "$sb/.workflow"
    printf '# TASK-QUEUE.md\n\n## Iteration: demo\n\n## Done\n' >"$sb/TASK-QUEUE.md"
    printf '# contract: lifecycle-kit/SPEC.md §check-stage-evidence\n\n---\n\n' >"$sb/.workflow/WORKFLOW-STATE.txt"
    if [[ "$2" == stamps ]]; then
        printf 'demo scope aaaaaaaa 2026-06-01 none\ndemo %s\n' "$KEY" >>"$sb/.workflow/WORKFLOW-STATE.txt"
    fi
    cp "$sb/TASK-QUEUE.md" "$sb/queue.before"
    cp "$sb/.workflow/WORKFLOW-STATE.txt" "$sb/state.before"
}

run_open() {  # $1=sandbox subdir, rest = argv after the tool name
    local sb="$1"; shift
    ( cd "$sb" && gate_env GATE_SDK_TMP_DIR="$sb/scratch" && gate_arm_run --enter-stage "$@" 2>&1 )
}

untracked_only() {  # $1=sandbox subdir  $2=assertion label
    cmp -s "$1/queue.before" "$1/TASK-QUEUE.md" || note "$2" "the queue was written"
    cmp -s "$1/state.before" "$1/.workflow/WORKFLOW-STATE.txt" || note "$2" "the state file was written"
}

# --- creation, with its directory, under the cursor's key ---
cr="$SANDBOX/create"
seed "$cr" stamps
out="$(run_open "$cr" --open-lead-journal)"; rc=$?
[[ "$rc" -eq 0 ]] || note create "want exit 0, got $rc -- $out"
[[ "$(cat "$cr/scratch/lead-journal.md" 2>/dev/null)" == "## lead-journal opened after $KEY" ]] \
    || note create-heading "the journal is not exactly the keyed heading: $(cat "$cr/scratch/lead-journal.md" 2>/dev/null)"
untracked_only "$cr" create-untracked

# --- the idempotent no-op on a re-run at the same cursor ---
cp "$cr/scratch/lead-journal.md" "$cr/journal.before"
out="$(run_open "$cr" --open-lead-journal)"; rc=$?
[[ "$rc" -eq 0 ]] || note noop "want exit 0, got $rc -- $out"
cmp -s "$cr/journal.before" "$cr/scratch/lead-journal.md" || note noop-bytes "the re-run wrote the journal"
grep -qF 'idempotent no-op' <<<"$out" || note noop-report "the re-run did not report a no-op: $out"

# --- append after an undisposed segment, keeping it byte for byte ---
ap="$SANDBOX/append"
seed "$ap" stamps
mkdir -p "$ap/scratch"
printf '## lead-journal opened after close zzzzzzzz 2026-05-01 none\n\n## prior findings\nprose\n' >"$ap/scratch/lead-journal.md"
cp "$ap/scratch/lead-journal.md" "$ap/journal.before"
out="$(run_open "$ap" --open-lead-journal)"; rc=$?
[[ "$rc" -eq 0 ]] || note append "want exit 0, got $rc -- $out"
[[ "$(head -c "$(wc -c <"$ap/journal.before")" "$ap/scratch/lead-journal.md")" == "$(cat "$ap/journal.before")" ]] \
    || note append-kept "the undisposed prior segment was not kept as the file's prefix"
[[ "$(tail -n 1 "$ap/scratch/lead-journal.md")" == "## lead-journal opened after $KEY" ]] \
    || note append-last "the keyed heading is not the last line"

# --- a disposed segment is dropped, every other segment kept in order ---
dr="$SANDBOX/drop"
seed "$dr" stamps
mkdir -p "$dr/scratch"
printf '## lead-journal opened after close xxxxxxxx 2026-04-01 none\n\n## discharged\nDISPOSED\n\n## lead-journal opened after close zzzzzzzz 2026-05-01 none\n\n## undisposed\nprose\n' \
    >"$dr/scratch/lead-journal.md"
out="$(run_open "$dr" --open-lead-journal)"; rc=$?
[[ "$rc" -eq 0 ]] || note drop "want exit 0, got $rc -- $out"
grep -qF '## discharged' "$dr/scratch/lead-journal.md" && note drop-disposed "the disposed segment survived"
grep -qF '## undisposed' "$dr/scratch/lead-journal.md" || note drop-kept "an undisposed segment was dropped"
[[ "$(tail -n 1 "$dr/scratch/lead-journal.md")" == "## lead-journal opened after $KEY" ]] \
    || note drop-last "the keyed heading is not the last line"
grep -qF 'dropped' <<<"$out" || note drop-report "the report does not name the dropped segment: $out"

# --- --simulate writes nothing and prefixes its report ---
si="$SANDBOX/simulate"
seed "$si" stamps
mkdir -p "$si/scratch"
printf '## lead-journal opened after close xxxxxxxx 2026-04-01 none\nDISPOSED\n' >"$si/scratch/lead-journal.md"
cp "$si/scratch/lead-journal.md" "$si/journal.before"
out="$(run_open "$si" --simulate --open-lead-journal)"; rc=$?
[[ "$rc" -eq 0 ]] || note simulate "want exit 0, got $rc -- $out"
cmp -s "$si/journal.before" "$si/scratch/lead-journal.md" || note simulate-bytes "--simulate wrote the journal"
untracked_only "$si" simulate-untracked
grep -qvE '^enter-stage \(simulate\): ' <<<"$out" && note simulate-prefix "a --simulate line lacks the prefix: $out"
grep -qF "$KEY" <<<"$out" || note simulate-heading "--simulate does not relay the heading it would write: $out"
grep -qF 'would drop' <<<"$out" || note simulate-drop "--simulate does not relay the segment it would drop: $out"

# --- a surplus argument is refused, nothing written ---
for surplus in extra --simulate; do
    su="$SANDBOX/surplus-${surplus#--}"
    seed "$su" stamps
    out="$(run_open "$su" --open-lead-journal "$surplus")"; rc=$?
    [[ "$rc" -eq 2 ]] || note "surplus-$surplus" "want exit 2, got $rc -- $out"
    [[ -e "$su/scratch/lead-journal.md" ]] && note "surplus-$surplus-write" "a refused open wrote the journal"
    untracked_only "$su" "surplus-$surplus-untracked"
done
grep -qF -- '--enter-stage --simulate --open-lead-journal' <<<"$out" \
    || note surplus-spelling "a trailing --simulate is not told the flag-first spelling: $out"

# --- a stampless state file keys the heading on none ---
no="$SANDBOX/none"
seed "$no" stampless
out="$(run_open "$no" --open-lead-journal)"; rc=$?
[[ "$rc" -eq 0 ]] || note none "want exit 0, got $rc -- $out"
[[ "$(cat "$no/scratch/lead-journal.md" 2>/dev/null)" == "## lead-journal opened after none" ]] \
    || note none-heading "a stampless state file did not key on none: $(cat "$no/scratch/lead-journal.md" 2>/dev/null)"

[[ "$fails" -eq 0 ]] || { echo "lead-journal-open.test: $fails assertion(s) failed"; exit 1; }
echo "lead-journal-open.test: clean (--open-lead-journal creates its keyed heading, appends after an undisposed segment, drops a disposed one, no-ops at the same cursor, simulates read-only under its prefix, refuses a surplus argument with nothing written, keys on none without a stamp, and never stamps)"
exit 0
