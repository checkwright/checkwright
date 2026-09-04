#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --rename end-to-end through a sandboxed enter-stage: both surfaces rewritten in one motion, the columns-2-to-last witness proved rather than assumed, the half-landed hand-rename healed, every refusal writing nothing, the idempotent no-op, and --simulate --rename writing nothing while prefixing every line
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the second sanctioned caller: this harness
# drives the arm from a non-git sandbox cwd, which bin/run-gates.sh refuses by design (it cds to
# the git toplevel and a `mktemp -d` is no repository), so it resolves the binary and the bridged
# environment through gate_arm_run rather than through that front-end.
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir  $2=header name  $3=stamp column-1 name
    local sb="$1" hdr="$2" col1="$3"
    mkdir -p "$sb/.workflow"
    cat >"$sb/TASK-QUEUE.md" <<EOF
# TASK-QUEUE.md

## Iteration: $hdr

## New Features

## Technical Debt

## Done
EOF
    cat >"$sb/.workflow/WORKFLOW-STATE.txt" <<EOF
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

$col1 scope aaaaaaaa 2026-06-01 none
$col1 build bbbbbbbb 2026-06-02 none
$col1 validate cccccccc 2026-06-03 none
EOF
    cp "$sb/TASK-QUEUE.md" "$sb/queue.before"
    cp "$sb/.workflow/WORKFLOW-STATE.txt" "$sb/state.before"
}

run_rename() {  # $1=sandbox subdir, rest = argv after the tool name
    local sb="$1"; shift
    ( cd "$sb" && gate_env GATE_SDK_TMP_DIR="$sb/scratch" && gate_arm_run --enter-stage "$@" 2>&1 )
}

