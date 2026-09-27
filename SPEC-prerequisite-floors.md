# SPEC amendment: prerequisite-floors

Every prerequisite states a floor, measured from the construct that forces it. The install page states which prerequisites each system needs. This is the iteration's theme, "all prerequisites should have floor versions and we should aim for wide platform/OS support with clearly documented prerequisites for each" (operator direction, 2026-09-27). The pin rule stays (operator direction, 2026-09-27): a floor is pinned only where a construct forces it, and the construct is recorded with it. The theme is met by finding each member's forcing construct, not by setting floors nobody's code forces.

**What is wrong today.** The toolchain roster (`PROBE_SET` in `native/src/toolfloor.rs`) floors only `bash` (4.3) and `cargo` (1.71). `git`, `curl`, `shellcheck` and `jq` are bare names, and the install page renders them as `—`. context-kit/SPEC.md §bin/env-probe says no construct forces a version on them. The survey below finds a forcing construct for each of them. For `git` it found one whose failure is silent. The install path's own tools (`sh`, `curl`, `tar`, the hashers, Windows PowerShell, `tar.exe`, Git for Windows, Node) and site-kit's Ruby and gems state no version anywhere a reader looks. They appear only in two "You need" lines and one prose sentence, and no gate reads any of these.

**Measured at authoring (2026-09-27).** The construct enumeration covered every spawn in shipped code (native/src non-test code, kit `bin/`, `lib/`, `checks/`, the two bootstraps and the install fences). It was checked against each tool's primary source. Where a floor rests on a behaviour, the old and floor versions were run:

| member | forcing construct (site) | floor | evidence |
|---|---|---|---|
| `git` | `git rev-parse --is-shallow-repository` (`native/src/gates/docs_cmd.rs`, `check-docs-cmd`) | 2.15 | git's RelNotes 2.15.0. Run: git 2.7.4 prints the flag back with exit 0, so the gate reads every clone as full with no error. git 2.17.1 answers `false`. |
| `curl` | `-S` / `--show-error`, in the `-fsS` the usage poller (`native/src/hook/poll.rs`) and the install lines pass | 5.9 | curl's `docs/cmdline-opts/show-error.md`, `Added: 5.9`. Every other flag used is 5.0 or older. |
| `shellcheck` | `-S warning` (`native/src/gates/shellcheck.rs`, `action_run_shell.rs`) | 0.6.0 | ShellCheck CHANGELOG. Run: 0.5.0 refuses `-S` with exit 3. Under 0.6.0 gate-sdk's fixture suite (43 pairs, the two ShellCheck gates' pairs among them) is clean. |
| `jq` | `--argjson`, `--slurpfile` and the regex builtins (`guard-kit/smoke/install.sh`, `installer/consumer-smoke/run-smoke.sh`) | 1.5 | jq NEWS. Run: 1.4 refuses `--argjson` with exit 2, and 1.5 accepts it and runs `test()`. |
| `bash`, `cargo` | unchanged: the nameref, and the crate graph's highest MSRV | 4.3, 1.71 | context-kit/SPEC.md §bin/env-probe |
| `sh` (install) | the bootstraps and `docs/install.sh` are POSIX sh throughout | any POSIX `sh` | read. No `[[`, arrays, `local` or `pipefail`. |
| `curl` (install) | `-fsSL`, `-o` | 5.9 | as above (`-L` 4.9) |
| `tar` (install) | `-xzf`, `-C` | any GNU or BSD `tar` | `-z` is not POSIX (POSIX specifies `pax`, which declines a compression flag), and both tars carry it |
| `sha256sum` or `shasum` (install) | `-c`, `shasum -a 256 -c` | any | no versioned flag |
| Windows PowerShell (install) | the bootstrap and the Windows install block | 5.1 | installer/SPEC.md §Requirements: the block runs under the one PowerShell every Windows host carries |
| `tar.exe` (install) | the Windows install block's `System32\tar.exe` | Windows 10 version 1803 | Microsoft's announcement of build 17063, released as 1803 |
| Git for Windows (install) | supplies `git` and, where owed, `bash` | its `git` and `bash` at their floors | the roster members themselves, so no second number |
| Node (install) | `npx` | 8.2 | npm 5.2.0 is the first npm whose `bin` carries `npx` (npm registry), and Node 8.2.0 the first to bundle npm ≥ 5.2 (nodejs.org dist index). The package's `bin` is the two bootstraps, with no JS shim. |
| Ruby (docs gates) | the gems below | 2.3 | rubygems.org: `kramdown-parser-gfm` 1.1.0 (its latest release) needs Ruby ≥ 2.3, and `liquid` 4.0.4 ≥ 2.1.0 |
| `kramdown-parser-gfm` | `check-docs-render-fidelity`'s default renderer | any release (1.1.0 is the latest) | no version-gated use, and the gates job installs it unpinned |
| `liquid` | `check-docs-liquid-parse`'s default parser | 4.0.4 exactly | the version GitHub Pages runs, the gates job's pin (site-kit/SPEC.md §check-docs-liquid-parse's exact-pin recipe) |

The git table row is why a floor is read off constructs rather than off a battery run. A battery run this session in an ubuntu:16.04 container, with git 2.7.4 and bash 4.3.48, was as green on every git-reading gate as the git 2.43 control. Its one extra red was a contributor test harness using `git init -b`. The construct that set the floor fails silently, and no differential battery can see that. **No CI floor leg is added for that reason.** Its oracle is blind to exactly the class that set the floor.

## What changes

### (1) The roster takes the measured floors {mechanical}

**Not yet applied.** In `native/src/toolfloor.rs`, `PROBE_SET`'s elements become `git:2.15`, `curl:5.9::delegation-kit`, `shellcheck:0.6::registered` and `jq:1.5::contributor`. `bash:4.3::derived` and `cargo:1.71::contributor` are unchanged. `doctor`, `--emit env-probe` and `check-install-toolchain` read the constant, so they hold the new floors with no change of their own. A member added later as a bare name still renders explicitly, as `any` (delta 3), so the page never shows a blank floor.

### (2) The SPEC records each construct {mechanical}

**Not yet applied.** In context-kit/SPEC.md §bin/env-probe, "The constrained members and what forces each" gains four bullets after the `cargo` bullet, one per newly floored member. Each names its construct and site and the fact that fixes the version, as the table above gives them. The `git` bullet also states the silent failure: below 2.15 the flag comes back as text and `check-docs-cmd` reads a shallow clone as full. The `cargo` bullet's closing sentences "Every other member is a bare name — no construct in the battery forces a version on it (the `jq` usage is 1.5-era throughout), so none is pinned." become:

> Every member carries a floor. A member the survey finds no versioned construct for renders `any` instead (installer/SPEC.md §Requirements), so the page never leaves a floor blank. **Honest limit:** a construct added later raises a floor silently unless its author re-runs the survey, because no gate reads a program's flags against a version table. The survey is re-run at any change that spawns a program in a new way. A battery run on an old host does not replace it: the git floor's construct fails silently there.

### (3) An unforced prerequisite says `any` {mechanical}

**Not yet applied.** `check-install-toolchain` (`native/src/gates/install_toolchain.rs`) renders a member with neither floor nor implementation token as `any` rather than `—`. docs/site-architecture.md's install-toolchain parity contract changes "or `—` for neither" to "or `any` for neither". After delta 1 no live member renders it. It stays the rendering a future unforced member gets, stated rather than blank.

In installer/SPEC.md §Requirements, the pin-rule bullet gains, after its first sentence:

> A prerequisite no construct forces says `any` rather than leaving the cell blank, so a reader can tell a considered absence from an omission.

### (4) The page states the floors and each system's prerequisites {design-bearing}

**Not yet applied.**
- **The toolchain block** in docs/install.md takes the new Version cells (`≥ 2.15`, `≥ 5.9`, `≥ 0.6`). Each Why cell names its construct in a clause, for example git's: "the gates read tracked files, the hooks fire at commit time, and `check-docs-cmd` reads `rev-parse --is-shallow-repository`". CONTRIBUTING.md's `jq` row takes `≥ 1.5`, with a Why naming `--argjson` and `--slurpfile`.
- **A third marker block** in §Requirements, `<!-- prerequisites:begin -->` … `<!-- prerequisites:end -->`, placed after the toolchain block. It holds every prerequisite the roster does not, as a table `| Tool | Minimum | Needed for | Why |`, with one row per install-table member above from `sh` (install) to `liquid`. The Needed-for cell is one of `installing on Linux and macOS`, `installing on Windows`, `installing with npx` or `site-kit's docs gates`, so a reader finds their system's rows by its name. The Git for Windows row's Minimum reads "`git` and `bash` at the floors above". The prose sentence on Ruby and the gems (the one beginning "To publish a docs site with site-kit's render-fidelity gate") is deleted, since its rows now carry it.
- **The two "You need" lines** in §macOS and Linux and §Windows each become a pointer to their rows: "**You need** the tools §Requirements lists for installing on Linux and macOS." and the Windows equivalent, which keeps its pointer to the Windows minimum. §With Node gains "(Node 8.2 or later, §Requirements)".

### (5) `check-install-platforms` holds the per-system block {design-bearing}

**Not yet applied.** `check-install-platforms` (`native/src/gates/install_platforms.rs`) gains arm F over the prerequisites block. It reds on three things:
- a row whose Minimum cell is empty or `—`;
- a system family the platform block declares that no row's Needed-for cell names, where a family is the first word of a platform row's System cell (`Linux`, `macOS`, `Windows`);
- a missing or empty prerequisites block, which is exit 2 like the platform block's own absence.

The arm lives in this gate rather than `check-install-toolchain` because the families it is held against are this gate's declarations. The fixture pair in `scripts/gate-tests/check-install-platforms/` gains the block. `good/` names every family its platform rows declare. `bad/` carries an empty Minimum and a declared family no row names. Each `expect.txt` is updated. The descriptor `scripts/check-install-platforms.gate` already couples `docs/install.md`, so its trigger needs no change.

### (6) The SPECs point at the block {mechanical}

**Not yet applied.**
- In installer/SPEC.md §Requirements, the first paragraph's list of each transport's tools ("Fetched from npm it needs Node, for `npx`. Fetched as the tarball … plus the `/bin/sh` or PowerShell that runs its script.") becomes: "Each transport's own tools, and their floors, are the install page's prerequisites block, one row per tool naming the systems it is needed for." The paragraph's argument about which requirements belong to a delivery path stays.
- The bullet "**The install page's requirement blocks, and why each reads as it does.**" changes "six marker blocks: the platform and toolchain tables" to "seven marker blocks: the platform, toolchain and prerequisites tables". A bullet is added after **The docs-site tier**: "**The prerequisites block.** It carries what the roster does not, because none of it is probed: the install path's tools are needed before a binary exists to probe them, and the docs gates' renderer is a knob-supplied command no `REGISTRY` row names. Arm F of `check-install-platforms` holds that every row states a Minimum and every declared system has a row." **The docs-site tier** bullet then ends by pointing at the block's rows for its versions.
- docs/site-architecture.md gains the prerequisites block's contract row beside the two install-parity rows: its grammar, arm F as its reader, and the Why cell as unread hand prose.

## Producers and consumers

- **The four new floors.** Producer: `PROBE_SET`. Consumers, all existing and all reading the constant: `doctor` (an owed member below its floor sets exit 1, so `init` refuses on it), `--emit env-probe`'s rendered verdict, and `check-install-toolchain`'s parity. The user-facing consequence: `init` and `doctor` now refuse `git` older than 2.15 on every profile, `curl` older than 5.9 where delegation-kit is selected, and `shellcheck` older than 0.6.0 where a registered gate needs it. `jq` is contributor-only and reaches no adopter verdict.
- **The `any` rendering.** Producer: `install_toolchain.rs`'s render. Consumer: the parity comparison against the pages. No live row carries it after delta 1.
- **The prerequisites block.** Producer: the page, hand-authored. Consumers: arm F (delta 5) and the reader. The Why and Needed-for cells' prose is unread except for the family names arm F reads out of Needed-for.
- **Roster-holding readers of the new names.** The marker pair `prerequisites:begin`/`:end` is read by arm F alone. docs/site-architecture.md's block roster names it (delta 6). No knob is minted.
- **Point 5.** No corpus narrows.
- **Point 6.** Every member's satisfying value is named in the table above: the roster's six members and the eleven page-only prerequisites, enumerated by the construct survey (`grep` of every `programs::` spawn and every install fence) and the page's two "You need" lines and docs-tier sentence.
- **Sibling dependency.** `linux-musl-artifacts` / `glibc-floor-lowering` change the platform block's Linux rows, and arm F reads that block's families. The family is the System cell's first word, `Linux`, under either route, so the two units compose in either order.

## Existing sections updated

Roster from `grep -n "PROBE_SET" -A 8 native/src/toolfloor.rs`, `grep -n "NEITHER" native/src/gates/install_toolchain.rs`, `grep -n "You need\|To publish a docs site" docs/install.md`, `grep -n "six marker blocks\|The pin rule\|docs-site tier" installer/SPEC.md`, `grep -n "Every other member is a bare name" context-kit/SPEC.md` and `grep -n "for neither" docs/site-architecture.md`, run 2026-09-27.

- `native/src/toolfloor.rs` (delta 1).
- context-kit/SPEC.md §bin/env-probe (delta 2).
- `native/src/gates/install_toolchain.rs`, docs/site-architecture.md's install-toolchain parity row, and installer/SPEC.md §Requirements' pin-rule bullet (delta 3).
- docs/install.md §Requirements, §macOS and Linux, §Windows and §With Node, and CONTRIBUTING.md's toolchain block (delta 4).
- `native/src/gates/install_platforms.rs` and `scripts/gate-tests/check-install-platforms/` (delta 5).
- installer/SPEC.md §Requirements and docs/site-architecture.md's block roster (delta 6).
- The on-site mirrors of `context-kit/SPEC.md` and `installer/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (deltas 2, 3 and 6).
- `.workflow/release-declarations.md` gets one Behavior changes bullet. `doctor`, and so `init`, now hold `git` ≥ 2.15 on every profile, `curl` ≥ 5.9 where delegation-kit is selected and `shellcheck` ≥ 0.6.0 where a registered gate needs it. A host below refuses at `doctor` and names the floor. Upgrade the tool (delta 1).

## Retired spellings

- None — the `—` Version cell delta 3 retires is a glyph the tree uses everywhere as punctuation, so a survivor scan over it would report prose rather than missed sites; the two surfaces that spelled it as a cell value are rostered above.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the four floors, the `any` token, the block and arm F.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `prerequisite-floor-versions` moves to Done in its landing commit, at a stage before the drain stage. Its oracles are local.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
