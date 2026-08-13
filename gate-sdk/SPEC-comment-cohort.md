# SPEC amendment: comment-cohort

The seventh gate cohort of the native port: the **`spec_comment_surface`
family** — four members on one corpus primitive, `check-comment-tier`,
`check-spec-pointer`, `check-deprecation-task` and `check-todo-task-liveness`.
Ruled by the operator 2026-08-13 knowing this cohort is **design-then-port
rather than cheap**: all four fail criterion 4, and the criterion's own remedy
is a designed answer rather than a waiver (§The port-candidate criteria,
criterion 4; §The first cohort, and the rule that selects the next — *"both are
designed, then ported"*).

This amendment designs that answer. It does not restate the porting procedure
(§Porting a gate to the binary substrate), the descriptor format (§The
`# graph:` manifest), the conservation contract (§Meta-gate conservation for
the binary substrate), the criteria roster (§The port-candidate criteria), or
the directive's grounds (TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's
sequence).

**The cohort was ruled against an incomplete rival set, and that is filed rather
than re-opened here.** `port-corpus-grouping-census-unbought` records that the
whole-corpus grouping the selection rule presumes has never been bought and that
two dispatches failed at it. Its cost is stated there. This amendment neither
re-opens it nor buys it, and no grouping census rides this unit.

## Four premises, each probed at this rev rather than inherited

### (i) The census is re-derived, not cited

`.workflow/survey-record.md` carries this cohort's sizing. The record was read
and its witness run rather than its prose trusted, and the figures below are
this stage's own oracle output at the authoring rev.

`bash gate-sdk/bin/port-blockers.sh` — 103 registered members scanned, 38 with a
requirement the report cannot decide (every one a `.gate` member with no
`--needs` answer, the undecidable count the criterion's own honest bound
predicts). **Not one of the four cohort members appears in the report**, so
criterion 7 clears for the cohort against the tree rather than against the
record. `bash gate-sdk/checks/check-gate-substrate-parity.sh` — clean: 103
members with one declaration each, 36 dispatching, 36 descriptors in parity with
the 36-subcommand roster, 0 reference-only, 31 substrate-sensitive members all
dispositioned, 44 implementation sources.

The membership was then re-derived from scratch rather than read off the record.
A tree-wide grep for `spec_comment_surface` and `spec_comment_surface_with_
templates` returns **exactly four callers**, all still shell:
`check-comment-tier.sh:185` (the with-templates form),
`check-spec-pointer.sh:156`, `check-todo-task-liveness.sh:42`,
`check-deprecation-task.sh:48`. All four are registered in `scripts/gates.list`
and all four are `dir=one valve=none tier=precommit`, so criterion 3 clears
family-wide and a green `check-graph` after the port is end-to-end proof the
manifest survived the substrate change.

### (ii) Criterion 4's recorded ground is true and is not the binding hazard

The hold recorded on `cohort-held-members-port-prerequisites` states the ground
as: the primitive's file set spans `*.rs`, so it scans the crate's own gate
modules and the parity oracle becomes self-referential. That is accurate. It is
also, read alone, weaker than it sounds — and a port that answered only the
sentence would ship the real hole.

**Self-scanning is not new.** This repo sets `CANON_KIT_SCAN_KIT_ROOTS=1`, so
all four gates already scan their own `.sh` sources today, and the corpus
already contains 36 `.gate` descriptors and 44 implementation sources put there
by six prior cohorts. The port adds no new corpus *shape*: a seventh cohort's
descriptor is an ordinary descriptor under a closed field roster, and its module
is an ordinary crate module. So the fear that the port introduces an unproven
shape is unfounded, and saying so is what stops a later reader over-designing
against it.

**What is genuinely true is narrower and worse.** The comparison that proves
parity runs over a corpus the port *mutates* — four `.sh` deleted, four `.gate`
and four `.rs` added — and assertion A forbids a descriptor and a script
coexisting in one resolve dir, so the comparison necessarily happens on the
pre-descriptor tree. The corpus the two implementations were compared over is
therefore not the corpus the surviving implementation runs over, and after the
port there is no second implementation left to notice a disagreement. That is
the hazard, and premise (iii) is why it currently has no answer at all.

### (iii) The fixture oracle is blind to exactly the arms that make criterion 4 bind

Criterion 2 is the port's parity instrument: *parity between substrates is
proved by running both against the same cases, never asserted*. For this family
that instrument is **blind**.

