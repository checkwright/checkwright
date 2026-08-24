# SPEC amendment: agent-id-doubt

Closes `subagent-stop-agent-id-attribution-doubt`. The `session` field bullet
forwards `agent_id` as the discriminator that *would* attribute a firing to one
agent, and the first read of real payloads puts that forward claim in doubt.

**The disposition was ruled before this stage opened and is executed, not
re-decided: state the doubt, drop the forward claim.** The third option the
entry names — re-take the reading under a fresh authorization — is
operator-class under the no-values privacy ruling (§What `background_tasks`
carries), was not granted, and is therefore not available to this amendment. No
delta below reads a payload value.

## What changes

### (1) The `session` bullet states a doubt where it forwarded a claim

The last two sentences of §The turn-end liveness hook's `session` bullet —
"The payload's `agent_id` is the discriminator that would, and logging it is a
grammar change this hook has not taken" — are replaced by a statement of what is
**known**, what is **doubted**, and at what evidentiary tier each sits.
**{design-bearing}**

The current text is a **forward promise on a shipped kit surface**: it tells
every consumer that a `SubagentStop` payload carries a field that can attribute a
firing to one agent, and that the only thing standing between them and that
attribution is a grammar delta this kit declined to take. If `agent_id` is
per-*firing* rather than per-*agent*, that sentence is not merely unhelpful — it
is an invitation to build an attribution grammar on a field that cannot
attribute, which reads as a working feature until two agents are compared.

The replacement asserts three things and no more:

- **Known, and measured rather than argued.** `session_id` cannot attribute: a
  dispatched agent and its dispatcher log the *same* value. This was already
  stated from the first firings and is **re-corroborated first-hand at this spec
  stage** — a worktree-isolated child's own log and its dispatcher's log carried
  the identical `session=` token, read from two different files in two different
  checkouts.
- **Doubted, and carried as an observation rather than a measurement.** One read
  of five firings in one session saw five **distinct** top-level `agent_id`
  values, none matching the stable identifier the same payloads' `background_tasks`
  array reported for the one live dispatched agent. If that holds, the field is
  per-firing.
- **Why the doubt cannot be settled from this tree.** The tracked record is
  `.workflow/subagent-stop-liveness.log`, which records the payload's top-level
  **key set and no values** by the no-values privacy ruling. Every one of its
  lines therefore carries `agent_id` as a key and none as a value, and settling
  the question means reading raw payloads — the read that ruling holds
  operator-class.

**What the bullet must not say afterwards.** It must not name `agent_id` as
*the* discriminator, must not describe the missing step as merely a grammar
change, and must not imply that a consumer taking that grammar delta would get
attribution. Those are the three readings the current sentence supports and the
whole content of the repair.

### (2) The no-attribution-field omission is restated to rest on the doubt rather than on the payload

§The turn-end liveness hook's paragraph recording the two fields deliberately
**not** carried says the omission of a session-attribution field is because "the
payload carries no attributing value". That parenthetical is narrowed to what is
actually established. **{design-bearing}**

The claim as written is stronger than the evidence: what is established is that
`session_id` does not attribute and that `agent_id`'s ability to is in doubt.
"The payload carries no attributing value" asserts a negative over the whole
payload that nothing here has checked, and it would be **falsified** — quietly,
by a surface nobody re-reads — if `agent_id` turns out to be per-agent after all.
The narrowed form keeps the omission's *conclusion* (no such field is carried,
and the two reasons the paragraph gives for not wanting one are untouched) while
resting it on the two facts that hold.

This delta exists because the two sentences are one claim written twice: leaving
the second would leave the SPEC contradicting itself the moment the first is
repaired, which is the drift the one-owner-per-fact rule exists to prevent.

## Producers and consumers

**No new state, event, interface or field.** Both deltas delete or narrow prose
on one existing section; the log grammar takes no delta, no field is added or
removed, no knob is added, and no code path changes. This is stated explicitly
rather than left as an empty section, because an amendment whose causal
completeness is vacuous should say so and say why.

- *Producer of the repaired text:* `delegation-kit/SPEC.md` §The turn-end
  liveness hook (template), which is authored, not generated. The kit's shipped
  `templates/subagent-stop-liveness.sh` carries `spec:` comments pointing at
  that section and is **unchanged** — the comments cite the section, never
  restate its content, so they do not move with it.
- *Consumer:* every reader of that section — a consumer wiring the hook, and any
  later session weighing an attribution grammar. The transition at which it is
  read is the one that made the entry envelope-class: the moment someone
  proposes logging `agent_id`.
- *Named reader of each surviving field:* unchanged. `session` keeps the reader
  the current bullet names (separating one top-level session's firings from
  another's in a shared log), and that reader is untouched by both deltas — what
  is dropped is a claim about a field the grammar does **not** carry.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 2 narrows an assertion's scope, so each reader's **red condition** is
enumerated:

- `check-surface-duplication` — reds when a canonical definition appears on a
  second surface. **Not monotone**: its verdict turns on matched text, so
  rewriting a paragraph can *add* a match against a sibling surface. The
  rewritten text must be diffed against §What `background_tasks` carries, which
  discusses the same fields, rather than assumed clear.
- `check-unmarked-claim` — reds on a claim carrying no evidentiary marker.
  **Not monotone**, and it is the reader this amendment most directly serves:
  delta 1 replaces an unmarked forward claim with tiered statements, so the
  text must carry its markers rather than merely hedge in prose.
- `check-measured-claim` — reds on a claim in the measured class with no
  re-derivation instruction. **Not monotone**: the re-corroborated `session_id`
  finding at delta 1 is a measured claim and needs its marker.
- `check-prose-enum` / `check-spec-pointer` — **monotone and cleared by
  inspection**: no delta adds an enumeration or a cross-surface pointer, and
  neither delta removes an existing pointer's target.
- `check-amendment-queue` — reds on an unpaired amendment or a dangling
  `[spec:]` ref. **Not monotone** (its red condition on the amendment axis is a
  zero count of pairing entries), which is why this file and its queue entry's
  tag land in one commit.

## Existing sections updated

- `delegation-kit/SPEC.md` §The turn-end liveness hook (template), the `session`
  field bullet — the forward claim is replaced by the three tiered statements
  (delta 1).
- `delegation-kit/SPEC.md` §The turn-end liveness hook (template), the
  no-field-without-a-reader paragraph — the session-attribution omission's
  parenthetical narrows to what is established (delta 2).
- `delegation-kit/SPEC.md` §What `background_tasks` carries — the paragraph
  listing the payload's other top-level fields names `agent_id` among them and
  must not be left implying it discriminates; it is the sibling surface delta 1's
  text is diffed against (deltas 1 and 2).

## Definition of Done

- [ ] **Causal completeness** — vacuous by construction here and recorded as
      such above: no new state, event, interface or field is introduced.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), discharged at the iteration
      rather than at this commit, sibling amendments being in flight for it.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The specific string to chase is any other
      surface naming `agent_id` as a discriminator.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
- [ ] **No payload value read** — the build stage discharges this amendment
      without reading a raw `SubagentStop` payload. Needing one to proceed is an
      escalation, not a licence.
