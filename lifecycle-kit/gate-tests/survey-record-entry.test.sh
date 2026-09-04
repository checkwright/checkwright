#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The survey record — the two --enter-stage halves end-to-end through a sandboxed entry: an ordinary stage entry prints the record's headings and never its findings, and the iteration-boundary entry truncates the record to its contract header, names it in the truncated set, and does not refuse on a non-empty record the way the gap inbox one line above does
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

seed() {  # $1=sandbox subdir; a boundary-ready consumer carrying two filed surveys
    local sb="$1"
    mkdir -p "$sb/.workflow" "$sb/scratch"
    cat >"$sb/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

## Technical Debt

## Done
EOF
    cat >"$sb/.workflow/WORKFLOW-STATE.txt" <<'EOF'
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
demo-iteration validate cccccccc 2026-06-03 none
demo-iteration close dddddddd 2026-06-04 none
EOF
    cat >"$sb/.workflow/survey-record.md" <<'EOF'
# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys.

## 2026-06-01 scope — which gates meet every port criterion?
- corpus: checks/
- oracle: bash run-gates.sh
- rev: 0123456789abcdef0123456789abcdef01234567
- edges: check-gate-substrate-parity 4
- finding: FINDING-BODY-MUST-NOT-BE-PRINTED

## 2026-06-02 build — which specs cite the retired knob?
- corpus: */SPEC.md
- oracle: none
- rev: 89abcdef0123456789abcdef0123456789abcdef
- edges: none
- finding: SECOND-FINDING-BODY
EOF
}

run_enter() {  # $1=sandbox subdir  $2=stage
    ( cd "$1" && gate_env GATE_SDK_TMP_DIR=scratch \
                          LIFECYCLE_KIT_SESSION_ID=deadbeef02 \
        && gate_arm_run --enter-stage "$2" 2>&1 )
}

# --- an ordinary stage entry: the read trigger prints questions, never findings ---
ord="$SANDBOX/ordinary"
seed "$ord"
before="$(cat "$ord/.workflow/survey-record.md")"
out="$(run_enter "$ord" build)"; rc=$?
[[ "$rc" -eq 0 ]] || note ordinary-entry "want exit 0, got $rc -- $out"
grep -qF 'which gates meet every port criterion?' <<<"$out" \
    || note read-trigger "the entry report names no survey heading: $out"
grep -qF 'which specs cite the retired knob?' <<<"$out" \
    || note read-trigger-second "the entry report names only one of two headings: $out"
grep -qF 'FINDING-BODY-MUST-NOT-BE-PRINTED' <<<"$out" \
    && note read-trigger-findings "the entry report printed a finding body ahead of its witness: $out"
[[ "$before" == "$(cat "$ord/.workflow/survey-record.md")" ]] \
    || note ordinary-write "an ordinary stage entry modified the survey record"

# --- the iteration-boundary entry truncates it, names it, and does not refuse ---
bnd="$SANDBOX/boundary"
seed "$bnd"
out="$(run_enter "$bnd" scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note boundary-entry "a non-empty survey record refused the boundary (exit $rc) -- $out"
body="$(grep -v '^#' "$bnd/.workflow/survey-record.md" | grep -c '[^[:space:]]')"
[[ "$body" -eq 0 ]] || note boundary-truncate "the boundary left $body body line(s) in the survey record"
head -n1 "$bnd/.workflow/survey-record.md" | grep -q '^# contract: ' \
    || note boundary-header "the boundary truncate did not keep the contract header"
tail -n1 "$bnd/.workflow/survey-record.md" | grep -q '^# contract: ' \
    || note boundary-header-trailing "the boundary truncate left a trailing blank run below the header; each append re-separates, so a retained blank grows by one per boundary"
grep -qF '.workflow/survey-record.md' <<<"$out" \
    || note boundary-report "the boundary report does not name the truncated record: $out"
grep -qF 'which gates meet every port criterion?' <<<"$out" \
    && note boundary-headings "the boundary entry printed headings it had just discarded: $out"

# --- the gap inbox still refuses on its close-skipped branch, so the asymmetry is real and not an
# --- accident: a survey owes nobody a disposition, an undrained gap owes one to a stage that can
# --- still run. (The post-close branch admits instead, and gap-inbox-route.test.sh owns that.)
gap="$SANDBOX/gap"
seed "$gap"
cat >"$gap/.workflow/WORKFLOW-STATE.txt" <<'EOF'
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
demo-iteration validate cccccccc 2026-06-03 none
EOF
printf '# contract: lifecycle-kit/SPEC.md §The committed gap inbox\n- 2026-06-05 — an untriaged gap\n' \
    >"$gap/.workflow/gap-inbox.md"
out="$(run_enter "$gap" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note gap-refusal "want the gap inbox to refuse the boundary (exit 1), got $rc -- $out"

[[ "$fails" -eq 0 ]] || { echo "survey-record-entry.test: $fails assertion(s) failed"; exit 1; }
echo "survey-record-entry.test: clean (ordinary entry prints headings and no findings and writes nothing; boundary entry truncates to the header, names it, and never refuses, while the gap inbox beside it still does)"
exit 0
