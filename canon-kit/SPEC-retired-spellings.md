# SPEC amendment: retired-spellings

**An amendment's `## Existing sections updated` roster can be short by a surface, and today only a
grep finds the missing one — a grep nobody is obliged to run and nothing records having run.** This
amendment mints the record: a mandatory `## Retired spellings` block on every amendment, and
`check-amendment-retired-spelling`, a born-native canon-kit gate that re-runs the declared greps and
reconciles every survivor against the roster.

**The admission left the design question open and this amendment closes it.** The queue entry
`amendment-roster-omission-detection` was admitted as a full unit by the operator on 2026-09-07,
through the lead's question relay in a lead session, lead-relayed — with its own text stating that
the admission does not settle whether **one mechanism covers both** the literal-substitution slice
and the renumber slice, and that spec owns the bounding. Ruled below, at §The two slices: **no, and
the ground is decidability rather than effort.** The literal slice ships as the gate; the renumber
slice is a stated non-target with its ground recorded and its durable fix routed to the entry that
already owns it.

## The two slices, and why one mechanism cannot cover both

The entry names two candidate slices of a class whose general form it already rules ungateable —
"deciding which surfaces an amendment *should* have listed is the semantics of the change", the same
conclusion `canon-kit/SPEC.md` §check-amendment-update-target reaches under **Deliberately not
asserted: roster completeness**. Nothing below re-opens that. What is at issue is only whether either
narrow slice is decidable.

**The literal-substitution slice is decidable, because the retired and the replacing spellings
occupy disjoint token spaces.** After a delta replaces literal `X` with literal `Y`, every remaining
occurrence of `X` in the tree is one of exactly two things: a site the change should have reached, or
a site deliberately left standing. Those two are distinguishable, and the discriminator already
exists in the document — the `## Existing sections updated` roster. A hit at a path the roster names
is a site the amendment claimed; a hit anywhere else is a site it did not. That is a claim about the
file and the tree jointly, which is exactly the shape a scanner reaches, and it is why this slice
gates while the general form does not.

**The renumber slice is not decidable, and the obstruction is structural.** When guard-kit's rule 19
was inserted and rules 19-23 became 20-24, the retired spellings and the replacing spellings occupy
the *same* token space. A tree site reading `rule 19` after that change is either a stale citation to
old-19 (which is now 20) or a correct citation to the newly inserted 19, and **no scanner can tell
which**, because the two are byte-identical and the tree carries no other discriminator. Worse, the
correct post-change sites are dense in the renumbered range rather than rare in it, so a survivor
scan over a renumber returns a hit set dominated by non-violations. That is precisely the cry-wolf
shape `canon-kit/SPEC.md` §check-amendment-update-target already refused a stronger arm for, on
`gate-sdk/SPEC.md` §When a gate earns its place.

The entry adds a second, independent obstruction which this ruling does not need but which points
the same way: the renumber's stale sites "share no spelling" — markdown ordinals, parenthesized
placement citations, a comma-list roster (`rules 14, 15 and 22`), ordinal prose (`seventeen rules
earlier`) — so even a regex generalization of the declaration reaches only some of them. Recorded
because a later reader will reach for "declare a pattern instead of a literal" as the obvious repair,
and it is worth knowing that the repair fails on two grounds and not one.

**So the renumber slice is a stated non-target here, and its durable fix is not a gate.** The queue
already carries the entry that owns it: `guard-rule-number-not-citable-outside-kit`, whose second
disposition is to state in `guard-kit/SPEC.md` that a rule number is not a citable identifier outside
the kit and to have every cross-corpus reference name the rule instead — the way one guard-kit rule
already cites `DOCTRINE.md` by name rather than by number, for this reason. That disposition
**dissolves** the slice rather than detecting it: a citation that names its referent has no numeric
relation left to decay, so there is nothing for a scanner to be ambiguous about. Routing rather than
duplicating is the point — a second mechanism here would gate a defect the other entry's fix
removes.

This amendment does not edit that entry. The cross-reference is filed to the committed gap inbox so
close lands it on the entry's own surface, per CLAUDE.md §Housekeeping's gap-capture rule; a
mid-iteration queue edit on an entry outside this batch is exactly the contention that rule exists
to stop.

