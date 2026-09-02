# SPEC amendment: declaration-cut

The port disposition of **gate-sdk's one member behind §lib/declaration.sh** —
`lib/declaration.sh` (59 lines) — off the shell substrate by **deletion**. This
is one of five stated-contract port cuts in
`declaration-install-and-stage-helper-cuts`, under the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE), the unit set composed at scope and ruled by
the **operator on 2026-09-03 over the lead relay**. The authority is stated
because a composition ruling recorded without one reads at the post-port triage
as more settled than it is.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 112 files scanned, 64 declared `no-port`, 0 temporarily held,
**48 owed**, `gate-sdk/lib/declaration.sh owed lines=59` among them.

**The member is not a gate** — §lib/declaration.sh says so in its own words, "A
sourced library, not a gate, so it owes no `good/`+`bad/` pair" — so no
`gates.list` row, no `.gate` descriptor and no binary-less residual roster moves.

**This cut is the one the previous iteration bought and did not cash.** The
2026-09-02 §upgrade-smoke port moved the declaration resolve in-crate and thereby
emptied this library's non-test shell caller set; §lib/declaration.sh records the
result in its own words — *unblocked and takeable*, never *done* — and
`kit-library-port-residue` repeats it verbatim. Taking it now is that purchase
being spent, not a fresh selection.

## What changes

### (1) The cut is a singleton behind one stated contract, and it takes the whole section

`gate-sdk/lib/declaration.sh` declares `gate-sdk/SPEC.md §lib/declaration.sh` in
its own line-2 `# spec:` header, and **no other tracked non-test `.sh` declares
that section** — probed against the oracle's own roster rather than inferred
{design-bearing}. So the section's owed set is one file and the cut takes all of
it. The 2026-09-03 ruling that a section is a cut's **outer bound, never its
minimum** therefore has nothing to bound here: there is no sequenced member to
leave behind, and no partial cut to consider.

A singleton is well-formed under the 2026-08-28 composer exactly as a group of
five is — the ruling's unit is the section, not a member count.

### (2) Criterion 6's road is deletion, and the twin dies with the caller set rather than being held

The compiled counterpart already exists and is live: `native/src/declaration.rs`
declares itself "the declaration grammar's compiled holder", and every member
that once shared the shell file reaches it {design-bearing}. What made the
duplication lawful was criterion 6's *unless* clause — a machine-held twin — and
§lib/declaration.sh recorded that disposition as **temporary rather than
permanent**, its stated test being whether the shell caller set empties.

**It has emptied, and the probe is the claim rather than the argument.** A grep
for sourcers of this file over every tracked `.sh` returns
`gate-tests/lib-declaration.test.sh` and `gate-tests/declaration-lib-parity.test.sh`
and nothing else; both carry the `*.test.sh` suffix the `--tree` corpus excludes
by that arm's own rule. No `.yml`, no `installer/` file, no `scripts/` consumer
and no `.claude/settings.json` grant names the path — each probed, and the
settings probe is the one the 2026-08-29 carve-out demands be counted rather than
assumed. Its answer here is **zero**.

So the road is the criterion's own strongest one, and §The port-candidate
criteria names it in as many words: **"A dead twin is deleted, not held"** —
"where a shell helper has no caller and its compiled counterpart is live and
tested, a standing parity obligation gates a duplication that removal disposes
of." The shell library is deleted; the crate holder is what remains.

**The enumeration criterion 6 demands was done.** Its clause binds "on a helper
**set**, not on the one helper an amendment happened to name". This member
sources no other library — it defines `DECL_TOKEN_RE` and its own functions and
reaches nothing else — so the set to enumerate is the file's own and it is empty
of further members. Recorded because the instruction exists to catch exactly the
omission that reading only the named helper produces.

### (3) `--declaration-parity` and its harness retire with the twin, and retiring them is the criterion-6 discharge rather than a loss of it

