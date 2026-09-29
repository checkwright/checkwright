# SPEC amendment: value-bar

companion/SPEC.md §The two tiers states the toolkit boundary absolutely: *Neither tier re-checks what a toolkit owns*. That forbids every overlap, including one a measurement shows is worth having. OpenSpec shows the case. At the pinned version, `openspec validate --strict` exits 0 on a change delta that disagrees with its base spec, and `openspec archive` refuses most such deltas but not all, and not under `--skip-specs`. §The tested claim still reads that OpenSpec's archive refuses a disagreeing delta, so no gate re-checks it.

This amendment replaces the absolute boundary with a checkable value bar. It also states how an overlap ships: as a recipe layer the adopter opts into, beside the toolkit's own check, never in the default recipe.

One queue entry pairs it: [toolkit-overlap-value-bar](../TASK-QUEUE.md#toolkit-overlap-value-bar). No overlap check lands with it. The first candidate is filed separately and is held to this bar when it is built.

**The rulings.**

- **The bar is a measured miss, pinned.** An overlap is a Checkwright check over ground a toolkit's own check covers. It is admitted only for a defect that the toolkit's check misses where it matters, and the miss must be measured, never argued. That means one of two things: the toolkit's check misses it at commit, where it catches it only later, or the toolkit offers a flag that skips the check. The miss is recorded with the toolkit version it was measured on, and pinned by a fixture the toolkit's own check is run against, so a toolkit release that closes the miss is seen rather than assumed.
- **Additive, and off by default.** The toolkit's own check stays enabled, and an overlap never replaces it. An adopter chooses between the toolkit's check alone, the default, and the toolkit's check with Checkwright's beside it (the Policy-as-choice rule). The choice is a recipe layer, `<toolkit>/overlap/`, applied as `--recipe <toolkit>-overlap` beside the toolkit's recipe, on the lifecycle layer's pattern. So the default install line carries no overlap, and the manifest records the choice as it records any recipe.
- **A toolkit's ground is named where the recipe is measured.** Each toolkit's recipe section states what that toolkit's own checks cover, so an overlap has a named check to be measured against.
- **The seam.** Companion only. The generic gate an overlap arms is a kit's and stays toolkit-blind. Its binding and the toolkit's name live in the layer, as every recipe line does (§The component).

## What changes

### (1) The boundary becomes the value bar {design-bearing}

**Not yet applied.** In §The two tiers, the sentence *Neither tier re-checks what a toolkit owns: OpenSpec's validator and Spec Kit's templates stay the toolkit's.* is replaced by a paragraph:

*Neither tier re-checks what a toolkit's own checks cover, unless an overlap clears the value bar. An overlap re-checks ground a toolkit's check covers, and is admitted only for a defect that check misses where it matters: a check that runs only after commit, or one a flag skips. The miss is measured at the toolkit's pin, recorded with that version, and pinned by a fixture that the toolkit leg runs the toolkit's own check against (§The toolkit legs). An overlap is additive and off by default. It ships as a recipe layer, `<toolkit>/overlap/`, which the adopter applies as `--recipe <toolkit>-overlap` beside the toolkit's recipe, and the toolkit's own check stays enabled.*

### (2) Each recipe names the toolkit's ground {mechanical}

**Not yet applied.** Each recipe section opens its list with the toolkit's own checks:

- §The Spec Kit recipe: *Spec Kit's own checks: none over the written specs. It ships templates and no validator.*
- §The OpenSpec recipe: *OpenSpec's own checks: `openspec validate --strict`, over the structure of specs and change deltas, and `openspec archive`, which refuses a MODIFIED, RENAMED or ADDED delta disagreeing with its base spec unless `--skip-specs` is passed.*

### (3) The tested claim stops asserting the archive covers agreement {mechanical}

**Not yet applied.** In §The tested claim, *The toolkit keeps what it owns: OpenSpec's archive refuses a delta that disagrees with its base spec, so no gate re-checks it.* becomes: *Delta-to-base agreement is OpenSpec's, at archive. A gate for it is an overlap, and none ships until one clears the value bar (§The two tiers).*

### (4) The fixture and leg shape of an overlap {design-bearing}

**Not yet applied.** §The fixtures gains a paragraph:

*`fixtures/<toolkit>/overlap/<gate>/` plants the defect an overlap exists for, over the layout. The companion arm applies the toolkit's recipe and overlap layer and asserts the gate reds it. The toolkit leg runs the toolkit's own check over the same planted tree and asserts the check misses it, so a pin move to a release that closes the miss reds the leg, and the overlap is weighed again.*

§The toolkit legs gains a numbered step after the OpenSpec validation: *for each `fixtures/<toolkit>/overlap/<gate>/`, runs the toolkit's own check over the layout with the defect planted, and asserts that it passes.*

No directory of this shape exists when this amendment lands, so the arm and the leg gain their loops with the first overlap. Until then, this amendment changes neither.

## Producers and consumers

Probe: `grep -n "re-checks\|toolkit owns\|keeps what it owns"` over `companion/SPEC.md`, `docs/*.md` and `companion/README.md` finds line 33 (§The two tiers) and line 78 (§The tested claim), and no page. The OpenSpec measurements come from the queue entry of the first candidate, measured on openspec 1.13.2, which is the `openspec` pin in `companion/toolkits.list`.

- **The value bar** (delta 1). Producer: this section. Consumers: the authoring stage of each later overlap unit, which measures against it, and the companion support table's marks, whose owner cites it.
- **The overlap layer** `<toolkit>/overlap/` (delta 1). Producer: the first overlap unit. Consumers: `init --recipe <toolkit>-overlap`, through installer/SPEC.md §Payload recipes, once `GATE_SDK_PAYLOAD_RECIPES` names the layer; the companion arm. No layer exists today, so no reader reds.
- **`fixtures/<toolkit>/overlap/<gate>/`** (delta 4). Producer: the first overlap unit. Consumers: the companion arm and the toolkit leg, each gaining its loop in that unit.

## Existing sections updated

Roster probe: the grep above.

- companion/SPEC.md — §The two tiers (delta 1), §The Spec Kit recipe and §The OpenSpec recipe (delta 2), §The tested claim (delta 3), §The fixtures and §The toolkit legs (delta 4).
- `docs/companion/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- None — the deltas rewrite prose and name a layer and a fixture shape; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls companion/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, checked by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Battery green** — the full battery green on the landing commit. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), in its landing commit.
