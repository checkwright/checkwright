# SPEC amendment: evaluator-discovery

Pairs `evaluator-agent-discovery`, narrowed by the set record to its surfaces (a), (b) and (d). An adopter is increasingly a coding agent that compares tools and recommends or selects one on a person's behalf, and the operator directed (2026-10-17, lead session, a direction, not a ruling) treating that agent as a first-class reader. Today it finds no surface written for it: no page says when to pick Checkwright and when not to, the site publishes no `llms.txt`, `installer/package.json` carries no `keywords`, and `native/Cargo.toml`'s `description` reads as an internal binary. This amendment adds a choosing page (a), a generated `llms.txt` and `llms-full.txt` behind a freshness gate (b), and registry metadata derived from the product statement (d).

**The seam.** Every surface here is repo-root-governed: `docs/` has no owning kit, and the two gates are consumer gates, each with its descriptor under `scripts/` and its rule in the crate. Their values are consumer config: the docs root through `SITE_KIT_DOCS_DIR`, the statement and keywords in `scripts/product-statement.conf`, and the site host in `docs/CNAME`. The arm and gate carry no literal of this site, no kit mechanism ships from this unit, and no private rule content crosses into a kit. The comparison on the choosing page rests on the private brief's research only for *which* projects it names. Every sentence it publishes about a project is a fact that project's own public README states, linked at the sentence.

**The choosing page is a page of its own, not a section of docs/positioning.md**, a decision of this stage that no operator direction carries. That page places Checkwright in the agent stack and contrasts design stances, and it refuses a per-tool comparison in its own words: its §The design-philosophy contrast draws the contrast at the level of stance, not feature. The choosing page's comparison is per project. A section there would carry a comparison the section above it refuses. The two pages also serve different readers. Positioning serves a reader already weighing the design, and the choosing page an agent deciding whether to recommend the tool at all. That agent quotes one page, and `llms.txt` links that page first.

**The refused alternative for (b): a hand-written `llms.txt`.** Every link it would carry is a page the nav already orders and titles, so a hand-written list is a second roster that drifts the day a page lands. Derivation-first makes it a projection of the nav model `check-docs-nav-reachable` already reads.

**The refused alternative for (d): hand-written keywords.** The keywords belong on the npm listing and on the crate reservation alike. Two hand copies are the drift the product statement exists to prevent, and the statement's gate already refuses a site whose field is absent, so the keywords join that source and the defect cannot recur unseen.

## What changes

### (1) The choosing page {design-bearing} {user-facing: evaluator-agent-discovery surface (a), operator direction 2026-10-17, lead-relayed: an evaluator-facing page, when to pick it and when not, a comparison against the same-seat entrants the private brief's SWOT names and a custom-CI baseline, the limits stated; the named entrants Chainloop and agent-attest beside the custom-CI baseline, operator direction 2026-10-17, lead-relayed, not a ruling}

A new page, `docs/choosing.md`. **Not yet applied.** Front matter: `title: Choosing`, `nav_order: 2` (delta 2), and `description: When Checkwright fits a repository where coding agents commit, when it does not, and how it compares with the alternatives.` H1: `Choosing Checkwright`. The page is written to page-authoring's rules (docs/page-authoring.md) and stays well under `CANON_KIT_PAGE_LENGTH_MAX_WORDS`. Each sentence below that a reader may quote stands alone, as docs/positioning.md §The bounded promise requires. Facts about Checkwright are linked to their home, never restated past one sentence.

Its sections, in order, and what each states:

1. **The opening paragraph**, no heading: one sentence naming the class of claim caught, in the bounded-promise form and linking docs/positioning.md §The bounded promise. A second sentence says the page is for a person or an agent deciding whether Checkwright fits.
2. **When it fits.** Coding agents commit to the repository, and the team wants a task marked done with no recorded test run, a skipped lifecycle stage, or a spec naming a path the tree lacks caught before merge. It wants checks that run at commit and in CI with no agent harness, with git the one tool the host must already have, which docs/requirements.md owns. It uses Spec Kit or OpenSpec and wants their artifacts held, which docs/spec-toolkits.md owns. It wants to try the tool and reverse it, which docs/install.md owns through `demo` and `uninstall`.
3. **When it does not fit.** The team needs code judged correct: the gates check that evidence exists and agrees with the tree, never what the code does, per docs/spec-toolkits.md §What the gates check against your code. It needs signed records of agent sessions, or a party outside the repository vouching for a run: Checkwright signs no record of a run, and evidence-kit's manifest is unsigned, the input such a party would verify. It wants the lifecycle workflow without the gates, which is not shipped (section 4).
4. **What you install.** The profiles as `installer/profiles.list` ships them, linking installer/SPEC.md §Profiles at its site mirror. `starter` is gate-sdk alone. `prose` adds canon-kit. `delegation` adds the kits whose subject is the agent session. `full` is every kit. The checks run without the workflow, since `starter` and `prose` carry no lifecycle-kit and the companion recipes gate another toolkit's workflow. The workflow does not run without the checks, since every profile carrying lifecycle-kit carries gate-sdk. Under `delegation`, lifecycle-kit's checks enforce nothing until the files each reads exist. A later ruling of `verify-workflow-decoupling` edits this section.
5. **Alternatives.** Three paragraphs, each stating what the alternative does, then where it fits better and where Checkwright does, with no score, table or count. The paragraphs about the two named projects carry one line saying they state each project as its README did on 2026-10-17, each paragraph linking the project's repository.
   - **Your own CI.** Pre-commit hooks, scripts the team writes, and required status checks can reproduce any single check here, for a team that writes and maintains it. Checkwright adds checks already written for the claims coding agents make. Each ships with a passing and a failing fixture and fails closed when it cannot run. It also adds the process state those checks read: the task queue, the stage stamps and the recorded test runs.
   - **Chainloop** (`github.com/chainloop-dev/chainloop`, Apache-2.0). It records coding-agent sessions, attesting them on `git push`, and CI outputs such as SBOMs, scans and test reports. Each record is a signed, timestamped in-toto attestation in the team's own storage, checked against Rego or WASM policies the team writes. A gating violation fails the push or the CI job. Its PR merge check is in the paid Platform. It fits where the requirement is signed, stored evidence of what agents and pipelines did, judged by the team's own policies. Checkwright fits where the claim to check is in the repository's own files, with no keys and no record store.
   - **agent-attest** (`github.com/zhengqiuyang/agent-attest`, MIT). It writes an ed25519-signed in-toto attestation per commit under `.agent-attest/`, recording the agent, a hash of the prompt, the run's timing and exit code, and the result of a verification command. It marks whether that command was executed or only claimed. Its `gate` fails a commit touching a configured protected path when the commit has no valid passing attestation. It runs on Node. It fits where the requirement is a signed chain of custody for agent commits on protected paths. Checkwright fits where the requirement is checks over the task, stage and spec state a claim rests on.
6. **Limits.** Links, one sentence each, to where each limit is owned: docs/positioning.md §The bounded promise, docs/spec-toolkits.md §Limits, and the observed-install record docs/install-evidence.md. One more sentence states that no check holds section 5's statements about other projects, which read as of the date the section gives.

### (2) The choosing page takes the second top-level nav slot {mechanical} {user-facing: as delta 1}

The page is top-level, after Home, so a first-time reader meets it before the methodology. Each top-level page from the current slot 2 down moves one slot. **Not yet applied.** Probed by `grep -H '^nav_order:' docs/*.md`:

| Page | `nav_order` today | Owed |
|---|---|---|
| `docs/index.md` | 1 | 1 |
| `docs/choosing.md` | (new) | 2 |
| `docs/methodology.md` | 2 | 3 |
| `docs/install.md` | 3 | 4 |
| `docs/spec-toolkits.md` | 4 | 5 |
| `docs/value.md` | 5 | 6 |
| `docs/kits.md` | 6 | 7 |
| `docs/releases.md` | 7 | 8 |

No child slot moves, since `nav_child_order` and `nav_parent` are untouched.

