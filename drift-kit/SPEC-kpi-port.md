# SPEC amendment: the drift-kit KPI cut onto the binary substrate

Rules this increment of `native-gate-port-remaining-corpus`. The cut was ruled by
the lead on 2026-08-29 (`lead 2026-08-29 own-authority`) under the standing
composer rule — **select the next cut by stated contract** — as the drift-kit
files sitting behind drift-kit/SPEC.md §The KPI plugin contract.

The entry demotes rather than completing; the port sequence, the priority
directive and every closed cohort stay where they are and are not restated.

**The corpus, re-derived at spec rather than inherited.**
`--emit port-blockers --tree` reds **141** owed of 153 scanned, 12 declared, 0
held; `--group` reds **0 owed, 0 takeable**, so the battery is finished and the
whole remainder is non-gate shell. drift-kit contributes 21 owed files; the ruled
cut is those less the three `bin/` tools that are not the KPI driver.

**The 18 resolve to 14 ported and 4 declared, ruled 2026-08-29.** Three carry
`# no-port:` and one carries `# port-until:` — deltas 7 and 8 below.

## What this cut is not, and it is the ruling that shapes everything else

**The consumer's shell-plugin extension point survives the port.** Ruled (a) on
2026-08-29. The ported driver keeps plugin resolution, **direct execution** and
the exported-env contract for consumer-authored `kpi-<name>.sh`; only the 13
bundled plugins move in-crate.

The ground is drift-kit's **own** governed surfaces, not analogy to gate-sdk's
parallel ruling: the README ships *"add your own by dropping a plugin in your
gates dir"* as an adopter feature, and `DRIFT_KIT_KPI_DIRS` exists solely for
consumer-first shadowing — a knob whose only purpose is an extension point **is**
that extension point's specification, and narrowing it is removing a documented
feature rather than porting one. Underneath that, CLAUDE.md §The provenance seam
decides it a second time: a consumer-first plugin registry is the
`check-graph`/`graph-vocab.sh` pattern in another dress, so narrowing the
extension point narrows the seam's own mechanism.

**If this cut proves too large, the port gives and the extension point does
not.** The narrowing shape is ruled in advance — driver plus native dispatch, with
the 13 plugin bodies split out — and delta 6 is deliberately authored as **one
separable delta** so that narrowing is a **batch-cut decision rather than a
re-spec**. Its cost line and the interaction that follows from deferring it are
stated there.

## What changes

### (1) The extensibility contract, stated in drift-kit's own SPEC

A new section making the plugin extension point drift-kit's own contract:
`DRIFT_KIT_KPI_DIRS`, consumer-first shadowing, direct execution, and the
exported-env read rule, each named as a promise to adopters rather than as an
implementation detail of the shell driver. {design-bearing}

**This delta exists because its absence was the defect.** The question "does the
extension point survive?" had to be escalated precisely because drift-kit/SPEC.md
has no extensibility section — the answer was inferable from a README sentence and
a knob's purpose, and inference is what the next session would have had to repeat.
Spec-over-precedent: after this the owner doc answers, and nobody infers again.

The section states **three tiers of resolution, consumer-first**: the consumer's
own `DRIFT_KIT_KPI_DIRS` entries, then sibling kit `kpis/` directories, then the
crate's built-in members — so a consumer file named `kpi-task-split.sh` shadows
the built-in of that name, which is the behavior the current shell resolver
already has and which the port must not silently drop.

### (2) `--emit-drift-report`, the driver as a bridged arm

The collator becomes a bridged arm of the binary, registered by a `pub mod` line
in `native/src/emit/mod.rs` and one `BRIDGED_ARMS` tuple. {design-bearing}

It must be a **bridged** arm and not a top-level flag: the front-end composes
`--emit-<name>` from its `--emit <name>` operand and resolves each member's
declared knob roster, and a hardcoded top-level flag receives no consumer
override at all — which for a kit whose entire surface is consumer-overridable
knobs would be a silent functional regression rather than a porting detail.

The arm keeps both modes (bare, and `--trend`) and keeps the degrade discipline
whole: exit is always 0, a failed plugin yields its visible `n/a (plugin failed)`
row, and a plugin whose *surface* is missing degrades in its own value.

### (3) The declared-knob problem, and the sentinel that closes it

The arm's declared reads use the **union sentinel** rather than a transcribed
list of `DRIFT_KIT_*` knob names. {design-bearing}

