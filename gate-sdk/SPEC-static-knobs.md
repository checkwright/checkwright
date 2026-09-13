# SPEC amendment: static-knobs

Queue entry: `config-seam-static-format`. It leads unit set `static-config-seam` under
**operator direction, 2026-09-13, lead-relayed**, and builds after the set's citation-liveness
pair, whose widened `check-docs-cmd` polices the paths this cut retires.

**Three design choices were escalated at this stage, and each carries operator direction,
2026-09-13, lead-relayed.** These are directions, not rulings, so a later scope or spec may
revise them.

- **The cut boundary.** Cut 1 is site-kit plus doctrine-kit, and only this entry is paired.
  `config-bridge-resolution-cost` stays Deferred and closes at the gate-sdk cut, which retires
  the bridge. That revises the unit-set direction scope recorded.
- **Precedence.** A scalar knob resolves from the environment, then the `.local` overlay, then
  the tracked file, then the crate default. Indexed and keyed knobs take no environment
  override. The order is declared per cut while migration is in progress, and
  `config-seam-overrides-harness-pin` stays Deferred (delta 3).
- **The format.** A dependency-free line grammar (delta 1 and §The format ruling).

## The question, and the figures re-measured

A knob's value is computed today by sourcing bash. The owning kit's `lib/*.sh` and the consumer's
`<gates-dir>/<kit>-config.sh` are sourced in a subshell by `gate-sdk/lib/gate.sh`, which emits
`env GATE_SDK_KNOB_<NAME>=… <binary> <member>` (gate-sdk/SPEC.md §lib/gate.sh). So every battery
run, every hook regeneration and every harness hook enters through bash before the binary runs.
Native Windows therefore needs Git-for-Windows bash at runtime (docs/install.md), and TRAJECTORY
objective 6's dual-implementable surface does not hold on the runtime path.

**Re-measured at this stage, against the entry's figures:**

- `# no-port` declarations: **70**, and **34** of them are the seam's format restated: 18
  config-seam files (10 `scripts/*-config.sh` plus 8 kit templates), 13 kit libraries, and the
  three bridge front-ends (`bin/run-gates.sh`, `bin/gen-pre-commit.sh`,
  `bin/run-consumer-smoke.sh`). The entry's figures hold.
- `*_CMD` knobs: **8**, not seven. Five are canon-kit's, where shell runs the command and the
  crate reads the string only as an emptiness signal. The other three are
  `CONTEXT_KIT_HOOK_CMD` (derived), and `DELEGATION_KIT_REFRESH_CMD` and
  `DELEGATION_KIT_LIVENESS_CMD`, which the crate executes.
- Fixture suites sourcing `lib/test-hermetic.sh`: **96**. There are 110 files that mention it,
  which is the figure scope counted, and the entry's 92 is older.
- Registered gates: **113, every one `.gate`-dispatched.** No shell gate remains, so knob
  resolution is the only bash on the battery's own path.
- Knob shapes across the 11 knob-bearing libraries: scalars and indexed arrays dominate. There
  are **5 associative knobs** (`GUARD_KIT_BREADTH_DECLARED`, `GUARD_KIT_RO_FORMS`,
  `QUEUE_KIT_LESSON_SINKS`, `LIFECYCLE_KIT_PREDECESSOR`, `EVIDENCE_KIT_SCENARIO_GLOBS`), **2
  config-built prefix families** (`EVIDENCE_KIT_RUN_`, `EVIDENCE_KIT_PARSER_`), roughly **60
  derived defaults** (most ride `GATE_SDK_WORKFLOW_DIR`, `GATE_SDK_TMP_DIR` or
  `GATE_SDK_GATES_DIR`), and **10 knobs gated on `GATE_SDK_RESOLVING_KNOB`**, all canon-kit's.
  Those 10 are the *output* of a consumer command.
- This repo's live configs compute in three files of ten: a same-kit knob reference
  (`scripts/canon-config.sh`), a cross-kit reference that sources a sibling config
  (`scripts/drift-config.sh`), and a `while` loop building the `EVIDENCE_KIT_RUN_` family over
  `gate_fixture_suites` (`scripts/evidence-config.sh`). The kit templates compute nothing.
