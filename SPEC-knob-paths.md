# SPEC amendment: knob-paths

Three filed gaps share one cause. A consumer path that a kit gate reads or writes is declared through a knob in `couples=`, but three other declaration sites still spell it as a literal, or spell it through a token that does not cover it.

- **The `# projection:` line** refuses a `knob:` token (gate-sdk/SPEC.md §The install disposition), so the four kit freshness members spell this repo's output paths literally: `docs/footprint.md`, `docs/enforcement.md`, `docs/check-graph.html` with the two hook paths, and `ROADMAP.md`. Each path already has a knob, and each member's `couples=` already names that knob.
- **The canonical-spec prune** bakes the mirror root `docs` into kit mechanism (canon-kit/SPEC.md §The shared spec adapters, the mirror-prune bullet), and the registry's prune field has no `knob:` source to relocate it with.
- **Two `couples=` literals** took none of doc-path-couples' dispositions.

**The ruling: one knob-path token for a declaration that names a place, resolved verbatim; the `.` root repaired in the rooted couples token it borrows from; and a mirror-root knob that moves the generator, the comparator and the prune together.**

A `couples=` token names a *pattern* a walker selects with, so its expansion is converted to a covering string pattern. A projection output and a pruned directory name a *place*. A knob-path token in those two fields therefore resolves to the knob's value **verbatim**, with no covering conversion. Its grammar is the couples token's: `knob:<NAME>` for a file knob, and `knob:<NAME>/<glob>` rooted at a directory knob or locator. Its readers go through one function, so the projection line and the prune field cannot disagree about what a token names.

The mirror root gets its own knob, `CANON_KIT_MIRROR_ROOT`, and **not** `CANON_KIT_LINK_ROOT`. Reusing the link root would let a site root move, but it would not cure the cost the entry names. An adopter whose site is `docs/` and whose own `SPEC.md` sits at `docs/<dir>/` would still have it pruned, because the mirror would still be `docs/`. The mirror needs only to sit *under* the link root, so that its pages and the site's links into it stay on-root. `check-docs-link-convention`'s off-root rule already reds a mirror placed outside the link root, so no second check is minted (canon-kit/SPEC.md §check-docs-link-convention).

**Measured at authoring (2026-09-23).**

- `GATE_SDK_ENFORCEMENT_FILE=site/enforcement.md native/target/release/checkwright-gates check-projection-roster` exits 1: `check-enforcement-fresh: projection glob 'docs/enforcement.md' is not covered by its own couples=`. The entry's inferred step holds.
- `git grep -n "^# projection:" -- '*.gate'` lists four kit members and four members in `scripts/`. The four in `scripts/` are this repo's own descriptors, whose literals are its configuration and stay (gate-sdk/SPEC.md §The `# graph:` manifest, the three keep-cases).
- `GATE_SDK_VERBOSE=1 bash gate-sdk/bin/run-gates.sh --for queue-kit/templates/deep/x.md` runs both `check-footprint-fresh` and `check-surface-ratchet`. `kit:templates/*.md` therefore already covers every depth under a kit's templates, and the `lifecycle-kit/templates/stages/*.md` literal is a hand-enumerated subset of it.
- `native/src/emit/enforcement_map.rs:324-356`, the monitor section, walks every file under `GATE_SDK_ENFORCE_SCAN_DIR` (default `.`) and filters on nothing but a marker line. `.github/workflows/*.yml` is an under-cover of that walk, not a cover. `check-enforcement-fresh` runs in 0.03s on this tree, so a cover of the whole walk costs the hook nothing it would notice.
- `native/src/registry.rs:698-706` (`rooted_pattern`) joins the root and the glob with `/`, then prefixes `*`. At a root of `.`, `knob:X/*` becomes `*./*`, which matches only a path containing `./` and so covers nothing a walk returns. This is read off the code, not run: no descriptor carries a rooted token over a `.`-valued knob today.
- `./native/target/release/checkwright-gates --scratch-run` over a loop of `--reads <member>` for every `--list` member finds 17 members declaring the `docs/*` prune: `check-manifest-count`, `check-prose-enum`, `check-spec-dod-singleton`, `check-spec-derivable-section`, `check-manifest-temporal`, `check-install-claim`, `check-payload-claim`, `check-docs-cmd`, `check-fence-command-head`, `check-fence-run`, `check-md-refs`, `check-tracking-claim`, `check-spec-embedded-source`, `check-knob-citation`, `check-spec-fence-balance`, `check-spec-pointer` and `check-surface-duplication`.

