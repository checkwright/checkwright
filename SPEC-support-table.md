# SPEC amendment: support-table

No page states which checks each companion toolkit gets. `docs/spec-toolkits.md` carries prose sections and no table, so a reader weighing a toolkit cannot see what the companion adds without reading both recipes.

This amendment adds a generated support table to that page, one row per check and one column per toolkit. Each cell marks the check as Checkwright's at a tier, as an overlap the adopter opts into, or as the toolkit's own. The rows derive from the companion's fixture tree and one new roster of the toolkit's own checks, which the toolkit leg also runs. A freshness gate holds the block.

One queue entry pairs it: [toolkit-support-table](TASK-QUEUE.md#toolkit-support-table). It lands after `toolkit-overlap-value-bar`, whose value bar (companion/SPEC.md §The two tiers) the `opt-in` and `toolkit` marks cite.

**The rulings.**

- **A row is a tested fact.** The companion's fixture tree already proves which gate catches what on each toolkit, and the consumer smoke's companion arm reds a claimed gate with no defect directory. So a Checkwright row derives from that tree, never from the recipe files. A recipe line such as `CANON_KIT_PROSE_SURFACE_GLOBS` arms many gates at once, so reading gates off recipe knobs would list every gate that reads the knob. Reading them off the fixtures lists what is proven.
- **The tier is the directory.** `fixtures/<toolkit>/defects/<gate>/` is a gate the default `prose` line arms and proves, so its cell reads `prose`. Moving to `full` only adds, so the cell stands for both lines. A gate proven only on the `full` line takes `fixtures/<toolkit>/full/<gate>/`: OpenSpec's lifecycle overlay moves there, as `full/check-stage-entry/`, so its row derives like the others. An overlap's `fixtures/<toolkit>/overlap/<gate>/` (the value bar) reads `opt-in`.
- **A toolkit's own checks are a roster the toolkit leg runs.** They cannot be derived from this tree, so they are declared in `companion/native.list`, one `<toolkit> <arguments…>` line per check. The toolkit leg runs each line's arguments with the toolkit's pinned package over the fixture layout and over each `full` overlay, replacing the two `validate` invocations it hard-codes today. So a `toolkit` row is also a tested fact, and the roster has a reader besides the table.
- **A column's name is the pin's.** `companion/toolkits.list` gains a display name after the version, since the table's header needs one and the toolkit key is lowercase. Columns follow the file's order.
- **The seam.** Repo-root: a repo-root emitter arm and freshness gate, the companion's files and the docs page. No kit surface changes.

**The row sources, a lead decision (not an operator direction).** These sources go past *rows derived from what each companion recipe arms per tier* in two places: a declared roster of the toolkit's own checks, and moving the lifecycle overlay's path. The lead chose both, as written above, over two alternatives, on two grounds. Dropping the standalone `toolkit` rows would under-deliver the entry's deliverable, which marks each check toolkit-native, Checkwright or adopter-selectable. A `native.list` read by the emitter alone would be the hand-kept list the Derivation-first rule refuses. The recipe-derived wording names a source that cannot be read per gate. The direction's constraint is a generated table, never hand-kept, under a freshness gate. A list with a second reader, the toolkit leg, is not a maintained copy, and it meets that constraint.

## What changes

### (1) The two rosters {mechanical}

**Applied.**

- `companion/native.list`, new, opening `# contract: companion/SPEC.md §The toolkit legs — one <toolkit> <arguments…> line per check the toolkit itself ships; the toolkit leg runs each with the toolkit's pinned package, and the support table lists each`. Its one line is `openspec validate --all --strict --no-interactive`.
- `companion/toolkits.list`: each line gains the toolkit's display name after its version: `speckit specify-cli 1.0.12 Spec Kit` and `openspec @fission-ai/openspec 1.13.2 OpenSpec`. Its contract comment reads `<toolkit> <package> <version> <name…>`. The leg's `pin()` reads fields 2 and 3 and is unaffected.

### (2) The lifecycle overlay moves, and the leg runs the roster {mechanical}

**Applied.** `git mv companion/fixtures/openspec/lifecycle companion/fixtures/openspec/full/check-stage-entry`. Its two readers follow: `installer/consumer-smoke/run-smoke.sh`'s `COMPANION_LC`, and `.github/workflows/gates.yml`'s OpenSpec validation step. That step becomes a loop over `companion/native.list`'s lines for each toolkit with a layout. It runs each line with `npx --yes "<package>@<version>"` over a copy of `fixtures/<toolkit>/layout/`, then again with each `fixtures/<toolkit>/full/*/` overlay copied over that copy.

installer/SPEC.md §The consumer smoke, the lifecycle leg's sentence, reads *It overlays `companion/fixtures/openspec/full/check-stage-entry/`*.

companion/SPEC.md §The fixtures: *A lifecycle overlay under `fixtures/openspec/lifecycle/`* becomes *A `full` overlay under `fixtures/openspec/full/check-stage-entry/`*, and the section gains: *`fixtures/<toolkit>/full/<gate>/` overlays the layout with what only the `full` line's gate reads, and names that gate.* §The toolkit legs, step 3, becomes: *runs each `native.list` line of a toolkit with a layout, over a copy of the layout and again with each `full` overlay copied over it.*

### (3) The emitter {design-bearing}

**Applied.** A repo-root arm, `--emit support-table [--write]`, in `native/src/emit/support_table.rs`. It renders a GitHub-Flavored Markdown table between `<!-- support-table:begin -->` and `<!-- support-table:end -->` in `docs/spec-toolkits.md`, with a blank line inside each marker.

- **Header**: `| Check |`, then each `toolkits.list` display name in file order.
- **Gate rows**: the union of the gate names under `companion/fixtures/<toolkit>/{defects,full,overlap}/` for every toolkit, sorted by name. The first cell is the name in a code span. A toolkit's cell is `prose` where its `defects/<gate>/` exists, else `full` where its `full/<gate>/` does, else `opt-in` where its `overlap/<gate>/` does, else `—`.
- **Toolkit rows**: one per `native.list` line, after the gate rows, in file order. The first cell is `<toolkit key> <arguments>` in a code span, the cell of the line's toolkit is `toolkit`, and every other cell is `—`.
- Without `--write` it prints the block. With it, it rewrites the block and touches nothing else.
- **Exit 2**: an unreadable `toolkits.list` or `native.list`; a `toolkits.list` line with no name; a `native.list` line naming a toolkit `toolkits.list` lacks; a fixture toolkit directory `toolkits.list` lacks; a block that is absent or occurs twice.

### (4) `check-support-table-fresh` {design-bearing}

**Applied.** A repo-root freshness gate, born native: `native/src/gates/support_table_fresh.rs`, `scripts/check-support-table-fresh.gate` at `tier=precommit`, registered in `scripts/gates.list`. It compares the page's block with the arm's rendering in process, and one stale block is one finding printing the regen command. It carries `# projection: docs/spec-toolkits.md` and couples `companion/toolkits.list`, `companion/native.list`, `companion/fixtures/*/*/*/**`, `docs/spec-toolkits.md` and its module. Its positional form is `check-support-table-fresh [companion-dir page]`, so a fixture tree stands in.

`scripts/gate-tests/check-support-table-fresh/`: `good/` holds a companion tree of two toolkits, one gate proven on both, one under `full/` on one of them and one under `overlap/`, a `native.list` line and a page whose block matches. `bad/` holds the same tree with the page's block missing the `full` row, and `expect.txt` holds the finding.

### (5) The page and the rosters it joins {mechanical}

**Applied.** `docs/spec-toolkits.md` gains `## Which checks each toolkit gets`, after §What the gates check against your code. It holds one sentence and the block. The sentence: *`prose`: the default line arms and tests it, and `full` keeps it. `full`: only the `full` line does. `opt-in`: an overlap you add beside the toolkit's own check ([the value bar](companion/SPEC.md#the-two-tiers)). `toolkit`: the toolkit's own check, which Checkwright leaves to it.*

docs/site-architecture.md §Generated projections and their freshness gates gains the row: *- **The toolkit support table** `<!-- projection: check-support-table-fresh -->` — `docs/spec-toolkits.md`'s marker block, one row per check proven in `companion/fixtures/` or listed in `companion/native.list`, one column per `companion/toolkits.list` line: `bash gate-sdk/bin/run-gates.sh --emit support-table --write` (`check-support-table-fresh` byte-gates it).*

companion/SPEC.md §The tested claim gains a closing sentence: *The support table on the toolkit page is derived from the fixture directories and `native.list`, so it states what is tested.*

## Producers and consumers

Probe: `ls companion/fixtures/*/defects companion/fixtures/openspec/lifecycle` and `git grep -n "toolkits.list\|fixtures/openspec/lifecycle\|openspec/lifecycle"` over the tracked tree, less the queue, the posts and the generated mirror. The rows at landing, derived by hand from that listing: `check-docs-cmd`, `check-md-refs`, `check-spec-fence-balance`, `check-spec-pointer` and `check-task-path-claim` read `prose` for both toolkits. `check-task-label-resolution` reads `prose` for Spec Kit and `—` for OpenSpec. `check-stage-entry` reads `—` for Spec Kit and `full` for OpenSpec. `openspec validate --all --strict --no-interactive` reads `—` for Spec Kit and `toolkit` for OpenSpec. No `opt-in` cell exists until an overlap lands.

- **`native.list`** (delta 1). Producer: this repository, edited when a toolkit ships a check. Consumers: the toolkit leg, which runs every line; the emitter, which reads the toolkit key and the arguments.
- **The display name** (delta 1). Consumer: the emitter's header. The leg reads no field past 3.
- **`fixtures/<toolkit>/full/<gate>/`** (delta 2). Consumers: the smoke's lifecycle leg, at its new path; the toolkit leg's overlay loop; the emitter, reading the directory's name.
- **The block** (deltas 3 and 5). Producer: the arm's `--write`. Consumers: the reader; `check-support-table-fresh`. Red condition: the block differs from the rendering. Exit 2: the block absent or doubled.

## Existing sections updated

Roster probe: the grep above, and `grep -n "^## \|^###" docs/spec-toolkits.md`.

- `companion/native.list`, `companion/toolkits.list` (delta 1).
- `companion/fixtures/openspec/lifecycle/`, moved (delta 2).
- `installer/consumer-smoke/run-smoke.sh` — `COMPANION_LC` (delta 2).
- `.github/workflows/gates.yml` — the OpenSpec validation step (delta 2).
- `installer/SPEC.md` — §The consumer smoke, the lifecycle leg (delta 2).
- `companion/SPEC.md` — §The component's `toolkits.list` bullet, for the display-name field (delta 1); §The fixtures and §The toolkit legs (delta 2); §The tested claim (delta 5).
- `native/src/emit/support_table.rs`, `native/src/emit/mod.rs`'s arm table (delta 3).
- `native/src/gates/support_table_fresh.rs`, `native/src/gates/mod.rs`'s dispatch table, `scripts/check-support-table-fresh.gate`, `scripts/gates.list`, `scripts/gate-tests/check-support-table-fresh/` (delta 4).
- `docs/spec-toolkits.md`; `docs/site-architecture.md` — §Generated projections and their freshness gates (delta 5).
- The new-gate fan-out that section rosters: `docs/enforcement.md`, `docs/value.md`'s rollup block and `docs/check-graph.html`, each regenerated by the command its gate prints (delta 4).
- `docs/companion/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1, 2 and 5).
- `docs/installer/SPEC.md`, the generated mirror, regenerated the same way (delta 2).
- `.workflow/surface-ceiling.txt` — the grown `docs/spec-toolkits.md` and `docs/site-architecture.md` rows re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling` (delta 5).

## Retired spellings

- `fixtures/openspec/lifecycle` — the lifecycle overlay's path, moved to `fixtures/openspec/full/check-stage-entry` (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Battery green** — the full battery, the root fixture suite, `cargo test` and the consumer smoke green on the landing commit, and the gates workflow's toolkit legs and native OS legs green on the mid-iteration push. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
