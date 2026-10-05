# SPEC amendment: code-blocks

Paired with the queue entry `docs-code-block-copy-wrap`. A docs-site code block carries no copy button and does not wrap: the layout sets `white-space: pre-wrap` on `.markdown-body pre`, but the theme's compiled stylesheet sets `white-space: pre` on `.markdown-body pre > code`, the element every fenced block renders its text in, so the layout's rule never reaches the text. Measured on a local render (jekyll 4.4.1, jekyll-theme-primer 0.6.0): `assets/css/style.css` carries that rule, and every fence renders as `pre.highlight > code`. And nothing holds a block to one paste: a reader who copies a multi-command block whole runs every line, so a failing step leaves the later ones running. A shell runs each pasted line as its own command, bracketed paste or not. On the install page that includes the Windows step-by-step recipe, whose checksum `throw` stops only its own line where the terminal delivers the paste line by line, so the unpack and `init` lines after it still run (inferred: PowerShell hosts differ in how a multi-line paste arrives, and none was probed; the `. { … }` wrapper deltas 3 and 5 give it makes the block one statement under either delivery).

The unit spans the site (`docs/`: its layout, an asset, a page-authoring rule and the pages it reshapes) and canon-kit (the gate holding the rule), so it sits at the root.

## What changes

### (1) Code wraps {mechanical} {user-facing: the entry's operator direction, 2026-10-05 — code blocks wrap}

docs/_layouts/default.html's `<style>` gains a rule on the code element, beside the existing `pre` rule:

> `.site-content .markdown-body pre > code { white-space: pre-wrap; overflow-wrap: anywhere; }`

Its selector outranks the theme's `.markdown-body pre > code`, so the wrap reaches the text in a plain fence and in a highlighted one alike, since both render `pre > code`. The `pre` rule's `white-space: pre-wrap; overflow-wrap: break-word` stays, for a `pre` with no `code` child. A wrapped line is a visual break only: the text the copy takes (delta 2) keeps each line whole.

### (2) A copy button on every code block {design-bearing} {user-facing: the entry's operator direction, 2026-10-05 — a copy-to-clipboard button on docs-site code blocks}

A new layout script, `docs/assets/code-copy.js`, loaded by the layout after `search.js`, gives every `pre` under `.markdown-body` a button:

- **What it copies:** the block's code text exactly as authored, the `code` element's text where there is one and the `pre`'s otherwise, read without the button's own label.
- **How:** `navigator.clipboard.writeText`. Where that API is absent (a page served over plain HTTP or a file URL), no button is added, so none is shown that cannot work.
- **What the reader sees:** a `button` labelled `Copy`, at the block's top right, keyboard-focusable, with an `aria-label` naming its action; its label reads `Copied` for about two seconds after a successful write.
- **Style:** its CSS sits in the layout's `<style>` beside the code rules and reads the theme variables (`--ck-border`, `--ck-code-bg`, `--ck-muted`, `--ck-fg`), so it tracks both themes; the button is positioned against its block, so it stays at the top right however the text wraps.

The script ships from the site's own origin like `search.js`, with no CDN and no service.

### (3) The page-authoring rule {mechanical} {user-facing: the entry's operator direction, 2026-10-05 — every code block runs on its own, its commands split or joined with `&&`; and the operator direction of 2026-10-05, lead-relayed (not a ruling), that a bare assignment with no command substitution may stand unjoined ahead of its command and that a multi-statement PowerShell block is wrapped as `. { … }`, structurally gated}

A new bold-lead paragraph in docs/site-architecture.md §Page-authoring rules, after **Collapsible regions**. *Not yet applied.*

> **Code blocks.** A reader copies a block whole, so a shell block is one paste: it runs to its end or stops at its first failure. It holds one command, or commands joined with `&&`. A sequence that cannot be joined is split into a block per step, and a later block may use what an earlier one set. An assignment that runs nothing may stand on its own line, since it cannot fail. A PowerShell block of more than one statement is wrapped as `. { … }`: Windows PowerShell 5.1 has no `&&`, and the dot-sourced block stops at a terminating error such as a `throw` while keeping the variables it sets. A block shown for reading rather than running — output, a transcript, a grammar sketch — takes `text` or `console`. Long lines wrap and the copy button takes the block as written. `check-fence-paste-unit` holds both forms over the pages `CANON_KIT_FENCE_PASTE_PAGES` names.

