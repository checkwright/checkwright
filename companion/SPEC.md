# Checkwright companion — design record

The companion package puts Checkwright's gates over a repository whose specs another toolkit writes. It holds, for each supported spec-authoring toolkit, a **recipe** that makes the `prose` profile govern that toolkit's tree, and a fixture tree that proves the recipe. For Spec Kit it also holds a catalog extension that installs Checkwright and applies the recipe. `companion/README.md` is the usage tier in front of this file.

## The component

`companion/` is repo-root-governed and not a kit. It carries no `checks/` and no `smoke/`, so no kit-root resolver admits it and no install payload carries it. It adds no kit mechanism: each recipe is consumer config, written in knobs the `prose` profile already reads. The toolkits' names live here, on the docs pages and on the front door, and never in a kit SPEC or a kit literal ([gate-sdk/SPEC.md §The provenance seam](../gate-sdk/SPEC.md#the-provenance-seam)).

- `toolkits.list` pins each toolkit, one `<toolkit> <package> <version>` line each. The toolkit name keys the line, the package is what the toolkit leg installs, and the version is what it pins and asserts (§The toolkit legs).
- `<toolkit>/recipe/` holds a recipe (§Recipes).
- `speckit/` is the Spec Kit extension around its recipe (§The Spec Kit extension). The directory is not named after the toolkit's repository, and a page links that repository at its root only, because `check-kit-ref-liveness` reds a `<name>-kit` path segment that names no kit root.
- `fixtures/<toolkit>/` holds the fixture tree and its planted defects (§The fixtures).

## Recipes

A recipe is three files and one procedure. `gate-sdk-config.knobs` and `canon-config.knobs` hold lines appended to the consumer's knob files of the same names. `unregister.list` names gates dropped from the consumer's `gates.list`. A recipe needs no file it would leave empty, except `unregister.list`, which the procedure reads and which then holds its header alone. Each file opens with a `#` comment citing this file, and both the append and the drop skip `#` lines.

**Each line answers a red.** A line is in a recipe because a fresh tree of the toolkit at its pin, with the `prose` profile installed, reds a gate on one of the toolkit's own idioms until the line is there. The companion arm of the consumer smoke holds that both ways: a line it shows unnecessary is deleted, and one it shows missing is added, each with its idiom recorded below.

### The procedure

The procedure is one POSIX `sh` block, run from the repository root after `init`, with `recipe` set to the recipe directory. It runs in a subshell under `set -e`, so a failed step stops it before the commit. It:

1. appends each recipe `*.knobs` file, minus its `#` lines, to the gates directory's file of the same name;
2. drops from `scripts/gates.list` every gate `unregister.list` names;
3. regenerates the hooks and the coupling graph through the installed binary, with `--emit git-hooks --write` and `--emit graph` into `scripts/CHECK-GRAPH.html`, the graph artifact's default path, since `check-graph` reds on a knob or roster change until both are regenerated;
4. runs the battery;
5. commits the gates directory.

It needs no `bash`, so it runs under `/bin/sh` on macOS and Linux and under Git for Windows' `sh`. The landing page, `docs/spec-toolkits.md`, carries it between `<!-- companion-recipe:begin -->` and `<!-- companion-recipe:end -->`. `speckit/commands/install.md` carries the same block under the same markers, opened by the line `recipe=.specify/extensions/checkwright/recipe`. The companion arm runs both copies and holds them equal apart from that line, so the documented steps are the tested steps.

**The procedure is a block, not a flag or a script.** An `init --recipe` flag would make a recipe `init`'s file, rewritten on its next run unless the manifest recorded the recipe as well. A shipped script would owe a PowerShell twin under the interpreter policy, and the block runs under a shell every supported host already has.

### The Spec Kit recipe

Measured on `specify init --here --non-interactive --integration claude --script sh`, which writes `.specify/` and the agent's skills, with one feature written under `specs/`:

- `GATE_SDK_PRUNE_EXTRA_DIRS = .specify`, in `gate-sdk-config.knobs`. Spec Kit's own scripts under `.specify/scripts/bash/` red `check-comment-tier`, since their comments carry no tier tag, and `check-path-dialect`, on their absoluteness tests. They are the toolkit's files and not the adopter's.
- `CANON_KIT_PROSE_SURFACE_GLOBS[] = specs/**/*.md`, in `canon-config.knobs`. The feature specs join the governed doc set. Without the line no claimed gate reads `specs/`, so every planted defect passes.
- `check-fence-command-head`, in `unregister.list`. The tasks template's parallel example is a `bash` fence of `Task:` lines, which no shell runs and the gate reds.

### The OpenSpec recipe

Measured on `openspec init --tools claude --no-animation --no-copilot-cloud .`, with a capability spec and one change:

- `CANON_KIT_PROSE_SURFACE_GLOBS[] = openspec/**/*.md`, in `canon-config.knobs`, for the reason the Spec Kit line has.
- `CANON_KIT_TEMPORAL_EXEMPT_PATHS[] = openspec/changes/**`, in the same file. A MODIFIED requirement in a change delta carries a `(Previously: …)` line, the form OpenSpec's own docs show, and `check-manifest-temporal` reds on it. Change deltas and the archive under `openspec/changes/` are history by design.
- `unregister.list` names no gate.

**One convention, and no dropped gate for it.** OpenSpec specs often repeat a scenario title under two requirements, which `check-spec-pointer`'s one-title-per-file rule reds. A citation of the second title binds to the first, so the rule holds for any spec that is cited. The recipe keeps the gate, and the landing page states the convention: a title is unique within its spec.

## The tested claim

The recipes are proved against four defect classes per toolkit, each caught by a named gate:

- a broken relative or anchored link, by `check-md-refs`;
- a dangling section citation, or a title carried twice in one spec, by `check-spec-pointer`;
- an unclosed fence, by `check-spec-fence-balance`;
- a documented command that invokes a missing script, by `check-docs-cmd`.

A recipe may drop a gate only where a toolkit idiom reds it. Dropping one of these four narrows the claim, so it is a change to this section and not a recipe edit.

**Two citation forms pass unchecked.** `check-spec-pointer` reads a bare `<path>.md §<heading>` citation's path as repo-relative, so a file-relative `spec.md §…` is skipped. A bare `§Requirement: <name>` citation of a missing requirement passes through the gate's lead-clause rule. The fixtures cite repo-relatively and cite requirements by anchored link, which `check-md-refs` resolves.

## The fixtures

`fixtures/<toolkit>/layout/` is a tree in the toolkit's layout at its pin, authored here. Its section structure follows the toolkit's own templates, and no template text is copied, so the tree tests the same gates and ships no third-party text. It carries every idiom a recipe line answers, so a dropped line reds the arm's green leg or lets a planted defect pass. It also carries cross-file links and at least one repo-relative section citation that resolves.

- **Spec Kit:** `.specify/memory/constitution.md`; a `.specify/scripts/bash/` script with ordinary comments and a `/*` absoluteness test; and one feature under `specs/001-release-notes/` with `spec.md`, `plan.md` and `tasks.md`, whose `tasks.md` carries the `bash` fence of `Task:` lines.
- **OpenSpec:** `openspec/config.yaml`; one capability spec under `openspec/specs/`; and one change under `openspec/changes/` with a proposal, tasks and a delta whose MODIFIED requirement carries a `(Previously: …)` line.

`fixtures/<toolkit>/defects/<gate>/` holds, per claimed gate, the files that replace their layout counterparts to plant that gate's defect. OpenSpec's `check-spec-pointer` defect is a scenario title carried twice in one spec, and its `check-md-refs` defect adds an anchored link to a missing requirement.

The OpenSpec layout passes `openspec validate --all --strict` at the pin, so the toolkit is its own fixture's oracle. This repository prunes every directory named `fixtures` from its own walks (`GATE_SDK_PRUNE_EXTRA_DIRS` in `scripts/gate-sdk-config.knobs`), since the trees carry each red on purpose. The companion arm governs them instead, inside a scratch consumer ([installer/SPEC.md §The consumer smoke](../installer/SPEC.md#the-consumer-smoke)).

## The Spec Kit extension