The standing oracle exists **because both holders exist**: §lib/declaration.sh
says the binary "carries a top-level `--declaration-parity` arm … for as long as
both exist" {design-bearing}. One holder cannot be held equal to itself, so the
flag and `gate-sdk/gate-tests/declaration-lib-parity.test.sh` come out in the
deleting commit. Keeping either would leave a lane whose comparison arm has no
second side — a harness that can only skip.

Concretely: `native/src/main.rs`'s `declaration_parity` function, its
`--declaration-parity` branch in the top-level flag layer, and its clause of the
usage string all go. **No dispatch roster moves**: the flag sits in the
hardcoded top-level layer that is deliberately absent from `--list`, which is
what keeps `check-gate-substrate-parity` assertion B's descriptor↔`--list`
equation untouched by its removal. §The non-gate arm's roster of top-level flags
loses one member and is edited here rather than left to drift.

**What the retirement must not do is drop a case.** The harness carries eleven
`compare` labels — `section-absent`, `section-tokens`, `section-explicitnone`,
`section-emptybullets`, `section-unparsed`, `section-mixed`, `section-proseonly`,
`record-clean`, `record-malformed`, `record-unterminated`, `record-empty` — and
seven coverage assertions that grep the shell side to refuse a vacuous hold. The
crate holder carries five unit tests of its own. **The build audits the eleven
against the five and lands any case the crate does not already reach as a crate
unit test, in the same commit.** This is the delta most easily executed as a pure
deletion, and executing it that way would silently trade a standing oracle for
nothing.

### (4) `gate-tests/lib-declaration.test.sh` retires too, and it retires for a different reason than its sibling

§lib/declaration.sh rules that this test "keeps its place unchanged: it is the
shell arm's own runtime lock-in and fails when only the shell is wrong, which a
comparison of the two structurally cannot" {design-bearing}. That sentence's
whole subject is the shell arm. With the shell arm deleted the test has no
subject — it is not a weaker oracle after the cut, it is an oracle of nothing —
so it is deleted rather than re-pointed at the binary, which would make it a
third, worse spelling of the crate's own unit tests.

**Its cases carry the same obligation delta (3) states**, and the build discharges
both in one audit: every behavior these 112 lines lock in either already has a
crate unit test or gains one in the deleting commit. Stated as an audit with an
output rather than a reading, because "the crate probably covers it" is the shape
this delta exists to refuse.

### (5) One conflation survives into the single holder, and the sentence that priced its repair changes

Both holders emit, on the refusal path, the tokens resolved *before* the offending
lines, so a container mixing a readable and an unreadable bullet reports the
readable one as unreadable {mechanical}. The verdict is unaffected in both and
only the diagnostic list is wrong. **The port does not repair it** — a port proves
parity and does not fix the rules it ports, and the defect is already filed.

What changes is the sentence that priced the repair: "Repairing it is a
two-holder edit in one unit or the parity test reds." After this cut it is a
one-holder edit and no parity test exists to red. That is a *cheaper* repair and a
*weaker* guard at once, and the section says both rather than only the first.

### (6) §lib/declaration.sh is restated for a single holder, and its heading is not renamed

The section becomes the compiled holder's contract {design-bearing}: the two
container arms and the token predicate unchanged, the trichotomy and the status
grammar unchanged, the three named callers unchanged in what they read and
changed in what they call, the caller-relations paragraph rewritten from "sourced
by three callers" to the crate's call sites, and the dual-holder machinery —
*temporary rather than permanent*, *the standing oracle*, *unblocked and
takeable* — replaced by the disposition it resolved to.

**The `### lib/declaration.sh` heading survives the file it names.** Ten
citations inside `gate-sdk/SPEC.md` and one in `docs/install.md` spell
`§lib/declaration.sh`, and renaming the heading strands every one of them; the
§upgrade-smoke port took exactly this decision for exactly this reason. A section
named after a deleted file reads oddly for one sentence and correctly forever, so
the section says in its opening line that it now owns the compiled holder and
keeps the name its citations resolve against. **This is the one thing in the cut
a tidy-up would break silently.**

