# SPEC amendment: foreign-lifecycle

A lifecycle stage machine over a tree whose specs another toolkit writes has no binding. `check-stage-entry` assertion C decides whether a build entry owes an align stamp by reading the amendment set. With the knobs at their defaults, a tree whose specs Spec Kit or OpenSpec writes has no amendment, so the signal never fires and the audit is skipped with no red. The entry asked for the binding to be stated per toolkit in existing knobs. The code shows that knobs alone cannot express it for either toolkit. The route is ruled: a path-aware amendment glob, the OpenSpec binding with a companion fixture proving the audit fires, Spec Kit stated as unbindable, and `full` plus the recipe as the adopter's profile (operator direction, 2026-09-27, lead-relayed, not a /consult ruling).

**Measured at authoring (2026-09-27).**

- `audit_signal` (`native/src/gates/stage_entry.rs`) and assertions D and E's walk match `LIFECYCLE_KIT_AMENDMENT_GLOB` against a file's **basename** only (`walk::pattern_match(&k.amendment_glob, &base(p))`). So no value names a file by its place in the tree.
- OpenSpec names its capability specs and a change's delta specs alike `spec.md`: `openspec/specs/<cap>/spec.md` and `openspec/changes/<change>/specs/<cap>/spec.md`. A basename glob selecting the deltas selects every capability spec as well, and fires the audit at every build entry of a tree with two capabilities.
- At the pinned `@fission-ai/openspec` 1.13.2, an archived change moves under `openspec/changes/archive/` (`OPENSPEC_ARCHIVE_DIR = 'openspec/changes/archive'` in the package's `dist/core/openspec-root.js`), and `archive` is a reserved change name. So a change's delta is in flight exactly while it sits at `openspec/changes/<change>/specs/<cap>/spec.md`, one level shallower than any archived delta.
- Spec Kit keeps each feature at `specs/<NNN-feature>/` after the feature ships. Which feature is current is the branch's name (`.specify/scripts/bash/`'s feature-dir helper), so nothing on disk tells an in-flight spec from a finished one.
- `installer/profiles.list`: `lifecycle-kit` is in `delegation`, `canon-kit` in `prose`, and only the derived `full` carries both. The companion recipes install `prose`.
- The entry's claim that `check-spec-pointer` breaks the same way on non-markdown artifacts does not hold. Both toolkits' specs are markdown, and each recipe already brings them into the governed set through `CANON_KIT_PROSE_SURFACE_GLOBS`. The one non-markdown artifact class, Spec Kit's `contracts/`, is named by no recipe and no fixture. So the pointer's reach needs no binding, and this amendment moves none.

## What changes

### (1) An amendment glob carrying `/` matches the path {design-bearing}

**Not yet applied.** In `native/src/gates/stage_entry.rs`, one predicate decides whether a walked file is an amendment, and both `audit_signal` and assertions D and E's walk call it. A glob carrying no `/` matches the file's basename, as today. A glob carrying `/` matches the file's repo-relative path **component-wise**: `*` and `?` stay inside one path component, and a `**` component matches any number of them. That is the discipline `walk::glob_files` applies to a root-relative glob. The predicate calls the walk module's component matcher (`dir_glob_match`, made public for it), and `native/src/gates/reads_couples.rs`'s private `glob_path_match`, a second spelling of the same matcher, is replaced by that call. The default `SPEC-*.md` carries no `/`, so no consumer's verdict moves.

A path-matched amendment is a component in its own directory, as a basename-matched one is. So two in-flight deltas in two directories fire the signal. The body-token arm is unchanged.

In lifecycle-kit/SPEC.md §Layout and configuration, the bullet "`LIFECYCLE_KIT_AMENDMENT_GLOB` / `LIFECYCLE_KIT_ROSTER_BASENAME` — the amendment filename shape and the canonical-spec basename assertion C scans (template dirs pruned); defaults `SPEC-*.md` / `SPEC.md`." becomes:

