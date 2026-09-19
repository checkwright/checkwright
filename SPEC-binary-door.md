# SPEC amendment: binary-door

`bash` (≥ 4.3) is the one unconditional floor member besides `git`
(`native/src/toolfloor.rs:8`, docs/install.md's toolchain block). The page names
what reaches it:

- guard-kit's hook;
- the shipped session templates;
- both generated git hooks;
- the gate library the front-end sources, whose nameref (`local -n`, in
  `_gate_prebinary_file_value`, `gate-sdk/lib/gate.sh:42`) sets the 4.3 floor.

This is rung 4a of the adopter-floor ladder. Its target is the starter profile
(gate-sdk alone) and the prose profile (gate-sdk and canon-kit), whose floor after
rungs 2 and 3 is `git` and `bash` (`installer/profiles.list`). For those two
profiles, three things still reach `bash`:

- **The generated hooks.** `scripts/git-hooks/pre-commit` and `commit-msg` start
  `#!/usr/bin/env bash` and use `mapfile`, process substitution, `[[ ]]`, `(( ))`
  and ANSI-C quoting (`native/src/emit/git_hooks.rs`, gate-sdk/SPEC.md
  §gen-pre-commit). Stock macOS runs them under bash 3.2, where `mapfile` does not
  exist.
- **`init` itself.** It runs the vendored front-end through `bash` to emit the
  hooks and the graph (`native/src/installer/init.rs:545-551`, `run_vendored`
  spawning `programs::BASH`).
- **The adopter's next steps.** `init`'s follow-up block tells every non-Windows
  adopter to run `bash gate-sdk/bin/run-gates.sh --install-hooks` and the battery
  (`init.rs:744-761`). docs/install.md's manual path says the same (`:468-471`,
  `:520`, `:534`, `:599`), and so does canon-kit's config template header
  (`canon-kit/templates/canon-config.knobs:1`).

**The binary is already a complete door from the repository root.** Run directly
from the root with no `GATE_SDK_ROOT` set, `checkwright-gates --run --gates-dir
scripts --only check-exec-bit` passes clean: the crate defaults `GATE_SDK_ROOT` to
`gate-sdk` (`native/src/walk.rs:221-228`), and `--run` without `--gates-dir` takes
the configured gates dir (`native/src/runner.rs:710`). The front-end adds four
things:

- It changes to the git toplevel. A root-relative spelling of the binary's path is
  itself run from the root, so the direct door needs no replacement.
- It locates the binary. The caller spells the path instead. `init` knows it,
  because `init` placed it (installer/SPEC.md §The install boundary, *The install
  location has one owner*).
- It resolves the gates-dir positional. That is front-end grammar only, and the
  direct door spells `--gates-dir`.
- It holds the fail-open status. Its callers are harness hooks, which are not
  starter or prose surfaces, and the sibling unit
  `fail-open-arm-status-second-source` holds it.

So the rung needs no front-end duty ported, only its starter and prose callers
re-pointed at the binary.

**What this amendment does not claim.** It does not call either profile git-only
on any surface. The unix install bootstrap, `installer/bin/checkwright.sh`, is
itself a bash script. Whether that counts against TRAJECTORY.md objective 1 is an
interpretation of the objective, which is the operator's to make. It is escalated
at this stage, and no surface here words the floor ahead of that ruling.

It is a root-level amendment because it spans gate-sdk (the hook emitter, the
gate library's matcher), the installer (`init`, doctor's roster reading, the
consumer smoke), the crate's probe roster and its owner context-kit/SPEC.md
§bin/env-probe, canon-kit's config template, and docs/install.md.

The tree does not already do this. The hooks' shebang is bash, `init` spawns
`programs::BASH`, the follow-up block prints `bash gate-sdk/bin/run-gates.sh`, and
`PROBE_SET`'s `bash` element carries no audience.

## What changes

**Batching.**

- Deltas 1 and 2 land in one commit. The emitter change makes every committed hook
  stale until regeneration, and `init` must emit the new shape in the same commit.