- Value hazards a format must carry without quoting rules: whitespace inside elements, which is
  pervasive; `=` inside an element (`LIFECYCLE_KIT_ENTRY_PREFLIGHT`); a literal `$` (a regex
  anchor); embedded double quotes (`SITE_KIT_RENDERER`'s Ruby snippet); an apostrophe. No
  default or config value carries a tab, a newline or a `#`.
- The crate: 253 call sites over `walk::knob_{array,map,prefix,scalar}`, plus a private twin in
  `spec.rs` and 5 sites that read the environment inline. Absent is an error and empty is a
  resolved-empty value. One dependency, `serde_json`. No shared file-grammar parser.

**The whole seam does not fit one iteration, and the unit is sized as its first increment.** It
is the format ruling, the resolution architecture every later cut inherits, and the first
migration cut. The entry outlives this amendment, so its terminal move is the corpus-increment
demotion (canon-kit/SPEC.md §Merging an amendment, step 4), and delta 9 writes the rule that
selects the next cut.

## The format ruling

Three candidates. One is taken.

- **Refused: TOML through a crate.** It is the most familiar to an adopter, and it clears
  gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency only at a price:
  a transitive set several packages wider than `serde_json`'s, and an MSRV input. The larger
  costs are elsewhere. TOML's string quoting reintroduces the hazards the measured values carry
  (the Ruby snippet's quotes, a regex `$`, an apostrophe) as escape rules an adopter can get
  wrong. It has no reference form, so the same-kit and cross-kit references would need a
  convention inside strings. And a TOML reader is not something a PowerShell or bash twin can
  implement in a few lines, which objective 6 wants of any surface that must survive.
- **Refused: a TOML subset under a hand parser.** An adopter writes valid TOML and the subset
  refuses it. A format that looks standard and is not is the worst of both.
- **Taken: a line grammar the binary parses, one value per line, raw to end of line.** It maps
  one-to-one onto the wire the bridge already speaks (an element per tab-joined field, a
  `<key>=<value>` pair per map entry), whose newline and tab refusals are this grammar's own
  limits. No quoting and no expansion exist, so every measured hazard is carried verbatim.
  Each line is independently parseable, so a refusal names its file and line. A twin in any
  interpreter is a dozen lines.

## The seam

- **Kit mechanism:** the knob-file grammar and reader, per-knob ownership routing, each migrated
  kit's defaults table (every default here is generic layout, never vocabulary), the precedence
  order, the legacy refusals, and the `--emit knob-roster` arm.
- **Consumer config:** each `<gates-dir>/<stem>-config.knobs` file and its gitignored `.local`
  overlay. The files remain the adopter's edit seam, so gate-sdk/SPEC.md §The config-seam port
  disposition's ground holds and only the file's substrate changes.
- **This repo's own:** `scripts/site-config.knobs`, which carries the project's host aliases
  (rule content the provenance seam keeps consumer-side).
- **Private rule content:** none in reach.

## What changes

### (1) The knob file grammar {design-bearing}

gate-sdk/SPEC.md gains §The knob file under §Layout and configuration. **Not yet applied:**

> A **knob file** is UTF-8 text read line by line. A line that is blank, or whose first
> non-blank character is `#`, is ignored. Every other line is one of three forms:
>
> ```
> NAME = value            a scalar
> NAME[] = element        one element of an indexed knob, in file order
> NAME[key] = value       one pair of a keyed knob
> ```
>
> `NAME` is a SCREAMING_SNAKE knob name. The first `=` on the line separates the head from the
> value. Blanks around the head and the value are trimmed, and the value is otherwise taken
> **verbatim to end of line**: no quoting, no escapes, no expansion, and `#` or `=` inside a
> value is data. A `key` is non-empty and carries no `]`, `=` or tab. A value carrying a tab is
> refused, the bridge's own element rule, and a newline is unexpressible by construction.
>
> A knob named in a file is **replaced whole**: its lines in that file are its entire value,
> and nothing merges with the default. `NAME =` with an empty value sets an indexed or keyed
> knob to the resolved-empty value, which is the reading the wire already gives an empty
> serialization. A knob's **shape is its owner's declaration**, and the file does not choose it.
> A line whose form disagrees with the declared shape is refused with its file and line, exit 2.
> So are an undeclared name for the owning kit, a malformed line, and a scalar or keyed pair
> given twice in one file. Leading or trailing blanks inside a value are unexpressible, and no
> live value carries them.
>
> **What the grammar deliberately lacks, and where each lack is answered.** It has no
> reference to another knob, no command substitution and no loop. Every derived value a kit
> library computes today moves to that kit's defaults table (delta 2). Every value a consumer
> config computes today is either a *reference* (`scripts/canon-config.sh`,
> `scripts/drift-config.sh`) or a *generated family* (`scripts/evidence-config.sh`), and neither
> is in the first cut. The cut that first migrates each rules its extension under this
> section's invariants: raw values, no expansion, no shell, and one owner per knob (delta 9).

### (2) Per-knob ownership: a migrated kit's knobs are resolved in the crate and never bridged {design-bearing}

gate-sdk/SPEC.md §lib/gate.sh, the paragraph beginning **So there is exactly one place a knob's
value is computed**. **Not yet applied**, replacing its first sentence:

> **So a knob has exactly one producer, and which one is a property of its owning kit.** A kit
> whose knobs are **static** has a defaults table in the crate, and its consumer file is a knob
> file (§The knob file). The crate resolves those knobs in process, and the bridge never carries
> them. A kit whose knobs are still **bridged** has its values computed in its shell library,
> exactly as below. No knob is ever both. So criterion 6's second producer cannot arise, however
> far a migration has progressed.

Mechanism:

- **The routing is behind the existing reader API.** `walk::knob_scalar`, `knob_array`,
  `knob_map` and `knob_prefix`, `spec.rs`'s private twin, and the five inline environment reads
  all resolve through one function. It consults the static ownership table first and the
  `GATE_SDK_KNOB_<NAME>` variable otherwise. The five inline sites are folded in, because a
  bypass reads a migrated knob as absent.
- **`--knobs <member>` omits statically owned names.** Its answer becomes *the knobs this
  member needs the bridge to carry*. `gate_command` therefore never asks a kit library for a
  knob the library no longer defines, and a member reading only static knobs emits the bare
  two-element argv. `registry::EVERY_COUPLES_KNOB`'s expansion, in the crate
  (`registry::couples_knob_names`) and in `lib/gate.sh`, drops statically owned names by the same
  rule. The shell twin learns the set from `--emit knob-roster` (delta 5), not from a second
  list.
- **A static kit's defaults table** is one module per kit under `native/src/knobs/`. Each row
  holds the name, the shape (scalar, indexed or keyed) and the default, and a derived default is
  a function of already-resolved knobs. The table is what `--emit knob-roster` prints and what
  the reader refuses an undeclared name against.
- **Resolution reads the files once per process per kit.** The cached result is cleared through
  `knobenv`'s guard in tests, on the rule §lib/gate.sh already states for process-global test
  state.

The kit-library disposition's own reopening condition is what this fires. gate-sdk/SPEC.md §The
kit-library port disposition, **What reopens it**: *the ground dissolves for an individual member
whose kit's knobs stop crossing the bridge*. So a migrated kit's library is not ported; it is
deleted, because it computes nothing.

### (3) Precedence and location of a static kit's knobs {design-bearing}

gate-sdk/SPEC.md §The knob file. **Not yet applied:**

> A static knob resolves, highest first, from:
>
> 1. **the environment, for a scalar knob only**: `NAME` exported by the invoker. An indexed or
>    keyed knob takes no environment override, because one name meaning two grammars is the
>    defect §lib/gate.sh's prefix rule exists against.
> 2. **the local overlay**, `<gates-dir>/<stem>-config.local.knobs`, gitignored by the consumer,
>    for values that must not be tracked;
> 3. **the tracked file**, `<KIT>_KNOB_FILE` when set (set but missing is exit 2, as the shell
>    loaders do today), else `<gates-dir>/<stem>-config.knobs` when it exists;
> 4. **the kit default** from the crate's table.
>
> `<stem>` is the file stem the kit's shell config used (`site`, `doctrine`), so a consumer
> finds the new file where the old one was. The gates directory is `GATE_SDK_GATES_DIR` from the
> environment, default `scripts`. It stays env-or-default for the reason §Layout and
> configuration already gives: a file cannot name its own directory.
>
> **The environment beats the file here, and this reverses the shell seam's measured
> behaviour on purpose.** Under the shell seam the file's own spelling decides, and a bare
> assignment wins, so a harness cannot make a knob authoritative by exporting it
> (§run-gate-tests records that hole). A static file has no spelling to choose with, so the
> order is chosen once, and the per-invocation value outranks the persistent one. That dissolves
> the harness-pin hole for every static scalar knob. It does not dissolve it for a bridged kit,
> and `GATE_SDK_TMP_DIR`, the attested instance, stays bridged until gate-sdk's cut.
>
> **While migration is in progress the two seams resolve in different orders, and each cut
> declares its kits' order.** A static kit puts the environment first; a bridged kit lets its
> file decide. The cut that moves a kit names that kit's order in its release declaration under
> Behavior changes. The declaration is owed only when the order actually changes: a loader that
> already let the environment win, such as doctrine-kit's, changes nothing.

The gates-directory default is one literal the crate now spells beside `lib/gate.sh` and every
kit loader. It is a locator, not a knob a file can set, so no single owner can hold it until
gate-sdk migrates. A crate unit test spawns `lib/gate.sh`, on the test-only bridge precedent in
`walk.rs`, and holds the two equal.

### (4) Legacy refusals and the renamed file knob {design-bearing}

For a static kit, the reader **refuses** with exit 2, naming the migration:

- when `<gates-dir>/<stem>-config.sh` or its `.local.sh` overlay exists. A shell config left
  behind at upgrade would otherwise be silently ignored, dropping the consumer's values. This is
  the fail-open `GATE_SDK_GRAPH_THEME`'s retirement already refused (§Layout and configuration).
- when `<KIT>_CONFIG_FILE` is set and names a **non-empty** file. An empty file is inert in
  either grammar, and `lib/test-hermetic.sh` pins every kit's `<KIT>_CONFIG_FILE` at one empty
  file. So the hermetic harness needs no edit and cannot trip the refusal, while a consumer's
  real pinned config is refused rather than dropped.

`<KIT>_CONFIG_FILE` → `<KIT>_KNOB_FILE` is a rename, because the grammar changed and a distinct
spelling is what a grammar change buys (§lib/gate.sh). For each migrated kit it is declared
under Renamed knobs (delta 10).

### (5) `--emit knob-roster` publishes the static table {design-bearing}

A new emitter arm (gate-sdk/SPEC.md §The non-gate arm) prints one line per statically owned
knob:

```
<NAME><TAB><shape><TAB><rendered default>
```

`<shape>` is `scalar`, `indexed` or `keyed`. A scalar prints one line. An indexed default prints
one line per element, in order, and a keyed default prints one line per pair, spelled
`<key>=<value>` and sorted by key. An empty indexed or keyed default prints one line with an
empty third field, the same resolved-empty reading the wire gives. A derived default renders as
its value in the invoking tree. The arm takes no knob and reads no file.

Its readers are named, and each is why it exists:

- **`check-docs-cmd` assertion B** builds its known-knob set from kit-root source by `git grep`.
  A migrated kit's names leave kit-root source with its library, and every doc mention of them
  would red. Its set becomes that grep unioned with the roster's first column (canon-kit/SPEC.md
  §check-docs-cmd).
- **`check-knob-default-coupling`** couples a default literal in kit source to the default its
  owning SPEC states. A migrated kit's defaults leave that corpus, which is a narrowing (point 5
  below). The roster becomes a third default idiom: a scalar default rendered there is coupled
  exactly as a guarded assignment is (canon-kit/SPEC.md §check-knob-default-coupling).