## What changes

### (1) A `.` root contributes no prefix to a rooted couples token {mechanical}

**Not yet applied.** In `native/src/registry.rs` `rooted_pattern`, a root that is `.` after the trailing-`/` trim, or that opens with `./`, is normalized first. `.` becomes the empty prefix and the token expands to `<glob>` alone. A leading `./` is dropped. The covering conversion then runs as today. A crate unit test pins `.` and `./ops` roots.

In gate-sdk/SPEC.md §The `# graph:` manifest, the sentence "It expands to the knob's value, any trailing `/` trimmed, then `/`, then `<glob>`, and the result takes the covering conversion below" becomes:

> It expands to the knob's value — trailing `/` trimmed, a leading `./` dropped, and `.` read as no prefix at all — then `/` and `<glob>`, and the result takes the covering conversion below.

### (2) The two undisposed couples literals {mechanical}

**Not yet applied.**

- `gate-sdk/checks/check-enforcement-fresh.gate`: `.github/workflows/*.yml` becomes `knob:GATE_SDK_ENFORCE_SCAN_DIR/*`. The member already declares that knob (`native/src/gates/mod.rs`, its registry row). The monitor walk reads every file under the knob's root, so the rooted token is the walk's own cover. At the default `.` it expands to `*` after delta 1, so the member fires on every commit. That over-trigger is the covering rule's sanctioned direction.
- `context-kit/checks/check-footprint-fresh.gate` and `context-kit/checks/check-surface-ratchet.gate` drop `lifecycle-kit/templates/stages/*.md`. Their sibling token `kit:templates/*.md` already covers it at every depth, so this is doc-path-couples' first disposition: a hand-enumerated cover for a walk that already has one.
- `lifecycle-kit/checks/check-shim-restatement.gate` drops the same literal on the same ground. It is lifecycle-kit's own path, so the keep-case for a path inside a kit root would allow it, but it is redundant beside `kit:templates/*.md`. It stood on a false premise, which lifecycle-kit/SPEC.md §check-shim-restatement states and which this delta corrects.

In lifecycle-kit/SPEC.md §check-shim-restatement, the sentence "Because the corpus find recurses but the `kit:templates/*.md` couple does not, the one kit template *sub*directory it misses — `lifecycle-kit/templates/stages/*.md`, the stage-skill templates each stage shim binds — is coupled explicitly beside it; a shim's own template is its likeliest collision surface, so it must re-trigger the gate." becomes:

> The `kit:templates/*.md` couple reaches every template subdirectory, since the field's `*` crosses `/` (gate-sdk/SPEC.md §Reading a `couples=` field's reach), so the stage-skill templates each stage shim binds — a shim's likeliest collision surface — re-trigger the gate with no couple of their own.

The two other lifecycle-kit descriptors carrying the literal, `check-skill-binding` and `check-stage-skill-coverage`, couple no `kit:` token beside it. There it is their whole cover of their own kit's path, which the keep-case allows, and it stays.

In gate-sdk/SPEC.md §check-enforcement-fresh, "the gate sources (so a `tier=` edit re-fires), `kpis.list`, the settings file, and the monitor-carrier workflows" becomes "the gate sources (so a `tier=` edit re-fires), `kpis.list`, the settings file, and every file the monitor walk reads, through `knob:GATE_SDK_ENFORCE_SCAN_DIR/*`".

### (3) The knob-path token {design-bearing}

**Not yet applied.** `native/src/registry.rs` gains `knob_paths(token) -> Result<Vec<String>, String>`, the one resolver for a declaration field that names a place:

- A bare literal returns itself.
- `knob:<NAME>` returns the knob's members verbatim: one element for a scalar, and every element for an indexed row. A packed row takes `knob:<NAME>.<field>` as the couples token does.
- `knob:<NAME>/<glob>` takes the rooted token's admissibility and its `.` normalization from delta 1, and returns `<root>/<glob>` with no covering conversion.
- `kit:` is refused. A place a kit gate writes or prunes belongs to the consumer's tree, never to a kit root.
- A knob no static kit owns is exit 2, as in the couples expansion. An empty value returns no element, and a rooted token over an empty root is a refusal, both on the couples token's grounds.

`COUPLES_PREFIXES` is unchanged. The knob-path token is a separate field grammar that shares a spelling with the couples token, and `valid_glob_token` never reads it.

