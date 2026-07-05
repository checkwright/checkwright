# shellcheck shell=bash
# Checkwright's spec-kit config (spec-kit/SPEC.md §Layout and configuration).
# This repo's component specs are the kit SPECs — a reference-spec corpus that
# documents contracts and legitimately carries no Definition-of-Done checklist,
# so check-spec-dod-singleton runs in at-most-one mode (a doubled DoD is still a
# violation; zero is allowed). Every other knob keeps the extracted default.
# check-surface-duplication is unregistered here — this repo has no glossary.
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_DOD_MODE=at-most-one
# The kits are this repo's own first-party components, not vendored dependencies,
# so the finders scan their SPEC.md/SPEC-*.md (the extraction default prunes kit
# roots, which is right for a consumer that merely vendored the kits beside
# gate-sdk — this repo is the exception that authors them).
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_SCAN_KIT_ROOTS=1