`spec_comment_surface` spans four arms — `*.sh`, the `*.gate` descriptor, `*.rs`,
and every tracked file in the workflow directory whatever its extension
(canon-kit/lib/spec.sh, `_spec_comment_surface`). `git ls-files` over the four
members' `gate-tests/` directories returns **`.sh` and `.md` sources and nothing
else**: not one `.gate`, not one `.rs`, not one workflow-directory entry, across
all eight case dirs.

So criterion 2's oracle exercises **one of four arms**, and the three it misses
are precisely the three that make criterion 4 bind. A port taken on the fixture
pairs as written would prove the `.sh` branch and ship the other three proved by
nothing but a live-tree run over a corpus the port itself changes. This is the
same vacuity shape §The first cohort already recorded once — *"a ported member's
pair would therefore have gone green over an arm with no implementation"* — and
finding it before the port rather than after is what the operator's
design-then-port ruling bought.

### (iv) The primitive's caller set empties, which decides criterion 6

`spec_comment_surface` has four callers and this cohort is all four. Porting the
cohort therefore leaves the shell primitive with **no caller in the tree**. The
same is true of `spec_queue_slugs` (canon-kit/lib/spec.sh:155), whose only two
callers are `check-todo-task-liveness` and `check-deprecation-task`.

That is what makes criterion 6 answerable in its strongest form here, and it is
the opposite of the disposition the same criterion earns on
`queue-lib-dual-implementation-parity`, whose shell library keeps seven live
consumers. Two units, one criterion, two dispositions — recorded in both
amendments so neither reads as the general rule.

## The seam, ruled rather than assumed

The three-way split this cohort has to hold, stated once so no delta re-decides
it:

- **Kit mechanism** — the corpus derivation itself, the four gates' rules, the
  descriptors, the widened fixture arms, and the couples tokens delta (4) adds.
  Every one is generic: a token naming `native/` would publish this repo's crate
  layout into a kit file and be false for every consumer besides, which is why
  delta (4)'s additions are bare globs matching the derivation.
- **Consumer config** — every vocabulary these gates interpret, unchanged by the
  port and unchanged in kind: the directive rosters, the whitelist and surface
  globs, the count collections, the deprecation-marker vocabulary, the queue
  file's location. Each crosses the bridge as a **resolved value**, so no
  spelling of a consumer's vocabulary becomes a kit literal and no compiled
  member spawns an interpreter to read consumer config.
- **Private rule content** — **none is in play, and saying so is the ruling.**
  This cohort ports mechanism whose every input is already either kit-generic or
  consumer-supplied; it introduces no term list, coupling vocabulary or product
  constant. The one place the seam could have been crossed is delta (4), and it
  is the one place this amendment refuses a literal.

## What changes

### The parity oracle, which is what this cohort exists to buy

**(1) The parity corpus becomes the fixture pair, widened to carry every arm of
the derivation.** [design-bearing] Each of the four members' `good/` and `bad/`
case dirs gains a `.gate` descriptor source, a `.rs` source, a `templates/`
source, and a tracked workflow-directory entry, each carrying the case's subject
matter — a compliant instance in `good/`, a violating one in `bad/`. The
`bad/expect.txt` of each grows the violation the new arm plants, so a dropped
arm reds rather than passing quietly.

This is the whole design, and its load-bearing property is that the fixture
corpus is **inert under the port**. `gate-tests` is a member of
`GATE_PRUNE_DIRS` (gate-sdk/lib/gate.sh), so nothing inside a fixture dir is
reachable from the live-tree walk, nothing there is a registry member, and no
port can add or remove a file in it. A corpus that carries every arm and that
the port cannot move is a parity oracle the port does not invalidate — which is
exactly what criterion 4 says a self-referential port must design and does not
say how to build.

It buys a second thing the port does not strictly owe and should not be
separated from the first: the widened `bad/` cases are a **standing** guard.
After the port, an edit that drops the `.rs` arm from the corpus walk makes all
four gates stop checking the crate's own sources and print `clean` — a false
green with no shell auditor left to see it. A `bad/` case whose only violation
lives in a `.rs` file reds on that edit forever, not just at port time.

**(2) The live-tree comparison is demoted from proof to smoke, and the demotion
is written down where criterion 2 is stated.** [design-bearing] Prior cohorts
proved parity on the fixture pairs, the live tree and the edge roots, and
treated the three as one oracle. For a member whose assertion target is gate
source the live-tree arm cannot be a proof, for premise (ii)'s reason. It is
retained — it is cheap and it finds real disagreements — but its verdict is
recorded as *no disagreement found on the pre-descriptor tree*, never as *parity
proved*. Criterion 4's roster entry gains that distinction, so the next cohort
of gate-source auditors inherits it rather than re-deriving it.