> - `LIFECYCLE_KIT_AMENDMENT_GLOB` / `LIFECYCLE_KIT_ROSTER_BASENAME` — what makes a file an amendment, and the canonical-spec basename, for assertion C's scan (template dirs pruned); defaults `SPEC-*.md` / `SPEC.md`. A glob with no `/` matches a file's basename. One carrying `/` matches its repo-relative path component by component, `*` inside one component and `**` across any number, for a layout where a file's place, not its name, marks it as in flight.

In lifecycle-kit/SPEC.md §check-stage-entry, assertion C's sentence "The signal reads the on-disk amendment tree (cwd-relative, gate-sdk prune set applied, and `templates/` paths excluded …)" gains after its parenthesis: "an amendment being a file the amendment glob matches, by basename or by path as that knob's bullet states". In the paragraph beginning **Assertions D and E read C's amendment walk**, "prune set, `templates/` exclusion and `LIFECYCLE_KIT_AMENDMENT_GLOB` basenames" becomes "prune set, `templates/` exclusion and `LIFECYCLE_KIT_AMENDMENT_GLOB`'s match".

**The declaration moves with the walk.** check-stage-entry's registry row declares its tree walk filtered by `name:knob:LIFECYCLE_KIT_AMENDMENT_GLOB`, and `check-reads-couples` puts a `name:` token to a file's basename. Under a path-shaped value, that reader would select no file the walk selects. So in `native/src/gates/reads_couples.rs`, a `name:` token carrying `/` selects as a `glob:` token does. In gate-sdk/SPEC.md §check-reads-couples, the paragraph beginning **The filter field is `<kind>:<source>`, and the kind is mandatory.** gains before "**The kind does not default.**":

> A `name:` token carrying `/` is put to the root-relative path as a `glob:` token is, because a walker matching a path-shaped name knob does exactly that. A basename never carries `/`, so the rule moves no basename match.

`lifecycle-kit/gate-tests/check-stage-entry.test.sh` gains two assertion C scenarios in the generic layout it already builds, naming no toolkit. With `LIFECYCLE_KIT_AMENDMENT_GLOB` set to `changes/*/specs/*/delta.md`, a build entry reds when two such deltas sit under one change in two directories. It stays clean when the only other match sits one level deeper, under `changes/archive/<name>/specs/<cap>/delta.md`. Its closing line's scenario count moves with them. A `reads_couples` unit test covers the `name:` rule both ways, and the existing registry-coverage tests hold the row.

### (2) The OpenSpec lifecycle layer {design-bearing}

**Not yet applied.** `companion/openspec/lifecycle/` holds a second recipe directory for OpenSpec, applied by the same procedure after the recipe, and only by an adopter installing `full`:

- `lifecycle-config.knobs`, the name of lifecycle-kit's config seam:
  - `LIFECYCLE_KIT_AMENDMENT_GLOB = openspec/changes/*/specs/*/spec.md`. It selects a change's delta specs while the change is in flight, and neither a capability spec nor an archived delta.
  - `LIFECYCLE_KIT_ROSTER_BASENAME = spec.md`. It makes each capability directory a roster directory, so a delta whose body cites another capability's `openspec/specs/<cap>/spec.md` reaches two components.
  - `LIFECYCLE_KIT_CONTRACT_TOKENS[] = spec.md`, the token that citation ends in.
- `unregister.list`, holding its header alone.

The directory is not named `recipe/`, so the companion arm's `companion/*/recipe/` toolkit roster does not read it as a third toolkit. A feature entry's `[spec:]` ref names the change's `proposal.md` by repo-relative path, which canon-kit's `[spec:]` resolution already takes (canon-kit/SPEC.md §check-amendment-queue).

In companion/SPEC.md §Recipes, a subsection is added after **The OpenSpec recipe**:

> ### The lifecycle layer
>
> An adopter who runs lifecycle-kit's stage machine installs `full`, the one profile carrying both lifecycle-kit and canon-kit, applies the toolkit's recipe, then applies `<toolkit>/lifecycle/` with the same procedure. The layer binds the knobs that let `check-stage-entry` see the toolkit's in-flight work as amendments, so a cross-component build entry owes an align stamp.
>
> - **OpenSpec.** `openspec/lifecycle/` binds the amendment glob to in-flight change deltas by path, the roster basename to `spec.md` and the contract token to `spec.md`. A change touching two capabilities fires the audit, and so does a delta citing another capability's spec. An archived change sits one level deeper, under `openspec/changes/archive/`, and is no amendment.
> - **Spec Kit: no layer.** A feature's directory persists after it ships and the current feature is the branch's name, so nothing on disk marks a spec as in flight. A glob over `specs/*/spec.md` would demand the audit at every build entry once two features exist. `check-stage-entry` therefore sees no amendment in a Spec Kit tree, and the align trigger there is the authoring stage's own recommendation.
>
> `check-spec-pointer`'s reach needs no layer. Both toolkits' specs are markdown, and each recipe already brings them into the governed set.

In companion/SPEC.md §Honest limits, a bullet is added: "A change whose two in-flight deltas sit under one capability in two changes fires the OpenSpec layer's audit, since each delta's directory is a component. The align waiver is the valve. On Spec Kit the audit trigger is not machine-held."

The paragraph under §The component, "It adds no kit mechanism: each recipe is consumer config, written in knobs the `prose` profile already reads.", becomes "It adds no kit mechanism: each recipe is consumer config in knobs the `prose` profile reads, and the lifecycle layer is config in knobs lifecycle-kit reads."

### (3) The companion arm proves the audit fires {design-bearing}

**Not yet applied.** `companion/fixtures/openspec/lifecycle/` holds an overlay for the OpenSpec layout: a queue file whose header names an iteration and whose feature entry carries `[spec: openspec/changes/<change>/proposal.md]`; `.workflow/WORKFLOW-STATE.txt` stamping that iteration through `build` with no `align` stamp; and a second capability's delta under the fixture's existing change, so the change touches two capabilities. Where the fixture's change is `add-digest` and its delta covers `notes`, the overlay adds `openspec/changes/add-digest/specs/<second-cap>/spec.md` and the capability spec it modifies, both passing `openspec validate --all --strict` at the pin.

The companion arm in `installer/consumer-smoke/run-smoke.sh` gains a lifecycle leg after the OpenSpec recipe's legs:

1. It installs `full` on the OpenSpec layout and runs the landing page's block with `recipe` set to `companion/openspec/recipe`, then to `companion/openspec/lifecycle`, asserting each green as `companion_green` does.
2. It copies the overlay in and runs `--only check-stage-entry`. It asserts exit 1 and a finding naming the two delta directories.
3. It removes the second delta and its capability spec, and asserts `--only check-stage-entry` exits 0.

In companion/SPEC.md §The fixtures, the OpenSpec bullet gains "and a lifecycle overlay under `fixtures/openspec/lifecycle/`, whose change touches two capabilities at a build cursor with no align stamp". In installer/SPEC.md §The consumer smoke, the companion arm's paragraph gains one sentence naming the lifecycle leg and its two verdicts.

**Inferred, cannot run before build:** the `full` profile's battery is green on the OpenSpec layout with the recipe and the layer applied — the layer and the leg do not exist until this delta lands, and the prose-profile measurement at the recipe's authoring does not reach `full`'s wider roster.

If a `full` gate reds on an OpenSpec idiom, that red is a recipe line owed under §Recipes' "each line answers a red", written into the layer with its idiom recorded, in the same batch.

### (4) The adopter pages name the profile {mechanical}

**Not yet applied.** docs/spec-toolkits.md gains one sentence after its recipe block: "Running lifecycle-kit's stage machine as well? Install `full` instead of `prose`, and on OpenSpec apply `companion/openspec/lifecycle/` with the same block. Spec Kit has no lifecycle layer." The sentence links companion/SPEC.md §The lifecycle layer in the citation form the page already uses for that file. `companion/README.md` gains the same sentence in its own voice.