### (3) The site publishes `llms.txt` and `llms-full.txt`, projected from the nav {design-bearing} {user-facing: evaluator-agent-discovery surface (b), operator direction 2026-10-17, lead-relayed: `llms.txt` and `llms-full.txt` on the site, a generated projection behind a freshness gate}

A new non-gate arm, `--emit llms`, renders two files at the docs root, which the Pages build copies verbatim to the site root as it copies any file that is neither markdown nor templated. Without `--write` it prints `llms.txt`'s rendering. `--write` writes both files and writes neither until both render.

**Its inputs.** The docs root is `SITE_KIT_DOCS_DIR`. The site title is the `title:` line of `<root>/_config.yml`. The host is the first line of `<root>/CNAME`, and a page's URL is `https://<host>/` plus its path under the root as the Pages build serves it: `index.md` as its directory, any other `.md` page with `.html` for `.md`. The summary is the product statement's plain rendering, read from `GATE_LOCAL_PRODUCT_STATEMENT_CONF` through the product statement's own parser. The page set and its order are the nav model `check-docs-nav-reachable` reads: front matter as the front-matter span, `title:`, `nav_order`, `nav_id`, `nav_parent`, `nav_child_order`, `nav_children_key`, `nav_suffix` and the `generated:` marker. That model moves out of `native/src/gates/docs_nav_reachable.rs` into a module both readers call, so the menu and the projection cannot disagree on which pages hold a slot. A page's note is its `description:` front-matter value, one pair of enclosing quotes dropped.

**`llms.txt`.**

```text
# <site title>

> <summary>

## <title of the first top-level page>

- [<page title>](<url>): <note>
- [<child title>](<url>)

## Optional

- [<label>](<url>)
```

- **A section per top-level page**, in `nav_order`, `docs/index.md` excepted since the header stands for it. Its heading is the page's title. Its items are the page, then its nav children in `nav_child_order`. An item carries `: <note>` only where the page sets `description:`.
- **`## Optional`** holds, in this order: each item's generated directory siblings, in path order, and its `nav_suffix` pairs, in written order, the items taken in the order above. Each is labelled `<item title> <sibling title>` or `<item title> <pair label>`. Then each top-level page naming `nav_children_key`, which leaves the sections above, its derived children unlisted.
- **No other page appears.** A page off the menu carries no nav key, so it reaches neither part. No list of exclusions is read.

**`llms-full.txt`** carries the same header, then the body of `docs/index.md` and of every item outside `## Optional`, in `llms.txt`'s order, each preceded by a line `Source: <url>` and a blank line. A body is the page with its front matter dropped, every HTML comment outside a fence dropped, and every relative inline link target rewritten to its absolute URL. A target opening with a scheme stays. An `#anchor` target takes the page's own URL before it. Any other target resolves against the page's directory, a `.md` path by the URL rule above and any other path as `https://<host>/<path>`, its anchor kept.

**It refuses at exit 2**, writing nothing, on an unreadable input, a `_config.yml` with no `title:` line, an empty `CNAME`, a nav page with no `title:`, a link target resolving outside the docs root, or a page in `llms-full.txt`'s set that carries a Liquid tag (`{{` or `{%`), whose raw source would publish template syntax.

Replacement for docs/generated-projections.md §Generated projections and their freshness gates, a new bullet after the product statement's. **Not yet applied.**

> - **The agent index** <!-- projection: check-llms-fresh --> — `docs/llms.txt` and `docs/llms-full.txt`, served at the site root for a coding agent evaluating the tool. Both are projected from the nav model `check-docs-nav-reachable` reads, so a page joins them by taking a menu slot: `llms.txt` lists every menu page by section, its note a page's `description:`, and `llms-full.txt` carries the home page and every page outside its `## Optional` section in full. The summary is the product statement's plain rendering. `bash gate-sdk/bin/run-gates.sh --emit llms --write` writes both, and `check-llms-fresh` byte-gates them.

### (4) `check-llms-fresh` holds both files fresh {design-bearing}

