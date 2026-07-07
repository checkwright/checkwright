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
trap 'rm -rf "$work"' EXIT

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
