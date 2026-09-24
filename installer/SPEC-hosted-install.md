# SPEC amendment: hosted-install

Without Node, the install page's path takes four steps on each system: download, verify, extract and run. installer/SPEC.md §The dependency boundary refuses a `curl … | sh` or `irm … | iex` one-liner, on the ground that "an unreviewed remote script fed straight to a shell is the counter-pattern of the claim the page opens with". That refusal was SPEC text and never an operator ruling.

**The operator reversed it by direction on 2026-09-24, lead-relayed, with one binding constraint: native installation with minimal external dependencies, and no npx-class dependency.** The mechanism is this amendment's to choose.

**The mechanism: the recipe itself, hosted, pinned and tracked.** Two scripts, `docs/install.sh` (POSIX sh) and `docs/install.ps1` (PowerShell), each perform exactly the page's four steps against one pinned release. The docs site serves them at `https://checkwright.dev/install.sh` and `https://checkwright.dev/install.ps1`, so the one-liners are:

- `curl -fsSL https://checkwright.dev/install.sh | sh`
- `irm https://checkwright.dev/install.ps1 | iex`

**Why this answers the refused ground rather than overriding it:**

- **What runs is reviewed text.** It is a file in this repository at a named version, carrying the same checksum step as the recipe. What changes is the moment a reader reviews it: before piping, at a URL, rather than line by line.
- **It adds no dependency.** On macOS and Linux it needs what the recipe needs (`curl`, `tar`, a hasher) plus the `/bin/sh` the recipe already runs. On Windows it needs only what Windows ships.
- **The four-step recipe stays on the page** as the path for a reader who reviews each step first.

**Refused mechanisms, each on the operator's constraint:**

- **A script that asks the GitHub API for the newest release at run time.** No release is Latest while the channel is `preview` (§The release channel: `releases/latest` answers 404). Reading the release list takes a JSON parse `sh` does not have, which reaches for `jq`, an external dependency.
- **An npm-backed one-liner.** It is the npx-class dependency the direction excludes.

**Measured and read at authoring (2026-09-24):**

- `docs/CNAME` is `checkwright.dev`, and the site is served from `docs/` on master.
- No `docs/*.sh` or `docs/*.ps1` exists yet (`ls docs/*.sh docs/*.ps1` exits 2).
- `GATE_SDK_LINT_EXTRA_DIRS` in `scripts/gate-sdk-config.knobs` is `installer/bin installer/consumer-smoke demo`, and `GATE_SDK_PORTABILITY_PATHS` is `installer/bin gate-sdk/lib/gate.sh gate-sdk/bin/run-gates.sh gate-sdk/bin/run-gates.ps1`. Neither reaches `docs/`.
- `CANON_KIT_FENCE_PROGRAMS_EXTRA` already carries `curl`.

**Inferred, cannot run before the close push serves the files:**

- that Pages copies both files to the site root byte for byte;
- that `irm` returns `install.ps1` as text under the content type Pages sends for `.ps1`;
- that Windows PowerShell 5.1 on a supported Windows host negotiates TLS 1.2 for `irm` by default.

The first push serving the files is the first time either URL resolves, and delta 5 names the probe that push owes.

## What changes

### (1) The two hosted scripts {design-bearing}

**Not yet applied.** `docs/install.sh` is POSIX sh under `#!/bin/sh`, written to installer/SPEC.md §Implementation's four settled constraints. `docs/install.ps1` is its twin, ASCII outside its comments, and runs under Windows PowerShell 5.1 and PowerShell 7. Each carries:

- **One pin line.** `pin='X.Y.Z'` in sh and `$pin = 'X.Y.Z'` in PowerShell, the newest release's version. It is the only version the script names.
- **Two environment overrides.**
  - `CHECKWRIGHT_VERSION`, the version to install, defaulting to the pin. It lets a reader install a named older release and the witness install a packed one.
  - `CHECKWRIGHT_RELEASE_BASE`, the download base, defaulting to `https://github.com/checkwright/checkwright/releases/download`. It exists for the witness, because `irm` cannot read `file://`.
