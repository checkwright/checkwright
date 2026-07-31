# shellcheck shell=bash
# spec: drift-kit/SPEC.md §Layout and configuration — this repo's drift-kit consumer config; set only what this repo overrides beyond the kit defaults
# spec: drift-kit/SPEC.md §Layout and configuration — DRIFT_KIT_STAGES derives from lifecycle-config's LIFECYCLE_KIT_STAGES (the sole roster owner) so the trajectory table renders exactly this repo's live roster; a parallel literal here would reintroduce the roster-drift this wiring exists to eliminate
# shellcheck source=scripts/lifecycle-config.sh
source "${BASH_SOURCE[0]%/*}/lifecycle-config.sh"
# shellcheck disable=SC2034  # consumed by drift-kit/bin/trajectory.sh after sourcing
DRIFT_KIT_STAGES=("${LIFECYCLE_KIT_STAGES[@]}")
# spec: drift-kit/SPEC.md §Layout and configuration — the icebox counterpart of queue-config's and canon-config's section knob; the three are independent by kit and set together by a consumer enabling the tier
# shellcheck disable=SC2034  # consumed by drift-kit KPI plugins after the collator's export loop
DRIFT_KIT_ICEBOX_SECTION=Icebox
