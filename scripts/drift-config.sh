# shellcheck shell=bash
# spec: drift-kit/SPEC.md §Layout and configuration — this repo's drift-kit consumer config; set only what this repo overrides beyond the kit defaults
# no-port: gate-sdk/SPEC.md §The config-seam port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this is the seeded-copy side of drift-kit's config seam, the file this repo actually edits and the more edited of the two, so porting it deletes the seam outright. It is also the file whose own template already declares on this exact ground, so leaving it owed was the incoherence the class ruling closes: the copy an adopter edits was denied a ground its template held. The 2026-08-24 vocabulary ruling left it owed deliberately — its stage roster derives from lifecycle-config and it holds no literal roster of its own — which answers "does this hold private vocabulary?" and never "is this an edit seam?"; that verdict is untouched here and the two grounds are cumulative. Structural, not a sizing judgment.
# spec: drift-kit/SPEC.md §Layout and configuration — DRIFT_KIT_STAGES derives from lifecycle-config's LIFECYCLE_KIT_STAGES (the sole roster owner) so the trajectory table renders exactly this repo's live roster; a parallel literal here would reintroduce the roster-drift this wiring exists to eliminate
# shellcheck source=scripts/lifecycle-config.sh
source "${BASH_SOURCE[0]%/*}/lifecycle-config.sh"
# shellcheck disable=SC2034  # consumed by drift-kit/lib/drift.sh's readers after sourcing
DRIFT_KIT_STAGES=("${LIFECYCLE_KIT_STAGES[@]}")
# spec: drift-kit/SPEC.md §Layout and configuration — the icebox counterpart of queue-config's and canon-config's section knob; the three are independent by kit and set together by a consumer enabling the tier
# shellcheck disable=SC2034  # consumed by drift-kit KPI plugins after the collator's export loop
DRIFT_KIT_ICEBOX_SECTION=Icebox
