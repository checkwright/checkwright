# SPEC amendment: comment-count

A restated count in a *comment* is invisible to `check-manifest-count`,
which couples manifests only: `# rules 1-8` in a consumer guard sat stale
while the ruleset grew, and no gate read it (queue: `comment-count-drift`).
Ruling: **block the shape in `check-comment-tier`** — a count is never
directive wording, so the comment gate owns it. The weighed alternative, a
source-coupled numeral scan with an allowlist, is rejected: legitimate
numerals abound in source (exit codes, indices, field positions) and the
false-positive rate would exceed the catch.

## What changes

`check-comment-tier` gains a **count-shape override**: a full-line comment
on the governed surface matching the count grammar — `<cardinal> <wedge>?
<collection-noun>` or `<collection-noun> <d+>-<d+>`, the shared adapter
from `spec-kit/SPEC-count-shapes.md`, over `SPEC_KIT_COUNT_COLLECTIONS` —
is flagged **even when it sits inside a blessed directive window**. A
directive's blessing covers its own wording wrapped, never a pinned total:
the fix is deleting the count or citing the owning collection
(`# spec:`-pointing at it), and `comment-tier-exempt: <reason>` remains
the valve — with the standing doctrine that blessing a restatement is
itself the defect. The partitive and comparator exemptions apply as in the
manifest gate (a bound or proportion in a directive is a rule, not a
total).

No new knob: the noun vocabulary enters once via
`SPEC_KIT_COUNT_COLLECTIONS`, and the wedge window rides
`SPEC_KIT_COUNT_WEDGE_WORDS`. Blocked on `manifest-count-shapes` landing
the shared grammar adapter.

## Producers and consumers

- Producer: the generated pre-commit hook / `run-gates.sh` running
  `check-comment-tier` over `spec_comment_surface_with_templates`,
  unchanged.
- Consumer: the committing operator via the gate's existing per-hit output
  (`file:line`, classification); the new override reports the matched
  count span so the fix is evident.
- The shared count-grammar adapter in `lib/spec.sh` is read by this gate
  at scan time — same reader transition as the manifest gate, one
  vocabulary source.

## Existing sections updated

- §check-comment-tier: the invariant paragraph gains the count-shape
  override (a count inside a directive window is flagged) and its
  rationale; the calibration paragraph names the shared grammar and the
  partitive/comparator carve-outs.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Fixture pair** — bad: a count inside a blessed directive window and a
      noun-then-range comment; good: a comparator bound in a directive, a
      partitive, an inline-code meta-reference.
