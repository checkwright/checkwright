# SPEC amendment: intra-stage batch supervision

Who supervises a stage that splits into batches, and the machinery statement
that makes the answer reachable. Surfaced 2026-07-17 under live budget
pressure (the release-in-lifecycle lead session); the enabling machinery
behavior was verified against the oracle at promotion (2026-07-17) rather than
assumed.

## What changes

### 1. Same-stage re-entry is stated (SPEC §The state machine)

The machinery already permits it — this half documents verified behavior, no
code changes:

- Entering the currently-stamped stage from a **new session** is legal and
  appends a fresh stamp. Verified: `bin/enter-stage.sh`'s idempotence guard
  keys on the full `(iteration, stage, session-id)` triple, so only the same
  session re-entering is a no-op; `check-stage-entry` assertion A keys on the
  *predecessor* stamp, which the first entry satisfied for every sibling.
- `check-stage-evidence`'s session-distinctness rule constrains **cross-stage**
  sharing only — its own help text already reads "same-stage re-entries may
  share or rotate freely"; that parenthetical is promoted into owned SPEC
  prose so a reader finds the rule in the owner doc, not a gate's error
  message.
- N sessions may therefore enter one stage, serialized by the shared
  index/HEAD like any concurrent sessions; each leaves its own stamp, so
  per-batch provenance rides the existing stamp grammar — no new grammar, no
  new stamp field.

### 2. Lead-owns-batching (templates/lead.md §Economics)

The doctrine clause the template lacks:

- **An intra-stage batch split is N sibling stage sessions, dispatched and
  validated by the lead** — each entering through `enter-stage.sh` (the
  same-stage re-entry above), each batching by shared surface exactly as the
  existing §Economics clause directs. This amendment adds the *supervision
  owner*; the batching criteria are unchanged.
- **A stage session never dispatches a sibling stage session.** A stage that
  sub-dispatches its own batches nests a second supervisor at the same tier,
  hidden from the lead's budget and context accounting — the redundancy the
  split posture exists to remove. Read-only fan-outs inside a stage stay
  sanctioned (the delegation nudge; CLAUDE.md §Agent execution).
- Sequencing residue the lead owns: sibling batch sessions of one stage run
  under whatever stamp regime is live. Under the current flip+stamp, a
  same-stage re-entry's flip rewrites the `[stage:]` field to its current
  value; if the deferred stage-cursor-extraction unit later retires the flip,
  re-entry becomes a pure stamp append. Either way the statement holds. The
  lead serializes batches that share a surface and may parallelize batches that
  do not, subject to the shared-index discipline.

## Producers and consumers

- **The same-stage stamp.** Producer: each sibling batch session's
  `enter-stage.sh` invocation (existing writer, existing enabling config).
  Consumers: `check-stage-evidence` (stamp grammar + name-axis agreement,
  unchanged; under the `stage` posture the distinctness map reads the stamps
  and permits same-stage sharing — the transition where the promoted
  parenthetical is exercised); the lead's dispatch bookkeeping (per-batch
  provenance when auditing which session produced which batch).
- **The doctrine clause.** Producer: templates/lead.md §Economics (shipped
  prose, live wherever the lead template is vendored). Consumers: a live lead
  choosing dispatch shape at batch time; a dispatched stage session declining
  to sub-dispatch a sibling (the clause is its authority to refuse); this
  repo's own iterations dogfooding the split posture.
- No new state, file, knob, or tag — both halves land in existing surfaces,
  so there is no field without a reader by construction.

## Existing sections updated (at merge)

- lifecycle-kit/SPEC.md §The state machine — gains the same-stage re-entry
  statement beside the honest-limit paragraph (which already discusses stamp
  semantics and the session-id derivation).
- lifecycle-kit/SPEC.md §check-stage-evidence — the distinctness prose names
  same-stage sharing as in-contract, not merely unpunished.
- lifecycle-kit/SPEC.md §templates/lead.md — the section mirroring the lead
  template's contract gains the lead-owns-batching clause.
- lifecycle-kit/templates/lead.md §Economics — the clause itself (the shipped
  surface).
- docs/orchestration.md §Running an iteration under a lead — the dispatch
  sequence mentions batch dispatch; gains one line citing the clause.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for wording this change
      retires (the distinctness parenthetical's sole-home status); nothing
      dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
