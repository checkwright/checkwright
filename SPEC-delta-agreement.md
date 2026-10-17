# SPEC amendment: delta-agreement

Pairs `openspec-delta-base-agreement`. A spec toolkit that writes changes as deltas against a base spec, adding, modifying, renaming or removing named items, can accept a delta that disagrees with its base: it names an item the base lacks, or adds one the base already carries. On OpenSpec at the pin, `validate --strict` passes every such delta and `archive` refuses most of them only after commit, a refusal `--skip-specs` bypasses. This amendment ships a toolkit-blind canon-kit gate for heading-set delta agreement and binds it to OpenSpec, splitting the binding along companion/SPEC.md §The tiers' value bar: the one disagreement no OpenSpec check catches is bound by the default recipe, and the disagreements `archive` refuses ship as the first overlap layer.

**The measurement the split rests on**, re-run at this stage on `@fission-ai/openspec` 1.13.2, the `toolkits.list` pin, over copies of `companion/fixtures/openspec/layout/`:

| delta | `validate --all --strict` | `archive -y` | `archive -y --skip-specs` |
| --- | --- | --- | --- |
| MODIFIED naming a requirement the base lacks | 0, an INFO line | 1 | 0 |
| RENAMED whose FROM the base lacks | 0, an INFO line | 1 | 0 |
| RENAMED whose TO the base already carries | 0, an INFO line | 1 | — |
| ADDED naming a requirement the base carries | 0, an INFO line | 1 | 0 |
| MODIFIED or REMOVED on a capability with no base spec | 0 | 1 | — |
| REMOVED naming a requirement an existing base lacks | 0, silent | 0, "already in sync" | 0 |

`archive` matches a header case-sensitively, and ignores blank runs around and inside the requirement's name: a lowercased name is "not found", while a doubled inner space, trailing blanks or no space after the colon all match.

**The refused alternative: the whole binding as one overlap.** The last row is no overlap: no OpenSpec check covers it, so leaving it opt-in withholds a check the default line can carry at no cost to the bar. **The other refused alternative: the whole binding in the default recipe.** Every other row re-checks ground `archive` covers, which the bar admits only as an additive layer off by default.

## What changes

### (1) canon-kit gains `check-delta-agreement` {design-bearing}

New section, `canon-kit/SPEC.md` §check-delta-agreement, after §check-task-label-resolution. **Not yet applied.**

> ### check-delta-agreement
>
> Invariant: every configured delta agrees with its base on which items exist. A **delta** is a markdown file whose repo-relative path matches `CANON_KIT_DELTA_PATH`, a one-group ERE (gate-sdk/SPEC.md §The POSIX ERE matcher), walked through the pruning walk. Its **base** is the file at the group's text, resolved under `CANON_KIT_DELTA_BASE_DIR`. A base's **items** are the group texts of its lines matching `CANON_KIT_DELTA_BASE_ITEM`, fences skipped.
>
> A **family** is a key of `CANON_KIT_DELTA_ITEM` and of exactly one of `CANON_KIT_DELTA_PRESENT` and `CANON_KIT_DELTA_ABSENT`. The `PRESENT` or `ABSENT` value is an ERE matching the family's **section heading**; the section runs from that line to the next heading of its level or shallower. The `ITEM` value is a one-group ERE, and each line in the section matching it names an item. Fences are skipped in a delta too. Two families may share a section heading, so one section can carry two assertions.
>
> - Where the base file exists, an item a `PRESENT` family names must be among its base's items.
> - Where it exists, an item an `ABSENT` family names must not be among them.
> - Where the base file does not exist, an item a family listed in `CANON_KIT_DELTA_BASE_REQUIRED` names is a finding. Any other family asserts nothing against a missing base.
>
> Item names compare case-sensitively after trimming and collapsing each run of blanks to one space. A finding names the delta, the line, the family, the item and the base, and whether the base lacks it, already carries it, or does not exist.
>
> The table validator holds `CANON_KIT_DELTA_ITEM`'s key set equal to the union of the other two keyed knobs', with no key in both, each ITEM and the path and base-item knobs to the one-group shape, and every `CANON_KIT_DELTA_BASE_REQUIRED` member to a family. A refusal is exit 2.
>
> Declared by `checks/check-delta-agreement.gate`, `# install: zero-config`, `# armed-by: CANON_KIT_DELTA_PATH`, dispatching to the binary, with `knob:CANON_KIT_DELTA_PATH` and `*.md` in `couples=`. An empty `CANON_KIT_DELTA_PATH` is the clean skip, taken before the validator reads the tables, and its clean line says no delta is configured. The pair names no toolkit: `good/` holds a delta whose present and absent families agree with its base, a fenced example naming items its base lacks, a section of a family sharing its heading with another, and a delta with no base carrying only items of a family not base-required; `bad/` holds a present item its base lacks, an absent item its base carries, and a base-required family's item on a delta with no base. Tier `precommit`.