- **`lib/gate.sh`'s `EVERY_COUPLES_KNOB` substitution** filters by the first column (delta 2).
- **An adopter**, who can no longer read a static kit's defaults out of a shell file. The
  roster is the discoverable replacement, and it discloses no gate rule: names and defaults
  are configuration surface, not rule source (TRAJECTORY objective 3 withholds rule source
  only).

### (6) The first cut: site-kit and doctrine-kit go static {mechanical}

Two kits, chosen by the selection rule (delta 9) and by what they prove together.

- **site-kit**: 7 knobs. `SITE_KIT_RENDERER` is already an argv array whose element carries
  embedded double quotes. `SITE_KIT_RENDERER_BATCH` is the one conditional default, filled only
  when `SITE_KIT_RENDERER` resolves from the kit default. Both site members also read the
  bridged `GATE_PRUNE_DIRS`, so this is the first **mixed** member, and it exercises the routing
  and the narrowed `--knobs` at once. This repo's `scripts/site-config.sh` is a real consumer
  file with an indexed knob.
- **doctrine-kit**: 3 scalar knobs and a `.local` overlay, which queue-kit's config also uses.
  It proves the overlay and the environment-over-file order: doctrine's loader uses `:=`, so the
  environment already wins for it today and its behaviour is unchanged.

