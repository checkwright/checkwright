# SPEC amendment: gate-tests-cut

The port disposition of **`gate-sdk/bin/run-gate-tests.sh` (188 lines), the one owed file
declaring gate-sdk/SPEC.md §run-gate-tests**: it ports to a bridged `--run-gate-tests`
`Arm::Run`, and every resolution it performs through `lib/gate.sh` stays in that library,
reached by a **bash spawn** rather than reimplemented. A stated-contract cut under the
port-first run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on its own per-cut feature entry
and packaged by the lead as one of this iteration's four.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *108 member(s) scanned,
0 group(s) formed, 0 undecidable, 108 already ported and excluded, 0 permanently shell and
excluded, 0 temporarily held and excluded; 0 still owed, 0 takeable at this cut* — no
takeable group, which is the budget arm's stated precondition (§The first cohort). The
selection ground is the **owed column** of `--emit port-blockers --tree` — *89 file(s)
scanned, 66 declared no-port, 0 temporarily held, 23 owed* — where this file reads
`owed lines=188`.

**This is the section's whole owed set, so the cut is the section and no outer-bound
judgment is owed.** §run-gate-tests names exactly one file. The sibling cuts of this
iteration take §lib/test-hermetic.sh, guard-kit/SPEC.md §Testing and context-kit/SPEC.md
§Testing's unblocked remainder; none of the four shares a section with another, which is
what makes them four amendments rather than one.

**Cut ordering, stated here because build reads it and nothing else records it.** This cut
and `SPEC-hermetic-cut.md` both touch the same reader — the bespoke `*.test.sh` corpus — and
the hermetic cut's delta 1 removes a second producer this runner's own absolutization is the
model for. Neither blocks the other and they may ride one batch; if they ride two, this one
first, because the hermetic cut's delta 1 cites this runner's line 24-28 shape as the
in-tree precedent it adopts and a reader of the merged §lib/test-hermetic.sh should find that
shape already described in the merged §run-gate-tests.

## What changes

### (1) The cut is the one owed file declaring §run-gate-tests, and it does not discharge gate-sdk

`bin/run-gate-tests.sh` reads `owed lines=188` and is the only owed file whose `# spec:`
pointer binds `## run-gate-tests` {mechanical}. Its reach and its section bound coincide, so
no second section is rewritten by construction — with the one exception every cut moves,
§The port disposition's owed-corpus prose, and the roster sentence in §The non-gate arm that
delta 7 adds it to.

**It does not discharge gate-sdk**, and the amendment says so because a reader will assume
otherwise from the kit's short owed column. `lib/test-hermetic.sh` (52) is the sibling cut of
this same iteration; `lib/inject.sh` (80) stays owed behind `doctrine-kit/bin/install-doctrine.sh`
under `kit-library-port-residue`'s stated sequencing. After both of this iteration's gate-sdk
cuts the kit's owed column holds `lib/inject.sh` alone.

### (2) The arm is `--run-gate-tests`, an `Arm::Run`, and the front-end needs no edit

The member ports as a **bridged-arm table** row — `("--run-gate-tests", Arm::Run(run_gate_tests::run), KNOBS)` in
`native/src/emit/mod.rs`'s `BRIDGED_ARMS`, with the arm's body in
`native/src/emit/run_gate_tests.rs` {design-bearing}.

**`Arm::Run` and not `Arm::Emit`, on the variant's own stated test.** The variant is a return
shape: an arm that renders a document is `Emit`, an arm that returns an exit code is `Run`.
This member's contract is a three-valued exit — 0 clean, 1 a logic failure in a gate or a
unit test, 2 a harness or fixture error — which §run-gate-tests states and every caller reads.
An `Emit` member could carry the report but not the verdict, and the verdict is what the
evidence-kit suite and the per-kit roster line consume.

**It must be a bridged-arm table member and not a hardcoded top-level flag**, because it needs
configuration: `GATE_SDK_TESTS_DIR`, `GATE_SDK_TMP_DIR` and the binary knob all reach it
today through `lib/gate.sh`. §The non-gate arm rules that choice forced — "a configured tool
ported as a top-level flag therefore resolves platform defaults and silently ignores every
consumer override". The declared knob roster is delta 4's.

