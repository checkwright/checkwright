#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §enforcement-map — degraded-registry coverage the hermetic fixtures cannot reach: each optional registry absent drops exactly its own section, and the gate registry (gate-sdk's core) always remains
set -uo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT" || exit 2
EMIT="gate-sdk/bin/enforcement-map.sh"
[[ -x "$EMIT" ]] || { echo "enforcement-map.test: emitter not found: $EMIT"; exit 2; }

emptydir="$(mktemp -d)"
trap 'rm -rf "$emptydir"' EXIT

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }

base="$(bash "$EMIT" --emit)"
for section in "## Blocking gates" "## Advisory KPIs" "## Guards" "## Session warnings" "## Validate suites" "## Monitors"; do
    assert_has baseline "$section" "$base"
done

nokpi="$(DRIFT_KIT_KPIS_FILE=/nonexistent/kpis.list bash "$EMIT" --emit)"
assert_absent no-kpis "## Advisory KPIs" "$nokpi"
assert_has    no-kpis "## Blocking gates" "$nokpi"

nohooks="$(CONTEXT_KIT_SETTINGS_FILE=/nonexistent/settings.json bash "$EMIT" --emit)"
assert_absent no-hooks "## Guards" "$nohooks"
assert_absent no-hooks "## Session warnings" "$nohooks"
assert_has    no-hooks "## Blocking gates" "$nohooks"

noev="$(EVIDENCE_KIT_CONFIG_FILE=/nonexistent/evidence-config.sh bash "$EMIT" --emit)"
assert_absent no-evidence "## Validate suites" "$noev"
assert_has    no-evidence "## Blocking gates" "$noev"

nomon="$(GATE_SDK_ENFORCE_SCAN_DIR="$emptydir" bash "$EMIT" --emit)"
assert_absent no-monitors "## Monitors" "$nomon"
assert_has    no-monitors "## Blocking gates" "$nomon"

# spec: gate-sdk/SPEC.md §enforcement-map — a marker is dormant in a template
# (an inert copy-source) and activates only where a consumer copies it live: a
# live marker projects a row, an identical one under templates/ does not.
scandir="$(mktemp -d)"; trap 'rm -rf "$emptydir" "$scandir"' EXIT
mkdir -p "$scandir/live" "$scandir/templates"
printf '# enforce: class=monitor live-surface-alpha\n' > "$scandir/live/probe.yml"
printf '# enforce: class=monitor template-surface-beta\n' > "$scandir/templates/probe.yml"
tmpl="$(GATE_SDK_ENFORCE_SCAN_DIR="$scandir" bash "$EMIT" --emit)"
assert_has    template-inert "live-surface-alpha" "$tmpl"
assert_absent template-inert "template-surface-beta" "$tmpl"

[[ "$fails" -eq 0 ]] || { echo "enforcement-map.test: $fails assertion(s) failed"; exit 1; }
echo "enforcement-map.test: clean (degraded-registry sections drop independently; gate registry always present)"
exit 0