Build, in order:

- `native/src/knobfile.rs` (the grammar), `native/src/knobs/{mod,site_kit,doctrine_kit}.rs`
  (the tables and routing), and the reader fold-in (delta 2), with unit tests for every refusal
  in deltas 1 and 4 and for the precedence order.
- `--emit knob-roster`, and `--knobs` narrowed.
- Delete `site-kit/lib/site.sh` and `doctrine-kit/lib/doctrine.sh`. `git mv
  scripts/site-config.sh scripts/site-config.knobs`, rewritten to the grammar and keeping its
  provenance comment as `#` lines. The fixture configs under
  `site-kit/gate-tests/check-docs-cname-parity/{good,bad}/scripts/` get the same move.
- `site-kit/gate-tests/check-docs-render-fidelity-batch.test.sh` sources `lib/site.sh` to test
  the fill rule. The fill rule becomes a unit test on `knobs::site_kit`, and the suite's
  per-case configs become knob files.
- `site-kit/smoke/violation.sh` writes a shell config and moves to a knob file.
  `native/src/emit/agents_md_smoke.rs` names `scripts/doctrine-config.sh` and follows.
- This repo's `CANON_KIT_COMMENT_SURFACE` gains depth-enumerated `*.knobs` globs, so a knob
  file's comments stay tier-governed, but only if `check-comment-tier` reads `#` comments on
  that extension. If it does not, file the gap rather than widen blind.
