# SPEC amendment: install-split

Paired with the queue entry `install-page-split`. The install page runs 321 lines and about 3,100 words, and no rule names a length at which a docs page splits: `check-surface-ratchet` stops a page growing and says nothing about a page that is already long. This amendment adds the length rule and its gate, splits the install page into a parent and three sub-pages, splits the site-architecture page into six, and shortens the two methodology essays the bound flags.

It spans the site (`docs/`), canon-kit (the gate), the installer's contracts (installer/SPEC.md) and this repository's local gates (`scripts/`, `native/src/gates`), so it sits at the root.

**The directions it rests on**, each given by the operator on 2026-10-05 through the lead session (a direction, not a ruling) unless marked otherwise:

- **D1** — the page is split into sub-pages, and a page-authoring rule, gate-held where possible, flags an over-long docs page (the queue entry).
- **D2** — the rule measures words, and this repository binds 1,500.
- **D3** — the site-architecture page is split in this unit.
- **D4** — the install page takes three children, `requirements.md`, `manual-install.md` and `maintenance.md`, and the parent keeps a Requirements pointer.
- **D5** — the site-architecture page becomes six pages by relocation: `site-architecture.md`, `page-authoring.md`, `generated-projections.md`, `projection-fan-outs.md`, `install-parity-contracts.md` and `install-block-contracts.md`.
- **D6** — the two essays over the bound are trimmed under it, with no new page.
- **L1** — the lead's decision, 2026-10-05: canon-kit owns the gate, on site-kit/SPEC.md §Out of scope, which gives the docs site's content to canon-kit.
- **L2** — the lead's decision, 2026-10-05: one queue entry and one amendment carry the whole unit.

**The measure used below.** Every word count in this amendment is whitespace-separated tokens on the lines after a page's front matter, comment-only lines dropped, taken over the tracked files `docs/*.md` and `docs/*/index.md` at `8a036ed84`. It is the measure delta 2 gives the gate, less multi-line comments, which no bound page carries in bulk.

**Landing order.** Delta 2 lands unregistered. Deltas 3 to 8 bring every page under the bound. Delta 9, the binding and the registration, lands last, since the gate reds from its first registered run until every page complies.

## What changes

### (1) The page-length rule {mechanical} {user-facing: D1 and D2 — a docs page past the bound is split or shortened; D6 — shortening is an admitted remedy}

A new bold-lead paragraph among the page-authoring rules, after **Code blocks**, on the page delta 6 moves those rules to. *Applied, on `docs/site-architecture.md`, where the rules sit until delta 6 moves them.*

> **Page length.** A page is short enough to read through. A page past `CANON_KIT_PAGE_LENGTH_MAX_WORDS` words is shortened, or split into a parent and sub-pages. The parent keeps the path every reader takes and links each sub-page once. A sub-page takes one subject a reader can skip: a reference table, a manual alternative, upkeep. A sub-page is a nav child of its parent where the parent is on the menu, and off-nav where the parent is. A generated page is left out, its length being its source's. `check-docs-page-length` holds the bound over the pages `CANON_KIT_PAGE_LENGTH_PAGES` names. Which subject moves to which sub-page is review's.

The bound's value stays in `scripts/canon-config.knobs` and the paragraph cites the knob.

### (2) check-docs-page-length {design-bearing} {user-facing: D1 — a gate holds the rule; D2 — it measures words; L1 — canon-kit ships it}

A new canon-kit gate, born native: `canon-kit/checks/check-docs-page-length.gate` (`precommit`, binary-dispatched, `install: zero-config`, `armed-by: CANON_KIT_PAGE_LENGTH_PAGES`), its rule in `native/src/gates/docs_page_length.rs`, with a `good/` and `bad/` fixture pair. A new section in canon-kit/SPEC.md, after §check-docs-page-repeat. *Applied, unregistered: `scripts/gates.list` carries its `# unregistered:` line until delta 9 registers it. The landed section adds that a comment marker inside a fenced block is the block's text.*

