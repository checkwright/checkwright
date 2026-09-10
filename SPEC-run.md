# SPEC amendment: run

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land; where replacement wording is given it is marked **Not yet applied** at the passage itself.
`spec` authors, build lands, and a reader arriving mid-iteration must not read a quoted sentence as
one already in the tree.

**The basename is short on purpose.** The entry's slug is 53 characters and the promotion writes two
lead-line tags beside it, which leaves nineteen columns inside `QUEUE_KIT_WRAP_BUDGET`. A shorter
basename than the slug is canon-kit's own stated fix for that squeeze, not a mismatch to avoid, and
`run` is the function family this amendment is about: `proc::run*`.

## The defect in one sentence, and why the obvious fix was already ruled out

The packer's tool preflight asks `proc::on_path("npm")`, which walks `PATH` over the `PATHEXT`
candidate set and answers **yes**; the packer then spawns the bare name `npm`, which Windows
resolves by its own rules and answers **no**. Measured at `gates` run `34522661342`:
`install-smoke-windows` and `install-smoke-powershell` are that run's only two failures, both
carrying `pack-installer: cannot run npm: program not found` — the *spawn's* text, not the
preflight's (`npm not found on PATH - the pack step cannot run`), so the probe demonstrably passed.

The entry already ruled out the obvious repair with its cause: routing the spawn through `which()`
returns the **extensionless** `npm` sh script that Node's Windows install leaves beside `npm.cmd`,
because `exe_candidates` puts the bare name **first** and `is_executable` on Windows is `p.is_file()`
alone. `CreateProcessW` cannot run that file, so the change would move the error message and nothing
else. A working repair must therefore do **two** things: make the spawn resolve, and make the
`PATHEXT` variants beat the bare name where the platform itself would.

## The fix site, settled from the specs rather than escalated

The entry frames two blast radii — "the narrow call-site fix in the packer" against "a resolver
every gate spawns through" — and says the narrow one "is not obviously the cheaper of the two."
Two stated rules decide it jointly, and neither is cited on the entry:

- **gate-sdk/SPEC.md §Fail-closed contract**, on the candidate set: it is *"owned by
  `exe_candidates` alone so no call site spells an extension."* A packer-local fix that spells an
  extension or an ordering is already forbidden. **The narrow site is not available as framed.**
- **gate-sdk/SPEC.md §check-graph**, on `resolve_interpreter`: *"Two others take the same mechanism,
  each on its own witnessed red and never on a sweep."* The **mechanism** is shared and lives in
  `proc.rs`; **adoption** is per-site and a witness is what points one.

So the answer is neither radius: the mechanism lands in `proc.rs`, the crate's stated single owner
of what an installed program may be named, and it applies to the packer's `npm` spawn on its
witnessed red. That is the shape the tree already runs for `bash`, and it needed no ruling — only
the two sentences above, read together.

## What made the two-radii framing wrong, measured rather than argued

The framing assumed `npm` is one site. It is one member of a **seven-member class**, and the class
was censused at this stage over `native/src` rather than estimated. Shipped-path modules that call
`on_path(X)` and then spawn `X` by bare name:

| module | probe | bare spawn |
|---|---|---|
| `evidence.rs` | `:332` `ps` | `:335` |
| `emit/queue_edges.rs` | `:60` `git` | `:71`, `:75` |
| `hook/poll.rs` | `:38` `curl` | `:111` |
| `emit/pack_installer.rs` | `:121` `npm`/`git`/`tar` | `:562`, `:347`+`:405`, `:418` |
| `gates/crate_arms.rs` | `:106` `CARGO` | `:53` |
| `gates/shellcheck.rs` | `:54` `PROGRAM` | `:77` |
| `gates/action_run_shell.rs` | `:788` `PROGRAM` | `:640` |

The last three are invisible to a literal-matching oracle because the program is a named `const`,
which is why they were found by reading. The census also counted the **whole** bare-literal spawn
population on the shipped path: `git` **111** sites, `bash` 24, `date` 7, `mktemp` 5, `jq` 2, and one
each of `uname`, `tar`, `npm`, `curl`, `cp`, `ps`. A call-site sweep over that population is not a
shape anyone can cost, and it is refused here on the measurement rather than on doctrine.

The pattern done **right** already exists three times and is what the deltas generalize:
`installer/doctor.rs:31-36`, `toolfloor.rs:124-125` and `emit/env_probe.rs:33-34` each probe, then
spawn the **resolved** value.