- Deltas 3 to 5 land in one commit. `check-install-toolchain` reds while the page's
  bullet and `PROBE_SET` disagree, and the consumer smoke's follow-up arm reds
  while the block and the page disagree.
- Delta 6 rides with deltas 3 to 5, because it proves them.

### (1) The generated hooks are POSIX sh {design-bearing}

**Not yet applied.** Both hooks start `#!/bin/sh` and use only POSIX shell:

- **Staged set.** `set -eu` replaces `set -euo pipefail`; the hook runs no
  pipeline whose status matters. The staged set is read into one newline-separated
  variable from `git diff --cached --name-only --diff-filter=ACMR`, and an empty
  set exits 0.
- **Matcher.** `staged_matches` walks that variable with `IFS` set to a newline and
  pathname expansion off, and matches with `case "$f" in $pat) … esac`. A `case`
  pattern's `*` spans `/` exactly as bash's `[[ == ]]` does, because POSIX gives
  the slash its special rule in pathname expansion alone (XCU 2.13.3), which a
  `case` pattern is not. So every glob selects what it selected. Probed here only
  under bash-as-`sh` (this host has no `dash`); the Linux leg of delta 1's marker
  below is where a strict shell runs it.
- **Wrapper.** `run_gate` keeps its contract (capture, reprint on failure or
  `GATE_SDK_VERBOSE`, the one-line green summary) in `[ ]`, `$(( ))` and
  `$( )`.
- **`mode=staged`.** The block builds its member's file operands with `set --`
  inside a function rather than an array. A path with a space stays one operand. A
  path with a newline was never one operand, under `mapfile` either.
- **Quoting.** An argv element outside `[A-Za-z0-9_./:=+,@%-]` is single-quoted,
  each embedded `'` spelled `'\''`, which carries a tab or a newline verbatim. The
  set is fixed, so the committed hooks stay byte-stable across clones.
- **The `gen=manual` region.** It is the consumer's own text and now runs under
  `sh`, so its contract is POSIX sh. No region exists in this tree
  (`grep -c '>>> manual:' scripts/git-hooks/pre-commit` prints `0`). An adopter
  region written in bash breaks at the next regeneration, and the release
  declaration names that upgrade step.

**The matcher body stays spliced from `gate_staged_matches`.** Its body in
`gate-sdk/lib/gate.sh:261-270` becomes the POSIX walk above over a newline-separated
`staged_all`. POSIX text runs identically in bash, so the library's own copy keeps
working. `native/src/runner.rs`'s standing cross-substrate comparison
(`:990-1080`), which drives the shell function, builds `staged_all` as a
newline-separated string rather than an array.

**The two-line exec shim stays refused.** gate-sdk/SPEC.md §gen-pre-commit refuses
a hook that execs one binary arm, for four reasons (`:11678-11701`). This delta
changes the hook's interpreter, not its shape, so each reason is untouched: the
per-gate argv list stays baked, resolution stays at generation time,
`check-graph` assertion D still compares a projection of the manifests, and
`gen=manual` still round-trips.

**Windows and macOS.** Git for Windows runs a hook through its bundled shell
whatever the shebang names, as §gen-pre-commit already records, so that host is
unchanged. `install-smoke-powershell` keeps committing through the hook with every
`bash` stripped from `PATH`. macOS and Linux run it under `/bin/sh`, which every
supported host carries. The hook therefore stops needing a `bash` beyond the
shell git itself runs hooks with.

**Inferred, cannot run before build:** that the `gates` workflow's Linux runners resolve `/bin/sh` to `dash`, so the smokes executing the regenerated hook there prove it POSIX — no run log prints the link today; build adds `readlink -f /bin/sh` to one Linux leg and reads it off that run.

### (2) `init` runs the binary it placed, never bash {design-bearing}

**Not yet applied.** `run_vendored` spawns the artifact `init` just placed (its
`artifact_dest`), with `--emit git-hooks --write` and `--emit graph`, at the
consumer's root and with `GATE_SDK_ROOT=gate-sdk` in the child's environment. That
is the value the front-end would have exported for a root-vendored gate-sdk.
`programs::BASH` leaves `init`.

