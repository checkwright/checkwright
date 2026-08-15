# SPEC amendment: emitter-substrate

**One ruling, owed once and applied six times.** The generated-projection freshness family is
six comparator+emitter pairs, and the question that has held it is not which members go first
but *what a ported emitter **is*** — the substrate contract describes a ported **gate** (a
`.gate` descriptor, a `good/`+`bad/` fixture pair, a `gates.list` registration) and has no
representation for a thing in the binary that emits a projection instead of returning a verdict.
This amendment answers that, ports the value-rollup triple against the answer, and retires the
hold the unruled question was placing on a member outside this cohort.

**The cohort is the operator's, not this amendment's**, and it is not re-argued: the census that
widened the family 3 → 6 and the ruling that scoped this unit to the value-rollup triple —
`gate-sdk/bin/enforcement-map.sh`, `context-kit/bin/footprint.sh`, `scripts/gen-value-rollup.sh`,
plus the two still-shell comparators over the first two — are the queue entry's.
`scripts/gen-docs-mirror.sh`, `drift-kit/bin/trajectory.sh` and `queue-kit/bin/roadmap.sh` stay
on that entry for a following cohort, and the ruling below is written to be applied to them
unchanged.

**Two of the three candidate designs are already foreclosed, so the open call is narrower than it
reads.** §check-roadmap-fresh's hold names three: (a) a ported comparator shells out to a shell
emitter, (b) the emission format is reimplemented in Rust *beside* the surviving shell script,
(c) the emitter itself collapses onto the binary. gate-sdk/SPEC.md already records that only (c)
pays — a ported byte-comparator spawning a shell emitter removes no shell — and TRAJECTORY.md
§PRIORITY DIRECTIVE closes the rest of the gap: *everything portable ports — the gates, the
runners and the install-lifecycle scripts alike*, and surviving shell is *residue justified case
by case, never a protected category*. (a) leaves the shell; (b) leaves it and adds a second
spelling to maintain. Neither is available. **What was genuinely open is the representation, and
that is what §1 rules.**

**The answer is not a new concept.** Stated first because the temptation here is to invent a
parallel descriptor format for emitters: the binary already carries six arms that are not gates,
and they already have a stated placement rule with a stated reason. The emitter port reuses it.

## What changes

### 1. The non-gate arm becomes a named class of the substrate contract

**`gate-sdk/SPEC.md` §Porting a gate to the binary substrate gains the class it has been using
without naming** — design-bearing, because naming a class fixes what may join it and on what
terms **[design-bearing]**.

The binary is a multi-call binary whose *gate* subcommands are dispatched by name out of
`gates::REGISTRY`. It also carries arms that are **not** gates — today `--list`, `--reads`,
`--knobs`, `--source-stamp`, `--queue-parity` and `--declaration-parity` — and each is spelled as
a **top-level `--`-prefixed flag handled in `main` before the registry lookup**, for one reason
its own `spec:` comment states in place: `check-gate-substrate-parity` assertion B equates the
`.gate` descriptor set with exactly the roster `--list` prints, so an arm inside that roster
would read as *a subcommand nothing declares* and red the gate. The placement is therefore
load-bearing rather than stylistic, and it is currently discoverable only by reading three
comment blocks in `native/src/main.rs`.

The class is named **non-gate arm** and specified by three properties:

- **It is a top-level `--`-prefixed flag, resolved before the registry lookup**, and it is
  **absent from `--list`**. This is what keeps assertion B's equality true in both directions.
- **It owes no `.gate` descriptor, no `gates.list` registration, and no `good/`+`bad/` fixture
  pair** — those three are the *gate* contract, and they attach to the thing that returns a
  verdict a battery reads. An arm that returns a document has no pass and no fail to fixture.
- **It owes a named reader instead.** A gate's reader is the battery; a non-gate arm has to name
  the caller that reads its output and the transition where it is read, or it is dead weight.
  Every existing member satisfies this (`--source-stamp` is read by `check-gate-binary-fresh`;
  `--queue-parity` and `--declaration-parity` by their parity harnesses), and stating it is what
  stops the class becoming a place to park unreachable code.

**Why a named class rather than three more comments.** The comments each explain one arm's
placement to whoever reads that arm. What none of them can do is tell a session *arriving with a
new non-gate thing to port* that a class exists and what it costs — which is precisely the
session this amendment exists for, six times over.

### 2. A ported emitter is a non-gate arm, `--emit-<projection>`

**The representation question is answered by membership rather than by invention** —
design-bearing, because it is the ruling the queue entry says this unit owes **[design-bearing]**.

