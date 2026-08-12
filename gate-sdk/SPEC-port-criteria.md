# SPEC amendment: port-criteria-expressiveness

Three defects in **one** section — gate-sdk/SPEC.md §The port-candidate criteria
— plus the relabel they enable at §The first cohort, and the rule that selects
the next. Three queue entries point here (`port-criterion-transitive-binding-reach`,
`port-criterion-aggregate-cost-blindness` half (1), `cohort-hold-criterion-label-defect`),
each of which already says a session taking one should read all three. One
amendment rather than three because the three edits interleave inside one
criterion roster: three amendments would each restate the section's structure to
place their own clause, which is the restatement doctrine forbids. **They land in
one build batch**; splitting them strands two entries against an amendment file
step 3 of the merge may not delete.

Nothing here reverses a ruling. The 2026-08-11 `check-roadmap-fresh` hold is
**relabelled, not lifted** (TRAJECTORY.md §The closed rulings, 2026-08-12); the
operator's acceptance of criterion 5's platform cost for `check-measured-claim`
stands; and TRAJECTORY.md §PRIORITY DIRECTIVE's rule that no criterion is an
eligibility screen governs every clause below.

## What changes

### (1) Criterion 4 gets its own predicate, in both directions — **design-bearing**

Criterion 4 today reads *"Not itself substrate-sensitive — porting a gate that
audits gate sources makes the parity proof self-referential."* The term
**substrate-sensitive** is already defined, once, at §Meta-gate conservation for
the binary substrate: *a registry member whose expanded `couples=` covers the
declaration path of a registry member*, derived at runtime by
§check-gate-substrate-parity assertion C. Criterion 4 borrows that term and means
something else by it, and the borrowing is the defect. The two questions:

- **Assertion C asks about the trigger.** Its job is anti-vacuity — a meta-gate
  whose *re-run trigger* reaches a gate declaration must carry a disposition, so a
  port cannot silently end an assertion. Over-selection is cheap there: the price
  of a false positive is one table row. It is deliberately trigger-shaped and it
  stays exactly as it is. **No assertion of `check-gate-substrate-parity` is
  weakened by this amendment.**
- **Criterion 4 asks about the assertion target.** Its stated hazard is a
  self-referential *parity proof*: porting a gate whose scanned corpus contains
  gate declarations changes that corpus at the moment of the port, so the
  cross-substrate comparison is over a corpus that stops existing. A `couples=`
  entry that exists only so a script edit re-runs the gate — the **reverse
  trigger** class §Meta-gate conservation already names — cannot create that
  hazard, because nothing about it is read as content.

Criterion 4 is therefore restated to name its own predicate and to disclaim the
other: *criterion 4 binds where a registry member's **declaration path is inside
the corpus the gate scans as content**. It is not assertion C's derived set: that
set is trigger-derived and deliberately wider, and a member selected by it through
a reverse-trigger couple does not fail criterion 4.* A conservation row and a
criterion-4 hold become independent facts about a member, which is what they
already were.