### (7) Every surface the deletion stales moves in the landing commit, probed rather than assumed

{mechanical}

- **`gate-sdk/SPEC.md`, four narratives whose premise is this file's survival** —
  §The declaration cohort's "the corpus that carries every branch are
  §lib/declaration.sh's" and its dual-holder paragraph, its deleted-paths caller
  roster note, and §upgrade-smoke's "the `--declaration-parity` lane already holds
  equal to the shell form". Each is false the moment the shell form goes, and
  **no gate reads any of them for truth**.
- **`docs/install.md:762`** — cites the section for the grammar the upgrade smoke
  reads. The citation survives; what it points at changes, and the sentence is
  checked rather than assumed unaffected.
- **`.claude/settings.json`** — **no edit owed, probed rather than assumed.** No
  grant names `lib/declaration.sh`, so the 2026-08-29 settings-grant carve-out is
  exercised **zero times** by this cut. That ruling demands the count be probed;
  this is the probe with the answer it did not have to have.
- **`scripts/*.gate` descriptors** — three couple `native/src/declaration.rs`
  (`check-release-bump`, `check-tightened-gates-grammar`,
  `check-tightened-gates-note-parity`). **None couples the shell path**, so no
  `# graph:` manifest is edited by the deletion; they are named here so a later
  reader does not read the silence as an oversight.
- **`gate-sdk/bin/run-gate-tests.sh`** — takes no edit: its corpus is
  `<tests-dir>/*.test.sh` by glob, so two files leaving is a corpus that shrinks
  by construction rather than a roster to maintain.

### (8) The regeneration fan-out this cut stales, and one trigger fires from a file no manifest names

{mechanical} Each is rostered with its trigger and regen command in
`docs/site-architecture.md` §Generated projections and discharged in the landing
commit:

- **The generated `pre-commit` and `commit-msg` hooks**, on **two independent
  triggers**. The baked `check-measured-claim` values carry `tree-shell-owed`,
  which this deletion moves; *and* the hooks bake `check-prose-enum`'s roster,
  which `scripts/enum-sets.sh:47` derives from `git ls-files <kit>/gate-tests`
  filtered to `*.test.sh` basenames — so **deleting two `gate-tests/*.test.sh`
  files stales the hooks from a file no manifest names either**, the same route
  that section records for a header gaining a `# no-port:` cause. Regenerate with
  `bash gate-sdk/bin/gen-pre-commit.sh --write`.
- **`docs/check-graph.html`**, asserted fresh with them by `check-graph`.
- **`docs/gate-sdk/SPEC.md`**, the generated on-site mirror, on the section
  rewrite (`check-docs-mirror-fresh`).
- **The gate binary**, `bash gate-sdk/bin/build-native.sh` — the crate changes and
  `check-gate-binary-fresh` holds its currency. The staging order matters and is
  stated: the hooks and the binary both derive through `git ls-files`, so the
  deletion is staged first and both are regenerated second.
- **`docs/footprint.md` does not move** — its measured set contains no file under
  `bin/`, `checks/`, `lib/` or `gate-tests/`, and this cut edits no injected
  block and no kit `templates/` markdown.

### (9) The host is `kit-library-port-residue`, and this cut delivers an increment rather than discharging a blocker

The host is not the standing composer entry {design-bearing}. §Porting a gate to
the binary substrate rules that "the composer entry hosts a cut that has no other
host; an entry already waiting on this cut's subject is the better host rather
than the leftover one", and `kit-library-port-residue` names
`gate-sdk/lib/declaration.sh` as the first of the four members still owed in its
own roster.

