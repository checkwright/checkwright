# SPEC amendment: adopter-floor

TRAJECTORY objective 1 says the adopter's dependency floor collapses to git. This
amendment starts that work. It rules a disposition for each member of the floor
`native/src/toolfloor.rs` `PROBE_SET` declares, ranks the resulting rungs, and
lands the first two:

- **sort:** the floor predicate compares versions in-process instead of spawning
  `sort -V`.
- **awk:** the four shipped adopter-side awk programs move to bash builtins, grep
  or wc.

Both members then leave `PROBE_SET`. The rendered floor becomes bash, git, jq,
curl and shellcheck, plus the contributor-only cargo. The later rungs are filed
as Deferred entries (delta 5).

It is a root-level amendment because it spans `native/`, context-kit, installer,
drift-kit, gate-sdk's templates and the site's `docs/install.md`.

**Objective 1's discharge reading is open.** Rungs 2 and 3 below leave members
that are owed only where the kit or gate reaching them is selected. Whether a
floor that is git-only for the default selection, with per-kit members, meets
*"docs/install.md declares a git-only floor"* is a reading of the objective. Only
the operator can give it, through `/consult`. Nothing this amendment lands states
the floor as git-only, and no delta rests on either reading.

The tree does not already do this. `native/src/toolfloor.rs` `floor_met` spawns
`programs::SORT` with `-V`, and `PROBE_SET` carries `awk` and `sort::coreutils`.

## The rung ladder

Dispositions per member, in rank order. Rung 1 is this amendment. The rest are
the Deferred entries delta 5 files. Each member's reach was read by
`git grep` over the tracked tree minus fixtures, `gate-tests` and `smoke`, and
the binary's spawns by `grep -rnE "programs::<MEMBER>\b" native/src`.

| rank | member | reach | disposition |
|---|---|---|---|
| — | `git` | the model itself | stays: the floor objective 1 names |
| 1 | `sort` | the binary's only spawn is `floor_met`'s `sort -V`, the construct that forces `sort::coreutils` | ported in-process (delta 1) |
| 1 | `awk` | the bootstrap's digest read and three shipped templates; the binary spawns none | sites moved to builtins, grep or wc (delta 3) |
| 2 | `curl` | `--usage-poll` alone (`native/src/hook/poll.rs`), delegation-kit's opt-in timer arm, which already refuses by name | conditional member: owed only where its reaching kit is selected |
| 2 | `jq` | guard-kit's hook alone (`guard-kit/lib/guard.sh`); the binary's use is `#![cfg(test)]` | conditional member as `curl`; full retirement is rung 4b |
| 3 | `shellcheck` | `check-shellcheck` (`# install: zero-config`, so `init` seeds it into every profile) and `check-action-run-shell` | kept as a rule that *is* an external program, owed only where its gates are registered; `init` stops seeding `check-shellcheck` into every profile, which re-rules its `zero-config` disposition |
| 4a | `bash` | the front-end and both generated hooks | the hooks and the front-end reach the binary directly |
| 4b | `jq` | `lib/guard.sh`'s seven spawn sites and one presence probe | the library stays shell (its `# no-port:`) and its JSON reads and renders go to the gate binary, whose parsing is in-process (`native/src/json.rs`) |

guard-kit's hook keeps its `bash` reach on every rung. Whether that bash is the
unavoidable interpreter objective 6 admits is `objective-6-guard-hook-twin-unruled`'s
`/consult`, not a rung here.

**Where the ladder leaves a starter-profile adopter.** After rung 1 the floor
`doctor` holds a starter adopter to is still `bash`, `git`, `jq`, `curl` and
`shellcheck`. After rungs 2 and 3 it is `git` and `bash`, not git alone. That is
why the objective-1 reading above stays open.

**Interaction with `adopter-floor-gnu-date-and-awk-unheld`.** That Deferred
entry's remaining half asks for a CI leg on a non-GNU awk. Delta 3 leaves no
shipped awk program, so after this unit the entry has no live site. Keeping it
for a future awk site or retiring it is a scope call. This amendment does
neither.

The binary also spawns `date`, `mktemp`, `cp` and (off unix) `ps`. It does so
under `GATE_SDK_PROGRAM_FLOOR`'s assumption that the host carries them, and
`PROBE_SET` never renders them. That residue is a real dependency no rung above
owns. So is whether stock macOS still needs the Homebrew coreutils once `sort -V`
is gone: the binary's remaining `date` spawns include `date -Is`. Both go to the gap inbox rather than onto this ladder
(`bash gate-sdk/bin/run-gates.sh --emit file-gap`), because its disposition was
not part of this unit's envelope.

