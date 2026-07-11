# Widget — a manifest restating kit knob values it does not own

The environment pins a foreign knob to a literal: set
`GATE_SDK_STAGES_FILE=stages.list` before the run.

The default is restated where the owning SPEC should hold it alone:
`GATE_SDK_QUEUE_FILE` — default `TASK-QUEUE.md`.

The short-derived prefix is caught the same way: `QUEUE_KIT_QUEUE_FILE`
defaults to `queue.md`, a second home for a value queue-kit's SPEC owns.
