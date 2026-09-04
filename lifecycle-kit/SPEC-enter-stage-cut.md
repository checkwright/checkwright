# SPEC amendment: enter-stage-cut

The port disposition of **`lifecycle-kit/bin/enter-stage.sh` (641 lines), the one
owed file declaring §bin/enter-stage.sh**, onto the binary substrate as a bridged
`Arm::Run` with two reachable callers. This is a stated-contract cut under the
port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and ruled by
the **lead on its own authority**, 2026-09-04, over the resume channel; it did not
reach the operator, and that is stated because a composition ruling recorded
without its authority reads at the post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port
oracle's `--tree` arm reads 99 files scanned, 64 declared `no-port`, 0 temporarily
held, **35 owed**. This cut takes one of that column, and **no ported member is a
gate** — §bin/enter-stage.sh rules the tool "advisory tooling, not a gate: no
fixture pair is owed", so this member's whole port surface is the non-gate class.

**The authorization is the build-window sizing ruling and not a by-construction
argument**, stated first because the survey that composed this cut reached for the
other one and was corrected. TRAJECTORY.md's 2026-09-03 operator ruling — *a port
cut is sized to fill one build window, never to a unit count* — refuses a
per-iteration unit-count target outright, so a one-file cut owes a **window
argument** and owes no defence of its cardinality. That the takeable column holds
ten singletons and that no queue entry hosts any of them are true and are
**description**; neither licenses anything, and a later selector reading them as
grounds would be inheriting a corrected reading.

**The window argument, in the terms the ruling measures.** At 641 lines this file
is the **largest owed file in the whole tree** — the next is 464 — and 41 per cent
of the takeable column (1546 owed takeable lines across ten files). Taking it
moves the owed corpus from 35 files / 4871 lines to 34 / 4230: **13 per cent of
the tree's remaining owed shell in one cut**, which no other takeable member comes
near. Its fan-out is the second half of the argument and it is measured rather
than estimated: `git grep -n enter-stage` returns **539 tracked matches**, of
which the work is 80 self-references inside its own SPEC section's neighbourhood,
seven hermetic harnesses that execute the file end to end, five call sites in the
kit's consumer smoke, two compiled surfaces holding its name as a string literal,
one settings grant, 32 queue lines and a hand-fix set no gate reaches. A second
member added beside it would be amortized against nothing: no owning section holds
a second owed member anywhere in the takeable set, so a wider cut buys a second
independent walk rather than a shared one. The window is filled by this file's own
fan-out, which is the condition the budget arm states.

**What affirmatively selects it.** lifecycle-kit/SPEC.md says twice — at `:842-843`
that `bin/enter-stage.sh` "ports in a cut of its own", and at `:2178-2181` that
"this file declares §bin/enter-stage.sh rather than that section and so **ports in
a different cut**, and a later cut selector meets that fact here, where it works".
It is the only takeable file any governed section names for a cut of its own.

## What changes

### (1) The cut is the one owed file declaring §bin/enter-stage.sh, and taking it discharges the section

`bin/enter-stage.sh` is the **only** owed file declaring `### bin/enter-stage.sh`
{design-bearing}. Probed rather than assumed: the kit's other shell members are
`lib/stages.sh` (251), `smoke/install.sh` (488), `smoke/violation.sh` (9) and
`templates/lifecycle-config.sh` (3), and the oracle reads **every one of them
`no-port`** — so this cut takes the kit's whole owed column to zero in one motion,
which no prior cut in this corpus has done. The stated-contract composer's *one
section, the one amendment* is satisfied by a clean singleton.

**The discharge is the section's and not the kit's, and the difference is worth a
sentence because here they coincide.** §bin/enter-stage.sh records itself
discharged, and — uniquely so far — the kit has no second owed member to name as
remainder. What remains in `lifecycle-kit/` after this cut is permanently shell by
declared cause: `lib/stages.sh` is the config bridge's sole `LIFECYCLE_KIT_*`
resolver (gate-sdk/SPEC.md §The kit-library port disposition), and the two smoke
members and the config template are the classes the oracle's corpus rule already
excludes from the port.

### (2) `--enter-stage [--simulate] <stage> | [--simulate] --rename <name>` — a bridged `Arm::Run`

The tool lands as `native/src/emit/enter_stage.rs` and registers as one
`BRIDGED_ARMS` row, `("--enter-stage", Arm::Run(crate::emit::enter_stage::run),
crate::emit::enter_stage::KNOBS)`, with its own `case` arm in
`gate-sdk/bin/run-gates.sh` beside `--install-lifecycle` {design-bearing}.

**`Arm::Run` and not `Arm::Emit`, on the arithmetic.** `native/src/main.rs:405-413`
maps `Arm::Emit` onto `exit(0)` for `Ok` and `exit(2)` for `Err`, so that family
collapses to `{0, 2}` and can never return 1. This tool's exit contract is
**three-state and every code is load-bearing**: 0 a stamp or a reported no-op,
**1 a refusal** (`check-stage-entry`, a `LIFECYCLE_KIT_ENTRY_PREFLIGHT` command,
the predecessor-journal assertion, and each of the four boundary refusals), 2 a
usage or configuration error. An `Emit` spelling would rewrite every refusal to
the misuse code, and the whole state machine reads that difference: the stage
template's first step rules "on a refusal, **do not force the entry** — escalate",
which is advice a session cannot follow if a refusal is indistinguishable from a
typo. It also emits no document at all, which is the second half of the
`--wait-probe` precedent for taking a spelling of its own.

**The precedent for its argument shape is `--install-lifecycle`, in this same
kit**: a bridged arm outside the `--emit-` family whose "contract is an action
with an exit status rather than a document" (`run-gates.sh:188-190`). `--simulate`
and `--rename` ride as operands exactly as they do today, so the argv grammar
crosses the seam **unchanged** and no caller's spelling is re-taught beyond the
program word.

**The status survives the front-end verbatim**, on the mechanism already probed
for `--usage-verdict` and `--wait-probe`: `exec_arm` ends in `exec`
(`run-gates.sh:118`), which replaces the shell process image, so the binary's
status *becomes* the front-end's. `ARM_UNAVAILABLE_STATUS` is reachable only on
the two pre-dispatch failure paths, and delta (11) rules what it must be here.

### (3) The declared roster is the prefix family `LIFECYCLE_KIT_*`, and that is a derivation rather than a transcription

The arm's `KNOBS` is `&["LIFECYCLE_KIT_*", "GATE_SDK_TMP_DIR"]` {design-bearing}.
The family form is not a convenience: this arm must resolve **two** knob sets, its
own fourteen and the declared rosters of the two gates it dispatches, and a
transcribed union of the three is a maintained copy that drifts the first time
either gate gains a knob.

