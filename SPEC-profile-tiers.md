# SPEC amendment: profile-tiers

The companion puts the `prose` profile over a spec toolkit's tree, and the Spec Kit recipe drops `check-fence-command-head` (companion/SPEC.md §The Spec Kit recipe). An adopter who arrives from a toolkit's catalog meets four of the battery's gates and has no documented way to meet the rest. This amendment gives each toolkit two documented install lines. `prose` stays the default, and `full` is the second, with its costs named. The Spec Kit recipe fits the gate it dropped by configuration instead, so no recipe drops a gate.

One queue entry pairs it: [companion-gate-widening](TASK-QUEUE.md#companion-gate-widening), the profile half of that direction. The spec-to-code half is [companion-spec-to-code-gates](TASK-QUEUE.md#companion-spec-to-code-gates).

**The direction.** The entry asked for "the widest profile whose gates apply to a spec-toolkit tree". Given three shapes, the operator chose a tiered one, relayed by the lead (a direction, not a /consult ruling). `prose` stays the default line on each toolkit page and in the Spec Kit extension. `full` is a second documented line on each page, with its costs named. The companion arm tests both, with a new Spec Kit `full` leg. The Spec Kit recipe re-arms `check-fence-command-head` by configuration, so no recipe drops a gate.

**The rulings.**

- **Two tiers, because `full` has two costs a catalog visitor has not agreed to.** `full` carries context-kit, drift-kit and guard-kit, each owing `bash` 4.3 or later (docs/install.md §Requirements), so `init` refuses on stock macOS until the Homebrew remedy runs. `full` also seeds Checkwright's own workflow surfaces beside the toolkit's: a task queue, a doctrine block in the agent file, and evidence files under `.workflow/`. `prose` owes neither. Both lines are tested, and moving from `prose` to `full` only adds (installer/SPEC.md §Profiles), so an adopter can start on the default and re-run `init` with the second line.
- **Every gate `full` registers runs green on each fixture.** A local pack at 0.28.0 installed each profile on each fixture tree with its recipe applied. `prose`, `delegation` and `full` were each green on both. Spec Kit's `full` passed 51 gates with the fence gate re-armed, and OpenSpec's passed 51. Under `full`, each of the four planted defects reds the gate that owns it, on both toolkits.
- **The re-arm is one vocabulary line.** On a Spec Kit tree with the gate kept, `check-fence-command-head` reds only the tasks file's parallel example, a `bash` fence of `Task:` lines. `CANON_KIT_FENCE_PROGRAMS_EXTRA[] = Task:` admits that head, and the battery goes green. The colon is part of the word, so `Task` alone does not match. The alternative the gate offers, another info string, is Spec Kit's template text, which no recipe edits.
- **OpenSpec's `full` line carries the lifecycle layer.** No gate `init` registers reads the layer's knobs, since no lifecycle-kit gate is `zero-config` (installer/SPEC.md §What init seeds). So adding it costs a `full` adopter nothing, and it saves a third line. Its reader arrives when a stage session runs. Spec Kit has no layer (companion/SPEC.md §The lifecycle layer).
- **The toolkit keeps its own checks.** No gate here re-checks what OpenSpec's validator or Spec Kit's templates own, under either tier.
- **The seam.** Repo-root only: companion recipes, companion and installer design records, docs and the smoke. The re-arm is consumer config in a knob canon-kit already ships. No kit surface changes.

**Sequencing.** This amendment's lines are `init --recipe` lines, so it lands with or after [companion-recipe-in-payload](TASK-QUEUE.md#companion-recipe-in-payload). Its Spec Kit `full` line sits on the Spec Kit page. That page is `docs/speckit.md` where [install-toolkit-page-structure](TASK-QUEUE.md#install-toolkit-page-structure) has landed, in an earlier batch or this one, and `docs/spec-toolkits.md` §Spec Kit otherwise. Where a delta below names "the Spec Kit page" or "the OpenSpec page", it means the one that exists at landing.

## What changes

### (1) The Spec Kit recipe re-arms the fence gate {mechanical}

**Not yet applied.** `companion/speckit/recipe/unregister.list` keeps its header line and loses `check-fence-command-head`. `companion/speckit/recipe/canon-config.knobs` gains `CANON_KIT_FENCE_PROGRAMS_EXTRA[] = Task:`.

companion/SPEC.md §The Spec Kit recipe, the bullet opening *`check-fence-command-head`, in `unregister.list`* is replaced by:

> - `CANON_KIT_FENCE_PROGRAMS_EXTRA[] = Task:`, in `canon-config.knobs`. The tasks template's parallel example is a `bash` fence of `Task:` lines, which `check-fence-command-head` reds as naming nothing that runs. The line admits `Task:` as a command head. The colon is part of the word.
> - `unregister.list` names no gate.

companion/SPEC.md §The tested claim: *A recipe may drop a gate only where a toolkit idiom reds it. Dropping one of these four narrows the claim, so it is a change to this section and not a recipe edit.* becomes *No recipe drops a gate. A toolkit idiom a gate reds is answered by a knob line fitting the gate to it. Dropping a gate would narrow what the companion ships, so it is a change to this section and not a recipe edit.*

`docs/spec-toolkits.md`: the opener's *a few lines of gate configuration and, for Spec Kit, one dropped gate, applied once after the install* becomes *a few lines of gate configuration, applied with the install*. The Spec Kit recipe sentence's *and drops `check-fence-command-head`, which reds on the task list's example block of `Task:` lines* becomes *and admits `Task:` as a command, the first word of each line in the task list's example block*. That sentence sits on the Spec Kit page. `companion/speckit/README.md`'s *and drops the one gate a Spec Kit task list trips* becomes *and admits the `Task:` lines of a Spec Kit task list's example block*.

### (2) The two tiers in the design record {design-bearing}

**Not yet applied.** companion/SPEC.md:

- The opener's *a **recipe** that makes the `prose` profile govern that toolkit's tree* becomes *a **recipe** that fits the `prose` and `full` profiles to that toolkit's tree*.
- §Recipes, **Each line answers a red**: *with the `prose` profile installed* becomes *with the `prose` or `full` profile installed*.
- After §Applying a recipe, the section companion-recipe-in-payload's amendment writes in place of §The procedure, a section **§The two tiers** is added:

  > Each toolkit has two install lines. The default, `prose`, puts canon-kit's document gates over the toolkit's tree, owes no `bash`, and seeds nothing beside the toolkit's own files. The second, `full`, installs every kit, so every gate a `full` install registers runs over the tree. It costs two things, and the toolkit's page names both. It needs `bash` 4.3 or later, since `full` carries kits that ship bash files (docs/install.md §Requirements). It also seeds Checkwright's own workflow surfaces beside the toolkit's: a task queue, a doctrine block in the agent file, and evidence files under `.workflow/`. Moving from `prose` to `full` only adds (installer/SPEC.md §Profiles), so an adopter who installed the default reaches `full` by running `init` with the second line.
  >
  > The documented `full` line sits between `<!-- companion-full:begin -->` and `<!-- companion-full:end -->` on each toolkit's page, in the same `text` form as the default line. On OpenSpec it also applies the lifecycle layer, which no gate `init` registers reads until a stage session runs (§The lifecycle layer). The Spec Kit extension installs the default only.
  >
  > Neither tier re-checks what a toolkit owns: OpenSpec's validator and Spec Kit's templates stay the toolkit's.

- §The lifecycle layer's first sentence, *An adopter who runs lifecycle-kit's stage machine installs `full`, …*, becomes *OpenSpec's `full` line (§The two tiers) applies the layer.* The rest of the paragraph stands.
- §The tested claim's first sentence becomes *The recipes are proved against four defect classes per toolkit, each caught by a named gate, on `prose` for both toolkits and on `full` for Spec Kit, whose `full` leg plants them again:* and the OpenSpec `full` line is held green by the lifecycle leg.

`companion/README.md`: *what makes the `prose` profile govern a Spec Kit or OpenSpec spec tree* becomes *what fits the `prose` and `full` profiles to a Spec Kit or OpenSpec spec tree*. Its lifecycle sentence becomes *Each toolkit also has a `full` line, which installs every kit and on OpenSpec applies `openspec/lifecycle/` too ([SPEC.md §The two tiers](SPEC.md#the-two-tiers)).*

### (3) The `full` lines on the pages {mechanical}

**Not yet applied.** The Spec Kit page gains, after the recipe sentence:

> **Every kit instead of `prose`.** After the extension's install, or in place of it, run your system's line from the [install page](install.md#install) with:
>
> <!-- companion-full:begin -->
>
> ```text
> checkwright init --profile full --recipe speckit
> ```
>
> <!-- companion-full:end -->
>
> `full` adds every kit's gates to the four above. It needs `bash` 4.3 or later, which stock macOS lacks ([macOS and Linux](install.md#macos-and-linux) has the remedy). It seeds Checkwright's own task queue, a doctrine block in your agent file, and evidence files under `.workflow/`, beside Spec Kit's own.

The OpenSpec page's lifecycle paragraph, which holds the `companion-full` block, opens instead with **Every kit instead of `prose`.** It names the same two costs. It then says the line also applies the lifecycle layer, so a change touching two capabilities owes the align stage before build once you run lifecycle-kit's stage machine.

`docs/spec-toolkits.md`: the opener gains *Each toolkit has two lines: `prose`, the default, and `full`, every kit.* §What is tested gains, after its first sentence, *On Spec Kit it also installs `full` with the recipe and asserts the battery green and each defect caught.*

### (4) The companion arm's Spec Kit `full` leg {design-bearing}

**Not yet applied.** installer/SPEC.md §The consumer smoke, the companion-arm paragraph, as written by companion-recipe-in-payload's amendment (its delta 7). If that unit lands in this batch, its text is written with these additions. Otherwise it is edited to add them:

- the arm also reads each toolkit's `companion-full` block, the Spec Kit page's and the OpenSpec page's, refusing an absent or empty one, or one whose line does not carry `--profile full`;
- *The Spec Kit toolkit then runs a `full` leg in a second consumer: `init` with its `companion-full` line's arguments, the battery green, and each planted defect red by the gate that owns it.*

`installer/consumer-smoke/run-smoke.sh`'s companion arm gains the leg, printing its own header, `companion arm for speckit full`, so it is its own scenario. The leg reuses the toolkit function with the argument words and a label. The header comment's companion clause names the leg.

docs/site-architecture.md §Generated projections and their freshness gates, the **toolkit install lines** bullet, gains *The Spec Kit page carries a `companion-full` block too, holding its `full` line, read by the same arm.*

## Producers and consumers

Probe: `.tmp`-scratch packs at 0.28.0 installing `prose`, `delegation` and `full` on both fixture trees with each recipe, the Spec Kit one with `unregister.list` emptied and the `Task:` line added, then each planted defect overlaid (survey filed at 8e44fe1b); `git grep -n "dropped gate\|drops\|prose. profile" -- companion docs/spec-toolkits.md`; docs/install.md §Requirements' toolchain rows read.

- **The re-armed gate** (delta 1). Producer: `init --recipe speckit`, which now registers `check-fence-command-head` and composes the `Task:` line into the canon-kit seam. Consumer: the battery, on the pre-commit hook and CI. Red condition: the companion arm's Spec Kit legs red if the line is missing, since the fixture's tasks file carries the `Task:` fence. The line is therefore held both ways, as §Recipes' **Each line answers a red** requires.
- **The `companion-full` blocks** (deltas 3 and 4). Producer: the hand-authored pages. Reader: the companion arm, the Spec Kit `full` leg and the OpenSpec lifecycle leg. The arm reds by name on an absent block or a line with no `--profile full`.
- **Readers of the tier prose** (delta 2): the companion SPEC's and README's mirrors, regenerated.
- **Red conditions for adopters.** A Spec Kit tree that applies the recipe from this release on gains one registered gate. A tree whose tasks hold a `bash` fence of other non-commands now reds there. The remedy is the gate's own help, another info string or a `CANON_KIT_FENCE_PROGRAMS_EXTRA` line.

## Existing sections updated

Roster probe: the probes above, plus `git grep -n "check-fence-command-head" -- companion docs installer`.

- `companion/speckit/recipe/unregister.list` and `companion/speckit/recipe/canon-config.knobs` (delta 1).
- `companion/SPEC.md` — §The Spec Kit recipe, §The tested claim (deltas 1 and 2); the opener, §Recipes, §The two tiers (new), §The lifecycle layer (delta 2).
- `docs/spec-toolkits.md` (deltas 1 and 3).
- `companion/speckit/README.md` (delta 1).
- `companion/README.md` (delta 2).
- `docs/speckit.md` and `docs/openspec.md`, where the page split has landed (deltas 1 and 3).
- `installer/SPEC.md` — §The consumer smoke (delta 4).
- `installer/consumer-smoke/run-smoke.sh` (delta 4).
- `docs/site-architecture.md` — §Generated projections and their freshness gates (delta 4).
- `.workflow/surface-ceiling.txt` — the rows of each grown `*/SPEC.md` and `docs/*.md` above, re-stamped with `--emit always-loaded --ceiling` in the growing commit (deltas 2, 3 and 4).
- `docs/companion/SPEC.md` — the generated on-site mirror, as are `docs/companion/README.md` and `docs/installer/SPEC.md`, each regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **the Spec Kit recipe**. It keeps `check-fence-command-head` registered and admits `Task:` as a fence command head. A tree that applies it through `init --recipe speckit` gains that gate (delta 1).

## Retired spellings

- None — the deltas change a recipe's lines, add a section and a smoke leg, and re-phrase prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery and the installer consumer smoke green on the landing commit, and the `companion-toolkits` leg green on the mid-iteration push. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