- **The recipe's steps, in the recipe's order.**
  - Make a fresh temporary directory outside the working tree.
  - Fetch `<base>/v<version>/checkwright-<version>.tgz` and its `.sha256`. The sh script uses `curl -fsSL`. The PowerShell script uses `Invoke-WebRequest -UseBasicParsing` with `$ProgressPreference` silenced, after adding TLS 1.2 to the process's security protocols.
  - Verify: `sha256sum -c`, else `shasum -a 256 -c`, in sh; `Get-FileHash` against the sidecar's first field in PowerShell.
  - Extract: `tar` in sh; the system directory's `tar.exe` in PowerShell, for the reason §The dependency boundary gives.
  - Run the bootstrap from the caller's working directory. The sh script runs `sh "<dir>/package/bin/checkwright.sh" <args>`. The PowerShell script runs `powershell -NoProfile -ExecutionPolicy Bypass -File "<dir>\package\bin\checkwright.ps1" <args>`.
- **The arguments.** `<args>` is the script's own arguments, or `init` when it has none. sh takes them as `curl … | sh -s -- <args>`, and PowerShell as `& ([scriptblock]::Create((irm …))) <args>`.
- **Cleanup and status.** The temporary directory is removed on every exit, by a `trap` in sh and a `finally` in PowerShell. The script exits with the bootstrap's status, and a failed download or checksum stops it before anything is extracted, naming the step.
- **Truncation safety.** Each script's body is one function, called on its last line, so a download cut short defines nothing and runs nothing.

### (2) §The dependency boundary states the one-liner {design-bearing}

**Not yet applied.** In installer/SPEC.md §The dependency boundary, the paragraph **The tarball recipe's shape.** becomes two paragraphs:

> **The one-line install.** The install page leads each system's section with one line: `curl -fsSL https://checkwright.dev/install.sh | sh`, or `irm https://checkwright.dev/install.ps1 | iex` on Windows. The line fetches a script the docs site serves from this repository, `docs/install.sh` or its PowerShell twin, and that script is the recipe below and nothing else: it downloads one pinned release and its digest, verifies it, unpacks it outside the working tree, and runs the bootstrap from the caller's directory with the caller's arguments (`init` by default), then removes what it unpacked. The script is tracked text at a named version, so what runs is what this repository reviewed. The pin moves only to a published tag (§Versioning), so the line never names a release that does not exist yet. The script adds no dependency to the recipe's own. Each body is a single function called on the last line, so a truncated fetch runs nothing. **The honest limit:** piping runs the script before the reader reads it unless they fetch it first. The script is served over the site's own TLS origin, so trusting it is trusting the site, one origin more than the recipe, which trusts only the release host. The recipe stays on the page for the reader who reviews each step first.
>
> **The recipe's shape.** Below the line, each system carries the same install as four steps — download, verify, extract, run — …

The rest of the old paragraph continues unchanged from "Each unpacks outside the repository". §Requirements' tarball sentence gains: "The one-line install needs the same, plus the `/bin/sh` or PowerShell that runs its script." In §Requirements, **The install blocks** bullet gains the witness (delta 4) as a following sentence.

### (3) The pin is held to the newest tag {design-bearing}

**Not yet applied.** A new repo-root gate, `check-install-pin`, is born native. It has a Rust module, `native/src/gates/install_pin.rs`, dispatched from `native/src/gates/mod.rs`; a descriptor, `scripts/check-install-pin.gate` (`precommit`, coupling `docs/install.sh` and `docs/install.ps1`); a `good/`+`bad/` pair under `scripts/gate-tests/check-install-pin/`; and a registration in `scripts/gates.list`. Its positional form, `check-install-pin [install-sh install-ps1 [version]]`, points it at a fixture pair, and the version is passed explicitly so the fixture holds still as this repository's tags move. That is `check-release-channel-parity`'s own fixture shape. Two invariants:

- **A: the twins agree.** Each script carries exactly one pin line of its grammar, and the two pins are equal.
- **B: the pin is the newest release.** The pin equals the newest tag by creator date, read by `check-release-channel-parity`'s reader (`native/src/gates/release_channel_parity.rs` `newest_tag`). That reader moves to a function both gates call, so the one question has one answer.