fields_2_to_nf() { awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { s = ""; for (i = 2; i <= NF; i++) s = s (i > 2 ? " " : "") $i; print s }' "$1"; }

unchanged() {  # $1=sandbox subdir  $2=assertion label
    cmp -s "$1/queue.before" "$1/TASK-QUEUE.md" || note "$2" "the queue was written"
    cmp -s "$1/state.before" "$1/.workflow/WORKFLOW-STATE.txt" || note "$2" "the state file was written"
}

# --- both surfaces rewritten in one motion, columns 2 through NF intact ---
ok="$SANDBOX/ok"
seed "$ok" old-name old-name
pre="$(fields_2_to_nf "$ok/state.before")"
out="$(run_rename "$ok" --rename new-name)"; rc=$?
[[ "$rc" -eq 0 ]] || note write "want exit 0, got $rc -- $out"
grep -q '^## Iteration: new-name$' "$ok/TASK-QUEUE.md" || note write-header "the queue header was not renamed"
[[ "$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF && $1 != "new-name" { c++ } END { print c + 0 }' \
    "$ok/.workflow/WORKFLOW-STATE.txt")" == 0 ]] \
    || note write-stamps "a stamp's column 1 was left at the old name"
[[ "$pre" == "$(fields_2_to_nf "$ok/.workflow/WORKFLOW-STATE.txt")" ]] \
    || note witness "columns 2 through NF (stage, session id, date, head) moved under the rename"
[[ "$(wc -l <"$ok/state.before")" == "$(wc -l <"$ok/.workflow/WORKFLOW-STATE.txt")" ]] \
    || note witness-shape "the rename changed the state file's line count"
grep -qF 'commit' <<<"$out" || note report "the report does not tell the caller to commit both files: $out"
grep -qF 'TASK-QUEUE.md' <<<"$out" || note report-queue "the report does not name the queue: $out"
grep -qF 'WORKFLOW-STATE.txt' <<<"$out" || note report-state "the report does not name the state file: $out"

# --- the half-landed hand-rename heals from either side ---
for half in header stamps; do
    hb="$SANDBOX/half-$half"
    if [[ "$half" == header ]]; then seed "$hb" new-name old-name; else seed "$hb" old-name new-name; fi
    out="$(run_rename "$hb" --rename new-name)"; rc=$?
    [[ "$rc" -eq 0 ]] || note "heal-$half" "want exit 0, got $rc -- $out"
    grep -q '^## Iteration: new-name$' "$hb/TASK-QUEUE.md" || note "heal-$half-header" "header not healed"
    [[ "$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF && $1 != "new-name" { c++ } END { print c + 0 }' \
        "$hb/.workflow/WORKFLOW-STATE.txt")" == 0 ]] || note "heal-$half-stamps" "stamps not healed"
done

# --- the idempotent no-op ---
idem="$SANDBOX/idempotent"
seed "$idem" same-name same-name
out="$(run_rename "$idem" --rename same-name)"; rc=$?
[[ "$rc" -eq 0 ]] || note idempotent "want exit 0, got $rc -- $out"
grep -qF 'idempotent no-op' <<<"$out" || note idempotent-report "the no-op was not reported as one: $out"
unchanged "$idem" idempotent-write

# --- every refusal writes nothing ---
ref="$SANDBOX/refusals"
for bad in "" "two words" "—" "Upper-Case" "-leading-dash"; do
    rm -rf "$ref"; seed "$ref" old-name old-name
    out="$(run_rename "$ref" --rename "$bad")"; rc=$?
    [[ "$rc" -eq 2 ]] || note "refuse[$bad]" "want exit 2, got $rc -- $out"
    unchanged "$ref" "refuse-write[$bad]"
done
rm -rf "$ref"; seed "$ref" old-name old-name
out="$(run_rename "$ref" --rename a b)"; rc=$?
[[ "$rc" -eq 2 ]] || note refuse-arity "--rename with two names: want exit 2, got $rc -- $out"
unchanged "$ref" refuse-arity-write

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the placeholder refusal precedes the slug grammar so its message names the writer that owns the value, not a malformed name
rm -rf "$ref"; seed "$ref" old-name old-name
out="$(run_rename "$ref" --rename "—")"
grep -qF 'boundary reset' <<<"$out" \
    || note refuse-placeholder-message "the placeholder refusal did not name the boundary reset: $out"

# --- a pre-flight refusal writes nothing: a stamp naming a stage the machine does not know ---
pf="$SANDBOX/preflight"
seed "$pf" old-name old-name
printf 'old-name notastage eeeeeeee 2026-06-05 none\n' >>"$pf/.workflow/WORKFLOW-STATE.txt"
cp "$pf/.workflow/WORKFLOW-STATE.txt" "$pf/state.before"
out="$(run_rename "$pf" --rename new-name)"; rc=$?
[[ "$rc" -eq 1 ]] || note preflight "a check-stage-evidence refusal: want exit 1, got $rc -- $out"
grep -qF 'STAGE-EVIDENCE' <<<"$out" || note preflight-relay "the gate's output was not relayed: $out"
unchanged "$pf" preflight-write

# --- --simulate --rename writes nothing and prefixes every line ---
sim="$SANDBOX/simulate"
seed "$sim" old-name old-name
out="$(run_rename "$sim" --simulate --rename new-name)"; rc=$?
[[ "$rc" -eq 0 ]] || note simulate "want exit 0, got $rc -- $out"
unchanged "$sim" simulate-write
if grep -qv '^enter-stage (simulate): ' <<<"$out"; then
    note simulate-prefix "an unprefixed line escaped --simulate: $out"
fi
grep -qF 'new-name' <<<"$out" || note simulate-relay "the simulate report does not name the new name: $out"

simn="$SANDBOX/simulate-noop"
seed "$simn" same-name same-name
out="$(run_rename "$simn" --simulate --rename same-name)"; rc=$?
[[ "$rc" -eq 0 ]] || note simulate-noop "want exit 0, got $rc -- $out"
grep -qF 'idempotent no-op' <<<"$out" || note simulate-noop-report "the would-be no-op was not reported: $out"
if grep -qv '^enter-stage (simulate): ' <<<"$out"; then
    note simulate-noop-prefix "the would-be no-op line is unprefixed: $out"
fi
unchanged "$simn" simulate-noop-write

[[ "$fails" -eq 0 ]] || { echo "rename-iteration.test: $fails assertion(s) failed"; exit 1; }
echo "rename-iteration.test: clean (--rename writes both surfaces with columns 2 through NF proved unchanged, heals a half-landed rename from either side, no-ops idempotently, refuses empty/non-slug/placeholder/arity and a red pre-flight with nothing written, and simulates read-only under its prefix)"
exit 0
