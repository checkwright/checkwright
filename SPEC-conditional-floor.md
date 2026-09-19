# SPEC amendment: conditional-floor

TRAJECTORY objective 1 rules that git is the only unconditional floor member,
and that a program a selected kit or arm spawns is that kit's declared
requirement, probed only where it is selected. Today `doctor`, and through it
`init`, holds every adopter to `jq`, `curl` and `shellcheck` whatever they
selected, and the binary's `date`, `mktemp`, `cp` and `ps` spawns rest on a host
assumption nothing probes or declares.

This amendment lands rungs 2 and 3 of the adopter-floor ladder and gives the
coreutils residue its dispositions:

- **Rung 2.** `jq` and `curl` become members owed only where the kit reaching
  them is selected: guard-kit's hook for `jq`, delegation-kit's `--usage-poll`
  arm for `curl`.
- **Rung 3.** `shellcheck` is owed only where a registered gate declares it, and
  `check-shellcheck` stops registering at `init`.
- **The residue.** `date` leaves the binary's unix spawn set. `mktemp` and `cp`
  are contributor-only. `date` and `ps` off unix ride Git for Windows' userland
  beside `bash`.

It is a root-level amendment because it spans `native/`, context-kit, gate-sdk,
installer, the site's `docs/install.md` and the `gates` workflow.

**Where this leaves the starter and prose profiles.** Their resolved floor
becomes `git` and `bash`. That is still not the git-only floor the objective
names: `bash` is rung 4a (`floor-bash-hooks-front-end`). No surface this
amendment writes calls either profile git-only.

The tree does not already do this. `native/src/toolfloor.rs` `PROBE_SET` carries
`jq`, `curl` and `shellcheck` with an empty audience, and `doctor::diagnose`
filters on `contributor` alone. `gate-sdk/checks/check-shellcheck.gate` declares
`# install: zero-config`, and `date` is spawned at `enter_stage.rs`,
`run_validate.rs`, `env_probe.rs` and `hook/wakeup.rs` on every host.

## What changes

**Batching.** Deltas 1 to 4 and 7 land in one commit: `check-install-toolchain`
reds while the page's bullets and `PROBE_SET`'s audience fields disagree, and the
consumer smoke's `jq`-less arm reds on the old assertion the moment doctor stops
refusing. Deltas 5 and 6 are independent and may land in their own commit. Delta
8 lands with delta 7's page edit or after it, never before: the legs read the
page's blocks.

### (1) The audience axis gains two conditional values {design-bearing}

**Not yet applied.** The fourth roster field (context-kit/SPEC.md
§bin/env-probe) names whose floor a member is, and its closed value set widens
from `contributor` to three kinds:

- **empty** — every adopter. Unchanged.
- **`contributor`** — no install-time role. Unchanged.
- **a kit name** — owed where that kit is selected. The value is the kit's
  directory name.
- **`registered`** — owed where a registered gate's requirement element (the
  data `--needs` prints, gate-sdk/SPEC.md §check-reads-couples) names the
  member. Derived from the registry, never listed.

`PROBE_SET` becomes, element for element:

| element | condition | the reach that forces it |
|---|---|---|
| `bash:4.3` | every adopter | unchanged; rung 4a |
| `git` | every adopter | the model itself |
| `jq:::guard-kit` | guard-kit selected | `guard-kit/lib/guard.sh` |
| `curl:::delegation-kit` | delegation-kit selected | `--usage-poll` (`native/src/hook/poll.rs`) |
| `shellcheck:::registered` | a registered gate declares it | `check-shellcheck`, `check-action-run-shell` |
| `cargo:1.71::contributor` | contributor | unchanged |

`toolfloor` owns the predicate that answers *is this member owed under this
selection*. A **selection** is a kit set and a registered gate set. The answer
has three values:

- **owed** — an empty audience, a kit name the kit set carries, or `registered`
  where some member of the gate set has a `REGISTRY` row whose requirement
  element names the member.
- **not owed** — `contributor`, a kit name the kit set lacks, or `registered`
  with no such row.
- **undecided** — a conditional value with no selection to read.

A registered name with no `REGISTRY` row is a consumer-declared shell gate. It
contributes nothing, because a consumer command is the consumer's requirement
(gate-sdk/SPEC.md §The program roster).