## What changes

**Batching.** Deltas 1 to 4 land in one build batch and one commit. Delta 2
narrows `PROBE_SET`, and `check-install-toolchain` reds while
`docs/install.md` still lists a member the roster lacks. Delta 2 also deletes
roster rows, which leaves delta 1's spawn unbuildable if it is still in place.
Delta 3 is independent in substance but lands in the same commit, so the page and
the roster move once. Delta 5 landed with this amendment at the spec stage.

### (1) The floor predicate compares versions in-process {design-bearing}

**Not yet applied.** `native/src/toolfloor.rs` `floor_met(min, found)` compares
the two tokens itself and spawns nothing. Both tokens are dotted digit runs by
construction: `found` comes from `version()`'s `[0-9]+(\.[0-9]+)+`, and `min`
comes from a roster element the kit authors. The comparison is:

- field-wise numeric;
- a shorter token is padded with zero fields, so `4.3` meets `4.3.0`;
- a field that is not an ASCII digit run, or one that overflows `u64`, answers
  `None`, which `check` renders as `uncomparable`.

`uncomparable` therefore keeps exactly one cause, a banner or token the predicate
cannot compare, and loses the other, *a `sort` without `-V`*, because no `sort` is
spawned.

Unit cases, each a behavioral assertion on `floor_met` or `check`:

- `4.3` vs `5.2.21` is met.
- `4.3` vs `4.2.46` is not met, and renders `below 4.2.46 4.3`.
- `1.71` vs `1.71` is met.
- `4.3` vs `4.3.0` is met.
- `4.10` vs `4.9` is not met: numeric fields, not a lexical comparison.
- An overflowing field answers `None`.

The `sort`-spawning test that pins the *without `-V`* cause is deleted with that
cause.

The one-holder paragraph in context-kit/SPEC.md §bin/env-probe keeps the spawn
on the grounds of a second holder that has since retired, so it is deleted rather
than re-argued.

### (2) `sort` and `awk` leave the probe roster and the program roster {design-bearing}

**Not yet applied.**

- **`PROBE_SET`** drops `awk` and `sort::coreutils`. It becomes `bash:4.3`,
  `git`, `jq`, `curl`, `shellcheck`, `cargo:1.71::contributor`.
