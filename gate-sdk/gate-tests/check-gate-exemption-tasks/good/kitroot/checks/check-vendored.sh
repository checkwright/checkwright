#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): a KIT-SHIPPED declaration whose
# temporary dispositions name the kit author's queue, not this tree's. Both
# annotations are unresolvable here, and both must be OUT OF SCOPE — this case is
# the adopter reading, where the kit root was vendored rather than authored.
# graph: couples=docs/*.md dir=one valve=none tier=precommit
# port-until: a-slug-only-the-kit-authors-queue-carries
set -uo pipefail

# exception-list: surfaces the kit excuses, on the kit author's own schedule
EXEMPT=(
    "kit-surface"   # until: another-slug-only-the-kit-authors-queue-carries
)
echo "VENDORED: clean (${#EXEMPT[@]} exemptions)"
