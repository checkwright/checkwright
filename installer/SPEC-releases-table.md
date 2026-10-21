# SPEC amendment: releases-table

Pairs `releases-page-table`. `docs/releases.md` lists its release notes as a bare list of version links, a Liquid loop over the pages carrying `release:`. The nav already lists every note under the page's `nav_children_key: release`, so the list repeats it and tells a reader skipping several versions nothing about which need action. This amendment replaces the list with a table, one row per note: version, date, bump class, whether the release needs action on upgrade, and a count for each declaration-bearing section. The table is a generated marker block, built from the notes' own sections, that a freshness gate holds to the notes.

**What the tree does today**, read at spec:

- `docs/releases.md` is front matter, one paragraph and the `{% assign notes = site.pages | where_exp: "p", "p.release" | sort: "path" | reverse %}` loop rendering `<ul>`. Its paragraph claims "no second copy to maintain and no index to regenerate". No gate and no other surface reads the loop (`grep -rn where_exp` over the tracked tree, outside the docs mirror and posts).
- 31 notes under `docs/posts/` carry `release:`, v0.1.0 to v0.31.0. Each filename opens with its `YYYY-MM-DD-` date, and no two carry one version. Every version is `x.y.0`.
- Every published note carries its `gates`, `knobs` and `behavior` sections, under the aliases `Tightened gates` and `Renamed knobs` for the first two. None carries an `authoring` or `platforms` section, and none carries the In brief summary table, which binds a note under composition only (`grep -l "| Section | Entries" docs/posts/*.md` finds none). So for published history the table is the only place the counts are stated.
- Every `Renamed knobs` bullet on a published note is a rename, carrying `→` after its lead token: one on v0.24.0 and thirteen on v0.26.0. Under today's `Knob changes` grammar an addition is ``- `NEW_NAME` — new``, with no arrow (§The upgrade contract).
- `docs/releases.md` sits under `docs/llms.txt`'s `## Optional` section, so `docs/llms-full.txt` does not carry it. `check-surface-ratchet` governs it through `CONTEXT_KIT_RATCHET_PATHS`' `docs/*.md`, at a `789cp` ceiling.

**The mechanism, and why it departs from the entry as filed.** The filing proposed that the note composer write the counts and an action flag as front-matter keys on each note, with a gate holding them equal to the note's sections. That is a hand-written copy on every note. On a note under composition it would also be a second copy of the summary table the same note carries. A derivable copy is generated and freshness-gated rather than maintained, so this amendment generates the table instead, in the shape the toolkit support table and the value rollup already take. Parsing sections in Liquid at render time stays refused as fragile, as filed. The columns are the entry's.

## What changes

### (1) The releases table's contract {mechanical} {user-facing: the entry's operator question of 2026-09-29, lead-relayed, "a stats table instead", with the columns its Deliverable names}

`installer/SPEC.md` gains a subsection at the end of §The upgrade contract, after the paragraph opening **Those four residue classes map onto the sections by design** and its honest limit. **Not yet applied.**

