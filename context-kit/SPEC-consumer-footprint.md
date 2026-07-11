# SPEC amendment: consumer-footprint-budget

State and hold the kits' consumer-resident footprint: what checkwright asks
a consumer's always-loaded surface to carry, owned in one place, with the
always-loaded meter's baseline as the consumer-side floor-holder. A consumer
project pursues its own objectives; the tooling must stay near-invisible in
its context budget.

## What changes

- **context-kit/SPEC.md gains §The consumer footprint** — context-kit is
  the owner ruling: the budget is context economics, context-kit's charter
  (it already owns the meter, the brevity gate, and the close-stage
  brevity pass; doctrine-kit owns *rules for delivery work*, not the kits'
  own installation contract). The section states:
  - **The budget rule**: a kit's resident ask is at most one pointer line
    on the consumer's always-loaded surface — the load-trigger-residency
    and always-loaded-shape doctrine rules applied to kit shipping. The
    one sanctioned block-sized ask is doctrine-kit's digest, bounded by
    its own one-line-per-rule shape (the doctrine's always-loaded-shape
    rule); its name-lockstep with the doctrine is the sibling
    doctrine-rule-lockstep amendment's gate.
  - **The roster, by citation**: each kit's resident ask is named by
    citing the kit SPEC section that owns it (delegation-kit's
    pre-authorization sentence + skill pointer: delegation-kit/SPEC.md
    §One template, a resident pointer; doctrine-kit's link + digest:
    doctrine-kit/SPEC.md; drift-kit's knowledge-friction capture bullet:
    drift-kit/SPEC.md §The knowledge-friction loop, which already states
    the one-bullet cost and its earn-back condition; every other kit:
    none — hooks, skills, gates, and SPECs are load- or
    event-triggered). The roster is citation-only
    so it cannot restate and drift; a kit adding a resident ask adds its
    row here, which is the review seam.
  - **The floor-holder ruling**: the meter + committed baseline ship as
    the consumer's floor-holder — the consumer install (§Layout and
    configuration's copy list + the smoke) includes creating
    `always-loaded-baseline.txt`, so growth of the resident surface is a
    visible delta at every close-stage brevity pass. The hold is
    *advisory by design*: a hard total-line gate cannot attribute growth
    (the consumer's own content shares the file and is theirs to grow),
    so a level gate would be a noisy check breeding exemptions — the
    high-false-positive defence the enforcement-first rule sanctions for
    keeping a class as stated manual duty. The mechanical holds that do
    exist stay: `check-brevity` bounds the one bulleted section its knob
    designates (single-section by design — context-kit/SPEC.md §The
    brevity gate; this repo points it at the conventions block, not the
    digest); the meter delta feeds `kpi-always-loaded`.

## Producers and consumers

- The new section is prose contract only: no new state, event, knob, or
  gate. Its producer is this repo (kit author) at kit-landing time — the
  kit-landing checklist gains no new step; the review seam is the roster
  row.
- The roster's consumers: a consumer evaluating adoption cost (reads the
  budget before vendoring) and this repo's own close-stage brevity pass
  (reads the roster when judging whether a new resident line is a kit ask
  or repo content).
- The baseline file's producer/consumer loop already exists
  (§The always-loaded meter); this amendment only rules that the install
  ships it — the smoke asserts the meter runs and the baseline file
  parses.

## Existing sections updated

- context-kit/SPEC.md §The always-loaded meter — cross-reference the new
  section as the contract the baseline holds for a consumer.
- context-kit/SPEC.md §Layout and configuration — the install copy list
  names the baseline creation step.
- docs/ (context-kit page or install page) — cite the new section from
  the adoption-facing docs so the budget is discoverable pre-vendor;
  cite, never restate.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
