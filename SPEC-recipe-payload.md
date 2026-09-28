# SPEC amendment: recipe-payload

A spec-toolkit recipe reaches an adopter by hand today. The OpenSpec page has the reader fetch the recipe files from raw.githubusercontent.com at a tag they type, then paste a `sh` block that appends knob lines, drops gates, regenerates the hook and the graph, runs the battery and commits (companion/SPEC.md §The procedure). A typed tag that differs from the installed release applies a recipe the gates were never tested with, and no Latest pointer can stand in for the tag, since every release is a pre-release (installer/SPEC.md §The release channel). The Spec Kit extension ships its recipe in its zip for the same reason. This amendment puts the recipes in the package and has `init` apply them by name, so a recipe always matches the release that applies it and the adopter types no version and fetches nothing.

One queue entry pairs it: [companion-recipe-in-payload](TASK-QUEUE.md#companion-recipe-in-payload).

**The rulings.**

- **`init --recipe <name>` applies a payload recipe, and the manifest records it.** companion/SPEC.md §The procedure refused an `init` flag because a recipe would become `init`'s file, rewritten on the next run unless the manifest recorded the recipe as well. The manifest records it. Every re-run then re-derives the same seam text, so the recipe lines sit on `init`'s ownership roster like the template lines around them, and `diff`, `update` and `uninstall` reach them with no new rule. The block's other ground, that a shipped script would owe a PowerShell twin, does not reach a flag, because the flag runs behind the invoke.
- **The packer carries recipes named by a keyed knob.** `GATE_SDK_PAYLOAD_RECIPES[<name>] = <dir>` names each recipe and the tracked directory it is packed from. The packer is kit mechanism generic over any publisher, so it takes its recipe set as config and mints no toolkit name. Each name is the token an adopter types and a publisher's choice, so no derivation from the tree could supply it, and a key-to-directory map is the knob shape that already exists for that. The default is empty, the off member.
- **Recipes sit beside `payload/`, never inside it.** Every directory under `payload/` is read as a kit, one reserved name excepted (installer/SPEC.md §Profiles). A second non-kit sibling there would need a second exclusion in every derivation that walks it. So the package gains `recipes/<name>/` at its root.
- **A recipe's lines are composed into the seam, never appended after it.** `init` writes each seam file as its kit template followed by each applied recipe's lines for that file, in `--recipe` order, and claims it before writing, as for any seam. An adopter's edited seam is kept, as it is today. The recipe lines it lacks are then printed with the remedy, and it is never overwritten.
- **The format moves to its applier.** A recipe's files and their meaning were companion/SPEC.md's while a pasted block applied them. `init` applies them now, so installer/SPEC.md owns the format, and companion/SPEC.md keeps what each toolkit's recipe holds and why.
- **The documented line is the tested line.** Each toolkit's install line sits in a marker block. The consumer smoke reads that block's `init` arguments and runs them against the packed tarball. That is the contract the pasted block's markers held for the procedure.
- **The seam.** Kit mechanism: `GATE_SDK_PAYLOAD_RECIPES` and the packer's placement (gate-sdk). Repo-root mechanism: the `--recipe` flag, the manifest key and the recipe format (installer). Consumer config: this repository's three keyed values, in `scripts/gate-sdk-config.knobs`. No private rule content, and no toolkit name enters a kit file.

**Refused.**

- **Deriving the recipe set from a glob.** `companion/*/recipe/` gives two of the three recipes, and the lifecycle layer is deliberately not under a `recipe/` directory (companion/SPEC.md §The lifecycle layer). The name an adopter types is a choice that no path spells.
- **A recipe file of its own that the kit reads beside the seam.** The knob reader takes one file per kit. A second file per recipe would be new knob-grammar machinery to do what composing the seam already does.
- **Appending recipe lines to a kept seam.** Merging into a file `init` does not own is the overwrite the ownership contract forbids, in a smaller form.

## What changes

### (1) The recipe format and `init --recipe` {design-bearing}

**Not yet applied.** installer/SPEC.md gains a section after §What init seeds:

> ## Payload recipes
>
> A **payload recipe** fits the kits to a tree some other tool lays out, such as a spec toolkit's. It is a directory under the package's `recipes/`, named by the token an adopter passes, and it holds two kinds of file. Each `<stem>.knobs` file holds knob lines for the seam file of the same name. `unregister.list` names gates, one per line, that the registry drops. A line opening with `#` is commentary in both, and any other file is not read. The packer places them (§The packer), and `init --recipe <name>` applies them.
>
> **`--recipe` repeats, and the order is the application order.** `--no-recipe` clears the set. A run passing neither re-applies the set the manifest records (§The manifest), just as a run passing no `--profile` re-applies the recorded profile. A run passing `--recipe` replaces the recorded set with the one it names. `update` forwards both flags unchanged (§update).
>
> **Applying one changes two things `init` already writes, and adds nothing to the tree.** Each seam file `init` writes is its kit template followed by the lines of every applied recipe's file of that name, in order. The registry `init` derives (§What init seeds) loses every gate an applied `unregister.list` names, and a name the registry does not carry is no finding, since a narrower profile may not register it. Both files are claimed before they are written, as every seam is. So a re-run with the same recipes rewrites what it wrote, and `diff` and `uninstall` read the recipe lines as `init`'s.
>
> **Refused before any write, at exit 2:** a name the package's `recipes/` does not carry, with the help line naming those it does; a recipe `.knobs` file naming a seam file the resolved profile does not write, with the help naming the profile to choose instead; and a composed seam the knob grammar refuses, such as two recipes setting one scalar, naming both recipes. Each is a selection the run cannot honour, so it refuses the way an unknown profile does.
>
> **A kept file is reported, never merged into.** When the adopter has edited a seam file or the registry, `init` keeps it (§init) and cannot place a recipe's change there. It prints each recipe line the kept seam lacks, and each gate the kept registry still registers that a recipe drops, with the remedy: add them by hand, or pass `--force` to take `init`'s file. A kept file that already carries every line, as a tree that applied a recipe by hand does, is reported as kept and nothing more.
>
> `--dry-run` plans the composed seams and the dropped gates through the same code, and prints them in its plan.
>
> **This subtracts from a profile's registry, and the lattice is still intact.** §Profiles' monotonicity is a claim about the registries profiles write, and §The consumer smoke asserts it over recipe-free installs. A recipe is consumer config applied after the profile is chosen. It is recorded in the manifest beside the profile and does not change which kits the profile vendors.

installer/SPEC.md §init's flag paragraph (the one opening **Re-running is idempotent and non-destructive**) is not rewritten. §Payload recipes owns the flag. `native/src/installer/init.rs` parses `--recipe` and `--no-recipe`, resolves each name under the package's `recipes/`, composes each seam from its template and the recipes' lines before the claim, drops the named gates from the planned registry, runs the three refusals ahead of any write, and reports a kept file's missing lines. The composition reads the payload's recipe files, and the registry drop is data read from `unregister.list`, so no gate name enters the crate and `check-install-disposition`'s third assertion still holds. Unit tests hold composition order, the three refusals, the kept-file report with the lines missing and with them present, the recorded-set default, `--no-recipe`, and a dry-run plan equal to the real run's writes.

### (2) The manifest records the recipes {design-bearing}

**Not yet applied.** installer/SPEC.md §The manifest's field table gains a row after `profile`:

> | `recipes` | the payload recipes applied, in application order, or absent when none were | a re-run of `init` re-applies them when no `--recipe` or `--no-recipe` is passed (§Payload recipes); `doctor` reports them beside the profile |

The field is absent, never empty, when no recipe applies, on the table's present-when-supplied rule, so a recipe-free install writes the same manifest it writes today. It is a new optional top-level key, additive within `checkwright-lock v1` (§The manifest, *A new optional top-level key*). A release built before it ignores the key: that run re-applies no recipe, so it keeps the seams as the adopter's changes. Its downgrade refusal stands in front of the run anyway. The residual manifest carries no `recipes`, for the reason it carries no `profile`. `native/src/installer/lock.rs` writes and reads the key, and §doctor's identity bullet, *reports the installed release, the upstream commit it came from, the profile and the kit set*, gains *and the recipes applied*, which `native/src/installer/doctor.rs` prints.

### (3) The knob and the packer's placement {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md §Layout and configuration gains, after the `GATE_SDK_PAYLOAD_LICENSE` bullet:

> - `GATE_SDK_PAYLOAD_RECIPES` (keyed, default empty): each pair names a payload recipe and the repo-relative directory it is packed from. The packer places each directory's tracked files at `recipes/<name>/` in the package root (installer/SPEC.md §The packer). A name is `[a-z0-9][a-z0-9-]*`, since an adopter types it and a directory carries it. Empty packs no recipe.

gate-sdk/SPEC.md §Consumer payload, after the paragraph opening **Each packed kit also carries the publisher's license text**, gains:

> **The package may also carry payload recipes**, named by `GATE_SDK_PAYLOAD_RECIPES`: consumer config a publisher ships for `init` to apply by name. They ride the package root beside the payload, never inside it, because every directory under the payload is a kit to its readers.

installer/SPEC.md §The packer: the footprint's first member, *under `installer/` and under each root the kit-root resolver yields, and the license file*, gains *, and each directory `GATE_SDK_PAYLOAD_RECIPES` names*. After the paragraph opening **The license text is placed where it ships**, it gains:

> **Recipes are extracted, like kits, from the stamped commit.** For each `GATE_SDK_PAYLOAD_RECIPES` pair the packer extracts the directory's tracked files with `git archive` at the stamped commit into `{asm}/recipes/<name>/`. It refuses at exit 2 a name outside the knob's grammar, and a directory with no tracked file at that commit, since a recipe the package cannot carry is a broken payload rather than a smaller one. Extraction is not a write into packed content, so the write set above stays three.

`native/src/emit/pack_installer.rs` implements the extraction and both refusals, and its `KNOBS` roster gains `GATE_SDK_PAYLOAD_RECIPES`. `native/src/knobs/gate_sdk.rs` gains a keyed row. Unit tests hold a two-recipe placement, the empty default, and both refusals.

### (4) The package roster and this repository's recipes {mechanical}

**Not yet applied.** `installer/package.json`'s `files` roster gains `"recipes/"`. installer/SPEC.md §Layout gains a bullet after `payload/`:

> - `recipes/` — the payload recipes (§Payload recipes), assembled at pack time from the directories `GATE_SDK_PAYLOAD_RECIPES` names, never tracked here.

`scripts/gate-sdk-config.knobs` gains three pairs, with a `# spec:` line citing companion/SPEC.md §Recipes:

```text
GATE_SDK_PAYLOAD_RECIPES[openspec] = companion/openspec/recipe
GATE_SDK_PAYLOAD_RECIPES[openspec-lifecycle] = companion/openspec/lifecycle
GATE_SDK_PAYLOAD_RECIPES[speckit] = companion/speckit/recipe
```

### (5) companion/SPEC.md cites the applier {design-bearing}

**Not yet applied.** companion/SPEC.md:

- The opening paragraph's *and a fixture tree that proves the recipe* sentence stands. The paragraph gains *The package carries the recipes, and `init --recipe <name>` applies one ([installer/SPEC.md §Payload recipes](../installer/SPEC.md#payload-recipes)).*
- §The component: *so no kit-root resolver admits it and no install payload carries it* becomes *so no kit-root resolver admits it. The package carries its recipes under `recipes/`, named in this repository's `GATE_SDK_PAYLOAD_RECIPES`: `openspec`, `openspec-lifecycle` and `speckit`.*
- §Recipes' first paragraph is replaced by: *A recipe is a directory in installer/SPEC.md §Payload recipes' format, and that section owns how `init` applies one. Each file opens with a `#` comment citing this file.* The paragraph opening **Each line answers a red** stands.
- §The procedure is replaced whole by a section **§Applying a recipe**:

  > An adopter applies a recipe with the install itself: `init --profile <profile> --recipe <name>`, through the one-line install or any other route to `init`. The install makes one commit with the recipe applied, and the manifest records the recipe, so an upgrade re-applies it with the release it was tested against. Each toolkit's documented line sits between `<!-- companion-install:begin -->` and `<!-- companion-install:end -->` as one `sh` fence holding one line: on the landing page for OpenSpec, and in `speckit/commands/install.md` for Spec Kit. The OpenSpec lifecycle line sits between `<!-- companion-lifecycle:begin -->` and `<!-- companion-lifecycle:end -->` on the landing page. The companion arm runs the `init` arguments of each line, so the documented steps are the tested steps.
  >
  > **A flag, where this section once refused one.** The refusal's ground was that a flag would make a recipe `init`'s file, rewritten on the next run, unless the manifest recorded the recipe as well. It records it now. Its second ground, a script's PowerShell twin, does not reach a flag that runs behind the invoke. So the one-line install carries a recipe on every host, with no `sh` block to paste and no tag to type.

- §The lifecycle layer: *applies the toolkit's recipe, then applies `<toolkit>/lifecycle/` with the same procedure* becomes *and passes `--recipe <toolkit> --recipe <toolkit>-lifecycle`*. *The directory is not named `recipe/`, so the companion arm's toolkit roster, read from `*/recipe/`, does not take it for a third toolkit* stands.
- §The Spec Kit extension: the `speckit.checkwright.install` bullet becomes *installs the release this extension version was tested with, with the recipe applied. It runs the one-line install with `CHECKWRIGHT_VERSION` set to `extension.version` and the arguments `init --profile <profile> --recipe speckit`, then runs the commands `init` printed.* The extension bullet's *`extension.yml`, two command files, a `README.md` and the recipe* becomes *`extension.yml`, two command files and a `README.md`*.
- §Packing the extension: the first bullet's extraction gains *, less `recipe/`, which the package carries instead*.
- §The toolkit legs, step 1: *`.specify/extensions/checkwright/extension.yml` and the recipe present* becomes *`.specify/extensions/checkwright/extension.yml` present*.
- §Honest limits: *The Windows route is documented and not run* stands. The recipe no longer adds a Git for Windows `sh` step to that route, and a leg that pipes the PowerShell line would retire the limit.

### (6) The extension, the READMEs and the page {mechanical}

**Not yet applied.**

- `companion/speckit/commands/install.md`: the **Install the release** step's two lines gain ` --recipe speckit` after `init --profile <profile>`. The `sh` line moves between `<!-- companion-install:begin -->` and `<!-- companion-install:end -->`. The **Apply the recipe** step and its block are deleted. The closing rule *drop a gate the recipe does not name* becomes *drop a gate*.
- `companion/speckit/README.md`: *It then applies the Spec Kit recipe in `recipe/`, which prunes …* becomes *with the Spec Kit recipe applied, which prunes …*; *It makes two commits: the install, then the recipe.* becomes *It makes one commit.*
- `companion/README.md`: *applied by one block of `sh` after `init`* becomes *applied by `init --recipe <toolkit>`*. *applied after the recipe by the same block* becomes *applied with `--recipe openspec-lifecycle`*. **Using it**'s OpenSpec bullet becomes *install with `--recipe openspec`*. The lifecycle sentence becomes *install `full` instead of `prose`, and on OpenSpec add `--recipe openspec-lifecycle`*.
- `docs/spec-toolkits.md` §OpenSpec: the install line becomes `curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --profile <profile> --recipe openspec` between `companion-install` markers. The Windows sentence names `init --profile <profile> --recipe openspec`. The fetch paragraph, its fence and *and run the recipe block below* are deleted. §The recipe block is deleted. The lifecycle paragraph gives its line, `… | sh -s -- init --profile full --recipe openspec --recipe openspec-lifecycle`, between `companion-lifecycle` markers. §What is tested's *applies the recipe with the block above* becomes *applies the recipe through the install line above*.
- `docs/install.md` §Choosing a profile: *take `prose` and fit it with [Spec Kit and OpenSpec](spec-toolkits.md)* becomes *pass `--recipe` with the toolkit's name, as [Spec Kit and OpenSpec](spec-toolkits.md) shows*.
- `installer/README.md`: the paragraph under the profile list gains *`--recipe <name>` applies a recipe the package carries, for a tree another tool lays out, such as a spec toolkit's.*

`<profile>` above is the profile [companion-gate-widening](TASK-QUEUE.md#companion-gate-widening) settles for each toolkit. If that unit lands in the same batch, write its profile. Otherwise write `prose`, today's.

### (7) The consumer smoke's companion arm {design-bearing}

**Not yet applied.** installer/SPEC.md §The consumer smoke, the paragraph opening **The companion arm** and the numbered list and paragraph after it are replaced by:

> **The companion arm** runs after the demo arm and proves each recipe on its fixture tree ([companion/SPEC.md](../companion/SPEC.md#recipes)). It prints one header per toolkit, so each is its own scenario. It reads each toolkit's install line from its `companion-install` block, the landing page's for OpenSpec and the extension's install command's for Spec Kit, and takes the words after `sh -s --` as that line's `init` arguments. It refuses an absent or empty block, or a line carrying no `--recipe`, rather than skipping it. For each toolkit it:
>
> 1. asserts that the installed package carries `recipes/<toolkit>/`, so the knob covers every recipe directory under `companion/*/recipe/`;
> 2. makes a scratch consumer, commits `companion/fixtures/<toolkit>/layout/` into it, runs `init` with the line's arguments from the packed tarball, turns the hooks on, and asserts one commit, a clean worktree, and the manifest's `recipes` equal to the line's;
> 3. asserts the battery green, reading its summary line, then asserts a bare `init` re-run leaves the tree object unchanged, since it re-applies the recorded recipes;
> 4. for each `companion/fixtures/<toolkit>/defects/<gate>/`, overlays its files, asserts the battery red with a `FAIL: <gate>` line, and restores the commit.
>
> The OpenSpec toolkit then runs a lifecycle leg in a second consumer. It runs `init` with the arguments of the `companion-lifecycle` line and asserts it green. It overlays `companion/fixtures/openspec/lifecycle/` and asserts `--only check-stage-entry` red naming every in-flight delta directory. It then asserts it clean once the overlay's deltas and capability specs are removed ([companion/SPEC.md §The lifecycle layer](../companion/SPEC.md#the-lifecycle-layer)). Each toolkit's defect set must cover the gates [companion/SPEC.md §The tested claim](../companion/SPEC.md#the-tested-claim) names, so a missing defect directory reds rather than shrinking the claim. The headers are literals, which the scenario parser needs. So the arm derives the recipe set from the `companion/*/recipe/` directories and reds on a recipe no header ran.

`installer/consumer-smoke/run-smoke.sh`'s companion arm is rewritten to it: `companion_block` reads a named marker pair's lines, `COMPANION_EXT_RECIPE` and the page-and-command equality are deleted, `companion_consumer` takes the argument words, `companion_green` becomes the `init`-and-battery assertion above, and the Spec Kit command leg merges into the Spec Kit toolkit, whose line is the command's. The header comment's companion clause is rewritten to match.

### (8) The site roster and the toolkit leg {mechanical}

**Not yet applied.** docs/site-architecture.md §Generated projections and their freshness gates, the bullet **The recipe block** is replaced by:

> - **The toolkit install lines** — `docs/spec-toolkits.md` carries two marker blocks, hand-authored, each one `sh` fence holding one line. The pair `companion-install:begin` and `companion-install:end` holds the OpenSpec install line, and the pair `companion-lifecycle:begin` and `companion-lifecycle:end` holds its lifecycle line. `companion/speckit/commands/install.md` carries a `companion-install` block holding the Spec Kit line. Their reader is the consumer smoke's companion arm, which runs each line's `init` arguments and reds by name on an absent or empty block ([companion/SPEC.md §Applying a recipe](companion/SPEC.md#applying-a-recipe)). The contract is the install blocks' own: no gate holds a block, and a red arrives with the smoke.

`.github/workflows/gates.yml`'s `companion-toolkits` job drops its `recipe/unregister.list` assertion. `scripts/ci-pack-extension.sh` excludes `recipe/` from the archive it extracts.

## Producers and consumers

Probe: `git grep -n "companion-recipe"`, `git grep -n "COMPANION_EXT_RECIPE\|recipe=.specify"`, `git grep -n recipe -- .github scripts`; `installer/package.json`'s `files`; `native/src/installer/init.rs` read for the flag parser, the seam plan and the registry claim; a local pack at 0.28.0 with the three recipes applied by the block to `prose`, `delegation` and `full` installs on both fixture trees, each green.

- **`--recipe` / `--no-recipe`** (delta 1). Producer: the adopter's `init` or `update` invocation, through any bootstrap. Consumers: `init`'s seam composition and registry derivation; the dry-run plan.
- **`recipes/<name>/` in the package** (deltas 3 and 4). Producer: `--pack-installer`, under `GATE_SDK_PAYLOAD_RECIPES`, which this repository sets (delta 4), so every release pack carries the three. Consumers: `init --recipe`; the companion arm's step 1 (delta 7).
- **`GATE_SDK_PAYLOAD_RECIPES`** (delta 3). Reader: the packer. Roster-holding readers: gate-sdk's static table and `--emit knob-roster`; the packer's `KNOBS` declaration, read by the knob-file derivation, `check-reads-couples` and `check-gate-substrate-parity`; `check-knob-citation`, satisfied by the SPEC bullet; `check-docs-cmd` assertion B, satisfied by the table row.
- **The `recipes` manifest key** (delta 2). Producer: `init`, when the applied set is non-empty. Readers: `init`'s re-run default; `doctor`'s identity block; the companion arm's step 2. `uninstall` writes no `recipes` into a residual manifest.
- **The marker blocks** (deltas 5, 6 and 8). Producer: the hand-authored page and command. Reader: the companion arm (delta 7).
- **Red conditions.** The companion arm reds on an absent block, a line with no `--recipe`, a toolkit recipe the package lacks, a recipe no header ran, and a manifest whose `recipes` differs from the line. The `companion-toolkits` leg loses a presence assertion and gains no red (delta 8). `check-md-refs` resolves the new `#payload-recipes` and `#applying-a-recipe` anchors once the sections land, which is why deltas 1 and 5 land in the commit that adds the links.
- **Sibling units.** [piped-install-argument-witness](TASK-QUEUE.md#piped-install-argument-witness) pipes the served scripts with arguments, and its push need orders it after this unit's `init` form. [install-toolkit-page-structure](TASK-QUEUE.md#install-toolkit-page-structure) moves the `companion-install` and `companion-lifecycle` blocks to the OpenSpec sub-page and repoints the arm's page path. If it lands in the same batch, the arm reads the sub-page. Otherwise it reads `docs/spec-toolkits.md`.
- **The live-site window.** Pages deploy on every master push, so the `--recipe` lines are served from the push that carries this unit. The one-line install stays pinned to the newest tag, which lacks the flag, until the release that carries it moves the pin (installer/SPEC.md §The hosted install pin). `check-front-door-verbs` reads verbs, not flags, and holds nothing here.

## Existing sections updated

Roster probe: the probes above, plus `git grep -n "companion-recipe:"` and `git grep -n "no install payload carries it"`.

- `installer/SPEC.md` — §Payload recipes, new (delta 1); §The manifest and §doctor (delta 2); §The packer (delta 3); §Layout (delta 4); §The consumer smoke (delta 7).
- `native/src/installer/init.rs` (delta 1).
- `native/src/installer/lock.rs` and `native/src/installer/doctor.rs` (delta 2).
- `native/src/emit/pack_installer.rs` and `native/src/knobs/gate_sdk.rs` (delta 3).
- `gate-sdk/SPEC.md` — §Layout and configuration, §Consumer payload (delta 3).
- `installer/package.json` and `scripts/gate-sdk-config.knobs` (delta 4).
- `companion/SPEC.md` — the opener, §The component, §Recipes, §The procedure → §Applying a recipe, §The lifecycle layer, §The Spec Kit extension, §Packing the extension, §The toolkit legs, §Honest limits (delta 5).
- `companion/speckit/commands/install.md` (delta 6).
- `companion/speckit/README.md` (delta 6).
- `companion/README.md` (delta 6).
- `docs/spec-toolkits.md` (delta 6).
- `docs/install.md` (delta 6).
- `installer/README.md` (delta 6).
- `installer/consumer-smoke/run-smoke.sh` (delta 7).
- `docs/site-architecture.md` (delta 8).
- `.github/workflows/gates.yml` and `scripts/ci-pack-extension.sh` (delta 8).
- `docs/companion/SPEC.md` — the generated on-site mirror, as are `docs/installer/SPEC.md`, `docs/installer/README.md`, `docs/gate-sdk/SPEC.md` and `docs/companion/README.md`, each regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **`init --recipe`**: the package carries payload recipes and `init` applies one by name, recorded in `checkwright.lock`. A tree that applied a recipe with the old block keeps its seams as the adopter's changes. It takes `init`'s files with `--force --recipe <name>` (deltas 1 and 2).

## Retired spellings

- `companion-recipe:begin` — the procedure block's opening marker, gone with the block (deltas 6, 7 and 8).
- `companion-recipe:end` — its closing marker (deltas 6, 7 and 8).
- `COMPANION_EXT_RECIPE` — the smoke's extension recipe path, gone with the command's block (delta 7).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — `bash gate-sdk/bin/build-native.sh`, the full battery, the gate-sdk fixture suite and the installer consumer smoke green on the landing commit, and the `companion-toolkits` leg green on the mid-iteration push. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
