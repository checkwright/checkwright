# SPEC amendment: registry-rosters

The binary carries several rosters — the crate's gate registry, the battery's
`scripts/gates.list`, the declared walk roots each member publishes through
`--reads`, and the non-gate `--emit` arm set — and **nothing joins any pair of
them**. This amendment makes four of those joins, and repairs the one place a
roster's value is computed for a caller that never receives it.

Queue entries: `bridged-knob-case-tmp-dir-override-inert` (deltas 1 and 2),
`reads-couples-skipped-set-has-no-census-oracle` (deltas 3 and 4),
`crate-registry-battery-roster-divergence-unasserted` (deltas 5, 6 and 7),
`unknown-emit-arm-diagnostic-misdirects` (deltas 8, 9 and 10). All four are
**operator directions of 2026-09-12**, revisable here; where authoring found a
premise that does not hold, the delta says so at the passage rather than
building around it.

**The seam is already ruled and is not re-opened** (scope, `e597f672`): pinning a
fixture case's scratch in every spelling a member may read is **kit mechanism** —
a case's sandbox location is the harness's own fact. Nothing becomes consumer
config; `GATE_SDK_TMP_DIR` already exists and its value stays the consumer's for
ordinary runs. Two shapes are foreclosed rather than ranked: computing the pin at
the **member** (a second producer on a single-producer bridge), and a **per-gate
opt-out knob** (a second source for one pin). Delta 2 exists because the SPEC
currently *promises* the second of those.

## What changes

### (1) The case scratch pin becomes an input to the config bridge, not an override of its output

`--run-gate-tests` supplies the pinned scratch to the shell that resolves a
case's argv, so the bridge computes it once and every spelling a member may read
carries it {design-bearing}.

**What is broken, traced rather than inferred.** `setup` computes the pin
correctly — `native/src/emit/run_gate_tests.rs:94` absolutizes
`GATE_SDK_TMP_DIR` against the invoker's root, which is the whole point of the
mechanism §run-gate-tests describes. `run_case` then pushes it into the per-case
child environment under the **bare** name
(`run_gate_tests.rs:333-334`). That reaches a `.sh` member, which sources
`lib/gate.sh` in its own process and reads the bare name — so the existing pin is
live, not dead code. It does **not** reach a `.gate` member: every crate-side
reader goes through `walk::knob_scalar`, which reads
`GATE_SDK_KNOB_<NAME>` and nothing else (`native/src/walk.rs:256-257`). The pin
covers one of the two spellings, which is exactly the entry's claim.

**Why pinning the prefixed name at the same point would not have worked, stated
because it is the obvious repair and it is wrong.** A `.gate` member's argv is
`env GATE_SDK_KNOB_GATE_SDK_TMP_DIR=<value> <binary> <name>`, so `env(1)` is
argv[0] and its own assignment is applied *after* the environment
`Command::env` built. The harness would be setting a variable the very next
process overwrites. The value has to be corrected **upstream of the bridge**, not
downstream of it.

**Where the repro-relative value actually enters.** `resolve_argv`
(`run_gate_tests.rs:418-437`) spawns `bash` with **no environment vector**, so
the pin never reaches the process that bakes the argv. `lib/gate.sh:48` —
`[[ -v GATE_SDK_TMP_DIR ]] || GATE_SDK_TMP_DIR=".tmp"` — therefore resolves the
deliberately repo-relative default, and `_gate_knob_emit` bakes `.tmp` into the
`env` prefix. Under the case's working directory that resolves inside the tracked
fixture corpus.

After this delta `resolve_argv` exports the pinned value into the resolving shell
**in the spelling that script already uses for the binary** — a positional the
script `export`s, beside `GATE_SDK_NATIVE_BIN` — rather than relying on ambient
inheritance. Three properties follow, and they are why this spelling was chosen
over rewriting the emitted argv element:

- **The bridge stays the single producer.** The harness supplies an *input*; the
  value crossing the seam is still the one `_gate_knob_emit` computed. Rewriting
  the emitted `GATE_SDK_KNOB_GATE_SDK_TMP_DIR=` element would have made the
  harness a second producer on a single-producer surface, which is the
  criterion-6 shape the seam ruling forecloses.
