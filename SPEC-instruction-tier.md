# SPEC amendment: instruction-tier

**No delta is applied; every replacement passage below is Not yet applied.**

This amendment serves `instruction-motivation-owner` and settles the reach question that entry leaves
open. Its sibling `instruction-surface-sweep` (SPEC-sweep.md) applies the rule placed here across the
instruction corpus, which is why that entry's blocked-by edge on this one stands and this
amendment's deltas land first.

It is a root-level amendment: its deltas span doctrine-kit, delegation-kit, lifecycle-kit and
canon-kit.

**Every delta re-phrases or extends an existing sentence rather than appending a paragraph.** That
follows operator direction, 2026-09-12: SPECs are not expected to grow each iteration, text is added
only where it adds value, and brief, phrase-shaped instructions read better to an LLM than verbose
ones.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-12 at the commit that stamped this stage.

**The reach gap holds at HEAD.** delegation-kit/SPEC.md §Operative residency opens *"A rule may be
restated as an imperative in a surface that does not own it"*, which is a sanction for a copy. A
template paragraph explaining why its own instruction is right copies nothing. Condition (b) says the
reasoning *stays with the owner*, and for that paragraph there is no owner to name.

**The rule is stated beside the sanction, not by widening it.** Three grounds:

1. The sanction is an exception to Content-tiering, keyed on a reader whose trigger never loads the
   owner. Original motivation has no owner to be unreachable from, so widening the sanction would
   graft a tier assignment onto an exception, and it would read as permitted rather than required.
2. The failure spans every kit that ships a template. The corpus derivation returns files under seven
   kit roots plus this repo's agent definitions, and §Operative residency belongs to delegation-kit
   alone. The doctrine owns cross-kit rules.
3. The rule is Content-tiering's own shape — one content tier per surface — applied to a surface
   class the rule does not yet name. Widest-true-tier placement puts it in the doctrine.

**History leaves; grounds relocate.** This adopts the lead's reading on the sweep entry, with one
bound. The operator direction on that entry (2026-09-11) makes version control the home of a
template's history. A failure an instruction exists to prevent is that instruction's ground, not its
history, so it relocates, undated as the provenance seam requires. §The delegation model already
admits that class.

**The authoring-time readers, and their limit.** Build lands every template edit, feature and debt
alike. The amendment template's Definition of Done reaches replacement text at authoring, at align,
and at merge. Neither reaches a close-stage compression pass or an operator-ruled hotfix; the filed
`close-differential-instruction-sweep` is the backstop for both. No gate is owed, which the entry
records as settled.

**The Definition-of-Done item names the rule rather than linking it.** `installer/profiles.list` puts
canon-kit in the `prose` profile, which vendors no doctrine-kit, so a link would dangle there.

**Not widened into a general re-phrase-before-append rule.** The operator direction reaches authoring
generally. This entry's envelope is the instruction surface's tier, so the wider reach is filed
through the gap inbox and not folded in here.

## What changes

### (1) Content-tiering names the instruction surface

`doctrine-kit/DOCTRINE.md` rule 1, **Content-tiering / SSOT**, gains one sentence after its opening
sentence {design-bearing}. **Not yet applied:**

> An instruction surface — template, agent definition, binding shim — owns the instruction tier: its
> grounds go to the section owning the mechanism (a failure it prevents included, undated), its
> history to version control.

Its *Enforced by:* line gains a closing clause. **Not yet applied:**

> ; the instruction-surface sentence is read at authoring — build's ritual, the amendment Definition
> of Done — and is not gateable.

The rule's name and `*Digest:*` trailer are unchanged. `check-doctrine-registration` assertions B, C
and E read only those, so the installed digest and this repo's agent-file block do not move.

### (2) §Operative residency marks where its reach ends and names its readers

The opening sentence of `delegation-kit/SPEC.md` §Operative residency gains a following sentence
{design-bearing}. **Not yet applied:**

> Grounds a surface authors for its own instruction are no restatement and take no sanction;
> Content-tiering / SSOT places them (doctrine-kit/DOCTRINE.md).

The section's reader sentence gains a parenthetical after *at authoring* {mechanical}. **Not yet
applied:** *(a) and (b) are read by whoever authors a restatement, at authoring — build's ritual,
the amendment Definition of Done — and again by a reviewer…*

### (3) The build ritual carries the read