- **`native/src/programs.rs`:**
  - `probed()` returns those six members.
  - The `AWK` and `SORT` rows are deleted. After delta 1, shipped code names
    neither, and a row no shipped code names is a `dead_code` finding under
    `check-crate-arms`' deny-warnings clippy.
  - The tests that name `programs::SORT` (`proc.rs`'s `resolve_floor_tool` case
    and `programs.rs`'s retarget case) retarget to another member.
  - `GATE_SDK_PROGRAM_FLOOR` keeps `awk` and `sort`. The shipped shell may still
    assume them, and the session-context template's `sort -u` is POSIX.
- **`native/src/proc.rs` `SYSTEM_DIR_HOMONYMS`:**
  - The `sort` row retires with its last spawner.
  - If `NoResolution::FallBack` then has no row, the variant retires too.
  - `resolve_floor_tool` keeps its own fall-back to the bare name. That
    fall-back never read the roster, which is what its reporting role needs.
- **The implementation-token axis** (`<name>:<min>:<impl>`) stays in the grammar
  and parser with no live member carrying a token. Its readers are parse cases,
  and a future member forced onto one implementation needs the axis. The
  context-kit prose stops using `sort::coreutils` as its worked example.

**Every member's value (point 6), for the probe-roster readers:**

- **doctor** (`native/src/installer/doctor.rs`) walks the six elements and
  renders five: `cargo` is filtered by audience.
- **The env-probe arm** walks all six.
- **`check-install-toolchain`** holds the six against the delta-4 page.

### (3) The shipped awk programs move to builtins, grep or wc {mechanical}

**Not yet applied.** Four sites, enumerated by
`git ls-files | grep -v -E "gate-tests|fixtures|/smoke/|^docs/" | xargs grep -lE "(^|[|;&(\` ])awk "`,
filtered to shipped shell. Each site's satisfying value:

- **`installer/bin/checkwright.sh` `verify_digest`:** the published digest is
  the first whitespace field of the `.sha256` file's first line, read with
  `read -r want _ < "$ARTIFACT.sha256"`.
- **`context-kit/templates/session-context.sh`, stage read:** the stage cursor is
  field 2 of the last non-blank line after the last `---` line, found by a bash
  `while read` loop over `$STATE_FILE`. An absent file or no data line yields
  empty, as today.
- **`context-kit/templates/session-context.sh`, dirty-component pipeline:**
  `git status --porcelain`'s last field and its first path segment come from
  parameter expansion in the existing `while read` stage. `sort -u` may stay.
- **`drift-kit/templates/kpi-deprecated-surface.sh`:** the count is
  `grep -hE ... | wc -l`, normalized to a bare integer by arithmetic expansion,
  because BSD `wc` pads its output.
- **`gate-sdk/templates/check-skeleton.sh`:** the placeholder scan becomes a
  `grep -Hn` capture. Status 0 and 1 are verdicts, and a status of 2 or more goes
  through `fail_closed … grep`. The skeleton keeps teaching the fail-closed
  capture, and adds the grep exit-1 carve-out §Fail-closed contract already
  states.

`scripts/session-context.sh`, this repo's instance of the session-context
template, takes the same two edits so the template-copy parity gate stays green.

### (4) The install page and the platform prose follow the roster {mechanical}

**Not yet applied.**

- **`docs/install.md` toolchain block:**
  - The `awk` and `sort` bullets are deleted.
  - The `bash` bullet gains one clause: the shipped shell assumes the POSIX
    userland `GATE_SDK_PROGRAM_FLOOR` names (gate-sdk/SPEC.md §lib/gate.sh). The
    page names the knob and never copies its list.
- **`docs/install.md` macOS prose:**
  - The paragraph above the macOS remedy block, and the paragraph after it, stop
    citing `sort` as a reason for coreutils.
  - Coreutils stays for `date`, a claim the page already makes. This delta
    neither rests on that claim nor re-probes it; the re-probe is the filed gap
    above.
  - The remedy block itself is unchanged, and the two macOS install-smoke legs
    run it as today.
- **`docs/install.md`, delivery-path paragraph:** `sha256sum` is no longer "a
  coreutils member the roster asserts anyway". It is stated as the Release
  tarball's own requirement.
- **The macOS legs in `.github/workflows/gates.yml`** retarget their
  persistence probe from `sort` to `date`, the member the block still exists
  for, and re-ground its comment.

### (5) The later rungs are filed as Deferred entries {mechanical}

Applied in this amendment's commit. Four Deferred entries, one per rung 2 to 4b
of the ladder above, each carrying `[cost:]`, `[surface:]`, its disposition, why
it is design-pending, and its cost while deferred:

- `floor-curl-jq-unconditional`
- `floor-shellcheck-unconditional`
- `floor-bash-hooks-front-end`
- `floor-jq-guard-lib`

The ranking lives on the ladder here until merge. After that it lives on the
entries' order in the Deferred section. The operator directed them as direct
entries, so they bypass the gap inbox.

## Producers and consumers

- **`floor_met`'s answer (delta 1).** Producer: `toolfloor::check` for a member
  carrying a min-version, which today is `bash:4.3` and the contributor-side
  `cargo:1.71`. Consumers: doctor's verdict and exit status, and through it
  `init`'s last precondition (installer/SPEC.md §doctor). The env-probe arm's
  rendered verdict in the profile file (context-kit/SPEC.md §bin/env-probe,
  *The rendered verdict*) is a second consumer. The verdict set is unchanged;
  `uncomparable` loses one reachable cause.
- **The narrowed `PROBE_SET` (delta 2), with each reader's red condition
  (point 5):**
  - `check-install-toolchain` reds on any disagreement between the page's list
    and the roster, so delta 4 lands in the same commit.
  - `programs.rs`'s *probe-set walk* test reds when `probed()` and `PROBE_SET`
    name different sets, so both move together.
  - Its *assertion A* test reds on an adopter-side row off the floor, the probe
    set and the payload. The deleted rows cannot red it, and `AWK`/`SORT` would
    still sit on `GATE_SDK_PROGRAM_FLOOR` if kept.
  - `check-crate-arms` reds on a rowed member nothing names, so the rows go.
  - doctor and the env-probe arm red on nothing. They render fewer lines, and a
    host without awk or GNU sort now passes doctor. That is the direction the
    unit was admitted for.
  - No reader asserts a count or a minimum over the roster (`grep -rn "PROBE_SET"
    native/src` names every reader).
- **The awk sites (delta 3).** Each producer is the shipped script itself. Its
  consumers are unchanged: the bootstrap's digest comparison, the session-context
  hook's stdout, the KPI's count line, and a copying author. The consumer smoke's
  bootstrap arm exercises the digest read end to end.
- **The macOS leg probe (delta 4).** Its only reader is the leg's own `case`.
- **The Deferred entries (delta 5).** Readers: the scope stage's ranking, the
  session board, `check-deferred-board-tags` and the queue entry budget.

## Existing sections updated

Rosters were produced by `git grep -n -E "sort -V|sort::coreutils|\`sort\`"` and
`git grep -n -w awk` over `*.md`, excluding `docs/*/SPEC.md` (generated
mirrors), plus the `grep -rnE "programs::..."` probe over `native/src`.

- `context-kit/SPEC.md` §bin/env-probe (deltas 1 and 2). The following passages
  are rewritten, not appended to:
  - the spawned-programs sentence (`uname`, `date`; `sort` goes);
  - the grammar's worked example, which drops `sort::coreutils`;
  - the `sort::coreutils` constrained-member bullet, deleted;
  - the `awk` sentence after the member list, deleted;
  - the floor predicate's *"Numeric comparison is `sort -V`"* and `uncomparable`'s
    two-cause sentence;
  - the *"`sort -V` is preserved"* paragraph, deleted;
  - the homonym paragraph's `sort` worked case, restated without a live member.
- `gate-sdk/SPEC.md` §check-graph (delta 2): the `SYSTEM_DIR_HOMONYMS` passage
  loses `sort` and its fall-back sentence, and so does the membership-criterion
  sentence that cites the system directory's `sort`.
- `gate-sdk/SPEC.md` §The non-gate arm's spawn-set roster, the
  `--emit-env-probe` sentence (delta 1).
- `gate-sdk/SPEC.md` §The port-candidate criteria, the toolfloor-parity
  retirement record's closing sentence (*"The environmental assertion survives
  as a unit test"*), rewritten to say the environmental cause retired with the
  spawn (delta 1).
- `native/src/toolfloor.rs` (deltas 1 and 2): `floor_met`, its binding comment,
  the `Verdict` comment's second cause and `PROBE_SET`. The `sort::coreutils`
  parse and predicate cases stay as grammar examples of the implementation token.
- `native/src/programs.rs` (delta 2): the rows, `probed()` and the retarget test.
- `native/src/proc.rs` (delta 2): `SYSTEM_DIR_HOMONYMS`, `NoResolution`,
  `resolve_floor_tool`'s binding comment and the `programs::SORT` test.
- `native/src/emit/env_probe.rs`, `native/src/installer/doctor.rs` and
  `native/src/gates/install_toolchain.rs` (delta 2): their `sort::coreutils`
  test inputs stay as grammar examples of a rendered implementation token, with
  no production change.
- `scripts/gate-tests/check-install-toolchain.test.sh` (delta 2): its
  `sort::coreutils` fixture rosters stay; a fixture roster is authored input and
  exercises the token axis.
- `installer/bin/checkwright.sh`, `context-kit/templates/session-context.sh`,
  `scripts/session-context.sh`, `drift-kit/templates/kpi-deprecated-surface.sh`
  and `gate-sdk/templates/check-skeleton.sh` (delta 3).
- `docs/install.md` §Requirements and the macOS paragraphs (delta 4).
- `.github/workflows/gates.yml`, both macOS remedy legs (delta 4).
- `TASK-QUEUE.md` Deferred (delta 5).
<!-- update-target-exempt: generated mirrors of the kit SPECs, regenerated by their freshness gate's printed command, never hand-edited -->
- `docs/context-kit/SPEC.md` and `docs/gate-sdk/SPEC.md`.
<!-- update-target-exempt: dated release posts are history; they record what shipped then and are never rewritten -->
- `docs/posts/2026-07-26-checkwright-v0-16-0.md`.

## Retired spellings

- `programs::SORT` — the program-roster row for `sort`, deleted once its last
  spawn goes (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template carries no grounds; a delta places them (the Content-tiering /
      SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `adopter-floor-collapse-rung-unqueued` moves to Done in
      the merge commit, a stage before the drain stage. The rungs it names are
      the Deferred entries delta 5 filed, so no demotion is owed.
- [ ] **Removals propagated** — every retired name is declared above and
      `check-amendment-retired-spelling` runs each against the tracked tree.
- [ ] **Gaps filed** — the binary's coreutils-spawn residue is filed to the gap
      inbox; any cross-component gap build discovers is resolved that session.
