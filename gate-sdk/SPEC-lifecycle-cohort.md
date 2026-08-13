# SPEC amendment: lifecycle-cohort

The sixth gate cohort of the native port: **lifecycle-kit taken as a near-whole
kit** — ten members on the `lib/stages.sh` derivation. Ruled by the operator
2026-08-13, and the ruling fixes three boundaries this amendment may not move:
`check-stage-entry` is **held** on the associative-array bridge;
`check-close-surfaces` is **out rather than held**, unsized rather than
deferred, so a later selector owes it a sizing; and a
`check-spec-derivable-section` ride-along was offered at zero marginal
derivation cost and **declined**. The cohort is not to be widened.

This amendment designs the port. It does not restate the porting procedure
(§Porting a gate to the binary substrate), the descriptor format (§The
`# graph:` manifest), the conservation contract (§Meta-gate conservation for
the binary substrate), the criteria roster (§The port-candidate criteria), or
the directive's grounds (TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's
sequence).

**What this cohort pays that no previous cohort paid is not the gates. It is
their callers.** Five cohorts have deleted a `checks/<name>.sh` and left every
caller undisturbed, because every caller was the fixture harness and the
fixture harness has resolved substrate-agnostically since the first cohort.
lifecycle-kit is the first kit whose **scenario runners**, whose **own `bin/`**,
and whose **own `smoke/`** invoke its members by script path. One of those
callers is `bin/enter-stage.sh`, the stamp mechanism of the state machine this
repo runs its lifecycle on. Sizing the cohort as ten gate ports and discovering
the callers at build is exactly the failure mode criterion 7 exists to prevent.

## Three premises this cohort is sized on, each probed rather than inherited

### (i) The census is re-derived, not cited

`.workflow/survey-record.md` carries the sizing, and the record's own witness
was run rather than its prose trusted. `bash gate-sdk/bin/port-blockers.sh` at
the authoring rev: 103 registered members scanned, 26 carrying a `.gate`
descriptor, 77 still shell — identical to the recorded finding, and **zero rows
for any of the twelve lifecycle-kit gates**, so criterion 7 clears kit-wide
against the tree rather than against the record. The corpus diff since the
recorded rev over `scripts/gates.list */checks */lib gate-sdk/SPEC.md
TASK-QUEUE.md` moved only `TASK-QUEUE.md`, and that delta is the scope stage's
own promotion work, touching no gate-composition input.

The membership was then re-derived from scratch rather than read off the record:
eleven of the twelve lifecycle-kit checks source `lib/stages.sh`, the twelfth is
`check-close-surfaces`, and all twelve are registered in `scripts/gates.list`.
Ten portable, one held, one out.

This matters beyond diligence. The scope stage's record was later found to carry
a provenance defect, and its rival sizings were corrected twice — once wrongly.
The figures above are this stage's own and stand on their own oracle.

### (ii) The by-kit selector's caveat is answered again, and the honest answer is weaker than queue-kit's

§The first cohort warns in the same breath that admits the queue-kit precedent:
a later selector must not read it as licence to take a kit whose gates share
nothing. Answering it here requires stating something the sizing survey did not.

**Only five of the ten actually call a `lifecycle_*` function** —
`check-stage-evidence`, `check-stage-skill-coverage`, `check-merge-attrs`,
`check-scratch-citation`, `check-lifecycle-registration`. The other five source
`lib/stages.sh` **for its knobs alone**. That is a weaker sharing claim than
queue-kit's, whose corpus derivation *is* the queue file, and weaker than
canon-kit's `spec_manifest_files`.

It is still the right cohort, and the reason is that the shared thing is the
**config surface**, not the walk. All fifteen knobs the ten members read are
defined by `lib/stages.sh`, so the bridge's does-not-define refusal cannot fire
for any of them, and every one crosses under criterion 6's discharge-by-
construction: the value is computed once, shell-side, and the binary holds no
default to drift. Porting the ten pays that config surface once and proves it
ten times. What is *not* shared ten ways is a corpus walk, and the amendment says
so rather than letting a later selector inherit an overstated precedent.

### (iii) The premise the hold rests on is false, and correcting it is part of this cohort