- **Both spellings are served by one pin.** The bare name is what the resolving
  shell reads and what a `.sh` member reads; the prefixed name is what the bridge
  derives from it. The pin at `run_case` stays exactly as it is — after this delta
  the two pins are the same value reaching the two substrates, rather than one
  live pin and one inert one.
- **Every future member is covered without a further edit.** The fix is at the
  knob's resolution, not at a member, so a gate ported tomorrow that declares
  `GATE_SDK_TMP_DIR` inherits it. Today exactly one registry member declares the
  knob — `check-crate-arms` (`native/src/gates/mod.rs:1580-1591`) — and an
  entry-shaped fix naming that member would have to be re-made at the second one.

**Per-case sandbox isolation is a deliberate non-target.** The pin resolves to the
invoker's root, so every case shares one scratch directory; giving each case its
own would additionally stop one case's cache greening another. That is a different
property from the one this unit closes (a gate depositing state inside the corpus
it is the oracle for), §run-gate-tests already commits to invoker-root
absolutization, and widening here would settle a question no entry asks. Recorded
so a later reader does not read the absence as an oversight.

### (2) §run-gate-tests' `source`-before-`cd` order is corrected, and the case-config promise is withdrawn

The section states the opposite of its own quoted script and of the code, and the
claim that rests on the mis-statement promises the shape the seam ruling
forecloses {design-bearing}.

`gate-sdk/SPEC.md:10577` reads *"the `--run-gate-tests` arm does read the case's
seam, because it enters the case dir before the source."* The section's **own**
quoted script ninety lines earlier (`gate-sdk/SPEC.md:10486`) and the live code
(`run_gate_tests.rs:421`) both run `source "$2/lib/gate.sh"` **before**
`cd "$1"`. Verified by literal reproduction of the script against two directories
each carrying a `scripts/gate-sdk-config.sh`: the config seam picked up the
**pre-`cd`** directory's config every time, never the case's.

Two consequences, and the second is why this is a delta rather than a typo fix:

- The section's claim at `gate-sdk/SPEC.md:10677-10678` that a case config naming
  its own scratch **still wins** is false today, and this amendment does not make
  it true.
- It should not be made true. A case config naming its own scratch is precisely
  the **per-gate opt-out** the seam ruling forecloses as a second source for one
  pin. So the sentence is **withdrawn** rather than repaired: the harness's pin is
  authoritative for the case invocation, and a case config's other knobs — which
  the live fixtures do use, `GATE_SDK_NATIVE_CRATE` and
  `GATE_SDK_CARGO_TARGET_DIR` among them — are unaffected, because they reach the
  member through the argv the pre-`cd` source already resolved.

**Not yet applied.** Replacement text for both passages is build's to land.

### (3) `--emit reads-census`: the `?` population, measured once by an oracle

A new non-gate arm prints every registry member declaring an undecidable walk
root, with its `?` count and the roots it does declare {design-bearing}.

`check-reads-couples` skips a root declared `?` (`reads_couples.rs:483-486`), and
the population of those skips is hand-swept. The entry records three sweeps in one
stage landing on 26, then 27, then at least 29, each miss caused by another
spelling of one idiom. **Re-measured here by oracle: 115 registry members, of
which 36 declare at least one `?`, across 51 `?` root-lines in total.** The arm
prints that, and the count is derivable from its lines rather than transcribed —
the same rule §check-reads-couples' own `--reads` report states, for the same
reason.

**What the arm reads is registry data and nothing else.** Field 2 of `GateEntry`
(`native/src/gates/mod.rs:134-141`) is the declared-roots slice, and it is already
answered per member by `--reads`; the arm is the roster form of a question the
binary answers one member at a time. No new config, no new registry field, and no
source parsing.

**The entry's own cheapness claim does not hold for the rest of its deliverable,
and that is recorded here rather than absorbed.** The entry asks for the `?` set
"together with its walk's root expression and filter argument", calling the arm "a
projection over data that exists". For the population that is true. For the other
half it is not: every `?` row in the registry is literally `("?", "", "")` — the
filter and prune fields are **empty**, because a member that could name them would
not be declaring `?`. The root expression and the filter argument exist only in
Rust source. **Delta 4 is what makes them reachable** — by a declared locator
rather than by an extracted copy — and this delta is the half that is a projection
over what the registry already holds. The two ship together: this one alone leaves
the Deliverable's other half unserved.

