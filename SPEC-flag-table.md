# SPEC amendment: flag-table

**Merged at build; held on disk until the Done move.** Every delta below is applied and merged into its canonical section. The Definition of Done moves the entry to Done only once the mid-iteration push's runs are green, and `check-amendment-queue` holds an active entry's `[spec:]` ref to this file, so the file is deleted in the Done-move commit.

`check-front-door-verbs` holds a verb the front door advertises to the pinned release, and reads nothing after the verb (installer/SPEC.md §The front door's verbs). So a flag landed mid-iteration and advertised after a route reds no gate and fires no release trigger. The site then advertises a line the one-liner refuses until an unforced release ships the flag. The recorded instance was `init --recipe` on the toolkit pages while the pinned v0.28.0 refused it (`unknown argument: --recipe`, exit 2). Two further holes let that instance through: the toolkit pages are not front-door pages, and a fenced line opening with `checkwright` is not a route.

This amendment gives the binary a flag roster beside `VERBS`, puts a flag table beside the verb table, extends invariant B to the flags after an advertised verb, adds the toolkit pages and the fenced `checkwright` route, and words the close's release trigger to match.

One queue entry pairs it: [front-door-flag-grammar](TASK-QUEUE.md#front-door-flag-grammar).

**The rulings.**

- **The roster's owner is the binary, as for verbs.** A flag is advertisable because a verb's parser accepts it. The roster is a const beside `VERBS`, and a crate test holds each parser and each `usage:` line to it, so the table, the parsers and the help cannot disagree.
- **A pinned tag carrying no flag table has the empty pinned flag set.** It is not a dormancy. The verb table's own reasoning rules it: a dormancy keyed to the absent table would green the very defect the gate exists for (installer/SPEC.md §The front door's verbs). Its consequence for this close is below.
- **The extension's install command is not a front-door page.** `companion/speckit/commands/install.md` runs the release named by the extension's stamped version, and the pack step stamps the tag the extension is packed from (companion/SPEC.md §Packing the extension). So its line always names a release that carries its own flags, and the pinned release is the wrong comparison for it.
- **An operand is out of reach.** The gate reads a flag, never its value. A recipe or profile name the pinned release lacks, after a released flag, is filed as a gap.
- **The seam.** Repo-root only: a repo-local gate, the installer's README and SPEC, and the close binding. No kit surface changes.

**The first release of the table: an operator direction, lead-relayed (not a /consult ruling).** The pinned v0.29.0 carries no flag table, and the front door advertises `--profile` after a route today (`docs/install.md` lines 117 and 176, `plugin/skills/install/SKILL.md` lines 11 and 12, and `installer/README.md` line 21) and `--recipe` on the toolkit pages. Under the empty pinned flag set, every such flag is pending from the landing commit, so this iteration's close must release, and a `none` or `deferred:` disposition reds B. A dormant flag arm on a tag without a table was the alternative, refused: it greens every flag until a release happens for another reason, `install-gate-selection`'s new `init` flags included. The release carries the deferred v0.30.0's floor (installer/SPEC.md §Versioning's second input).

## What changes

### (1) The binary's flag roster {design-bearing}

**Applied.** `native/src/installer/mod.rs` gains `FLAGS`, beside `VERBS`. It holds one row per flag, each row the flag's `--`-spelling and the verbs whose parsers accept it. `-h` and `--help` take no row: they are the help arm every verb carries (§The verbs). The rows at this amendment, read off the parsers in `init.rs` and `uninstall.rs` and off `update.rs`'s forwarding to `init`:

| flag | verbs |
| --- | --- |
| `--profile` | `init`, `update` |
| `--recipe` | `init`, `update` |
| `--no-recipe` | `init`, `update` |
| `--dry-run` | `init`, `update`, `uninstall` |
| `--force` | `init`, `update`, `uninstall` |
| `--no-commit` | `init`, `update`, `uninstall` |