### (4) check-fence-paste-unit {design-bearing} {user-facing: the operator direction of 2026-10-05, lead-relayed (not a ruling) — arm A admits a bare assignment, arm B gates the `. { … }` wrap}

A new canon-kit gate, born native: `canon-kit/checks/check-fence-paste-unit.gate` (`precommit`, binary-dispatched, `install: zero-config`, `armed-by: CANON_KIT_FENCE_PASTE_PAGES`), its rule in `native/src/gates/fence_paste_unit.rs`, with a `good/` and `bad/` fixture pair. A new section in canon-kit/SPEC.md, after §check-fence-run. *Not yet applied.*

> ### check-fence-paste-unit
>
> Invariant: on a declared page, every shell fence is one paste, and every PowerShell fence of more than one statement is one script block. A reader copies a fence whole, and a pasted sequence keeps running past a failed step. `check-fence-command-head` shows each head can run and `check-fence-run` runs what a doc marks; neither asks whether the fence stops where a step fails.
>
> - **Corpus:** files matching `CANON_KIT_FENCE_PASTE_PAGES`, an array of globs expanded like every canon-kit glob knob, default empty. An empty expansion is a clean `0 page(s)`.
> - **A shell fence** is a `bash`, `sh` or `shell` fence, the set `check-fence-command-head` reads, scanned by the crate's shell scanner. Its **top-level lists** are the runs of commands that a newline, `;` or `&` separates outside every subshell, group, compound command and substitution; `&&`, `||`, `|` and a line continuation join commands into one list, and a compound command is one command whatever it holds. A heredoc body is data.
> - **Arm A — a list a failure does not stop.** Every top-level list but the last must be a **bare assignment**: every word a `NAME=value` whose value carries no command substitution. Any other list followed by a further one is a finding, since its failure leaves the next one running.
> - **A PowerShell fence** is a `powershell`, `pwsh` or `ps1` fence. Its statement lines are its non-blank lines whose first non-blank character is not `#`.
> - **Arm B — an unwrapped PowerShell sequence.** A fence of two or more statement lines must open with the line `. {` and close with the line `}`, its first and last statement lines. Windows PowerShell 5.1 has no `&&`, and the dot-sourced block stops at a terminating error while keeping the variables it sets for a later block.
> - **Valve: the info string.** A fence shown for reading takes another language (`text`, `console`), as for `check-fence-command-head`; there is no line marker.
>
> **Red** names the page, the fence's opening line and, for arm A, the line of the list a failure would not stop. Arm A's `help:` line is *join the commands with `&&`, or split the block into one per step*; arm B's, *wrap the statements as `. { … }`*. The clean line counts pages, shell fences and PowerShell fences, so an armed corpus with no fence reads zero rather than passing unseen. **Exit 2:** an unreadable page. `tier=precommit`, `install: zero-config`, armed by `CANON_KIT_FENCE_PASTE_PAGES`.
>
> **Honest limits.** A native program's non-zero exit is no terminating error under Windows PowerShell 5.1, so arm B's block runs on past it; a step a later one depends on is checked in the block itself, as the install recipe's checksum `throw` is. Arm A reads the separators, not the commands: a joined list whose commands do not depend on each other passes, and whether a sequence should be joined or split is review's.

The scanner contract: native/src/bashscan.rs gains an entry point reporting, for each command head, the separator that put it in command position and whether it sits at the text's top level. `command_heads` and its callers are unchanged. The `spec:` comment on the new entry point cites §check-fence-paste-unit.

The knob row joins canon-kit/SPEC.md §Layout and configuration after `CANON_KIT_CITATION_LINK_PAGES`. *Not yet applied.*

> - `CANON_KIT_FENCE_PASTE_PAGES` — the pages `check-fence-paste-unit` holds, an array of globs, empty by default.

### (5) This repository's binding and the pages it reshapes {mechanical} {user-facing: the entry's operator direction, 2026-10-05 — commands split or joined with `&&`; the PowerShell blocks wrapped per the operator direction of 2026-10-05, lead-relayed (not a ruling)}

