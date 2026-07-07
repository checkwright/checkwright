#!/usr/bin/env bash
# spec: some/SPEC.md §thing — the header itself is fine
set -uo pipefail

# This standalone block cites no owning section and carries no directive.
# It restates the design rationale that belongs in the SPEC, so both lines
# are flagged: relocate to the owning section or delete restated prose.
frob() { echo x; }
