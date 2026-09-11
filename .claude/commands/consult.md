Execute the template at lifecycle-kit/templates/consult.md, applying the bindings below.

## Bindings

**entry-reading** — three surfaces, in this order: `TRAJECTORY.md` (the
objectives of the running pivot and any standing ruling), `TASK-QUEUE.md` (the
live queue and the Deferred rungs with their costed grounds), and
`BRIEF.local.md` (the private companion brief — untracked, and the surface that
carries what no public file may). Read `ROADMAP.md` only as a projection of the
queue, never as a second source.

**landing-surfaces** — by class (lifecycle-kit/SPEC.md §The steering
vocabulary), and `TRAJECTORY.md` is the narrowest surface, not the default:

- A **ruling** — an override of a business-as-usual instruction until that
  instruction is updated, or a reversal of a standing ruling → `TRAJECTORY.md`,
  naming the instruction it overrides and its discharge. Nothing else lands
  there. Where the instruction can be updated in this session, update it: the
  ruling is then discharged on landing and never enters the ledger.
- A decision about mechanism → the owning kit's `SPEC.md`, undated, with its
  engineering grounds; this project's reading of a kit template → the
  `.claude/commands/` binding that names the template; a standing rule → one
  CLAUDE.md line pointing at its mechanism.
- Who decided, when, through what channel, and what was refused → the landing
  commit's message. Git history is the archive; no file restates it.
- Work — a gap, a task, a promotion signal → `TASK-QUEUE.md` via
  `run-gates.sh --emit file-gap` mid-iteration, or a direct entry when the
  operator directs one (CLAUDE.md §Housekeeping).

A decision fitting none of the four lands as a queue entry naming the surface it
is owed on — the queue is the fallback because it is drained every close.
