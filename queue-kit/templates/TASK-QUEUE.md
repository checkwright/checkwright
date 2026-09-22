# TASK-QUEUE.md — <project> work queue

## Iteration: —

  The lifecycle iteration header (lifecycle-kit), the shape a Checkwright consumer ships.

  A queue-only adopter deletes this line and the `---` below; both are inert to the queue gates.

---

## New Features

  The feature section: every entry here is spec-ready, its `[spec:]` tag naming its amendment.

  No example sits here, since a spec-ready one would dangle its ref; see Technical Debt.

## Technical Debt

### example-feature

A top-level active entry: its slug is the heading, and its tags ride the line under it.

Selection picks the first entry with no blocked-by tag, in section order.

### downstream-feature

[blocked-by: example-feature]

An entry blocked on another, its blocker named on the tag line.

It cites a live entry with a link: [example-feature](#example-feature).

#### example-subtask

A sub-task is a deeper heading inside its parent, with the same grammar.

### example-debt

The second active section; the same grammar as New Features.

## Deferred

  Parked work, excluded from selection.

### example-deferred

[cost: once/low] [surface: TASK-QUEUE.md]

A design-pending entry, its cost class and primary surface on the tag line.

**Cost while deferred:** what staying parked costs, for a later scope to weigh.

Filed 2026-01-01 by scope — the defer date, its mark and date on one line.

## Done

- retired-example-slug

## Lessons Learned
