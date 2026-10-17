# SPEC amendment: manifest-license

Pairs `crate-manifest-license-unheld`. The crate manifest `native/Cargo.toml` declares a license the source does not carry, and no gate reads a package manifest's license field: `check-license-line` reads the README and site license lines only (docs/site-architecture.md §The license line). This amendment corrects the field and widens that gate to hold every tracked package manifest's license field to the license `scripts/license-line.conf` declares, in one unit by Enforcement-first.

**The seam.** `check-license-line` is a consumer gate: its descriptor sits under `scripts/`, its rule in the crate, and every value it holds is consumer config: the license, the manifest roster and the manifest names in `scripts/license-line.conf` and the descriptor's `# knob:` lines. No kit mechanism ships from this unit, and no private rule content enters one.

**The refused alternative: a sibling gate.** The deliverable leaves the choice open. A sibling would read the same source for the same license value and duplicate its parser, its malformed-source refusals and its coverage-walk shape. The source already declares where the license is stated, so a manifest is one more kind of site. **Nor a manifest list without a walk:** the defect this unit closes is a manifest no gate knew about, and a declared-only roster repeats it for the next manifest added.

## What changes

### (1) The crate manifest declares the source's license {mechanical} {user-facing: crate-manifest-license-unheld's Deliverable, "the field set to the source's license"}

`native/Cargo.toml`'s `[package]` `license` field takes the value `scripts/license-line.conf`'s `license` key declares, `Apache-2.0`. **Not yet applied.** The crate reads neither `CARGO_PKG_LICENSE` nor any other license value (`git grep -n 'CARGO_PKG_LICENSE' native/src`, no hit), so the edit changes no output, only what a reader of the public manifest is told.

The set record orders this field before the description edit on the same manifest (`evaluator-agent-discovery` surface (d)). The act is the same whether or not that sibling lands in the same batch: the two edits touch different lines. Where both land in one batch, this field goes first and the description second.

### (2) `check-license-line` holds every package manifest's license field {design-bearing}

The source gains one key and the gate a second coverage walk and a field reader per manifest kind.

- **The key.** `manifest <path>` declares a tracked package manifest, one path per line, root-relative and `/`-spelled. A path declared twice, or naming a kind the gate has no reader for, is a malformed source.
- **The reader, by kind.** A `Cargo.toml` is read for the `license` key of its `[package]` table, the span from that header to the next table header, its basic-string value unquoted. A `.json` manifest is parsed, through the crate's JSON reader, for its top-level object's `license` string. A `.yml` or `.yaml` manifest is read for the first line, at any indentation, whose key is `license`, its value trimmed and one pair of enclosing quotes dropped. A value that is not a string, or a `.json` manifest that does not parse, is a manifest the gate cannot read.
- **The comparison.** Each declared manifest's field equals the source's `license` value exactly. A declared manifest whose field differs reds, naming the value it reads and the value it owes. One carrying no field reds as carrying none.
- **The coverage walk.** Every file under the root whose name `GATE_LOCAL_LICENSE_MANIFEST_NAMES` lists, the gate-sdk prune set applied, that carries a license field by the reader above, is a declared manifest. One that is not reds as undeclared. A walked file carrying no field is no finding, so a manifest with nothing to state needs no declaration.
- **The knob.** `GATE_LOCAL_LICENSE_MANIFEST_NAMES[]`, a descriptor `# knob:` array, this repository's values `Cargo.toml`, `package.json`, `plugin.json` and `extension.yml`.
- **Output.** A finding is one indented line per manifest, as a site's is. The `help:` block adds that a manifest's field takes the declared license and that every manifest carrying one is declared. The clean line adds the count of manifests holding the license.

**Every member's satisfying value** (the causal-completeness check, point 6), probed by `git ls-files | grep -E '(^|/)(Cargo.toml|package.json|plugin.json|extension.yml)$' | grep -v gate-tests`, six files, each declared in `scripts/license-line.conf` with a `manifest` line:

| Manifest | Field read today | Owed |
|---|---|---|
| `companion/speckit/extension.yml` | `license: "Apache-2.0"` | holds |
| `installer/package.json` | `"license": "Apache-2.0"` | holds |
| `native/Cargo.toml` | `license = "MIT"` | `Apache-2.0`, by delta 1 |
| `plugin/.claude-plugin/plugin.json` | `"license": "Apache-2.0"` | holds |
| `plugin/plugin.json` | `"license": "Apache-2.0"` | holds |
| `reserve/crates/Cargo.toml` | `license = "Apache-2.0"` | holds |

`.claude-plugin/marketplace.json` is no member: its name is not on the knob and it carries no top-level license field, its plugin's license being `plugin/plugin.json`'s. The fixture manifests under `scripts/gate-tests/` and `gate-sdk/gate-tests/` sit inside the prune set.

Replacement for docs/site-architecture.md §The license line, both paragraphs. **Not yet applied.**

