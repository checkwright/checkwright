# SPEC amendment: align-claims

**No delta is applied; every replacement passage below is Not yet applied.**

This amendment serves `align-checklist-fanout-calibration`. It answers the three questions that
entry's residue names: how much of the fix is a checklist edit to align's template, how much is a
further gate, and whether a "keeps" list stands as an input at all. It answers nothing else.

**Out of scope, stated so its silence is not read as a ruling:** whether a miss that build absorbs
in-session counts against align's tier. That question sits on the revert signal in this repo's lead
binding (`.claude/commands/lead.md`, the `ruling-config` slot). No delta here reads that signal,
widens it or narrows it.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-12 against `lifecycle-kit/templates/stages/align.md` at the commit that stamped this stage.

**The miss the latest reading recorded is outside every check align's template carries today.** An
amendment asserted that one gate applies a non-empty-value check which a different gate holds. One
line of the first gate's source falsifies it. Walked against each paragraph of the template that
checks something:

- The self-audit paragraph (**Audit the amendment against itself before auditing it against the
  tree.**) reaches an author-stated count and an unowned update target. The miss is neither.
- The tool-behaviour paragraph reaches *a grammar the amendment states about a tool's behaviour*,
  meaning a modelled language. A claim that a gate applies a check is a behaviour claim and no
  grammar, so the paragraph's own subject excludes it.
- The roster paragraph (**The `## Existing sections updated` roster is checked from the tree**)
  names a semantic over-claim among its residue, but its subject is a surface the change touches
  that the roster omits. A claim inside the amendment's own design is not such a surface.
- The wires-cleanly paragraph reaches *every cross-component literal*, verified at the consumer's
  match arms, and claims of absence. A claim about which gate holds which check names no literal
  crossing a seam, and it is a claim of presence.

So a checklist edit is owed, and it widens one paragraph's subject rather than adding a paragraph
class.

**Why no further gate.** Deciding whether an amendment's sentence truly describes a program means
reading the program against prose. A scanner would first have to decide which sentences are claims
and which program each one names. That is exactly the class Enforcement-first's false-positive
carve-out keeps out of the battery. The carve-out requires an event-keyed cadence, and one already
exists: the align stage itself, which `check-stage-entry` assertion C forces on every
cross-component amendment.

**Why a "keeps" list is a claim to verify and never the audit's boundary.** A list of what a change
leaves untouched is the author's claim about the change. It is written from the same model of the
tree that produced whatever the amendment got wrong, so an audit that scopes itself by the list
inherits that blind spot. The first reading's residue is the same shape seen from the positive side:
ten defects, six of them a section the update roster failed to name. The retired-spelling block
exists because a roster has no negative form. A keeps list is a negative form, but one the author
writes rather than one a gate checks, so each item is a negative claim. Align already names negative
claims its weakest evidence.

**Oracle-first is not crossed.** Its source clause forbids opening gate source to predict a verdict.
Checking whether an amendment describes a program truly is claim verification, not verdict
prediction. One invocation stays the first move; the implementing line is the fallback where no
invocation is cheap.

**The sweep lands on the same file.** `instruction-surface-sweep` (SPEC-sweep.md)
rewrites `align.md` to instructions only and moves its attested cases into §templates/stages/.
Deltas 1 and 2 give instruction text only, so either form of the template takes them unchanged.
Delta 3's grounds join whatever that sweep moves into the same section. Whichever batch lands second
merges into the other's text rather than overwriting it.

## What changes

### (1) The tool-behaviour paragraph widens to every claim about an existing program

The `align.md` paragraph whose lead-in is **A grammar the amendment states about a tool's behaviour
is run against that tool.** is replaced whole {mechanical}. **Not yet applied:**

> **A claim an amendment makes about an existing program is checked against that program.** A
> grammar it models, a check it says a gate applies, a refusal it says an arm makes: one invocation
> settles it, or, where no invocation is cheap, the line that implements it. When the named program
> does not hold the behaviour, find the one that does.

The attested case the replaced paragraph carries leaves the template and moves into delta 3's
paragraph, as the instruction-surface rule requires (SPEC-instruction-tier.md delta 1).

### (2) An amendment's statement of what it leaves untouched is a claim, not a boundary