**The ground here is the stronger of the two that section distinguishes.** The
2026-09-02 hosting delivered *no* member and discharged one member's blocker; this
one delivers a member. So the entry's roster loses `declaration.sh` outright and
its `declaration.sh` paragraph — *unblocked and takeable, never delivered* — is
replaced rather than corrected. The remaining three members are untouched:
`inject.sh` still moves behind its sourcers, `test-hermetic.sh` still waits on
`hermetic-bin-suffix-pin-placement`, `toolfloor.sh` still sits behind the
installer's behind-invoke relocation.

**The entry demotes at build and the demotion arithmetic is done here rather than
met as a red.** Its deliverable is a corpus and this is one increment of it, so
the terminal move is a demotion to `## Deferred` under `[design-pending]`
(canon-kit/SPEC.md §Merging an amendment) — and a demotion, unlike a Done move,
lands the entry back inside `check-queue-entry-budget`'s per-entry cap. Measured:
the entry's extent is `TASK-QUEUE.md` lines 70–118 = **49 raw lines**, carrying
**two** `ruled:` lines and no `recurrence:` line; the discount is at most one line
per grammar, so its counted size is **48** against `QUEUE_KIT_ENTRY_LINE_CAP` =
50 — **two lines of headroom**, and this promotion's own `ruled:` line is free
only because the entry already spends its one `ruled:` discount. So the build has
**two** lines to spend on collapsing the four-line `declaration.sh` paragraph into
its delivered form; since the replacement is shorter than what it replaces, the
demotion should *gain* headroom rather than spend it, and the build asserts that
rather than assuming it.

## Producers and consumers

The amendment introduces **no new state, no new event, no new field, no new knob
and no new interface.** It **removes** one interface — the `--declaration-parity`
top-level flag — and one implementation, the shell library. Every reader of the
declaration grammar already reads the compiled holder.

- **Producer that stops existing** — `gate-sdk/lib/declaration.sh`'s three
  entry points (`decl_section_bullets`, `decl_section_tokens`,
  `decl_record_tokens`). Their enabling path was being sourced; nothing sources
  them after the two `*.test.sh` files go.
- **Producer that survives unchanged** — `native/src/declaration.rs`'s three
  public entry points, whose bound §lib/declaration.sh already fixes: the
  container arm alone, the markdown arm's verdict, and the record arm. **No entry
  point is added by this cut**, which is the check that it is a deletion rather
  than a redesign.
- **Consumers, five, none of them created or removed here** — the three gate
  modules of §The declaration cohort (`check-tightened-gates-grammar`,
  `check-tightened-gates-note-parity`, `check-release-bump`), §upgrade-smoke's
  arm, and — until this cut — the parity arm. The fifth leaves the roster with the
  flag; the other four read the same holder at the same transitions and see no
  change at all.
- **Consumer of the retirement** — `gate-sdk/bin/run-gate-tests.sh`, at every
  battery run and every validate stage, over a `*.test.sh` glob that shrinks by
  two. It reads the corpus, not a roster, so the retirement needs no registration
  edit and produces no dangling name.
- **Consumer of the moved count** — `check-measured-claim`'s oracle, transitively
  through the `tree-shell-owed` key and the resolved values the generated hooks
  bake, which is why delta (8)'s regeneration is an update target and not
  housekeeping.

**One corpus is narrowed — the tracked non-test `*.sh` tree loses one file, and
the kit `gate-tests/` corpus loses two — so each reader's RED CONDITION is
enumerated rather than its subject** (canon-kit/SPEC.md §The causal-completeness
check, point 5). The 2026-08-30 attestation is the reason: pruning the file that
held a declaration's sole instance flipped `check-install-claim` green to red,
because its red condition is a zero count.

- **`check-prose-enum`** — reds when a governed prose enumeration disagrees with
  its derived set. **Not monotone**: the derived set includes the
  `gate-tests/*.test.sh` basename roster, so deleting two files *changes* the set
  and any prose enumerating it goes red. Cleared by delta (8)'s regeneration plus
  a check of the enumerating prose, never by inspection.
