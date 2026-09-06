#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The ruling-staleness probe — the seam a crate unit test cannot see:
# that the battery runner's --emit front-end resolves the arm at all, and that each of the three
# consumer knobs actually reaches the report through the shell bridge. The discriminating case is
# the EMPTIED record knob, which a hardcoded implementation passes against this repo's own config
# and fails only here. The bands, the manual operand, the undeclared closure and the verbatim citing
# row are pinned in the module's own #[cfg(test)] tests, where check-crate-arms runs them.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUN_GATES="$ROOT/gate-sdk/bin/run-gates.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

git -C "$SANDBOX" init -q
cat >"$SANDBOX/RECORD.md" <<'EOF'
# a ruling record

**A ruling that directed work.** Discharge event: the widget lands.
discharge: widget-ruling  echo the widget landed
ruling: widget-ruling  the widget run

**A ruling whose condition no command settles.** Discharge event: the brief says so.
discharge: brief-ruling  manual the untracked brief records it

**A conditioned ruling nobody declared.** Discharge event: the gate turns green.
EOF
cat >"$SANDBOX/CITERS.md" <<'EOF'
the widget run was the rule until it was retired
the widget run is the rule and binds this cut
EOF

# --- the front-end resolves the arm at all, and the record knob reaches it through the bridge ---
checks=$((checks + 1))
out="$( cd "$ROOT" && env LIFECYCLE_KIT_RULING_RECORD="$SANDBOX/RECORD.md" \
    bash "$RUN_GATES" --emit ruling-staleness 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note resolve "the front-end did not resolve --emit ruling-staleness (exit $rc): $out"
grep -q 'widget-ruling	fired' <<<"$out" \
    || note bridge-record "a configured record did not reach the arm through the bridge: $out"
grep -q 'brief-ruling	manual' <<<"$out" \
    || note manual-band "a manual condition did not report as owed to judgment: $out"
grep -q 'undeclared conditions' <<<"$out" \
    || note undeclared-section "the report carried no undeclared section: $out"

# --- the discriminating case: an emptied record knob reports configured-off, never a false clean ---
checks=$((checks + 1))
out="$( cd "$ROOT" && env LIFECYCLE_KIT_RULING_RECORD= \
    bash "$RUN_GATES" --emit ruling-staleness 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note empty-record-rc "an unconfigured record did not exit 0 (exit $rc): $out"
grep -q 'no ruling record configured' <<<"$out" \
    || note empty-record "an emptied record knob still reported against some record: $out"

# --- the citers knob reaches the citing sweep, and both readings of one name come back ---
checks=$((checks + 1))
out="$( cd "$SANDBOX" && env LIFECYCLE_KIT_RULING_RECORD=RECORD.md \
    LIFECYCLE_KIT_RULING_CITERS='CITERS.md' \
    bash "$RUN_GATES" --emit ruling-staleness 2>&1 )"
[[ "$(grep -c 'CITERS.md:' <<<"$out")" -eq 2 ]] \
    || note bridge-citers "the configured citing corpus did not yield both rows verbatim: $out"

# --- the timeout knob reaches the dispatcher: an oracle outrunning it is a dispatch failure ---
checks=$((checks + 1))
printf '**Slow.** Discharge event: never.\ndischarge: slow-ruling  sleep 5\n' >"$SANDBOX/SLOW.md"
out="$( cd "$ROOT" && env LIFECYCLE_KIT_RULING_RECORD="$SANDBOX/SLOW.md" \
    LIFECYCLE_KIT_RULING_ORACLE_TIMEOUT=1 \
    bash "$RUN_GATES" --emit ruling-staleness 2>&1 )"
grep -q 'slow-ruling	dispatch-failure' <<<"$out" \
    || note bridge-timeout "the configured bound did not reach the dispatcher: $out"

# --- a configured record that does not exist is a refusal, never an empty clean answer ---
checks=$((checks + 1))
( cd "$ROOT" && env LIFECYCLE_KIT_RULING_RECORD="$SANDBOX/nope.md" \
    bash "$RUN_GATES" --emit ruling-staleness >/dev/null 2>&1 )
[[ "$?" -eq 2 ]] || note missing-file "a configured-but-absent record did not exit 2"

if [[ "$fails" -gt 0 ]]; then
    echo "ruling-staleness.test.sh: $fails case(s) failed"
    exit 1
fi
echo "ruling-staleness.test.sh: clean (the --emit front-end resolves the arm; the record, citers and timeout knobs each reach the report through the bridge; an emptied record reports configured-off and an absent one refuses; $checks checks)"
exit 0
