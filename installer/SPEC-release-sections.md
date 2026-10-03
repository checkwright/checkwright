# SPEC amendment: release-sections

**A release note's section set becomes one knob-owned roster that every reader derives from.** The note gains a linked summary table that names who acts on each section, and two new sections. **Gate-authoring changes** is for adopters who write or shadow gates. **Platforms** is derived from the diff of docs/install.md's gated platforms table since the previous tag. Two sections are renamed so that their names cover what they hold: **New and tightened gates**, which also lists new gates, and **Knob changes**, which also lists removals and additions. Each keeps its former heading as a permanent alias.

**The section set is settled by direction.** The operator gave it on 2026-10-04, answering in the lead session, lead-relayed (not a ruling): six sections in the order below, both aliases permanent. The entry's deliverable reads *audience-keyed sections*. This amendment keys the audience in the summary table's **Who acts** column, not in the headings, and the operator chose that shape knowing it. So the headings name what changed, and the table names who acts on it.

The section names are crate literals in five modules, by `git grep -n -e "In brief" -e "Tightened gates" -e "Renamed knobs" -e "Behavior changes" -- native/src`:

- `release_bump.rs`;
- `tightened_gates_grammar.rs`;
- `release_change_declared.rs`;
- `release_declaration_parity.rs`;
- `emit/upgrade_smoke.rs`.

The first four are this repository's withheld gates (`scripts/*.gate`). The smoke is a shipped gate-sdk arm. `declaration.rs` names sections only in its tests, as gate-sdk/SPEC.md §lib/declaration.sh rules.

**Why each renamed section keeps its former heading as an alias.** Only the gate-name section is machine-read on *published* notes:

- `check-tightened-gates-grammar` walks every note with no version cutoff;
- the upgrade smoke reads TO's note out of a `git archive` of the TO tag;
- `check-release-bump` counts the newest note's sections.

A rename of an archived tag's note cannot be migrated. The smoke takes any historical pair (`GATE_SDK_UPGRADE_FROM`/`_TO`), so the alias is permanent rather than a window. The corpus is 31 release notes. All 31 carry the three declaration-bearing headings, and In brief appears in 10 (`grep -h '^## ' docs/posts/*.md | sort | uniq -c`).

**Measured at authoring:** v0.31.0's note is 743 words and v0.30.0's 1,056. The queue entry's 34,527 for v0.26.0 stands (`wc -w`).

## What changes

### (1) The roster knob and its reader {design-bearing} {user-facing: operator direction 2026-10-04, lead-relayed — the six sections in this order, the two aliases permanent}

`native/src/knobs/gate_sdk.rs` and a new `native/src/release_sections.rs`.

**`GATE_SDK_RELEASE_SECTIONS`** is an indexed knob. Each element is `<role>: <heading>`, and element order is note order. Its default, one element per line:

- `brief: In brief`
- `gates: New and tightened gates`
- `knobs: Knob changes`
- `authoring: Gate-authoring changes`
- `platforms: Platforms`
- `behavior: Behavior changes`

**`GATE_SDK_RELEASE_SECTION_ALIASES`** is an indexed knob in the same element grammar. Its default:

- `gates: Tightened gates`
- `knobs: Renamed knobs`

**The roles are kit constants.** The heading is the consumer's. Each role fixes three things: its token rule, whether it bears a declaration, and whether a note may state it as `None`.

| Role | Token rule | Declaration-bearing | `None` body |
|---|---|---|---|
| `brief` | none | no | no |
| `gates` | `GateName` | yes | yes |
| `knobs` | `Backticked` | yes | yes |
| `authoring` | `Bolded` | yes | yes |
| `platforms` | `Backticked`, the target triple | yes | yes |
| `behavior` | `Bolded` | yes | yes |

The validator refuses, at exit 2 where the knob file is read:

- an unknown role;
- a role given twice in the roster;
- an empty heading;
- an alias heading equal to any roster heading.

A consumer may omit a role, and every reader then treats that section as absent from the grammar.

`release_sections::roster()` resolves both knobs into `(role, heading, aliases)` rows. It is the one place a reader learns a section's name. `declaration.rs` stays unconfigured: its heading predicate takes a set of names in place of one. So `section_bullets`/`section_tokens` take `&[&str]`, and a container matches its first heading equal to any of them. The holder still names no section.

