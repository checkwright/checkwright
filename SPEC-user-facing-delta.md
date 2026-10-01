# SPEC amendment: user-facing-delta

**A delta that changes what a user meets is marked and settled where the amendment is authored, and the audit stage checks the mark.** The authoring template labels each delta with its work class (lifecycle-kit/templates/stages/spec.md, *Label every delta*), and that label measures the judgment executing the delta demands. Whether a delta changes what an adopter or a site reader meets is a different axis, so a user-facing choice can ride a `{mechanical}` delta, reach no operator, and be reversed a later iteration at the cost of a unit of its own. Build's question triage already stops on a user-facing change outside an amendment's envelope (lifecycle-kit/templates/stages/build.md), but a choice the amendment itself makes reads to build as inside it.

The rule spans lifecycle-kit's stage templates and canon-kit's statement of what an amendment's delta grammar excludes, so this amendment sits at the repository root.

**Read at authoring:**

- `spec.md`'s *Label every delta* paragraph defines the tag by execution judgment alone, and lifecycle-kit/SPEC.md §templates/stages/ carries its grounds under *The spec.md template*.
- `build.md`'s triage names "user-facing semantics" as an envelope change that stops and surfaces. The consumer's stage-session roster (`.claude/agents/stage-session.md`) escalates "any user-facing semantics the amendment did not already settle". Neither surface says how an amendment records a settled one.
- `scope.md` sends a default-roster scope that authors to `spec.md`'s how-to, so a rule landed there reaches both rosters with no scope edit.
- No crate module parses an amendment's brace tags (`git grep -n "design-bearing" -- native/src` returns nothing). canon-kit/SPEC.md §The amendment lifecycle states that the work-class tag stays outside the delta-ID and retired-spelling grammars.

## What changes

### (1) The authoring template marks and settles a user-facing delta

`lifecycle-kit/templates/stages/spec.md` gains a paragraph directly after *Label every delta with its work class*. {design-bearing} {user-facing: the entry's deliverable, a spec-stage rule surfacing such a delta, which mints the mark an author types} **Not yet applied.**

> **Mark a delta that changes what a user meets, and settle it before you commit.** A delta is **user-facing** when it changes what an adopter or a reader of the consumer's published surfaces meets beyond the wording of its prose: a command's output, exit or default, a name they type or read, what an install or a hook does, or what a published page reaches or is labelled. The work-class tag does not answer this, so a `{mechanical}` delta can be user-facing. Such a delta carries `{user-facing: <direction>}` beside its work-class tag, naming the recorded direction that settles the choice — on its queue entry, or one the operator gives now. Where none settles it, it is an envelope question: escalate it to the lead where one is live, else ask the operator, and write the answer into the mark before you commit the amendment.

### (2) The audit and build templates read the mark

`lifecycle-kit/templates/stages/align.md` gains a paragraph directly after *Audit the amendment against itself before auditing it against the tree*. {mechanical} **Not yet applied.**

> **Read every delta for what a user meets.** A delta changing it carries the authoring stage's `{user-facing: …}` mark naming the direction that settles it (lifecycle-kit/templates/stages/spec.md). An unmarked one, or one whose direction does not settle the choice it makes, is an envelope question for the lead or the operator, never one to reword away.

In `lifecycle-kit/templates/stages/build.md`'s question triage, "A *change to the envelope* (narrowing or widening asserted behavior, user-facing semantics) stops and surfaces to the user" becomes "A *change to the envelope* (narrowing or widening asserted behavior, or user-facing semantics no delta's `{user-facing:}` mark settles) stops and surfaces to the user".

### (3) The stage contract carries the grounds

lifecycle-kit/SPEC.md §templates/stages/ gains two passages. {design-bearing} **Not yet applied.**

Under *The spec.md template*, after the paragraph **A delta's work-class label is written inline, not rostered.**:

> **A user-facing delta is marked and settled at authoring, because the work-class tag measures another axis.** The tag records the judgment executing a delta demands, and a choice changing what a user meets can demand none, so it rides a `{mechanical}` delta and reaches no operator. Build's triage stops on a user-facing change outside the envelope, but a choice the amendment makes reads to build as inside it, so the authoring stage settles it and the mark records the settling direction. The mark's value is that direction, because a bare flag would record the question and drop its answer. **Honest limit:** whether a delta is user-facing is semantic, so no gate reads the mark, and a delta both its author and the audit misjudge ships unsurfaced.