**This is the port's hardest design point and it is a doctrine problem, not a
plumbing one.** The plugin env contract is built on `compgen -v DRIFT_KIT_`
specifically so that *"a config override reaches writer and reader alike with no
fixed export list to drift out of parity"* — the derivation **is** the contract.
A bridged arm receives only its **declared** knobs, so the naive port replaces a
derivation with a maintained roster and lands squarely on the derivation-first
rule: *derive the derivable, never maintain it*.

The close already exists in the tree and was minted for this exact shape — the
registry-scoped union sentinel that declares "every registered knob" instead of
enumerating them. The arm declares the sentinel; the roster stays derived.

### (4) Plugin resolution, direct execution and the env export, in the crate

The arm implements delta 1's three-tier resolution, executes a resolved consumer
plugin **directly** (not via `bash <path>`, so the execute bit still governs and
the fail-visible row still fires), and exports every scalar `DRIFT_KIT_*` plus
the two driver handoffs — `DRIFT_KIT_KIT_ROOTS` and `DRIFT_KIT_ITERATION_START`
— into the child environment. {design-bearing}

The two handoffs are recomputed every run and are **not** consumer knobs; they
must reach a consumer plugin exactly as they reach a built-in, or a consumer
plugin becomes a second-class reader of the contract the extension point
promises.

### (5) `check-template-registry-parity`'s population predicate widened

Native dispatch becomes a recognized shipping mechanism, so a registry line
naming a natively-dispatched member resolves rather than reds. {design-bearing}

Ruled (b) on 2026-08-29. Shell shims were refused: thirteen files whose only
purpose is to satisfy a gate would then need their own port dispositions,
converting a gate's blind spot into permanent corpus. Scoping the gate off
drift-kit was refused as scoping enforcement off the thing it exists to check.
The honest reading is that the gate **predates the substrate** — and every later
kit port meets it — so enforcement-first puts its fix in the unit that breaks it.

**The widened predicate must not be satisfiable by the population going empty,
and this is the delta's load-bearing constraint.** Today the gate reads
`if !Path::new(&dir).is_dir() { skipped += 1; continue; }` — so *deleting* the
`kpis/` directory sends the template into the **skipped** arm and all thirteen
registered names go **silently ungated**. A fail-open is strictly worse than the
red it replaces, because a later session reads a green board with no way to see
it. Whatever the widened predicate does, an empty or absent sibling directory
beside a **non-empty** registry template must remain a finding.

Full `good/`+`bad/` fixture pair, per the four gate contracts, which do not bend
for a widening.

### (6) The thirteen bundled plugin bodies, in-crate

The thirteen `kpi-*.sh` bodies become built-in members dispatched by name behind
delta 4's resolver, and their shell originals are deleted. {design-bearing}

**Authored as one delta on purpose: this is the narrowing seam.** Deltas 1–5 and
7–10 are the driver, the contract, the extension point and the dispositions;
this delta alone is the 558 lines of plugin body. If the lead narrows the cut,
this delta defers whole and nothing above it is re-specified.

Two consequences of deferring it, stated here so the decision is made with them
rather than discovered after:

> **THE CUT RULE, ruled by the lead 2026-08-29 and binding rather than advisory:
> if delta 6 is cut, delta 5 is cut with it, and lines 25-26 of the settings
> reconciliation below are cut with both.** They are one unit at batch-cut and may
> not be separated.

The ground is enforcement-first, inverted: deferring delta 6 leaves the thirteen
shell files **shipped**, so nothing in the increment breaks registry parity — and
a gate widened ahead of the breakage it anticipates is speculative enforcement,
landing unexercised by the very thing it exists for. The doctrine is that the fix
and the gate that catches it land in **one** unit; delta 5 travels with delta 6
because delta 6 is what makes delta 5 true.

The rest of what deferring costs, so the call is made with it rather than after:

- `--tree`'s owed count falls by one rather than by fourteen, and the entry
  demotes having ported the driver alone.
- Of the four settings grants this cut orphans, only the `drift-report.sh` pair
  dies; the `kpis/*.sh` pair survives with the files. See the reconciliation
  below, which is itself split along this seam.

Carried hazards for whoever implements it: `kpi-prompt-friction` and
`kpi-always-loaded` **shell out to sibling kits** and one of them *prose-parses*
the other tool's output, which is an undeclared cross-kit output contract;
`kpi-amendment-age` uses `git log --follow`; `kpi-task-split` resolves a slug
through `git log -1 --grep`; and `kpi-price-table-age` restates a default that
`stage-economics.sh` owns, which the port converts from an in-substrate duplicate
into a **cross-substrate** one.