`check-stage-entry`'s hold cites the bridge's associative-array limit. That
limit is real. The paragraph stating it is not accurate, and it was verified at
source rather than taken from the gap that reported it.

Exactly three knob-shaped `declare -A` instances exist in the tree:

- `QUEUE_KIT_LESSON_SINKS` — queue-kit/lib/queue.sh. Sole reader
  `queue-kit/bin/lesson-sink.sh`. No gate reads it.
- `EVIDENCE_KIT_SCENARIO_GLOBS` — evidence-kit/lib/evidence.sh. Read **by key**
  by `evidence-kit/checks/check-evidence-baseline.sh`.
- `LIFECYCLE_KIT_PREDECESSOR` — default in lifecycle-kit/lib/stages.sh, consumer
  override in `scripts/lifecycle-config.sh`. Read **by key** by
  `lifecycle-kit/checks/check-stage-entry.sh`.

So the bridge section's claim that `QUEUE_KIT_LESSON_SINKS` is *the* live
instance and *not a blocker because no gate reads it* is false twice over:
three instances exist, and two of the three are gate-read. The sentence was true
when written and has aged; correcting an aged fact where it stands is ordinary
work, and no operator ruling asserts it, so nothing here reverses a ruling.

Shipping a hold whose cited ground states a false premise ships the defect,
which is why the correction rides this cohort rather than waiting for the unit
that lifts the hold.

## What changes

### The premise correction

**(1) §lib/gate.sh's bridge section is corrected where it stands.**
[design-bearing] The `declare -A` limit's statement keeps its rule — the bridge
carries scalars and indexed arrays, a key channel does not exist, and a gate
reading an associative knob is not a port candidate until the wire format grows
keys. What is rewritten is the instance claim that follows it. The corrected
text names all three instances, distinguishes the one with no gate reader from
the two with gate readers, and states the consequence the old sentence
foreclosed: the limit is **exercised**, in two kits, and is a live port
prerequisite rather than a documented curiosity carried forward from a cohort
that never touched it. The closing sentence's purpose survives verbatim in
force — it was written so a later kit's port would not discover the limit at
implementation time, and this cohort is the port that would have.

No correction is appended beside the old sentence and no annotation marks it
superseded: two readings of one fact is the defect whichever wears the label
*current*.

**(2) `cohort-held-members-port-prerequisites` gains one shared prerequisite
line.** [mechanical] That entry owns the roster of held members and the kits it
spans, and the associative-array bridge is now a prerequisite **shared across
two kits** — `check-stage-entry` (lifecycle-kit) and `check-evidence-baseline`
(evidence-kit) — which is a shape the entry has never carried and which no
per-member line would express. The line names the wire-format change the two
members jointly wait on, so the next selector reaching either kit finds one
prerequisite rather than re-deriving the same gap twice.

The entry stands at 44 lines against `QUEUE_KIT_ENTRY_LINE_CAP` of 50, measured
with `bash queue-kit/bin/queue-index.sh --extent`, so the write fits with room
and needs neither compression nor authorization. Named here so a build session
does not read a long entry as a wall.

### The callers, which are this cohort's real work

**(3) Seven scenario runners convert from a script path to `gate_run`.**
[design-bearing] `lifecycle-kit/gate-tests/` carries nine `*.test.sh` runners
and every one of them holds its gate as a `checks/<name>.sh` path. Seven name a
cohort member — `check-stage-evidence`, `check-skill-binding`,
`check-shim-restatement`, `check-lesson-disposition`, `check-merge-attrs`,
`check-survey-record`, `check-lifecycle-registration`. Delete those seven
scripts and the runners fail on a missing file.

The conversion target exists and needs no design: `gate_run <name> <checks-dir>
<args>` in `gate-sdk/lib/test-hermetic.sh`, which resolves through
`gate_command` exactly as the fixture harness does and which every one of these
runners already has in scope, since all nine source `test-hermetic.sh`. The
precedent is `canon-kit/gate-tests/check-tracking-claim.test.sh` — **the only
runner in the tree that names a ported member**, and therefore the only
worked instance of this conversion.

