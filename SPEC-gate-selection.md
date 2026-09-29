# SPEC amendment: gate-selection

An adopter cannot choose which kits or gates `init` installs, or replace one with their own. `init` takes a profile and recipes and nothing finer (installer/SPEC.md §init). The profiles are fixed rosters, a recipe is publisher-shipped, and a hand edit to `gates.list` survives a re-run only as an adopter-changed file that then stops receiving the registry's updates (§What init seeds). The registry already resolves consumer-first, so a gate of the same name in the adopter's gates directory shadows a kit's (gate-sdk/SPEC.md §Layout and configuration). No adopter page presents that as the replace path.

This amendment adds an install-time selection: `init` and `update` flags that add or remove a kit from the profile's set and a gate from the derived registry. `checkwright.lock` records the selection, so a re-run and `update` re-apply it. The replace path is documented.

One queue entry pairs it: [install-gate-selection](TASK-QUEUE.md#install-gate-selection).

**The rulings.**

- **Selection is consumer config applied after the profile, as a recipe is.** It sits on the recipes' footing in §Payload recipes: the profile lattice is a claim about the registries profiles write, and the consumer smoke asserts it over selection-free installs. So a selection off the lattice leaves the lattice intact.
- **The recorded-set precedent is the recipes'.** A run passing no selection flag re-applies the recorded selection. A run passing any replaces it whole, and `--no-selection` clears it. So the command line that made an install states its whole selection, and a re-run cannot half-merge two.
- **gate-sdk cannot be removed.** It is the runner, the hook generator and the registry every other kit's gates resolve through (`installer/profiles.list`'s starter criterion), so removing it leaves a tree with no battery.
- **A gate is added by the registry's own resolver.** `--with-gate` admits a name the registry would resolve on this tree: a vendored kit's `checks/` member, or the adopter's own gate in the gates directory. That second case is the add-your-own path, and it survives re-runs where a hand-edited `gates.list` stops receiving updates. A kit gate declared `never` is refused, since its kit declares that it cannot hold on a vendored tree (gate-sdk/SPEC.md §The install disposition).
- **A removed kit's files stay.** `init` never deletes (§The manifest), so removing a kit on a re-run takes it out of `kits`, `GATE_SDK_KIT_DIRS` and the registry, and leaves its directory on disk and on the roster, as the smoke's narrowing arm already shows for a smaller profile.
- **The seam.** Repo-root only: the installer's crate module, SPEC, README and smoke, and `docs/install.md`. No kit surface changes, and no gate or kit name enters the crate: selection names are adopter data read from argv and the manifest.