### (2) Every reader derives from the roster {design-bearing}

Each module drops its literal and its `consumer-value-exempt` valve, and reads its role through `roster()`:

- **`release_bump.rs`.**
  - *Presence:* on a note **under composition**, every roster role must be present under its heading. A roster role absent from a **tagged** newest note counts zero, since that note is history and is not retro-fitted, and a section found by an alias counts.
  - *Floor:* the floor sums the bullets of every declaration-bearing role.
  - *Table:* on a note under composition, the summary table must be present (delta 3).
- **`tightened_gates_grammar.rs`** reads the `gates` role, heading or alias, on every note and on the surface.
- **`release_declaration_parity.rs`** compares note and surface for the `gates`, `knobs`, `authoring` and `behavior` roles, each under its token rule, and delta 4's platforms arm.
- **`release_change_declared.rs`** accepts the surface's `behavior` or `authoring` bullets as the declaration it requires, since a changed gate skeleton or library is a gate-authoring change.
- **`emit/upgrade_smoke.rs`** reads the `gates` role, heading or alias, from TO's note and from TO's surface. The smoke resolves the knobs in the invoking tree, not in TO's archive, so an archived note is read under today's heading and alias set. The aliases exist for exactly that case.

Fixtures:

- each withheld gate's pair under `scripts/gate-tests/` moves its notes and surfaces to the new headings;
- one `good/` note per gate keeps a former heading, to exercise the alias;
- `check-release-bump`'s `bad/` gains a note under composition missing its Platforms section;
- `release_sections.rs`' unit tests cover each validator refusal.

### (3) The summary table {design-bearing} {user-facing: operator direction 2026-10-04, lead-relayed — In brief opens with a Section, Entries and Who acts table, which carries the audience keying}

**Where it sits.** `In brief` opens with a pipe table and its bullets follow it.

**What it holds.** The table has one row per declaration-bearing roster role, in roster order, under the header `| Section | Entries | Who acts |`:

- **Section** is a link `[<heading>](#<anchor>)`, the anchor being the heading's generated id under the rule `check-md-refs` resolves anchors by.
- **Entries** is the section's bullet count, `0` for a `None` body.
- **Who acts** is prose. RELEASING.md's skeleton gives each row a fixed phrase:
  - gates: *everyone: these may red on upgrade*;
  - knobs: *if you set knobs*;
  - authoring: *if you write or shadow gates*;
  - platforms: *if you install on a listed platform*;
  - behavior: *if you depend on a listed behavior*.

**Who holds it.** `check-release-bump` holds a note under composition:

- the table is present;
- its row set equals the declaration-bearing roles, in order;
- each link names its section's anchor;
- each count equals that section's bullets.

The audience cell is not read.

### (4) Platforms is derived from the platforms table {design-bearing}

`release_declaration_parity.rs`, arm P, on a note under composition.

**Which tag it diffs against.** The previous release is the newest tag below the note's version, read by `git tag --list 'v*'` and ordered as `check-release-bump` orders notes.

**What it compares.** It reads docs/install.md's `platforms` marker block at that tag (`git show <tag>:<page>`) and in the tree, with `install_platforms.rs`' row reader, which this arm makes `pub(crate)`. The page path is `GATE_LOCAL_INSTALL_PAGE`, the knob `check-install-platforms` already reads. A triple is **changed** when it was added, removed, or its Minimum or Status cell moved.

**When it reds.** The note's Platforms lead tokens, backticked triples, must equal the changed set in both directions. The finding prints the bullets the diff derives, so the composing session transcribes them. A tag whose page carries no `platforms` block reads as the empty table, which is the case for tags before the block existed.

**Not on the surface.** The surface carries no Platforms section. The section is derived at composition and never declared by a landing session.

### (5) The release declaration surface and its producers {mechanical}

- `.workflow/release-declarations.md` renames its headings to the roster's: `## Tightened gates` becomes `## New and tightened gates`. The readers accept the alias, so either order of landing is green. Its `# contract:` header is unchanged.
- **lifecycle-kit/templates/stages/build.md**, the paragraph *Declare what a vendoring consumer will meet*. Its first sentence's trigger list becomes *A unit that lands or tightens a gate, renames, removes or adds a knob, changes the gate-authoring contract (the gate library, the descriptor grammar, the skeleton or the runner's member contract), or removes a kit tool or changes what a kit script, template or default does.*