**Fail-closed conditions, exit 2:**

- a missing or duplicated pin line;
- a pin that is not `<major>.<minor>.<patch>`;
- a newest tag that does not parse as semver.

A repository with no tags has no version line, so B is dormant there, and the clean line says so, as `check-release-channel-parity`'s does. A shallow CI checkout carries no tags and meets the same dormancy.

**Why the pin trails the tag rather than leading it.** Pages deploys on every master push, and a release's assets exist only after its tag's `publish` run. A pin moved before the tag would, for that window, name a release the one-liner cannot download. Moved after the tag, it is at worst one release stale, and every release it names exists.

In installer/SPEC.md §Versioning, a new subsection **The hosted install pin** carries the two invariants, the exit-2 set, the dormancy, and the paragraph above.

In `RELEASING.md` step 4, the drain paragraph gains: "The same commit moves both hosted install scripts' pin line to the new tag, which `check-install-pin` demands from the first commit after the tag."

### (4) The CI witness runs each one-liner {mechanical}

**Not yet applied.** In `.github/workflows/gates.yml`, two legs gain a step after their page-install step.

**`install-smoke-sh-linux`**, whose `/bin/sh` is dash:

- Place the packed tarball and a fresh digest under `<serve>/v<version>/`, and copy `docs/install.sh` to `<serve>/install.sh`.
- Serve `<serve>` on `127.0.0.1` with `python3 -m http.server`.
- From a fresh consumer repository, run `curl -fsSL http://127.0.0.1:<port>/install.sh | CHECKWRIGHT_VERSION=<version> CHECKWRIGHT_RELEASE_BASE=http://127.0.0.1:<port> sh`.
- Assert the exit status and a written `checkwright.lock`.

**`install-smoke-pwsh-windows`** does the same through `irm http://127.0.0.1:<port>/install.ps1 | iex`, with the two overrides set in `$env:`, under Windows PowerShell.

The local server is the witness's own tool and no adopter's. The runner images carry Python.

**What the witness does not reach, stated:** the served copy at the live URL, Pages' content type, and TLS. The HTTP path is local and plain. Delta 5's first-observation probe covers the first two.

### (5) The page and the front doors lead with the line {mechanical}

**Not yet applied.**

- **`docs/install.md` §macOS and Linux and §Windows.** Each opens with its one line in a fence, "from your repository root", and one sentence saying the script is `docs/install.sh` (or `.ps1`) in the repository, to read at the URL before piping it. Two more sentences:
  - The argument forms: `sh -s -- --profile prose`, `sh -s -- demo`, `sh -s -- uninstall`, and the scriptblock form on Windows.
  - For `uninstall`, `CHECKWRIGHT_VERSION` names the installed version.

  The existing recipe follows under "Or step by step". The macOS remedy block stays ahead of both, since a profile owing `bash` needs it either way. The `install-primary: tarball` declaration is unchanged, because the line fetches the tarball.
- **`docs/index.md`.** "each a download, a checksum and one command" becomes the two one-liners, and the `npx` block stays under "With Node on the machine".
- **`README.md` §Install.** "with one recipe for macOS and Linux and one for Windows" becomes "one line for macOS and Linux and one for Windows, each fetching a pinned release, verifying it and running `init`".
- **`installer/README.md`.** "follow the Release tarball recipe for your system" becomes "run the one line for your system", with the recipe named as the step-by-step alternative.
- **`docs/site-architecture.md` §Generated projections and their freshness gates.** Gains a row: *The hosted install scripts*. `docs/install.sh` and `docs/install.ps1` are served verbatim, and `check-install-pin` holds their pin to each other and to the newest tag.
- **`scripts/gate-sdk-config.knobs`.** `GATE_SDK_LINT_EXTRA_DIRS` gains `docs`, whose only direct `*.sh` member is `docs/install.sh`, so `check-shellcheck` lints it under the dialect its shebang selects. `GATE_SDK_PORTABILITY_PATHS` gains `docs/install.sh docs/install.ps1`, so `check-portability-floor`'s pattern and ASCII arms read both.
- **The first observation of the served files.** No tree state settles the three inferred premises above. The first push that serves the files, which is the iteration's close push unless the lead places one earlier, is their first observation. Its probe is three reads:
  - `curl -sI https://checkwright.dev/install.ps1` for the content type;
  - `curl -fsSL` of each URL compared byte for byte against its tracked file;
  - on a Windows host, or the `pwsh` an Ubuntu runner carries, `(irm https://checkwright.dev/install.ps1).GetType().FullName` reading `System.String`.

  A failing read reopens delta 1's Windows spelling and is filed as a gap. This amendment is deleted at merge, so the spec stage reports the probe to the lead, which routes it into the dispatch of the stage that reads that push.