- **`check-graph`** — reds when a committed generated hook or the graph artifact
  differs byte-for-byte from its generator's output. **Not monotone**: the
  deletion moves both the baked `tree-shell-owed` value and the baked
  `check-prose-enum` roster. Cleared by delta (8).
- **`check-gate-binary-fresh`** — reds when the committed binary's source stamp
  disagrees with the crate's. **Not monotone**: the crate changes. Cleared by the
  rebuild in delta (8).
- **`check-docs-mirror-fresh`** — reds on a `docs/` mirror not byte-equal to its
  kit source. **Not monotone** on the section rewrite. Cleared by delta (8).
- **`check-md-refs`** — reds on an internal markdown link resolving to no tracked
  file or heading slug. **Clear on the deletion** (nothing links the script path)
  and **red if the heading is renamed**, which delta (6) forbids.
- **`check-spec-pointer`** — reds on a `# spec:` header naming a section that does
  not exist. **Clear**: the two deleted `*.test.sh` files take their own pointers
  with them, and delta (6) keeps the section they name alive.
- **`check-measured-claim`** — reds when a `measured:` marker disagrees with its
  oracle or names a key the oracle does not emit. `tree-shell-owed` moves; whether
  a tracked marker binds it is **checked by scanning the markers** in the landing
  commit rather than assumed either way.
- **`check-settings-paths`** — reds on a literal repo-relative `.sh` grant that
  does not resolve. **Not monotone in general — this is the reverse-trigger gate a
  port is the subject of — but clear here**, probed in delta (7): no grant names
  either deleted path. The trap is stated with it: the generated hook matches
  staged `ACMR` paths, so a *deleted* `.sh` never fires the trigger and only a
  whole-tree battery run would have caught a stranded grant.
- **`check-test-hermetic`** — reds on a `gate-tests/*.test.sh` that does not
  source the hermeticity bootstrap first. **Monotone**: two files leaving can only
  remove findings. Cleared by inspection.
- **`check-gate-substrate-parity`** — assertion B equates the `.gate` descriptor
  set with the `--list` roster; assertion G reads declared dispositions.
  **Monotone here**: no `.gate` moves, no disposition field is added or removed,
  and `--declaration-parity` was never in `--list`. Cleared by inspection.
- **`check-shellcheck`, `check-exec-bit`, `check-comment-tier`,
  `check-path-dialect`, `check-tree-terms`, `check-gate-exemption-tasks`,
  `check-evidence-baseline`** — each verdict is **monotone** in the narrowing:
  removing three `.sh` files can only remove findings, the shellcheck corpus is
  nowhere near empty, no `.gate` roster moves, no file declares a temporary
  disposition, and no validate suite's name changes. Cleared by inspection.
- **On the addition side**, `check-comment-tier` and `check-path-dialect` reach
  every crate line the audit of deltas (3) and (4) adds, from the commit it lands
  in.

**Cross-component signal: this amendment's component set is one** — gate-sdk. Its
sibling amendments this iteration reach lifecycle-kit and doctrine-kit, so
`check-stage-entry` assertion C fires on the amendment-files-span-two-components
arm across the iteration's set and the **align stamp is demanded at the build
stage's entry**. Stated here so the build session is not the one that learns it.

## Existing sections updated

- `gate-sdk/SPEC.md §lib/declaration.sh` — restated as the compiled holder's
  contract: the arms, the trichotomy and the status grammar unchanged; the
  caller-relations paragraph rewritten from three shell sourcers to the crate's
  call sites; the *owed to the port* / *temporary rather than permanent* /
  *unblocked and takeable* / *the library is dual* / *the standing oracle*
  machinery replaced by the disposition it resolved to; the surviving conflation
  re-priced. The `### lib/declaration.sh` **heading is unchanged** (deltas 1, 2,
  3, 4, 5 and 6).