**The entry's "39" is a third quantity again, and it is unauditable today.** 39 is
the *decidable* subset — the `?` roots that in fact resolve to a static literal,
as against the kit-literal-fallback branches §check-reads-couples rules
legitimately undecidable (`gate-sdk/SPEC.md:14992-15007`). Separating those needs
exactly the classifier delta 4 would retire. No command in the tree reproduces 39,
and this amendment does not assert it.

### (4) Every `?` declares its ground, and the ground names where the walk is

A `?` root gains a required ground and a resolvable source locator, so the
decidable/undecidable split is **declared** rather than re-derived by reading Rust
source {design-bearing}.

**Decision of the intent oracle (the scope session), 2026-09-12, lead-routed:
ship delta 3 and delta 4.** Recorded with its class, because a decision may be
revised by a later session where a direction or a consult ruling may not, and
writing it up as either would wrongly harden it.

The shape: field 2's tuple gains a fourth element, the **ground** a `?` is
declared on, and a member declaring `?` with an empty ground **fails to
compile** — the same construction that makes the roots themselves un-omittable.
`--reads` gains the ground on a `?` line; `--emit reads-census` groups by it. Two
ground values are live today and §check-reads-couples already names both classes:
`fallback`, for the kit-literal fallback branch of a runtime-selected corpus
helper, which that section rules legitimately undecidable and files under its own
open semantics question; and `dynamic`, for a root genuinely computed at runtime.
A third value is deliberately **not** offered — a `?` that is in fact a hardcoded
literal or a positional with a literal default has no ground, because it should
be declaring the root. Under this delta such a member does not compile, which is
what converts the entry's recurring hand sweep into a build-time impossibility.

**The ground carries a source locator, and that is the delta's own defence against
being satisfied mechanically.** Each `?` declares `<class>@<module>:<line>` — the
class above, and the location of the walk the `?` stands for. The requirement
exists because of a fact about the corpus this amendment measured: **all 51 `?`
root-lines are the identical spelling `("?", "", "")` with zero counterexamples**,
which is one default replicated fifty-one times, not fifty-one considered
declarations. Build authors all of them in one pass, and a ground written in bulk
to satisfy a compiler reads as considered when it was mechanical — the failure
class this tree already carries an open entry about on the audit roster's
pre-stamp.

A locator is what closes that, where a prose justification would not:

- **It cannot be written without performing the classification.** You cannot name
  the line of the walk without finding the walk, and finding the walk *is* the act
  of deciding which class it is. The evidence requirement and the classification
  become one act rather than two, the second of which is skippable.
- **It is machine-checkable, where honesty is not.** A crate-side assertion holds
  each locator to resolving, and to resolving **inside the declaring member's own
  module**. That is presence and placement, which a gate can decide — deliberately
  *not* whether the class is correct, which it cannot, and which is the shape
  §check-queue-entry-budget's own refusals name as the standing failure.
- **It is a pointer, not prose.** One token per site, no restatement, no
  fifty-one-paragraph tier to maintain. Content-tiering applied to a registry
  field.
- **It answers the Deliverable's other half better than an extractor would.** The
  entry asks for each `?` "together with its walk's root expression"; a locator
  *points at* the expression instead of transcribing it, so no copy exists to
  drift. Derivation-first, and it retires the static-analyser route a previous
  iteration already measured wrong three times running.

**Why this is the nearest satisfiable form of the entry rather than a widening.**
The Deliverable as written cannot be satisfied at all — there is no root expression
or filter argument in the registry to enumerate, every `?` row being empty in both
fields. So delta 3 alone is not the cheap version of this unit along the same axis;
it is *further* from the Deliverable, and it leaves the entry live to be re-ranked
at the next boundary while the iteration pays the fixed cost anyway. The envelope is
"measured once by an oracle rather than re-swept per session", and a compiler is an
oracle.