> ### check-docs-page-length
>
> Invariant: no declared page runs past `CANON_KIT_PAGE_LENGTH_MAX_WORDS` words.
>
> - **Corpus:** files matching `CANON_KIT_PAGE_LENGTH_PAGES`, an array of globs expanded like every canon-kit glob knob, default empty, less the files matching `CANON_KIT_PAGE_LENGTH_EXCLUDE`, an array of globs, default empty. An empty expansion is a clean `0 page(s)`.
> - **The measure** is the page's whitespace-separated tokens on every line after the front-matter block. HTML comments are skipped. Table rows and fenced blocks count, since a reader scrolls them.
> - **The bound** is `CANON_KIT_PAGE_LENGTH_MAX_WORDS`, a positive integer or `off`, default `off`. Which length is too long is the consumer's calibration. With the bound `off` the gate prints a clean line saying nothing was asserted.
> - **No valve.** A page over the bound is shortened or split. A generated page, whose length is its source's, is left out through `CANON_KIT_PAGE_LENGTH_EXCLUDE`.
>
> **Red** is one finding per page over the bound, naming the page, its count and the bound. The `help:` line is *shorten the page, or split it into a parent and sub-pages*. The clean line counts the pages and names the longest with its count, so the headroom is readable without a red. **Exit 2:** an unreadable page, or a bound that is neither a positive integer nor `off`. `tier=precommit`, `install: zero-config`, armed by `CANON_KIT_PAGE_LENGTH_PAGES`.
>
> **Honest limits.** A word count is not reading time (§check-prose-bounds). A split moves length and removes none, and what goes on which page is review's. A page whose words sit in comments passes.

The knob rows join canon-kit/SPEC.md §Layout and configuration after `CANON_KIT_PAGE_REPEAT_PAGES`. *Applied, reworded as one row under `check-prose-bounds`' repeated-phrase arm.*

> - `CANON_KIT_PAGE_LENGTH_PAGES` — the pages `check-docs-page-length` holds, an array of globs, empty by default. `CANON_KIT_PAGE_LENGTH_EXCLUDE` — an array of globs dropped from that set, empty by default. `CANON_KIT_PAGE_LENGTH_MAX_WORDS` — the bound, a positive integer or `off`, default `off`.

**The seam.** The gate, its three knobs and their empty or `off` defaults are kit mechanism. The pages, the excluded generated pages and the number 1,500 are this repository's config (delta 9). No private rule content is involved.

**Refused: a line bound.** The site writes one paragraph per line, so a line count reads a 6,400-word page of 121 lines as short. **Refused: a second arm on `check-prose-bounds`.** Its corpus is the governed specs and its unit is the sentence and the paragraph; a page bound over the site pages would need a second corpus knob inside one gate.

### (3) The install page becomes a parent and three sub-pages {design-bearing} {user-facing: D1 and D4 — three children, the Requirements pointer kept on the parent}

Every sub-page is a flat file under `docs/`, carrying `nav_parent: install` and a `nav_child_order`, so `docs/_includes/nav.html` lists it under Install with no chrome change. No heading text changes; a section moves whole, with its marker blocks.

*Applied, with four calibrations inside the envelope.* (a) A moved `###` section is a `##` section under its page's own H1. (b) Going further links `requirements.md#what-a-commit-costs`, `manual-install.md` and `maintenance.md`: the Requirements section already holds the bare `requirements.md` link, and `check-docs-page-repeat` reds a second. (c) `[Managing](#managing)` sat inside the macOS step-by-step region, so the re-pointed link is on `docs/manual-install.md`, which therefore links one sibling section; Requirements still links none, so no page is a hub. (d) The Manual steps page opens with one paragraph a region under its one-liner did not need: run the system's first block on the install page, pick a version, and where the tools are listed. The parent's route list expands WSL, since the expansion moved with the platforms table.

| Page | `title:` | H1 | `nav_child_order` | Takes |
|---|---|---|---|---|
| `docs/install.md` | Install | Install and upgrade | (parent, `nav_id: install`) | the opening paragraph; a `## Requirements` section of one sentence linking the Requirements page; `## Install` with its `install-primary` declaration and route list; Try it first; macOS and Linux and Windows, each down to and including its one-line install and the paragraph on passing arguments, with its `macos-remedy` or `windows-remedy` block; With Node; From a plugin marketplace; With another coding agent; Choosing a profile; Going further, which gains one link to each sub-page |
| `docs/requirements.md` | Requirements | What an install requires | 1 | everything under today's `## Requirements`: the platforms table under a new heading, *Supported systems*; Installing and running the shipped gates, with the `toolchain` and `prerequisites` blocks; What a commit costs, with the `commit-cost` block; Writing your own shell gates; Writing your own Rust gates |
| `docs/manual-install.md` | Manual steps | Installing step by step | 2 | the body of each **Step by step** region, under a *macOS and Linux* and a *Windows* heading: the fetch fence, the attestation fence, the marked `unix-install` or `windows-install` block, and the verify and uninstall lines. The `<details>` wrapper is dropped, since the page is the depth |
| `docs/maintenance.md` | Maintenance | Managing and upgrading an install | 3 | `## Managing` with Requiring the CI check, and `## Upgrading` with its `Release channel:` line |

