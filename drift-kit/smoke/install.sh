#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Testing — advisory report smoke; drift-kit ships no gate, so the
# installer proves the report itself inline (guard-kit's precedent). Also gate-sdk/SPEC.md §Consumer smoke.
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored drift-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

mkdir -p scripts
cp "$SMOKE_KIT_ROOT/templates/drift-config.sh" scripts/drift-config.sh
cp "$SMOKE_KIT_ROOT/templates/kpis.list"       scripts/kpis.list

work="$(mktemp -d "${TMPDIR:-/tmp}/drift-smoke.XXXXXX")"
trepo="$(mktemp -d "${TMPDIR:-/tmp}/traj-smoke.XXXXXX")"
trap 'rm -rf "$work" "$trepo"' EXIT

cp "$SMOKE_KIT_ROOT/templates/kpis.list" "$work/kpis.list"
echo 'kpi-does-not-exist' >> "$work/kpis.list"   # a registry naming a missing plugin

cat > "$work/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md
## Iteration: smoke  [stage: build]
## Deferred
- **foo** [needs-spec] — a thing. Surfaced 2025-01-01.
## Done
EOF

registered="$(grep -cEv '^[[:space:]]*(#|$)' "$work/kpis.list")"

report() {
    DRIFT_KIT_KPIS_FILE="$work/kpis.list" \
    DRIFT_KIT_QUEUE_FILE="$work/TASK-QUEUE.md" \
    DRIFT_KIT_TMP_DIR="$work" \
    DRIFT_KIT_TIMINGS_FILE="$work/no-such-timings.txt" \
    bash "$SMOKE_KIT_ROOT/bin/drift-report.sh" "$@"
}

fail() { echo "drift-kit/smoke/install.sh: $1" >&2; exit 1; }

set +e
out="$(report)"; rc=$?
set -e
[[ "$rc" -eq 0 ]] || fail "full report exited $rc (advisory report must exit 0)"
grep -q '^=== Drift KPIs' <<<"$out" || fail "missing report header"
grep -q '^--- Lead'        <<<"$out" || fail "missing Lead section header"
grep -q '^--- Lag'         <<<"$out" || fail "missing Lag section header"
grep -q 'Read trend across sessions' <<<"$out" || fail "missing footer"

total_rows="$(awk '/^--- Lead/{f=1} /^Read trend/{f=0} f && /^  [^ ]/{c++} END{print c+0}' <<<"$out")"
[[ "$total_rows" -eq "$registered" ]] || fail "expected one row per registered KPI ($registered), got $total_rows"

grep -q 'kpi-does-not-exist.*n/a' <<<"$out" || fail "missing plugin did not yield a visible n/a row"

awk '/^--- Lag/{f=1;next} /^Read trend/{f=0} f' <<<"$out" | grep -q 'knowledge friction.*n/a' \
    || fail "kpi-knowledge-friction did not render an n/a row under the Lag section (log absent in the throwaway consumer)"

set +e
trend="$(report --trend)"; trc=$?
set -e
[[ "$trc" -eq 0 ]] || fail "--trend exited $trc"
trend_lines="$(printf '%s' "$trend" | grep -c '')"
[[ "$trend_lines" -eq 1 ]] || fail "--trend must emit exactly one line, got $trend_lines"

# spec: drift-kit/SPEC.md §The published-evidence extractor — a hermetic fake-history
# repo with one closed, range-bounded iteration must emit exactly that iteration's row.
git -C "$trepo" init -q
mkdir -p "$trepo/.workflow"
tcommit() {
    git -C "$trepo" add -A
    git -C "$trepo" -c user.email=smoke@example.invalid -c user.name=smoke commit -q -m "$1"
}
printf 'alpha scope s1 2025-01-01\n' > "$trepo/.workflow/WORKFLOW-STATE.txt"; tcommit "feat: alpha scope"
printf 'alpha build s2 2025-01-01\n' >> "$trepo/.workflow/WORKFLOW-STATE.txt"; tcommit "feat: alpha build"
printf 'alpha close s3 2025-01-02\n' >> "$trepo/.workflow/WORKFLOW-STATE.txt"; tcommit "fix: alpha close"
printf 'beta scope s4 2025-01-03\n'  > "$trepo/.workflow/WORKFLOW-STATE.txt";  tcommit "feat: beta scope"

set +e
traj="$( cd "$trepo" && bash "$SMOKE_KIT_ROOT/bin/trajectory.sh" --emit )"; jrc=$?
set -e
[[ "$jrc" -eq 0 ]] || fail "trajectory --emit exited $jrc (advisory tool must exit 0)"
grep -q '^| iteration |' <<<"$traj" || fail "trajectory missing table header"
[[ "$(grep -c '^| alpha ' <<<"$traj")" -ge 1 ]] || fail "trajectory emitted no closed-iteration row (expected alpha)"
if grep -q '^| beta ' <<<"$traj"; then fail "trajectory emitted the in-flight (unclosed) beta row"; fi