**It also leaves the entry's own evidence verifiable.** The entry argues census
unreliability from 26/27/29 against 39. This authoring measured 51 and 36, and
nothing in the tree reproduces 39. Under delta 3 alone, an entry about census
unreliability would keep an uncensused headline number; the ground partition is
what finally produces a figure of the class 39 was meant to be.

### (5) `check-gate-substrate-parity` assertion I joins the crate registry to the battery registration

A ninth assertion compares the crate's dispatch roster with `scripts/gates.list`
in both directions, each residue member taking a recorded disposition
{design-bearing}.

Assertion B already equates the `.gate` descriptor set with the subcommand roster
(`gate-sdk/SPEC.md:12877-12879`), and the gate derives both in-process —
descriptors by globbing the resolve dirs, the roster from
`gates::names_with_owners()` (`gate_substrate_parity.rs:551-577`). **Neither path
reads `scripts/gates.list` at all**, which is the entry's premise and it holds at
the code. Re-measured at this authoring: 115 crate members against 113
registration lines, residue exactly `check-surface-duplication` and
`check-producer-liveness`, and the **reverse direction empty**.

Assertion I reds when the two rosters' symmetric difference is not exactly the
declared set of delta 6. Both directions are asserted, and the empty one is
asserted deliberately: a name in `gates.list` that the crate does not carry is a
battery that cannot run its own registration, which today no member exhibits and
which nothing would otherwise catch.

**It costs no new configuration.** `registry::list_path(gates_dir)`
(`native/src/registry.rs:55-57`) already derives the registration file's path from
the gates dir the gate already resolves, and the descriptor already carries
`scripts/gates.list` in its `couples=`, so the trigger is already correct and no
manifest edit is owed.

**Why the disposition is required rather than the divergence merely reported.** The
entry's measured cost is that "two members stay dispatchable and unrun with their
causes stated only in prose no reader joins". A bare report would leave the causes
where they are. Requiring a disposition is what moves the cause onto the surface
the assertion reads.

### (6) `gates.list` admits an `# unregistered: <name> — <reason>` declaration

The registration file gains one declaration form, so a deliberately unregistered
member records its cause in the consumer's own file {design-bearing}.

**The grammar is reused, not minted.** `# smoke-unregistered: <gate-name> —
<reason>` already exists for the same shape of fact — a member deliberately absent
from a roster, with a stated cause, both fields machine-read
(`gate-sdk/SPEC.md:8480`). This is that grammar on a second roster, and the
parallel is stated in the section so the two cannot drift into two dialects.

**The provenance seam decides where the cause lives, and it is the reason the
declaration is not a kit table.** Why *this* consumer does not register
`check-surface-duplication` (it declares no glossary, for the provenance-seam
reason) and does not register `check-producer-liveness` (its own validate battery
would red against its own lock) are **consumer facts about a consumer's tree**. A
kit SPEC holding them would publish one consumer's configuration as kit content,
and would rot the moment a second adopter registered either member. So the kit
owns the **grammar** and the assertion; the consumer's own `gates.list` owns the
**cause**. This is the `check-graph` / `graph-vocab.sh` pattern on a smaller
surface.

**No member set moves, checked rather than assumed.** Both readers of this file
filter comment lines already — `registry::members` (`native/src/registry.rs:12-18`)
and its shell twin `gates_list_members` (`gate-sdk/lib/gate.sh:216-218`) — so a
declaration line is invisible to every existing reader, and the 113-member count
is unchanged by adding them. The declaration is therefore additive to an existing
file format rather than a change to it.

### (7) The two live residue members take their declarations

`scripts/gates.list` gains one `# unregistered:` line per residue member, each
carrying the cause its prose site already states {mechanical}.

`check-surface-duplication`'s cause is `canon-kit/README.md:34` — "needs a glossary
(exits 2 without one)" — which is a **kit** statement and generic to any adopter,
so it stays where it is and the declaration states this tree's instance of it.
`check-producer-liveness`' cause is `scripts/lifecycle-config.sh:12`, whose live
clause is that "a registered liveness gate would red every validate run against
its own lock"; the pointer there is retained and the declaration carries the
consumer-side fact. Content tiering: the declaration is the machine-read surface
and owns the cause in one line; neither prose site gains a restatement, and
`scripts/lifecycle-config.sh:12` keeps its existing `# spec:` pointer unchanged.