A generated projection's emitter becomes a non-gate arm named `--emit-<projection>`, where
`<projection>` is the projection's own name: `--emit-enforcement-map`, `--emit-footprint`,
`--emit-value-rollup`. Its stdout is byte-identical to what the shell emitter's `--emit` printed,
because the committed projection is unchanged by this unit and its freshness gate byte-compares
against it.

**With exactly one exception, which is §9's and has to be carved out here or §2, §7 and the
Definition of Done contradict it.** Each of the three emitted documents contains its **own
generator citation** — the "generated by `bash <script>`, do not hand-edit" line the page carries
about itself — and §9 moves every such citation onto the new regen command. So the ported
emitter's stdout differs from the shell emitter's on precisely those lines, and the committed page
changes by precisely those lines. Everything else — every measured figure, every row, every
heading, the class taxonomy — is unchanged, and a diff anywhere else is the port defect the
Definition of Done means.

**This is sequenced rather than waived, so the parity arm keeps its full strength.** The port is
proved in two steps: the compiled emitter first renders with the citation **unchanged** and is
diffed byte for byte against the shell emitter (§7's check, literally byte-for-byte, on the
measured content); only then is the citation moved to the new regen command as §9's mechanical
edit, and the page regenerated. A parity arm that had to allow a known-differing line would be
proving something weaker, and the weakening is avoidable for the cost of running it before the
one-line change instead of after.

Three properties follow and are stated so the following cohort inherits them:

- **The shell emitter is deleted, not kept.** That is the whole dual-maintenance win, and a port
  that leaves the script standing has bought (b) — the design already refused.
- **The `--emit` flag on the old scripts does not survive as a compatibility spelling.** These
  are repo-internal generators reached from documented regen commands and from their own
  comparators, both of which move in this unit; there is no third caller, and a compatibility
  arm would be a second spelling of exactly the thing being retired.
- **The arm is the only entry point *into the crate*; the emission itself is a library function.**
  Each emitter lands as `pub fn emit(...) -> Result<String, …>` in its own module, with the
  `--emit-…` arm a thin `println!` wrapper. §4 and §5 are why that split is not decoration.
  **A documented regen command stands in front of the arm, and must**, because a non-gate arm
  receives no configuration: the config bridge is built by `gate_command` for a `.gate`-declared
  member alone, and `native/src/walk.rs`'s `kit_roots` is transported rather than re-derived by
  standing crate invariant, so a bare binary invocation cannot resolve what the enforcement-map
  emitter reads. The front-end is an arm on `gate-sdk/bin/run-gates.sh`, which already sources the
  shell library and already owns bridged dispatch, so the emitter port adds no shell file and the
  526 lines of emission logic still die. This is a *reachability* claim discharged, not a second
  spelling: nothing else may enter the crate's emission path.

**Why `--emit-<projection>` and not a single `--emit <projection>` taking an operand.** The
operand form reads better and is refused: it puts the projection's identity in *data* rather than
in the arm, so a typo resolves at run time instead of at the match, and — decisively — it invents
a dispatch table for a set the crate otherwise keys by function name, which is the "no mapping
table exists to drift" property `gates/mod.rs` states for the gate roster and there is no reason
to abandon one arm-class over.

**Both premises are about Rust, so this refusal governs the crate arm and nothing else.**
`run-gates.sh` is already a dispatcher, already carries a dispatch table by nature, and has no
checked match to protect, so neither premise transfers to it. The shell front-end therefore spells
the projection however that script already spells dispatch, operand form included, while the crate
arm stays `--emit-<projection>`. Recorded so a later reader does not read the two surfaces as
governed by one rule.

### 3. The three emitters port, and `enforcement-map.sh`'s `jq` dependency dies with it

**Three shell scripts (526 lines) become three modules; the port is not uniform, and the
differences are the work** — design-bearing, because each carries a derivation, not a
transcription **[design-bearing]**.

- **`context-kit/bin/footprint.sh` (129 lines)** is the cheapest: its inputs are a tracked-kit
  roster and per-file line/byte sums, and `native/src/walk.rs` already carries `kit_roots()`,
  `find_files` and `glob_files`. Its one non-obvious behavior is the always-loaded marker-block
  extraction it does with `awk`, which becomes a reader over the same marker grammar §6 gives a
  writer for.