**The worked instance is live, machine-checked, and is why this clause is bought
now rather than reasoned about later.** Running assertion C's derivation over the
live registry reports **`check-template-registry-parity` substrate-sensitive** —
its `kit:*/*.sh` expands to `gate-sdk/*/*.sh`, which covers
`gate-sdk/checks/check-shellcheck.sh`. Its scanned corpus is `<kit>/<name>/` for
each `<kit>/templates/<name>.list`, live `drift-kit/kpis/` and
`gate-sdk/msg-patterns/`, neither holding a gate declaration; its conservation row
already records exactly that reading (*"their corpus is kit templates and the
template registry, not gate declarations"*). Under the borrowed term the member
reads as a criterion-4 failure and under its own predicate it does not — and
`SPEC-kit-roots.md` needs the answer to take it.

**The other direction — transitive reach — is the same clause's other half.**
`port-criterion-transitive-binding-reach` records the born-native bullet stating
criterion 4 *"does not bind where a member's `couples=` names no registry member's
declaration path and takes no conservation-table row"*, a conjunction
`check-measured-claim` falsified: its `couples=` names `scripts/*.sh`, an
**emitter** that treats declaration paths as a set, so the member earns a
conservation row while its own assertion target is the governed-prose surface. The
restatement above dissolves that misprediction rather than patching it, and the
born-native bullet is rewritten to say so in two sentences instead of one
conjunction: **criterion 4 does not bind, because the gate's assertion target is
not gate source; a conservation row may still be owed, because `couples=` reaches a
declaration-path-processing oracle transitively, and that is assertion C's
question, not this criterion's.** The `check-measured-claim` conservation row
gains one back-reference to the clause and drops nothing.

### (2) Criterion 5 gains a cohort half, and it is measured rather than reasoned — **design-bearing**

Criterion 5 is stated per member (*"Its vendored form stays runnable"*) and
discharged per member by omit-and-declare. `port-criterion-aggregate-cost-blindness`
records the arithmetic that defeats: seven `spec_manifest_files` members ported in
one batch (`f602642d`), each individually passing, and the **aggregate** left a
binary-less payload with no gate asserting markdown-link liveness at all. No
per-member statement can see that, because the quantity is not any member's
runnability — it is the **residual coverage of the shell-declared battery** a
consumer keeps when its host has no artifact.

The added half, stated as an obligation on the porting session:

> **Criterion 5 is priced per member and paid per cohort.** A cohort's aggregate
> cost is the **binary-less residual**: what a consumer whose payload carries no
> artifact for its host still catches once every member of the cohort is a
> descriptor. That is **measured, never reasoned** — the oracle exists and is
> `installer_smoke`'s value arm, which plants a real defect in adopter-authored
> prose and asserts some profile below the maximum catches it
> (installer/README.md §The consumer smoke). A cohort records that verdict in its
> own amendment, against the post-cohort registry. **N members each individually
> runnable is not a discharge**, and citing the per-member reading as one is the
> defect this half exists to name.
>
> The verdict is a **price, not a screen** — TRAJECTORY.md §PRIORITY DIRECTIVE
> forbids reading any criterion as an eligibility gate. A cohort that empties a
> value class still lands, carrying a designed answer named in its amendment:
> restore the class shell-side, make it binary-gated by a declaration the adopter
> receives, or accept and document. What a cohort may not do is land **unpriced**.

**Half (2) is deferred and this clause does not pre-empt it.** Restoring
markdown-link coverage for binary-less payloads keeps `port-criterion-aggregate-cost-blindness`'s
baseline slug, so `installer_smoke` stays held `fail`
(`.workflow/validate-baseline.txt`). This clause names that held row as the
machine-held record of an **unpaid** aggregate price: a held baseline row is the
price staying visible, and it is the only enforcement this half buys.

**The honest limit, stated rather than discovered.** Nothing *forces* a future
cohort to take the measurement — the obligation is prose on the porting session,
because a gate that checked it would have to know what a cohort is, and a cohort
is a queue-and-amendment concept the gate layer does not carry. Filed as a gap
rather than waved at.

### (3) Criterion 7 disclaims the spawn-target question, and the hold is relabelled — **design-bearing**

Criterion 7 already closes with *a blocker "never reads on whether a gate ports"*,
and its roster is derived by `bin/port-blockers.sh` against the tree. That report
clears `bash <emitter> --emit` because `bash` is on `GATE_SDK_PROGRAM_FLOOR` —
probed at scope by running it, which reports **none** of the six
generated-projection freshness gates. Meanwhile §The first cohort holds
`check-roadmap-fresh` naming criterion 7. Both passages are right about different
things, and the label is the defect (operator-ruled 2026-08-12).

- Criterion 7 gains one clause: **it adjudicates only whether the payload carries
  the program a rule invokes. Whether the *target* of a sanctioned spawn is itself
  ported is a cohort-composition question and criterion 7 does not reach it** — a
  clause the criterion's own closing sentence already implies and which a reader
  applying the roster literally will otherwise re-derive.
- §The first cohort's `check-roadmap-fresh` bullet is relabelled off criterion 7
  and onto **cohort composition**, keeping its stated ground verbatim (*nothing in
  the cohort ports the emitter it shells out to*). The 2026-08-11 hold stands and
  is relabelled; a write-up reading it as a reversal has mis-read it.

**The per-member derivation the ruling left to this stage — made, over all six.**
Every member spawns `bash <emitter> --emit`; the emitters are
`context-kit/bin/footprint.sh`, `drift-kit/bin/trajectory.sh`,
`gate-sdk/bin/enforcement-map.sh`, `scripts/gen-value-rollup.sh`,
`scripts/gen-docs-mirror.sh`, `queue-kit/bin/roadmap.sh`. **None of the six is
ported**, so on the key as ruled every member is held for a cohort that ports no
emitter — the family's cost field guessed the key would clear four, and it clears
none. What differs per member is what clearing *costs*, and that is the part worth
recording:

| Member | Beyond the byte-compare | What it owes past its emitter |
|---|---|---|
| `check-footprint-fresh`, `check-trajectory-fresh`, `check-enforcement-fresh` | nothing | nothing — a spawn and a string compare |
| `check-value-rollup-fresh` | marker-block extraction | nothing — the block grammar is the projection's, not a corpus derivation |
| `check-docs-mirror-fresh` | an orphan sweep, its own walk over `<root>/docs` | the walk (the crate already carries it) **and** a fail-closed repair: the sweep silences stderr, so an unreadable tree reads as no orphans |
| `check-roadmap-fresh` | a second assertion over `TASK-QUEUE.md` through `queue_roadmap_entries` | a **criterion 6** answer — `bin/roadmap.sh` calls that same adapter, which is what makes queue-kit/SPEC.md's *"the emitter and the gate can never disagree"* true; porting the gate alone duplicates it with nothing machine-held |

**The transferable conclusion, and it re-points the ordering rule.** The cheap
cohort here is **the six emitters, not the six gates**. A ported byte-comparator
spawning a shell emitter removes no shell, so it buys nothing against the
dual-maintenance ground TRAJECTORY.md §PRIORITY DIRECTIVE rests on; of the three
candidate designs the hold names, only *collapse the emitter itself onto the
binary* pays. §The first cohort's selection rule (*the largest set of
criteria-clearing gates sharing one corpus derivation*) mis-selects this family
because what its members share is a **spawn shape**, not a corpus — recorded as a
worked limit on that rule rather than as a change to it.