Replacement for `canon-kit/SPEC.md` §Layout and configuration, a new bullet after the one opening `` `CANON_KIT_TASK_LIST_GLOBS` ``. **Not yet applied.**

> - `CANON_KIT_DELTA_PATH`, the one-group ERE naming a delta and its base, and `CANON_KIT_DELTA_BASE_DIR`, the directory a base resolves under, default empty for the repository root. `CANON_KIT_DELTA_BASE_ITEM`, the one-group ERE naming an item in a base. `CANON_KIT_DELTA_PRESENT` and `CANON_KIT_DELTA_ABSENT`, keyed: per family, the ERE matching its section heading. `CANON_KIT_DELTA_ITEM`, keyed: per family, the one-group ERE naming an item in that section. `CANON_KIT_DELTA_BASE_REQUIRED`, indexed: the families whose items need the base to exist. All default empty (§check-delta-agreement).

Owed with it: the seven rows in `native/src/knobs/canon_kit.rs` and the table validator's arm beside the task-label one; the gate's row in the gate table of `native/src/gates/mod.rs`; the gate module and its crate cases (each finding shape, the shared-heading section, the fence skip, the name normalization, the missing base); the `.gate` file and the fixture pair; the gate's row in `canon-kit/README.md`'s roster and its line in `canon-kit/smoke/install.sh`'s registered roster, which `check-install-disposition` holds a superset of the zero-config members; and its registration in `scripts/gates.list`, disarmed here as `check-task-path-claim` is. The generated `docs/enforcement.md`, the rollup block of `docs/value.md` and `docs/check-graph.html` regenerate with the commands their freshness gates print.

### (2) The OpenSpec recipe binds the uncovered disagreement, and the first overlap layer binds the rest {design-bearing} {user-facing: as delta 4}

`companion/openspec/recipe/canon-config.knobs` gains five lines, and a new layer `companion/openspec/overlap/` holds `canon-config.knobs` and an empty `unregister.list`, packed as the recipe `openspec-overlap` by a `GATE_SDK_PAYLOAD_RECIPES[openspec-overlap]` line in `scripts/gate-sdk-config.knobs`. The spellings below are the design; the companion arm's run is their oracle, and build corrects a spelling to what it returns.

Recipe lines: `CANON_KIT_DELTA_PATH = ^openspec/changes/[^/]+/specs/(.+/spec\.md)$`, `CANON_KIT_DELTA_BASE_DIR = openspec/specs`, `CANON_KIT_DELTA_BASE_ITEM = ^### Requirement:[[:space:]]*(.+)$`, `CANON_KIT_DELTA_PRESENT[removed] = ^## REMOVED Requirements`, `CANON_KIT_DELTA_ITEM[removed] = ^### Requirement:[[:space:]]*(.+)$`. The path ERE's change segment cannot match `archive`'s dated child, so an archived change is no delta.

