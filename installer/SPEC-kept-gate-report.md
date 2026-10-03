# SPEC amendment: kept-gate-report

**When `init` keeps a `gates.list` you edited, it also reports each member it registered there that its own registry no longer carries, a gate whose install disposition moved off `zero-config` among them.** Today the kept-registry report (`payload_recipe::report_kept`, called from `native/src/installer/init.rs` where `claim` keeps the registry) prints only the recipe drops and `--without-gate` drops the kept file still registers. A member the derivation stopped starting is in neither set, so an adopter who edited the file keeps it registered and is told nothing (§What init seeds, *A disposition change reaches trees already installed*).

**The prior starting set is already recorded, so the lock gains no field.** The queue entry assumed the lock must record the set init registered, because a member you registered deliberately looks the same as one `init` registered. But `files` already holds `gates.list` at the hash `init` last wrote there, and that hash is carried forward unchanged while you keep the file (§The manifest). It is a filtered `git hash-object`, the same id `git add` stores. `init` stages what it writes, so the blob is in the object store, and `git cat-file blob <hash>` returns the registry `init` last wrote. Checked in a scratch repository: `git hash-object gates.list` printed the id that `git ls-files -s` showed staged. After the file was edited and staged again, `git cat-file blob <id>` still printed the first content, exit 0.

## What changes

### (1) A kept registry's retired members are reported {design-bearing} {user-facing: the lead decision recorded on kept-gate-disposition-report — a report of kept members the derivation no longer starts}

`native/src/installer/init.rs`, the kept branch of the registry write, and `native/src/installer/payload_recipe.rs`.

When `claim` keeps the registry, `init` reads the **registry it last wrote** by passing `prior`'s `gates.list` hash to `git cat-file blob`. A member is **retired** when it meets all four conditions:

- the kept file registers it;
- the registry `init` last wrote registers it;
- the registry this run plans does not;
- it is not in `dropped`.

Members are compared with `crate::registry::members`, the reader that already compares the kept file with `dropped`. A member in `dropped` keeps its `drop:` line, so it is reported once. `--dry-run` reaches the same branch and prints the same report.

`report_kept` takes the retired set as a third list, which the two seam call sites pass empty, and prints one line per member:

      retire: check-foo — init's registry no longer starts it (# install: on-surface)

The parenthetical gives the member's disposition, read from its declaration, which is resolved the way §doctor resolves a member for its `# armed-by:` line. It is left off when the declaration does not resolve or carries no `# install:` line. A kit dropped with `--without-kit`, or a `--with-gate` that a replaced selection no longer carries, shows up as a retired member with no `on-surface` parenthetical. That is true: `init`'s registry no longer starts it.

