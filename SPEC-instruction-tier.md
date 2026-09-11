# SPEC amendment: instruction-tier

**No delta is applied; every replacement passage below is Not yet applied.**

This amendment serves `instruction-motivation-owner` and rules the reach question that entry leaves
open. Its sibling `instruction-surface-sweep` (SPEC-sweep.md) applies the rule placed
here across the instruction corpus, which is why that entry's blocked-by edge on this one stands and
why this amendment's deltas land first.

It is a root-level amendment because its deltas span four components: doctrine-kit, delegation-kit,
lifecycle-kit and canon-kit.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-12 at the commit that stamped this stage.

**The reach gap holds at HEAD.** delegation-kit/SPEC.md §Operative residency opens *"A rule may be
restated as an imperative in a surface that does not own it"*. That is a sanction for a copy. A
template paragraph explaining why its own instruction is right is a copy of nothing. Condition (b)
says the reasoning *stays with the owner*, and for such a paragraph there is no owner to name.

**The rule is stated beside the sanction, not by widening it.** Three grounds:

1. The sanction is an exception to Content-tiering, keyed on a reader whose trigger never loads the
   owner. Original motivation has no owner to be unreachable from. Widening the sanction would graft
   a tier assignment onto an exception clause, and it would read as something permitted rather than
   something required.
2. The failure spans every kit that ships a template. The corpus derivation
   (`git ls-files "*/templates/*.md" ".claude/agents/*.md"` minus fixture copies) returns files
   under seven kit roots plus the consumer's agent definitions. §Operative residency belongs to
   delegation-kit, and a delegation-kit rule governing lifecycle-kit's templates is a cross-kit reach.
   The doctrine exists to own exactly that kind of rule.
3. The rule is Content-tiering's own shape — one content tier per surface — applied to a surface
   class the rule's text does not yet name. Widest-true-tier placement lands it in the doctrine,
   which is true for every consumer.

**History is removed; grounds relocate.** This adopts the lead's reading recorded on the sweep entry,
with one bound. The operator direction on that entry (2026-09-11) makes version control the
preferred home of a template's history, so an attested incident, a dated measurement, or a note of
which session did what leaves the surface. A failure an instruction exists to prevent is different:
it is the instruction's ground, not its history, and it relocates stated undated, as the provenance
seam requires of a kit SPEC. §The delegation model already admits exactly that class — a failure
surface, a calibration history, a bound that is correctness rather than preference. The discriminator
is whether the instruction would stand unjustified without the passage.

**The authoring-time readers, and their honest limit.**

- Build lands every template edit, feature and debt alike, so its ritual reaches every landing.
- The amendment template's Definition of Done reaches the replacement text an authoring stage
  proposes, and is read again by align and at merge.
- Neither reaches a close-stage compression pass or an operator-ruled hotfix. The filed
  `close-differential-instruction-sweep` is the backstop for both once it lands.
- No gate is owed. The entry records that as settled, and this amendment inherits it.

**Why the Definition-of-Done item cites the rule by name and not by path.** `installer/profiles.list`
puts canon-kit in the `prose` profile, which vendors no doctrine-kit. A path into doctrine-kit would
dangle in that consumer's vendored amendment template. A rule name cannot dangle.

## What changes

### (1) Content-tiering gains the instruction-surface clause

`doctrine-kit/DOCTRINE.md` rule 1, **Content-tiering / SSOT**, gains a paragraph after its opening
statement and before *Under agent work* {design-bearing}. **Not yet applied:**

> An **instruction surface** — a skill or stage template, an agent definition, a binding shim — owns
> the instruction tier: the operative instruction, its disposition and its command. The grounds that
> justify an instruction belong to the mechanism it drives and live in the section owning that
> mechanism; where no section owns it, the move creates one. The history behind an instruction — an
> attested incident, a dated measurement, which session did what — belongs to version control and
> leaves the surface. A failure an instruction exists to prevent is grounds rather than history, and
> moves as grounds, stated undated.

Its *Enforced by:* line gains a sentence. **Not yet applied:**

> The instruction-surface clause is not gateable — whether a sentence instructs or justifies is a
> reading — so it is read at authoring, by the build stage's ritual
> ([lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) §templates/stages/) and by the amendment
> template's Definition of Done ([canon-kit/SPEC.md](../canon-kit/SPEC.md) §The amendment
> lifecycle).

The rule's `*Digest:*` trailer is unchanged. So is its name, which `check-doctrine-registration`
assertions B and C read. The installed digest therefore does not move, and neither does this repo's
agent-file block.

### (2) §Operative residency states where its reach ends

`delegation-kit/SPEC.md` §Operative residency gains a paragraph directly after the one whose lead-in
is **The anti-licence clause is part of the rule, not commentary.** {design-bearing}. **Not yet
applied:**

> **The sanction reaches restatements and nothing else.** Content an instruction surface authors
> that no other surface owns — the grounds for its own instruction — is not a restatement, so
> (a)–(c) never license it and (b)'s owner does not exist for it. Its placement is Content-tiering's
> instruction-surface clause (doctrine-kit/DOCTRINE.md, Content-tiering / SSOT): the grounds go to
> the section owning the mechanism, and where none exists the move creates it, as §The delegation
> model was given the grounds of the template's worktree rules.

The same section's reader paragraph names its authoring-time readers. Its first sentence is replaced
{mechanical}. **Not yet applied:**

> Each condition has a reader at a transition rather than being self-evident: (a) and (b) are read by
> whoever authors a restatement, at authoring — a build session under its stage ritual, an authoring
> stage under the amendment template's Definition of Done — and again by a reviewer or an authoring
> stage assessing an existing one; (c) is the pointer any later reader — and any content-tiering
> check — follows back to the owner.