> Every README's `## License` section, the site footer and every package manifest's license field state the one license `scripts/license-line.conf` declares. A README or the footer carries it as one sentence: `Licensed under Apache-2.0`, then where that copy's license text is. A site a reader follows by link takes the linked form, the license name linked to the text. A README that travels without the repository takes the located form, which names `LICENSE` and says where it sits beside that copy. A manifest carries the license name alone, the value its registry reads. The source declares the license, each location clause, each site's form and each manifest. `check-license-line` holds each site to its rendering and each manifest's field to the license, and holds every README carrying the heading and every manifest carrying a license field to a declaration, printing the expected value on a red. The mirrors are generated from their sources and are not sites. The footer states the license on every page, so a hand-written page under the source's `pages` root carries no License section of its own, and the gate reds one that does.
>
> A markdown site's section, blank lines dropped, is exactly its one line. An `.html` site carries exactly one line opening `Licensed under`, in the linked form only. A manifest's field is read by its kind: the `license` key of a `Cargo.toml`'s `[package]` table, the top-level `license` string of a `.json` manifest, the first `license:` key of a `.yml` one, its quotes dropped. A declared manifest carrying no field reds. The coverage walks read every `README.md` under the root, every file whose name `GATE_LOCAL_LICENSE_MANIFEST_NAMES` lists, and every other `.md` page under `pages`. All three apply the gate-sdk prune set, and the README and page walks skip a file whose front matter sets `generated: true`, the block read as the front-matter span. The gate exits 2 when it cannot read the source, a site or a manifest. It also exits 2 on a malformed source: a key missing, repeated or unknown, a duplicate declaration, a clause named but undeclared, a form other than `link` or `at`, or a manifest of a kind it has no reader for. Its positional form is `check-license-line [source root]`.

## Producers and consumers

- **The `manifest` key.** Producer: a maintainer's edit to `scripts/license-line.conf`, six lines at this unit. Consumer: `check-license-line`, reading the source in process. `git grep -n 'license-line.conf'` over the tracked tree, `TASK-QUEUE.md` and this file excluded, names besides it the gate's descriptor and fixtures, the site-architecture page, the graph artifact, and the installer spec's packing paragraph with its mirror. That paragraph cites the location clauses alone, which no delta moves.
- **The manifest names knob.** Producer: the descriptor's `# knob:` lines, which every deployed configuration of this repository loads through the runner. Consumer: the coverage walk.
- **The field readers.** Each reads a file the manifests' own tools already read; the gate writes nothing. The value read is consumed by the comparison and printed in a finding; it has no other reader.
- **The verdict.** Consumer: the pre-commit hook and `--run`, by the gate's exit status, the same channel as today. The `precommit` tier is unchanged.
- **Roster-holding readers.** The gate's name, tier and class are unchanged, so `docs/enforcement.md`, `docs/value.md`'s rollup and `scripts/gates.list` are not moved. Its `couples=` widens, so the graph artifact is.
- **Red conditions.** No delta narrows a corpus. The coverage walk widens the gate's corpus, and its new red conditions are the three findings above, each over a member enumerated with its satisfying value.

## Existing sections updated

Roster produced by `git grep -n 'check-license-line\|license-line.conf'` over the tracked tree, excluding `TASK-QUEUE.md`, and by the fixture listing `ls -R scripts/gate-tests/check-license-line`.

- `native/Cargo.toml`, the `license` field (delta 1).
- docs/site-architecture.md §The license line, both paragraphs, replaced as above (delta 2).
- `native/src/gates/license_line.rs` — the `manifest` key in the parser and its admitted-key list, the readers, the comparison, the coverage walk, the findings, the `help:` block and the clean line, with crate unit tests for each reader and each new malformed-source refusal (delta 2).
- `native/src/gates/mod.rs`, the `check-license-line` registry row — a declared walk root `name:knob:GATE_LOCAL_LICENSE_MANIFEST_NAMES` beside the README root, and the new knob in the row's declared-knob list. The coverage walk is a walk the recorder observes, so an undeclared root fails the crate's declared-roots unit test (gate-sdk/SPEC.md §check-reads-couples). The row's existing `?` root cites `license_line.rs` by line number and moves with the code above it (delta 2).
- `scripts/check-license-line.gate` — the manifests joining `couples=`, the knob array's four `# knob:` lines, and the `# spec:` line naming the manifest half (delta 2).
- `scripts/license-line.conf` — the six `manifest` lines (delta 2).
- `scripts/gate-tests/check-license-line/good/` and `bad/` — each case's `.gate` copy carrying the new knob lines; `good/` a declared manifest of each kind holding the license; `bad/` a declared manifest whose field differs, one carrying no field, and an undeclared manifest carrying one, with `expect.txt` naming each finding (delta 2).
- `docs/check-graph.html`, regenerated for the widened `couples=` (delta 2).

## Retired spellings

- None — the deltas correct a field value and widen a gate's corpus; no name or path is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it; the merged section reads as one document a reader who never saw the amendment can use alone.
- [ ] **Oracles** — `bash gate-sdk/bin/build-native.sh`, the battery (`--run`) and the `scripts/` fixture suite green, the `bad/` case reddening on each of its three manifest findings.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — `crate-manifest-license-unheld` moved to `## Done` in the build commit that merges this amendment, a stage before the drain stage; no remote run is its oracle.
