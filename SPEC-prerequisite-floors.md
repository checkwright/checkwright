# SPEC amendment: prerequisite-floors

Every prerequisite states a floor, measured from the construct that forces it. The install page states which prerequisites each system needs. This is the iteration's theme, "all prerequisites should have floor versions and we should aim for wide platform/OS support with clearly documented prerequisites for each" (operator direction, 2026-09-27). The pin rule stays (operator direction, 2026-09-27): a floor is pinned only where a construct forces it, and the construct is recorded with it. The theme is met by finding each member's forcing construct, not by setting floors nobody's code forces.

**Two further operator directions (2026-09-27, lead-relayed, not rulings) widen the page's half.** Every optional prerequisite is marked optional, with the condition that makes it owed. The operator's example is `shellcheck`: its row reads as a general dependency, though only a registered gate needs it. And the page splits prerequisites by what the adopter runs: the shipped gates, a shell gate they write, and a Rust gate they write. The Rust half states what is possible today. The registry resolves a member only to a `.sh` script or a `.gate` descriptor (`registry::resolve`), a descriptor dispatches only to a subcommand the published binary already carries, and an install carries no crate (gate-sdk/SPEC.md §The port-candidate criteria). So an adopter cannot write a Rust gate. The split also shows that a custom gate on native Windows is a bash script today. PowerShell and Rust custom gates are filed for a later scope and are outside this amendment.

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
- **A third marker block** in §Requirements, `<!-- prerequisites:begin -->` … `<!-- prerequisites:end -->`, placed after the toolchain block. It holds every prerequisite the roster does not, as a table `| Tool | Minimum | Needed for | Why |`, with one row per install-table member above from `sh` (install) to `liquid`. The Needed-for cell is one of `required to install on Linux and macOS`, `required to install on Windows`, `optional: only to install with npx` or `optional: only if you register site-kit's docs gates`, so a reader finds their system's rows by its name and sees at once whether a row binds them. The Git for Windows row's Minimum reads "`git` and `bash` at the floors above". The prose sentence on Ruby and the gems (the one beginning "To publish a docs site with site-kit's render-fidelity gate") is deleted, since its rows now carry it.
- **The two "You need" lines** in §macOS and Linux and §Windows each become a pointer to their rows: "**You need** the tools §Requirements lists for installing on Linux and macOS." and the Windows equivalent, which keeps its pointer to the Windows minimum. §With Node gains "(Node 8.2 or later, §Requirements)".

### (5) `check-install-platforms` holds the per-system block {design-bearing}

**Not yet applied.** `check-install-platforms` (`native/src/gates/install_platforms.rs`) gains arm F over the prerequisites block. It reds on four things:
- a row whose Minimum cell is empty or `—`;
- a row whose Needed-for cell opens with neither `required` nor `optional:`;
- a system family the platform block declares that no row's Needed-for cell names, where a family is the first word of a platform row's System cell (`Linux`, `macOS`, `Windows`);
- a missing or empty prerequisites block, which is exit 2 like the platform block's own absence.

The arm lives in this gate rather than `check-install-toolchain` because the families it is held against are this gate's declarations. The fixture pair in `scripts/gate-tests/check-install-platforms/` gains the block. `good/` names every family its platform rows declare. `bad/` carries an empty Minimum, an unmarked Needed-for cell and a declared family no row names. Each `expect.txt` is updated. The descriptor `scripts/check-install-platforms.gate` already couples `docs/install.md`, so its trigger needs no change.

### (6) The SPECs point at the block {mechanical}

**Not yet applied.**
- In installer/SPEC.md §Requirements, the first paragraph's list of each transport's tools ("Fetched from npm it needs Node, for `npx`. Fetched as the tarball … plus the `/bin/sh` or PowerShell that runs its script.") becomes: "Each transport's own tools, and their floors, are the install page's prerequisites block, one row per tool naming the systems it is needed for." The paragraph's argument about which requirements belong to a delivery path stays.
- The bullet "**The install page's requirement blocks, and why each reads as it does.**" changes "six marker blocks: the platform and toolchain tables" to "seven marker blocks: the platform, toolchain and prerequisites tables". A bullet is added after **The docs-site tier**: "**The prerequisites block.** It carries what the roster does not, because none of it is probed: the install path's tools are needed before a binary exists to probe them, and the docs gates' renderer is a knob-supplied command no `REGISTRY` row names. Arm F of `check-install-platforms` holds that every row states a Minimum and every declared system has a row." **The docs-site tier** bullet then ends by pointing at the block's rows for its versions.
- docs/site-architecture.md gains the prerequisites block's contract row beside the two install-parity rows: its grammar, arm F as its reader, and the Why cell as unread hand prose.

### (7) The toolchain block's Needed cell says required or optional {mechanical}

**Not yet applied.** `render` in `native/src/gates/install_toolchain.rs` spells the audience cell so that the page cannot state an optional tool without its condition. An empty audience renders `required` where it rendered `every profile`. A kit list renders `optional: if your profile includes <kits>`, with the names joined by `, ` and the last two by ` or `. So bash's cell reads `optional: if your profile includes context-kit, drift-kit or guard-kit`. `registered` renders `optional: if you register a gate that runs it` where it rendered `registered gates`. `contributor` keeps `contributors`, since that row appears only in CONTRIBUTING.md, whose readers are all contributors. The constants `EVERY_PROFILE` and `REGISTERED_GATES` take the new spellings. The unit tests asserting `render`'s output, and the gate's fixture pages under `scripts/gate-tests/check-install-toolchain/`, take them too. docs/install.md's toolchain header cell `Needed by` becomes `Needed`, since the cell now carries a condition rather than a party. In docs/site-architecture.md's install-toolchain parity contract, the Needed-by sentence ("`every profile` for none; `contributors` for `contributor`; `registered gates` for `registered`; a kit list as its names joined by `, `") is rewritten to the four spellings above. In installer/SPEC.md §Requirements, **The audience token** bullet's "`registered gates` is held only where a gate registered in `gates.list` needs the tool. `every profile` binds every adopter's machine" becomes:

> `optional: if you register a gate that runs it` is held only where a gate registered in `gates.list` needs the tool, and `optional: if your profile includes …` only where the profile carries a named kit. `required` binds every adopter's machine

`doctor`'s own not-probed lines are unchanged. They already name the condition (installer/SPEC.md §doctor).

### (8) The page splits prerequisites by what the adopter runs {design-bearing}

**Not yet applied.** docs/install.md §Requirements keeps its opening paragraph and the platform block. Everything after them falls under three `###` headings:

- **`### Installing and running the shipped gates`** holds the toolchain block's lead sentence, the toolchain block, the prerequisites block (delta 4), and the paragraph on where a missing tool comes from and the `--emit env-probe` self-check. Its lead sentence becomes: "The tools each shipped gate runs. `required` binds every install. An `optional` row binds you only under the condition it names:".
- **`### Writing your own shell gates`**, in prose. A gate you write is a copy of gate-sdk's `templates/check-skeleton.sh`. It is a bash script that sources gate-sdk's library, so it needs the `bash` floor above on every system, whatever your profile. `doctor` does not check that, because the roster owes `bash` only through the kits you vendor. On native Windows that bash is Git for Windows' bash: a gate cannot be written in PowerShell today. `check-shellcheck` lints your gate if you register it, and then `shellcheck`'s row binds you.
- **`### Writing your own Rust gates`**, in prose. You cannot today. A compiled gate is a subcommand of the published gate binary, and an install carries no crate to add one to. The one link is to gate-sdk/SPEC.md §The port-candidate criteria, where that fact is owned.

The Windows "You need" line (delta 4) adds that Git for Windows' bash is what runs any shell gate you write, and links §Writing your own shell gates.

In installer/SPEC.md §Requirements, the **The `bash` audience** bullet gains, after its sentence on the starter and prose profiles:

> A shell gate an adopter writes is outside the audience: it is not a kit file, so the derivation cannot see it, and `doctor` does not owe `bash` for it. The install page states that requirement in prose, native Windows included, where Git for Windows' bash is the only shell a custom gate runs under.

The limit that `doctor` does not owe `bash` for a consumer-registered shell gate is stated, not closed. Closing it would be a fourth derivation arm, which is new asserted behaviour outside this amendment's envelope. It is filed to the gap inbox with this amendment's commit.

## Producers and consumers

- **The four new floors.** Producer: `PROBE_SET`. Consumers, all existing and all reading the constant: `doctor` (an owed member below its floor sets exit 1, so `init` refuses on it), `--emit env-probe`'s rendered verdict, and `check-install-toolchain`'s parity. The user-facing consequence: `init` and `doctor` now refuse `git` older than 2.15 on every profile, `curl` older than 5.9 where delegation-kit is selected, and `shellcheck` older than 0.6.0 where a registered gate needs it. `jq` is contributor-only and reaches no adopter verdict.
- **The `any` rendering.** Producer: `install_toolchain.rs`'s render. Consumer: the parity comparison against the pages. No live row carries it after delta 1.
- **The prerequisites block.** Producer: the page, hand-authored. Consumers: arm F (delta 5) and the reader. The Why and Needed-for cells' prose is unread except for the family names arm F reads out of Needed-for.
- **The required/optional spellings.** Producer: `render` (delta 7) for the toolchain block, and the page's hand-authored prerequisites block (delta 4). Consumers: `check-install-toolchain`'s parity, which compares the cell whole, and arm F's `required`/`optional:` prefix test. `doctor` and env-probe render their own audience text and do not read these cells.
- **The three headings.** Producer: the page. Consumer: the reader. No gate reads a heading. Arm F and the toolchain parity find their blocks by marker, so moving the blocks under a `###` heading moves no reader.
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
- `native/src/gates/install_toolchain.rs` (constants and tests), `scripts/gate-tests/check-install-toolchain/` pages and expectations, docs/install.md's and CONTRIBUTING.md's toolchain headers, docs/site-architecture.md's Needed-by sentence, and installer/SPEC.md §Requirements' audience-token bullet (delta 7).
- docs/install.md §Requirements, restructured under three headings, and §Windows' "You need" line, plus installer/SPEC.md §Requirements' `bash` audience bullet (delta 8).
- The on-site mirrors of `context-kit/SPEC.md` and `installer/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (deltas 2, 3, 6, 7 and 8).
- `.workflow/release-declarations.md` gets one Behavior changes bullet. `doctor`, and so `init`, now hold `git` ≥ 2.15 on every profile, `curl` ≥ 5.9 where delegation-kit is selected and `shellcheck` ≥ 0.6.0 where a registered gate needs it. A host below refuses at `doctor` and names the floor. Upgrade the tool (delta 1).

## Retired spellings

- None — the three cell spellings deltas 3 and 7 retire (`—`, `every profile`, `registered gates`) are also ordinary prose across the tree, and `git grep -l` finds each in SPECs, release posts and fixtures at sites unrelated to the table, so a survivor scan would report prose rather than missed sites; the surfaces that spell them as cell values are rostered above.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the four floors, the `any` token, the required/optional spellings, the block and arm F.
- [ ] **Instruction surfaces: instruction only.** No template changes.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `prerequisite-floor-versions` moves to Done in its landing commit, at a stage before the drain stage. Its oracles are local.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work goes to the gap inbox.