**Sequencing.** This lands after [front-door-flag-grammar](TASK-QUEUE.md#front-door-flag-grammar), so its flags meet the flag table (delta 5), and before [installer-install-brevity](TASK-QUEUE.md#installer-install-brevity), which passes over the sections this rewrites.

## What changes

### (1) The selection flags {design-bearing}

**Not yet applied.** `init` gains five flags, and `update` forwards them unchanged (§update):

- `--with-kit <kit>`, repeatable: adds a payload kit to the profile's kit set;
- `--without-kit <kit>`, repeatable: removes one;
- `--with-gate <gate>`, repeatable: registers a gate the derived registry does not carry;
- `--without-gate <gate>`, repeatable: drops one from it;
- `--no-selection`: clears the recorded selection.

The `<flag>=<value>` form is accepted for the four that take an operand, as it is for `--profile`.

**The composition.** The vendored kit set is the profile's set, plus each `--with-kit`, less each `--without-kit`, in payload order. The registry is the derivation over that set (§What init seeds), less every recipe drop and every `--without-gate`, plus every `--with-gate`. An explicit `--with-gate` therefore outranks a recipe's `unregister.list`. That set is what `init` vendors, declares in `GATE_SDK_KIT_DIRS`, records as `kits`, and hands `doctor` as the selection (§doctor), so the toolchain floor is the selected set's.

**Refused before any write, at exit 2**, each with a `help:` line naming what the package or tree does carry:

- a kit the payload does not carry;
- `--without-kit gate-sdk`;
- a `--with-gate` name the registry's resolver would not resolve on this tree, or whose resolved declaration reads `# install: never`;
- a `--without-gate` name that no vendored kit's `checks/` ships and the gates directory does not carry, which is a typo rather than a choice;
- one name given to both `--with-kit` and `--without-kit`, or to both `--with-gate` and `--without-gate`;
- `--no-selection` beside any other selection flag.

A `--without-gate` naming a shipped gate the derived registry does not carry, such as an `on-surface` member, is no refusal: the plan reports it, and it is recorded, so the drop holds once the member is registered some other way.

`init --help` gains a `kits:` line listing the payload's kits beside the profiles and recipes it lists.

`--dry-run` plans the selected kit set, the added and dropped gates and the recorded selection through the same code, and prints them.

### (2) The manifest records the selection {design-bearing}

**Not yet applied.** `checkwright.lock` gains an optional top-level `selection` object, absent when nothing is selected. It holds up to four arrays: `with-kits`, `without-kits`, `with-gates` and `without-gates`. Each is present only when non-empty and holds names in the order given, and keys sort at every level as the emitter already rules.

installer/SPEC.md §The manifest's table gains a row:

> | `selection` | the kits and gates added to or removed from the profile, or absent when none were | a re-run of `init` re-applies it when no selection flag is passed; `doctor` reports it beside the profile and recipes |

The paragraph on additive keys gains `selection` beside `recipes`. A release built before it re-applies no selection, so it reverts the registry and kit set to the profile's. The file protection keeps the adopter's edits, and the downgrade refusal stands in front of that run. The residual manifest's dropped-field list gains `selection`.

### (3) The rewritten install sections {design-bearing}

**Not yet applied.** installer/SPEC.md:

- **§init**, the opener: *vendors the selected profile's kit source* becomes *vendors the selected kit set: a profile's, adjusted by any selection (§Selecting kits and gates)*. The *resolves the profile's kit set, and refuses a profile resolving to no kit* sentence of the `doctor` precondition becomes *resolves the selected kit set, and refuses one resolving to no kit or a selection it cannot honour*.
- **A section `## Selecting kits and gates`** is added after §Payload recipes. It holds delta 1's composition, refusals and re-run rule, and the replace path:

  > **Replacing a gate is shadowing, and needs no flag.** A gate file of the same name in your gates directory, `<name>.sh` or `<name>.gate`, resolves before every kit's (gate-sdk/SPEC.md §Layout and configuration). `init` never writes it, so it is yours across every re-run, `diff` and `uninstall` pass it by, and an upgrade's note names a kit gate that tightened under your shadow in its Tightened gates (§The upgrade contract). A gate of your own under a new name is added with `--with-gate`, which the manifest records.

- **§Payload recipes**, the refusal *a recipe `.knobs` file naming a seam file the resolved profile does not write* becomes *… the selected kit set does not write*. Its lattice paragraph gains *A selection stands on the same footing (§Selecting kits and gates).*
- **§What init seeds**: *One function unions the result over a profile's kits, and it has exactly one caller: the registry `init` writes* becomes *One function unions the result over a kit set, and its one caller is the registry `init` writes, which the recipe drops and the selection then adjust.* The seam-can-express paragraph's *subtraction in the band between them has nothing to derive from* is unchanged: a selection is adopter data, not a derivation.
- **§doctor**, the `init` bullet: *hands the profile's kit set, and a gate set uniting `profile::gate_set` for that profile with …* becomes *hands the selected kit set, and a gate set uniting the registry it will write with …*. The inside-an-install bullet's report list gains *the selection*.
- **§Profiles** gains one sentence after the lattice paragraph: *A selection adjusts a profile's set per install and is not a profile (§Selecting kits and gates).*

`native/src/installer/init.rs` parses and composes. `profile.rs` keeps resolving a profile, and a selection step between it and `recipe::gates` applies the composition. `lock.rs` emits and reads `selection`, and `doctor.rs` reports it.

### (4) The adopter pages {mechanical}

**Not yet applied.** `docs/install.md` §Choosing a profile gains, after the profile list's closing paragraph:

> **Choosing kits and gates.** `--with-kit` and `--without-kit` add a kit to your profile or take one out, and `--with-gate` and `--without-gate` do the same for a gate. Each repeats, `checkwright.lock` records them, and a re-run keeps them until you pass new ones or `--no-selection`. `gate-sdk` stays, since it runs the others. To replace a gate with your own, put a gate of the same name in your gates directory: it runs instead of the kit's, and `init` never overwrites it.

`installer/README.md` §Choosing a profile gains, after its last paragraph: *`--with-kit`, `--without-kit`, `--with-gate` and `--without-gate` adjust a profile per install, and a gate file of your own under a kit gate's name, in your gates directory, replaces it ([SPEC.md §Selecting kits and gates](SPEC.md#selecting-kits-and-gates)).*

### (5) The flag roster {mechanical}

**Not yet applied.** If [front-door-flag-grammar](TASK-QUEUE.md#front-door-flag-grammar) landed in an earlier batch or lands in this one, `FLAGS` and `installer/README.md`'s flag table gain five rows, each naming `init` and `update`: `--with-kit`, `--without-kit`, `--with-gate`, `--without-gate` and `--no-selection`. `init`'s and `update`'s `usage:` lines gain them. If it lands later, its delta 1 reads these rows off the parsers, and its crate test holds them.

### (6) The consumer smoke's selection arm {design-bearing}

**Not yet applied.** `installer/consumer-smoke/run-smoke.sh` gains a selection arm. It names no gate or kit literally, and reads each name off the package: the payload's kit roots, the derived registry and each gate's `# install:` line. It runs over one consumer at the lattice minimum:

1. `init` with `--with-kit` naming a payload kit outside the minimum's set, `--without-gate` naming the first member of the minimum's registry, and `--with-gate` naming an `on-surface` member of the added kit. It asserts one commit, `kits` holding both kits, the registry lacking the dropped member and carrying the added one, the manifest's `selection` equal to what was passed, and the battery's resolved kit roots equal to `kits`. The battery is not asserted green, since an `on-surface` member may red on a tree lacking its surface.
2. A bare re-run leaves the tree object unchanged.
3. A re-run with `--no-selection` restores the minimum's registry and `kits`, records no `selection`, and leaves the added kit's directory on disk.
4. `--without-kit gate-sdk`, an unknown kit and an unresolvable `--with-gate` each exit 2 and write nothing.

installer/SPEC.md §The consumer smoke gains the arm's paragraph, and the script's header comment its clause.

## Producers and consumers

Probe: `native/src/installer/{init,profile,recipe,payload_recipe,lock,doctor,update}.rs` read for the kit-set, registry and manifest paths; `git grep -n -i "shadow"` over `docs/install.md`, `installer/README.md` and `gate-sdk/README.md`, which found no adopter mention; gate-sdk/SPEC.md §Layout and configuration line 24 for the resolution order; §The install disposition for `never`.

- **The selection flags** (delta 1). Producer: the adopter's argv, through the bootstrap to `init`, or forwarded by `update`. Consumers: the composition step, the refusals and the `--dry-run` plan.
- **`selection`** (delta 2). Producer: `init`'s manifest emitter. Readers: `init`'s re-run, on no selection flag; `doctor`'s identity block; the smoke's arm. A reader older than it ignores it. `uninstall` does not read it, since the roster is `files`.
- **The selected kit set.** Readers: the vendoring loop, the seam derivation, the `GATE_SDK_KIT_DIRS` write, `kits`, and `doctor`'s floor. So the consumer smoke's count assertion, the battery's kit roots against `kits`, holds on a selected install too.
- **The adjusted registry.** Reader: the battery. `check-install-disposition` assertion C is unaffected: the crate still holds no gate name.
- **Red conditions for adopters.** No existing install changes: an install with no selection records none, and a re-run passing none applies none.

## Existing sections updated

Roster probe: the probe above, plus `git grep -n "profile's kit set\|resolved profile\|gate_set" -- installer native/src/installer docs/install.md`.

- `installer/SPEC.md` — §init, §Selecting kits and gates (new), §Payload recipes, §What init seeds, §doctor, §Profiles (delta 3); §The manifest (delta 2); §The consumer smoke (delta 6).
- `native/src/installer/init.rs`, `profile.rs`, `lock.rs`, `doctor.rs` (deltas 1, 2 and 3).
- `native/src/installer/mod.rs` and `update.rs`, where `FLAGS` exists (delta 5).
- `docs/install.md`, `installer/README.md` (delta 4, and delta 5's flag table).
- `installer/consumer-smoke/run-smoke.sh` (delta 6).
- `docs/installer/SPEC.md` and `docs/installer/README.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `.workflow/surface-ceiling.txt` — the grown `installer/SPEC.md`, `installer/README.md` and `docs/install.md` rows re-stamped with `--emit always-loaded --ceiling` in the growing commit (deltas 2, 3, 4 and 6).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **`init` and `update`**: five selection flags, recorded under the manifest's new `selection` key (deltas 1 and 2).

## Retired spellings

- None — the deltas add flags, a manifest key, a section and a smoke arm, and re-phrase prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery, `cargo test` and the installer consumer smoke green on the landing commit, and the gates workflow's install-smoke legs green on the mid-iteration push. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