`speckit/` is a Spec Kit extension: `extension.yml`, two command files, a `README.md` and the recipe.

- **`extension.yml`** declares `schema_version: "1.0"`, the extension's identity (`id: checkwright`), `requires.speckit_version` at `>=` the `speckit` pin in `toolkits.list` and git as a required tool. It provides two commands and one hook, and carries two to four lowercase tags.
- **`speckit.checkwright.install`** (`commands/install.md`) installs the release this extension version was tested with. It runs the one-line install with `CHECKWRIGHT_VERSION` set to `extension.version`, spelled with the verb, `init --profile prose`, since the line runs `init` only when given no argument. It then runs the commands `init` printed and the recipe block.
- **`speckit.checkwright.check`** (`commands/check.md`) runs the battery and reports its verdict, each red with its finding and its `help:` line. It fixes nothing unasked.
- **The `after_implement` hook** is optional and offers the check.
- **`README.md`** says what the extension adds, how to install it and where the landing page is, which the catalog requires of a listed extension. Its description line in `extension.yml` stays under the catalog's 200 characters.

Each command is an agent instruction. Spec Kit hands the command bodies and the hook to the agent and executes neither, and its claude integration renders the commands as skills.

**The version is stamped at pack time.** The tracked `extension.yml` carries `version: "0.0.0"`, as `installer/package.json` does, and the pack step writes the tag's version into the packed copy (§Packing the extension). A tracked version would be a second version line to bump by hand. The install command reads the stamped value, so a listing installs the kits it was tested with.

## Packing the extension

`scripts/ci-pack-extension.sh <version> <out-dir>` builds `checkwright-companion-<version>.zip` and its `.sha256` sidecar:

- it extracts `companion/speckit/`'s tracked files at `HEAD` with `git archive` into a scratch `checkwright-companion-<version>/`, adding the repository's `LICENSE`;
- it rewrites the one `version: "0.0.0"` line to `<version>`, and refuses when that line is absent or occurs twice;
- it zips the directory with `python3 -m zipfile -c`, so one top-level directory holds `extension.yml`, and writes the sidecar in the `<hex>  <name>` form the other assets use.

It probes `git`, `python3`, `tar` and a hasher before any step, refusing by name, and its scratch lives outside the worktree. `publish.yml`'s `pack` job runs it with the tag's version into the directory the `release` job attaches, and the asset name is declared in [gate-sdk/SPEC.md §Consumer payload](../gate-sdk/SPEC.md#consumer-payload), which `check-release-assets` holds at the tag.

A Spec Kit archive install looks for `extension.yml` at the archive's root or inside exactly one top-level directory. The asset takes the second shape. A GitHub tag archive would wrap the whole repository in its top-level directory, where no `extension.yml` sits.

## The toolkit legs

The toolkits' own tools are oracles, and nothing here re-implements their schemas. The `companion-toolkits` job in `.github/workflows/gates.yml` reads `toolkits.list` and:

1. installs the pinned `specify-cli`, runs `specify init --here --non-interactive --integration claude --script sh --ignore-agent-tools` in an empty scratch directory, since the non-interactive init refuses a non-empty one and a runner carrying no `claude` CLI, and packs the extension at version `0.0.0`. It serves the zip from a local web server and installs it with `specify extension add checkwright --from <url>`, answering the trust prompt. It asserts exit 0, `.specify/extensions/checkwright/extension.yml` and the recipe present, and the hook registered in `.specify/extensions.yml`;
2. asserts that `extension.yml`'s `requires.speckit_version` is `>=` the pinned version;
3. runs the pinned `openspec validate --all --strict` inside a copy of `fixtures/openspec/layout/`.

A pin moves by editing its `toolkits.list` line, and the job then proves the new version on the next push.

## Honest limits

- The Spec Kit fixture's fidelity rests on the pinned templates' structure, since Spec Kit ships no validator.
- The agent-run parts of the extension, its command bodies and its hook, are exercised by no harness. The toolkit leg proves the archive installs, and the companion arm runs the install command's recipe block, but no run follows an agent through either command.
- The Windows route is documented and not run.
- A pinned toolkit version says nothing about the next one.
