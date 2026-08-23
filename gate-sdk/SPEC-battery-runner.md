# SPEC amendment: battery-runner-port

The battery's **dispatch** moves into the binary. `gate-sdk/bin/run-gates.sh`
stops being a loop that resolves and execs one member at a time and becomes a
front-end that resolves one bridged environment and execs once; the registry
walk, the two selectors, the timings, the omission accounting, the output
contract and — new here — a worker pool land in a `--run` arm of the multi-call
binary.

**This unit is the port tail's first, operator-ruled 2026-08-23**
(TRAJECTORY.md §PRIORITY DIRECTIVE), on the ground that it is the only unit that
moves wall-clock and that every remaining shell member dispatches through the
seam it removes.

**The arithmetic is measured on this tree at this stage, not inherited.** A warm
battery is `TOTAL 24990` ms over 106 members (`.tmp/gate-timings.txt`). Resolving
`gate_command` for all 106 members and executing none of them is **5.019 s** —
and the 90 members that individually cost under 200 ms sum to **4841 ms**, so
essentially the entire cost of the battery's fastest 90 gates *is* the bridge.
`bash gate-sdk/bin/gen-pre-commit.sh --emit` is **6.637 s**, which is the whole
of `check-graph`'s 6976 ms. Against that, the binary answers `--knobs` in 1 ms
and `--list` in 1 ms.

**What the unit is not.** It builds no result cache, so
`gate-battery-result-cache` stays open and its inherited invalidation question
stays unanswered (delta 9). It does not delete `bin/gen-pre-commit.sh`, whose
non-port is a structural ruling this amendment holds rather than reopens
(§gen-pre-commit, ratified by the operator 2026-08-21) — so `check-graph`'s bash
spawn survives the unit and `graph-port-bash-spawn-residue` is made **cheaper,
not spent** (delta 5). And one bash process per run survives by construction,
because criterion 6 puts a knob's value in exactly one place and that place is
the owning kit's shell library (delta 3's opening paragraph).

**That last residue is the completion predicate's reach, and it is
operator-confirmed 2026-08-23 rather than read here.** TRAJECTORY.md §The closed
rulings states the port's completion as *the battery runs from the hook to the
binary with no bash in between*. The confirmed reading: the predicate governs the
**hook-to-binary path**, which carries no bash today and keeps none under delta
5's ruling; the surviving knob resolver is one bash process per battery run, and
this unit owns its **cost**, not its existence. No TRAJECTORY ruling moves.

**Two widening readings were put to the operator and declined**, named here so a
later session finds them decided rather than undiscovered. The first is the
**emit-arm path**: a `--emit-` member is bridged, so a crate-side hook emitter
could *receive* the resolved knob union through the bridge rather than compute
it — which is a plausible answer to §gen-pre-commit's structural premise and is
**declined for now**, not refuted. The second is a reversal of that section's
2026-08-21 ratification outright. Both are declined, so neither is live work and
neither is this unit's; a session that rediscovers the first should read this
paragraph as its disposition. `install-step-relocation` inherits the same
answer — the hook's shape does not change, so that entry is unblocked and
unchanged rather than resolved.

## What changes

### (1) `--run`: the battery runner is a bridged arm of the binary

The dispatch loop moves into the crate as a **non-gate arm** (§The non-gate
arm), reached as `--run` and resolved in `main` before the registry lookup.
**Design-bearing.**

It satisfies that class's three properties: it is `--`-prefixed and absent from
`--list`, which is what keeps §check-gate-substrate-parity assertion B's
descriptor-set equality true in both directions; it owes no `.gate` descriptor,
no `gates.list` registration and no `good/`+`bad/` pair, because it returns no
verdict a battery reads; and it names its caller — `bin/run-gates.sh`, and
through it every caller the census below records.

**It joins the *bridged* table, not the flag list, and that choice is forced
rather than stylistic.** §The non-gate arm rules that "only an emitter-table
member is bridged … a configured tool ported as a top-level flag therefore
resolves platform defaults and silently ignores every consumer override — which
is not a calibration between two workable shapes but the difference between
working and appearing to." The runner reads `GATE_SDK_GATES_DIR`, the kit roots
and `GATE_SDK_TMP_DIR`, so a hardcoded top-level flag is the second shape.