That page is also [catalog-landing-docs-polish](TASK-QUEUE.md#catalog-landing-docs-polish)'s. If the polish lands in the same batch as this delta, the polish writes this sentence to its own page rules. If it lands in a later batch, it polishes the sentence this delta left.

## Producers and consumers

- **The path-matched amendment set.** Producer: delta 1's predicate over the walked tree. Consumers: assertion C's two arms, and assertions D and E's marker walk, both in `check-stage-entry`. The enabling config is the OpenSpec layer's glob, set by every adopter who applies it, and in this repository by the companion leg (delta 3).
- **The `name:` token rule.** Producer: the registry row's declared filter. Consumer: `check-reads-couples`' `filter_token_selects`. The row's value is basename-shaped in every tree that sets no layer, so no live verdict moves.
- **The layer's knob lines.** Producer: `companion/openspec/lifecycle/lifecycle-config.knobs`, appended by the procedure. Consumer: lifecycle-kit's knob resolution, read by `check-stage-entry` (all three knobs) and by `--emit close-surfaces`, which reads `LIFECYCLE_KIT_ROSTER_BASENAME` over the resolved kit roots. In a consumer those roots carry no canonical spec under either basename, because the payload withholds every SPEC, so the close-surface set does not move.
- **Roster-holding readers.** The companion arm's toolkit roster derives from `companion/*/recipe/`, and the layer's directory name keeps out of it. No kit roster gains a name.
- **Point 5.** No corpus narrows at the default. Under the layer, the amendment set narrows from every `spec.md` to in-flight deltas. Assertion C reds on *finding* two components, so it is monotone under the narrowing, and D and E red on markers found, likewise.
- **Point 6.** No corpus-wide obligation.
- **Sibling dependency.** [lead-clause-heading-family](TASK-QUEUE.md#lead-clause-heading-family) was filed as reshaped by this unit, since both were thought to move `check-spec-pointer`'s reach. This amendment moves no pointer reach, so that entry stands as filed.

## Existing sections updated

Roster from `git grep -n "LIFECYCLE_KIT_AMENDMENT_GLOB\|LIFECYCLE_KIT_ROSTER_BASENAME"`, `grep -n "The filter field is" gate-sdk/SPEC.md`, `grep -n "adds no kit mechanism\|Honest limits\|OpenSpec:" companion/SPEC.md` and `grep -n "companion arm" installer/SPEC.md`, run 2026-09-27.

- `native/src/gates/stage_entry.rs`, `native/src/walk.rs`, `native/src/gates/reads_couples.rs`, `lifecycle-kit/gate-tests/check-stage-entry.test.sh`, lifecycle-kit/SPEC.md §Layout and configuration and §check-stage-entry, and gate-sdk/SPEC.md §check-reads-couples (delta 1).
- `companion/openspec/lifecycle/` and companion/SPEC.md §The component, §Recipes and §Honest limits (delta 2).
- `companion/fixtures/openspec/lifecycle/`, `installer/consumer-smoke/run-smoke.sh`, companion/SPEC.md §The fixtures and installer/SPEC.md §The consumer smoke (delta 3).
- docs/spec-toolkits.md and `companion/README.md` (delta 4).
- The on-site mirrors `docs/lifecycle-kit/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md` and `docs/companion/SPEC.md`, regenerated by the command `check-docs-mirror-fresh` prints (all deltas).
- `.workflow/release-declarations.md` gets one Behavior changes bullet. `LIFECYCLE_KIT_AMENDMENT_GLOB` matches a file's path when the value carries `/`, so a tree can mark its in-flight specs by location. A value with no `/` behaves as before. Nothing to do (delta 1).

## Retired spellings

- None — no name is retired; the basename match stays the rule for a glob with no `/`.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the path-matched amendment set, the `name:` token rule and the layer's three knob lines.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `foreign-spec-lifecycle-unowned` moves to Done in its landing commit, at a stage before the drain stage. Its oracles are local: the stage-entry test, the crate tests and the consumer smoke's companion arm.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
