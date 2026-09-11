# SPEC amendment: test-cwd-isolation

## What changes

### (1) The two registry-coverage unit tests enter a fixture case on a child, never on the test process {design-bearing}

`every_registry_member_declares_the_roots_it_walks` and
`every_registry_member_declares_the_programs_it_spawns` (`native/src/gates/mod.rs`)
each call `std::env::set_current_dir(<case>)` around every member run, and restore
it afterwards. Their own `spec:` line says the case "is entered exactly as the
--run-gate-tests arm enters it". It is not.

§run-gate-tests rules that the runner "sets that directory on the spawn and never
enters it itself", so no code path can leave the process in a case dir. The tests
enter the directory in-process, in a binary whose other cases run on sibling
threads.

**The change.**

- **The parent** keeps its loop, its `knobenv::lock()` and its knob bridging.
  Per case, it spawns the running test binary (`std::env::current_exe()`),
  filtered `--exact` to one ignored **observer** case. The child's working
  directory is the case, set on the spawn through the crate's funnel
  (`proc::run_merged_in(.., Some(case))`, the route the runner takes). The
  member's name rides a child-scoped environment entry on that spawn, never a
  process-global write.
- **The observer** runs the member with the thread-local recorder on. It prints
  one sentinel-prefixed line per observed root or program, and the member's exit
  status.
- **The parent** parses those lines and asserts exactly what it asserts today:
  observed ⊆ declared, no exit 2, a non-empty case set and a non-empty
  observation. The subject of each assertion is unchanged; only where the run
  happens moves.

The observer is `#[ignore]`d, so an ordinary `cargo test` skips it. It panics
without the parent's marker, so a `--include-ignored` run cannot execute it
in-process. No invocation in the tracked tree passes `--include-ignored` today. Probed
tree-wide: the only `--ignored` spellings are `git ls-files --others --ignored`
in canon-kit, a git flag rather than cargo's, and `check-crate-arms`' test arm
passes no filter.

**Cost, measured rather than estimated.** A filtered `--exact` re-exec of the
release test binary (693 tests) took 20 ms for twenty invocations, about 1 ms
each. There are 230 fixture case directories and two tests, so the added harness
overhead is under a second. The members' own run time was already paid
in-process.

### (2) The crate's in-process working-directory writes are pinned by a roster test {mechanical}

A unit test pins where the crate may change its own working directory
{mechanical}.

It sits beside `no_module_outside_proc_constructs_a_subprocess_itself` in
`native/src/proc.rs`, because the remedy it names is a spawn, and it reuses that
roster's shape. Every crate source is scanned for the code spelling
`set_current_dir(`. A site is an offender unless `cwd-write-exempt: <cause>` sits
in the four lines above it, the same window and the same named-cause valve as
that roster's `spawn-funnel-exempt:`.

**The valve token rides a `spec:` directive, never a bare comment.** Its
precedent's one live site shows the form:
`// spec: gate-sdk/SPEC.md §Fail-closed contract — spawn-funnel-exempt: <cause>`
at `native/src/emit/wait_probe.rs`. That is how the token clears
`check-comment-tier` without registration, so neither token is listed in any
comment-tier roster.

**The scope rule is the opposite of its neighbor's.** The `Command` roster drops
`#[cfg(test)]` items by design. This pin **keeps** them, because a test is
exactly the writer it exists to catch.

**It matches the code spelling, never the bare word.** That is the neighbor's own
rule: `native/src/emit/close_surfaces.rs` carries a directive comment naming the
API as the reason that arm anchors its paths, and that comment states the rule
rather than breaking it.

**The one valved site today** is `native/src/emit/pack_installer.rs`'s `pack()`,
whose cause is that the `--pack-installer` arm's own process enters the tree it
packs, and that no test reaches `pack()`.

- Probed: the file's seven cases call `parse`, `resolve_version`, `is_commit`,
  `report`, `inside` and `dirty_cause` only.
- The only caller of `pack()` is `pack_installer::run`, reached through the arm
  table.

A valve at the site replaces an allowlist in the test, so the cause travels with
the code it excuses and no second list can drift from it.

The red message names delta 1's route: set the directory on the spawn.

**Its honest limit.** A spelling roster cannot see a dependency that changes the
working directory, and it cannot see a future test that reaches `pack()` through
the arm table. The second is the probe above, re-run; the first has no instance
in the resolved graph today.

### (3) gate-sdk/SPEC.md states that a test never writes the working directory in-process, and why that is not serialized {design-bearing}

It lands in the passage that owns the crate's process-global test state, "The
bridge is process-global, and the crate's own tests are serialized against it".
The new paragraph says the working directory is the second process-global a case
can write, and it is ruled the opposite way from the knob environment, on a
structural ground.

**Why the knob environment can be serialized.** Its readers are named: a case
reads a knob it bridged, so the case that writes one can hold the guard across
the reads that matter.

**Why the working directory cannot.** Its readers are implicit. Every relative
path the kernel resolves reads it — a relative knob default, `.tmp`, `.workflow`,
a `git` spawned with no `-C`. A reader cannot take a guard it does not know it
needs.