**The value set is held closed by a unit test in `toolfloor.rs`.** Every
audience value is empty, `contributor`, `registered`, or the name of a kit root
the authoring tree carries (`walk::kit_roots`). A misspelled kit name would
otherwise be a condition nothing ever satisfies, and the member would silently
leave every floor.

**`native/src/programs.rs`:**

- The `JQ`, `CURL` and `SHELLCHECK` rows carry the same audience string as their
  `PROBE_SET` element, which assertion B already demands.
- Assertion A's adopter-side filter changes from *empty audience* to *any
  audience but `contributor`*. It still holds each such row to the floor's
  default, to `PROBE_SET`, or to the payload. Without this change the three
  conditional rows would leave the assertion altogether.

### (2) doctor renders the floor a selection owes {design-bearing}

**Not yet applied.** `doctor::diagnose` takes the selection, and each caller
supplies it:

- **`init`** resolves the profile's kit set before calling doctor. Today
  `profile::kits` and its empty-set refusal run just after doctor; they move
  ahead of it. Both precede any write, so the ordering rule in installer/SPEC.md
  §init still holds, and a profile resolving to no kit stays a payload refusal
  at exit 2, never a toolchain fault. The gate set is `profile::gate_set` for
  that profile, united with the members (`registry::members`) of a registry
  already on disk at `<gates-dir>/gates.list`. The union matters because `init` protects an edited
  registry rather than rewriting it, so a gate the adopter registered by hand
  still reaches the battery.
- **Bare `doctor` inside an install** reads the kit set from the manifest's
  `kits` field and the gate set from the members (`registry::members`) of the
  registry the manifest records. It reads these before rendering the toolchain block, while
  the output keeps its order.
- **Bare `doctor` with no install**, or over a residue, has no selection.

Rendering per member:

- **owed** — probed and rendered as today. It sets the verdict as today.
- **not owed** — skipped outright, on the ground the `contributor` skip already
  states: showing an adopter a tool they do not need invites them to install it.
- **undecided** — rendered unprobed, as
  `  jq           not probed — owed where guard-kit is selected`, or, for
  `registered`, `owed where a registered gate needs it`. It never sets the
  verdict.

Unit cases, on the predicate and on `render_member`'s caller:

- A selection of `{gate-sdk}` with a gate set naming no `shellcheck` requirement
  owes only `bash` and `git`.
- Adding `guard-kit` to the kit set owes `jq`.
- A gate set carrying `check-action-run-shell` owes `shellcheck`.
- No selection renders `jq`, `curl` and `shellcheck` as `not probed` and leaves
  the verdict to `bash` and `git`.

### (3) `check-shellcheck` becomes `on-surface` {design-bearing}

**Not yet applied.** `gate-sdk/checks/check-shellcheck.gate` declares
`# install: on-surface`. For an adopter, the subject is the shell they author
under their gates directory. The vendored kit shell it would otherwise lint is
the publisher's, and the publisher's own battery already lints it. So `init`
stops registering the gate in every profile, and the zero-config set no longer
carries a member whose requirement element names `shellcheck`. An adopter who
authors a gate script registers the gate, and doctor then owes `shellcheck`
through delta 2's `registered` arm.

gate-sdk/SPEC.md §check-shellcheck, *What ends at the port*, is rewritten:

- The gate is `on-surface`: it ships to every adopter who vendors gate-sdk, and
  it arms when their own shell exists.
- The paragraph's deregistration reasoning for *this* tree is unchanged.

The retirement table's `check-shellcheck` row (§Meta-gate conservation for the
binary substrate) takes the same one-word correction.

`installer/profiles.list`'s starter comment stops listing *a broken shell script
under their gates directory* among the zero-config catches. The clause still
holds through the other three.

### (4) The consumer smoke's `jq`-less arm asserts the conditional floor {design-bearing}

**Not yet applied.** `installer/consumer-smoke/run-smoke.sh`'s `jq`-less arm
keeps its per-arm mask and its `diff` / `uninstall --dry-run` assertions. What
it asserts about `init` and `doctor` inverts:

- **`doctor` in a directory with no install** runs first. It exits 0 on a host
  meeting `bash` and `git`, and renders `jq` as `not probed`. The kit its line
  names is the kit the next two assertions use, so the harness names no kit.