## Producers and consumers

- **The hosted scripts.**
  - Producers: `docs/install.sh` and `docs/install.ps1`, served by Pages.
  - Consumers: an adopter's `sh` or `iex`; the witness steps.
  - Every field has a reader. The pin is read by the scripts and by `check-install-pin`. `CHECKWRIGHT_VERSION` and `CHECKWRIGHT_RELEASE_BASE` are read by the scripts, set by the witness, and documented for an adopter's older-version install.
- **`check-install-pin`.**
  - Producer: the pre-commit hook and the battery.
  - Consumer: the committing session, through its finding.
  - The enabling registration is `scripts/gates.list`.
- **Roster-holding readers of the new gate name.**
  - `scripts/gates.list`.
  - `native/src/gates/mod.rs`'s dispatch.
  - The generated `scripts/git-hooks/pre-commit`, `docs/enforcement.md` and `docs/check-graph.html`, each regenerated by its freshness gate's printed command.
  - `check-gate-substrate-parity`, which reads the descriptor.
  - It is not added to `.workflow/release-declarations.md`, because it is repo-local and reds no consumer tree, like `check-release-channel-parity`, which no note declares.
- **Roster-holding readers of the new files.**
  - `check-shellcheck` and `check-portability-floor`, through the knob edits in delta 5.
  - `check-fence-command-head` reads the new `sh` fences, whose heads, `curl` and `sh`, are configured programs.
- **Point 5.** No corpus narrows. The two knob edits widen corpora.
- **Point 6.** Not obliged.

## Existing sections updated

Roster from `git grep -n "one-liner\|one recipe per system\|one recipe for macOS\|a download, a checksum and one command\|Release tarball recipe\|drain it at the tag"` and `grep -n "LINT_EXTRA_DIRS\|PORTABILITY_PATHS" scripts/gate-sdk-config.knobs`, run 2026-09-24.

- `docs/install.sh`, `docs/install.ps1` (delta 1).
- `installer/SPEC.md` §The dependency boundary and §Requirements (delta 2).
- `installer/SPEC.md` §Versioning, `RELEASING.md` step 4, `native/src/gates/install_pin.rs`, `native/src/gates/release_channel_parity.rs`, `native/src/gates/mod.rs`, `scripts/check-install-pin.gate`, `scripts/gate-tests/check-install-pin/`, `scripts/gates.list` (delta 3).
- `.github/workflows/gates.yml` (delta 4).
- `docs/install.md`, `docs/index.md`, `README.md`, `installer/README.md`, `docs/site-architecture.md`, `scripts/gate-sdk-config.knobs` (delta 5).
<!-- update-target-exempt: generated projections, each regenerated by its freshness gate's printed command -->
- `scripts/git-hooks/pre-commit`, `docs/enforcement.md`, `docs/check-graph.html`, `docs/installer/SPEC.md`, `docs/installer/README.md`.

## Retired spellings

- None — the refused one-liner was a sentence, not a governed name, and no name is renamed or deleted.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the scripts, their overrides and the pin gate.
- [ ] **Instruction surfaces: instruction only.** The page and README edits carry the command and its one-line purpose, and the grounds stay in installer/SPEC.md.
- [ ] **Merged with no information lost.** §The dependency boundary's recipe paragraph is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls installer/SPEC-*.md`).
- [ ] **Entry moved.** `install-hosted-one-liner` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work, and the first-observation probe's result if it reopens delta 1.
