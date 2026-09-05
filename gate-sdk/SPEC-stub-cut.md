# SPEC amendment: stub-cut

The port disposition of **`gate-sdk/bin/run-gates.sh` (503 lines), the one owed file
declaring gate-sdk/SPEC.md §run-gates**: it ports down to the stub that section already
names, the seventeen per-arm dispatch branches move into the binary's own argv parsing,
and the binary-less shell dispatch loop **retires** rather than porting. A
stated-contract cut under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE),
selected by `native-gate-port-remaining-corpus`'s composer and packaged by the lead.

**The composer's own precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *108 member(s)
scanned, 0 group(s) formed, 0 undecidable, 108 already ported and excluded, 0
permanently shell and excluded, 0 temporarily held and excluded; 0 still owed, 0
takeable at this cut* — **no takeable group**, which is the budget arm's stated
precondition (§The first cohort). The size arm is permanently exhausted, so the budget
arm composes, and this file is ordered early **inside that arm** because it grows by a
branch per bridged arm — 407 lines at one reading, 421 at the next, 503 at this one.
That is an ordering rationale and **not** the blocker-retiring override: §The first
cohort's own discriminator is *queued behind*, and a member the instrument still
reports takeable was never queued behind anything. The selection ground for the cut
itself is the **owed column** of `--emit port-blockers --tree` — *94 file(s) scanned,
64 declared no-port, 0 temporarily held, 30 owed*, 3956 lines — never the registry
`--group` read, which answers for the battery and not for the tree.

**The disposition is ruled and this amendment does not re-open it.**
`run-gates-front-end-cut-legality-unruled`'s 2026-09-05 operator ruling in consult
took **disposition (i)**, and §run-gates already carries the mechanism: *"The
front-end's port disposition: it ports down to a stub, and the loop above retires with
the cut."* Two readings are refused there and stay refused here — porting the file
whole with the loop inside, and declaring the whole file on a subtraction ground that
grows with every arm. What §run-gates states is the **destination**; what it does not
state, and what this amendment owes, is the **residual argv grammar** — what the stub
must still know when there is no binary to ask — and the re-scoping of the installer's
binary-less leg that the loop's retirement forces. Those are deltas (2) through (7).

**`binary-less-dispatch-loop-retirement` folds into this cut**, by the same ruling.
It is not a sibling unit and does not get its own entry, its own amendment or its own
batch: its whole deliverable — the loop, the leg the loop's contract is asserted by,
and the nine `.workflow/validate-baseline.txt` rows that record the unpaid price — is
deltas (3), (5) and (6) here.

## What changes

### (1) The cut is the one owed file declaring §run-gates, and it does not discharge the kit

`bin/run-gates.sh` reads `owed lines=503` and is the **only** owed file whose `# spec:`
pointers bind `### run-gates` {mechanical}. Its reach and its section bound therefore
coincide, and no second section is rewritten by construction — with one stated
exception, §The port disposition's owed-corpus prose, which every cut moves.

**It does not discharge gate-sdk**, and the amendment says so because this is the kit a
reader would assume otherwise about. The kit's owed column keeps
`bin/run-gate-tests.sh` (188), `lib/inject.sh` (80) and `lib/test-hermetic.sh` (52) on
their own separate grounds, and `bin/build-native.sh` (110) leaves it by declaration
rather than by port in a sibling unit of this same batch. This cut settles one section.

### (2) The residual argv grammar: the crate's parser gains the front-end spelling as one alias step, and the stub passes argv verbatim

This is the cut's central design ruling and the reason its delta set is not
all-mechanical {design-bearing}.

**The obstacle is stated first, because "exec the binary with argv verbatim" reads as
free until you look at what stands between.** `exec_arm` (`run-gates.sh:144-158`) does
not merely exec: it calls `gate_knob_env "$arm" "$@"` (`:155`), and that function
(`lib/gate.sh:384-401`) runs `"$bin" --knobs "$arm" "$@"` — it **asks the binary** what
the arm reads and resolves exactly that set. So the bridged environment is already
derived by the crate, and the only thing the shell contributes to it is the **arm
name**, which today it *composes*: the front end turns `--emit graph` into
`--emit-graph` (`:178`) and turns a bare run, `--only` and `--for` into
`--run --gates-dir <dir> …` (`:296-305`). Passing argv verbatim therefore means the
crate must accept the front end's spelling at **both** doors — `--knobs` and dispatch —
or the stub keeps composing and the branches have not moved.