**The front-end needs no edit at all, and that is measured rather than assumed.**
`bin/run-gates.sh` reduced to a residue that resolves the repo root, resolves one bridged
environment and execs the binary: its argv grammar treats *every* leading `--<token>` that is
not one of the five forms it names as an arm name handed to `exec_arm` untouched
(`bin/run-gates.sh:45-70`). So `bash gate-sdk/bin/run-gates.sh --run-gate-tests <args>`
works the moment the table row exists. What the port *does* owe is the usage line, delta 6.

**`ARM_UNAVAILABLE_STATUS` stays 2 for this arm**, by falling outside the two-name test at
`bin/run-gates.sh:42`: a test runner whose binary is absent has not passed, and 0 is reserved
for a harness-integration arm gating a user action, which this is not.

### (3) Every `lib/gate.sh` resolution stays in `lib/gate.sh`, reached by a bash spawn

This is the cut's central design ruling and the reason its delta set is not all-mechanical
{design-bearing}.

**The obstacle, stated first, because it is the ground a naive port would fail on.** The
runner calls `gate_command` once per case (`bin/run-gate-tests.sh:50`), and `gate_command`
resolves each member's declared knobs by sourcing the owning kit's `lib/*.sh` in a subshell
(`gate-sdk/lib/gate.sh:443-460`, through `gate_knob_env` and `gate_knob_env_set`). A
crate-side resolver doing the same would be the **second producer** criterion 6 refuses, and
§The kit-library port disposition states the consequence in its strongest form: for a bridged
knob "there is exactly one place that value is computed — the kit's shell library — and the
binary holds no default to drift". This is precisely the ground `bin/run-consumer-smoke.sh`
carries its own `# no-port:` on (§The non-gate arm, *The class gained no member from the
consumer smoke*), so a reader arriving here will reasonably expect this member to be
undeportable for the same reason.

**It is not, and the discriminating road already exists in tree.** `gate-sdk/lib/consumer-smoke.sh`'s
own `# no-port:` header records it: the upgrade suite "is now a *spawn-side* caller: its
bridged arm invokes these helpers in a bash that sources this unchanged library, which
creates no second producer and is why that port cleared criterion 6 on the
**duplication-absent road**". The mechanism is `native/src/emit/upgrade_smoke.rs:380-460` —
`csmoke()` spawns `bash -c '<source line>; <helper> "$@"'` and reads the helper's answer off
stdout. The difference between the two members is not the library they call but whether the
crate re-derives what the library computes: `run-consumer-smoke.sh` was declared because its
*accounting* re-derives a resolution, while this runner only ever **consumes** `gate_command`'s
answer.

**The ruling: the ported runner resolves each case's dispatch by spawning
`bash -c 'source "$1/lib/gate.sh"; shift; cd "$1"; shift; gate_command "$@"' bash <sdk> <casedir> <gate> <gate-dirs>...`
and reads the resolved argv off stdout, one element per line, exactly as
`mapfile -t argv` reads it today.** Three properties of the shell form survive unchanged and
each is the reason a cheaper shape was refused:

- **The `cd` into the case dir happens on the shell side, before `gate_command` runs.**
  §run-gate-tests rules that the dispatch executable and the gate dirs resolve at the
  invoker's root while the **knob values resolve inside the case dir**, because a kit library
  reads its `<KIT>_CONFIG_FILE` cwd-relative and "resolving them at the invoker's root instead
  hands the binary this repo's consumer config while the script beside it reads the fixture's
  — which is not one oracle over two substrates but two oracles". Putting the `cd` inside the
  spawned shell keeps that split byte-identical.
- **A refusal is a status, not a parse.** `gate_command` exits non-zero having named the
  refusal on stderr; the spawn's stderr is inherited and its status becomes the harness's
  `HARNESS:` line, so the two existing failure texts at `:51` and `:55` are unchanged.
