# SPEC amendment: test-runner-cut

The port disposition of **delegation-kit's two `bin/`-level members behind
§Testing** — `bin/run-usage-tests.sh` (226 lines) and `bin/run-trend-tests.sh`
(63), 289 lines — off the shell substrate. This is one of the iteration's two
port cuts under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed
at scope and ruled option (a) by the **lead on its own authority**, 2026-09-02,
over the resume channel; it did not reach the operator, and that is stated
because a composition ruling recorded without its authority reads at the
post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 115 files scanned, 64 declared `no-port`, 0 temporarily held,
**51 owed**. This cut takes two of that column. **Neither member is a gate** —
neither has ever joined `gates.list` and neither owes a `good/`+`bad/` fixture
pair, which §Testing already states in its own words ("No fixture pair owed —
neither script is a gate") — so no gate roster and no binary-less residual
roster moves, which is the whole of what criterion 5's per-cohort measurement
asks of a cut with no gate in it.

**The survey behind the composition is carried rather than re-bought, and its
witness was run at this stage's entry.** The record it sits in is
boundary-truncated scratch, so a pointer into it resolves to nothing one
iteration after this is written (lifecycle-kit/SPEC.md §The survey record).
Question: *which owned specification section is the next well-formed port cut,
and what does each candidate group unblock?* — corpus `'*.sh' ':!*.test.sh'`;
oracle `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree`; rev
`4942ae172b943ec84c0c05c6b867baffbbd049fb`; finding: of 51 owed files, six
groups of two or more sit behind one stated-contract section, and this is the
one unblocked candidate with no external sequencing on it. Witness: the corpus
diff since that rev is **empty** and the oracle still reads 51 owed, so the
finding is cited rather than re-derived.

## What changes

### (1) The two members are one cut, and the ground is their own headers, not their directory

`native-gate-port-remaining-corpus`'s composer ruling of 2026-08-28 selects a cut
**by stated contract** — the owed files behind one specification section, ported
behind the one amendment that section needs — and refuses size- and kit-ordered
composers on the ground that a cut assembled for convenience *averages* grounds
its members do not share {design-bearing}. These two share one ground and it is
not their kit: **each declares `delegation-kit/SPEC.md §Testing` in its own
`# spec:` header**, verbatim — `run-usage-tests.sh:2` "decision-table runner for
usage-verdict"; `run-trend-tests.sh:2` "assertion runner for usage-trend over a
fixture history".

**Membership is complete and was probed, not inferred.** Exactly four tracked
`.sh` files declare that section, and the other two —
`delegation-kit/smoke/install.sh` and `delegation-kit/smoke/violation.sh` —
carry `# no-port:` at `install.sh:4` and `violation.sh:3` and are permanently
shell under the smoke class ruling. `guard-kit/bin/run-guard-tests.sh` is the
same *shape* and declares **guard-kit's own** §Testing, so it is not a member;
that near miss is recorded because the shape is what a reader matches on and the
declaration is what decides.

**One sentence in the section will be misread and is named here so it is not.**
§Testing ends "Both scripts stay on the shell substrate permanently and carry
`# no-port:` saying so" — that clause reaches the two `smoke/` members and only
those. Neither runner carries a `# no-port:` and neither is covered by it.

### (2) The family is not the emit family, and the exit-status contract is why

Both runners speak **exit 0 / 1 / 2** — 2 for a harness refusal (a missing or
non-executable subject, an absent fixture, and for `run-usage-tests.sh` the
anti-vacuity `ran == 0` guard), 1 for an assertion failure, 0 clean
{design-bearing}. gate-sdk/SPEC.md §The non-gate arm rules that "a member whose
stated contract is its child's exit status cannot take the `--emit-` spelling at
all", because `Arm::Emit` collapses every error to 2 and every success to 0. So
the bridged-arm road for these two is **not** `--emit-usage-tests`; it is two
new `Arm::Run` members with operand-style top-level spellings and two new
front-end arms in `bin/run-gates.sh` beside `--usage-poll` and `--lesson-sink`.

**That road is refused, and the ground is what it would cost against what it
would buy.** It mints two top-level entry points into the binary and two
front-end arms whose only caller is a validate suite reading an exit code, and
each would have to join `BRIDGED_ARMS` with an **empty** declared roster — see
delta (6), where declaring a knob is not a neutral choice but the port's worst
available error. Two rows, two flags and two front-end branches, for no
capability the crate's own test lane does not already carry.

### (3) Both runners retire into crate tests, and the section's own precedent is the shape

Each runner becomes an in-crate test module reading the **same committed
fixtures** — `delegation-kit/usage-tests/cases.tsv` (23 non-comment rows, 11 tab
columns) and `delegation-kit/usage-tests/trend-history.log` (20 lines) — off
disk, unchanged and unmoved {design-bearing}.

**This is not a new shape; it is this section's own, already shipped.** §Testing
records that `usage-tests/dispatch-guard-cases.tsv` "stays on disk and is now
read by the crate test that replaced `bin/run-dispatch-guard-tests.sh`", and
"moving it into Rust literals would trade a reviewable table for a recompile".
That reader is `native/src/hook/dispatch.rs`, whose own comment says "the kit's
own decision table, read from disk rather than transcribed into Rust literals …
This test replaces its shell driver". Both fixtures here take the identical
disposition and for the identical reason.

**The section's rule about *when* a runner retires is widened, and the widening
is the delta rather than an aside.** §Testing today says "The runners retired
with their subjects; one table stayed and one did not, and the split is the rule
rather than an accident" — narration of a cut whose subjects had moved in-crate,
so a `#[test]` could reach them in process. Here the subjects **stay shell**:
`bin/usage-verdict.sh` declares §usage-verdict and `bin/usage-trend.sh` declares
§Trend reporter, so neither is in this cut by the composer's own words. The rule
the section states is therefore narrower than the rule it needs, and it becomes:
a runner retires into the crate when its **cases** can be driven from there,
whether the subject is reached in process or spawned. The stated-with-subject
form was true of its instance and is not the general case.

### (4) The subjects stay shell and the tests spawn them, which clears criteria 6 and 7 together

Each test invokes its subject exactly as the shell runner does — `bash
delegation-kit/bin/usage-verdict.sh <usage> <cred>` and `bash
delegation-kit/bin/usage-trend.sh <history>` — with the child's stdout and
stderr **merged**, because every assertion is made against the merged stream and
capturing them apart would make the fail-soft no-leak conjunct vacuously true
{design-bearing}.

- **Criterion 7 clears outright.** Every program either runner spawns —
  `bash`, `date`, `env`, `grep`, `mktemp`, `rm`, `touch`, `cat` — is on
  `GATE_SDK_PROGRAM_FLOOR`, and §The port-candidate criteria rules that "a rule
  shelling out to `bash <emitter>` clears this criterion, because `bash` is on
  the floor, however unported that emitter is". The shape is already shipped in
  non-test crate code: `native/src/hook/budget.rs` spawns the still-shell
  verdict tool through its knob "exactly as the shell member did", recording
  that "the verdict binary stays external and stays spawned: it is a member of
  the kit-`bin/` owed cohort, not of this cut". This cut says the same sentence
  about the same tool from the test side.
- **Criterion 6 clears by deletion, the strongest road.** Its discriminator is
  "whether the shell caller set empties, not taste", and here it empties
  completely: nothing in the tree calls either runner but
  `scripts/evidence-config.sh`'s two suite commands, which delta (9) retires.
  No twin is created, so no standing cross-substrate lane is owed and the
  duplication is **absent** rather than machine-held.

**`touch -d "@<epoch>"` is the one spawn worth naming**, because it sets the
credentials file's mtime and that mtime is the whole login-window input. It is
on the floor, so criterion 7 does not bind. It is nonetheless a **POSIX-only**
call, and the port does not buy its way out: the test already spawns `bash` into
a shell subject, so it is unix-bound for as long as §usage-verdict is, and a
crate-floor move bought here would purchase a portability this cut cannot
deliver. The floor question comes due at **§usage-verdict's own cut**, when the
bash spawn goes and the mtime set is the last unix-ism left — named here so that
cut meets it already priced rather than discovering it. gate-sdk/SPEC.md records
what such a move costs: an MSRV bump is a `check-crate-arms` input that
un-suppresses clippy lints against unchanged code, and the 1.56→1.71 move
surfaced four findings in modules its cohort never edited.

### (5) The hermetic contract survives as an explicit child environment, and the poison export stays a real parent export

§Testing's stated contract for these runners is a **hermetic sandbox** — each
case runs "in a throwaway sandbox with no consumer config on the lookup path
**and** with ambient `DELEGATION_KIT_*` exports stripped at each gate
invocation, so the gate exercises its own defaults hermetic to the host repo",
each runner "under a deliberate poison export that fails its own table loudly if
the strip ever breaks" {design-bearing}. Three properties, each preserved
explicitly because each is a place a reimplementation would quietly differ:

- **The strip is the whole `DELEGATION_KIT_*` namespace, derived at run time.**
  The shell computes it as `env | grep -o '^DELEGATION_KIT_[A-Za-z0-9_]*'` into
  an `env -u NAME …` list applied at **every** subject invocation. In the crate
  it becomes an explicit child environment built by iterating the process's own
  variables and dropping every one with that prefix — a *stronger* expression of
  the same rule, and one that cannot be satisfied by a hardcoded list. A
  hardcoded list re-creates precisely the failure the poison export exists to
  catch.
- **The poison export must remain a real export in the parent process.**
  `run-usage-tests.sh:15` exports `DELEGATION_KIT_PAUSE_PCT=0` and
  `run-trend-tests.sh:15` exports `DELEGATION_KIT_USAGE_HISTORY=$HIST` — the
  second being the *same knob* the reporter reads, which is what makes the
  unset-knob case at `run-trend-tests.sh:53-54` meaningful at all. If the port
  keeps only a clean child environment and drops the parent-side poison, every
  assertion still passes and the proof becomes vacuous: nothing would fail if
  the strip broke. This is the single thing in the cut that a correct-looking
  reimplementation loses silently, and it is stated as a requirement rather than
  left to the strip's description.
- **The subject runs with its cwd inside the throwaway sandbox**, so no consumer
  config sits on the lookup path.

### (6) The declared-knob roster question does not arise, and it would have been the port's worst error

Neither runner holds an **inline knob default** — probed rather than assumed:
every `DELEGATION_KIT_*` knob either touches is already defaulted in
`delegation-kit/lib/delegation.sh`, and the only `${…:-…}` forms in either file
are the two positional defaults, which are not kit knobs. So gate-sdk's rule
that "a default the deleted shell driver held inline moves into the owning kit's
library in the same cut that deletes the driver" **binds zero times on this
cut** {design-bearing}. That is a clean result and it is stated as one, because
a cut that reports no knob movement is otherwise indistinguishable from a cut
that failed to look.

**And the roster is empty on purpose, which is the sharper half.** Had these
landed as bridged arms, the natural move would have been to declare
`DELEGATION_KIT_USAGE_HISTORY` — the knob the trend runner sets from its
positional. That is exactly wrong: `gate_knob_env` resolves a declared knob from
the *tree's own config* and hands the resolved value in, which is what the
ambient strip exists to prevent. A declared roster would let a host override
reshape the decision table the section says must encode the kit defaults. The
crate-test road makes the question moot, and recording why keeps the next
reader from re-opening it as an omission.

**Two literals in `run-usage-tests.sh` restate library values and must stay
literals.** `:61-62` asserts `width=2 ` against `DELEGATION_KIT_FAN_WIDTH`'s
default and `:168`'s diagnostic names `REFRESH_MIN_AGE`'s 60 seconds. These are
*test expectations of the kit defaults*, which is what the contract wants; a
port that reads them from the library instead would make the table agree with
whatever the library says rather than assert what it should say.

### (7) The two input-corpus positionals retire with the files, and nothing documented dies with them

Each runner takes one optional positional — `[cases.tsv]` and
`[history-fixture]` — and both are **input-corpus positionals** under gate-sdk's
distinguishing test: each selects what the rule analyses rather than redirecting
config `gate_command` has already resolved {mechanical}. So the test binds zero
times and neither is an unportable argument.

They retire nonetheless, because a crate test drives the committed fixtures and
has no caller to pass one. That is an interface removal, so the doc obligation
is checked rather than waved: the only place either positional is documented is
each file's own `usage:` header comment, which dies with the file. No governed
surface, no README line and no suite command passes one — `EVIDENCE_KIT_RUN_*`
invokes each bare. So no sentence is left standing over a removed argument.

### (8) The traps a reimplementation would spring, enumerated because each is silent

Each of these changes a verdict without changing a message {design-bearing}:

- **`width=2 ` carries a trailing space** (`run-usage-tests.sh:61`). Without it,
  `width=20` matches and the fan-width assertion stops discriminating.
- **`now` is captured once**, at `run-usage-tests.sh:27`, and every case's
  snapshot timestamps are computed relative to that one reading. §Testing states
  why — static fixtures "would age into permanent STALE" — and a per-case clock
  read introduces cross-case skew that flakes exactly the at-or-over boundary
  rows the table exists to pin.
- **The fail-soft leak assertion is negative and rides the merged stream**
  (`run-usage-tests.sh:153`): the stub's `fetch failed` must *not* appear in the
  verdict output, because callers relay that line verbatim. Captured apart, it
  passes vacuously.
- **`grep -c ''` counts a final unterminated line**; a line-iterator count does
  not. The `append` column is asserted against it, and being wrong by one is
  silent.
- **`assert_count`'s patterns are anchored on two literal spaces**
  (`run-trend-tests.sh:39-40`, `^  \[5h\]`). The reporter's indentation is
  load-bearing.
- **`run-trend-tests.sh:53-56` reads `rc=$?` after an `&&`-guarded compound.**
  The transliteration is fragile even where it is correct today; the crate form
  asserts the child's own status directly rather than reproducing the idiom, and
  that is a **spelling** change with an identical verdict, not a rule change.
- **The eleven trend needles are exact golden strings.** A port that
  "improves" any of them to a pattern weakens the assertion it ports.

Everything else is incidental spelling the target language expresses directly —
`grep -qF` into `contains`, here-strings, `mktemp -d`, the `case "$axis"`
dispatch, `$(( … ))` arithmetic — criterion 7's second class, priced by the cut
at one line each.

### (9) Two validate suites fold into `native_crate`, and the coverage strengthens rather than moves

`scripts/evidence-config.sh` loses `usage_tests` and `trend_tests` from
`EVIDENCE_KIT_SUITES` and loses both `EVIDENCE_KIT_RUN_*` values
{design-bearing}. Their coverage lands in the existing `native_crate` suite
(`cargo test --release --manifest-path native/Cargo.toml`) **and** in
`check-crate-arms`, whose descriptor is `tier=precommit` and whose stated job is
that "the crate's lint and test arms run at commit time".

**So the assertions move from validate-only to every-commit-plus-validate, and
the cost was measured rather than argued**: `run-usage-tests.sh` runs its 33
cases in **0.96 s** wall clock on one Linux machine, 2026-09-02, and
`run-trend-tests.sh` is one subject invocation plus sixteen assertions. Against
a crate test arm that already runs at every commit, that is noise. Stated with a
number because "it moves a validate cost onto every commit" is the right
objection to raise and the wrong one to leave unpriced.

`.workflow/validate-baseline.txt` loses its `usage_tests` and `trend_tests`
rows. **This is stated as a requirement rather than assumed**, because no gate
catches an orphan row — `check-evidence-baseline` asserts suites-to-coverage and
has no reverse check — and the attested consequence is on disk: the cut that
retired the two guard runners left `budget_guard_tests` and
`dispatch_guard_tests` rows behind, and they are still there. Those two are
**not** this cut's to remove; they are filed to the gap inbox as their own
class, and this delta removes only the two rows it creates.

### (10) The test module is sited where `check-crate-arms`' trigger reaches it

The new module lands at the crate's **top level**, `native/src/usage_tests.rs`,
declared `#[cfg(test)]` from `main.rs` {design-bearing}. The siting is forced
rather than stylistic: `check-crate-arms`' trigger is
`staged_matches 'native/Cargo.toml' 'native/build.rs' 'native/src/*.rs'
'native/src/gates/*.rs'`, so a module under `native/src/hook/` or
`native/src/emit/` would not re-run the crate's lint and test arms on its own
edit — the gate would be silent over exactly the file it exists to hold.

That omission is a **defect in the gate's manifest, not in this cut**, and it is
filed to the gap inbox rather than fixed here: widening a `couples=` manifest
restages the generated hooks and is gate-manifest work, which the port-only run
does not admit. This delta routes around it by siting, and says so, so that the
next reader does not read the placement as arbitrary.

**Siting is the only lever, and this is the one of the iteration's three units
where it works** — stated because a build lead cutting batches needs the
contrast. A crate test module can be placed inside the trigger; a bridged arm and
a hook member cannot. `native/src/main.rs:345` dispatches every bridged arm
through a table lookup, `emit::lookup(first)`, so a new arm lands wholly under
`native/src/emit/` with no `main.rs` edit to drag a matching path into the
commit; a hook member is already registered, so the same is true of it. Both
sibling amendments this iteration therefore run
`bash gate-sdk/bin/run-gates.sh --only check-crate-arms` explicitly in their
landing commits, and each says so in its own text. **This cut needs no such
step**: the module matches `native/src/*.rs` and `main.rs` gains its
`#[cfg(test)] mod usage_tests;` declaration, so the trigger fires twice over.

### (11) Every path-bearing surface moves in the deleting commit, and four of them no gate sees

The roster is probed rather than assumed {mechanical}:

- **`scripts/evidence-config.sh:13, 25, 26`** — the suite roster and the two
  commands (delta 9). That file carries `# no-port:` and is edited, never
  ported.
- **`README.md:134-135`** — two fenced invocations inside the
  `battery-roster` marker block. `check-battery-roster` is **bidirectional**, so
  editing `evidence-config.sh` without editing these reds in both directions at
  once; the block is hand-maintained and ruled never generated.
- **`delegation-kit/README.md:99-100`** — two more fenced invocations, in the
  `## Test` block, outside `check-battery-roster`'s corpus but inside
  `check-docs-cmd` assertion A's.
- **`delegation-kit/SPEC.md:2479-2480`** — the layout tree's two `bin/` rows,
  in first-word position inside a fence, so `check-docs-cmd` reaches them.
- **`delegation-kit/SPEC.md:2743` and `:2794`** — the two **inline** citations
  in §Testing's prose. `check-docs-cmd` is blind to these: it sees only
  invocation-position paths inside fences. Leaving them is the failure the
  deferred entry `cited-script-path-liveness-inline` names by name — a green
  battery over prose naming files the same commit deleted — so they are carried
  as explicit update targets here. Fixing these four instances is **not** fixing
  that entry, whose subject is the missing gate.
- **`.claude/settings.json`** — **no edit owed, probed rather than assumed.**
  No grant names either script; the only covering entries are the globs
  `Bash(bash */bin/run-*-tests.sh)` and its `*` twin, which survive the deletion
  on `guard-kit/bin/run-guard-tests.sh` and `context-kit/bin/run-index-tests.sh`.
  So `native-gate-port-remaining-corpus`' 2026-08-29 settings-grant carve-out is
  exercised **zero times** by this cut — the probe that ruling demands, with the
  answer it did not have to have.

### (12) The regeneration fan-out this cut stales

Deleting two owed `.sh` files and editing the suite roster stales four
generated projections {mechanical}, each rostered with its trigger and regen
command in docs/site-architecture.md §Generated projections and discharged in
the landing commit:

- **The generated `pre-commit` hook**, on two independent triggers: it bakes
  `EVIDENCE_KIT_SUITES` and every `EVIDENCE_KIT_RUN_*` value, and it bakes
  `check-measured-claim`'s resolved values, whose `tree-shell-owed` key moves
  51 → 49. `check-graph` is the red, and it fires on the bare deletion alone,
  before any doc edit.
- **`docs/check-graph.html`**, asserted fresh with it.
- **`docs/enforcement.md`** (`check-enforcement-fresh`) — its validate-suite
  table carries a row per suite, and two rows leave.
- **`docs/value.md`**'s marker block (`check-value-rollup-fresh`), downstream of
  the enforcement map's per-kit counts.
- **`docs/delegation-kit/SPEC.md` and `docs/delegation-kit/README.md`**
  (`check-docs-mirror-fresh`), on the kit-surface edits above.

**`check-measured-claim` itself is clear**, and the asymmetry is worth stating
because it is the easiest finding to miss: `tree-shell-owed` moves, but no
tracked `.md` binds it with a `measured:` marker, so the movement lands on
`check-graph` through the baked hook value and nowhere else.

### (13) Three live entries this cut re-instances and does not settle

Each is named so the build meets it as a known re-instance rather than as a
discovery {mechanical}:

- **`shipped-bin-removal-deprecation-path`** — nothing obliges a session
  deleting a shipped `bin/` tool to mint a deprecation marker, so its roster
  "stays empty by construction". This cut deletes two shipped `bin/` tools and
  mints none. The behaviour it describes is taken unchanged; minting a marker
  inside a port cut would be the non-port design work the composer refuses.
- **`cited-script-path-liveness-inline`** — delta (11)'s four inline sites are
  this entry's named failure mode, repaired instance-wise and not as a class.
- **`bridged-arm-requirements-undeclared`** — inert here, because delta (2)
  declines the bridged-arm road and no arm is minted, so no spawn set goes
  undeclared. Named because a reader tracking that gap would otherwise expect
  this cut to widen it.

**`check-test-hermetic` reaches neither substrate**, before or after: its
assertion-A corpus is `gate-tests/*.test.sh` and its assertion-B corpus is
credential-managing smoke scripts, and a crate `#[cfg(test)]` module is outside
both. The ambient-strip discipline is held by prose and by the poison export on
both sides of this cut. That is unchanged rather than lost, and it is recorded
so the change is not read as having created the hole.

### (14) The parity oracle is the same cases through both substrates while both are alive

Criterion 2's demand for a non-gate member is "the same cases, both substrates,
while both implementations exist", and that is bought once, at port time
{mechanical}. Concretely: with both the shell runners and the crate tests in the
tree, drive `cases.tsv`'s 23 rows and `trend-history.log` through each and
compare the per-case verdicts and the final exit codes. This is the discharge
the sibling cut already executed and recorded — `7a4da575` ran "6 budget cases,
9 dispatch cases and all 148 guard cases" against the ported arm with both
substrates alive before deleting anything — and it carries that discharge's
standing limit: it proves the two agree then, and nothing machine-held keeps
them agreeing after, which is why the shell originals are **deleted** rather
than left running beside the crate tests.

### (15) This amendment pairs to the standing composer entry, and the host is forced rather than chosen

The host is **`native-gate-port-remaining-corpus`** {mechanical}. gate-sdk's own
§The port disposition rules it: "A cut files no per-cut residue entry: the
scoping stage promotes the standing composer entry
`native-gate-port-remaining-corpus` itself, carrying that cut's own `[spec:]`
amendment ref, and that entry states its own build-stage demotion."

**That sentence admits exactly one cut per iteration, and this iteration has
two — so the arithmetic is stated here and the gap is filed rather than
absorbed.** `[spec:]` and `[roadmap:]` are both lead-line-scoped and
`check-queue-wrap`'s floor resolves to 100, so the entry's fixed lead-line part
— `- **native-gate-port-remaining-corpus** ` plus ` [roadmap: now/reliability]`,
`[design-pending]` retiring at the promotion — is 66 columns, leaving 34. One
tag costs `9 + len(basename)`, so **one** fits at a basename up to 25 characters
and a **second** cannot at any naming, the shortest legal basename being
`SPEC-a.md` at 9 for a cost of 18 against the 16 that remain. This cut takes the
one slot; the sibling cut's host and its ground are that amendment's, stated
there. The SPEC sentence's insufficiency is filed to the gap inbox as its own
finding, because this is the **third** iteration to re-derive it and the
2026-09-01 ruling that resolved it landed in an amendment that merged and was
deleted.

### (16) The demotion re-prices the host entry, and the compression is owed in the demoting commit

`native-gate-port-remaining-corpus` returns to the deferred section under
`[design-pending]` at build — its own body says it "DEMOTES at build, never
`## Done`" — and canon-kit/SPEC.md §Merging an amendment rules that a demotion,
unlike a Done move, lands the entry **back inside `check-queue-entry-budget`'s
per-entry cap** {mechanical}. The active sections are uncapped, so the promotion
itself costs nothing; the debt falls due at the demotion.

The headroom is measured rather than estimated, and it is **zero**. The entry's
extent is `TASK-QUEUE.md` lines 19–69 = 51 lines; the count discounts **at most
one line of each declaration grammar**, and the entry carries eight `ruled:`
lines and no `recurrence:` line, so seven of the eight are counted and the
count is 50 against `QUEUE_KIT_ENTRY_LINE_CAP` = 50. **Every line the build adds
to this entry is paid for by a compression in the same commit**, and a `ruled:`
line is not free here — the discount is one per grammar, not one per line.
Stated because neither owner states it alone: the cap is queue-kit's and the
demotion is canon-kit's, and a build meeting it as a red is a batch spent
rediscovering an arithmetic this amendment already did.

## Producers and consumers

The amendment introduces **no new interface, no new state, no new event, no new
field, no new knob and no new flag.** It deletes two shell entry points and
moves their assertions into an existing test lane. That is unusual enough for
this class that it is stated first: the causal-completeness check has little
surface to bite on here precisely because the cut mints nothing.

- **Producer of the assertions** — `native/src/usage_tests.rs`, a `#[cfg(test)]`
  module, at two transitions: `cargo test` under **`check-crate-arms`** at every
  commit that stages a file its trigger matches (delta 10 is what keeps this
  module one of those files), and `cargo test --release` under the
  **`native_crate` validate suite** at every validate stage. Two producers of
  one verdict, where there was one.
- **Consumer of the verdict** — the **pre-commit battery**, which fails the
  commit on a red crate test arm; and the **validate session's evidence file**,
  through `native_crate`'s row in `.workflow/validate-baseline.txt`. Both are
  existing readers of an existing suite; neither is created here.
- **Producer of the fixtures** — unchanged. `usage-tests/cases.tsv` and
  `usage-tests/trend-history.log` stay tracked, at their paths, in their
  formats, edited by whoever adds a case, exactly as `dispatch-guard-cases.tsv`
  does today.
- **Consumer of the shell subjects** — the crate tests, by `bash` spawn, at both
  transitions above. `bin/usage-verdict.sh` and `bin/usage-trend.sh` gain a
  caller of a new kind and lose none: `native/src/hook/budget.rs` already
  spawns the first from shipped crate code.
- **Enabling configuration** — none is added, and none must be set per install.
  The tests resolve no knob (delta 6) and read no bridged environment; they
  build each child's environment themselves, which is the hermetic contract
  rather than a configuration.

**One corpus is narrowed — the tracked non-test `*.sh` tree loses two files —
and its readers' red conditions are enumerated rather than their subjects**
(canon-kit/SPEC.md §The causal-completeness check, point 5), because a narrowing
is not monotone in general:

- **`check-docs-cmd` assertion A** — reds on a repo-relative `.sh` path in
  **invocation position inside a fence** in a governed doc that resolves to no
  tracked file. **Not monotone**: the deletion *adds* violations, at three sites
  in three files. Cleared by delta (11), in the same commit, never by
  inspection.
- **`check-battery-roster`** — reds **bidirectionally**: a configured suite
  whose normalized command matches no `README.md` roster line, *or* a roster
  line matching no configured suite. Not monotone in either direction. Cleared
  by delta (11).
- **`check-graph`** — reds when the committed `pre-commit`, `commit-msg` or
  graph artifact differs byte-for-byte from its generator's `--emit` output. Not
  monotone: the deletion moves the baked `tree-shell-owed` value. Cleared by
  delta (12)'s regen.
- **`check-enforcement-fresh` / `check-value-rollup-fresh` /
  `check-docs-mirror-fresh`** — each reds on a byte difference between a
  committed projection and a live emit. Not monotone, for the same reason.
  Cleared by delta (12).
- **`check-measured-claim`** — reds when a `measured:` marker disagrees with its
  recomputed oracle, or names a key the oracle does not emit. `tree-shell-owed`
  moves 51 → 49; **no tracked marker binds that key**, checked by scanning the
  markers rather than assumed, so its finding set does not move.
- **`check-settings-paths`** — reds on a literal repo-relative `.sh` grant that
  does not resolve. Not monotone in general; **clear here**, probed: no grant
  names either file (delta 11).
- **`check-exec-bit`, `check-shellcheck`, `check-comment-tier`,
  `check-path-dialect`, `check-kit-ref-liveness`, `check-md-refs`,
  `check-knob-default-coupling`** — each reds on a property of a scanned file or
  a dangling reference, and each verdict is **monotone** in the file set here:
  removing two `.sh` files can only remove findings, `delegation-kit` stays a
  live root, both `EVIDENCE_KIT_RUN_*` knob names stay defined for the surviving
  suites, and neither file holds a knob-default site. Cleared by inspection —
  and `check-shellcheck`'s empty-target-set exit 2 is unreachable, the corpus
  spanning every kit.
- **On the addition side**, `check-comment-tier` and `check-path-dialect` both
  reach `*.rs`, so the new module is in their corpus from the commit it lands
  in; `check-path-dialect` is a newly tightened gate and the module handles
  paths.

**Cross-component signal: this amendment's component set is three** —
delegation-kit, gate-sdk (§The non-gate arm's family ruling, which delta 2
applies and does not change) and the consumer surfaces under `scripts/` — and
two **sibling** amendments land this iteration, one of them in `gate-sdk/`. So
`check-stage-entry` assertion C fires on the amendment-files-span-two-components
arm and the **align stamp is demanded at the build stage's entry**. Stated here
so the build session is not the one that learns it.

## Existing sections updated

- `delegation-kit/SPEC.md §Testing`, the `usage-verdict` runner paragraphs — the
  decision-table runner is restated as the crate test module: the same
  `cases.tsv` columns, the same sandbox-and-strip discipline expressed as an
  explicit child environment, the same poison export as a parent-process export,
  and the same beside-the-table assertions (refresh, roll witnesses,
  fan-width). Every stated honest limit survives (deltas 3, 4, 5 and 8).
- `delegation-kit/SPEC.md §Testing`, the `usage-trend` paragraph — same
  treatment for the assertion runner over the static history fixture, including
  the static-epoch property and the two fail-closed arms (deltas 3, 5 and 8).
- `delegation-kit/SPEC.md §Testing`, the retired-guard-runners paragraph — "the
  runners retired with their subjects" is widened to the rule it needs: a runner
  retires into the crate when its **cases** can be driven from there, the
  subject reached in process **or spawned**. The `dispatch-guard-cases.tsv`
  precedent gains its second and third members (delta 3).
- `delegation-kit/SPEC.md §Testing`, the fixture-exemption sentence — "No
  fixture pair owed — neither script is a gate" survives and is joined by
  criterion 2's actual discharge for a non-gate member, the both-substrates
  comparison bought once at port time (delta 14).
- `delegation-kit/SPEC.md §Layout and configuration`, the layout tree — loses
  the two `bin/` rows (delta 11).
- `gate-sdk/SPEC.md §The non-gate arm` — no ruling changes; named because delta
  (2) is that section's exit-status/`--emit-` ruling **applied**, and a reader
  checking why this cut minted no arm starts there. The class roster gains no
  member (deltas 2 and 13).
- `scripts/evidence-config.sh`, `README.md`'s battery-roster block,
  `delegation-kit/README.md`'s test block, and
  `.workflow/validate-baseline.txt` — the suite retirement and its two roster
  mirrors and one baseline pair (deltas 9 and 11).
- `docs/site-architecture.md` — no ruling changes; named because delta (12)'s
  fan-out is read off it and a reader checking that the fan-out was honoured
  starts there (delta 12).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — promoted into
  `## New Features` with `[design-pending]` swapped for this amendment's
  `[spec:]` ref. It **demotes** at build rather than reaching `## Done`, its
  deliverable being a corpus of which this is one increment, and the demotion
  re-prices it against a per-entry cap at which it already sits (deltas 15 and
  16).
- The generated projections this cut stales — the generated `pre-commit` hook,
  `docs/check-graph.html`, `docs/enforcement.md`, `docs/value.md`'s marker
  block, and the two `docs/delegation-kit/` mirrors. All are rostered with their
  triggers and regen commands in `docs/site-architecture.md` §Generated
  projections (delta 12).

<!-- update-target-exempt: three deferred entries this cut re-instances without settling; each is cited in delta 13 and none takes a write, the port-only run promoting nothing -->
- `TASK-QUEUE.md`, the `shipped-bin-removal-deprecation-path`,
  `cited-script-path-liveness-inline` and `bridged-arm-requirements-undeclared`
  entries — deliberately unwritten.

## The provenance seam

**What ships as kit mechanism:** the crate test module — the case-table walk, the
explicit child environment, the subject spawn and the assertion set. All generic.

**What stays the consumer's:** the two fixtures. `usage-tests/cases.tsv` and
`usage-tests/trend-history.log` stay **on disk, tracked, at their paths, in their
formats**, and are not transcribed into Rust literals — the disposition §Testing
already took for `dispatch-guard-cases.tsv` on the ground that "moving it into
Rust literals would trade a reviewable table for a recompile". The seam here is
kit test data rather than consumer config, and the rule that keeps it out of the
binary is the same one: a table a reader can edit does not become a compiled
literal because its reader changed language.

**What becomes consumer config:** nothing new, and pointedly so. Delta (6) is the
seam held: the declared knob roster is empty because a declared knob would let a
consumer's tree config reshape a table that must encode the kit defaults. The
knobs themselves stay exactly where they are, in
`delegation-kit/lib/delegation.sh`, which is permanently shell as the config
bridge's sole resolver for this kit.

**What is not crossed:** no term list, no vocabulary and no product constant
enters the crate. The only literals the module carries are the two that assert
kit defaults, and delta (6) rules they stay literals rather than being read from
the library.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), the none-remain half discharged
      at the **iteration** rather than at the commit, this iteration carrying a
      sibling `delegation-kit` amendment.
