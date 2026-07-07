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

# check-comment-tier landed after these sources were written, so their comment
# blocks predate the content-home rule. The sweep to relocate design rationale
# into the owning SPECs (and delete restated-code prose) is real prose work,
# deferred to the comment-tier-sweep task and drained kit by kit; this roster is
# the debt ledger — every clean and future source is gated now, and each exact
# path drops as its kit is swept. exception-list entries stay held to a live task
# by check-gate-exemption-tasks.
# exception-list: not-yet-swept sources (drain: comment-tier-sweep)
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_COMMENT_WHITELIST=(
    scripts/bash-guard.sh                          # until: comment-tier-sweep
    scripts/context-config.sh                      # until: comment-tier-sweep
    scripts/delegation-config.sh                   # until: comment-tier-sweep
    scripts/friction-config.sh                     # until: comment-tier-sweep
    scripts/session-context.sh                     # until: comment-tier-sweep
    scripts/spec-config.sh                         # until: comment-tier-sweep
)
