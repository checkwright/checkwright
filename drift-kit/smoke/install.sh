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
## Iteration: smoke
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

# spec: drift-kit/SPEC.md §Testing — kpi-price-table-age over purpose-built tables: the
# dated-header reads, each row's independent degradation, and the inversion the KPI
# exists for (fresh age, expired prices, in one report).
ptkpi() { DRIFT_KIT_PRICE_TABLE="$1" bash "$SMOKE_KIT_ROOT/kpis/kpi-price-table-age.sh" "${2:-}"; }

pt_both="$work/pt-both.tsv"
cat > "$pt_both" <<EOF
# priced-as-of: $(date -d '3 days ago' +%F) — trailing prose the reader must ignore
# prices-valid-through: $(date -d '10 days' +%F)
# model	input	output	cache_read	cache_creation
test-model	0.000001	0.000002	0.0000001	0.000002
EOF
set +e
ptout="$(ptkpi "$pt_both")"; ptrc=$?
set -e
[[ "$ptrc" -eq 0 ]] || fail "kpi-price-table-age exited $ptrc (advisory plugins always exit 0)"
[[ "$(grep -c '^lead' <<<"$ptout")" -eq 2 ]] || fail "kpi-price-table-age did not emit its two lead rows over a fully dated table"
grep -q 'priced 3d ago (as-of' <<<"$ptout" || fail "age row did not read the priced-as-of: header: $ptout"
grep -q 'expires in 10d (through' <<<"$ptout" || fail "expiry row did not read the prices-valid-through: header: $ptout"
pttrend="$(ptkpi "$pt_both" --trend)"
[[ "$pttrend" == 'price 3d' ]] || fail "kpi-price-table-age --trend not 'price 3d': $pttrend"

pt_inv="$work/pt-inverted.tsv"
cat > "$pt_inv" <<EOF
# priced-as-of: $(date +%F)
# prices-valid-through: $(date -d 'yesterday' +%F)
# model	input	output	cache_read	cache_creation
test-model	0.000001	0.000002	0.0000001	0.000002
EOF
ptinv="$(ptkpi "$pt_inv")"
grep -q 'priced 0d ago' <<<"$ptinv" || fail "inversion fixture: age row should read fresh (0d), got: $ptinv"
grep -q 'EXPIRED 1d ago — re-verify (through' <<<"$ptinv" \
    || fail "inversion fixture: a lapsed prices-valid-through: must read EXPIRED even beside a fresh age row: $ptinv"
[[ "$(ptkpi "$pt_inv" --trend)" == 'price 0d' ]] \
    || fail "inversion fixture: the trend fragment tracks age only, and must still emit"

pt_noexp="$work/pt-noexpiry.tsv"
printf '# priced-as-of: %s\ntest-model\t0.000001\t0.000002\t0.0000001\t0.000002\n' "$(date -d '1 day ago' +%F)" > "$pt_noexp"
ptne="$(ptkpi "$pt_noexp")"
grep -q 'priced 1d ago' <<<"$ptne" || fail "age row must still report when the optional expiry header is absent: $ptne"
grep -q 'n/a (no prices-valid-through: header)' <<<"$ptne" \
    || fail "absent optional expiry header must degrade its own row visibly, not the age row: $ptne"

pt_noage="$work/pt-noage.tsv"
printf '# prices-valid-through: %s\ntest-model\t0.000001\t0.000002\t0.0000001\t0.000002\n' "$(date -d '5 days' +%F)" > "$pt_noage"
ptna="$(ptkpi "$pt_noage")"
grep -q 'n/a (no priced-as-of: header)' <<<"$ptna" || fail "absent priced-as-of: must degrade the age row visibly: $ptna"
grep -q 'expires in 5d' <<<"$ptna" || fail "the expiry row degrades independently of the age row: $ptna"
[[ -z "$(ptkpi "$pt_noage" --trend)" ]] || fail "--trend must emit nothing when the age value is n/a"

pt_bad="$work/pt-bad.tsv"
printf '# priced-as-of: not-a-date\n# prices-valid-through: 2026-13-45\ntest-model\t0.000001\t0.000002\t0.0000001\t0.000002\n' > "$pt_bad"
ptbad="$(ptkpi "$pt_bad")"
grep -q 'n/a (unparseable priced-as-of date)' <<<"$ptbad" || fail "malformed priced-as-of must read as unparseable, not as absent: $ptbad"
grep -q 'n/a (unparseable prices-valid-through date)' <<<"$ptbad" || fail "malformed prices-valid-through must read as unparseable, not as absent: $ptbad"

ptmiss="$(ptkpi "$work/no-such-price-table.tsv")"
[[ "$(grep -c '^lead' <<<"$ptmiss")" -eq 1 ]] || fail "with no table the KPI emits one row, not an expiry row for a table that is not there: $ptmiss"
grep -q 'n/a (no price table)' <<<"$ptmiss" || fail "absent price table must degrade fail-visibly: $ptmiss"

