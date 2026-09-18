# SPEC amendment: spawn-floor

**Two records are overdue a re-measurement.**

- **The published floor.** `PROBE_SET` (`native/src/toolfloor.rs`) states a floor
  that doctor, the env-probe arm and docs/install.md all repeat. Some of its
  forcing constructs no longer exist.
- **The registry's requirement declaration.** It compares a member's declared
  program against the literal string a spawn passed. So a member that resolves its
  interpreter to a path cannot declare anything its spawn would match.

**Measured at spec, 2026-09-18, against the tracked tree.** Each finding below
corrects a premise one of the two paired entries rests on.

- **`awk::GNU` has no forcing construct.**
  `git ls-files '*.sh' '*.rs' | xargs grep -ln '\bawk\b'` was read site by site. The
  awk *programs* that ship are all POSIX:
  - `installer/bin/checkwright.sh`'s digest read (`NR==1{print $1}`);
  - `context-kit/templates/session-context.sh`;
  - `drift-kit/templates/kpi-deprecated-surface.sh`;
  - `gate-sdk/templates/check-skeleton.sh`.

  Elsewhere awk is not run:
  - the binary spawns awk only in `native/src/ere.rs`'s test oracle;
  - `guard-kit/lib/guard.sh` names awk only as a word it inspects;
  - neither generated hook contains awk (`grep -c awk scripts/git-hooks/*` is 0).

  docs/install.md still says the hooks force awk. That ground is gone.
- **`sort::coreutils` still stands, on a different construct.** The recorded
  binding construct, `realpath --relative-to` in the gate library, has been
  re-expressed lexically in the binary (`native/src/gates/docs_link_convention.rs`,
  `native/src/emit/docs_mirror.rs`). `stat -c` has no live site. What still forces
  GNU coreutils is GNU `date -d`, spawned by `native/src/emit/kpi/mod.rs` and
  `native/src/emit/queue_index.rs`, with `sort -V` in the floor predicate itself.
- **The claim "the roster and the spawn set disagree in both directions" was an
  artifact of what scope's census measured.** Its oracle, `proc::run("<literal>"`,
  sees neither `run_streamed` nor a program passed through a variable.
  - `sort` is spawned (`toolfloor.rs` `floor_met`).
  - `shellcheck` is spawned (`check-shellcheck`).
  - `date`, `mktemp` and `cp` are spawned and unrostered because they sit on
    `GATE_SDK_PROGRAM_FLOOR`, the set the payload assumes present, which is not
    the set doctor probes.
  - `uname`, `ps` and `tar` are spawned and are on neither set (checked against
    `native/src/knobs/gate_sdk.rs`'s default). That is the real residue of the
    census, and delta 3 names it.

  A corrected survey block is filed beside this amendment.
- **`--needs` has no adopter-side reader.** Its readers are the CLI flag
  (`native/src/main.rs`) and `native/src/emit/port_blockers.rs`. Nothing in
  `installer/`, `docs/` or `.github/` reads it. The registry-needs entry's cost
  ground ("the roster an adopter provisions from") is therefore false.
- **No registry member spawns a resolved path today.** `proc::resolve_interpreter`
  has no production caller. `proc::resolve_floor_tool`'s three callers (toolfloor,
  doctor, env-probe) are not registry members. The construction is latent, as the
  entry says.