**(3) The auditor-stays-shell principle is examined and ruled not to bind, and
the residual is answered rather than waived.** [design-bearing] §Meta-gate
conservation keeps `check-gate-substrate-parity` and `check-install-disposition`
on shell so that *the auditor never depends on the substrate it audits*. A
selector applying that sentence mechanically would hold all four of these
members too, and the operator ruling did not.

The distinction is exact rather than a judgment call. Those two adjudicate the
**declaration and dispatch relation** — whether a gate declares itself, whether
a descriptor and a subcommand agree — so a compiled form could pass itself with
a broken binary, which is a false green. These four adjudicate **comment
content** on governed sources. On a tree with no working binary they are omitted
and declared rather than silently passing (§The install disposition,
§check-gate-binary-fresh), so the failure mode is a *declared coverage loss*,
never a false green.

The one residual the distinction leaves is real and is named rather than
absorbed: a corpus-walk regression after the port silences the four gates over
the crate's own sources, and no shell auditor remains. Delta (1)'s widened
`bad/` cases are the answer, and they are the answer *because* they are fixture
cases rather than a second implementation — the consumer receives them with
every other kit file and can run them without the crate.

### The corpus's trigger, which the earlier widening left behind

**(4) All four `# graph:` manifests gain the couples their corpus already
scans.** [design-bearing] The primitive was widened to `*.gate` and `*.rs` when
the conservation contract landed; **no member's `couples=` followed it**. Read
off the four declarations at this rev:

- `check-comment-tier` — `scripts/*.sh,kit:*.sh,.workflow/*`
- `check-spec-pointer` — `*SPEC*.md,*README.md,CLAUDE.md,scripts/*.sh,kit:*.sh,.workflow/*`
- `check-todo-task-liveness` — `scripts/*.sh,kit:*.sh,.workflow/*.txt,TASK-QUEUE.md`
- `check-deprecation-task` — `scripts/*.sh,kit:*.sh,.workflow/*.txt,TASK-QUEUE.md`

The generated hook's matcher lets `*` span `/`, so `kit:*.sh` does cover a
kit's `checks/*.sh` declaration paths. **Nothing covers `*.gate` and nothing
covers a crate implementation source.** Against this tree that is 36 descriptors
and 44 implementation sources inside the corpus and outside the trigger: staging
an edit to a ported gate's descriptor or module re-runs none of the four gates
that read it. The port makes it worse in the one way that matters — the four
members' own declarations join the untriggered class.

The fix is `*.gate` and `*.rs` added to all four, and two spellings corrected
from `.workflow/*.txt` to `.workflow/*` where the corpus takes every tracked
workflow file whatever its extension. Both added tokens are **bare globs, not
consumer literals**: a `native/` token in a kit gate's manifest would publish
this repo's crate layout into a kit file and cross the provenance seam, and it
would be false for every consumer besides. The bare glob matches the corpus
derivation exactly, which is the honest declaration and the reason
over-selection is cheap here (§The port-candidate criteria, criterion 4 — the
couples set is trigger-shaped and wide on purpose).

This is design-bearing rather than mechanical because the widening moves the
members inside assertion C's derivation more deeply than they already sat, and
the verdict on that is taken by running the derivation, not by predicting it —
delta (10).

### The primitive

**(5) `spec_comment_surface` and `spec_queue_slugs` port to the crate, and the
shell forms are deleted rather than duplicated.** [design-bearing] Both land in
`native/src/spec.rs`, beside the manifest-set primitives its family already
carries, reached through `crate::walk`'s pruned walk and the bridged knobs.
Premise (iv) is why the shell forms go: their caller sets empty with this
cohort, so criterion 6 is satisfied in the form the criterion itself calls
strongest — *the duplication is not machine-held, it is absent* — the same
disposition the config bridge earned and for the same reason.

Two costs are stated rather than banked. First, `spec_comment_surface`,
`spec_comment_surface_with_templates` and `spec_queue_slugs` are **documented
kit library surface**, so their deletion removes three names a consumer's own
gate could have called; a consumer wanting them shadows the gate, which is the
answer canon-kit already gives for the finder-choice question in the same
section. Second, `spec_manifest_files`'s shell form **stays** — members outside
this family still call it — so `check-spec-pointer`'s port leaves that
pre-existing duplication exactly where the canon-kit cohort accepted it. That is
cited, not re-opened.