**The ruling: the crate's argv parser takes one normalization step at its top, and the
fused arm names stay the arm table's one spelling.** `--emit <name>` normalizes to
`--emit-<name>`; a leading `--only`, `--for`, `--`, or a bare gates-dir positional
normalizes to the `--run` arm with the argv the shell composes today; every other
leading token is already an arm name and passes untouched. The step runs **before**
both `if first == "--knobs"` (`native/src/main.rs:405`) and `emit::lookup(first)`
(`:434`), so the two doors cannot disagree — which is the property the shell version
could only hold by hand.

**Two alternatives are refused, each on its own ground.**

- **Teaching the crate a second, parallel arm table keyed on the front-end spelling** is
  refused as a second source: `--emit-<name>` is the spelling `.gate` dispatch and every
  hermetic `gate_native_bin` caller already use (§check-gate-substrate-parity), and a
  table that must be edited twice per arm is exactly the per-arm growth this cut exists
  to delete, relocated rather than removed.
- **Leaving the composition in the stub** is refused because it is the whole of what
  grows: seventeen `case` arms exist to compose seventeen arm names, and a stub that
  keeps composing keeps growing by one per bridged arm — the subtraction §run-gates
  refuses in its other limb, arrived at from the other side.

**One asymmetry is admitted rather than hidden.** After this delta the crate accepts
`--emit graph` and `--emit-graph` as the same arm. That is an alias, and an alias is a
cost. It is taken because the *front-end* spelling is the one every human caller, every
workflow file and every settings grant in every consumer tree already types, and
because the alias lives in one function next to the table rather than in a second tree.

**`--only` gains a bounded argv channel, and it is single-member-or-refuse.** Ruled by
the lead on its own authority, 2026-09-05, on an escalation from this stage: argv after a
`--` separator is **forwarded to the selected gate**, and forwarded **only when the
selection resolves to exactly one member**. A `--` with two or more selected members is a
**refusal**, not a broadcast — handing one argument vector to N gates is the failure the
constraint exists to foreclose, and it is spelled as a refusal rather than a silent
first-member rule so a caller cannot get a narrower run than it asked for and not know.

**Why the channel is owed here rather than worked around in evidence-kit.** The sibling
`preflight-front-end-cut` re-points nine `LIFECYCLE_KIT_ENTRY_PREFLIGHT` entries onto this
form, and every one of them carries an argument. Without the channel the re-point does not
merely lose coverage on some entries — it **refuses every one of them**, because
`native/src/emit/enter_stage.rs:1260-1263` appends `<queue> <state>` to each entry's argv
and `--only` consumes every remaining token as a member name
(`native/src/runner.rs:69-72`), so the appended pair resolves as two unregistered gates.
The capability being moved is real: `scripts/gate-exec.sh` resolved a gate name and
invoked it *with* arguments, and the operator ruled the caller now lives here.

**The alternative the escalation recommended was refused, and its premise was the
error.** Widening `check-producer-liveness` to read both corpora with no positional
contradicts `evidence-kit/SPEC.md:1045-1049` — *"The two modes are told apart by the
argument being a directory, not by a flag: the caller already knows which it holds"* —
and would retire as an accident the `.run`/`.lock` split that `:1051-1056` rules
*"load-bearing rather than cosmetic"* and `:1206-1212` rules keeps both entry kinds side
by side. It would also have failed on the appended-argv fact above, which no knob
addresses. Recorded because a later reader meeting the same gap will reach for the same
workaround.

**The existing `# spec:` at `run-gates.sh:331` is amended, not deleted.** Its sentence —
*"empty for every member under `--only`, which names gates and has no paths to hand one"*
— stays **true of the battery case** and was never written about a single-member
selection. It gains that case rather than losing its subject, which is the difference
between correcting a claim and erasing the reasoning behind it. `Selected { name, args }`
(`native/src/runner.rs:30-33`) already carries the per-member argv field this fills, so
the type does not change.

### (3) The binary-less dispatch loop is deleted, not ported, and its standing byte-comparison goes with it