## Why a declaration, when the roster's own omission is what failed

The obvious objection is that a declaration of retired spellings has the same hole as the roster it
repairs: an author who forgot to list a surface will equally forget to list a spelling, and a
mechanism that only checks what someone remembered to declare catches nothing on the case that
matters. The objection is real and it is the reason for the block's exact shape.

**The roster's omission is undetectable because the roster has no negative form.** A short roster and
a correct one are the same document — nothing distinguishes "these are all the surfaces" from "these
are the surfaces I found". There is no line an author fails to write, so there is nothing for a gate
to miss.

**A retired-spelling block does have a negative form, and this amendment makes it mandatory.** Every
amendment carries the section, and an amendment that retires nothing says so in one line. An author
who skips the section is now writing an amendment that reds, which is a caught error class where the
short roster was not. What remains uncaught is an author who declares the section, retires a
spelling, and does not name it — real, and strictly smaller than what is uncaught today.

**And the obligation is not new.** `canon-kit/templates/SPEC-amendment.md`'s Definition of Done
already carries **Removals propagated** — "grepped every spec for names this change retired; nothing
dangles". Every amendment is already obliged to run this grep; nothing has ever recorded the run or
checked its result, and the checkbox's stated corpus (`every spec`) is narrower than the surfaces the
attested misses landed on. So the declaration does not add a duty. It writes down a duty that exists,
in a form a gate can re-execute — the enforcement-first move, where the fix and the gate that catches
it land in one unit.

## What changes

### (1) The `## Retired spellings` block joins the amendment grammar

`canon-kit/SPEC.md` §The amendment lifecycle gains the block's grammar beside the delta-ID grammar it
already owns, and `canon-kit/templates/SPEC-amendment.md` gains the section with its authoring
comment {design-bearing}.

The section is **mandatory on every amendment** and its body is one of two forms:

- **The negative form** — a single bullet whose first word is `None` (case-insensitive), followed by
  an em dash and a non-empty reason. `- None — no delta of this amendment retires a spelling.`
- **The positive form** — one or more bullets, each carrying a **backticked spelling** and a
  **delta citation** in the citation grammar §The amendment lifecycle already pins (`delta <N>`,
  `deltas <N>`, comma and `and` continuation, the possessive, `all deltas`). The citation grammar is
  reused rather than re-specified, so the two blocks cannot drift into two dialects of the same
  token.

A bullet's entry is the bullet line plus its indented continuation, the same wrap-straddling boundary
arm B of §check-amendment-update-target crosses, for the same reason: a citation that wrapped across
a newline is still one subject.

The **valve** is `<!-- retired-spelling-exempt: <reason> -->` on the bullet's first line or the one
above, riding the shared exempt window (`canon-kit/SPEC.md` §lib/spec.sh), reason mandatory per the
`comment-tier-exempt:` convention. An exempt bullet leaves the declared-spelling count as well as the
finding — the same disposition the sibling valve takes, so a reader of the clean line is never
looking at a count that silently shrank.

**The work-class tag is outside this grammar**, exactly as it is outside the delta-ID grammar and for
the reason stated there: its owner is the authoring-stage template and its reader is the lead at
batch-cut.

### (2) `check-amendment-retired-spelling` — a born-native canon-kit gate

A new gate, Rust module plus `.gate` descriptor in `canon-kit/checks/`, registered in
`scripts/gates.list` {design-bearing}.

**Born native** with no exception argued, per CLAUDE.md's standing rule; criterion 4
(`gate-sdk/SPEC.md` §The port-candidate criteria) clears for the same reason it clears for its
sibling — the corpus is `spec_amendments`, which reaches no gate declaration path.

Three arms:

- **A — the grammar.** Red (exit 1) when an amendment carrying `## What changes` has no
  `## Retired spellings` section, when that section's body is neither the negative form nor at least
  one well-formed positive bullet, when the negative form carries an empty reason, or when a positive
  bullet carries no backticked spelling or no citation. This is the arm that closes the omission hole
  §Why a declaration turns on, and it is the arm B and C depend on.