**Probed rather than assumed.** `native/src/gates/mod.rs:972-1028` gives
`check-stage-entry` thirteen declared names and `check-stage-evidence` eleven,
and **all but one are `LIFECYCLE_KIT_*`** — including the associative
`LIFECYCLE_KIT_PREDECESSOR` and two scan-scoped `(".", KNOB)` rows. The one
exception is `check-stage-entry`'s own `GATE_PRUNE_DIRS`, resolved at that
gate's own child dispatch (`runner.rs`'s `child_knobs`, delta 6) rather than
through this arm's bridged environment, so it costs the family claim nothing
this arm itself must carry. So one family covers what the arm needs to bridge,
with nothing outside it and nothing inside it unused by somebody.

**The bridge already carries the family and the array shapes.** `gate-sdk/lib/gate.sh:241`
rules that "the trailing `*` selects the prefix family", resolved by `compgen`
inside the owning kit's already-sourced subshell; `:261-277` serializes an indexed
array as one tab-joined `GATE_SDK_KNOB_<NAME>` element and `_gate_knob_pairs`
serializes an associative one as `<key>=<value>` pairs. `native/src/walk.rs`'s
`knob_array` is the receiving half, and `native/src/stages.rs:7` already calls it
for `LIFECYCLE_KIT_STAGES`. So the five array knobs this tool reads
(`STAGES`, `BOUNDARY_TRUNCATE`, `BOUNDARY_PRESERVE`, `BOUNDARY_REQUIRE`,
`ENTRY_PREFLIGHT`) cross on shipped mechanism and this cut mints no serialization.

**It is the family case and not the sentinel case, and gate-sdk/SPEC.md:2662-2674
is the paragraph that tells them apart** — recorded because the two look
interchangeable. `EVERY_REGISTERED_KNOB` expands over *the tree's registry*, scoped
by the arm's own `--gates-dir` argv; this arm has no `--gates-dir` in its grammar
and dispatches two **named** gates in one kit, so the sentinel would under-report
to nothing while the family answers exactly. This is the third instance of the
prefix family after `EVIDENCE_KIT_RUN_*` and `DRIFT_KIT_*`, and the first where
the family is a *whole kit's* namespace rather than a sub-family inside one.

**`GATE_SDK_TMP_DIR` rides beside it** because the scratch dir is the temp state
file's home, the boundary wipe's subject, and the resume journal's parent; it is a
`GATE_SDK_*` name outside the family and is declared explicitly. Its literal
default site leaves this file and eleven other tracked shell holders survive, so
`check-knob-default-coupling` loses a comparison and never a subject.

### (4) Two reachable callers, and the second one is what keeps every hermetic assertion true

This is the cut's central seam ruling {design-bearing}. The front-end is the
**session's** spelling; the binary reached through `gate_native_bin` is the
**hermetic harness's**, and both dispatch one implementation.

**The forcing fact, probed by running it rather than read off the source.**
`gate-sdk/bin/run-gates.sh:13` is `cd "$(git rev-parse --show-toplevel 2>/dev/null)"`
with a refusing `||` arm. Run from a non-git `mktemp -d`, the front-end exits
**2** with `run-gates: not inside a git repository` — bash's `cd ""` returns 1
("cd: null directory"), so the refusal arm fires rather than the cd silently
no-opping. That is the whole hazard, and it is a hazard because **seven hermetic
harnesses drive this tool inside a non-git `mktemp -d` and none of them `git
init`s**: `boundary-scratch-wipe`, `boundary-stage-journal`,
`boundary-worktree-refusal`, `gap-inbox-route`, `preflight-valve`,
`rename-iteration` and `survey-record-entry`, all `.test.sh` under
`lifecycle-kit/gate-tests/`. `boundary-worktree-refusal.test.sh:14` says
"(no git init)" in so many words, and its `# spec:` line asserts
"**a non-git tree skips the check rather than failing on it**" as a named case.

**So the front-end alone is refused, and the refusal is a parity ruling rather
than a convenience.** A port may not change a verdict across the seam; routing
every caller through the front-end would convert a supported non-git invocation
into a configuration error, which is a behaviour change no gate in the battery
would report.

**The second caller is sanctioned, not invented.** gate-sdk/SPEC.md §The non-gate
arm sanctions a **caller** reaching the binary and forbids only a second *entry
point into the emission path* — the sanction `enter-stage.sh:290` already invokes
for `--emit-session-id` today, from this very file. Three shipped facts make the
harness migration a known shape rather than a new one:

- `gate-sdk/lib/test-hermetic.sh:14` already exports `GATE_SDK_NATIVE_BIN`
  **absolute**, with the recorded reason "a bespoke test runs its gate from a
  sandbox cwd where the knob's repo-relative default resolves to nothing" — which
  is this case verbatim.
- Six bespoke tests already resolve `BIN="$(gate_native_bin)"` and drive the
  binary directly: `context-kit/gate-tests/check-settings-pins.test.sh:16` and
  `toolfloor-parity.test.sh:31`, `evidence-kit/gate-tests/evidence-lib-parity.test.sh:34`,
  `guard-kit/gate-tests/guard-lib-parity.test.sh:36` and `scan-prompts.test.sh:15`,
  `queue-kit/gate-tests/queue-lib-parity.test.sh:35`.
- `gate_knob_env <arm>` (`gate-sdk/lib/gate.sh:377`) builds the bridged environment
  for a **non-gate arm** specifically, and `gate_knob_env_one` (`:431`) is its
  arity-one face "for a harness resolving a single knob". The harness needs no new
  helper and this cut mints none.

**The refused alternative is widening the front-end's `cd`**, and it is refused on
its blast radius: gate-sdk/SPEC.md:237-241 rules that "Paths are repo-root-relative;
**every entry point** `cd`s to `git rev-parse --show-toplevel` before resolving
them", so a conditional cd would weaken the precondition every gate in the battery
rests on to accommodate one arm's test harness. The second caller costs one line
per harness and nothing outside them.

### (5) The cwd widening the port brings, stated rather than shipped silently

Through the front-end the process cwd becomes the git toplevel before the arm
runs, and the cwd is an **input** {design-bearing}.
`native/src/emit/session_id.rs:47-57` derives source 3's sessions dir as
`<config-home>/projects/<slug(cwd)>` when `LIFECYCLE_KIT_SESSIONS_DIR` is unset.

Three cases, each stated because only the second is a change anyone will notice:

- **At the repo root** — cwd equals the toplevel, so the derivation is
  byte-identical and nothing moves. This is every stage session's case.
- **From a subdirectory** — today the tool **fails**, its knob paths being
  repo-relative (`enter-stage: queue file not found: TASK-QUEUE.md`); after the
  port it **succeeds**, and derives the session id off the toplevel slug, which is
  the project directory the harness actually keys transcripts on. That is a
  widening in the safe direction, and gate-sdk/SPEC.md:237-241 is the governed
  sentence the port brings this tool into compliance with rather than a licence
  the port invents.
