# SPEC amendment: layout-neutral

**Four kit-shipped gate-sdk members and one emitter arm read this repository's layout where the consumer's own value is already on hand, and each is made to read the consumer's, with no new knob.** `check-gate-substrate-parity` defaults its conservation doc to `<GATE_SDK_ROOT>/SPEC.md`, which the payload withholds (`GATE_SDK_PAYLOAD_WITHHOLD` defaults to `SPEC.md smoke`), so a bare run in an installed tree exits 2. `check-template-copy-parity` globs `<root>/*/templates/*.sh` (`template_copy_parity.rs`), so under a subdirectory vendoring its corpus empties and it reads clean. `check-tree-terms` and `check-portability-floor` exempt their rosters by the shipped basename prefix (the two `SELF_EXEMPT_PREFIX` constants), so a consumer whose pattern-file knob names another file reds on its own roster. `--emit enforcement-map` links each kit to `<kit>/index.md` (`kit_cell`), so an adopter's page ships one dead link per kit. Each premise was read in source at authoring; the tree satisfies none of the fixes.

**Landing order.** Delta 4 and the anchor re-pointing of delta 8 land before withheld-literals' delta 3, which edits the same emitter module and registry rows; SPEC-withheld-literals.md states the order.

**Measured at authoring.** `git ls-files` over names beginning `msg-patterns` or `portability-patterns` returns `scripts/msg-patterns.list`, `scripts/portability-patterns.list`, the two `gate-sdk/templates/` starter rosters and fixture files under `gate-tests/`, so delta 3 moves no verdict in this tree. `git ls-files docs` tracks `docs/<kit>/index.md` for every kit the enforcement page rows, so delta 4 leaves `docs/enforcement.md` unchanged.

## What changes

### (1) The conservation doc is owed by the publishing tree {design-bearing} {user-facing: gate-sdk-layout-assumptions's deliverable, the parity gate made layout-neutral with a release declaration for the verdict change}

`native/src/gates/gate_substrate_parity.rs`. `rule()` reads the doc (positional 2, default `<sdk root>/SPEC.md`) before it computes the publishing predicate, `walk::authoring_tree` over `GATE_SDK_NATIVE_CRATE`, already computed for assertions B and F. The order flips, and the absent-doc refusal depends on the predicate:

- **A doc given as a positional and missing:** exit 2 in every tree, since an explicit steer at a corpus that is not there cannot answer.
- **No positional, default doc missing, publishing tree:** exit 2, as today, so a deleted SPEC cannot turn the anti-vacuity assertion C green.
- **No positional, default doc missing, any other tree:** assertion C and assertion B's `reference-only` allowance are not asserted, the allowance is empty, and the clean line's tail reads `{sensitive} substrate-sensitive member(s) not dispositioned here, the conservation doc being absent in a tree that publishes no crate` where it read `… all dispositioned`. Every other assertion reads no doc and runs unchanged.
- **Doc present, section absent:** exit 2 in every tree, as today.

A payload-carried copy of the section was refused as a second home for the dispositions, publishing a SPEC body the payload withholds; a new knob was refused because no adopter file holds dispositions for kit-shipped members and the positional already steers; skipping whenever the doc is absent was refused because it loses fail-closed in the one tree that owes the doc.

### (2) `check-template-copy-parity` pairs the templates of each kit root {design-bearing} {user-facing: gate-sdk-layout-assumptions's deliverable, the template-copy gate made layout-neutral with a release declaration for the verdict change}

`native/src/gates/template_copy_parity.rs`: the corpus is `<scan root>/<kit root>/templates/*.sh` for each kit root `walk::kit_roots_under(<scan root>)` returns (the roots `GATE_SDK_KIT_DIRS` names, else the derivation, spelled against the scan root and narrowed to those under it), replacing the `*/templates/*.sh` glob. `walk::kit_roots` is refused here: it spells a root against the working directory, while the scan root is the gate's first argument. A kit root equal to the scan root is dropped, as the old glob never paired it. The pairing, the `*-config.sh` exclusion, the unpaired skip and the three assertions are unchanged. The member's registry row in `native/src/gates/mod.rs` declares `GATE_SDK_KIT_DIRS` beside `GATE_SDK_GATES_DIR`, and its walk-root declaration takes the `?` line `check-template-registry-parity`'s row carries. The clean line gains `; M kit root(s) read`, a report with no floor, so an emptied corpus is visible. The descriptor's `couples=` changes `*/templates/*.sh` to `kit:templates/*.sh`, so the trigger reaches a vendored root.