`run-gates.sh:308-503` — the registry-not-found handling and its `--only` steer, the
member listing and the *names no gates* refusal, `RESOLVE_DIRS`, `select_for` /
`select_only`, the dispatch loop with its timing and capture, and the summary and
omission accounting — is removed whole {design-bearing}.

**Its only served branch has nothing left to dispatch, and that is measured rather than
argued.** The loop serves criterion 5's omit-and-declare install alone. Ten kit
`checks/` roots hold **97 `.gate` files and zero `.sh` files**; the only `.sh` under any
`checks/` path are fixture files inside pruned gate-test trees. So an artifact-less host
seeds a `gates.list` whose live member set is empty, and `native/src/runner.rs:424-428`
refuses it — `eprintln!("{}: {} names no gates", TOOL, list); return 2` — rather than
passing vacuously. The branch the duplication was admitted for cannot be entered.

**Nothing in the loop is unique to it**, which is why deletion and not a port is the
whole of the work: the `--only` steer already exists in the crate at
`native/src/runner.rs:576-585`, the unregistered-name refusal at `:207`, the
empty-registry refusal at `:424-428`, and the selection, dispatch, timing and summary
are the `--run` arm's by construction.

**`gate-sdk/gate-tests/run-arm-contract.test.sh:115-120` is deleted with it.** That
block is criterion 6's *discharge* — it forces `GATE_SDK_NATIVE_BIN` at an absent path
to drive the script into the loop and diffs the loop's transcript byte-for-byte against
the arm's. With one dispatcher there is no second transcript, and the block would be
asserting a comparison against the stub's absent-binary diagnostic. Its summary line at
`:123` names the comparison and is reworded. **Every other assertion in that file
survives** — the output contract, the front-end's refusals, `--for`'s
note-not-failure case and the determinism triple — because none of them names the loop,
and after delta (2) they are the executable oracle for whether the stub still serves
every caller. **A build session re-greps before deleting**: the roster of loop
references was derived by grep rather than by reading every fixture, and the
oracle-first rule says the suite decides, not the roster.

### (4) The stub keeps one piece of per-arm knowledge — the unavailable status — because it is exactly what cannot be asked of an absent binary

`ARM_UNAVAILABLE_STATUS` (`run-gates.sh:143`) is 2 for every arm whose verdict a battery
or a session reads and **0** for a harness-integration arm gating a user action, which
§The non-gate arm rules must decline rather than wedge the session {design-bearing}.
Two arms take the 0: `--hook` (`:190`) and `--statusline` (`:198`).

**This one datum cannot follow the others in-crate, and the reason is not an oversight
in delta (2).** It is read on precisely the path where the binary is **absent or not
executable** (`:150-153`) — the branch that prints the build remedy naming
`bash gate-sdk/bin/build-native.sh`. A property the binary would have to be running to
report is unavailable exactly when it is needed, so the stub holds it: a two-name test
on the leading token, not a seventeen-arm table.

**It is a second source and the amendment says so rather than claiming otherwise.** The
crate states each arm's unavailable status in that arm's own contract prose; the stub
states it for two of them in shell. Nothing holds the two in lockstep today. What bounds
the exposure is that the shell half is **two names and a default**, so it grows only
when a new fail-open arm is minted — and the honest closure, a parity assertion over the
fail-open set, is a gate this cut does not build. It is filed rather than flagged.

### (5) The one usage text moves in-crate with the arm table it describes, and `--help` on a binary-less tree changes

`usage()` (`run-gates.sh:20-134`) is 115 of the file's 503 lines — the single largest
block — and §run-gates rules it *"the one usage text, the stdout body of a help request
and the stderr body of an unrecognized-option refusal"* {design-bearing}. It moves,
because it is a per-arm description that grows by one paragraph per bridged arm and
belongs beside the table delta (2) makes authoritative.

**The consequence is a real behavior change and is stated rather than discovered at
build.** Today `-h` and `--help` print and exit 0 before any binary check, so they work
on a tree with no binary. After this cut they reach the binary like every other arm, and
on a binary-less tree they get delta (4)'s diagnostic — which names the build command —
at the unavailable status rather than the usage text at 0. **That trade is taken
deliberately**: a usage text listing seventeen arms none of which can run is a worse
answer than one line naming the one command that makes them run. Keeping a second short
usage in the stub is refused outright as the SSOT breach it plainly is.

