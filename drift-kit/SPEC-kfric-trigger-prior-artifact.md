# SPEC amendment: kfric-trigger-prior-artifact-consultation

## What changes

The knowledge-friction loop's capture trigger (drift-kit/SPEC.md §The
knowledge-friction loop, step 1) enumerates the non-owning surfaces a session
re-derives a fact from — "an implementation, a gate's source, a commit
message". That roster does **not** name the case where a session derives a new
deliverable by **consulting a prior or sibling deliverable** — reading the last
release note to author the next, copying a prior SPEC's structure to shape a
new one. This is the same doctrine failure as re-deriving off an implementation
(a fact that no doc owns, reconstructed by looking at where it happened to
appear before), but the roster's examples do not cue it, so
precedent-imitation self-reports nowhere and stays invisible to every content
gate.

This amendment **broadens the capture roster's cue** to name prior-artifact /
sibling-deliverable consultation as a stampable non-owning surface, and widens
the always-loaded CLAUDE.md kfric bullet's parenthetical so the resident cue is
not narrower than the SPEC roster it summarizes.

**Envelope — a cue broadening, nothing else.** The log grammar
(`<date> <fact> ← <surface>`), the `bin/kfric.sh` affordance, the raw-append
fallback, the close-stage triage (`templates/close-knowledge.md`), and the
`kpi-knowledge-friction` aggregate are all **unchanged**. Only the recognized
*set of non-owning surfaces* widens.

**The seam is reinforced, not relaxed.** §The knowledge-friction loop already
draws the channel seam: kfric is the narrow sensor for a *re-derived fact*
only; a *work-shaped* finding (a gap, a task, a defect) routes to the committed
gap inbox (lifecycle-kit/SPEC.md §The committed gap inbox), never this log. The
broadening must not blur that seam. A conclusion of the form "this artifact's
chrome should be **owned or generated** rather than copied" is work-shaped and
belongs in the gap inbox; what kfric captures is the narrower *fact re-derived*
— the specific value or structure reconstructed from the prior artifact because
no doc owns it. The amendment states this distinction inline at the widened
cue, so the broadening cannot be read as a licence to overload the log with
backlog items.

## Producers and consumers

- **Producer** — any session that catches itself deriving a new deliverable by
  consulting a prior/sibling one. The producer is the same human/session
  recognition that already drives every kfric capture; this amendment adds one
  recognized trigger *case* to that recognition, no new code path. The
  affordance stays `bin/kfric.sh "<fact>" "<surface>"` (unchanged), and the raw
  append stays the legal fallback.
- **Consumer** — unchanged. The close-stage triage walks the log line by line
  and gives each re-derived fact a doc owner (or a pointer); the
  `kpi-knowledge-friction` KPI trends the per-iteration line count. Neither
  reads provenance, so a line captured under the new cue is triaged and
  aggregated identically to any other.
- **No new field, file, knob, or tag.** The log line's shape is unchanged, so
  there is nothing whose reader must be newly named — the existing triage and
  KPI are the readers, already named above.

## Existing sections updated

- **drift-kit/SPEC.md §The knowledge-friction loop, step 1 (Capture)** — extend
  the non-owning-surface roster to name prior-artifact / sibling-deliverable
  consultation, and add the one-sentence seam clarification (work-shaped
  "should be owned/generated" → gap inbox; kfric captures the re-derived fact).
- **CLAUDE.md — the Knowledge-friction capture bullet** — widen the
  parenthetical ("off an implementation, a gate's source, or a commit") to
  include a prior/sibling deliverable, so the always-loaded cue matches the
  broadened SPEC roster. Keep it one line (always-loaded shape).

No wire contract, no fenced source — nothing to embed.

## Definition of Done

- [ ] **Causal completeness** — the one new trigger case names its producer
      (session recognition, existing `kfric.sh` affordance) and its consumers
      (unchanged close triage + `kpi-knowledge-friction`); no new field, so no
      unread field.
- [ ] **Merged with no information lost** — the roster broadening and the seam
      sentence land in drift-kit/SPEC.md §The knowledge-friction loop step 1;
      the CLAUDE.md bullet widens in place; both read as one coherent surface.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      drift-kit (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired; confirm no other surface
      restates the narrower roster (the CLAUDE.md bullet is the only summary).
- [ ] **Gaps filed** — any cross-component gap found during the work filed as a
      debt task, resolved in-session if it is a build-time causal gap.
