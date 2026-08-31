# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — the knob table (every knob, its default, when to set it) and how to install this config; set only what you override
# no-port: gate-sdk/SPEC.md §The config-seam port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this file is delegation-kit's side of the adopter's config seam — the template `init` seeds into `<gates-dir>/delegation-config.sh` under installer/README.md §What init seeds' derivation — so it IS the surface an adopter edits rather than kit mechanism reaching one, and porting it leaves nothing to edit. Its one content line is DELEGATION_KIT_READONLY_TYPES commented out and empty, because the read-only agent-type names are the consumer's own vocabulary a kit must never carry (CLAUDE.md §The provenance seam) — a line that exists to be filled in by the adopter is the seam at its sharpest. Structural, not a sizing judgment.

# spec: delegation-kit/SPEC.md §Layout and configuration — D2's roster: the consumer's own read-only-dispatched agent-type names; default empty (D2 inert) — never a kit-shipped vocabulary (CLAUDE.md §The provenance seam)
# shellcheck disable=SC2034  # consumed by the --hook agent-dispatch-guard arm, which the config bridge resolves this roster for
# DELEGATION_KIT_READONLY_TYPES=()
