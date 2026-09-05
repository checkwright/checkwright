#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Testing — advisory report smoke; drift-kit ships no gate, so the
# installer proves the report itself inline (guard-kit's precedent). Also gate-sdk/SPEC.md §Consumer smoke.
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored drift-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — leg 3 of the class ruling of 2026-08-30, which reaches this file by its GROUND rather than by its scope: that ruling stated-contract cut covers the recipes answering to §Consumer smoke and this one answers to drift-kit/SPEC.md §Testing, but leg 3 holds of it identically. Leg 3: it vendors to an adopter with its kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers in existence are this repo own validate suites — so it costs an adopter no interpreter dependency. Leg 2 assertion-B half does not reach here, drift-kit shipping no checks/ at all. This replaces the temporary hold on kit-smoke-port-disposition-cohort, the entry that ruling closes; structural, not a sizing judgment.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SMOKE_KIT_ROOT/../gate-sdk/lib/gate.sh"

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
- **foo** [design-pending] — a thing. Surfaced 2025-01-01.
## Done
EOF

registered="$(grep -cEv '^[[:space:]]*(#|$)' "$work/kpis.list")"

# spec: gate-sdk/SPEC.md §The non-gate arm — the collator is a compiled arm reached through the
# front-end that resolves its bridged knobs, the shape the trajectory extractor below already has.
DRIFT_ARM="$SMOKE_KIT_ROOT/../gate-sdk/bin/run-gates.sh"

report() {
    DRIFT_KIT_KPIS_FILE="$work/kpis.list" \
    DRIFT_KIT_QUEUE_FILE="$work/TASK-QUEUE.md" \
    DRIFT_KIT_TMP_DIR="$work" \
    DRIFT_KIT_TIMINGS_FILE="$work/no-such-timings.txt" \
    bash "$DRIFT_ARM" --emit drift-report "$@"
}

fail() { echo "drift-kit/smoke/install.sh: $1" >&2; exit 1; }

# spec: drift-kit/SPEC.md §Testing — one member exercised in isolation *through* the collator
# rather than beside it: a one-name registry is the collator's own selector, so an isolated probe
# runs the real resolution and rendering path instead of a bypass the kit does not ship.
solo() {
    local name="$1"
    shift
    printf '%s\n' "$name" > "$work/solo.list"
    DRIFT_KIT_KPIS_FILE="$work/solo.list" \
    DRIFT_KIT_QUEUE_FILE="${DRIFT_KIT_QUEUE_FILE:-$work/TASK-QUEUE.md}" \
    DRIFT_KIT_TMP_DIR="$work" \
    DRIFT_KIT_TIMINGS_FILE="$work/no-such-timings.txt" \
    bash "$DRIFT_ARM" --emit drift-report "$@"
}

# spec: drift-kit/SPEC.md §The report skeleton — the rendered rows of one labelled section, which
# is where a member's rows land once the collator owns the frame and the member owns no printf.
rows() { awk -v b="^--- $1" '$0 ~ b {f=1;next} /^$/{f=0} f && /^  [^ ]/'; }
row_count() { rows "$1" | grep -c ''; }

# spec: drift-kit/SPEC.md §The report skeleton — the trend line is the joined fragments behind one
# `drift: ` lead, so a single-member probe reads its fragment back off that frame.
trend_frag() { local o; o="$(solo "$@" --trend)"; printf '%s' "${o#drift: }"; }

set +e
out="$(report)"; rc=$?
set -e
[[ "$rc" -eq 0 ]] || fail "full report exited $rc (advisory report must exit 0)"
grep -q '^=== Drift KPIs' <<<"$out" || fail "missing report header"
grep -q '^--- Lead'        <<<"$out" || fail "missing Lead section header"
grep -q '^--- Lag'         <<<"$out" || fail "missing Lag section header"
grep -q 'Read trend across sessions' <<<"$out" || fail "missing footer"

total_rows="$(awk '/^--- Lead/{f=1} /^Read trend/{f=0} f && /^  [^ ]/{c++} END{print c+0}' <<<"$out")"
[[ "$total_rows" -ge "$registered" ]] || fail "expected at least one row per registered KPI ($registered), got $total_rows"

grep -q 'kpi-does-not-exist.*n/a' <<<"$out" || fail "missing plugin did not yield a visible n/a row"

