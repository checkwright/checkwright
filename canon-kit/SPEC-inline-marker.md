# SPEC amendment: inline-marker

§check-measured-claim binds a `measured:` marker as a full-line HTML comment on
the line above its claim, and the claim is the whole paragraph below. A claim
that stands **mid-paragraph** can carry no marker, so it goes stale unwatched.
Putting a comment on its own line there splits the rendered paragraph. Hoisting
it to the paragraph's top binds it to every sentence, so arm C fails closed as
soon as a second sentence carries a cardinal. This amendment adds an **inline**
form. A marker written inside a line binds the one sentence it follows, and the
three arms apply to that sentence unchanged.

**The measurements this amendment rests on (2026-09-22).**

- **The attested instance has since been retired, and the limit has not.** The
  entry cites TRAJECTORY.md's port-figure paragraph. `git log -S "thirty-five owed" -- TRAJECTORY.md`
  shows a later close removed it, and the file now carries no figure at all. The
  contract limit is canon-kit's and ships with it: an adopter whose governed prose
  carries a mid-paragraph measured claim gets a gate that cannot reach it. The
  entry's not-icebox clause rests on that, and it still holds.
- **An inline HTML comment already sits in this tree's governed prose.**
  canon-kit/SPEC.md §Layout and configuration carries an inline
  `prose-enum-exempt:` comment at the end of a prose line, and so does its
  generated mirror under `docs/` (`grep -c` finds one in each). Markdown passes
  an inline comment through as raw inline HTML, which a browser does not display,
  and the paragraph is not split.
- **The discharge window already sees an inline marker.**
  `spec::walk_prose_multi` tests the exempt window with `contains`
  (`native/src/spec.rs`), so §check-manifest-count's `measured:` discharge
  already fires on a marker anywhere on the line or the line above. The other two
  readers test `starts_with`. `measured_claim.rs` does so in `collect` and
  `bound_claim`, and `unmarked_claim.rs` in its paragraph test.
- **No live claim in this tree needs the form today.** The only markers on the
  measured surface are the two full-line `gate-substrates` markers in
  CONTRIBUTING.md and SECURITY.md. No sweep is owed.
- **One mid-line occurrence exists, and it is a specimen.** A `git grep` for
  the marker opener over the measured surface globs, excluding line-initial
  hits, finds only canon-kit/SPEC.md §check-manifest-count's
  `` `<!-- measured: <key>=<value> -->` ``. It sits in an inline code span,
  which is why delta 1 excludes code spans. Without that rule the specimen
  would fail arm B closed.

**Which figures to mark stays the consumer's policy.** The entry's second half
is that most dated figures are frozen attestations, and marking them would fight
the rule that a date freezes a claim. That is a reading of the consumer's own
record. The kit ships the grammar only. In this tree the reading already has a
home: TRAJECTORY.md's *correcting an aged fact* act names which figures are
corrected where they stand.

*Ruled out: a hoisted marker carrying an anchor phrase.* A marker at the
paragraph's top that quotes the words of the sentence it binds would work, but
it is a second copy of the claim, and it drifts when the sentence is reworded.
The inline form binds by position, which a reword does not move.

## What changes

### (1) §check-measured-claim admits an inline marker binding one sentence {design-bearing}

**Not yet applied.** Replace the paragraph opening **The marker binds a claim to
an oracle key and its measured value**, and its fenced grammar, with:

> **The marker binds a claim to an oracle key and its measured value**, as an
> HTML comment in one of two positions:
>
> ```
> <!-- measured: <key>=<value> -->
> ```
>
> **Full-line**, alone on the line immediately above the claim, binding the
> paragraph below. **Inline**, anywhere else in a line of prose, binding the one
> sentence the marker follows. A mid-paragraph claim takes the inline form,
> because a full-line comment there would split the rendered paragraph. An
> occurrence inside an inline code span is the grammar being shown, not a marker,
> just as an occurrence in a fence is.

Replace the sentences from "The claim a marker binds is the paragraph below it"
through "…which one the marker holds —" with:

> The claim a full-line marker binds is the paragraph below it, ending at a blank
> line, a fence, a second full-line marker or the end of file, less any sentence
> an inline marker in it binds. An inline marker binds the sentence ending at the
> marker. Paragraph text is rejoined across its wraps, and a sentence ends at `.`,
> `?`, `!` or `;` followed by whitespace, which is §check-provenance-seam's
> sentence. A terminator separated from the marker only by whitespace closes the
> bound sentence rather than opening it. Marker text is never part of any
> claim. **The authoring contract arm C prices:** a bound claim carrying more
> than one distinct cardinal is ambiguous, and the gate fails closed rather than
> guessing which one the marker holds —