Measured by line range on today's page: the parent about 1,230 words, Requirements about 1,100, Manual steps about 290 and Maintenance about 490.

- **Each OS section on the parent ends with one link** to its section of the Manual steps page, in place of the collapsed region.
- **Anchors that stay on the parent**, because a page outside this tree's reach links them: `#install`, `#try-it-first`, `#macos-and-linux`, `#windows`, `#from-a-plugin-marketplace`, `#choosing-a-profile` and `#requirements`. installer/README.md, which ships in every released payload, links three of them by URL; gate-sdk/README.md, vendored into adopter trees, links `#requirements`; one dated post links `#from-a-plugin-marketplace`.
- **Two in-page links on the parent change target.** `[Writing your own shell gates](#writing-your-own-shell-gates)` in the Windows section becomes a link to that section of the Requirements page, and `[Managing](#managing)` a link to the Maintenance page's section, since each fragment dangles once its section moves. `check-md-refs` reds each.
- **The agent prompt** under With another coding agent keeps its two `install.html` URLs, whose anchors stay.
- **No sub-page is a hub.** `check-docs-nav-reachable` reds a child that every sibling links and that links each of them. Requirements links no sibling; the other two link Requirements and not each other.
- **`check-install-claim`** scans any section whose heading opens *Install*. *Installing and running the shipped gates* is scanned today and moves with its body, so its first transport line is unchanged. No new heading under a `##` opens with that word; the H1s are not scanned.

**With `install-windows-stop-policy`.** That sibling edits the Windows fetch fence and the `windows-remedy` block. Where it has landed, which is the recorded order, this delta moves the fetch fence as edited. Where it has not, this delta moves the fence as it stands, and the sibling then edits it on `docs/manual-install.md` and the remedy block on `docs/install.md`.

### (4) The install gates read a page set {design-bearing}

One scalar knob, `GATE_LOCAL_INSTALL_PAGE`, names the page five local gates read: `check-install-toolchain`, `check-install-platforms`, `check-install-pin`, `check-release-channel-parity` and `check-release-declaration-parity`. After delta 3 their blocks sit on three pages. Two of the reads are at a release tag, by the same path:

- `check-install-platforms` arm G reads the page at the pinned tag through `pinned_release::show_at`, which refuses when the tag carries no such path. A knob repointed at a new page would exit 2 on every run until a release carrying that page is pinned.
- `check-release-declaration-parity` arm P reads a path absent at the previous tag as the empty table. Every declared triple would then derive as added in the next release note.

So the knob becomes an array, `GATE_LOCAL_INSTALL_DOCS[]`, declared on `scripts/check-install-pin.gate` with `docs/install.md`, `docs/requirements.md`, `docs/manual-install.md` and `docs/maintenance.md`, and `GATE_LOCAL_INSTALL_PAGE` is retired.

- **A tree read** takes every member's text. A block, or the `Release channel:` line, is read from the one member carrying it. A block no member carries is the exit 2 its absence is today, and a block two members carry is the exit 2 a repeated block is today.
- **A tag read** takes the members the tag carries. A tag carrying none refuses as arm G refuses today; a tag carrying some and no block leaves arm G's Minimum half dormant and arm P's base empty, as today. So the pinned release's page, `docs/install.md` alone at the current pin, still serves both arms, and the first release cut from the split tree serves them from `docs/requirements.md`.
- **One text for arm F.** `check-install-platforms` arm F holds the prerequisites rows against the platform families. Both blocks move to `docs/requirements.md` together.
- **`check-install-pin` invariant C** reads the members as one fetch surface, so the recipes on `docs/manual-install.md` stay held to the pinned release's asset names. Invariant D's exactly-one `commit-cost` block is counted across the members.
- **`check-install-toolchain`'s placement rule** names the member carrying the `toolchain` block as the home of a non-contributor row.
- **A finding names the member** the block or line was read from, never the set.
- **The positional forms are unchanged.** A positional page is one file and wins over the knob, so every fixture `args` file and bespoke test keeps passing one page.
- **Each of the five descriptors couples the four member paths literally**, as it coupled the literal page. *Applied so, against this delta's first wording, which coupled `knob:GATE_LOCAL_INSTALL_DOCS`: a `knob:` token names a kit's knob (gate-sdk/SPEC.md §The `# graph:` manifest, whose fail-closed rule refuses a name no static kit owns), and the crate test `the_couples_knob_sentinel_expands_to_the_static_names_the_corpus_carries` reds a descriptor-declared one. The asserted behaviour is unchanged: an edit to any member fires each gate.* The first wording, kept for the record of what was refused: each couples `knob:GATE_LOCAL_INSTALL_DOCS` in place of the literal page. `scripts/check-release-bump.gate` and `scripts/check-tightened-gates-grammar.gate` couple the page and read no install-page knob; a `knob:` token is admissible only where the gate's registry row in `native/src/gates/mod.rs` declares the knob (gate-sdk/SPEC.md §The `# graph:` manifest). Each therefore couples the four member paths literally, so an edit to any member still triggers it, and neither registry row gains the knob.