awk '/^--- Lag/{f=1;next} /^Read trend/{f=0} f' <<<"$out" | grep -q 'knowledge friction.*n/a' \
    || fail "kpi-knowledge-friction did not render an n/a row under the Lag section (log absent in the throwaway consumer)"

# spec: drift-kit/SPEC.md §The knowledge-friction loop — three log states, three lines. The
# absent arm above is one; without these two the empty state is indistinguishable from a
# measurement of zero, which is the reading the loop refuses.
kfric() { DRIFT_KIT_KNOWLEDGE_LOG="$1" solo kpi-knowledge-friction; }
kfric_trend() { DRIFT_KIT_KNOWLEDGE_LOG="$1" trend_frag kpi-knowledge-friction; }
: > "$work/kfric-empty.log"
printf '2026-01-01 a fact - a surface\n' > "$work/kfric-full.log"

kfric "$work/kfric-empty.log" | grep -q 'not evidence of zero friction' \
    || fail "an empty knowledge log did not carry its non-inference at the point of reading"
if kfric "$work/kfric-empty.log" | grep -q 'lower bound'; then
    fail "an empty knowledge log reported the non-empty sentence, which reads a count of nothing as a measurement of zero"
fi
kfric "$work/kfric-full.log" | grep -q '1 re-derivation(s) logged this iteration (lower bound)' \
    || fail "a non-empty knowledge log lost its lower-bound qualifier"

# spec: drift-kit/SPEC.md §The knowledge-friction loop — --trend's grammar does not move for the
# empty state; a series spanning a grammar change is two series, not one.
[[ "$(kfric_trend "$work/kfric-empty.log")" == 'kfric 0' ]] \
    || fail "--trend changed grammar for the empty log, breaking comparability across the change"
[[ "$(kfric_trend "$work/kfric-full.log")" == 'kfric 1' ]] \
    || fail "--trend did not report the non-empty count"

# spec: drift-kit/SPEC.md §Testing — the per-member contribution probe: the report's own
# 'n/a (plugin failed)' mask, read through the dispatch a session gets, since the row-count
# floor above cannot see a silent member.
if grep -q 'n/a (plugin failed)' <<<"$out"; then
    fail "a registered member produced nothing and the report masked it: $(grep 'n/a (plugin failed)' <<<"$out")"
fi

# spec: drift-kit/SPEC.md §Testing — and the control, so the assertion above is not vacuous: a
# member that *does* fail must reach that row rather than vanishing from the report.
mkdir -p "$work/failing"
printf '#!/usr/bin/env bash\nexit 3\n' > "$work/failing/kpi-deliberately-broken.sh"
chmod +x "$work/failing/kpi-deliberately-broken.sh"
printf 'DRIFT_KIT_KPI_DIRS=("%s")\n' "$work/failing" > "$work/failing-config.sh"
broke="$(DRIFT_KIT_CONFIG_FILE="$work/failing-config.sh" solo kpi-deliberately-broken)"
grep -q 'n/a (plugin failed)' <<<"$broke" \
    || fail "a member exiting non-zero must reach the fail-visible row, or the assertion above passes vacuously: $broke"

# spec: drift-kit/SPEC.md §The extensibility contract — the extension point end to end: resolve,
# execute directly, shadow a built-in of its own name, read both handoffs, and receive a knob only
# the consumer's config declares. A port is where each of those is dropped silently.
mkdir -p "$work/consumer-kpis"
cat > "$work/consumer-kpis/kpi-knowledge-friction.sh" <<'SHADOW'
#!/usr/bin/env bash
[[ "${1:-}" == "--trend" ]] && exit 0
printf 'lead\tconsumer shadow\troots=%s start=%s custom=%s\n' \
    "$(printf '%s' "$DRIFT_KIT_KIT_ROOTS" | grep -c '')" \
    "${DRIFT_KIT_ITERATION_START+set}" \
    "$DRIFT_KIT_SMOKE_CUSTOM"
SHADOW
chmod +x "$work/consumer-kpis/kpi-knowledge-friction.sh"
printf 'DRIFT_KIT_KPI_DIRS=("%s")\nDRIFT_KIT_SMOKE_CUSTOM=reached\n' "$work/consumer-kpis" > "$work/consumer-config.sh"
xp() { DRIFT_KIT_CONFIG_FILE="$work/consumer-config.sh" solo kpi-knowledge-friction; }

xpout="$(xp)"
grep -q 'consumer shadow' <<<"$xpout" \
    || fail "a consumer plugin did not shadow the bundled member of its own name: $xpout"