- **`init --profile "$PROFILE_MIN"`** on the `jq`-less `PATH` exits 0. The
  lattice minimum selects no kit owing `jq`, so the floor it meets carries none.
  This replaces both `assert_jq_blocked` calls on the minimum.
- **`init` at a profile whose kit set carries that kit** exits 1, names the
  toolchain floor, and renders `jq` as `NOT FOUND`, with no manifest left behind.
  The profile is found through the harness's own `profile_kits` reader.
  `assert_jq_blocked` keeps this one call.
- **`doctor` inside the minimum's install** exits 0 and renders no `jq` line.

installer/SPEC.md §The consumer smoke, *The `jq`-less arm*, is rewritten to
match. installer/SPEC.md §Requirements' *A `jq`-less machine is still refused an
install* paragraph becomes: a `jq`-less machine is refused only where the
selection carries the kit that reaches `jq`.

### (5) `date` leaves the binary's unix spawn set {design-bearing}

**Not yet applied.**

- **The three `date +%F` sites** — `enter_stage.rs` `date_today`,
  `run_validate.rs` `date_today` and the env-probe arm's date line — call
  `emit::kpi::today_iso()`. That helper is in-process on unix, already serves
  `file_gap`, `kfric`, `file_install`, `file_survey`, `overhead_meter` and
  `stage_economics`, and spawns `date +%F` only off unix. An empty answer takes
  each caller's existing refusal.
- **`hook/wakeup.rs` `local_stamp`** builds its `date -Is` timestamp in-process
  on unix: `YYYY-MM-DDTHH:MM:SS±HH:MM` from the current epoch and the local
  offset `libc` reports for it. The shape is byte-compatible with GNU
  `date -Is`, and a unit test pins it against a fixed epoch and offset. The log
  line the close triage reads keeps its grammar (guard-kit/SPEC.md
  §wakeup-guard). Off unix the spawn stays.
- **`programs::DATE`** becomes `#[cfg(not(unix))]`, as `PS` already is. A unix
  build names it nowhere, and a row nothing names is a `dead_code` finding under
  `check-crate-arms`.

After this delta, `date` and `ps` are spawned only off unix. There they resolve
from Git for Windows' `usr/bin`, the userland the `bash` member already requires
on that platform (docs/install.md §Requirements). They are not new members. They
share `bash`'s package and rung 4a decides their fate with it.

`GATE_SDK_PROGRAM_FLOOR` keeps `date`, `mktemp`, `cp` and `ps`, because the
shipped shell may still assume them.

### (6) `mktemp` and `cp` take the contributor audience {mechanical}

**Not yet applied.** The `MKTEMP` and `CP` rows in `native/src/programs.rs`
carry `contributor`. Every spawner is a source-clone arm:

- `--run-demo`, `--run-consumer-smoke`'s builder (`csmoke.rs`),
  `--upgrade-smoke` and `--agents-md-smoke` need the kit sources and a
  cargo-built binary.
- The payload withholds `smoke/` (`GATE_SDK_PAYLOAD_WITHHOLD`).
- `--pack-installer` is the publisher's own.

Enumerated by `grep -rn "programs::\(MKTEMP\|CP\)\b" native/src`. With the
contributor audience, assertion A stops binding them, and a later adopter-side
spawn of either would have to reclaim an adopter audience.

### (7) The install page states the conditional floor {design-bearing}

**Not yet applied.** In docs/install.md §Requirements:

- **The toolchain block.** The `jq`, `curl` and `shellcheck` bullets carry the
  new parentheticals `(@guard-kit)`, `(@delegation-kit)` and `(@registered)`.
  Each bullet says what reaches the member and that doctor requires it only
  there. The `shellcheck` bullet's *adopter requirement, no audience token,
  refused rather than half-installed* passage is rewritten: `check-shellcheck`
  arms when the adopter registers it for their own gate scripts, and
  `check-action-run-shell` when they register it for their workflows. Each
  bullet names its install source per platform in prose: the distribution
  package, Homebrew, or Chocolatey.
- **The audience paragraph** (*`@contributor` is the only one there is*) is
  rewritten to the three kinds from delta 1. An unmarked bullet is what every
  adopter is held to. A kit-named or `@registered` bullet is held only where the
  selection reaches it.
