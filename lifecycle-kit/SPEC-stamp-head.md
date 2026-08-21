# SPEC amendment: stamp-provenance ordering

Closes `stage-stamp-ordering-unenforced`: a stage stamp proves the stage skill
was invoked, and nothing proves it was invoked **before** the work it authorizes.
The gate that would catch a late stamp is path-coupled to surfaces a work commit
need not touch, so it never runs; and when it does run it reads point-in-time
disk state, where a stamp that landed last is byte-identical to one that landed
first.

## The fork this amendment settles, and the class call that follows

The entry carried one unresolved fork through six declines — the **cheap half**
(widen a gate's couple set so a stage's own output surfaces re-fire it) against
the **history assertion** (assert from git history that no commit touching a
stage's output surfaces precedes its stamp) — with the standing worry that buying
the cheap half first forecloses the reading the assertion needs. Both halves are
**refused**, and neither is foreclosed by the other, because the assertion this
amendment buys attaches somewhere neither of them looked.

- **The cheap half is not bought, and it is not needed.** The assertion below
  fires at the **stamp commit**, which by definition writes
  `.workflow/WORKFLOW-STATE.txt` — already in `check-stage-evidence`'s
  `couples=`. The gate runs there today. A widened couple set would buy re-firing
  at commits the assertion does not read, at the price of running the gate on
  every kit-source edit in the tree.
- **The history assertion is not bought, and the surface-set question that made
  it expensive is the reason.** "A stage's own output surfaces" is not derivable
  from the tree: a build stage's outputs are approximately everything, and scope,
  spec and close all reach kit sources too. A per-stage surface roster is
  consumer rule content that must be configured, maintained and kept honest, and
  its precision is unproven in either direction — coarse, it false-fires; narrow,
  it misses. The assertion below needs no roster and no history walk.

**The class is `feature`.** The entry left the call to "the promoting scope call",
a sentence scope corrected as stale under this roster's split authoring. The call
is taken here, and it is taken on a ground the entry's own two-branch framing did
not carry: the assertions land *inside* `check-stage-evidence`, which the entry
called the debt path, but they read a **fifth field on the stamp line** — a
change to a file format four kits parse, which is a contract another component
must honor, and canon-kit/SPEC.md §The amendment lifecycle's new-names litmus
makes that a feature however small the diff.

## What changes

### (1) The stamp records the commit it was taken at

The stage-evidence data-line grammar gains a fifth field:

```
<iteration> <stage> <session-id> <date> <head>
```

`<head>` is the abbreviated commit `git rev-parse --short HEAD` returned at the
instant `bin/enter-stage.sh` wrote the stamp, or the literal `none` where the
tool found no git work tree or no commit to name. The field is **required**;
`none` is a value, not an omission. **Design-bearing** — the field's existence,
its sentinel and the required-not-optional call are the amendment's load-bearing
choice; writing it is a two-line change in one tool.

The field is appended rather than inserted so every positional reader of fields
one to four is unmoved.

**Why `none` cannot be written inside a repository.** A permanently optional
field is a permanent disarm switch: any writer that omitted it would silently
opt out of the assertion the field exists to serve, which is enforcement-first
inverted — shipping the fix beside its bypass. So the grammar requires five
fields, and delta (3)'s assertion refuses a `none` on a newly introduced stamp
when the gate itself finds a git work tree. Outside a work tree — a fixture case,
a vendored tree under test — `none` is the honest value and the assertion is
inert.

### (2) The one-time grammar migration

`check-stage-evidence`'s grammar assertion moves from *exactly four fields* to
*exactly five*. Existing stamps in a live state file are rewritten in the same
unit: for each, the `<head>` is the **first parent of the commit that introduced
that stamp line**, recoverable from history, or `none` where it is not.
**Mechanical**, with one judgment inside it — a stamp whose introducing commit
cannot be identified takes `none` rather than a guess.

This repo's file carries two stamps at the time of writing, both from this
iteration. The migration is bounded by the iteration, because
`bin/enter-stage.sh`'s boundary reset truncates the file at the next first-stage
entry and every stamp after this unit is written five-field by construction.

**This is a breaking change to a shipped file format, and it is stated rather
than softened.** A consumer who vendors the upgrade mid-iteration reds until they
rewrite their own stamps. The release policy's security-or-supply-chain trigger
does not fire (TRAJECTORY.md §The closed rulings), and the honest mitigation is
the boundary: a consumer who upgrades at an iteration boundary pays nothing.

### (3) `check-stage-evidence` gains the stamp-provenance assertion

A stamp is **newly introduced** when no data line in `HEAD`'s version of the
state file carries the same `<session-id> <head>` pair. Identity is that pair
rather than the whole line, because `bin/enter-stage.sh --rename` rewrites
column 1 of every data line and must not read as re-introducing all of them.
For every newly introduced stamp, inside a git work tree: **`<head>` must name
`HEAD`.** **Design-bearing.**

The comparison is prefix-shaped — the recorded abbreviation, at least seven
characters, must prefix the full `HEAD` — so a consumer's `core.abbrev` does not
enter the contract.

What it catches, and why it catches it without history or a surface roster: a
session that stamps first and commits the stamp *after* work commits has moved
`HEAD` between the write and the commit, so the recorded head is stale and the
gate reds naming both commits. That is exactly the attested 2026-08-07 firing,
where a build batch stamped as its third commit behind two commits of build-stage
edits and the difference was invisible to every gate.

Outside a git work tree the assertion is inert and `none` is accepted; inside
one, `none` on a newly introduced stamp is a red, which is what keeps the
inertness from being a disarm.

### (4) `check-stage-evidence` gains the stamp-commit purity assertion

Delta (3) alone is defeated by a session that writes the stamp and its work into
**one** commit: `HEAD` has not moved, so the recorded head is current. The
complement closes it. Where a stamp is newly introduced, the commit introducing
it changes **only**: **design-bearing.**

- the state file — for any stage's stamp; and additionally
- the queue file, the two kit-owned boundary surfaces (lesson evidence, survey
  record), the members of `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`, and the gap-inbox
  file — for the **first stage**'s stamp only, because the iteration-boundary
  reset legitimately writes all of them in one motion.

The exemption set is **derived from the configuration the boundary reset already
reads**, so no roster is minted and none can rot. The assertion is staged-scoped:
it reads the staged path set, and is inert in a battery run introducing no stamp.

This mechanizes a sentence every stage template already carries — *commit the
stamp on its own* — which is enforcement-first applied to prose that had no gate
behind it.

### (5) The reader sweep, and the one reader that actually breaks

A parse-level survey ran at this stage on 2026-08-21 over every component that
reads the state file, with no stderr silenced. **Mechanical**, and the finding is
that the blast radius is far smaller than the reader count suggests: **exactly one
parser breaks.**

- **Breaks — `native/src/gates/stage_evidence.rs`**, whose grammar assertion is
  `if f4.is_empty() || f.len() > 4`. A five-field stamp is rejected outright, and
  because this is the commit-time gate every stamp commit runs, the writer and
  this line must move in one commit or every future stamp reds.
- **Writes — `lifecycle-kit/bin/enter-stage.sh`**, the sole production writer
  (`stamp_line="$stamp_iter $stage $id $today"`), feeding both the boundary-reset
  stamp and the plain append.
- **Survives** — every other parser, Rust and shell alike, reads positionally or
  by prefix: the crate's shared cursor primitive (`stages.rs`, `.nth(1)`),
  `stage_entry.rs`, `evidence_manifest.rs`, `evidence.rs`, `emit/trajectory.rs`'s
  diff prefix match; `lifecycle-kit/lib/stages.sh`, `enter-stage.sh`'s own
  idempotency read (`read -r f_iter f_stage f_id _`), the three
  `awk '{ l = $2 }'` cursor readers in context-kit, this repo's `scripts/`, and
  delegation-kit's statusline, `evidence-kit/lib/evidence.sh`, and drift-kit's
  pickaxe search and its `read -r iter stage session8 _date _rest`, which already
  names a catch-all.
- **Not stamp readers at all**, recorded so the sweep is not re-run over them:
  `merge_attrs.rs` (reads `.gitattributes`), `drift-kit/bin/overhead-meter.sh`
  (counts bytes), and both `workflow-state-guard.sh` copies (compare a path).

Every writer of a synthetic stamp line — the `lifecycle-kit/gate-tests/*.test.sh`
set, `lifecycle-kit/smoke/install.sh`, `drift-kit/smoke/install.sh` — emits five
fields. None asserts a count, so all would keep passing unchanged, which is
precisely why they must be updated deliberately: **no existing fixture exercises a
five-field stamp**, so the field ships with no coverage unless one is added.

The build unit re-runs this sweep against the tree it edits rather than trusting
the roster: a roster in an amendment is a dated measurement, and a reader added
meanwhile is exactly the one a stale list misses.

### (6) `--rename`'s columns-2-4 witness is widened

`bin/enter-stage.sh --rename` proves it touched only column 1 by comparing
`awk '… { print $2, $3, $4 }'` before and after, and its test twin
`gate-tests/rename-iteration.test.sh` hardcodes the same three columns. A fifth
field would ride **outside** that witness, so `--rename` could drop or corrupt it
and neither the tool nor its test would notice. The witness widens to `$2`
through `$NF` in both places, and the SPEC sentence naming it as the
*columns-2-4 witness* is renamed with it. **Design-bearing** — the alternative,
an explicit column-5 check, was refused because it re-hardcodes the arity the
gap came from.

## The honest limit, and the entry it hands the remainder to

**The one case neither assertion reaches**: a session that does its work, commits
it, and only *then* runs `bin/enter-stage.sh`. The stamp legitimately records the
post-work `HEAD`, the stamp commit is pure, and both assertions pass. The work
still preceded the mark.

**That case is unreachable from the entry stamp alone, and the reason is
structural rather than a gap in this design.** The interval between stage X-1's
stamp and stage X's stamp holds X-1's legitimate work and X's illegitimate
pre-stamp work, and nothing in the tree separates them: only a *declared*
boundary can, and the two available declarations are the surface roster this
amendment refuses and an **exit mark** that does not exist. No exit-stamp concept
exists anywhere in lifecycle-kit, and minting one is
`stage-completion-unattested`'s deliverable, filed and distinct on exactly this
axis — there the mark is on time and the deliverable absent, here the deliverable
exists and the mark is mistimed. Closing the remaining case is therefore a
**composition** of the two entries rather than more work inside this one, and
recording that is what stops the next reader re-deriving the surface roster.

What this amendment does close, and it is the entry's stated generalization: the
claim that *every entry-time assertion is satisfiable retroactively* no longer
holds for this one. The recorded head is bound to a commit that already existed
when the stamp was written, and a stamp introduced later is checked against the
`HEAD` of its own introduction, so re-satisfying it means re-stamping — which is
a fresh, honest stamp rather than a forged one.

**Same-stage re-entry is in reach by shape, not by exemption.** A second
session's stamp records its own `HEAD` and is committed on it, so it passes; the
entry's worry that a cheap approximation misfires on a re-entry
(`batch-split-stamp-ownership`) does not arise, because the assertion never
compares two stamps to each other.

**The concurrent-session false fire is real and its remedy is cheap.** This repo
shares a git index between sessions, so a sibling committing between a stamp's
write and its commit moves `HEAD` and reds the stamp. The remedy line says so and
names re-running `bin/enter-stage.sh`, which appends a fresh stamp at the current
`HEAD` — a same-stage re-entry, in-contract, and cheaper than any weakening of
the assertion that would admit the case it exists to catch.

## Producers and consumers

**`<head>`, the fifth stamp field.**
*Producer:* `lifecycle-kit/bin/enter-stage.sh`, at the append that writes the
stamp — the sole sanctioned writer, and the one every stage template's first step
invokes, so the field is produced by the live path rather than only under test.
Its enabling config is none: the field is unconditional, which is what delta (1)
buys by refusing an optional spelling.
*Consumers:* `check-stage-evidence`'s grammar assertion, which reads it as a
field; its provenance assertion (delta 3), which reads it against `git rev-parse
HEAD` at the transition where a newly introduced stamp is detected; and no other.
*Reader named for the field, at a named transition:* the provenance assertion, at
the newly-introduced test. No other reader is added, and a field with no reader
would be removed.

**The newly-introduced test.**
*Producer:* `check-stage-evidence`, reading `HEAD`'s version of the state file
through `git show`. *Consumer:* both new assertions, as their gate. *Red
condition:* neither assertion reds on the *absence* of a newly introduced stamp —
both are inert then. Both red on **finding** a violation among the stamps they do
select, which is monotone in the violation set.

**The purity assertion's exemption set.**
*Producer:* `lifecycle-kit/lib/stages.sh`, which already resolves every member of
it for the boundary reset. *Consumer:* the purity assertion, at the staged-path
comparison. *Red condition:* it reds on **finding** a staged path outside the
permitted set — monotone; widening the exemption set can only remove violations,
and narrowing it can only add them, so a config change is safe to reason about by
inspection in both directions.

**Existing integration prose describing the prior flow** is inventoried in the
next section. The four-field grammar is stated in prose in more than one governed
surface, and every statement of it is a reader in the sense that matters here: a
sentence asserting four fields is false the moment the fifth lands.

## Existing sections updated

Each names the delta that owns it.

- **lifecycle-kit/SPEC.md §check-stage-evidence** — the grammar sentence moves
  from four fields to five and states the `none` sentinel (deltas 1, 2); the two
  new assertions and their inertness conditions land here (deltas 3, 4); the
  section's *Honest limit* paragraph gains the case neither reaches and the
  composition with `stage-completion-unattested` (this amendment's §The honest
  limit); the *uncommitted window* paragraph is re-read, since the provenance
  assertion narrows it — a hand-written stamp must now also carry a head that
  matches, which the guard template's prose should not overstate as closure.
- **lifecycle-kit/SPEC.md §The state machine** — the evidence-file bullet carries
  the grammar's owning statement, the fenced
  `<iteration> <stage> <session-id> <YYYY-MM-DD>` line; it gains the fifth field
  and the `none` sentinel (delta 1). The cursor sentence — the last data line's
  `<stage>` token — is unaffected and is re-read to confirm it.
- **lifecycle-kit/SPEC.md §bin/enter-stage.sh** — the stamp the tool appends
  (delta 1); the *columns-2-4 witness* sentence, renamed with the widened witness
  (delta 6); and the boundary reset's written-surface set as the purity
  exemption's source (delta 4).
- **lifecycle-kit/SPEC.md §check-stage-entry** — it reads the same file through
  the same shared primitive; the survey clears it, and the entry is kept so the
  merge records the clearance rather than leaving it unstated (delta 5).
- **lifecycle-kit/README.md** — restates the grammar in its own words and is
  updated with it (delta 1).
- **lifecycle-kit/templates/stages/\*.md** — all six carry the *First step* stamp
  paragraph, and each spells the four fields (delta 1). The *commit the stamp on
  its own* sentence gains its gate and stops being unenforced prose (delta 4).
- **`installer/lib/common/recipe.sh`** — seeds a fresh state file whose header
  comment states the four-field grammar; it is a second copy of the statement
  rather than a parser, and it moves with the owner (delta 1).
- **drift-kit/SPEC.md §The stage-economics meter** — restates the grammar while
  explicitly attributing ownership to lifecycle-kit, so it is a documented
  pointer rather than a second SSOT; it takes the same mechanical edit and keeps
  its attribution (delta 1).
- **The gates' own message text** — `stage_evidence.rs` and `stage_entry.rs`
  embed the four-field shape in help and error strings. These are not parsers,
  but a message understating the grammar sends a reader to the wrong repair, so
  they move with the assertion (deltas 2, 5).
- **scripts/lifecycle-config.sh** — unchanged; the purity exemption derives from
  the members it already declares, and no knob is added. Recorded so the merge
  does not look for one.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`), discharged at the iteration
      rather than at the commit.
- [ ] **Removals propagated** — grepped every spec, README and template for a
      statement of the four-field grammar; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work are
      resolved that session, not deferred.