# spec: drift-kit/SPEC.md §The KPI plugin contract — the iteration-start handoff is asserted
# *present*, not non-empty: no baseline is derivable in a throwaway consumer, and the contract's
# promise there is the empty string rather than an absent variable.
grep -qE 'roots=[1-9][0-9]* start=set' <<<"$xpout" \
    || fail "the consumer plugin did not receive both driver handoffs (kit roots, iteration start): $xpout"
grep -q 'custom=reached' <<<"$xpout" \
    || fail "a knob only the consumer config declares did not reach the plugin — the export set is transcribed, not derived: $xpout"

# spec: drift-kit/SPEC.md §The KPI plugin contract — invoked directly, never under `bash`, so the
# execute bit still governs and a non-executable plugin degrades fail-visibly instead of running.
chmod -x "$work/consumer-kpis/kpi-knowledge-friction.sh"
grep -q 'n/a (plugin failed)' <<<"$(xp)" \
    || fail "a non-executable consumer plugin ran anyway — the collator is invoking it under an interpreter, so the execute bit no longer governs"
chmod +x "$work/consumer-kpis/kpi-knowledge-friction.sh"

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
printf 'alpha scope s1 2025-01-01 none\n' > "$trepo/.workflow/WORKFLOW-STATE.txt"; tcommit "feat: alpha scope"
printf 'alpha build s2 2025-01-01 none\n' >> "$trepo/.workflow/WORKFLOW-STATE.txt"; tcommit "feat: alpha build"
printf 'alpha close s3 2025-01-02 none\n' >> "$trepo/.workflow/WORKFLOW-STATE.txt"; tcommit "fix: alpha close"
printf 'beta scope s4 2025-01-03 none\n'  > "$trepo/.workflow/WORKFLOW-STATE.txt";  tcommit "feat: beta scope"

# spec: gate-sdk/SPEC.md §The non-gate arm — the extractor is a compiled arm reached through the
# front-end that resolves its bridged knobs. The binary knob is absolutised because the run
# happens after a cd, where its repo-relative default would resolve to nothing.
TRAJ_BIN="$PWD/$(gate_native_bin)"
set +e
traj="$( cd "$trepo" && GATE_SDK_NATIVE_BIN="$TRAJ_BIN" \
    bash "$SMOKE_KIT_ROOT/../gate-sdk/bin/run-gates.sh" --emit trajectory )"; jrc=$?
set -e
[[ "$jrc" -eq 0 ]] || fail "the trajectory arm exited $jrc (advisory emission must exit 0)"
grep -q '^| iteration |' <<<"$traj" || fail "trajectory missing table header"
[[ "$(grep -c '^| alpha ' <<<"$traj")" -ge 1 ]] || fail "trajectory emitted no closed-iteration row (expected alpha)"
if grep -q '^| beta ' <<<"$traj"; then fail "trajectory emitted the in-flight (unclosed) beta row"; fi

# spec: drift-kit/SPEC.md §Testing — the synthetic-transcript classifier smoke:
# known category bytes in (smoke/overhead-fixture.jsonl), known percentages out.
fixture="$SMOKE_KIT_ROOT/smoke/overhead-fixture.jsonl"
ovlog="$work/ovh-log.txt"
meter() { DRIFT_KIT_OVERHEAD_LOG="$ovlog" bash "$DRIFT_ARM" --emit overhead-meter "$fixture"; }

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
kout="$(DRIFT_KIT_OVERHEAD_LOG="$ovlog" solo kpi-overhead)"; krc=$?
set -e
[[ "$krc" -eq 0 ]] || fail "kpi-overhead exited $krc"
[[ "$(row_count Lead <<<"$kout")" -eq 2 ]] || fail "kpi-overhead did not emit its two lead rows over a live log"
grep -q 'byte-proxy' <<<"$kout" || fail "kpi-overhead rows missing the byte-proxy caveat"
ktrend="$(DRIFT_KIT_OVERHEAD_LOG="$ovlog" trend_frag kpi-overhead)"
grep -qE '^ovh [0-9]+%$' <<<"$ktrend" || fail "kpi-overhead --trend not 'ovh <n>%': $ktrend"

set +e
kna="$(DRIFT_KIT_OVERHEAD_LOG="$work/no-such-overhead.txt" solo kpi-overhead)"; knrc=$?
set -e
[[ "$knrc" -eq 0 ]] || fail "kpi-overhead (log absent) exited $knrc"
grep -q 'n/a' <<<"$kna" || fail "kpi-overhead did not degrade to a visible n/a row without a log"