- **B — the survivor reconciliation.** For each declared spelling, scan the reconciliation corpus and
  red on every occurrence at a path no `## Existing sections updated` bullet names. A roster bullet
  names a path by its **leading backticked token** — measured rather than assumed: sixteen roster
  bullets across the four amendments live at authoring time, sixteen leading backticked paths, no
  bullet opening with a bare section mark, a bare path or prose (`.workflow/survey-record.md`,
  2026-09-08, with its witness). The convention is written down nowhere else, so this delta is also
  where it becomes contract rather than habit. The amendment file itself is never its own violation.
- **C — the dangling citation.** Red when a bullet cites an `<N>` no `### (<N>)` heading defines,
  `all deltas` in an amendment defining none included. Without C, A and B both pass on a block whose
  bullets cite deltas that were renumbered out from under them — the same failure arm C of
  §check-amendment-update-target closes, reached through the second block.

**The reconciliation corpus is `git ls-files` minus the amendment set, minus the configured
exclusion.** Tracked files only, so an untracked scratch file is not a violation and a green run
before staging is vacuous for exactly the file it most needs to see — the property
`gate-sdk/SPEC.md` §Enforcement tiers states of the whole battery, restated nowhere and inherited
here.

**Fail-closed (exit 2):** a scan root that is not a directory; an **unwalkable** scan root; an
amendment carrying `## Retired spellings` but no `## What changes`, where no bullet *can* be owned
and no arm could say which to blame; a file the reader cannot read; and a failure to enumerate the
reconciliation corpus. The unwalkable-root posture follows §check-amendment-update-target and
**not** §check-amendment-queue: an empty amendment set here hides every violation silently, where
the queue gate can afford one because its other direction contradicts it. That asymmetry is already
owned at §lib/spec.sh and is cited rather than restated.

**Output.** On clean, the amendments scanned, the spellings declared, and how many amendments took
the negative form — a count on the clean line and not only on the red one, so the block's uptake is
readable without a failure. On red, each finding as `<amendment>:<line>: <spelling> survives at
<path>:<line>, named by no roster bullet`.

**Fixture pair** at `canon-kit/gate-tests/check-amendment-retired-spelling/{good,bad}/`, per the
shipped-gate contract. `bad/` carries one violation per arm — a missing section, a positive bullet
with no citation, an unreconciled survivor and a dangling citation — so each arm has an executable
statement; `good/` exercises the negative form, a wrapped citation, the valve, and a survivor at a
path the roster does name.

**The backfill is the gate's own first run, and it includes this file — but only because this
consumer configures it so.** Probed to two levels rather than one, because the first answer was
misleading. An amendment written to `canon-kit/` **is** scanned here, which `check-amendment-queue`
demonstrated by reding on this very file before its queue entry existed. The reason is **not** that
the finder spares a kit directory: both substrates prune a kit root that is a strict descendant of
the scan root (`canon-kit/lib/spec.sh`'s `_spec_prune_kit_roots`, `native/src/spec.rs`'s
`prune_kit_roots`), and `canon-kit` is one of this repo's derived kit roots. It is scanned because
`scripts/canon-config.sh` sets `CANON_KIT_SCAN_KIT_ROOTS=1` — the dogfooding knob, since this repo's
kits are its own governed content rather than a dependency's.

**That is a consumer-config fact and it is stated here so a kit reader does not generalize it.** A
consumer leaving that knob at its default and vendoring canon-kit would place an amendment in a kit
directory and have it silently fall out of the corpus — no gate, no pairing, no verdict. Nothing in
this unit changes that, and the amendment's own placement relies on the knob rather than on a
guarantee.

So arm A reds on every amendment live at the landing commit, this one included if it is still on disk
then. It is therefore authored below carrying the block it specifies — self-exemplifying rather than
exempt, which also gives the build session a worked instance of the grammar to copy.

### (3) The exclusion knob, and why its kit default is empty

`CANON_KIT_RETIRED_SPELLING_EXCLUDE` — a tab-separated glob list of paths held out of the
reconciliation corpus, defaulting to **the empty set** in the kit and configured by this repo
{design-bearing}.

Which surfaces are history-bearing is a **consumer** fact, not a kit one: a consumer's queue file,
its ruling record and its scratch directory are named by that consumer's own configuration, and a
retired spelling survives in all three legitimately — that is what a history surface is for. A kit
default naming them would be a kit literal carrying this project's layout, which CLAUDE.md
§The provenance seam refuses; the knob is the sanctioned form, the same shape
`check-graph`/`scripts/graph-vocab.sh` takes.