**Refused: one knob per block.** It leaves the two tag reads pointing at a path the pinned tag lacks, and needs a former-path knob to answer them. **Refused: leaving the gate-read blocks on `docs/install.md`.** They are most of the page's length.

*Applied.* The set read is `native/src/gates/install_docs.rs`, and its contract is installer/SPEC.md §The hosted install pin, *The install pages*. `native/src/gates/pinned_release.rs` is unedited: the tag read composes its `carries` and `show_at`.

### (5) The install page's other readers follow the blocks {mechanical}

*Applied.* Sites beyond this delta's roster, landed with it: `native/targets.list`'s comments naming the platforms page, `docs/install.sh` and `docs/install.ps1`'s header citations of the recipe's page, installer/README.md's two sentences placing the prerequisites and toolchain tables, and gate-sdk/SPEC.md's two mentions of the commit-cost figure's and the `toolchain` block's page.

- **`.github/workflows/gates.yml`** reads the page by literal path. The `unix-install` and `windows-install` extractions read `docs/manual-install.md`; the `native-artifacts-roster` step's platforms awk reads `docs/requirements.md`. The `macos-remedy` and `windows-remedy` extractions and the `irm … | iex` line read stay on `docs/install.md`. In the Windows step that extracts both a remedy and an install block, the block reader takes the page as a second argument. Each step's error and banner text names the page it reads.
- **`scripts/ci-build-artifact.sh`** sets `platforms_page` to `docs/requirements.md`. It also runs at publish, on the tag's own tree, so the path and the page move in one commit.
- **`GATE_LOCAL_FRONT_DOOR_SURFACES`** on `scripts/check-front-door-verbs.gate` gains `docs/manual-install.md` and `docs/maintenance.md`, which carry route and verb lines, and the descriptor's `couples=` gains both. A page left off is unscanned with no red.
- **Section citations** follow their content, and `check-spec-pointer` and `check-md-refs` resolve each:
  - a `docs/install.md §Requirements` citation, or a link to `install.md#requirements`, that means the platforms table, the toolchain or prerequisites tables or the bash floor cites the section of `docs/requirements.md` that holds it. The roster below names each site.
  - `docs/install.md §Upgrading` and `docs/install.md §Managing` become the same sections of `docs/maintenance.md`, and `install.md#requiring-the-ci-check` becomes `maintenance.md#requiring-the-ci-check`.
  - citations of `§Install`, `§With Node` and `§Windows` stand.
- **installer/SPEC.md** restates where things sit: §Requirements' *The install page's requirement blocks* passage (the three tables on the Requirements page, each remedy block on the install page, each install block on the Manual steps page, no collapsed region); §The release channel's home for the declaration line (the Maintenance page's Upgrading section); §The hosted install pin's third fetch surface and invariant D's page (the install pages `GATE_LOCAL_INSTALL_DOCS` names); §The front door's verbs' page roster, which gains the two sub-pages; §The upgrade contract's arm P, which names the new knob; and §The consumer smoke's sentences naming the page a leg reads.
- **RELEASING.md** step 4 writes the commit-cost figure into `docs/requirements.md`'s block. **`.claude/commands/close.md`**'s elapsed-time limb cites `docs/maintenance.md §Upgrading`.
- **`.workflow/surface-ceiling.txt`** gains a row per new page and a lowered row for `docs/install.md`, written by the command `check-surface-ratchet` prints, after the new files are staged.

Dated posts are immutable and are not edited: `docs/posts/2026-09-26-checkwright-v0-26-0.md` cites `docs/install.md §Requirements` and `docs/posts/2026-09-28-checkwright-v0-28-0.md` links `install.md#from-a-plugin-marketplace`, and both headings stay on the parent.

### (6) The site-architecture page becomes six off-nav pages {mechanical} {user-facing: D3 and D5 — six pages by relocation, under the names D5 lists}

