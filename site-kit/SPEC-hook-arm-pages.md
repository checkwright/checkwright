# SPEC amendment: hook-arm-pages

Pairs `hook-arm-adopter-page`. A consumer weighing one session-hook arm today reads a kit SPEC of 0.1 to 1 MB to learn what the arm reads, how it exits, which knobs it takes and what its settings entry is. An adopter told to check the release tarball's build attestation ran the verify command and saw nothing on stdout or stderr at exit 0, so could not tell what had been verified. This amendment adds one adopter-facing page per arm, `subagent-stop-liveness` first, with its knob table generated from the crate and its release status in its front matter, and gives the manual verify step a visible result. It settles `site-documents-unreleased` for these pages only, and that entry stays open for every other page.

**What the tree does today**, read from `docs/site-architecture.md`, `docs/page-authoring.md`, `docs/_layouts/default.html`, `docs/_includes/nav.html`, `docs/generated-projections.md`, `docs/manual-install.md`, `RELEASING.md` §The procedure, `native/src/hook/mod.rs` and the kit knob tables under `native/src/knobs/`:

- **No page documents one arm.** An arm's contract lives in its owning kit's SPEC: `subagent-stop-liveness` in delegation-kit/SPEC.md §The turn-end liveness hook and its wiring paragraph, with gate-sdk/SPEC.md §The harness-integration arm for the dispatch. No docs page carries a `hooks/` path.
- **The crate already names each arm's knobs.** Each `HOOKS` row carries the member's knob slice, the names its arm reads, which `hook::run` resolves before calling the member. Each name's shape and default sit in its owning kit's table (`native/src/knobs/`). No projection renders the slice.
- **No page carries a release status.** The layout reads `page.title`, `page.url` and `page.description` and no other key. Every page describes master, which `site-documents-unreleased` records as a gap.
- **The manual verify step prints nothing on success.** `docs/manual-install.md` runs `gh attestation verify "$cw/checkwright-$v.tgz" --repo checkwright/checkwright --source-ref "refs/tags/v$v"` in its POSIX block and the same in its PowerShell block, with no output flag. What a successful verify prints, and on which stream, is not probed here. The adopter's report is carried as reported.
- **A generated block inside a hand-written page has a pattern.** `docs/value.md` and `docs/releases.md` are hand-framed prose around one marker block an `--emit <arm> --write` regenerates and a `check-<name>-fresh` byte-gates (docs/generated-projections.md).

## What changes

### (1) The page per arm {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): one hand-written page per arm at `docs/hooks/<member>.md`}

**The path and the slot.** One hand-written page per arm, at `docs/hooks/<member>.md`, its file name the `HOOKS` member. It is a nav child of the Install page (`nav_parent: install`), since adopting one arm alone is an install path, and the Install page links it once. Install already has three children, so the overview rule binds: the arm page links Install, and Install links it (docs/site-architecture.md §Site chrome and the nav contract). **Not yet applied.**

**What the page answers**, in the order an evaluator asks. Each answer is stated in adopter terms and links the SPEC mirror section that owns it, so the page summarises and the SPEC stays the contract (docs/page-authoring.md, *Living pages*):

1. What the arm refuses, and why a session would want it refused.
2. What it reads: the payload keys it relies on, the launch records in its scratch dir, and the knob file.
3. How it exits, and where each notice lands: the refusal text, the decline notice, the stop log, and the observe log.
4. Its knobs: the generated block (delta 2).
5. Installing it alone: `init --user <member>`, the settings block it prints, and the `--observe` trial before enforcing (installer/SPEC-user-install.md, gate-sdk/SPEC-hook-observe.md). The manual path links docs/manual-install.md and its verify step (delta 4), and is not restated.
6. Removing it: `uninstall --user`, and the settings entry to delete.

