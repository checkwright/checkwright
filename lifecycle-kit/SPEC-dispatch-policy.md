# SPEC amendment: dispatch-policy

## What changes

templates/lead.md already rules that the ruling-class roster is config, not
per-dispatch prose (§Policy is config, not prose). This amendment widens that
section from the ruling classes to **all standing dispatch policy**, and adds
the dispatch-granularity ruling the first orchestrated iteration ran without.

- **Residency rule (widened).** Everything true of *every* dispatch — journal
  mechanics, environment wiring, index-sharing cautions, escalation shape —
  lives in the tracked agent definition the dispatch names (the
  `ruling-config` slot's file); a dispatch prompt carries only what varies per
  dispatch: the stage skill to invoke, the batch's task slugs, and pointers
  specific to that batch. The tell that content is misplaced: the same
  sentence appearing in two dispatch prompts. The agent definition points at
  owning docs, never restates them (content-tiering) — e.g. it cites
  delegation-kit's resume-journal mechanics rather than transcribing them.
- **Dispatch granularity (new Economics content).** Both naive defaults are
  ruled out with cause: whole-queue-in-one rides past every split trigger the
  delegation protocol names; one-dispatch-per-task pays context setup times N
  while buying no parallelism (committing agents serialize on the shared git
  index regardless). The codified shape: **batch units that share a kit/SPEC
  surface into one dispatch** (derived context is reused where it is actually
  common), and **split where the model tier changes or a delegation-kit split
  trigger fires** — per-batch model tiering is the dominant window lever, not
  token counts.
- **Transcript audit (one-time worklist item, this repo).** The first
  orchestrated iteration's dispatched-session transcripts are audited for
  further re-derivation classes (facts every dispatched session re-derived
  that a tracked surface should own); each hit is filed per the gap
  disposition rule or lands in the agent definition in the same unit.

## Producers and consumers

- **Standing-policy content** — producer: the consumer's tracked agent
  definition (this repo: `.claude/agents/stage-session.md`); consumer: every
  dispatched stage session at context load, and the lead author who no longer
  re-derives dispatch-prompt boilerplate. The template's `ruling-config` slot
  already names the file; no new slot is needed.
- **Granularity rule** — producer: the templates/lead.md Economics edit;
  consumer: the lead session at dispatch-planning time. It consumes existing
  delegation-kit signals (split triggers, the budget guard's verdict) — no
  new state, event, or wire format is introduced.
- **Audit findings** — producer: the build/validate session running the
  transcript audit; consumer: the queue (filed tasks) or the agent definition
  (landed residency), per finding.

## Existing sections updated

- templates/lead.md §Policy is config, not prose: widened as above (the
  section's one-source argument is unchanged — it now covers a larger set).
- templates/lead.md §Economics — batch, and compact at handoff: gains the
  granularity paragraph beside the existing batching rule.
- lifecycle-kit/SPEC.md §templates/lead.md: prose describing the lead's
  dispatch duties reflects the widened residency rule.
- `.claude/agents/stage-session.md` (consumer side, same unit): gains the
  standing-policy block — journal-mechanics citation, any environment wiring
  that survives SPEC-session-id-subagent.md (which removes the sessions-dir
  override from the standing set), and the shared-index caution *by citation
  to CLAUDE.md*, not restatement.

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