### (8) An unknown `--emit` arm gets its own refusal, naming the arm roster and nothing else

`no_such_arm` replaces the gate-roster refusal on the emit path, printing the
`--emit` roster and citing the section that owns it {design-bearing}.

**Reproduced, not reasoned.** `normalize()` (`native/src/main.rs:283-289`) rewrites
`--emit <name>` to `--emit-<name>` with no validity check; `emit::lookup` misses;
control reaches `gates::lookup`, misses again, and lands in `no_such_gate`
(`native/src/main.rs:42-50`), which prints `gates::names_with_owners()` — the
crate's own 115-member registry, **not** `scripts/gates.list`'s 113 registration
lines; the two rosters delta 5 measures apart are conflated at this call site's
fallback, and the fix below is scoped to the roster it actually prints. It is
**byte-for-byte the same code path** a mistyped gate name takes — there is no
arm diagnostic in the binary at all. A reader who typed `--emit` is handed the one
roster that cannot contain what they meant.

After this delta the emit path carries its own miss. `normalize` already knows the
argv began with `--emit`, so the information needed to route the refusal is present
at the point of failure and no new state is introduced. `no_such_arm` prints the
live `--emit-` arm names off `BRIDGED_ARMS` the same way `names()` already does —
33 of them at this authoring, before this amendment's own delta 3
(`--emit-reads-census`) and, where it lands in the same batch,
`SPEC-compression-legibility.md` delta 2 (`--emit-entry-history`) each add one
more; the count is never transcribed, so neither addition is a further edit here
— cites `gate-sdk/SPEC.md §The non-gate arm` as the roster's owner, prints **no
gate roster**, and exits 2 unchanged.

**The exit code and the `help:` shape are unchanged**, so the output contract's
existing assertions are untouched: this delta changes which roster a `help:` line
carries, never whether there is one.

### (9) The output contract states that a refusal may cite a governing SPEC section

§Output contract gains one rule, which the tree already follows everywhere
{design-bearing}.

The entry frames delta 8 as deciding "whether a refusal may cite a SPEC section — a
coupling the gate-output contract does not currently carry". **That is right about
the contract text and wrong about the practice, and the correction is what makes
this delta cheap.** §Output contract (`gate-sdk/SPEC.md:946`) is silent on message
sourcing, while the coupling is already taken in live refusal paths across both
substrates: `native/src/gates/crate_arms.rs:21`, `guard-kit/lib/guard.sh:1578` and
`:1580`, `gate-sdk/bin/build-native.sh:23`, `:33` and `:107`, and
`gate-sdk/bin/run-consumer-smoke.sh:42`, `:111` and `:197`.

So the delta **states a rule the tree already keeps** rather than minting a
coupling: a `help:` line may name the governing SPEC section that owns the remedy,
and the citation is a **pointer, never a restatement** — the content-tiering rule
applied to refusal text, which is also what stops a message growing into a copy of
the section. The rule is stated because a silent convention is one a later reader
may read as an accident and remove.

### (10) The bridge's knob-probe refusal names the whole arm argv

`gate_knob_env`'s refusal prints the arm's full argv rather than its first token,
so the two-token `--emit <name>` form is not reported as `--emit` {mechanical}.

On the same mistyped-arm invocation a reader meets a *second* misleading line
before delta 8's: `gate_knob_env` (`gate-sdk/lib/gate.sh:400-409`) is called with
`$arm` = `--emit`, and its `printf` reports "the config bridge could not report
what **`--emit`** reads" — naming a token that is not the arm, at the one moment
the reader is trying to learn what the arm set is.

**This sits inside the entry's envelope rather than beside it**, and the reason is
stated because the opposite reading is available: the entry's subject is the
diagnostic **at the moment of miss**, and this line is the first thing that moment
prints. Fixing the second line and leaving the first would ship a corrected
diagnostic underneath an uncorrected one.

## Producers and consumers

