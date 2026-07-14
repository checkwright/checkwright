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

# spec: drift-kit/SPEC.md §Testing — the synthetic-transcript classifier smoke:
# known category bytes in (smoke/overhead-fixture.jsonl), known percentages out.
fixture="$SMOKE_KIT_ROOT/smoke/overhead-fixture.jsonl"
ovlog="$work/ovh-log.txt"
meter() { DRIFT_KIT_TMP_DIR="$work" DRIFT_KIT_OVERHEAD_LOG="$ovlog" bash "$SMOKE_KIT_ROOT/bin/overhead-meter.sh" "$fixture"; }

set +e
mout="$(meter)"; mrc=$?
set -e
[[ "$mrc" -eq 0 ]] || fail "overhead-meter exited $mrc (advisory tool must exit 0)"
grep -q 'byte-proxy' <<<"$mout" || fail "meter stdout missing the byte-proxy caveat"
[[ -s "$ovlog" ]] || fail "meter wrote no log line"
[[ "$(grep -c '' "$ovlog")" -eq 1 ]] || fail "meter log has more than one line for one session"

logln="$(cat "$ovlog")"
grep -qE '^[0-9-]+ [0-9A-Za-z]+ total=[0-9]+ gov=[0-9]+ gate=[0-9]+ pct=[0-9]+$' <<<"$logln" \
    || fail "log line does not match the documented grammar: $logln"

tot=$(LC_ALL=C awk '{t+=length($0)} END{print t+0}' "$fixture")
taskb=$(LC_ALL=C awk '/ordinary task work/{t+=length($0)} END{print t+0}' "$fixture")
gtotal=$(sed -E 's/.* total=([0-9]+) .*/\1/' <<<"$logln")
ggov=$(sed -E 's/.* gov=([0-9]+) .*/\1/' <<<"$logln")
ggate=$(sed -E 's/.* gate=([0-9]+) .*/\1/' <<<"$logln")
gpct=$(sed -E 's/.* pct=([0-9]+)$/\1/' <<<"$logln")
[[ "$gtotal" -eq "$tot" ]] || fail "meter total ($gtotal) != fixture bytes ($tot)"
[[ "$ggov" -eq $(( tot - taskb )) ]] || fail "gov ($ggov) != total-taskline ($(( tot - taskb ))): task line miscounted"
(( ggate > 0 && ggate < ggov )) || fail "gate ($ggate) is not a positive proper subset of gov ($ggov)"
[[ "$gpct" -eq $(( (ggov * 100 + gtotal / 2) / gtotal )) ]] || fail "pct ($gpct) != round(100*gov/total)"

meter >/dev/null   # re-measure replaces the session's line, never doubles it
[[ "$(grep -c '' "$ovlog")" -eq 1 ]] || fail "re-measure double-counted the session (dedup broken)"

set +e
kout="$(DRIFT_KIT_OVERHEAD_LOG="$ovlog" bash "$SMOKE_KIT_ROOT/kpis/kpi-overhead.sh")"; krc=$?
set -e
[[ "$krc" -eq 0 ]] || fail "kpi-overhead exited $krc"
[[ "$(grep -c '^lead' <<<"$kout")" -eq 2 ]] || fail "kpi-overhead did not emit its two lead rows over a live log"
grep -q 'byte-proxy' <<<"$kout" || fail "kpi-overhead rows missing the byte-proxy caveat"
ktrend="$(DRIFT_KIT_OVERHEAD_LOG="$ovlog" bash "$SMOKE_KIT_ROOT/kpis/kpi-overhead.sh" --trend)"
grep -qE '^ovh [0-9]+%$' <<<"$ktrend" || fail "kpi-overhead --trend not 'ovh <n>%': $ktrend"

set +e
kna="$(DRIFT_KIT_OVERHEAD_LOG="$work/no-such-overhead.txt" bash "$SMOKE_KIT_ROOT/kpis/kpi-overhead.sh")"; knrc=$?
set -e
[[ "$knrc" -eq 0 ]] || fail "kpi-overhead (log absent) exited $knrc"
grep -q 'n/a' <<<"$kna" || fail "kpi-overhead did not degrade to a visible n/a row without a log"

# spec: drift-kit/SPEC.md §Testing — the writer/reader-divergence assertion: under one
# DRIFT_KIT_METRIC_DIR override (no explicit OVERHEAD_LOG), writer and reader must
# compute the same default log path, or a default drift splits them silently.
mdir="$work/metric"
DRIFT_KIT_METRIC_DIR="$mdir" bash "$SMOKE_KIT_ROOT/bin/overhead-meter.sh" "$fixture" >/dev/null \
    || fail "overhead-meter failed under a DRIFT_KIT_METRIC_DIR-only override"
[[ -s "$mdir/overhead-log.txt" ]] || fail "writer did not resolve DRIFT_KIT_METRIC_DIR into its default log path"
set +e
kmd="$(DRIFT_KIT_METRIC_DIR="$mdir" bash "$SMOKE_KIT_ROOT/kpis/kpi-overhead.sh")"; kmrc=$?
set -e
[[ "$kmrc" -eq 0 ]] || fail "kpi-overhead exited $kmrc under the shared DRIFT_KIT_METRIC_DIR override"
if grep -q 'n/a' <<<"$kmd"; then fail "writer/reader default divergence: reader missed the log the writer wrote under one DRIFT_KIT_METRIC_DIR override"; fi
[[ "$(grep -c '^lead' <<<"$kmd")" -eq 2 ]] || fail "kpi-overhead did not read the metric-dir log the meter wrote"