The rule the call site states, that a consumer's artifacts are the ones its own
gate-sdk makes, holds unchanged. The placed binary is that gate-sdk's binary, and
the front-end did nothing but locate it.

installer/SPEC.md's clearance of `init`'s behind-invoke `bash` spawn (`:626-634`,
cleared "because `bash` is on `GATE_SDK_PROGRAM_FLOOR`") is rewritten. `init`
spawns only `git` and the binary it placed.

### (3) The follow-up block names the binary {design-bearing}

**Not yet applied.** `follow_up_front_end` returns the placed binary's path as
`./<path>`, with the `.exe` suffix where the host's artifact carries one, on every
host:

```
next:
  ./scripts/checkwright-gates --install-hooks   # opt this clone into the generated pre-commit hook
  ./scripts/checkwright-gates --run             # the battery, green on what was just vendored
```

The block's grammar holds (installer/SPEC.md §init). The target is the first
token with a `/`, and no interpreter precedes it, so the consumer smoke's
follow-up arm asserts the mode bit, which a placed artifact carries. One spelling
serves every host, because both `sh` and PowerShell run a `./`-prefixed relative
path, so the Windows branch of `follow_up_front_end` is deleted.

**Inferred, cannot run before build:** that PowerShell 5.1 and 7 both run `./scripts/checkwright-gates.exe` spelled with forward slashes from the repository root — no Windows host is reachable from this stage; the Windows install-smoke leg's follow-up arm executes the printed line once delta 3 lands.

installer/SPEC.md §init's sentence on the block (`:208-211`, "A Windows host is
told to run the PowerShell front-end … Every other host is told to run the bash
front-end") becomes: every host is told to run the binary `init` placed, by its
root-relative path.

### (4) `bash` is owed only where a selected kit reaches it {design-bearing}

**Not yet applied.** With deltas 1 to 3, neither the starter nor the prose profile
reaches `bash`. So `PROBE_SET`'s element gains an audience, and the audience field
admits a **kit list**: kit names joined by `+`, owed where any listed kit is
selected. `+` is used because the element must stay one shell word, and the
install page's parenthetical already splits on `,`.

The element becomes:

`bash:4.3:::context-kit+delegation-kit+drift-kit+guard-kit+lifecycle-kit`

**Point 6, each kit's reach, enumerated.** The probe is
`git grep -l -e "bash gate-sdk/bin/run-gates.sh" -e "^#!/usr/bin/env bash" -- '*-kit/*' 'gate-sdk/*'`,
excluding tests, smokes, fixtures, SPECs and READMEs. A kit is in the list when a
surface it ships runs `bash`, or tells a session or the harness to run it:

- **context-kit:** `templates/session-context.sh`, and the SessionStart wiring that
  runs it with `bash`.
- **delegation-kit:** `templates/agent-execution.md`, a procedure a session
  executes.
- **drift-kit:** `templates/kpi-deprecated-surface.sh` and
  `templates/economics.md`.
- **guard-kit:** `lib/guard.sh`, `templates/bash-guard.sh`,
  `templates/settings-hooks.json` and `templates/close-triage.md`.
- **lifecycle-kit:** the stage templates, `lead.md` and `upgrade.md`.

evidence-kit and queue-kit appear in the probe only through the header comment of
their config templates. That is reference text, not a spawn, and neither kit can
be selected outside a profile that already carries the kits above.
**gate-sdk:**

- `bin/run-gates.sh` stays as a door, but no starter path needs it after deltas 2
  and 3.
- `bin/build-native.sh` is contributor-side.
- `templates/check-skeleton.sh` is the adopter's own opt-in shell gate.
- `templates/gates-workflow.yml` runs on a CI runner, not on the adopter's host.

**canon-kit:** only its config template's header, which delta 5 re-points.

The owed-predicate (context-kit/SPEC.md §bin/env-probe, *The owed-predicate*)
reads a list as the union of its kit names. The unit test holding every audience
value to a kit root (`toolfloor.rs`,
`every_audience_value_is_closed_over_the_kit_roots`) checks each list member.
`check-install-toolchain` reads the parenthetical's `@` field whole, so the page
bullet renders the list verbatim (`native/src/gates/install_toolchain.rs:58-59`).
The env-probe arm's `<audience>-only` rendering (context-kit/SPEC.md `:595-596`)
prints it the same way.

