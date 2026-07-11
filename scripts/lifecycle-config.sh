# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's lifecycle-kit consumer config: the boundary-truncate and entry-preflight knobs wire evidence-kit's manifest across the seam; every other knob keeps the platform default
# shellcheck disable=SC2034  # consumed by lifecycle-kit/lib/stages.sh after sourcing
LIFECYCLE_KIT_BOUNDARY_TRUNCATE=(.workflow/validate-evidence.txt)
LIFECYCLE_KIT_ENTRY_PREFLIGHT=('close=evidence-kit/checks/check-evidence-manifest.sh .workflow/validate-evidence.txt')
