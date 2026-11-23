# SPEC amendment: binary-only-recipe

Pairs `binary-only-ci-recipe` and `meta-gate-ownership-scope`. An adopter running gates from the release binary alone has no documented route. Its CI container holds a read-only checkout and no vendored kit, and it must re-derive which forms run, which write, and which read files it does not own. This amendment adds one docs page holding the recipe. It rules out a registry-free `--run`, since an out-of-tree gates directory already keeps the SARIF verdict, and it names the ownership scope the meta-gates already have, with no new knob.

**What the tree does today**, measured on scratch repositories holding no vendored kit, the binary run under `env -i` with `PATH` and `HOME` alone:

- **The direct form runs a compiled-in gate with no registry.** `checkwright-gates <gate-name>` runs `check-spec-dod-singleton`, `check-amendment-update-target`, `check-amendment-retired-spelling` and `check-spec-fence-balance` with no `gates.list`, no descriptor and no knob file. It writes nothing, since `git status --porcelain --untracked-files=all --ignored` stays empty, and each still runs on a tree made read-only with `chmod -R a-w`. It prints no SARIF.
- **Some gates need more than the binary.** `check-amendment-retired-spelling` exits 2 outside a git worktree. `check-amendment-queue` exits 2 on a tree with no `TASK-QUEUE.md`.
- **`--run` needs a registry, and the registry may sit outside the tree.** `--run` with no gates directory exits 2. `--run --only <name>` of a name no `gates.list` registers exits 2. `--run --gates-dir <dir>`, with `<dir>` outside the checkout holding only a `gates.list`, fails each member as unresolved. Copy each member's `.gate` descriptor beside it and the run passes. It writes its SARIF log to an absolute `GATE_SDK_SARIF_FILE` and its scratch to an absolute `GATE_SDK_TMP_DIR`, and leaves the checkout's status empty.
- **A knob file can sit outside the tree too.** `CANON_KIT_KNOB_FILE` naming an absolute path outside the checkout is read. One naming a missing path exits 2 (§The knob file, the tracked layer's locator).
- **A top-level directory holding `checks/` or `smoke/` reads as a kit root.** `--emit-kit-roots` lists `tools` for a tree holding `tools/checks/`, unless `GATE_SDK_KIT_DIRS` is set (§Layout and configuration).
- **The meta-gates an adopter fears are already opt-in.** Every gate reading CI workflows or commit messages by default declares `# install: on-surface`. The comment-surface gates reach an actions-shaped YAML file only under `CANON_KIT_COMMENT_ACTIONS=on`, which defaults `off` (canon-kit/SPEC.md §Layout and configuration). The default-reading gates are the six `check-action-*` gates gate-sdk ships, `check-workflow-tiering`, `check-commit-msg`, `check-commit-subject`, `check-stamp-subject` and `check-dispatch-entry` (probe: `grep -H "install:" gate-sdk/checks/check-action-*.gate gate-sdk/checks/check-commit-*.gate gate-sdk/checks/check-workflow-tiering.gate lifecycle-kit/checks/check-stamp-subject.gate lifecycle-kit/checks/check-dispatch-entry.gate`), so `init` registers none of them. The action gates scan from their root positional: `checkwright-gates check-action-pinning installer` scans `installer` alone. The commit gates read the message file the `commit-msg` hook hands them, and with no argument they print a clean skip and exit 0.
- **No page carries the route.** Neither `docs/orchestration.md` nor `docs/choosing.md` names it, and `docs/install.md`'s children cover the installed tree, the manual steps and maintenance.

## What changes

### (1) A binary-only CI recipe page {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record and the paired entry's deliverable: a binary-only CI recipe page linked from both pages}

A new page, `docs/binary-only.md`, front matter `title: Binary-only CI`, `nav_parent: install`, `nav_child_order: 4`, and a `description:` naming a read-only CI container with no vendored kit. Its H1 is *Running gates from the binary alone*. It is a child of the install page because its reader is installing, and the page divides that subject with its siblings rather than restating them. **Not yet applied.**

The page's sections, each stating its fact once and linking its owner:

- **When this page applies.** A CI job holding the release binary and a checkout, with no `init` and no vendored kit. A tree `init` installed uses the CI action instead (installer/SPEC.md §The CI action), and the page links it.
- **Getting the binary.** The Release tarball, verified as the install page says, and the binary at `payload/artifact/<target>/` inside it (installer/SPEC.md §Layout). The page links the install page and restates no digest step.
- **Running one gate.** `checkwright-gates <gate-name>` from the checkout root. Exit 0 is clean, 1 a finding, and 2 a gate that could not run. It writes nothing and runs on a read-only tree. The gates that need no kit and no knob are named as the four above, with what each checks linked to its SPEC section. A gate taking a root positional scans that directory alone.
- **Running a set with a SARIF verdict.** A gates directory outside the checkout holding a `gates.list` and each member's `.gate` descriptor, copied from the tarball's `payload/<kit>/checks/`. The run is `checkwright-gates --run --gates-dir <dir>`, with `GATE_SDK_SARIF_FILE` and `GATE_SDK_TMP_DIR` absolute and outside the checkout. The section says why the descriptor is needed (delta 2) and that the run then leaves the checkout clean.
- **Configuring a gate without writing to the tree.** `<KIT>_KNOB_FILE` naming the knob file, whether a tracked file of the read-only checkout or one outside it, and a missing one exits 2. `GATE_SDK_KIT_DIRS` set when a top-level directory holds `checks/` or `smoke/` that is no kit.
- **What the container must provide.** git, and a git worktree for `check-amendment-retired-spelling`. A checkout owned by another user than the job's is refused by git as dubious ownership, and the remedy is git's own, `safe.directory`, linked to git's documentation. This one is git's documented behavior, not measured here. Don't register `check-amendment-queue` on a tree with no `TASK-QUEUE.md`.
- **Which files a gate reads.** Delta 3's scope, in the reader's words: a gate that reads workflows or commit messages is opt-in, and registering it is the adopter's choice. The action gates take the directory to scan, and the commit gates read only the message being committed.

The two linking pages each gain one sentence at their first mention of CI, linking the new page. On `docs/orchestration.md` that sentence goes under *Where to go next*. On `docs/choosing.md` it goes under *What you install*. Neither page restates the recipe. The install page's *Going further* list links each of its children, so it gains one bullet for this one.

**Its oracle is a fixture, not a fence.** The page's commands are not marked `fence-runnable`, because a fence runs in a scratch holding no release tarball to copy descriptors from. Instead `gate-sdk/gate-tests/binary-only-recipe.test.sh` runs both of the page's forms on a scratch tree with no kit, made read-only for the direct form. It asserts the exit codes, the SARIF log's presence at its absolute path, the checkout's empty status after each form, the root positional's narrowing, and exit 2 for a missing `<KIT>_KNOB_FILE`. A drift in either form reds the fixture, and the page names the fixture as the recipe's oracle.

### (2) A registry-free `--run` is refused {design-bearing} {user-facing: the paired entry's deliverable, which delegates the choice to this stage (operator direction 2026-11-24, lead-relayed, not a ruling): rule whether a registry-free `--run` over compiled-in gates is needed for the SARIF verdict to survive}

`--run` keeps requiring a registry, and §run-gates gains a paragraph saying why. **Not yet applied:**

> **A tree with no vendored kit still runs `--run` through a registry, which may sit outside it.** A gates directory outside the checkout, holding a `gates.list` and each member's `.gate` descriptor, is a registry like any other: `--run --gates-dir <dir>` resolves each member there, and with `GATE_SDK_SARIF_FILE` and `GATE_SDK_TMP_DIR` absolute and outside the checkout, the run writes nothing into it. So a read-only container gets the SARIF verdict without a registry-free mode. The descriptor is required rather than inferred from the compiled-in name, because it is where a member's `# graph:` manifest, `# install:` disposition and `# armed-by:` knob live, and a member resolved without one would carry none of them. A second, name-only resolution path is also the shape §Layout and configuration's single resolution order exists to prevent. A run of one gate without a registry is the direct form, `checkwright-gates <gate-name>`, which prints no SARIF. The binary-only recipe page (`docs/binary-only.md`) carries both forms for an adopter, and `gate-tests/binary-only-recipe.test.sh` holds them.

- **Why not the registry-free mode the entry allows.** The SARIF verdict is its whole motive, and the out-of-tree registry delivers that already, as measured. A descriptor-free member would also lose what its descriptor declares, unseen. `doctor` would have no `# armed-by:` to read, so a disarmed member would read as coverage, the defect §The install disposition's arming declaration exists to prevent.
- **The cost the refusal keeps.** An adopter copies one descriptor file per member from the tarball. The page states that cost.

### (3) The meta-gates' ownership scope is registry membership and the scan root {design-bearing} {user-facing: the meta-gate-ownership-scope entry's deliverable, paired into this amendment by this iteration's set record (operator direction 2026-11-24, lead-relayed, not a ruling): an ownership scope for the meta-gates, or the existing knob that already gives one, named}

No new knob. §The install disposition gains a paragraph naming the scope that already exists. **Not yet applied:**

> **A gate reading what an adopter may not own is in scope only where the adopter puts it.** CI workflows and commit messages are such surfaces: a workflow may be copied from upstream, and a commit subject may follow another project's convention. Every gate reading one by default is `on-surface`, so `init` registers none of them. The comment-surface gates read a workflow only under `CANON_KIT_COMMENT_ACTIONS=on`, a switch that defaults off. The adopter scopes a gate in three ways, none a knob. **Membership:** the adopter's `gates.list` registers the gate or leaves it out. **The scan root:** a gate scanning YAML takes its root as its positional (§check-action-pinning), so the direct form `checkwright-gates <gate> <dir>` scans the directory the adopter owns. **The hook surface:** a gate reading a commit message reads only the message file the `commit-msg` hook hands it, the commit being made, and with no argument it skips clean. A path-scope knob over these gates was refused, since each scope above already exists and a knob would be a second spelling of membership.

- **Why no knob.** The entry asks for paths the adopter declares. The registry is that declaration for whole surfaces, and the root positional is that declaration for a directory. A knob listing owned paths would also need its own reader in every gate that walks a tree, which is a per-gate contract for a scope two existing mechanisms already give.
- **The honest limit.** A gate registered through `--run` scans from the tree's root, because the registry carries no per-member operand. An adopter wanting one action gate scoped to a subdirectory inside the battery leaves it unregistered and registers a consumer `.sh` member, under a name of its own, that calls the direct form with that root. The page states it.

## Producers and consumers

- **`docs/binary-only.md`** (new page) — producer: this amendment's build. Consumers: the reader, through the install page's derived child entry; `check-docs-nav-reachable`, which holds its menu slot; the agent index (`docs/llms.txt` and `docs/llms-full.txt`), which it joins by taking a slot and which `--emit llms --write` regenerates; and `check-md-refs`, `check-citation-link`, `check-docs-page-repeat`, `check-docs-render-fidelity` and the page-length gate, each of which reads it through its `docs/*.md` glob (probe: `grep -n "docs/\*\.md" scripts/canon-config.knobs`).
- **The two inbound links** — producers: `docs/orchestration.md` and `docs/choosing.md`. Consumer: `check-md-refs`.
- **`gate-sdk/gate-tests/binary-only-recipe.test.sh`** (new fixture) — producer: this amendment's build. Consumer: the gate-sdk fixture suite (`--run-gate-tests`), which runs each bespoke `*.test.sh` in its directory.
- **The registry-free refusal and the ownership-scope paragraph** — prose in `gate-sdk/SPEC.md`. Consumers: the page that links them, and a reader of either section. No arm or gate behavior changes, so no code reads them.
- **Point 5 (narrowing).** Nothing narrows. No arm, gate, knob or default changes. The page documents measured behavior, and the fixture pins it.
- **Point 6 (every member).** The members are the meta-gates delta 3 names, and each carries `# install: on-surface` (the probe above). Each of the six shipped action gates reads its scan root from its first positional, defaulting to `.`, and refuses a root that is no directory (probe: `grep -n "args.first" native/src/gates/action_*.rs`). The fixture exercises the positional on `check-action-pinning`, and the paragraph names the class rather than each gate.

## Existing sections updated

Roster from `git grep -n -e "binary-only" -e "registry-free" -e "ownership scope" -e "nav_parent: install"` over the tracked tree and `git grep -n -i "CI" -- docs/orchestration.md docs/choosing.md`, the generated mirrors and the queue excluded.

- `gate-sdk/SPEC.md` §run-gates, which gains the out-of-tree registry paragraph (delta 2), and §The install disposition, which gains the ownership-scope paragraph (delta 3).
- `docs/gate-sdk/SPEC.md` — the generated mirror, regenerated by `--emit docs-mirror --write` (deltas 2 and 3).
- `docs/binary-only.md` — new (delta 1).
- `docs/orchestration.md` *Where to go next* and `docs/choosing.md` *What you install* — one linking sentence each (delta 1).
- `docs/install.md` *Going further* — one bullet linking the new child (delta 1).
- `docs/llms.txt` and `docs/llms-full.txt` — regenerated by `--emit llms --write`, since the page takes a menu slot (delta 1).
- `gate-sdk/gate-tests/binary-only-recipe.test.sh` — new (deltas 1, 2 and 3).

## Retired spellings

- None — the page and both paragraphs are additions, and no name is removed.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the page, its two inbound links, the fixture and the two paragraphs.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — each paragraph lands in the section it names; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `binary-only-ci-recipe` and `meta-gate-ownership-scope` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Their oracle is local, the gate-sdk fixture suite and the battery. The page's live render is read by `pages-build-deployment` at close's push, which gates no move.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
