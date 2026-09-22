# SPEC amendment: install-per-os

An adopter's first contact assumes a developer toolchain. `docs/index.md`'s quick-try block leads with `npx checkwright`, while `docs/install.md` calls the Release tarball the primary path because it adds no runtime dependency, so the landing page contradicts the install page and gate-sdk/SPEC.md §The adopter constraints. `docs/install.md` opens with target triples and release-leg status, spreads its prerequisites across per-profile tags, and gives one bash quick start. A native Windows adopter has no recipe at all except npx.

**The ruling: per-OS native recipes, with no piped script.** Operator direction (2026-09-22, lead session): this iteration writes one recipe for macOS and Linux and one for Windows. Each recipe is the four steps installer/SPEC.md §The dependency boundary already rules: download, verify, extract, run. npx becomes the alternative for a reader who has Node, and `docs/index.md` leads with the no-Node path. The hosted one-line bootstrap is filed as the Deferred entry `install-hosted-one-liner`, and installer/SPEC.md's refusal of a `curl … | sh` one-liner stays in force.

**Where the contributor facts go (spec's call).** The platform and toolchain blocks stay on `docs/install.md`, under a `## Requirements` section moved to the end of the page as a reference. Three readers parse the platform block from that file: `native/src/gates/install_platforms.rs`, whose default path is `docs/install.md`; `native/src/gates/install_toolchain.rs`, which reads the toolchain block; and the `native-artifacts-roster` job in `.github/workflows/gates.yml`. So moving the blocks to another page would move three readers for a reader-facing gain that the reordering already buys. Keeping the heading also keeps every `docs/install.md §Requirements` citation that names those blocks resolving. The two remedy blocks are adopter actions, so they move into their OS sections.

## What changes

### (1) `docs/install.md`: the adopter path first, one section per OS {design-bearing}

**Applied.** Rewrite the page in this order. Headings are fixed, and prose is proposed text that build may tighten.

1. **The opening paragraph** keeps its first three sentences (vendored, committed, the one compiled piece). Replace the second paragraph with: "Pick your system below. Each path downloads a release, checks it against its published digest, unpacks it outside your repository, and runs `init`, with no runtime to install first. With Node on the machine, `npx checkwright init` does the same install in one command (§With Node)." Keep the footprint-page sentence.
2. **`## Install`**, carrying the `<!-- install-primary: tarball -->` declaration (moved from §Quick start), then: "Start from a clean git repository. Pick a version from the [releases](https://github.com/checkwright/checkwright/releases) page and put it in place of `X.Y.Z`, once, on the recipe's first line." Then its subsections:
   - **`### macOS and Linux`**. Prerequisites: git, `curl`, `tar`, and `sha256sum` or the `shasum` macOS ships. Then the macOS bash floor: the sentence introducing the `macos-remedy` block, and the block itself with its markers unchanged, moved from §Requirements. Install: a fetch fence, then the `unix-install` block (delta 5), both `sh`:

     ```sh
     v=X.Y.Z
     cw="$(mktemp -d)"   # unpack outside the repository
     curl -fsSLo "$cw/checkwright-$v.tgz" "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz"
     curl -fsSLo "$cw/checkwright-$v.tgz.sha256" "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz.sha256"
     ```

     ```sh
     ( cd "$cw" \
       && { sha256sum -c "checkwright-$v.tgz.sha256" || shasum -a 256 -c "checkwright-$v.tgz.sha256"; } \
       && tar -xzf "checkwright-$v.tgz" )
     sh "$cw/package/bin/checkwright.sh" init   # from your repository root
     ```

     Verify: "`git show --stat HEAD` lists everything the install brought in. Run the commands `init` prints to finish the setup." Uninstall: "`sh "$cw/package/bin/checkwright.sh" uninstall` reverses it in one commit, from the same version you installed; download it again if `$cw` is gone."
   - **`### Windows`**. Prerequisites: Git for Windows, then the sentence introducing the `windows-remedy` block and the block itself, markers unchanged, moved from §Requirements. PowerShell, `Get-FileHash` and `tar.exe` ship with Windows 10 and later, so there is nothing else to install. Install, in PowerShell: a fetch fence, then the `windows-install` block (delta 5):

     ```powershell
     $v = 'X.Y.Z'
     $cw = Join-Path ([IO.Path]::GetTempPath()) ([IO.Path]::GetRandomFileName())
     New-Item -ItemType Directory $cw | Out-Null
     $ProgressPreference = 'SilentlyContinue'
     $url = "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz"
     Invoke-WebRequest $url -OutFile "$cw\checkwright-$v.tgz" -UseBasicParsing
     Invoke-WebRequest "$url.sha256" -OutFile "$cw\checkwright-$v.tgz.sha256" -UseBasicParsing
     ```

     ```powershell
     $want = (Get-Content -Raw "$cw\checkwright-$v.tgz.sha256").Split(' ')[0]
     if ((Get-FileHash "$cw\checkwright-$v.tgz" -Algorithm SHA256).Hash -ne $want) { throw 'checksum mismatch: download both files again' }
     & "$env:SystemRoot\System32\tar.exe" -xzf "$cw\checkwright-$v.tgz" -C $cw
     powershell -NoProfile -ExecutionPolicy Bypass -File "$cw\package\bin\checkwright.ps1" init
     ```

     Verify as above. Uninstall: the same last line with `uninstall` in place of `init`.
   - **`### With Node`**: "`npx checkwright init` runs the same `init` from the npm package, and it carries a build attestation the tarball cannot (installer/SPEC.md §The dependency boundary)." Also `npx checkwright demo`.
   - **`### Choosing a profile`**: the profile list and the refusal and re-run paragraph from today's §Quick start, unchanged. The `demo` sentence moves here too: "Add `demo` in place of `init` to watch the whole arc in a scratch repository first; it installs nothing."
3. **`## Managing`** opens with: "`checkwright <verb>` below means the last line of your install recipe with `<verb>` in place of `init`, or `npx checkwright <verb>`." The rest is unchanged.
4. **`## Upgrading`**, unchanged.
5. **`## Requirements`**, moved here and retitled in prose (not heading) as the reference for contributors and for anyone checking a platform. It keeps the platforms block with its sentence and legend, the toolchain block with its sentence, the hasher and docs-site Ruby sentence, and the `--emit env-probe` self-check. The last paragraph, "Both blocks change your machine…", moves with the two remedy blocks, into the OS section that holds each.
6. **`## Going further`**, unchanged.

**Ran at build:** `check-install-claim` passes on the new `## Install` section. It redded installer/README.md §Quick start once, on the hyphenated "Release-tarball" this delta's text proposed, which the transport vocabulary does not match; the landed sentence spells "Release tarball".

### (2) `docs/index.md`: the landing page leads without Node {mechanical}

**Applied.** Replace the paragraph beginning "Try it with Node on the machine" and its fence with:

> Install it with no runtime first: [macOS and Linux](install.md#macos-and-linux) or [Windows](install.md#windows), each a download, a checksum and one command from the root of a clean repository. `demo` in place of `init` runs the whole arc in a scratch repository of its own and removes it, and `uninstall` takes the install back out.
>
> With Node on the machine, the same arc is one command each:

followed by today's fence, unchanged.

### (3) `installer/README.md`: point at the per-OS recipes {mechanical}

**Applied.** In §Before you run it, rewrite the transport sentence as: "The Release tarball needs `curl`, `tar` and either hasher (`sha256sum`, or the `shasum` stock macOS ships instead) on macOS and Linux, and nothing Windows 10 and later does not ship on Windows; npm needs Node, for `npx`." In §Quick start, replace the recipe fence and the paragraph above it with: "From a clean git repository, at its root, follow the Release-tarball recipe for your system on the install page: [macOS and Linux](https://checkwright.dev/install.html#macos-and-linux) or [Windows](https://checkwright.dev/install.html#windows). It unpacks outside the repository, because `init` refuses a worktree that is not clean." The npx sentence and the paragraphs after it stay. A second copy of each recipe here is what this delta removes: installer/README.md already restated the one recipe the page carried.

### (4) installer/SPEC.md: two recipes on one shape {design-bearing}

**Applied.** In §The dependency boundary, rewrite the paragraph beginning "**The tarball recipe's shape.**" as:

> **The tarball recipe's shape.** The install page carries one recipe per system, macOS and Linux in `sh` and Windows in PowerShell, and each is four steps — download, verify, extract, run — rather than a `curl … | sh` or `irm … | iex` one-liner, because an unreviewed remote script fed straight to a shell is the counter-pattern of the claim the page opens with. Each unpacks outside the repository, because `init` refuses a worktree that is not clean. The version is set once, on the recipe's first line. The Windows recipe names `tar.exe` under the system directory, because the `windows-remedy` block puts Git's `usr\bin` ahead of it on `PATH`, and Git's GNU `tar` reads a drive-letter path as a remote host. It runs the bootstrap under `powershell -ExecutionPolicy Bypass`, scoped to that one process, because Windows' default policy refuses a downloaded script. The `package/` prefix is `npm pack`'s doing: npm builds the asset on the release runner, and consuming it needs no Node. **The checksum's honest limit:** it travels from the same origin over the same encrypted session as the tarball, so verifying it catches a corrupted or truncated download and is no evidence that the release host was uncompromised; the property carrying that is a build attestation, which the npm channel's `--provenance` mints and this channel does not.

In §Requirements, rewrite "Fetched as the tarball attached to a GitHub Release it needs none — `curl` (or `wget`), `tar`, and either hasher, `sha256sum` or the `shasum` stock macOS ships instead, then `sh package/bin/checkwright.sh init`." as:

> Fetched as the tarball attached to a GitHub Release it needs none: on macOS and Linux `curl`, `tar` and either hasher, `sha256sum` or the `shasum` stock macOS ships instead; on Windows only what Windows 10 and later ships, PowerShell's `Invoke-WebRequest` and `Get-FileHash` and the system `tar.exe`.

In the paragraph beginning "**The install page's requirement blocks, and why each reads as it does.**", replace "The page renders four marker blocks bare" with "The page renders six marker blocks bare: the platform and toolchain blocks in its closing §Requirements, and in each OS section that system's remedy block and its install block". Then add a bullet after **The Windows remedy.**:

> - **The install blocks.** Each OS section's recipe is split at the network: a fetch fence that only a published release can serve, and a marked install block — verify, extract, run — that the install-smoke legs run verbatim against a packed tarball placed where the fetch would have put it. So everything after the download is measured on the host it is written for, and the two download lines are the part left to the release. The Windows block runs the bootstrap under Windows PowerShell 5.1, the one PowerShell every Windows host carries, and its leg is what holds the bootstrap to that host.

### (5) The two install blocks and their witnesses {design-bearing}

**Applied.** Wrap each install fence from delta 1 in a marker pair, `<!-- unix-install:begin -->` / `<!-- unix-install:end -->` and `<!-- windows-install:begin -->` / `<!-- windows-install:end -->`, on the remedy blocks' grammar (one fence per pair, markers on their own lines).

- **The baseline Linux leg** (`install-smoke` today) gains a step after its pack. It makes a clean scratch consumer, sets `v` to the packed version and `cw` to a scratch directory, and copies the packed tarball there as `checkwright-$v.tgz` beside a `.sha256` written by `sha256sum` in the release job's format. It extracts the `unix-install` block by its markers, the way the PowerShell leg extracts `windows-remedy`, reds by name on an empty extraction, and runs the block under `sh` from the consumer's root. It asserts exit 0 and a `checkwright.lock` in the consumer.
- **The PowerShell bootstrap's leg** (`install-smoke-powershell` today) gains the same step after its existing init, reading the `windows-install` block and running it under `pwsh` with `$v` and `$cw` set. The block's own last line starts Windows PowerShell 5.1. It asserts the same two things.

The act depends on the sibling entry `install-smoke-leg-names-mix-two-axes`:

- **If it lands in the same build batch or earlier,** the steps land in the renamed jobs, `install-smoke-sh-linux` and `install-smoke-pwsh-windows`, and every mention this delta adds uses those names.
- **If it lands later,** the steps land in `install-smoke` and `install-smoke-powershell`, and that unit's rename sweep carries them.

Add a row for the two install blocks to `docs/site-architecture.md` §Generated projections and their freshness gates, beside the remedy-block row. It names each block's reader leg and the verbatim-run contract, and it states that the fetch fences have no reader.

**Inferred, cannot run before build:** `installer/bin/checkwright.ps1` completes `init` under Windows PowerShell 5.1 — only the PowerShell leg on a Windows runner can run it, after this delta lands. If the leg shows it cannot, the Windows block starts `pwsh` instead, and the Windows section names PowerShell 7 as a prerequisite.

**Inferred, cannot run before build:** the system `tar.exe` extracts the payload tarball intact on `windows-latest` — measured by the same leg step.

### (6) The install-failure template names both recipes {mechanical}

**Applied.** In `.github/ISSUE_TEMPLATE/install-failure.yml`, the Transport dropdown's `Release tarball (curl, sha256sum, tar)` option becomes two:

```
        - Release tarball, macOS or Linux (curl, sha256sum or shasum, tar)
        - Release tarball, Windows (PowerShell)
```

### (7) Retarget the remedy-block citations {mechanical}

**Applied.** Every tracked citation of `docs/install.md §Requirements` that names a remedy block, rather than the platform or toolchain block, is retargeted to the OS section now holding it (`§macOS and Linux` or `§Windows`). The members, from `git grep -n "install.md §Requirements"` and `grep -n "Requirement" docs/site-architecture.md` read line by line: the comments at `.github/workflows/gates.yml` lines 382, 1246, 1268 and 1582-1584, and the remedy-block row of `docs/site-architecture.md` (line 58, "Requirements section carries two more marker blocks"). The other `§Requirements` citations name the platform block, the toolchain block or the bash floor, and they still resolve.

## Producers and consumers

- **The recipes** (deltas 1 and 3). Produced by `docs/install.md`. Consumed by the adopter, by `check-install-claim` through the moved declaration and the `## Install` section (`CANON_KIT_INSTALL_SECTION_RE` matches `Install`), and by `check-fence-command-head` for the two `sh` fences, whose heads are `v=`, `cw=`, `curl`, `(` and `sh`, all admitted today. The PowerShell fences are outside that gate's `bash`/`sh`/`shell` reach, and delta 5's legs are their oracle.
- **The two install blocks** (delta 5). New marker names. Produced by `docs/install.md`; consumed by the two leg steps, which extract them by marker. The roster holder is `docs/site-architecture.md` §Generated projections and their freshness gates (the new row). Each block's fields, `$v`/`v` and `$cw`/`cw`, are read by the block's own lines and set by the fetch fence or by the leg.
- **The moved blocks.** The platform and toolchain blocks keep their markers and their file. `install_platforms.rs` and `install_toolchain.rs` find them by an exact-line match on the begin marker, and the roster job's awk by `/^<!-- platforms:begin -->$/` (read 2026-09-22), so none of the three keys on position or section heading.
- **The remedy blocks.** They keep their markers and move within the file. The legs that run them extract by marker over the whole file (`.github/workflows/gates.yml` lines 783-789 for `windows-remedy`), so the move changes no reader.
- **The Deferred entry** `install-hosted-one-liner`. Filed in the promoting commit on the operator's direction, with its cost and surface tags; its reader is a later scope.
- **Point 5.** No corpus narrows.
- **Point 6.** Delta 7 obliges every remedy-block citation; the members are enumerated there, each retargeted to its OS section.

## Existing sections updated

Roster from `git grep -n -e "install.md §Requirements" -e "npx checkwright" -e "Quick start" -e "remedy:begin" -- . ':!docs/posts' ':!TASK-QUEUE.md'`, run 2026-09-22 and read line by line.

- `docs/install.md` — the whole page (deltas 1 and 5).
- `docs/index.md` — the quick-try paragraph (delta 2).
- `installer/README.md` — §Before you run it and §Quick start (delta 3).
- `installer/SPEC.md` — §The dependency boundary's recipe paragraph and §Requirements (delta 4).
- `.github/workflows/gates.yml` — the two leg steps (delta 5) and the remedy-block comments (delta 7).
- `docs/site-architecture.md` — the install-block row (delta 5) and the remedy-block row (delta 7).
- `.github/ISSUE_TEMPLATE/install-failure.yml` — the Transport dropdown (delta 6).
- `docs/installer/SPEC.md`, the generated mirror (delta 4).
- `docs/installer/README.md`, the generated mirror (delta 3).

## Retired spellings

- None — no delta retires a name; §Quick start's heading leaves `docs/install.md`, and no tracked link targets its anchor (`git grep -n -E "install(\.md|\.html)#"` returns nothing).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the two install blocks and the reordered page.
- [ ] **Instruction surfaces: instruction only.** The page carries steps; the grounds sit in installer/SPEC.md.
- [ ] **Merged with no information lost.** installer/SPEC.md §The dependency boundary and §Requirements read as one document.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `install-path-developer-first` moves to Done in the merge commit, at a stage before the drain stage, once the watched push its `[observed-by:]` tag names shows both install-block steps green.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