- **The pinned bridge input** (delta 1).
  - *Producer:* `--run-gate-tests`' `resolve_argv`, exporting the value `setup`
    already computes at `run_gate_tests.rs:94`. Reachable in the deployed
    configuration: the arm runs on every kit fixture suite, which the commit-time
    battery invokes.
  - *Consumers:* `lib/gate.sh:48`'s resolution, which reads the bare name; then
    `_gate_knob_emit`, which derives `GATE_SDK_KNOB_GATE_SDK_TMP_DIR` from it; then
    `walk::knob_scalar` in the member. Named reader today:
    `check-crate-arms`' `cache_path` (`native/src/gates/crate_arms.rs:39-47`), at
    the source-stamp cache read and write.
  - *No new field:* the knob already exists and its value stays the consumer's for
    ordinary runs. What changes is which process sees the harness's pin.
- **`--emit reads-census`** (delta 3).
  - *Producer:* the crate's emit dispatch, over registry field 2 — data that
    already fails to compile when omitted.
  - *Consumer:* a session sizing the skipped population, and
    §check-reads-couples' cadence row, which currently names a population no
    command answers. Every emitted field has a named reader: the member name and
    the `?` count are read at the cadence review; the declared roots are read
    beside them to tell a wholly-undecidable member from a partly-declared one.
  - *No count line is emitted*, so no transcribed total can drift from the lines.
- **The `?` ground and its locator** (delta 4).
  - *Producer:* the registry entry itself, compile-enforced. Reached in the
    deployed configuration at the landing commit, since all 51 live `?` roots take
    one.
  - *Consumers:* `--reads`, on a `?` line; `--emit reads-census`, grouping by the
    class and printing the locator as the column the Deliverable asked for.
  - *Named reader of the refusal:* the compiler, at the member's registration —
    which is the point: a member with no ground cannot be added.
  - *Named reader of the locator:* a crate-side assertion, at the registry's
    unit-test pass, whose **red condition** is a locator that does not resolve or
    that resolves outside the declaring member's own module. It does **not** red on
    a wrong class, which no gate can decide — stated so a later reader does not
    read the silence as coverage.
- **Assertion I** (delta 5).
  - *Producer:* `check-gate-substrate-parity`, at its existing roster derivation;
    the registration file is reached through `registry::list_path`, already in the
    crate.
  - *Consumer:* the battery, on every precommit run — the gate's existing tier.
  - *Red condition, named rather than its subject:* it reds when the symmetric
    difference of the two rosters is not exactly the declared set — so it reds on
    an **undeclared** residue member, on a **stale** declaration naming a member
    that is in fact registered, and on a `gates.list` name the crate does not
    carry. It does **not** red on the size of the residue, which is a consumer's
    business.
- **The `# unregistered:` declaration** (deltas 6 and 7).
  - *Producer:* the consumer, editing its own `gates.list`; delta 7 is the first
    deployed instance, so the producer is reached in this tree rather than only in
    fixtures.
  - *Consumer:* assertion I, at the comparison above. Both fields are read: the
    name joins the rosters, the reason is what the assertion requires to be
    non-empty.
  - *Every field has a named reader*, and there is no third field because there is
    no third reader.
- **`no_such_arm`** (deltas 8 and 10).
  - *Producer:* the crate's top-level dispatch, on the emit path `normalize`
    already identifies; and `gate_knob_env`, for the argv it names.
  - *Consumer:* the human or agent that mistyped the arm — the only consumer a
    diagnostic has, which is why its correctness is the whole deliverable.
  - *Reader of the roster it prints:* the same reader, at the next invocation.
