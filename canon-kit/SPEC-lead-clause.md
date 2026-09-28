# SPEC amendment: lead-clause

`check-spec-pointer`'s prose-citation pass admits a heading's lead clause, the text before its first comma, em dash or colon, as a match (canon-kit/SPEC.md §check-spec-pointer). Over a family of headings that share a lead clause, that admission resolves citations of sections that do not exist. OpenSpec titles every requirement `Requirement: <name>` and every scenario `Scenario: <name>`, so a bare-prose `§Requirement: Account lockout` resolves through the lead clause `Requirement` in any spec that carries a requirement, whether or not one of them is about lockout. A linked citation is held by `check-md-refs`' anchor check, so the loss is the bare-prose form. companion/SPEC.md §The tested claim records the loss as one of two forms that pass unchecked. This amendment bounds the admission so that a lead clause names one section or none.

One queue entry pairs it: [lead-clause-heading-family](../TASK-QUEUE.md#lead-clause-heading-family).

**The rulings.**

- **Two bounds, each closing a case the other leaves open.** The *family bound* refuses a lead clause that another heading in the same file shares. A lead that several sections carry names none of them, on the ground §check-spec-pointer already gives for a title carried twice. The *separator bound* refuses the lead clause when the fragment continues past it with the heading's own separator. A citation spelling `Requirement: ` has spelled more than the lead, so it is citing a whole heading and must match one whole. The separator bound closes the one-member case, a spec holding a single requirement, where there is no family. The family bound closes a lead cited with no separator after it (`§Requirement (lockout)`), where the fragment never reaches a separator.
- **Measured neutral on this tree.** A variant build of the gate, run over this repository with each bound on, alone and together, returned the same clean verdict as the shipped gate: 5,630 directives and 3,271 prose citations. The lead clause admits eleven citations here, and none continues with its heading's separator. No manifest file here carries a lead-clause family. The same build reds `§Requirement: Account lockout` in the OpenSpec fixture spec under either bound, and passes a citation naming a real requirement whole.
- **The one over-red, stated.** The separator bound refuses a citation that names a lead and then runs on with the same separator as prose, `§Economics — the model tiers per batch` against a heading `Economics — batch, and compact where it pays`. None occurs in this tree. The remedy is the whole heading, or a comma in place of the prose dash.
- **No knob.** Both bounds follow from what a lead clause is for, citing one sentence-shaped heading by its opening, so a consumer has nothing to calibrate. A family in a consumer's own specs is exactly where the admission misleads.
- **The seam.** Kit mechanism only: the gate's resolver in canon-kit. The toolkit that motivated it is named in companion/SPEC.md and nowhere in canon-kit.

**Refused.**

- **The family bound alone.** It leaves a spec with one requirement open, and OpenSpec specs with a single requirement are ordinary.
- **Unique across the whole manifest set** rather than per file. A no-path citation already resolves against the union, and a lead that one file uses once and another uses as a family is still one section's name in the first file. Per-file uniqueness is the invariant §check-spec-pointer holds titles to.
- **Dropping the lead-clause admission.** The eleven sentence-shaped citations here would each need their full heading, and the admission is right for every one of them.

## What changes

### (1) The two bounds on the lead clause {design-bearing}

**Not yet applied.** canon-kit/SPEC.md §check-spec-pointer, the paragraph opening *`check-comment-tier` owns the directive's *shape**: the sentence *and a prose citation may name a heading by its lead clause, the text before its first comma, em dash or colon, since a sentence-shaped heading is cited by that clause and the fragment runs on into prose* is followed by:

> A lead clause names one section or none, so two bounds hold it. It is not admitted where another heading in the same file has the same lead clause, since a family such as `Requirement: <name>` shares a lead that names none of its members. Nor is it admitted where the fragment continues past it with the heading's own separator, since a citation spelling the separator cites the whole heading and must match it whole.

The prose-citation paragraph's closing *Honest limit: a short heading is a prefix of many fragments, so the union under-reds around short titles and never over-reds* becomes *Honest limit: a short heading is a prefix of many fragments, so the union under-reds around short titles. Its one over-red is the separator bound's: a citation naming a lead and running on with the heading's own separator as prose.*

`native/src/gates/spec_pointer.rs`: each `Heading` records the separator its lead clause was cut at. `HeadingCache::headings` drops the lead of every heading whose lead another heading in that file shares. `Heading::prefix_of` refuses the lead where the fragment's remainder opens with that separator. The `lead_clause` directive comment gains both bounds.

### (2) The fixture pair {mechanical}

**Not yet applied.** `canon-kit/gate-tests/check-spec-pointer/bad/` gains a manifest file whose headings are a family, `Requirement: Export` and `Requirement: Import`, and a second manifest file with a single `Requirement: Only` heading. It also gains prose citations: `§Requirement: Account lockout`, red under both bounds; `§Requirement (lockout)`, red under the family bound alone; and, against the single heading, `§Requirement: Other`, red under the separator bound alone. `bad/expect.txt` gains the three findings. `good/` gains a family file cited by a whole requirement heading running on into prose, and keeps its lead-clause citation of *The probe is asymmetric* — but reworded, since the separator bound now reaches it. `good/README.md`'s citing sentence today runs the lead clause straight into the heading's own separator (*"named by its lead clause, §The probe is asymmetric, all resolve"* — the comma-space after the citation is the heading's own cut separator, so the separator bound would refuse it and this fixture would start failing its own `good/` claim). The sentence is reworded so the citation's continuation does not reopen with that separator, for instance *"named by its lead clause: §The probe is asymmetric works the same way"* (a colon precedes the citation instead of a comma, and the continuation is a plain space, not the heading's comma).

### (3) The companion's tested claim {mechanical}

**Not yet applied.** companion/SPEC.md §The tested claim: the paragraph opening **Two citation forms pass unchecked** becomes:

> **One citation form passes unchecked.** `check-spec-pointer` reads a bare `<path>.md §<heading>` citation's path as repo-relative, so a file-relative `spec.md §…` is skipped. A bare `§Requirement: <name>` must name a requirement whole, since a lead clause shared by a family resolves nothing (canon-kit/SPEC.md §check-spec-pointer). The fixtures cite repo-relatively.

## Producers and consumers

Probe: `grep -n "lead" native/src/gates/spec_pointer.rs`; `git grep -n "lead clause"` over the tracked tree; a variant build of `native/` in scratch, run as `check-spec-pointer` over this tree and over a `prose` consumer holding the OpenSpec fixture with the recipe, with each bound on alone and together.

- **The two bounds** (delta 1). Producer: the gate's resolver, on every run of `check-spec-pointer`, which is `install: zero-config` and so registered in every consumer carrying canon-kit. Consumers: the committing session through the output contract, on the pre-commit hook, `run-gates.sh` and CI; the fixture pair (delta 2).
- **Red conditions.** The gate reds on more citations than before, never on fewer, so a tree green today can turn red. Measured on this tree: none turns. A consumer's bare-prose citation of a missing requirement reds, which is the point. The companion arm's green legs do not move: the variant build with both bounds on stays clean over `prose` consumers holding the OpenSpec and Spec Kit fixtures, and a `full` one holding the OpenSpec fixture, each with its recipe applied. Its `check-spec-pointer` defect is a duplicated title, which still reds.
- **Readers of the claim** (delta 3): the companion SPEC's mirror, regenerated.

## Existing sections updated

Roster probe: the probes above, plus `git grep -n "Two citation forms"` and `git grep -n "never over-reds"`.

- `canon-kit/SPEC.md` — §check-spec-pointer (delta 1).
- `native/src/gates/spec_pointer.rs` (delta 1).
- `canon-kit/gate-tests/check-spec-pointer/bad/` and `canon-kit/gate-tests/check-spec-pointer/good/` (delta 2).
- `companion/SPEC.md` — §The tested claim (delta 3).
- `.workflow/surface-ceiling.txt` — `canon-kit/SPEC.md`'s row, re-stamped with `--emit always-loaded --ceiling` if the merge grows it, since `check-surface-ratchet` governs every `*/SPEC.md` here (delta 1).
- `docs/canon-kit/SPEC.md` — the generated on-site mirror, as is `docs/companion/SPEC.md`, each regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1 and 3).
- `.workflow/release-declarations.md` — Tightened gates gains a bullet led by `check-spec-pointer`: a prose citation's lead clause is no longer admitted where another heading in the file shares it, or where the citation continues with the heading's own separator, so a bare citation of a missing member of a heading family such as `Requirement: <name>` now reds (delta 1).

## Retired spellings

- None — every delta adds a bound, fixtures or prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — `bash gate-sdk/bin/build-native.sh`, the full battery, and the canon-kit fixture suite green on the landing commit. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
