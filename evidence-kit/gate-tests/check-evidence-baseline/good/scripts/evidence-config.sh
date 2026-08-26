# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §check-evidence-baseline — the fixture's own suite roster, so the green case crosses the NON-EMPTY suite-coverage arm rather than the vacuous empty one
# shellcheck disable=SC2034  # consumed by evidence-kit/lib/evidence.sh after sourcing
EVIDENCE_KIT_SUITES=(gates unit)