### (7) Three `# no-port:` declarations, each stating its ground

`drift-kit/lib/drift.sh`, `drift-kit/templates/drift-config.sh` and
`drift-kit/templates/kpi-deprecated-surface.sh` declare, and **each declaration
states its ground explicitly rather than citing a precedent file**.
{design-bearing}

The distinction is the ruling's and it changes what gets written: the **ground**
here has precedent — `gen-pre-commit.sh` declares on exactly it — but the
**class** of kit `lib/*.sh` files has never been swept. Writing the ground rather
than gesturing at a sibling is what lets that cohort inherit a stated reason
instead of a precedent-by-example.

- `lib/drift.sh` — the config bridge's **sole resolver** for `DRIFT_KIT_*` knobs;
  resolving a knob means sourcing the owning kit's lib, and three
  **already-compiled** arms source this one. A crate-side resolver would be the
  second producer criterion 6 refuses. Structural, not a sizing judgment.
- `templates/drift-config.sh` — two lines, both comments, seeded into the
  adopter's own gates dir. It is the adopter's config seam; porting it deletes
  the seam.
- `templates/kpi-deprecated-surface.sh` — the provenance seam's **worked
  example**: it ships as a template rather than as a bundled plugin precisely
  because its marker spelling is a consumer literal, and it reads consumer array
  knobs by sourcing a consumer config. Porting it publishes consumer content as
  kit mechanism.

### (8) `smoke/install.sh` held, and the cohorts filed

`drift-kit/smoke/install.sh` takes `# port-until:` against a new cohort entry, and
the two undeclared cohorts are filed as deferred entries. {mechanical}

The hold is not a close call: roughly 400 of its 559 lines assert on `kfric.sh`,
`overhead-meter.sh` and `stage-economics.sh` — the three tools this cut's envelope
**excludes** — and it is their only harness. Porting the harness for three
excluded tools is incoherent with the exclusion; shipping it would be the
envelope leaking.

**Two entries, not one.** The grounds differ in kind and averaging them would
produce a filing that answers neither: a kit `lib/*.sh` is **load-bearing at
runtime** for the config bridge, while a `smoke/install.sh` is a **test surface**
whose port question is about harness bootstrap. Each entry carries its own ground
and its own cohort census.

### (9) The shell originals deleted, and the parity oracle

Every ported file is **deleted**, never left beside its port. {mechanical}

Corpus membership is `git ls-files`, so a surviving file must carry a marker —
and a marker **declares that it is not porting**. Only never-porting files
declare, so leaving a ported original in place would either red the tree or
falsely declare it permanent.

Parity is asserted by **byte-identity of the emitted report** across the two
substrates, captured before the delete and diffed after — the shipped oracle for
this class, and the reason no fixture pair is owed for a plain script port.

**The delete carries its settings grants with it, in the same commit.** Part of
the operator's 2026-08-29 ruling rather than a convenience: a commit that deletes
a granted target and leaves its grant standing opens a window in which the
committed settings name a file that is gone, and it is that window the
same-commit requirement exists to close — not the tidiness of the removal. So the
commit deleting `drift-kit/bin/drift-report.sh` removes settings lines 19-20, and
the commit deleting the thirteen plugins (delta 6) removes lines 25-26. Lines
21-24 are untouched in both. The scope of the removal is the files **this commit
deletes**, never the `drift-kit/` prefix.

### (10) The fan-out this cut stales

Each with its trigger, since a port stales more than it changes. {mechanical}

`check-gate-binary-fresh` — discharged **only** by
`bash gate-sdk/bin/build-native.sh`, never by the battery and never by
`cargo test`. `check-crate-arms` — the crate's lint and test arms.
`check-docs-mirror-fresh` — drift-kit's and gate-sdk's mirrors.
`scripts/measured-claims.sh` — reads `--tree`'s owed trailer, which moves.
`check-exec-bit` goes **vacuous** over the kit's `kpis/*.sh` while its
`scripts/kpi-*.sh` arm **must be retained**, because that arm is what still
governs the consumer extension point. `check-footprint-fresh` does **not** fire.

**`check-settings-paths`, RULED — and the grant count is reconciled here because
a wrong number is how a live grant gets deleted beside the dead ones.**