- **`gate-sdk/bin/enforcement-map.sh` (273 lines)** is the substantive one. It reads `gates.list`,
  the KPI list, the settings file and a sourced evidence config, and it **shells to `jq`** to read
  the hook rows out of `.claude/settings.json`. `native/src/json.rs` already wraps `serde_json`,
  so the port **retires a non-floor program from the battery's own path** — `jq` is not in
  `GATE_SDK_PROGRAM_FLOOR`, and this is a dividend of the port rather than a separate unit.
  Stated with its boundary: this is the *battery* surface. It does **not** discharge
  `installer-jq-silent-degradation`, which is the shipped installer's own read path and a
  different surface with a different failure.
- **`scripts/gen-value-rollup.sh` (124 lines)** is the join, and §5 is its whole story.

The evidence config that `enforcement-map.sh` reaches by `source` is the one input whose port is
not mechanical: a sourced shell file is not readable by a Rust module without re-implementing
shell assignment semantics. **It is not a flat `KEY=value` file and cannot be read as one.**
`scripts/evidence-config.sh` builds `EVIDENCE_KIT_SUITES` by iterating `gate_fixture_suites`,
declaring one `EVIDENCE_KIT_RUN_<suite>` per row as it goes; that loop contributes eleven of the
twenty-four suites, and the remaining thirteen are consumer declarations no derivation reaches. A
flat reader therefore resolves neither half on its own, and the crate cannot recover the roster by
deriving it either.

**So the ported emitter receives its configuration rather than parsing it**, through the
substrate's existing config bridge — the same transport every ported gate already rides, carrying
values the shell library resolved by sourcing the owning kit's `lib/*.sh`, which sources the
consumer config, which runs the loop. Nothing about the evidence config is special to the crate;
it is simply not the emitter's job to interpret it. What this costs is one extension to the bridge
protocol, §3a below, and no new `EVIDENCE_KIT_*` knob at all.

**The port *removes* a knob read rather than adding one, which is worth stating because
`EVIDENCE_KIT_CONFIG_FILE` looked like the hardest input and turns out not to be an input at all.**
The shell emitter reads that path to `source` the file; the ported emitter receives
`EVIDENCE_KIT_SUITES` and the `EVIDENCE_KIT_RUN_*` family across the bridge and never opens it, so
the knob leaves the emitter's declaration entirely. Both of its behaviours are preserved **by their
rightful owner** rather than re-implemented: a consumer that has not adopted the registry resolves
`EVIDENCE_KIT_SUITES` to the empty array `evidence-kit/lib/evidence.sh` defaults it to, so the
Validate-suites section is absent exactly as before; and a consumer whose knob is *explicitly set to
a missing file* hits that library's own `exit 2`, which fires inside the bridge's resolution
subshell and refuses the whole invocation. The adopted-but-broken-refuses / not-adopted-degrades
distinction the shell emitter spelled out for itself is therefore kept without the emitter spelling
anything, and **its set-ness could not have crossed the bridge anyway** — a guarded default would
have collapsed the two modes into the refusing one.

**The three interests the flat-reader mechanism was carrying survive intact, and they are what
this design is held to:** no subprocess spawned from inside the emitter — the failure §4 retires;
**fail-closed rather than silent-empty** on a value that does not resolve, naming what did not;
and **no shell interpreter reimplemented in Rust**, which receiving rather than parsing satisfies
completely rather than approximately.

### 3a. The config bridge gains a prefix-form knob declaration

**A declared knob name may be a prefix, and the bridge resolves every variable matching it** —
design-bearing, because it widens a protocol every ported gate rides **[design-bearing]**.

`--knobs` publishes a **static** roster, so a member cannot name knobs whose names depend on
another knob's *value* — which is exactly `EVIDENCE_KIT_RUN_<suite>`'s shape. The prefix form
closes that, and the widening lands in gate-sdk's bridge rather than in evidence-kit's knob
roster, which is why this is the shape the operator took: no consumer-facing contract gains a
member.

Three properties, each stated because leaving it to the implementation is how a protocol acquires
an accidental contract:

- **Ordering, anchored explicitly.** The bridge resolves a prefix against the variables defined at
  the point `_gate_knob_value` has finished sourcing the owning kit's `lib/*.sh` — the same
  instant a static knob's value is read, not some later one. "After the kit lib is sourced" means
  after *that* source completes, which is what puts the consumer config's loop-declared variables
  in scope, since the kit library is what sources the consumer config.
- **Fail-closed, and distinct from a static knob's empty.** A static knob that resolves to the
  empty string is a *resolved-empty set* and passes; a **prefix matching no variable at all** is a
  refusal naming the prefix, because the member asked for a family and the family is absent — the
  same distinction `walk.rs` already draws between an unset bridged knob and an empty one, applied
  one level up.