Overlap lines: the families `modified` (PRESENT, `^## MODIFIED Requirements`, item `^### Requirement:[[:space:]]*(.+)$`), `renamed` (PRESENT, `^## RENAMED Requirements`, item ``^- FROM: `### Requirement:[[:space:]]*(.+)` ``), `renamed-to` (ABSENT, the same heading, item ``^- TO: `### Requirement:[[:space:]]*(.+)` ``) and `added` (ABSENT, `^## ADDED Requirements`, the requirement item), and `CANON_KIT_DELTA_BASE_REQUIRED[] = modified`, `renamed` and `removed`, one line each.

Replacement for `companion/SPEC.md` §The OpenSpec recipe, its first paragraph and a new bullet before `unregister.list`'s. **Not yet applied.**

> OpenSpec's own checks: `openspec validate --strict`, over the structure of specs and change deltas, and `openspec archive`, which refuses a delta disagreeing with its base spec unless `--skip-specs` is passed. Measured at 1.13.2, `validate` passes every disagreeing delta, and one disagreement passes `archive` too: a REMOVED requirement an existing base lacks, which `archive` reads as already removed.
>
> - The five `CANON_KIT_DELTA_*` lines in `canon-config.knobs` arm `check-delta-agreement` on each in-flight change's deltas against `openspec/specs/`, for the REMOVED family alone, the disagreement no OpenSpec check catches. Without them the gate asserts nothing. A requirement's name compares case-sensitively and blank-collapsed, as `archive` matches one.

New subsection after §The lifecycle layer, `companion/SPEC.md` §The OpenSpec overlap. **Not yet applied.**

> ### The OpenSpec overlap
>
> `openspec/overlap/` is the first overlap layer (§The tiers), applied as `--recipe openspec-overlap` beside the recipe, and the page's `companion-overlap` line carries both. It binds the disagreements `archive` refuses and `validate --strict` passes at the pin: a MODIFIED or RENAMED requirement the base lacks, a RENAMED target or an ADDED requirement the base already carries, and a MODIFIED, RENAMED or REMOVED delta on a capability with no base spec. Each clears the bar twice: `archive` runs after the change is committed, and `--skip-specs` skips it. The layer adds families to `check-delta-agreement` and arms nothing the recipe does not, so applied alone it asserts nothing. OpenSpec's own checks stay enabled.

Replacement for `companion/SPEC.md` §The tested claim, the sentences of its fourth paragraph from "Delta-to-base agreement" to "the value bar (§The tiers)." **Not yet applied.**

> OpenSpec also carries a REMOVED requirement its base lacks, by `check-delta-agreement`, and its overlap a disagreement of each family `archive` refuses, by the same gate (§The OpenSpec overlap).

Replacement for `companion/SPEC.md` §The fixtures, the sentence of its fourth paragraph naming OpenSpec's defects gains a clause, and its overlap paragraph is rewritten. **Not yet applied.**

> OpenSpec's `check-delta-agreement` defect adds a REMOVED requirement the notes spec lacks to the change's delta.
>
> `fixtures/<toolkit>/overlap/<gate>/` plants the defects an overlap exists for, over the layout, one per family the layer binds, each in a file of its own so a finding names it alone. The companion arm applies the toolkit's recipe and overlap layer and asserts the gate reds, naming every file the overlay carries. The toolkit leg runs the toolkit's own check over the same planted tree and asserts the check passes it, so a pin move to a release that closes a miss reds the leg, and the overlap is weighed again (§The tiers).

Replacement for `companion/SPEC.md` §The support table, the gate-row bullet's cell sentences. **Not yet applied.**

> A toolkit's cell reads `prose` where its `defects/<gate>/` exists, since the default line arms and tests it and each tier above it only adds, and `prose + opt-in` where its `overlap/<gate>/` exists too. Otherwise it reads `full` where its `full/<gate>/` exists, `opt-in` where its `overlap/<gate>/` exists (§The tiers), and `—` elsewhere.

Replacement for `companion/SPEC.md` §The toolkit legs, step 3, absorbing step 4. **Not yet applied.**

