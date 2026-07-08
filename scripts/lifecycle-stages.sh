# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's lifecycle-kit consumer config: only the boundary-truncate knob is overridden; every other knob keeps the platform default
# shellcheck disable=SC2034  # consumed by lifecycle-kit/lib/stages.sh after sourcing
LIFECYCLE_BOUNDARY_TRUNCATE=(.workflow/validate-evidence.txt)
