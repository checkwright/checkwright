# SPEC amendment: toolkit-pages

Two landing pages have outgrown their structure. docs/install.md §Install leads with per-system sections and routes WSL nowhere a reader looking for it would see. §Windows is native-only, and WSL appears only in the platform table's first row and the sentence under it. docs/spec-toolkits.md serves two companions that diverge, an extension and a recipe with a lifecycle layer, and its OpenSpec section repeats the install page's macOS-and-Linux line while sending Windows readers to that page. The Spec Kit catalog listing needs a page of its own to land on. This amendment adds a route picker to the install page and splits the toolkit page into an overview and one page per toolkit, each of which points to the install page instead of repeating it.

One queue entry pairs it: [install-toolkit-page-structure](TASK-QUEUE.md#install-toolkit-page-structure).

**The rulings.**

- **A picker, not per-system pages.** Each system's section is about 55 lines. Requirements, profiles, managing and upgrading are shared by every system, and per-system pages would repeat them or scatter them. A list of anchors at the top of §Install is a picker without script, which the site's static render needs.
- **WSL is the Linux route, with one caution.** A WSL install places the Linux gate binary, and the generated pre-commit hook runs that binary (`scripts/checkwright-gates`, read from a probe consumer's hook). Git for Windows cannot start a Linux executable, so a checkout installed from WSL takes its commits from WSL. The picker says so where a WSL reader arrives.
- **Three toolkit pages, flat under Install.** The site nav renders two levels, a page and its children (`docs/_includes/nav.html`). The two toolkit pages therefore sit beside the overview as children of Install, rather than beneath it. The overview keeps its URL, which the README, the front page and the companion READMEs link.
- **The toolkit pages point, never restate.** Neither carries an install line of its own. OpenSpec's install is `checkwright init` with its arguments, `checkwright` standing for the reader's line on the install page, as [companion-recipe-in-payload](TASK-QUEUE.md#companion-recipe-in-payload)'s amendment writes it (its delta 6). Spec Kit's is the extension's install command, which runs that line.
- **The file names are `speckit.md` and `openspec.md`.** They match `companion/speckit/` and `companion/openspec/`. A `spec-kit` path segment is read as a kit root by `check-kit-ref-liveness`, the ground companion/SPEC.md §The component gives for the directory's name.
- **The seam.** Repo-root docs only. No kit surface changes.

## What changes

### (1) The route picker and the WSL route {mechanical}

**Not yet applied.** docs/install.md §Install, after its first paragraph and before §Try it first, gains:

> Pick your route:
>
> - **macOS or Linux:** [the one line and its steps](#macos-and-linux).
> - **Windows:** [natively, in PowerShell](#windows).
> - **Windows through WSL:** the [macOS and Linux](#macos-and-linux) line, in your WSL shell. The install places the Linux gate binary and the pre-commit hook runs it, so commit to that repository from WSL too: Git for Windows cannot start the Linux binary.
> - **With Node:** [`npx checkwright init`](#with-node).
> - **In Claude Code:** [the plugin marketplace](#from-a-plugin-marketplace).

§Windows' first sentence, *You need the tools …*, is preceded by *This is the native Windows route. Under WSL, take [macOS and Linux](#macos-and-linux).*

### (2) The overview and the two toolkit pages {design-bearing}

**Not yet applied.** `docs/spec-toolkits.md` keeps its front matter (`nav_parent: install`, `nav_child_order: 1`), its heading, §What the gates catch, §What is tested and §Limits. Its opening paragraph's last two sentences become *Each toolkit has a page: [Spec Kit](speckit.md), whose extension installs Checkwright with the recipe applied, and [OpenSpec](openspec.md), whose recipe you pass to `init`.* §Spec Kit, §OpenSpec and the lifecycle paragraph move to the new pages. §What is tested keeps its lifecycle sentence and links it to the OpenSpec page.

`docs/speckit.md` is new, with `title: Spec Kit`, `nav_parent: install` and `nav_child_order: 2`. Under the heading *Checkwright for Spec Kit*, it holds, in order:

- the moved §Spec Kit text: the extension's `specify extension add` fence, the install command paragraph and the recipe sentence;
- *The install command runs your system's line from the [install page](install.md#install), and that page's [Requirements](install.md#requirements) apply.*;
- *Spec Kit has no lifecycle layer ([companion/SPEC.md §The lifecycle layer](companion/SPEC.md#the-lifecycle-layer)).*;
- *What the gates catch, what is tested and the limits: [Spec Kit and OpenSpec](spec-toolkits.md).*

`docs/openspec.md` is new, with `title: OpenSpec`, `nav_parent: install` and `nav_child_order: 3`. Under the heading *Checkwright for OpenSpec*, it holds, in order:

- the moved §OpenSpec install sentence and its `companion-install` block;
- the recipe sentence;
- the **Keep each title unique within a spec** paragraph;
- the lifecycle paragraph and its `companion-full` block;
- the same closing pointer to the overview.

This delta's act depends on [companion-recipe-in-payload](TASK-QUEUE.md#companion-recipe-in-payload). If that unit landed in an earlier batch, the two marker blocks move from `docs/spec-toolkits.md` to `docs/openspec.md` unchanged. If it lands in this batch, its amendment's delta 6 writes them onto `docs/openspec.md` directly. This unit never lands before it, because the OpenSpec section it moves still carries the pasted recipe block until that unit retires it.

### (3) The readers of the marker blocks and the landing page {mechanical}

**Not yet applied.** Each reader that names the OpenSpec blocks' page names `docs/openspec.md`:

- `installer/consumer-smoke/run-smoke.sh`'s companion arm reads the OpenSpec `companion-install` and `companion-full` blocks from `docs/openspec.md`;
- installer/SPEC.md §The consumer smoke's companion-arm paragraph says *`docs/openspec.md`'s for OpenSpec* where it says *the landing page's for OpenSpec*;
- docs/site-architecture.md §Generated projections and their freshness gates, the **toolkit install lines** bullet, names `docs/openspec.md` as the page carrying the two blocks;
- companion/SPEC.md §Applying a recipe says *on the OpenSpec page, `docs/openspec.md`* where it says *on the landing page*.

The same two acts apply. If companion-recipe-in-payload landed earlier, each of these is an edit of the text it merged. If it lands in this batch, its amendment's deltas 5, 7 and 8 write `docs/openspec.md` in the first place.

companion/SPEC.md §The OpenSpec recipe's *and the landing page states the convention* becomes *and the OpenSpec page states the convention*. §The Spec Kit extension's `README.md` bullet, *where the landing page is*, stands, and `companion/speckit/README.md`'s **Learn more** first link becomes `[Checkwright for Spec Kit](https://checkwright.dev/speckit.html)`, keeping its description.

## Producers and consumers

Probe: `git grep -n "spec-toolkits\|landing page"` over the tracked tree; `docs/_includes/nav.html` read; a probe consumer's generated `scripts/git-hooks/pre-commit` and `scripts/gate-sdk-config.knobs` read for the binary the hook runs; `git grep -n "install.md#"`.

- **The new pages** (delta 2). Producer: the committing session. Consumers: the Pages build and the nav include, through `nav_parent`; `check-docs-nav-reachable`, which holds each page to a `title:` and to nav reachability, met by the nav slot; `check-md-refs` and `check-spec-pointer` over their links and citations; `check-surface-ratchet`, which reds a governed file with no row, so each new page gets its row in the commit that adds it.
- **The marker blocks' page** (delta 3). Reader: the companion arm, which reds by name on an absent or empty block, so a move that misses the arm reds the smoke.
- **The picker's anchors** (delta 1). Reader: `check-md-refs`, over same-file anchors that exist today.
- **Red conditions.** No reader loses its subject. `docs/spec-toolkits.md` shrinks, and its ceiling row stays above it, so the ratchet does not red. `docs/install.md` grows past its row, so the growing commit re-stamps it with `--emit always-loaded --ceiling`.

## Existing sections updated

Roster probe: the probes above, plus `grep -n "docs/" .workflow/surface-ceiling.txt`.

- `docs/install.md` — §Install, §Windows (delta 1).
- `docs/spec-toolkits.md` (delta 2).
- `docs/speckit.md` and `docs/openspec.md`, new (delta 2).
- `.workflow/surface-ceiling.txt` — rows for the two new pages and the grown install page, stamped with `--emit always-loaded --ceiling` (deltas 1 and 2).
- `installer/consumer-smoke/run-smoke.sh` (delta 3).
- `installer/SPEC.md` — §The consumer smoke (delta 3).
- `docs/site-architecture.md` — §Generated projections and their freshness gates (delta 3).
- `companion/SPEC.md` — §Applying a recipe, §The OpenSpec recipe (delta 3).
- `companion/speckit/README.md` (delta 3).
- `docs/installer/SPEC.md` — the generated on-site mirror, as are `docs/companion/SPEC.md` and `docs/companion/README.md`, each regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 3).

## Retired spellings

- None — the overview keeps its path and every moved passage keeps its words; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery and the installer consumer smoke green on the landing commit, and `check-docs-render-fidelity` green over the new pages. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