# spec: drift-kit/SPEC.md §Testing — the writer/reader-divergence assertion: under one
# DRIFT_KIT_METRIC_DIR override (no explicit OVERHEAD_LOG), writer and reader must
# compute the same default log path, or a default drift splits them silently.
mdir="$work/metric"
DRIFT_KIT_METRIC_DIR="$mdir" bash "$DRIFT_ARM" --emit overhead-meter "$fixture" >/dev/null \
    || fail "overhead-meter failed under a DRIFT_KIT_METRIC_DIR-only override"
[[ -s "$mdir/overhead-log.txt" ]] || fail "writer did not resolve DRIFT_KIT_METRIC_DIR into its default log path"
set +e
kmd="$(DRIFT_KIT_METRIC_DIR="$mdir" solo kpi-overhead)"; kmrc=$?
set -e
[[ "$kmrc" -eq 0 ]] || fail "kpi-overhead exited $kmrc under the shared DRIFT_KIT_METRIC_DIR override"
if grep -q 'n/a' <<<"$kmd"; then fail "writer/reader default divergence: reader missed the log the writer wrote under one DRIFT_KIT_METRIC_DIR override"; fi
[[ "$(row_count Lead <<<"$kmd")" -eq 2 ]] || fail "kpi-overhead did not read the metric-dir log the meter wrote"

# spec: drift-kit/SPEC.md §Testing — kpi-price-table-age over purpose-built tables: the
# dated-header reads, each row's independent degradation, and the inversion the KPI
# exists for (fresh age, expired prices, in one report).
ptkpi() { DRIFT_KIT_PRICE_TABLE="$1" solo kpi-price-table-age; }
pttrend_of() { DRIFT_KIT_PRICE_TABLE="$1" trend_frag kpi-price-table-age; }

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
[[ "$(row_count Lead <<<"$ptout")" -eq 2 ]] || fail "kpi-price-table-age did not emit its two lead rows over a fully dated table"
grep -q 'priced 3d ago (as-of' <<<"$ptout" || fail "age row did not read the priced-as-of: header: $ptout"
grep -q 'expires in 10d (through' <<<"$ptout" || fail "expiry row did not read the prices-valid-through: header: $ptout"
pttrend="$(pttrend_of "$pt_both")"
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
[[ "$(pttrend_of "$pt_inv")" == 'price 0d' ]] \
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
[[ -z "$(pttrend_of "$pt_noage")" ]] || fail "--trend must emit nothing when the age value is n/a"

pt_bad="$work/pt-bad.tsv"
printf '# priced-as-of: not-a-date\n# prices-valid-through: 2026-13-45\ntest-model\t0.000001\t0.000002\t0.0000001\t0.000002\n' > "$pt_bad"
ptbad="$(ptkpi "$pt_bad")"
grep -q 'n/a (unparseable priced-as-of date)' <<<"$ptbad" || fail "malformed priced-as-of must read as unparseable, not as absent: $ptbad"
grep -q 'n/a (unparseable prices-valid-through date)' <<<"$ptbad" || fail "malformed prices-valid-through must read as unparseable, not as absent: $ptbad"

ptmiss="$(ptkpi "$work/no-such-price-table.tsv")"
[[ "$(row_count Lead <<<"$ptmiss")" -eq 1 ]] || fail "with no table the KPI emits one row, not an expiry row for a table that is not there: $ptmiss"
grep -q 'n/a (no price table)' <<<"$ptmiss" || fail "absent price table must degrade fail-visibly: $ptmiss"

# spec: drift-kit/SPEC.md §Testing — kpi-incident-recurrence over purpose-built queues:
# the sum across declarations, the highest-count slug, the trend fragment, and the two
# degradations. Fixture-stable, like kpi-price-table-age: it reads a file the fixture writes.
irkpi() { DRIFT_KIT_QUEUE_FILE="$1" solo kpi-incident-recurrence; }
irtrend_of() { DRIFT_KIT_QUEUE_FILE="$1" trend_frag kpi-incident-recurrence; }

ir_q="$work/recurrence-queue.md"
cat > "$ir_q" <<'EOF'
# TASK-QUEUE.md
## Iteration: smoke
## Deferred
- **thrice** [design-pending] — a finding re-filed three times.
  recurrence: thrice 2026-08-01 2026-08-02 2026-08-04