- **Nothing is re-implemented.** The crate learns the argv and nothing about how it was
  derived, which is what makes the duplication *absent* rather than machine-held.

**Whether the spawn is per case or batched is left to build as a mechanical calibration,
under one binding constraint**: the resolution for a case must run with the process cwd
inside that case's own directory, and batching must not change any resolved value. A single
spawn looping over `(gate, casedir)` pairs and emitting a delimited table satisfies that
constraint and is not a second implementation of the case loop; per-case spawning is the
literal transcription. Neither is ruled here because no stated contract distinguishes them
and the choice is a cost, not a semantic.

**Two alternatives are refused, each on its own ground.**

- **A crate-side knob resolver** is refused as criterion 6's second producer, on the ground
  §The kit-library port disposition states for the whole class. It is the failure this delta
  exists to name, because it is the shape a session sizing the member reaches for first.
- **Declaring the member `# no-port:` on the consumer-smoke ground** is refused because the
  ground does not hold of it: that declaration rests on the harness *re-deriving* a
  resolution, and this runner performs no accounting of its own. Declaring here would
  over-declare against a file holding no bridge content, which §Porting a gate to the binary
  substrate names as mis-sizing the predicate with nothing red to catch it.

### (4) The declared knob roster, and the two argv positions

The arm declares `GATE_SDK_TESTS_DIR`, `GATE_SDK_TMP_DIR`, `GATE_SDK_NATIVE_BIN` and
`GATE_KIT_ROOTS_HERE` {design-bearing}. The first two are read directly (the tests-dir
default and the scratch pin); the third is what the arm absolutizes and re-exports for the
case invocation; the fourth is what `gate_check_dirs` resolves the default gate-declaration
dir set from, and it crosses the bridge rather than being re-derived for the same
second-producer reason delta 3 gives.

**Both positional arguments port unchanged, and the verdict is taken per position against
§The non-gate arm's distinguishing test rather than off a count.** `$1` is the tests dir —
a **scan-root positional** the rule itself composes, the shape three sixth-batch members
already carry, and fail-closed (an absent tree exits 2 saying so, `:37`). `${@:2}` is the
gate-declaration dir set — an **input-corpus positional** selecting what the rule analyses,
the shape `check-gate-tamper`'s `--fixture` establishes. Neither redirects something
`gate_command` has already resolved, so neither is the unportable kind, and no documented
sentence is deleted.

**The second position's fail-open defect is carried, not fixed.** §run-gate-tests records
that `${@:2}` **replaces** the resolved default with a `[[ -d ]]` filter that drops a
non-existent member silently, so a wrong checks dir empties the set and every gate reports
`HARNESS: <gate> resolves in none of:` with nothing after the colon. That is filed as
`fixture-runner-checks-dir-fails-open` and stays filed: fixing it here would change the
member's interface inside a port, which is the shape §The non-gate arm's argument test exists
to keep separate from an implementation move. The ported arm reproduces the filter and the
symptom exactly, and the merged section keeps the paragraph naming the entry.

### (5) The behavioral surface held byte-identical, and the one deliberate narrowing

Every reported line, exit code and matching rule is reproduced as specified rather than as
convenient {design-bearing}. The parts a port would most easily get wrong, each with the
sentence in §run-gate-tests that fixes it:

- **The `args` splitting rule.** `#` lines are stripped and the surviving text is word-split
  on whitespace into argv, "**not** taken one argument per line — so an argument containing a
  space is unexpressible in this file, and any second implementation of the runner (the
  crate's own parity test among them) must reproduce the splitting rule, not guess it".
  The port splits on ASCII whitespace runs, which is what the default `IFS` gives.
- **The one deliberate narrowing, stated because it is unobservable.** The shell form's
  `args=($(grep -v '^#' "$casedir/args"))` is unquoted, so bash applies **pathname expansion**
  after word-splitting, against the *invoker's* cwd rather than the case's. The ported arm
  performs word-splitting alone. Two facts make this a narrowing to state and not a defect to
  preserve: §run-gate-tests specifies word-splitting and says nothing of globbing, so the
  expansion is unspecified behavior rather than contract; and it is unreachable in tree —
  measured this session, `git ls-files '*/gate-tests/*/args' | xargs grep -n '[*?[]'` returns
  **no non-comment hit across 140 args files**. Recorded here because a later reader
  comparing the two implementations will find the difference and must not read it as an
  oversight.
- **The expect-line conjunction.** Every non-blank line must appear literally in the case's
  combined output; a blank or whitespace-only line asserts nothing; matching is
  order-independent; and a failing case names **every** missing line rather than the first.
- **The output contract, asserted at runtime.** A `good/` case must emit
  `^<NAME>: clean (<parenthetical>)$` and a `bad/` case a `help:` remedy line, on top of exit
  code and `expect.txt`.
- **The three-valued accounting and its two trailers**, including `0 pairs` beside a unit
  count for a unit-test-only dir, and exit 2 only when the tests dir holds neither pairs nor
  `*.test.sh`.

**The `cd` leaves the harness process entirely, which is stronger than the shell form.** The
shell runs each case inside a `$( cd "$casedir" && … )` subshell; the ported arm sets the
child's working directory on the spawn (`Command::current_dir`) and never mutates its own.
The hermeticity the `cd` buys is unchanged — a gate resolving `<KIT>_CONFIG_FILE` under its
cwd still finds only the case's own files — and the harness gains an invariant the shell
form only had by construction: no code path can leave the process in a case dir.

### (6) The named consumers, re-pointed in the same commit

A ported runner whose callers still name the script is a member with two live entry points
{mechanical}. Three callers are named and all three move:

- **`scripts/evidence-config.sh:10`** composes the per-kit fixture suites' run command as
  `bash gate-sdk/bin/run-gate-tests.sh $_tests${_checks:+ $_checks}` inside the
  `gate_fixture_suites` loop, so **every** per-kit suite re-points with one edit. The form is
  the one line 28 already uses for the upgrade suite:
  `bash gate-sdk/bin/run-gates.sh --run-gate-tests …`.
- **README.md §This repo, governed** carries the per-kit runner roster line a contributor
  runs by hand, and §run-gate-tests cites that line by name when it explains that this repo's
  consumer remainder keeps no `checks/` dir and passes the tests dir alone.
- **`gate-sdk/lib/test-hermetic.sh`'s two `spec:` comments** name §run-gate-tests for the
  binary pin and the dispatch rule; the sibling cut rewrites the first and both stay pointed
  at the same section.

**The script is deleted in the commit that lands the arm, never left running beside it**, and
the ordering §run-gate-tests fixes is honoured: the ported member's fixture pairs pass against
the new dispatch **before** the script it replaces is deleted.

### (7) §The non-gate arm's roster gains the member, undated

The class roster gains `--run-gate-tests` with its owning section named and **no landing-date
cohort label** {mechanical}. The roster's dated cohort labels are the subject of this
iteration's sibling unit `kit-spec-provenance-seam-sweep`, whose amendment rules them
provenance and retires them; adding a fourth dated cohort here would land work that unit then
removes. A member added undated is the shape that unit's amendment leaves behind, so this cut
writes it that way from the start rather than creating and deleting the same label.

### (8) The runner's own coverage, and what it may not become

`gate-tests/run-gate-tests.test.sh` — the bespoke test that drives the runner over scratch
fixture trees to pin the expect-line conjunction — is **re-pointed at the arm and kept**
{mechanical}. It reaches its subject through `gate_arm_run` (`lib/test-hermetic.sh:27`), the
arm counterpart of `gate_run`, which is exactly what that helper exists for and needs no
change to it. Its inner invocation stays bounded by a tests dir holding fixture dirs and no
`*.test.sh`, so it runs pairs and returns rather than recursing.

**The member owes no `good/`+`bad/` fixture pair and must not be given one**, on
§run-gate-tests' own standing ground: the runner is a test layer parallel to the gates, never
a `gates.list` member, so a pair for it would sit outside `check-gate-fixture-coverage`'s
registry authority set and be audited by nothing. The port does not change that; §The
non-gate arm rules the same for the class.

## Producers and consumers

**New interface: the `--run-gate-tests` bridged arm.**
*Producer* — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` row, dispatched in `native/src/main.rs:485-498`
before the registry lookup; its enabling configuration is the bridged environment
`gate_knob_env` builds for it, which `bin/run-gates.sh:32-37` resolves and execs and which
`gate_arm_run` resolves for the bespoke test. Reachable with no front-end edit
(delta 2).
*Consumers* — (a) `scripts/evidence-config.sh`'s `EVIDENCE_KIT_RUN_<suite>` values for every
per-kit fixture suite, read by `--run-validate` at the validate stage; (b) the contributor
running the roster line in README.md §This repo, governed; (c)
`gate-tests/run-gate-tests.test.sh` through `gate_arm_run`. Each reads the arm's **exit
status** and its report on stdout.

**New crate module: `native/src/emit/run_gate_tests.rs`.**
*Producer* — the `Arm::Run` function above. *Consumer* — `main.rs`'s arm dispatch only; the
module exports nothing else, so no second caller is created.

**Existing interface whose flow changes: `gate_command`'s caller set.**
*Producer* — unchanged, `gate-sdk/lib/gate.sh`. *Consumer* — the caller that was
`bin/run-gate-tests.sh:50` becomes the bash the arm spawns, sourcing the same unchanged
library. The library's own text is untouched, so it gains no second producer and its
`# no-port:` ground is unaffected.

**Fields and their readers.** The arm introduces no new message and no new field. The one
value it publishes is its exit status, whose readers are the three named above and whose
transition is process exit; the report on stdout is read by a human and by
`EVIDENCE_KIT_PARSER=exit-code`, which reads the status alone.

**This delta set narrows no corpus**, so point 5 of the causal-completeness check does not
bind: the tests dir, the gate-declaration dir set and the args corpus are all unchanged in
extent, and the one narrowing in delta 5 is a narrowing of an *unspecified expansion* over a
corpus measured empty rather than of a scanned corpus. The two readers whose red conditions
would otherwise need enumerating are stated anyway, because they are what proves the port:
`check-gate-fixture-coverage` reds on a registered gate with no pair (unchanged — the runner
registers no gate), and the evidence-kit suite reds on a non-zero exit (unchanged — the
arm's three-valued exit is delta 2's contract).

## Existing sections updated

- **gate-sdk/SPEC.md §run-gate-tests** — the runner's mechanism restated for the arm: the
  dispatch resolution's bash spawn, the cwd discipline, the argv positions, the deliberate
  narrowing and the re-pointed callers (deltas 2, 3, 4, 5, 6, 8).
- **gate-sdk/SPEC.md §The non-gate arm** — the class roster gains `--run-gate-tests`,
  undated; its spawned-program set gains `bash` for this member, in the prose paragraph that
  records those (deltas 2 and 7).
- **gate-sdk/SPEC.md §The port disposition** — the owed-corpus prose every cut moves; gate-sdk's
  owed column after this cut and its sibling (delta 1).
- **gate-sdk/SPEC.md §lib/gate.sh** — the sentence naming this runner as "the one caller that
  needs the dispatch executable rather than the whole command" stays true of the spawned
  resolution and is re-pointed at it (delta 3).
- **README.md §This repo, governed** — the per-kit runner roster line (delta 6).
- **`scripts/evidence-config.sh`** — the composed per-suite run command and the file's own
  `# no-port:` header, which names the runner commands as this repo's test topology (delta 6).
- **`gate-sdk/lib/test-hermetic.sh`** — its two `spec:` comments naming §run-gate-tests
  (delta 6); the sibling cut owns the line they annotate.
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; it stales on the line-count change alone and no delta owns its content --> `docs/footprint.md` and `docs/value.md`'s rollup block.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable
      producer and a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than at this commit while
      the sibling gate-sdk amendments are in flight.
- [ ] **Removals propagated** — grepped every spec, config and doc for
      `run-gate-tests.sh`; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