> 3. runs each `native.list` line of a toolkit with a layout, with the toolkit's pinned package, over a copy of the layout and again with each `full` and each `overlap` overlay copied over it, and asserts that each passes. An overlap overlay passing is the miss the overlap exists for.

`companion/SPEC.md` §Honest limits drops the bullet opening "No overlap has shipped": both loops land here.

### (3) The companion arm runs the overlap line and holds every planted file named {design-bearing}

Replacement for `installer/SPEC.md` §The consumer smoke, the companion arm's numbered list, a new step after step 4. **Not yet applied.**

> 5. where `companion/fixtures/<toolkit>/overlap/` holds a gate directory, reads the toolkit's `companion-overlap` block as it reads the install block, refusing one absent, empty or not carrying `--recipe <toolkit>-overlap`. It re-runs `init` in the same consumer with that line's arguments, asserts one commit and the battery green, then for each `overlap/<gate>/` overlays its files, asserts the battery red with a `FAIL: <gate>` line whose block names every file the overlay carries, and restores the commit.

The arm's refusal set also reds a `companion-overlap` block on a toolkit with no overlap directory, so a line no leg runs cannot stand on a page.

### (4) The OpenSpec page, the support table, the toolkits page and the workflow leg {mechanical} {user-facing: openspec-delta-base-agreement's Deliverable, "its OpenSpec binding in the recipe", split along companion/SPEC.md §The tiers' value bar, which already settles that an overlap is opt-in}

- `docs/openspec.md` gains, after the recipe paragraph, one sentence naming the REMOVED check, and a paragraph introducing the overlap with its line between `<!-- companion-overlap:begin -->` and `<!-- companion-overlap:end -->`, in the page's `text` form: `checkwright init --profile prose --recipe openspec --recipe openspec-overlap`. The paragraph says it re-checks at commit what `openspec archive` refuses later or skips under `--skip-specs`, and that OpenSpec's own checks stay on.
- `docs/spec-toolkits.md` §What the gates catch gains an OpenSpec bullet for the REMOVED disagreement and a sentence for the overlap; its legend line gains `prose + opt-in`. The support-table block regenerates with `--emit support-table --write`.
- `.github/workflows/gates.yml`'s `companion-toolkits` step for each toolkit's own checks adds `companion/fixtures/"$tk"/overlap/*/` to its overlay loop.

**The release this rides.** `openspec-overlap` on `docs/openspec.md` is an advertised recipe operand, held to the pinned release by `check-front-door-verbs` and admitted while pending (installer/SPEC.md §The front door's verbs). With the v0.32.0 cut the set record grants, the line stands. Without a cut, close withdraws the overlap line or files the release, as that admission requires.

**The leg's oracle is local.** Build runs the workflow step's body with `RUNNER_TEMP` pointed at scratch and `npx` at the pin, as this stage's measurement did, so the entry reads no remote run and its push need stays the set's closing push, whose `gates` run is the remote witness.

## Producers and consumers

- **`check-delta-agreement` (delta 1).**
  - Producer: the battery, at pre-commit and CI, wherever `init` registers canon-kit (`prose`, `complement`, `full`), armed by a seam carrying `CANON_KIT_DELTA_PATH`. The deployed configuration setting it is the OpenSpec recipe (delta 2), so the gate is live on every OpenSpec install line.
  - Consumers: the adopter, through the battery's FAIL block; `doctor`'s disarmed line, read off `# armed-by:` (installer/SPEC.md §doctor); the companion arm's step 4 and step 5 (delta 3); the support-table arm, through the fixture directories (delta 2).
  - Roster-holding readers of the minted names, from `grep -rln "CANON_KIT_TASK_LABEL_CITES" native/src canon-kit gate-sdk scripts`, the sibling gate's knobs taken as the model: `native/src/knobs/canon_kit.rs` (rows and validator), the gate table in `native/src/gates/mod.rs` (each gate's declared knob roster, which takes the seven names and the prune knobs the task gates declare), `canon-kit/SPEC.md` §Layout and configuration, and the generated `docs/canon-kit/SPEC.md`. `check-knob-citation` reads a cited knob against the rows, so the rows land with the citations. `check-install-disposition` reads the `# install:` line; the third assertion holds no gate name in the recipe module, and the binding is data in a recipe file.
  - Fields: each finding's delta, line and item, read by the adopter; the family and the base, read by the adopter and by step 5's every-file assertion, which reads the delta path alone.
