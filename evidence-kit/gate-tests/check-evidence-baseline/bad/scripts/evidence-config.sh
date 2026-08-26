# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §check-evidence-baseline — `ghost` is configured and carries no row, which is the suite-coverage violation this half of the pair holds beside the missing-slug one
# shellcheck disable=SC2034  # consumed by evidence-kit/lib/evidence.sh after sourcing
EVIDENCE_KIT_SUITES=(gates unit ghost)