The verdict moves both ways: under a subdirectory vendoring, pairs the glob never reached are compared, and a top-level directory carrying `templates/*.sh` that is no kit root is no longer paired. An installed tree is unaffected, since the installer writes the kit set into the knob.

### (3) Self-exemption names the resolved roster, not the shipped basename {design-bearing} {user-facing: gate-sdk-layout-assumptions's deliverable, both pattern gates made layout-neutral with a release declaration for the verdict change}

`native/src/gates/tree_terms.rs` and `native/src/gates/portability_floor.rs`: the two `SELF_EXEMPT_PREFIX` constants and their basename-prefix tests retire. A tracked path is self-exempt when it is one of:

- **a pattern file the gate resolved**: for `check-tree-terms` the files `GATE_SDK_MSG_PATTERN_FILES` and `GATE_SDK_MSG_PATTERN_FILES_LOCAL` name, or the positional list; for `check-portability-floor` those `GATE_SDK_PORTABILITY_PATTERNS` names, or the positional list. The comparison is lexical against the path `git ls-files` prints, a leading `./` stripped and an absolute value spelled against the working directory.
- **a kit's own starter roster**: `<kit root>/templates/<name>`, each kit root from `walk::kit_roots` (spelled against the working directory, as `git ls-files` prints the tracked paths), `<name>` the basename of that gate's pattern-knob default: `msg-patterns.list` for `check-tree-terms` (the `GATE_SDK_MSG_PATTERN_FILES` default; the local list ships no starter) and `portability-patterns.list` for `check-portability-floor`. Those defaults are derived rows (`in_gates_dir` in `native/src/knobs/gate_sdk.rs`) with no readable default value, so each basename is hoisted to a `pub const` beside its derivation, which both the derivation and the gate read, holding the literal once inside the knob tables. The shipped roster spells the shapes it bans and is vendored, tracked, in every consumer, so scanning it would red every adopter.

Both members' registry rows declare `GATE_SDK_KIT_DIRS`. The verdict moves both ways: a roster the knob names under another basename stops reddening on itself, and a tracked file merely beginning with the prefix that no knob names (a stale `msg-patterns.old.list`) is now scanned and reds on a banned shape in it.

### (4) The enforcement map links a kit page only where the tree tracks one {design-bearing} {user-facing: gate-sdk-layout-assumptions's deliverable, the emitted page made layout-neutral with a release declaration for the verdict change}

`native/src/emit/enforcement_map.rs`, `kit_cell`: a kit cell is `[<kit>](<kit>/index.md)` when `<dir of GATE_SDK_ENFORCEMENT_FILE>/<kit>/index.md` is a tracked file (the `tracked()` test `owner_ref` applies), and plain `<kit>` text otherwise, as the `(consumer)` group already is. The link is relative to the page, so the test is made at the same spot and a dead link is unrepresentable; tracked rather than present, so a local untracked page cannot make a local run fresh where CI reds. `kit_cell` is called from `render()`, so `emit()` reads the knob, computes the set of kits whose page is tracked and hands it to `render()`, which passes each row's membership to `kit_cell`; `measure()` is untouched, so the value rollup's structured read is unchanged. The `--emit-enforcement-map` row in `native/src/emit/mod.rs` declares `GATE_SDK_ENFORCEMENT_FILE`.

**Honest limits:** the link base is the knob's directory, so a page redirected to another path, or compared at another path through `check-enforcement-fresh`'s positional, is linked as if it sat at the knob's. And the page now depends on a tracked file's presence that `check-enforcement-fresh`'s `couples=` cannot name, since a `knob:` token roots at a directory knob and this is a file knob's directory. Adding a kit page alone does not fire the gate at commit; the full battery and CI do.

### (5) Fixtures and tests, one per change {mechanical}