- Regenerate the pre-commit hook, whose baked argv for the site members loses the `SITE_KIT_`
  assignments, and the kit SPEC mirrors.

### (7) The kits' own SPEC sections {design-bearing}

site-kit/SPEC.md §lib/site.sh and doctrine-kit/SPEC.md §lib/doctrine.sh become §Knob defaults in
each kit, each stating its knobs' shapes and defaults with the conditional-default rule and its
ground moved intact, and pointing at gate-sdk/SPEC.md §The knob file for grammar and precedence.
Each kit's §Layout and configuration names `<stem>-config.knobs` and `<KIT>_KNOB_FILE`. site-kit's
README step 3 ("copy a `site-config.sh`") follows.

### (8) The seam's standing rules restated where they are stated {design-bearing}

- gate-sdk/SPEC.md §Layout and configuration, the loader paragraph: it describes the shell seam,
  and it gains one sentence naming the static seam for a static kit, plus a pointer to §The
  knob file.
- gate-sdk/SPEC.md §The config-seam port disposition: its class is *the files you edit*, and
  that ground holds unchanged. A static kit's knob file is not shell, so §port-blockers never
  scans it and it leaves the class by substrate, not by port. The class shrinks at each cut, and
  **What reopens it** is untouched.
- gate-sdk/SPEC.md §The kit-library port disposition: its membership is derived, and a deleted
  library is simply not a member. One sentence records that the reopening condition has fired
  for the static kits, and that deletion rather than port is the disposition.
- gate-sdk/SPEC.md §The port-candidate criteria, criterion 6, and §Meta-gate conservation for the
  binary substrate's `check-knob-default-coupling` row: both cite the single-producer rule and
  take its reworded form (delta 2).
- installer/README.md §What init seeds calls the config seam "permanently shell". That becomes
  "the edit seam, whatever its substrate". Init's derivation (`templates/*-config.sh`) is
  untouched this cut, because neither static kit ships a config template. The first cut whose
  kit does ship one widens the derivation to `templates/*-config.knobs` (delta 9).

### (9) The rule that selects the next cut {design-bearing}

gate-sdk/SPEC.md §The knob file ends with the selection rule, not a dated plan. **Not yet
applied:**