**`--help`'s status where the binary is present is unchanged at 0**, which
`run-arm-contract.test.sh`'s refusals block already asserts and keeps asserting.

### (6) The installer's binary-less leg re-scopes at this cut, from a green battery to a declared refusal

§run-gates already rules the shape — *"the installer's binary-less leg re-scopes at the
same cut to assert the declared omission and, where nothing survives, say so"* — and
this delta is the mechanism {design-bearing}. The leg is
`installer/consumer-smoke/run-smoke.sh:372-399` over `BARE_PROFILE=prose`.

**The assertion that must invert, and the assertion that must not.** The leg reaches the
battery through the shared `assert_install()` (`:164-260`), whose lines `:171-175` run
the battery and require exit 0 **and** the literal `All N gates passed`. On a
binary-less install that is now exit **2** and `run-gates: <list> names no gates` on
stderr. The **disclosure** half is untouched and becomes the leg's whole point:
`BARE_OMITTED > 0` (`:397-398`), fed by the independently-derived expected omission set
at `:243-254`, still holds — what changes is its meaning, from *some members lost* to
*every member lost*.

**The plumbing does not exist and this delta is where it is designed, not where it is
discovered.** `assert_install()` is shared by every profile leg, so the rc-and-phrase
check cannot be inverted in place without breaking the covered-platform legs that
depend on it. The ruling: **the battery assertion becomes a parameter of the helper, not
a branch inside it** — the helper takes the expected battery outcome from its caller and
every existing leg passes the green expectation it asserts today, while the binary-less
leg passes the refusal expectation. A branch keyed on some property of the install
("did anything survive?") is refused: it would make the helper re-derive a fact its
caller already knows, and a helper that infers what it should have been told is how a
leg silently stops asserting.

**The three open questions the folded entry left are answered here, each on a probed
fact.**

- **Should an all-omitted install refuse at `init`?** **No.** `plan_gates()`
  (`installer/lib/init.sh:208-230`) writes the registry member by member with no
  all-omitted case, and exits 0. Refusing there would turn an honest omit-and-declare
  install — the only install an uncovered platform can have — into no install at all,
  losing the vendored kits, the manifest and the seeded surfaces along with the battery.
  The disclosure is already correct; what was missing is that the adopter is not told
  the *consequence*, which is the next answer.
- **What does `doctor` report?** A line it does not have today.
  `installer/lib/doctor.sh:130-145` reports per-reason omission counts and treats
  24-of-26 and 26-of-26 identically, so an adopter whose battery cannot run at all reads
  the same sentence as one who lost two members. `doctor` gains one line, emitted only
  when **no live member survives**, naming the consequence and the remedy. It is placed
  in `doctor` rather than `init` because `doctor` is the surface an adopter consults
  when something does not work, and this is the answer to that question.
- **Does the leg keep a non-zero omission count as its completeness assertion?**
  **Yes, unchanged.** `BARE_OMITTED > 0` still discriminates a leg that declared its
  omissions from one that declared none, and tightening it to *equals the whole roster*
  is refused: that number is derived from the vendored tree at `:243-254` and pinning
  the leg to it would make a leg fail on any future roster change that has nothing to
  do with the property being asserted.

### (7) All nine `.workflow/validate-baseline.txt` rows recover together, and the eight `ignore` rows are not eight independent recoveries

`.workflow/validate-baseline.txt:96-104` carries one `fail` row and eight `ignore` rows,
every one slugged `binary-less-dispatch-loop-retirement` {mechanical}. They move to
`pass` in the same commit, and the slug is dropped from each — evidence-kit/SPEC.md
§Baseline manifest requires a slug exactly when the status is `fail` or `ignore` and
forbids one on `pass`.

**Why they are one recovery and not nine, which matters because it is the difference
between a real assertion and a bookkeeping edit.** `fail()` is `exit 1`
(`run-smoke.sh:18`) and the binary-less leg runs first in file order, so the eight later
arms — download, toolchain-free, jq-less, upgrade, cross-version-reversal, seam,
narrowing and artifact — were never *reached*, let alone failed. They are `ignore`
because the run stopped, so the single repair in delta (6) makes all eight discharge for
the first time. **A build session therefore does not take their `pass` on faith**: the
file is held constant and edited by human commit only (evidence-kit/SPEC.md §Baseline
manifest), so the rows are rewritten only after a run in which those eight arms actually
executed and passed.

