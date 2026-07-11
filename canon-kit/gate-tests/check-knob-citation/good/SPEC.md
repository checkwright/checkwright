# Widget — a manifest that cites knob names without restating their values

Cite a knob by name and point at the owning roster: `GATE_SDK_STAGES_FILE`
names the stage list, and gate-sdk's SPEC owns its default — this prose states
no value, so nothing here can drift.

A knob whose default defers to another kit's knob is a name citation, not a
value statement: the queue resolves through `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`
when the outer knob is unset.

Saying a knob has a default is legal as long as the literal stays in the SPEC:
`GATE_SDK_WORKFLOW_DIR` has a default, and these paths default through
gate-sdk's `GATE_SDK_QUEUE_FILE` — the words name the owner, never the value.

A bare number with no knob token in reach is a human tripwire, not a gate hit:
the registry lists 42 rows.

<!-- knob-citation-exempt: a genuine local restatement rides the valve -->
`GATE_SDK_STAGES_FILE` defaults to `stages.list` in this vendored snapshot.
