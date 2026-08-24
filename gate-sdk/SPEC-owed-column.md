# SPEC amendment: owed-column-disposition

Three tracked shell files that `--tree` reports as **owed** are ruled, declared
and subtracted, so the completion predicate stops counting a ruled file as
unexamined work. Two of them are the port's own instrument —
`gate-sdk/bin/port-blockers.sh` and this repo's `scripts/measured-claims.sh` —
and the third, `gate-sdk/bin/gen-pre-commit.sh`, carries a closed non-port ruling
nothing reads. Their dispositions are **not the same disposition**, and the
premise that they were is the first thing this amendment breaks.

**The defect, re-measured at this stage rather than cited.**
`bash gate-sdk/bin/port-blockers.sh --tree` reads **153 file(s) scanned, 0
declared no-port, 0 temporarily held, 153 owed**. §port-blockers rules that owed
reaching zero *is* TRAJECTORY.md's completion sentence made decidable, and rules
`owed` the default-by-absence so an undeclared file is over-counted as work
rather than lost. With **zero** declarations tree-wide the column is doing only
half its job: it counts correctly and it **discriminates nothing**. A file whose
port is refused by a ratified structural ruling, a file held behind named work,
and a file nobody has ever looked at all read the same row. That is the misread
the arm was built to prevent, arriving through the arm's own empty declaration
set.

**What this unit is and is not.** It rules **three** dispositions and declares
three files. It does **not** rule the remaining 150, and it mints no roster over
them: §port-blockers' own refusal of a maintained list applies to a list of
rulings exactly as it applies to a list of files, and an amendment ruling 150
dispositions would be that list wearing a design document's clothes. It ports
nothing. It mints no non-gate arm name — delta 5 rules the *route* and refuses
the name, which is the closed-roster rule (§The non-gate arm) held rather than
bent.

**Why the two instrument files part company, and why that is the design rather
than an asymmetry to apologise for.** The queue entry asks one question about
"the tool that measures the completion predicate", and the two files it names are
different kinds of thing.

- `gate-sdk/bin/port-blockers.sh` is **kit mechanism**: it sits in a kit root,
  rides the installer payload with that root, and every adopter runs it. The
  PRIORITY DIRECTIVE's *everything portable ports* reaches it with nothing to
  excuse it, and §The non-gate arm already fixes its family — it needs the gates
  dir, the kit roots, the prune set and the program floor, so it is a
  **bridged-arm table member** or it is a tool that silently ignores every
  consumer override. Its disposition is therefore **held**, not refused.
- `scripts/measured-claims.sh` is **not kit mechanism at all**. It is the *value*
  of a consumer knob — `CANON_KIT_MEASURED_CLAIMS_CMD` — and this tree has
  already ruled its content consumer-owned on provenance-seam grounds, in the
  knob's own comment at `scripts/canon-config.sh`: the keys "and the facts behind
  them are consumer config for the same provenance-seam reason the transport and
  payload vocabularies are". Measured rather than argued: `scripts/` is **not in
  the payload** — `scripts/pack-installer.sh` assembles the payload from
  `gate_kit_roots_rel` and nothing else — so this file reaches no adopter today,
  and porting it into `native/` would put this project's claim vocabulary into
  **every** adopter's binary, which is the seam crossed in the one direction
  CLAUDE.md §The provenance seam (never cross it) names.

**The precedent that looks like it settles the second bullet the other way, and
what it actually covers.** TRAJECTORY.md §PRIORITY DIRECTIVE's 2026-08-14 ruling
puts "the gates this repo declares under `scripts/`" into `native/`, accepting
exactly the cost above — an adopter's binary carrying another project's repo
rules. That ruling is not reversed, narrowed or reread here: its subject is
**gates**, which had no other destination, a gate being a thing the registry must
dispatch by name. A knob's *value* has a destination that is not the binary — the
knob — so the argument that forced the gates does not reach this file, and
nothing about this declaration touches the gates the ruling moved.

**The generalisation is deliberately not taken, and it is filed rather than
skipped.** The cause stated in delta 1 is true of a class — a consumer's
config-and-emitter scripts under its own gates directory — and this amendment
declares exactly **one** file under it. Ruling the class would dispose of roughly
two dozen rows of a predicate this unit does not own, and it is close enough to
the 2026-08-14 ruling's edge to be an operator question rather than an authoring
one. Filed to the gap inbox at this stage; the cause is written as a rule so the
class question has something to be asked *about*, never as a licence to sweep.

## What changes

### (1) `scripts/measured-claims.sh` declares `# no-port:`

