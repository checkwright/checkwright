# SPEC amendment: align-claims

**No delta is applied; every replacement passage below is Not yet applied.**

This amendment serves `align-checklist-fanout-calibration`. It answers the three questions that
entry's residue names — how much is a checklist edit, how much a further gate, whether a "keeps" list
stands as an input — and nothing else.

**Out of scope, so the silence is not read as a ruling:** whether a miss that build absorbs
in-session counts against align's tier. That question sits on the revert signal in this repo's lead
binding (`.claude/commands/lead.md`, `ruling-config`); no delta reads, widens or narrows it. It is
filed through the gap inbox so it survives this entry's Done move.

**Every delta re-phrases in place or adds the fewest words that carry the point.** That follows
operator direction, 2026-09-12: SPECs are not expected to grow each iteration, and brief,
phrase-shaped instructions read better to an LLM than verbose ones.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-12 against `lifecycle-kit/templates/stages/align.md` at the commit that stamped this stage.

**The latest reading's miss is outside every check align's template carries.** An amendment asserted
that one gate applies a non-empty-value check which a different gate holds. One line of the first
gate's source falsifies it. Walked against each paragraph of the template that checks something:

- **The self-audit paragraph** (**Audit the amendment against itself before auditing it against the
  tree.**) reaches an author-stated count and an unowned update target. The miss is neither.
- **The tool-behaviour paragraph** reaches *a grammar the amendment states about a tool's behaviour* —
  a modelled language. A claim that a gate applies a check is not a grammar.
- **The roster paragraph** (**The `## Existing sections updated` roster is checked from the tree**)
  names a semantic over-claim among its residue. Its subject is a touched surface the roster omits,
  and a claim inside the amendment's own design is not such a surface.
- **The wires-cleanly paragraph** reaches cross-component literals, checked at the consumer's read
  site, and claims of absence. The miss named no literal crossing a seam, and it was a claim of
  presence.

So a checklist edit is owed. It widens one paragraph's subject rather than adding a paragraph class.

**Why no further gate.** Deciding whether an amendment's sentence truly describes a program is a
reading of the program against prose. A scanner would first have to pick out which sentences are
claims and which program each names, which is the class Enforcement-first's false-positive carve-out
keeps out of the battery. That carve-out asks for an event-keyed cadence, and one already exists:
align itself, which `check-stage-entry` assertion C forces on every cross-component amendment.

**Why a "keeps" list is a claim, never the audit's boundary.** The list is the author's claim about
the change, written from the same model of the tree that produced any miss in it. An audit bounded by
it inherits that blind spot. The first reading's residue is the positive-side twin: ten defects, six
of them a section the update roster failed to name. A keeps list is therefore a negative claim, and
the wires-cleanly paragraph already names negative claims the audit's weakest evidence. The fix folds
into that sentence.

**Oracle-first is not crossed.** Its source clause forbids opening gate source to predict a verdict.
Checking that an amendment describes a program truly is claim verification, not verdict prediction.
Invocation stays the first move.

**The sweep lands on the same file.** `instruction-surface-sweep` (SPEC-sweep.md) rewrites `align.md`
to instructions only and moves its attested cases into §templates/stages/. Deltas 1 and 2 are
instruction text, so either form of the template takes them. Delta 3 joins whatever that sweep moves.
Whichever batch lands second merges into the other's text.

## What changes

### (1) The tool-behaviour paragraph covers every claim about an existing program

The `align.md` paragraph whose lead-in is **A grammar the amendment states about a tool's behaviour
is run against that tool.** is replaced whole by a shorter one {mechanical}. **Not yet applied:**

> **A claim about an existing program is checked against that program** — a modelled grammar, a
> check a gate applies, a refusal an arm makes: one invocation, or the implementing line where none is
> cheap. If the named program lacks the behaviour, find the one that holds it.

The replaced paragraph's attested case moves into delta 3 as grounds (SPEC-instruction-tier.md
delta 1).

### (2) A keeps list joins the negative-claim sentence

In `align.md`'s paragraph whose lead-in is **Every amendment's "wires cleanly against the current
tree" is a hypothesis, and the align audit is its first test.**, the sentence opening *A **negative**
existence claim ("no such harness/helper/gate exists yet") is the audit's weakest evidence shape:* is
re-phrased in place {mechanical}. **Not yet applied:**

> A **negative** claim — "no such harness/helper/gate exists yet", or an amendment's list of what it
> keeps or leaves untouched, which bounds nothing — is the audit's weakest evidence shape:

### (3) §templates/stages/ carries the grounds in brief

`lifecycle-kit/SPEC.md` §templates/stages/ gains a short paragraph after the one on the `close`
template's release-disposition step {design-bearing}. Build places it in whatever form the sweep has
left that section, merging any align case the sweep relocated there. **Not yet applied:**

> **`align`'s claim checks: scoped by subject, ungated.** A behaviour claim about a program is neither
> a modelled grammar nor a seam-crossing literal, so align checks it against the program, invocation
> first — claim verification, not the verdict prediction Oracle-first forbids. First earned by a
> pin-path grammar that the gate and the modelled tool each read differently, caught only by build's
> first differential run. A keeps list is the author's claim, read as negative claims, never as
> scope. No scanner decides whether prose describes a program, so the cadence is align itself
> (§check-stage-entry, assertion C).

## Producers and consumers

- **The two checklist instructions (deltas 1 and 2).**
  - Producer: `align.md`, loaded by every align session through the consumer's align binding.
  - Consumer: that session, auditing each amendment.
  - No new state, event, field or interface is introduced.
- **The other readers of `align.md`, with each one's red condition:**
  - `check-skill-binding` reds on a slot mismatch; both slots are untouched.
  - `check-stage-skill-coverage` reds on a missing resume-journal last step; it stays.
  - `check-shim-restatement` reds on a span a binding shim copies from the template corpus. Removed
    text can only remove a copied span. Added text is new, so build runs the gate.
  - `check-footprint-fresh` and `check-value-rollup-fresh` red on a stale byte-compare of pages
    counting template lines, and are regenerated.
- **Narrowing (point 5).** Delta 1 shortens a paragraph. The only reader whose verdict turns on
  removed template text is `check-shim-restatement`, and its red is monotone in the removed span. No
  reader reds on finding none, asserts an exact count, or holds a floor over this file.
- **This repo's `audit-fanout` binding** names which corpus to audit, which is consumer residue. The
  claim checks are generic, so they land in the kit template and the binding is unchanged.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §templates/stages/ — the grounds paragraph (delta 3).
- `docs/lifecycle-kit/SPEC.md` — generated mirror (delta 3).
- `docs/footprint.md` and `docs/value.md` — generated, counting template lines (deltas 1 and 2).

## Retired spellings

- None — delta 1 replaces a paragraph whose lead-in no surface outside `align.md` cites (probed with
  `git grep -n "grammar the amendment states"`), and delta 2 keeps the sentence's words and adds a
  clause.

## Definition of Done

- [ ] **Causal completeness** — both checklist instructions have a named producer and consumer; no
      field is introduced.
- [ ] **Instruction surfaces: instruction only** — deltas 1 and 2 are instruction; delta 3 places their
      grounds.
- [ ] **Merged by re-phrasing** — `align.md` ends no longer than it started; delta 3 is the only
      added paragraph (operator direction, 2026-09-12).
- [ ] **Merged with no information lost** — delta 3 integrated beside anything the sweep relocated;
      the replaced paragraph's case present in that section.
- [ ] **Amendment deleted** — this file removed on merge; `ls lifecycle-kit/SPEC-*.md` empty.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found during the work goes through the gap inbox.
