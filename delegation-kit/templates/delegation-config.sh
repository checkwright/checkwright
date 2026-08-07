# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — the knob table (every knob, its default, when to set it) and how to install this config; set only what you override

# spec: delegation-kit/SPEC.md §Layout and configuration — D2's roster: the consumer's own read-only-dispatched agent-type names; default empty (D2 inert) — never a kit-shipped vocabulary (CLAUDE.md §The provenance seam)
# shellcheck disable=SC2034  # consumed by templates/agent-dispatch-guard.sh after sourcing
# DELEGATION_KIT_READONLY_TYPES=()