The consumer's measured-claim emitter carries a permanent no-port cause in its
header block, and `--tree` reports it `no-port` instead of `owed`. **Mechanical.**

The declaration is one header line beside the file's existing `# spec:` line.
Its cause states the two grounds above as one sentence — that the file is the
value of `CANON_KIT_MEASURED_CLAIMS_CMD` rather than kit mechanism, that
`scripts/` rides no payload, and that its keys are this repo's vocabulary the
provenance seam keeps out of a kit artifact. §The `# graph:` manifest's payload
rule binds: the cause is free text and must be non-empty, which is also what
makes the line a **directive** rather than the restatement
`check-comment-tier` would delete.

**No second field, and no `# port-until:` alternative.** A held port names live
work owed; there is no port owed here and minting an entry for one would put a
seam crossing into the pickable set. The mutual exclusion §The `# graph:`
manifest states is therefore satisfied trivially, and a file carrying both would
read as `owed` anyway.

### (2) `gate-sdk/bin/gen-pre-commit.sh` declares `# no-port:`

The hook generator's closed non-port ruling becomes readable by the arm that
counts it, citing criterion 6's single-producer rule. **Mechanical.**

The ruling already exists and is not in doubt: §gen-pre-commit rules that this
generator does not port, that the cause is **structural rather than a sizing
judgment** — the hook bakes resolved argv, resolving a knob means sourcing the
owning kit's shell library, and §lib/gate.sh rules exactly one place a knob's
value is computed — and that the refusal is **ratified by the operator,
2026-08-21**. The cause the header carries is that sentence, not a new argument.

**This is the first tracked file in this tree to sit squarely in the class the
`--tree` arm's substitution names**, and landing it is what turns that
substitution from a worked example into a live one: §port-blockers rules "the
bootstrap is permanently shell by a closed ruling, so it **declares**; the
battery runner is simply **owed** until its port lands", and until now nothing in
the tree had ever taken the first branch.

### (3) `gate-sdk/bin/port-blockers.sh` declares `# port-until:`, naming its own entry

The oracle declares itself **held** behind the live entry that owes its port, so
its row reads as named work rather than as unexamined work. **Mechanical.**

The slug is `port-oracle-instrument-self-disposition`, the entry this amendment
promotes, which sits on a bullet lead line in an active section and therefore
resolves under §check-gate-exemption-tasks' live-slug map. The entry's terminal
move at build is a **demotion, not a Done move** (canon-kit/SPEC.md §Merging an
amendment, the entry-outlives-the-amendment branch): the deliverable is the
port, this amendment delivers the disposition increment, and the deferred section
is where the entry waits for the next one. A Done move would strand the
declaration at a dead slug and red `check-gate-exemption-tasks` — which is the
gate doing its job, and the reason the terminal move is stated here rather than
left to the build session to infer.

**The reflexivity the entry asks to be priced is real and is not a problem.**
When the port lands, the arm's implementation and the arm's own row leave the
tree in the same commit — the measurement is not a fixed point and its last act
is to remove itself from its own corpus. §port-blockers already records that the
arm reads a corpus it is inside and that nothing about the derivation is
self-referential, because the arm reads **headers, not behaviour**. A header the
tool reads about itself is the same kind of fact as a header it reads about any
other file.

### (4) A `# no-port:` cause asserts permanence **under the ruling that stands**

§port-blockers gains the field's honest reading: a declared cause is a statement
about the disposition in force, never an oracle about future rulings, and a
*declined-not-refuted* option elsewhere does not bar the declaration.
**Design-bearing.**

This is the objection the debt entry was filed with, and it has to be answered in
the SPEC rather than in a commit message, because the next declarer will hit it
in exactly the same shape. The objection: `# no-port:` asserts PERMANENT, and
§gen-pre-commit records two options — moving `--emit` into the binary, and
reopening the 2026-08-21 ratification — as **declined for now rather than
refuted**, so a permanent declaration reads as overstating a question that is
open on its merits.

**It does not, and the reason generalises.** Every closed ruling in this tree is
reopenable by the operator; that is what TRAJECTORY.md's escalate-rather-than-
reverse rule is *for*. If an open reopening-path defeated a `# no-port:`, the
field would be undeclarable on any file whatsoever, and a field with no
satisfiable declaration is a field that does not exist. So the reading is fixed
here: **`# no-port:` declares the disposition in force under the ruling that
stands**, and the day a ruling is reopened and goes the other way, the
declaration is edited by the unit that reopened it — the same way every other
consequence of a reversed ruling is handled.

