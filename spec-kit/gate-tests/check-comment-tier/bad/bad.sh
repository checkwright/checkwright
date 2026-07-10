#!/usr/bin/env bash
# spec: some/SPEC.md §thing — the header itself is fine
set -uo pipefail

# spec: some/SPEC.md §over — this directive opens a three-line window
#   covering its own line and two continuations, so these first lines ride
#   it, but the paragraph keeps going past the cap and this fourth line
#   spills outside the window, flagged as relocated restatement to trim.
frob() { echo x; }

# This standalone block cites no owning section and carries no directive.
# It restates the design rationale that belongs in the SPEC, so both lines
# are flagged: relocate to the owning section or delete restated prose.
baz() { echo y; }

# spec: some/SPEC.md §count — this window ships six gates today, and the pinned
#   total is flagged even though the directive blesses the wording around it.
qux() { echo z; }

# usage: bad.sh --tally   (the ordered set is rules 1-8, pinned at both ends)
quux() { echo w; }