A consumer gate, born native: `scripts/check-llms-fresh.gate`, its rule in `native/src/gates/llms_fresh.rs` calling the arm's rendering in process, and a `good/`+`bad/` pair under `scripts/gate-tests/check-llms-fresh/`. **Not yet applied.**

- **Assertion.** Each of `docs/llms.txt` and `docs/llms-full.txt` equals its rendering byte for byte. A stale file is one finding naming it, and the `help:` line prints the `--write` command above.
- **Exit 2** on every refusal the arm makes, and on an absent or unreadable projection file.
- **Its descriptor**, on the product-statement descriptor's model: `couples=` reaching every page the nav model reads, the mirrors included, `<root>/_config.yml`, `<root>/CNAME`, the product statement's source and both projection files; `# projection:` naming both files; `tier=precommit`; `# knob:` lines for `SITE_KIT_DOCS_DIR` and `GATE_LOCAL_PRODUCT_STATEMENT_CONF`. Its positional form is `check-llms-fresh [docs-root]`.
- **Its fixture pair** is a docs root of a few pages: a top-level page with a child, a child with a generated mirror sibling, a page naming `nav_children_key`, one `description:`, a `_config.yml`, a `CNAME`, and a product-statement source. `good/` carries both files fresh. `bad/` carries both stale and names each finding in `expect.txt`.
- **Registration** in `scripts/gates.list`, with the projections docs/projection-fan-outs.md §The new-gate fan-out lists.

### (5) The product statement gains keywords and the crate manifest as a sixth site {design-bearing} {user-facing: evaluator-agent-discovery surface (d), operator direction 2026-10-17, lead-relayed: registry metadata, `installer/package.json` carrying no `keywords` and `native/Cargo.toml`'s `description` reading as an internal binary; the keyword values `coding-agents ai-agents pre-commit ci spec-driven`, operator direction 2026-10-17, lead-relayed, not a ruling}

`scripts/product-statement.conf` gains a third key, and `native/Cargo.toml` becomes a site. **Not yet applied.**

- **The key.** `keywords <word>…`, exactly once, one to five words, each an ASCII letter followed by up to nineteen ASCII letters, digits, `-` or `_`. That is the narrower of the two registries' grammars, crates.io's, so one list serves both. A word outside it, or a sixth word, is a malformed source. The words: `coding-agents ai-agents pre-commit ci spec-driven`.
- **Its rendering.** A `.json` site carries a `"keywords"` field and a `.toml` site a `keywords` field, each on one line, an array of the words as quoted strings in source order, separated by `, `: `"keywords": ["coding-agents", "ai-agents", "pre-commit", "ci", "spec-driven"],`. A `.md` or `.yml` site carries none. The field is a second slot beside `description`, found and refused on the description slot's terms: exactly one per manifest site, an absent or repeated one exiting 2. `--write` rewrites it in place and inserts none, so build places the line in each manifest site first.
- **The sixth site.** `native/Cargo.toml` joins `GATE_LOCAL_PRODUCT_STATEMENT_SURFACES[]`, so its `description` takes the plain rendering and it carries the keywords. The crate reads no `CARGO_PKG_DESCRIPTION` (`git grep -n 'CARGO_PKG_DESCRIPTION' native/src`, no hit) and is never published, so the field is read by people and agents reading the manifest and by nothing else.

The manifest sites and what each carries after this delta, probed by the descriptor's `# knob:` lines:

| Site | `description` | `keywords` |
|---|---|---|
| `README.md` | the markdown block | none |
| `docs/index.md` | the markdown block | none |
| `docs/_config.yml` | plain | none |
| `installer/package.json` | plain | placed, by this delta |
| `reserve/crates/Cargo.toml` | plain | placed, by this delta |
| `native/Cargo.toml` | plain, by this delta | placed, by this delta |

Replacement for docs/generated-projections.md §Generated projections and their freshness gates, the product statement's bullet and its four sub-bullets. **Not yet applied.**

