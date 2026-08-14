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

**This is the third exercise of the override, and the second where the engine is
the payoff** — §The POSIX ERE matcher is the first (*"what it buys is the
engine"*), `check-roadmap-fresh`'s hold the earlier one. Paying a JSON reader
against two members retires `jq` from the battery for every member but two, both
with named owners (delta 7).

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

### 2. The crate gains a JSON reader, and its grammar is sized to JSON rather than to this consumer

`native/src/json.rs` lands as the cohort's engine, on the `native/src/ere.rs`
precedent: a dependency-free reader whose public surface is bounded and named
here, so a later port extends it deliberately rather than by accident.
**[design-bearing]**

- **`Json::parse(&str) -> Result<Value, JsonError>`** accepts **RFC 8259 JSON in
  full** — every scalar form, nesting, the full escape set, and the numeric
  grammar including exponents. It is not sized to the keys this consumer's
  settings file happens to carry, for the reason §The canon-kit
  `spec_manifest_files` cohort fixes for globs and §The POSIX ERE matcher
  restates for patterns: the config surface permits what this consumer does not
  write, and a narrow reader silently mis-parses the first consumer who writes
  it.
- **`Value`** models the six JSON shapes and preserves an object's **input key
  order**. The ordering is load-bearing rather than incidental — the rendering
  below depends on it, and a map that reorders keys makes the rendered form a
  function of the implementation's hash seed.
- **`Path::compile(&str) -> Result<Path, PathError>`** and
  **`Value::get(&Path) -> Result<Option<&Value>, PathError>`** — the pin-path
  grammar delta 3 fixes, and its evaluation.
- **`Value::render_compact(&self) -> String`** — the compact rendering used in
  findings text only, never as a comparison surface (delta 4 is why).

There is no mutation API, no filter language, no streaming parser and no
serializer for constructed values; adding one is a design decision with its own
reader rather than an omission to fill in. The reader is **byte-oriented over
UTF-8**, and an input that is not valid UTF-8 is a parse error rather than a
lossy conversion — a settings file the harness cannot have written.

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
**documented grammar** instead, and refuses loudly outside it. The ground: `jq`
is a language, not an engine, and implementing it is not a cohort; the knob's
whole documented job is *naming a settings key*, which a path expression
expresses in full. Measured against this tree, all three live pins are plain
dot-field paths, so the narrowing is a guard for consumers rather than a change
to this repo — the same standing the ERE refusals took.

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
- **`check-installer-no-deps` — excluded with cause.** The cohort buys the
  engine, and membership beyond what proves the engine adds risk without adding
  payoff. As the first `scripts/`-declared gate to enter `native/` it would drag
  in the tranche's unanswered first-mover questions — whether a
  consumer-declared member earns a conservation row, how
  `check-gate-substrate-parity` assertion B's owner column reads a member no kit
  ships — which are `consumer-gate-port-disposition`'s design work and are
  budgeted there. It is named in that entry as the tranche's **cheapest first
  mover**: one gate, one small corpus, engine already paid.

**The honest claim about `jq`, which is the one this cohort must not overstate.**
After the cohort, `jq` is retired from the battery **but for those two members**,
both with named owners. It is **not** retired from the shipped install path at
all — `installer/lib/` shells to it and degrades silently, which
`installer-jq-silent-degradation` owns. "The cohort retires jq" is false in both
directions and is the sentence a later reader will otherwise write.

### 8. The provenance seam, ruled, because a settings reader is where it bites

`native/src/json.rs` carries **no settings key, no pin path and no permission
vocabulary**. **[design-bearing]**

The pins manifest and the settings file are consumer config; the path grammar is
sized to a grammar rather than to a corpus, so there is nothing for a project
term to attach to. The permission-entry grammar `check-settings-paths`
hand-compiles (`Bash(…)`, an `env` prefix, an interpreter word) is a **public
harness format**, the same class as the `# graph:` manifest grammar the crate
already parses, and is not a private vocabulary. The check this reduces to: a
grep across `native/src/` for any pinned key spelling — `autoMemoryEnabled`,
`CLAUDE_CODE_`, `worktree.baseRef` — returns nothing.

### 9. The acceptance oracle is a differential run against `jq` itself

`json.rs`'s unit arm stands up a generated corpus of documents and paths and
compares **the ported gate's verdict** — exit code and finding set — against the
shell gate's, which is `jq`'s answer by construction. It runs under
`check-crate-arms`, so a divergence is a commit-time red. **[design-bearing]**

The oracle compares verdicts rather than `jq`'s rendered stdout, deliberately:
delta 4's whole ground is that the rendering is version-dependent, so an oracle
pinned to those bytes would fail on a contributor's newer `jq` while the gate was
correct. This is the `ere.rs`-against-`awk` shape with the comparison surface
moved one layer out.

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

## Producers and consumers

**This amendment introduces one new interface (the JSON reader's public surface),
one new configuration surface (the context-kit library), and no new state,
event, message or field.** A port re-implements existing rules on the compiled
substrate; the descriptors, the dispatch seam, the manifest format and the parity
harness exist and are unchanged.

- **`native/src/json.rs`'s public surface** (delta 2). Producer: the crate,
  compiled into the binary. Consumers: exactly two, both named —
  `native/src/gates/settings_pins.rs` (parse, `Path::compile`, `Value::get`,
  `render_compact` for findings) and `native/src/gates/settings_paths.rs` (parse
  and a hand-compiled allow-array access). No third consumer exists at landing,
  and the bounded surface in delta 2 is what keeps a later one deliberate.
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

**Every new field has a named reader.** The reader's `Value` carries no fields of
its own beyond the JSON shapes; the pins gate's finding record (pin path,
expected, actual) is read by the operator at exit 1 and by the fixture pair's
`expect.txt`, and the paths gate's checked count is read by its `good/`
expectation — which is the one that catches a predicate that vacuously matched
nothing.

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
  owned by deltas 1, 7, 9, 10. It gains a cohort subsection, **§The JSON
  reader**, named for what the cohort buys on §The POSIX ERE matcher's precedent
  rather than for a group key. It records the members, the shared derivation, the
  size-arm exhaustion and the override it selected under, the undecidable count,
  the reader's contract and its refusals, the two holds, the parity evidence and
  the measured residual with its acceptability ruling.
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
- **context-kit/README.md** — owned by delta 12, because `check-readme-roster`
  reds in both directions on a roster that does not carry the new spelling.
- **`.workflow/validate-baseline.txt`** — owned by delta 10, if the residual
  measurement leaves an unpaid price; read the file for whether a held `fail` row
  stands rather than assuming one does.
- **`docs/` mirror and `docs/install.md`** — owned by deltas 1 and the
  `check-measured-claim` reader above: the mirror regenerates and the marked
  `ported-gate-members` literal is re-emitted from its oracle.

## Definition of Done

- [ ] **Causal completeness** — the reader's public surface has two named
      consumers and no third; the bridged knobs' producer is a library the bridge
      actually sources, proved by a green dispatch rather than by inspection;
      every narrowed corpus's reader has its **red condition** enumerated, with
      the reds-on-empty (`check-shellcheck`) and inverse (`assertion A`) cases
      named rather than cleared by the "a narrower corpus can only remove
      violations" reflex.
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
- [ ] **Merged with no information lost** — §The JSON reader integrated into §The
      first cohort, and the rule that selects the next; the context-kit sections
      rewritten in place rather than appended to.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than
      at the commit while `SPEC-born-native.md` is in flight.
- [ ] **Removals propagated** — grepped every spec for the two `.sh` paths and
      for the retired `jq -c` rendering contract; nothing dangles.
- [ ] **Terminal move is a demotion** — `native-gate-port-remaining-corpus`
      returns to deferred under `[design-pending]` with `[roadmap:]` intact.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
