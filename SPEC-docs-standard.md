# SPEC amendment: docs-standard

The front door and the install page repeat themselves, and the install page buries the per-OS path under a Requirements section that reads as a contributor's reference. Nothing states a page-level rule against the repetition or holds one. This amendment states the rule, gates its decidable half, rebuilds the two pages against it, and states each supported system's minimum OS version — the Linux one as a glibc floor the build holds the shipped artifact to.

Two queue entries pair it: [customer-docs-quality-standard](TASK-QUEUE.md#customer-docs-quality-standard) (deltas 1 to 8) and [glibc-floor-unstated](TASK-QUEUE.md#glibc-floor-unstated) (delta 9, with the Minimum cells delta 4 writes).

**The rulings.**

- **The standard is one rule with a gated half.** Within a page, a fact is stated once and a target linked once; across pages, a fact has one home. The within-page half has a syntactic slice with live instances, a relative link target or a long sentence stated twice, and a new canon-kit gate holds it. The across-page half is review's (delta 1 says why).
- **Requirements stays one section and moves above Install.** It is where an adopter answers *can I install here*, which comes before *how*. It keeps its heading, so every `docs/install.md §Requirements` citation in the tree stays true. Both of its marker blocks become tables, and the contributor-only tools leave the page for `CONTRIBUTING.md`.
- **The step-by-step recipes collapse, the one-liners and remedy blocks stay visible.** A `<details markdown="1">` block renders its markdown under the site's parser, and every reader of the recipes' marker blocks reads raw lines, so a collapsed block changes no reader.
- **The Linux floor is measured, not pinned.** The x86_64 Linux build leg keeps riding `ubuntu-latest` (native/runners.list's recorded ride). The shared build body measures each artifact's floor and refuses one that differs from the page's, so an image migration that raises the floor reds the first push that meets it. Pin-or-raise is decided there, with the evidence in hand.
- **The seam.**
  - Kit mechanism: `check-docs-page-repeat` and its two knobs, shipped by canon-kit and off by default, and gate-sdk/SPEC.md §Consumer payload's generic floor sentence.
  - This repository's own: the corpus binding, the page rule's text in docs/site-architecture.md, the two install-page gates and their table grammars, the build body's floor step, and every floor value.
  - No private rule content is involved.

**Refused.**

- **A page-grain link rule** (a page links another page once, whatever the fragment). 14 of the 25 corpus pages would red, most of them kit pages that cite several README sections downward. canon-kit/SPEC.md §check-docs-link-convention requires that anchored shape.
- **A cross-page sentence arm.** The corpus's cross-page verbatim repeats are sibling-page chrome, each kit page's pointer to its README, which the rule admits. The front-door copies are the summary tier it admits. The attested cross-page defect is paraphrase, which no scanner decides.
- **Closing each OS section into its own collapsed block.** It hides the headings that inbound links target (`install.md#macos-and-linux`, `#windows`), and once the recipes collapse the sections are short.
- **Pinning the Linux build image for a lower floor.** Pinning `ubuntu-22.04` would lower the floor, but it reverses the ride without a ruling, and the image retires on GitHub's clock. It is filed as a deferred gap instead, costed by the hosts 2.39 excludes.
- **Splitting macOS and Linux into two sections.** They share one one-liner and one recipe, so a split states each twice on one page — the defect this unit removes.

**Measured at authoring.**

- **The floors.** The v0.26.0 Release archives, downloaded with `gh release download v0.26.0 --pattern 'checkwright-gates-0.26.0-*'`, measure as follows. `readelf -V --wide` on both `*-unknown-linux-gnu` binaries lists `GLIBC_2.39` as the highest `libc.so.6` version needed, flags `none`. `pidfd_spawnp` and `pidfd_getpid` bind it; everything else is at 2.34 or below. `llvm-objdump --macho --private-headers` reads `minos 11.0` (`LC_BUILD_VERSION`) on `aarch64-apple-darwin` and `version 10.12` (`LC_VERSION_MIN_MACOSX`) on `x86_64-apple-darwin`. A local build on a glibc 2.43 host also measures `GLIBC_2.39`.
- **The runner tools.** actions/runner-images' `Ubuntu2404-Readme.md` lists `binutils` 2.42, and `macos-15-arm64-Readme.md` lists the Xcode Command Line Tools, which carry `otool`.
- **The render.** The site renderer (`SITE_KIT_RENDERER`'s default kramdown GFM program) renders a `<details markdown="1">` block holding a fence, a marker comment and a table as parsed markdown. `docs/_config.yml` sets no `parse_block_html`. `details` and `summary` are in check-docs-render-fidelity's known-element set (`native/src/gates/docs_render_fidelity.rs`).
- **The repeat census.** Scanning `docs/*.md` and `docs/*/index.md` (25 pages), with fences, HTML comments, front matter and table rows skipped, finds repeats on four pages only:
  - `docs/doctrine-kit/index.md`: `README.md#install` twice;
  - `docs/gate-sdk/index.md`: `README.md#quick-start` and `../install.md` twice each;
  - `docs/index.md`: `positioning.md`, `install.md#install`, `orchestration.md` and `kits.md` twice each;
  - `docs/install.md`: `installer/SPEC.md#requirements` twice, and three sentences twice.
- **The readers.** A read-only sweep of the tracked tree (`git grep` for each marker, heading and anchor, with each gate module read) finds three readers that break on a table: `check-install-platforms` and `check-install-toolchain`, which exit 2 on zero bullets, and the gates workflow's `native-artifacts-roster` step, which exits 1 and leaves no build legs. `check-md-refs` resolves the inbound anchors `install.md#install`, `#macos-and-linux` and `#windows`, and `index.md#the-kits`. check-install-claim scans `## Install` from its heading to the next H2–H6 heading (`CANON_KIT_INSTALL_SECTION_RE` is `^(Quick start|Install)`). The remedy, install and demo-proof extractors read marker lines alone, so heading order and a `<details>` wrapper leave them unchanged.

## What changes

### (1) The page-level standard {design-bearing}

**Not yet applied.** docs/site-architecture.md §Page-authoring rules gains a paragraph after its first:

> **One statement, one home.** A page states each fact, instruction and command once, and links each target once, at its first mention; a later mention is plain text or an in-page anchor. Across pages a fact has one home page, and every other page links it. A nav parent and its children divide a subject between them, so a child never restates its parent. The front door — `README.md` and `docs/index.md` — is the one summary tier. It may carry the product statement and the one-line install commands, and it links the page that explains them. `check-docs-page-repeat` holds the within-page half over the pages `CANON_KIT_PAGE_REPEAT_PAGES` names: a relative link target, or a sentence of eight words or more, stated twice on one page. The across-page half is review's. A cross-page restatement is usually a paraphrase, which no scanner decides, and the verbatim repeats across this site's pages are sibling-page chrome, which the rule admits.

### (2) The gate: `check-docs-page-repeat` {design-bearing}

**Not yet applied.** canon-kit/SPEC.md gains a section after §check-docs-restatement-parity:

> ### check-docs-page-repeat
>
> Invariant: no declared page states a relative link target, or a sentence of `CANON_KIT_PAGE_REPEAT_MIN_WORDS` words or more, twice.
>
> - **Corpus:** files matching `CANON_KIT_PAGE_REPEAT_PAGES`, an array of globs expanded like every canon-kit glob knob, default empty. An empty expansion is a clean `0 page(s)`.
> - **Prose only.** Fenced blocks, HTML comments, the front-matter block and table rows (a line whose first non-blank character is `|`) are skipped. A table is a record set, and a registry table links one target per row by design.
> - **Arm A — a repeated link.** The link extractor is §check-md-refs' bracket-then-paren match. A target with a scheme, a `mailto:` target and a pure `#anchor` are out of scope. The target is compared as written, path and fragment together, after a leading `./` is dropped, so two sections of one page are two targets.
> - **Arm B — a repeated sentence.** A prose line is split into sentences after a `.`, `!` or `?` followed by whitespace. A link reduces to its text and a list or quote marker is dropped. Case and runs of whitespace are folded. A sentence of at least `CANON_KIT_PAGE_REPEAT_MIN_WORDS` words (default `8`) that occurs again on the page is a repeat.
> - **Valve:** `page-repeat-exempt: <reason>` on the later occurrence's line or the one above, in the shared exempt window (§The shared spec adapters). The reason is mandatory.
>
> **Red** is one finding per later occurrence, naming the page, its line, the first occurrence's line and the repeated target or sentence. Each arm carries its own `help:` line. A link's is *link the first mention and make the later one plain text or an in-page anchor*. A sentence's is *state it once and point back to it*. The clean line counts pages, links and sentences. **Exit 2:** an unreadable page, or a `CANON_KIT_PAGE_REPEAT_MIN_WORDS` that is not a positive integer. `tier=precommit`, `install: zero-config`, armed by `CANON_KIT_PAGE_REPEAT_PAGES`.
>
> **Deliberately not asserted: a page-grain link rule, or a repeat across pages.** A page citing several sections of one README downward is the anchored shape §check-docs-link-convention requires, so the grain is the target, fragment included. A restatement across pages is usually a paraphrase, which no scanner decides.

canon-kit/SPEC.md §Layout and configuration gains the two knob rows:

> - `CANON_KIT_PAGE_REPEAT_PAGES` — array of globs, default empty: the pages `check-docs-page-repeat` holds.
> - `CANON_KIT_PAGE_REPEAT_MIN_WORDS` — default `8`: the shortest sentence `check-docs-page-repeat`'s arm B counts.

The implementation:

- `native/src/gates/docs_page_repeat.rs` holds the rule, and the gate table in `native/src/gates/mod.rs` registers it.
- `native/src/knobs/canon_kit.rs` declares the two knobs.
- `canon-kit/checks/check-docs-page-repeat.gate` carries `# graph: couples=knob:CANON_KIT_PAGE_REPEAT_PAGES dir=one valve=none tier=precommit`, `# install: zero-config`, `# armed-by: CANON_KIT_PAGE_REPEAT_PAGES` and the `# spec:` line.
- Its `good/`+`bad/` pair: `bad/` repeats a link and a sentence on one page. `good/` carries one target under two fragments, a repeat inside a fence, a table and a comment, a valved repeat, and a sentence one word under the threshold.
- The module's unit tests hold the sentence splitter, the fold, and both exit-2 paths.
- canon-kit/README.md's gate roster and canon-kit/smoke/install.sh register it.

`.workflow/release-declarations.md`'s Tightened gates section gains `` - `check-docs-page-repeat` `` with its one-line intent. It arms only where a consumer sets the corpus knob.

### (3) This tree binds the corpus and repairs the kit pages {mechanical}

**Not yet applied.**

- `scripts/canon-config.knobs` binds `CANON_KIT_PAGE_REPEAT_PAGES` to `docs/*.md` and `docs/*/index.md`. The two globs miss `docs/posts/` and the SPEC and README mirrors, whose links are their source's.
- `scripts/gates.list` registers `check-docs-page-repeat` beside `check-docs-restatement-parity`.
- `docs/doctrine-kit/index.md`'s second `README.md#install` link (its "Run this arm as the kit README spells it" line) becomes plain text.
- `docs/gate-sdk/index.md`'s second `README.md#quick-start` link (line 19) becomes plain text. So does its second `../install.md` link, the closing "or the install guide".

`docs/index.md` and `docs/install.md` are repaired by deltas 6 and 7. The battery with the gate registered is the oracle for all four pages.

### (4) The supported-systems table {design-bearing}

**Not yet applied.** docs/install.md §Requirements' `platforms:begin` block becomes a table, with one row per declared triple:

```text
| System | Minimum | Binary | Status |
|---|---|---|---|
| <gloss> | <floor> | `<triple>` | joined |
| <gloss> | <floor> | `<triple>` | held: <precondition> |
```

**The grammar.** A row is a block line whose first non-blank character is `|` and which carries a backticked run. The header and delimiter rows carry none. The triple is the row's first backticked run, and the state is its last cell, trimmed: `joined`, or `held:` followed by a non-empty precondition. The Minimum cell is the triple's OS floor, and an empty one is a finding: a support claim with no floor is half a claim. No cell may carry `|`. The rows, with each value measured or carried:

| Triple | System | Minimum | Source |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` | Linux on x86-64, and WSL | glibc 2.39 | measured, v0.26.0 |
| `aarch64-unknown-linux-gnu` | Linux on arm64 | glibc 2.39 | measured, v0.26.0 |
| `aarch64-apple-darwin` | macOS on Apple silicon | macOS 11 | measured, `minos 11.0` |
| `x86_64-apple-darwin` | macOS on Intel | macOS 10.12 | measured, `version 10.12` |
| `x86_64-pc-windows-msvc` | Windows on x86-64 | Windows 10 | carried: the page's existing "Windows 10 and later" |
| `aarch64-pc-windows-msvc` | Windows on ARM64 | Windows 10 | carried, as above |

All six rows are `joined`. The evidence narratives in today's bullets go: each triple's join predicate is `native/targets.list`'s header and gate-sdk/SPEC.md §Consumer payload's. The ARM64-Windows emulation note is already covered by installer/SPEC.md §The gate binary. The sentence above the table states the tested systems by pointing, not by listing: every joined system is built and installed on every push, on the runner images `native/runners.list` names (a self-repo blob link).

**The readers move with it.**

- `native/src/gates/install_platforms.rs`' `declarations()` reads rows by the grammar above. It adds the empty-Minimum finding, and its `help:` lines and unit tests spell the table.
- `.github/workflows/gates.yml`'s `native-artifacts-roster` step reads the same grammar in awk. The triple comes from the row's first backticked run, the state from its last `|`-delimited cell, trimmed.
- `scripts/gate-tests/check-install-platforms/` rewrites both cases' `install.md` as tables. `bad/` gains an empty Minimum cell, and its `expect.txt` gains that finding.
- docs/site-architecture.md's install-platforms parity row replaces its bullet-grammar sentences with this grammar. Its "four-way" binding stays four-way; the build body (delta 9) is a reader of the Minimum cell and is named there.
- `scripts/check-install-platforms.gate`'s `# spec:` line names the table.

### (5) The toolchain table, and contributor tools apart {design-bearing}

**Not yet applied.** docs/install.md §Requirements' `toolchain:begin` block becomes a table of the adopter's rows: `bash`, `git`, `curl` and `shellcheck`. A second `toolchain:begin` block in `CONTRIBUTING.md` holds the contributor's rows: `jq` and `cargo`.

```text
| Tool | Version | Needed by | Why |
|---|---|---|---|
| `<name>` | <version> | <audience> | <purpose> |
```

**The grammar.** A row is a block line opening with `|` whose first cell is one backticked name. The Version cell is `≥ <floor>`, an implementation token, both joined by `, `, or `—` for neither. The Needed-by cell renders the roster element's audience:

| Audience | Needed-by cell |
|---|---|
| none | `every profile` |
| `contributor` | `contributors` |
| `registered` | `registered gates` |
| a kit list | the kit names joined by `, ` |

A `derived` audience is resolved over the kit roots, as today, and compared as the kit set. The Why cell is prose and unread.

**The gate.** `native/src/gates/install_toolchain.rs` reads both blocks and holds whole-element parity between their union and `PROBE_SET`, both directions, as today. It adds a placement assertion: a `contributors` row only in `CONTRIBUTING.md`, every other row only on docs/install.md. Its positional form takes the two pages. `scripts/check-install-toolchain.gate` couples `CONTRIBUTING.md` and names it in its `# spec:` line. Both fixture cases and `scripts/gate-tests/check-install-toolchain.test.sh` spell the table, and one bespoke case places a contributor row on the install page.

**The prose.** Each Why cell is one clause.

- The `bash` row's clause names the construct that forces the floor, a nameref (`local -n`) in the gate library. That keeps installer/SPEC.md §Requirements' pin rule true.
- The rest of today's `bash` bullet moves to installer/SPEC.md §Requirements as grounds, beside *The audience token*, or is deleted where it is history. That rest is the derivation, the POSIX-sh hooks and bootstrap, `run-gates.ps1`, the Git for Windows bash for guard-kit, the GNU-construct declaration and the civil-date note.
- The `cargo` bullet's build-before-commit clause already sits in `CONTRIBUTING.md`'s *Build the gate binary* bullet, so the row points at it.
- *The audience token* bullet reads the Needed-by cell in place of the `@` token.

`CONTRIBUTING.md`'s *Battery-green in CI* bullet names the adopter toolchain on the install page plus the contributor table beside it. gate-sdk/README.md's toolchain-contract sentence names the table and installer/SPEC.md §Requirements, where the forcing constructs are.

### (6) The install page, rebuilt per system {design-bearing}

**Not yet applied.** docs/install.md takes this outline. The marker blocks' contents, the one-line install lines and the `Release channel:` line are unchanged unless named.

1. **Opening paragraph** — the vendoring and digest sentences. The footprint link stays.
2. **`## Requirements`**, moved above Install:
   - one sentence on where an install is possible, with the tested-systems pointer (delta 4);
   - the platforms table, then one legend sentence for `joined` and `held` and the refusal elsewhere;
   - the toolchain table (delta 5), introduced as the tools the gates run, by who needs them;
   - one closing paragraph: Ruby with `kramdown-parser-gfm` (and `liquid`) for site-kit's two parser gates, the `--emit env-probe` self-check, and a pointer to `CONTRIBUTING.md` for building Checkwright.
   - Today's paragraph restating the delivery-path tools goes, since each OS section below states its own. Its one transport-wide fact moves to §With Node, the one path no OS section covers: `init` verifies the binary with `sha256sum` or `shasum` and refuses without one.
3. **`## Install`**, carrying `<!-- install-primary: tarball -->` and one opening paragraph.
   - The paragraph says where to run it: *from your repository's root, in a new project or an existing one with every change committed* — `init` makes one commit and refuses a dirty worktree. It replaces *Start from a clean git repository*.
   - It says the one line downloads the newest Release tarball, verifies it and runs `init`, with a step-by-step form under each line, and that `npx checkwright init` does the same with Node.
   - The Release-tarball sentence precedes the `npx` one and the first `###`, since check-install-claim reads that span.
4. **`### Try it first`** — today's **Try it first.** paragraph, moved under its own heading, which docs/index.md and README.md link.
5. **`### macOS and Linux`**:
   - a **You need** line: git, `curl` and `tar`, plus `sha256sum` on Linux or the `shasum` macOS ships;
   - the macOS bash paragraph and the `macos-remedy` block;
   - the one-liner, with its arguments and uninstall sentence;
   - then `<details markdown="1">` with `<summary>Step by step</summary>`, holding the download fence, the `unix-install` block and the verify and uninstall notes.
6. **`### Windows`**:
   - a **You need** line: Git for Windows, with PowerShell, `Get-FileHash` and `tar.exe` shipping with Windows (the version is the table's);
   - the `windows-remedy` block;
   - the `irm … | iex` line, alone on its fence line, since the PowerShell witness reads it by `^irm (\S+) \| iex$`, with its arguments sentence;
   - the same collapsed **Step by step** holding the download fence and the `windows-install` block.
7. **`### With Node`**, which gains the hasher sentence above, and **`### Choosing a profile`**, unchanged.
8. **`## Managing`**, which gains the sentence the two remedy blocks each carry today, stated once: a remedy block changes your machine and not your repository, so `uninstall` leaves it in place.
9. **`## Upgrading`** and **`## Going further`**, unchanged.

**The page's repeats go.** The remedy sentence is stated once, in §Managing. The second *Then verify, extract and run `init`* sentence is rephrased. The *same pair on one run* sentence goes with the platform prose, and `installer/SPEC.md#requirements` is linked once.

**Two site-architecture rows move with it.** The remedy-blocks and install-blocks rows in docs/site-architecture.md §Generated projections and their freshness gates each gain a clause: the install block and its fetch fence sit inside the section's collapsed **Step by step**, and every leg reads raw lines, so the wrapper is invisible to them.

### (7) The front door's repeats {mechanical}

**Not yet applied.** docs/index.md:

- *Why that split is the whole design is the layer model on [Where Checkwright sits](positioning.md).* becomes *Why that split is the whole design: [Positioning](#positioning).*
- §Try it first's paragraph becomes one sentence: *`demo` runs the whole arc in a scratch repository of its own and removes it, without touching yours; what it does and what it needs first: [Try it first](install.md#try-it-first).*
- §Install's opener becomes *From your repository's root, in a new project or an existing one with every change committed, the same three routes without `demo` install the kits as one commit, the first two from the Release tarball:*
- §Install's closing line becomes *Per-system prerequisites, profiles and the other verbs: [Install and upgrade](install.md).*
- *agent [orchestration](orchestration.md)* in §What that buys you becomes plain text, since §Positioning links the page.
- §Start here drops its Install and Kit Reference items, which §Install and §The kits already link, and renumbers.

### (8) README.md's install opener {mechanical}

**Not yet applied.** README.md §Try it first's paragraph becomes *`demo` runs the whole arc in a scratch repository of its own and removes it, without touching yours; what it does and what it needs first: [docs/install.md](docs/install.md) §Try it first.* §Install's opener takes delta 7's wording.

### (9) The build holds each artifact to its declared floor {design-bearing}

**Not yet applied.** `scripts/ci-build-artifact.sh`, the one body both the release build leg and the CI producer run, gains a floor step after the copy and before the digest. A refused artifact therefore never gets a sidecar.

- It reads the row for `<target>` from docs/install.md's `platforms:begin` block, the row whose first backticked run is the target. No row is a refusal: a built target the page does not declare is the first bound broken.
- **`*-unknown-linux-gnu`.** The declared floor is the row's `glibc <X.Y>` token. The measured floor is the highest `GLIBC_<version>` the binary's version-needs section lists, read with `readelf -V --wide`.
- **`*-apple-darwin`.** The declared floor is the row's `macOS <X[.Y]>` token. The measured floor is `otool -l`'s `minos` under `LC_BUILD_VERSION`, or `version` under `LC_VERSION_MIN_MACOSX`.
- **Every other target** is declared only, and the step prints that it measured nothing.
- **Equal, or refuse.** The two are compared as numbers with trailing zero components dropped, so `11` equals `11.0`. A floor stated too high turns away hosts the artifact runs on, and one stated too low lets a loader failure through, so the check wants equality. The refusal names both values and the two remedies:
  - pin the target's runner in `native/runners.list` to an image with the older library;
  - or raise the row's Minimum cell, a support narrowing that the release declaration surface's Behavior changes declares.
- An absent floor token, or an absent `readelf` or `otool`, is a refusal naming what is missing. The step never passes unmeasured.
- The script's `# spec:` line for the step cites gate-sdk/SPEC.md §Consumer payload, like its others.

gate-sdk/SPEC.md §Consumer payload's *A roster line is a support commitment, so it is bounded twice* paragraph gains, after its second sentence:

> A platform's OS floor is a property of the built artifact rather than of the build image, so where the consumer's declaration states one, the shared build body measures each artifact against it and refuses a difference. Which floor a platform states, and where, is the consumer's.

installer/SPEC.md §Requirements' *Platform posture* bullet gains the floor's grounds in two sentences. The Linux floor is the glibc symbol set the artifact binds, measured by the build body against the platforms table. The x86_64 leg rides `ubuntu-latest`, so an image migration that raises it reds the push that meets it rather than shipping silently.

native/runners.list's ride paragraph gains one sentence: a ride that raises an artifact's OS floor reds the shared build body, which is where pin-or-raise is decided.

## Producers and consumers

- **The Minimum cell.**
  - Producer: a hand edit of the platforms table.
  - Consumers: `check-install-platforms`, which reds an empty cell at commit, and the shared build body, which reads the `glibc`/`macOS` token for its target on every CI producer run and every release build leg. Both paths are enabled on this tree: `native-artifacts` runs on every push, `publish` on every tag.
  - Red conditions: empty, at commit. At push or tag: absent for a measured target, or unequal to the measurement.
  - Each member's value: the six rows in delta 4.
- **The platforms table's row grammar.**
  - Consumers: `check-install-platforms` (triple, state, Minimum); the `native-artifacts-roster` step (triple and state, for the producer matrix and the keyed object the Intel and Windows legs read); the build body (triple and Minimum).
  - Red conditions: the gate exits 2 on a block with no row, and the roster step exits 1 on an empty leg list. Both are unchanged, and both are what a half-migrated grammar trips.
- **The toolchain tables.**
  - Producer: hand edits on two pages.
  - Consumer: `check-install-toolchain`, in parity with `PROBE_SET`, plus placement.
  - Red conditions: a missing, extra or unequal element; a row on the wrong page; no row in either block (exit 2, as today).
  - Each member's value: `bash` (≥ 4.3; context-kit, drift-kit, guard-kit, derived), `git` (every profile), `curl` (delegation-kit) and `shellcheck` (registered gates) on the install page; `jq` (contributors) and `cargo` (≥ 1.71, contributors) in `CONTRIBUTING.md`.
- **`CANON_KIT_PAGE_REPEAT_PAGES` and `CANON_KIT_PAGE_REPEAT_MIN_WORDS`.**
  - Producer: `scripts/canon-config.knobs` sets the first. The second rides its default.
  - Consumer: `check-docs-page-repeat` at every commit, since the gate is precommit-tier and registered.
  - Roster-holding readers of a minted knob name:
    - `check-knob-default-coupling` couples `CANON_KIT_PAGE_REPEAT_MIN_WORDS`' table default to the SPEC row delta 2 writes.
    - `check-knob-citation` bars stating either value outside canon-kit/SPEC.md.
    - `check-docs-cmd` (B) reds a backticked knob name that no kit code carries. So delta 1's text, which names `CANON_KIT_PAGE_REPEAT_PAGES`, lands in the commit that declares the knob, or after it.
- **`page-repeat-exempt:`**, a new comment directive. It is a full-line comment only where a valve rides alone above its line, so `check-comment-tier` owes it a row only if the build lands one on a governed surface. The corpus's four repaired pages need none.
- **The `### Try it first` heading.** Its anchor has two readers, docs/index.md (delta 7) and README.md (delta 8). `check-md-refs` resolves the first. The second is prose.

## Existing sections updated

Roster probes, each over the tracked tree: `git grep -n` for each of the seven marker names and for the anchors `install.md#` and `index.md#the-kits`; `git grep -l -F` for `(joined)`, `@contributor` and `@registered`; and the read-only reader sweep above, over `.github/workflows/`, `native/src/`, `scripts/`, `installer/` and every SPEC.

- `docs/site-architecture.md` — §Page-authoring rules (delta 1); the install-platforms parity row (deltas 4 and 9); the install-toolchain parity row (delta 5); the remedy-blocks and install-blocks rows (delta 6).
- `canon-kit/SPEC.md` — the new §check-docs-page-repeat and §Layout and configuration's two knob rows (delta 2).
- `native/src/gates/docs_page_repeat.rs`, `native/src/gates/mod.rs`, `native/src/knobs/canon_kit.rs`, `canon-kit/checks/check-docs-page-repeat.gate`, `canon-kit/gate-tests/check-docs-page-repeat/`, `canon-kit/README.md`, `canon-kit/smoke/install.sh`, `.workflow/release-declarations.md` (delta 2).
- `scripts/canon-config.knobs`, `scripts/gates.list`, `docs/doctrine-kit/index.md`, `docs/gate-sdk/index.md` (delta 3).
- `docs/install.md` (deltas 4, 5 and 6).
- `native/src/gates/install_platforms.rs`, `scripts/check-install-platforms.gate`, `scripts/gate-tests/check-install-platforms/bad/install.md`, `scripts/gate-tests/check-install-platforms/good/install.md`, the `bad/expect.txt` beside them, and `.github/workflows/gates.yml`'s `native-artifacts-roster` step (delta 4).
- `native/src/gates/install_toolchain.rs`, `scripts/check-install-toolchain.gate`, `scripts/gate-tests/check-install-toolchain/`, `scripts/gate-tests/check-install-toolchain.test.sh`, `CONTRIBUTING.md`, `gate-sdk/README.md` (delta 5).
- `installer/SPEC.md` — §Requirements' *The audience token* bullet and the grounds moved from the `bash` bullet (delta 5); its *Platform posture* bullet (delta 9).
- `docs/index.md` (delta 7).
- `README.md` (delta 8).
- `scripts/ci-build-artifact.sh`, `gate-sdk/SPEC.md` §Consumer payload, `native/runners.list` (delta 9).
- `docs/installer/SPEC.md`, `docs/canon-kit/SPEC.md`, `docs/canon-kit/README.md`, `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md` — the generated on-site mirrors, regenerated with `--emit docs-mirror --write` (deltas 2, 5 and 9).
- `docs/enforcement.md`, `docs/value.md`, `docs/check-graph.html`, `scripts/git-hooks/pre-commit`, `.workflow/surface-ceiling.txt` — the new-gate fan-out (docs/site-architecture.md §Generated projections and their freshness gates), each regenerated by the command its gate prints (delta 2).

## Retired spellings

- `(joined)` — the bullet join-state parenthetical, retired for the table's Status cell (delta 4).
- `@contributor` — the toolchain bullet's audience sigil, retired for the Needed-by cell's `contributors` (delta 5).
- `@registered` — as above, for `registered gates` (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment remains for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Rendered and run** — `check-docs-render-fidelity` green over the rebuilt page, the full battery green with `check-docs-page-repeat` registered, and both toolchain and platforms fixture suites green.
- [ ] **Read on the remote** — the push carrying deltas 4 and 9 is read green before either entry moves: the `native-artifacts-roster` step derives all six legs, and each of the four measured build legs prints its floor step equal. Both entries move to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), customer-docs-quality-standard on deltas 1 to 8 and glibc-floor-unstated on delta 9.