Text moves; no contract sentence is rewritten except to name a page delta 3 or this delta moved. Each new page is a flat file under `docs/`, joins `scripts/docs-offnav.list`, and opens with a `title:` block and one sentence saying what it holds. A page carrying a regeneration command carries its own `door-contributor:` declaration before its first heading, as `docs/site-architecture.md` does (guard-kit/SPEC.md §check-door-binding).

| Page | Takes | About |
|---|---|---|
| `docs/site-architecture.md` | the opening paragraph, which gains one link to each page below; §Site chrome and the nav contract; §The license line | 750 words |
| `docs/page-authoring.md` | every paragraph of §Page-authoring rules, under that heading, plus delta 1's | 1,250 |
| `docs/generated-projections.md` | §Generated projections and their freshness gates, under that heading: its opening paragraph and every row keyed with a `projection:` comment (the SPEC mirror, the value rollup, the enforcement map, the trajectory projection, the install-evidence projection, the roadmap projection, the product statement, the toolkit support table, the graph artifact, the agent-definition tiers) | 1,220 |
| `docs/projection-fan-outs.md` | the KPI-roster, new-tag-class-member and new-gate fan-outs, each under a heading of its bold lead, and the three closing paragraphs on which derived surface earns a row, under *Which derived surface earns a row* | 910 |
| `docs/install-parity-contracts.md` | the install-toolchain and install-platforms parity contracts, each under a heading of its bold lead | 1,300 |
| `docs/install-block-contracts.md` | the prerequisites block, the commit-cost block, the remedy blocks, the install blocks, the toolkit install lines and the hosted install scripts, each under a heading of its bold lead | 1,170 |

- **The roster stays one section of one file.** `check-projection-roster` reads its keyed rows from the section `GATE_SDK_PROJECTION_ROSTER_SECTION` names in the file `GATE_SDK_PROJECTION_ROSTER` names. Every keyed row moves to `docs/generated-projections.md` under the unchanged heading, so `scripts/gate-sdk-config.knobs` repoints the file knob and leaves the section knob.
- **A moved row's heading is its bold lead** in sentence case, less the article where the lead opens with one only by grammar: *The install-platforms parity contract* stays as written. The body follows as paragraphs.
- **The contract rows name the install pages delta 3 made**: the three Requirements tables and the commit-cost block on `docs/requirements.md`, the remedy blocks on `docs/install.md`, the install blocks on `docs/manual-install.md` with no collapsed region to mention, and the knob delta 4 mints where a row names the page a gate reads.
- **The new-gate fan-out's closing sentence** points at the page holding the staging-order paragraphs, which is its own.

### (7) The site-architecture page's readers follow the sections {mechanical}

- **`docs/site-architecture.md §Generated projections and their freshness gates`** is cited by `# spec:` directives across `native/src`, the local gate descriptors, the two workflows and installer/SPEC.md. Each citation moves to the page and section now holding the row it cites: a projection's freshness gate to `docs/generated-projections.md` under the unchanged heading; the install-toolchain and install-platforms readers to their contract's section of `docs/install-parity-contracts.md`; a reader of a prerequisites, commit-cost, remedy, install or hosted-script contract to that section of `docs/install-block-contracts.md`.
- **`docs/site-architecture.md §Page-authoring rules`** becomes `docs/page-authoring.md §Page-authoring rules` at every site.
- **Citations of §Site chrome and the nav contract and §The license line stand.**
- **CLAUDE.md** names the roster's home as `docs/generated-projections.md` in §This repo is governed by its own kits. Its Housekeeping line on `docs/` stands: the load-triggered page still leads to every other.
- **A fixture descriptor** under a `gate-tests/` tree that copies a live descriptor's `# spec:` line follows its live descriptor.
- **`.workflow/surface-ceiling.txt`** gains a row per new page and a lowered row for `docs/site-architecture.md`.

`check-spec-pointer` resolves every re-pointed citation, and `check-citation-link` and `check-md-refs` every link. Neither tells a citation moved to the wrong row's section from one moved to the right one; the session reads each directive's gloss against the section it names.

### (8) The two essays come under the bound {design-bearing} {user-facing: D6 — both trimmed under the bound, no new page}

`docs/orchestration.md` runs 1,772 words and `docs/positioning.md` 1,675. Each is shortened below `CANON_KIT_PAGE_LENGTH_MAX_WORDS` by the three brevity moves the queue's brevity entries name (run-on structure, archaeology, restatement). Every heading stays, since `docs/install.md` links `positioning.md#running-under-an-agentsmd-harness` and both pages are linked by section elsewhere. No claim a gate or another page cites is dropped; a paragraph that restates what a linked page owns is cut to its link. Each page's `.workflow/surface-ceiling.txt` row is lowered in the same commit.

