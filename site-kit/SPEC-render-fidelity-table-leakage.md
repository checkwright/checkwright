# SPEC amendment: render-fidelity-table-leakage

## What changes

`check-docs-render-fidelity` gains a third per-page assertion beside fence
and heading leakage:

- **No table leakage.** The count of rendered `<table>` elements is never
  *less than* the count of source GFM table starts. A table start is a
  pipe-carrying row immediately followed by a delimiter row (dashes, colons,
  pipes — the `| --- |` shape), both placed outside any code context by the
  gate's existing fence-aware scan. The red signature is a source table that
  renders as literal-pipe paragraph text: kramdown terminates a table only on
  a blank line, so a table whose last row abuts a following non-blank line can
  collapse into a paragraph — the same source-vs-rendered count-parity method
  the heading assertion already uses.
- **One-sided direction.** Rendered may exceed source (a raw-HTML `<table>` in
  source is legitimate and renders without a GFM start); only a deficit reds.
- **Honest limit extended.** Count parity can be masked by an offsetting
  raw-HTML `<table>` on the same page (one broken GFM table plus one HTML
  table balances the counts); and the assertion set is fences, headings, and
  tables — still silent on divergences that corrupt none of the three. The
  detector is deliberately conservative (delimiter-row anchored): a table
  kramdown accepts but the scan does not count can only under-count source
  starts, which false-cleans, never false-reds.
- **Fixture.** The gate's fixture pair gains a bad case reproducing the
  observed incident shape — a GFM table whose last row abuts a non-blank
  marker line — red under the table assertion while green under fence and
  heading (proving the new assertion carries the class the existing two
  missed). Motivating instance, 2026-07-13: the docs value page's generated
  rollup table abutted its `:end` marker and shipped as a literal-pipe
  paragraph with the gate silent; the emitter fix (a trailing blank line)
  landed then — this assertion mechanizes the channel.

## Producers and consumers

- **The leakage** — producer: kramdown's blank-line table termination meeting
  a source edit or emitter output that abuts a table against a non-blank
  line. Consumer: the new assertion inside the gate's existing per-page pass
  (same renderer oracle, same front-matter strip).
- **The red** — producer: the gate at precommit tier; consumer: the committing
  session via the battery/hook.
- No new knobs, no new state, no new fields — the assertion rides the gate's
  existing scan and renderer plumbing.

## Existing sections updated

- site-kit/SPEC.md §check-docs-render-fidelity: the invariant sentence gains
  the table clause; the two-property list becomes three; the honest-limit
  paragraph is rewritten per above (fences/headings/tables, the masking
  caveat, the conservative-detector direction).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