**The ruling (`operator`, 2026-08-29, relayed by the lead).** Removing a
permission grant whose target file a **ruled port cut deletes** is **outside** the
2026-08-22 bar: it is a pure narrowing forced by the cut, removing a capability
rather than adding one, and the cut's authority to delete the file is already
ruled. **The bar itself is untouched** and stands unchanged for every other
permission-settings edit — this is a carve-out for one mechanically-decidable
class, not a relaxation. **Build removes the dead lines in the same commit as the
delete**, so a grant and its target die together and no window exists in which the
settings name a file that is gone. That same-commit requirement is part of the
ruling, which is why it is stated in delta 9 — the delta that owns the delete —
rather than left here as prose.

**The reconciliation, probed at spec rather than carried.**
`grep -n "drift-kit" .claude/settings.json` returns **eight** grants, lines 19-26,
not the two an earlier reading of this amendment asserted. They split three ways
and only the first four are this cut's:

| lines | grant | disposition |
|---|---|---|
| 19-20 | `drift-kit/bin/drift-report.sh`, bare and `*` | **dies with delta 9's delete of the driver's shell entry point** |
| 25-26 | `drift-kit/kpis/*.sh`, bare and `*` | **dies with delta 6's delete of the thirteen plugins** |
| 21-24 | `kfric.sh`, `overhead-meter.sh`, `stage-economics.sh` (four grants) | **SURVIVE UNTOUCHED** |

Lines 21-24 point at the three `bin/` tools this cut's envelope **excludes**. They
are live grants for live files and removing them would be a capability loss the
ruling does not cover and the cut does not force. **Four die, four survive**, and
the four that die are split by the cut rule above: if delta 6 is cut, **only lines
19-20 go**.

## Producers and consumers

**The new interfaces are one bridged arm, one resolution contract and one widened
gate predicate.**

- **Producer, named and reachable:** the `--emit-drift-report` arm, registered in
  `BRIDGED_ARMS` (delta 2) and reached two ways that both exist today —
  `run-gates.sh --emit drift-report`, and the session-start hook via
  `CONTEXT_KIT_DRIFT_REPORT`. Its enabling configuration is that knob, already
  emitted in context-kit's shipped template, so the producer is reachable in the
  deployed configuration and not only in tests.
- **Consumer 1 — the session-context hook, at session start.** Mechanism:
  `scripts/session-context.sh` and `context-kit/templates/session-context.sh`
  each test `[[ -f "$DRIFT_REPORT" ]]` and then run it, with `|| true`
  swallowing failure. **That `-f` test is a path existence check, and the ported
  arm is not a path** — so this is a **silent** break of the session-start trend
  line, and context-kit/SPEC.md compounds it by calling the target a "script".
  Both call sites and that prose move with the port; without them the port
  succeeds and the feature disappears with no red anywhere.
- **Consumer 2 — a consumer-authored `kpi-*.sh`, at plugin execution** (deltas 1
  and 4). Mechanism: direct execution with the exported `DRIFT_KIT_*` environment.
  This is the consumer the whole Q3 ruling exists to protect, and it is the one
  a port is most likely to drop silently.
- **Consumer 3 — `check-template-registry-parity`, at commit time** (delta 5),
  reading the registry template against the sibling directory's population.
- **Consumer 4 — `native/src/emit/enforcement_map.rs`**, which **hardcodes**
  `format!("{}/kpis/{}.sh", …)` and a SPEC anchor for the KPI contract. It is a
  second, already-compiled implementation of plugin resolution, and it reds or
  silently mis-maps if delta 4's resolver and it disagree.

**Every field has a named reader.** The arm adds no message and no field; the two
driver handoffs it exports are existing fields whose readers are the plugins, and
delta 4 exists precisely to keep both readers — built-in and consumer — reading
them at the same transition.

**Red conditions, named rather than subjects.** Deltas 6 and 9 **narrow** a corpus
(thirteen files deleted), so point 5 binds and each reader is enumerated by what
makes it red — not by what it is about:

- **`check-template-registry-parity`** — reds when a registry line names no
  shipped artifact (assertion B) **or** when a shipped artifact is unregistered
  (A). **Non-monotone, and it is the attested shape**: emptying `kpis/` reds all
  thirteen lines, while **deleting** the directory flips the template into the
  `skipped` arm and reds **nothing** — a narrowing that *removes* a violation by
  removing the check. Not clearable by inspection; delta 5 exists for it.
- **`check-exec-bit`** — reds on a non-executable member of its scan set. Its red
  condition is per-file, so it goes **vacuous** rather than red over an emptied
  `kpis/`; a vacuous pass is not a pass, which is why its `scripts/` arm is
  retained explicitly rather than left to survive by accident.