- **Parity** (`gate-sdk/gate-tests/check-gate-substrate-parity.test.sh`): three sandbox cases — a non-publishing tree, no positional, default doc absent: exit 0 and the not-dispositioned tail; the same tree with the crate source tracked: exit 2 naming the doc; a missing positional doc in a non-publishing tree: exit 2. The `good/`+`bad/` pair is unchanged.
- **Template copy** (`gate-sdk/gate-tests/check-template-copy-parity/`): a pair under `vendor/delegation-kit/templates/` with `GATE_SDK_KIT_DIRS = vendor/delegation-kit` in the case's knob file, agreeing in `good/` and divergent in `bad/` (a red the old glob read clean), and a top-level `templates/*.sh` directory with no kit marker, skipped. `check-template-copy-parity.test.sh` gives its sandbox `widget-kit` a kit marker so the default-root case still pairs.
- **Tree terms** (`gate-sdk/gate-tests/check-tree-terms/`): `good/` names its roster by a non-prefixed path and carries a kit starter roster under a declared kit root, each spelling a banned shape; the prefix-named fixture files leave `good/`. `bad/` gains a prefix-named file no knob names, carrying a banned shape.
- **Portability floor** (`gate-sdk/gate-tests/check-portability-floor/`): the same shape.
- **Enforcement map**: `check-enforcement-fresh/good` tracks the kit page its committed page links; a unit test in `enforcement_map.rs` pins both arms of `kit_cell`; `gate-sdk/gate-tests/enforcement-map.test.sh` asserts a run with `GATE_SDK_ENFORCEMENT_FILE` in an untracked scratch directory writes the kit names as plain text.

A case's `scripts/gate-sdk-config.knobs` row steers `GATE_SDK_KIT_DIRS` for the kit-root resolver these reads call: `--emit kit-roots` run with `check-enforcement-fresh/good` as the working directory prints that case's `demo-kit`.

### (6) The SPEC text states each rule {mechanical}

gate-sdk/SPEC.md. **Not yet applied.**

§check-gate-substrate-parity, after the assertion C bullet:

> **The conservation doc is owed by the tree that authors the dispositions.** Assertion C and assertion B's `reference-only` allowance read §Meta-gate conservation for the binary substrate, which the payload withholds (§Consumer payload). A doc given as a positional is a steer, and its absence is exit 2. With no positional the doc is `<gate-sdk root>/SPEC.md`: absent in a publishing tree (assertion F's predicate) it is exit 2, so a deleted doc cannot turn C green; absent in any other tree, C and the allowance are not asserted and the clean line counts the members left undispositioned. A doc present with no conservation section is exit 2 in every tree.

§check-template-copy-parity, the first sentence of *Scope derives from layout, never a roster* becomes:

> The pairing is `<kit root>/templates/<name>.sh` ↔ `<gates-dir>/<name>.sh` for each kit root resolved under the scan root (the optional first argument, default the git toplevel), so a subdirectory vendoring is paired where it sits.

and in the same section the clean line is said to count the kit roots read, and *the glob is exactly two levels deep and cannot reach a `checks/` segment* becomes *the glob is one `templates/*.sh` per kit root and cannot reach a `checks/` segment*.

§check-tree-terms, *Any file whose basename begins `msg-patterns` is self-exempt, a *prefix* match: a pattern list contains what it bans, and a private list is kept out of the tree by gitignore, not by this gate.* becomes:

> A file is self-exempt when it is a pattern file the gate resolved (the two knobs' files, or the positional list) or a kit's own starter roster under `<kit root>/templates/`: a pattern list contains what it bans, and a private list is kept out of the tree by gitignore, not by this gate.

§check-portability-floor, *Any file whose basename begins `portability-patterns` is self-exempt, on §check-tree-terms' shape* becomes *A pattern file this gate resolved, or the kit's own starter roster, is self-exempt, on §check-tree-terms' shape*.

§enforcement-map, *The **kit column links each kit's docs page** (`<kit>/index.md`, relative to the page under the docs root); the `(consumer)` group owns no kit page and stays plain text.* becomes:

> The **kit column links a kit's docs page** (`<kit>/index.md`, relative to the page) where the page's directory tracks it, so a tree with no docs layout carries no dead link; a kit with no page, and the `(consumer)` group, stay plain text.

The starter rosters' header comments, `gate-sdk/templates/msg-patterns.list` and `gate-sdk/templates/portability-patterns.list`, and this repo's copies `scripts/msg-patterns.list` and `scripts/portability-patterns.list`, replace the prefix sentence with *The pattern files this gate resolves are self-exempt.* The prefix comment over `SELF_EXEMPT_PREFIX` in `tree_terms.rs` leaves with the constant (delta 3); the module's header comment already states the exemption without the prefix and stands. A comment-only template edit is outside `check-release-change-declared`'s class T.

### (7) The release declarations {mechanical}

`.workflow/release-declarations.md`, in installer/SPEC.md §The upgrade contract's grammar. **Not yet applied.**

Under Tightened gates:

> - `check-template-copy-parity` — pairs each kit root's `templates/*.sh` (the roots `GATE_SDK_KIT_DIRS` names or the derivation finds) rather than `<root>/*/templates/*.sh`, so a subdirectory vendoring's copies are now compared and a divergent one reds; a top-level directory with templates and no kit marker is no longer paired. Re-copy the template, or declare `# copy-divergence: <reason>` per finding.
> - `check-tree-terms` — a file is self-exempt when it is a pattern file the two pattern knobs resolve to or the kit's starter roster, rather than any name beginning `msg-patterns`, so a tracked `msg-patterns` file no knob names is scanned and reds on a banned shape in it. Point the knob at each roster you keep, or remove the stray file.
> - `check-portability-floor` — the same change over `GATE_SDK_PORTABILITY_PATTERNS` and the `portability-patterns` prefix. Nothing to do unless a stray file of that name sits in your corpus.
> - `check-enforcement-fresh` — reds a committed enforcement page linking `<kit>/index.md` for a kit whose page your tree does not track, since the emitter now writes that kit as plain text. Regenerate with `--emit enforcement-map`.

Under Behavior changes:

> - **gate-sdk `check-gate-substrate-parity`, in a tree that carries no crate source** — a missing default `<gate-sdk root>/SPEC.md` no longer exits 2: disposition coverage and the `reference-only` allowance are not asserted, the clean line counts the members left undispositioned, and every other assertion now runs where the gate refused. A doc given as a positional still exits 2 when missing. Nothing to do.
> - **`--emit enforcement-map`** — the kit column writes a kit's name as plain text where the page's directory tracks no `<kit>/index.md`. Regenerate the page; nothing else to reconcile.

### (8) Generated projections follow {mechanical}

The `couples=` change regenerates the pre-commit hook and the graph artifact (docs/site-architecture.md §Generated projections and their freshness gates prints each command); `docs/gate-sdk/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 6. The edits shift lines the registry's `dynamic@src/…:<line>` walk-root anchors pin for these members, so build re-points each anchor `check-reads-couples` names.

## Producers and consumers

- **No new state, knob, event or interface.** Every value read has an owner: `GATE_SDK_KIT_DIRS`, `GATE_SDK_ENFORCEMENT_FILE`, `GATE_SDK_MSG_PATTERN_FILES`, `GATE_SDK_MSG_PATTERN_FILES_LOCAL`, `GATE_SDK_PORTABILITY_PATTERNS`, `GATE_SDK_NATIVE_CRATE`. No knob table, SPEC knob roster or `scripts/*.knobs` row changes.
- **Declared-knob rosters the reads land on**, by `git grep -n '"check-template-copy-parity"\|"check-tree-terms"\|"check-portability-floor"\|"--emit-enforcement-map"' -- native/src`: the three members' rows in `native/src/gates/mod.rs` and the arm row in `native/src/emit/mod.rs` (deltas 2, 3 and 4). Their readers, `check-reads-couples`, the registry unit tests and `check-graph`'s `couples=` expansion, are held by the same edits.
- **Red conditions where a corpus narrows or widens (point 5):**
  - *Parity, narrowed in a non-publishing tree:* readers are `check-gate-substrate-parity/good/expect.txt` (its doc-read tail unchanged) and delta 5's sandbox cases. A kit member lacking a disposition no longer reds in an adopter tree, by design; it still reds in the publishing tree.
  - *Template copy, widened under a nested vendoring and narrowed for an unmarked top-level directory:* reds where a vendored template and its copy diverge; readers are its pair, its `.test.sh`, and the consumer smoke's battery, which vendors kits whole and so is unmoved.
  - *Tree terms and portability floor:* reds where an unnamed prefix file carries a banned shape; readers are the two pairs and this repo's battery, unmoved (the probe above).
  - *Enforcement map:* `check-enforcement-fresh` reds on a committed page linking an untracked kit page (none here); `check-value-rollup-fresh` reads structured values and is unmoved.
- **Verdict consumers:** the committing session through the output contract, and an upgrading adopter through delta 7's declarations.
- **Reshaped elsewhere:** the iceboxed `template-copy-parity-yaml-widening` widens the same pairing, now per kit root; it is not folded in.

## Existing sections updated

Roster by `git grep -n -F '*/templates/*.sh'`, `git grep -n 'SELF_EXEMPT_PREFIX'`, `git grep -n -F 'msg-patterns*'`, `git grep -n -F 'portability-patterns*'`, `git grep -n -F 'copy pair(s)'` and `git grep -n 'all dispositioned'` over the tracked tree.

- `gate-sdk/SPEC.md` — §check-gate-substrate-parity, §check-template-copy-parity, §check-tree-terms, §check-portability-floor and §enforcement-map (delta 6).
- `native/src/gates/gate_substrate_parity.rs` — the doc read's order and the clean line (delta 1).
- `native/src/gates/template_copy_parity.rs` — the per-kit-root corpus and clean line (delta 2).
- `native/src/gates/tree_terms.rs` — the self-exemption and its comment (delta 3).
- `native/src/gates/portability_floor.rs` — the self-exemption (delta 3).
- `native/src/knobs/gate_sdk.rs` — the two pattern-roster basenames hoisted to constants (delta 3).
- `native/src/emit/enforcement_map.rs` — `kit_cell` and its unit test (deltas 4 and 5).
- `native/src/gates/mod.rs` — the three members' rows (deltas 2 and 3), and the `dynamic@` anchors `check-reads-couples` names once the edits shift their lines, among them `check-gate-substrate-parity`'s, `check-enforcement-fresh`'s and `check-value-rollup-fresh`'s (delta 8).
- `native/src/emit/mod.rs` — the enforcement-map arm row (delta 4).
- `gate-sdk/checks/check-template-copy-parity.gate` — its `couples=` (delta 2).
- `gate-sdk/gate-tests/` — the fixture pairs and `.test.sh` suites delta 5 names (delta 5).
- `gate-sdk/templates/msg-patterns.list` — its header comment (delta 6).
- `gate-sdk/templates/portability-patterns.list` — its header comment (delta 6).
- `scripts/msg-patterns.list` — its header comment (delta 6).
- `scripts/portability-patterns.list` — its header comment (delta 6).
- `.workflow/release-declarations.md` — six bullets (delta 7).
- `scripts/git-hooks/pre-commit` — regenerated (delta 8).
- `docs/check-graph.html` — regenerated (delta 8).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 8).

## Retired spellings

- `*/templates/*.sh` — the template-copy glob, in its source and descriptor (delta 2).
- `SELF_EXEMPT_PREFIX` — the two basename-prefix constants (delta 3).
- `msg-patterns*` — the prefix-glob exemption in the starter roster's comment, this repo's copy and the constant's comment (deltas 3 and 6).
- `portability-patterns*` — the same, for the portability roster (deltas 3 and 6).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the roster comments state the rule, not its grounds.
- [ ] **Merged with no information lost** — each gate's section reads as one section stating its layout-neutral read.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration while sibling gate-sdk amendments are in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declarations above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Release declarations before the tag** — delta 7's bullets land in the commit landing each behavior.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery and gate-sdk's fixture suite in the merging batch.
- [ ] **The entry is done** — `gate-sdk-layout-assumptions` moves to Done in the commit landing its last delta, before the drain stage.