- **Red conditions of the affected readers under a narrowing** (causal-completeness
  point 5). **One delta narrows a corpus and it is checked rather than argued.**
  Delta 1 moves where `check-crate-arms` resolves its cache, which narrows nothing
  it scans but does change a path it reads: its red condition is a failing `cargo`
  arm, never a cache hit or miss, so a relocated cache costs a re-run and cannot
  flip a verdict. Delta 6 adds **comment** lines to `gates.list`; both readers
  already filter them (`registry.rs:12-18`, `gate.sh:216-218`), so the member set
  is bit-identical and no member-count reader can move — this is the one place a
  "a narrower corpus only removes violations" reflex would have been wrong in the
  other direction, and it is why the two twins were both read. Deltas 3, 5, 8, 9
  and 10 are additive: a new arm, a new assertion, a new refusal branch, a stated
  rule, and a widened `printf` argument.
  - `check-gate-fixture-coverage` reds on a registered member with no `good/`+`bad/`
    pair. Delta 5 adds an assertion to an existing member rather than a member, so
    its derived set is unchanged; deltas 3 and 8 ship their own fixture pairs.
  - `check-gate-output` reds on a missing `: clean` or `help:` line. Delta 8 keeps
    both, and delta 9 states a rule about a `help:` line's *content*, which that
    gate does not read — recorded because the opposite assumption would make
    delta 9 look like a gate change.
  - `check-graph` reds when the generated hook diverges from the manifests. No
    `couples=` field changes, because the one the join needs is already declared.
  - `check-crate-arms` reds on a failing crate lint or test arm; every delta
    touching the crate lands its unit tests in the same commit.

## Existing sections updated

- `gate-sdk/SPEC.md` §run-gate-tests, the scratch-pin passage at 10653-10661
  (delta 1). It states which process the pin enters and why that is upstream of the
  bridge rather than beside it, and records that the bare and prefixed spellings
  are one pin reaching two substrates. **Not yet applied.**
- `gate-sdk/SPEC.md` §run-gate-tests, line 10577 (delta 2). The order is corrected
  to `source` before `cd`, matching the section's own quoted script at 10486.
  **Not yet applied.**
- `gate-sdk/SPEC.md` §run-gate-tests, lines 10677-10678 (delta 2). The
  case-config-wins claim is **withdrawn**, with the seam ruling's foreclosure of a
  per-gate opt-out as its ground, so the withdrawal reads as a decision rather than
  a deletion. **Not yet applied.**
- `gate-sdk/SPEC.md` §check-reads-couples (deltas 3 and 4). The section gains the
  census arm as the answer to the population its cadence row asks for, and records
  the measured figures (36 members, 51 `?` roots) as of this authoring with the
  oracle that produces them, so a later reader re-runs rather than trusts. It also
  gains the two ground values, the refusal of a third, the locator's grammar and
  its placement rule (delta 4).
- `gate-sdk/SPEC.md` §The non-gate arm (deltas 3 and 8). `--emit-reads-census`
  joins the roster; the section is named as the roster `no_such_arm` cites, which
  is what makes it a governed surface rather than documentation.
- `gate-sdk/SPEC.md` §check-gate-substrate-parity (delta 5). Assertion I is added
  beside B with its both-directions statement, its red conditions, and the note
  that it reads the registration file through the existing path derivation so no
  knob is minted.
- `gate-sdk/SPEC.md` §Layout and configuration (deltas 6 and 7). The `gates.list`
  format gains the `# unregistered:` declaration, stated as reusing
  §Consumer smoke's grammar and pointing at it rather than restating it, and
  recording that both member readers already filter comments so no existing reader
  moves.
- `gate-sdk/SPEC.md` §Consumer smoke (delta 6). One clause: the
  `# smoke-unregistered:` grammar now has a sibling on a second roster, so a reader
  editing one knows the other exists. **Not yet applied.**
- `gate-sdk/SPEC.md` §Output contract (delta 9). The refusal-citation rule, with
  its pointer-never-restatement limit.
- `scripts/gates.list` (delta 7) — the two declarations.
- `native/src/emit/run_gate_tests.rs`'s `# spec:` pointer at lines 91-93 (delta 1).
  It currently says the pin's scope "is the pair loop, never this process"; after
  delta 1 the pin also enters the resolving shell, which is a different and wider
  claim. **Not yet applied.**
- `scripts/git-hooks/pre-commit` and `docs/check-graph.html` (deltas 5, 6, 7) — the
  generated projections. Regenerate with
  `bash gate-sdk/bin/gen-pre-commit.sh --write`, then
  `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`.
- `docs/gate-sdk/SPEC.md` (all deltas) — the generated on-site mirror; regenerate
  with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