scripts/canon-config.knobs binds `CANON_KIT_FENCE_PASTE_PAGES[] = docs/*.md` and `docs/*/index.md`, the hand-authored site pages `CANON_KIT_PAGE_REPEAT_PAGES` binds; the generated mirrors are outside both globs, their fences being their kit sources'. scripts/gates.list registers the gate.

The members the gate reds at this binding, and each one's satisfying value. Probe: a fence scan over the tracked files the two globs match, at `46df0b05e`, classifying each shell fence's newline-separated lists by the arm-A rule and each PowerShell fence by its statement-line count:

- docs/install.md, the `macos-remedy` block (opens line 118): the three commands joined, `brew install bash \` / `  && echo … >> ~/.zprofile \` / `  && export PATH="$(brew --prefix)/bin:$PATH"`.
- docs/install.md, the download fence (line 139): `v=X.Y.Z` stays its own line; the `cw="$(mktemp -d)"` line and both `curl` lines are joined with `&&`, the *unpack outside the repository* comment moving to the prose above.
- docs/install.md, the `unix-install` block (line 156): the `init` line joins the extracting subshell, `( cd "$cw" … && tar -xzf "checkwright-$v.tgz" ) \` / `  && sh "$cw/package/bin/checkwright.sh" init`.
- docs/install.md, the `windows-remedy` block (line 183), the download fence (line 204) and the `windows-install` block (line 224): each wrapped as `. {` … `}` around its unchanged statements.
- docs/gate-sdk/index.md (line 21): the two arms joined, `"$gates" --run-gate-tests gate-sdk/gate-tests gate-sdk/checks \` / `  && "$gates" --install-hooks`, the assignment line kept.
- docs/queue-kit/index.md (line 22, `fence-runnable`): the two arms joined with `&&` the same way, the assignment line kept; the fence still runs under `check-fence-run`.
- docs/context-kit/index.md (line 21) and docs/lifecycle-kit/index.md (line 21): each arm roster split into one fence per arm, each opening with the `gates=` assignment line, since the arms are alternatives a reader runs one at a time and the placeholder each carries (`<file.md>`, `<stage>`) is filled per use.

The install-smoke legs run the three marked install-page blocks verbatim: the unix leg through `sh -c`, the Windows legs through `Invoke-Expression` with `$cw` and `$v` set in the calling scope (.github/workflows/gates.yml, the steps citing installer/SPEC.md §The consumer smoke). A joined list runs under `sh -c` as before, and a dot-sourced block reads and keeps the caller's variables, so neither leg's step is edited; the next push's legs witness the reshaped blocks. installer/SPEC.md §The consumer smoke's *a remedy step runs the page's block verbatim* stays true and is not edited.

## Producers and consumers

- **The wrap rule and the button** (deltas 1, 2): produced by the layout on every page Jekyll renders through it, which `docs/_config.yml`'s defaults make every page; consumed by the reader's browser. No config enables them.
- **The gate** (delta 4): produced by the battery and the generated pre-commit hook wherever `CANON_KIT_FENCE_PASTE_PAGES` matches a file; this repository's binding (delta 5) sets it, so a deployed configuration arms it. Its findings' readers are the committer and the battery report.
- **The scanner's separator record** (delta 4): its one reader is `fence_paste_unit.rs`. `command_heads`, which `check-fence-command-head`, `check-fence-run`, the port report and `check-gate-substrate-parity` read, keeps its output.
- **The knob** (delta 4): read by the new gate alone; its roster reader is the knob registry (`native/src/knobs/canon_kit.rs`), whose row `--emit knob-roster` prints and `check-knob-citation` resolves a citation against. The knob template canon-kit/templates/canon-config.knobs lists none of its page-glob siblings (`grep PAGE_REPEAT canon-kit/templates/canon-config.knobs` finds nothing) and is not edited.
- **Roster-holding readers of the new gate name**, each red when the name is missing: `scripts/gates.list` (the registry), the gate dispatch table in `native/src/gates/mod.rs`, canon-kit/README.md's gate roster, canon-kit/smoke/install.sh's roster, `.workflow/release-declarations.md` (a new gate is a release-visible change), and the generated projections the gate moves — `scripts/git-hooks/pre-commit`, docs/enforcement.md, docs/check-graph.html, docs/value.md's rollup, and the on-site mirrors of canon-kit's SPEC and README — each regenerated by the command its freshness gate prints. The roster is the one `check-docs-page-repeat`'s landing touched (`git show --stat 8924490c1`), the nearest precedent: a page-scoped canon-kit gate with a glob knob.
- **Readers of the reshaped pages** (delta 5): `check-fence-command-head` (every head still runs: a command joined with `&&` is a head like any other, and the gate reads no PowerShell fence), `check-fence-run` (the queue-kit and canon-kit kit-page fences stay marked and runnable; a split context-kit or lifecycle-kit fence carries no marker today and gains none), `check-docs-restatement-parity` (every arm and knob token the kit pages carry is unchanged), `check-docs-page-repeat` (fences are skipped, so the repeated `gates=` line is no finding), `check-docs-render-fidelity` (a re-rendered page), and the install-smoke legs above. `check-surface-ratchet` holds ceiling rows (`.workflow/surface-ceiling.txt`) for docs/install.md, docs/site-architecture.md, canon-kit/SPEC.md and the kit pages docs/context-kit/index.md, docs/lifecycle-kit/index.md, docs/gate-sdk/index.md and docs/queue-kit/index.md; each page this unit grows past its row has the row re-stamped in the same commit.
- **Corpus narrowing:** none; the gate adds a corpus and narrows none.
- **Every field has a reader:** the finding's page, line and arm are read by the committer; the clean line's three counts by the battery report.

## Existing sections updated

- docs/_layouts/default.html — the code-element wrap rule (delta 1); the button's style and the script tag (delta 2).
- `docs/assets/code-copy.js` — new (delta 2).
- docs/site-architecture.md §Page-authoring rules — the **Code blocks** paragraph (delta 3).
- canon-kit/SPEC.md §check-fence-paste-unit, new, and §Layout and configuration's knob row (delta 4).
- `native/src/bashscan.rs`, `native/src/gates/fence_paste_unit.rs`, `native/src/gates/mod.rs`, `native/src/knobs/canon_kit.rs`, `canon-kit/checks/check-fence-paste-unit.gate`, `canon-kit/gate-tests/check-fence-paste-unit/` (delta 4).
- canon-kit/README.md's gate roster, canon-kit/smoke/install.sh, `.workflow/release-declarations.md` (delta 4).
- scripts/canon-config.knobs, scripts/gates.list (delta 5).
- docs/install.md, docs/gate-sdk/index.md, docs/queue-kit/index.md, docs/context-kit/index.md, docs/lifecycle-kit/index.md (delta 5).
- scripts/git-hooks/pre-commit, docs/enforcement.md, docs/check-graph.html, docs/value.md, the canon-kit SPEC and README mirrors, and `.workflow/surface-ceiling.txt` where a ceiling moves (deltas 4 and 5).

## Retired spellings

- None — no delta of this amendment retires a spelling; the layout's `pre` rule, the scanner's `command_heads` and the install blocks' markers all stay.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the gate, its knob and the scanner's separator record.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — the new SPEC section and the page-authoring paragraph read as one document with their neighbours.
- [ ] **Amendment deleted** — this file removed on merge; no root `SPEC-*.md` of this unit remains.
- [ ] **Removals propagated** — none declared.
- [ ] **Gaps filed** — any page the gate reds beyond delta 5's roster is fixed in the batch or filed.
- [ ] **Rendered** — a local render (`ruby -e 'require "jekyll"; Jekyll::Commands::Build.process({"source"=>"docs","destination"=>".tmp/site-render"})'`) shows a long line wrapping inside a highlighted and a plain block in both themes, and the button copying a multi-line block whole.
- [ ] **Built and run** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, and the fixture suites of canon-kit and of gate-sdk, whose port report and parity gate read `bashscan.rs`.
- [ ] **Entry moved** — `--queue done docs-code-block-copy-wrap` a stage before the drain stage, once the install-smoke legs of the push carrying the reshaped install blocks are green; until then the entry bridges with a `[spec:]` path ref to docs/site-architecture.md.