**So the table's contract is restated as its property.** What the existing
members share is not that each emits a document — `--emit-queue-index extent`
answers two integers and emits none — but that each is **bridged**: `--knobs`
publishes its roster and a front-end resolves it. The table is therefore the
**bridged-arm table**, keyed by the arm's own flag spelling, and `--emit-` is
demoted from a family name to a per-arm spelling. Nothing about the existing
arms changes; what changes is that a non-emitting member can join without a
third stretch of the word *emit*. §The non-gate arm already declined to rename
the family for `extent` on the ground that renaming is "a gate-sdk unit of its
own" — this is that rename, taken by the unit that needs it, and taken as a
generalization of the stated property rather than a new class.

**Its declared knob roster is derived, never maintained.** `--knobs --run`
answers the **union** of every registry member's declared knobs, every sibling
bridged arm's, and the runner's own three, computed from the crate's registry
and table at compile time. A maintained union would rot against a churning
roster the moment a member's knob set changed, which derivation-first forbids;
the union is exactly the data `gates::knobs` and the table already hold.

### (2) `bin/run-gates.sh` becomes the front-end and nothing else

The script keeps its whole argument grammar and every refusal, and loses its
body. **Design-bearing.**

**Preserved verbatim, because ~25 in-tree callers depend on it:** the
`[gates-dir]` positional; `--only <name>…`, `--for <path>…`, `--emit <arm>
[args…]`, `-h`/`--help`, `--`; the §The bin/-tool contract behaviors (help on
stdout at exit 0, an unrecognized leading-dash first argument as usage on stderr
at exit 2, a leading-dash name refused wherever it stands); the empty-list
refusals and their message shapes; and the `--only` steer that turns a
positional naming a default-registry member into a remedy line beside the
refusal. Every one of these is a documented surface, so a silent change is a
documented flag that does nothing — the state §The non-gate arm calls worse than
no flag.

**What it does:** resolve the repo root, source `lib/gate.sh`, parse argv,
resolve the `--run` arm's bridged environment in **one** `gate_knob_env` call,
and `exec` the binary with the parsed selection forwarded. `--emit <arm>` keeps
its existing shape unchanged — it is already exactly this, one arm resolved and
exec'd, which is the precedent the runner's own arm is built on.

**What leaves:** `select_for`, `select_only`, `pathspec_matches`, the per-member
`gate_command` capture, the timing arithmetic, the omission report and the
summary. `gate_command` itself is **not** deleted: the census finds fifteen
caller sites across seven kits — `run-gate-tests.sh`, `install-hooks.sh`,
`gen-pre-commit.sh`, `run-consumer-smoke.sh`, `lib/test-hermetic.sh`,
`lifecycle-kit/bin/enter-stage.sh`, `scripts/gate-exec.sh` among them — so the
runner stops being a caller and the function keeps its contract.

### (3) Knob resolution batches by owning kit, and the value is proved member-independent

The bridge's cost is one subshell per *(member × declared knob)*, each sourcing
the owning kit's whole `lib/*.sh`. It becomes one subshell per *owning kit per
run*. **Design-bearing.**

**The precondition is `config-bridge-resolution-cost`'s own open question, and
it is answered by probe rather than assumed.** That entry defers on "whether one
knob's resolved value may legitimately differ between two members in the same
run". It may not, **by construction today**: `_gate_knob_value` takes the gate
name as its second argument and uses it in exactly one place, the
does-not-define refusal's message; resolution reads the knob name and the
sourced library and nothing else, and `_gate_knob_prefix_values` is identical in
this respect. A knob's value is therefore a function of the knob and the tree's
config alone, which is what makes a per-run resolution value-identical to a
per-member one. Recorded here because the entry filed it as the question the
choice turns on, and a batching delta that did not answer it would be assuming
the thing.

**The mechanism.** `gate_knob_env` takes a *set* of names, partitions it by
`_gate_knob_owning_kit`, and for each kit sources that kit's `lib/*.sh` once in
one subshell and emits every knob owned by it. Emission order is the requested
order, so the argv a caller receives is unchanged. The prefix arm resolves
inside the same per-kit subshell.

