#!/usr/bin/env bash
# graph: couples=corpus/*.md dir=one valve=none tier=precommit
# The check-shim-restatement shape: a recursive walk under a shallow one-level
# couple. corpus/sub/deep.md is one level deeper than the couple reaches, so an
# edit to it would not fire this gate — the reads-not-subset-of-couples bug.
gate_find "corpus" -name '*.md' -type f