- **A prefix is a resolution set, never a roster.** It says *resolve values for these names*, never
  *these are the members*. The roster comes from the roster knob: `suite_rows` iterates
  `EVIDENCE_KIT_SUITES` and looks each name up. This is load-bearing rather than pedantic —
  `EVIDENCE_KIT_RUN_ID` matches the prefix and is evidence-kit's run identifier, not a suite. A
  reader treating the matched set as the roster would emit it as one.

### 4. The two comparators port, and they call the emitter in-process

**`check-enforcement-fresh` and `check-footprint-fresh` become registry gates whose emit arm is a
function call rather than a spawn** — design-bearing, because the in-process call is the thing
that makes the port worth its cost **[design-bearing]**.

Both comparators today string-compare `$(bash <emitter> --emit)` against the committed
projection, and `native/src/fresh.rs` — *"the freshness family's shared shape"* — exists to give a
**ported comparator** that same spawn against a **still-shell** emitter. With §2 and §3 landed
there is nothing left to spawn: the ported comparator calls its emitter module's `emit()`
directly, in-process.

This is what retires `fresh::emit()`'s `bash` hop for these two members. `fresh.rs` is **not**
deleted — `check-trajectory-fresh`, `check-value-rollup-fresh` and `check-docs-mirror-fresh` were
ported ahead of their emitters and still spawn — but it stops being the family's only shape, and
the section that documents it says so, because a following cohort porting the remaining three
emitters retires each remaining caller in turn.

**The existing fixture pairs transfer unchanged, and that is a fact worth checking rather than
assuming.** Both comparators carry a two-positional hermetic mode (`projection-file emit-file`)
that bypasses the live emitter entirely, and both `good/`+`bad/` pairs under
`gate-sdk/gate-tests/check-enforcement-fresh/` and
`context-kit/gate-tests/check-footprint-fresh/` drive exactly that mode with pre-baked
`emit.txt`/`projection.txt`. The ported gates keep the two-positional mode, so the pairs move as
they are. **What those pairs do not prove is the emitter arm** — they were built to steer *off*
it — which is §7's subject and must not be mistaken for coverage this delta already has.

**Accepted cost, named because the ruling that creates it says to name it.** A `.gate`-declared
member is *omitted* on any platform `native/targets.list` carries no artifact for, and that
roster is one target today. Porting these two therefore attaches that omission to two gates that
run everywhere now. This is the standing, operator-re-affirmed cost of every port under
TRAJECTORY.md §The closed rulings, not a new one this unit introduces, and it is filed against
`born-native-omission-accumulation` rather than re-costed here.

### 5. The value-rollup join collapses in-process, and the markdown round-trip dies with it

**The join stops being two subprocesses and a pair of `awk` re-parsers, and becomes two function
calls over typed values** — design-bearing, because deleting a serialization boundary changes
what the join can be wrong about **[design-bearing]**.

`scripts/gen-value-rollup.sh` today runs `enf="$(bash gate-sdk/bin/enforcement-map.sh --emit)"`
and `foot="$(bash context-kit/bin/footprint.sh --emit)"`, then recovers structure from those two
markdown documents with two `awk` programs — deriving the class taxonomy from `##` headings and
the per-kit figures from table rows. It reads the emitters **live** rather than reading the
committed pages, deliberately, so a stale page cannot poison the rollup.

The live-read property is preserved and strengthened: the ported join calls each emitter module's
**structured** producer, not its rendered markdown. Each emitter module therefore exposes two
levels — the projection's data (the per-kit rows, the class taxonomy) and the rendering of it —
and `emit()` is the second applied to the first. The rollup consumes the first.

**This is the delta that pays for the cohort's shape.** Scoping the unit to a triple rather than
to three unrelated emitters is what makes the join expressible at all: parsing a sibling's
rendered output back into structure is a round-trip that exists only because the two ends were
separate processes, and it is the part of these 526 lines most able to be silently wrong — a
heading reworded in the enforcement page changes what the rollup computes, with nothing to say
so. After this delta the taxonomy is a value the rollup receives.

The rollup's own output stays byte-identical, because `check-value-rollup-fresh` byte-gates it
against the committed block and that gate is already native and is not in this cohort's scope.

### 6. A marker-block writer lands in the crate, because there is only a reader today

**`gate-sdk/lib/inject.sh`'s marker-block injection gains a Rust counterpart** — design-bearing,
because it is a new shared library surface with more than one caller **[design-bearing]**.

