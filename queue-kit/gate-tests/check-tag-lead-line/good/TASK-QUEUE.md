# TASK-QUEUE.md

## Iteration: demo

## New Features

### alpha-feature

[blocked-by: beta-feature]

build the alpha surface.

### beta-feature

the prerequisite groundwork.

#### beta-subtask

[blocked-by: alpha-feature]

a nested unit of beta, its own tag line its lead rather than a body line of its parent.

## Technical Debt

### tidy-logs

[drain-exempt: validate-half pending]

collapse the duplicated log lines. a continuation mentioning [drain-exempt: whatever] is tolerated (lead carries the class).

### watch-the-run

[observed-by: ci-run]

its completion predicate is an observation, not a tree state. a continuation mentioning [observed-by: ci-run] is tolerated (lead carries the class).

## Deferred

### gamma-feature

[cost: once/low] [surface: queue-kit] [recurrence: 2026-01-01, 2026-01-02] [not-icebox-eligible: 2026-01-03 a standing fact]

revisit when the alpha surface settles. so is one mentioning [cost: once/low] or [surface: queue-kit] (lead carries both).

## Done

- old-finished-task

## Lessons Learned

- **lead-tagged-lesson** [attend] — the attend tag rides the lead line, as required.
  a continuation line that merely mentions [attend] is tolerated (lead carries it).