> - **The product statement** <!-- projection: check-product-statement-fresh --> — `scripts/product-statement.conf` is the one statement of what Checkwright is and the words it is found by. `bash gate-sdk/bin/run-gates.sh --emit product-statement --write` renders it into six sites. README.md's and docs/index.md's `product-statement` blocks take the markdown rendering. The plain rendering goes to the `description` field of `docs/_config.yml` (every page's default meta description), of `installer/package.json` (npm's listing, at the next release), of `reserve/crates/Cargo.toml` and of `native/Cargo.toml`. The three manifests also carry the keywords. `check-product-statement-fresh` holds all six. The repository's GitHub About and its topics are out of a gate's reach. They are set by hand, after the account-selection step in `OPS.local.md`. The `site-health` workflow's About arm compares the About text daily with `installer/package.json`'s description and files the site-health issue on a difference.
>   - **The source** holds three keys in the `key value…` grammar, `category`, `summary` and `keywords`, each exactly once and no other. A `category` or `summary` value may carry `*…*` emphasis and no other markdown. A backtick, a bracket, an angle bracket, a pipe or a control character is refused, so the plain rendering loses nothing. `keywords` takes one to five words, each an ASCII letter then up to nineteen letters, digits, `-` or `_`, crates.io's grammar and the narrower of the two registries'.
>   - **The renderings.** Markdown is `**<category>** <summary>`, written between the `<!-- product-statement:begin -->` and `<!-- product-statement:end -->` lines with a blank line inside each marker. Plain is `<category> <summary>` with every `*` dropped, written as the field's double-quoted string with `"` and `\` escaped. The keywords are one line, an array of quoted words in source order, written as a `.json` or `.toml` site's `keywords` field. A site's extension selects its renderings. `--write` rewrites each site's block or fields and touches nothing else, and it writes no site until every site has placed its own.
>   - **The gate** compares in process with the arm's renderings, one finding per stale slot. It exits 2 on an unreadable source or site, on a missing, repeated or unknown key, on a refused character or keyword, and when a site's block or field is absent or occurs twice. Its positional form is `check-product-statement-fresh [source site…]`.
>   - **Out of reach.** npm's listing trails the tree until the next publish, and crates.io carries the reservation's until it is republished, so no arm compares either.

## Producers and consumers

- **The choosing page.** Producer: build, by hand. Consumers: the Pages build and the nav include, by its `nav_order`; `llms.txt`, as the first section's item, by delta 3; the prose and docs gates whose page globs take `docs/*.md` (`scripts/canon-config.knobs`: the manifest, page-repeat, page-length and citation-link sets), so no glob needs a member added. Its `description:` is read by the layout's meta description (`docs/_layouts/default.html` line 9) and by delta 3's note. No other page sets one, so the layout's fallback to the site description holds for every other page.
- **The nav slots.** Producer: delta 2's front-matter edits. Consumers: the nav include and `check-docs-nav-reachable`, which reads `nav_order` as a slot's presence. Delta 3 reads the order, and the renumbering keeps it total and unique.
- **The nav model module.** Producer: the factoring in delta 3. Consumers: `check-docs-nav-reachable` and the llms arm, by call. Its fields are the nine front-matter facts above, each read by both, `description:` by the arm alone. The gate's verdict is unchanged by the move, its crate unit tests moving with the code they test.
- **`docs/llms.txt` and `docs/llms-full.txt`.** Producer: `--emit llms --write`, run by whoever edits an input, prompted by the gate's red. Consumers: an agent fetching the site root, which cannot run the emitter, so the files earn a roster row by docs/projection-fan-outs.md §Which derived surface earns a row; and `check-llms-fresh`, by byte comparison. `check-projection-roster` reads the gate's `# projection:` line and requires the keyed row delta 3 adds.
- **The `keywords` key.** Producer: a maintainer's edit to the source. Consumers: the product-statement arm and gate, in process; npm's and crates.io's search, at each package's next publish. The value has no other reader.
- **The sixth site.** Producer: the descriptor's knob line. Consumers: the arm and gate. The crate's build reads the manifest, and neither field changes what it builds.
- **Roster-holding readers.** A new gate joins `scripts/gates.list`, the crate's gate table in `native/src/gates/mod.rs`, and the generated `docs/enforcement.md`, `docs/value.md` rollup and `docs/check-graph.html`. A new arm joins the crate's arm table in `native/src/emit/mod.rs`. The new page joins `.workflow/surface-ceiling.txt` where `check-surface-ratchet` measures it. The product-statement gate's `couples=` and `# projection:` gain `native/Cargo.toml`.
- **Red conditions.** No delta narrows a corpus. Delta 5 widens the product-statement gate's site set by one member, its satisfying values tabled above.

