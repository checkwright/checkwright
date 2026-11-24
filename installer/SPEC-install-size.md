# SPEC amendment: install-size

Pairs `adopter-footprint-figure`. An adopter deciding whether to vendor Checkwright into a repository asks what the install adds to it: how many files, how many bytes, and how much of that is test fixtures. No page answers. This amendment publishes the answer on the requirements page in two parts. The kit files are derived from the tree, per profile and per kit, and byte-gated. The gate binary is measured at each release beside the commit-cost figure, because it is a build product no tree derivation can size. Gate-tests stay in the payload, and the figure states their share.

**What the tree does today**, read from `installer/SPEC.md` §init, §The personal install, §The user install, §Profiles, §The packer and §The hosted install pin, `gate-sdk/SPEC.md` §Consumer payload and §Fixture-pair discipline, and `docs/requirements.md`:

- **No surface states an install's size.** The requirements page states what a commit costs in time, and nothing states what an install places (probe: `git grep -n -i -e "install size" -e "install-size"`, which finds none; `grep -n "^##" docs/requirements.md`).
- **A repository install commits three things.** `init` writes the profile's kits as the packer packed them, each kit's tracked files minus `GATE_SDK_PAYLOAD_WITHHOLD` (default `SPEC.md smoke`). It writes the gate binary into the gates directory as a tracked install artifact, and it writes the seeds: `gates.list`, the config seams and the manifest. `init --personal` writes the same files untracked. `init --user` places nothing in the repository (probe: `git grep -n "tracked copy of the binary" -- installer/SPEC.md`; `git grep -n "GATE_SDK_PAYLOAD_WITHHOLD" -- native/src/emit/pack_installer.rs`).
- **The binary is most of it.** Here, `full`'s kit files are about 1,600 files and 1.8 MB, of which gate-tests are about 1,400 files and 1.2 MB. A release build of the binary is about 12.6 MB, one file (probe: `git ls-files` over each kit root minus the withheld shape, summed with `wc -c`; `wc -c` on `native/target/release/checkwright-gates`).
- **The binary's size is not in the tree.** It is built per target by the publish workflow, so a derivation over tracked files cannot size it. The one release-measured figure the install pages carry is the commit-cost block, held to the pin by `check-install-pin` invariant D (probe: `git grep -n "measured at v" -- docs/requirements.md`).
- **Gate-tests stay, by an existing rule.** The fixture pair ships because it is the consumer's whole verification oracle once the source is withheld, and a fixture may not be dropped to slim a payload (gate-sdk/SPEC.md §Fixture-pair discipline, *The pair ships*; §Consumer payload). This amendment discloses the share and changes nothing about what ships (probe: `git grep -n "whole verification oracle" -- gate-sdk/SPEC.md`).

## What changes

### (1) The install-size arm and its freshness gate {design-bearing} {user-facing: feature deliverable on the paired entry, a derived install figure per install shape, published and freshness-gated, and whether gate-tests stay in the payload}

A section `## The install size` is added to `installer/SPEC.md` after §Profiles. **Not yet applied:**

