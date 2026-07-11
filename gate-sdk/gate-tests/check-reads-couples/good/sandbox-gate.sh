#!/usr/bin/env bash
# graph: couples=corpus/*.md,corpus/sub/*.md dir=one valve=none tier=precommit
# A sandbox gate: its recursive corpus walk is fully covered by a two-level
# couple (the sibling glob reaches the deeper file). A second walk is exempt;
# a third has an undecidable root.
gate_find "corpus" -name '*.md' -type f
gate_find "." -type f  # reads-couples-exempt: sandbox whole-tree scan, refire owned elsewhere
gate_find "$DYNAMIC" -name '*.md' -type f