### (9) This repository's binding and the registration {mechanical} {user-facing: D2 — the bound is 1,500 words}

`scripts/canon-config.knobs` binds `CANON_KIT_PAGE_LENGTH_PAGES[]` to `docs/*.md` and `docs/*/index.md`, the hand-authored site pages `CANON_KIT_PAGE_REPEAT_PAGES` binds; `CANON_KIT_PAGE_LENGTH_EXCLUDE[]` to `docs/evidence-data.md`, `docs/enforcement.md`, `docs/footprint.md` and `docs/install-evidence.md`, the generated pages `CONTEXT_KIT_RATCHET_PATHS` already leaves out; and `CANON_KIT_PAGE_LENGTH_MAX_WORDS = 1500`. `scripts/gates.list` registers the gate.

The members over the bound at this binding, and each one's satisfying value. Probe: the measure stated at the head of this amendment, over the files the two globs match:

- `docs/site-architecture.md`, 6,453 words: delta 6's six pages, the longest about 1,300.
- `docs/install.md`, 3,119: delta 3's four pages, the longest about 1,230.
- `docs/orchestration.md`, 1,772, and `docs/positioning.md`, 1,675: delta 8's trim.
- `docs/evidence-data.md`, 5,502, and `docs/enforcement.md`, 1,661: generated, left out by the exclude knob.

Every other matched page is 717 words or fewer. `docs/page-authoring.md` and `docs/install-parity-contracts.md` land within 250 words of the bound, so the next rule or contract added to either meets it.

## Producers and consumers

- **The gate** (delta 2): produced by the battery and the generated pre-commit hook wherever `CANON_KIT_PAGE_LENGTH_PAGES` matches a file and the bound is set; delta 9 sets both, so a deployed configuration arms it. Its findings' readers are the committer and the battery report.
- **Its three knobs** (delta 2): read by the gate alone. Their roster reader is the knob registry, `native/src/knobs/canon_kit.rs`, whose rows `--emit knob-roster` prints and `check-knob-citation` resolves a citation against.
- **Every field has a reader:** a finding's page, count and bound are read by the committer; the clean line's page count and longest page by the battery report and by an author judging headroom.
- **Roster-holding readers of the new gate name**, each red when the name is missing: `scripts/gates.list`, the gate dispatch table in `native/src/gates/mod.rs`, canon-kit/README.md's gate roster, `canon-kit/smoke/install.sh`, `.workflow/release-declarations.md` (a new gate and three knobs are release-visible), and the generated projections a new gate moves, each regenerated by the command its freshness gate prints. The roster is the one the nearest precedent's landing touched, a page-scoped canon-kit gate with a glob knob: `git show --stat 1d25ac540`.
- **`GATE_LOCAL_INSTALL_DOCS`** (delta 4): produced by its `# knob:` lines on `scripts/check-install-pin.gate`, the one producer gate-sdk/SPEC.md §The declaration cohort allows a consumer knob. Its readers are the five gates delta 4 names, each named in its registry row's knob list in `native/src/gates/mod.rs`.
- **The sub-pages** (deltas 3 and 6): produced by hand. Each new flat `docs/<name>.md` is matched by every `docs/*.md` glob this repository binds, so `check-md-refs`, `check-docs-page-repeat`, `check-citation-link`, `check-fence-paste-unit`, `check-prose-tells`, `check-spec-pointer`, `check-install-claim` and `check-payload-claim` read it with no config edit, and `check-docs-render-fidelity`, `check-docs-liquid-parse`, `check-docs-collapsible` and `check-docs-nav-reachable` walk it under the docs dir. `check-surface-ratchet` reds a governed page with no ceiling row, which deltas 5 and 7 write.
- **Readers whose corpus narrows** (point 5), each named with its red condition:
  - `check-install-platforms`, `check-install-toolchain`, `check-install-pin` and `check-release-channel-parity` red or refuse on *finding none* of their block or line. Delta 4's set read finds each on its new page.
  - `check-install-pin` invariant C reds a fetch surface spelling no asset token. The set read keeps the recipes in the surface.
  - `check-install-claim` reds on zero or two `install-primary` declarations across the governed docs. The one declaration stays on `docs/install.md`.
  - Each install-smoke leg exits 1 by name on an empty extraction. Delta 5 points each at the page holding its block.
  - `scripts/ci-build-artifact.sh` refuses a target with no platforms row. Delta 5 points it at the table.
  - `check-projection-roster` reds a declaring gate with no keyed row in the roster section. Delta 6 moves every keyed row with the section and repoints the file knob.
  - `check-docs-nav-reachable` reds a page with no menu entry and no off-nav listing. Delta 3's children carry `nav_parent`; delta 6's pages join the off-nav list.
