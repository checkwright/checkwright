#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §enforcement-map — degraded-registry coverage the hermetic fixtures cannot reach: a knob unset with its default absent drops exactly its own section (the gate registry — gate-sdk's core — always remains), while a knob explicitly set to a missing path refuses (exit 2, empty stdout, stderr names the knob)
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT" || exit 2
EMIT="gate-sdk/bin/enforcement-map.sh"
[[ -x "$EMIT" ]] || { echo "enforcement-map.test: emitter not found: $EMIT"; exit 2; }

emptydir="$(mktemp -d)"; scandir="$(mktemp -d)"; scratch="$(mktemp -d)"
trap 'rm -rf "$emptydir" "$scandir" "$scratch"' EXIT

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }

# The baseline asserts every real registry projects its section. Evidence
# routes through EVIDENCE_KIT_CONFIG_FILE, which the hermetic bootstrap pins to
# an empty file; drop that pin here so the emitter resolves this repo's real
# evidence-config.sh (the other registries already read their real defaults).
base="$(env -u EVIDENCE_KIT_CONFIG_FILE bash "$EMIT" --emit)"
for section in "## Blocking gates" "## Advisory KPIs" "## Guards" "## Session warnings" "## Validate suites" "## Monitors"; do
    assert_has baseline "$section" "$base"
done

# spec: gate-sdk/SPEC.md §enforcement-map — adopted-but-broken refuses: a knob
# explicitly set to a missing path exits 2 before the first stdout byte, the
# stderr line naming the knob; the settings knob also refuses a set file jq
# cannot parse.
errfile="$scratch/stderr"
assert_strict() {  # $1=case $2=knob-name $3=knob-value
    local out rc
    out="$(env "$2=$3" bash "$EMIT" --emit 2>"$errfile")"; rc=$?
    [[ "$rc" -eq 2 ]]  || { echo "FAIL [$1]: expected exit 2, got $rc"; fails=$((fails + 1)); }
    [[ -z "$out" ]]    || { echo "FAIL [$1]: expected empty stdout"; fails=$((fails + 1)); }
    grep -qF -- "$2" "$errfile" || { echo "FAIL [$1]: stderr does not name $2"; fails=$((fails + 1)); }
}
assert_strict strict-kpis     DRIFT_KIT_KPIS_FILE        /nonexistent/kpis.list
assert_strict strict-settings CONTEXT_KIT_SETTINGS_FILE  /nonexistent/settings.json
assert_strict strict-evidence EVIDENCE_KIT_CONFIG_FILE   /nonexistent/evidence-config.sh
assert_strict strict-scandir  GATE_SDK_ENFORCE_SCAN_DIR  /nonexistent/scandir
printf '{ not json\n' > "$scratch/bad-settings.json"
assert_strict strict-settings-unparseable CONTEXT_KIT_SETTINGS_FILE "$scratch/bad-settings.json"

# spec: gate-sdk/SPEC.md §enforcement-map — not-adopted degrades: an unset knob
# whose default path is absent drops exactly its own section. A scratch
# GATE_SDK_GATES_DIR holding only a gates.list copy leaves the kpis.list and
# evidence-config.sh defaults absent; adding kpis.list back restores only the
# KPI section, so the sections drop independently.
mkdir -p "$scratch/gates"
cp "scripts/gates.list" "$scratch/gates/gates.list"
deg="$(env -u DRIFT_KIT_KPIS_FILE -u EVIDENCE_KIT_CONFIG_FILE GATE_SDK_GATES_DIR="$scratch/gates" bash "$EMIT" --emit)"
assert_absent degrade-both "## Advisory KPIs" "$deg"
assert_absent degrade-both "## Validate suites" "$deg"
assert_has    degrade-both "## Blocking gates" "$deg"
cp "scripts/kpis.list" "$scratch/gates/kpis.list"
deg2="$(env -u DRIFT_KIT_KPIS_FILE -u EVIDENCE_KIT_CONFIG_FILE GATE_SDK_GATES_DIR="$scratch/gates" bash "$EMIT" --emit)"
assert_has    degrade-independent "## Advisory KPIs" "$deg2"
assert_absent degrade-independent "## Validate suites" "$deg2"
assert_has    degrade-independent "## Blocking gates" "$deg2"

# The settings default (.claude/settings.json) is cwd-relative: run from a cwd
# without one, the unset knob degrades — both hook sections drop, the gate
# registry (reached through an absolute GATE_SDK_GATES_DIR) remains.
nohooks="$(cd "$emptydir" && env -u CONTEXT_KIT_SETTINGS_FILE -u DRIFT_KIT_KPIS_FILE -u EVIDENCE_KIT_CONFIG_FILE GATE_SDK_GATES_DIR="$scratch/gates" bash "$ROOT/$EMIT" --emit)"
assert_absent no-hooks "## Guards" "$nohooks"
assert_absent no-hooks "## Session warnings" "$nohooks"
assert_has    no-hooks "## Blocking gates" "$nohooks"

nomon="$(GATE_SDK_ENFORCE_SCAN_DIR="$emptydir" bash "$EMIT" --emit)"
assert_absent no-monitors "## Monitors" "$nomon"
assert_has    no-monitors "## Blocking gates" "$nomon"

# spec: gate-sdk/SPEC.md §enforcement-map — a marker is dormant in a template
# (an inert copy-source) and activates only where a consumer copies it live: a
# live marker projects a row, an identical one under templates/ does not.
mkdir -p "$scandir/live" "$scandir/templates"
printf '# enforce: class=monitor live-surface-alpha\n' > "$scandir/live/probe.yml"
printf '# enforce: class=monitor template-surface-beta\n' > "$scandir/templates/probe.yml"
tmpl="$(GATE_SDK_ENFORCE_SCAN_DIR="$scandir" bash "$EMIT" --emit)"
assert_has    template-inert "live-surface-alpha" "$tmpl"
assert_absent template-inert "template-surface-beta" "$tmpl"

[[ "$fails" -eq 0 ]] || { echo "enforcement-map.test: $fails assertion(s) failed"; exit 1; }
echo "enforcement-map.test: clean (set-but-missing knobs refuse; unset-with-default-absent sections drop independently; gate registry always present)"
exit 0