Under *The align template*, the paragraph **Claim checks are scoped by subject and ungated.** gains a closing sentence: "The user-facing read is the second look at the authoring stage's own judgment, and the last before build reads the mark as settled."

### (4) canon-kit names the mark outside the delta grammar

In canon-kit/SPEC.md §The amendment lifecycle, two sentences are re-phrased. {mechanical} **Not yet applied.**

- "The **work-class tag** an amendment may carry (`{mechanical}` / `{design-bearing}`) is **outside** this grammar. Its owner is the roster's authoring-stage template and its reader is the iteration lead at batch-cut, so inside the delta token it would tie two independent conventions to one string." becomes "A delta's **tags**, the work-class tag (`{mechanical}` / `{design-bearing}`) and the user-facing mark (`{user-facing: …}`), are **outside** this grammar. Their owner is the roster's authoring-stage template, and their readers are the iteration lead at batch-cut and the audit stage, so inside the delta token they would tie independent conventions to one string."
- "The **work-class tag** stays outside this grammar too." becomes "A delta's tags stay outside this grammar too."

### (5) The site mirrors follow

`docs/lifecycle-kit/SPEC.md` and `docs/canon-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, in the commits landing deltas 3 and 4. {mechanical}

## Producers and consumers

- **The `{user-facing: <direction>}` mark (delta 1).** A new amendment-text convention, minted by the authoring template.
  - Producer: the authoring stage (`spec.md`, or a default-roster scope following it), at each user-facing delta, before the amendment commit. Its enabling config is any consumer whose roster runs either stage; nothing needs setting.
  - Its one field, the direction, has three readers. The audit stage checks it settles the choice (delta 2). The build stage reads it as an envelope already settled, so the triage does not stop on that choice (delta 2). The lead reads it when it routes a build escalation about the same choice.
  - Roster-holding readers of amendment text: `check-amendment-update-target` and `check-amendment-retired-spelling` read `### (<N>)` headings and bullets under their own sections, and a brace mark in a delta body is neither; `check-stage-entry` assertions D and E read only the inferred markers. No gate parses brace tags, so no roster gains a row.
- **The escalation (delta 1).** It rides the existing four-header block to the lead (lifecycle-kit/templates/lead.md §The escalation protocol), which routes an envelope question to the intent oracle or the operator, and its answer lands on the amendment as the direction it is (lifecycle-kit/SPEC.md §The steering vocabulary). No new channel.
- **No corpus narrows**, and no gate verdict moves.

## Existing sections updated

Roster produced by `git grep -n "work-class\|user-facing\|envelope" -- lifecycle-kit canon-kit/SPEC.md .claude/agents`, with each hit read.

- `lifecycle-kit/templates/stages/spec.md` — the new paragraph after *Label every delta* (delta 1).
- `lifecycle-kit/templates/stages/align.md` — the new paragraph (delta 2).
- `lifecycle-kit/templates/stages/build.md` — the triage sentence (delta 2).
- `lifecycle-kit/SPEC.md` — §templates/stages/, *The spec.md template* and *The align template* (delta 3).
- `canon-kit/SPEC.md` — §The amendment lifecycle, the two work-class sentences (delta 4).
- `docs/lifecycle-kit/SPEC.md`, `docs/canon-kit/SPEC.md` — the regenerated mirrors (deltas 3, 4 and 5).

The remaining hits need no edit. `lifecycle-kit/templates/lead.md` reads work-class labels at batch cut, and the mark adds nothing to its tiering. `.claude/agents/stage-session.md`'s envelope roster already escalates a user-facing change "the amendment did not already settle", which the mark is how an amendment records.

## Retired spellings

- None — delta 4 re-phrases two canon-kit sentences in place and every other delta adds text, so no spelling leaves the tree.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The entry moves** — `user-facing-delta-unsurfaced` moves to Done with `--queue done` in the commit deleting this file, at its build batch, before the drain stage.