- **The remote oracle.** The install-smoke legs and the `native-artifacts-roster` step run only in CI, so the re-pointed extractions are first witnessed by the push the queue entry's push need names.

## Existing sections updated

- `docs/page-authoring.md` — new; the page-authoring rules and the **Page length** paragraph (deltas 1 and 6).
- `canon-kit/SPEC.md` — §check-docs-page-length, new, and §Layout and configuration's knob rows (delta 2).
- `canon-kit/checks/check-docs-page-length.gate`, `canon-kit/gate-tests/check-docs-page-length/`, `native/src/gates/docs_page_length.rs`, `native/src/knobs/canon_kit.rs` — new gate, pair, rule and knob rows (delta 2).
- `canon-kit/README.md`, `canon-kit/smoke/install.sh`, `.workflow/release-declarations.md` — the gate roster, the smoke roster and the release declaration (delta 2).
- `docs/install.md` — reduced to the parent (delta 3).
- `docs/requirements.md`, `docs/manual-install.md`, `docs/maintenance.md` — new (delta 3).
- `scripts/check-install-pin.gate` — the knob array, and the couple (delta 4).
- `scripts/check-install-platforms.gate` — the couple, and its `# spec:` line (deltas 4 and 7).
- `scripts/check-install-toolchain.gate` — the couple, and its `# spec:` line (deltas 4 and 7).
- `scripts/check-release-channel-parity.gate`, `scripts/check-release-declaration-parity.gate`, `scripts/check-release-bump.gate`, `scripts/check-tightened-gates-grammar.gate` — the couple (delta 4).
- `native/src/gates/install_pin.rs` — the set read, invariants C and D across members, and its `spec:` citations (deltas 4 and 7).
- `native/src/gates/install_platforms.rs` — the set read, arm G's tag read, and its `spec:` citations (deltas 4 and 7).
- `native/src/gates/install_toolchain.rs` — the set read, the placement rule's home, and its `spec:` citations (deltas 4 and 7).
- `native/src/gates/release_channel_parity.rs` — the set read (delta 4).
- `native/src/gates/release_declaration_parity.rs` — arm P's tree and tag reads (delta 4).
- `native/src/gates/mod.rs` — the dispatch row for the new gate, each registry knob list naming the retired knob, and its `spec:` citations (deltas 2, 4 and 7).
- `native/src/gates/pinned_release.rs` — a tag read over the members a tag carries (delta 4).
- `.github/workflows/gates.yml` — the re-pointed extractions and their messages, and its `# spec:` citations (deltas 5 and 7).
- `.github/workflows/site-health.yml` — its `# spec:` citation (delta 7).
- `scripts/ci-build-artifact.sh` — `platforms_page`, and its `# spec:` citation (deltas 5 and 7).
- `scripts/check-front-door-verbs.gate` — two surfaces and their couples (delta 5).
- `installer/SPEC.md` — the passages delta 5 lists, the retired knob's name in §The upgrade contract, and its citations of the projections section (deltas 4, 5 and 7).
- `docs/installer/SPEC.md` — the generated mirror of installer/SPEC.md, regenerated (deltas 4, 5 and 7).
- `companion/SPEC.md` — its citations of the Requirements and Managing sections (delta 5).
- `docs/companion/SPEC.md` — the generated mirror of companion/SPEC.md, regenerated (delta 5).
- `.claude/commands/close.md` — the Upgrading citation (delta 5).
- `gate-sdk/SPEC.md`, `site-kit/SPEC.md`, `gate-sdk/README.md`, `CONTRIBUTING.md`, `RELEASING.md`, `installer/profiles.list`, `scripts/ci-macos-floor.sh`, `docs/speckit.md`, `docs/spec-toolkits.md` — each citation of the install page's Requirements content or CI-check section, and RELEASING.md's commit-cost step (delta 5).
- `installer/README.md` — the sentence placing the tarball recipe below the one line, which names the Manual steps page (delta 5).
- `docs/site-architecture.md` — reduced to the chrome, the license line and the index (delta 6).
- `docs/generated-projections.md`, `docs/projection-fan-outs.md`, `docs/install-parity-contracts.md`, `docs/install-block-contracts.md` — new (delta 6).
- `scripts/docs-offnav.list` — five pages (delta 6).
- `scripts/gate-sdk-config.knobs` — `GATE_SDK_PROJECTION_ROSTER`, and its `# spec:` gloss (delta 6).
- `native/src/emit/mod.rs` — `spec:` citations of the projections section (delta 7).
- `native/src/emit/product_statement.rs` — the same (delta 7).
- `native/src/emit/value_rollup.rs` — the same (delta 7).
- `native/src/gates/docs_nav_reachable.rs` — `spec:` citations of the page-authoring rules (delta 7).
- `native/src/gates/install_evidence_fresh.rs` — `spec:` citations of the projections section (delta 7).
- `native/src/gates/product_statement_fresh.rs` — the same (delta 7).
- `native/src/gates/trajectory_fresh.rs` — the same (delta 7).
- `native/src/gates/value_rollup_fresh.rs` — the same (delta 7).
- `scripts/check-install-evidence-fresh.gate` — its `# spec:` citation of the projections section (delta 7).
- `scripts/check-product-statement-fresh.gate` — the same (delta 7).
- `scripts/check-trajectory-fresh.gate` — the same (delta 7).
- `scripts/check-value-rollup-fresh.gate` — the same (delta 7).
- `scripts/product-statement.conf` — its header citation of the projections section (delta 7).
- `scripts/canon-config.knobs` — its citations of the page-authoring rules, and the three knob lines (deltas 7 and 9).
- `CLAUDE.md` — the roster's home (delta 7).
- `docs/orchestration.md`, `docs/positioning.md` — shortened (delta 8).
- `scripts/gates.list` — the registration (delta 9).
- `.workflow/surface-ceiling.txt` — a row per new page and each lowered row (deltas 5, 7 and 8).
- `scripts/git-hooks/pre-commit`, `docs/enforcement.md`, `docs/check-graph.html`, `docs/value.md`, `docs/canon-kit/SPEC.md`, `docs/canon-kit/README.md` and every other on-site mirror of a SPEC or README named above — regenerated by the command each freshness gate prints (all deltas).