**Which arms owe a page.** A page is owed for every member whose `HOOKS` tree-free column is true, the set an adopter can run on its own (gate-sdk/SPEC-hook-tree-free.md). Today that is `subagent-stop-liveness` alone. A page for a tree-bound member is allowed and not owed, and a widening of the column brings its page in the commit that widens it (delta 2's gate). **Not yet applied.**

**Grounds.** A page per arm, rather than one page for all arms, keeps each evaluator's read to the arm being weighed, and gives each a stable URL to link from an adopter's own runbook. Hand-writing the prose and generating only the knob table follows where drift lands. A knob's name and default change with the crate and are mechanically derivable. The prose describes behaviour the SPEC owns and no generator can summarise.

### (2) The generated knob block {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): a generated, freshness-gated knob block from the HOOKS knob slice}

**The arm.** gate-sdk gains `--emit hook-knobs`. For each page in the directory `GATE_SDK_HOOK_PAGES_DIR` names, it renders the block between `<!-- hook-knobs:begin -->` and `<!-- hook-knobs:end -->` from the member's `HOOKS` knob slice. It renders one row per knob in slice order, with four columns:

- the knob;
- its owning kit;
- its shape (scalar, indexed or keyed);
- its default, as the owning kit's table resolves it with no file layer. A derived default renders as the value it derives to, and an empty default renders as `(empty)`.

The block lists the member's own knob slice, so the dispatcher's `GATE_SDK_HOOK_OBSERVE_LOG` (gate-sdk/SPEC-hook-observe.md delta 2) is not a row. The page's prose names it and its default beside the `--observe` trial. `--write` rewrites each page's block and touches nothing else. Without `--write` the arm prints the blocks, which is the byte-fresh projection. **Not yet applied.**

**The gate.** `check-hook-knobs-fresh` (gate-sdk, born native: a Rust module, a `.gate` descriptor and a `good/`+`bad/` fixture pair) byte-gates each block against the arm's projection, printing the `--write` command on red. It also reds:

- a page whose file name is no `HOOKS` member;
- a page with no marker pair, or with more than one;
- a tree-free member with no page.

It exits 2 on an unreadable page or directory. **Not yet applied.**

**The knob.** `GATE_SDK_HOOK_PAGES_DIR` (default empty): the directory of arm pages, relative to the work-tree top. Empty turns the arm and the gate off, the gate printing its skip line. That is the off member of the choice: a consumer with no docs site has nothing to hold. This repo binds `docs/hooks` in `scripts/gate-sdk-config.knobs`. gate-sdk/SPEC.md §Layout and configuration gains the bullet. **Not yet applied.**

**Grounds for gate-sdk.** `HOOKS` and every kit's knob table are the crate's. The page directory is the consumer's configuration, the same split the value rollup's projection and its page take.

### (3) The `since:` key, its banner and its release rule {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): `since:` front matter holding `unreleased` or a version, rendered as a banner by the layout, rewritten by the release sweep, and a gate refusing a tag while any page says `unreleased`; this settles site-documents-unreleased for these pages only}

**The key.** A page under the governed set carries `since:` in its front matter, with the value `unreleased` or a release, `vX.Y.Z`: the first release whose binary does what the page says. A page landing with unreleased behaviour says `unreleased`. A page edited to describe new unreleased behaviour goes back to `unreleased`. **Not yet applied.**

**The banner.** `docs/_layouts/default.html` renders the key above the page's body. `unreleased` renders a notice that the page describes behaviour on master that no published release carries yet, linking the Releases page. A version renders a one-line note naming the first release that carries the behaviour. A page without the key renders neither. The notice's style sits in the site's stylesheet under `docs/assets/`. `check-docs-liquid-parse` holds the edit. **Not yet applied.**

**The rewrite.** The direction's *release sweep* is the release procedure's note step, RELEASING.md §The procedure step 1, not the release-sweep skill, which runs at a major release alone and would leave a minor release's pages saying `unreleased`. When a release note is authored, `bash gate-sdk/bin/run-gates.sh --emit docs-since --write` rewrites every `since: unreleased` on a governed page to the note's version, in the note-authoring commit. The note's version is read through the untagged-note reader `check-release-declaration-parity` arms on (gate-sdk/SPEC.md §upgrade-smoke). With no untagged note the arm refuses at exit 2, naming the missing note. RELEASING.md step 1 gains the sentence. **Not yet applied.**

**The gate.** `check-docs-since` (site-kit, born native) holds the governed set:

- every page carries `since:`, valued `unreleased` or `vX.Y.Z`;
- while an untagged release note exists, a page saying `unreleased` reds, printing the rewrite command.

The note is untagged from its authoring commit through the stamp commit the tag names (RELEASING.md step 4). So the tag lands on a tree where no governed page says `unreleased`, and a tag is refused where the battery is the authority over the commit it names. It exits 2 on an unreadable page, on a malformed front-matter span (read as canon-kit/SPEC.md §The shared spec adapters' front-matter span), and on two untagged notes. **Not yet applied.**

**The knob.** `SITE_KIT_SINCE_PAGES` (default empty): the governed set, as page globs relative to the docs root. Empty turns the gate off with its skip line. This repo binds `hooks/*.md`, the scope the direction settles. site-kit/SPEC.md §Layout and configuration and §Knob defaults gain the row. **Not yet applied.**

**Honest limits.**

- Whether a page's behaviour is in fact unreleased is the author's judgment. The gate holds the key's presence and its rule at the cut, not its truth.
- An iteration that closes on `release none` or a deferral keeps `unreleased` until a later note.
- The banner reaches the governed pages alone. Every other page still describes master, which `site-documents-unreleased` keeps open.

### (4) The verify step's visible result {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): the attestation-verify output as `--format json` plus jq, probed at build and marked inferred until then}

**The command.** `docs/manual-install.md`'s POSIX verify line gains `--format json`, piped through `jq` to print one line naming the verified subject's digest and the workflow that signed it. The PowerShell block reads the same JSON with `ConvertFrom-Json`, so a Windows adopter needs no `jq`. Each block keeps the step's paste-unit form (docs/page-authoring.md, *Code blocks*). A failed verify still exits non-zero, and the block stops there. **Not yet applied.**

**What is inferred until build.** That a successful verify without `--format json` prints nothing an adopter can read, and the JSON paths the filter reads, are both inferred. The build probes them against a published release's tarball before writing the filter. Where the probe finds the plain form already prints a readable result, the block keeps the plain form and the probe's output is shown as the expected result instead. **Not yet applied.**

**Requirements.** `docs/requirements.md`'s `gh` row is unchanged. A `jq` row joins it as optional, for reading the verify result on POSIX. **Not yet applied.**

### (5) Cases, registrations, the release declaration and the mirrors {mechanical}

Each **Not yet applied.**

- **The gates' fixture pairs**:
  - `check-hook-knobs-fresh`: `good/` a fresh block; `bad/` a stale row, a page naming no member, a missing marker pair, and a tree-free member with no page.
  - `check-docs-since`: `good/` an `unreleased` page with no untagged note and a versioned page with one; `bad/` an `unreleased` page beside an untagged note, a missing key, and a malformed value.
  - The rewrite arm's crate case, and its refusal with no untagged note.
- **The registrations**: both gates in `scripts/gates.list`; their knobs bound in this repo's knob files; the new page added to the `CANON_KIT_*_PAGES` lists that govern docs pages (page repeat, citation links, fence paste units, page length); and the knob block's roster entry in docs/generated-projections.md.
- **The page** `docs/hooks/subagent-stop-liveness.md`, with `since: unreleased`, and the Install page's link to it.
- **The release declaration** (`.workflow/release-declarations.md`), under `## New and tightened gates`: `check-hook-knobs-fresh` and `check-docs-since`, each off until its knob is set. Under `## Knob changes`: `GATE_SDK_HOOK_PAGES_DIR` and `SITE_KIT_SINCE_PAGES`.
- **The regenerated surfaces**: the mirrors `docs/gate-sdk/SPEC.md` and `docs/site-kit/SPEC.md`, and the agent index, each by the command docs/generated-projections.md rosters for it.

## Producers and consumers

- **The arm page** (delta 1). Producer: the session landing or changing an arm. Consumers: the adopter, the nav, and both gates.
- **The knob block** (delta 2). Producer: `--emit hook-knobs --write`. Consumers: the adopter, and `check-hook-knobs-fresh`.
- **`GATE_SDK_HOOK_PAGES_DIR`** (delta 2). Producer: the consumer's knob file. Consumers: the arm and the gate.
- **The `since:` key** (delta 3). Producers: the page author, and `--emit docs-since --write` at a release note. Consumers: the layout's banner, and `check-docs-since`.
- **`SITE_KIT_SINCE_PAGES`** (delta 3). Producer: the consumer's knob file. Consumers: the rewrite arm and the gate.
- **The verify block** (delta 4). Producer: `docs/manual-install.md`. Consumers: the adopter, and the arm page's link.

The page conventions were found in `docs/site-architecture.md` and `docs/page-authoring.md`, the knob slice by `grep -n "subagent-stop-liveness" -A8 native/src/hook/mod.rs`, the knob tables by `grep -rn "DELEGATION_KIT_STOP_LOG\|GATE_SDK_TMP_DIR" native/src/knobs`, the layout's keys by `grep -n "page\." docs/_layouts/default.html`, the verify lines by `grep -rn "attestation verify" --include=*.md docs` outside the mirrors and posts, and the untagged-note reader by `grep -n "untagged" gate-sdk/SPEC.md`.

## Existing sections updated

- `site-kit/SPEC.md` §Layout and configuration, §Knob defaults, and a new section `## check-docs-since` (delta 3).
- `gate-sdk/SPEC.md` §Layout and configuration, and a new section for `check-hook-knobs-fresh` beside the arm's other projections (delta 2).
- `docs/site-architecture.md` §Site chrome and the nav contract: the `since:` key and its banner (delta 3).
- `docs/generated-projections.md`: the knob block's roster entry (delta 2).
- `RELEASING.md` §The procedure step 1: the rewrite (delta 3).
- `docs/manual-install.md` and `docs/requirements.md` (delta 4); `docs/install.md`, linking the arm page (delta 1).
- `docs/_layouts/default.html` and the site stylesheet (delta 3).
- `native/src/` gains the two gate modules and the two emit arms, with their tests. `site-kit/checks/` and gate-sdk's `checks/` gain the `.gate` descriptors and fixture pairs (delta 5).

## Retired spellings

- None — every change adds a page, a key, a gate, an arm or a knob, and no existing name is renamed.

## Definition of Done

- [ ] **Causal completeness** — every delta's **Not yet applied.** is applied, and each user-facing mark's direction is visible in the merged section it lands in.
- [ ] **Instruction surfaces** — the arm page, the Install page's link, the manual verify step, and RELEASING.md's step 1 say what the merged sections say. Delta 4's inferred items are probed and either confirmed or corrected.
- [ ] **Merged with no information lost** — every delta's content sits in the section **Existing sections updated** names, including each honest limit.
- [ ] **Amendment deleted** — `ls site-kit/SPEC-*.md` lists no `SPEC-hook-arm-pages.md`.
- [ ] **Removals propagated** — none are owed: nothing is removed.
- [ ] **Gaps filed** — a page for each tree-bound member an adopter could weigh, and the `since:` key's widening to other pages, are filed with `--emit file-gap`, costed, or recorded under `site-documents-unreleased`, never flagged and skipped.
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery (`bash gate-sdk/bin/run-gates.sh --run`), the crate tests, the gate-sdk and site-kit fixture suites, and a local render of the site (the Jekyll invocation in the machine profile) showing the banner. No remote run observes the page before close. `pages-build-deployment` on the closing push serves it.