**An empty default is the conservative one, not a fail-open.** The exclusion can only ever remove
findings, so a consumer that configures nothing gets a noisier gate rather than a blinder one. This
is worth stating because the reflex on reading "defaults to empty" is to look for the hole.

This repo's value holds out `TASK-QUEUE.md`, `TRAJECTORY.md`, `.workflow/` and every kit's
`gate-tests/` tree. It is carried where every other consumer knob value is, on the registry line and
in the generated hook's environment — no new mechanism.

### (4) The `# graph:` manifest and the generated projections

The descriptor declares its couples — the amendment glob, its own module and the shared spec reader —
at `precommit` tier, and every projection a new gate stales is regenerated in the same unit
{mechanical}.

The generated pre-commit hook is never hand-edited; the full fan-out a new gate stales is rostered in
`docs/site-architecture.md` §Generated projections and their freshness gates, and each freshness gate
prints its own command on red. This delta is the roster's work, not a judgement call.

### (5) The Definition-of-Done checkbox stops restating the grep and cites the block

`canon-kit/templates/SPEC-amendment.md`'s **Removals propagated** checkbox is rewritten to point at
the `## Retired spellings` block and its gate rather than to restate a grep it cannot check
{design-bearing}.

Two things are wrong with the checkbox as it stands and this delta fixes both. It restates an
obligation the amendment can now record, which is the content-tiering defect — one content tier per
surface, point rather than restate. And its stated corpus, "every spec", is **narrower than where the
attested misses landed**: the misses are on descriptor lines, shell comments, compiled help strings
and workflow YAML, none of which is a spec. So the checkbox as written could be honestly discharged
by an author who then shipped the miss. The rewrite widens the corpus by citing the gate's rather
than naming a new one.

### (6) The align stage's duty narrows to the residue it still holds

`lifecycle-kit/templates/stages/align.md` §"The `## Existing sections updated` roster is checked from
the tree, not from the amendment" is rewritten: its literal-substitution half is now mechanized, and
what stays human is named {design-bearing}.

The rule today instructs the align session to grep for a replaced literal and reconcile every
survivor against the roster. Arm B is that instruction, executed. Leaving the prose unchanged would
leave the tree telling a session to hand-run a check the battery now runs — the oracle-first defect,
and the reflex that produces it is exactly "the gate is new, keep the manual step for a while".

What align keeps is stated rather than left implied, because the residue is the larger half: the
misses that are neither a literal substitution nor a renumber — a stale prose sentence, a semantic
over-claim, a cross-reference dangled by a deletion rather than a substitution. Those are the
irreducibly semantic judgment `gate-sdk/SPEC.md` §When a gate earns its place leaves to a human, and
the rewritten paragraph says so and names the renumber slice's routing (§The two slices) so an align
session does not re-derive it.

## Producers and consumers

**New state — the `## Retired spellings` block and the gate's verdict (deltas 1, 2 and 3).**

- **Producer of the block:** the authoring-stage session, writing the amendment against
  `canon-kit/templates/SPEC-amendment.md` (delta 1). Its enabling configuration is the template
  itself, which every amendment is copied from and which ships in the kit — there is no configuration
  a deployment could fail to set, and no amendment can be authored without meeting it, because arm A
  reds on the section's absence.
- **Producer of the verdict:** `check-amendment-retired-spelling`, dispatched through the gate
  binary's subcommand arm, run by `gate-sdk/bin/run-gates.sh` from `scripts/gates.list` at every
  battery invocation and by the generated pre-commit hook at `precommit` tier (delta 4). Its enabling
  configuration is the registry line and the descriptor, both landing in this unit;
  `CANON_KIT_RETIRED_SPELLING_EXCLUDE` (delta 3) is optional by construction and its unset state is
  a live, correct configuration rather than a broken one.
- **Consumers:** the battery's summary and exit status; the pre-commit hook, which blocks the commit;
  the `--run-gate-tests` arm through the fixture pair; and the **align session** (delta 6), whose
  duty is now defined as the complement of what arm B covers, so it reads the gate's verdict to know
  what is left for it.

**Fields and their named readers.**