- **The opening.** *GNU-first* becomes: the engine assumes the POSIX userland
  `GATE_SDK_PROGRAM_FLOOR` names, and no shipped adopter path needs GNU
  coreutils. **Inferred, cannot run before build:** that a stock macOS host without Homebrew coreutils installs the starter profile and runs its battery clean — delta 8's macOS legs are the oracle, and no local command reaches a BSD userland.
- **The macOS remedy block** becomes `brew install bash`, plus the two lines that
  put Homebrew's `bin` ahead of `/bin` on `PATH`, now and in `~/.zprofile`. The
  paragraphs around it stop citing `date` and `shellcheck` as reasons.
- **The Windows remedy block** keeps its `PATH` lines and drops
  `choco install shellcheck jq -y`. The paragraph above it stops calling those
  two floor members Git for Windows does not supply.

### (8) The install-smoke legs follow the remedy blocks {design-bearing}

**Not yet applied.** In `.github/workflows/gates.yml`:

- **Both macOS legs.** The persistence probe retargets from `date` resolving
  under coreutils' `gnubin` to `bash` resolving under `$(brew --prefix)/bin` in a
  fresh login shell. The comments that ground the probe in coreutils are
  re-grounded.
- **Both Windows legs.** The `shellcheck` presence check and its *SCAFFOLDING
  FAILED* line are deleted, and so is the comment block calling `shellcheck` an
  adopter floor member the leg must provision. Each leg installs `starter`, which
  owes neither `shellcheck` nor `jq`. The runner image may carry them anyway, so
  the leg asserts nothing about their absence.

## Producers and consumers

- **The widened audience values (delta 1).** Producer: `PROBE_SET`, the crate's
  own constant. Consumers, each with its red condition:
  - `check-install-toolchain` compares the whole `name:min:impl:audience`
    quadruple, verbatim after the `@` sigil (`install_toolchain.rs`
    `listed_quads` / `render`). It reds until delta 7's bullets carry the same
    tokens, and it needs no code change.
  - `programs.rs` assertion B reds unless the three rows carry the same strings.
  - Assertion A reds on a non-contributor row off the floor, `PROBE_SET` and the
    payload. The three rows are on `PROBE_SET`.
  - The env-probe arm walks the whole roster and marks any non-empty audience as
    `<audience>-only` (`env_probe.rs` `audience_mark`). It renders
    `guard-kit-only`, `delegation-kit-only` and `registered-only` with no code
    change.
  - The new closed-set unit test reds on a value naming no kit root.
- **The selection (delta 2).** Producers: `init` (profile kit set, derived gate
  set and on-disk registry) and doctor's own manifest read. Consumer: the
  owed-predicate, read once per member. Each field has a reader. The kit set is
  read by the kit-name arm and the gate set by the `registered` arm.
- **The verdict and exit status.** Unchanged in kind. `init`'s last precondition
  now fails on fewer hosts, and only on a member the selection owes.
- **The `on-surface` disposition (delta 3).** Readers:
  - The recipe's derivation. It stops registering the gate, which is the intent.
  - `check-install-disposition` assertion A. `on-surface` is in the closed
    vocabulary.
  - Assertion B binds `zero-config` members only, so gate-sdk's smoke may go on
    registering the gate.
  - The consumer smoke and `--run-demo` register through each kit's
    `smoke/install.sh`, never through `init`'s roster, so gate-sdk's
    `smoke/violation.sh` still meets a registered `check-shellcheck`.
  - The profile lattice's monotonicity assertion is derived and reds on nothing.
- **The date helpers (delta 5).** Producers: `kpi::today_iso` and the new
  in-process stamp. Consumers: the stamp line `--enter-stage` appends, the
  evidence line `--run-validate` writes, the env-probe block's `Probed` line
  (excluded from its change detection), and the wakeup log's timestamp field.
  Each keeps its grammar.
- **Point 6, every roster member's satisfying value.** The members are the six
  elements of the table in delta 1, and each row names its value. Enumerated by
  reading `PROBE_SET`.

## Existing sections updated

