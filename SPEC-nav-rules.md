# SPEC amendment: nav-rules

The site nav decides reach page by page. A page may hold a menu entry, or be reached only through a relative-link walk that `check-docs-nav-reachable` seeds from the nav set (docs/site-architecture.md §Site chrome and the nav contract). Nothing sets nav labels beyond one rule, *title is the terse nav label, H1 the descriptive full form*. So the spec-toolkit overview sits beside its own two subpages as a sibling under Install, under a label its children repeat. And the Releases entry lists every release note as an open child, a list that grows with each release.

This amendment makes reach strict: every docs page holds its own menu entry, or is listed off-nav by design. It collapses the Releases children, gives the non-kit SPEC mirrors their menu entry through a new front-matter key, and makes the toolkit overview a top-level parent of its subpages. It also states the label rules and gates the overview-as-sibling rule.

Two queue entries pair it: [toolkit-nav-hierarchy](TASK-QUEUE.md#toolkit-nav-hierarchy) (deltas 1, 2 and 5) and [nav-strict-reachability](TASK-QUEUE.md#nav-strict-reachability) (deltas 3, 4 and 6). Delta 7 is shared.

**The rulings.**

- **Strict reach is feasible for every page, so the index-page fallback is not taken.** The operator's direction prefers every page its own menu entry and admits reach through an index page only where that proves infeasible. The probe below finds three pages with no menu entry once the link walk goes. Each gets one through delta 4's key, so no page needs the fallback.
- **The Releases children collapse behind a `details` element, closed by default and opened on a release note.** A reader on a note sees it marked current in an open list. Elsewhere the list stays one line. No JavaScript is added.
- **The non-kit SPEC mirrors take a suffix on the page that documents their component.** `installer/`, `companion/` and `plugin/` hold a SPEC mirror and no `index.md`, so the kit suffix rule, which hangs a mirror on its directory's nav child, cannot reach them. A `nav_suffix` key names each such page on the nav entry that documents it. Their README mirrors stay off-nav: `scripts/docs-offnav.list` already lists them as package front doors with no site reader, and an explicit key renders only the pages it names.
- **The overview-as-sibling rule is gated through a structural proxy.** An overview is a nav child that every sibling links and that links every sibling, under a parent with three or more children. Two children linking each other are a pair and not an overview, hence the floor. The proxy fires on the live defect and nowhere else on the tree (Producers and consumers). The other label rules stay review's, since a label's parallelism and an H1's agreement are judgments no scanner decides.
- **The seam.** Repo-root only: the docs chrome, the docs pages, a repo-root gate and its fixture pair. No kit surface changes.

**Sibling unit.** [nav-spec-suffix-restore](TASK-QUEUE.md#nav-spec-suffix-restore), a debt entry, drops the SPEC-title exclusion from the include and from the gate's model. Delta 6 depends on it: with the link walk gone, a kit SPEC mirror is reached only by the suffix. If the restore lands in the same batch, delta 6 lands in the same commit as the restore or after it, and the exclusion drop stays the restore's. If the restore is batched later, delta 6 waits for it: landed alone, the strict model reds all eleven kit SPEC mirrors.

## What changes

### (1) The toolkit overview becomes a top-level parent {mechanical}

**Applied.** Front matter only; no page body moves.

- `docs/spec-toolkits.md`: `title: Spec toolkits`, `nav_order: 4`, `nav_id: toolkits`. Its `nav_parent` and `nav_child_order` lines go. Its H1 becomes `# Spec toolkits: gating what Spec Kit and OpenSpec write`.
- `docs/speckit.md` and `docs/openspec.md`: `nav_parent: toolkits`, their `nav_child_order` renumbered to 1 and 2.
- The top-level slots after it move down one: `docs/value.md` to `nav_order: 5`, `docs/kits.md` to 6, `docs/releases.md` to 7. `docs/value.md`'s front matter sits outside its generated marker block, so the edit is by hand. `docs/install.md` keeps `nav_id: install` and has no children left.
- Link texts naming the old label read `Spec toolkits`: `docs/install.md` line 229, `docs/openspec.md` line 35, `docs/positioning.md` line 30, `docs/speckit.md` line 35 and `companion/README.md` line 20.

### (2) The label rules {mechanical}

**Applied.** docs/site-architecture.md §Page-authoring rules: its first paragraph's opening sentence is rewritten, and the page gains one paragraph after it:

*A page's `title:` is its nav label and its H1 the label's full form: the H1 names the label's subject, as* Install *and* Install and upgrade *do.*

***Nav labels.** Sibling labels are parallel in form: all nouns, or all a product's names. A child's label never repeats its parent's, since the parent already frames it. An overview is the parent of its subpages, never their sibling. `check-docs-nav-reachable` holds the last rule (§Site chrome and the nav contract). The other two are review's.*

### (3) The Releases children collapse {design-bearing}

**Not yet applied.** In `docs/_includes/nav.html`, a top-level entry naming `nav_children_key` renders its link and children inside one `<details>`: `<details><summary>` holding the entry's link, then the children's `<ul class="nav-children">`, then `</details>`. The element carries `open` when the current page carries the key, so a release note shows its list open with the note marked current. The Releases page and every other page show it closed. The `nav_id` branch is unchanged.

`docs/_layouts/default.html` gains the rules that keep the summary's link one line with its marker, beside the `.nav-tree` rules. Measure in a browser at desktop width and in a narrow window, closed and open. `check-docs-collapsible` reads `.md` pages only, so the include's element is outside its form rule.

docs/site-architecture.md §Site chrome and the nav contract, the sentence on `nav_children_key`, ends: *…labeled by the key's value (the release notes under the Releases page), inside a `details` element that is open only on a page carrying the key.*

### (4) The `nav_suffix` key {design-bearing}

**Not yet applied.** A nav page, top-level or child, may carry `nav_suffix: <label>=<path> …`: space-separated pairs, each `<path>` relative to the docs root. The include renders each pair as a suffix link labelled `<label>` to the page at `<path>`, in the `.nav-suffix` span the kit suffix already uses, after that entry's own suffix links. The layout's `.has-suffix` rules widen from `.nav-children` to `.nav-tree`, so a top-level entry carries a suffix too.

- `docs/install.md`: `nav_suffix: spec=installer/SPEC.md plugin=plugin/SPEC.md`.
- `docs/spec-toolkits.md`: `nav_suffix: spec=companion/SPEC.md`.

`scripts/docs-offnav.list`'s comment, on the sibling SPEC page, becomes: *the sibling SPEC page is NOT listed, being reached from the `nav_suffix` of the page documenting its component.*

docs/site-architecture.md §Site chrome and the nav contract gains, after the suffix sentence: *A nav page's `nav_suffix` names further pages its entry suffixes, as `<label>=<path>` pairs relative to the docs root: the SPEC mirror of a component with no kit page, on the page documenting that component.*

### (5) The overview-as-sibling arm {design-bearing}

**Applied.** `check-docs-nav-reachable` gains an arm. For each `nav_id` with three or more children, a child that every other child links and that links every other child is a finding: *`<page>`: an overview among its siblings under `<nav_id>`. Every sibling links it and it links each of them, so make it their parent with `nav_id`.* A link is the gate's own `links_of` scan, a relative `.md` target, anchors dropped.

### (6) Strict reachability {design-bearing}

**Not yet applied.** `check-docs-nav-reachable`'s reach becomes the **menu set** alone. That is every page holding a nav slot (`nav_order`, or `nav_parent` naming a top-level `nav_id`), every derived child, every generated sibling of a nav child's `index.md`, and every page a menu entry's `nav_suffix` names. The link walk is removed. A page outside the menu set and off the allowlist is a finding, as today. A `nav_suffix` pair that is malformed, lacking its `=`, or that names no docs page is a finding: *`<page>`: `nav_suffix` names `<path>`, which is not a docs page.* The `help:` lines drop *or link it from a nav page*, and name `nav_suffix` for a mirror page.

The clean line reads: *each carries a title block and holds a menu entry — a nav slot, a derived child, a suffix link or a `nav_suffix` pair — or is allowlisted off-nav.*

docs/site-architecture.md §Site chrome and the nav contract: *reachability from the rendered nav (a nav slot, a relative-link walk seeded from the nav set, or the generated-sibling suffix rule)* becomes *a menu entry of its own: a nav slot, a derived child, a generated-sibling suffix link, or a `nav_suffix` pair*. That sentence gains: *Reach through a page's body is not reach: a page off the menu is either given an entry or listed off-nav by design.* The `.gate` descriptor's `# spec:` line and the module's head comment are reworded to match.

### (7) The fixture pair {mechanical}

**Not yet applied.** `scripts/gate-tests/check-docs-nav-reachable/`:

- `good/`: `linked.md` moves to `bad/`. `good/` gains a top-level page carrying `nav_suffix` naming a generated page in a directory with no `index.md`. The SPEC-titled mirror is re-described as reached by the suffix, under the restore's change. `good/args`' comment is updated.
- `bad/`: gains `linked.md`, reached only by a link from a nav page, and a `nav_suffix` pair naming an absent page. It also gains a `nav_id` parent with three children, one of which every sibling links and which links each sibling. `bad/expect.txt` gains the finding texts of deltas 5 and 6, and `bad/args`' comment is updated.

## Producers and consumers

Probe: `git ls-files docs` less `docs/posts/`, with `grep -H -E '^(title|nav_order|nav_parent|nav_id|nav_child_order|nav_children_key|generated):'` over each page. Of 57 non-post pages, the menu set after deltas 1, 4 and 6 and the restore holds 51: 7 top-level entries, 18 children, 23 kit-suffix pages (eleven README and eleven SPEC mirrors and `docs/doctrine-kit/DOCTRINE.md`), and 3 `nav_suffix` pages. The other 6 are allowlisted in `scripts/docs-offnav.list`. Of 31 posts, 30 carry `release:` and are derived children, and the announcement post is allowlisted. The three pages the link walk alone reaches today are `docs/installer/SPEC.md`, `docs/companion/SPEC.md` and `docs/plugin/SPEC.md`.

Overview proxy probe: `grep -o -E '\]\([a-z-]+\.md'` over each parent's children, and `grep -c -o -E '\]\(\.\./[a-z-]+/'` over the kit indexes. Before delta 1, `docs/spec-toolkits.md` links both siblings and both link it, a finding. `docs/openspec.md` links both siblings, but `docs/speckit.md` does not link it, so it is clean. After delta 1, Spec toolkits and Value have two children each and are below the floor. Among Why Checkwright's three children, `docs/positioning.md` links only `docs/orchestration.md`, and no kit index links another. So the arm is clean.

- **`nav_suffix`** (delta 4). Producer: a nav page's front matter. Consumers: `docs/_includes/nav.html`, which renders the links, and `check-docs-nav-reachable`, which adds each named page to the menu set and reds a pair naming no page. `docs/search.json` reads `nav_parent` and `nav_id` alone and is untouched.
- **The collapsed children** (delta 3). Producer: the include. Consumer: the reader. No gate reads the element. The Pages build renders it, on the iteration's mid-iteration push.
- **The overview finding** (delta 5). Red condition: a child every sibling links and linking every sibling, under a parent of three or more children. Clean on the tree after delta 1, and red on it before.
- **Strict reach** (delta 6) narrows the reach set. The gate's red condition is a page outside the reach set and off the allowlist, a violation set that can only grow as reach narrows. Its members are the three `nav_suffix` pages, which delta 4 places, and the eleven kit SPEC mirrors, which the restore places. No reader counts the reach set, and the clean line counts pages, not reach.

## Existing sections updated

Roster probe: `git grep -n -i "link walk\|nav-reachable\|nav_reachable\|nav_order\|nav_parent: install\|spec-toolkits"` over the tracked tree, less `docs/posts/`, the generated mirrors and `docs/check-graph.html`.

- `docs/spec-toolkits.md`, `docs/speckit.md`, `docs/openspec.md`, `docs/positioning.md`, `docs/value.md`, `docs/kits.md`, `docs/releases.md`, `docs/install.md`, `companion/README.md` (deltas 1 and 4).
- `docs/site-architecture.md` — §Page-authoring rules (delta 2), §Site chrome and the nav contract (deltas 3, 4 and 6).
- `docs/_includes/nav.html`, `docs/_layouts/default.html` (deltas 3 and 4).
- `scripts/docs-offnav.list` (delta 4).
- `native/src/gates/docs_nav_reachable.rs`, `scripts/check-docs-nav-reachable.gate` (deltas 4, 5 and 6).
- `scripts/gate-tests/check-docs-nav-reachable/` (delta 7).
- `docs/companion/README.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 1).
- `.workflow/surface-ceiling.txt` — any grown `docs/` page re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling` in the growing commit (deltas 1, 2 and 4).

## Retired spellings

- None — the deltas re-point front matter, add a key, an element and a gate arm, and remove the link walk, which no surface names as a token; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it; the merged page reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Rendered** — the collapsed Releases list and both `nav_suffix` entries checked in a browser on the live site after the mid-iteration push, since only the Pages build runs the include; `check-docs-liquid-parse` holds its Liquid at commit.
- [ ] **Battery green** — the full battery, the root fixture suite and `cargo test` green on the landing commit, and the gates and Pages runs green on the mid-iteration push. Both entries move to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