The remedy in the fail-close help line becomes "split the sentence, or mark the
sentence carrying the measurement inline". The `bad/` fixture gains an inline
marker whose sentence lacks its cardinal while a neighbouring sentence carries
it, and it must red on arm C. The `good/` fixture gains a paragraph with a
full-line marker and an inline-marked sentence, each carrying its own cardinal,
plus a backticked marker specimen naming an unemitted key, and it must pass.

The contract is fixed here. What stays generative is the sentence scanner and
its placement inside `collect`, which today reads full lines only.

### (2) §check-unmarked-claim and §check-manifest-count read the inline form {mechanical}

**Not yet applied.** In §check-unmarked-claim, replace "**A paragraph is the
unit, and it is §check-measured-claim's paragraph**, so the both gates agree on
what a marker binds: …" through "…a property of the paragraph rather than of a
line." with:

> **A paragraph is the unit, and it is §check-measured-claim's paragraph**, so
> both gates agree on what a marker binds. A full-line marker heading the block
> discharges the whole paragraph. An inline marker discharges a class match that
> starts inside the sentence it binds, and no other, so a marked figure does not
> vouch for an unmarked claim beside it.

In §check-manifest-count, replace "It rides the same per-site window the exempt
marker does — the marker line and the claim below it —" with:

> It rides the same per-site window the exempt marker does: the marker's line and
> the line below it, for a full-line and an inline marker alike.

The gate already behaves this way, so that edit is text only. Both gates' `help:`
lines replace "on the line above" with "on the line above, or inline after the
sentence". Each fixture pair gains an inline-marked case: `check-unmarked-claim`'s
`good/` passes one, and its `bad/` carries an inline marker on the wrong sentence
that must still red.

## Producers and consumers

- **The inline marker** (delta 1). Producer: an author, on any file the measured
  surface globs reach. That is reachable, because this tree sets
  `CANON_KIT_MEASURED_CLAIMS_CMD` and the surface globs. Consumers:
  `check-measured-claim`, whose arms A, B and C read its key, value and bound
  sentence at the scan transition. `check-unmarked-claim` reads its bound span
  (delta 2). `check-manifest-count` reads its line through the existing window.
- **Fields.** The key and value are read by arms A and B. The bound sentence is
  read by arm C and by `check-unmarked-claim`. The marker's line is read by the
  finding's report and by `check-manifest-count`'s window. There is no new field.
- **Other prose gates** read an inline marker's text the way they read an inline
  exempt tag today. `check-prose-tells`' rhythm assertion counts its words in the
  sentence it sits in. That is unchanged behavior for a comment it already meets,
  and no finding on this tree depends on it.
- **Point 5**, bound by delta 1, which narrows the full-line marker's bound claim
  by the inline-bound sentences. Arm C's red condition is a marker cardinal absent
  from its claim, or two distinct cardinals in it. Removing a sentence bound
  elsewhere can take away the only token carrying a full-line marker's cardinal.
  Today no paragraph on the measured surface carries both forms. The one mid-line
  occurrence is a code-span specimen, which delta 1 excludes. So the narrowing
  reds nothing, and it can only bite where an author added an inline marker. That author is the one to split the paragraph.
- **Point 6** does not bind. No delta obliges every member of a corpus.

## Existing sections updated

Rosters from `grep -rn "MEASURED_MARKER" native/src` and
`git grep -n "line above\|measured:" -- canon-kit/SPEC.md native/src/gates/`, run 2026-09-22.

- canon-kit/SPEC.md §check-measured-claim, the marker-grammar and bound-claim paragraphs (delta 1).
- canon-kit/SPEC.md §check-unmarked-claim, the paragraph-unit paragraph (delta 2).
- canon-kit/SPEC.md §check-manifest-count, the sanctioned-discharge paragraph (delta 2).
- `native/src/gates/measured_claim.rs`, `collect`, `bound_claim` and the help line (delta 1).
- `native/src/gates/unmarked_claim.rs`, the paragraph test and the help line (delta 2).
- `native/src/gates/manifest_count.rs`, the help line (delta 2).
- `canon-kit/gate-tests/check-measured-claim/` and `canon-kit/gate-tests/check-unmarked-claim/` (deltas 1 and 2).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`.

## Retired spellings

- None — no delta of this amendment removes a name; the marker keeps its spelling and gains a position.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the inline marker.
- [ ] **Instruction surfaces: instruction only.** Not reached, since no template
      or shim changes.
- [ ] **Merged with no information lost.** Each replaced paragraph is re-phrased,
      and the full-line form reads exactly as it did.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entry moved.** `measured-marker-cannot-sit-mid-paragraph` moves to Done
      in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