**What that costs, stated rather than absorbed.** The field is now *weaker* than
its name suggests to a first reader, so the cause's job grows: a cause that names
the ruling it rests on lets a later reader find out whether that ruling still
stands, and a cause that names none leaves them nothing to check. Both causes
this amendment writes name their ruling.

**Option (c) is refused with it**, and naming the refusal is what keeps it from
being re-proposed: accepting `owed` as correct "on the ground that a file with
any open port option is owed" makes the owed column mean *no ruling has been
taken here **and** no ruling could be reopened*, which is unreachable for every
file in the corpus and which would retire the completion predicate rather than
report it. Option (b) — minting a queue entry for the emit-arm path and declaring
`# port-until:` against it — is refused on the entry's own ground, unchanged:
minting an entry for an operator-declined option puts that option into the
pickable set, which is a scope act taken by the wrong stage.

### (5) The oracle's port route is ruled; its arm name is deliberately not minted

§port-blockers records that `port-blockers.sh` ports as a **bridged-arm table
member** of the non-gate arm class, and records that the flag spelling is not
minted until the port lands. **Design-bearing.**

**The family is forced, not chosen.** §The non-gate arm rules that "the family
choice is forced for any tool that needs configuration at all": only a
bridged-arm table member publishes a knob roster a front-end resolves, while a
hardcoded top-level flag "resolves platform defaults and silently ignores every
consumer override — which is not a calibration between two workable shapes but
the difference between working and appearing to". This tool reads the gates dir,
the kit roots, `GATE_PRUNE_DIRS` / `GATE_SDK_PRUNE_EXTRA_DIRS` and
`GATE_SDK_PROGRAM_FLOOR`. So the route follows from a rule already on the books;
recording it is de-risking the port, not deciding it.

**Most of what the port needs already exists in the crate, which is the second
half of why the route is recordable now — verified by reading the crate rather
than assumed from the SPEC.** `registry.rs` owns the `gates.list` member
grammar, `gate_resolve`'s declaration path, the `# graph:` field read and the
`couples=`/`trigger=` kit expansion — the registry walk both registry arms run
on; `walk.rs` owns the prune resolution the `--tree` corpus needs **and the
corpus enumeration itself** — `walk::tracked_shell_tree()` is already, by its
own header, "the compiled face of the `--tree` corpus rule: tracked `*.sh`,
minus the `*.test.sh` suffix, minus `path_pruned`," and it is not a stub built
for this unit: `check-gate-exemption-tasks` already calls it for its own
tree-scoped arm. What the port owes beyond them is narrower than a first read
of the two files suggests: the header-block read — today a `header_block`
helper private to `gate_exemption_tasks.rs`, doing the identical
shebang/comment/blank-line read this arm needs, so the porting unit's choice is
promoting it to a shared home or duplicating four lines, not writing it fresh —
and the three trailers, which exist nowhere yet.

**And the name is refused until then.** §The non-gate arm rules a member's caller
is owed, and the closed-roster rule that a key or a field is "minted **with** its
reader or not at all" applies to a flag spelling identically: a spelling written
into a SPEC ahead of an implementation is a reservation. The porting unit mints
it, with its caller — which is already known and is named here as a constraint on
that unit rather than as an interface: `scripts/measured-claims.sh` reads the
`--tree` trailer today by spawning `bash`, and it will read the arm instead, so
the port that deletes the shell tool **must** land the emitter's call-site change
in the same commit or the consumer's claim oracle exits 2.

### (6) The predicate moves to 150 owed, and the generated hook moves with it

The three declarations change `--tree`'s trailer, and the trailer's owed count is
baked verbatim into a byte-gated artifact, so the regeneration is part of the
unit. **Mechanical.**

After deltas 1–3 the trailer reads **153 file(s) scanned, 2 declared no-port, 1
temporarily held, 150 owed**, and the build re-runs the arm rather than asserting
those numbers.

**The non-obvious half, and the one that reds the commit.**
`scripts/measured-claims.sh` emits the trailer's owed count as the
`tree-shell-owed` key; `scripts/git-hooks/pre-commit` bakes
`GATE_SDK_KNOB_CANON_KIT_MEASURED_VALUES` **verbatim**, so the value sits
literally in the committed hook and every one of these declarations stales it.
The build stages first and regenerates second — §gen-pre-commit's own ordering
rule, because the derivation reads `git ls-files` rather than the worktree — and
`check-graph`'s byte-freshness assertion is the reader that reds until it does.
The irony is worth one sentence in the SPEC and no more: the declaration that
`gen-pre-commit.sh` does not port stales the hook `gen-pre-commit.sh` generates.