- `gate-sdk/SPEC.md §The declaration cohort` — the paragraphs whose premise is the
  shell form's survival: the branch-carrying corpus attribution, the dual-holder
  paragraph, and the deleted-paths caller-roster note. Ungated prose, so an
  explicit target (deltas 2 and 7).
- `gate-sdk/SPEC.md §upgrade-smoke` — the sentence resting on "the
  `--declaration-parity` lane already holds equal to the shell form", whose lane
  this cut retires (deltas 3 and 7).
- `gate-sdk/SPEC.md §The non-gate arm` — the roster of hardcoded top-level flags
  deliberately absent from `--list` loses `--declaration-parity`, and the sentence
  naming the parity harnesses that call them loses one member (delta 3).
- `gate-sdk/SPEC.md §The port-candidate criteria` — no ruling changes; named
  because criterion 6's *a dead twin is deleted, not held* gains its worked
  instance on a **documented** surface, where its stated bound to this point was
  undocumented surface (queue-kit's done-slug helper). The bound is not widened —
  this library's twin is disposed of by the caller set emptying, which is the
  clause's own test — and saying so is what keeps a later reader from reading the
  bound as gone (delta 2).
- `gate-sdk/SPEC.md §Porting a gate to the binary substrate` — no ruling changes;
  named because delta (9) reads its host rule and this is the first cut hosting on
  the *delivers an increment* ground the section distinguishes (delta 9).
- `docs/install.md` — the sentence citing `§lib/declaration.sh` for the grammar
  the upgrade smoke reads, checked against the restated section (delta 7).
- `docs/site-architecture.md` — no ruling changes; named because delta (8)'s
  fan-out is read off it, including the `gate-tests/*.test.sh` → `enum-sets.sh` →
  baked-hook route (delta 8).
- `TASK-QUEUE.md`, the `kit-library-port-residue` entry — promoted into
  `## New Features` with `[design-pending]` swapped for this amendment's
  `[spec:]` ref, carrying the pairing's own `ruled:` line in the same commit as
  the ruling's content; at build its member roster drops `declaration.sh` and its
  `declaration.sh` paragraph is replaced by the delivery. It **demotes** at build
  rather than reaching `## Done`, inside two lines of headroom (deltas 1 and 9).

<!-- update-target-exempt: the composer entry takes no write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's section -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), the none-remain half discharged at the
      **iteration** rather than at the commit, this iteration carrying sibling
      amendments.
- [ ] **Removals propagated** — grepped every spec, README, settings file, gate
      descriptor and doc for the deleted path, for the two deleted test
      basenames, and for `--declaration-parity`; nothing dangles, the four
      **ungated** prose claims delta (7) names included.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved by the roster, not by the trailer** — the `--tree` arm
      lists no `gate-sdk/lib/declaration.sh` row, taken as a per-file roster diff
      and not as a trailer delta.
- [ ] **The caller set was empty before the delete, proved by the probe** — a
      grep for sourcers over every tracked `.sh` returned the two `*.test.sh`
      files and nothing else, and `.claude/settings.json` named neither path.
- [ ] **No case was traded for a deletion** — the eleven parity-harness labels and
      the shell lock-in's cases are audited against the crate holder's unit tests,
      and every case the crate did not already reach lands as a crate unit test in
      the same commit. The audit's output is recorded, not the reading.
- [ ] **The heading survives** — `### lib/declaration.sh` is not renamed, and the
      ten in-SPEC `§lib/declaration.sh` citations plus `docs/install.md`'s still
      resolve.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the two
      generated hooks (on both triggers), the graph artifact, the `docs/gate-sdk/`
      mirror and the gate binary, staged before regenerated.
- [ ] **The host entry demotes inside its cap** — `kit-library-port-residue`
      returns to `## Deferred` under `[design-pending]` with `declaration.sh`
      delivered out of its member roster, and its counted size is at or under
      `QUEUE_KIT_ENTRY_LINE_CAP` **in the demoting commit**.
