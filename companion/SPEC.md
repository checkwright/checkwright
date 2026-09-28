# Checkwright companion — design record

The companion package puts Checkwright's gates over a repository whose specs another toolkit writes. It holds, for each supported spec-authoring toolkit, a **recipe** that fits the `prose` and `full` profiles to that toolkit's tree, and a fixture tree that proves the recipe. The package carries the recipes, and `init --recipe <name>` applies one ([installer/SPEC.md §Payload recipes](../installer/SPEC.md#payload-recipes)). For Spec Kit it also holds a catalog extension that installs Checkwright and applies the recipe. `companion/README.md` is the usage tier in front of this file.

## The component

`companion/` is repo-root-governed and not a kit. It carries no `checks/` and no `smoke/`, so no kit-root resolver admits it. The package carries its recipes under `recipes/`, named in this repository's `GATE_SDK_PAYLOAD_RECIPES`: `openspec`, `openspec-lifecycle` and `speckit`. It adds no kit mechanism: each recipe is consumer config in knobs the `prose` profile reads, and the lifecycle layer is config in knobs lifecycle-kit reads. The toolkits' names live here, on the docs pages and on the front door, and never in a kit SPEC or a kit literal ([gate-sdk/SPEC.md §The provenance seam](../gate-sdk/SPEC.md#the-provenance-seam)).

- `toolkits.list` pins each toolkit, one `<toolkit> <package> <version>` line each. The toolkit name keys the line, the package is what the toolkit leg installs, and the version is what it pins and asserts (§The toolkit legs).
- `<toolkit>/recipe/` holds a recipe (§Recipes).
- `openspec/lifecycle/` holds OpenSpec's lifecycle layer (§The lifecycle layer).
- `speckit/` is the Spec Kit extension around its recipe (§The Spec Kit extension). The directory is not named after the toolkit's repository, and a page links that repository at its root only, because `check-kit-ref-liveness` reds a `<name>-kit` path segment that names no kit root.
- `fixtures/<toolkit>/` holds the fixture tree and its planted defects (§The fixtures).

## Recipes

A recipe is a directory in installer/SPEC.md §Payload recipes' format, and that section owns how `init` applies one. Each file opens with a `#` comment citing this file.

**Each line answers a red.** A line is in a recipe because a fresh tree of the toolkit at its pin, with the `prose` or `full` profile installed, reds a gate on one of the toolkit's own idioms until the line is there. The companion arm of the consumer smoke holds that both ways: a line it shows unnecessary is deleted, and one it shows missing is added, each with its idiom recorded below.

### Applying a recipe

An adopter applies a recipe with the install itself: `init --profile prose --recipe <name>`, through the one-line install or any other route to `init`. The install makes one commit with the recipe applied, and the manifest records the recipe, so an upgrade re-applies it with the release it was tested against. Each toolkit's documented line sits between `<!-- companion-install:begin -->` and `<!-- companion-install:end -->` as one fence holding one line. For OpenSpec it is on the landing page, a `text` fence reading `checkwright init …`, where `checkwright` stands for the reader's install line (docs/install.md §Managing's convention), so the page points to the install page rather than restating it. For Spec Kit it is in `speckit/commands/install.md`, an `sh` fence holding the one-line install the agent runs. Each toolkit's `full` line takes a marker pair of its own (§The two tiers). The companion arm runs each line's words from `init` to the line's end, so the documented steps are the tested steps.

**A flag, where this section once refused one.** The refusal's ground was that a flag would make a recipe `init`'s file, rewritten on the next run, unless the manifest recorded the recipe as well. It records it now. Its second ground, a script's PowerShell twin, does not reach a flag that runs behind the invoke. So the one-line install carries a recipe on every host, with no `sh` block to paste and no tag to type.

### The two tiers

Each toolkit has two install lines. The default, `prose`, puts canon-kit's document gates over the toolkit's tree, owes no `bash`, and seeds nothing beside the toolkit's own files. The second, `full`, installs every kit, so every gate a `full` install registers runs over the tree. It costs two things, and the toolkit's page names both. It needs `bash` 4.3 or later, since `full` carries kits that ship bash files (docs/install.md §Requirements). It also seeds Checkwright's own workflow surfaces beside the toolkit's: a task queue, a doctrine block in the agent file, and evidence files under `.workflow/`. Moving from `prose` to `full` only adds (installer/SPEC.md §Profiles), so an adopter who installed the default reaches `full` by running `init` with the second line.

The documented `full` line sits between `<!-- companion-full:begin -->` and `<!-- companion-full:end -->` on each toolkit's page, in the same `text` form as the default line. On OpenSpec it also applies the lifecycle layer, which no gate `init` registers reads until a stage session runs (§The lifecycle layer). The Spec Kit extension installs the default only.

Neither tier re-checks what a toolkit owns: OpenSpec's validator and Spec Kit's templates stay the toolkit's.

### The Spec Kit recipe

Measured on `specify init --here --non-interactive --integration claude --script sh`, which writes `.specify/` and the agent's skills, with one feature written under `specs/`:

- `GATE_SDK_PRUNE_EXTRA_DIRS = .specify`, in `gate-sdk-config.knobs`. Spec Kit's own scripts under `.specify/scripts/bash/` red `check-comment-tier`, since their comments carry no tier tag, and `check-path-dialect`, on their absoluteness tests. They are the toolkit's files and not the adopter's.
- `CANON_KIT_PROSE_SURFACE_GLOBS[] = specs/**/*.md`, in `canon-config.knobs`. The feature specs join the governed doc set. Without the line no claimed gate reads `specs/`, so every planted defect passes.
- `CANON_KIT_FENCE_PROGRAMS_EXTRA[] = Task:`, in `canon-config.knobs`. The tasks template's parallel example is a `bash` fence of `Task:` lines, which `check-fence-command-head` reds as naming nothing that runs. The line admits `Task:` as a command head. The colon is part of the word. The gate's other remedy, another info string, would edit Spec Kit's template text, which no recipe edits.
- `unregister.list` names no gate.

### The OpenSpec recipe

Measured on `openspec init --tools claude --no-animation --no-copilot-cloud .`, with a capability spec and one change:

- `CANON_KIT_PROSE_SURFACE_GLOBS[] = openspec/**/*.md`, in `canon-config.knobs`, for the reason the Spec Kit line has.
- `CANON_KIT_TEMPORAL_EXEMPT_PATHS[] = openspec/changes/**`, in the same file. A MODIFIED requirement in a change delta carries a `(Previously: …)` line, the form OpenSpec's own docs show, and `check-manifest-temporal` reds on it. Change deltas and the archive under `openspec/changes/` are history by design.
- `unregister.list` names no gate.

**One convention, and no dropped gate for it.** OpenSpec specs often repeat a scenario title under two requirements, which `check-spec-pointer`'s one-title-per-file rule reds. A citation of the second title binds to the first, so the rule holds for any spec that is cited. The recipe keeps the gate, and the landing page states the convention: a title is unique within its spec.

### The lifecycle layer

OpenSpec's `full` line (§The two tiers) applies the layer, as `--recipe openspec-lifecycle` beside the recipe. The layer binds the knobs that let `check-stage-entry` see the toolkit's in-flight work as amendments, so a cross-component build entry owes an align stamp. The directory is not named `recipe/`, so the companion arm's toolkit roster, read from `*/recipe/`, does not take it for a third toolkit.

- **OpenSpec.** `openspec/lifecycle/` binds the amendment glob to in-flight change deltas by path, the roster basename to `spec.md` and the contract token to `spec.md`. A change touching two capabilities fires the audit, and so does a delta citing another capability's spec. An archived change sits one level deeper, under `openspec/changes/archive/`, and is no amendment. A queue entry's `[spec:]` ref names the change's `proposal.md` by repo-relative path.
- **Spec Kit: no layer.** A feature's directory persists after it ships and the current feature is the branch's name, so nothing on disk marks a spec as in flight. A glob over `specs/*/spec.md` would demand the audit at every build entry once two features exist. `check-stage-entry` therefore sees no amendment in a Spec Kit tree, and the align trigger there is the authoring stage's own recommendation.

`check-spec-pointer`'s reach needs no layer. Both toolkits' specs are markdown, and each recipe already brings them into the governed set.

## The tested claim

The recipes are proved against four defect classes per toolkit, each caught by a named gate, on `prose` for both toolkits and on `full` for Spec Kit, whose `full` leg plants them again:

- a broken relative or anchored link, by `check-md-refs`;
- a dangling section citation, or a title carried twice in one spec, by `check-spec-pointer`;
- an unclosed fence, by `check-spec-fence-balance`;
- a documented command that invokes a missing script, by `check-docs-cmd`.

The OpenSpec `full` line is held green by the lifecycle leg.

No recipe drops a gate. A toolkit idiom a gate reds is answered by a knob line fitting the gate to it. Dropping a gate would narrow what the companion ships, so it is a change to this section and not a recipe edit.

**Two citation forms pass unchecked.** `check-spec-pointer` reads a bare `<path>.md §<heading>` citation's path as repo-relative, so a file-relative `spec.md §…` is skipped. A bare `§Requirement: <name>` citation of a missing requirement passes through the gate's lead-clause rule. The fixtures cite repo-relatively and cite requirements by anchored link, which `check-md-refs` resolves.

## The fixtures

`fixtures/<toolkit>/layout/` is a tree in the toolkit's layout at its pin, authored here. Its section structure follows the toolkit's own templates, and no template text is copied, so the tree tests the same gates and ships no third-party text. It carries every idiom a recipe line answers, so a dropped line reds the arm's green leg or lets a planted defect pass. It also carries cross-file links and at least one repo-relative section citation that resolves.

- **Spec Kit:** `.specify/memory/constitution.md`; a `.specify/scripts/bash/` script with ordinary comments and a `/*` absoluteness test; and one feature under `specs/001-release-notes/` with `spec.md`, `plan.md` and `tasks.md`, whose `tasks.md` carries the `bash` fence of `Task:` lines.
- **OpenSpec:** `openspec/config.yaml`; one capability spec under `openspec/specs/`; and one change under `openspec/changes/` with a proposal, tasks and a delta whose MODIFIED requirement carries a `(Previously: …)` line. A lifecycle overlay under `fixtures/openspec/lifecycle/` adds a second capability to that change, at a build cursor with no align stamp.

`fixtures/<toolkit>/defects/<gate>/` holds, per claimed gate, the files that replace their layout counterparts to plant that gate's defect. OpenSpec's `check-spec-pointer` defect is a scenario title carried twice in one spec, and its `check-md-refs` defect adds an anchored link to a missing requirement.

The OpenSpec layout passes `openspec validate --all --strict` at the pin, with and without the lifecycle overlay, so the toolkit is its own fixture's oracle. This repository prunes every directory named `fixtures` from its own walks (`GATE_SDK_PRUNE_EXTRA_DIRS` in `scripts/gate-sdk-config.knobs`), since the trees carry each red on purpose. The companion arm governs them instead, inside a scratch consumer ([installer/SPEC.md §The consumer smoke](../installer/SPEC.md#the-consumer-smoke)).

## The Spec Kit extension

`speckit/` is a Spec Kit extension: `extension.yml`, two command files and a `README.md`.

- **`extension.yml`** declares `schema_version: "1.0"`, the extension's identity (`id: checkwright`), `requires.speckit_version` at `>=` the `speckit` pin in `toolkits.list` and git as a required tool. It provides two commands and one hook, and carries two to four lowercase tags.
- **`speckit.checkwright.install`** (`commands/install.md`) installs the release this extension version was tested with, with the recipe applied. It runs the one-line install with `CHECKWRIGHT_VERSION` set to `extension.version` and the arguments `init --profile prose --recipe speckit`, then runs the commands `init` printed.
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
3. runs the pinned `openspec validate --all --strict` inside a copy of `fixtures/openspec/layout/`, then again with the lifecycle overlay's `openspec/` copied over it.

A pin moves by editing its `toolkits.list` line, and the job then proves the new version on the next push.

## Honest limits

- The Spec Kit fixture's fidelity rests on the pinned templates' structure, since Spec Kit ships no validator.
- The agent-run parts of the extension, its command bodies and its hook, are exercised by no harness. The toolkit leg proves the archive installs, and the companion arm runs the install command's install line, but no run follows an agent through either command.
- The Windows route is documented and not run. A recipe adds no step of its own to it, and a leg piping the PowerShell line would retire this limit.
- A pinned toolkit version says nothing about the next one.
- A change whose two in-flight deltas sit under one capability in two changes fires the OpenSpec layer's audit, since each delta's directory is a component. So does a delta citing its own capability's spec, which sits outside the delta's directory. The align waiver is the valve. On Spec Kit the audit trigger is not machine-held.
