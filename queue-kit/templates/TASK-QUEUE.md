# TASK-QUEUE.md — <project> work queue

## Iteration: —

  The lifecycle iteration header (lifecycle-kit): the composed-tree shape a
  Checkwright consumer ships. A queue-only adopter with no lifecycle-kit
  deletes this line and the `---` below; both are inert to the queue gates.

---

## New Features

  The feature section: every entry here is spec-ready, carrying a `[spec:]` tag
  whose ref names the amendment file its design lives in (written at the scope
  stage). No example entry sits here — a spec-ready one would dangle its ref; the
  grammar shapes are shown under Technical Debt, an active section with identical
  grammar.

## Technical Debt

- **example-feature** — a top-level active entry: a bold kebab-case slug, an
  em-dash, then free prose. Selection picks the first entry with no blocked-by
  tag, in section order (see downstream-feature).
- **downstream-feature** — an entry blocked on another. [blocked-by: example-feature]
  - **example-subtask** — an indented bold bullet is a sub-task (same grammar,
    same slug namespace).
  - a plain indented bullet (no bold lead-in) is a prose note, left alone.
- **example-debt** — the second active section; same grammar as New Features.

## Deferred

  Parked work, excluded from selection. `###` subsections are presentation
  only (indented, so this note stays off the column-0 grammar path).

### Someday

- **example-deferred** — a design-pending entry. [needs-spec]

## Done

- retired-example-slug

## Lessons Learned