- **Through the second caller** — no `cd` happens at all, so the harness's cwd is
  its sandbox exactly as today, and every harness pins `LIFECYCLE_KIT_SESSION_ID`
  anyway (`boundary-worktree-refusal.test.sh:55`), so source 1 short-circuits the
  derivation before the cwd is read.

**`enter-stage.sh:290`'s `# spec:` comment does not survive the port**, and that
is this delta's concrete deliverable. Its whole ground is that the front-end cds
and the arm must therefore be reached around it; after the port the arm *is*
behind that front-end for its session caller, so the comment is rewritten to state
the new fact — the session id is derived **in process** at the toplevel, and the
one caller that needs the untouched cwd is the hermetic harness, which does not go
through the front-end at all. A comment kept verbatim here would assert a
mechanism the port deleted.

### (6) The two pre-flight gates keep being dispatched, never called in process

§bin/enter-stage.sh rules that both arms "name a gate and never a substrate":
`check-stage-entry` for the stamp path and `check-stage-evidence` for `--rename`,
each resolved through `gate_command` rather than by script path, with an argv the
bridge refused to build taken as exit 2 {design-bearing}. That contract survives
the port **on mechanism the crate already ships**, and the tempting shortcut is
forbidden rather than merely dispreferred.

`native/src/runner.rs:258-302` is the compiled `gate_command`: `registry::resolve`
finds the member, a `.gate` becomes the argv `[self_exe, name]` with
`child_knobs(declared, union)`, and anything else is the resolved `.sh` path
verbatim; `native/src/proc.rs:593` spawns it and returns the child's status and
merged output. **The in-process call is refused where that code stands** —
`runner.rs:256-257` records the ground as "the declared-knob discipline, fault
isolation and the surviving `.sh` members" — so the ported arm calls
`stage_entry::run` **through a child process** and never as a function, however
much cheaper the direct call looks from inside one binary.

Two consequences follow and are stated because a port is where they get lost. A
consumer shadowing either gate with its own `.sh` still shadows it, which is the
resolution order gate-sdk/SPEC.md §Layout and configuration owns. And the
`${#argv[@]} -eq 0` refusal keeps a compiled twin: a member that resolves in no
gates dir is exit 2 with the dispatcher's own diagnostic, never an entry
pre-flighted by a check that did not run.

### (7) `LIFECYCLE_KIT_ENTRY_PREFLIGHT` keeps exec'ing consumer argv with no interpreter word

Each matching entry's `<command>` is split on whitespace and exec'd as argv with
**no interpreter word prepended**, so the configured path rides its own exec bit
{design-bearing}. That sentence is load-bearing in this tree rather than generic:
`scripts/lifecycle-config.sh:14` records that "enter-stage.sh execs the configured
argv with no interpreter word, so a ported member's `.gate` descriptor is a
non-executable data file and the entry is refused", which is why all nine of this
repo's wired entries route through `scripts/gate-exec.sh`.

The compiled arm therefore spawns each entry with `Command::new(argv[0])` and no
shell, and **inherits its own environment** to the child — including the bridged
`GATE_SDK_KNOB_*` set, which is what lets `scripts/gate-exec.sh` resolve the gate
it names. Stated as a delta because a compiled arm that built a clean environment
for its children would break all nine entries at once and no fixture would see it:
these commands run only at a real stage entry.

The valve's whole reach is this arm and no other, unchanged: the ledger is read
only where a `LIFECYCLE_KIT_ENTRY_PREFLIGHT` command refuses, so a malformed
ledger cannot wedge an entry that never needed a valve.

### (8) The worktree scan's liveness predicate is `evidence::pid_alive`, and the knob still buys the dependency

`ek_pid_alive` already has a compiled twin, `native/src/evidence.rs:132`, carrying
the same pid grammar and the same fail-closed reading of an unanswerable probe
{design-bearing}. So the ported scan calls it directly and the classification
table crosses unchanged.

**What changes is the shape of the dependency, and the change is a narrowing.**
Today the tool sources `evidence-kit/lib/evidence.sh` **only when
`LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` is non-empty**, so an unconfigured consumer
owes no second vendored kit, and a pattern set with the library unreachable is
exit 2. In the crate the predicate is already linked, so the conditional source
disappears and with it the exit-2 arm — a refusal that can no longer occur because
its cause cannot occur. §bin/enter-stage.sh's paragraph is rewritten to say so
rather than left asserting a fail-closed branch the binary has no way to take.

**The consumer-side half is untouched and must be, on the provenance seam.**
`LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` keeps its **empty** kit default and this
repo's pattern stays in `scripts/lifecycle-config.sh:35`, which is declared
`no-port` on the operator-ruled provenance-seam class. A compiled default spelling
one harness's lock vocabulary would publish it, which is exactly the seam
CLAUDE.md §The provenance seam holds. The regex is a POSIX ERE with one capture
group and the crate's engine must accept the shipped pattern; that the tree's own
pattern parses under the compiled matcher is a Definition-of-Done item rather than
an assumption, because a regex-dialect mismatch here silently reclassifies every
worktree as `unclassified` and turns a refusal into a pass.

### (9) The boundary wipe, the truncate and the journal opener move with their pinned orderings

Three writes ride the boundary and their **order is contract**, not implementation
{design-bearing}: the truncate rewrites tracked members down to their header, the
scratch wipe deletes untracked members outright and runs **last** so this run's own
temporaries are never candidates, and the journal opener runs **after the wipe**
so its skeleton is not silently deleted by it.

Each carries a spelling the port could lose:

- **The header run stops at a markdown `## ` heading as well as at the first data
  line.** §bin/enter-stage.sh rules that half load-bearing: on the survey record,
  whose blocks *are* `## ` headings, a bare "keep every leading `#`" rule carries
  one stale survey across the boundary and the record's own read trigger then
  advertises it. The awk at `:572` also holds blanks **pending** and flushes them
  only behind a following header line, so the retained blank run does not grow by
  one per boundary. Both behaviours are compiled and both are asserted.
- **The wipe's keep-list matches a basename at any depth**, which is the tool's
  current behaviour and a filed defect (`boundary-wipe-preserve-basename-reach`,
  recurrence-stamped 2026-09-04). The port **preserves it exactly**: fixing it
  inside a port cut would change a verdict across the seam, and the entry that owns
  the fix is not this cut's. Named so a build session does not take the port as
  licence to correct it.
- **`.gitkeep` stays a kit invariant the consumer cannot unset**, and shipping it
  as `LIFECYCLE_KIT_BOUNDARY_PRESERVE`'s default stays ruled out — a defaulted
  array is *replaced* on assignment, so protection would decrease as configuration
  increases. In the crate the invariant is a `const` outside the knob, which
  expresses the same ruling more directly than a bash default could.
