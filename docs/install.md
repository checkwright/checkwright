---
title: Install
nav_id: install
nav_order: 3
nav_suffix: spec=installer/SPEC.md plugin=plugin/SPEC.md
---

# Install and upgrade

This page gets the kits into your repository, keeps them current, and takes them out again. Checkwright is vendored: `init` copies the kit source into your tree and commits it, so what governs your repository is committed and reviewable. The gate binary is the one compiled piece, and it is checked against a published digest before anything runs. The [footprint page](footprint.md) prices each kit in agent context.

## Requirements

An install is possible only on a system below, where a prebuilt gate binary is published. Every joined system is built and installed on every push, on the runner images [`native/runners.list`](https://github.com/checkwright/checkwright/blob/master/native/runners.list) names.

<!-- platforms:begin -->

| System | Minimum | Binary | Status |
|---|---|---|---|
| Linux on x86-64 (glibc), and WSL | glibc 2.39 | `x86_64-unknown-linux-gnu` | joined |
| Linux on arm64 (glibc) | glibc 2.39 | `aarch64-unknown-linux-gnu` | joined |
| Linux on x86-64 (any C library) | Linux 3.2 | `x86_64-unknown-linux-musl` | joined |
| Linux on arm64 (any C library) | Linux 4.1 | `aarch64-unknown-linux-musl` | joined |
| macOS on Apple silicon | macOS 11 | `aarch64-apple-darwin` | joined |
| macOS on Intel | macOS 10.12 | `x86_64-apple-darwin` | joined |
| Windows on x86-64 | Windows 10 | `x86_64-pc-windows-msvc` | joined |
| Windows on ARM64 | Windows 10 | `aarch64-pc-windows-msvc` | joined |

<!-- platforms:end -->

`joined` means the current release publishes a binary for that system, at the floor shown; `held` means it is supported but not published yet, and names the run that would publish it. On any other system `init` refuses. On Linux, `init` takes the glibc binary where it runs and the static one everywhere else. Windows Subsystem for Linux (WSL) takes the Linux binary.

### Installing and running the shipped gates

The tools each shipped gate runs. `required` binds every install. An `optional` row binds you only under the condition it names:

<!-- toolchain:begin -->

| Tool | Version | Needed | Why |
|---|---|---|---|
| `bash` | ≥ 4.3 | optional: if your profile includes context-kit, drift-kit or guard-kit | each ships a file your host runs with bash, and the gate library uses a nameref (`local -n`) |
| `git` | ≥ 2.15 | required | the gates read tracked files and the hooks fire at commit time; `check-docs-cmd` reads `rev-parse --is-shallow-repository` |
| `curl` | ≥ 5.9 | optional: if your profile includes delegation-kit | its usage poller (`--usage-poll`) fetches its source with `curl -fsS`, and `-S` arrived in 5.9 |
| `shellcheck` | ≥ 0.6 | optional: if you register a gate that runs it | `check-shellcheck` and `check-action-run-shell` run [ShellCheck](https://www.shellcheck.net/) with `-S warning` once you register them |

<!-- toolchain:end -->

The install itself and the optional docs gates need these as well. `doctor` does not probe them:

<!-- prerequisites:begin -->

| Tool | Minimum | Needed for | Why |
|---|---|---|---|
| `sh` | any POSIX `sh` | required to install on Linux and macOS | the one-line install and the bootstrap are POSIX sh throughout |
| `curl` | 5.9 | required to install on Linux and macOS | the install line fetches with `curl -fsSL`, and `-S` arrived in 5.9 |
| `tar` | any GNU or BSD `tar` | required to install on Linux and macOS | `tar -xzf` unpacks the release tarball |
| `sha256sum` or `shasum` | any | required to install on Linux and macOS | the release tarball and the gate binary are checked against their published digests |
| Windows PowerShell | 5.1 | required to install on Windows | the one-line install, the install block and the bootstrap run under it; its `Get-FileHash` checks the digests |
| `tar.exe` | Windows 10 version 1803 | required to install on Windows | the install block unpacks the tarball with `System32\tar.exe` |
| Git for Windows | `git` and `bash` at the floors above | required to install on Windows | it supplies `git`, and the sh it bundles, which runs the pre-commit hook; `bash` where a row above owes it |
| Node | 8.2 | optional: only to install with npx | the first Node to bundle an npm carrying `npx` |
| Ruby | 2.3 | optional: only if you register site-kit's docs gates | the two gems below need it |
| `kramdown-parser-gfm` | any | optional: only if you register site-kit's docs gates | `check-docs-render-fidelity` re-renders every page through it |
| `liquid` | 4.0.4 exactly | optional: only if you register site-kit's docs gates | `check-docs-liquid-parse` parses with the version GitHub Pages runs |

<!-- prerequisites:end -->

A missing tool comes from your distribution's package on Linux or WSL, from Homebrew on macOS, or from Chocolatey on native Windows. To check your machine, run the gate binary with `--emit env-probe`: it writes an untracked `ENV.local.md` with each tool's version and verdict. Building Checkwright itself takes more: see the [contributing guide](https://github.com/checkwright/checkwright/blob/master/CONTRIBUTING.md).

### What a commit costs

The pre-commit hook runs the gates your profile registers whose triggers match the staged files. It runs them one after another and stops at the first red. The figure runs every gate whose triggers match a file over every tracked file, in a repository the one-line install has just initialized with `full`, the profile that contains every other. A gate that reads the staged set from git itself sees nothing staged, so the figure is a reference reading, not a ceiling.

<!-- commit-cost:begin -->

On `linux x86_64` with `14` logical CPUs, the hook ran `49` pre-commit gates over every tracked file in **2141 ms**, the median of three, measured at v0.31.0.

<!-- commit-cost:end -->

To measure your own repository, run the gate binary with `--measure-commit`. It stages nothing and times the same run over every tracked file. Two costs sit outside the figure. First, the hook starts through `sh`, whose start cost is unmeasured; on Windows that is the `sh` Git for Windows bundles, which runs emulated on Arm. Second, a gate you register yourself adds its own time.

### Writing your own shell gates

A gate you write is a copy of gate-sdk's `templates/check-skeleton.sh`. It is a bash script that sources gate-sdk's library, so it needs the `bash` floor above on every system, whatever your profile. `doctor` checks it once your `gates.list` registers the gate. On native Windows that bash is Git for Windows' bash: a gate cannot be written in PowerShell today. `check-shellcheck` lints your gate if you register it, and then `shellcheck`'s row binds you.

### Writing your own Rust gates

There is no supported path today: a compiled gate is a subcommand of the published gate binary, and an install carries no crate to add one to. A fork that adds the subcommand and ships its own build can, since descriptors resolve consumer-first and so its own `.gate` descriptor runs. That fork then owns its own release, digest and upgrades, outside this project's. [gate-sdk/SPEC.md](gate-sdk/SPEC.md#the-port-candidate-criteria) owns this.

## Install

<!-- install-primary: tarball -->

Run the one line for your system from your repository's root, in a new project or an existing one with every change committed: `init` makes one commit and refuses a dirty worktree. The line downloads the newest Release tarball, checks it against its published digest, unpacks it outside your repository and runs `init`, with no runtime to install first. A step-by-step form sits under each line. For it, pick a version from the [releases](https://github.com/checkwright/checkwright/releases) page and put it in place of `X.Y.Z` on its first line. With Node, `npx checkwright init` does the same ([With Node](#with-node)).

Pick your route:

- **macOS or Linux:** [the one line and its steps](#macos-and-linux).
- **Windows:** [natively, in PowerShell](#windows).
- **Windows through WSL:** the [macOS and Linux](#macos-and-linux) line, in your WSL shell. The install places the Linux gate binary and the pre-commit hook runs it, so commit to that repository from WSL too: Git for Windows cannot start the Linux binary.
- **With Node:** [`npx checkwright init`](#with-node).
- **In Claude Code:** [the plugin marketplace](#from-a-plugin-marketplace).
- **With another coding agent:** [a prompt to give it](#with-another-coding-agent).

### Try it first

`demo` in place of `init` runs the whole arc in a scratch repository of its own and removes it, without touching yours: it installs the `full` profile, runs the battery green, commits a task marked done with no evidence behind it and shows the claim caught, then withdraws the claim and runs green again. Its spellings are `sh -s -- demo` on the macOS and Linux line or the script-block form with `demo` on Windows; with Node, `npx checkwright demo`. `full` owes `bash` 4.3 or later ([Requirements](#requirements)). On stock macOS run the Homebrew bash block below first; on Windows, the `PATH` block.

### macOS and Linux

**You need** the tools [Requirements](#requirements) lists for installing on Linux and macOS.

On macOS, stock bash is 3.2, below the floor. If your profile owes `bash` (Requirements), run this block, which installs Homebrew's bash and puts it first on your `PATH`, now and in `~/.zprofile`. If your login shell is bash, use `~/.bash_profile` instead. The floor's grounds are in [installer/SPEC.md](installer/SPEC.md#requirements) and context-kit's [env-probe](context-kit/SPEC.md#binenv-probe).

<!-- macos-remedy:begin -->

```sh
brew install bash
echo "export PATH=\"$(brew --prefix)/bin:\$PATH\"" >> ~/.zprofile
export PATH="$(brew --prefix)/bin:$PATH"
```

<!-- macos-remedy:end -->

Then, from your repository root:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh
```

The script is `docs/install.sh` in this repository; read it at <https://checkwright.dev/install.sh> before you pipe it. To pass arguments, name the verb first, since the line runs `init` only when given none: end the line with `sh -s -- init --profile prose`, `sh -s -- demo` or `sh -s -- uninstall`.

<details markdown="1">
<summary>Step by step</summary>

Download the release:

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

To uninstall, `sh "$cw/package/bin/checkwright.sh" uninstall` reverses it in one commit. If `$cw` is gone, download any release, as [Managing](#managing) says.

</details>

### Windows

This is the native Windows route. Under WSL, take [macOS and Linux](#macos-and-linux).

**You need** the tools [Requirements](#requirements) lists for installing on Windows, where the minimum Windows version is stated too. Git for Windows' bash is also what runs any shell gate you write ([Writing your own shell gates](#writing-your-own-shell-gates)).

The pre-commit hook needs no `bash` on your `PATH`: git runs it through the sh Git for Windows bundles, and it hands off to the gate binary. The battery needs no shell: run the gate binary with `--run`, or `gate-sdk/bin/run-gates.ps1` from PowerShell. On Windows on Arm, Git for Windows' sh runs under emulation, so each commit pays one emulated start.

Run this block in PowerShell first. It puts Git's `usr\bin` and `bin` on your `PATH`, for this session and every later one:

<!-- windows-remedy:begin -->

```powershell
$git = Split-Path (Split-Path (Split-Path ((git --exec-path) -replace '/', '\')))
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ";$git\usr\bin;$git\bin", 'User')
$env:PATH = "$git\usr\bin;$git\bin;$env:PATH"
```

<!-- windows-remedy:end -->

Then, in PowerShell, from your repository root:

```powershell
irm https://checkwright.dev/install.ps1 | iex
```

The script is `docs/install.ps1` in this repository; read it at <https://checkwright.dev/install.ps1> before you pipe it. To pass arguments, run it as a script block, the verb first: `& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --profile prose`, or `demo`, or `uninstall`.

<details markdown="1">
<summary>Step by step</summary>

Download the release, in PowerShell:

```powershell
$v = 'X.Y.Z'
$cw = Join-Path ([IO.Path]::GetTempPath()) ([IO.Path]::GetRandomFileName())
New-Item -ItemType Directory $cw | Out-Null
$ProgressPreference = 'SilentlyContinue'
$url = "https://github.com/checkwright/checkwright/releases/download/v$v/checkwright-$v.tgz"
Invoke-WebRequest $url -OutFile "$cw\checkwright-$v.tgz" -UseBasicParsing
Invoke-WebRequest "$url.sha256" -OutFile "$cw\checkwright-$v.tgz.sha256" -UseBasicParsing
```

Then check the digest, unpack, and run `init`, still from your repository root:

<!-- windows-install:begin -->

```powershell
$want = (Get-Content -Raw "$cw\checkwright-$v.tgz.sha256").Split(' ')[0]
if ((Get-FileHash "$cw\checkwright-$v.tgz" -Algorithm SHA256).Hash -ne $want) { throw 'checksum mismatch: download both files again' }
& "$env:SystemRoot\System32\tar.exe" -xzf "$cw\checkwright-$v.tgz" -C $cw
powershell -NoProfile -ExecutionPolicy Bypass -File "$cw\package\bin\checkwright.ps1" init
```

<!-- windows-install:end -->

Verify as above. To uninstall, run the `init` line with `uninstall` in place of `init`.

</details>

### With Node

`npx checkwright init` runs the same `init` from the npm package, and it carries a build attestation the tarball cannot ([installer/SPEC.md](installer/SPEC.md#the-dependency-boundary)). `init` verifies the gate binary with `sha256sum` or `shasum`, and refuses without one. It needs Node 8.2 or later ([Requirements](#requirements)).

### From a plugin marketplace

In Claude Code, add Checkwright's marketplace and install its plugin:

```text
/plugin marketplace add checkwright/checkwright
/plugin install checkwright@checkwright
```

The plugin installs no kit: it registers Checkwright's lifecycle skills and its guards. Ask for its `install` skill in your repository: it chooses a profile with you and runs the same `init` this page documents. Then ask for its `adopt` skill: it fits the knobs to your layout, and works through the first reds with you until the battery is green or each has a disposition you accept. Any Agent Plugins client loads the repository's `plugin/` directory by its own install route ([plugin/README.md](plugin/README.md)).

### With another coding agent

Give your agent this prompt:

```text
Install Checkwright in this repository. Choose a profile with me from https://checkwright.dev/install.html#choosing-a-profile, then install it as https://checkwright.dev/install.html#install describes for this host, and run each command init prints. Then follow gate-sdk/templates/adopt.md.
```

The last step is the same walk the plugin's `adopt` skill runs.

### Choosing a profile

`init` vendors the kits, writes a `gates.list` and the config files they read, records the install in `checkwright.lock`, and makes one commit. It ends by printing the commands that finish the setup; run them.

Choose a profile with `--profile`. If another toolkit writes your specs, pass `--recipe` with the toolkit's name, as [Spec toolkits](spec-toolkits.md) shows. With none, a first install takes `starter` and a re-run keeps the profile the install recorded:

- `starter` — the gate SDK on its own;
- `delegation` — adds the kits for agent sessions;
- `prose` — adds canon-kit, for a repository of documents;
- `full` — everything.

Moving to a profile that contains yours only adds. `init` refuses outside a git work tree, on a dirty worktree (`--no-commit` stages instead of committing), or when `checkwright doctor` finds a missing tool. Re-running it is safe: it reports files you have edited instead of overwriting them, unless you pass `--force`. `--dry-run` prints the plan and writes nothing.

**Choosing kits and gates.** `--with-kit` and `--without-kit` add a kit to your profile or take one out. `--with-gate` and `--without-gate` do the same for a gate. Each repeats. `checkwright.lock` records them, so a re-run keeps them until you pass new ones or `--no-selection`. `gate-sdk` stays, since it runs the others. To replace a gate with your own, put a gate of the same name in your gates directory. It runs instead of the kit's, and `init` never overwrites it.

## Managing

`checkwright <verb>` below means the one line with `<verb>` as its argument (`sh -s -- <verb>` on macOS and Linux, the script-block form on Windows, since `irm … | iex` takes none), your install recipe's `init` line with `<verb>` in place of `init`, or `npx checkwright <verb>`. Each verb answers in its exit status, so a CI step can gate on it.

- `checkwright doctor` checks this machine against Requirements and reports what is installed.
- `checkwright diff` lists the vendored files you have changed. Exit `0` means none.
- `checkwright update` upgrades the install to the version you are running.
- `checkwright uninstall` reverses the install in one commit.

`uninstall` needs no version: any release that reads your `checkwright.lock` runs it, so the one line reverses an install an older release made. `uninstall` removes only files `init` wrote and you left untouched. It keeps and reports any you edited, and never removes a file you wrote. Run it with `--dry-run` first to see the plan. A remedy block changes your machine and not your repository, so `uninstall` leaves it in place.

### Requiring the CI check

The pre-commit hook is a local backstop anyone can skip. `init` also commits `.github/workflows/gates.yml`, which runs the battery on every push and pull request at the release you installed and marks each red on the pull request. Make its check required in your branch protection, so a red battery blocks the merge, and keep the workflow file where the authors it checks cannot edit it.

## Upgrading

Release channel: **preview**

While the channel reads `preview`, versions are `0.x`, a minor may break things, and every GitHub Release is marked pre-release, so none shows as Latest. Take a release from the releases list, or name an explicit version. Never rely on the Latest pointer.

An upgrade has two phases:

1. Run `checkwright update` from the new version, then run the commands it prints. If a kit now ships a `.knobs` config in place of a `*-config.sh`, move your settings into the `.knobs` file and delete the old one.
2. Run the full battery. The gates that go red are your worklist. The release note says why each one moved.

Every release note opens with **In brief**: a table linking each section below with its entry count and who acts on it, then a few plain bullets on what you get and whether you must act. Its **New and tightened gates**, **Knob changes**, **Gate-authoring changes**, **Platforms** and **Behavior changes** sections list what you reconcile. Each says "None." when there is nothing. All notes are on the [releases page](releases.md). The bump rules and the note grammar: [Versioning](installer/SPEC.md#versioning) and [The upgrade contract](installer/SPEC.md#the-upgrade-contract).

An installed tree checks for a newer release once a week with `git ls-remote`, which shows the upstream host your IP address, and `doctor` and the session-context hook say when one exists. Set `GATE_SDK_UPDATE_CHECK = off` in your gates directory's `gate-sdk-config.knobs` to turn it off.

## Going further

- **Vendoring by hand**, and what a compiled gate discloses: [Vendoring without the installer](installer/SPEC.md#vendoring-without-the-installer).
- **Reviewing the hook** before you install it: [Reviewing the pre-commit hook](installer/SPEC.md#reviewing-the-pre-commit-hook).
- **Running under an `AGENTS.md` harness**: [the adapter recipe](positioning.md#running-under-an-agentsmd-harness).
- **How the installer works**: [installer/SPEC.md](installer/SPEC.md).

Back to the [kit map](index.md#the-kits) or [why Checkwright](methodology.md).
