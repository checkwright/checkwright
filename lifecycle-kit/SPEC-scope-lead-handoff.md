# SPEC amendment: scope-lead-handoff

## What changes

The scope stage-skill template's close-out (templates/skills/scope.md, the
"recommend the next stage" paragraph) gains a named slot, **`handoff`**: once
the promotion commit has landed and the iteration is named, the skill presents
the consumer's orchestration hand-off alongside the next-stage recommendation.

- **Template text stays generic.** It names the moment (promotion landed,
  iteration named, next stage recommended) and defers the content wholly to
  the slot. It hardcodes neither a lead command nor a compaction step — a
  consumer command and a harness built-in are consumer content, the same seam
  the `ritual`/`exit-condition` slots already hold.
- **Slot semantics (binding authoring rule applies).** The binding points at
  the consumer's documented start-sequence choice by citation — never a
  restatement of the steps (lifecycle-kit/SPEC.md §templates/skills/, the
  binds-residue-cites-procedure rule). A lead-less or harness-less consumer
  binds the slot with a "no lead — run the stages by hand" line;
  `check-skill-binding` requires every template slot bound, so a re-vendor
  surfaces the new slot in every consumer shim as a red naming the missing
  binding (the slot-set drift channel the upgrade-path queue entry names as
  Phase-B's mechanical half).
- **This repo's binding** (lands in the same commit as the template edit —
  `check-skill-binding` holds shim↔template slot parity): present the two
  branches — compact-then-`/lead` per docs/orchestration.md §Running an
  iteration under a lead, versus manual stage-by-stage steering with the
  compacted scope session consulted on questions — pointing at that section,
  never restating its steps (the start sequence's SSOT is that page, owned by
  lifecycle-kit/SPEC.md §templates/lead.md).

## Producers and consumers

- **Slot content** — producer: the consumer shim's `handoff` binding (this
  repo: the scope shim under `.claude/commands/`). Consumer: the scope session
  executing the template at close-out, and the user reading the presented
  hand-off — the decision point the feature exists to surface.
- **Slot name** — producer: the template edit; consumers:
  `check-skill-binding` (slot↔binding parity, red on an unbound slot) and
  `check-shim-restatement` (the binding body's copy-shape against the
  template corpus).
- No new state, no new fields, no new knobs — one slot name on an existing
  governed surface.

## Existing sections updated

- templates/skills/scope.md: the close-out paragraph is the prior-flow prose
  and is updated in place by the template edit (recommendation, then
  hand-off).
- lifecycle-kit/SPEC.md §templates/skills/: no change needed — the section
  states the slot grammar and contract generically and restates no per-template
  slot roster (derivation-first), so a new slot lands without spec drift.
- docs/orchestration.md §Running an iteration under a lead: stays the start
  sequence's home; this feature adds pointers to it, no edits of it.

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
