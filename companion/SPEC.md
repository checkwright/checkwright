# Checkwright companion — design record

The companion package puts Checkwright's gates over a repository whose specs another toolkit writes. It holds, for each supported spec-authoring toolkit, a **recipe** that fits each install tier (§The tiers) to that toolkit's tree, and a fixture tree that proves the recipe. The package carries the recipes, and `init --recipe <name>` applies one ([installer/SPEC.md §Payload recipes](../installer/SPEC.md#payload-recipes)). For Spec Kit it also holds a catalog extension that installs Checkwright and applies the recipe. `companion/README.md` is the usage tier in front of this file.

## The component

`companion/` is repo-root-governed and not a kit. It carries no `checks/` and no `smoke/`, so no kit-root resolver admits it. The package carries its recipes under `recipes/`, named in this repository's `GATE_SDK_PAYLOAD_RECIPES`: `openspec`, `openspec-lifecycle` and `speckit`. It adds no kit mechanism: each recipe is consumer config in knobs the `prose` profile reads, and the lifecycle layer is config in knobs lifecycle-kit reads. The toolkits' names live here, on the docs pages and on the front door, and never in a kit SPEC or a kit literal ([gate-sdk/SPEC.md §The provenance seam](../gate-sdk/SPEC.md#the-provenance-seam)).

- `toolkits.list` pins each toolkit, one `<toolkit> <package> <version> <name…>` line each. The toolkit name keys the line, the package is what the toolkit leg installs, the version is what it pins and asserts (§The toolkit legs), and the display name heads the toolkit's column in the support table (§The support table), in the file's order.
- `native.list` rosters the checks each toolkit itself ships, one `<toolkit> <arguments…>` line each. The toolkit leg runs every line and the support table lists it, so the roster has two readers and no line goes untested.
- `exclusions.list` records the kits each toolkit's `complement` line leaves out, one `<toolkit> <kit> <surface…>` line each: the kit, and the toolkit's own surface that already does its job. The companion arm holds each toolkit's `complement` line to its rows (§The tiers).
- `<toolkit>/recipe/` holds a recipe (§Recipes).
- `openspec/lifecycle/` holds OpenSpec's lifecycle layer (§The lifecycle layer).
- `speckit/` is the Spec Kit extension around its recipe (§The Spec Kit extension). The directory is not named after the toolkit's repository, and a page links that repository at its root only, because `check-kit-ref-liveness` reds a `<name>-kit` path segment that names no kit root.
- `fixtures/<toolkit>/` holds the fixture tree and its planted defects (§The fixtures).

## Recipes

A recipe is a directory in installer/SPEC.md §Payload recipes' format, and that section owns how `init` applies one. Each file opens with a `#` comment citing this file.

**Each line answers a red, answers a red of a house rule the adopter registers, or arms a gate that registers disarmed.** Install any tier on a fresh tree of the toolkit at its pin. It reds a gate on one of the toolkit's own idioms until the line is there. For a house rule, it would red once the adopter registers that rule with `--with-gate`. Or a gate asserts nothing on that tree until the line points it there. The companion arm of the consumer smoke holds that both ways: a line it shows unnecessary is deleted, and one it shows missing is added, each with its idiom recorded below.

### Applying a recipe

An adopter applies a recipe with the install itself: `init --profile prose --recipe <name>`, through the one-line install or any other route to `init`. The install makes one commit with the recipe applied, and the manifest records the recipe, so an upgrade re-applies it with the release it was tested against. Each toolkit's documented line sits between `<!-- companion-install:begin -->` and `<!-- companion-install:end -->` as one fence holding one line. For OpenSpec it is on the OpenSpec page, `docs/openspec.md`, a `text` fence reading `checkwright init …`, where `checkwright` stands for the reader's install line (docs/install.md §Managing's convention), so the page points to the install page rather than restating it. For Spec Kit every line is in `speckit/commands/install.md`, each an `sh` fence holding the one-line install the agent runs, and the Spec Kit page's `complement` and `full` lines carry the same words from `init`. Each toolkit's `complement` and `full` lines take a marker pair each (§The tiers). The companion arm runs each line's words from `init` to the line's end, so the documented steps are the tested steps.

**A flag, not a pasted block.** The manifest records the recipe, so a re-run re-applies it rather than rewriting it, and a flag behind the invoke owes no PowerShell twin. So the one-line install carries a recipe on every host, with no `sh` block to paste and no tag to type.

### The tiers

Each toolkit has three install lines, each a profile with the toolkit's recipe applied, and each contains the one before it, so moving up only adds (installer/SPEC.md §Profiles). An adopter reaches the next tier by running `init` with its line, or asks the install command for it.

- **`prose`**, the default, puts canon-kit's document gates over the toolkit's tree and owes no `bash`. Of Checkwright's workflow surfaces it seeds only a task-queue skeleton, the file canon-kit's task-liveness gates resolve a marker against (installer/SPEC.md §What init seeds).
- **`complement`** installs every kit except those whose job the toolkit already does, so every gate the toolkit does not already do runs over the tree. It is `full` with a `--without-kit` per left-out kit (installer/SPEC.md §Selecting kits and gates), so a kit the payload gains joins it. A kit is left out when the toolkit, at its pin, ships the surface the kit's subject governs, and `exclusions.list` records each with that surface; the line's `--without-kit` set is exactly the toolkit's rows. It needs `bash` 4.3 or later, since it carries kits that ship bash files (docs/install.md §Requirements), and seeds evidence-kit's files under `.workflow/` beside the queue skeleton.
- **`full`** installs every kit. It also needs `bash` 4.3 or later, and seeds Checkwright's own workflow beside the toolkit's: queue-kit's task queue, the stage machine's state file, a doctrine block in the agent file, and evidence files under `.workflow/`.

**A left-out kit is the path to replacing that piece of the toolkit's workflow.** Run the `complement` line again without that kit's `--without-kit`: a run passing any selection flag replaces the recorded selection whole. Each toolkit's page carries its `complement` line between `<!-- companion-complement:begin -->` and `<!-- companion-complement:end -->` and its `full` line between `<!-- companion-full:begin -->` and `<!-- companion-full:end -->`, in the same `text` form as the default line. On OpenSpec the `full` line also applies the lifecycle layer, which no gate `init` registers reads until a stage session runs (§The lifecycle layer). The Spec Kit extension's install command installs any of the three, by the user's input, its `complement` and `full` lines each in its own block.

No tier re-checks what a toolkit's own checks cover, unless an overlap clears the value bar. An overlap re-checks ground a toolkit's check covers, and is admitted only for a defect that check misses where it matters: a check that runs only after commit, or one a flag skips. The miss is measured at the toolkit's pin, recorded with that version, and pinned by a fixture that the toolkit leg runs the toolkit's own check against (§The toolkit legs), so a toolkit release that closes the miss is seen rather than assumed. An overlap is additive and off by default. It ships as a recipe layer, `<toolkit>/overlap/`, which the adopter applies as `--recipe <toolkit>-overlap` beside the toolkit's recipe, on the lifecycle layer's pattern, and the toolkit's own check stays enabled. The default install line carries no overlap, and the manifest records the layer as it records any recipe. The gate an overlap arms is a kit's and stays toolkit-blind; the layer holds its binding.

### The Spec Kit recipe

Spec Kit's own checks: none over the written specs. It ships templates and no validator.

Measured on `specify init --here --non-interactive --integration claude --script sh`, which writes `.specify/` and the agent's skills, with one feature written under `specs/`:

- `GATE_SDK_PRUNE_EXTRA_DIRS = .specify`, in `gate-sdk-config.knobs`. Spec Kit's own scripts under `.specify/scripts/bash/` red `check-comment-tier`, since their comments carry no tier tag, and `check-path-dialect`, on their absoluteness tests. They are the toolkit's files and not the adopter's.
- `CANON_KIT_PROSE_SURFACE_GLOBS[] = specs/**/*.md`, in `canon-config.knobs`. The feature specs join the governed doc set. Without the line no claimed gate reads `specs/`, so every planted defect passes.
- `CANON_KIT_FENCE_PROGRAMS_EXTRA[] = Task:`, in `canon-config.knobs`. The tasks template's parallel example is a `bash` fence of `Task:` lines, which `check-fence-command-head` reds as naming nothing that runs. The line admits `Task:` as a command head. The colon is part of the word. The gate's other remedy, another info string, would edit Spec Kit's template text, which no recipe edits.
- `CANON_KIT_TASK_LIST_GLOBS[] = specs/*/tasks.md`, in `canon-config.knobs`, arms `check-task-path-claim` and `check-task-label-resolution` on each feature's task list.
- `CANON_KIT_TASK_LABEL_CITES[story] = \[US([0-9]+)\]` and `CANON_KIT_TASK_LABEL_DEFINES[story] = ^### User Story ([0-9]+)[ ]`, in the same file, bind the user-story family: a task's `[USn]` label names the `### User Story n` heading in the `spec.md` beside it. The define pattern's trailing space is a bracket, since the knob grammar trims a value's trailing blank. A converge task's `per FR-003` is not bound, since each claimed gate has one planted defect and the family would be untested; an adopter adds that family with two knob lines.
- `unregister.list` names no gate.

### The OpenSpec recipe

OpenSpec's own checks: `openspec validate --strict`, over the structure of specs and change deltas, and `openspec archive`, which refuses a MODIFIED, RENAMED or ADDED delta disagreeing with its base spec unless `--skip-specs` is passed.

Measured on `openspec init --tools claude --no-animation --no-copilot-cloud .`, with a capability spec and one change:

- `CANON_KIT_PROSE_SURFACE_GLOBS[] = openspec/**/*.md`, in `canon-config.knobs`, for the reason the Spec Kit line has.
- `CANON_KIT_TEMPORAL_EXEMPT_PATHS[] = openspec/changes/**`, in the same file. A MODIFIED requirement in a change delta carries a `(Previously: …)` line, the form OpenSpec's own docs show, and `check-manifest-temporal` reds on it. Change deltas and the archive under `openspec/changes/` are history by design.
- `CANON_KIT_TASK_LIST_GLOBS[] = openspec/changes/*/tasks.md`, in the same file, arms `check-task-path-claim` on each in-flight change's task list. The single `*` keeps `openspec/changes/archive/` out, since an archived change's paths are history.
- `unregister.list` names no gate.

**One convention, left to the adopter.** OpenSpec specs often repeat a scenario title under two requirements, and a citation of the second title binds to the first. `check-spec-pointer`'s one-title-per-file rule reds the repeat when `CANON_KIT_SPEC_POINTER_TITLE_ONCE` is `on`, a house rule the recipe does not arm; the OpenSpec page states the convention and the knob line.

### The lifecycle layer

OpenSpec's `full` line (§The tiers) applies the layer, as `--recipe openspec-lifecycle` beside the recipe. The `complement` line leaves lifecycle-kit out, so it carries no layer, and `init` refuses the layer beside it, since the layer writes a seam no selected kit writes (installer/SPEC.md §Payload recipes). The layer binds the knobs that let `check-stage-entry` see the toolkit's in-flight work as amendments, so a cross-component build entry owes an align stamp. The directory is not named `recipe/`, so the companion arm's toolkit roster, read from `*/recipe/`, does not take it for a third toolkit.

- **OpenSpec.** `openspec/lifecycle/` binds the amendment glob to in-flight change deltas by path, the roster basename to `spec.md` and the contract token to `spec.md`. A change touching two capabilities fires the audit, and so does a delta citing another capability's spec. An archived change sits one level deeper, under `openspec/changes/archive/`, and is no amendment. A queue entry's `[spec:]` ref names the change's `proposal.md` by repo-relative path.
- **Spec Kit: no layer.** A feature's directory persists after it ships and the current feature is the branch's name, so nothing on disk marks a spec as in flight. A glob over `specs/*/spec.md` would demand the audit at every build entry once two features exist. `check-stage-entry` therefore sees no amendment in a Spec Kit tree, and the align trigger there is the authoring stage's own recommendation.

`check-spec-pointer`'s reach needs no layer. Both toolkits' specs are markdown, and each recipe already brings them into the governed set.

## The tested claim

The recipes are proved against the defect classes below, each caught by a named gate, on `prose` and `complement` for both toolkits and on `full` for Spec Kit, whose `complement` and `full` legs plant them again. Both toolkits carry:

- a broken relative or anchored link, by `check-md-refs`;
- a dangling section citation, by `check-spec-pointer`;
- an unclosed fence, by `check-spec-fence-balance`;
- a documented command that invokes a missing script, by `check-docs-cmd`;
- a ticked task naming a path that does not exist, by `check-task-path-claim`.

Spec Kit also carries a task citing a user story its spec does not define, by `check-task-label-resolution`.

The OpenSpec `full` line is held green by the lifecycle leg. Each `complement` leg also asserts the left-out kits absent: the manifest's `kits` holds none of the toolkit's `exclusions.list` kits, and its recorded selection removes exactly them, after the install and after a bare re-run and a bare `update`. Delta-to-base agreement is OpenSpec's, at archive. A gate for it is an overlap, and none ships until one clears the value bar (§The tiers). `plan.md`'s source tree draws the structure a feature will have, so no gate reads it either.

No recipe drops a gate. A toolkit idiom a gate reds is answered by a knob line fitting the gate to it. Dropping a gate would narrow what the companion ships, so it is a change to this section and not a recipe edit.

**One citation form passes unchecked.** `check-spec-pointer` reads a bare `<path>.md §<heading>` citation's path as repo-relative, so a file-relative `spec.md §…` is skipped. A bare `§Requirement: <name>` must name a requirement whole, since a lead clause shared by a family resolves nothing (canon-kit/SPEC.md §check-spec-pointer). The fixtures cite repo-relatively.

The support table on the toolkit page is derived from the fixture directories and `native.list`, so it states what is tested (§The support table).

## The fixtures

`fixtures/<toolkit>/layout/` is a tree in the toolkit's layout at its pin, authored here. Its section structure follows the toolkit's own templates, and no template text is copied, so the tree tests the same gates and ships no third-party text. It carries every idiom a recipe line answers, so a dropped line reds the arm's green leg or lets a planted defect pass. It also carries cross-file links and at least one repo-relative section citation that resolves.

- **Spec Kit:** `.specify/memory/constitution.md`; a `.specify/scripts/bash/` script with ordinary comments and a `/*` absoluteness test; and one feature under `specs/001-release-notes/` with `spec.md`, `plan.md` and `tasks.md`. Its `tasks.md` carries the `bash` fence of `Task:` lines, `[US1]` labels and a ticked `T003` naming a source file under `src/`.
- **OpenSpec:** `openspec/config.yaml`; one capability spec under `openspec/specs/`; and one change under `openspec/changes/` with a proposal, tasks and a delta whose MODIFIED requirement carries a `(Previously: …)` line. Its task `1.1` is ticked and names a source file under `src/`. A `full` overlay under `fixtures/openspec/full/check-stage-entry/` adds a second capability to that change, at a build cursor with no align stamp.

Each source file is neither markdown nor a shell script, so no other claimed gate reads it.

`fixtures/<toolkit>/defects/<gate>/` holds, per claimed gate, the files that replace their layout counterparts to plant that gate's defect. OpenSpec's `check-spec-pointer` defect is a citation of a requirement its spec does not carry, and its `check-md-refs` defect adds an anchored link to a missing requirement. Each toolkit's `check-task-path-claim` defect is its ticked task naming an absent path, and Spec Kit's `check-task-label-resolution` defect adds a task citing `[US2]`, which `spec.md` does not define.

`fixtures/<toolkit>/full/<gate>/` overlays the layout with what only the `full` line's gate reads, and names that gate.

`fixtures/<toolkit>/overlap/<gate>/` plants the defect an overlap exists for, over the layout. The companion arm applies the toolkit's recipe and overlap layer and asserts the gate reds it. The toolkit leg runs the toolkit's own check over the same planted tree and asserts the check misses it, so a pin move to a release that closes the miss reds the leg, and the overlap is weighed again (§The tiers).

The OpenSpec layout passes `openspec validate --all --strict` at the pin, with and without the `full` overlay, so the toolkit is its own fixture's oracle. This repository prunes every directory named `fixtures` from its own walks (`GATE_SDK_PRUNE_EXTRA_DIRS` in `scripts/gate-sdk-config.knobs`), since the trees carry each red on purpose. The companion arm governs them instead, inside a scratch consumer ([installer/SPEC.md §The consumer smoke](../installer/SPEC.md#the-consumer-smoke)).

## The Spec Kit extension

`speckit/` is a Spec Kit extension: `extension.yml`, two command files and a `README.md`.

- **`extension.yml`** declares `schema_version: "1.0"`, the extension's identity (`id: checkwright`), `requires.speckit_version` at `>=` the `speckit` pin in `toolkits.list` and git as a required tool. It provides two commands and one hook, and carries two to four lowercase tags.
- **`speckit.checkwright.install`** (`commands/install.md`) installs the release this extension version was tested with, with the recipe applied. It runs the one-line install with `CHECKWRIGHT_VERSION` set to `extension.version` and the arguments `init --profile prose --recipe speckit`, or its `complement` or `full` line's when the user's input asks for that tier, naming the tier's costs before running it, then runs the commands `init` printed. The user's input reaches the body as `$ARGUMENTS` and the command's `argument-hint` reaches the rendered skill, so one command offers every tier.
- **`speckit.checkwright.check`** (`commands/check.md`) runs the battery and reports its verdict, each red with its finding and its `help:` line. It fixes nothing unasked.
- **The `after_implement` hook** is optional and offers the check.
- **`README.md`** says what the extension adds, how to install it and where the landing page is, which the catalog requires of a listed extension. Its description line in `extension.yml` stays under the catalog's 200 characters.

Each command is an agent instruction. Spec Kit hands the command bodies and the hook to the agent and executes neither, and its claude integration renders the commands as skills.

**The version is stamped at pack time.** The tracked `extension.yml` carries `version: "0.0.0"`, as `installer/package.json` does, and the pack step writes the tag's version into the packed copy (§Packing the extension). A tracked version would be a second version line to bump by hand. The install command reads the stamped value, so a listing installs the kits it was tested with.

## Packing the extension

`scripts/ci-pack-extension.sh <version> <out-dir>` builds `checkwright-companion-<version>.zip` and its `.sha256` sidecar:

- it extracts `companion/speckit/`'s tracked files at `HEAD` with `git archive` into a scratch `checkwright-companion-<version>/`, less `recipe/`, which the package carries instead, adding the repository's `LICENSE`;
- it rewrites the one `version: "0.0.0"` line to `<version>`, and refuses when that line is absent or occurs twice;
- it zips the directory with `python3 -m zipfile -c`, so one top-level directory holds `extension.yml`, and writes the sidecar in the `<hex>  <name>` form the other assets use.

It probes `git`, `python3`, `tar` and a hasher before any step, refusing by name, and its scratch lives outside the worktree. `publish.yml`'s `pack` job runs it with the tag's version into the directory the `release` job attaches, and the asset name is declared in [gate-sdk/SPEC.md §Consumer payload](../gate-sdk/SPEC.md#consumer-payload), which `check-release-assets` holds at the tag.

A Spec Kit archive install looks for `extension.yml` at the archive's root or inside exactly one top-level directory. The asset takes the second shape. A GitHub tag archive would wrap the whole repository in its top-level directory, where no `extension.yml` sits.

## The toolkit legs

The toolkits' own tools are oracles, and nothing here re-implements their schemas. The `companion-toolkits` job in `.github/workflows/gates.yml` reads `toolkits.list` and:

1. installs the pinned `specify-cli`, runs `specify init --here --non-interactive --integration claude --script sh --ignore-agent-tools` in an empty scratch directory, since the non-interactive init refuses a non-empty one and a runner carrying no `claude` CLI, and packs the extension at version `0.0.0`. It serves the zip from a local web server and installs it with `specify extension add checkwright --from <url>`, answering the trust prompt. It asserts exit 0, `.specify/extensions/checkwright/extension.yml` present, and the hook registered in `.specify/extensions.yml`;
2. asserts that `extension.yml`'s `requires.speckit_version` is `>=` the pinned version;
3. runs each `native.list` line of a toolkit with a layout, with the toolkit's pinned package, over a copy of the layout and again with each `full` overlay copied over it;
4. for each `fixtures/<toolkit>/overlap/<gate>/`, runs the toolkit's own check over the layout with the defect planted, and asserts that it passes.

A pin moves by editing its `toolkits.list` line, and the job then proves the new version on the next push.

## The support table

`docs/spec-toolkits.md` carries a table with one row per check and one column per toolkit, between `<!-- support-table:begin -->` and `<!-- support-table:end -->` with a blank line inside each marker. `bash gate-sdk/bin/run-gates.sh --emit support-table` prints the block, and `--write` rewrites it and touches nothing else. A row is a tested fact, so it derives from the fixture tree and `native.list`, never from the recipe files: one recipe line arms many gates, while a defect directory proves one.

- **The header** is `| Check |`, then each `toolkits.list` display name in file order.
- **A gate row** is one per gate name under any toolkit's `fixtures/<toolkit>/{defects,full,overlap}/`, sorted by name, the name in a code span. The directory is the tier. A toolkit's cell reads `prose` where its `defects/<gate>/` exists, since the default line arms and tests it and each tier above it only adds. Otherwise it reads `full` where its `full/<gate>/` exists, `opt-in` where its `overlap/<gate>/` exists (§The tiers), and `—` elsewhere.
- **A toolkit row** follows the gate rows, one per `native.list` line in file order: the line's `<toolkit> <arguments…>` in a code span, `toolkit` in that toolkit's cell and `—` in every other.
- **Exit 2**, from the arm and from its gate, on a roster it cannot read: an unreadable `toolkits.list` or `native.list`, a `toolkits.list` line with no name, or a `native.list` line naming a toolkit `toolkits.list` lacks or carrying no arguments. It also exits 2 on a fixture toolkit directory `toolkits.list` lacks, and on a block that is absent or occurs twice.

`check-support-table-fresh` compares the page's block with the arm's rendering in process, one finding for a stale block, printing the regen command. Its positional form is `check-support-table-fresh [companion-dir page]`, so a fixture tree stands in.

## Honest limits

- The Spec Kit fixture's fidelity rests on the pinned templates' structure, since Spec Kit ships no validator.
- The agent-run parts of the extension, its command bodies and its hook, are exercised by no harness. The toolkit leg proves the archive installs, and the companion arm runs each of the install command's `sh` lines, but no run follows an agent through either command.
- The Windows route is documented and not run. A recipe adds no step of its own to it, and a leg piping the PowerShell line would retire this limit.
- A pinned toolkit version says nothing about the next one.
- The exclusion record is measured at each toolkit's pin, from what its init writes and the commands it ships. A toolkit release that adds or drops a workflow surface is seen only when a pin move measures it again.
- No overlap has shipped, so neither the companion arm nor the toolkit leg yet loops over `fixtures/<toolkit>/overlap/`; the first overlap adds both loops.
- A change whose two in-flight deltas sit under one capability in two changes fires the OpenSpec layer's audit, since each delta's directory is a component. So does a delta citing its own capability's spec, which sits outside the delta's directory. The align waiver is the valve. On Spec Kit the audit trigger is not machine-held.