- **once** [design-pending] — a finding re-filed once.
  recurrence: once 2026-08-04
## Done
EOF
set +e
irout="$(irkpi "$ir_q")"; irrc=$?
set -e
[[ "$irrc" -eq 0 ]] || fail "kpi-incident-recurrence exited $irrc (advisory plugins always exit 0)"
[[ "$(row_count Lag <<<"$irout")" -eq 1 ]] || fail "kpi-incident-recurrence must emit exactly one lag row: $irout"
grep -q '4 re-filing(s) recorded' <<<"$irout" || fail "the count must sum dates across every declaration, not count declarations: $irout"
grep -q 'highest thrice at 3' <<<"$irout" || fail "the highest-count slug row is missing or wrong: $irout"
[[ "$(irtrend_of "$ir_q")" == 'recur 4' ]] || fail "kpi-incident-recurrence --trend not 'recur 4': $(irtrend_of "$ir_q")"

irnone="$(irkpi "$work/TASK-QUEUE.md")"
grep -q 'n/a (no recurrence declaration in the queue)' <<<"$irnone" \
    || fail "a queue with no declaration must degrade fail-visibly rather than reporting 0: $irnone"
[[ -z "$(irtrend_of "$work/TASK-QUEUE.md")" ]] || fail "--trend must emit nothing when no declaration exists"
grep -q 'n/a (no queue file)' <<<"$(irkpi "$work/no-such-queue.md")" \
    || fail "an absent queue file must degrade fail-visibly"

# spec: drift-kit/SPEC.md §Testing — the stage-economics join over a synthetic fixture set:
# a WORKFLOW-STATE stamp file, a transcript whose basename normalizes to the stamped
# session8, and a placeholder price table. Known tokens in, known trend line out.
sedir="$work/sessions"; mkdir -p "$sedir"
cat > "$sedir/agent-sess1234deadbeef.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"m1","model":"test-model","usage":{"input_tokens":10,"output_tokens":5,"cache_read_input_tokens":100,"cache_creation_input_tokens":20}}}
{"type":"assistant","message":{"id":"m1","model":"test-model","usage":{"input_tokens":10,"output_tokens":8,"cache_read_input_tokens":100,"cache_creation_input_tokens":20}}}
{"type":"assistant","message":{"id":"m2","model":"test-model","usage":{"input_tokens":4,"output_tokens":3,"cache_read_input_tokens":50,"cache_creation_input_tokens":10}}}
EOF
printf 'smoke build sess1234 2025-01-01 none\n' > "$work/se-state.txt"
printf 'model\tinput\toutput\tcache_read\tcache_creation\ntest-model\t1\t2\t3\t4\n' > "$work/se-prices.tsv"
selog="$work/se-log.txt"

econ() {   # $1 = price-table path (a missing path exercises the degradation)
    DRIFT_KIT_STATE_FILE="$work/se-state.txt" \
    DRIFT_KIT_SESSIONS_DIR="$sedir" \
    DRIFT_KIT_PRICE_TABLE="$1" \
    DRIFT_KIT_STAGE_ECONOMICS_LOG="$selog" \
    bash "$DRIFT_ARM" --emit stage-economics
}

set +e
eout="$(econ "$work/se-prices.tsv" 2>&1)"; erc=$?
set -e
[[ "$erc" -eq 0 ]] || fail "stage-economics exited $erc (advisory tool must exit 0)"

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

# spec: drift-kit/SPEC.md §Testing — history ∪ live over the trajectory extractor's
# fake-history repo, whose live state file already carries only beta's stamp.
hdir="$work/hist-sessions"; mkdir -p "$hdir"
for sid in s2 s4; do
    cat > "$hdir/$sid.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"m1","model":"test-model","usage":{"input_tokens":1,"output_tokens":1,"cache_read_input_tokens":1,"cache_creation_input_tokens":1}}}
EOF
done
hlog="$work/hist-log.txt"
set +e
# spec: gate-sdk/SPEC.md §The non-gate arm — the binary knob is absolutised for the same reason
# the trajectory arm's own fake-history run does: after the cd, a repo-relative default resolves
# to nothing in the throwaway repo.
hout="$( cd "$trepo" && GATE_SDK_NATIVE_BIN="$TRAJ_BIN" \
    DRIFT_KIT_STATE_FILE=".workflow/WORKFLOW-STATE.txt" \
    DRIFT_KIT_SESSIONS_DIR="$hdir" \
    DRIFT_KIT_PRICE_TABLE="$work/se-prices.tsv" \
    DRIFT_KIT_STAGE_ECONOMICS_LOG="$hlog" \
    bash "$DRIFT_ARM" --emit stage-economics 2>&1 )"; hrc=$?
