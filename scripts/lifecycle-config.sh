# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's lifecycle-kit consumer config: the boundary-truncate and entry-preflight knobs wire evidence-kit's manifest across the seam, the boundary-require knob makes close's release disposition a boundary precondition, the session-boundary posture is set below; every other knob keeps the platform default
# shellcheck disable=SC2034  # consumed by lifecycle-kit/lib/stages.sh after sourcing
LIFECYCLE_KIT_BOUNDARY_TRUNCATE=(.workflow/validate-evidence.txt .workflow/release-disposition.txt)
LIFECYCLE_KIT_BOUNDARY_REQUIRE=(.workflow/release-disposition.txt)
LIFECYCLE_KIT_ENTRY_PREFLIGHT=('close=evidence-kit/checks/check-evidence-manifest.sh .workflow/validate-evidence.txt')
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's session-boundary posture: 'iteration' sanctions the lead's inline fallback; cost accepted that the dogfood evidence stops demonstrating the strict posture
LIFECYCLE_KIT_SESSION_BOUNDARY=iteration
# spec: lifecycle-kit/SPEC.md §The close-surface roster — this repo's declaration surfaces beyond the kit SPECs: the always-loaded agent file, the doctrine deliverable, and the stage-skill bindings, which own the consumer-side capture surfaces no kit may name
LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS=("*/SPEC.md" "CLAUDE.md" "doctrine-kit/DOCTRINE.md" ".claude/commands/*.md")
# spec: lifecycle-kit/SPEC.md §check-stage-entry — this repo splits amendment authoring into a dedicated trigger-gated `spec` stage (scope bounds, spec authors, align verifies); the roster and its `[spec]=scope` predecessor edge are set together — a roster member absent from the predecessor map fails config-load validation. Only `scope` (LIFECYCLE_KIT_FIRST_STAGE) resets the evidence file; `spec` appends. `spec` is omitted as any stage's mandatory predecessor (same calibration as the trigger-gated audit stage), so align/build/validate/close edges are unchanged.
LIFECYCLE_KIT_STAGES=(scope spec align build validate close)
declare -A LIFECYCLE_KIT_PREDECESSOR=([spec]=scope [align]=scope [build]=scope [validate]=build [close]=validate)