## What changes

### (1) The candidate set is ordered the way the platform orders it

`exe_candidates` puts the `PATHEXT` variants **before** the bare name; the bare name stays a
candidate and moves last {design-bearing}.

**This narrows a stated contract without reversing it, and the check is textual.**
gate-sdk/SPEC.md §Fail-closed contract says: *"The bare name stays a candidate, so a caller naming
`cargo.exe` and a Unix host both resolve through the same loop."* Both grounds survive intact.
`cargo.exe` remains in the list, at the end, and still resolves because no `cargo.exe.EXE` exists. A
Unix host passes `pathext == None`, whose arm returns `vec![program]` and has **no ordering to
change** — so not one verdict moves on any platform the battery runs on. What moves is an ordering
the sentence never stated, and it moves toward the platform's own rule: `cmd.exe` resolves a typed
name through `PATHEXT` and will not execute an extensionless file at all, so bare-first was the
POSIX rule applied on a host that does not use it.

**Not yet applied.** The sentence gains one clause:

> The bare name stays a candidate, so a caller naming `cargo.exe` and a Unix host both resolve
> through the same loop — but it is the **last** candidate, not the first. Windows resolves a typed
> name through `PATHEXT` and will not execute an extensionless file, so a bare-first order is the
> POSIX rule applied on the one platform that does not use it, and it is what let an extensionless
> `npm` sh script beat the `npm.cmd` shim that Node's own Windows install ships beside it.

Design-bearing because the delta is a contract narrowing and its blast radius had to be established
rather than assumed. It was: `proc::which`'s **value** has exactly two readers,
`emit/env_probe.rs:49` and `:101`, both rendering a path for a human; `proc::on_path`'s **boolean**
cannot move at all, the candidate set being the same set in a different order; and
`resolve_outside_system_dir` shares `exe_candidates`, so `resolve_interpreter` and
`resolve_floor_tool` inherit the order and get the `.exe` they already wanted by luck.

### (2) The spawn resolves through the same function the probe resolves through

Every `proc::run*` helper resolves its program before `Command::new`, on Windows only, falling back
to the bare name where nothing resolves {design-bearing}. The invariant this buys, and it is the
whole unit in one sentence:

> On every host, if `on_path(P)` answers true then the spawn of `P` reaches the file `which(P)`
> named. One resolution serves the probe and the spawn, so the two cannot disagree.

Three properties are load-bearing and each is stated so build does not have to re-derive it.

**It is a pass-through off Windows, and that is not an optimisation.**
`proc::resolve_floor_tool`'s own `#[cfg(not(windows))]` arm already states the ground: *"resolving
here would swap the spawned literal for an absolute path on every host the battery runs on."* The
funnel takes that arm verbatim. Every POSIX verdict is byte-identical.

**It falls back rather than refusing.** Where nothing resolves, the bare name is spawned exactly as
today and today's error text is what a caller sees. So the delta is monotone: no gate can newly red
on it, and no adopter meets a refusal that did not already exist. This is `resolve_floor_tool`'s
posture, not `resolve_interpreter`'s, and the two are kept apart deliberately — the refusing
resolver's ground is a *system-directory homonym*, which is delta 1 of SPEC-interp.md and not this
unit's business.

**It resolves AFTER `recorder::note(program)`, and that placement is the whole reason it is safe.**
`proc::recorder::note` is called at the top of each helper with the argument as passed, and it is
the channel `every_registry_member_declares_the_programs_it_spawns` observes. Resolving after the
note leaves every registry declaration matching by construction. This matters beyond tidiness: the
deferred entry `registry-needs-conflates-requirement-and-spawn` records a **live** disagreement —
`gates/mod.rs` declares `check-graph`'s requirement as bare `bash` while `graph.rs` spawns a
resolved absolute path, and `declaration_covers` matches by exact equality. That disagreement exists
because the two pointed repairs resolve **at the call site** and hand `proc::run` a path. A funnel
inside the owner cannot produce it. This amendment does **not** answer that entry's open contract
question — whether a `# graph:` declaration is a host REQUIREMENT or a literal argv[0] — and takes
care not to: under the funnel the question has no live instance whichever way it is later ruled.

### (3) What `Command::new` does with a resolved `.cmd` is an observation, so the fix is a ladder and the run selects its rung

The unit lands **rung 1** and names the oracle that decides whether rung 2 is owed
{design-bearing}.