**A carried claim is corrected here, because a later selector would inherit it.**
"All six steer their fixture pairs off the live emitter through the `EMIT_SRC`
positional arm" is true of **five**. `check-docs-mirror-fresh` has no `EMIT_SRC`
arm — its single positional is `[root]`, its `good/` and `bad/` args files are `.`,
and it executes the live `gen-docs-mirror.sh` against the synthetic case tree. It
is the one member of the family whose pair already proves the emitter-executing
arm, and the vacuity warning does not apply to it. The warning stands unweakened
for the other five, and their parity must be bought by a live-tree run or a
constructed scenario — criterion 2's `# no-fixture:` treatment, reached through a
bypass arm instead of an absent pair.

## Producers and consumers

This amendment introduces **no state, event, interface, message or field**. Every
delta is prose inside two sections of one already-governed document, so the
causal-completeness check binds on the readers of those sections rather than on a
new flow.

- **Producer of every clause**: a session editing `gate-sdk/SPEC.md`. No enabling
  config; the file is in `CANON_KIT_MANIFEST_FILES` and every prose gate already
  scans it. Delta (3)'s table is prose in the same file.
- **Consumers, named individually.**
  - **Criterion 4's restatement** is read by (a) the *next cohort-selecting
    session* — the reader §The port-candidate criteria opens by addressing — and
    (b) `SPEC-kit-roots.md`, which cites it to take `check-template-registry-parity`.
    No code reads criterion 4: assertion C's derivation is the only mechanized
    reader of *substrate-sensitivity*, and this amendment leaves it untouched.
  - **Criterion 5's cohort half** is read by the porting session at cohort-cut
    time, and its measurement is produced by `installer_smoke`
    (`scripts/evidence-config.sh:25`) and consumed by
    `.workflow/validate-baseline.txt`'s held row plus the cohort's own amendment.
  - **Criterion 7's clause and the relabel** are read by the next selector and by
    `cohort-held-members-port-prerequisites`, whose `check-roadmap-fresh` bullet
    cites §The first cohort for the ground and must not restate the new label.
  - **The per-member table** is read by whichever cohort takes the freshness
    family; it supersedes that entry's cost-field guess, which the entry itself
    marks as a guess.

