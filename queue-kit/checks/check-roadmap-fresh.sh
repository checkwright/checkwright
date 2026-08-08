#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,ROADMAP.md,scripts/queue-config.sh dir=one valve=none tier=precommit trigger=TASK-QUEUE.md,ROADMAP.md,scripts/queue-config.sh
# install: on-surface
# spec: queue-kit/SPEC.md §check-roadmap-fresh — every [roadmap:] tag names a configured horizon and track and pairs with exactly one roadmap-summary: declaration, and the projection page's marker block is the byte-fresh emission of bin/roadmap.sh
#
# usage: check-roadmap-fresh.sh [projection-file] [emit-file]
#   bare: compare the marker block in QUEUE_KIT_ROADMAP_FILE against `roadmap.sh --emit`.
#   two args: compare the block extracted from projection-file to a pre-baked emit-file (hermetic fixture).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

PROJECTION="${1:-$QUEUE_KIT_ROADMAP_FILE}"
EMIT_SRC="${2:-}"

# spec: queue-kit/SPEC.md §check-roadmap-fresh — no configured page is the clean skip for a consumer that publishes no roadmap, matching check-queue-slug-liveness's empty-globs behavior
if [[ -z "$PROJECTION" ]]; then
    echo "ROADMAP-FRESH: clean (QUEUE_KIT_ROADMAP_FILE empty — this consumer publishes no roadmap)"
    exit 0
fi

[[ -f "$QUEUE_KIT_QUEUE_FILE" ]] \
    || { echo "check-roadmap-fresh: queue file not found: $QUEUE_KIT_QUEUE_FILE" >&2; exit 2; }

# spec: queue-kit/SPEC.md §check-roadmap-fresh — assertion B runs first: a tag naming an unconfigured horizon is silently dropped from the emission, so a freshness verdict taken before the fields are validated would pass a page that quietly lost an item
horizons=" ${QUEUE_KIT_HORIZONS[*]+"${QUEUE_KIT_HORIZONS[*]}"} "
tracks=" ${QUEUE_KIT_TRACKS[*]+"${QUEUE_KIT_TRACKS[*]}"} "
bad=()
tagged=0
while IFS=$'\t' read -r ntags fieldv slug nsum _summary; do
    [[ -n "$slug" ]] || continue

    # spec: queue-kit/SPEC.md §check-roadmap-fresh — assertion C, both directions: the tag decides projection and the declaration is the only prose that may be projected, so neither is meaningful without the other
    if [[ "$ntags" -eq 0 ]]; then
        bad+=("$slug: carries a roadmap-summary: declaration but no [roadmap:] tag; a dead marking is what a dropped or reflowed tag looks like from the page's side")
        continue
    fi
    tagged=$((tagged + 1))
    # spec: queue-kit/SPEC.md §check-roadmap-fresh — the declaration count and the tag's fields are independent, so both are reported in one run rather than costing the author a second round trip
    [[ "$nsum" -eq 1 ]] \
        || bad+=("$slug: is [roadmap:]-tagged but carries $nsum roadmap-summary: declaration(s); exactly one is required")

    if [[ "$ntags" -ne 1 ]]; then
        bad+=("$slug: carries $ntags [roadmap:] tags; an entry takes at most one")
        continue
    fi
    if [[ "$fieldv" != */* || "$fieldv" == */*/* ]]; then
        bad+=("$slug: field '$fieldv' does not parse as <horizon>/<track>")
        continue
    fi
    h="${fieldv%%/*}"; t="${fieldv#*/}"
    [[ "$horizons" == *" $h "* ]] || bad+=("$slug: unknown horizon '$h'")
    [[ "$tracks" == *" $t "* ]]   || bad+=("$slug: unknown track '$t'")
done < <(queue_roadmap_entries "$QUEUE_KIT_QUEUE_FILE")

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-roadmap-fresh: invalid roadmap curation in $QUEUE_KIT_QUEUE_FILE"
    echo "(an unconfigured value or an unpaired marking drops the entry off the page"
    echo "with nothing else to notice):"
    for b in "${bad[@]}"; do echo "  $b"; done
    echo "  help: an entry joins the page with a [roadmap: <horizon>/<track>] tag on its lead"
    echo "        line, both values drawn from QUEUE_KIT_HORIZONS / QUEUE_KIT_TRACKS, plus"
    echo "        exactly one indented 'roadmap-summary: <text>' declaration in its body —"
    echo "        the only prose the public page prints (queue-kit/SPEC.md §bin/roadmap.sh)."
    exit 1
fi

BEGIN="<!-- ${QUEUE_KIT_ROADMAP_MARKER}:begin -->"
END="<!-- ${QUEUE_KIT_ROADMAP_MARKER}:end -->"

# spec: queue-kit/SPEC.md §check-roadmap-fresh — a configured path with no page, no markers, or a half marker pair is a broken install, not a clean skip
[[ -f "$PROJECTION" ]] || { echo "check-roadmap-fresh: projection not found: $PROJECTION" >&2; exit 2; }
nb="$(grep -cF -- "$BEGIN" "$PROJECTION")"
ne="$(grep -cF -- "$END" "$PROJECTION")"
[[ "$nb" -eq 1 && "$ne" -eq 1 ]] || {
    echo "check-roadmap-fresh: $PROJECTION needs exactly one '$BEGIN' + '$END' pair (found $nb and $ne)" >&2
    exit 2
}

block="$(awk -v b="$BEGIN" -v e="$END" '
    $0 == b { inb = 1; next }
    $0 == e { inb = 0; next }
    inb     { print }
' "$PROJECTION")"; st=$?
fail_closed "$st" check-roadmap-fresh awk

if [[ -n "$EMIT_SRC" ]]; then
    [[ -f "$EMIT_SRC" ]] || { echo "check-roadmap-fresh: emit source not found: $EMIT_SRC" >&2; exit 2; }
    emitted="$(cat "$EMIT_SRC")"; st=$?
    fail_closed "$st" check-roadmap-fresh cat
else
    GEN="$KIT/bin/roadmap.sh"
    [[ -x "$GEN" ]] || { echo "check-roadmap-fresh: emitter not found: $GEN" >&2; exit 2; }
    emitted="$(bash "$GEN" --emit)"; st=$?
    fail_closed "$st" check-roadmap-fresh roadmap
fi

if [[ "$block" != "$emitted" ]]; then
    echo "check-roadmap-fresh: the $QUEUE_KIT_ROADMAP_MARKER block in $PROJECTION is stale vs bin/roadmap.sh:"
    diff <(printf '%s\n' "$emitted") <(printf '%s\n' "$block") | head -20 || true
    echo "  help: regenerate — bash queue-kit/bin/roadmap.sh --write"
    exit 1
fi

echo "ROADMAP-FRESH: clean (the $QUEUE_KIT_ROADMAP_MARKER block in $PROJECTION byte-matches bin/roadmap.sh; $tagged tagged entry/entries name a configured horizon and track)"
exit 0