- **The overlap layer (delta 2).** Producer: the packer, from the new `GATE_SDK_PAYLOAD_RECIPES` key. Consumers: `init --recipe openspec-overlap` (installer/SPEC.md §Payload recipes) and the companion arm's step 5. Readers of the recipe roster, from `grep -rn "GATE_SDK_PAYLOAD_RECIPES" native/src --include=*.rs -l`: the packer, `check-front-door-verbs`' recipe set, and the companion arm, whose step 1 derives the toolkit set from `companion/*/recipe/` and so does not read `overlap/` as a toolkit.
- **The `companion-overlap` block (deltas 3 and 4).** Producer: `docs/openspec.md`. Consumers: the companion arm (delta 3), and `check-front-door-verbs`, which reads its `checkwright` line as an advertisement.
- **Point 5:** delta 2's path ERE narrows nothing an existing reader holds a count over: the gate is new, and the support table gains a row.
- **Point 6:** the overlap obliges each family it binds; each family's satisfying value is its planted defect in `overlap/check-delta-agreement/`, measured against `archive` at the pin in the table above, and the default family's is the `defects/check-delta-agreement/` plant.

## Existing sections updated

The roster below came from `grep -n "overlap\|value bar\|Delta-to-base" companion/SPEC.md installer/SPEC.md docs/spec-toolkits.md docs/openspec.md` and `grep -n "overlay" .github/workflows/gates.yml` over the tracked tree.

- `canon-kit/SPEC.md`, a new §check-delta-agreement and §Layout and configuration (delta 1).
- `native/src/knobs/canon_kit.rs`, the gate table in `native/src/gates/mod.rs`, the gate module, `canon-kit/checks/`, `canon-kit/gate-tests/`, `canon-kit/README.md`, `canon-kit/smoke/install.sh` and `scripts/gates.list` (delta 1).
- `docs/enforcement.md`, the rollup block of `docs/value.md` and `docs/check-graph.html`, the generated projections the new gate moves (delta 1).
- `companion/openspec/recipe/canon-config.knobs` and the new `companion/openspec/overlap/` (delta 2).
- `scripts/gate-sdk-config.knobs`, the `GATE_SDK_PAYLOAD_RECIPES` key (delta 2).
- `companion/SPEC.md` §The OpenSpec recipe, a new §The OpenSpec overlap, §The tested claim, §The fixtures, §The support table, §The toolkit legs and §Honest limits (delta 2).
- `companion/fixtures/openspec/defects/check-delta-agreement/` and `companion/fixtures/openspec/overlap/check-delta-agreement/` (delta 2).
- `native/src/emit/support_table.rs`, the cell rule, and the OpenSpec claimed-gate list in `native/src/emit/installer_smoke/companion.rs` (delta 2).
- `installer/SPEC.md` §The consumer smoke, the companion arm, and `native/src/emit/installer_smoke/companion.rs` (delta 3).
- `docs/openspec.md`, `docs/spec-toolkits.md` and `.github/workflows/gates.yml` (delta 4).
- `docs/canon-kit/SPEC.md`, `docs/canon-kit/README.md`, `docs/companion/SPEC.md` and `docs/installer/SPEC.md`, the generated mirrors (all deltas).

## Retired spellings

- None — no delta retires a spelling; the overlap's path pattern and the support-table cell values keep theirs, and `prose + opt-in` is added beside them.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Fixture suites** — canon-kit's and the installer consumer smoke's companion arm run green, and the workflow step's body run locally at the pin.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