- **`check-settings-paths`** — reds on a settings path naming no file. **Zero-count
  shaped and non-monotone**: the delete *creates* the violation, which is the
  attested shape that makes a narrowing unsafe to clear by inspection. **Ruled and
  fixed in-cut** — the same commit that deletes removes exactly the orphaned lines
  (19-20, and 25-26 only if delta 6 lands), per the reconciliation above.
- **`check-gate-binary-fresh`** — reds when the committed binary predates the
  crate sources. Monotone; cleared only by `build-native.sh`.
- **`check-crate-arms`, `check-docs-mirror-fresh`, `check-comment-tier`,
  `check-knob-citation`** — monotone in their violation sets; clear by inspection
  once the sources and mirrors move together.
- **The consumer smoke** (`drift-kit/smoke/install.sh`) — reds when an asserted
  behavior is absent. It is **held, not ported**, so it keeps asserting the shell
  contract and is the strongest available oracle that delta 4 did not drop the
  extension point.

## Existing sections updated

- drift-kit/SPEC.md §The KPI plugin contract — resolution, invocation and the
  exported-env rule re-stated for two substrates rather than one, with the
  direct-execution and execute-bit clauses kept as consumer-facing promises
  (deltas 1, 2, 4).
- drift-kit/SPEC.md — the **new extensibility section** (delta 1).
- drift-kit/SPEC.md §Bundled KPIs — the thirteen become built-in members; their
  measurement semantics are unchanged and are not restated (delta 6).
- drift-kit/SPEC.md §Layout and configuration — `DRIFT_KIT_KPI_DIRS` gains its
  role in the three-tier resolution; the declared-knob sentinel is named
  (deltas 1, 3).
- gate-sdk/SPEC.md §check-template-registry-parity — the population predicate and
  the empty-population constraint (delta 5).
- gate-sdk/SPEC.md §The non-gate arm — the arm roster gains a member (delta 2).
- gate-sdk/SPEC.md §port-blockers — the declared and held dispositions (deltas 7, 8).
- context-kit/SPEC.md and both `session-context.sh` copies — the drift-report
  target is an arm, not a script path (delta 2).
- `native/src/emit/enforcement_map.rs` — its hardcoded plugin path and SPEC anchor
  (delta 4).
- TASK-QUEUE.md — the entry's terminal move is a **demotion**, not a Done move, and
  the entry lands back inside `check-queue-entry-budget`'s cap at **0 lines of
  headroom**, so it is compressed in the same commit that demotes it (all deltas).
- `docs/drift-kit/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/context-kit/SPEC.md` —
  generated mirrors, stale the moment any delta lands (`all deltas`).

- `.claude/settings.json` — the orphaned grants at lines 19-20, and 25-26 only if
  delta 6 lands, removed in the same commit as the delete they follow from
  (deltas 6, 9). Lines 21-24 are **not** touched: they grant live files this cut
  excludes.

## Definition of Done

- [ ] **Causal completeness** — the arm has a named, reachable producer and four
      named consumers at four transitions, including the session-start hook whose
      breakage would otherwise be silent.
- [ ] **The extension point demonstrably survives** — a consumer-authored
      `kpi-*.sh` in a consumer gates dir resolves, executes directly, shadows a
      built-in of the same name, and reads both driver handoffs. Held by the smoke,
      which is deliberately not ported.
- [ ] **The derivation stays derived** — the arm declares the union sentinel; no
      transcribed `DRIFT_KIT_*` roster is introduced anywhere.
- [ ] **Byte-identical report across substrates** — captured before the delete,
      diffed after.
- [ ] **Merged with no information lost** — each addition in its proper section;
      the extensibility contract lands as drift-kit's own, not as a citation to
      gate-sdk's parallel ruling.
- [ ] **Amendment deleted** — this file removed on merge; none remain for drift-kit.
- [ ] **Terminal move is a DEMOTION with compression**, not a Done move: the entry
      is a corpus and this is one increment, and it returns to the deferred pool at
      0 headroom carrying more ruling content than it left with.
- [ ] **Removals propagated** — every ported original deleted, and every surface
      naming a deleted path swept (both `session-context.sh` copies, the compiled
      enforcement map, and exactly the orphaned settings grants — four of the
      eight, never the four that grant the excluded tools).
- [ ] **Gaps filed** — the two cohort entries filed; further cross-component gaps
      routed to the gap inbox.