### (6) The owning sections state the roster {mechanical}

**Not yet applied.**

- **installer/SPEC.md §The upgrade contract.** Three spots change:
  - The roster paragraph (*Release notes are dated posts…*) is rewritten. The section set is `GATE_SDK_RELEASE_SECTIONS`, whose roles and default headings it names, with the aliases and why they are permanent.
  - The section bullets are re-keyed by role: *In brief* gains the table and its grammar; *New and tightened gates*; *Knob changes*, which gains the addition form `- `NEW_NAME` — new`; the new *Gate-authoring changes* and *Platforms*; and *Behavior changes*.
  - The four-residue-class paragraph re-maps the classes. Shadowed and custom gates go to New and tightened gates plus Gate-authoring changes, and own-config knobs to Knob changes. It no longer counts "three sections".
- **installer/SPEC.md §Versioning.** The Patch and Minor bullets read *every declaration-bearing section*, unchanged in wording. The `check-release-bump` paragraph names the roster in place of *tightened gates, renamed knobs or behavior changes*.
- **gate-sdk/SPEC.md §lib/declaration.sh.** Three spots change:
  - The three per-section bullets become one per token rule, keyed by role and not by heading.
  - *The holder carries no section name* stays true. It gains that names arrive as a set from `release_sections::roster()`, which `GATE_SDK_RELEASE_SECTIONS` and its alias knob own.
  - The consumer count names the four callers plus the roster reader.