**`GATE_SDK_RESOLVING_KNOB` becomes a set** — the requested names for that kit,
one per line — and its readers test **membership** rather than equality. The
live readers are canon-kit's five gated blocks (`canon-kit/lib/spec.sh`, the
enum-set, measured, install-transport, payload-claim and claim-class id/pattern
pairs), whose whole purpose is that a knob costing a consumer subprocess is
computed only when asked for. Under a batch they compute once per run instead of
once per requesting member, which is the saving, and a block whose pair is not
in the set still does not compute, which is the property preserved. This is the
one cross-kit contract change in the unit and it is why canon-kit is a second
component here.

**The four refusals are unchanged** and still name the offending knob (and, on a
keyed knob, the offending key): a newline in an element, a tab in an element, a
key containing `=`, and a knob the owning kit's library does not define. They
are cited rather than counted, because the count has already drifted once — the
`battery-runner-port` entry and `lib/gate.sh`'s own `spec:` line both say
*three*, which the keyed arm superseded. Delta 10 fixes both spellings.

**A refusal in a batch names its own knob and fails the whole call**, exactly as
today: the bridge is fail-closed and a partially-resolved environment is the
fail-open dressed as a default that the does-not-define refusal exists against.

**Its second reader is the hook generator**, which is where the unit's claim on
`config-bridge-resolution-cost` is actually paid: `command_rel` calls
`gate_command` for all 102 `tier=precommit` members, so `--emit`'s 6.637 s is
that same per-knob fan-out with a manifest read on top. The generator's own
lines stay shell (delta 5); what changes underneath it is this delta.

### (4) A member dispatches as a child process, and the threads are the concurrency

The `--run` arm runs each member in a child process and supervises the children
from a worker pool. **Design-bearing.**

**The in-process call was weighed and refused, on three grounds and one
measurement.** A compiled member is a function in the same binary, so calling it
directly is available and looks free.

- **It would silently retire the declared-knob discipline.** A gate reads its
  knobs from the process environment. In-process dispatch means one environment
  carrying the union, so a member reading a knob it never declared would
  *succeed* — and `--knobs` exists precisely so that "the crate declares only
  the knobs its own code reads" is held to executed behavior. That is the
  does-not-define refusal's failure in the other direction, and nothing would
  catch it.
- **It would lose fault isolation.** A member that panics or aborts must red
  *that member*; in one process it takes the battery with it. `run-gates.sh` has
  this for free from `exec` and the port must not spend it.
- **It would fork the runner in two.** Six of 106 members resolve to `.sh`
  today, and consumer shadowing of a ported member with a shell script is a
  permanent contract (§lib/gate.sh, `.sh` beats `.gate` within a dir). A
  dispatch path that only reaches compiled members needs the exec path anyway,
  so the in-process arm buys a second implementation rather than replacing one.
- **The measurement prices what is forgone**: the binary answers in 1 ms, so 106
  self-execs cost on the order of 0.1–0.2 s against the 5.019 s this unit
  removes — under one percent of the saving.

**`env` leaves the battery's dispatch path.** The runner sets each child's
environment directly rather than prefixing an `env` argv element; §lib/gate.sh's
sanction for `env` was that "the dispatcher is already bash", and this
dispatcher is not. The `env` prefix stays in the generated hook's baked argv,
which is still emitted by a bash producer.

**The dispatch-stderr seam is discharged by removal rather than reproduced.**
`.tmp/gate-dispatch-stderr.txt` exists because `gate_command`'s stdout *is* argv
and a stray diagnostic on the merged stream becomes `argv[0]`. The `--run` arm
builds argv as data and never parses a stream into it, so the failure class is
structurally absent; the file retires, having no reader in tree outside the
runner's own error branch. **The rule stays in `lib/gate.sh`'s contract** for
`gate_command`'s fifteen surviving callers — what retires is this runner's
capture file, not the seam.

### (5) The generated pre-commit hook keeps its baked per-gate argv