**(6) The canon-kit queue walk stays an independent implementation and is not
collapsed onto the crate's queue module.** [design-bearing] `spec_queue_slugs`
re-implements queue-kit's bullet lead-line predicate on purpose: canon-kit is
one of the format's independent holders under the re-implement-and-cite-from-
both-ends rule (queue-kit/SPEC.md §The queue format), and the holder census is
gate-sdk/SPEC.md §check-gate-exemption-tasks, which prices the arrangement and
explicitly rules that whether the holders earn a shared derivation is a
different unit.

Two Rust modules in one crate is still two independent holders; one shared
function is not. The tidy a porting session will reach for — point
`native/src/spec.rs` at `native/src/queue.rs`'s `live_slugs` and `done_slugs`,
which already exist and already work — **silently ends an arrangement two SPECs
price and a third forecloses re-deciding**. Named here because it is the
cheapest wrong move available at implementation time and nothing in the tree
would red on it.

That census also owes a repair, and it is this delta's: it cites
`spec_queue_slugs (canon-kit/lib/spec.sh)` by locus, and the locus moves. The
holder count is unchanged — a holder moved substrate, none was added or
removed — so the repair is the citation, never the number.

### The four members

**(7) Port the four to compiled subcommands, deleting each shell original in the
motion that lands its descriptor.** [design-bearing] Assertion A forbids a
`<name>.sh` and a `<name>.gate` coexisting in one resolve dir, and delta (1)'s
parity run is bought while both implementations still exist. Each descriptor
carries its `# graph:` manifest as delta (4) rewrites it, its `# spec:` pointer,
its `# install:` disposition unchanged, and nothing else.

`check-comment-tier` carries the volume: a classifier over full-line comment
runs in four comment styles, a directive-window model with a run cap, a
positional-construct allowance, and the shared count adapter folded in. It is
implementation volume rather than new mechanism.

**(8) `check-deprecation-task` compiles its consumer marker vocabulary through
the crate's ERE matcher; `check-comment-tier` does not and the screen-out
stands.** [mechanical] `CANON_KIT_DEPRECATION_MARKERS` is a consumer array
joined into an alternation and interpreted as a pattern, which is the shape
`native/src/ere.rs` exists for and which three ERE-cohort members already
compile. `check-comment-tier`'s standing screen-out from the ERE roster is
canonical at §The POSIX ERE matcher and is not revisited: it composes fixed
alternations from consumer *token* arrays, which is joining, not interpreting.

**(9) The two liveness gates' `declare -A` use is local, not the bridge's
associative-array hold.** [mechanical] `check-todo-task-liveness.sh:30` and
`check-deprecation-task.sh:36` each declare `IS_LIVE`/`IS_DONE` maps. Those are
**internal working maps built from the queue walk's own output**, not knobs, so
neither member is a second instance of the prerequisite `check-stage-entry` and
`check-evidence-baseline` share. Written out because the grep that finds the
bridge's real blockers finds these two as well, and a session sizing the cohort
off that grep would hold the wrong members. Every knob the four read is a scalar
or an indexed array defined by `canon-kit/lib/spec.sh`, so the bridge's
does-not-define refusal cannot fire — a claim reached by reading definitions,
and therefore re-verified against `--knobs` at build before the first descriptor
lands.

### Standing obligations

**(10) The conservation-table verdict is taken by running the derivation, not by
reading this amendment.** [mechanical] All four members already carry rows, each
written ahead of the port; delta (4) widens their couples, which can only widen
the derived set. The rows' existing text — *retained, corpus extended* — is
re-read against the post-port tree and edited only where the runtime derivation
makes it false. A row written because this amendment predicted one would be a
maintained roster answering a question assertion C never asked.

**(11) Criterion 5 is priced per member and paid per cohort, and this cohort
rules the class it empties.** [design-bearing] Three of the four —
`check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness` — are
`install: zero-config` and are therefore seeded into a freshly initialised
consumer's registry; `check-deprecation-task` is `on-surface`. So unlike the
lifecycle cohort, which grew the omitted roster by zero for exactly that reason,
this one is shaped to grow it.

The **number is measured, never reasoned**: the instrument is `installer_smoke`'s
binary-less leg, run against the post-cohort registry, after the cohort's own
commit, from a clean checkout because the packer refuses a dirty worktree. No
figure is predicted here — the ERE cohort's amendment predicted a bookkeeping
edit, was wrong, and the build correctly no-op'd it.

