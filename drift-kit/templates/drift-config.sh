# shellcheck shell=bash
# no-port: drift-kit/SPEC.md §Layout and configuration — two lines, both comments, seeded into the adopter's own gates dir. It **is** the adopter's config seam rather than kit mechanism reaching it, so porting it deletes the seam: there would be nothing left for a consumer to edit. Structural, and stated rather than cited for the reason lib/drift.sh's declaration gives.
# spec: drift-kit/SPEC.md §Layout and configuration — the knob table (every knob, its default, when to set it) and how to install this config; set only what you override
