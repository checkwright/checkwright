# SPEC amendment: win-build

Closes the compile half of `gate-binary-target-roster-widening`. The crate does
not compile for `x86_64-pc-windows-msvc`, so no Windows run can produce an
artifact, so every downstream Windows blocker is unreachable and the target
roster cannot widen even in principle.

**This amendment unblocks; it does not widen.** gate-sdk/SPEC.md §Consumer
payload bounds a roster line twice — it may not exceed what the install
documentation states, and *a target joins only when a green run has produced and
exercised its artifact*. No such run exists for any triple but
`x86_64-unknown-linux-gnu`, and nothing here produces one. `native/targets.list`
therefore ends this unit with exactly the line it starts with, and delta 5 is
what stops a later reader treating "unblocked" as "joinable".

**The queue entry's sizing was falsified at scope and re-verified here against
`c2cdef35`.** The entry's original "portability pass over four modules" is
wrong: `native/src/proc.rs:67`/`:75` are `#[cfg(unix)]`/`#[cfg(not(unix))]` twin
functions, `native/src/install.rs:163`/`:176` are the same shape, and
`native/src/proc.rs:335` is an inline `#[cfg(unix)]` block inside `exit_code`
that vanishes on non-unix. A grep over every `.rs` file in the crate for
`std::os::unix`, `PermissionsExt`, `MetadataExt`, `CommandExt`, `OsStrExt` and
`ExitStatusExt` returns those five sites and one more, and the one more is the
only un-gated one. The repair is a de-duplication, not a pass.

**One finding is deliberately *not* folded in, and it is filed rather than
carried.** `native/src/proc.rs:61` splits `PATH` on `':'`, which is wrong on a
Windows host (`;` is the separator, and a drive letter carries a colon);
`std::env::split_paths` is the portable API. It **compiles**, so the compiler
census could not see it and delta 4's oracle will not catch it either. Folding
it in would widen this unit from *the crate builds for the triple* to *the crate
behaves correctly on the triple*, which is a different and unbounded claim, and
no run has ever exercised the binary on Windows to bound it. It goes to the gap
inbox.

## What changes

### (1) The crate has one `is_executable`, and it is the portable one

`native/src/gates/gate_binary_fresh.rs:12-17` — a private `is_executable(&str)`
taking `std::os::unix::fs::PermissionsExt` unconditionally — is **deleted**, and
its one call site at `:112` reaches `proc::is_executable` instead, which is
already `#[cfg(unix)]`/`#[cfg(not(unix))]` twinned. `proc::is_executable` becomes
`pub` and takes the `&std::path::Path` it already takes; the call site wraps its
`String` in `Path::new`. **{mechanical}**

This is the whole compile blocker. On the native Windows runner it is `E0433` on
the `use` and `E0599` on `.mode()`, and `build-native.sh` exits 101 before cargo
emits anything, which is why every Windows blocker downstream of it was measured
as unreached rather than as passing.

**One behavioural difference rides the de-duplication and it is the right
direction.** The deleted local returns true for any path whose mode carries an
execute bit, including a directory; `proc::is_executable` additionally requires
`m.is_file()`. `check-gate-binary-fresh` uses the predicate to decide whether the
path `GATE_SDK_NATIVE_BIN` names is a runnable binary, and a directory there is
not one, so the shared predicate is strictly more correct at this call site. On
non-unix the twin is `p.is_file()`, which is the honest answer where the
filesystem carries no execute bit — executability on Windows is an extension
question, and that question belongs to delta 2, not here.

### (2) The executable suffix becomes a derived value with one owner

`gate-sdk/lib/gate.sh` gains `gate_exe_suffix [<triple>]`: it prints `.exe` and
nothing else when the argument matches `*-windows-*`, or — with no argument —
when the **host** is Windows (`uname -s` matching `MINGW*`, `MSYS*`, `CYGWIN*` or
`Windows_NT`), and prints empty otherwise. Three readers take it and no reader
spells the suffix itself. **{design-bearing}**

- `GATE_SDK_NATIVE_BIN`'s **default** (`gate-sdk/lib/gate.sh:85`) becomes
  `native/target/release/checkwright-gates` plus the **host** suffix. A consumer
  that pins the knob explicitly keeps its exact value; only the default moves,
  which is the one place the tree currently assumes a suffix-less cargo artifact.