`align.md` gains a paragraph directly after the one whose lead-in is **Every amendment's "wires
cleanly against the current tree" is a hypothesis, and the align audit is its first test.**
{mechanical}. **Not yet applied:**

> **An amendment's list of what it keeps, leaves unchanged or does not touch bounds nothing.** Read
> each item as a negative claim and check it the way the paragraph above checks absence.

### (3) §templates/stages/ carries the grounds for align's claim checks

`lifecycle-kit/SPEC.md` §templates/stages/ gains a paragraph after the one on the `close` template's
release-disposition step {design-bearing}. The build session places it in whatever form the sweep has
left that section, and merges any align case the sweep relocated there. **Not yet applied:**

> **The align template's claim checks are scoped by subject, and no gate backs them.** A claim that
> a gate applies a check is a statement about a program without being a modelled grammar or a
> literal crossing a seam, so a checklist scoped to those two shapes passes it. The template
> therefore checks every claim an amendment makes about an existing program against that program,
> by invocation first. That is claim verification, not the verdict prediction Oracle-first's source
> clause forbids. A modelled grammar is the case that first earned the check: an amendment specified
> a pin-path grammar admitting a leading bracket step, the gate refused any path not opening with
> `.`, and the modelled tool read a leading `["k"]` as an array literal rather than an index. That
> three-way disagreement was caught only by the first differential run at build.
>
> A list of what an amendment leaves untouched is the author's claim about the change, written from
> the model that produced any miss in it, so an audit bounded by it inherits the blind spot. The
> template reads it as negative claims instead.
>
> Whether prose truly describes a program is a reading no scanner makes without flagging correct
> prose, so these checks stay a stated duty. Their cadence is the align stage itself, which every
> cross-component amendment fires (§check-stage-entry, assertion C).

## Producers and consumers

- **The two checklist instructions (deltas 1 and 2).**
  - Producer: `align.md`, loaded by every align session through the consumer's align binding.
  - Consumer: that session, during its audit of each amendment.
  - No new state, event, field or interface is introduced.
- **The other readers of `align.md`, with each one's red condition:**
  - `check-skill-binding` reds on a slot mismatch between template and binding. Both slots,
    `consistency-gate` and `audit-fanout`, are untouched.
  - `check-stage-skill-coverage` reds when an executed stage surface lacks the resume-journal last
    step. That step is untouched.
  - `check-shim-restatement` reds on a span a binding shim copies from the template corpus. Delta 1
    removes template text, which can only remove a copied span. Deltas 1 and 2 add text, which a
    shim could in principle already match, so build runs the gate rather than inferring its verdict.
  - `check-footprint-fresh` and `check-value-rollup-fresh` red on a stale byte-compare of generated
    pages whose figures count template lines. Both deltas move those counts, so both pages are
    regenerated.
- **Narrowing (point 5).** Delta 1 removes a paragraph from a governed corpus. The only reader whose
  red condition turns on the removed text is `check-shim-restatement`, and its red, a found copy, is
  monotone in the removed span. No reader reds on finding none, asserts an exact count, or holds a
  floor over this file.
- **This repo's `audit-fanout` binding** (`.claude/commands/align.md`) names which corpus to audit,
  which is consumer residue. The claim checks are generic, so they land in the kit template and the
  binding is unchanged.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §templates/stages/ — the grounds paragraph (delta 3).
- `docs/lifecycle-kit/SPEC.md` — the generated mirror, regenerated by the command its freshness gate
  prints (delta 3).
- `docs/footprint.md` and `docs/value.md` — generated pages whose figures count template lines
  (deltas 1 and 2).

## Retired spellings

- None — delta 1 replaces a paragraph whose bold lead-in no surface outside `align.md` cites (probed
  with `git grep -n "grammar the amendment states"`), and no other delta removes a name.

## Definition of Done

- [ ] **Causal completeness** — the two checklist instructions have a named producer and a named
      consumer; no field is introduced.
- [ ] **Instruction surfaces carry instructions** — deltas 1 and 2 are the instruction alone; their
      grounds are placed by delta 3.
- [ ] **Merged with no information lost** — delta 3 integrated into §templates/stages/ beside
      anything the sweep relocated there, not appended; the replaced paragraph's attested case is
      present in that section.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed through the gap
      inbox.
