#!/usr/bin/env bash
# spec: CLAUDE.md §Housekeeping — narrated end-to-end adoption walkthrough on the consumer-smoke mechanics; exit 0 asserts the whole arc (vendor → clean pass → violation blocked → fix → green), the evidence-kit 'demo' validate suite each validate stage re-runs.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$REPO/gate-sdk/lib/gate.sh"

# spec: CLAUDE.md §Housekeeping — DEMO_TMP_DIR is the only knob; default is the smoke harness's scratch base
BASE="${DEMO_TMP_DIR:-${TMPDIR:-/tmp}}"
SCRATCH="$(mktemp -d "$BASE/demo-consumer.XXXXXX")"
cleanup() { rm -rf "$SCRATCH"; }
trap cleanup EXIT

banner() {
    printf '\n'
    printf '════════════════════════════════════════════════════════════\n'
    printf '  %s\n' "$1"
    printf '════════════════════════════════════════════════════════════\n'
}
say() { printf '  %s\n' "$*"; }
fail() { printf '\nDEMO: FAIL — %s\n' "$*"; exit 1; }

run_battery() { ( cd "$SCRATCH" && bash gate-sdk/bin/run-gates.sh ) 2>&1; }

roots=()
while IFS= read -r r; do roots+=("$r"); done < <(gate_kit_roots)

banner "ACT 1 — Vendor the kits into a fresh consumer"
say "A new project adopts Checkwright by vendoring the kits and running each"
say "kit's installer. No global install, no network — the kits are copied in."
git -C "$SCRATCH" init -q
printf '.tmp/\n' > "$SCRATCH/.gitignore"
git -C "$SCRATCH" add -A
git -C "$SCRATCH" -c user.email=demo@example.invalid -c user.name=demo \
    commit -q --allow-empty -m "seed"
for r in "${roots[@]}"; do
    cp -R "$r" "$SCRATCH/$(basename "$r")"
done
for r in "${roots[@]}"; do
    kit="$(basename "$r")"
    say "vendor + install: $kit"
    ( cd "$SCRATCH" && SMOKE_KIT_ROOT="$SCRATCH/$kit" bash "$SCRATCH/$kit/smoke/install.sh" ) \
        || fail "$kit installer errored"
done
say "→ gates.list written, pre-commit hook generated. The consumer is governed."

banner "ACT 2 — A clean commit passes the battery"
say "With the kits installed and zero further config, the gate battery is green."
git -C "$SCRATCH" add -A
git -C "$SCRATCH" -c user.email=demo@example.invalid -c user.name=demo \
    commit -q --no-verify -m "installed baseline"
out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    printf '%s\n' "$out"
    fail "the battery was not green on the freshly installed consumer"
fi
say "$(grep -E 'All [0-9]+ gates passed' <<<"$out")"

banner "ACT 3 — A violation is caught before it lands"
say "A contributor introduces a defect — here, gate-sdk's craftable smoke"
say "violation (an unread shell variable). The battery must turn red and name"
say "the gate, with its finding and a help line pointing at the remedy."
vio="$SCRATCH/gate-sdk/smoke/violation.sh"
expected="$( ( cd "$SCRATCH" && SMOKE_KIT_ROOT="$SCRATCH/gate-sdk" bash "$vio" ) | head -n1 )"
[[ -n "$expected" ]] || fail "the violation script printed no expected-gate name"
say "expected gate: $expected"
out="$(run_battery)"; rc=$?
[[ "$rc" -ne 0 ]] || fail "the violation did not turn the battery red (expected $expected)"
grep -qF "FAIL: $expected" <<<"$out" || { printf '%s\n' "$out"; fail "a gate other than $expected caught it"; }
printf '\n'
awk -v g="$expected" '
    $0 ~ ("===== " g " =====") { on=1 }
    on { print "  | " $0 }
    on && $0 ~ ("FAIL: " g) { exit }
' <<<"$out"
say "→ blocked. The pre-commit hook would have rejected this commit."

banner "ACT 4 — Fix, re-run, green"
say "The contributor drops the offending change and re-runs; the battery is"
say "green again — the same gate that blocked now passes."
( cd "$SCRATCH" && git reset -q --hard && git clean -qfd )
out="$(run_battery)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qE 'All [0-9]+ gates passed' <<<"$out"; then
    printf '%s\n' "$out"
    fail "the battery did not return to green after the fix"
fi
say "$(grep -E 'All [0-9]+ gates passed' <<<"$out")"

banner "DEMO: clean — the full adoption arc behaved"
say "vendor → clean pass → violation blocked → fix → green"
exit 0
