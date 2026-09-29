# SPEC amendment: license-line

The license line is worded two ways and owned nowhere. The site footer in `docs/_layouts/default.html` reads *Licensed under Apache-2.0*, linked to `LICENSE`. Every README's `## License` section reads *Apache-2.0. The license text is …*, with one of four location endings. The site's README mirrors render that section on the same page as the footer, so a reader meets both phrasings at once, and the next README drifts unseen.

This amendment words every site in the footer's style, as one sentence whose location clause is its only variant. It declares each site's clause in one source file, and adds a repo-root gate holding every README's section and the footer to its rendering.

One queue entry pairs it: [license-line-owned-sentence](TASK-QUEUE.md#license-line-owned-sentence).

**The rulings.**

- **Two shapes, one sentence.** A site whose reader can follow a link takes the **linked** form, `Licensed under [Apache-2.0](<target>).`: the root README, with the relative `LICENSE`, and the footer, with the repository's blob URL. A README that travels without the repository takes the **located** form, ``Licensed under Apache-2.0, with the text in `LICENSE` <where>.``. The `<where>` clause is the one per-site variant, and it states where that copy's license text actually is.
- **Each clause states the true placement, and one README's changes.** The probe below reads each placement off its producer. `plugin/README.md` says its text is at the repository root in the source tree. But `plugin/LICENSE` is tracked beside it and held equal to the root's by `check-plugin-parity` assertion F, so its clause becomes *beside this file*. `installer/README.md` drops its GitHub link: the package carries the text beside it, and the located form says so.
- **A checker, not a generator.** The product statement's shape, a source rendered into marked sites by a `--write` arm, would put a marker pair into sixteen READMEs that ship to adopters, for a line that changes once. The gate prints each site's expected line on a red, so a fix is a paste. The source still holds the one statement, so the sites derive from it and are held to it (the Derivation-first rule).
- **Coverage is derived.** Every tracked `README.md` carrying a `## License` heading must be a declared site, so a new README's line is held from its first commit. The generated mirrors under `docs/` are excluded by their `generated: true` front matter, since they are the docs-mirror's output of a declared source.
- **The seam.** Repo-root only: a repo-root gate, its source file, the site architecture page and the installer's packer section. The kit READMEs change wording, which ships to adopters, but no kit mechanism changes, so no release declaration is owed.

**Sibling unit.** [homepage-license-duplicate](TASK-QUEUE.md#homepage-license-duplicate), a debt entry, deletes `docs/index.md`'s `## License` section, and widens this gate to catch it. The site chrome states the license on every page, so a hand-written page under the declared pages root that carries the heading repeats it. The gate reds that page, and the deletion lands in the widening's commit. This gate holds the license instance only. The general class, site chrome duplicating a page statement, is a gap filed apart.

## What changes

### (1) The source {mechanical}

**Applied.** `scripts/license-line.conf`, in the `key value…` line grammar of `scripts/product-statement.conf`, `#` comments and blank lines ignored. Its first line is `# contract: docs/site-architecture.md §The license line`.

- `license <id>` — the license's name, once: `license Apache-2.0`.
- `text <file>` — the license file's name, once: `text LICENSE`.
- `pages <dir>` — the root of the pages the site chrome renders on, at most once: `pages docs`.
- `where <name> <clause…>` — a named location clause, one per placement:
  - `where installed beside this file in an installed copy and at the repository root in the source tree`
  - `where beside beside this file`
  - `where root at the repository root`
  - `where package beside this file in the package`
- `site <path> link <target>` or `site <path> at <name>` — one per site, `<path>` repo-relative:

| site | form |
| --- | --- |
| `README.md` | `link LICENSE` |
| `docs/_layouts/default.html` | `link https://github.com/checkwright/checkwright/blob/master/LICENSE` |
| `installer/README.md` | `at package` |
| `plugin/README.md` | `at beside` |
| `companion/README.md` | `at root` |
| `companion/speckit/README.md` | `at installed` |
| each of the eleven kit roots' `README.md` | `at installed` |

### (2) The sites {mechanical}

**Applied.** Each `## License` section's body becomes its site's one rendered line (delta 3), and the footer's license line likewise. The footer already reads as its rendering, so only the sixteen READMEs change. The generated mirrors are regenerated after.

### (3) `check-license-line` {design-bearing}

**Applied.** A repo-root gate, born native: `native/src/gates/license_line.rs`, `scripts/check-license-line.gate` at `tier=precommit`, registered in `scripts/gates.list`.

- **Rendering.** A markdown site's linked form is `Licensed under [<license>](<target>).`. Its located form is ``Licensed under <license>, with the text in `<text>` <clause>.``. An `.html` site's linked form is `Licensed under <a href="<target>"><license></a>.`. An `.html` site declared `at` exits 2.
- **Reading a site.** In a markdown site, the lines of its `## License` section, blank lines dropped, must be exactly one line, equal to the rendering. In an `.html` site, exactly one line, trimmed, opens `Licensed under`, and it must equal the rendering.
- **Coverage.** Every `README.md` under `root`, walked with the gate-sdk prune set applied, whose text carries a `## License` heading, and whose front matter does not set `generated: true`, must be a declared site. Where the source declares `pages`, every other `.md` page under it, walked with the same prune set, whose front matter does not set `generated: true`, must carry no `## License` section.
- **Red**, one finding each: a site whose line differs, printing the expected line; a markdown site with no `## License` section, or one with more than one line; an `.html` site with no `Licensed under` line, or more than one; an undeclared README carrying the heading; a page under `pages` carrying it.
- **Exit 2**: an unreadable source or site; a missing or repeated `license` or `text`; a repeated `pages`; an unknown key; a `where` name declared twice or a `site` naming an undeclared one; a site declared twice; a `site` line whose form is neither `link` nor `at`.
- **Clean line**: `LICENSE-LINE: clean (<n> site(s) carry the license line; <m> README(s) declared; <k> page(s) carry none)`.
- **Positional form**: `check-license-line [source root]`, `root` the tree the coverage walk reads, so a fixture tree stands in for the repository.
- **Descriptor**: `couples=scripts/license-line.conf,README.md,*/README.md,*/*/README.md,docs/_layouts/default.html,docs/*.md,docs/*/*.md,native/src/gates/license_line.rs`, `dir=one valve=none`. Its `# spec:` line cites docs/site-architecture.md §The license line.

### (4) The owning section {mechanical}

**Applied.** docs/site-architecture.md gains `## The license line`, after §Page-authoring rules:

*Every README's `## License` section and the site footer carry one sentence: `Licensed under Apache-2.0`, then where that copy's license text is. A site a reader follows by link takes the linked form, the license name linked to the text. A README that travels without the repository takes the located form, which names `LICENSE` and says where it sits beside that copy. `scripts/license-line.conf` declares the license, each location clause and each site's form. `check-license-line` holds each site to its rendering and every README carrying the heading to a declaration, printing the expected line on a red. The mirrors are generated from their sources and are not sites. The footer states the license on every page, so a hand-written page under the source's `pages` root carries no License section of its own, and the gate reds one that does.*

### (5) The packer's placement names its readers {mechanical}

**Applied.** installer/SPEC.md §The packer, the paragraph *The license text is placed where it ships, and the tree keeps one copy*, gains a closing sentence: *Each packed README's license line states this placement, from `scripts/license-line.conf`'s location clauses, so a change to where the packer places the text changes those clauses (docs/site-architecture.md §The license line).*

### (6) The fixture pair {mechanical}

**Applied.** `scripts/gate-tests/check-license-line/`, each case a tree with a `license-line.conf` and an `args` naming it and the case root:

- `good/`: a root `README.md` in the linked form, a `kit/README.md` in the located form, a `default.html` footer, and a `docs/kit/README.md` carrying `generated: true` and a differently worded section, which the coverage walk skips; `pages docs`, with a `docs/page.md` carrying no section and a generated `docs/mirror.md` carrying one, which the page walk skips.
- `bad/`: a README whose line differs, a declared README with no `## License` section, an undeclared `other/README.md` carrying the heading, and, under `pages docs`, a hand-written `docs/index.md` carrying it. `expect.txt` holds the four findings and one expected-line print.

## Producers and consumers

Probe: `git grep -n -A2 "^## License" -- '*README.md' ':!docs'` lists sixteen source sections. `git grep -c "The license text is"` adds the fourteen generated mirrors under `docs/`, which is thirty lines. Placement was read off each producer. `installer/SPEC.md` §The packer places `LICENSE` in every packed kit root and at the package root. `installer/package.json`'s `files` lists `LICENSE`. `companion/SPEC.md` §Packing the extension adds it to the Spec Kit extension's archive. `git ls-files '*LICENSE*'` shows `plugin/LICENSE` tracked. `companion/README.md` ships in no package, since the payload carries `companion/<toolkit>/recipe/` alone. Each clause in delta 1 follows from these.

- **`scripts/license-line.conf`** (delta 1). Producer: this repository. Consumer: `check-license-line`, reading every key. `license` and `text` are read by every rendering, each `where` clause by the sites naming it, each `site` by the reading and coverage steps.
- **The rendered line** (deltas 2 and 3). Producer: each site's author, held by the gate. Consumers: the reader, on GitHub, npm, an installed copy or the site; `check-docs-mirror-fresh`, which holds each mirror to its source.
- **Coverage** (delta 3). Red condition: a README carrying the heading with no declaration. Members: every source README in the probe above, each with the form in delta 1's table.

## Existing sections updated

Roster probe: `git grep -n -i "license"` over `installer/SPEC.md`, `companion/SPEC.md`, `plugin/SPEC.md`, `gate-sdk/SPEC.md`, `docs/site-architecture.md`, `scripts/` less fixtures, `.github/` and `native/src/installer/`.

- `scripts/license-line.conf` (delta 1).
- `README.md`, `installer/README.md`, `plugin/README.md`, `companion/README.md`, `companion/speckit/README.md` and the eleven kit roots' `README.md` (delta 2).
- `native/src/gates/license_line.rs`, `native/src/gates/mod.rs`'s dispatch table, `scripts/check-license-line.gate`, `scripts/gates.list` (delta 3).
- `docs/site-architecture.md` — the new §The license line (delta 4).
- `installer/SPEC.md` — §The packer (delta 5).
- `scripts/gate-tests/check-license-line/` (delta 6).
- The new-gate fan-out docs/site-architecture.md §Generated projections and their freshness gates rosters: `docs/enforcement.md`, `docs/value.md`'s rollup block and `docs/check-graph.html`, each regenerated by the command its gate prints (delta 3).
- The generated mirrors under `docs/` of every changed README and of `installer/SPEC.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 2 and 5).
- `.workflow/surface-ceiling.txt` — any grown governed page re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling` in the growing commit (deltas 3 and 4).

## Retired spellings

- None — the deltas reword a sentence and add a source, a gate and a section; the old lead-in is prose rather than a name, path or token, and the gate's rendering check is what retires it.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Battery green** — the full battery, the root fixture suite and `cargo test` green on the landing commit, and the gates workflow green on the mid-iteration push, whose Windows and macOS legs compile the new module. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