- **A wipe delete that fails because a preserved basename sits inside a doomed
  subdirectory is noise, never an abort**, so the compiled walk swallows that one
  error class and no other.

### (10) `--rename`'s columns-2-to-last witness reads to end of line and is compiled as such

The witness compares fields 2 through `NF` of every data line before and after,
and it reads to the end of the line rather than to a pinned column {design-bearing}.
§bin/enter-stage.sh records why an explicit column-5 check was refused: it
re-hardcodes the arity the original gap came from, and a field riding *outside* a
pinned witness could be dropped with neither the tool nor its test noticing —
which is what the four-field spelling did to `<head>` the moment that field landed.

A compiled rewrite is where a pinned arity gets reintroduced by accident, because
a struct with five named fields is the obvious shape and it is the wrong one. The
arm therefore keeps a **positional** representation: field 1 is rewritten, fields
2..n are joined and compared as one string, and no code path names the field count.
The refusal on a mismatch stays exit 2 with nothing written.

### (11) `ARM_UNAVAILABLE_STATUS` is 2 here, and the reason is the bootstrap it creates

An absent or unbuildable binary is exit 2 for this arm, joining `--install-lifecycle`,
`--usage-poll`, `--usage-verdict`, `--lesson-sink`, `--upgrade-smoke` and
`--wait-probe`, and against the `--hook` and `--statusline` decline-with-0 posture
{design-bearing}. The caller is a stage session's first step whose failure must be
visible, and a silent 0 would let a session believe it had stamped.

**The bootstrap this cut creates is the honest cost and is stated here rather than
discovered.** Today an absent binary is already fatal to this tool — it needs the
`--emit-session-id` arm and refuses with a build instruction at `:293` — so the
*dependency* is not new. What is new is that **stage motion itself now has no
shell path at all**: before the port a consumer with a broken binary could still
be told, by this tool, how to fix it; after it, the same consumer gets the
front-end's generic unavailable diagnostic. The mitigation is that the diagnostic
already names the fix (`Build it: bash gate-sdk/bin/build-native.sh`,
`run-gates.sh:113`), and the amendment's obligation is to keep that line reachable
rather than to invent a second one.

### (12) Criterion 2's discharge is the both-substrates comparison, bought once, before the delete

This member ships no `good/`+`bad/` fixture pair and owes none — it is not a gate,
which §bin/enter-stage.sh already states {design-bearing}. What stands in its place
is the both-substrates comparison, and this member's is unusually cheap to buy
because **the seven hermetic harnesses already are the corpus**.

The procedure, stated so build does not invent one. In one session, with both
implementations present, run each of the seven harnesses against each
implementation and compare, per case:

- the **exit status**, which is the three-state contract delta (2) preserves;
- the **stdout and stderr text**, byte for byte, including every `help:` line and
  every `enter-stage (simulate): ` prefix — the simulate prefix is asserted by
  `rename-iteration.test.sh:125` and `:135` with a `grep -qv`, so a single
  unprefixed line is a real defect the comparison must catch;
- the **written files** — `.workflow/WORKFLOW-STATE.txt`, `TASK-QUEUE.md`'s header
  line, each truncated member, the valve ledger, and the opened journal — byte for
  byte, and the **unwritten** ones proved unchanged on every refusal path;
- the **wiped set**, as the report names it, over a scratch tree seeded with a
  nested `.gitkeep` so the at-any-depth reach of delta (9) is compared rather than
  reasoned about.

**One field is not compared for equality and saying so is the honest half**: the
`<head>` column is read live and the `<session-id>` column is derived, so both are
pinned by the harnesses (`LIFECYCLE_KIT_SESSION_ID=deadbeef01`) or fall to `none`.
Where a case does not pin them, what is compared is the **shape** — eight hex
characters, a short sha or the literal `none` — and not the value.

**The live-tree arm is available and is owed on top**, unlike the previous cut in
this corpus: this repo is a real consumer with a real queue, so a `--simulate` of
every configured stage under both implementations is a second comparison bought at
no risk, `--simulate` writing nothing by contract.

### (13) The criterion-5 residual, and it is the widest in this corpus so far

Criterion 5 asks what a consumer whose payload carries no artifact for its host
still has after the cut {design-bearing}. This cut registers no gate, so the
binary-less leg's omitted-member roster and its count do not move.

**What such a consumer loses is the ability to run the lifecycle at all**, and
that is a wider loss than any prior cut in this corpus has taken. The tool is the
first step of all six stage templates and the sole writer of the stamp that is the
cursor, so a host with no artifact cannot enter a stage, cannot cross an iteration
boundary and cannot rename an iteration. Named plainly rather than softened,
because the honest reading is that this member's port raises the cost of the
unported-platform gap rather than being neutral to it, and the entry that owns
that gap is `powershell-installer-surface` — the sequence's last member — not
anything this cut could discharge.

**The mitigation that does exist is stated with it.** The loss is total but it is
also *loud and immediate*: the front-end's unavailable diagnostic names the build
command, and the failure lands at the session's first step rather than midway
through work. A consumer on an unported platform was already unable to run the
battery; what changes is that they can no longer run the lifecycle's bookkeeping
either, which removes a partial-adoption path this corpus had not previously
priced.

### (14) The lifecycle lib-parity harness this cut owes

`native/src/stages.rs` is `lib/stages.sh`'s Rust counterpart and **nothing
compares them** {design-bearing}. Probed: every other kit whose library is
permanently shell ships a parity harness — `evidence-lib-parity.test.sh`,
`queue-lib-parity.test.sh`, `guard-lib-parity.test.sh`,
`context-kit/gate-tests/toolfloor-parity.test.sh` — and `lifecycle-kit/gate-tests/`
holds no `*parity*` member at all.

The cut is what makes this owed rather than merely absent. `lib/stages.sh` stays
shell by declared cause, and the port moves this tool's readers of it — the header
parse and its iteration extraction, the stage-membership test, the cursor read, the
journal path derivation, the journal-written predicate and the journal opener —
onto the crate's side of a seam with no comparator across it. The two spellings of
the opening-line shape are the sharpest case: §bin/enter-stage.sh rules that writer
and reader "share one spelling of that line's shape in `lib/stages.sh` **so writer
and reader cannot drift**", and after this cut the writer is compiled and the
reader is not.

So `lifecycle-kit/gate-tests/stages-lib-parity.test.sh` lands **with** the port,
built on the four existing harnesses' shape: a table of inputs run through both the
sourced shell function and the binary's arm, compared byte for byte.

### (15) A compiled gate holds this path as a string literal, and that is the one un-gated regression this cut can ship

`native/src/gates/stage_skill_coverage.rs:20-46` detects "a stage skill's executed
surface invokes the stamp tool" by scanning every `.md` under
`LIFECYCLE_KIT_SKILLS_DIR` for the **literal** `enter-stage.sh` followed by a
whitespace-separated stage word, then cross-checking that word against the live
stage set {design-bearing}. Its orphan-skill half at `:104-112` rests on the same
match.

