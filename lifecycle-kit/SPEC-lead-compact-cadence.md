# SPEC amendment: lead-compact-cadence

## What changes

templates/lead.md's Economics section names one compact point today — at
handoff, after the promotion commit and before the first dispatch. The first
orchestrated iteration showed that is one point too few: roughly 70k tokens of
validation residue accreted by build acceptance were re-read on every later
cold wake. The protocol extends with **acceptance-boundary compact
suggestions**:

- **The moment.** After a stage session's work is accepted — its commits
  validated, its rulings landed in governed surfaces — and before the next
  dispatch, the lead *suggests* a compact to the operator. Compaction is
  operator-invoked; the lead can only recommend (the honest limit, stated in
  the template), so the suggestion is one line in the lead's acceptance
  message, not a new mechanism.
- **The calibration rule (when to suggest, not blanket).** A compact pays
  when the remaining cold wakes times the compressible residue exceed one
  context re-read. That makes the *early* acceptance boundaries — the audit
  and build stages, with the most residue accreted and the most wakes still
  ahead — the paying ones, and the late boundaries (validate, close) not
  worth the operator interruption. The template states the rule, not a stage
  roster, so a consumer with a different stage set derives its own paying
  boundaries.
- **The keep-instruction.** Identical to the handoff compact's, unchanged:
  keep per-amendment rationale, rejected alternatives, and the ruling-class
  roster; drop tool output and file contents. One instruction, stated once —
  the acceptance-boundary paragraph cites the handoff bullet rather than
  restating it.

## Producers and consumers

- **The suggestion** — producer: the lead session at the stage-acceptance
  transition (after validate-after-commit passes for that stage's work);
  consumer: the operator, who invokes the compact or declines. No stamp, no
  file, no message-format change — the existing lead→operator channel carries
  one added line.
- **The calibration rule** — producer: the templates/lead.md Economics edit;
  consumer: the lead session deciding whether the boundary pays.
- No new state, fields, or knobs.

## Existing sections updated

- templates/lead.md §Economics — batch, and compact at handoff: the compact
  bullet generalizes from one named point to the handoff point plus
  acceptance boundaries under the calibration rule; the section heading's
  "compact at handoff" widens accordingly.
- lifecycle-kit/SPEC.md §templates/lead.md: the lead-economics description
  reflects the added cadence.
- docs/orchestration.md: the lead-driven sequence mentions compacts where it
  already narrates the handoff one — a pointer-level edit, no restated
  mechanics.

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