The report's header becomes `<file> is yours, so init's change is not placed there:`, since a retired member is not a recipe or selection change. The help line names every remedy:

- add or remove the lines by hand;
- keep a retired gate deliberately with `--with-gate <gate>`, which the manifest records, after which it is no longer retired;
- or pass `--force` to take `init`'s file.

The report holds across runs: the recorded hash stays `init`'s while you keep the file, so the member is reported until you remove it or name it with `--with-gate`.

**Fallback.** When the recorded `gates.list` row is absent, the step classifies nothing. When `git cat-file` fails because the blob is gone, `init` prints one line and classifies nothing:

      note: the registry init last wrote (<hash>) is not in this repository's object store, so kept members are not classified

The blob is lost only after `init --no-commit` staged a registry that you edited before committing, and git's pruning then collected it. That loss ends no run.

Unit tests in `payload_recipe.rs`, for the set function:

- a member in all three registries is not retired;
- a member the plan dropped is retired;
- a member in `dropped` is reported as a drop, not as retired;
- a member only the kept file carries is not retired;
- an empty prior registry retires nothing.

### (2) The selection arm witnesses a retired member {mechanical}

`native/src/emit/installer_smoke/moves.rs`, the `selection` arm. The step goes after the `--no-selection` assertions and before the three refusals:

1. Re-run with the arm's original selection flags. Its `--with-gate` member resolves only through the `--with-kit` kit, so both are passed.
2. Append a comment line to `gates.list` and commit it, so the registry is kept.
3. Re-run with `--no-selection`. Assert that the output carries `retire: <member>` and that the kept file still registers the member.
4. Re-run with the original selection flags again. Assert that no `retire:` line names the member.

The member's name is read off the package, as the arm already reads it. The arm's summary line names the witness.

### (3) The owning sections state the report {mechanical}

installer/SPEC.md. **Not yet applied.**

- §What init seeds, the paragraph *A disposition change reaches trees already installed*: its last two sentences become *A gate moving the other way leaves an unedited `gates.list` at that run. One you edited keeps it, and `init` reports it as retired (§Payload recipes), so a release carrying such a move still owes the sentence in its note only to a tree vendored without the installer.*
- §Payload recipes, the paragraph *A kept file is reported, never merged into*: its second sentence becomes *It prints each recipe line the kept seam lacks, each dropped gate the kept registry still registers, and each **retired** member: one the kept registry and the registry `init` last wrote both carry, and this run's registry does not.* A following sentence names that registry's source: the `gates.list` blob at the hash `files` records, read with `git cat-file`. A further sentence states the `note:` line for a blob gone from the object store. Its remedy sentence gains `--with-gate <gate>` to keep a retired gate deliberately.
- §The manifest, the paragraph *A recorded hash is what `init` last wrote at that path*: one sentence is added. For `gates.list`, that hash is also how `init` reads its previous registry (§Payload recipes).
- §The consumer smoke, the paragraph *The selection arm*: one sentence before the refusals states the step in delta 2.

### (4) The release declaration and the mirror {mechanical}

- `.workflow/release-declarations.md` gains a Behavior changes bullet in its grammar. Its lead is **checkwright init, update**, followed by: a kept `gates.list` now reports each gate `init` no longer registers there, a disposition moved off `zero-config` among them; remove the line, or keep the gate with `--with-gate`.
- `docs/installer/SPEC.md` is regenerated with the command its freshness gate prints, in the commit landing delta 3.

## Producers and consumers

- **The retired set.**
  - Producer: `init`'s kept-registry branch, on every `init` and `update` that keeps an edited registry. The enabling state is an install whose lock records `gates.list`, which every install since the manifest existed carries.
  - Consumer: `report_kept`, on stdout, read by the adopter.
  - The smoke's selection arm reads the `retire:` line. It is its only machine reader, by `grep -rn "is yours, so\|still_registered\|report_kept" native/src installer`. That grep found three `report_kept` call sites in `init.rs`, the registry's and two seam sites, and no smoke reader today.
- **The `gates.list` blob read.**
  - Producer: `git add` of the registry `init` writes, an existing act.
  - Consumer: the new `git cat-file blob` call. No other reader changes.
  - The `files` hash keeps its meaning, so the wire key stays `checkwright-lock v1`, and no lock reader changes: `Manifest::read`'s callers in `init.rs`, `doctor.rs`, `diff.rs` and `uninstall.rs`, and the smoke's `Lock` wrapper.
- **Fields:** none is added. Point 4 is vacuous.
- **Red conditions (point 5):** no corpus narrows. The selection arm gains an assertion. It reds when the `retire:` line is absent after the kept `--no-selection` run, or present after the `--with-gate` run.

## Existing sections updated

Roster by `grep -n "kept\|is yours\|disposition change\|still registers" installer/SPEC.md` and `grep -rn "report_kept\|still_registered" native/src`, over the tracked tree.

- `native/src/installer/init.rs` — the kept branch's retired set and blob read (delta 1).
- `native/src/installer/payload_recipe.rs` — `report_kept`'s third list, header, help line and the set function's tests (delta 1).
- `native/src/emit/installer_smoke/moves.rs` — the selection arm's kept-registry step (delta 2).
- `installer/SPEC.md` — §What init seeds, §Payload recipes, §The manifest and §The consumer smoke, one paragraph each (delta 3).
- `.workflow/release-declarations.md` — the Behavior changes bullet (delta 4).
- `docs/installer/SPEC.md` — the regenerated mirror (delta 4).

## Retired spellings

- None — the report's header sentence is printed text no surface spells, and no name is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — no template or agent definition is touched.
- [ ] **Merged with no information lost** — §Payload recipes states the kept-registry report whole, its three line classes and its blob source.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`), discharged at the iteration while sibling installer amendments are in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery, and the consumer smoke (`--run-consumer-smoke`) with the selection arm's new step.
- [ ] **The entry is done** — `kept-gate-disposition-report` moves to Done in the commit landing delta 3, before the drain stage.