In gate-sdk/SPEC.md §The install disposition, the projection-declaration sentence "`<globs>` is comma-separated in `couples=`' literal-glob syntax and carries no `kit:` or `knob:` token, because an output path is the consumer's tree rather than a kit-relative read; it is read through `registry::PROJECTION`" becomes:

> `<globs>` is comma-separated. Each member is a literal glob or a **knob-path token** — `knob:<NAME>` or `knob:<NAME>/<glob>`, resolved to the knob's value verbatim rather than to a covering pattern, since an output path names a place and not a selection — and never `kit:`, since an output path is the consumer's tree. A kit-shipped descriptor names its output through the knob that relocates it, on the `couples=` field's own literal rule. A line whose every member resolves empty declares no projection on this tree, which is a consumer's designed state (an unset page knob) and not a finding. It is read through `registry::PROJECTION` and resolved through `registry::knob_paths`.

In gate-sdk/SPEC.md §check-projection-roster, assertion A becomes:

> - **A.** A declaring member carries one `# projection:` line with at least one member. No member carries `kit:`. Every resolved path is covered by the member's own expanded `couples=` under the field's one matcher (§Reading a `couples=` field's reach). A member whose line resolves to no path is outside the declaring set, so a roster key naming it reds under C.

In the same section, the exit-2 set "a registered member that does not resolve or cannot be read, and a `couples=` that does not expand" gains "or a `# projection:` knob-path token that does not resolve".

`native/src/gates/projection_roster.rs` `declaring_gates` resolves each line through `knob_paths` and drops a member that resolves to nothing. `declaration_findings` refuses `kit:` alone and asserts coverage over the resolved paths. `native/src/emit/projection_witness.rs` needs no edit, because it takes its members from `declaring_gates` and its perturbation set from triggers.

### (4) The four kit descriptors name their outputs through their knobs {mechanical}

**Not yet applied.** Each `# projection:` line takes the knob its own `couples=` already carries:

- `context-kit/checks/check-footprint-fresh.gate`: `knob:CONTEXT_KIT_FOOTPRINT_FILE`.
- `gate-sdk/checks/check-enforcement-fresh.gate`: `knob:GATE_SDK_ENFORCEMENT_FILE`.
- `gate-sdk/checks/check-graph.gate`: `knob:GATE_SDK_GRAPH_ARTIFACT,knob:GATE_SDK_HOOKS_DIR/pre-commit,knob:GATE_SDK_HOOKS_DIR/commit-msg`.
- `queue-kit/checks/check-roadmap-fresh.gate`: `knob:QUEUE_KIT_ROADMAP_FILE`. That knob's default is empty (queue-kit/SPEC.md §Layout and configuration), so on a tree publishing no roadmap the member leaves the declaring set, where today it declares a page the tree does not have.

The `scripts/` descriptors keep their literals. A new `good/` case in `gate-sdk/gate-tests/check-projection-roster/` relocates a projection through a knob and stays green. The `bad/` case gains a `kit:` member where its prefixed-token line stood.

### (5) The registry prune field takes a knob-path source {design-bearing}

**Not yet applied.** A prune member in a `RootDecl` may be a knob-path token (delta 3), resolved through `registry::knob_paths` by each of its two readers:

- `check-reads-couples`' consumption path, before `under_declared_prune` matches.
- Unit test A's prune half in `native/src/gates/mod.rs` (`every_registry_member_declares_the_roots_it_walks`), before comparing against `run.pruned`, resolved in the fixture case's own configuration as the walk itself resolved it.

`--reads` prints the token unresolved, on the filter field's precedent, so the registry stays config-independent data. In gate-sdk/SPEC.md §check-reads-couples, the prune paragraph's first sentence "The prune field is a list of root-relative directory globs the walk refuses to descend into, and it is held to executed behaviour" becomes:

> The prune field is a list of root-relative directory globs the walk refuses to descend into — each a literal or a knob-path token (§The install disposition) that the field's two readers resolve verbatim, as the filter field's `knob:` source is resolved by its reader and printed unresolved — and it is held to executed behaviour.

### (6) The mirror root is `CANON_KIT_MIRROR_ROOT` {design-bearing}

**Not yet applied.**

- **The knob.** `native/src/knobs/canon_kit.rs` gains `Row::scalar("CANON_KIT_MIRROR_ROOT", "docs")`. The table validator refuses an empty value, `.`, an absolute path and any `..` segment. A mirror at the repository root would write `<dir>/SPEC.md` over its own sources, and a mirror outside the tree cannot be tracked.
- **The generator.** `native/src/emit/docs_mirror.rs` writes under the resolved root, and its usage text names the knob instead of splicing `mirror_root!()`.
- **The comparator.** `native/src/gates/docs_mirror_fresh.rs` takes the destination prefix, the orphan-sweep root and the help line's staging path from the knob. Its registry root, `(crate::spec::MIRROR_ROOT, "name:lit:SPEC.md,README.md,DOCTRINE.md", …)`, becomes a `?` carrying `dynamic@src/gates/docs_mirror_fresh.rs:<line>` of the sweep. That follows `check-docs-link-convention`'s knob-valued root. The member declares `CANON_KIT_MIRROR_ROOT`.
- **The prune.** `native/src/spec.rs` drops `mirror_root!`, `MIRROR_ROOT` and the const `CANON_SPEC_PRUNE`. `canonical_specs` builds its prune at run time from `**/templates` and `<CANON_KIT_MIRROR_ROOT>/*`. `CANON_SPEC_PRUNE_DECL` becomes `**/templates,knob:CANON_KIT_MIRROR_ROOT/*` (delta 5). Each of the 17 members named above declares `CANON_KIT_MIRROR_ROOT` beside `CANON_KIT_SPEC_NAME`. The spec.rs unit test holding the two spellings equal compares the declaration's resolved form with the run-time prune.
- **This repo** inherits the default. No verdict here moves.
- **The fixture.** `scripts/gate-tests/check-docs-mirror-fresh/` gains a `good/` case whose case-local canon knob file sets a relocated root, with its mirror written there. A crate test in `spec.rs` shows an adopter `SPEC.md` at `docs/<dir>/` discovered once the root moves to `docs/ref`.

In canon-kit/SPEC.md §Layout and configuration's Knobs list, after the `CANON_KIT_SCAN_KIT_ROOTS` bullet, add:

> - `CANON_KIT_MIRROR_ROOT` — the directory the on-site mirror is written under and the canonical-spec finder prunes, default `docs`. It must sit at or under `CANON_KIT_LINK_ROOT` so the mirror's links stay on-root, which `check-docs-link-convention`'s off-root rule holds; empty, `.`, absolute or `..`-bearing is malformed config.

In canon-kit/SPEC.md §The shared spec adapters, the mirror-prune bullet's last three sentences, "So the path is kit mechanism rather than a consumer's layout, and it has one spelling, … it needs a `knob:` source for a registry prune that gate-sdk does not yet have.", become:

> The root is `CANON_KIT_MIRROR_ROOT`, read by the generator, the walk, the docs-mirror freshness comparator and — as a knob-path prune — every member's registry declaration, so the four cannot disagree about it (gate-sdk/SPEC.md §check-reads-couples). An adopter whose own `SPEC.md` sits at `<site>/<dir>/` moves the mirror beneath it, to `<site>/ref` for instance, and the finder discovers that spec again.

In canon-kit/SPEC.md §The reference-link grammar, "a generated, freshness-gated projection under `docs/<dir>/`" becomes "a generated, freshness-gated projection under `<mirror root>/<dir>/` (`CANON_KIT_MIRROR_ROOT`, §Layout and configuration)".

## Producers and consumers

- **`CANON_KIT_MIRROR_ROOT`** (delta 6).
  - Producer: canon-kit's table, default `docs`, which this repo inherits.
  - Consumers: the generator, the comparator, `canonical_specs`, and the registry prune declaration through delta 5's two readers.
  - Roster-holding readers: `check-knob-default-coupling` reds a knob whose owning SPEC states no default, which the Knobs bullet does state. `check-graph`'s admissibility loop reads `couples=` and not the prune field, so no descriptor token is owed. `--emit knob-roster` is derived. `templates/canon-config.knobs` is a comment-only pointer. `check-kit-ref-liveness` resolves the name against canon-kit's table.
- **The knob-path token** (delta 3). Producer: the four descriptors in delta 4 and the prune declaration in delta 6. Consumers: `check-projection-roster`, which resolves it at `declaring_gates`, and delta 5's two prune readers. `--projection-witness` reads only the declaring set. No other reader of `# projection:` exists: `git grep -n "registry::projection(" native/src` returns `projection_roster.rs` alone.
- **Point 5, narrowing.**
  - Delta 2 drops three redundant literals and widens one trigger. `check-reads-couples` is the reader whose verdict turns on a couples field's reach. `check-enforcement-fresh`, `check-footprint-fresh`, `check-surface-ratchet` and `check-shim-restatement` each declare only `?` roots (`--reads`), so it asks nothing of them. None of the three drops moves a trigger, since each dropped glob sits inside its sibling `kit:` token's reach.
  - Delta 4's roadmap member leaving the declaring set is a narrowing. `check-projection-roster` assertion B reds a declaring member with no row, and C reds a key naming a non-declaring member. This tree binds `QUEUE_KIT_ROADMAP_FILE = ROADMAP.md` (`scripts/queue-config.knobs`), so the member stays declaring and its row stays valid.
  - Delta 6 moves `check-docs-mirror-fresh`'s root to `?`. `check-reads-couples` reds an uncovered read under a declared root, and it skip-counts a `?`, so this removes one asserted walk and adds one counted skip. Its clean line's skip count rises by one, and it asserts no exact count.
- **Point 6.** The obliged corpora are enumerable. There are four kit `# projection:` lines (delta 4 names each value), two undisposed literals (delta 2), and 17 prune-declaring members (the probe above; each declares the knob).

## Existing sections updated

Roster from `git grep -n "^# projection:" -- '*.gate'`, `git grep -n "MIRROR_ROOT\|mirror_root!\|CANON_SPEC_PRUNE" native/src`, `git grep -n "docs/<dir>/\|monitor-carrier\|trailing \`/\` trimmed" -- '*/SPEC.md'` and the `--reads` loop above, run 2026-09-23.

- `gate-sdk/SPEC.md` §The `# graph:` manifest (delta 1).
- `native/src/registry.rs` `rooted_pattern` and its unit test (delta 1).
- `gate-sdk/SPEC.md` §check-enforcement-fresh (delta 2).
- `gate-sdk/checks/check-enforcement-fresh.gate`, `context-kit/checks/check-footprint-fresh.gate` and `context-kit/checks/check-surface-ratchet.gate` (deltas 2 and 4).
- `lifecycle-kit/checks/check-shim-restatement.gate` and `lifecycle-kit/SPEC.md` §check-shim-restatement (delta 2).
- `native/src/registry.rs` `knob_paths` (delta 3).
- `native/src/gates/projection_roster.rs` and the `gate-sdk/gate-tests/check-projection-roster/` cases (deltas 3 and 4).
- `gate-sdk/SPEC.md` §The install disposition and §check-projection-roster (delta 3).
- `gate-sdk/checks/check-graph.gate` and `queue-kit/checks/check-roadmap-fresh.gate` (delta 4).
- `gate-sdk/SPEC.md` §check-reads-couples and `native/src/gates/reads_couples.rs` (delta 5).
- `native/src/gates/mod.rs`: unit test A's prune half (delta 5), and `check-docs-mirror-fresh`'s root declaration plus the 17 members' knob declarations (delta 6).
- `native/src/spec.rs`: the constants, `canonical_specs` and the equality test (delta 6).
- `native/src/emit/docs_mirror.rs` (delta 6).
- `native/src/gates/docs_mirror_fresh.rs` (delta 6).
- `native/src/knobs/canon_kit.rs` and `scripts/gate-tests/check-docs-mirror-fresh/` (delta 6).
- `canon-kit/SPEC.md` §Layout and configuration, §The shared spec adapters and §The reference-link grammar (delta 6).
- `scripts/git-hooks/pre-commit` and the graph artifact, regenerated (all deltas).
- `.workflow/release-declarations.md`, one Behavior changes bullet (all deltas):
  - a `# projection:` line may name its output through `knob:`, and the kit freshness gates now do, so relocating a projection through its knob no longer reds `check-projection-roster`;
  - `check-enforcement-fresh` fires on any file under `GATE_SDK_ENFORCE_SCAN_DIR`;
  - the new `CANON_KIT_MIRROR_ROOT` relocates the on-site SPEC mirror and the canonical-spec prune together.
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/canon-kit/SPEC.md` and `docs/lifecycle-kit/SPEC.md`.

## Retired spellings

- `MIRROR_ROOT` — the crate constant replaced by the knob's resolved value (delta 6).
- `mirror_root!` — the macro deleted with the constant (delta 6).
- `CANON_SPEC_PRUNE` — the const prune list `canonical_specs` builds at run time instead (delta 6).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the token, the knob and each re-spelled declaration.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved.** `projection-path-literals`, `docs-mirror-root-unrelocatable` and `couples-literals-undisposed` move to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on both declared spellings.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
