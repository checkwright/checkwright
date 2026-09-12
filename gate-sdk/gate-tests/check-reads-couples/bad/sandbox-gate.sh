#!/usr/bin/env bash
# graph: couples=corpus/*.md dir=one valve=none tier=precommit
# The check-shim-restatement shape: a recursive walk under a shallow one-level couple.
# corpus/sub/deep.md is one level deeper than the couple reaches segment-wise, so this
# violates the authoring rule that globs never cross '/' — which is what this gate asserts.
# NOT a prediction about the trigger: the hook's matcher spans '/', so `corpus/*.md` does
# fire on corpus/sub/deep.md. The old wording said the edit "would not fire this gate" and
# was false about the deployed matcher; it misled four readings in one iteration and nearly
# shipped a change that flipped this very case to exit 0.
gate_find "corpus" -name '*.md' -type f