- **The fixture-vacuity half is already discharged as a stated honest limit** in
  gate-sdk/SPEC.md §The `# graph:` manifest ("The reach of that guarantee is bounded
  by the fixture corpus", naming `check-graph`). No delta here reopens it.

**What this unit does not reach, and where it went.** A git-only floor is not
reachable in this unit:

- bash stays forced, by the shipped shell front end, the gate library and the hooks
  (`native-windows-bash-floor`'s subject), and by the binary's consumer-command
  executors;
- coreutils stays forced by `date -d`.

Both are filed as a costed gap. So is the mechanization that would hold `PROBE_SET`
against a machine-read spawn set, which is blocked because an `ARMS` row carries no
requirement element.

## What changes

### (1) The published awk member narrows to an unconstrained `awk`

`PROBE_SET`'s `awk::GNU` becomes `awk` {mechanical}. docs/install.md's
`<!-- toolchain:begin -->` block renders the member as plain `awk`. Its bullet is
re-phrased: awk is reached by the activation bootstrap and the shipped templates,
all in POSIX awk, so no implementation is required. The paragraph deferring the
decision to `interpreter-floor-gawk-residue-empty` is deleted, because this delta
takes that decision. context-kit/SPEC.md §bin/env-probe's constrained-member list
drops the `awk::GNU` bullet. The "narrowing the element is owed" sentence goes with
it, since the narrowing is taken. **Not yet applied.**

The readers need no code change: doctor, the env-probe arm and
`check-install-toolchain` read the one array. The parity gate compares the rendered
block against the array, so the block and the array land in one commit.

### (2) Each remaining constrained member names its live forcing construct

Two passages are re-phrased so that each constrained member cites a construct the
tree still runs {mechanical}. **Not yet applied.**

- **context-kit/SPEC.md §bin/env-probe, the `sort::coreutils` bullet.** GNU
  coreutils is forced by `date -d` in the binary's KPI and queue-index arms. The
  floor predicate's own `sort -V` is its representative. `realpath --relative-to`
  and `stat -c` leave the bullet, because neither has a live site.
- **docs/install.md, the `sort` (coreutils) bullet.** Re-phrased the same way.

The `bash:4.3` bullet stands: the nameref in `gate-sdk/lib/gate.sh` is live. The
`cargo:1.71::contributor` bullet stands unchanged.

### (3) The roster's relation to the payload's assumed set is stated

context-kit/SPEC.md §bin/env-probe, "What it probes", gains one re-phrased sentence
{design-bearing}. **Not yet applied.**

- `PROBE_SET` is what doctor verifies before an install.
- The binary spawns programs that are off it. `date`, `mktemp` and `cp` rest on
  `GATE_SDK_PROGRAM_FLOOR`'s assumption (gate-sdk/SPEC.md §lib/gate.sh) and are
  not probed.
- `uname` (the env-probe arm), `ps` (the pid predicate's fallback leg) and `tar`
  (the installer packer) are on neither set. Each is named with the arm that
  spawns it.
- Nothing yet holds the sets in a checked relation, and that mechanization is the
  filed gap.

Stated so a later census does not read an assumed program's absence from the roster
as drift. This is an honest limit, not a rule the roster is derived by.

### (4) A declared program is a requirement, matched by name

gate-sdk/SPEC.md §The `# graph:` manifest rules the `<program>` line kind a
**requirement**: the program a host must carry, named as `PATH` names it, never the
literal argv[0] a spawn happened to pass {design-bearing}. **Not yet applied.**

- **The recorder notes names.** `proc.rs`'s test-scoped recorder notes the spawned
  program's **name**:
  - a bare name is noted as passed;
  - a value containing a path separator is noted as its final component, minus the
    host's executable suffix (`std::env::consts::EXE_SUFFIX`).

  A member that spawns `resolve_interpreter("bash")`'s absolute path therefore
  declares `bash` and is covered. `declaration_covers` is unchanged, because it
  compares what the recorder hands it and its `walked` reader has no paths to
  normalize. A `?<TAB><knob>` declaration still absorbs the knob's command word
  through the wildcard, whatever form that word takes.
- **Why a requirement rather than a literal.** `--needs`' one machine reader,
  port-blockers' default arm, filters each line against `GATE_SDK_PROGRAM_FLOOR`,
  which is a set of names. An absolute path never matches it, so under the literal
  reading a resolving member's requirement would be reported off-floor on every
  host. The literal reading also makes host-independent declaration impossible for
  exactly the members objective 2 needs to resolve.
- **Unit test.** A recorder test drives `proc::run` with an absolute path to a
  program the test host carries, and asserts that the name is noted. On a Windows
  host it drives the suffixed form too.
- §Fail-closed contract's recorder paragraph ("notes the program it is about to
  spawn") becomes "notes the name of the program it is about to spawn, a path
  reduced to its final component", with the ground above cited, not restated.

## Producers and consumers

- **The narrowed member (delta 1).**
  - Producer: `PROBE_SET`.
  - Consumers: doctor's roster walk (`native/src/installer/doctor.rs`), which now
    accepts a non-GNU banner; the env-probe arm's verdict column; and
    `check-install-toolchain`'s quadruple parity against the docs block.
  - Enabling config: none, because the roster carries no knob.
- **The forcing-construct prose (delta 2) and the roster relation (delta 3).**
  Readers are the next census and the next narrowing session. No machine reader.
- **The recorder's name normalization (delta 4).**
  - Producer: `proc.rs` `recorder::note`, test-scoped.
  - Consumer: unit test A (`every_registry_member_declares_the_programs_it_spawns`),
    through `recorder::stop`.
  - Field read: the name alone.
- **Roster-holding readers.** No knob, gate, arm, tag or line kind is minted. New
  `// spec:` lines bind to §The `# graph:` manifest or §bin/env-probe.
- **Point 5 (narrowing).** Delta 1 narrows a roster element. The red conditions of
  its readers:
  - `check-install-toolchain` reds on a per-name quadruple mismatch in either
    direction, so it reds until the block and the array agree. Delta 1 lands them
    together.
  - Doctor reds on a member's `absent`, `below` or `wrong-impl` verdict. An
    unconstrained member can no longer be `wrong-impl`, so doctor's verdict can
    only move from red to green.
  - The Windows install-smoke leg asserts doctor exits 0 (`.github/workflows/gates.yml`).
    It can only stay green.
  - `scripts/gate-tests/check-install-toolchain/{good,bad}/probe.sh` carry their
    own rosters and do not read `PROBE_SET`.
  - Delta 4 narrows nothing the test asserts: the observed set can only lose
    distinct spellings of one name.
- **Point 6 (members).** Delta 4 obliges every registry member whose observed spawns
  must remain covered. The recorder's spawn values are enumerated by
  `grep -rhoE 'proc::[a-z_]+\(\s*"[A-Za-z0-9_.-]+"' native/src`. Every literal is a
  bare name, so normalization leaves each noted value unchanged. Members spawning
  through a variable (a knob's command word, `GATE_SDK_NATIVE_BIN`) are declared
  `?` or `?<TAB><knob>` and stay absorbed by the wildcard. No member's declaration
  needs to change.

## Existing sections updated

- `native/src/toolfloor.rs`: `PROBE_SET`'s awk element and its test literals
  (delta 1).
- `native/src/installer/doctor.rs`: a unit test uses `awk::GNU` as its sample
  implementation-constrained element for the `wrong-impl` rendering. It is
  re-pointed to `sort::coreutils` with a non-coreutils banner, so no test holds up a
  roster element the roster no longer carries (delta 1).
- `native/src/emit/env_probe.rs`: the same re-pointing, on its `wrong-impl` render
  test (delta 1).
- `docs/posts/2026-07-26-checkwright-v0-16-0.md`: a dated release post that states
  the floor as it was then. It is history and stays unedited (delta 1).
- `docs/install.md` §Requirements: the toolchain block's awk and sort bullets
  (deltas 1 and 2).
- `context-kit/SPEC.md` §bin/env-probe: the constrained-member list and the "What
  it probes" paragraph (deltas 1, 2 and 3).
- `gate-sdk/SPEC.md` §The `# graph:` manifest: the `<program>` line kind ruled a
  requirement (delta 4).
- `gate-sdk/SPEC.md` §Fail-closed contract: the recorder paragraph (delta 4).
- `native/src/proc.rs`: `recorder::note` and its unit test (delta 4).
- `.workflow/release-declarations.md`: one bullet declaring that doctor and `init`
  no longer require GNU awk (delta 1).
- `TASK-QUEUE.md`, at merge, in the build session that merges this file and before
  the drain stage is entered (all deltas):
  - `toolchain-floor-spawn-on-native-windows` and
    `registry-needs-conflates-requirement-and-spawn` move to Done;
  - the Icebox one-liner `interpreter-floor-gawk-residue-empty`, whose decision
    delta 1 takes, is retired to Done in the same commit;
  - that commit's message names delta 4's requirement-by-name ruling as what closed
    `registry-needs-conflates-requirement-and-spawn`, and delta 1 as what closed
    the gawk entry.
- `docs/context-kit/SPEC.md`, `docs/gate-sdk/SPEC.md`: generated mirrors,
  regenerated (all deltas).

Roster produced by `git grep -n "awk::GNU\|awk\` (GNU)\|relative-to\|recorder::note"`
and `git grep -n -- "--needs"` over the tracked tree, excluding `TASK-QUEUE.md`. It
is a floor that build re-derives.

## Retired spellings

- `awk::GNU` — the implementation-constrained roster element (delta 1).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each delta.
- [ ] **Roster and block agree**: `check-install-toolchain` is clean, and the
      env-probe arm and doctor both report `awk` as `ok` on this host.
- [ ] **Name normalization held**: the recorder test passes, and unit test A stays
      green over the whole registry.
- [ ] **Merged with no information lost**: the measured grounds above (which awk
      sites exist, which coreutils construct binds, why the declaration is a
      requirement) survive in the merged prose.
- [ ] **Queue moves placed before the drain stage**, including the icebox
      retirement.
- [ ] **Amendment deleted**: this file is removed on merge.
- [ ] **Gaps filed**: the unreachable floor remainder and the ARMS
      requirement-element mechanization are in the gap inbox (filed at spec), along
      with any gap build finds.
