---
title: Install
nav_order: 3
---

# Install and upgrade

This page gets the kits into your repository, keeps them current, and takes them out again. Checkwright is vendored: `init` copies the kit source into your tree and commits it, so what governs your repository is committed and reviewable. The gate binary is the one compiled piece, and it is checked against a published digest before anything runs.

Pick your system below. One line downloads a release, checks it against its published digest, unpacks it outside your repository, and runs `init`, with no runtime to install first. The same steps follow it one at a time. With Node on the machine, `npx checkwright init` does the same install in one command (§With Node). See the [footprint page](footprint.md) for what each kit costs your agent's context.

## Install

<!-- install-primary: tarball -->

Start from a clean git repository. The one line installs the newest release. For the step-by-step recipe, pick a version from the [releases](https://github.com/checkwright/checkwright/releases) page and put it in place of `X.Y.Z`, once, on the recipe's first line.

### macOS and Linux

You need git, `curl`, `tar`, and `sha256sum` or the `shasum` macOS ships.

On macOS, stock bash is 3.2, below the floor. If your profile owes `bash` (§Requirements), run this block. It installs Homebrew's bash and puts it first on your `PATH`, now and in `~/.zprofile`. If your login shell is bash, use `~/.bash_profile` instead:

<!-- macos-remedy:begin -->

```sh
brew install bash
echo "export PATH=\"$(brew --prefix)/bin:\$PATH\"" >> ~/.zprofile
export PATH="$(brew --prefix)/bin:$PATH"
```

<!-- macos-remedy:end -->

This block changes your machine, not your repository, so `checkwright uninstall` does not undo it. Why the floor is what it is: [installer/SPEC.md](installer/SPEC.md#requirements) and context-kit's [env-probe](context-kit/SPEC.md#binenv-probe).

Then, from your repository root:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh
```

The script is `docs/install.sh` in this repository; read it at <https://checkwright.dev/install.sh> before you pipe it. To pass arguments, end the line with `sh -s -- --profile prose`, `sh -s -- demo` or `sh -s -- uninstall`. For `uninstall`, set `CHECKWRIGHT_VERSION` to the version you installed: `curl -fsSL https://checkwright.dev/install.sh | CHECKWRIGHT_VERSION=X.Y.Z sh -s -- uninstall`.

Or step by step. Download the release:

```sh
v=X.Y.Z
cw="$(mktemp -d)"   # unpack outside the repository
curl -fsSLo "$cw/checkwright-$v.tgz" "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz"
curl -fsSLo "$cw/checkwright-$v.tgz.sha256" "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz.sha256"
```

Then verify, extract and run `init` from your repository root:

<!-- unix-install:begin -->

```sh
( cd "$cw" \
  && { sha256sum -c "checkwright-$v.tgz.sha256" || shasum -a 256 -c "checkwright-$v.tgz.sha256"; } \
  && tar -xzf "checkwright-$v.tgz" )
sh "$cw/package/bin/checkwright.sh" init   # from your repository root
```

<!-- unix-install:end -->

To verify, `git show --stat HEAD` lists everything the install brought in. Run the commands `init` prints to finish the setup.

To uninstall, `sh "$cw/package/bin/checkwright.sh" uninstall` reverses it in one commit, from the same version you installed; download it again if `$cw` is gone.

### Windows

You need Git for Windows. Then run this block in PowerShell. It puts Git's `usr\bin` and `bin` on your `PATH`, for this session and every later one:

<!-- windows-remedy:begin -->

```powershell
$git = Split-Path (Split-Path (Split-Path ((git --exec-path) -replace '/', '\')))
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ";$git\usr\bin;$git\bin", 'User')
$env:PATH = "$git\usr\bin;$git\bin;$env:PATH"
```

<!-- windows-remedy:end -->

This block changes your machine, not your repository, so `checkwright uninstall` does not undo it. Why: [installer/SPEC.md](installer/SPEC.md#requirements). PowerShell, `Get-FileHash` and `tar.exe` ship with Windows 10 and later, so there is nothing else to install.

Then, in PowerShell, from your repository root:

```powershell
irm https://checkwright.dev/install.ps1 | iex
```

The script is `docs/install.ps1` in this repository; read it at <https://checkwright.dev/install.ps1> before you pipe it. To pass arguments, run it as a script block: `& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) --profile prose`, or `demo`, or `uninstall`. For `uninstall`, set `$env:CHECKWRIGHT_VERSION` to the version you installed first.

Or step by step. Download the release, in PowerShell:

```powershell
$v = 'X.Y.Z'
$cw = Join-Path ([IO.Path]::GetTempPath()) ([IO.Path]::GetRandomFileName())
New-Item -ItemType Directory $cw | Out-Null
$ProgressPreference = 'SilentlyContinue'
$url = "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz"
Invoke-WebRequest $url -OutFile "$cw\checkwright-$v.tgz" -UseBasicParsing
Invoke-WebRequest "$url.sha256" -OutFile "$cw\checkwright-$v.tgz.sha256" -UseBasicParsing
```

Then verify, extract and run `init` from your repository root:

<!-- windows-install:begin -->

```powershell
$want = (Get-Content -Raw "$cw\checkwright-$v.tgz.sha256").Split(' ')[0]
if ((Get-FileHash "$cw\checkwright-$v.tgz" -Algorithm SHA256).Hash -ne $want) { throw 'checksum mismatch: download both files again' }
& "$env:SystemRoot\System32\tar.exe" -xzf "$cw\checkwright-$v.tgz" -C $cw
powershell -NoProfile -ExecutionPolicy Bypass -File "$cw\package\bin\checkwright.ps1" init
```

<!-- windows-install:end -->

Verify as above. To uninstall, run the same last line with `uninstall` in place of `init`.

### With Node

`npx checkwright init` runs the same `init` from the npm package, and it carries a build attestation the tarball cannot ([installer/SPEC.md](installer/SPEC.md#the-dependency-boundary)). `npx checkwright demo` runs the whole arc in a scratch repository first.

### Choosing a profile

`init` vendors the kits, writes a `gates.list` and the config files they read, records the install in `checkwright.lock`, and makes one commit. It ends by printing the commands that finish the setup; run them. Add `demo` in place of `init` to watch the whole arc — install, a green battery, one mistyped link caught, the fix — in a scratch repository first; it installs nothing.

Choose a profile with `--profile`:

- `starter` — the gate SDK on its own;
- `delegation` — adds the kits for agent sessions;
- `prose` — adds canon-kit, for a repository of documents;
- `full` — everything.

Moving to a profile that contains yours only adds. `init` refuses outside a git work tree, on a dirty worktree (`--no-commit` stages instead of committing), or when `checkwright doctor` finds a missing tool. Re-running it is safe: it reports files you have edited instead of overwriting them, unless you pass `--force`. `--dry-run` prints the plan and writes nothing.

## Managing

`checkwright <verb>` below means the one line with `<verb>` as its argument, the last line of your install recipe with `<verb>` in place of `init`, or `npx checkwright <verb>`. Each verb answers in its exit status, so a CI step can gate on it.

- `checkwright doctor` checks this machine against §Requirements and reports what is installed.
- `checkwright diff` lists the vendored files you have changed. Exit `0` means none.
- `checkwright update` upgrades the install to the version you are running.
- `checkwright uninstall` reverses the install in one commit.

`uninstall` removes only files `init` wrote and you left untouched. It keeps and reports any you edited, and never removes a file you wrote. Run it with `--dry-run` first to see the plan.

The pre-commit hook is a local backstop anyone can skip. Make the gate battery a required status check in CI, so a red battery blocks the merge, and keep that check where the authors it holds cannot edit it.

## Upgrading

Release channel: **preview**

While the channel reads `preview`, versions are `0.x`, a minor may break things, and every GitHub Release is marked pre-release, so none shows as Latest. Take a release from the releases list, or name an explicit version. Never rely on the Latest pointer.

An upgrade has two phases:

1. Run `checkwright update` from the new version, then run the commands it prints. If a kit now ships a `.knobs` config in place of a `*-config.sh`, move your settings into the `.knobs` file and delete the old one.
2. Run the full battery. The gates that go red are your worklist. The release note says why each one moved.

Every release note opens with **In brief**: a few plain bullets on what you get and whether you must act. Its **Tightened gates**, **Renamed knobs** and **Behavior changes** sections list what you reconcile. Each says "None." when there is nothing. All notes are on the [releases page](releases.md). The bump rules and the note grammar: [Versioning](installer/SPEC.md#versioning) and [The upgrade contract](installer/SPEC.md#the-upgrade-contract).

## Requirements

This section is the reference for contributors and for anyone checking a platform. Checkwright runs on Linux, macOS and native Windows, with Windows Subsystem for Linux (WSL) served by the Linux line. Install is possible only where a prebuilt gate binary is published:

<!-- platforms:begin -->

- `x86_64-unknown-linux-gnu` (joined) — Linux on x86-64. A Windows adopter who chooses WSL resolves to this triple too, so that route is served by this line rather than by one of its own.
- `aarch64-apple-darwin` (joined) — Apple silicon Macs. The gates workflow builds this target on every run, and a macOS install-smoke leg installs that build — the producer's own upload, nothing rebuilt on the smoke's host — and reaches its artifact-present branch. That pair on one run is what `native/targets.list`'s header asks for, and it is what this line rests on.
- `x86_64-apple-darwin` (joined) — Intel Macs. `macos-latest` is arm64, so the leg above measures Apple silicon and asserts nothing here; this line rests on an Intel leg of its own, which installs the Intel producer's upload on an Intel host and reaches the same artifact-present branch. The same pair on one run, asked for by the same header.
- `x86_64-pc-windows-msvc` (joined) — Windows on x86-64, natively, under Git for Windows' bash. The gates workflow builds this target on a Windows runner. A Windows install-smoke leg then installs that build — the producer's own upload, nothing rebuilt on the smoke's host — reaching its artifact-present branch. The same pair on one run the two macOS lines rest on, asked for by the same header. WSL stays a route you may choose instead, served by the Linux line.
- `aarch64-unknown-linux-gnu` (held: one `gates` run carrying both a `native-artifacts` green for this triple and a green `install-smoke-sh-linux-arm64` that consumed that upload and reached its artifact-present branch) — Linux on arm64. The installer's host detector already maps this host, so an adopter on it reaches a refusal rather than a wrong artifact; what is missing is the run.
- `aarch64-pc-windows-msvc` (held: one `gates` run carrying both a `native-artifacts` green for this triple and a green `install-smoke-sh-windows-arm64` that consumed that upload and reached its artifact-present branch) — Windows on ARM64, natively. An adopter here reaches a refusal rather than an emulated x86-64 artifact; what is missing is the run.

<!-- platforms:end -->

`joined` means a binary is published for that platform. `held` means it is supported but not published yet, and names the run that would publish it. On any other platform `init` refuses.

Put these tools on your `PATH`. A bullet marked `@contributor` is only for building Checkwright itself. A bullet naming a kit is owed only if your profile includes that kit, and `@registered` only if a gate you register needs the tool:

<!-- toolchain:begin -->

- `bash` (≥ 4.3, @context-kit+drift-kit+guard-kit) — owed only where the profile you select carries one of those kits, because each ships a file your host runs with bash. That list is **derived** from the kits themselves at probe time rather than held by hand, so a kit that later ships such a file joins it without anyone remembering to; context-kit/SPEC.md §bin/env-probe states the predicate a kit is measured against. Prose telling you to type a bash command is not that predicate, and no kit ships it. The starter and prose profiles reach none of them, until your own docs mark a fence runnable: canon-kit's `check-fence-run` then runs it with `bash`, and canon-kit joins the list from the next `doctor`. Both generated git hooks are POSIX sh, run by the `/bin/sh` git itself uses. `init` generates them and names its follow-up commands through the gate binary it placed. The unix install bootstrap (`installer/bin/checkwright.sh`) is POSIX sh too, run by the OS's own `/bin/sh`, so installing a starter or prose profile reaches no `bash` either. On native Windows, run a front-end battery from PowerShell through `gate-sdk/bin/run-gates.ps1`, the twin of `run-gates.sh`. Git for Windows' bundled bash serves guard-kit's hook there, because the harness runs both its `Bash` tool and its hook commands under that shell. The floor is the highest construct the shipped shell runs: a nameref (`local -n`) in the gate library, which every kit shell surface that reads a knob before locating the binary sources. Associative arrays, `mapfile`, and the lowercasing case expansion are more widespread but only reach 4.0. The shipped shell also assumes the POSIX userland `GATE_SDK_PROGRAM_FLOOR` names (gate-sdk/SPEC.md §lib/gate.sh). A GNU-only construct it uses is named on this page and cited by a `# portability-declared:` marker at its site, so the declaration and the enforcement are one fact with one owner — `check-portability-floor` reds a new undeclared use before it reaches this page. On Linux and macOS the gate binary reads civil dates itself rather than through GNU `date -d`.
- `git` — the gates read tracked files and the hooks fire at commit time; the model is git-native end to end.
- `jq` (@contributor) — a **contributor** requirement: guard-kit's smoke recipe and the installer's consumer smoke read JSON with it. No adopter needs it. guard-kit's hook reads the harness's JSON payload through the gate binary, so `checkwright doctor` never asks for `jq` and every profile installs without it.
- `curl` (@delegation-kit) — delegation-kit's usage poller (`--usage-poll`) fetches its source with it, and refuses by name where `curl` is absent, so `checkwright doctor` requires it only where delegation-kit is selected. Most hosts already carry it; where one does not, take it from your distribution's package, from Homebrew on macOS, or from Chocolatey on native Windows.
- `shellcheck` (@registered) — required only where a gate you register needs it, so `checkwright doctor` owes it only then and no profile's `init` asks for it. `check-shellcheck` arms when you register it for your own gate scripts under your gates directory, and `check-action-run-shell` when you register it for your workflows; either one then runs [ShellCheck](https://www.shellcheck.net/) at commit time, where a lint finding blocks the commit. Take it from your distribution's package on Linux or WSL, from Homebrew (`brew install shellcheck`) on macOS, and from Chocolatey (`choco install shellcheck`) on native Windows.
- `cargo` (≥ 1.71, @contributor) — a **contributor** requirement with **no install-time role at all**: the `native/` crate carries the gate implementations that dispatch to a binary subcommand, and the floor is the highest MSRV in the crate's resolved dependency graph. Gates in this repo now dispatch there, so a contributor builds the binary (`bash gate-sdk/bin/build-native.sh`) **before committing** — `cargo test` compiles a different artifact and does not discharge it; CI builds, lints and tests the crate every run. Installing Checkwright never builds it and never will. The binary reaches an adopter as a prebuilt Release asset for a declared target and is digest-verified before it is run. Where no asset matches your host the install is refused outright, since every step of one is behind that binary — so no install path asks you for Rust. That publish path builds and attaches those assets from the tag itself, so what any one release carries is read off its own Release page rather than asserted here. A gate on that substrate shells out to git at runtime and embeds nothing.

<!-- toolchain:end -->

`init` also needs `sha256sum` or `shasum` to verify the binary, and refuses without one; on macOS and Linux the tarball path needs `curl` and `tar` besides, and the npm path needs Node. To publish a docs site with site-kit's render-fidelity gate, you also need Ruby with the `kramdown-parser-gfm` gem. To check your machine, run the gate binary with `--emit env-probe`: it writes an untracked `ENV.local.md` with each tool's version and verdict.

## Going further

- **Vendoring by hand**, and what a compiled gate discloses: [Vendoring without the installer](installer/SPEC.md#vendoring-without-the-installer).
- **Reviewing the hook** before you install it: [Reviewing the pre-commit hook](installer/SPEC.md#reviewing-the-pre-commit-hook).
- **Running under an `AGENTS.md` harness**: [the adapter recipe](positioning.md#running-under-an-agentsmd-harness).
- **How the installer works**: [installer/SPEC.md](installer/SPEC.md).

Back to the [kit map](index.md#the-kits) or [why Checkwright](methodology.md).