**`tree-shell-owed` is bound by no `<!-- measured: -->` marker anywhere in the
tree** — verified by grep over the whole tree at this stage, not assumed — so
`check-measured-claim` arm A has nothing to compare and no prose sentence changes
meaning under the move. That is the *reason* the value can move in this unit at
all, and it is recorded because the next value move may not be so lucky.

**The docs mirror moves too**, gate-sdk/SPEC.md being a mirrored surface
(docs/site-architecture.md §Generated projections and their freshness gates).
The regen commands are the gates' own on red; the fan-out is named here so the
build knows it before the battery tells it.

## Producers and consumers

**`# no-port:` on `scripts/measured-claims.sh` and on
`gate-sdk/bin/gen-pre-commit.sh` (new state, deltas 1 and 2).**
*Producer:* this amendment's build session, writing one header line into each
file's leading run of shebang, comment and blank lines. There is no registration
step and no enabling config — §port-blockers' widened field reaches any tracked
script by nothing but the file being tracked, which is exactly why it can reach a
corpus that owns no descriptor.
*Consumers, two, each at a named transition:* the `--tree` arm, at the row's
disposition column and at the trailer's `declared no-port` counter; and
`check-comment-tier`, at the directive-class test, where `no-port:` is an
already-recognised class (canon-kit/SPEC.md §check-comment-tier) with a mandatory
non-empty payload — verified in that section's roster at this stage rather than
assumed from the widening amendment that put it there.
*Consumers it does not gain, stated because their code looks like it would
widen:* `check-gate-exemption-tasks`, whose liveness arm reads `# port-until:`
alone and has no reader for a cause; and `check-gate-substrate-parity` assertions
G and H, whose subject is the gate registry — none of these three files is a
registered member, so neither has anything to say about one.

**`# port-until: port-oracle-instrument-self-disposition` on
`gate-sdk/bin/port-blockers.sh` (new state, delta 3).**
*Producer:* the same build session, in the same header block.
*Consumers, three, each at a named transition:* the `--tree` arm, at the
disposition column and the `temporarily held` counter; `check-gate-exemption-
tasks`, at the slug-liveness assertion, resolving the slug against the queue's
New Features / Technical Debt / Deferred span on a bullet lead line; and
`check-comment-tier` at the directive-class test.
*Its enabling path is the queue entry's own liveness*, which this stage's
promotion creates and which the build's terminal **demotion** preserves. That is
the field's only enabling configuration and it is the one a Done move would
destroy.

**The moved trailer counts and the `tree-shell-owed` value (changed state,
delta 6).**
*Producer:* the `--tree` arm's walk, recomputed at every invocation;
`scripts/measured-claims.sh` re-emits it on every run of the emitter.
*Consumers, and every field has one:* `declared no-port` and `temporarily held`
are read by a human session asking whether the port is done, at that question;
`owed` is read by that session **and** by `scripts/measured-claims.sh` at the
emitter's own run, and through it by `check-graph`'s byte-freshness assertion at
every commit that stages a coupled path, the resolved value being baked into
`scripts/git-hooks/pre-commit`. No new field is introduced by this amendment, so
no field is minted without a reader.

**The non-gate arm's flag spelling (deliberately no producer and no consumer,
delta 5).** Recorded as an absence: nothing emits it, nothing reads it, and it
does not exist until the porting unit lands it with its caller. Its future caller
is named — `scripts/measured-claims.sh`'s trailer read — as a constraint on that
unit.

**Red conditions named, because deltas 1–3 narrow the owed set and one reader is
not monotone in it.** Point 5 of the causal-completeness check binds here in its
attested shape, so each reader's *red condition* is enumerated rather than its
subject:

- **`check-graph`** — red condition: *the committed hook differs byte-for-byte
  from `--emit`*. **Not monotone**, and this is the one that flips green to red
  under the narrowing: the hook bakes the owed count, so removing three files
  from the owed set changes a byte in a committed artifact. Cleared by
  regeneration inside the unit (delta 6), never by inspection.
- **`check-measured-claim`** — arm A's red condition is *a marker whose bound
  value differs from the oracle's*; arm B's is *a marker naming a key absent from
  the roster*. `tree-shell-owed` is named by **no marker**, so arm A has no
  subject here and arm B's roster is unchanged. This is the clearance that had to
  be bought by inspection rather than reasoned, and it was: the grep is over the
  whole tree with stderr unsilenced.
- **`check-gate-exemption-tasks`** — red condition: *a declared slug that does
  not resolve to a live queue entry*. **Monotone** in the declaration set:
  delta 3 adds the tree's first `# port-until:` declaration, so the gate acquires
  a subject it did not have. Cleared by the promotion this stage performs, and
  re-armed against the build's terminal move.