The entry's open design question. **Ruled: the baked per-gate argv list is
retained; the two-line `run --hook` shim is refused.** **Design-bearing.**

The shim is attractive — it would delete ~450 generated lines, end
regeneration-on-config-edit, and put one matcher where there are two. Four costs
refuse it, and the first is decisive.

- **A platform with no published artifact would lose its whole hook.**
  Criterion 5's ruled install model omits a member the host has no verified
  binary for from `gates.list` and records it there; the generated hook on such
  a consumer still carries its **shell** members' blocks and still runs them. A
  hook whose entire body is `exec <binary> --run --hook` has nothing to exec.
  The unsupported-platform branch that criterion 5 exists to keep alive would
  die at the hook.
- **It moves resolution onto every commit.** The baked hook pays zero at commit
  time; a shim re-walks the registry, re-reads every manifest and re-resolves
  the knob union on each one.
- **It retires `check-graph` assertion D's subject.** The hook would stop being
  a projection of the manifests, so nothing would hold a manifest edit to the
  hook — and §The non-gate arm names that exact failure: "the gate simply never
  runs on the edit that broke it, and only a full battery finds it."
- **It kills the `gen=manual` round-trip**, a shipped extensibility point with
  no replacement in the shim.

**What the ruling leaves standing, stated rather than discovered later.**
`check-graph` keeps spawning `bash bin/gen-pre-commit.sh` for assertions D and
E, so `graph-port-bash-spawn-residue` is **not** discharged by this unit — its
cost falls with delta 3 and its subject survives. `install-step-relocation`'s
`gen-pre-commit.sh --write` step stays un-relocated, which is the binding the
entry named: the answer is that the hook's shape does not change, so that entry
is unblocked and unchanged rather than resolved.

**This does not reverse TRAJECTORY.md §The closed rulings' own two-line-shim
reading — it is scoped ahead of this unit's corpus, not against it.** That
paragraph states what *port complete* means, tied to `shell-gate-tail-port`,
whose completion is exactly the six-of-106 `.sh` residue this refusal turns
on going to zero: once no member dispatches to `.sh`, criterion 5's stranded-
platform branch has no shell block left to lose, and the shim TRAJECTORY
describes stops costing what it costs here. This unit's refusal is therefore
the correct reading *now*, not a narrower ruling standing against a wider one
already made — the two are the same ruling read at two different corpus
states.

**And the `staged_matches` dual implementation is priced here, not elsewhere.**
§run-gates rules `--for` "identical to the generated hook's staged-path
matching", and the hook's copy is `bin/gen-pre-commit.sh`'s verbatim awk splice
of `gate_staged_matches`' body. Moving `--for` into the crate makes that two
implementations with live consumers on both sides — criterion 6's *unless*
clause exactly, which admits the duplication only where a machine notices
divergence. The discharge is the shape queue-kit's `lib/queue.sh` already uses:
an executed cross-substrate comparison over one canned corpus of glob/path
pairs, run by the owning kit's fixture lane, standing rather than taken once
(delta 8).

### (6) The concurrency contract

`gate-battery-parallel-execution` filed the contract and named four parts; this
delta is all four. **Design-bearing.**

- **Workers.** `std::thread` and `std::sync` only — **no new crate dependency**.
  The crate carries exactly one today (`serde_json`), and objective 4 makes
  footprint a first-class cost paid per target on every adopter's machine, so a
  scheduler crate is a cost the standard library already covers. Worker count
  defaults to `std::thread::available_parallelism()`.
- **`GATE_SDK_JOBS` is the one new knob**, gate-sdk's, config-via-env per the
  kit convention; `1` restores the serial run, which is what makes a suspected
  interference reproducible without a rebuild. It is spelled as env rather than
  argv against §run-gates' own ruling on that line: the ruling puts *selection*
  in argv because an ambient selector would silently narrow a battery while the
  summary still reported a pass, and worker count changes no member's
  membership — it is execution configuration, the same class as
  `GATE_SDK_VERBOSE`.
- **Deterministic output ordering.** Each member's captured output is buffered
  and flushed in **registry order**, never completion order, so a red reads the
  same way twice and a transcript diffs against itself. The summary's failed-set
  is likewise registry-ordered.