The **judgment** the criterion leaves to the cohort is ruled: **accept and
declare.** A consumer on an uncovered platform loses the governed comment
surface entirely, and receives that omission declared in its own `gates.list`
rather than as a broken battery. The two rivals are refused with cause —
restoring the class shell-side reinstates the exact duplication delta (5)
deletes, which enforcement-first ranks below removal; and a binary-gated
declaration is what the omit path already is. The honest limit rides with the
ruling: this is a real subtraction for an uncovered host, it lands because the
2026-08-09 directive ports the whole corpus, and it shrinks as targets are
published rather than being repaired by this unit.

**(12) `native-gate-port-remaining-corpus` is demoted, not moved to Done.**
[mechanical] It is the whole corpus; a Done move would assert a finished port
and drop it from the public roadmap projection. It returns to the deferred
section under `[design-pending]`. The entry stands at 46 lines against
`QUEUE_KIT_ENTRY_LINE_CAP` of 50, so the cohort-count edit fits, and it stays a
count edit: every cohort's composition is canonical at §The first cohort, and a
second copy in the entry would be one more to drift.

**(13) `cohort-held-members-port-prerequisites` loses two lines and gains
none.** [mechanical] `check-spec-pointer`'s criterion-6 hold is discharged by
delta (5), and the four-member criterion-4 correction the entry carries is spent
once the cohort lands. Both come out. The entry's remaining holds —
`check-tree-terms`, the associative-array bridge pair, `check-roadmap-fresh` —
are untouched, and no new hold is created by this cohort.

**(14) The bookkeeping fan-out, derived from the projection roster rather than
predicted.** [mechanical] `.workflow/tightened-gates.txt` gains one line per
newly-compiled member. `scripts/gates.list` is **not** touched: registration is
by bare name and a `.sh`→`.gate` swap does not move it. The projections that do
move are the generated pre-commit hook — delta (4) changes four manifests, so it
moves whether or not the port did — and the docs mirror, both regenerated per
docs/site-architecture.md §Generated projections rather than hand-edited.
`bash gate-sdk/bin/build-native.sh` and the full battery are both owed at every
commit and neither discharges the other.

## Producers and consumers

**The four `.gate` descriptors — producer.** Four new descriptor files under
`canon-kit/checks/`, each created by the porting commit that deletes its shell
sibling. Their consumers are the ones the closed field roster already names, and
no field is added: `# graph:` is read by `gate_expand_couples_var` and through it
by `gen-pre-commit`, `check-graph` and `run-gates --for`; `# spec:` by
`check-spec-pointer` — which is now one of the four, so for the first time a
member's own descriptor is read by a member of its own cohort. `# install:` is
read by `check-install-disposition` and by `init`'s seeding.

**The four subcommands — producer.** Four new entries in the crate's gate
registry, one per member, dispatched by the name that already identifies the
gate everywhere else. The consumer is `gate_command`, which reads the
descriptor's presence as the dispatch declaration; there is no mapping table to
update, because the one fact is the file's presence. Each entry declares its
walk roots beside itself, which unit test A holds to the roots observed while
running the member over its own fixture cases (§Meta-gate conservation,
`check-reads-couples`) — and delta (1) widens exactly those cases, so the
declared-read verification gets stronger with the parity oracle rather than
separately.

**`spec_comment_surface` and `spec_queue_slugs` in `native/src/spec.rs` —
producer, and the shell producers are removed in the same commit.** The
consumers are the four compiled members and nothing else; that is premise (iv),
and it is what licenses the removal. The knobs both read cross the bridge as
resolved values, so there is one place each is computed and the crate holds no
default to drift.

**The widened fixture arms — producer and consumer are the same harness.**
`run-gate-tests.sh` produces each case's invocation through `gate_command` and
consumes its exit code and, for `bad/`, its output against `expect.txt`. The new
arms add no field and no protocol: a `.rs` file in a case dir is an ordinary
input file to a walk that already accepts one. **The named reader of every new
arm is the case's own `expect.txt`** — an arm whose planted violation no
`expect.txt` names is an arm with no reader and is removed rather than added,
which is the field-with-no-reader rule applied to a corpus arm.

**The workflow-directory arm has one producer property worth stating**, because
it is the arm most likely to be built wrong: the primitive admits a file there
only when `git ls-files` reports it tracked. A fixture case's workflow entry is
tracked in this repository, so the arm is live in the fixture run; a case that
merely creates the file at run time would exercise nothing and read as coverage.