- `gate-sdk/bin/build-native.sh:68`'s `BN_ART` takes the **target** suffix, not
  the host's: it strips any `.exe` from `gate_native_bin`'s basename and appends
  `gate_exe_suffix "$BN_TARGET"`, where `BN_TARGET` is the `--target` value the
  loop at `:60-66` already parses and empty when none was passed. A cross build
  from Linux for a Windows triple emits `<name>.exe` under
  `target/<triple>/release/`, and today's derivation looks for `<name>` there and
  reports "cargo reported success but no artifact is at …" — the message is
  correct about the path and wrong about the cause, which is exactly the
  misdiagnosis this delta removes.
- `scripts/pack-installer.sh:136` derives its per-target artifact name inside the
  roster loop rather than once before it, as `gate_exe_suffix "$target"` appended
  to the stripped basename. Today's single host-derived `binary` is right only
  while every roster line is the host's platform class. Fixed here, with the
  roster still one line, because the alternative is a landmine that fires on the
  commit that widens the roster and nowhere before it.

`installer/lib/init.sh`'s `select_artifact` needs **no** change: it discovers the
artifact name with `find … -maxdepth 1 -type f ! -name '*.sha256'` and asserts
exactly one, so it is already name-agnostic and a `.exe` satisfies it unchanged.
Named because the natural instinct is to add a fourth reader here, and adding one
would replace a working derivation with a spelling.

### (3) `target_of_host()` answers for the Windows host class

`installer/lib/init.sh:98`'s case map gains an arm: `uname -s` matching
`MINGW*`, `MSYS*` or `CYGWIN*` with `uname -m` `x86_64` maps to
`x86_64-pc-windows-msvc`. Today `MINGW64_NT-10.0-26100/x86_64` — the measured
runner's own answer — matches nothing and the function returns empty.
**{design-bearing}**

**The map answers "which published artifact fits this host", and that is why the
msvc triple is the right answer even under a MinGW shell.** The `uname` string
reports the *shell environment*, not the toolchain that built the artifact the
adopter is about to receive; what the roster will carry when it widens is the
triple a Windows CI leg builds, and the measured runner's own `rustc` host triple
is `x86_64-pc-windows-msvc`. Whether an msvc-built binary runs on an arbitrary
Windows host is a question **no run has answered**, and withholding that answer
until one has is precisely §Consumer payload's join bound doing its job.

**A third omit-reason token is refused, and the refusal is the surface's own
rule.** The temptation here is to split `substrate-unavailable` into
*host-unmapped* and *target-not-published*, since `select_artifact` collapses
them. installer/README.md §The gate binary states the bar: "A third token would
need a third remedy to earn its place." Both cases have the *same* remedy — there
is no adopter action, the platform is not in the support roster — so no token is
minted and the collapse stands.

**Reachability, stated rather than assumed.** The arm's reader is
`installer/lib/init.sh:120-121`, live on every install today; its verdict is
unchanged until `native/targets.list` names the triple, because the roster
comparison at `:121` still fails. It is reachable **now** for testing through
`GATE_SDK_NATIVE_TARGETS_FILE` (gate-sdk/SPEC.md §Layout and configuration),
which is the same steering knob installer/README.md §The consumer smoke already
documents as the roster re-entry.

### (4) A Linux cross-check step is the permanent oracle for the whole unix-only class

`.github/workflows/gates.yml`'s `gates` job gains one step after *build the
native gate binary*: `rustup target add x86_64-pc-windows-msvc` followed by
`cargo check --release --manifest-path native/Cargo.toml --target
x86_64-pc-windows-msvc`. It stays a bash `run:` on the runner's preinstalled
toolchain, matching the two steps above it and their stated tamper-floor reason.
**{design-bearing}**

**Why this is worth a step.** `cargo check` does not link, so the MSVC linker is
not needed and a Linux runner can answer the question; `rustup target add` fetches
a precompiled `std` for the triple. What it buys is larger than delta 1: it is a
standing regression gate over the **entire** `std::os::unix` class, so the next
un-gated `PermissionsExt` reds on the commit that adds it rather than on a
Windows adopter's machine. It costs **no additional push and no additional
workflow run** — it rides the `gates` job this repo already watches to green, and
CLAUDE.md's one-to-two-pushes-per-iteration budget is untouched.

