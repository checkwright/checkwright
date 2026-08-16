Execute the template at delegation-kit/templates/agent-execution.md, applying the bindings below.

## Bindings

**shared-file-roster** — the generated `scripts/git-hooks/pre-commit` and
`docs/check-graph.html` (regenerate through the owning unit, never
hand-edit); `scripts/gates.list` with the `scripts/*-config.sh` knob files;
`TASK-QUEUE.md` with `.workflow/WORKFLOW-STATE.txt`; and any `SPEC-*.md`
amendment a unit is mid-merge on — the git index and HEAD besides, shared by
every committing agent regardless of source disjointness.

**validate-battery** — `bash gate-sdk/bin/run-gates.sh`, then the
`run-gate-tests.sh <kit>/gate-tests <kit>/checks` row for the kit **owning each
gate whose behaviour or output text the commit changed** — which is where that
gate's fixture pair lives, and is not always a kit the commit edited. Selecting
by edited kit instead is attested to miss: a commit that changed a
`scripts/`-owned gate's stale message ran gate-sdk's and context-kit's rows and
never `scripts/`, and the stale fixture surfaced a stage later as a validate red.
The first arm structurally cannot cover this — `run-gates.sh` runs gates, not
their fixture pairs, so a gate whose message moved still passes itself. When the
owning set is unclear, run every row; the full roster is CLAUDE.md §This repo is
governed by its own kits. After a rename, re-run `check-graph` and confirm the
renamed gate's fixture pair still resolves.
