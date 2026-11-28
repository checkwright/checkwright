# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-11-28 scope — Which deferred entries rank into the set after the carried bullets?
- corpus: TASK-QUEUE.md .workflow/gap-inbox.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges; bash gate-sdk/bin/run-gates.sh --emit queue-flow
- rev: a47079efbf6071c191473e517d8057091f8c94a7
- finding: the Deferred section's 17 entries and the four carried bullets: no candidate carries an inbound edge; every roadmap row is gated outside the tree; the takeable rows are features or design-pending except exit-status-funnel-typing (527 call sites, no in-tree 2-vs-3 reader); mean-filed 1.2
- inferred: that no deferred row's cost class is session or iteration, read from the tag lines
