# Install fixture

## Requirements

<!-- platforms:begin -->

| System | Minimum | Binary | Status |
|---|---|---|---|
| Linux on x86-64 (glibc) | glibc 2.39 | `x86_64-unknown-linux-gnu` | joined |
| Linux on x86-64 (any C library) | Linux 3.2 | `x86_64-unknown-linux-musl` | joined |
| macOS on Apple silicon | macOS 11 | `aarch64-apple-darwin` | held: a green producer leg consumed by a platform smoke leg |

A line after the table is prose and declares nothing: `i686-unknown-linux-gnu` is not a row.

<!-- platforms:end -->

<!-- prerequisites:begin -->

| Tool | Minimum | Needed for | Why |
|---|---|---|---|
| `sh` | any POSIX `sh` | required to install on Linux and macOS | the bootstrap is POSIX sh |
| Ruby | 2.3 | optional: only if you register the docs gates | the gems need it |

<!-- prerequisites:end -->