- The bullet's **spelling** is read by arm B as the search term, and by the align session as the
  record of what the amendment claims to have retired.
- The bullet's **citation** is read by arm C against the delta set, and by a build session deciding
  which delta owns a survivor it must fix.
- The negative form's **reason** is read by arm A, which reds on an empty one, and by a reviewer
  weighing whether an amendment that retires nothing really retires nothing — the transition being
  the align stage's read of the amendment against the tree.
- The valve's **reason** is read by arm A and by the same reviewer, at the same transition.
- Arm B's **survivor path and line** are read by the build session that fixes the site or extends the
  roster.
- The clean line's three **counts** are read by whoever asks whether the block is being used or being
  discharged with `None` — the transition being close's own surface read.

No field is introduced that no reader consumes.

**Existing integration prose updated.** Four surfaces describe the prior flow and are updated in this
amendment rather than left to drift: §The amendment lifecycle, which owns the amendment's grammar and
today owns only the delta-ID half of it (delta 1); §check-amendment-update-target's **Deliberately
not asserted: roster completeness** paragraph, which today hands the *whole* other half to the align
stage and after this unit hands it a strictly smaller half (delta 2); the template's DoD checkbox
(delta 5); and the align stage's roster rule (delta 6).

**No gate's corpus is narrowed by any delta**, so the causal-completeness check's point 5 does not
bind. The battery's member set grows by one and no existing member's corpus, glob or file set is
touched. The two narrowings in this unit are of **human** duties — the align rule (delta 6) and the
DoD checkbox (delta 5) — and neither is a reader with a red condition. Stated explicitly because a
unit that visibly narrows two obligations invites clearing readers by inspection, and here the
inspection has nothing to clear.

## Existing sections updated

- `canon-kit/SPEC.md` §The amendment lifecycle — the grammar section, which today pins the delta-ID
  and citation grammar and says nothing about a retired-spelling block; the citation grammar is cited
  from the new block rather than copied (delta 1).
- `canon-kit/SPEC.md` §check-amendment-update-target, its **Deliberately not asserted: roster
  completeness** paragraph — it hands the undecidable half to the align stage, and that half is
  smaller after this unit; the paragraph names the new gate and keeps its own refusal of the two
  stronger arms unchanged (delta 2).
- `canon-kit/SPEC.md` §Layout and configuration — the knob roster, which gains
  `CANON_KIT_RETIRED_SPELLING_EXCLUDE` with its empty default and the seam ground for it (delta 3).
- `canon-kit/templates/SPEC-amendment.md` — the shipped skeleton, which gains the section and its
  authoring comment (delta 1) and whose **Removals propagated** checkbox is rewritten (delta 5).
- `lifecycle-kit/templates/stages/align.md` — the roster rule, whose literal-substitution half is
  mechanized and whose residue is named (delta 6).
- `scripts/gates.list` — the registry the battery resolves members from, and the carrier of this
  repo's exclusion value (deltas 2 and 3).
- `docs/site-architecture.md` §Generated projections and their freshness gates — the generated-hook
  fan-out a new gate stales, and the docs mirror of every kit SPEC this unit edits (delta 4).

<!-- update-target-exempt: every amendment live at the gate's landing commit gains the mandatory section as a backfill that landing forces, not as a delta — arm A reds on each of them the moment the gate registers, so the backfill is the gate's first run rather than a separate claim, and citing a delta for it would put this unit's authorship on documents other units own -->
- Every amendment live at the gate's landing commit — backfilled, not authored, by this unit.

<!-- update-target-exempt: guard-kit/SPEC.md is deliberately untouched — the renumber slice's durable fix is routed to guard-rule-number-not-citable-outside-kit at §The two slices and no delta here writes that surface -->
- `guard-kit/SPEC.md` — routed away, not written.

## Retired spellings

<!-- This amendment carries the block it specifies (delta 2, "the backfill is the gate's own first
     run"). It is a worked instance of delta 1's grammar as well as a declaration. -->

- None — no delta of this amendment retires a spelling; every delta adds a section, a gate, a knob
  or a paragraph, and delta 5's checkbox rewrite and delta 6's rule rewrite both replace prose
  wholesale rather than substituting a literal that could survive elsewhere.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
</content>
