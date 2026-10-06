---
title: Requirements
nav_parent: install
nav_child_order: 1
---

# What an install requires

## Supported systems

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

## Installing and running the shipped gates

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
| Git for Windows | `git` at the floor above | required to install on Windows | it supplies `git`; `bash` only where a row above owes it |
| `gh` | any carrying `gh attestation` | optional: to check the build attestation | the one-line install runs `gh attestation verify` when `gh` is signed in, and says so when it is not |
| Node | 8.2 | optional: only to install with npx | the first Node to bundle an npm carrying `npx` |
| Ruby | 2.3 | optional: only if you register site-kit's docs gates | the two gems below need it |
| `kramdown-parser-gfm` | any | optional: only if you register site-kit's docs gates | `check-docs-render-fidelity` re-renders every page through it |
| `liquid` | 4.0.4 exactly | optional: only if you register site-kit's docs gates | `check-docs-liquid-parse` parses with the version GitHub Pages runs |

<!-- prerequisites:end -->

A missing tool comes from your distribution's package on Linux or WSL, from Homebrew on macOS, or from Chocolatey on native Windows. To check your machine, run the gate binary with `--emit env-probe`: it writes an untracked `ENV.local.md` with each tool's version and verdict. Building Checkwright itself takes more: see the [contributing guide](https://github.com/checkwright/checkwright/blob/master/CONTRIBUTING.md).

## What a commit costs

The pre-commit hook runs the gates your profile registers whose triggers match the staged files. It runs them one after another and stops at the first red. The figure runs every gate whose triggers match a file over every tracked file, in a repository the one-line install has just initialized with `full`, the profile that contains every other. A gate that reads the staged set from git itself sees nothing staged, so the figure is a reference reading, not a ceiling.

<!-- commit-cost:begin -->

On `linux x86_64` with `14` logical CPUs, the hook ran `49` pre-commit gates over every tracked file in **2141 ms**, the median of three, measured at v0.31.0.

<!-- commit-cost:end -->

To measure your own repository, run the gate binary with `--measure-commit`. It stages nothing and times the same run over every tracked file. Two costs sit outside the figure. First, the hook's own start: git starts it with no shell on any OS, and the `gates` workflow's probe step prints what that costs on each OS and architecture. Second, a gate you register yourself adds its own time.

## Writing your own shell gates

A gate you write is a copy of gate-sdk's `templates/check-skeleton.sh`. It is a bash script that sources gate-sdk's library, so it needs the `bash` floor above on every system, whatever your profile. `doctor` checks it once your `gates.list` registers the gate. On native Windows that bash is Git for Windows' bash: a gate cannot be written in PowerShell today. `check-shellcheck` lints your gate if you register it, and then `shellcheck`'s row binds you.

## Writing your own Rust gates

There is no supported path today: a compiled gate is a subcommand of the published gate binary, and an install carries no crate to add one to. A fork that adds the subcommand and ships its own build can, since descriptors resolve consumer-first and so its own `.gate` descriptor runs. That fork then owns its own release, digest and upgrades, outside this project's. [gate-sdk/SPEC.md](gate-sdk/SPEC.md#the-port-candidate-criteria) owns this.