- **`check-comment-tier`** — red condition: *a full-line comment on a governed
  source that is not a recognised directive*. **Not monotone** in the obvious
  direction either, because these deltas *add* comment lines: each is a potential
  new violation unless its class is recognised and its payload non-empty. Both
  classes are already in the roster, so the clearance is by inspection of that
  roster plus a non-empty cause on every line written.
- **`port-blockers --group`** and **`check-gate-substrate-parity` G/H** — red
  conditions are over the **gate registry**: a member with a contradictory or
  empty declaration, a declaration on a `.gate` descriptor. None of the three
  files is a registered member, so all three arms are empty of subject here.
  Named because their fields are the same two, which is the whole reason a reader
  would assume they widen.

## Existing sections updated

- **gate-sdk/SPEC.md §port-blockers**, the `--tree` section — the field's
  permanence reading (a cause asserts the disposition in force, not an oracle
  about future rulings, else the field is undeclarable anywhere), the refusal of
  the any-open-option reading of `owed`, and the tree's first declarations named
  as the substitution's live instance rather than its worked example (deltas 4
  and 6).
- **gate-sdk/SPEC.md §port-blockers**, the reads-a-corpus-it-is-inside paragraph
  — the instrument's two files are no longer both plain rows: one is held behind
  its own entry and one is refused on a seam ground, and the reflexivity of the
  port's own last act is recorded there (deltas 3 and 5).
- **gate-sdk/SPEC.md §gen-pre-commit** — the closed non-port ruling gains its
  declaration, and the declined-not-refuted paragraph gains the sentence that the
  declaration does not close either option (deltas 2 and 4).
- **gate-sdk/SPEC.md §The non-gate arm** — `port-blockers` is recorded as a ruled
  future bridged-arm member whose spelling is unminted, and the caller constraint
  that binds its porting unit (the emitter's trailer read moves in the same
  commit) is stated there rather than left to that unit to discover (delta 5).
- **gate-sdk/SPEC.md §The `# graph:` manifest** — the payload rule gains the
  guidance the permanence reading creates: a cause names the ruling it rests on,
  so a later reader can check whether that ruling still stands (delta 4).
- **`scripts/measured-claims.sh`** — the header gains its `# no-port:` line
  (delta 1).
- **`gate-sdk/bin/gen-pre-commit.sh`** — the header gains its `# no-port:` line
  (delta 2).
- **`gate-sdk/bin/port-blockers.sh`** — the header gains its `# port-until:` line
  (delta 3).
- **`scripts/git-hooks/pre-commit`** — regenerated, the baked
  `tree-shell-owed` value moving with the trailer; never hand-edited (delta 6).
- **`docs/gate-sdk/SPEC.md`** — the on-site mirror of every gate-sdk/SPEC.md edit
  above, regenerated by its own arm (all deltas).
<!-- update-target-exempt: a no-change confirmation the ruling produces, owned by no delta -->
- **TRAJECTORY.md §PRIORITY DIRECTIVE**, the 2026-08-14 consumer-gates ruling —
  **unchanged**, and listed so the build confirms that rather than assuming it:
  the ruling's subject is the gates this repo declares under `scripts/`, delta 1
  declares a non-gate, and no gate the ruling moved is touched.
<!-- update-target-exempt: a no-change confirmation, owned by no delta by construction -->
- **canon-kit/SPEC.md §check-comment-tier** — the directive roster is
  **unchanged**: `no-port:` and `port-until:` are already members with the
  mandatory-payload test. Listed so the build reads the roster rather than
  trusting this sentence.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), the none-remain half discharged at the
      iteration rather than at the commit.
- [ ] **Terminal move taken deliberately** — `port-oracle-instrument-self-
      disposition` **demotes** to the deferred section under `[design-pending]`
      rather than moving to `## Done`, because its deliverable is the port and
      this amendment delivers the disposition increment; a Done move strands
      delta 3's declaration at a dead slug.
      `gen-pre-commit-tree-declaration-absent` is discharged in full and moves to
      `## Done`.
- [ ] **The oracle is re-run, never cited** — `--tree` reports 2 declared
      no-port, 1 temporarily held and 150 owed at the end of the unit, and the
      regeneration fan-out (the generated pre-commit hook, the on-site SPEC
      mirror) is green rather than assumed.
- [ ] **Removals propagated** — grepped every spec for names this change retired
      (any prose reading the declaration set as empty, or reading the `--tree`
      substitution as having no live instance); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks; the class question delta 1 declines to rule — whether the
      consumer-config-emitter cause disposes of `scripts/*.sh` as a class — is
      filed rather than flagged-and-skipped.