`gen-value-rollup.sh` has two modes: `--emit` prints the block body, and the bare invocation
rewrites the `<!-- value-rollup:begin/end -->` block **in place** in `docs/value.md` through
`inject_marker_block`. The crate has no counterpart — `marker_block()` exists in
`native/src/gates/value_rollup_fresh.rs` but is a private **reader**, and nothing in Rust writes a
marker block.

So the port adds one: a marker-block module carrying the read half (extracted from its current
private home, so there is one implementation rather than two) and a write half that matches
`inject.sh`'s contract on a marker hit — markers matched exactly, content between them replaced,
the file otherwise untouched — and **tightens** it on a miss. `inject.sh` itself appends a fresh
block when the begin marker is absent (`gate-sdk/lib/inject.sh`'s own `else` branch, `action="appended"`);
the port does not carry that branch forward, refusing instead on an absent, unbalanced, or
out-of-order marker pair, never a silent append. The refusal direction is stated because the
failure it prevents is the expensive one: a generator that appends when it cannot find its markers
corrupts a hand-authored page and the freshness gate then reports the corruption as staleness.
**This is a deliberate divergence from the shell original, not a preservation of it** — `inject.sh`
keeps its append-on-absent behavior for its other, unported callers (the installer's attribute and
registration blocks among them), so until each remaining caller ports in turn, the same miss reads
two different ways depending on which implementation reaches it. Recorded here so a later reader
does not take "preserves the contract" to mean the two stay identical.

`inject.sh` itself is **not** deleted by this unit — other shell callers remain (the lifecycle
installer's attribute and registration blocks among them) — and this delta explicitly does not
claim otherwise. The duplication is real, is bounded, and is the ordinary transitional state the
port runs in; retiring `inject.sh` belongs to whichever unit ports its last caller.

### 7. A transition-scoped parity arm proves the port, and is deleted with the script it proves

**A ported emitter owes no fixture pair, so what establishes that its output is unchanged has to
be named rather than assumed** — design-bearing, because it rules on a verification obligation the
gate contract does not reach **[design-bearing]**.

The obligation is real and easy to miss: §1 correctly frees a non-gate arm from the
`good/`+`bad/` requirement, and §4 correctly notes that the comparators' existing pairs steer
*off* the emitter. Left there, the port would land three emitters with nothing whatsoever holding
their output to what the shell emitters produced, and the freshness gates would go green because
the committed pages would have been regenerated **by the new code** — the check that cannot fail.

So the port carries a **parity arm for the duration of the port and no longer**: for each of the
three, the compiled emitter's output is diffed against the shell emitter's over the live tree,
byte for byte, in the same commit that ports it and **before** the shell script is deleted. It is
transition mechanism by construction, not a gate: TRAJECTORY.md §PRIORITY DIRECTIVE records that
a cross-substrate parity gate *earns its keep during a transition while as a steady state it only
protects a duplicate that should not exist*, and this arm's subject is deleted at the end of the
transition, so keeping it would be keeping a check on a thing that no longer exists.

**What holds the line afterwards** is stated so the deletion is not read as a coverage drop: the
projections stay byte-gated by their freshness comparators against committed pages, and the
regeneration that would mask a defect is exactly what the parity arm ran against beforehand. The
honest limit: after the transition, a change to a ported emitter is held by its comparator and by
review, not by an independent oracle — the same posture every other generated projection in this
tree already runs under, and not a new exposure this unit creates.

### 8. `check-roadmap-fresh`'s hold is retired as answered, without porting `roadmap.sh`

**The hold's subject was an unruled design, and §1 and §2 rule it** — design-bearing, because
retiring a hold is a disposition and the reasons have to be right **[design-bearing]**.

`check-roadmap-fresh` is held on the port's per-member sequencing key with the recorded reason
that the emitter design is unruled and its three candidates unadjudicated. That reason is spent:
the candidates are adjudicated above, the representation is ruled, and `queue-kit/bin/roadmap.sh`
ports by applying §2 unchanged. What the hold becomes is an ordinary unported member of the
following cohort, held by nothing but sequence.

**This is a retirement, not a reversal** — the ruling's subject is discharged, which is the
distinction TRAJECTORY.md draws and the licence for a stage session to perform it. It is also
what earns this unit the blocker-retiring override the queue entry claims, and the queue entry's
own record of that claim is the thing this delta makes true rather than a second copy of it.

### 9. Every regen command and every citation of the three scripts moves

**Nine documented commands and the prose around them name `bash <script> --emit`** —
mechanical **[mechanical]**. No gate reads a regen command, so the roster is enumerated here
rather than left to a grep under build pressure. **Each moves to the `run-gates.sh` front-end
form, not to a bare binary invocation** — §2 is why — so a regen command stays runnable by hand in
a fresh shell. The moving citations: `docs/site-architecture.md`
§Generated projections (the three regen commands, the sentence stating that the rollup reads the
two emitters live, and the ordering hazard that footprint regenerates *after* `git add`);
each freshness gate's own failure text, which prints its regen command on red; `docs/value.md`,
`docs/enforcement.md` and `docs/footprint.md` where they name their generator; `README.md` §This
repo, governed where it names the battery's emitters; and the `docs/` mirror of every kit SPEC
touched, which is generated and must be regenerated rather than hand-edited.

## Producers and consumers

**This amendment introduces one contract class, three non-gate arms, two registry gates, one
shared library module, one transition-scoped parity arm and one extension to the config-bridge
protocol. It introduces no new tag, no new evidence surface, and no new committed file.**

**The prefix-form knob declaration (§3a).** *Producer:* the config bridge — `gate_command` reading
a member's `--knobs` output and `_gate_knob_value` resolving each name, both in
`gate-sdk/lib/gate.sh`. Its enabling configuration is nothing: the bridge already runs for every
`.gate`-dispatched member, and a member declaring no prefix is unaffected. *Consumer:* the ported
`check-enforcement-fresh`, at every battery run, reading the resolved `EVIDENCE_KIT_RUN_<suite>`
family to attribute each validate suite to its enforcing kit. **Its fields have named readers:**
the resolved values are read by the emitter's suite rows at render, and the *set* of matched names
is read by nothing — deliberately, per §3a's third property, since the roster comes from
`EVIDENCE_KIT_SUITES`. **This introduces no `EVIDENCE_KIT_*` knob**, which is the whole reason the
widening lands here: evidence-kit's consumer-facing roster is untouched, and the added contract is
gate-sdk's own transport.

**The non-gate arm class (§1).** *Producer:* this amendment, landing in gate-sdk/SPEC.md.
*Consumer:* the session porting a non-gate thing to the binary — a human or agent reader, never a
gate — at the moment it asks what a ported emitter owes. Stated because no gate can decide *is
this thing a gate*, and an unstated class is re-derived per member; six members are exactly the
case where that cost compounds.

**Three `--emit-<projection>` arms (§2, §3).** *Producer:* `native/src/main.rs`'s top-level match,
before the registry lookup. Its enabling configuration is nothing — the arms are compiled into the
binary the payload already ships and the battery already dispatches to, so they are reachable on
the ordinary path in every consumer, not only in tests. **That is a reachability claim and not a
claim that an arm consumes no inputs**: an arm needing configuration receives it from the
`run-gates.sh` front-end §2 puts in front of it, which is a *caller*, never an enabling condition a
consumer has to satisfy. *Consumers, both named at named
transitions:* (i) the **regen command** in `docs/site-architecture.md`, read by the session
refreshing a stale projection, at the moment its freshness gate goes red; and (ii) **nothing
else** — the comparators and the rollup deliberately do **not** consume the arms, they consume the
library functions behind them (§4, §5). That split is why the arms are thin wrappers, and it is
what keeps the arm from acquiring a second, spawn-shaped caller.

**`check-enforcement-fresh` and `check-footprint-fresh` as registry members (§4).** *Producer:*
`gates::REGISTRY`, dispatched by name from `main`, with a `.gate` descriptor at
`gate-sdk/checks/` and `context-kit/checks/` respectively. Their enabling configuration is their
existing `gates.list` registration, which does not change. *Consumers:* the battery at every
`run-gates.sh` invocation; the generated pre-commit hook, which must be **regenerated** because a
gate's `# graph:` manifest is what the hook is projected from; and
`check-gate-substrate-parity` assertion B, which reads both new descriptors and both new `--list`
rows at every battery run and is the reason §1's placement rule is load-bearing.

**The marker-block module (§6).** *Producer:* the crate, compiled in. *Consumers:* the
value-rollup emitter's write-in-place mode, at every bare invocation; and the ported
`check-value-rollup-fresh`'s existing block reader, which moves onto the shared read half in the
same commit — the reason the reader is extracted rather than duplicated. Its **fields have named
readers**: the extracted block body is read by the comparator at its byte compare, and the marker
pair is read by the writer at the injection; there is no third field and no state written.

**The parity arm (§7).** *Producer:* the build session, in the porting commit. *Consumer:* that
same session, at the transition where it decides whether the shell script may be deleted. It is
deliberately **not** a gate, has no descriptor, no registration and no reader after the
transition — which is why §7 states its deletion as part of the design rather than leaving a
dead check for a later session to discover and preserve.

**Existing integration prose describing the prior flow is updated, not left to drift** — see
below. The two flows that genuinely change are the freshness comparison (it stops spawning a
shell emitter and starts calling a function) and the value-rollup join (it stops re-parsing a
sibling's markdown and starts receiving its structure).

**A narrowing is present and its red conditions are named, not merely its subjects.** Deleting
three shell scripts narrows the corpus several readers walk, and the causal-completeness rule
binds here because a narrowing is not clearable by inspection unless each reader's verdict is
monotone in the violation set. The readers whose **red condition is a count, a floor, or a
find-none** — and which therefore have to be re-run rather than reasoned about:
`check-gate-substrate-parity` (equality in both directions: a descriptor with no `--list` row
**and** a row with no descriptor); `port-blockers.sh`'s roster and the counts any surface quotes
from it, which *shrink* — a claim of the form "N gates remain unported" is a live count and a
narrowing changes it; `check-graph` and the generated pre-commit hook, whose projection is derived
from the `# graph:` manifests being added; `check-kit-ref-liveness` and `check-md-refs`, whose
red condition is an unresolvable path — these go red on any surviving citation of a deleted
script, which is what makes §9's roster a correctness obligation rather than tidiness; and
`check-comment-tier` over the new modules' comments. The monotone readers — the freshness
comparators themselves, whose verdict is a byte difference — are the only ones clearable by
inspection.

## Existing sections updated

- **gate-sdk/SPEC.md §Porting a gate to the binary substrate** — §1 and §2. The section gains the
  non-gate arm class with its three properties and its placement reason, and the statement that a
  ported emitter is a member of it. This is where the ruling lives permanently; the queue entry
  carries no copy of it.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — §1's other half. Assertion B's roster
  equality already holds; what is added is the forward-facing sentence saying *why* a non-gate arm
  sits outside `--list`, so the next arm is placed correctly rather than discovered to be
  misplaced by a red.
- **gate-sdk/SPEC.md, the generated-projection freshness family table** — §3, §4 and §8. The
  table's standing claim that not one of the six emitters is ported becomes three of six, and the
  recorded "the honest number is therefore zero" note for the three comparators ported ahead of
  their emitters is corrected where it stands rather than annotated: two of this cohort's members
  now have their emitters, so the dual-maintenance win for them is real. `check-roadmap-fresh`'s
  hold entry loses its unruled-design reason and keeps only its sequence.
- **gate-sdk/SPEC.md §The port-candidate criteria** — §3's `jq` dividend. `enforcement-map.sh`'s
  criterion-7 external-program dependency is retired by the port rather than waived, and the
  section records it as an instance of the directive's *the technical problems those criteria name
  are engineering work the port owes, not exclusions it may take*.
- **gate-sdk/SPEC.md §lib/gate.sh** — §3a, the bridge-protocol extension, and the reason this
  amendment's component footprint reaches gate-sdk's transport at all. The section documents
  `gate_command`'s argv resolution and `_gate_knob_value`'s per-knob resolution; both gain the
  prefix form, with its ordering anchor, its fail-closed-on-no-match rule, and the statement that a
  prefix is a resolution set and never a roster. The `--knobs` arm's own description gains the
  prefix as a legal element of what a member may publish.
- **gate-sdk/SPEC.md §lib/inject.sh** — §6. The section gains the Rust counterpart, the shared
  read half, the explicit statement that the shell library survives for its remaining shell
  callers, and the statement that the write half's absent-marker handling diverges from (tightens)
  the shell original's rather than preserving it.
- **gate-sdk/SPEC.md §enforcement-map and §check-enforcement-fresh** — §3 and §4. Both sections
  currently describe a shell script: the emitter section documents `bin/enforcement-map.sh --emit`
  and the comparator section documents `checks/check-enforcement-fresh.sh`'s bare-mode spawn of it.
  Each gains the ported shape in place — the non-gate `--emit-enforcement-map` arm for the former,
  the in-process function call for the latter — so neither continues to describe a spawn once this
  unit lands.
- **context-kit/SPEC.md §bin/footprint.sh, §check-footprint-fresh and §Layout and configuration**
  — §3 and §4. The emitter's implementation substrate changes and its invocation moves; its inputs,
  its output and its `--emit` contract do not. The comparator's section stops describing a spawn
  (`checks/check-footprint-fresh.sh`'s bare-mode `bash <emitter> --emit`) and states the in-process
  call instead. The file-tree listing's two `.sh` entries — `bin/footprint.sh` and
  `checks/check-footprint-fresh.sh` — retire the way `check-settings-pins.gate` and
  `check-settings-paths.gate` already show two lines above them in the same tree: the emitter's
  line drops (no `bin/` file survives it) and the comparator's becomes a `.gate` entry.
- **queue-kit/SPEC.md §check-roadmap-fresh** — §8. The hold's recorded reason is deleted, not
  annotated as answered, and what remains is an ordinary sequence position.
- **docs/site-architecture.md §Generated projections** — §9. Three regen commands, the live-read
  sentence, and the footprint-after-`git add` ordering hazard, which survives the port unchanged
  and must not be dropped with the script that carried it.
- **`native/src/main.rs`, `native/src/gates/mod.rs`** — §2, §3, §4: three top-level arms, three
  emitter modules, two REGISTRY entries, two module declarations.
- **`gate-sdk/checks/check-enforcement-fresh.gate`, `context-kit/checks/check-footprint-fresh.gate`**
  — §4: two new descriptors, and therefore a **regenerated** pre-commit hook.
- **`scripts/git-hooks/pre-commit`, `.workflow/` graph artifact, the enforcement map, the
  footprint and value rollups, the `docs/` mirror** — §9's generated fan-out. Each is regenerated
  by its own command, never hand-edited; `docs/site-architecture.md` §Generated projections is the
  roster and each freshness gate prints its command on red.
- **TASK-QUEUE.md** — `freshness-emitter-port-cohort` takes the **demotion** branch of the
  terminal move, not the Done move: its deliverable is a six-member corpus and this amendment
  delivers three, so it returns to the deferred section under `[design-pending]` with its
  `[spec:]` dropped, and the following cohort re-promotes it with a fresh amendment. A Done move
  would assert a finished port and drop a live item from the public roadmap projection.

## Definition of Done

- [ ] **Causal completeness** — every new arm, gate and module has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **The parity arm ran before the deletion, not after** — for each of the three emitters, the
      compiled output was diffed against the shell emitter's over the live tree and matched byte
      for byte **while the shell script still existed**. A regenerated page proves nothing, and
      this is the check the port is easiest to land without.
- [ ] **The narrowing's non-monotone readers were re-run, not reasoned about** — every reader
      named in §Producers and consumers whose red condition is a count, a floor, or a find-none,
      re-run green after the three deletions; no surviving citation of a deleted script anywhere in
      the tree.
- [ ] **The shell scripts are gone** — the win is the deletion; a port that leaves them standing
      has bought the design already refused, and the emitters' `--emit` flags did not survive as
      compatibility spellings.
- [ ] **The join is structural** — the value rollup consumes its siblings' data, not their
      rendered markdown, and no `awk` re-parse of a sibling's output survives.
- [ ] **Byte-identical projections, but for each page's own generator citation** — every measured
      figure, row and heading in `docs/enforcement.md`, `docs/footprint.md` and `docs/value.md`'s
      block is unchanged by this unit; the sole licensed diff is the self-citation §9 moves, and a
      diff anywhere else is a port defect, never a licensed improvement. §2 carries the carve-out
      and the two-step sequencing that keeps the parity arm byte-exact despite it.
- [ ] **The marker writer refuses rather than appends** — proved by running it against a file with
      absent, unbalanced and reversed markers; a silent append is the failure mode §6 exists to
      prevent.
- [ ] **The hook and every generated projection are regenerated** — two new `.gate` descriptors
      stale the pre-commit hook and the graph artifact; the full fan-out is
      docs/site-architecture.md §Generated projections'.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); each merged spec reads as one coherent document a reader who never
      saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`). Discharged at the **iteration**, not at this commit, where sibling
      amendments are in flight.
- [ ] **Removals propagated** — grepped every surface for the three retired script paths and for
      the retired "not one of them is ported" claim; nothing dangles.
- [ ] **The terminal move is the demotion, not Done** — the queue entry returns to deferred under
      `[design-pending]`, keeping its `[roadmap:]` tag and its place in the public projection.
- [ ] **The prefix form is held by a fixture pair** — §3a changes gate-sdk mechanism every ported
      gate rides, so it lands under the four gate-sdk contracts like any other: a `good/`+`bad/`
      pair driving a prefix that matches and a prefix that matches nothing, the second proving the
      refusal rather than a resolved-empty pass.
- [ ] **Gaps filed** — anything the evidence-config bridge read could not take, and any following-
      cohort obligation this unit discovered, filed rather than absorbed.
