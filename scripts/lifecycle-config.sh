# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's lifecycle-kit consumer config: the boundary-truncate and entry-preflight knobs wire evidence-kit's manifest across the seam, the boundary-require knob makes close's release disposition a boundary precondition, the session-boundary posture is set below; every other knob keeps the platform default
# shellcheck disable=SC2034  # consumed by lifecycle-kit/lib/stages.sh after sourcing
LIFECYCLE_KIT_BOUNDARY_TRUNCATE=(.workflow/validate-evidence.txt .workflow/release-disposition.txt)
LIFECYCLE_KIT_BOUNDARY_REQUIRE=(.workflow/release-disposition.txt)
LIFECYCLE_KIT_ENTRY_PREFLIGHT=('close=evidence-kit/checks/check-evidence-manifest.sh .workflow/validate-evidence.txt')
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's session-boundary posture: 'iteration' sanctions the lead's inline fallback; cost accepted that the dogfood evidence stops demonstrating the strict posture
LIFECYCLE_KIT_SESSION_BOUNDARY=iteration
