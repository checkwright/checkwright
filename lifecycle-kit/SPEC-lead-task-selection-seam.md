# SPEC amendment: lead-task-selection-seam

The lead template gains the opening-an-iteration contract it currently
lacks: who selects an iteration's units. Observed failure (2026-07-16, this
repo's first split-posture lead): the template says "its whole authority is
dispatch and answers", so a lead opening an iteration improvised — it
pre-selected deferred slugs and handed scope a finished menu, pre-empting
scope's own selection contract ("Identify tasks", the boundary sweep, the
premise re-verification) and adding no information the operator lacked. The
designed path already exists end-to-end (scope surveys and proposes; queue
changes escalate; the operator rules); the fix is stating it where the lead
reads, so the next lead cannot re-improvise the shortcut. This iteration's
own scope ran the designed flow by hand-carried directive and it held —
that run is the shape this amendment writes down.

## What changes

**1. `templates/lead.md` — a new section, "Opening an iteration"** (between
"The lead model" and "The escalation protocol"). Generic content:

- The lead never selects the unit set. Selection is the scope stage's
  contract — the survey, the intake boundary sweep, and the re-verification
  of each queued premise against the current tree are scope's first job, and
  a pre-made list pre-empts exactly the half that catches a stale premise.
- Before dispatching scope, the lead obtains the operator's **standing
  directive** for the iteration — a theme bounding scope's survey, never a
  slug list — and passes it in the scope dispatch prompt **verbatim** (the
  directive varies per iteration, so per the policy-is-config rule it
  belongs in the dispatch prompt, not the agent definition). Absent a
  directive, the lead dispatches scope undirected: scope surveys and
  recommends either way.
- Scope's proposed unit set returns as an ordinary four-header escalation;
  the lead routes it like any scope/queue change — ruled by the operator,
  or by the lead only where the answer is derivable from the governed
  surfaces (the same routing rule the escalation protocol already states).
- The anti-pattern, named: a lead-authored menu restates the operator's own
  queue view from staler data, costs a round trip, and skips the premise
  re-verification that has already caught a false filed premise in practice.

**2. `templates/skills/scope.md` — one paragraph in the task-triage area**,
the counterpart the scope session reads: a standing directive received from
a lead or operator is a *theme bounding the survey*, never a unit list — the
intake sweep and the premise re-verification run regardless, and the
proposed unit set is escalated for ruling before promotion (under no lead it
surfaces to the user, the stage's ordinary stop — the destination changes,
the proposal step does not).

No new slot: the contract is generic protocol identical for every consumer;
who the operator is, and which channel carries the directive, are already
consumer facts outside the template (the ruling-config slot's standing
policy names the postures; the directive itself is per-dispatch content).
Consumer shims (`.claude/commands/lead.md`, `.claude/commands/scope.md`)
therefore need no binding change — `check-skill-binding`'s slot parity is
untouched.

## Producers and consumers

- **The standing directive** — producer: the operator, at iteration open
  (or absent, stated so); carrier: the lead's scope dispatch prompt,
  verbatim; consumer: the scope session's survey step, which reads it as the
  selection theme. No new file, tag, or field — the directive lives in the
  dispatch prompt and dies with it; everything durable it produces lands in
  the queue entries and amendments scope writes (stamps-authoritative:
  the message thread is transport, never a store).
- **The unit-set escalation** — producer: the scope session (the existing
  four-header shape, no new grammar); consumer: the lead's routing rule,
  then the operator. Both mechanisms exist today; this amendment only
  states *when* they carry selection.

## Existing sections updated

- lifecycle-kit/SPEC.md §templates/lead.md — the template-contract paragraph
  gains the opening-an-iteration protocol in its enumeration of what the
  template owns.
- `templates/lead.md` "Its whole authority is *dispatch* and *answers*"
  (the sentence the improvisation grew from) — extended to name the third
  thing the lead carries at iteration open: the operator's directive,
  relayed not authored.
- This repo's `TASK-QUEUE.md` deferred entry (the filing) — consumed by this
  promotion; the related-class note it carries
  (release-in-iteration-lifecycle, same lead-silence class) is honored by
  landing both templates' edits in this one iteration.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