The roster is the union of three probes at `8a036ed84`: `git grep -l` for the five retired spellings below over the tracked tree; `git grep -n -o` for section citations and anchors into `docs/install.md`; and the precedent landing's `git show --stat 1d25ac540`.

## Retired spellings

- `GATE_LOCAL_INSTALL_PAGE` — replaced by the array knob `GATE_LOCAL_INSTALL_DOCS` (delta 4).
- `docs/site-architecture.md §Generated projections and their freshness gates` — the section moves to `docs/generated-projections.md`, and its contract rows to the two contract pages (delta 7).
- `docs/site-architecture.md §Page-authoring rules` — the section moves to `docs/page-authoring.md` (delta 7).
- `docs/install.md §Upgrading` — the section moves to `docs/maintenance.md` (delta 5).
- `docs/install.md §Managing` — the section moves to `docs/maintenance.md` (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the gate, its three knobs, the page-set knob and each moved block's reader.
- [ ] **Instruction surfaces: instruction only** — `.claude/commands/close.md` and CLAUDE.md each change one pointer and gain no grounds.
- [ ] **Merged with no information lost** — every sentence of the two split pages is on exactly one of the pages that replace them, and the new SPEC section and rule paragraph read as one document with their neighbours.
- [ ] **Amendment deleted** — this file removed on merge; no root amendment of this unit remains.
- [ ] **Removals propagated** — the five declared spellings survive nowhere `check-amendment-retired-spelling` reads.
- [ ] **Gaps filed** — a citation whose row has no evident new section, and any page the gate reds beyond delta 9's roster, is fixed in the batch or filed.
- [ ] **Rendered** — a local Jekyll build shows the three install children under Install in the menu, and each new page rendering its tables and fences.
- [ ] **Built and run** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, and the fixture suites of canon-kit and of this repository's `scripts/` gates.
- [ ] **Entry moved** — `--queue done install-page-split` a stage before the drain stage, once the install-smoke legs and the `native-artifacts-roster` step of the push carrying the re-pointed extractions are green; until then the entry bridges with a `[spec:]` path ref to canon-kit/SPEC.md. *The push is spent and read green, 2026-10-05: every install-smoke leg ran the block from the page now holding it, and the `native-artifacts-roster` step and the artifact floor step read the platforms table on `docs/requirements.md`. The Done move waits on deltas 6 to 9 alone.*