**Deliberate non-updates, recorded so a reader does not go looking.**
`canon-kit/README.md:34` is unchanged: its statement is generic to any adopter and
is the kit's to make, so delta 7's declaration is this tree's instance of it rather
than a relocation. `scripts/lifecycle-config.sh:12` is unchanged for the same
reason on the other side — it owns why the gate is wired where it is, which is a
different fact from why this tree does not register it.

## Retired spellings

- `a case config naming its own scratch still wins` (delta 2) — the claim at
  `gate-sdk/SPEC.md:10677-10678` is withdrawn rather than repaired, because the
  shape it promises is the per-gate opt-out the seam ruling forecloses. No
  identifier is retired with it; the spelling is a prose claim and the grep is over
  prose.

Delta 8 adds a **sibling** refusal for the emit path; the existing gate-path
refusal keeps its name and its behavior, so no identifier is retired there either.
Stated below the roster rather than as a bullet in it, because a bullet in that
roster declares a retirement and this is the opposite.

## Definition of Done

- [ ] **Causal completeness** — every new state, event and interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Every `?` ground names its evidence, and the locator is checked by oracle** —
      each of the 51 sites declares `<class>@<module>:<line>`, a crate-side
      assertion holds every locator to resolving inside its own declaring member's
      module, and a seeded bad locator reds it. A ground with no locator does not
      compile, so the bulk pass cannot skip the step that makes it a classification.
- [ ] **The ground partition is reported with both classes non-empty, or the
      landing states why.** All 51 rows are the identical spelling today, so a pass
      returning 51 of one class and zero of the other is the bulk-authoring
      signature rather than a result — §check-reads-couples names both classes as
      live, and an empty one is a finding to resolve before landing, never a figure
      to report.
- [ ] **A `?` that should be a declared root does not survive the pass** — at least
      one site is checked against the two shapes §check-reads-couples calls
      decidable (a hardcoded literal root, a positional with a literal default),
      and any found is converted to a declared root rather than given a ground.
- [ ] **The scratch fix is verified by the corpus, not by the diff** — delete
      `gate-sdk/gate-tests/check-crate-arms/good/.tmp/`, run the `gate_sdk` fixture
      suite, and confirm the directory does **not** reappear while the pinned
      location does. The pre-fix state is reproducible today and is the baseline.
- [ ] **Both spellings are exercised** — a `.gate` member and a `.sh` member each
      resolve the pinned scratch, so the fix is not silently one-sided in the other
      direction.
- [ ] **The census figures are the oracle's** — §check-reads-couples' recorded
      36/51 are re-run at the landing commit and corrected if the tree moved; a
      figure that disagrees with the arm is the arm's to win.
- [ ] **Assertion I reds on a seeded divergence** — a fixture pair in which a
      registered-but-unlisted member carries no declaration, and one in which a
      declaration names a member that is in fact registered.
- [ ] **No member set moved** — `scripts/gates.list` still yields 113 members from
      both readers after delta 7.
- [ ] **The diagnostic is read, not diffed** — `--emit <typo>` prints the arm
      roster, cites §The non-gate arm, and prints no gate name; and the knob-probe
      line above it names the whole arm argv.
- [ ] **Fixture pairs for every shipped assertion and arm** (gate-sdk/SPEC.md
      §Consumer smoke's kit-landing checklist).
- [ ] **Full battery green** (`bash gate-sdk/bin/run-gates.sh`),
      `bash gate-sdk/bin/build-native.sh`, and the gate-sdk, canon-kit and
      evidence-kit fixture suites.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended; the two stale §run-gate-tests passages
      corrected and withdrawn in place rather than left beside their corrections.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`). No sibling amendment is in flight for
      gate-sdk, so the none-remain half is satisfiable at this amendment's own
      merge commit.
- [ ] **Queue transition at the merging build batch, not at close** — the drain
      stage's entry refuses a non-empty active queue, so the batch merging this
      amendment moves all four entries in the same commit.
- [ ] **Removals propagated** — `## Retired spellings` above holds, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — a cross-component gap found during the work is resolved that
      session, not deferred.