- [ ] **Removals propagated** — grepped every spec, skill, template, README,
      config file, settings file and baseline for the two deleted paths and for
      the two retired suite names; nothing dangles, the four **inline**
      citations delta (11) names included.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm lists no `delegation-kit/bin/run-usage-tests.sh` and no
      `delegation-kit/bin/run-trend-tests.sh` row, taken as a per-file roster
      diff and not as a trailer delta.
- [ ] **Parity was executed with both substrates alive** — the 23 table rows,
      the beside-the-table assertions and the trend fixture were driven through
      both implementations and their verdicts and exit codes compared, *before*
      either shell file was deleted.
- [ ] **The poison export is a parent-process export** — proved by breaking the
      strip in a scratch run and observing the table fail, not by reading the
      code. A vacuous hermeticity proof is the one failure this cut can ship
      green.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hook, the graph artifact, the enforcement map, the value rollup,
      both `docs/delegation-kit/` mirrors and the gate binary.
- [ ] **The baseline rows this cut creates are removed** — `usage_tests` and
      `trend_tests` are gone from `.workflow/validate-baseline.txt`, checked by
      reading the file rather than by trusting the absence of a red, since no
      gate catches an orphan row.
- [ ] **The host entry demotes inside its cap** — `native-gate-port-remaining-corpus`
      returns to `## Deferred` under `[design-pending]` and its counted size is
      at or under `QUEUE_KIT_ENTRY_LINE_CAP` **in the demoting commit**, spent
      narrative compressed in place, the entry having zero headroom today.