- **Rung 1 — the funnel's own spawn.** `Command::new(<resolved path>)`. For a resolved `.exe` or
  `.com` this is settled. For a resolved `.cmd` or `.bat` it is not: `std`'s handling of batch files
  changed across releases and the argument-escaping repair for CVE-2024-24576 landed in **1.77.2**,
  while `native/Cargo.toml:5` pins `rust-version = "1.71"` below it. What that pin's `std` does with
  a `.cmd` is not derivable on Linux.
- **The oracle.** The next `gates` run on `install-smoke-windows`. A green leg means rung 1 is
  sufficient and rung 2 is never built. A red leg naming a bad executable format means rung 2 is
  owed, and the leg's own log says which.
- **The push that produces it is already granted and is not a new placement.** TRAJECTORY.md's
  iteration-scoped three-push grant (operator, 2026-09-10) allocates them by purpose in its own
  words — *"One at build to buy the Windows observation, one at close, one for the release tag."*
  So the build-stage push this rung selection waits on **is** the grant's first push. Nothing here
  asks for a new authorization and nothing here is a fresh ruling; this bullet records the standing
  grant applying, which is what a later reader needs in order not to re-buy the question. It is
  also, independently, what SPEC-obs-drain.md's placement rule would ask of an `[observed-by:]`
  iteration, so the two agree and neither is load-bearing on the other.
- **Rung 2, specified now so a red run does not cost a design round.** A named helper that invokes
  the resolved batch file through the command processor, taken by a call site that **opts in**
  rather than by the funnel. It is opt-in precisely because of the CVE class: composing a command
  processor's command line is where argument injection lives, and the pin's `std` does not escape
  for it. The opt-in restricts rung 2 to a call site whose argv is a **compile-time literal set with
  no caller-supplied string** — the packer's `&["pack"]` is exactly that, and an argv carrying
  interpolated data cannot reach the helper by accident because reaching it is an edit.

**Raising the pin is the third option and it is declined with in-tree grounds, not on taste.**
Moving to 1.77.2 would hand the problem to `std` and delete rung 2. Two stated costs refuse it.
gate-sdk/SPEC.md already declined `trim-paths` on the same axis — *"taking it would raise the
toolchain floor, which runs against the objective that exists to collapse that floor rather than
raise it"* — and it records, by controlled experiment rather than inference, that an MSRV bump
**un-suppresses clippy lints against unchanged code**, the 1.56→1.71 move having surfaced four
findings in modules its cohort never edited. A floor move is its own unit with its own clippy
budget, and smuggling one in as a Windows spawn fix would be the widest possible change for the
narrowest observed cause. Recorded here so a later reader meets the decline with its reasons rather
than re-deriving them.

### (4) The packer's preflight roster is corrected on the converse gap the census turned up

`emit/pack_installer.rs` probes `["npm", "git", "tar"]` and then spawns a fourth program,
`mktemp` at `:365`, with no preflight at all {mechanical}.

That is the same asymmetry pointing the other way: a program the pack step needs, whose absence
reports as a generic spawn failure rather than as the member's own documented refusal at the shell
form's own point in the order — which is the whole of what §Fail-closed contract's wrapper contract
exists to buy. `mktemp` joins the probed set. Mechanical: the roster is a literal array and the
refusal text is already parameterised by `tool`.

## Producers and consumers

**No new state, no new event, no new file, and no new emitted artifact.** One new **interface** (the
funnel's resolution step) and one conditional new interface (rung 2's helper). Surveyed across the
whole component set — `native/src` in full, `installer/`, and every gate registry declaration — with
no stderr suppressed on any path grep.

- **The funnel's resolution step** (delta 2). *Producer:* every `proc::run*` helper, on every spawn,
  on Windows. Its enabling config is **none** — the behaviour is compiled in under `cfg(windows)`
  and no knob turns it on, which is deliberate: a knob would be a second way for the probe and the
  spawn to disagree, which is the defect. *Consumers:* the operating system, which receives a path
  rather than a name; and `proc::recorder`, which by placement receives the **unresolved** name and
  is therefore unchanged. *Existing integration prose updated:* §Fail-closed contract's `on_path`
  bullet, which today describes the probe's resolution and is silent on the spawn's — the silence
  that made the two divergeable.
- **The reordered candidate set** (delta 1). *Producer:* `exe_candidates`, unchanged as the single
  owner. *Consumers, all four named with the transition each reads at:* `which` (at a probe, for the
  boolean and the path), `resolve_outside_system_dir` (at an interpreter or floor-tool resolution),
  the funnel (at every Windows spawn), and `proc.rs`'s own unit tests, whose fixed expectation
  vectors encode the order and must be re-ordered with it or they assert the old contract.