> ### The releases table
>
> `docs/releases.md` carries one row per release note, between `<!-- release-table:begin -->` and `<!-- release-table:end -->` with a blank line inside each marker. `bash gate-sdk/bin/run-gates.sh --emit release-table` prints the block, and `--write` rewrites it and touches nothing else. A note is a file under `GATE_LOCAL_RELEASE_POSTS_DIR` carrying a non-empty `release:` key, so a note joins the table as it joins the nav. The page is `GATE_LOCAL_RELEASES_PAGE`.
>
> - **Rows** run newest first, ordered by version as `check-release-bump` orders notes (§Versioning).
> - **The header** is `| Release | Date | Bump | Action on upgrade |`, then the heading of each declaration-bearing roster role, in roster order.
> - **Release** is the note's `release:` value, linked to the note by its path relative to the page.
> - **Date** is the `YYYY-MM-DD` that opens the note's filename.
> - **Bump** is the component that moved from the next-older row, `major`, `minor` or `patch`, and `—` on the oldest row.
> - **Action on upgrade** is `yes` where the note's `gates` section carries a bullet, or its `knobs` section carries a rename or removal, a bullet whose lead token is followed by `→`. Otherwise it is `no`. A knob addition asks nothing of an upgrader, so it does not set the flag.
> - **A count cell** is the bullet count of that role's section, found under its heading or an alias, counted as the summary table counts it. A `None` body is `0`, and `—` marks a note carrying no such section because it was published before the section existed.
> - **Exit 2**, from the arm and from its gate, on an unreadable posts dir, note or page. It also exits 2 on a `release:` value outside `check-release-bump`'s version grammar, two notes carrying one version, a note filename not opening with a date, a roster the validator refuses, and a block that is absent or occurs twice.
>
> `check-release-table-fresh` (this repo's `scripts/`) compares the page's block with the arm's rendering in process. A stale block is one finding, and it prints the regen command. Its positional form is `check-release-table-fresh [posts-dir page]`, so a fixture tree stands in.
>
> The table is generated rather than maintained. Liquid would have to re-parse each note's sections at render time, and counts the composing session wrote into a note's front matter would be a hand-kept copy of its sections. The arm reads them through the declaration holder every release gate reads, so the table and the gates cannot disagree on what a section holds.

### (2) The gates section's reader roster names the table {mechanical}

Replacement for `installer/SPEC.md` §The upgrade contract, the sentence opening **The gates section is machine-read on published notes**. **Not yet applied.**

> The gates section is machine-read on published notes: `check-tightened-gates-grammar` walks every note, the upgrade smoke reads `TO`'s note out of `TO`'s archive, `check-release-bump` counts the newest note's sections, and the releases table counts every note's (§The releases table).

### (3) The releases page carries the table {mechanical} {user-facing: as delta 1}

`docs/releases.md`, below its front matter, which is unchanged, becomes the following. The marker block's body is the arm's output, written by `--write` and never by hand. **Not yet applied.**

> # Release notes
>
> Every tagged release carries one dated note — the phase-B checklist the [upgrade contract](installer/SPEC.md#the-upgrade-contract) names, its bump chosen by that file's [§Versioning](installer/SPEC.md#versioning) criteria. The GitHub Release for each tag points at the same note.
>
> The table reads every note for you, newest first. **Action on upgrade** says whether a clean upgrade may turn a gate red or leave a knob you set renamed or removed. Each column after it counts that section's entries, and a dash marks a note published before the section existed. The table is generated from the notes, so a note joins it by carrying its `release:` front-matter key.
>
> <!-- release-table:begin -->
>
> *(the arm's rendering)*
>
> <!-- release-table:end -->

### (4) The arm, the gate and their holders {design-bearing} {user-facing: as delta 1, for the arm name `--emit release-table` an adopter's binary carries}

All **Not yet applied.**

- **`native/src/emit/release_table.rs`** renders the block to delta 1's contract and rewrites it under `--write`. It finds notes with `release_bump::front_matter_release`, orders them with `release_bump::parse_version`, and counts sections with `declaration::section_bullets` under `release_sections::roster()`'s name sets. It forks none of these. For finding and rewriting the block, it reuses `marker.rs` where that module's semantics match delta 1's exit-2 cases, else it reuses the support table's reader. It never adds a third block reader.
- **`native/src/emit/mod.rs`** registers `--emit-release-table` in `ARMS` with `Grammar::Flags(&["--write"])`, declaring `GATE_LOCAL_RELEASE_POSTS_DIR`, `GATE_LOCAL_RELEASES_PAGE`, `GATE_SDK_RELEASE_SECTIONS` and `GATE_SDK_RELEASE_SECTION_ALIASES`.
- **`native/src/gates/release_table_fresh.rs`** is the gate, in the shape of `support_table_fresh.rs`, registered in `native/src/gates/mod.rs` with the same four knobs.
- **`scripts/check-release-table-fresh.gate`** carries a `# graph:` line whose `couples=` holds `docs/posts/*.md` and `docs/releases.md`, `dir=one valve=none tier=precommit`. It also carries `# projection: docs/releases.md`, a `# spec: installer/SPEC.md §The releases table` line, and `# knob: GATE_LOCAL_RELEASES_PAGE = docs/releases.md`. `GATE_LOCAL_RELEASE_POSTS_DIR` stays declared on `scripts/check-release-bump.gate` alone. The member is registered in `scripts/gates.list`.
- **A fixture pair** under `scripts/gate-tests/check-release-table-fresh/`. `good/` holds a posts dir of three notes and a page whose block the arm regenerated, never hand-edited. The three notes are one under alias headings, one carrying a knob addition and no rename, and one whose bump is a patch. `bad/` holds the same posts and a page whose block is one count stale.
- **Crate unit tests** in `release_table.rs` hold the derivations a pair cannot hold as separate verdicts:
  - each bump class, and `—` on the oldest row;
  - `yes` from a gates bullet, from a knob rename and from a knob removal, and `no` from a knob addition alone;
  - `0` for a `None` body against `—` for an absent section;
  - an alias-headed section counted under its role;
  - every delta 1 exit-2 case.
- **The new-gate fan-out** (docs/projection-fan-outs.md §The new-gate fan-out): `docs/enforcement.md`, `docs/value.md`'s rollup, `docs/check-graph.html`, `docs/llms-full.txt` and `.workflow/surface-ceiling.txt` regenerate. The roster row is delta 5. No kit owns the member, so no `smoke/install.sh` heredoc changes.

### (5) The generated-projections roster row {mechanical}

`docs/generated-projections.md` gains a bullet after **The toolkit support table**'s. **Not yet applied.**

> - **The releases table** <!-- projection: check-release-table-fresh --> — `docs/releases.md`'s marker block, one row per release note: `bash gate-sdk/bin/run-gates.sh --emit release-table --write` (`check-release-table-fresh` byte-gates it; [installer/SPEC.md §The releases table](installer/SPEC.md#the-releases-table) owns the rows). A note-authoring commit stales it, so the close that composes a note regenerates it in the same commit.

### (6) The release runbook names the regeneration {mechanical}

Replacement for `RELEASING.md` §The procedure, step 1, the sentence opening **The note joins the nav by its `release:` key with no further step**, through its end. **Not yet applied.**

> The note joins the nav by its `release:` key with no further step: the [Releases page](docs/releases.md) names that key in `nav_children_key`, so the note renders as a derived nav child, and no `scripts/docs-offnav.list` entry or allowlist growth is owed per release. Its row in that page's releases table is generated: `check-release-table-fresh` reds the note-authoring commit and prints the command that regenerates the table, which rides that commit ([installer/SPEC.md §The releases table](installer/SPEC.md#the-releases-table)).

### (7) The upgrade page points at the table {mechanical}

Replacement for `docs/maintenance.md` §Upgrading, the sentence "All notes are on the [releases page](releases.md)." **Not yet applied.**

> The [releases page](releases.md) tabulates every note, with whether each needs action on upgrade.

### (8) The page leaves the surface ratchet {mechanical}

After delta 3, the page grows one generated row per release, and its authored prose is a page header and two paragraphs. The ratchet's own rule leaves a generated copy out, because a copy's size is its source's. `scripts/context-config.knobs` gains `CONTEXT_KIT_RATCHET_PATHS[] = :(exclude)docs/releases.md` after the `docs/install-evidence.md` exclusion. The `comment-tier-exempt:` comment above that run then reads "…and the whole-file projections and the releases page, whose growth is a generated row per release, are excluded because their size is their sources'". `.workflow/surface-ceiling.txt` drops the page's row, re-stamped with `--emit always-loaded --ceiling`. **Not yet applied.**

### (9) The release declaration and the mirror {mechanical}

`.workflow/release-declarations.md`, under **Behavior changes**, gains one bullet. **Not yet applied.**

> - **`--emit release-table`** — a new gate-binary arm that prints, or with `--write` rewrites, the publishing repository's releases-page table. In a tree whose gates directory leaves `GATE_LOCAL_RELEASE_POSTS_DIR` or `GATE_LOCAL_RELEASES_PAGE` undeclared, it exits 2, naming the undeclared knob. Nothing to do; the arm serves the publishing repository's own releases page.

The docs mirror regenerates: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **The releases table** (deltas 1, 3 and 4). Its producer is the arm under `--write`, run by the build that lands this unit and by each close that composes a note, which the gate's red forces in the note-authoring commit (delta 6). The two knobs it reads are declared on `scripts/` descriptors this repo's battery registers, so the producer is reachable in a deployed configuration. Its consumers:
  - the reader of the published page;
  - Pages' renderer, a GitHub-Flavored Markdown table inside the page body;
  - `check-release-table-fresh`, comparing in process.
- **Each column has a named reader.** The upgrader reads **Action on upgrade** to choose which notes to open. They read **Release** to open one and **Date** and **Bump** to place it on the version line. The count cells say which sections a note fills. No cell is read by a machine.
- **The note's sections, read at a new transition.** The arm reads them through the holder every release gate reads, so it counts a section as those gates read it. It reads every note and refuses none for its sections' content, so it adds no red on a published note.
- **Roster-holding readers, each red condition named:**
  - `check-projection-roster` reds a `# projection:` line with no keyed row on the roster, and a keyed row with no declaring gate. Deltas 4 and 5 land together.
  - `check-docs-render-fidelity` reds a page whose rendered tables number fewer than its source's GFM table starts. The block opens its table after a blank line, so it renders as one.
  - `check-md-refs` reds a link resolving to no file. Each Release cell links `posts/<file>.md`, the form `docs/index.md` already uses for a post.
  - `check-docs-nav-reachable` reads `nav_children_key`, which delta 3 leaves in place.
  - `check-surface-ratchet` reds a governed file above its ceiling. Delta 8 leaves the page ungoverned, and a row for an ungoverned file is ignored.
  - `check-amendment-queue` reds a `[spec:]` ref resolving to no file. The paired entry carries this file's name.
  - `check-docs-mirror-fresh` reds the installer mirror until it regenerates (delta 9).
  - The new-gate fan-out's surfaces each red their own staleness (delta 4).
  - `check-release-declaration-parity` holds the next composed note's Behavior changes tokens equal to the surface's. Delta 9's bullet is on the surface, so that note transcribes it.
- **A note under composition.** Its row joins the table in the commit that authors it, before its tag, exactly as it joins the nav today. Its count cells are the bullets its sections carry in that commit.

## Existing sections updated

- `installer/SPEC.md` §The upgrade contract, a new closing subsection (delta 1), and its sentence naming the gates section's machine readers (delta 2).
- `docs/releases.md` (delta 3).
- `native/src/emit/release_table.rs`, `native/src/emit/mod.rs`, `native/src/gates/release_table_fresh.rs`, `native/src/gates/mod.rs`, `scripts/check-release-table-fresh.gate`, `scripts/gates.list` and `scripts/gate-tests/check-release-table-fresh/` (delta 4).
- `docs/enforcement.md`, `docs/value.md`, `docs/check-graph.html` and `docs/llms-full.txt` (delta 4), named by docs/projection-fan-outs.md §The new-gate fan-out.
- `docs/generated-projections.md` (delta 5).
- `RELEASING.md` §The procedure, step 1 (delta 6).
- `docs/maintenance.md` §Upgrading (delta 7).
- `scripts/context-config.knobs` and `.workflow/surface-ceiling.txt` (deltas 4 and 8).
- `.workflow/release-declarations.md` (delta 9).
- `docs/installer/SPEC.md`, the generated mirror (all deltas).

The page's readers were found by `grep -rn "releases.md\|where_exp"` over the tracked tree, outside the docs mirrors and the posts. The surfaces it reaches in the ratchet and the agent index were found by reading `scripts/context-config.knobs`, `.workflow/surface-ceiling.txt` and `docs/llms.txt`.

## Retired spellings

- None — no delta retires a name; the Liquid loop delta 3 drops is page markup no other surface spells.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The entry moves** — `releases-page-table` moves to Done with the merge, before the drain stage, once the mid-iteration push's `pages-build-deployment` run is green, per build's remote-oracle rule.