It is labelled design-bearing rather than mechanical for a reason the sweep
shape hides. `git log --diff-filter=D` over the `*.test.sh` glob returns
**empty**: no scenario runner has ever been deleted, and every runner other than
that one names an unported member. So no cohort has ever converted seven, and
the per-runner judgment is real — a runner asserting on stderr text, passing
per-case environment (for which `gate_env` exists beside `gate_run`), or
invoking its gate more than one way each needs its assertions re-read against
the dispatch rather than pattern-replaced. The mechanical part is the
substitution; the judgment is whether each runner's assertions still mean what
they meant.

**(4) `bin/enter-stage.sh` dispatches substrate-agnostically, and this is the
one that must not be discovered at build.** [design-bearing] It runs
`check-stage-evidence` by script path in the rename arm, and
`check-stage-entry` by script path in the preflight. The second survives — that
member is held. The first is a cohort member, and this script is the stamp
mechanism of the state machine: every stage entry in this repo, and in every
consumer running the lifecycle, goes through it.

The design question is not how to dispatch but what the dispatch costs the kit.
`enter-stage.sh` sources `lib/stages.sh` and nothing else; resolving a gate
through `gate_command` means lifecycle-kit's `bin/` takes a dependency on
`gate-sdk/lib/gate.sh`. That dependency is precedented **inside the same kit** —
every one of its `checks/` sources exactly that library — but not in `bin/`, and
taking it silently would make lifecycle-kit's tool layer newly unable to run in
a tree that vendored the kit without gate-sdk. The port takes the dependency and
says so, because the alternative — a second dispatch resolver written into
`bin/` — is the duplicate the substrate exists to remove.

Two properties of the call site bind the implementation. The gate is invoked
with **two positional arguments** (a temp queue and a temp state file), which
`gate_command` carries unchanged since the argv it yields is prefix-shaped. And
the preflight reads the gate's **combined output** and its exit status to
compose a refusal message; the bridged argv leads with `env`, so a caller
needing the executable rather than the command takes the first element that is
neither `env` nor a `NAME=VALUE` assignment — the rule §lib/gate.sh already
states for `run-gate-tests`, applying to a second caller for the first time.

**(5) `smoke/install.sh` loses three path invocations and one stale oracle
string.** [design-bearing] The kit's consumer smoke invokes
`check-lifecycle-registration` once and `check-merge-attrs` twice by path,
inside a **vendored scratch consumer** — the one context where the `.sh` is
genuinely absent after the port and where the binary's presence is the smoke's
own problem rather than an ambient fact. The smoke must resolve the gate the way
a consumer does, which is the point of the smoke; what it may not do is assume a
built binary without saying what happens when there is none. `gate_command`'s
answer is already the right one — absent or non-executable is exit 2, never a
skip and never a pass — and the smoke inherits it rather than inventing a
fallback.

The fourth site is data, not dispatch: the smoke files a survey record whose
**oracle field** names `bash checks/check-survey-record.sh`. Nothing executes it
and no gate validates it, which is exactly why it would survive the port as a
quietly false witness command in the one artifact whose whole purpose is being
re-runnable. It is corrected with the rest.

### The ten members

**(6) Port the ten to compiled subcommands, deleting each shell original in the
motion that lands its descriptor.** [design-bearing] Assertion A forbids a
`<name>.sh` and a `<name>.gate` coexisting in one resolve dir, and criterion 2's
parity proof is bought while both implementations still exist — the only order
in which parity can be proved at all. Each descriptor carries the `# graph:`
manifest unchanged, its `# spec:` pointer, and nothing else.

The fifteen knobs the ten read are all scalars or indexed arrays and all defined
by `lib/stages.sh`, so **no library move is owed** — unlike `check-kit-
registration`, whose port needed one before the bridge could carry its knobs.
That claim was reached by reading definitions rather than by running the bridge,
so it is re-verified at build against `--knobs` before the first descriptor
lands.

Two members carry the volume. `check-scratch-citation` composes an awk program
held in a shell variable into a second awk program, over a paragraph-joined
scan with per-line start offsets — no mechanical transliteration exists and it
ports as ordinary Rust. `check-shim-restatement` runs a parameterised n-gram
generator and a two-file awk join through a temp index. Both are implementation
volume rather than new mechanism; neither needs an engine the crate lacks.