**Nothing else holds this coupling and nothing would report its loss.** Retire the
string from the six command files without moving the Rust literal in the same
commit and the gate keeps printing `STAGE-SKILL-COVERAGE: clean` on a corpus its
pattern can no longer match — a green gate asserting nothing, which is the exact
failure class gate-sdk's fail-closed contract exists against and the one shape a
`--for` selection cannot surface, the gate coupling to the *skills* directory
rather than to this path. It is a delta rather than a line in the roster below
because it is the only surface in the whole fan-out whose breakage is **silent and
functional** rather than silent and cosmetic.

The literal becomes the arm's invocation form and the pattern's trailing-token
rule is unchanged, so the gate's contract is untouched and only its subject moves.
Its own unit tests at `:178-187` and its `good/`+`bad/` fixture pair move with it,
which is what keeps the change from being self-certifying.

**A second compiled surface holds the path as user-facing text**, and it is named
here rather than in the roster for the same reason at lower severity:
`native/src/hook/workflow_state.rs:2,35,66` is the `PreToolUse` guard that refuses
a hand-edit of the state file, and its refusal **tells the session to run
`bash lifecycle-kit/bin/enter-stage.sh <stage>`**. Left unmoved it coaches every
blocked session toward a path that no longer exists — the guard's whole value being
that its refusal names the sanctioned writer.

### (16) The four deferred entries filed against this file each get a stated disposition

`TASK-QUEUE.md` carries four `[design-pending]` entries whose subject is a defect
in the file this cut deletes, and the amendment rules each rather than leaving a
build session to infer one from the rewrite {design-bearing}:

- **`enter-stage-flag-position-silently-ignored`** (`:562`, whose own body marks
  itself "DISTINCT from every entry about what the entry gate ASSERTS") and
  **`enter-stage-arg-position-silent-drop`** (`:2873`) — both are argv-grammar
  defects of the shell parser. They are **carried forward, not fixed here**: the
  port preserves the argv grammar by delta (2), so both defects survive the seam
  intact and their entries stay live against the arm. Fixing either inside a
  port cut would change a verdict across the seam, which is the discipline the
  whole corpus runs on. Each entry's body is repointed at the arm in the
  deleting commit.
- **`enter-stage-refusal-help-contradicts-its-guard`** (`:7740`) — a text defect in
  a refusal's `help:` line, unchanged by the substrate and carried forward on the
  same ground.
- **`enter-stage-simulate-no-write-fixture`** (icebox, `:9711`, cited again at
  `:2904` and `:6557`) — **stays iceboxed, and delta (12) does not discharge it.**
  The tempting reading is the other one and it is refused here rather than left for
  build to take: delta (12)'s comparison does assert, on every refusal path of all
  seven harnesses, that nothing was written — but it is **bought once, before the
  delete**, which its own heading says. **A one-shot parity comparison is not a
  fixture.** The entry's whole text is "Guard present, **unpinned by a fixture**",
  which asks for something that runs *again*; the comparison is consumed by the
  delete and leaves the guard exactly as unpinned the day after as it is today.
  Moving the entry to `## Done` on that ground would record work as done that was
  not done.
  **The door this cut does open is narrow and is stated so build can walk through
  it deliberately.** If build lands a **standing** no-write assertion on the
  refusal paths — a test in the ported crate's own suite that runs in the battery,
  not the one-off comparison — the entry is discharged and build moves it, naming
  the test that pins it. Absent that test it stays iceboxed and joins the other
  three as carried forward.

**Two queue lines cite hard line numbers inside the deleted file** and dangle
whatever else happens: `:6058` names `enter-stage.sh:339` and `:9670` names
`:469,:499`. Both are repointed at the section rather than at a line, since a line
citation into a compiled module is the same defect one substrate later.

### (17) Every remaining path-bearing surface moves in the deleting commit

The roster is probed rather than assumed and split by what a gate catches
{mechanical}:

- **The one surface `check-docs-cmd` reds on** — `lifecycle-kit/README.md:117`, a
  fenced `bash lifecycle-kit/bin/enter-stage.sh <stage>`. Measured against that
  gate's actual corpus rule (canon-kit/SPEC.md §check-docs-cmd: the governed
  manifest set, fenced invocation position only), which is why it is **one** line
  and not the dozens an inline-backtick count would suggest: every
  `*/templates/*.md` hit is slot-bearing and therefore outside the manifest set.
- **The silent set, named because no gate reaches it.** An inline-backtick path is
  outside `check-docs-cmd`'s corpus even inside the manifest set, so all of these
  are hand fixes: `CLAUDE.md:124,126`; `lifecycle-kit/README.md:15,101,126,131`;
  ~80 lines in `lifecycle-kit/SPEC.md`; `evidence-kit/SPEC.md` (seven lines);
  `delegation-kit/SPEC.md` (five); `gate-sdk/SPEC.md:839,4879`;
  `queue-kit/SPEC.md:583`'s `close-surface:` grammar line; `RELEASING.md:21`;
  `.claude/commands/scope.md:11`; the six stage templates plus `lead.md` and
  `consult.md`; `docs/orchestration.md`; and — a **fenced** invocation that no gate
  still reaches, because the page is hand-authored and outside the manifest set —
  `docs/lifecycle-kit/index.md:28`.
- **Two `.workflow/` contract headers** — `preflight-valve.txt:1` and
  `release-disposition.txt:1` each open `# contract: lifecycle-kit/SPEC.md
  §bin/enter-stage.sh`. Nothing validates that line, so they are safe only because
  delta (17)'s heading rule keeps the section name; recorded so a later heading
  rename knows they exist.
- **Five `docs/posts/*.md` release notes name the path and are NOT touched** —
  `2026-07-17`, `2026-07-18`, `2026-07-19`, `2026-07-22` and `2026-08-21`. They are
  dated provenance describing the tool as it stood at each release, and a sweep
  that "fixed" them would rewrite history. Named explicitly because a mechanical
  grep-and-replace reaches them and a careful one must not.
- **`.claude/settings.json:29`** — the one grant, delta (18)'s.
- **The seven hermetic harnesses** — each moves from `bash "$ENTER"` to the
  binary-plus-bridged-env form of delta (4).
- **`lifecycle-kit/smoke/install.sh`** — five call sites (`:115`, `:180`, `:225`,
  `:405`, `:467`) driving the boundary reset, the pre-flight, the valve, the
  boundary-require branches and gap-inbox routing, all moving to the front-end
  spelling. Its `for t in enter-stage.sh` help-coverage loop at `:261-269` and the
  `av`/`av_run` scaffold at `:240-252` become **dead code**: that loop already
  shrank once when `file-gap.sh` ported out, its own comment says so, and this cut
  empties it. The `-h`/`--help` coverage folds into the ported-arm pattern the
  file already uses. It is `no-port` itself and is edited, not ported.
