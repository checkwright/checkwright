#!/usr/bin/env bash
# graph: couples=corpus/sub/*.md dir=one valve=none tier=precommit
# A recursive walk over corpus under a couple that starts one prefix too deep: corpus/top.md is
# outside the string reach of corpus/sub/*.md, so an edit to it fires no trigger — the miss this
# gate reds on.
gate_find "corpus" -name '*.md' -type f