**Three honest limits ship with it, because a claim without its bound is what
this unit is repairing.**

- **A compiler halts at its first error batch**, so a green check proves the
  crate compiles and a red one names one batch, never the full set. The scope
  census over all 139 `.rs` files found no other un-gated site and **cannot prove
  absence**; this step is what converts that from a claim into a run.
- **It proves compilation, not correctness.** `proc.rs:61`'s `PATH` split
  compiles cleanly for the triple and is wrong there. The step is silent on every
  such defect, and the filed gap is the disposition for the one that is known.
- **It is not a gate and must not become one.** It needs `rustup` and network,
  and this repo's own contributor floor does not carry `rustup`
  (context-kit/SPEC.md §bin/env-probe; the authoring machine here has a distro
  `rustc` and no `rustup` at all). A registry member that cannot run in a
  contributor's tree is a member that reds for the wrong reason, so this stays a
  CI instrument, exactly as `platform-support-ci-matrix`'s probe job is.

### (5) The roster does not widen, and its header says so in this iteration's terms

`native/targets.list`'s header keeps its one line and its bound, and its
Windows sentence is corrected: it currently sends a reader to
`powershell-installer-surface` for native Windows, which is no longer where the
blockers live. It states instead that the crate now *compiles* for the msvc
triple under delta 4's oracle, that compiling is not the join bound, and that the
join still waits on a run that produced **and exercised** an artifact —
`platform-support-ci-matrix`'s, whose promotion condition is unchanged.
**{design-bearing}**

Stated as its own delta because the failure mode is specific and likely: an
iteration named *unblock* invites the next reader to read the removal of a
blocker as the arrival of a permission. It is not. Deltas 1 to 4 move a target
from *impossible* to *eligible to be measured*, and nothing here measures one.

## Producers and consumers

**`gate_exe_suffix` (delta 2)** — a new interface, the only new name in this
amendment.

- *Producer:* `gate-sdk/lib/gate.sh`, evaluated at source time for the
  `GATE_SDK_NATIVE_BIN` default and per call for the other two readers.
  **Enabling config actually set:** none is required — it reads `uname` and its
  own argument, both present in every tree that sources the library, so it is
  live on this repo's first battery run rather than test-only.
- *Consumers,* three, each by direct call and each named above: `gate.sh`'s own
  knob default (host form); `build-native.sh`'s `BN_ART` derivation (target
  form, at the artifact-existence assertion, after cargo returns);
  `pack-installer.sh`'s per-target artifact name (target form, once per roster
  line, before the `-f` assertions at `:145`).
- *Named reader for every field:* the function returns one value and has no
  fields. `BN_TARGET` is a new **local** in `build-native.sh` and its reader is
  the `BN_ART` assignment three lines below it, in the same run — it is the
  `--target` value the existing loop already extracts into `BN_OUT`, given a name
  because two derivations now need it.

**The Windows arm of `target_of_host` (delta 3)** introduces no state. Its
producer is the existing function, its consumer is `select_artifact:120-121` in
the same call, and its verdict is unchanged until the roster names the triple —
stated in the delta rather than left for a reader to discover.

**The cross-check step (delta 4)** produces a CI verdict and no in-tree state.
Its consumer is the pushing session through the `gates` workflow's own
conclusion, which CLAUDE.md already makes the release oracle for a master push.
It writes nothing, so no surface reads a new artifact.