**(7) Two knobs this repo's own configuration makes invisible to a live-tree
parity run.** [design-bearing] The canon-kit cohort's transferable lesson —
the live tree is the oracle a session trusts most and it proves nothing about a
branch it does not execute — applies twice here, and both instances are
`scripts/lifecycle-config.sh`'s doing.

`LIFECYCLE_KIT_SESSION_BOUNDARY` is set to `iteration`, which **disables**
`check-stage-evidence`'s cross-stage session-id distinctness map. Every parity
run in this tree takes the disabled branch, so the enabled one ships unproven
unless a constructed scenario drives it.

`LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS` contains no `**`, so
`check-scratch-citation`'s globstar path is never exercised live. Criterion 6's
glob commitment binds the reader anyway — a Rust matcher over a bridged knob is
`**`-capable, matching the `globstar` the shell side enables — and this is the
member that commitment was written for. A constructed scenario proves it;
a live-tree run cannot.

A third property is worth naming because it silently narrows every parity run
in this tree: this repo runs a **six**-stage machine with a `spec` stage, so
`LIFECYCLE_KIT_STAGES` and `LIFECYCLE_KIT_PREDECESSOR` are both overridden. Any
parity comparison run here reads the override, never the kit default.

**(8) `check-merge-attrs` implements the set difference, not the pipeline that
computed it.** [design-bearing] The shell reaches for three `comm` invocations
over six process substitutions. The kit-roots cohort already ruled the general
case and it applies verbatim: `sort`'s collation is locale-dependent and a set
difference is not, so the port implements the rule the contract states rather
than the mechanism the shell reached for. Naming it here stops the port
transliterating a `comm` pipeline into a Rust sort-and-walk that inherits a
locale dependency the contract never had.

**(9) `check-lifecycle-registration` is a freshness gate in miniature, and the
freshness family's hold key does not reach it.** [design-bearing] It
byte-compares tracked text in the agent file against a live derivation, which is
structurally what the six generated-projection freshness gates do — and those
are held per member on the question *is this gate's emitter ported?*

The key does not hold this one, and the difference is exact rather than a
judgment call. The family's members shell out to `bash <emitter>`; this member's
"emitter" is `lifecycle_registration_block`, a **library function it sources
directly**, and that library is the derivation this cohort compiles. So the
emitter ports *in this cohort*, in the same motion as the gate that reads it,
and the hold's condition is satisfied rather than waived. Written out because a
selector applying the family's rule mechanically would hold this member, and the
operator ruling did not.

**(10) `check-stage-evidence`'s missing fail-closed guard dissolves rather than
being repaired.** [mechanical] It is the only one of the ten that does not
source `gate-sdk/lib/gate.sh`, so it has no `fail_closed`, and its awk runs
unguarded inside a process substitution — a spawn whose failure would read as an
empty result rather than as an error. No `# spec:` line accounts for the
omission and it is a real gap in the shell form.

It is not repaired in shell, because the shell form is deleted in the same
motion. The compiled member has no subprocess at that site at all, so the
failure mode it left open is absent by construction rather than guarded. Stated
so the port is not read as having silently dropped an assertion, and so nobody
opens a repair against a file that no longer exists.

### Standing obligations

**(11) The conservation-table verdict is taken by running the derivation, not by
reading this amendment.** [mechanical] Assertion C's substrate-sensitive set is
derived at runtime from each member's expanded `couples=`, and §The
port-candidate criteria fixes the discipline: the corpus is derived from the
tree, so the verdict is taken by running the derivation at cohort-cut time.

The manual derivation says no row is owed — only `check-scratch-citation` and
`check-shim-restatement` carry a `kit:` token, expanding to `<kit>/lib/stages.sh`
and `<kit>/templates/*.md`, neither of which reaches any kit's `checks/`, and
the remaining eight couple literal non-`checks/` paths. That is a reading, not
an oracle verdict: `check-gate-substrate-parity` is fail-closed and could not be
run where the derivation was performed. **Build runs it first**, before the
first descriptor lands, and the row question is answered by its output.

