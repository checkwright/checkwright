Execute the template at delegation-kit/templates/agent-execution.md, applying the bindings below.

## Bindings

**shared-file-roster** — the generated `scripts/git-hooks/pre-commit` and
`docs/check-graph.html` (regenerate through the owning unit, never
hand-edit); `scripts/gates.list` with the `scripts/*-config.sh` knob files;
`TASK-QUEUE.md` with `.workflow/WORKFLOW-STATE.txt`; and any `SPEC-*.md`
amendment a unit is mid-merge on — the git index and HEAD besides, shared by
every committing agent regardless of source disjointness.

**validate-battery** — `bash gate-sdk/bin/run-gates.sh`, then the touched kit's
`run-gate-tests.sh <kit>/gate-tests <kit>/checks` row (the `*/gate-tests`
derivation CI and validate loop over); the full battery-and-fixture roster is
CLAUDE.md §This repo is governed by its own kits. After a rename, re-run
`check-graph` and confirm the renamed gate's fixture pair still resolves.
