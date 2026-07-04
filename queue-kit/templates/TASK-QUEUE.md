# TASK-QUEUE.md — <project> work queue

## New Features

- **example-feature** — a top-level active entry: a bold kebab-case slug, an
  em-dash, then free prose. Selection picks the first entry with no blocked-by
  tag, in section order (see downstream-feature).
- **downstream-feature** — an entry blocked on another. [blocked-by: example-feature]
  - **example-subtask** — an indented bold bullet is a sub-task (same grammar,
    same slug namespace).
  - a plain indented bullet (no bold lead-in) is a prose note, left alone.

## Technical Debt

- **example-debt** — the second active section; same grammar as New Features.

## Deferred

  Parked work, excluded from selection. `###` subsections are presentation
  only (indented, so this note stays off the column-0 grammar path).

### Someday

- **example-deferred** — a design-pending entry. [needs-spec]
- **spec-ready-entry** — a spec-ready entry. [spec: SPEC-example.md]

## Done

- retired-example-slug

## Lessons Learned