**(12) TRAJECTORY.md's sixth-cohort ruling is deleted by the landing session.**
[mechanical] The ruling names its own discharge event — *the session that lands
the cohort deletes this ruling* — so the session that meets it deletes rather
than judges, and deleting a ruling whose subject is finished decides nothing.
A retirement's blast radius is derived rather than rostered, so the landing
session greps for inbound citations of the deleted text, repairs each at its
source, and regenerates the docs mirror rather than hand-editing the second
copy. Both properties that make the grep non-obvious apply: every kit-SPEC
citation appears twice, and a citation naming a **surviving section** still
resolves after the ruling inside it is gone.

**(13) The queue entry is demoted, not moved to Done, and it has no line to
spare.** [mechanical] `native-gate-port-remaining-corpus` is the whole corpus;
a Done move would assert a finished port and drop it from the public roadmap
projection. It returns to the deferred section under `[design-pending]`.

The constraint the demotion carries is measured: the entry sits at **exactly 50
lines against the 50-line cap**. Recording the sixth cohort there must therefore
be a net-zero edit — the count of closed cohorts and the trailing provenance
line, nothing added — because every cohort record is canonical at §The first
cohort and a second copy in the entry would be one more to drift. If the edit
cannot be made net-zero, the relief is compression by *answering* a ground, never
by dropping one.

**(14) The bookkeeping fan-out, measured off the fifth cohort's own commit
rather than predicted.** [mechanical] `.workflow/tightened-gates.txt` gains one
line per newly-compiled member — expect **ten**. `scripts/gates.list` is **not**
touched: registration is by bare name and a `.sh`→`.gate` swap does not move it.
`lifecycle-kit/README.md` is **not** touched: it lists bare gate names with no
extension, and the ERE cohort's amendment predicted exactly this edit, was
wrong, and the build correctly no-op'd it — the prediction is not repeated here.
The projections that do move are the generated pre-commit hook and the docs
mirror, regenerated per docs/site-architecture.md §Generated projections rather
than hand-edited. `bash gate-sdk/bin/build-native.sh` and the full battery are
both owed at every commit, and neither discharges the other.

**(15) Criterion 5 is priced per member and paid per cohort.** [mechanical] The
amendment records the **binary-less residual** measured against the
post-cohort registry, after the cohort's own commit, since the packer refuses a
dirty worktree. Ten members each individually runnable is not a discharge; the
aggregate is.

## Producers and consumers

**The `.gate` descriptors — producer.** Ten new descriptor files under
`lifecycle-kit/checks/`, each created by the porting commit that deletes its
shell sibling. Their consumers are the ones the descriptor's closed field roster
already names, and no field is added: `# graph:` is read by
`gate_expand_couples_var` and through it by `gen-pre-commit`, `check-graph` and
`run-gates --for`; `# spec:` by canon-kit's `check-spec-pointer`. No member
takes `# no-fixture:` — all ten keep the fixture pair they ship with.

**The subcommands — producer.** Ten new entries in the crate's gate registry,
one per member, dispatched by the name that already identifies the gate
everywhere else. The consumer is `gate_command`, which reads the descriptor's
presence as the dispatch declaration; there is no mapping table to update
because the one fact is the file's presence.

**The fifteen bridged knobs — no new producer.** Every knob the ten read is
already declared and already defaulted by `lib/stages.sh`, and the bridge
carries the *resolved* value. The reader is the compiled member, through
`--knobs`. What must be verified at build rather than assumed is the enabling
half of producer-reachability: that `--knobs` reports each of the fifteen for
the member that reads it, since a knob the owning library does not define is the
bridge's third refusal and would fail-close on every invocation. The claim that
all fifteen are defined was reached by reading `lib/stages.sh`, not by running
the bridge.

**The library's load-time validation — an existing producer whose reach is
unchanged.** `lib/stages.sh` accumulates configuration errors and exits 2 on a
malformed config, including the predecessor-map membership checks and the
terminal-drain refusal. Under the bridge that validation runs **shell-side**
during knob resolution, in the subshell the kit's libraries are sourced in, so a
compiled member inherits it without a Rust twin. This is criterion 6's
discharge-by-construction rather than a parity test, and it is the reason the
port adds no configuration validation of its own.