Rosters were produced by an audit sweep over the tracked tree, excluding generated
mirrors, dated posts and the queue. It grepped for `jq`, `curl`, `shellcheck`,
`PROBE_SET`, `audience`, `probed()`, `by_name`, `macos-remedy`, `windows-remedy`
and `check-shellcheck`. The spawn rosters came from
`grep -rnE "programs::(DATE|MKTEMP|CP|PS)\b" native/src`.

- `native/src/toolfloor.rs`: `PROBE_SET`, the owed-predicate and the
  closed-set test (delta 1).
- `native/src/programs.rs`: the `JQ`, `CURL` and `SHELLCHECK` rows and assertion
  A (delta 1); `DATE`'s cfg (delta 5); the `MKTEMP` and `CP` audience (delta 6).
- `native/src/installer/doctor.rs` and `native/src/installer/init.rs` (delta 2).
- `gate-sdk/checks/check-shellcheck.gate` (delta 3).
- `installer/consumer-smoke/run-smoke.sh`, the `jq`-less arm and its binding
  comments, plus the harness's header comment clause *init is blocked by
  doctor's floor verdict* (delta 4).
- `native/src/emit/enter_stage.rs`, `native/src/emit/run_validate.rs`,
  `native/src/emit/env_probe.rs` and `native/src/hook/wakeup.rs` (delta 5).
- `context-kit/SPEC.md` §bin/env-probe (deltas 1, 5 and 6). These passages are
  rewritten, not appended to:
  - the audience-axis paragraph, whose value set widens;
  - the *what it probes* spawned-programs sentence, now `uname` and, off unix,
    `date`;
  - the *`date`, `mktemp`, `cp` and `ps` rest on the floor's assumption*
    sentence, which gains their dispositions;
  - the rendered-verdict paragraph's `contributor-only` example, which widens to
    `<audience>-only`.
- `gate-sdk/SPEC.md` §The program roster: assertion A's adopter-side reading and
  the contributor members' sentence (deltas 1 and 6).
- `gate-sdk/SPEC.md` §The non-gate arm: the sentence placing `date` under
  `--emit-queue-index` and its siblings, and `--emit-env-probe`'s set, both
  qualified *off unix* (delta 5).
- `gate-sdk/SPEC.md` §check-shellcheck and the retirement table's
  `check-shellcheck` row (delta 3).
- `installer/SPEC.md`: §Requirements' `jq` paragraphs (delta 4), §doctor's
  *Which toolchain* paragraph and its roster paragraph (delta 2), §init's doctor
  ordering (delta 2), and §The consumer smoke's `jq`-less arm (delta 4).
- `installer/profiles.list`: the starter comment (delta 3), and the prose
  comment's *on a git-only floor*, which becomes a pointer to the floor
  docs/install.md states (delta 7).
- `docs/install.md` §Requirements (delta 7): the toolchain block's `jq`,
  `curl` and `shellcheck` bullets, the audience paragraph, the opening
  paragraph, and both remedy blocks and their surrounding paragraphs.
- `.github/workflows/gates.yml`: the two macOS legs and the two Windows legs
  (delta 8).
<!-- update-target-exempt: generated mirrors of the kit SPECs, regenerated by their freshness gate's printed command, never hand-edited -->
- `docs/context-kit/SPEC.md`, `docs/gate-sdk/SPEC.md` and `docs/installer/SPEC.md`.
- `.workflow/release-declarations.md` §Behavior changes: one bullet each for
  doctor's conditional floor, `check-shellcheck`'s `on-surface` disposition and
  the shrunk remedy blocks, appended by the landing session (all deltas).

## Retired spellings

- None — no delta retires a name: `programs::DATE` survives under a cfg, and
  every roster member keeps its name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template carries no grounds; a delta places them.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved** — `floor-curl-jq-unconditional`,
      `floor-shellcheck-unconditional` and `floor-coreutils-residue-unowned`
      move to Done in the merge commit, a stage before the drain stage.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the
      block above against the tracked tree.
- [ ] **Oracle run** — the consumer smoke's `jq`-less arm is green, and so are
      the `gates` workflow's two macOS and two Windows install-smoke legs on the
      pushed commit, which discharges delta 7's inferred marker.
- [ ] **Gaps filed** — any cross-component gap build discovers is resolved that
      session.
