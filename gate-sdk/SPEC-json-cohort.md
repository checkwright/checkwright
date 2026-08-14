# SPEC amendment: json-cohort

The ninth port cohort — **two members, and what it buys is a JSON reader.**

`check-settings-paths` and `check-settings-pins` (context-kit) share one corpus
derivation, `.claude/settings.json` — the pins gate adding the pins manifest
beside it — and both carry the criterion-7 `jq` requirement. The cohort is
selected under the selection rule's **documented override** (§The first cohort,
and the rule that selects the next: *"A cohort that retires a blocker several
later cohorts are queued behind outranks a larger one that retires none"*),
never by member count. Operator-ruled 2026-08-14.

**The size arm of the selection rule is exhausted, and that is the fact this
cohort turns on.** Run at the scoping session, `bash
gate-sdk/bin/port-blockers.sh --group` reported **104 members scanned, 47 groups,
0 undecidable, 42 already ported — 62 remaining shell**, and the largest
*takeable* derivation group is **one member**: the 14-member group is
operator-ruled not a cohort (`fail_closed` derives no corpus), two 2-member
groups each hold one member that stays shell, and the remaining 44 groups are
singletons. The rule carries no degenerate-case clause for that state, so it is
not stuck — the override still selects — but it no longer selects by size, and a
cohort taken under the override must say so rather than let a later reader read
"two members" as the largest group.

**This is the third exercise of the override, and the payoff is the retired
blocker rather than a built engine** — §The POSIX ERE matcher is the first
(*"what it buys is the engine"*), `check-roadmap-fresh`'s hold the earlier one.
The distinction matters and is drawn here rather than borrowed: the ERE engine
had to be **built**, and paying it against three members was what justified the
override. A JSON reader does not have to be built (delta 2), so what this cohort
pays is small and what it retires is unchanged — `jq` leaves the battery for
every member but two, both with named owners (delta 7). A cheaper cohort for the
same retired blocker strengthens the override rather than weakening it, and a
later reader must not cite this cohort as precedent for *"what it buys is the
engine"* when what it bought was a `cargo add`.

## What changes

### 1. The cohort's selection evidence is recorded, from a run at the cut

The build session re-runs `--group` at cohort cut and records **the members, the
group key they shared, the group's size, the undecidable count, and the largest
takeable group's size** in the cohort's spec section — the eighth cohort's
recording contract, plus the last figure, which is what makes the override
legible. **[mechanical]**

The figures in this amendment's preamble are the *scoping* session's dated run
and are the ground for selecting the cohort, not the evidence the section
records: the recorded evidence is the build session's own run, because the tool's
output is not stable across tree changes and the undecidable count is the bound
on any claim about size.

### 2. The JSON reader is a dependency, and the cohort writes no parser

`native/Cargo.toml` takes **`serde_json`** and the cohort implements no JSON
parser. **[design-bearing]**

**The premise this delta was first authored under was false, and the correction
is the operator's.** The crate is under no no-dependency prohibition and never
was: the constraint is the **adopter's** — their machine requires only git and
the pre-compiled binaries — and TRAJECTORY.md §The objectives' objective 4
governs what an adopter installs and can uninstall, saying nothing about the
crate's build graph. A statically linked binary carrying `serde_json` leaves an
adopter on git-only. And no consumer ever compiles the crate: building from
vendored source at install time is **void** (TRAJECTORY.md §The objectives) and
`native/` ships no `checks/` or `smoke/`, so it is not a kit and `init` never
vendors it. The build graph therefore never reaches an adopter at all.

Under the corrected premise, hand-writing a JSON parser is a maintained copy of a
solved problem whose failure mode is a *silent mis-parse of an operator's
settings file* — escapes, surrogate pairs and the numeric grammar are fiddly
exactly where a wrong answer looks like a clean one. The dependency is the
cheaper and the safer half of that trade, and the parity oracle (delta 9) is what
proves it either way.

**What the hand-roll design owned and this one does not, stated so the merge does
not preserve dead requirements.** The bounded five-item public surface, the
input-key-order requirement and the no-third-consumer clause were properties of a
parser *we* owned. Under a dependency:

- The object model is `serde_json::Value` and the findings rendering is that
  crate's own serializer, deterministic by its contract. **Key order stops being
  ours to specify and stops mattering** — RULING 2's structural comparison is
  what makes it not matter. Had the comparison stayed byte-shaped, a dependency's
  map ordering would have become a new coupling, which is worth recording as a
  case where two independently-ruled decisions happened to fit.
- What stays **ours and bounded** is the pin-path layer: `Path::compile(&str) ->
  Result<Path, PathError>` and its evaluation over a `serde_json::Value`, plus
  delta 4's two preserved shell semantics. That is the only new surface this
  cohort authors, and it is where the bounding argument now attaches: no filter
  language, no mutation, no second grammar.

**Numeric equality needs one explicit rule**, because the dependency's model does
not give it for free: `serde_json::Value`'s equality compares a `Number` by
variant, so `1` and `1.0` are unequal, while `jq` — and therefore delta 4's
structural comparison — treat them as one value. **Numbers compare by their
`f64` value; every other shape compares by `Value`'s own equality.** A port that
takes `Value::eq` wholesale ships a false mismatch on a pin nobody would think to
test.