> **A kit migrates when every shape its knobs and its consumers' configs use is expressible,
> and the cut that first needs a shape rules its grammar under this section's invariants.** The
> shapes not yet ruled, each with the reason it cannot be deferred past its kit:
>
> - **A command knob** (`*_CMD`) crosses as an **indexed** knob holding an argv, which the
>   binary spawns directly, never a shell string, so a consumer on native Windows names its own
>   interpreter. The five canon-kit command knobs whose output feeds `GATE_SDK_RESOLVING_KNOB`
>   are resolved by the crate spawning the argv when a member first reads a dependent knob, and
>   that retires the resolving-knob gating.
> - **A reference to another knob**, same-kit or cross-kit, is the one splice form the file
>   admits, ruled at the first cut whose consumer config needs it.
> - **A generated family** (`EVIDENCE_KIT_RUN_` over the fixture suites) is a kit-side
>   derivation the owning kit's table performs, never a loop in consumer config.
> - **guard-kit's rule content** is bash a consumer composes against `lib/guard.sh`. Its cut
>   rules whether the rules become data or the guard stays the one shell hook.
> - **gate-sdk migrates last**, with `lib/gate.sh`, the three bridge front-ends and the
>   `GATE_SDK_KNOB_` wire. That cut retires the bridge, so it is the one that discharges the
>   bridge's residual cost and the harness-pin hole for bridged knobs, and the one that moves
>   docs/install.md's bash floor for a native Windows runtime.
>
> `lib/test-hermetic.sh` and the 96 suites sourcing it are bash test harnesses rather than seam
> surface. They follow a migrated kit only where a suite writes that kit's config, as delta 6's
> render-fidelity suite does.

### (10) Release declarations {mechanical}

`.workflow/release-declarations.md`. **Not yet applied:**

- Renamed knobs: `SITE_KIT_CONFIG_FILE` → `SITE_KIT_KNOB_FILE`, and `DOCTRINE_KIT_CONFIG_FILE` →
  `DOCTRINE_KIT_KNOB_FILE`. The file they name changes grammar, and a set old name naming a
  non-empty file is refused.
- Behavior changes: **`<gates-dir>/site-config.sh`, `doctrine-config.sh`** and their `.local.sh`
  overlays are replaced by `.knobs` files in the line grammar (gate-sdk/SPEC.md §The knob file).
  A left-behind shell config is refused at exit 2. For site-kit, an exported scalar knob now
  outranks the file. Rewrite each `NAME=(a b)` as `NAME[] = a` lines, and each `NAME=v` as
  `NAME = v`.

## Producers and consumers

- **The knob file** (new state). *Producer:* the consumer (here, `scripts/site-config.knobs`
  and the fixture configs). The init seeding path is not exercised this cut, since no static kit
  ships a template. *Consumer:* the crate's reader, in process, at the first read of a knob its
  kit owns.
- **The static ownership table** (new interface). *Producer:* the kit module under
  `native/src/knobs/`. *Consumers:* the reader routing, `--knobs` narrowing, `--emit
  knob-roster`, and the refusal of undeclared names.
- **Each row's fields.** Name is read by all four consumers. Shape is read by the grammar
  refusal and the roster. Default is read by the reader at precedence step 4 and by the roster.
- **The `--emit knob-roster` output** (new interface), with its three consumers named at delta 5.
  Each consumer reads column one; `check-knob-default-coupling` also reads columns two and three.
- **The narrowed `--knobs` answer** (changed interface). *Consumers:* `gate_command`,
  `gen-pre-commit`'s baked argv, and `runner.rs`'s `child_knobs`, which filters the battery's
  union per member. A member whose knobs went static is handed nothing for them, and it resolves
  them itself.
- **The legacy refusals and `<KIT>_KNOB_FILE`**. *Producer:* the reader. *Consumer:* the
  operator through exit 2 and its message.
- **Red conditions (point 5), because deltas 2 and 6 narrow three corpora.**
  - *Bridge-carried knob set:* `check-graph` assertion D compares a regenerated hook
    byte-for-byte, and it reds on staleness, so the hook regeneration is in delta 6.
    `check-reads-couples` and `check-gate-substrate-parity` read `--knobs` and `couples=` in
    agreement; neither holds a count or a floor, but the sentinel expansion must drop static
    names in both twins at once or the shell twin refuses. Build runs both.
  - *`check-docs-cmd` assertion B's known-knob set:* it reds on a doc knob absent from the set,
    which is *not* monotone under narrowing: removing the libraries adds violations. The roster
    union (delta 5) is the repair, and it lands in the same commit as the deletion.
  - *`check-knob-default-coupling`'s source corpus:* it reds on disagreement and on a SPEC
    lacking a stated default. Both are monotone in the violation set, so the narrowing cannot
    add a red, but it silently drops coverage of the migrated defaults. The roster idiom
    (delta 5) restores coverage.
  - *§port-blockers' `--tree` owed and no-port counts:* the two libraries and
    `scripts/site-config.sh` leave the scanned set. Any gate or measured claim asserting an
    exact no-port count reds, and `check-measured-claim` re-runs its oracles. Build runs the
    battery rather than inspecting.
  - *`check-template-copy-parity`:* excludes `*-config.sh` by suffix. No template moves this
    cut, so it is unaffected.