`lifecycle-kit/templates/stages/build.md` §Session ritual gains a two-line paragraph after the
build-time question triage {mechanical}. **Not yet applied:**

> **Instruction-surface edits carry the instruction only.** Grounds → the mechanism's owning section,
> same commit; history → the commit message (doctrine-kit/DOCTRINE.md, Content-tiering / SSOT).

In `lifecycle-kit/SPEC.md` §templates/stages/, the opening sentence's list of what the templates
carry gains one item {mechanical}. **Not yet applied:** *…its stage-local doctrine, the
instruction-surface read in `build` (the one stage every template edit passes), and the
resume-journal last step…*

### (4) The amendment template's Definition of Done carries it

`canon-kit/templates/SPEC-amendment.md` §Definition of Done gains an item directly after **Causal
completeness** {mechanical}. **Not yet applied:**

> - [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent
>       definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).

In `canon-kit/SPEC.md` §The amendment lifecycle, the closing sentence's checklist roster gains the
item in place {mechanical}. **Not yet applied:** *…includes causal completeness, instruction-only
replacement text (the doctrine rule named, not linked: the prose profile vendors no doctrine-kit),
merged-with-no-information-lost, …*

## Producers and consumers

- **The instruction-surface sentence (delta 1).**
  - Producer: `doctrine-kit/DOCTRINE.md`. Every profile that vendors a stage template also vendors
    doctrine-kit.
  - Consumers: a build session at each template edit, through delta 3; an authoring stage, align and
    a merging build session at each amendment, through delta 4.
  - No field is introduced.
- **The ritual paragraph (delta 3).** Producer: `build.md`, loaded through the consumer's build
  binding. Consumer: that session, at the edit.
- **The Definition-of-Done item (delta 4).** Producer: the amendment template, copied at authoring.
  Consumers: the authoring stage, align, and build at merge.
- **Readers of the touched files, with each one's red condition:**
  - `check-doctrine-registration`: a rule name missing from the digest, a digest name with no rule,
    or a rule without exactly one `*Digest:*` trailer. Delta 1 changes neither the name nor the
    trailer.
  - `check-skill-binding`: a slot mismatch. Delta 3 adds no slot.
  - `check-stage-skill-coverage`: an executed stage surface without its journal last step. Delta 3
    keeps it.
  - `check-shim-restatement`: a span a shim copies from the template corpus. Deltas 3 and 4 add
    text, so build runs the gate.
  - `check-footprint-fresh`, `check-value-rollup-fresh`: a stale byte-compare of pages counting
    template lines. Deltas 3 and 4 move those counts.
  - canon-kit prose gates over `DOCTRINE.md` and the SPECs: every added phrase is undated, cites
    sections by heading, and states no bare count.
- **Narrowing (point 5).** No delta narrows a corpus.

## Existing sections updated

- `doctrine-kit/DOCTRINE.md` rule 1, Content-tiering / SSOT — its statement and *Enforced by:* line
  (delta 1).
- `delegation-kit/SPEC.md` §Operative residency — the opening sentence and the reader sentence
  (delta 2).
- `lifecycle-kit/SPEC.md` §templates/stages/ — the opening sentence (delta 3).
- `canon-kit/SPEC.md` §The amendment lifecycle — the checklist sentence (delta 4).
- `docs/doctrine-kit/DOCTRINE.md`, `docs/delegation-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md`,
  `docs/canon-kit/SPEC.md` — generated mirrors, regenerated with the command the mirror freshness
  gate prints (all deltas).
- `docs/footprint.md` and `docs/value.md` — generated pages counting template lines (deltas 3
  and 4).

## Retired spellings

- None — every delta extends a sentence or adds an item; no name is removed.

## Definition of Done

- [ ] **Causal completeness** — the sentence, the ritual paragraph and the checklist item each have a
      named producer and consumers; no field is introduced.
- [ ] **Instruction surfaces: instruction only** — deltas 3 and 4 add instruction only.
- [ ] **Merged by re-phrasing** — each addition extends an existing sentence where one exists, and
      each touched section grows by no more than the phrases above (operator direction, 2026-09-12).
- [ ] **Merged with no information lost** — each merged section reads whole without this file.
- [ ] **Amendment deleted** — no root-level `SPEC-instruction-tier.md` remains.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found during the work goes through the gap inbox.