- **The timings file stops being a contended writer.** Per-member elapsed times
  are collected in memory and written once after the join, in registry order,
  `TOTAL` last — the grammar `drift-kit/kpis/kpi-gate-runtime.sh` and
  `drift-kit/bin/drift-report.sh` read, unchanged. `TOTAL` becomes the sum of
  per-member times, as it is today, and therefore stops equalling wall-clock;
  it always was the sum, and under a pool the difference becomes visible, which
  is worth stating for the two readers above.
- **Per-gate scratch isolation, split by what the directory is for.** Each child
  gets a private `TMPDIR` under the run's scratch, which is where a member's
  *anonymous* temporaries land. `GATE_SDK_TMP_DIR` stays **shared and is not
  isolated**, because it is the home of a declared, content-keyed cache whose
  whole value is surviving the run: `check-crate-arms` writes
  `crate-arms-<hash>.green` there and reads it on the next battery, and a
  per-member scratch would silently retire the cache the 2026-08-23 ruling
  bought. The rule the split states, and which a later member is measured
  against: **an anonymous temporary is private; a declared cache is shared and
  its filename must carry its key.**
- **The projection-ordering constraint is discharged by an invariant, not a
  scheduler.** The filed entry names "an ordering constraint between gates that
  regenerate projections and gates that read them". Measured at this stage: a
  full `run-gates.sh` over this tree reports `All 106 gates passed.` and leaves
  `git status --porcelain` byte-identical and `.tmp/` with no new entry — no
  registered member regenerates anything another reads, because the freshness
  family compares against an in-memory render rather than writing one. The
  contract states that as the invariant a member must not break, and the
  scheduler assumes it; a member that wrote a projection another member read
  would be a defect against this contract rather than a scheduling input.

**The subsumption reading, stated because the queue does not reconcile it.**
`gate-battery-parallel-execution`'s own clause says "the port subsumes this",
naming `native-gate-binary-port` — the whole-corpus entry, which predates this
unit's slug. Its stated deliverable is "the concurrency contract, not the
scheduler", and this delta is that contract in full, so the entry is discharged
here on its own terms rather than deferred to the wider entry it happened to
name.

### (7) One crate-side registry module, and the copies collapse onto it

The `--run` arm needs the registry walk, declaration resolution and the manifest
fields the descriptor grammar carries; five gate modules already carry a private
copy of `gates_list_members`' grammar. **Design-bearing.**

A `native/src/registry.rs` owns: the `gates.list` member grammar (neither blank
nor comment); declaration resolution consumer-first across the kit check dirs
with `.sh` beating `.gate` **within** a dir; the manifest-field read; and
`couples=`/`trigger=` expansion. `gate_binary_fresh`, `kit_enum`,
`gate_fixture_coverage`, `core_files` and `graph` drop their copies onto it.

**It is a universal layer, so no descriptor gains a coupling for it.** §The non-gate
arm's transitive-coupling rule reaches "the modules whose edit can change *this*
member's verdict and nothing else's", and stops at the layers every gate reaches
— naming a universal layer in each descriptor would spell one fact per member
and re-run the whole battery from the hook on any edit to it. The holders are
§check-crate-arms and the binary's source stamp, exactly as for `walk.rs` and
`proc.rs`.

### (8) Fixtures, tests and the two standing oracles

**Design-bearing**, because two of the four are new oracles rather than new
cases.

- **A `--run` arm owes no `good/`+`bad/` pair** (§The non-gate arm). Its
  coverage is the crate's own tests plus the two comparisons below.
- **The `staged_matches` cross-substrate oracle** (delta 5): one canned corpus
  of glob/path pairs fed to `gate_staged_matches` and to the crate matcher,
  verdicts compared byte for byte, standing in the fixture lane. This is the
  criterion-6 discharge and it expires at no edit, which is what separates it
  from a parity proof taken once.
- **A determinism oracle**: the same battery run twice under `GATE_SDK_JOBS=1`
  and under the default, with the two transcripts required byte-identical modulo
  the timings file. Without it, ordering determinism is advice.