## Existing sections updated

Rosters produced by `git grep -n 'product-statement\|product_statement'` and `git grep -n 'nav_order'` over the tracked tree, `TASK-QUEUE.md` and the on-site mirrors excluded, and by `grep -H '^nav_order:' docs/*.md`.

- `docs/choosing.md`, new (delta 1).
- `docs/methodology.md`, `docs/install.md`, `docs/spec-toolkits.md`, `docs/value.md`, `docs/kits.md`, `docs/releases.md` — the `nav_order` line, per the table (delta 2).
- `native/src/gates/docs_nav_reachable.rs` — the front-matter model and the children derivation moved into a shared module it calls, its unit tests moved with them (delta 3).
- `native/src/emit/llms.rs`, new, and its row in `native/src/emit/mod.rs`'s arm table (delta 3).
- `docs/llms.txt` and `docs/llms-full.txt`, new and generated (delta 3).
- docs/generated-projections.md §Generated projections and their freshness gates — the agent-index bullet (delta 3) and the product statement's bullet and sub-bullets (delta 5), replaced as above.
- `scripts/check-llms-fresh.gate`, `native/src/gates/llms_fresh.rs`, its row in `native/src/gates/mod.rs`, `scripts/gate-tests/check-llms-fresh/good/` and `bad/`, and `scripts/gates.list` (delta 4).
- The projections a new gate stales, per docs/projection-fan-outs.md §The new-gate fan-out: `docs/enforcement.md`, `docs/value.md`'s rollup block, `docs/check-graph.html` and `.workflow/surface-ceiling.txt` (delta 4).
- `scripts/product-statement.conf` — the `keywords` line, and its header comment naming the arm's six sites (delta 5).
- `native/src/emit/product_statement.rs` — the key, its grammar, the keywords slot and rendering, the `.toml` and `.json` sites carrying it, and the module's opening `spec:` comment, which names five sites (delta 5).
- `native/src/gates/product_statement_fresh.rs` — the second slot per manifest site (delta 5).
- `scripts/check-product-statement-fresh.gate` — `native/Cargo.toml` in `couples=`, in `# projection:` and as a sixth `GATE_LOCAL_PRODUCT_STATEMENT_SURFACES[]` line (delta 5).
- `scripts/gate-tests/check-product-statement-fresh/good/` and `bad/` — the `keywords` key in each source, the field in each `.json` and `.toml` site, and a stale keywords field in `bad/` (delta 5).
- `installer/package.json` and `reserve/crates/Cargo.toml` — the keywords line placed, then written by `--write` (delta 5).
- `native/Cargo.toml` — the keywords line placed, then it and the description written by `--write` (delta 5).

## Retired spellings

- `Multi-call binary carrying the gate implementations that have moved off the shell substrate` — `native/Cargo.toml`'s description, replaced by the product statement (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it; the merged section reads as one document a reader who never saw the amendment can use alone.
- [ ] **Oracles** — `bash gate-sdk/bin/build-native.sh`, the battery (`--run`), and the `scripts/` fixture suite green, every gate the shared nav module and the product-statement module implement being a `scripts/` gate; `check-llms-fresh`'s and `check-product-statement-fresh`'s `bad/` cases reddening on each finding they name.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — `evaluator-agent-discovery` moved to `## Done` in the build commit that merges this amendment, a stage before the drain stage. No remote run is its oracle; that the site serves both files is read at close's watched push and gates nothing.