- **gate-sdk/SPEC.md §upgrade-smoke.** The surface paragraph (*up to three `## ` sections named exactly as the note's…*) becomes *a `## ` section per declared role, under its roster heading or an alias*. The producer paragraph's per-section sentences name roles.
- **gate-sdk/SPEC.md §Layout and configuration** gains the two knobs.
- **lifecycle-kit/templates/upgrade.md, step 2.** Its two section names become *the New and tightened gates section* and *the Knob changes section*, and it gains *and read Gate-authoring changes if you write or shadow gates*.
- **lifecycle-kit/SPEC.md §Layout and configuration**, the knob-rename deprecation passage (`grep -n "Renamed knobs\|Behavior changes" lifecycle-kit/SPEC.md`): `Renamed knobs` becomes `Knob changes`, and `Behavior changes` keeps its name.
- **canon-kit/SPEC.md**, the valve bullet naming *Renamed knobs*: it becomes *Knob changes*.
- **RELEASING.md step 1.**
  - The skeleton's *three variable sections* bullet becomes *the declaration-bearing sections*, named by role with the knob as their owner.
  - In brief gains the table and the five fixed audience phrases.
  - Platforms is composed from `check-release-declaration-parity`'s printed derivation, not from the surface.
  - The allowed-red slot names *New and tightened gates*.
- **docs/install.md §Upgrading**, its last paragraph: it names In brief's table and the five sections.

### (7) The release declaration and the mirrors {mechanical}

- `.workflow/release-declarations.md` gains these bullets:
  - Under Knob changes, two addition bullets in delta 6's form: `GATE_SDK_RELEASE_SECTIONS` and `GATE_SDK_RELEASE_SECTION_ALIASES`.
  - Under Behavior changes, a bullet led **release notes**: the six section names, the summary table and the two permanent aliases. Nothing to do; a consumer reading notes by heading reads either spelling.
- `docs/installer/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/lifecycle-kit/SPEC.md` and `docs/canon-kit/SPEC.md` are regenerated with the command their freshness gate prints, in the commit landing delta 6.

## Producers and consumers

- **The two knobs.**
  - Producer: the gate-sdk table defaults; this repository sets no line.
  - Consumer: `release_sections::roster()`, called by the four withheld gates and the smoke.
  - Roster-holding readers of the new names: the table, §Layout and configuration (held by `check-knob-citation` and `check-knob-default-coupling`), `--emit knob-roster`, and each reading member's declared-knob slice, which `check-reads-couples` holds.
- **The roles.** Each role is read by named transitions:
  - `brief`: presence and the table, by `check-release-bump`.
  - `gates`: the grammar gate, parity, the smoke and the bump count.
  - `knobs` and `behavior`: parity and the bump count.
  - `authoring`: parity, the bump count and `check-release-change-declared`.
  - `platforms`: parity's arm P and the bump count.
- **Red conditions (point 5):**
  - The roster widens the required set on a note under composition. `check-release-bump` reds a note missing a roster section, but only one under composition, so the 31 tagged notes stay green. That holds both for its presence check and for its count over a tagged newest note, which counts an absent role as zero.
  - The grammar gate reads more headings, through the alias, and reds on no new note.
  - `check-tightened-gates-grammar`'s whole-corpus walk finds the gates section in each historical note under the alias. Without the alias it would read `Absent` on all 31.
  - `check-release-declaration-parity` gains arm P. It reds a note under composition whose Platforms tokens differ from the derived diff.
- **Every member's satisfying value (point 6):** each of the 31 published notes satisfies the grammar gate through the `gates` alias (`grep -c '^## Tightened gates' docs/posts/*checkwright-v*.md`, 31 files with one each). No new presence rule reaches a tagged note.

## Existing sections updated

Roster by `git grep -n -e "Tightened gates" -e "Renamed knobs" -e "Behavior changes" -e "In brief" -- ':!docs/posts' ':!docs/*/SPEC.md' ':!scripts/gate-tests' ':!TASK-QUEUE.md'`, over the tracked tree.

- `native/src/knobs/gate_sdk.rs` — the two knobs and their validator (delta 1).
- `native/src/release_sections.rs` — the roster reader (delta 1).
- `native/src/declaration.rs` — the heading-set predicate (delta 1).
- `native/src/gates/release_bump.rs` — roster presence, the count and the table (deltas 2 and 3).
- `native/src/gates/tightened_gates_grammar.rs` — the gates role (delta 2).
- `native/src/gates/release_declaration_parity.rs` — the roles and arm P (deltas 2 and 4).
- `native/src/gates/release_change_declared.rs` — behavior or authoring (delta 2).
- `native/src/gates/install_platforms.rs` — the row reader made `pub(crate)` (delta 4).
- `native/src/emit/upgrade_smoke.rs` — the gates role (delta 2).
- `scripts/gate-tests/` — the four withheld gates' fixture pairs (deltas 2, 3 and 4).
- `.workflow/release-declarations.md` — the renamed headings and the declarations (deltas 5 and 7).
- `lifecycle-kit/templates/stages/build.md` — the declare paragraph (delta 5).
- `lifecycle-kit/templates/upgrade.md` — step 2 (delta 6).
- `installer/SPEC.md` — §The upgrade contract and §Versioning (delta 6).
- `gate-sdk/SPEC.md` — §lib/declaration.sh, §upgrade-smoke and §Layout and configuration (delta 6).
- `lifecycle-kit/SPEC.md` — the two section names (delta 6).
- `canon-kit/SPEC.md` — the valve bullet (delta 6).
- `RELEASING.md` — step 1's skeleton (delta 6).
- `docs/install.md` — §Upgrading (delta 6).
- `docs/installer/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/lifecycle-kit/SPEC.md`, `docs/canon-kit/SPEC.md` — regenerated mirrors (delta 7).

## Retired spellings

<!-- retired-spelling-exempt: the heading is kept for good as an alias, so its survivors are deliberate: 31 published notes, the gate fixtures that exercise the alias, and the generated docs mirrors -->
- `Tightened gates` — renamed New and tightened gates, kept as an alias (delta 1). Every live site that names the section is a roster bullet above.
<!-- retired-spelling-exempt: the heading is kept for good as an alias, so its survivors are deliberate: 31 published notes, the gate fixtures that exercise the alias, and the generated docs mirrors -->
- `Renamed knobs` — renamed Knob changes, kept as an alias (delta 1). Every live site that names the section is a roster bullet above.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — build.md's and upgrade.md's replacement text carries no grounds.
- [ ] **Merged with no information lost** — §The upgrade contract states the roster whole; no surface names the section set as a fixed three.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declarations above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery, the withheld-gate fixture suite, and `--upgrade-smoke` on the landing commit.
- [ ] **The entry is done** — `release-note-section-set-derivation` moves to Done in the commit landing delta 6, before the drain stage.