- **`gate-sdk/gate-tests/run-*.test.sh` widens** to hold the front-end's
  refusals and the arm's output contract: the exact green phrase, each
  `FAIL:` tail shape (`(exit N)`, `(dispatch harness error, exit 2)`,
  `(unresolved)` — cited rather than counted, on delta 10's own ground, since
  today's tree carries three and a child-process dispatch may add a signal
  shape this amendment does not name), the omission line staying off the
  summary line, the `--only` unregistered-name refusal at exit 2, the `--for`
  uncoupled-path note at exit 0, and the help/unrecognized-option split across
  stdout and stderr.

### (9) `gate-battery-result-cache` stays open

Recorded as a delta so the disposition is a decision rather than an omission.
**Mechanical.**

The unit builds no cache. What it does deliver is the precondition the entry
named — the couples-keyed bookkeeping becomes an in-process map rather than
shell — so the entry's *shape* improves and its **invalidation** question is
untouched: whether a gate whose real inputs exceed its declared `couples=` would
be skipped while stale, and whether `docs-renderer-batch-contract`'s 2026-08-01
content-hash refusal generalizes. Both stay that entry's first work.

### (10) The stale refusal count is corrected at its one remaining spelling

**Mechanical.**

`gate-sdk/lib/gate.sh`'s `gate_command` `spec:` line says "each of the three
knob-resolution refusals". §lib/gate.sh has said **four** since the keyed arm
landed. It is de-literalized to cite the section rather than carry a count,
which is the de-literalization rule applied to the thing that already drifted.
**The queue entry carried the same stale count once** ("the bridge's three
refusals kept") but its own promotion edit already de-literalized it to "every
refusal the bridge states kept … which owns the roster and its count" — verified
on this tree at align, so build's remaining work here is the one code comment,
not two spellings.

## Producers and consumers

**`--run` (new arm).** *Producer:* `bin/run-gates.sh`'s `exec`, on every bare or
selected battery run — the enabling configuration is the bridged environment
that same script resolves, which is emitted on every invocation and not only
under test. *Consumers:* the CI workflow `.github/workflows/gates.yml` and the
shipped `gate-sdk/templates/gates-workflow.yml` (exit code); `bin/upgrade-smoke.sh`,
`bin/run-consumer-smoke.sh`, `installer/consumer-smoke/run-smoke.sh`,
`demo/run-demo.sh`, `context-kit/smoke/agents-md.sh` and
`gate-sdk/smoke/install.sh` (the green phrase, by grep);
`gate-sdk/gate-tests/run-dispatch-streams.test.sh` (the green phrase, exact
string); `scripts/parse-gates-log.sh` (the `PASS:`/`FAIL:` tails), reached
through `scripts/evidence-config.sh`'s `EVIDENCE_KIT_RUN_gates` and read by
evidence-kit's `gates` suite as its scenario record.

**`--knobs --run`.** *Producer:* the crate's compile-time union over
`gates::REGISTRY` and the bridged-arm table. *Consumer:* `gate_knob_env`, called
once by the front-end before the exec. Its **named reader** for every element is
the crate module that reads that knob — the union is a projection of readers, so
an element with no reader is not expressible.

**`GATE_SDK_JOBS` (new knob).** *Producer:* the consumer's environment or
`scripts/gate-sdk-config.sh`; unset resolves to
`available_parallelism()`. *Consumer:* the `--run` arm's pool constructor, read
once at start. *Reader at a named transition:* the worker-count decision, before
the first member is dispatched. It is deliberately **not** bridged — it is
gate-sdk's own execution config on `GATE_SDK_VERBOSE`'s terms, read from the
environment the front-end already exports.

**Per-child `TMPDIR` (new state).** *Producer:* the `--run` arm, per member,
before spawn. *Consumer:* whatever the member's implementation or a spawned
program writes without naming a directory. *Named reader:* none in tree today,
which is the point — the value is that an unnamed temporary cannot collide
between two concurrently running members. `GATE_SDK_TMP_DIR` is untouched and
its named readers are `check-crate-arms`' cache, the timings file, and
drift-kit's two readers of it.

**Retired: `.tmp/gate-dispatch-stderr.txt`.** Its only reader was
`run-gates.sh`'s own exit-2 branch. **Red condition named, per the
causal-completeness check's point 5, because this is a narrowing:** no gate reds
on this file's absence — no `couples=` names it and no manifest, count or
coverage assertion reaches it; the only readers are `drift-kit`'s two, and those
read `gate-timings.txt`, which survives with its grammar intact. The narrowing
therefore adds no violation, which is the claim that has to be checked rather
than assumed.

**Unchanged producers whose consumers must be re-verified at build**, because
the runner stops being their caller and the contract is what they read:
`gate_command` (fifteen sites), `gate_resolve` (seven), `gates_list_members`
(nine), `gate_staged_matches` (two — the runner's `--for` and the generator's
awk splice, which delta 5's oracle now holds together).

## Existing sections updated

- **gate-sdk/SPEC.md §run-gates** — rewritten from a script's contract to an
  arm's contract plus a front-end's argument grammar: the output contract, the
  two selectors, the omission accounting and the env-versus-argv ruling stay
  where they are and gain the worker-count line; the dispatch-capture paragraph
  narrows to `gate_command`'s surviving callers (deltas 1, 2, 4, 6).
- **gate-sdk/SPEC.md §lib/gate.sh** — the array-knob config bridge's resolution
  becomes per-kit-batched, `GATE_SDK_RESOLVING_KNOB` becomes a set, and the
  member-independence of a resolved value is stated as the property the batch
  rests on; the `env`-in-the-dispatch-path paragraph gains the runner as the
  caller it no longer applies to (deltas 3, 4, 10).
- **gate-sdk/SPEC.md §The non-gate arm** — the class's first bridged
  non-emitting member, and the table's restatement from *emitter* to *bridged
  arm* with `--emit-` demoted to a spelling (delta 1).
- **gate-sdk/SPEC.md §gen-pre-commit** — the non-port ruling is **held**, and
  the section records that the generator's cost falls with the batched bridge
  while its spawn survives; the hook's shape is stated as ruled rather than open
  (deltas 3, 5).
- **gate-sdk/SPEC.md §check-graph** — the measured 77%-is-the-two-spawns figure
  is superseded by the batched bridge and re-measured at build; assertion D's
  subject is unchanged, which is the ruling delta 5 takes (deltas 3, 5).
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 6** —
  the `staged_matches` dual implementation joins `lib/queue.sh` as a worked
  instance of the *unless* clause discharged by a standing comparison rather
  than a deletion (deltas 5, 8).
- **gate-sdk/SPEC.md §Layout and configuration** — `GATE_SDK_JOBS` joins the
  knob roster with its default (delta 6).
- **canon-kit/SPEC.md §lib/spec.sh** — the five `GATE_SDK_RESOLVING_KNOB`-gated
  resolutions test set membership rather than equality (delta 3).
- **evidence-kit/SPEC.md §bin/run-validate.sh's `gates` suite**, wherever it
  states what the scenario parser reads — the tail-line grammar is unchanged and
  the section says so explicitly, because it is the surface a reader would
  otherwise have to re-derive (delta 6).
- **drift-kit/SPEC.md §Bundled KPIs**, on the `kpi-gate-runtime` entry — `TOTAL` is the
  sum of per-member times and under a pool no longer approximates wall-clock
  (delta 6).
<!-- update-target-exempt: a no-change confirmation, owned by no delta by construction -->
- **CLAUDE.md §This repo is governed by its own kits** — the battery command is
  unchanged, which is why this target is listed: the sentence stays true and the
  build must confirm it rather than assume it.
<!-- update-target-exempt: a no-change confirmation the ruling produces, owned by no delta -->
- **docs/site-architecture.md §Generated projections** — the pre-commit hook's
  row is unchanged under delta 5's ruling; listed for the same
  confirm-rather-than-assume reason.
- **`native/src/main.rs`'s usage line and `bin/run-gates.sh --help`** — both
  enumerate the arms, so both gain `--run` (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (`gate-dispatch-stderr.txt`, the per-member bridge's per-knob
      subshell, the *three refusals* spelling); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