`doctor`, `diff` and `demo` take no flag. If [install-gate-selection](TASK-QUEUE.md#install-gate-selection) lands in the same batch, its five selection flags are rows too, each naming `init` and `update` (SPEC-gate-selection.md, delta 5). If it lands in a later batch, that delta adds them.

A crate test holds three things. First, each row's flag parses for every verb its row names, with no `unknown argument` refusal. `--profile=<name>` and `--recipe=<name>` are the same row as the spaced form. Second, a flag no row names for a verb is refused at exit 2 by that verb. Third, the `--`-tokens of each verb's `usage:` line equal its rows. So `update`'s usage line gains `[--recipe <name>]... [--no-recipe]`, which its forwarding already accepts.

installer/SPEC.md §The verbs' first paragraph gains, after *rather than by a roster beside it*: *The same holds for each verb's flags: `FLAGS` beside `VERBS` names each flag and the verbs that accept it, and a crate test holds every parser and `usage:` line to it.*

### (2) The flag table {mechanical}

**Applied.** `installer/README.md` gains a table after the verb table's closing paragraphs, and before `## Choosing a profile`, headed `| flag | verbs | means |`. Its rows are delta 1's, each `means` cell one clause, such as *vendor this profile* for `--profile`, and *print the plan, write nothing* for `--dry-run`. The first column is code-spanned and the second lists code-spanned verbs, comma-separated.

### (3) Invariant B reads the flags after an advertised verb {design-bearing}

**Applied.** installer/SPEC.md §The front door's verbs is rewritten:

- The opener becomes: *Every route on the front door resolves to the newest release, so a verb the front door advertises, and each flag after it, must be one that release carries.*
- **Invariant A** becomes *the tables are the binary's rosters*: the verb table's first-column code spans equal `VERBS` as a set, as today. Then: *The flag table, the table whose header row's first cell is `flag`, yields one pair per verb its second column lists beside the first column's flag, and those pairs equal `FLAGS`'s as a set.*
- **Invariant B**:
  - The route list gains *a fenced code line whose text opens with `checkwright`*. That is the toolkit pages' form, where `checkwright` stands for the reader's install line (companion/SPEC.md §Applying a recipe).
  - After the verb rule, B gains: *After an advertised verb, each following token of the same code text is read until a token that is `|`, `;`, `&&`, `||` or `)`, or that opens with `#` or `>`. A read token opening with `-`, other than `-h` or `--help`, is an **advertised flag** of that verb, cut at its first `=`. A placeholder verb advertises no flag. Each advertised flag must be in the **pinned flag set**: invariant A's flag table read at the tag `v<pin>`, as pairs with its verb. A tag whose `installer/README.md` carries no flag table has the empty pinned flag set.*
  - The front-door pages gain `docs/speckit.md` and `docs/openspec.md`. The section says why `companion/speckit/commands/install.md` is not one, on the ruling above.
- **The pending admission** is restated over both rosters: *A verb or flag B would red is admitted while the binary carries it, the verb in `VERBS` or the pair in `FLAGS`, …*. The rest of the paragraph stands, and its two remedies read *releasing or withdrawing the advertisement*.
- The fail-closed list gains *a flag table that is absent or empty at HEAD*. A tag's absent flag table is the empty set, not a refusal.
- The positional form is unchanged: `pinned-readme` now carries both tables.
- **Honest limits** gains: *A flag named apart from its route, as in "pass `--recipe`", is out of reach, and so is a flag's value: a recipe or profile name the pinned release lacks, after a flag it carries, reds nothing.*

`native/src/gates/front_door_verbs.rs` implements it. `advertised` reads the fenced `checkwright` route as well as the span one. A reader of the flag table sits beside `verb_table`, on the same header rule. The flag walk follows each verb token. The findings name the page, the line, the verb and the flag, and the `help:` line for a flag finding reads *release the flag so the pin carries it (RELEASING.md), or withdraw it from the page*. The unit test asserting `verbs("checkwright uninstall", false)` is empty is inverted: a fenced line opening with `checkwright` is now a route. The clean line gains the flag-site count and any pending flags.

`scripts/check-front-door-verbs.gate`: `couples=` gains `docs/speckit.md,docs/openspec.md`, and its `# spec:` line names the flags.

### (4) The release trigger names the flag {mechanical}

**Applied.** `.claude/commands/close.md`'s release-policy bullet becomes: *A front-door verb or flag the pinned release lacks. `check-front-door-verbs` reds a `none` or `deferred:` disposition while the front door advertises one, so release, or withdraw the advertisement in the same close.*

### (5) The fixture pair {mechanical}

**Applied.** `scripts/gate-tests/check-front-door-verbs/`:

- `good/readme.md` and `good/pinned.md` gain a flag table, and `good/released.md` advertises a released flag after a verb in a span and in a fenced `checkwright` line. `good/pending.md` advertises a flag `FLAGS` carries that the pinned table lacks, under the good disposition, and a flag after a placeholder, which reads nothing.
- `bad/` gains three findings: a flag the pinned table lacks while the disposition withholds, a flag neither the pinned table nor `FLAGS` carries, and a `readme.md` flag table missing one of `FLAGS`'s pairs. Each `expect.txt` is updated.
- A pinned readme with a verb table and no flag table, under a withholding disposition, reds each advertised flag. That is the empty-set ruling, and it rides `bad/`.

## Producers and consumers

Probe: `native/src/installer/{init,uninstall,update,doctor,diff,demo}.rs` read for each parser's match arms and `usage:` line; `native/src/gates/front_door_verbs.rs` read whole; a scratch awk over `README.md`, `docs/index.md`, `docs/install.md`, `installer/README.md`, `plugin/skills/install/SKILL.md`, `docs/speckit.md`, `docs/openspec.md` and `docs/spec-toolkits.md` for fenced lines opening `checkwright` and for route-advertised tokens. The fenced-`checkwright` lines are on `docs/speckit.md` line 26 and `docs/openspec.md` lines 14 and 28 alone. The route-advertised flags are `--profile` on `docs/install.md` 117 and 176, `installer/README.md` 21 and `plugin/skills/install/SKILL.md` 11 and 12, and `--recipe` on the two toolkit pages.

- **`FLAGS`** (delta 1). Producer: the crate source. Consumers: each verb's parser and `usage:` line, through the crate test; invariant A's flag half, in process; invariant B's pending admission.
- **The flag table** (delta 2). Producer: `installer/README.md`, carried by every tag. Consumers: invariant A at HEAD and B's pinned flag set at `v<pin>`. Red condition: A reds a pair the table and `FLAGS` do not share; an absent or empty table at HEAD exits 2.
- **Advertised flags** (delta 3). Producer: the front-door and toolkit pages. Consumer: B. It reds a flag outside the pinned set that `FLAGS` lacks, or that `FLAGS` carries while the disposition withholds. The clean line lists the pending ones.
- **The widened page set** (delta 3). It reds no page today beyond the pending flags the empty pinned flag set makes: the two toolkit pages carry `init` and `--profile`, `--recipe` after it, and `init` is in the pinned verb set.
- **The trigger** (delta 4). Reader: the close session's release-policy step, which B's red enforces at the disposition commit.

## Existing sections updated

Roster probe: `git grep -n "front-door-verbs\|front_door_verbs\|verb table"` over the tracked tree, excluding `docs/posts/` and the generated mirrors.

- `native/src/installer/mod.rs`, `native/src/installer/update.rs` (delta 1).
- `installer/SPEC.md` — §The verbs (delta 1), §The front door's verbs (delta 3).
- `installer/README.md` (delta 2).
- `native/src/gates/front_door_verbs.rs`, `scripts/check-front-door-verbs.gate` (delta 3).
- `docs/site-architecture.md` — §Generated projections and their freshness gates, the hosted-install-scripts bullet: *the tag whose verb table the front door is held to* becomes *the tag whose verb and flag tables the front door is held to* (delta 3).
- `.claude/commands/close.md` — the release-policy trigger (delta 4).
- `scripts/gate-tests/check-front-door-verbs/` (delta 5).
- `docs/installer/SPEC.md` and `docs/installer/README.md`, the generated mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `.workflow/surface-ceiling.txt` — the grown `installer/SPEC.md` and `installer/README.md` rows re-stamped with `--emit always-loaded --ceiling` in the growing commit (deltas 1, 2 and 3).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **`update`**: its `usage:` line names the recipe flags it already forwarded (delta 1).

## Retired spellings

- None — the deltas add a const, a table, a route form and two pages, and re-phrase prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery, the root fixture suite and `cargo test` green on the landing commit, and the gates workflow green on the mid-iteration push. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