# spec: drift-kit/SPEC.md §Testing — the stage-economics join over a synthetic fixture set:
# a WORKFLOW-STATE stamp file, a transcript whose basename normalizes to the stamped
# session8, and a placeholder price table. Known tokens in, known trend line out.
sedir="$work/sessions"; mkdir -p "$sedir"
cat > "$sedir/agent-sess1234deadbeef.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"m1","model":"test-model","usage":{"input_tokens":10,"output_tokens":5,"cache_read_input_tokens":100,"cache_creation_input_tokens":20}}}
{"type":"assistant","message":{"id":"m1","model":"test-model","usage":{"input_tokens":10,"output_tokens":8,"cache_read_input_tokens":100,"cache_creation_input_tokens":20}}}
{"type":"assistant","message":{"id":"m2","model":"test-model","usage":{"input_tokens":4,"output_tokens":3,"cache_read_input_tokens":50,"cache_creation_input_tokens":10}}}
EOF
printf 'smoke build sess1234 2025-01-01\n' > "$work/se-state.txt"
printf 'model\tinput\toutput\tcache_read\tcache_creation\ntest-model\t1\t2\t3\t4\n' > "$work/se-prices.tsv"
selog="$work/se-log.txt"

econ() {   # $1 = price-table path (a missing path exercises the degradation)
    DRIFT_KIT_STATE_FILE="$work/se-state.txt" \
    DRIFT_KIT_SESSIONS_DIR="$sedir" \
    DRIFT_KIT_PRICE_TABLE="$1" \
    DRIFT_KIT_STAGE_ECONOMICS_LOG="$selog" \
    bash "$SMOKE_KIT_ROOT/bin/stage-economics.sh"
}

set +e
eout="$(econ "$work/se-prices.tsv" 2>&1)"; erc=$?
set -e
[[ "$erc" -eq 0 ]] || fail "stage-economics exited $erc (advisory tool must exit 0)"

if command -v jq >/dev/null 2>&1; then
    [[ -s "$selog" ]] || fail "stage-economics wrote no trend line"
    [[ "$(grep -c '' "$selog")" -eq 1 ]] || fail "stage-economics log has more than one line for one (iteration,stage,model) triple"
    seln="$(cat "$selog")"
    grep -qE '^[0-9-]+ smoke build test-model in=14 out=11 cr=150 cw=30 cost=606\.[0-9]+$' <<<"$seln" \
        || fail "trend line does not match the documented grammar/values: $seln"

    econ "$work/se-prices.tsv" >/dev/null   # re-measure replaces the triple's line, never doubles it
    [[ "$(grep -c '' "$selog")" -eq 1 ]] || fail "re-measure double-counted the triple (dedup broken)"

    : > "$selog"
    set +e
    dout="$(econ "$work/no-such-price-table.tsv")"; drc=$?
    set -e
    [[ "$drc" -eq 0 ]] || fail "stage-economics (price table absent) exited $drc"
    grep -q 'cost=n/a' "$selog" || fail "absent price table did not degrade the cost cell to n/a"
    grep -q 'incomplete' <<<"$dout" || fail "degraded run did not carry the incomplete-pricing caveat"
else
    grep -q 'jq not found' <<<"$eout" || fail "stage-economics without jq must emit its degradation notice"
fi

# spec: drift-kit/SPEC.md §Testing — history ∪ live over the trajectory extractor's
# fake-history repo, whose live state file already carries only beta's stamp.
if command -v jq >/dev/null 2>&1; then
    hdir="$work/hist-sessions"; mkdir -p "$hdir"
    for sid in s2 s4; do
        cat > "$hdir/$sid.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"m1","model":"test-model","usage":{"input_tokens":1,"output_tokens":1,"cache_read_input_tokens":1,"cache_creation_input_tokens":1}}}
EOF
    done
    hlog="$work/hist-log.txt"
    set +e
    hout="$( cd "$trepo" && DRIFT_KIT_STATE_FILE=".workflow/WORKFLOW-STATE.txt" \
        DRIFT_KIT_SESSIONS_DIR="$hdir" \
        DRIFT_KIT_PRICE_TABLE="$work/se-prices.tsv" \
        DRIFT_KIT_STAGE_ECONOMICS_LOG="$hlog" \
        bash "$SMOKE_KIT_ROOT/bin/stage-economics.sh" 2>&1 )"; hrc=$?
    set -e
    [[ "$hrc" -eq 0 ]] || fail "stage-economics over fake history exited $hrc (advisory tool must exit 0)"
    grep -q '^alpha build s2 ' "$trepo/.workflow/WORKFLOW-STATE.txt" \
        && fail "fake-history premise broken: alpha's stamp is still in the live state file"
    grep -q ' alpha build test-model ' "$hlog" \
        || fail "a stamp surviving only in committed history did not price (truncation immunity lost)"
    grep -q ' beta scope test-model ' "$hlog" \
        || fail "the live file's stamp did not price (the union dropped its live arm)"
fi