**Honest limit, filed.** The list is hand-held against the kits' shipped reach. A
kit that later ships a bash surface without joining the list is under-declared,
and nothing reds. A derivation, meaning doctor or a gate reading the vendored kits
for a bash reach, is filed to the gap inbox at this stage with its cost, not built
here.

docs/install.md's `bash` bullet is rewritten to name the kits and their reach, and
to drop the generated hooks and the front-end from it. The macOS paragraph
(`:73-97`) says the remedy block is needed only where a selected kit owes `bash`.
The remedy block itself, and the two macOS install-smoke legs that run it
verbatim, are unchanged. doctor's line for an undecided member (installer/SPEC.md
§doctor, today `owed where guard-kit is selected`) names a list as `owed where any
of context-kit, delegation-kit, … is selected`.

context-kit/SPEC.md §bin/env-probe's forcing-construct paragraph for `bash:4.3`
(`:480-493`) names the nameref's site as "`gate-sdk/lib/gate.sh`'s couples
expander, which the gate runner sources". It becomes
`_gate_prebinary_file_value`, which the front-end, guard-kit's library and
context-kit's session template source.

### (5) The starter and prose adopter text names the binary {mechanical}

**Not yet applied.** Each place a starter or prose adopter is told to run the
front-end names the binary instead, by the path `GATE_SDK_NATIVE_BIN` holds (the
gates dir, where `init` placed it). No literal install path is spelled in a kit
file:

- **docs/install.md.** Step 4 of *Vendoring the kits* (`:468-471`). *Reviewing the
  pre-commit hook* (`:516-548`), whose account of the hook as "bash" and "tracked
  bash" becomes POSIX sh. The full-battery aside (`:599`).
- **`canon-kit/templates/canon-config.knobs:1`.** The header's
  `bash gate-sdk/bin/run-gates.sh --emit knob-roster` becomes the binary's
  `--emit knob-roster` arm, spelled through the knob.

The other kits' templates and settings wiring keep the front-end. Their adopters
owe `bash` by delta 4, and re-pointing them is the wider door sweep filed below.

### (6) The consumer smoke proves a bash-less starter and prose install {design-bearing}

**Not yet applied.** The consumer smoke gains a **bash-less arm**, built on the
`jq`-less arm's per-arm `PATH` farm (`installer/consumer-smoke/run-smoke.sh:1011`):
a `PATH` carrying `git` and the userland the binary spawns, and no `bash`. At the
starter profile and at the prose profile it:

- runs `init`;
- executes each follow-up command;
- makes one clean commit through the installed hook;
- makes one commit the hook must refuse by gate name.

A `bash` lookup anywhere on that path fails the arm with the command that made it.
The arm does not mask `/bin/sh`. That is the shell git runs hooks with, which this
rung keeps.

**Point 5.** The arm is new and reds on any exit other than the four it expects.
The follow-up arm's red conditions are unchanged: a target that is not a file, a
target without its mode bit where no interpreter is spelled, and a refused flag.
Delta 3's block satisfies all three.

## Producers and consumers

- **The POSIX hooks (delta 1).** Producer: `git_hooks::pre_commit` and
  `commit_msg`. Consumers:
  - git, at commit time;
  - `check-graph` assertion D's byte compare (`native/src/gates/graph.rs:628-657`);
  - `check-hook-exec-bit`, mode only;
  - `gate-sdk/smoke/install.sh`, which regenerates and executes the hook
    (`:75,141,152,326`);
  - the upgrade smoke and the AGENTS.md smoke, which regenerate it
    (`native/src/emit/upgrade_smoke.rs:667`,
    `native/src/emit/agents_md_smoke.rs:280`);
  - `install-smoke-powershell`, which commits through it.
  
  None parses the hook's shell, so the interpreter change reaches each as a
  regenerated byte stream.