- **`lifecycle-kit/SPEC.md §bin/enter-stage.sh`** — rewritten as the arm's section
  per deltas (1) through (16). **The heading stays**, and that is load-bearing on a
  measured asymmetry: `check-kit-ref-liveness` backstops the ~80 `§bin/enter-stage.sh`
  pointers *inside* the manifest set, and at least six citations outside it
  (`RELEASING.md`, both `.workflow/` headers, `.claude/commands/scope.md`, several
  queue lines, `docs/lifecycle-kit/index.md`) have **no** gate behind them. Keeping
  the heading is therefore not a preference — half the pointer corpus would go
  stale silently under a rename.
- **`drift-kit/bin/overhead-meter.sh:70` needs no edit, probed** — its awk
  classifier matches the bare substring `enter-stage`, which `--enter-stage`
  contains, so the meter keeps classifying these lines as stage cost.
- **`gate-sdk/SPEC.md §The non-gate arm`** — the class roster gains
  `--enter-stage`, recorded as the first member with **two sanctioned callers** and
  the first to declare a whole kit's prefix family (deltas 2, 3, 4).
- **`gate-sdk/bin/run-gates.sh`** — the new `case` arm and its `usage()` line.
- **The `docs/` mirrors** of every edited SPEC and README, which are generated and
  are delta (19)'s. The six auto-regenerating mirrors need no hand edit; the
  hand-authored `docs/` pages above do.

### (18) The settings grant this cut deletes rides the ruled carve-out

`.claude/settings.json:29`, `"Bash(bash lifecycle-kit/bin/enter-stage.sh *)"`, is
the **only** grant naming this file or `lifecycle-kit/bin`, and it is removed **in
the same commit as the delete** {mechanical}. That is inside
`native-gate-port-remaining-corpus` ruling (2) (operator, 2026-08-29, lead-relay):
removing a grant whose target a ruled port cut deletes is a pure narrowing forced
by the cut and sits outside the 2026-08-22 bar, which stands unchanged for every
other permission-settings edit.

Two things the ruling pins and this delta honours: the scope is **the files that
commit deletes**, never a path prefix — and the count is **probed, never assumed**,
the attested probe having quadrupled one kit's. Here the probe returned one. The
replacement reach is already granted — `.claude/settings.json:11-12` carry
`Bash(bash gate-sdk/bin/run-gates.sh)` and `Bash(bash gate-sdk/bin/run-gates.sh *)`
— so no grant is **added** and the 2026-08-22 bar is never approached from the
other side.

**The dead grant is not silent, and this was probed rather than assumed** because
the natural reading is the other one. `native/src/gates/settings_paths.rs:130-136`
skips a candidate containing `*` — but the candidate is the **command token**, here
`lifecycle-kit/bin/enter-stage.sh`, and the trailing `*` is a separate token — so
the `is_file()` test runs and `check-settings-paths` **reds** on the delete. It is
absent from this file's own 30-gate `--for` selection, coupling to
`.claude/settings.json` rather than to the deleted path, so it surfaces in the
battery rather than in the path-scoped run; either way it is discharged in the
landing commit.

### (19) The regeneration fan-out this cut stales

Deleting one owed `.sh` moves `measured-claims.sh`'s `tree-shell-owed` key from
**35 to 34**, read off `--emit port-blockers --tree`'s trailer {mechanical}.
docs/site-architecture.md §Generated projections rules that a tree edit moving a
measured claim stales the generated `pre-commit` and `commit-msg` hooks, whose
baked invocation carries `check-measured-claim`'s resolved values verbatim, and
`docs/check-graph.html` with them. The SPEC and README edits stale their on-site
mirrors and the crate change stales the binary; `check-graph`,
`check-docs-mirror-fresh` and `check-gate-binary-fresh` are the reds, and all are
discharged in the landing commit.

**Three keys checked rather than assumed.** `ported-gate-members` (108) counts
registry members and this cut registers no gate; `gate-substrates` stays `native`
because an `Arm` adds no `.gate` descriptor; and `tree-shell-owed` is bound by
**no** `measured:` marker in any tracked `.md`, so no governed sentence goes stale
and the staleness is the baked hook alone. The other two keys' citations
(`docs/install.md:212` for the first, four surfaces for the second) are unaffected
for the same reason.

### (20) The queue entry is promoted, and the promotion is a section move measured to fit

`native-gate-port-remaining-corpus` moves out of `## Deferred` into
`## New Features` with `[design-pending]` swapped for
`[spec: SPEC-enter-stage-cut.md]` {mechanical}. Both facts are measured:

- **It is a move, not an in-place swap.** canon-kit/SPEC.md §check-amendment-queue
  assertion (b) reds a `[spec:]`-tagged entry left in a design-pending section, so
  the promotion crosses the section boundary *and* swaps the tag — the checksum
  property that section describes.
- **The lead line fits with two columns to spare.** Measured with a byte count,
  not by eye: `- **native-gate-port-remaining-corpus** [spec: SPEC-enter-stage-cut.md] [roadmap: now/reliability]`
  is **98** columns against `QUEUE_KIT_WRAP_BUDGET=100` (`queue-kit/lib/queue.sh:37`).
  The repo-relative spelling would overflow; the bare basename is the form
  canon-kit/SPEC.md §check-amendment-queue admits and the only one that fits.

**The entry demotes at build and never reaches `## Done`**, which its own body
rules — its deliverable is a corpus, so the next cut re-promotes with a fresh
amendment (canon-kit/SPEC.md §Merging an amendment). The demotion returns it to the
position the promoting commit's own diff records, and re-prices it against the
50-line cap the active sections do not apply, so the demoting commit may add no
line to it.

## Producers and consumers

The amendment introduces **one interface** — one bridged flag carrying the tool's
existing argv grammar — and **no new state, no new event, no new field and no new
knob**. It retires none either: every knob the arm reads survives with the same
name, the same default and the same owner, and the only shape that changes is the
transport (a sourced bash variable becomes a bridged `GATE_SDK_KNOB_*` element).

- **Producer** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row
  (`"--enter-stage"`, `Arm::Run(crate::emit::enter_stage::run)`,
  `crate::emit::enter_stage::KNOBS`), plus one `case` arm in
  `gate-sdk/bin/run-gates.sh` — required, unlike an `--emit-` member, because the
  front-end composes only the `--emit-<name>` spelling and this member cannot take
  it (delta 2). The enabling config is the table row itself: `--knobs` publishes
  the family roster and `gate_knob_env` resolves it before the exec, so nothing is
  configured per install.
- **Producer, second caller** — a hermetic harness resolving `gate_native_bin` and
  `gate_knob_env --enter-stage`, exercising the arm from a non-git sandbox cwd
  (delta 4). Its enabling config is `GATE_SDK_NATIVE_BIN`, which
  `gate-sdk/lib/test-hermetic.sh:14` already exports absolute for exactly this
  class of caller.