### (3) The build ritual carries the authoring-time read

`lifecycle-kit/templates/stages/build.md` §Session ritual gains a paragraph directly after the
build-time question triage {mechanical}. **Not yet applied:**

> **An edit to an instruction surface lands the instruction only.** Its grounds land in the section
> owning the mechanism, in the same commit; any history behind it goes in the commit message, not on
> the surface (doctrine-kit/DOCTRINE.md, Content-tiering / SSOT).

`lifecycle-kit/SPEC.md` §templates/stages/ gains a paragraph after the one on the last step
{design-bearing}. **Not yet applied:**

> **The build template carries the instruction-surface read because build lands every edit.**
> Content-tiering's instruction-surface clause (doctrine-kit/DOCTRINE.md) is read at authoring, and
> build is the one stage every template edit passes — a feature's merge and a debt unit's direct edit
> alike — so its ritual is the reader that reaches both. The step cites the doctrine rather than
> restating the clause. A close-stage compression pass and an operator-ruled hotfix pass no build
> session, and no reader here reaches them.

### (4) The amendment template's Definition of Done carries it too

`canon-kit/templates/SPEC-amendment.md` §Definition of Done gains an item directly after
**Causal completeness** {mechanical}. **Not yet applied:**

> - [ ] **Instruction surfaces carry instructions** — replacement text this amendment gives a
>       template, an agent definition or a binding shim is the instruction alone; a delta places its
>       grounds in the section owning the mechanism (the Content-tiering / SSOT rule's
>       instruction-surface clause).

`canon-kit/SPEC.md` §The amendment lifecycle's closing sentence on the shipped template is replaced
{mechanical}. **Not yet applied:**

> The shipped amendment template ends in a Definition-of-Done checklist that includes causal
> completeness, instruction-only replacement text for instruction surfaces,
> merged-with-no-information-lost, the file-deleted assertions, and gap filing. The instruction item
> names the doctrine rule rather than linking it, because canon-kit ships in a profile that vendors
> no doctrine-kit and a link would dangle there.

## Producers and consumers

- **The instruction-surface clause (delta 1).**
  - Producer: `doctrine-kit/DOCTRINE.md`, vendored with doctrine-kit. It is reachable in this repo
    through the agent file's doctrine link, and in every profile that vendors a stage template, since
    each such profile vendors doctrine-kit.
  - Consumers: a build session at every template edit, through the ritual paragraph (delta 3); an
    authoring stage, align, and a merging build session at every amendment, through the
    Definition-of-Done item (delta 4).
  - Fields: none.
- **The ritual paragraph (delta 3).**
  - Producer: `build.md`, loaded by every build session through the consumer's build binding.
  - Consumer: that session, at the edit.
- **The Definition-of-Done item (delta 4).**
  - Producer: the amendment template, copied when an amendment is authored.
  - Consumers: the authoring stage writing replacement text; align auditing the amendment against
    itself; build ticking the checklist at merge.
- **Readers of the touched files, with each one's red condition.**
  - `check-doctrine-registration` reds on a rule name absent from the digest, a digest name with no
    rule, or a methodology rule without exactly one `*Digest:*` trailer. Delta 1 adds a paragraph
    inside rule 1 and changes neither its name nor its trailer.
  - `check-skill-binding` reds on a slot mismatch; delta 3 adds no slot to `build.md`.
  - `check-stage-skill-coverage` reds when an executed stage surface lacks the resume-journal last
    step; delta 3 leaves that step in place.
  - `check-shim-restatement` reds on a span a binding shim copies from the template corpus. Delta 3
    adds template text, so build runs the gate rather than inferring its verdict.
  - `check-footprint-fresh` and `check-value-rollup-fresh` red on a stale byte-compare of pages that
    count template lines. Deltas 3 and 4 move those counts.
  - The canon-kit prose gates over `DOCTRINE.md` and the SPECs (links, section pointers, temporal
    markers, bare counts) red on what the new text says. Every replacement passage is undated, names
    its sections by heading, and states no bare count.
- **Narrowing (point 5).** No delta narrows a corpus. Delta 2 replaces one sentence with a superset
  of itself.

## Existing sections updated

- `doctrine-kit/DOCTRINE.md` rule 1, Content-tiering / SSOT — the statement and its *Enforced by:*
  line (delta 1).
- `delegation-kit/SPEC.md` §Operative residency — the reach paragraph and the reader sentence
  (delta 2).
- `lifecycle-kit/SPEC.md` §templates/stages/ — the paragraph on the build template's read (delta 3).
- `canon-kit/SPEC.md` §The amendment lifecycle — the closing sentence on the Definition-of-Done
  checklist (delta 4).
- `docs/doctrine-kit/DOCTRINE.md`, `docs/delegation-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md` and
  `docs/canon-kit/SPEC.md` — generated mirrors, regenerated by the command the mirror freshness gate
  prints (all deltas).
- `docs/footprint.md` and `docs/value.md` — generated pages counting template lines (deltas 3
  and 4).

## Retired spellings

- None — every delta adds text beside what stands, and delta 2's replaced sentence keeps every name
  it carried.

## Definition of Done

- [ ] **Causal completeness** — the clause, the ritual paragraph and the checklist item each have a
      named producer and named consumers; no field is introduced.
- [ ] **Instruction surfaces carry instructions** — deltas 3 and 4 add instruction text only; their
      grounds are placed in §templates/stages/ and §The amendment lifecycle.
- [ ] **Merged with no information lost** — each addition integrated into its section, not appended;
      each merged section reads whole without this file.
- [ ] **Amendment deleted** — this file removed on merge; no root-level `SPEC-instruction-tier.md`
      remains.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed through the gap
      inbox.