## Producers and consumers

This cut introduces **no new knob, no new file and no new log**. It moves an existing
producer (the front end's argv composition and its usage text) into an existing consumer
(the crate's argv parser), deletes a duplicate dispatcher, and changes one assertion in
one smoke leg. The survey below is over the whole component set — every tracked file was
grepped for `run-gates.sh` and for `binary-less`, with stderr left open.

**The interface whose producer moves: the front end's argv, as the crate receives it.**

- **Producer, before:** `run-gates.sh:165-305` composes a crate arm name and its argv
  from the front-end argv, then `exec_arm` asks the binary for that arm's knobs and execs
  it.
- **Producer, after:** `native/src/main.rs`'s normalization step, ahead of the `--knobs`
  door at `:405` and the arm-table door at `:434`. The stub resolves the repo root,
  sources `lib/gate.sh`, makes the one `gate_knob_env` call, and execs with argv
  unchanged.
- **Consumer:** `gates::knobs` / `emit::knobs` (`main.rs:417-419`) at the `--knobs` door,
  and `emit::lookup` / `gates::lookup` at the dispatch door. Both consume the normalized
  first token, which is why the step sits above both rather than inside either.
- **Enabling config:** none is added. The bridged environment is still resolved by
  `gate_knob_env` through `gate_knob_env_set`, batched by owning kit, and no knob's
  producer moves.

**The one datum that does not move, and its named reader:** the unavailable status of
delta (4). Producer, the stub's two-name test on the leading token. Reader, the stub
itself at the absent-binary branch, at exactly one transition — a dispatch attempted
against a binary that is not there. No other caller reads it, which is why it is not an
interface.

**Every caller of the front end, surveyed, with what this cut owes it.** All of them
keep working by construction, since delta (2) preserves the front-end spelling; the
column records what each *proves* rather than what each risks.

| caller | site | what it exercises |
| --- | --- | --- |
| harness statusline | `.claude/settings.json:98` | `--statusline`, delta (4)'s fail-open |
| harness hooks (four) | `.claude/settings.json:116,136,140,149` | `--hook`, delta (4)'s fail-open |
| CI battery and baseline diff | `.github/workflows/gates.yml:59,141` | bare run and `--diff-baseline` |
| the shipped workflow template | `gate-sdk/templates/gates-workflow.yml:44` | bare run in a consumer tree |
| vendoring | `installer/lib/init.sh:305` | `--emit graph`, delta (2)'s alias |
| adopter next-steps | `installer/lib/init.sh:414-415` | the printed `--install-hooks` and bare run |
| consumer smoke | `installer/consumer-smoke/run-smoke.sh:171,298,304` | the battery, and delta (6)'s leg |
| this kit's own smoke | `gate-sdk/smoke/install.sh` | bare, verbose, `--only`, `--help`, refusal, `--emit port-blockers` in three forms |
| the evidence suite commands | `scripts/evidence-config.sh:18-19,22,28` | the battery and `--upgrade-smoke`, baked into every kit's smoke through the generated hook |
| session context | `scripts/session-context.sh:10,13`, `context-kit/templates/session-context.sh:11` | `--emit stage-rules` |
| every stage template | `lifecycle-kit/templates/stages/*.md` | `--enter-stage`, `--emit close-surfaces`, `--emit session-id` |
| the guard template | `guard-kit/templates/settings-hooks.json:14,20` | `--hook` in an adopter's own settings |
| the front end's own remedy | `run-gates.sh:152` | names `build-native.sh`; **survives**, and the sibling declaration cut depends on it |

**The one reader that must change and is not a caller:**
`scripts/gate-tests/subagent-stop-reader.test.sh:78` asserts the absent-binary message
names `build-native.sh`. Delta (4) keeps that message, so the assertion holds — recorded
because it is the one place a test reads the diagnostic's *text* rather than its status,
and a reworded diagnostic would red it.

## Existing sections updated

- `gate-sdk/SPEC.md` §run-gates, the paragraph **"The front-end keeps a shell dispatch
  loop for one branch"** (`:9101-9114`) — its whole subject retires (delta 3). The
  criterion-6 admission and the executed-comparison discipline it records are rewritten
  as the closed history they become, not deleted: a later reader asking why the arm and a
  shell dispatcher were ever required byte-identical needs the answer.
- `gate-sdk/SPEC.md` §run-gates, the paragraph **"The front-end's port disposition"**
  (`:9116-9132`) — moves from a scheduled disposition to a landed one, and gains the
  three things it does not state: the residual argv grammar and its refused alternatives
  (delta 2), the unavailable-status residue and its honest limit (delta 4), and the usage
  text's relocation with its behavior change (delta 5).
- `gate-sdk/SPEC.md` §run-gates, the **two-halves** paragraph (`:9081-9090`) — the split
  it describes ("the front-end owns the argument grammar") is exactly what delta 2
  moves, so the sentence naming argv as the front end's is corrected at its source rather
  than left to contradict the disposition paragraph below it.
- `gate-sdk/SPEC.md` §run-gates, the **output contract** and the `--only` steer prose
  (`:9200-9226`) — unchanged in substance, re-attributed (delta 3): they are the arm's,
  and with the loop gone there is no second producer to hedge against.
- `gate-sdk/SPEC.md` §run-gates, the **`--only` selector's contract** — it gains the
  `--` argv channel, its single-member precondition and its multi-member refusal
  (delta 2). This is the one place the grammar is stated, and the sibling
  `preflight-front-end-cut` cites it rather than restating it.
- `gate-sdk/SPEC.md` §The port-candidate criteria, criterion 5 — its
  vendored-form-stays-runnable branch keeps its rule and loses its one shell instance
  (deltas 3, 6). The criterion is not retired: it still governs what an artifact-less
  host is *told*, which is what delta (6) re-scopes rather than removes.
- `gate-sdk/SPEC.md` §run-gate-tests — the required byte-identical transcript comparison
  between the two dispatchers (delta 3).
- `gate-sdk/SPEC.md` §The port disposition — the owed-corpus prose this cut moves by one
  file (all deltas). Per §The first cohort, a budget batch records only findings, and this
  cut's finding is delta (2)'s: where a front end's grammar and a binary's arm table must
  agree, the alias belongs in the file holding the table, because that is the only place
  the knob door and the dispatch door cannot drift apart.
- `installer/README.md` §The gate binary — the omit-and-declare install's adopter-facing
  description, which today promises a green battery on a host with no artifact
  (deltas 3, 6). This is the sentence `binary-less-dispatch-loop-retirement` was filed
  for: *it changes what an adopter is told they receive*.
- `evidence-kit/SPEC.md` §Baseline manifest — no rule changes; the nine rows it governs
  do (delta 7). Named as a target because the recovery must be read against the manifest's
  own slug rule rather than edited freehand.

<!-- update-target-exempt: the deleted assertion block's own spec comment is removed with the block, so no delta can claim it as a surviving target -->
- `gate-sdk/gate-tests/run-arm-contract.test.sh:116-117` — the `# spec:` naming criterion
  5's omit-and-declare branch, removed with the comparison it annotates.

## Definition of Done

- [ ] **Causal completeness** — the normalized argv has a named producer
      (`main.rs`'s step, above both doors) and named consumers (`--knobs` and the arm
      table); the unavailable status has a named reader at a named transition; no new
      field is added, and every caller of the front end is dispositioned above.
- [ ] **The oracle decides, not the roster** — `gate-sdk/smoke/install.sh` and
      `gate-sdk/gate-tests/run-arm-contract.test.sh` both green, and the tree re-grepped
      for `run-gates.sh` before the stub is called done.
- [ ] **The nine baseline rows are earned, not edited** — rewritten only after a
      consumer-smoke run in which the eight previously-unreached arms executed and passed.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); §run-gates reads as one document to a reader
      who never saw this amendment, with the criterion-6 admission preserved as history.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every spec grepped for the names this change retires
      (the loop, the byte-comparison, the front end's argv ownership); nothing dangles.
- [ ] **The oracle re-read, not the arithmetic trusted** —
      `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` re-run at the landing
      commit and its owed count recorded.
- [ ] **Gaps filed** — the fail-open-set parity assertion delta (4) names, and any
      cross-component gap found during the work, filed to the committed gap inbox.