- **The POSIX matcher body.** Consumers: the splice, and `runner.rs`'s cross-substrate
  comparison.
- **`init`'s child (delta 2).** Its consumer is `init`'s own staging of the two
  generated files, unchanged.
- **The follow-up block (delta 3).** Readers: the adopter, and the consumer
  smoke's follow-up arm.
- **The kit-list audience (delta 4).** Readers:
  - `toolfloor::owed`;
  - its closure test;
  - `check-install-toolchain`;
  - doctor's installed and bare reports, including the undecided line;
  - the env-probe arm, which marks the audience.
- **The bash-less arm (delta 6).** Its reader is the consumer smoke's exit status,
  through the `installer_smoke` validate suite.

## Existing sections updated

Rosters produced by `git grep -n "bash gate-sdk/bin/run-gates.sh"` over the
adopter-facing files named above, `grep -n "bash" docs/install.md`, and by reading
gate-sdk/SPEC.md §gen-pre-commit, installer/SPEC.md §init and §The consumer
smoke, and context-kit/SPEC.md §bin/env-probe.

- `gate-sdk/SPEC.md` §gen-pre-commit: "The hooks are bash and stay one
  implementation on every platform" (`:11567-11577`) becomes POSIX sh. The
  *Blocks* and quoting paragraphs (`:11630-11632`, `:11742-11754`) describe the
  POSIX forms. The shim refusal (`:11678-11701`) gains one sentence: the
  interpreter changed and the shape did not (delta 1).
- `gate-sdk/SPEC.md` §lib/gate.sh, `gate_staged_matches`' description (delta 1).
- `native/src/emit/git_hooks.rs`, `gate-sdk/lib/gate.sh:261-270`, the comparison
  in `native/src/runner.rs`, and both committed hooks, regenerated (delta 1).
- `native/src/installer/init.rs` and installer/SPEC.md §init (`:208-211`,
  `:626-634`) (deltas 2 and 3).
- installer/SPEC.md §doctor, its undecided-member line, and doctor's renderer in
  `native/src/installer/doctor.rs` (delta 4).
- `native/src/toolfloor.rs` and its tests, and context-kit/SPEC.md §bin/env-probe:
  the audience-axis bullet "a kit name" becomes "a kit name, or several joined by
  `+`"; the owed-predicate sentence; the forcing-construct paragraph (delta 4).
- docs/install.md: the toolchain block, the macOS paragraph, *Vendoring the kits*
  step 4, *Reviewing the pre-commit hook*, and the `:599` aside (deltas 4 and 5).
- `canon-kit/templates/canon-config.knobs:1` (delta 5).
- `installer/consumer-smoke/run-smoke.sh` and installer/SPEC.md §The consumer
  smoke, which gains the bash-less arm (delta 6).
- `.workflow/release-declarations.md` §Behavior changes, appended by the landing
  session: one bullet that the hooks are POSIX sh and a `gen=manual` region must
  be too, and one that `bash` is owed only by the listed kits (deltas 1 and 4).
<!-- update-target-exempt: generated mirrors of the kit SPECs and the site page, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md`, `docs/context-kit/SPEC.md`.

## Filed at this stage, not built here

Two pieces of work this unit touches are filed to the gap inbox with their cost,
on scope-gated intake:

- **The wider door sweep.** Every other kit's templates, settings wiring and stage
  procedures re-pointed at the binary, the PowerShell twin's retirement once no
  adopter surface names it, and the end of the front-end's fail-open set with it.
- **The derived `bash` audience.** Delta 4's honest limit.

## Retired spellings

- None — the `bash` element keeps its name and its floor and gains an audience,
  and no name is removed.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `floor-bash-hooks-front-end` moves to Done in the merge
      commit, a stage before the drain stage.
- [ ] **A bash-less starter and prose install** — the consumer smoke's bash-less
      arm passes on the `gates` job's Linux leg.
- [ ] **No floor overclaim** — no surface this unit writes calls a profile
      git-only while the bootstrap question is unruled.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed** — the wider door sweep and the derived audience are filed at
      spec; any cross-component gap build discovers is resolved that session.