## Existing sections updated

- `gate-sdk/SPEC.md` — §Layout and configuration: new §The knob file (deltas 1, 3 and 9) and the
  loader paragraph (delta 8, one sentence naming the static seam); §lib/gate.sh's single-producer
  paragraph and sentinel expansion (delta 2); §The config-seam port disposition, including its
  hand-authored `site-config.sh` mention, §The kit-library port disposition, §The port-candidate
  criteria and §Meta-gate conservation for the binary substrate (delta 8); §The non-gate arm's
  `knob-roster` arm (delta 5).
- `canon-kit/SPEC.md` — §check-docs-cmd assertion B and §check-knob-default-coupling (delta 5).
- `site-kit/SPEC.md` — §lib/site.sh becomes §Knob defaults; §Layout and configuration names the
  knob file and `SITE_KIT_KNOB_FILE` (delta 7).
- `site-kit/README.md` — step 3's config copy (delta 7).
- `doctrine-kit/SPEC.md` — §lib/doctrine.sh becomes §Knob defaults; §Layout and configuration
  names the knob file and `DOCTRINE_KIT_KNOB_FILE` (delta 7).
- `installer/README.md` — §What init seeds (delta 8).
- `native/src/knobfile.rs` — new, the grammar (deltas 1 and 6).
- `native/src/knobs/` — new, the per-kit tables and routing (deltas 2, 3 and 4).
- `native/src/walk.rs` — reader routing (delta 2).
- `native/src/spec.rs` — its private reader twin folded in (delta 2).
- `native/src/registry.rs` — `--knobs` narrowing and the sentinel expansion (delta 2).
- `native/src/emit/agents_md_smoke.rs` — its `doctrine-config.sh` literal (delta 6).
- `gate-sdk/lib/gate.sh` — the sentinel filter (delta 2).
- `site-kit/lib/site.sh` — deleted (delta 6).
- `doctrine-kit/lib/doctrine.sh` — deleted (delta 6).
- `scripts/site-config.sh` — moved to `scripts/site-config.knobs` (delta 6).
- `site-kit/gate-tests/` — the cname-parity fixture configs and the render-fidelity batch suite
  (delta 6).
- `site-kit/smoke/violation.sh` — its written config (delta 6).
- `scripts/git-hooks/pre-commit` — regenerated (delta 6).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/site-kit/SPEC.md` — generated mirror, regenerated (delta 7).
- `docs/site-kit/README.md` — generated mirror, regenerated (delta 7).
- `docs/doctrine-kit/SPEC.md` — generated mirror, regenerated (delta 7).
- `scripts/canon-config.sh` — `CANON_KIT_COMMENT_SURFACE` (delta 6).
- `.workflow/release-declarations.md` — Renamed knobs and Behavior changes (delta 10).

## Retired spellings

- `site-kit/lib/site.sh` — deleted (delta 6).
- `doctrine-kit/lib/doctrine.sh` — deleted (delta 6).
- `site-config.sh` — replaced by `site-config.knobs` (delta 6).
- `doctrine-config.sh` — replaced by `doctrine-config.knobs` (delta 6).
- `SITE_KIT_CONFIG_FILE` — renamed `SITE_KIT_KNOB_FILE` (delta 4).
- `DOCTRINE_KIT_CONFIG_FILE` — renamed `DOCTRINE_KIT_KNOB_FILE` (delta 4).

## Definition of Done

- [ ] **Causal completeness** — every new state and interface above has a reachable producer
      and a named consumer.
- [ ] **Instruction surfaces: instruction only** — refusal messages carry the remedy, not the
      grounds.
- [ ] **Merged with no information lost** — the format ruling's refused candidates relocate into
      §The knob file.
- [ ] **Amendment deleted**, and the entry is **demoted, not moved to Done** (corpus increment).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — the comment-tier reach question, if it lands as a gap.