**`gate_run` — an existing consumer gaining seven callers.** Its contract is
unchanged; what changes is that seven lifecycle runners begin using it, taking
its count of callers from one to eight. `gate_env` is beside it for any runner
that sets per-case environment, and no runner needs a new primitive.

**`gate_command` — an existing consumer gaining two callers outside the test
lane.** `bin/enter-stage.sh` and `smoke/install.sh` become the first non-test
callers to resolve a gate through it. Its `env`-prefixed argv and its exit-2 on
an absent binary are both existing contract, read here by new readers.

**Every field this amendment introduces has a named reader**, and the check ran
in the direction that catches the omission: the descriptors' field roster is
closed and adds none, the subcommand registry entries are read by
`gate_command`, and no knob is created. The one *removal* is the ten shell
scripts, and its readers are enumerated in deltas (3), (4) and (5) — that
enumeration is the amendment, not a side note.

**The narrowing check, run in the direction that is not monotone.** Deleting ten
scripts narrows the corpus several gates walk, and "a narrower corpus can only
remove violations" is false. Each affected reader's **red condition** was
enumerated rather than its subject: `check-gate-substrate-parity` assertion A
reds on *coexistence*, so removal can only satisfy it; assertion B reds on an
implementation with no descriptor, and the descriptors land in the same commit;
assertion C reds on a derived member with **no** disposition row, a zero-count
condition that is not monotone under narrowing — which is precisely why delta
(11) runs the derivation rather than reasoning about it. `check-shellcheck` and
`check-comment-tier` lose ten files from their corpus and red only on findings,
so both are monotone and clear by inspection.

## Existing sections updated

- **gate-sdk/SPEC.md §lib/gate.sh** — owned by delta (1). The bridge's
  associative-array paragraph keeps its rule and its closing purpose; the
  instance claim is rewritten in place to name all three knobs and the two gate
  readers.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by delta (6), with (2) and (9). Cohort composition is canonical here, so
  the sixth cohort's members, its delivered count, `check-stage-entry`'s hold and
  its ground, and `check-close-surfaces`'s unsized status land in this section
  and nowhere else. Delta (9)'s reasoning attaches to the freshness family's
  per-member hold key already stated here, as the worked case where the key is
  satisfied rather than waived. The by-kit caveat gains premise (ii)'s honest
  qualification — the kit and corpus boundaries coincide again, but on a config
  surface rather than a walk.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by delta (15), the
  cohort's criterion-5 residual recorded where the criterion's per-cohort
  accounting lives.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by delta (11), and **conditionally**: edited only if the runtime derivation
  selects a member, unedited if it selects none. A row added because this
  amendment predicted one would be a maintained roster answering a question
  assertion C never asked.
- **lifecycle-kit/SPEC.md §bin/enter-stage.sh** — owned by delta (4). The
  preflight's dispatch changes from a script path to the resolved command, and
  the kit's `bin/` layer gains its stated gate-sdk dependency.
- **lifecycle-kit/SPEC.md §Testing** — owned by delta (3), the scenario runners'
  dispatch convention.
- **TASK-QUEUE.md `cohort-held-members-port-prerequisites`** — owned by delta
  (2), the shared associative-bridge prerequisite spanning two kits.
- **TASK-QUEUE.md `native-gate-port-remaining-corpus`** — owned by delta (13),
  the net-zero demotion.
- **TRAJECTORY.md §PRIORITY DIRECTIVE** — owned by delta (12), the ruling
  deleted outright on its own named discharge event.

## Definition of Done

- [ ] **Causal completeness** — every new descriptor, subcommand and caller
      change names its producer and its consumer; no field is added that lacks a
      reader; delta (11)'s derivation is run rather than reasoned, and delta
      (6)'s `--knobs` verification is run rather than read.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper section, the merged spec readable by someone who never saw this
      amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — the ten shell scripts deleted, and every caller
      in deltas (3), (4) and (5) converted; `bash gate-sdk/bin/run-gate-tests.sh
      lifecycle-kit/gate-tests` and the lifecycle consumer smoke both green
      against the compiled members.
- [ ] **Gaps filed** — anything found and not fixed routed to the gap inbox with
      its cost, never flagged and skipped.
