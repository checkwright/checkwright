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
| `bash` | ≥ 4.3 | optional: if your docs mark a fence runnable or you register a shell gate | canon-kit's `check-fence-run` runs a marked fence with bash, and a shell gate sources the gate library, which uses a nameref (`local -n`) |
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
| Git for Windows | `git` at the floor above | required to install on Windows | it supplies `git`, and once the install page's `PATH` block has run, the `sh` and `sha256sum` `npx` runs the bootstrap with; `bash` only where a row above owes it |
| `gh` | any carrying `gh attestation` | optional: to check the build attestation | the one-line install runs `gh attestation verify` when `gh` is signed in, and says so when it is not |
| `jq` | any | optional: to read the attestation check by hand on Linux and macOS | the manual verify step prints what `gh` verified through `jq`; the Windows block reads it without `jq` |
| Node | 8.2 | optional: only to install with npx | the first Node to bundle an npm carrying `npx` |
| Ruby | 2.3 | optional: only if you register site-kit's docs gates | the two gems below need it |
| `kramdown-parser-gfm` | any | optional: only if you register site-kit's docs gates | `check-docs-render-fidelity` re-renders every page through it |
| `liquid` | 4.0.4 exactly | optional: only if you register site-kit's docs gates | `check-docs-liquid-parse` parses with the version GitHub Pages runs |

<!-- prerequisites:end -->

The hook wiring the kits ship for Claude Code names the gate binary as the hook's `command` and passes `--hook` and the member in an `args` array, so the harness starts it with no shell. That takes **Claude Code 2.1.139 or later**. The floor is read from Claude Code's changelog, whose entry for that version adds the hook `args` field, and is not run here: the wiring was run on 2.1.292, and no build below the floor was tried. What an older build does with a registration carrying `args` is undocumented, so do not merge the wiring on one.

A missing tool comes from your distribution's package on Linux or WSL, from Homebrew on macOS, or from Chocolatey on native Windows. To check your machine, run the gate binary with `--emit env-probe`: it writes an untracked `ENV.local.md` with each tool's version and verdict. Building Checkwright itself takes more: see the [contributing guide](https://github.com/checkwright/checkwright/blob/master/CONTRIBUTING.md).

## What an install places

A repository install commits the profile's kit files, the gate binary and a handful of seeded config files; a personal install (`init --personal`) writes the same files untracked, and a user install (`init --user`) places nothing in the repository. Most of the kit files are each gate's fixture pair, which ships because it is how you check the binary's gates against their own cases ([gate-sdk/SPEC.md §Fixture-pair discipline](gate-sdk/SPEC.md#fixture-pair-discipline)); each figure is rounded to two significant figures, and a kit selection is the sum of its kits' rows:

<!-- install-size:begin -->

| Profile | Kits | Files | Of which gate-tests | Size |
| --- | --- | --- | --- | --- |
| `starter` | 1 | 720 | 660 | 540 kB |
| `workflow` | 3 | 980 | 880 | 1.0 MB |
| `delegation` | 8 | 1,200 | 1,100 | 1.6 MB |
| `prose` | 2 | 1,100 | 1,000 | 770 kB |
| `full` | 11 | 1,700 | 1,500 | 1.9 MB |

| Kit | Files | Of which gate-tests | Size |
| --- | --- | --- | --- |
| canon-kit | 430 | 390 | 230 kB |
| context-kit | 69 | 47 | 48 kB |
| delegation-kit | 46 | 35 | 92 kB |
| doctrine-kit | 12 | 9 | 64 kB |
| drift-kit | 6 | 0 | 24 kB |
| evidence-kit | 69 | 62 | 94 kB |
| gate-sdk | 720 | 660 | 540 kB |
| guard-kit | 42 | 30 | 220 kB |
| lifecycle-kit | 190 | 160 | 410 kB |
| queue-kit | 84 | 70 | 93 kB |
| site-kit | 74 | 66 | 64 kB |

<!-- install-size:end -->

The gate binary is one file and differs per target, so it is measured at each release rather than derived:

<!-- install-binary:begin -->

On `linux x86_64`, the gate binary `init` placed is **13.5 MB**, measured at v0.43.0.

<!-- install-binary:end -->

## What a commit costs

The pre-commit hook runs the gates your profile registers whose triggers match the staged files. It runs them one after another and stops at the first red. The figure runs every gate whose triggers match a file over every tracked file, in a repository the one-line install has just initialized with `full`, the profile that contains every other. A gate that reads the staged set from git itself sees nothing staged, so the figure is a reference reading, not a ceiling.

<!-- commit-cost:begin -->

On `linux x86_64` with `14` logical CPUs, the hook ran `44` pre-commit gates over every tracked file in **170 ms**, the median of three, measured at v0.43.0.

<!-- commit-cost:end -->

To measure your own repository, run the gate binary with `--measure-commit`. It stages nothing and times the same run over every tracked file. Two costs sit outside the figure. First, the hook's own start: git starts it with no shell on any OS, and the `gates` workflow's probe step prints what that costs on each OS and architecture. Second, a gate you register yourself adds its own time.

## Writing your own gates

A gate you write needs more than an install does: [Writing gates](authoring-gates.md) lists it for shell, PowerShell and Rust.