- **Consumer of the stamp** — `check-stage-evidence` and `check-stage-entry` at
  every subsequent entry, and `lifecycle_current_stage` wherever the cursor is
  read. The stamp's five fields are unchanged, so this consumer is unaffected by
  anything but the writer's substrate.
- **Consumer of the pre-flight dispatch** — `check-stage-entry` for the stamp path
  and `check-stage-evidence` for `--rename`, each spawned as a child process
  through the compiled resolver, receiving the `<queue> <state>` argv it always had
  and the bridged environment the arm resolved (delta 6).
- **Consumer of the `LIFECYCLE_KIT_ENTRY_PREFLIGHT` spawn** — this repo's nine
  wired entries, all routing through `scripts/gate-exec.sh`, which reads the gate
  name from its argv and the bridged knobs from the inherited environment
  (delta 7). Named because a compiled arm that sanitized its child environment
  would break all nine at once and no fixture would see it.
- **Consumer of the worktree classification** — the boundary refusal and the
  mid-iteration advisory, both reading the same scan so the two cannot disagree
  about what a path is; the class is *printed* and nothing reaps on it, which is
  the premise §bin/enter-stage.sh's one-capture-group ruling rests on and which
  this cut does not disturb (delta 8).
- **Consumer of the valve consumption** — the closing stage's disposition step,
  reading the ledger's `used` lines, and the arm itself reading the prior-`used`
  count to print at the moment of admission. The ledger's line set stays the arming
  session's alone: this tool writes no line ever and rewrites one token of one line
  (delta 7).
- **Consumer of the opened journal** — the entering session, which writes into it,
  and the **next** stage's predecessor-journal assertion, which reads it and must
  be able to tell the opener's own bytes from a session's. That discrimination is
  `lifecycle_stage_journal_written`'s, and delta (14) is the harness that keeps its
  two spellings in step across the new seam.
- **Consumer of the survey-record read trigger** — the entering stage session, at
  the one moment it is guaranteed to be looking. The trigger prints the record's
  `## ` headings and never its findings, which the port preserves verbatim.
- **Consumer, at the front end** — a session running
  `bash gate-sdk/bin/run-gates.sh --enter-stage <stage>` as its stage template's
  first step. §The non-gate arm already rules a session reaching a mode through the
  front-end a caller in good standing.

**One corpus is narrowed, and its readers' red conditions are enumerated rather
than their subjects** (canon-kit/SPEC.md §The causal-completeness check, point 5).
The narrowing is the deletion of one file from the tracked `*.sh` tree, and the
reader set is derived from `run-gates.sh --for lifecycle-kit/bin/enter-stage.sh`,
never a hand-picked subset:

- `check-docs-cmd` — reds on a fenced invoked repo-relative `.sh` path that **does
  not resolve**, so its verdict is *not* monotone under this narrowing: removing
  the file **adds** a violation at every fenced invocation of it. Cleared by delta
  (17), in the same commit, not by inspection.
- `check-spec-pointer` — reds on a `<path>.md §<heading>` citation that does not
  resolve. Non-monotone **if the section heading goes**, and it does not: delta
  (17) keeps `### bin/enter-stage.sh`, which is what keeps every citing surface
  green. Named because the natural reading of "the file is deleted" takes the
  section with it.
- `check-settings-paths` — reds on a permission grant naming a path that does not
  exist, so the delete **adds** a violation rather than removing one. Non-monotone;
  cleared by delta (18) in the same commit.
- `check-measured-claim` — `tree-shell-owed` moves 35 → 34; the reader that moves
  is the baked hook invocation, cleared by delta (19). Whether any tracked `.md`
  binds that key behind a `measured:` marker is checked rather than assumed.
- `check-graph` / `check-docs-mirror-fresh` / `check-gate-binary-fresh` — red on a
  stale hook, artifact, mirror or binary, non-monotone for the same baked-value
  reason. Cleared by delta (19).
- `check-comment-tier` — monotone in the `.sh` set and **gaining** in the `.rs`
  set: the file's `# spec:` directives become an obligation on the new module
  rather than a risk, and each lands on the construct it directed. Two must survive
  as written because each states a property the compiled form could silently lose —
  the columns-2-to-last witness's reach to `$NF` (delta 10) and the boundary wipe's
  ordering after the truncate (delta 9).
- `check-path-dialect` — its corpus is **both** substrates, so the file moves
  between halves rather than leaving; the obligation transfers to the module and
  the gate is monotone on neither side by itself.
- `check-knob-default-coupling` — named because delta (3) removes a literal
  `GATE_SDK_TMP_DIR` default site and a reader assumes something must follow.
  Probed: other tracked holders survive, so the removal can only remove
  comparisons. Monotone, cleared by inspection.
- `check-assertion-strength` — named because a prior cut in this corpus lost a
  gate's whole reach to a delete. Whether this file declares an `# exit:` header is
  a probe the landing commit owes, and the amendment states the obligation rather
  than the answer.
- `check-gate-fixture-coverage`, `check-gate-substrate-parity`,
  `check-gate-assertions`, `check-gate-fail-closed`, `check-gate-output` — named
  together because a reader assumes a ported member moves them, and none does: no
  `.gate` descriptor is added or removed and no registry row changes, the member
  never having been a gate.
- `check-shellcheck`, `check-exec-bit`, `check-tree-terms`, `check-unmarked-claim`
  and the remaining members of the selection — monotone in the scanned set; each
  can only lose findings. Cleared by inspection.
- `check-crate-arms` — gains rather than loses: one new module and its
  `#[cfg(test)]` coverage join the lint and test arms it runs at every commit.

**Cross-component signal: this amendment's component set is four** —
lifecycle-kit (§bin/enter-stage.sh, §lib/stages.sh, §The state machine, §The
committed gap inbox, §The survey record, §templates/stages/), gate-sdk (§The
non-gate arm, §run-gates, §lib/gate.sh), the crate's emit layer, and this repo's
`.claude/` bindings and `scripts/` config — so `check-stage-entry` assertion C
fires and the **align stamp is demanded at the next stage's entry**. Stated here so
the build session is not the one that learns it, and it is why this session
recommends the audit stage next.

## Existing sections updated

- `lifecycle-kit/SPEC.md §bin/enter-stage.sh` — restated as the arm's section: the
  invocation form and its two operand modes, the declared prefix-family roster, the
  `Arm::Run` shape and why the `--emit-` family is unavailable, the two sanctioned
  callers and the non-git ground for the second, and the three-state exit contract
  (deltas 2, 3, 4, 11). The tool's own contracts — the boundary reset's write set,
  the header predicate, the wipe's ordering and its `.gitkeep` invariant, the four
  boundary refusals, the valve's four narrowings and its honest limit, the
  predecessor-journal assertion and its escape, the worktree classification table,
  `--simulate`'s full roster and `--rename`'s witness — are **unchanged in
  substance** and re-expressed against the compiled reader (deltas 8, 9, 10).
  **This section records the cut**, per the composer entry's own rule that each
  closed cut's record lives in the contract section that cut selected, and records
  the section as **discharged with no shell residue in the kit at all** — the first
  such discharge in the port (delta 1).