A lock over the working directory would therefore serialize its writers against
each other and leave every reader exposed. That guard's name would claim more
than its assertion covers. The two writers today already hold `knobenv::lock()`,
and that serializes them against each other only by where the lock happens to be
taken.

**Censused at this amendment's authoring, all 693 tests.** One test reaches the
working directory without the lock: `emit/drift_report.rs`'s
`the_iteration_name_drops_its_stage_tag_and_an_absent_queue_derives_nothing`.

- **The reach.** Its `iteration_start` spawns `git log` over the relative pathspec
  `./WORKFLOW-STATE.txt`, with no `-C`. That is the instance the ground above
  predicts: a reader that does not know it reads the working directory, spelled
  as a git argument no lock author would think to guard.
- **Why it cannot red today.** Its pickaxe matches no commit from any directory
  inside the repository, so its assertion is invariant to where a sibling test
  has moved the process. The race is latent, not live.
- **What that does to the entry's cost line.** The two attested
  `check-crate-arms` reds never read the failing test's name. They are at least
  as consistent with the stop-liveness module flake, measured beside this at 8 of
  30 isolated runs, as with this race.

The rule is prevention: the next such reader need not carry a pickaxe that
matches nothing.

No other process-global writer exists. Environment writes occur only in
`knobenv.rs`, and the crate spells no `umask` and no signal handler.

So the rule is elimination, not serialization: §run-gate-tests' invariant, applied
to the crate's own tests. A case directory is set on a spawn and never entered.

**Refused, with grounds:**

- **Widening `knobenv` to "process-global test state".** This was the entry's
  filed candidate. It serializes writers only, and it renames a module's charter
  past what its assertion can hold.
- **Running the two tests single-threaded through a split `check-crate-arms`
  argv.** That changes a gate's contract and CI's invocation, and a
  contributor's plain `cargo test` still races.
- **Threading a root argument into the members.** It deletes the coverage the
  tests exist for, because working-directory resolution *is* the production path
  (the same ground the knob passage gives for refusing parameters).

## Producers and consumers

- **The observer child** (delta 1).
  - *Producer:* each coverage test's per-case loop, spawning `current_exe()`
    through `proc::run_merged_in` with the case as working directory.
  - *Consumer:* the parent test, which reads the child's lines through the
    returned `Merged`'s `output()` and its exit status through `code()`. The
    member's name and the marker ride that call's `env` slice, which is
    child-scoped.
  - *Recorder boundary:* `run_merged_in` notes its own spawn into the calling
    thread's recorder under test, so the parent never has a recorder started
    across the observer spawn. Otherwise the `--needs` test would record its own
    harness re-exec as a member's spawn. Recording happens only inside the child.
  - The sentinel line's fields (a kind, then one root or program) have one reader:
    the parent's parse, at the `declaration_covers` assertion.
  - The child's exit status has one reader: the parent's `rc != 2` assertion.
  - The marker environment entry has one reader: the observer's guard, at entry.
  - Reached on every `cargo test` run, including `check-crate-arms`' test arm,
    which passes no filter.
- **The `set_current_dir` roster test** (delta 2).
  - *Producer:* the crate's source tree.
  - *Consumer:* `check-crate-arms`' test arm, at commit time and in CI's battery.
- **Red conditions** (point 5): no corpus narrows. The roster test's red condition
  is a spelling-count mismatch in either direction. So deleting the `pack()`
  call later reds the pin too, and that is intended: the allowlist must move with
  the site.

## Existing sections updated

- `gate-sdk/SPEC.md` §lib/gate.sh — the passage "The bridge is process-global, and
  the crate's own tests are serialized against it" (delta 3).
- `gate-sdk/SPEC.md` §Meta-gate conservation for the binary substrate, the
  `check-reads-couples` row's unit test **A** clause (delta 1). "Every member run
  over its own cases with recording on" gains "each case in a child whose working
  directory is the case".
- `gate-sdk/SPEC.md` §The `# graph:` manifest, unit test A for `--needs` (delta 1),
  with the same clause.
- `gate-sdk/SPEC.md` §Fail-closed contract, the spawn-funnel routing passage that
  names `spawn-funnel-exempt:` (delta 2). The `Command` roster gains its sibling
  `set_current_dir` pin in the same module, with the opposite scope rule and a
  `cwd-write-exempt:` valve on the same convention.
- `native/src/gates/mod.rs` (delta 1): the two tests, the observer, and their
  `spec:` lines.
- `native/src/proc.rs` (delta 2): the roster test.
- `docs/gate-sdk/SPEC.md` (all deltas). It is the generated mirror; regenerate it
  with `--emit docs-mirror --write`.

## Retired spellings

- None — no delta retires a literal. `set_current_dir` keeps its production site,
  and delta 2 pins that site rather than removing the name.

## Definition of Done

- [ ] **Causal completeness** — the observer child, its sentinel line and the
      roster pin each have a named producer and consumer, and every field has a
      reader.
- [ ] **No in-process working-directory write in any test** — the delta 2 pin is
      green, with exactly one allowlisted site.
- [ ] **Coverage unchanged** — each coverage test still reports its case count and
      its observation count, and both still exceed zero.
- [ ] **Merged with no information lost; amendment deleted** — none remain for the
      component.
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Gaps filed** — a cross-component gap found in build is resolved that
      session.