**Every reader's red condition, because delta (3) narrows nothing and delta (1)
narrows a *description* rather than a corpus.** The three that could red on this
edit are prose gates over `gate-sdk/SPEC.md`, and each is monotone in its
violation set only in one direction, so they are enumerated rather than cleared by
inspection:

- `check-comment-tier` / `check-spec-pointer` red on a **restatement** relocated
  behind a tag rather than deleted. The relabel in delta (3) moves no prose behind
  a tag; it rewrites a label in place. `check-spec-pointer` also reds on a `# spec:`
  line whose section does not exist — delta (3) renames **no heading**, which is
  the property that keeps every existing `§The first cohort, and the rule that
  selects the next` citation resolving.
- `check-md-refs` reds on a reference that resolves to nothing. Every new
  cross-reference above names an existing section or an existing path
  (`installer/README.md §The consumer smoke`, `scripts/evidence-config.sh`,
  `.workflow/validate-baseline.txt`, `bin/port-blockers.sh`), each of which is on
  disk today.
- `check-queue-entry-budget` reds on an entry over 50 lines. All three promoted
  entries take a tag swap and no body growth, so none moves toward the cap.
- `check-prose-enum` reds on a prose enumeration disagreeing with its derived set.
  Delta (3) adds a table of six member names; it is **not** an enum-gated set (the
  gate's sets are the queue tag vocabulary and the kit roster), and no clause here
  states a count of anything derived.

## Existing sections updated

- **gate-sdk/SPEC.md §The port-candidate criteria** — criterion 4's text and its
  born-native bullet (delta 1); criterion 5 gains its cohort half (delta 2);
  criterion 7 gains the spawn-target disclaimer (delta 3).
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** — the
  `check-roadmap-fresh` bullet's label, and the per-member table plus the
  ordering-rule limit (delta 3). Owned by delta (3); no other delta writes here.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — one
  back-reference on the `check-measured-claim` row to criterion 4's new clause
  (delta 1). Nothing else on that row changes, and no disposition is edited.

No canonical section outside `gate-sdk/SPEC.md` is touched. Two surfaces cite this
material and are checked rather than edited: `cohort-held-members-port-prerequisites`
(queue entry, cites §The first cohort for the hold's ground — still true after the
relabel) and TRAJECTORY.md §The closed rulings (records the ruling this discharges;
the ruling's own text says it *"discharges when the relabel lands"*, so the entry is
retired at close, not rewritten here).

## Definition of Done

- [ ] **Causal completeness** — no new state/event/interface; each clause names its
      producer (a SPEC edit) and its consumers by name, and each reader's red
      condition is enumerated above rather than cleared by inspection.
- [ ] **Merged with no information lost** — each clause integrated into its own
      criterion or bullet, not appended as a coda; §The port-candidate criteria
      still reads as one roster of seven.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`),
      and all three pointing entries move together in that commit.
- [ ] **Removals propagated** — grep for the phrase *substrate-sensitive* across
      every spec and confirm each site means assertion C's derived set, not
      criterion 4's predicate.
- [ ] **Gaps filed** — the unenforced aggregate-measurement obligation (delta 2's
      honest limit) is on the gap inbox.