# spec: drift-kit/SPEC.md §The stage-economics meter — the attribution invariant over its own fixture
# set: one session bearing two stamps bills once (its last), and a transcript matching no stamp is
# counted as the under-count bound. Its own sessions dir, state file, and log — the flat fixture set's
# log is asserted to hold exactly one line, so a second row there would red that assertion, not this one.
if command -v jq >/dev/null 2>&1; then
    dbldir="$work/dbl-sessions"; mkdir -p "$dbldir"
    cp "$sedir/agent-sess1234deadbeef.jsonl" "$dbldir/agent-sess1234deadbeef.jsonl"
    cp "$sedir/agent-sess1234deadbeef.jsonl" "$dbldir/orphan9876.jsonl"
    printf 'smoke scope sess1234 2025-01-01\nsmoke build sess1234 2025-01-01\n' > "$work/dbl-state.txt"
    dbllog="$work/dbl-log.txt"
    set +e
    dblout="$( DRIFT_KIT_STATE_FILE="$work/dbl-state.txt" \
        DRIFT_KIT_SESSIONS_DIR="$dbldir" \
        DRIFT_KIT_PRICE_TABLE="$work/se-prices.tsv" \
        DRIFT_KIT_STAGE_ECONOMICS_LOG="$dbllog" \
        bash "$SMOKE_KIT_ROOT/bin/stage-economics.sh" 2>&1 )"; dblrc=$?
    set -e
    [[ "$dblrc" -eq 0 ]] || fail "stage-economics over the two-stamp fixture exited $dblrc"
    [[ "$(grep -c '' "$dbllog")" -eq 1 ]] \
        || fail "one session with two stamps billed more than one row (the over-count defect is back)"
    grep -q ' smoke build test-model ' "$dbllog" \
        || fail "the two-stamp session was not attributed to its last stamp"
    grep -q 'yielded (no row): smoke scope' <<<"$dblout" \
        || fail "the collapsed stamp was not named in the caveat (a silent collapse is not an honest one)"
    grep -q '1 transcript(s) in the sessions dir match no stamp' <<<"$dblout" \
        || fail "the unstamped-transcript bound did not report the orphan transcript"

# spec: drift-kit/SPEC.md §The stage-economics meter — the supervision row, derived from the nested transcript
# tier: a dispatched stage session sits under <lead>/subagents/ while its lead sits flat beside it,
# so the lead is named by the path and needs no stamp. Its own dir/state/log, same reason as above.
    supdir="$work/sup-sessions"; mkdir -p "$supdir/lead0001dead/subagents"
    cp "$sedir/agent-sess1234deadbeef.jsonl" "$supdir/lead0001dead/subagents/agent-supa1234feed.jsonl"
    cat > "$supdir/lead0001dead.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"L1","model":"test-model","usage":{"input_tokens":2,"output_tokens":3,"cache_read_input_tokens":4,"cache_creation_input_tokens":5}}}
EOF
    printf 'supiter build supa1234 2025-01-01\n' > "$work/sup-state.txt"
    suplog="$work/sup-log.txt"
    sup() {   # $1 = supervision label
        DRIFT_KIT_STATE_FILE="$work/sup-state.txt" \
        DRIFT_KIT_SESSIONS_DIR="$supdir" \
        DRIFT_KIT_PRICE_TABLE="$work/se-prices.tsv" \
        DRIFT_KIT_STAGE_ECONOMICS_LOG="$suplog" \
        DRIFT_KIT_SUPERVISION_LABEL="$1" \
        bash "$SMOKE_KIT_ROOT/bin/stage-economics.sh" 2>&1
    }
    set +e
    supout="$(sup supervision)"; suprc=$?
    set -e
    [[ "$suprc" -eq 0 ]] || fail "stage-economics over the nested-tier fixture exited $suprc"
    [[ "$(grep -c ' supiter supervision test-model ' "$suplog")" -eq 1 ]] \
        || fail "the nested-tier fixture did not yield exactly one supervision row: $supout"
    grep -qE ' supiter supervision test-model in=2 out=3 cr=4 cw=5 cost=40\.[0-9]+$' "$suplog" \
        || fail "the supervision row does not carry the lead transcript's own usage"
    grep -q ' supiter build test-model ' "$suplog" \
        || fail "the dispatched stage session lost its own row to the supervision derivation"

    : > "$suplog"
    sup lead-burn >/dev/null
    grep -q ' supiter lead-burn test-model ' "$suplog" \
        || fail "DRIFT_KIT_SUPERVISION_LABEL did not name the row (the label is not a literal)"

    : > "$suplog"
    printf 'supiter build supa1234 2025-01-01\nsupiter supervision nolead12 2025-01-01\n' > "$work/sup-state.txt"
    set +e
    colout="$(sup supervision)"
    set -e
    grep -q 'colliding with DRIFT_KIT_SUPERVISION_LABEL' <<<"$colout" \
        || fail "a stamp naming the label did not raise the collision notice"
    [[ "$(grep -c ' supiter supervision ' "$suplog")" -eq 0 ]] \
        || fail "the collision did not suppress the supervision row"
fi
