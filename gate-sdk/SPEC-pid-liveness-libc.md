# SPEC amendment: pid-liveness-libc

The crate's pid-liveness predicate has two owners, `evidence::pid_alive`
(`native/src/evidence.rs:318`) and a private `pid_alive` in
`native/src/emit/wait_probe.rs:499`. Both reach `kill -0` by spawning
`bash -c 'kill -0 "$1"'`, because `std` has no spelling for signal 0 and the crate
carries no `libc`. `evidence::pid_alive` then falls back to `ps -p`, off
`GATE_SDK_PROGRAM_FLOOR`, for the case the builtin's exit status cannot tell
apart: a process that exists but cannot be signalled (EPERM) from one that is
gone (ESRCH). A direct `kill(pid, 0)` returns that distinction as `errno`. So on
unix one `libc` call replaces both bash spawns and the `ps` leg. This amendment
takes `libc` as the crate's second dependency, **on unix targets only**. It also
makes `evidence::pid_alive` the one liveness owner, and it leaves the native
Windows build on today's route.

**The dependency is ruled here, against the bar the SPEC already states** (gate-sdk/SPEC.md
§The settings cohort, and the crate's first dependency): no filesystem walk, no
subprocess, no socket, an MSRV at or below the crate's floor, and a transitive set
that is small, enumerable and admitted under the same clauses. `libc` clears every
clause. Its build script spawns `rustc` for a version probe at **build** time. That
is the same shape as the admitted `serde_core` and `zmij` build scripts, and the
bar's subprocess clause is about what the gate binary spawns when it runs.

**Why native Windows keeps the bash route.** On `x86_64-pc-windows-msvc`, the
pids that `.run` records and locks carry come from an MSYS shell's `$!`, which is
MSYS's own pid namespace. Only MSYS's `kill` builtin resolves those pids. A Win32
`OpenProcess` would read the wrong namespace. The `libc` crate also has no `kill`
on that target. So the non-unix build keeps `bash -c 'kill -0'` plus the `ps -p`
fallback, unchanged. The shape is a `cfg(unix)` / `cfg(not(unix))` pair, as in
`proc.rs`, `install.rs` and `wait_probe.rs`.

**Probed at authoring, 2026-09-18.**

- Adding `[target.'cfg(unix)'.dependencies] libc = "0.2"` to a copy of
  `native/Cargo.toml` and resolving it (`cargo metadata`) adds exactly one
  package to `Cargo.lock`: `libc 0.2.189`. Its only dependency,
  `rustc-std-workspace-core`, is optional behind `rustc-dep-of-std`. It is **not**
  carried into the lock.
- The crate's `Cargo.toml` declares `rust-version = "1.65"`, below this crate's
  1.71.
- `grep -rn unsafe native/src` returns nothing. This amendment adds the crate's
  first `unsafe` block (delta 2).
- The tree does not already do this: `grep -n libc native/Cargo.toml` returns
  nothing, and both `bash -c` sites and the `ps` leg read at HEAD
  (`evidence.rs:322`, `evidence.rs:332`, `wait_probe.rs:500`).

## What changes

### (1) `libc` joins the crate on unix targets, admitted by name {mechanical}

- `native/Cargo.toml` gains `[target.'cfg(unix)'.dependencies]` with
  `libc = "0.2"`.
- The tracked `native/Cargo.lock` gains the one `libc` package the probe above
  resolved.
- `ADMITTED_CRATES` in `native/src/walk.rs`'s tests gains the row
  `("libc", "raw C bindings, unix targets only; no walk, no subprocess, no socket")`.
  Assertion `every_crate_in_the_resolved_graph_is_admitted_by_name` reds without
  that row.

Run `bash gate-sdk/bin/build-native.sh`: the source stamp covers `Cargo.toml` and
`Cargo.lock` (§check-gate-binary-fresh).

### (2) `evidence::pid_alive` answers from `kill(2)` on unix {design-bearing}

`native/src/evidence.rs`. The pid grammar check stays first and unchanged: an
empty pid, a leading `0` or a non-digit reads not-alive with nothing probed. Then:

- **unix** (`#[cfg(unix)]`): parse the pid as `libc::pid_t`. A pid that does not
  fit reads **not alive**, since no process can hold it. `libc::kill(pid, 0)`
  returning `0` is alive. On `-1`, read `errno` with
  `std::io::Error::last_os_error().raw_os_error()`:
  - `EPERM` is **alive**: the process exists and is not ours, the held reading
    the predicate exists to give.
  - `ESRCH` is **not alive**.
  - Any other value is `Err(PidProbe::Unanswered(<message naming the pid and the
    OS error>))`. It is unreachable for signal 0 on a positive pid, and it fails
    closed rather than reading free.

  The call is the crate's first `unsafe` block, and it carries a directive naming
  why it is sound: `kill` takes two integers and touches no memory the crate owns.
- **not unix** (`#[cfg(not(unix))]`): today's body, verbatim — `bash -c 'kill -0'`,
  then `on_path("ps")`, then `ps -p`.

`PidProbe` changes too:

- `PidProbe::Spawn(String)` is renamed `PidProbe::Unanswered(String)`. Its unix
  producer is no spawn, so the old name would misstate it.
- `PidProbe::PsAbsent` becomes `#[cfg(not(unix))]`, and so do its two match
  arms: `run_validate.rs` `probe_message` and `producer_liveness.rs`
  `probe_failed`. The same goes for `refuse_absent_ps`, because an unconstructed
  variant is a dead-code finding under `check-crate-arms`' deny-warnings clippy
  on the unix target.

The three callers are `run_validate.rs:272`, `enter_stage.rs:1624` and
`producer_liveness.rs:33`. Only the two match sites above change, for the rename.
`enter_stage.rs` reads any `Err` as orphaned and names no variant.

**Verdict-preserving by construction, with one deliberate difference.** Bash's
`kill -0` builtin *is* `kill(pid, 0)`. So on a live, dead or zombie pid the two
routes return the same answer. The zombie case matters to
delegation-kit/SPEC.md §bin/wait-probe, and both routes read a zombie as alive.
The one difference is EPERM. The old route could not see it and took `ps -p` to
answer existence. The new route reads it directly, so the fallback program is not
needed on unix. `ps` therefore stops being a unix requirement. The absent-`ps`
refusal (§Fail-closed contract, the fifth wrapper) cannot fire there, because
nothing is left to be absent.

**Unit cases** (`producer_liveness.rs` tests, beside the existing ones):

- On unix, PID `1` reads alive whatever the test's uid, through EPERM when the
  test is unprivileged.
- `99999999999` reads not alive without a probe, because it does not fit
  `pid_t`.

The existing own-pid and `2147483646` cases stay.

### (3) `wait_probe`'s private predicate is deleted {mechanical}

`native/src/emit/wait_probe.rs`: `fn pid_alive` and its directive go. The sweep's
`while pid_alive(&pid)` becomes
`while crate::evidence::pid_alive(&pid).unwrap_or(false)`. This keeps the old
behaviour where a probe that cannot answer ends the wait, because the private
function mapped a spawn failure to `false`. On unix the sweep now spawns nothing
per poll. On the recorded pids, which are the probe's own children and share its
uid, the verdict is unchanged (delta 2's construction argument). delegation-kit/SPEC.md
§bin/wait-probe states the liveness rule as "`kill -0` on the recorded pid",
which stays true, and names no spawn for it, so its text does not change.

### (4) `check-producer-liveness` declares its programs per platform {mechanical}

The `gates::REGISTRY` row at `native/src/gates/mod.rs:1907` declares
`&[("bash", ""), ("ps", "")]` on every target. It takes a `#[cfg]` pair of consts
instead: empty on unix, and today's two rows on `not(unix)`. Unit test A
(`every_registry_member_declares_the_programs_it_spawns`) is *observed ⊆
declared*, so the empty unix set is held by the unix CI run, which observes no
spawn. The row's directive is rewritten to say the unix predicate spawns nothing.

**Readers of the narrowed declaration (causal-completeness point 5).** The
`--needs` consumers are `main.rs:361` (printing) and `port_blockers.rs:342`
(the report's per-member rows). Neither reds on an empty set. The report
prints no off-floor requirement for this member on unix, which is the truth. No
golden or test pins `ps` for this member: `git grep -n '"ps"' native/src` hits
the registry row, `evidence.rs` and `scan_prompts.rs`'s unrelated `docker ps`
roster.

### (5) The SPEC passages that state the bash-and-`ps` route are rewritten {design-bearing}

**Not yet applied**, each passage below:

- **gate-sdk/SPEC.md §Fail-closed contract**, the paragraph "**A wrapper's
  program can be a shell builtin, …**" (lines 1542-1552), is replaced by:

  > **A wrapper's program can be a shell builtin, and the route to it is chosen
  > per platform.** The pid predicate's first leg is signal 0. On unix the crate
  > calls `kill(2)` through `libc`, which returns *exists but not yours* as
  > `EPERM`, so no fallback program is needed and the unix build declares none.
  > On a non-unix build the pids are an MSYS shell's, which only that shell's
  > builtin resolves. So the leg stays `bash -c 'kill -0'`, whose exit status
  > conflates EPERM with ESRCH, and `ps -p` answers existence behind it. Spawning
  > `/bin/kill` there would mint a second off-floor requirement, and probing with
  > `ps` alone would require it on every call. The non-unix declared set carries
  > both, because unit test A is *observed ⊆ declared* and floor membership is what
  > the report filters on, not what the registry records.

- **gate-sdk/SPEC.md §Fail-closed contract**, the preceding paragraph "**The fifth
  wrapper found the third shape …**" (lines 1527-1540). Its sentence "the port
  refuses, on the fallback leg only, because a `kill -0` that answers never
  reaches the program" gains the clause "— a leg only the non-unix build still
  has".
- **gate-sdk/SPEC.md §The port-candidate criteria**, criterion 7's `--run-validate`
  instance (lines 4777-4787). "`ps` is reached transitively through the pid
  predicate's second leg and stays" becomes "`ps` stays only on non-unix builds,
  where the pid predicate keeps its second leg". Keep the next sentence's ruling,
  which forbids a false **free** reading, as the reason the leg exists. Replace its
  "`kill -0` conflates *no such process* with *not yours*" with "the builtin's exit
  status conflates …".
- **gate-sdk/SPEC.md §The port-candidate criteria**, the fifth-wrapper paragraph
  (lines 4964-4984). "the honest route to it is `bash -c` — on the program floor —
  rather than a second off-floor dependency" becomes "the honest route to it is
  `kill(2)` through `libc` where the platform has one, and `bash -c` on the
  program floor where the pids are a shell's own — never a second off-floor
  dependency".
- **gate-sdk/SPEC.md §port-blockers**, "It is now a `.gate` member whose `--needs`
  declares `ps` outright" (line 12361). It becomes "whose `--needs` declares `ps`
  outright on the one platform class that still spawns it, and nothing on unix".
- **gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency**. The
  sentence "The resolved graph is **11 packages** — …" moves its count to the
  lock that owns it. Its replacement (the de-literalization rule; the lock is the
  owner):

  > Every package in the resolved graph, activated or carried by the lock as an
  > unactivated optional dependency, is named with its admitting clause in
  > `walk.rs`'s allowlist, which is the machine-held form of this bar rather than
  > a second list beside it.

  The section then gains one paragraph after "**The floor moved, …**":

  > **The second dependency is `libc`, taken on unix targets only.** It cleared
  > the bar with an empty transitive set and an MSRV below the floor, and it is
  > what lets the pid predicate read `EPERM` in-process
  > (evidence-kit/SPEC.md §The producer-liveness lock). Its one `unsafe` call is
  > `kill(pid, 0)`, which touches no memory the crate owns. The target scoping is
  > the point of the admission and not a size trim: the native Windows build
  > resolves pids in a shell's namespace, where the crate's `kill` would read the
  > wrong processes.

- **evidence-kit/SPEC.md §The producer-liveness lock**, the paragraph "The
  liveness predicate is the one all three readers share, …" (lines 442-450). Its
  second and third sentences become:

  > Signal 0 is the cheap existence probe, and the reading it must never give is a
  > false **free**: a producer running under another uid exists but cannot be
  > signalled. On unix the predicate calls `kill(2)` and reads `EPERM` as held and
  > `ESRCH` as gone. On a non-unix build it reaches the shell's `kill -0` builtin,
  > whose exit status conflates the two, so `ps -p` runs as the fallback and any
  > evidence of existence means held.

  The `/proc` sentence stays.
- **evidence-kit/SPEC.md**, the paragraph "**Where `ps` is absent the predicate
  cannot answer, …**" (line 452), is scoped by an opening clause: "On a non-unix
  build, where `ps` is the fallback leg, …". The rest is unchanged.
- **evidence-kit/SPEC.md §bin/run-validate.sh**, "**`sha256sum` leaves the spawn
  set and `ps` does not.**" (lines 597-604). It becomes "**`sha256sum` leaves the
  spawn set, and `ps` leaves it on unix.**", and the body's `ps` sentence
  becomes: "`ps` stays only on a non-unix build, reached through the pid
  predicate's fallback leg, which §The producer-liveness lock rules the content of
  the rule there rather than incidental spelling".
- **evidence-kit/SPEC.md**, "The lock cannot be classified at all where `ps` is
  absent" (line 649). It becomes "On a non-unix build the lock cannot be
  classified at all where `ps` is absent".
- **evidence-kit/SPEC.md**, "the `kill -0`-then-`ps -p` pair means" (line 1080)
  becomes "the pid predicate's EPERM-is-held reading means".
- **evidence-kit/SPEC.md §check-producer-liveness**, the fixture paragraph
  (lines 1137-1143). "which is exactly what the predicate's `ps -p` leg makes
  reliable: under `kill -0` alone, an unprivileged run reads init as dead" becomes
  "which is exactly what the predicate's EPERM reading makes reliable: under the
  builtin's exit status alone, an unprivileged run reads init as dead".
- **evidence-kit/SPEC.md §check-producer-liveness**, "**It is a wrapper, and the
  requirement lives in the library …**" (lines 1176-1184). It is replaced by:

  > **On a non-unix build it is a wrapper, and the requirement lives in the
  > library rather than in the gate's own text.** There the pid predicate tries the
  > `kill -0` builtin through `bash -c` and falls back to `ps -p`, which is off
  > `GATE_SDK_PROGRAM_FLOOR`. So that build's registry row declares `ps` for the
  > fallback leg, and `bash`, which is on the floor and so uncounted. On unix the
  > predicate is one `kill(2)` call and the row declares nothing. The lock reader
  > spawns nothing on either.

- **evidence-kit/SPEC.md**, "**The absent-`ps` refusal is a deliberate divergence
  …**" (line 1186). It opens with "On a non-unix build, …". The measured
  comparison below it stays as the record it is.
- **context-kit/SPEC.md §bin/env-probe**, "`uname` (this arm), `ps` (the pid
  predicate's fallback leg) and `tar` and `npm` (the installer packer) are on
  neither set" (lines 404-405). It becomes "`uname` (this arm), `ps` (the pid
  predicate's fallback leg, non-unix builds only) and …".

### (6) Regenerate the projections the SPEC edits stale {mechanical}

Run the regen each freshness gate prints on red. That covers at least the mirrors
`docs/gate-sdk/SPEC.md`, `docs/evidence-kit/SPEC.md` and `docs/context-kit/SPEC.md`
(docs/site-architecture.md §Generated projections and their freshness gates).

## Producers and consumers

- **The `libc` dependency** (delta 1). Its producer is `Cargo.toml`. Its
  roster-holding reader is `walk.rs`'s `ADMITTED_CRATES` assertion, which reds on
  an unadmitted crate and on a stale row. `check-gate-binary-fresh` reads the
  manifest and lock through the source stamp. `check-crate-arms` runs clippy at
  deny-warnings and the tests on the host, which is Linux here. The
  `native-artifacts` job (`.github/workflows/gates.yml`) builds every target in
  `native/targets.list` on that target's own runner, so its
  `x86_64-pc-windows-msvc` build proves that the `cfg(not(unix))` body compiles
  without `libc`.
- **`PidProbe::Unanswered`** (delta 2). Its producers are the unix errno fallthrough
  and the non-unix spawn failure. Its consumers are the two existing match sites,
  which print its message and exit 2 (point 4: its one field, the message, is read
  by both).
- **The platform-conditional `needs` row** (delta 4). Its producer is the
  registry. Its consumers are `--needs` and `--emit port-blockers`, and unit test A
  holds it to behaviour on each CI platform.
- **Point 5 (narrowing).** Delta 4 narrows a declaration on unix. Its readers
  are enumerated there, and none holds a minimum or an exact count. Delta 3
  removes a function whose only caller is the sweep.
- **Point 6.** No delta obliges each member of a corpus.

## Existing sections updated

- `gate-sdk/SPEC.md` §Fail-closed contract, §The port-candidate criteria, §port-blockers,
  §The settings cohort, and the crate's first dependency (delta 5).
- `evidence-kit/SPEC.md` §The producer-liveness lock, §bin/run-validate.sh,
  §check-producer-liveness (delta 5).
- `context-kit/SPEC.md` §bin/env-probe (delta 5).
- `native/Cargo.toml`, `native/Cargo.lock` and `native/src/walk.rs` (delta 1).
- `native/src/evidence.rs` — the predicate and the `PidProbe` rename (delta 2).
- `native/src/emit/run_validate.rs` — the `probe_message` match (delta 2).
- `native/src/gates/producer_liveness.rs` — the `probe_failed` match and unit
  cases (delta 2).
- `native/src/emit/wait_probe.rs` (delta 3).
- `native/src/gates/mod.rs` (delta 4).
- `docs/gate-sdk/SPEC.md`, `docs/evidence-kit/SPEC.md`, `docs/context-kit/SPEC.md`
  (delta 6).

## Retired spellings

- `PidProbe::Spawn` — renamed `PidProbe::Unanswered` by delta 2, because its
  unix producer is no spawn.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Both platform classes green** — `check-crate-arms` green on the host,
      where unit test A observes no spawn from the pid predicate; the pushed run's
      `native-artifacts` build is green for `x86_64-pc-windows-msvc`; and
      `bash gate-sdk/bin/build-native.sh` plus the battery at the commit.
- [ ] **The entry moves to Done before the drain stage** — at the batch that
      merges this amendment.
