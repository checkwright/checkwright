# SPEC amendment: help-arm-scope

§The bin/-tool contract states three behaviors for a `bin/` tool whose positionals
are free text: help on stdout at exit 0, refusal of an unrecognized `-`-led
positional, and `--` ending option processing. It leaves unstated whether the help
half binds a tool taking **no** positionals, or one **forwarding** its argv to
another program. That scope sets the corpus before any member is fixed. **This
amendment rules the scope. The help half binds every `bin/` tool, whatever its
positionals, except a total forwarder whose whole contract is its target's. The
refusal and `--` halves stay scoped to free text.** It gives the one live member its
arm and its smoke coverage.

**The census (2026-09-21).** Five tracked `bin/` files exist outside `gate-tests/`
(`git ls-files '*/bin/*' | grep -v /gate-tests/`). They are `gate-sdk/bin/build-native.sh`
and `gate-sdk/bin/run-gates.sh` with its `.ps1` twin, and
`installer/bin/checkwright.sh` with its `.ps1` twin. The entry's census command,
`git ls-files '*/bin/*.sh' | grep -v '/gate-tests/' | xargs grep -L -- '--help'`,
prints `gate-sdk/bin/build-native.sh` alone. The entry's supporting claim that
`checkwright.sh` carries a `-h | --help)` branch is no longer true. It lost it when
every verb moved behind the invoke, and it forwards every argument to the verified
artifact. So the proxy passes it only on a comment.

- `run-gates.sh` maps `-h | --help` to the usage path, and gate-sdk's smoke asserts
  exit 0 with `usage: run-gates.sh` on stdout. Its `.ps1` twin carries the same
  branch (`run-gates.ps1:102`).
- `build-native.sh` forwards its whole argv to `cargo build`. So `--help` prints
  cargo's usage, which says nothing of the tool's own toolchain floor, remap flags
  or banned-pattern verification. No smoke touches it.
- `checkwright.sh` is a total forwarder. installer/SPEC.md §The verbs rules that the
  binary owns the verb roster, so `checkwright --help` reaches the artifact's own
  usage. That is the answer the caller wants.

**Why the help half binds beyond free text.** It is discoverability, not absorption.
The contract's own measured cost is a session hunting for a mode that got a wrong
answer in place of usage, and that cost does not depend on the positional's shape.
The shipped precedent already reads it this way: `--enter-stage` takes
membership-validated positionals, owes no refusal, and answers help at exit 0 by its
own directive. **Why a total forwarder is exempt.** The help half exists so a caller
can learn what the tool does. A tool that adds nothing to its target's behavior is
answered by the target's own help, and intercepting it would hide exactly the usage
the caller came for. `build-native.sh` is not total: it adds a toolchain floor
check, remap flags and a verification step that `cargo build --help` cannot
describe.

## What changes

### (1) §The bin/-tool contract states the help half's scope {mechanical}

**Not yet applied.** After the three-behavior list, add:

> **The help half binds every `bin/` tool; the refusal and `--` halves bind
> free-text positionals.** Help is discoverability, owed whatever a tool's
> positionals are, including none and including a forwarded argv. The one exemption
> is a **total forwarder** — a tool whose whole contract is its target's, so the
> target's own help is the answer (installer/SPEC.md §The verbs rules the bootstrap
> one). A forwarder that adds behavior of its own is not total, and takes `-h` /
> `--help` as its first argument before anything is forwarded. A `.ps1` twin
> follows its `.sh` member.

### (2) `build-native.sh` answers help before forwarding {mechanical}

**Not yet applied.** It adds a first-argument `-h | --help` branch to
`gate-sdk/bin/build-native.sh`, ahead of the cargo probe. The branch prints the
file's existing usage block (its three `# usage:` lines) on stdout and exits 0,
before `gate.sh` is sourced or `cargo` is reached. The usage block gains one line
saying that cargo's own help is `cargo build --help`, which the tool no longer
forwards. The text is fixed by the file, so this is a transcription.

### (3) gate-sdk's smoke covers it {mechanical}

**Not yet applied.** Beside the `run-gates.sh --help` assertion in
`gate-sdk/smoke/install.sh`, it asserts that `bash gate-sdk/bin/build-native.sh --help`
exits 0 with `usage: build-native.sh` on stdout. It runs on a scratch consumer with no
crate and, where the host has none, no cargo. That proves the branch precedes both
refusals.

## Producers and consumers

- **The scope rule (delta 1).** Its readers are a `bin/` tool's author and the smoke
  owner. It is prose, held behaviorally, as the section already rules. No gate reads
  it, and delta 1 does not change that ruling.
- **The help arm (delta 2).** Producer: the tool. Consumers: a caller probing for
  modes, and delta 3's assertion.
- **Point 6.** The corpus is the five `bin/` files. Their satisfying values:
  `run-gates.sh` and `run-gates.ps1` already comply. `build-native.sh` complies by
  delta 2. `checkwright.sh` and `checkwright.ps1` comply as total forwarders, with no
  change.

## Existing sections updated

Rosters from the census commands above, reading `gate-sdk/bin/build-native.sh`, and
`grep -n help gate-sdk/bin/run-gates.ps1`.

- gate-sdk/SPEC.md §The bin/-tool contract (delta 1).
- `gate-sdk/bin/build-native.sh` (delta 2).
- `gate-sdk/smoke/install.sh` (delta 3).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — no name is removed.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Delta 1 re-phrases the scope sentence it
      qualifies rather than appending a caveat.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `bin-tool-help-arm-absent-tree-wide` moves to Done in the
      merge commit, at a stage before the drain stage.
- [ ] **Smoke green.** gate-sdk's consumer smoke passes with delta 3's assertion.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