- **Rung 2's helper** (delta 3), conditional. *Producer:* built only if the oracle red fires.
  *Consumer:* exactly one opt-in call site, the packer's `npm pack`. *Every field has a named
  reader:* the helper carries no new field; its argv restriction is a property of the call site, not
  a parameter, precisely so there is no field a caller could set wrongly.
- **The field that is deliberately not added.** A knob naming which rung to take was considered and
  is **refused**: it would be a field whose reader is a human guessing at a platform question the
  run already answers, and it would let a host silently take the wrong rung.

**Two producer/consumer edges cross inside this iteration's batch A, and the lead sequences both.**
SPEC-interp.md's governed interpreter-name set is a **consumer of delta 2's funnel** — it upgrades
the funnel's resolution for one name class and has nothing to attach to without it. So this unit
lands first, or the two land together. And SPEC-interp.md's revert of the two call-site resolutions
in `gates/graph.rs` and `installer/init.rs` is only correct once the funnel exists.

**No edge crosses to batch B.** That batch touches `lifecycle-kit` and `queue-kit`; this one touches
`native/`, `installer/` and `gate-sdk/`. Nothing either emits is the other's input.

## Existing sections updated

- `gate-sdk/SPEC.md` §Fail-closed contract, the `on_path` bullet's candidate-set sentence — the
  ordering clause added (delta 1).
- `gate-sdk/SPEC.md` §Fail-closed contract, the same bullet — the spawn's own resolution stated
  beside the probe's, which is the silence that let them diverge, together with the invariant naming
  the two as one resolution (delta 2).
- `gate-sdk/SPEC.md` §Fail-closed contract, the wrapper-contract preamble — `run`'s `Err` arm is
  described as *"the backstop for a program that vanishes between the probe and the spawn"*, which
  is now the only case it backstops rather than one of two (delta 2).
- `installer/README.md` §The packer gains the preflight tool set's documentation for the first
  time — `mktemp` joins `npm`, `git` and `tar` as a probed member, and the section states the
  converse gap this delta closes. **Align-found, not this delta's own drift:**
  `native/src/emit/pack_installer.rs:119`'s `# spec:` comment already cites this section for a
  *"the preflight tool set narrows with the spawned set"* note that installer/README.md has never
  actually carried; this delta is what finally lands the text the comment has been promising
  (delta 4).
- `native/src/proc.rs`'s own `# spec:` comments on `exe_candidates`, `which`, and each `run*` helper
  — the resolution step is a directive a reader of the spawn needs at the spawn (deltas 1 and 2).
- `native/src/proc.rs`'s unit tests `a_populated_pathext_is_read_rather_than_the_fallback`,
  `an_absent_or_blank_pathext_falls_back_to_the_default_set` and
  `a_platform_without_the_suffix_question_probes_the_bare_name_only` — the first two encode the
  candidate order in a literal vector and assert the old contract until reordered; the third asserts
  the Unix arm and must be **unchanged**, which is the delta's own no-op proof (delta 1).

## Retired spellings

- None — no delta of this amendment retires a spelling. Delta 1 re-orders a list and delta 2 adds a
  step; both keep every existing name, and `exe_candidates`, `which`, `on_path`,
  `resolve_interpreter` and `resolve_floor_tool` all survive with their current spellings and their
  current callers. Rung 2's helper, if the oracle calls for it, is a **new** name and retires
  nothing.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired
      spellings` above, and `check-amendment-retired-spelling` runs each declaration against the
      whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The Unix no-op is asserted, not claimed** — a test pins that the non-Windows arm returns the
      program unaltered, on the shape `a_posix_floor_tool_is_spawned_under_its_bare_name` already
      has, so the pass-through cannot rot into a resolution.
- [ ] **The ordering is asserted from a host that cannot execute it** — the Windows arms are
      exercised by injected `(PATH, PATHEXT, existence-predicate)` inputs, the
      cannot-exercise-locally doctrine §Fail-closed contract already names, with a case in which the
      extensionless file and the `.CMD` shim both exist and the shim wins.
- [ ] **The oracle is read, not assumed** — delta 3's rung is selected by an actual
      `install-smoke-windows` result, and whichever rung lands, the reading is recorded on the
      entry before the unit is called done.