**No knob is created and no field is added by this cohort.** The one *removal*
is four shell scripts and three shell library functions, and its readers are
enumerated in premise (iv) and delta (5) — that enumeration is the amendment,
not a side note.

**The narrowing check, run in the direction that is not monotone.** Deleting
four scripts narrows several gates' corpora, and *a narrower corpus can only
remove violations* is false. Each affected reader's **red condition** was
enumerated rather than its subject. `check-gate-substrate-parity` assertion A
reds on *coexistence*, so a removal can only satisfy it; assertion B reds on an
implementation no descriptor dispatches to, and the descriptors land in the same
commit; assertion C reds on a derived member with **no** disposition row, a
zero-count condition that is **not** monotone under narrowing — which is exactly
why delta (10) runs the derivation instead of reasoning about it, and why delta
(4)'s widening is sequenced with it rather than after it. `check-shellcheck`
loses four files and reds only on findings, so it is monotone and clears by
inspection. `check-gate-exemption-tasks` reds on an absurdly *large* derived
slug set and on an *empty* exemption array — both zero-or-floor shaped, neither
monotone — and delta (6) touches its prose rather than its derivation, so its
verdict is taken from a run.

## Existing sections updated

- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by deltas (2) and
  (11). Criterion 4 gains the fixture-corpus discharge and the live-tree arm's
  demotion from proof to smoke, stated as the general answer for a gate-source
  auditor rather than as this cohort's local trick. Criterion 5's per-cohort
  accounting gains this cohort's measured residual and its ruled judgment.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by deltas (7) and (5). Cohort composition is canonical here, so the
  seventh cohort's members, its delivered count, and the discharge of
  `check-spec-pointer`'s criterion-6 hold land in this section and nowhere else.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by delta (10), and **conditionally**: the four existing rows are re-read
  against the post-port tree and edited only where the runtime derivation makes
  them false. Delta (3)'s ruling — that the auditor-stays-shell principle is
  scoped to the declaration-and-dispatch adjudicators — lands beside the two rows
  that state it, because a later selector reads it there.
- **gate-sdk/SPEC.md §check-gate-exemption-tasks** — owned by delta (6), the
  holder census's citation of `spec_queue_slugs` repointed to its new locus with
  the count unchanged.
- **gate-sdk/SPEC.md §Fixture-pair discipline** — owned by delta (1). The rule
  that a ported member's fixture pair must exercise **every arm of the corpus
  derivation it ports**, not merely one, and that the pair is the parity oracle
  precisely because it is inert under the port.
- **canon-kit/SPEC.md §lib/spec.sh** — owned by deltas (5) and (6). The governed
  comment surface's bullet and the queue-resolution bullet move from *shell only*
  to the crate, the *which primitive is on which substrate* paragraph is
  corrected where it stands, and the independent-holder property is stated as
  surviving the port rather than left to be inferred.
- **canon-kit/SPEC.md §check-comment-tier, §check-spec-pointer,
  §check-todo-task-liveness, §check-deprecation-task** — owned by deltas (4),
  (7) and (8): each member's declaration spelling, its widened couples, and for
  `check-deprecation-task` its ERE locus.
- **queue-kit/SPEC.md §The queue format** — owned by delta (6), the independent
  holder's locus only. The rule is unchanged and is not restated.
- **TASK-QUEUE.md `cohort-held-members-port-prerequisites`** — owned by delta
  (13), two spent lines removed.
- **TASK-QUEUE.md `native-gate-port-remaining-corpus`** — owned by delta (12),
  the cohort-count demotion.

## Definition of Done

- [ ] **Causal completeness** — every new descriptor, subcommand and fixture arm
      names its producer and its consumer; no field is added that lacks a reader,
      and no fixture arm is added that no `expect.txt` reads; delta (10)'s
      derivation and delta (9)'s `--knobs` verification are **run**, not reasoned.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section, the merged spec readable by someone who
      never saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge. The none-remain
      assertion (`ls gate-sdk/SPEC-*.md`) is discharged **at the iteration**, not
      at this commit: a sibling gate-sdk amendment is in flight this iteration
      (`SPEC-diff-renderer.md`), so only the batch merging the last of the two
      can satisfy it.
- [ ] **Removals propagated** — four shell scripts and three shell library
      functions deleted, every citation of them repaired at its source, and
      `bash gate-sdk/bin/run-gate-tests.sh canon-kit/gate-tests` green against
      the compiled members with the widened arms.
- [ ] **Gaps filed** — anything found and not fixed routed to the gap inbox with
      its cost, never flagged and skipped.
