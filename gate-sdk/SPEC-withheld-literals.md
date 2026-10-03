# SPEC amendment: withheld-literals

**`check-consumer-value-literal` widens its corpus from the kit-shipped gates' crate modules to every gate module, and each consumer value the widened corpus reds on moves to a descriptor-declared knob or an existing static knob, or takes the valve with its ground.** Today a member whose descriptor resolves to the gates dir is left out (§check-consumer-value-literal), so the 31 gates this repo withholds (`ls scripts/*.gate`) keep their literals: `const DEFAULT_<NAME>` positional defaults, projection paths, install-page and workflow names, release-note section names. Each is configurable only by a positional a fixture passes, since `scripts/gates.list` rows carry no arguments.

**The knob half has no mechanism yet.** §The declaration cohort rules that a knob no static kit's prefix owns is declared on the `.gate` descriptor of the consumer gate that reads it, resolving from the environment and then the descriptor default, and leaves the descriptor line's grammar to the first member that needs it. None has: `git grep -n '# knob:' -- '*.gate' '*.rs' '*SPEC.md'` returns nothing, and the knob resolver (`native/src/knobs/mod.rs`, `static_row`) refuses every name no static kit owns. This amendment is that first member, so it owns the grammar.

**Measured at authoring**, by a scratch re-implementation of the corpus derivation (the registry's module cut, the literal reader and both arms) over the 31 withheld members:

- **Corpus.** 65 files outside `knobs/`; 14 are already in the shipped corpus and read clean today, and 2089 literals are read in the other 51.
- **Findings:** 88, 66 path-arm and 22 heading-arm; 22 are `const DEFAULT_<NAME>`, 21 of them in withheld modules. Three are census artefacts (a kit root the scratch roster missed, and the generator's own headings counted twice), so delta 3's table carries the rest.
- **Borderline**, no arm firing (a templated path, a kit-owned path, a file-format name, a ref pattern, a one-word heading): about 70, which the gate does not read and this amendment does not edit.

**Inferred, cannot run before build:** that the real gate's widened findings equal delta 3's table — the census approximates the binary's walk and prune set, and the widened gate exists only once delta 2 lands.

**One unit, by direction.** The amendment is sized well beyond the entry's event/low filing, and whether to split it was put to the operator, who answered 2026-10-03, lead-relayed (a direction, not a ruling): keep it one unit as authored. Deltas 1 to 3 land together because delta 2's widened gate reds on every unconverted literal.

**Landing order against the sibling amendments.** Delta 3 lands after criterion-five's delta 5, which removes `check-install-platforms`' arm D, drops two knobs from that member's registry row and prunes its fixture pair, so the conversion edits the module and row that remain. It also lands after layout-neutral's delta 4, which reshapes `kit_cell` in `emit/enforcement_map.rs`, a module delta 3 valves. Delta 2 lands last of all three amendments' code, so the widened gate's first run reads every sibling edit; whichever commit lands last re-points the `dynamic@` anchors `check-reads-couples` names. The amendments' SPEC edits touch disjoint paragraphs of gate-sdk/SPEC.md and their release-declaration bullets append, so neither needs an order.

## What changes

### (1) A descriptor declares its knobs on `# knob:` lines {design-bearing} {user-facing: gate-sdk/SPEC.md §The declaration cohort, which rules that a knob no static kit's prefix owns is declared on the reading gate's descriptor, resolving from the environment then the descriptor default, and leaves the line grammar to the first member needing it, this one; with the operator direction recorded on withheld-gate-literal-knobs}

A `.gate` descriptor may carry `# knob:` lines, the text after the token being **one knob-file line** in §The knob file's line forms (`NAME = value`, or `NAME[] = element` per element of an indexed knob), read by the knob-file line reader rather than a second parser. A declared name carries the `GATE_LOCAL_` prefix, which no static kit's prefix matches; a `# knob:` line naming a static kit's knob, or a name declared on two descriptors, is a refusal at exit 2 naming the files, so each name has one producer.

The knob resolver gains a branch beside the static table: a name no static kit owns is looked up in the descriptors of the resolved gates dir and resolves from the environment, then the declaring line's value; an undeclared name stays refused, as today. A reader's precedence is its positional operand, then the knob, so every fixture steering a gate through its positional is unchanged, and the positional's compiled `const DEFAULT_<NAME>` fallback is deleted. A member declares the names it reads in its registry row's knob slice, which `check-reads-couples` and `check-gate-substrate-parity` hold to what its module reads. A knob-file layer stays unruled: `scripts/*.knobs` refuses a `GATE_LOCAL_` name, the fail-closed state for an unowned name.

**A fixture case declares its own.** §run-gate-tests resolves knob values inside the case dir, so the descriptor branch reads the case's own gates dir, where no live descriptor sits. A case that reaches a declared read without steering it through a positional therefore carries the declaring line in its own `<gates-dir>/<gate>.gate`, holding the default or the case's value, as it carries its own knob file; the `check-consumer-value-literal` pair already carries descriptors inside its cases. A heading-valued knob has no positional spelling, since a case's `args` file is split on whitespace. Exporting the live descriptor's values onto the case invocation is refused: the case would read this repository's configuration beside its own, the two oracles §run-gate-tests refuses for knob values.

Unit tests: a declared scalar and an indexed knob resolve environment-first; a static name on a `# knob:` line refuses; two declarations of one name refuse; an undeclared `GATE_LOCAL_` name stays refused, whatever the environment carries.

### (2) The gate reads every gate module {design-bearing}

`native/src/gates/consumer_value_literal.rs`. The corpus is the union of every registry member's `registry::module_files` cut, whichever directory its descriptor resolves to, less every file under `<GATE_SDK_NATIVE_SRC>/knobs/`; a member resolving nowhere is still left out. The exit-2 condition *no kit-shipped member resolves* becomes *no member resolves*. The clean line's counts stay a report with no floor. The gate stays consumer-declared in this repo's gates dir, its `couples=` unchanged.

The module's own spellings of the old scope lose *kit-shipped*: its header `spec:` comment, the exit-2 message *no kit-shipped member resolves under any kit root* (now *no member resolves under the gates dir or any kit root*), the red header *kit-shipped gate source carries a consumer value as a literal*, and the clean line's *kit-shipped member(s) swept*, which becomes *member(s) swept*. So do `scripts/check-consumer-value-literal.gate`'s `# spec:` line and `good/expect.txt`'s clean line, whose counts move with the new `good/` case.

Fixtures, `scripts/gate-tests/check-consumer-value-literal/`: `good/`'s module whose member is withheld to the case's gates dir carries its literal valved; the same literal unvalved becomes a `bad/` case with its `expect.txt` line.

This delta lands with or after the last of delta 3's conversions, never before: the widened gate reds on every unconverted literal.

### (3) Every finding takes a knob or a valve {design-bearing} {user-facing: the operator direction recorded on withheld-gate-literal-knobs, avoid any hard code in gates, settling that the shared emit modules behind the publisher's repo-local arms read their paths from knobs}

Build runs the widened gate and disposes of every finding it reports; none is re-expected. The tables are the predicted roster and the gate's first run binds. **K** is a new `GATE_LOCAL_` knob declared on the owning descriptor, its default the literal; **R** an existing static knob read instead, joined with a fixed file name where the literal was a path under it; **V** a `// consumer-value-exempt:` valve citing the ground given.

**New knobs**, each declared once, on the first gate listed:

| Name | Shape | Default | Read by |
|---|---|---|---|
| `GATE_LOCAL_KIT_REGISTRY_PAGE` | scalar | `docs/kits.md` | check-docs-kit-parity |
| `GATE_LOCAL_OFFNAV_LIST` | scalar | `scripts/docs-offnav.list` | check-docs-nav-reachable |
| `GATE_LOCAL_FRONT_DOOR_SURFACES` | indexed | the seven paths `front_door_verbs.rs` lists | check-front-door-verbs |
| `GATE_LOCAL_CHECKOUT_SECTION` | scalar | `## This repo, governed` | check-front-door-verbs |
| `GATE_LOCAL_README_FILE` | scalar | `README.md` | check-front-door-verbs, check-license-line |
| `GATE_LOCAL_INSTALLER_DIR` | scalar | `installer` | check-front-door-verbs, check-installer-no-deps, check-packed-links |
| `GATE_LOCAL_INSTALL_EVIDENCE_PAGE` | scalar | `docs/install-evidence.md` | check-install-evidence-fresh |
| `GATE_LOCAL_INSTALL_PAGE` | scalar | `docs/install.md` | check-install-pin, check-install-platforms, check-install-toolchain, check-release-channel-parity |
| `GATE_LOCAL_INSTALL_SH` | scalar | `docs/install.sh` | check-install-pin, check-plugin-parity, and `pinned_release.rs`' readers |
| `GATE_LOCAL_INSTALL_PS1` | scalar | `docs/install.ps1` | check-install-pin |
| `GATE_LOCAL_BOOTSTRAP_SH` | scalar | `installer/bin/checkwright.sh` | check-install-platforms |
| `GATE_LOCAL_BOOTSTRAP_PS1` | scalar | `installer/bin/checkwright.ps1` | check-install-platforms |
| `GATE_LOCAL_CONTRIBUTING_FILE` | scalar | `CONTRIBUTING.md` | check-install-toolchain |
| `GATE_LOCAL_RELEASE_POSTS_DIR` | scalar | `docs/posts` | check-release-bump, check-release-declaration-parity, check-tightened-gates-grammar, check-kit-ref-liveness |
| `GATE_LOCAL_EVIDENCE_PAGE` | scalar | `docs/evidence-data.md` | check-trajectory-fresh, check-kit-ref-liveness |
| `GATE_LOCAL_LICENSE_CONF` | scalar | `scripts/license-line.conf` | check-license-line |
| `GATE_LOCAL_LICENSE_HEADING` | scalar | `## License` | check-license-line |
| `GATE_LOCAL_WORKFLOWS_DIR` | scalar | `.github/workflows` | check-npm-publish-spec |
| `GATE_LOCAL_PLUGIN_DIR` | scalar | `plugin` | check-plugin-parity |
| `GATE_LOCAL_MARKETPLACE_FILE` | scalar | `.claude-plugin/marketplace.json` | check-plugin-parity |
| `GATE_LOCAL_PRODUCT_STATEMENT_CONF` | scalar | `scripts/product-statement.conf` | check-product-statement-fresh |
| `GATE_LOCAL_PRODUCT_STATEMENT_SURFACES` | indexed | the five sites `product_statement.rs` lists | check-product-statement-fresh |
| `GATE_LOCAL_SUPPORT_TABLE_PAGE` | scalar | `docs/spec-toolkits.md` | check-support-table-fresh |
| `GATE_LOCAL_COMPANION_DIR` | scalar | `companion` | check-support-table-fresh |
| `GATE_LOCAL_VALUE_PAGE` | scalar | `docs/value.md` | check-value-rollup-fresh |

A path under a knob's directory keeps its file name as a literal tail, a file-format name and no consumer value (`package.json`, `profiles.list` and `README.md` under `GATE_LOCAL_INSTALLER_DIR`).

**Findings by gate:**

| Gate (module) | Finding | Disposition |
|---|---|---|
| check-docs-kit-parity | `docs/kits.md` | K `GATE_LOCAL_KIT_REGISTRY_PAGE` |
| check-docs-mirror-fresh (`docs_mirror_fresh.rs`, `emit/docs_mirror.rs`) | `README.md`, a mirrored member name | V: the kit layout's member names, which every kit root carries |
| check-docs-nav-reachable | `docs`; `scripts/docs-offnav.list` | R `SITE_KIT_DOCS_DIR`; K `GATE_LOCAL_OFFNAV_LIST` |
| check-front-door-verbs | `installer/README.md`, `installer/profiles.list` | under `GATE_LOCAL_INSTALLER_DIR` |
| | `native/src/emit/mod.rs`, `native/src/main.rs` | R `GATE_SDK_NATIVE_SRC`, joined with `emit/mod.rs` and `main.rs` |
| | the seven-path surface list; `README.md`; `## This repo, governed` | K `GATE_LOCAL_FRONT_DOOR_SURFACES`, `GATE_LOCAL_README_FILE`, `GATE_LOCAL_CHECKOUT_SECTION` |
| check-guard-registration | `The rule roster` | V: guard-kit/SPEC.md's section, a kit constant the docs mirror also carries |
| check-install-evidence-fresh | `docs/install-evidence.md` | K `GATE_LOCAL_INSTALL_EVIDENCE_PAGE` |
| check-install-pin | `docs/install.sh`, `docs/install.ps1`, `docs/install.md` | K `GATE_LOCAL_INSTALL_SH`, `_PS1`, `GATE_LOCAL_INSTALL_PAGE` |
| check-install-platforms | `docs/install.md`; `native/targets.list`; the two bootstraps | K; R `GATE_SDK_NATIVE_TARGETS_FILE`; K `GATE_LOCAL_BOOTSTRAP_SH`, `_PS1` |
| (`pinned_release.rs`) | `docs/install.sh` | K `GATE_LOCAL_INSTALL_SH` |
| check-install-toolchain (and `toolfloor.rs`) | `docs/install.md`, `CONTRIBUTING.md`; `native/src/toolfloor.rs` | K; K `GATE_LOCAL_CONTRIBUTING_FILE`; R `GATE_SDK_NATIVE_SRC` joined |
| check-installer-no-deps | `installer/package.json`; `scripts`, the npm lifecycle key | under `GATE_LOCAL_INSTALLER_DIR`; V: a `package.json` key, no layout |
| check-kit-ref-liveness | `docs/posts/`, `docs/evidence-data.md`; `.workflow/release-declarations.md` | K; R `GATE_SDK_WORKFLOW_DIR` joined |
| check-kit-roots-dialect | `scripts` | R the gates-dir knob's default |
| check-license-line | `scripts/license-line.conf`, `## License`, `README.md` | K `GATE_LOCAL_LICENSE_CONF`, `GATE_LOCAL_LICENSE_HEADING`, `GATE_LOCAL_README_FILE` |
| check-npm-publish-spec | `.github/workflows` | K `GATE_LOCAL_WORKFLOWS_DIR` |
| check-packed-links (`emit/pack_installer.rs`) | `installer/package.json`, `installer` | `GATE_LOCAL_INSTALLER_DIR` |
| check-plugin-parity | `plugin`, `.claude-plugin/marketplace.json`; `docs/install.sh`; `LICENSE` | K; K `GATE_LOCAL_INSTALL_SH`; R `GATE_SDK_PAYLOAD_LICENSE` |
| check-product-statement-fresh (`emit/product_statement.rs`) | `scripts/product-statement.conf`; the five sites | K `GATE_LOCAL_PRODUCT_STATEMENT_CONF`, `_SURFACES` |
| check-release-bump | `docs/posts`; `In brief`, `Tightened gates`, `Renamed knobs`, `Behavior changes` | K `GATE_LOCAL_RELEASE_POSTS_DIR`; V: the release-note section set, which `release-note-section-set-derivation` makes one knob-owned roster |
| check-release-change-declared | `Behavior changes` | V, the same ground |
| (`installer/mod.rs`) | `scripts`, `CLAUDE.md`, `TASK-QUEUE.md`, `.workflow/WORKFLOW-STATE.txt` | V: installer/SPEC.md §What init seeds, the layout `init` writes a new tree against |
| check-release-channel-parity | `docs/install.md`; `.github/workflows/publish.yml` | K `GATE_LOCAL_INSTALL_PAGE`; R `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` |
| check-release-declaration-parity | `docs/posts`; `Tightened gates`, `Renamed knobs`, `Behavior changes` | K; V, the release-note ground |
| check-rule-citation | `## The delegation model` | V: delegation-kit/SPEC.md's section, a kit constant |
| check-support-table-fresh (`emit/support_table.rs`) | `docs/spec-toolkits.md`, `companion` | K `GATE_LOCAL_SUPPORT_TABLE_PAGE`, `GATE_LOCAL_COMPANION_DIR` |
| check-tightened-gates-grammar | `docs/posts`; `Tightened gates` | K; V, the release-note ground |
| check-trajectory-fresh | `docs/evidence-data.md` | K `GATE_LOCAL_EVIDENCE_PAGE` |
| check-value-rollup-fresh (`emit/value_rollup.rs`, `emit/enforcement_map.rs`) | `docs/value.md`; the projection's own section headings; kit SPEC headings and paths | K `GATE_LOCAL_VALUE_PAGE`; V: headings the generator writes; V: kit constants |

`emit/upgrade_smoke.rs` holds `Tightened gates` and `declaration.rs` holds the section names only in its test module, so neither is in the corpus; both belong to `release-note-section-set-derivation`'s roster. A descriptor's `couples=` keeps its literal paths: a `knob:` token roots at a directory knob, so a file knob has no couples spelling, and a descriptor is outside the crate corpus.

**A knob is read where its literal was, and nowhere earlier.** A converted read sits on the path that read the literal, so a path that never opened the file reads no knob and keeps its verdict in every tree.

**The `check-docs-kit-parity` row's walk root turns `?`.** Its registry row declares the literal root `docs` because the index glob hangs off the directory of a positional whose default was a literal (`native/src/gates/mod.rs`, the row's `spec:` comment). Once that default is `GATE_LOCAL_KIT_REGISTRY_PAGE`'s value the root is knob-valued, so the row declares `?` as `check-docs-link-convention`'s knob-valued root does, and the comment states that ground.

**The shared emit modules.** `emit/product_statement.rs`, `emit/support_table.rs`, `emit/value_rollup.rs` and `emit/pack_installer.rs` back both a withheld gate and an arm of the shipped binary, each a publisher-local projection or the publisher's packer (docs/site-architecture.md §Generated projections and their freshness gates). Run in a git tree carrying none of this repository's files, `--emit product-statement`, `--emit support-table`, `--emit value-rollup --write` and `--pack-installer` each exit 2 today on the missing file their literal names, and the bare `--emit value-rollup` exits 0, its page read sitting under `--write` alone. After the conversion the four refusing paths exit 2 naming the undeclared knob, and the bare value-rollup arm reads no knob and is unchanged. Delta 6 declares the moved message.

### (4) The owning sections state the widened rule {mechanical}

gate-sdk/SPEC.md. **Not yet applied.**

- §check-consumer-value-literal's invariant becomes *every string literal in a gate's crate modules that names a consumer's tracked path or heading is read from a knob, or its site is declared*; the first sentence of *The corpus is derived, never listed* becomes *The corpus is the union of every registry member's `registry::module_files` cuts, whichever directory its descriptor resolves to, less every file under `<GATE_SDK_NATIVE_SRC>/knobs/`, since the knob tables and the descriptor knob lines are the sanctioned producers of a consumer value*; the *Verdicts* exit-2 clause reads *when no member resolves*; *Consumer-declared, and why*'s fixture sentence adds that one module's member is withheld to the case's gates dir.
- §The fourth budget batch, the paragraph *Every consumer value a member reads is a knob with one producer*: its last sentence becomes *§check-consumer-value-literal holds the crate-literal form for every gate, shipped or withheld.*
- §Consumer payload, the paragraph *A kit gate ships because it serves an adopter, configurably*: its sentence on the configurability half's crate-literal form names every gate rather than the shipped ones.
- §The declaration cohort, the paragraph *A knob no static kit's prefix owns is declared on the `.gate` descriptor…* is rewritten to delta 1's grammar: the `# knob:` line as one knob-file line, the `GATE_LOCAL_` prefix, one declaration per name, resolution from the positional, the environment, then the declared value, and the `scripts/*.knobs` refusal as the unowned state. It states that `check-docs-cmd` assertion B and `check-knob-citation` derive their vocabularies from kit prefixes, so a `GATE_LOCAL_` name in prose is held by neither.

### (5) Projections {mechanical}

`docs/gate-sdk/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 4; any other projection the battery reds on is regenerated with the command its freshness gate prints.

### (6) The release declarations {mechanical}

`.workflow/release-declarations.md`, under Behavior changes, in installer/SPEC.md §The upgrade contract's grammar, each in the commit landing its behavior. **Not yet applied.** The resolver and the four arms ship in the binary, so each change is a shipped one whatever no adopter is expected to meet.

> - **gate-sdk knob resolution, `.gate` descriptors** — a descriptor in your gates directory may declare a knob no kit owns on a `# knob:` line, one knob-file line naming a `GATE_LOCAL_` knob, which resolves from the environment and then the declared value. A `# knob:` line naming a kit's knob, or a name two descriptors declare, is refused at exit 2. Nothing to do; no kit-shipped gate reads such a knob.
> - **`--emit product-statement`, `--emit support-table`, `--emit value-rollup --write`, `--pack-installer`** — in a tree whose gates directory declares none of their `GATE_LOCAL_` knobs, each still exits 2, naming the undeclared knob rather than the missing file; the bare `--emit value-rollup` is unchanged. Nothing to do; these arms serve the publishing repository's own projections and packer.

## Producers and consumers

- **The `# knob:` line.** Producer: a descriptor in this repo's gates dir, or in a fixture case's own gates dir (delta 1); a withheld descriptor never ships, so no adopter tree holds one. Consumers: the knob resolver's new branch, called through `walk::knob_scalar` and the indexed read; the member's registry knob slice.
- **The environment** is the only consumer-set layer for a declared name, which is what makes a withheld gate configurable here beyond a fixture's positional. The scrub that strips an inherited static-prefix variable from a child resolving its own tree (`knobs::inherited_under_static_prefixes`) is not widened to `GATE_LOCAL_`: such a value reaching a child is refused there unless the child's own gates dir declares the name, and no scratch tree a smoke builds carries a declaring descriptor.
- **Roster-holding readers of the new names**, by `grep -rn "static_row\|declared_knobs\|knob_files" native/src --include=*.rs` and `grep -n "prefix" native/src/gates/docs_cmd.rs native/src/gates/knob_citation.rs`: the descriptors and each reading member's registry row; `check-reads-couples` and `check-gate-substrate-parity`, holding the slice to the module's reads; the resolver's refusal text. `check-docs-cmd` assertion B and `check-knob-citation` derive kit prefixes and do not see `GATE_LOCAL_`, the stated honest limit. `--emit knob-roster` and `check-knob-default-coupling` read the static table only and are unchanged.
- **Red conditions on the corpus change (point 5):**
  - `check-consumer-value-literal` reds on every unconverted finding, which is why delta 2 lands last, and exits 2 only when no member resolves, which a compiled member table forbids; its counts carry no floor.
  - *Inverse:* `good/`'s withheld literal moves to `bad/` unvalved, so a half-applied widening reds the fixture.
  - *Superset:* `check-reads-couples` reds when observed walk roots exceed declared ones, so a gate now walking a knob-valued directory declares it with `?`, as `check-docs-link-convention`'s row does today and `check-docs-kit-parity`'s row will (delta 3).
  - *The kit-shipped half:* its corpus is a subset of the new one, so its verdicts do not move.
- **Every member's satisfying value (point 6):** each finding in delta 3's table carries a knob and default, an existing knob, or a valve and ground; the release-note section names take the valve on `release-note-section-set-derivation`'s ground. A finding the real gate reports beyond the table takes the same three dispositions at build.

## Existing sections updated

Roster by `grep -n "kit-shipped\|crate-literal\|consumer-declared\|A knob no static" gate-sdk/SPEC.md`, `grep -rln "DEFAULT_\|fresh::positional" native/src/gates native/src/emit` and the census above, over the tracked tree.

- `gate-sdk/SPEC.md` — §check-consumer-value-literal, §The fourth budget batch, §Consumer payload and §The declaration cohort, one paragraph each outside the first (delta 4).
- `native/src/knobs/mod.rs` — the resolver's descriptor branch (delta 1).
- `native/src/registry.rs` — the descriptor `# knob:` reader (delta 1).
- `native/src/gates/consumer_value_literal.rs` — the widened corpus and its tests (delta 2).
- `scripts/gate-tests/check-consumer-value-literal/` — the fixture pair (delta 2).
- `native/src/gates/` — each module delta 3's table names (delta 3).
- `native/src/emit/` — each emit module delta 3's table names (delta 3).
- `native/src/gates/pinned_release.rs` — its install-script read (delta 3).
- `native/src/installer/mod.rs` — the four valves (delta 3).
- `native/src/toolfloor.rs` — its own-path read (delta 3).
- `native/src/gates/mod.rs` — each converted member's knob slice, `check-docs-kit-parity`'s walk root and its comment, and the `dynamic@` anchors `check-reads-couples` names once the valves shift their lines (delta 3).
- `scripts/` — each owning `.gate` descriptor's `# knob:` lines (delta 3).
- `scripts/gate-tests/` — each case that reaches a declared read without its positional, carrying the declaring line in its own gates dir (delta 1).
- `scripts/check-consumer-value-literal.gate` — its `# spec:` line (delta 2).
- `.workflow/release-declarations.md` — two Behavior changes bullets (delta 6).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 5).

## Retired spellings

- None — the deleted `DEFAULT_<NAME>` constants are crate-private names no doc spells, and the change retires no surface name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — no template or agent definition is touched.
- [ ] **Merged with no information lost** — §The declaration cohort states the descriptor-knob grammar whole, and §check-consumer-value-literal reads as one section over every gate.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration while sibling gate-sdk amendments are in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Release declarations before the tag** — delta 6's bullets land in the commit landing each behavior.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery with the widened gate green, and this repo's withheld-gate fixture suite in the batch landing delta 2.
- [ ] **The entry is done** — `withheld-gate-literal-knobs` moves to Done in the commit landing delta 2, before the drain stage.