**The bar this first dependency was taken under.** It is a bar rather than a
precedent, and a candidate failing any clause is a design question rather than a
`cargo add`: it performs **no filesystem walk** (the condition
`native/src/walk.rs`'s own assertion states — delta 13); it spawns no subprocess
and opens no socket; its MSRV is at or below the crate's floor, or the floor
moves deliberately (delta 14); and its **transitive** set is small, enumerable,
and admitted under the same clauses. The standing policy this cohort does not
invent — a maintained roster, an audit cadence, licence review — is filed
costed rather than settled here by the first case to need it.

### 3. The pins knob's grammar is narrowed to a path expression, and an out-of-subset pin is a fail-closed refusal

`CONTEXT_KIT_SETTINGS_PINS`' left-hand side stops being *an arbitrary `jq`
filter* and becomes **a path expression**, defined here and enforced by
`Path::compile`. **[design-bearing]**

The accepted grammar, complete:

```
path   := '.' | step+
step   := '.' ident | '.' '"' string '"' | '[' '"' string '"' ']' | '[' int ']'
ident  := [A-Za-z_][A-Za-z0-9_]*
int    := '-'? [0-9]+
```

Everything else — pipes, filters, `?` error suppression, slices, iteration,
functions, arithmetic — is refused by `compile`, and the gate turns that refusal
into **exit 2** naming the offending pin, the knob it came from, and the
construct. That is §Fail-closed contract applied to a parser, the shape §The
POSIX ERE matcher already took for GNU's ERE extensions.

**This is a narrowing of a consumer config surface, taken openly, and it is not
the move the ERE cohort's foreclosure forbids.** That foreclosure binds
*sizing an implementation* to what one consumer happens to write while the
documented grammar stays wider — the silent mis-scan. This delta moves the
**documented grammar** instead, and refuses loudly outside it. The knob's whole
documented job is *naming a settings key*, which a path expression expresses in
full; measured against this tree, all three live pins are plain dot-field paths,
so the narrowing is a guard for consumers rather than a change to this repo —
the same standing the ERE refusals took.

**Operator-ruled 2026-08-14, and the alternative was priced before the ruling,
not after.** The ground this delta was first argued on — *implementing `jq` is
not a cohort* — is **retired**, because delta 2's correction makes a `jq`
implementation purchasable: a crate such as `jaq` would keep the
arbitrary-filter promise for the price of a dependency. That option was put to
the operator against this one, with the tradeoff stated, and the **documented
path grammar was chosen**. So the narrowing stands on the ruling and on the
knob's documented job, and a later reader must not reopen it on the observation
that a filter crate exists — that observation is what the ruling was made
against. What would reopen it is a *different* fact, and the escalation rule
holds: a finding that changes the shape of that choice goes to the operator
rather than being decided at build.

### 4. The pin comparison becomes structural, and the `jq -c` rendering stops being the contract

`check-settings-pins` compares the pinned value to the settings value **as parsed
JSON values**, not as rendered strings, and the pins grammar's right-hand side is
documented as *a JSON value* rather than *the exact `jq -c` rendering*.
**[design-bearing]**

The ground is that a byte-rendering contract is **unachievable across `jq`
versions and therefore cannot be a parity target**: `jq` 1.6 re-renders every
number through a double, `jq` 1.7 preserves an unmutated literal, so
`{"x":1e3}` renders `1000` under one and `1e3` under the other. A crate holding
byte parity with "`jq -c`" would be holding parity with whichever `jq` the
comparison ran against. Structural comparison has no such dependency, diverges
from the shell only in the forgiving direction (an expected side written with
non-canonical spacing now matches), and diverges on nothing this tree contains.

Two semantics are **preserved deliberately rather than improved**, because a port
proves parity and does not repair rules:

- **A path evaluating to `null` is the absent branch**, whether the key is absent
  or explicitly `null`. The shell cannot tell the two apart (it compares `jq`'s
  `null` output) and the crate can; reproducing the conflation is the faithful
  port. A session wanting the distinction files an entry against the merged
  spec.
- **Indexing follows `jq`'s own type rules**: a field step on `null` yields
  `null`; a field step on a string, number, boolean or array, and an index step
  on an object, are **errors**, which the gate classifies as a malformed pin
  (exit 2) exactly as the shell's non-zero `jq` status does today.

**Operator-ruled 2026-08-14 alongside delta 3, and untouched by the dependency
correction.** This delta's ground is a property of `jq`'s own output across its
versions, not of what the crate may link, so taking `serde_json` neither
strengthens nor weakens it — a dependency does not make the shell side's bytes
stable. Recorded because the two deltas were ruled together and a reader
re-checking one will reach for the other.

### 5. `check-settings-paths` ports with its extraction predicate hand-compiled

The allow-array read (`.permissions.allow[]?`, string-typed entries only) and the
command-token predicate — `Bash(…)` framing, skipped `env NAME=VALUE` prefixes, a
skipped `bash`/`sh` interpreter word, the `.sh` suffix test, the `*`-pattern skip,
and the **checked count** the clean line reports — port as direct code against
`Value`. **[design-bearing]**

This is §The POSIX ERE matcher's boundary applied unchanged: *a pattern the kit
owns is hand-compiled; a pattern a consumer supplies goes through the engine.*
The allow-array access is kit-owned and takes no path expression; only the pins
gate's left-hand side is consumer-supplied and only it reaches `Path::compile`.
The `read -ra`-without-expansion property the shell states is not a portable
concern in Rust, but the property it protects is: splitting the grant must not
glob it against the tree, and the checked count is what the fixture pair pins.

### 6. context-kit gains a shell library, because the config bridge sources one

Both knobs' defaults and the consumer-config load move out of the check scripts
into a context-kit library the bridge can source. **[design-bearing]**

The mechanism forces it: `_gate_knob_value` sources `<kit>/lib/*.sh` in a
subshell and **refuses a knob the library does not define** (§lib/gate.sh).
`CONTEXT_KIT_SETTINGS_FILE` and `CONTEXT_KIT_SETTINGS_PINS` are defaulted inside
the check scripts today, and `CONTEXT_KIT_CONFIG_FILE` / the gates-dir
`context-config.sh` are sourced there too — so a `.gate`-dispatched member would
resolve neither. The library takes queue-kit's `lib/queue.sh` shape: the config
load first, then each knob's default. Both defaults are repo-relative, which the
bridge requires — a bridged value may not carry an absolute path, because the
resolved argv is baked verbatim into the tracked pre-commit hook.

The library's readers are **every context-kit gate, not only the two ported
ones**: `check-memory-off` reads the pins path and its own memory-dir knob,
`check-brevity` reads the brevity section, `check-footprint-fresh` its measured
surfaces. Each sources the library rather than re-defaulting, which is the
single-home form `check-knob-default-coupling` asserts and which keeps the
shell-side and bridged values one value rather than two.

**Re-judged against delta 2's correction and unchanged, which is worth writing
down because this delta looks like it could have been constraint-driven.** It is
not: it is forced by the **config bridge's own mechanism** —
`gate-sdk/lib/gate.sh`'s `_gate_knob_value` sources `<kit>/lib/*.sh` and exits 2
when the sourced library defines no such knob — and that mechanism is indifferent
to what the crate links. A crate carrying `serde_json`, or carrying a TOML reader,
or carrying nothing at all, receives its knobs the same way. The delta stands
verbatim.

### 7. What the cohort does not take, and why

Two members that a mechanical reading puts in are excluded, each with its ground
recorded here so the next selector does not re-derive it. **[design-bearing]**

- **`check-memory-off` — held on criterion 2, named and not ported.** Its
  `--fixture <dir>` arm is a different code path from its live arm, whose corpus
  is the harness memory directory under `HOME` and is not in the tree at all, so
  the fixture pair proves nothing about the part being ported. §The first cohort
  records the same hold from the first selection. What it owes is criterion 2's
  **constructed-scenario** discharge — the state stood up in a throwaway tree
  with both implementations run over it — and that is the cohort taking it, not
  this one. It is `jq`-bound and this cohort's engine does not unblock it: its
  blocker is the oracle, not the dependency.
- **`check-installer-no-deps` — excluded with cause, and the ruling stands on the
  ground that survived delta 2's correction.** As the first `scripts/`-declared
  gate to enter `native/` it would drag in the tranche's unanswered first-mover
  questions — whether a consumer-declared member earns a conservation row, how
  `check-gate-substrate-parity` assertion B's owner column reads a member no kit
  ships — which are `consumer-gate-port-disposition`'s design work and are
  budgeted there. Its **other** original ground, that membership beyond what
  proves the engine adds risk without payoff, is materially weaker now the engine
  is a dependency rather than a build; it is recorded as retired rather than
  quietly kept, and the exclusion does not rest on it. The exclusion is ruled and
  the member is named in that entry as the tranche's **cheapest first mover**:
  one gate, one small corpus, the reader already linked.

**The honest claim about `jq`, which is the one this cohort must not overstate.**
After the cohort, `jq` is retired from the battery **but for those two members**,
both with named owners. It is **not** retired from the shipped install path at
all — `installer/lib/` shells to it and degrades silently, which
`installer-jq-silent-degradation` owns. "The cohort retires jq" is false in both
directions and is the sentence a later reader will otherwise write.

### 8. The provenance seam, ruled, because a settings reader is where it bites

The pin-path layer this cohort authors carries **no settings key, no pin path and
no permission vocabulary**. **[design-bearing]**

The pins manifest and the settings file are consumer config; the path grammar is
sized to a grammar rather than to a corpus, so there is nothing for a project
term to attach to. The permission-entry grammar `check-settings-paths`
hand-compiles (`Bash(…)`, an `env` prefix, an interpreter word) is a **public
harness format**, the same class as the `# graph:` manifest grammar the crate
already parses, and is not a private vocabulary. The check this reduces to: a
grep across `native/src/` for any pinned key spelling — `autoMemoryEnabled`,
`CLAUDE_CODE_`, `worktree.baseRef` — returns nothing.

The seam reaches the dependency too, and the reading is short: a general-purpose
JSON parser is **grammar, not vocabulary**, so it can no more carry a project
term than `ere.rs` could. What the seam still forbids is a *consumer-shaped*
dependency — one selected because it encodes some tree's terms — and the bar in
delta 2 excludes that class by construction.

### 9. The acceptance oracle is a differential run against `jq` itself

The crate's unit arm stands up a generated corpus of documents and paths and
compares **the ported gate's verdict** — exit code and finding set — against the
shell gate's, which is `jq`'s answer by construction. It runs under
`check-crate-arms`, so a divergence is a commit-time red. **[design-bearing]**

The oracle compares verdicts rather than `jq`'s rendered stdout, deliberately:
delta 4's whole ground is that the rendering is version-dependent, so an oracle
pinned to those bytes would fail on a contributor's newer `jq` while the gate was
correct. This is the `ere.rs`-against-`awk` shape with the comparison surface
moved one layer out.

**Delta 2 narrows what this arm is proving, and narrowing it is the point.** With
a hand-written parser the arm carried the parser too; with a dependency it proves
the layer this cohort actually authors — `Path` evaluation, the numeric-equality
rule, the two preserved shell semantics — against `jq`. The generator therefore
covers *path* shapes rather than *document* shapes: missing keys, explicit
nulls, a field step onto a scalar, an index step onto an object, negative
indices, and quoted keys carrying dots. A generator that varied documents and
held paths fixed would spend its budget re-testing `serde_json`.

Its cost, stated rather than absorbed: the arm needs `jq` at **contributor**
time. That touches criterion 7 not at all — the criterion adjudicates what a
rule the *payload* carries invokes — and it adds nothing to this repo's
contributor floor today, because delta 7's two held members keep `jq` in the
battery anyway. When the last of those ports, this arm is the remaining
requirement and is re-examined then rather than inherited silently.

### 10. Criterion 5 is priced by measurement, with the acceptability judgment fixed in advance

The cohort records the **binary-less residual** — the roster a consumer whose
payload carries no artifact for its host loses, and its count — measured with
`installer_smoke`'s binary-less leg, after this cohort's own commit, from a clean
checkout reached **by path**. **[design-bearing]**

Both members are `install: on-surface`, so the lifecycle-kit cohort's precedent
**predicts** growth of zero: a member no `init` seeds cannot be a member an
artifact-free `init` loses. That is a prediction the measurement rules on, never
a discharge — criterion 5 is explicit that N members each individually runnable
is not one. The judgment, fixed now so a build session facing a number has a
decision procedure: **zero → the cohort lands on that finding, with the
`on-surface` limit restated rather than banked** (an adopter who later brings the
surface into existence on a binary-less host does lose the member, declared);
**non-zero → the cohort lands only with one of criterion 5's three designed
answers named in its spec section** — restore the class shell-side, make it
binary-gated by a declaration the adopter receives, or accept and document.

### 11. The terminal queue move is a demotion, not a Done move

On completion `native-gate-port-remaining-corpus` drops its `[spec:]` tag and
returns to the deferred section under `[design-pending]`, `[roadmap:
now/reliability]` intact. **[mechanical]**

The entry's deliverable is the whole corpus and this amendment delivers one
increment; a Done move would assert a finished port with 60 members unported and
silently drop the item from the public roadmap projection. Stated because neither
half has a gate behind it. The entry has taken this demotion at each of the last
six cohorts.

### 12. What the cohort inherits unchanged, cited rather than restated

The per-member procedure is §Porting a gate to the binary substrate; the payload
rule is §Consumer payload; the criteria are §The port-candidate criteria.
**[mechanical]** Two standing obligations that bite per cohort, named so the
build session does not rediscover them: parity is proved **while both
implementations still exist**, since assertion A forbids a descriptor and a
script coexisting in one resolve dir; and the shell original is **deleted** for a
ported member rather than left beside it.

### 13. The crate's dependency assertion is widened, exactly as its own message instructs

`native/src/walk.rs`'s `the_crate_vendors_no_walker_because_it_vendors_nothing`
asserts that `[dependencies]` is empty. It is **widened to an allowlist, not
deleted**, and it is renamed, because its current name states the retired
premise. **[design-bearing]**

**Its purpose is sound and survives the correction intact.** The assertion is not
a no-dependency prohibition — it is the other half of `check-reads-couples`'
anti-vacuity mechanism (§Meta-gate conservation for the binary substrate): unit
test A asserts each member's *observed* walk roots are a subset of its declared
ones, and a **vendored walker cannot be caught by a spelling roster**, so a
dependency that walks the filesystem would make test A assert over an incomplete
observation. The test's own failure message already prescribes this delta —
*"confirm the new dependency performs no filesystem walk, then widen this test
deliberately rather than deleting it"* — which is foresight paying, and this
cohort is the first case to spend it.

Two changes, each with its ground:

- **The allowlist is over the resolved graph (`Cargo.lock`), not the direct
  `[dependencies]` table.** A transitive crate walks the filesystem exactly as
  visibly as a direct one, and reading only the manifest would admit an entire
  subtree unexamined. Each admitted crate is named with the clause of delta 2's
  bar it was admitted under, so the test is the machine-held form of that bar
  rather than a second list beside it.

  **This requires tracking the lockfile, which the tree does not do today**, and
  the dependency is what makes that a defect rather than a preference:
  `native/Cargo.lock` is **gitignored and untracked**. With an empty graph
  nothing turned on it. With a dependency, three readers need it — the allowlist
  above, which cannot assert over a graph it must resolve from the network; the
  publish workflow's per-target matrix, which must build every target from one
  resolution rather than from whatever the registry served that leg; and
  `check-gate-binary-fresh`'s source stamp, whose input set is
  `git ls-files` and therefore cannot see an untracked lock at all. So the lock
  becomes tracked with the dependency, in the same commit, and the `.gitignore`
  line goes with it. Found by sweep, not assumed — this amendment asserted a
  "committed `Cargo.lock`" one draft earlier and the tree did not have one.
- **One clause of the failure message is deleted as false**: *"an empty
  dependency set is also what gate-sdk/SPEC.md's vendoring model states the
  payload rests on"*. The payload rests on a **prebuilt binary**; the crate is
  never vendored and its build graph never reaches a consumer. That clause is one
  of the instances of the conflation the operator corrected, and it is fixed
  here rather than left because a test's message is read exactly when someone is
  deciding whether to add a dependency.

### 14. The toolchain floor is re-derived, never assumed to hold

A dependency's own MSRV can exceed the crate's, so the floor is **measured
against the resolved graph at build** and, if it moves, moves on **every** surface
stating it. **[mechanical]**

The roster is six surfaces, swept rather than recalled, because a floor bump that
misses one is a silent disagreement between what the tree requires and what it
tells a contributor to install: `native/Cargo.toml`'s `rust-version`;
`context-kit/lib/toolfloor.sh`'s `PROBE_SET`; context-kit/SPEC.md's prose
documenting that floor; `docs/install.md`'s Requirements bullet;
`scripts/check-install-toolchain.sh`, the freshness gate holding the last two in
parity; and the toolchain fixtures under `context-kit/index-tests/` and
`scripts/gate-tests/`, which **hardcode the value in their own cases** and would
therefore keep passing against a stale one rather than catching the drift.

Named in advance rather than met at CI, because the failure arrives as a compile
error on a contributor's older toolchain rather than as a red gate, and because
the floor is a **contributor-facing** number that delta 2's correction does not
touch: the adopter installs no toolchain at all.

### 15. The cold-build cost claim is re-measured where the dependency changes it

§upgrade-smoke's cost argument rests on a cold release build of the crate being
cheap, and a dependency changes both the time and the **network** assumption: a
cold registry cache needs a fetch, and the smoke builds in a detached worktree.
The build session re-measures and records the figure. **[mechanical]**

The pre-dependency claim in that section is **already wrong** and is corrected
outside this amendment as debt (it is a false statement about the tree, not a
design change): measured on this machine, `--offline`, from an empty target
directory, a cold `cargo build --release` of the crate as it stands takes
**≈2.5 s**, not the *"few hundred milliseconds"* the section claims. What this
delta owes is the *post-dependency* figure. If the smoke's worktree build cannot
complete on a machine with a cold registry cache and no network, that is a
finding to **escalate** rather than absorb — it would move a cost from the
contributor onto the smoke's environment assumptions.

**The same cost lands on CI, and the surface that states it is not prose.**
`.github/workflows/publish.yml` records that the empty dependency set *"is what
makes a matrix leg network-free and cheap"*, and neither that workflow nor
`gates.yml` provisions a cargo-registry cache. So a first dependency adds a cold
fetch to **every** matrix leg and every CI build step, uncushioned. The comment
is true today and becomes false with the dependency, so it is this delta's to
correct rather than the debt sweep's; whether a registry cache is worth adding is
a measurement the build session takes with the figure in hand, not a reflex.

### 16. `ere.rs` stays hand-written, and the ground is its semantics rather than the retired premise

The correction makes *"why is there a hand-written regex engine?"* a live
question, and the answer is recorded here so a later reader does not read
`native/src/ere.rs` as a casualty of a premise that turned out false.
**[design-bearing]**

§The POSIX ERE matcher justifies the hand-roll partly on *"the crate vendors
nothing — asserted rather than assumed, by the unit test that fails the build on
a non-empty dependency list"*. That clause is **retired** by delta 2's
correction. The engine stays anyway, on a ground the correction does not touch:
its contract is **POSIX leftmost-longest** span reporting, which is the semantics
`awk`'s `RSTART`/`RLENGTH` gives and which the ecosystem's ordinary matchers do
not — leftmost-first is the common default, and §The POSIX ERE matcher already
records that the two agree on every `is_match` and disagree on spans. A
dependency with the wrong span semantics would be a silent regression in exactly
the place that section says a boolean-only oracle cannot see. Replacing it is
therefore a costed design question with a real candidate set, not a cleanup this
cohort performs in passing, and this amendment does **not** open it.

## Producers and consumers

**This amendment introduces one new interface (the pin-path layer), one new
configuration surface (the context-kit library), one new build-graph dependency,
and no new state, event, message or field.** A port re-implements existing rules
on the compiled substrate; the descriptors, the dispatch seam, the manifest
format and the parity harness exist and are unchanged.

- **The pin-path layer** (delta 2). Producer: the crate, compiled into the
  binary. Consumers: exactly two, both named — `native/src/gates/settings_pins.rs`
  (`Path::compile`, evaluation, the numeric-equality rule) and
  `native/src/gates/settings_paths.rs` (a hand-compiled allow-array access over
  `serde_json::Value`, taking no path expression). No third consumer exists at
  landing, and delta 2's bounding is what keeps a later one deliberate.
- **`serde_json` in the resolved build graph** (delta 2). Producer:
  `native/Cargo.toml` and the committed `Cargo.lock`. Consumers, all named
  because a dependency with no named reader is exactly what delta 13's allowlist
  refuses: the two gate modules above; `native/src/walk.rs`'s widened assertion,
  which reads the resolved graph and admits each member under a stated clause;
  the publish workflow's per-target build, which now fetches it; and
  `gate-sdk/bin/build-native.sh`, whose first build on a cold machine needs the
  registry. **No consumer-side reader exists and that is the correction's whole
  content**: the adopter receives a linked artifact, never the graph.
- **The pins path grammar** (delta 3). Producer: a consumer authoring
  `scripts/settings-pins.conf`; this repo's producer is that committed file, with
  three live pins. Consumer: `Path::compile`, at gate invocation. The refusal
  path's consumer is the operator reading exit 2's message, which names the pin,
  the knob and the construct — a refusal whose text does not name the knob sends
  its reader to the wrong file.
- **`CONTEXT_KIT_SETTINGS_FILE` and `CONTEXT_KIT_SETTINGS_PINS` across the config
  bridge** (delta 6). Producer: the new context-kit library, sourced by
  `_gate_knob_value` in a subshell, having first sourced the consumer's
  `context-config.sh`. Consumer: `gate_command`, which asks the binary
  `--knobs <name>`, resolves each declared knob and emits it as a
  `GATE_SDK_KNOB_<NAME>=` argv element; the binary reads it back at dispatch.
  **The enabling config is emitted everywhere it must be**: the crate declares
  only the knobs its own code reads (`--knobs` is registry data a member cannot
  compile without), so an unread bridged knob is unconstructible and an
  undeclared read one fails closed at `--knobs` rather than defaulting silently.
  The shell-side readers — the three remaining context-kit gates — source the
  same library, so one value has one home across both substrates.
- **Each ported member's `.gate` descriptor.** Producer: the build session,
  writing it as the `.sh` is deleted. Consumer: `gate_resolve` at dispatch,
  reaching `gate_command` and the subcommand. Every descriptor's reader already
  exists, which is what makes a port a port.
- **The recorded selection evidence and the measured residual** (deltas 1, 10).
  Producer: the build session, into the cohort's spec section. Consumer: **the
  session cutting the tenth cohort**, which reads the undecidable count to know
  how blind the "largest takeable group is one" claim was, and the residual to
  know which value class is already thin.

**Every new field has a named reader.** The dependency's `Value` carries no
fields of ours; the pins gate's finding record (pin path, expected, actual) is
read by the operator at exit 1 and by the fixture pair's `expect.txt`, and the
paths gate's checked count is read by its `good/` expectation — which is the one
that catches a predicate that vacuously matched nothing. The allowlist entries
delta 13 adds carry one field each, the admitting clause, and its reader is the
next session weighing a dependency.

### The narrowings, and each reader's red condition

This delta narrows two corpora: the gate-source walk (two `.sh` files deleted)
and the pins grammar (delta 3). Point 5 of the causal-completeness check binds,
so the readers are enumerated by **what makes each red**, never by subject.

- **`check-shellcheck`** — reds on a ShellCheck finding, and separately **exits 2
  on an empty target set**. A reds-on-finding-none condition, not monotone.
  Clear here because the walk spans every kit's `lib/`, `bin/`, `checks/` and
  `templates/`, and context-kit retains three check scripts plus its libraries;
  re-checked at build rather than inherited, since the ground is a count.
- **`check-gate-fixture-coverage`** — its subject is a member's fixture pair, and
  both pairs are **retained**. Enumerated because deleting the script and
  orphaning its pair are one action apart.
- **`check-gate-substrate-parity`** — assertion A reds on a descriptor and a
  script **coexisting** in one resolve dir, so the narrowing *satisfies* it and a
  half-done one violates it — the inverse of the reflex. Assertion B reds when
  the descriptor set and the `--list` roster disagree, including the owner
  column, so both new subcommands declare `context-kit` as owner. Assertion C
  re-derives the substrate-sensitive set; see the conservation row below.
- **`check-reads-couples`** — reds when a member's observed walk roots are not a
  subset of its declared ones, and the binary answers `--reads` for a
  `.gate` member. Both new members read **named files** rather than walking, so
  each declares its read set beside its dispatch entry and unit test A holds the
  declaration to what the fixture cases observe. A `?` declaration would be the
  wrong answer here: both roots are bounded and knowable.
- **`check-measured-claim`** — reds when a marked literal disagrees with its
  oracle, and `docs/install.md` carries
  `<!-- measured: ported-gate-members=42 -->`. This cohort moves that number, so
  the marker's oracle disagrees the moment the descriptors land and the page is
  re-emitted with the cohort. A concrete instance of the row §Meta-gate
  conservation records for it: a port *moves its value*, which is the mechanism
  working.
- **`check-settings-paths` itself** — reds on a committed allow entry naming a
  `.sh` path that does not resolve, which is precisely what deleting a check
  script creates. **Measured this session against `.claude/settings.json`:** the
  allow array's only literal `checks/` grants name `gate-sdk/checks/check-graph.sh`
  and `lifecycle-kit/checks/check-stage-entry.sh`; the `*/checks/check-*.sh`
  grants carry a `*` in the command token and are skipped by the predicate. **No
  entry names either member**, so this cohort strands nothing and needs no
  operator settings edit — the landing-order constraint context-kit/SPEC.md
  states (*the operator prunes, then the gate registers*) does not bite here.
  Re-verified at build, because the file is operator-owned and may have moved.
- **`check-docs-cmd`** — reds on a governed doc fencing a command path that does
  not resolve. Both scripts are cited in context-kit/SPEC.md prose (§check-settings-pins,
  §check-settings-paths, §Layout and configuration's tree) as backticked paths
  rather than fenced commands; the spec text changes with delta 4/6 regardless,
  and the docs mirror regenerates. Named because "we deleted the script" and "the
  docs still name it" is the class this gate exists for.
- **`check-graph`, `check-enforcement-fresh`, `check-value-rollup-fresh`,
  `check-footprint-fresh`, the generated pre-commit hook** — each reds on
  staleness against a changed declaration set. Every ported member changes the
  manifests those projections derive from, so all stale by construction and are
  regenerated with the cohort; each prints its own regen command on red.
- **`check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness`,
  `check-deprecation-task`, `check-readme-roster`, `check-exec-bit`** — each reds
  over the widened comment surface: the corpus follows the rule to
  `native/src/gates/*.rs` and the `.gate` descriptor, whose lines are directives
  by construction, and the descriptor must be **non**-executable.
- **The pins-grammar narrowing's own reader is `check-settings-pins`**, and its
  red condition under the narrowing is **exit 2 naming the pin** — loud and
  unmissable, never a silent pass. A narrowing that turned an out-of-subset pin
  into a *clean* verdict would be the failure this enumeration exists to catch,
  and delta 3 rules the opposite.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by deltas 1, 2, 7, 9, 10. It gains a cohort subsection, **§The settings
  cohort, and the crate's first dependency** — named for what the cohort actually
  sets a precedent for. It is deliberately *not* named on §The POSIX ERE
  matcher's engine pattern: an earlier draft called it §The JSON reader, and that
  name would have advertised a built engine where a `cargo add` is what happened.
  It records the members, the shared derivation, the size-arm exhaustion and the
  override it selected under, the undecidable count, the dependency and the bar
  it cleared, the pin-path contract and its refusals, the two holds, the parity
  evidence and the measured residual with its acceptability ruling.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by delta 1. Its existing `check-settings-paths` row stands on its reasoning and
  gains the sentence the reverse-trigger row above already carries for its
  members: this member is itself `.gate`-dispatched from this cohort, so the row
  describes a ported gate. Assertion C's derivation is re-run at the cut and any
  member it newly selects owes a row.
- **context-kit/SPEC.md §check-settings-pins** — owned by deltas 3, 4, 6. The
  grammar sentence (*"one `<jq path> = <expected JSON>` per line … the expected
  side is the exact `jq -c` rendering"*) is replaced by delta 3's path grammar
  and delta 4's structural comparison; the absent-key disposition gains the
  null-conflation rule; the fail-closed roster loses *"no `jq`"* and gains *a pin
  outside the path grammar*.
- **context-kit/SPEC.md §check-settings-paths** — owned by deltas 5, 6. Its
  closing criterion-7 paragraph (*"`jq` is not on `GATE_SDK_PROGRAM_FLOOR`, so
  this gate fails port criterion 7 the day it lands and owes designed-away work
  at its port"*) is **discharged and rewritten**, not deleted: the work it named
  is this cohort, and the paragraph becomes the record that it was paid. The
  `--fixture <dir>` arm's description follows the implementation to the
  subcommand.
- **context-kit/SPEC.md §Layout and configuration** — owned by delta 6. The tree
  listing loses two `checks/*.sh` entries and gains their descriptors; the new
  `lib/` file is listed with the knob roster it now owns.
- **context-kit/README.md** — owned by deltas 5 and 6, because
  `check-readme-roster` reds in both directions on a roster that does not carry
  the new spelling.
- **`.workflow/validate-baseline.txt`** — owned by delta 10, if the residual
  measurement leaves an unpaid price; read the file for whether a held `fail` row
  stands rather than assuming one does.
- **`docs/` mirror and `docs/install.md`** — owned by deltas 1 and the
  `check-measured-claim` reader above: the mirror regenerates and the marked
  `ported-gate-members` literal is re-emitted from its oracle.
- **gate-sdk/SPEC.md §The POSIX ERE matcher** — owned by deltas 2 and 16. Its
  clause *"the crate vendors nothing — asserted rather than assumed, by the unit
  test that fails the build on a non-empty dependency list — so porting it means
  hand-writing an ERE engine"* is corrected: the crate may take dependencies, the
  test is an allowlist (delta 13), and the engine stands on its leftmost-longest
  semantics rather than on the retired premise.
- **gate-sdk/SPEC.md §Meta-gate conservation, the `check-reads-couples` row** —
  owned by delta 13. Unit test B is stated there as *"no module outside that walk
  implementation names a filesystem-walk API or vendors a walker"*; the vendored
  half is now held by an allowlist over the resolved graph rather than by an
  empty one, and the row says so, because a reader checking whether test A is
  still sound will start there.
- **gate-sdk/SPEC.md §upgrade-smoke** — owned by delta 15, which records the
  post-dependency cold-build figure. The pre-dependency claim in that section is
  false independently of this cohort and is corrected as debt outside this
  amendment; the delta owes only the new measurement.
- **`native/Cargo.toml`, `native/Cargo.lock` and `.gitignore`** — owned by
  deltas 2 and 13: the dependency, its resolved graph, and the ignore line that
  keeps the lock untracked today. All three move in one commit, since an
  allowlist over an untracked lock asserts over a file the stamp cannot see.
- **`context-kit/lib/toolfloor.sh` and the five other floor surfaces delta 14
  rosters** — owned by delta 14, and only if the measurement moves the floor.
- **`.github/workflows/publish.yml`'s build-step comment** — owned by delta 15:
  it states the empty dependency set is what makes a matrix leg network-free, and
  that stops being true with the dependency.
- **gate-sdk/SPEC.md §The canon-kit `spec_manifest_files` cohort** — owned by
  delta 16, which rules the disposition its *"the crate vendors nothing … so a
  port hand-writes the engine"* sentence rests on. The sentence itself is a false
  premise standing in the tree now and is corrected as debt outside this
  amendment; what the delta owes the section is the surviving ground.

## Definition of Done

- [ ] **Causal completeness** — the pin-path layer has two named consumers and no
      third; the dependency has every named reader in delta 2's producer list and
      no consumer-side one; the bridged knobs' producer is a library the bridge
      actually sources, proved by a green dispatch rather than by inspection;
      every narrowed corpus's reader has its **red condition** enumerated, with
      the reds-on-empty (`check-shellcheck`) and inverse (`assertion A`) cases
      named rather than cleared by the "a narrower corpus can only remove
      violations" reflex.
- [ ] **The dependency cleared the bar and the machine holds it** — each crate in
      the resolved graph admitted under a named clause of delta 2; the walk.rs
      assertion widened to that allowlist over the **lock**, not the manifest,
      renamed, and its false clause deleted; `native/Cargo.lock` tracked and its
      `.gitignore` line removed in the same commit.
- [ ] **The floor and the build costs are measured, not assumed** — the MSRV
      re-derived against the resolved graph and, if moved, moved on all six
      surfaces delta 14 rosters; the post-dependency cold-build figure recorded;
      the publish workflow's network-free claim corrected.
- [ ] **Selection evidence recorded** — members, shared derivation, group size,
      undecidable count and the largest-takeable-group size, from a `--group` run
      at the cut.
- [ ] **Parity proved while both implementations existed** — per member, over the
      fixture pair and the live tree, before the `.sh` is deleted; the
      differential arm (delta 9) landed and running under `check-crate-arms`.
- [ ] **The narrowings are loud** — an out-of-subset pin exits 2 naming pin, knob
      and construct; a fixture case pins that arm.
- [ ] **Criterion 5 priced** — residual measured with the binary-less leg after
      this cohort's commit, from a clean checkout by path; acceptability ruled
      per delta 10.
- [ ] **The `jq` claim is stated honestly** — retired from the battery but for
      `check-memory-off` and `check-installer-no-deps`, and not retired from the
      install path at all.
- [ ] **Provenance seam held** — no settings key, pin path or permission
      vocabulary in `native/src/`.
- [ ] **Merged with no information lost** — §The settings cohort, and the crate's
      first dependency integrated into §The first cohort, and the rule that
      selects the next; the context-kit sections rewritten in place rather than
      appended to.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than
      at the commit while `SPEC-born-native.md` is in flight.
- [ ] **Removals propagated** — grepped every spec for the two `.sh` paths and
      for the retired `jq -c` rendering contract; nothing dangles.
- [ ] **Terminal move is a demotion** — `native-gate-port-remaining-corpus`
      returns to deferred under `[design-pending]` with `[roadmap:]` intact.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