**Deltas 1 and 5 introduce no state, event or interface.** Delta 1 deletes a
private function and moves a call; delta 5 rewrites prose in a file whose readers
(the publish matrix, `pack-installer.sh`, the installer's payload copy) all read
the file's **live lines**, which are unchanged.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
No delta narrows a corpus — no file is pruned, no glob tightened, no tracked path
removed — so the monotone-verdict analysis has no subject here. The readers that
*do* move are enumerated by red condition anyway, because two of them are not
monotone in the ordinary sense and would pass silently if only their subject were
named:

- `check-knob-default-coupling` — reds when a knob's default as stated in the
  governed SPEC differs from the default the code resolves. **Not monotone, and
  it is the one this amendment must move in the same commit:** delta 2 changes
  `GATE_SDK_NATIVE_BIN`'s resolved default, so gate-sdk/SPEC.md §Layout and
  configuration's stated default goes red unless it takes the host-suffix clause
  with it. Listed as an update target below.
- `check-gate-binary-fresh` — reds when the registered binary is older than the
  crate source it was built from. **Monotone here and cleared by inspection**:
  delta 1 edits crate source, so the binary is stale until
  `bash gate-sdk/bin/build-native.sh` runs, which is the commit-time obligation
  CLAUDE.md §Housekeeping already states for this tree. No behaviour of the gate
  changes; only its own module does.
- `check-crate-arms` — reds when the crate's lint or test arms fail. **Not
  monotone**: it asserts arms *pass*, so making `proc::is_executable` public
  reds it if any lint objects to a newly public item without a doc or with an
  unused-import residue at the old call site. Both arms run in the battery and
  are the oracle for delta 1.
- `check-action-pinning` / `check-action-run-shell` — red on an unpinned `uses:`
  and on a `run:` without the contracted shell handling. **Monotone under an
  addition** and cleared by construction: delta 4 adds a `run:` step and no
  `uses:`, which is why it is a `run:` at all.
- `check-install-claim` and `check-payload-claim` — red on a governed doc
  asserting an install or disclosure class the owning surface contradicts. **Not
  monotone** (both hold minimum-count conditions over declarations). Delta 3
  touches installer/README.md §The gate binary's prose and delta 5 touches the
  roster header, so both surfaces are update targets below and are re-read at
  merge rather than assumed clear.

## Existing sections updated

- `gate-sdk/SPEC.md` §Layout and configuration — `GATE_SDK_NATIVE_BIN`'s stated
  default gains the host-suffix clause, and `gate_exe_suffix` joins the accessor
  roster (delta 2).
- `gate-sdk/SPEC.md` §lib/gate.sh — the new accessor's contract: its two forms,
  its `uname` host set, and the rule that no other surface spells `.exe`
  (delta 2).
- `gate-sdk/SPEC.md` §build-native — the artifact-path derivation paragraph,
  which currently derives from the crate dir and any `--target` alone, gains the
  suffix term and the corrected diagnosis of the message at `:70` (delta 2).
- `gate-sdk/SPEC.md` §Consumer payload — the per-target artifact name is
  target-derived rather than host-derived, stated where the one-payload-carries-
  every-target rule lives; and the join bound is restated as *unchanged by an
  unblocking*, which is delta 5's claim written on the surface that owns the
  bound (deltas 2 and 5).
- `gate-sdk/SPEC.md` §check-gate-binary-fresh — the module no longer carries its
  own executability predicate; the section names `proc::is_executable` as the
  one spelling and records the `is_file()` tightening (delta 1).
- `installer/README.md` §The gate binary — `target_of_host`'s Windows arm, the
  reason the msvc triple is the right answer under a MinGW shell, and the
  explicit refusal of a third omit-reason token on that section's own
  three-remedies rule (delta 3).
- `native/targets.list` — the header's Windows sentence and its
  `powershell-installer-surface` pointer (delta 5).
- `.github/workflows/gates.yml` — the `gates` job's step list, and the step's own
  comment carrying the three limits rather than deferring them (delta 4).
- `installer/consumer-smoke/run-smoke.sh` — no content change; re-read at merge
  to confirm the `blocked()` helper at `:17` and the roster comparison at
  `:43-44` still compare a **host** triple against a roster whose artifact names
  are now target-derived, since delta 2 moves the name and not the comparison.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged **at the iteration** rather
      than at this commit, a sibling gate-sdk amendment being in flight
      (canon-kit/SPEC.md §Merging an amendment, step 3).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The binary is rebuilt in the same commit** — `bash
      gate-sdk/bin/build-native.sh` beside the battery, neither discharging the
      other (CLAUDE.md §Housekeeping).
- [ ] **The census is re-run, not cited** — the `std::os::unix` grep over the
      crate is re-run against the tree at build; delta 4's step is what proves it,
      and its first green run is the evidence, not this file.
