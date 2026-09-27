# Install fixture

## Requirements

<!-- platforms:begin -->

| System | Minimum | Binary | Status |
|---|---|---|---|
| Linux on x86-64 | glibc 2.39 | `x86_64-unknown-linux-gnu` | joined |
| macOS on Apple silicon | macOS 11 | `aarch64-apple-darwin` | held: |
| macOS on Intel | macOS 10.12 | `x86_64-apple-darwin` | held: a green producer leg consumed by a platform smoke leg |
| Windows on ARM64 | | `aarch64-pc-windows-msvc` | held: a green producer leg consumed by a platform smoke leg |
| Windows on x86-64 | Windows 10 | `x86_64-pc-windows-msvc` | joined |

<!-- platforms:end -->

<!-- prerequisites:begin -->

| Tool | Minimum | Needed for | Why |
|---|---|---|---|
| `sh` | — | required to install on Linux and macOS | the bootstrap is POSIX sh |
| Ruby | 2.3 | the docs gates, on macOS | the gems need it |

<!-- prerequisites:end -->
