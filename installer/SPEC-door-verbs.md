# SPEC amendment: door-verbs

The front door advertises commands an adopter runs now: the one-line install with a verb, its PowerShell script-block form, and `npx checkwright <verb>`. Every route resolves to the newest published release (§The hosted install pin holds the hosted scripts there, and npm's latest is the same tag), so a verb the front door advertises must be one that release carries. Nothing holds that today: `demo` is advertised on four pages while the pin names 0.25.0, which has no `demo`.

**The ruling: a repo-local gate, not a policy line alone.** `check-front-door-verbs` holds every route-advertised verb to the pinned release's verb set. A verb the running binary carries and the pin lacks is admitted while the closing iteration has not yet stamped its release disposition. Once the disposition is `none` or `deferred:`, it reds. So the gate is the enforcement of a release trigger: a close whose front door advertises an unreleased verb either releases or withdraws the advertisement. The close binding names the trigger, so the close reads it as policy before it meets the red.

**The seam.** This is this repository's config, not kit mechanism. The page roster, the route spellings and the hosted pin are this project's front door, so the gate is repo-local like `check-install-pin`, specified in installer/SPEC.md, and no kit ships it.

**Refused: holding advertised verbs to the pin at every commit, with no pending admission.** The tree is red on that reading from the landing commit until the close's drain moves the pin. A verb lands in build and its release is cut at close, so no build could commit through it.

**Refused: a dormancy that skips a pinned tag carrying no machine-readable verb roster.** `native/src/installer/mod.rs`'s `VERBS` is absent at v0.25.0, where the verbs were `installer/lib/*.sh`. A dormancy keyed to that absence would turn the gate green on exactly the defect it exists for. The pinned set is read instead from `installer/README.md`'s verb table, which both substrates carry. That table is the npm package's own README, and this unit holds it to `VERBS` from here on.

**Refused: release-policy prose alone.** A trigger bullet with no gate is the flagged-and-skipped shape. The done-claim-demo close read the defect and deferred anyway, which is the measurement.

**Measured at authoring:**

- **The pinned release lacks `demo`.** `git show v0.25.0:installer/README.md` carries a `| verb | asks | reads |` table listing `init`, `doctor`, `diff`, `update` and `uninstall`, and `git ls-tree v0.25.0 installer/lib/` lists one `.sh` per verb and no `demo`.
- **The table is `VERBS` at HEAD.** `installer/README.md` §What you can run lists the six verbs `native/src/installer/mod.rs`'s `VERBS` carries, in the same order.
- **Ten route-advertised `demo` sites.** `grep -n -o -E '(sh -s --|install\.ps1\)\)\)|npx checkwright|`checkwright) +[^ `]+'` over `README.md docs/index.md docs/install.md installer/README.md` finds `demo` at README.md 14, 18 and 22, docs/index.md 19, 23 and 27, docs/install.md 18 (twice) and 44, and installer/README.md 21. Every other token it finds is a released verb, a flag, or the `<verb>` placeholder on docs/install.md 142.
- **The readers exist.** `native/src/gates/release_channel_parity.rs` exports `newest_tag`, and `native/src/gates/install_pin.rs`'s `pin_of` reads each hosted script's pin line. `.workflow/release-disposition.txt` carries one line per closing iteration, `<iteration> release <field> — <basis>`, the field being `vX.Y.Z`, `none` or `deferred:vX.Y.Z` (lifecycle-kit/SPEC.md §templates/stages/).

## What changes

### (1) The contract: the front door advertises what the pin carries {design-bearing}

**Not yet applied.** installer/SPEC.md gains a subsection after §The hosted install pin:

> ### The front door's verbs
>
> Every route on the front door resolves to the newest release, so a verb the front door advertises must be one that release carries. `check-front-door-verbs` (this repo's `scripts/`) holds two invariants:
>
> - **Invariant A — the verb table is the binary's roster.** The first-column code spans of `installer/README.md`'s verb table (the table whose header row's first cell is `verb`) equal, as a set, the verbs the binary's own `VERBS` carries, read in-process. The table is the npm package's README, so a release carries its own verb set in a file every tag holds.
> - **Invariant B — an advertised verb is released.** A **route** is `sh -s --`, `install.ps1)))`, `npx checkwright`, or `checkwright` opening a code span. The token after a route, inside a code span or a fenced code line on a front-door page, is an advertised verb when it matches `[a-z][a-z0-9-]*`. A leading-dash token is a flag and advertises the default verb, and a placeholder such as `<verb>` advertises none. The front-door pages are `README.md`, `docs/index.md`, `docs/install.md` and `installer/README.md`. Each advertised verb must be in the **pinned set**: invariant A's table read at the tag `v<pin>`, where `<pin>` is §The hosted install pin's value.
>
> **The pending admission.** A verb B would red is admitted while it is in `VERBS` and `.workflow/release-disposition.txt` carries no line for the iteration the queue header names. It is also admitted when that line's field is a release, `vX.Y.Z`, because that release is cut from this tree. A `none` or `deferred:` field reds it. The admission makes B a release trigger held at the close's disposition commit: an advertised verb the pinned release lacks leaves the close two remedies, releasing or withdrawing the advertisement. A verb in neither `VERBS` nor the pinned set is never admitted, since no release will carry it.
>
> The gate fails closed, exit 2, on an unreadable page, on a verb table that is absent or empty at HEAD or at the tag, on a tag that exists and carries no `installer/README.md`, and on this iteration's disposition line when its field parses as none of the three forms. The pin's own refusals are §The hosted install pin's. Where the tag `v<pin>` does not resolve, as in a shallow checkout, B is dormant and the clean line says so. A still runs. Its positional form, `check-front-door-verbs [readme pinned-readme disposition queue page...]`, points it at a fixture tree, where the pinned table is read from a file so the fixture holds still as the tags move.
>
> **Honest limits.** A verb named in a code span apart from its route, as in "or `demo`", is out of reach. The gate holds the release decision, not the live site: between a push carrying a new advertisement and the close's tag, the site advertises a verb the one-liner cannot run. The pin moves in the commit after the tag (RELEASING.md step 4), so the live one-liner reaches the new release on the push carrying that commit.

### (2) The gate {design-bearing}

**Not yet applied.** `native/src/gates/front_door_verbs.rs`, declared by `scripts/check-front-door-verbs.gate` and registered in `scripts/gates.list` after `check-install-pin`:

- The descriptor carries `# graph: couples=README.md,docs/index.md,docs/install.md,installer/README.md,docs/install.sh,docs/install.ps1,.workflow/release-disposition.txt dir=one valve=none tier=precommit` and a `# spec:` line pointing at delta 1's section.
- The pinned set is `git show v<pin>:installer/README.md`, run through the one program resolver. The pin comes from `install_pin::pin_of` over `docs/install.sh`, made `pub(crate)`, and tag existence from `git rev-parse --verify --quiet refs/tags/v<pin>`.
- The iteration comes from the queue header through the crate's queue reader, at the queue file `QUEUE_KIT_QUEUE_FILE` names. The disposition file is `<GATE_SDK_WORKFLOW_DIR>/release-disposition.txt`, the derivation `native/src/gates/release_bump.rs` already makes, lifted to one shared function rather than spelled twice.
- The verb table reader is one function over markdown text, called for both the HEAD file and the tag's blob, so the two sides cannot parse differently.
- Output follows gate-sdk/SPEC.md §Output contract. A finding names `<page>:<line>`, the verb, and which reading failed: not in the pinned set with a `none` or `deferred:` disposition, or in neither set. An A finding names the verb and the side missing it. Each class takes its own `help:` line. B's names the two remedies, releasing (RELEASING.md) or withdrawing the advertisement. A's names the table and `VERBS`. The clean line counts pages, advertised sites, the pinned set's version, and any verb admitted as pending, by name.
- Unit tests cover the route tokenizer (each route, a flag, a placeholder, a token outside any code span), the table reader, and the three disposition forms plus the absent line.

### (3) The fixture pair {mechanical}

**Not yet applied.** `scripts/gate-tests/check-front-door-verbs/`, in the positional form:

- `good/` has one page advertising a pinned verb through each of the four routes, a flag route, and a placeholder. A second page advertises `demo`, absent from the pinned table, with the queue's iteration carrying no disposition line, which is the pending admission.
- `bad/` has a page advertising `demo` against a pinned table lacking it and a disposition line `<iteration> release deferred:v9.9.9 — <basis>`. It also has a route naming a token in neither set, and a current table missing one `VERBS` verb. That is three findings across two classes.

The current table in both cases lists `VERBS`' verbs, so a verb added to the binary also changes these fixtures. That coupling is the cost of reading A in-process.

### (4) The close binding names the trigger {mechanical}

**Not yet applied.** In `.claude/commands/close.md`'s release-policy trigger list, after **Explicit operator direction**:

> - **A front-door verb the pinned release lacks.** `check-front-door-verbs` reds a `none` or `deferred:` disposition while the front door advertises one, so release, or withdraw the advertisement in the same close.

## Producers and consumers

- **The pending admission's state** — a verb in `VERBS`, outside the pinned set, with no disposition line for the iteration.
  - Producer: a build that adds or advertises a verb. It is enabled on this tree, where `demo` sits in it at landing.
  - Consumer: the gate's B arm, which reads the disposition file at the close's disposition commit. There a `none` or `deferred:` field reds it, and a `vX.Y.Z` field leaves it admitted until the drain commit moves the pin past it.
  - Red condition: a `none` or `deferred:` line for the iteration, or a verb in neither set.
- **The disposition line**, an existing surface with a new reader. Its producer is close's release-disposition step. Its readers are the boundary entry (lifecycle-kit/SPEC.md §bin/enter-stage.sh), `check-release-bump`, and now this gate, which reads only the line keyed by the queue's iteration and only its field.
- **The verb table's new obligation (A).** Its producer is whoever edits `VERBS` or `installer/README.md`. Its consumer is the gate at HEAD, and at the tag as B's pinned set once the release carries it. Each verb's satisfying value is one table row, and the six rows at HEAD satisfy it.
- **Downstream.** At this iteration's close, disposition `release v0.26.0` keeps B green. The drain commit moves the pin to 0.26.0, whose tag carries `demo` in the table. After the scope boundary truncates the disposition file, B reads the pinned set alone.

## Existing sections updated

Roster probe: `git grep -n "check-install-pin"` over the tracked tree, for the sites that name the sibling gate's placement and registration.

- `installer/SPEC.md` — the new subsection after §The hosted install pin (delta 1).
- `native/src/gates/front_door_verbs.rs`, `native/src/gates/mod.rs`'s gate table, `native/src/gates/install_pin.rs`'s `pin_of` visibility, and `native/src/gates/release_bump.rs`'s disposition-path derivation lifted to the shared function (delta 2).
- `scripts/check-front-door-verbs.gate`, `scripts/gates.list` (delta 2).
- `scripts/gate-tests/check-front-door-verbs/` (delta 3).
- `.claude/commands/close.md` — the release-policy trigger list (delta 4).
- `docs/enforcement.md`, `docs/value.md`'s rollup block (its `(consumer)` row's Blocking-gates count), `docs/check-graph.html`, `scripts/git-hooks/pre-commit`, `docs/installer/SPEC.md` — generated projections, regenerated by the command each freshness gate prints on red (deltas 1 and 2). The new-gate fan-out (docs/site-architecture.md §Generated projections and their freshness gates) is the checklist; `docs/footprint.md` is exempt on that section's own ground, a gate being neither a script nor a crate module it measures.

## Retired spellings

- None — the unit adds a gate and a trigger and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The gate runs on this tree** — the full battery green with `check-front-door-verbs` registered, its clean line naming `demo` as pending against the pinned 0.25.0, and its fixture pair green. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`). The release that carries `demo` is the close's disposition, not this entry's completion.