- `lifecycle-kit/SPEC.md §bin/enter-stage.sh`, the evidence-kit dependency
  paragraph — the conditional source and its exit-2 arm are retired, the predicate
  being linked rather than sourced; the knob keeps its empty default and the
  consumer keeps the pattern (delta 8).
- `lifecycle-kit/SPEC.md §lib/stages.sh` — gains the parity obligation the cut
  creates: the library stays shell as the config bridge's sole resolver, and the
  harness that keeps `native/src/stages.rs` in step with it is named here rather
  than left implicit (delta 14).
- `lifecycle-kit/SPEC.md §Testing` — the criterion-2 discharge sentence gains this
  member's comparison, including the two columns compared by **shape** rather than
  by equality and the reason, and the live-tree `--simulate` arm this member can
  buy where the previous cut could not (delta 12).
- `lifecycle-kit/SPEC.md §The state machine`, §The committed gap inbox and §The
  survey record — each names this tool's invocation and each moves to the arm's
  spelling; the survey record's read-trigger paragraph additionally loses the
  sentence that this is "the survey record's surviving shell half", which the cut
  makes false (deltas 9, 17).
- `lifecycle-kit/SPEC.md §templates/stages/` — the first-step invocation every
  stage template carries, and the valve-arming obligation that names the tool
  (delta 17).
- `lifecycle-kit/templates/` — all six stage templates, `lead.md` and `consult.md`,
  whose `--simulate` step is the mode's designed consumer (delta 17).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains `--enter-stage`,
  recorded as the first member with **two sanctioned callers** and the first to
  declare a whole kit's **prefix family** rather than a sub-family; the paragraph
  distinguishing the family from the registry sentinel gains this member as its
  third instance (deltas 2, 3, 4).
- `gate-sdk/SPEC.md §run-gates` — the front-end's git-toplevel `cd` gains the
  statement that it is why a bridged arm needing a non-git cwd is reached by its
  second caller rather than by widening the cd (delta 4).
- `gate-sdk/bin/run-gates.sh` — the new `case` arm and its `usage()` line (delta 2).
- `.claude/settings.json` — the grant naming the deleted path removed in the
  deleting commit under ruling (2)'s carve-out; no grant added (delta 18).
- `scripts/lifecycle-config.sh` — its `# spec:` comment about the entry-preflight
  argv's missing interpreter word survives verbatim, the behaviour being preserved;
  named because a reader assumes a compiled spawn changes it (delta 7).
- `CLAUDE.md`, `README.md`, `docs/orchestration.md` — the backticked invocations,
  none of which any gate reaches (delta 17).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — moved from
  `## Deferred` to `## New Features` with `[design-pending]` swapped for this
  amendment's `[spec:]` ref; it **demotes** at build and never reaches `## Done`,
  which its own body already rules (delta 20).
- The generated projections this cut stales — the on-site SPEC, README and page
  mirrors, the generated `pre-commit`/`commit-msg` hooks, `docs/check-graph.html`,
  and the gate binary itself. All are rostered with their triggers and regen
  commands in `docs/site-architecture.md` §Generated projections (delta 19).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone, and the
      merge holds CLAUDE.md §The provenance seam: the dated rulings and authorities
      above land in git history, never in a kit SPEC's prose.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`), the none-remain half discharged at
      the iteration rather than at the commit.
- [ ] **The both-substrates comparison was taken before the delete** — all seven
      hermetic harnesses run against each implementation in one session, comparing
      exit status, stdout and stderr byte for byte including every `help:` line and
      every simulate prefix, every written file, every unwritten file on every
      refusal path, and the wiped set over a nested-`.gitkeep` scratch tree; plus a
      live-tree `--simulate` of every configured stage under both (delta 12).
- [ ] **The three-state status survives end to end** — a refusal exits **1**
      through `bash gate-sdk/bin/run-gates.sh --enter-stage <stage>`, proved by
      running one, not by reading `exec` (delta 2).
- [ ] **The non-git invocation still works through the second caller** — a
      boundary entry in a non-git `mktemp -d` stamps, and the linked-worktree check
      skips rather than failing, taken by running it (delta 4).
- [ ] **The pre-flight gates ran as children, not as functions** — proved by a
      consumer-shadowed `.sh` member of `check-stage-entry` being the one that
      runs, which no in-process call could satisfy (delta 6).
- [ ] **All nine wired entry-preflight commands still resolve their gate** — taken
      by a real stage entry, since these commands run nowhere else, and the child's
      inherited `GATE_SDK_KNOB_*` environment is what makes them resolve (delta 7).
- [ ] **The shipped worktree pattern parses under the compiled matcher** — this
      repo's `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` classifies a real locked worktree
      `live`, proved by creating one, because a dialect mismatch silently
      reclassifies every path as `unclassified` and turns a refusal into a pass
      (delta 8).
- [ ] **The boundary orderings hold** — the wipe runs after the truncate and after
      the tool's own temporaries are gone, the journal opener runs after the wipe,
      the header run stops at a `## ` heading, and the retained blank run does not
      grow by one per boundary; each taken by running a real boundary, not by
      reading the code (delta 9).
- [ ] **The wipe's at-any-depth basename reach is preserved, not corrected** —
      the filed defect is unchanged by this cut and its entry still owns the fix
      (delta 9).
- [ ] **`--rename` names no field count** — the witness compares fields 2..NF as
      one joined string and no code path holds an arity (delta 10).
- [ ] **The lib-parity harness landed with the port** —
      `lifecycle-kit/gate-tests/stages-lib-parity.test.sh` exists, is registered in
      the kit's runner, and compares both spellings of the opening-line shape
      (delta 14).
- [ ] **Removals propagated** — grepped every spec, template, README, skill,
      command, settings file, gate-test, smoke script and committed workflow
      surface for the deleted path; the silent set of delta (17) is fixed by hand
      and nothing dangles.
- [ ] **The section heading survived** — `check-spec-pointer` is green and every
      citation of §bin/enter-stage.sh still resolves (delta 17).
- [ ] **The settings narrowing was probed, not assumed** — the grant count removed
      is the count that commit's deletes produce, and no grant was added (delta 18).
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC/README/page mirrors, and the
      gate binary (delta 19).
- [ ] **The promotion and the demotion both fit** — the promoted lead line
      measures 98 columns, and the entry returns to Deferred at build with no line
      added against a re-measured cap (delta 20).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