> ## The install size
>
> `docs/requirements.md` states what an install places, in one block between `<!-- install-size:begin -->` and `<!-- install-size:end -->` with a blank line inside each marker. `bash gate-sdk/bin/run-gates.sh --emit install-size` prints the block, and `--write` rewrites it and touches nothing else. The page is `GATE_LOCAL_INSTALL_SIZE_PAGE`.
>
> - **A kit's figure** is what the packer packs for it: the index's tracked files under the kit root, minus the `GATE_SDK_PAYLOAD_WITHHOLD` shape, through the packer's own pathspec, each sized by its blob in the index. **Its gate-tests** are those of its files under `<kit>/gate-tests/`.
> - **The kits** are `walk::kit_roots`, the set the packer packs, and **a profile's figure** sums the kits the profile module resolves for it, `full` included, so a profile row cannot disagree with `init`.
> - **The block** holds two tables. The profile table's header is `| Profile | Kits | Files | Of which gate-tests | Size |`, one row per profile in the profile module's order. The kit table's header is `| Kit | Files | Of which gate-tests | Size |`, one row per kit in name order, so a `--with-kit` or `--without-kit` selection is a sum the reader can make.
> - **Every figure is rounded to two significant figures** after summing. A size is in decimal units, `kB` below 1 MB and `MB` from it, with one decimal place in MB.
> - **Exit 2**, from the arm and from its gate, outside a repository, on a kit root or profile roster the resolvers refuse, on an unreadable page, and on a block that is absent or occurs twice.
>
> `check-install-size-fresh` (this repo's `scripts/`) compares the page's block with the arm's rendering in process. A stale block is one finding, and it prints the regen command. Its positional form is `check-install-size-fresh [page emission]`, so a fixture supplies the emission the index would.
>
> **Honest limits.** The figure is the tree's, so between releases it describes the next release rather than the pinned one. The packer's per-kit additions, a license copy and the carried close-surface roster, and its README link rewrite are not counted, and each is smaller than the rounding.

- **Why derived from the index, not measured from a pack.** The packer's selection is a pathspec over tracked files, so the index answers it with no pack, no scratch and no network, which a pre-commit gate needs. Reading blob sizes from the index rather than the worktree makes the figure the one the next commit carries, whatever is unstaged.
- **Why the tree and not the pinned release.** Sizing the pin would need the kit roots and the profile roster as they were at a past tag, and a tag a shallow CI clone does not hold. The commit-cost figure tolerates the pin because no gate re-derives it.
- **Why two significant figures.** Each fixture a commit adds would move an exact count, and the gate would demand a regen on most kit commits. Two figures move only when the answer to an adopter's question moves.
- **Why both tables.** The profile table answers the question as asked. The kit table makes the figure additive for a selection the profiles do not name, instead of a row per selection.
- **Why gate-tests are a column, not a removal.** §Fixture-pair discipline rules them in, and an adopter weighing the cost needs to see that most of the kit files are the oracle that lets them check the binary.

### (2) The binary's size, measured at release {design-bearing} {user-facing: feature deliverable on the paired entry, the files and bytes an install places}

gate-sdk/SPEC.md §measure-commit's output gains a third line, and installer/SPEC.md §The hosted install pin's invariant D is rephrased over both release-measured blocks. **Not yet applied:**

> ```text
> measure-commit: <median>ms median of 3 (<a>ms, <b>ms, <c>ms) — <m> pre-commit member(s) over <p> tracked path(s), every path selected
> host: <os> <arch>, <n> logical CPU(s)
> binary: <bytes> bytes
> ```
>
> The binary line is the size of the running executable, read from its metadata with no spawn. Run through the installed gate binary, as the release step runs it, that is the artifact `init` placed for this host's target. The line is absent where the platform cannot name the running executable.

> - **Invariant D — the release-measured figures are the pinned release's.** The install pages carry exactly one `<!-- commit-cost:begin -->` … `<!-- commit-cost:end -->` block and exactly one `<!-- install-binary:begin -->` … `<!-- install-binary:end -->` block across their members. Each holds exactly one `measured at v<X.Y.Z>` token, whose version equals the pin. So the commit that moves the pin carries new measurements (RELEASING.md §The procedure, *Tag the stamp commit, then drain the release declaration surface*). **Honest limit:** D reads the version and never the host or the figure, so a figure copied forward under a new version passes.

The `install-binary` block sits in the requirements page's install section beside the derived block, one sentence: on `<os> <arch>`, the gate binary `init` placed is **<size>**, measured at v<X.Y.Z>, the size in the derived block's units. RELEASING.md's step writes it from the same run that writes the commit-cost block.

- **Why measured, and why at release.** The binary is a per-target build product, so no derivation over the tree can size it, and the figure an adopter meets is the released artifact's. The release step already runs the installed binary in a fresh `full` install to measure the commit cost, so the measurement costs one more line of the same run.
- **Why on `--measure-commit` and not a new arm.** The release step's measuring run is that arm's, and the arm already reports the host it ran on. A sibling arm would need its own named caller for one line the release step reads beside the other two.
- **Why its own block, not a clause of the commit-cost sentence.** The commit-cost block is the answer to what a commit costs. The binary is part of what an install places, so it sits with the derived figure under that heading, and D holds both.
- **Refused alternative: a pointer to the Release assets instead of a figure.** It leaves the published figure about seven times smaller than what a repository install commits, while saying nothing false. An adopter reading only the table would be misled.

### (3) The requirements page's install section {mechanical}

`docs/requirements.md` gains `## What an install places`, before `## What a commit costs`: two sentences of hand prose naming the install shapes, then the derived block and the binary block. A repository install commits the profile's kit files, the binary and a handful of seeded config files; a personal install writes the same files untracked; a user install places nothing in the repository. The prose states that gate-tests ship as the consumer's oracle (gate-sdk/SPEC.md §Fixture-pair discipline) and that the binary differs per target.

## Producers and consumers

- **`--emit install-size [--write]`** (new non-gate arm) — producer: `native/src/emit/install_size.rs`, reading `walk::kit_roots`, the packer's pathspec (made crate-visible) and the profile module. Named caller: `check-install-size-fresh`, and the commit that moves a kit's tracked set, which regenerates through `--write`.
- **`check-install-size-fresh`** (new gate) — `scripts/check-install-size-fresh.gate`, `# projection: docs/requirements.md`, `# knob: GATE_LOCAL_INSTALL_SIZE_PAGE = docs/requirements.md`, `couples=` naming the page, `installer/profiles.list`, `native/src/emit/install_size.rs` and the kit roots' files; tier `precommit`. Module `native/src/gates/install_size_fresh.rs`. Fixture pair under `scripts/gate-tests/check-install-size-fresh/`: `good/` a page whose block equals a supplied emission, `bad/` a stale one. The derivation's cases, a scratch repository with a withheld file, an unstaged edit and a `gate-tests/` file, are the arm module's unit tests.
- **`GATE_LOCAL_INSTALL_SIZE_PAGE`** (new knob) — producer: the gate's declaration; consumers: the arm and the gate.
- **The `install-size` block** (new) — producer: the arm; consumers: the gate and an adopter reading the page.
- **`--measure-commit`'s binary line** (new) — producer: the arm; named caller: the release step writing the `install-binary` block.
- **The `install-binary` block** (new) — producer: the release step; consumers: invariant D of `check-install-pin` (`native/src/gates/install_pin.rs`), and an adopter.
- **Point 5 (narrowing).** Invariant D now also refuses, at exit 2, an install page set carrying no `install-binary` block or two, and reds a block measured at another version. That reaches this repo's own `scripts/` gate alone, and the commit landing this amendment carries the block. `--measure-commit`'s first two lines are unchanged; a reader taking exactly two lines meets a third.
- **Point 6 (every member).** The profiles: every profile the module resolves, the four rostered and `full` (probe: `grep -v '^#' installer/profiles.list | cut -f1 | sort -u`), each a row. The install shapes: the repository, personal and user installs (probe: `git grep -n -e "^### The personal install" -e "^### The user install" -- installer/SPEC.md`), each named in the page's prose. The release-measured blocks: commit-cost and install-binary, both under D (probe: `git grep -n "measured at v" -- docs/requirements.md` after the delta, two hits).

## Existing sections updated

Roster from `git grep -n -e "commit-cost" -e "measure-commit" -e "Invariant D" -e "GATE_SDK_PAYLOAD_WITHHOLD"` over the tracked tree, the posts, the queue and the generated mirrors excluded, narrowed to the sites stating a release-measured block, the measuring arm's output, or what the payload carries.

- `installer/SPEC.md` — §The install size, new (delta 1); §The hosted install pin, invariant D (delta 2).
- `gate-sdk/SPEC.md` §measure-commit — the output block and the binary-line paragraph, and its *Named caller* line names the binary line's reader (delta 2).
- `native/src/emit/install_size.rs` — new, with its unit tests (delta 1). `native/src/emit/mod.rs` — the arm's row and its knob roster (delta 1). `native/src/emit/pack_installer.rs` — `pathspec` made crate-visible (delta 1).
- `native/src/gates/install_size_fresh.rs` and `native/src/gates/mod.rs` — new gate and its row (delta 1). `scripts/check-install-size-fresh.gate`, `scripts/gates.list`, `scripts/gate-tests/check-install-size-fresh/{good,bad}/` — new (delta 1).
- `native/src/emit/git_hook.rs` — the measure-commit binary line and its unit test (delta 2).
- `native/src/gates/install_pin.rs` and `scripts/gate-tests/check-install-pin/{good,bad}/` — invariant D over both blocks: `good/` gains an `install-binary` block, `bad/` one measured at a stale version (delta 2).
- `RELEASING.md` §The procedure, *Tag the stamp commit, then drain the release declaration surface* — the measuring run writes the binary line into the `install-binary` block too (delta 2).
- `docs/requirements.md` — `## What an install places`, new (delta 3).
- `docs/install-block-contracts.md` — *The install-size block* and *The install-binary block* sections beside *The commit-cost block*, and the hosted install scripts section's D sentence (deltas 1 and 2).
- `docs/install.md` §Choosing a profile — one sentence pointing to the figure (delta 3).
- `docs/generated-projections.md` — a row for the install-size block with its `<!-- projection: check-install-size-fresh -->` marker (delta 1).
- Generated projections the sections move: `docs/installer/SPEC.md`, `docs/gate-sdk/SPEC.md` and `docs/llms-full.txt`, each regenerated by the command its gate prints on red (docs/generated-projections.md) (all deltas).
- `.workflow/release-declarations.md` — *Behavior changes* gains `--emit install-size` and `--measure-commit`'s binary line (deltas 1 and 2).

## Retired spellings

- None — the arm, the gate, the knob and the blocks are additions, and invariant D's title is rephrased rather than a spelling retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, the gate, the knob, both blocks and the binary line.
- [ ] **Instruction surfaces: instruction only** — the RELEASING step states the action; no template changes.
- [ ] **Merged with no information lost** — §The install size lands in installer/SPEC.md, invariant D is rephrased in place, and the binary line lands in gate-sdk §measure-commit; each merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `adopter-footprint-figure` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Its oracle is local: the unit tests, the fixture suite of this repo's `scripts/` gates, the build and the battery. The merge commit writes the `install-binary` block's first figure at the current pin, from the size of the binary a fresh `full` install at that pin places, since that release predates the binary line. The next release step rewrites it from `--measure-commit`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