set -e
[[ "$hrc" -eq 0 ]] || fail "stage-economics over fake history exited $hrc (advisory tool must exit 0)"
grep -q '^alpha build s2 ' "$trepo/.workflow/WORKFLOW-STATE.txt" \
    && fail "fake-history premise broken: alpha's stamp is still in the live state file"
grep -q ' alpha build test-model ' "$hlog" \
    || fail "a stamp surviving only in committed history did not price (truncation immunity lost)"
grep -q ' beta scope test-model ' "$hlog" \
    || fail "the live file's stamp did not price (the union dropped its live arm)"

# spec: drift-kit/SPEC.md §The stage-economics meter — the attribution invariant over its own fixture
# set: one session bearing two stamps bills once (its last), and a transcript matching no stamp is
# counted as the under-count bound. Its own sessions dir, state file, and log — the flat fixture set's
# log is asserted to hold exactly one line, so a second row there would red that assertion, not this one.
dbldir="$work/dbl-sessions"; mkdir -p "$dbldir"
cp "$sedir/agent-sess1234deadbeef.jsonl" "$dbldir/agent-sess1234deadbeef.jsonl"
cp "$sedir/agent-sess1234deadbeef.jsonl" "$dbldir/orphan9876.jsonl"
printf 'smoke scope sess1234 2025-01-01 none\nsmoke build sess1234 2025-01-01 none\n' > "$work/dbl-state.txt"
dbllog="$work/dbl-log.txt"
set +e
dblout="$( DRIFT_KIT_STATE_FILE="$work/dbl-state.txt" \
    DRIFT_KIT_SESSIONS_DIR="$dbldir" \
    DRIFT_KIT_PRICE_TABLE="$work/se-prices.tsv" \
    DRIFT_KIT_STAGE_ECONOMICS_LOG="$dbllog" \
    bash "$DRIFT_ARM" --emit stage-economics 2>&1 )"; dblrc=$?
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
printf 'supiter build supa1234 2025-01-01 none\n' > "$work/sup-state.txt"
suplog="$work/sup-log.txt"
sup() {   # $1 = supervision label
    DRIFT_KIT_STATE_FILE="$work/sup-state.txt" \
    DRIFT_KIT_SESSIONS_DIR="$supdir" \
    DRIFT_KIT_PRICE_TABLE="$work/se-prices.tsv" \
    DRIFT_KIT_STAGE_ECONOMICS_LOG="$suplog" \
    DRIFT_KIT_SUPERVISION_LABEL="$1" \
    bash "$DRIFT_ARM" --emit stage-economics 2>&1
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
printf 'supiter build supa1234 2025-01-01 none\nsupiter supervision nolead12 2025-01-01 none\n' > "$work/sup-state.txt"
set +e
colout="$(sup supervision)"
set -e
grep -q 'colliding with DRIFT_KIT_SUPERVISION_LABEL' <<<"$colout" \
    || fail "a stamp naming the label did not raise the collision notice"
[[ "$(grep -c ' supiter supervision ' "$suplog")" -eq 0 ]] \
    || fail "the collision did not suppress the supervision row"

# spec: drift-kit/SPEC.md §Testing — the fan-out fixture: a three-level tree flat in one subagents dir,
# whose grandchild is what makes the walk testable. Its own dir/state/log, same reason as above.
fodir="$work/fo-sessions"; mkdir -p "$fodir/folead0001dead/subagents"
fosub="$fodir/folead0001dead/subagents"
cat > "$fodir/folead0001dead.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"L1","model":"test-model","usage":{"input_tokens":2,"output_tokens":3,"cache_read_input_tokens":4,"cache_creation_input_tokens":5}}}
EOF
cat > "$fosub/agent-fostage1feed.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"S1","model":"test-model","usage":{"input_tokens":10,"output_tokens":20,"cache_read_input_tokens":30,"cache_creation_input_tokens":40}}}
EOF
cat > "$fosub/agent-fochild2feed.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"C1","model":"test-model","usage":{"input_tokens":1,"output_tokens":2,"cache_read_input_tokens":3,"cache_creation_input_tokens":4}}}
EOF
cat > "$fosub/agent-fogrand3feed.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"G1","model":"test-model","usage":{"input_tokens":100,"output_tokens":200,"cache_read_input_tokens":300,"cache_creation_input_tokens":400}}}
EOF
# spec: drift-kit/SPEC.md §Testing — the fixture's second same-stage session: no usage of its own, so
# it anchors without emitting a row and its child must fold rather than replace the first anchor's.
printf '{"type":"user","message":{"role":"user","content":"no usage here"}}\n' > "$fosub/agent-fostage4feed.jsonl"
cat > "$fosub/agent-fochild5feed.jsonl" <<'EOF'
{"type":"assistant","message":{"id":"C5","model":"test-model","usage":{"input_tokens":1000,"output_tokens":2000,"cache_read_input_tokens":3000,"cache_creation_input_tokens":4000}}}
EOF
printf '{"agentType":"stage-session","spawnDepth":1}\n' > "$fosub/agent-fostage1feed.meta.json"
printf '{"agentType":"Explore","parentAgentId":"fostage1feed","spawnDepth":2}\n' > "$fosub/agent-fochild2feed.meta.json"
printf '{"agentType":"fork","isFork":true,"parentAgentId":"fochild2feed","spawnDepth":3}\n' > "$fosub/agent-fogrand3feed.meta.json"
printf '{"agentType":"stage-session","spawnDepth":1}\n' > "$fosub/agent-fostage4feed.meta.json"
printf '{"agentType":"Explore","parentAgentId":"fostage4feed","spawnDepth":2}\n' > "$fosub/agent-fochild5feed.meta.json"
printf 'foiter build fostage1 2025-01-01 none\nfoiter build fostage4 2025-01-01 none\n' > "$work/fo-state.txt"
folog="$work/fo-log.txt"
fan() {   # $1 = fan-out suffix
    DRIFT_KIT_STATE_FILE="$work/fo-state.txt" \
    DRIFT_KIT_SESSIONS_DIR="$fodir" \
    DRIFT_KIT_PRICE_TABLE="$work/se-prices.tsv" \
    DRIFT_KIT_STAGE_ECONOMICS_LOG="$folog" \
    DRIFT_KIT_FANOUT_SUFFIX="$1" \
    bash "$DRIFT_ARM" --emit stage-economics 2>&1
}
set +e
foout="$(fan '+fanout')"; forc=$?
set -e
[[ "$forc" -eq 0 ]] || fail "stage-economics over the fan-out fixture exited $forc"
[[ "$(grep -c ' foiter build+fanout test-model ' "$folog")" -eq 1 ]] \
    || fail "the three-level fixture did not yield exactly one fan-out row: $foout"
grep -qE ' foiter build\+fanout test-model in=1101 out=2202 cr=3303 cw=4404 cost=33030\.[0-9]+$' "$folog" \
    || fail "the fan-out row is not the sum over the whole subtree — a walk that stopped at depth 2, or two anchors racing under the dedup key instead of folding: $(cat "$folog")"
grep -q ' foiter build+fanout 2 anchors ' <<<"$foout" \
    || fail "two anchors sharing one (iteration, stage) did not fold into one row: $foout"
grep -qE ' foiter build test-model in=10 out=20 cr=30 cw=40 cost=300\.[0-9]+$' "$folog" \
    || fail "the stage row lost its own usage to the subtree (the fold this row exists to refuse): $(cat "$folog")"
grep -q 'resolved no anchor' <<<"$foout" \
    && fail "an intact meta layer must resolve every dispatched transcript: $foout"
grep -q 'match no stamp and resolved no anchor' <<<"$foout" \
    && fail "the unstamped bound still counts transcripts the fan-out pass attributed: $foout"

: > "$folog"
fan '-subtree' >/dev/null
grep -q ' foiter build-subtree test-model ' "$folog" \
    || fail "DRIFT_KIT_FANOUT_SUFFIX did not name the row (the suffix is not a literal)"

: > "$folog"
printf 'foiter build fostage1 2025-01-01 none\nfoiter build fostage4 2025-01-01 none\nfoiter build+fanout nostage1 2025-01-01 none\n' > "$work/fo-state.txt"
focol="$(fan '+fanout')"
grep -q 'colliding with DRIFT_KIT_FANOUT_SUFFIX' <<<"$focol" \
    || fail "a stamp whose stage ends in the suffix did not raise the collision notice"
[[ "$(grep -c 'build+fanout test-model' "$folog")" -eq 0 ]] \
    || fail "the collision did not suppress the fan-out row"
printf 'foiter build fostage1 2025-01-01 none\nfoiter build fostage4 2025-01-01 none\n' > "$work/fo-state.txt"

# spec: drift-kit/SPEC.md §Testing — the fan-out fixture's two degradation assertions, neither of
# which may be dropped for brevity: they are what make the coupling's bound testable, not asserted.
: > "$folog"; fan '+fanout' >/dev/null
grep -v 'build+fanout' "$folog" | sort > "$work/fo-intact.txt"
rm -f "$fosub/agent-fogrand3feed.meta.json"
: > "$folog"
fodeg="$(fan '+fanout')"
grep -q '1 dispatched transcript(s) resolved no anchor' <<<"$fodeg" \
    || fail "a missing meta record must raise the counted unresolved notice: $fodeg"
grep -q '1 transcript(s) in the sessions dir match no stamp and resolved no anchor' <<<"$fodeg" \
    || fail "an unresolved transcript must fall back into the unstamped bound: $fodeg"
grep -qE ' foiter build\+fanout test-model in=1001 out=2002 cr=3003 cw=4004 ' "$folog" \
    || fail "the degraded run must still price every resolvable transcript, never guess the unresolved one: $(cat "$folog")"
grep -v 'build+fanout' "$folog" | sort > "$work/fo-degraded.txt"
diff -q "$work/fo-intact.txt" "$work/fo-degraded.txt" >/dev/null \
    || fail "a degraded fan-out pass changed a row it does not own: $(diff "$work/fo-intact.txt" "$work/fo-degraded.txt")"

# spec: drift-kit/SPEC.md §Testing — the second arm: the whole meta layer gone.
rm -f "$fosub"/*.meta.json
: > "$folog"
fonone="$(fan '+fanout')"
grep -q 'no dispatch-attribution records beside the transcripts' <<<"$fonone" \
    || fail "an absent meta layer must emit its single notice: $fonone"
[[ "$(grep -c 'build+fanout' "$folog")" -eq 0 ]] \
    || fail "an absent meta layer must emit no fan-out row at all"
sort "$folog" > "$work/fo-nometa.txt"
diff -q "$work/fo-intact.txt" "$work/fo-nometa.txt" >/dev/null \
    || fail "losing the meta layer changed a row it does not own: $(diff "$work/fo-intact.txt" "$work/fo-nometa.txt")"

# spec: drift-kit/SPEC.md §The knowledge-friction loop — the discriminating half of the capture arm's argv contract, the seam a crate unit test cannot see: the front-end resolves --emit kfric, a leading-dash field is still exit 2 in EITHER slot through the bridge, and the knowledge log is byte-unchanged after the refusal (a test reading exit codes alone passes the bug). The grammar cases are pinned in the ported module's own #[cfg(test)] tests, where check-crate-arms runs them; the -h/--help cases retired with the port, the help half belonging to the substrate.
kflog="$work/kfric-argv.log"
printf '2026-01-01 a real fact ← a real surface\n' > "$kflog"
cp "$kflog" "$work/kfric-argv.before"
kf() { DRIFT_KIT_KNOWLEDGE_LOG="$kflog" bash "$DRIFT_ARM" --emit kfric "$@"; }

for a in "--list" "second"; do
    kfrc=0
    if [[ "$a" == "--list" ]]; then kf --list a >/dev/null 2>&1 || kfrc=$?
    else kf "$a" --list >/dev/null 2>&1 || kfrc=$?; fi
    [[ "$kfrc" -eq 2 ]] || fail "--emit kfric refused a flag with exit $kfrc, want 2 (slot: $a)"
done
kfso="$(kf a --list 2>/dev/null)" || true
[[ -z "$kfso" ]] || fail "--emit kfric wrote usage to stdout on a refusal: $kfso"
cmp -s "$work/kfric-argv.before" "$kflog" \
    || fail "--emit kfric wrote the knowledge log on a refusal path"

# spec: gate-sdk/SPEC.md §The bin/-tool contract — '--' ends option processing, so the refusal never
# makes a legitimate filing unfileable; and a SET consumer knob reaches the arm's write through the
# shell bridge, which the refusal above cannot show.
kf -- "--list is captured at exit 0" "a surface" >/dev/null
grep -q -- '--list is captured at exit 0 ← a surface$' "$kflog" \
    || fail "--emit kfric -- did not file a fact beginning with a dash"
