# SPEC amendment: demo-verb

An adopter who has only the published package has no walkthrough to run. The
walkthrough that exists, `--run-demo`, copies kit source from the tree it runs in,
so it works only from a clone. This amendment adds a `demo` verb that runs the
walkthrough from the installer's own package, installs nothing into the adopter's
repository, and uses no program beyond the adopter floor.

**The entry's inferred premise was run, and it is false where it matters.**
Master was packed at `c80c47ec` (`--pack-installer --version 0.0.0-probe
--artifacts <one-target dir>`, with the target roster narrowed to the host by
`GATE_SDK_NATIVE_TARGETS_FILE`). The payload carries all eleven kit roots, 1306
entries. It carries **no kit-level `smoke/` directory**:
`tar -tzf <tgz> | grep -E '^package/payload/[^/]+/smoke/'` returns nothing,
because `GATE_SDK_PAYLOAD_WITHHOLD` defaults to `SPEC.md smoke`. `--run-demo`'s
first act runs each kit's `smoke/install.sh`, and its third act runs
`gate-sdk/smoke/violation.sh`. In a scratch repository, with the package unpacked:

- `init` exited 0 with the starter profile, one kit and 550 tracked files.
- `checkwright demo` was refused by the binary's usage arm
  (`unrecognized option: --demo`, exit 2).
- The bootstrap's `--run-demo` and the vendored `run-gates.sh --run-demo` each
  stopped at act 1 with `DEMO: FAIL(env) — run-demo: gate-sdk installer errored`
  (`smoke/install.sh: No such file or directory`, exit 2).
- `uninstall` removed 549 files and left an empty tree.

So carrying the kit roots is not enough. The walkthrough as written depends on
the one shape the payload withholds, and on `cp` and `mktemp`, which
gate-sdk/SPEC.md §The program roster classes as `contributor` because only
source-clone arms spawn them. This amendment does not un-withhold `smoke/`.
Instead, the adopter's walkthrough is rebuilt on the path an adopter actually
takes.

**Which profile, and which defect, measured.** The walkthrough has to show a red
an adopter recognises, and the starter profile does not produce one. A starter
battery (13 gates, 1.3 s) stays green on the consumer smoke's value-arm defect, a
mistyped relative link in `README.md`. It reds only on gate-authoring defects,
such as a registered gate with no source or fixture pair. A `full` install from
the same package took 0.4 s to init (1322 tracked files), and its battery took
1.5 s: 44 gates green. On the same link typo it reddened on `check-md-refs`
(`./README.md: link target 'docs/gide.md' → docs/gide.md is not a tracked file or
directory`), and it went green again once the link was removed. `full` is the
payload-derived profile (installer/SPEC.md §Profiles), so choosing it names no
roster member. The value-arm defect is the one the consumer smoke already asserts
some profile catches, so the demo gains no second defect to maintain.

It is a root-level amendment because it spans `installer/` (the verb contract and
its smoke), `native/` (the arm), gate-sdk (§Consumer smoke's walkthrough
paragraph, which must now tell the two walkthroughs apart) and the `docs/` site
(the home page's try-it block).

**The tree does not already do this.** `native/src/installer/mod.rs`'s `VERBS`
holds `--init`, `--doctor`, `--diff`, `--update` and `--uninstall`, and nothing
else. The probe above shows the refusal.

## What changes

**Batching.** Deltas 1 to 3 land together: the arm, its oracle and its contract.
Delta 4 rides with them, because a verb no page names is a verb no adopter meets.

### (1) `checkwright demo` runs the adoption arc from the package, in a scratch repository {design-bearing}

**Not yet applied.** A sixth adopter verb, `--demo`, is added to
`installer::VERBS` and `TOP_LEVEL_FLAGS` (`native/src/installer/demo.rs`). It
reaches the binary through the bootstrap's unchanged argv rule
(installer/SPEC.md §The install boundary, step 5), so neither bootstrap is edited.
The arc:

1. **Resolve the package** with `installer::package`. The verb accepts no operand:
   an operand is refused with `usage: demo` and exit 2, the same ground
   `--run-demo` stands on (gate-sdk/SPEC.md §Consumer smoke). Run from a source
   checkout, it gets the existing no-payload refusal, whose help already says to
   run it from an installed package. The contributor's walkthrough is
   `--run-demo`, and the refusal's help names it.
2. **Make the scratch repository in-process.** The base is `DEMO_TMP_DIR` when it
   is set and non-empty, else `std::env::temp_dir()`. The directory is created
   with `std::fs`, never with `mktemp`, and a `Drop` guard removes it on every
   exit path, the shape `--run-demo` established. `git init`, then a seed commit
   under a fixed demo identity passed as `-c` flags, so an unconfigured host's
   missing `user.name` cannot stop it.
3. **Act 1, install.** The verb spawns its own executable
   (`std::env::current_exe`) as `--init --profile full`, with the scratch
   repository as its working directory. `init`'s own narration goes to the
   terminal. What is shown is exactly what an adopter's `init` does.
4. **Act 2, a clean battery.** The verb spawns the door `init` placed in the
   scratch tree, `<gates-dir>/checkwright-gates --run` (installer/SPEC.md §The
   gate binary). It must exit 0 and print the summary's `All <N> gates passed`
   token. The battery stays spawned rather than in-process, on
   §Consumer smoke's ground: the subject is the installed consumer's own door.
5. **Act 3, a caught defect.** The verb writes the consumer smoke's value-arm
   defect: a `README.md` carrying one mistyped relative link. The battery must
   exit non-zero, and some `FAIL:` block's finding lines must name `README.md`.
   The verb names no gate. The assertion is that the defect was caught, not which
   member caught it, on installer/SPEC.md §The consumer smoke's value-claim
   ground. Every reddened block is quoted back through its invariant line with
   `--run-demo`'s excerpt rule.
6. **Act 4, fixed.** The verb deletes the defect and re-runs the battery, which
   must be green again.

Exit 0 means every act held. Exit 1 carries `DEMO: FAIL — <act>` on stdout for
the three assertions: not green after install, the defect not caught, and not
green after the fix. Exit 2 carries `DEMO: FAIL(env) — <cause>` on stderr for a
harness that could not be stood up: no payload, the scratch not creatable, a
failed `git`, or `init` exiting non-zero. The grammar is §Consumer smoke's 0/1/2,
so a reader of either walkthrough reads one grammar.

**What it never touches.** The verb never reads or writes the invoking
directory. It runs outside any repository, and inside one it leaves the tree
object and the status byte-unchanged. That is the "nothing installed into the
adopter's repo" half of the deliverable, and delta 2 asserts it.

**What it spawns.** The verb itself spawns `git` and its own executable, and
nothing else. `git` is the adopter floor. The executable is the binary the
bootstrap just verified. So the verb adds no member to the tool-floor roster and
reclaims no `contributor` program for an adopter audience.

The battery it runs is a different matter. It spawns whatever the `full`
profile's registered gates spawn, which is that profile's own floor (the
tool-floor roster, context-kit/SPEC.md). **Inferred, not run:** the `full`
battery is green on a Windows host whose only shell is the one the platform
ships. The probes above ran on Linux. Build settles this on the PowerShell
install-smoke leg, which drives the demo arm of delta 2. If the claim fails
there, a gate whose program is missing exits 2 under the fail-closed contract,
so act 2 fails loudly and never passes quietly.

**Code shared with `--run-demo`, not copied.** The banner, the green-token
matcher and the excerpt move out of `native/src/emit/demo.rs` into one module
that both arms call. The two arcs differ in their acts, so each keeps its own
`walkthrough`.

### (2) The consumer smoke runs the verb from the packed package {design-bearing}

**Not yet applied.** `installer/consumer-smoke/run-smoke.sh` gains a **demo arm**
after the value arm. It runs once, not per profile, because the verb fixes its
own profile. The arm runs `checkwright demo` through the bootstrap of the package
installed from the tarball, with the working directory set to the consumer that
`init` just wrote. It asserts four things:

- exit 0, and the `DEMO: clean` line;
- that consumer's tree object and `git status --porcelain` are identical before
  and after the run;
- `DEMO_TMP_DIR`, pointed at a directory the harness owns, is empty afterwards,
  so the scratch was torn down;
- `checkwright demo extra` exits 2 with the usage line.

**Point 5, the red condition.** Each of the four is its own named failure line,
and the arm fails the suite. A verb that ran green and left a scratch behind is a
red and never a pass, and so is one that wrote into the invoking tree.

The PowerShell install-smoke leg in `.github/workflows/gates.yml` also runs
`checkwright demo` once and asserts exit 0. It is the one leg on a host with no
POSIX shell, so it is where the Windows claim in delta 1 is settled.

The consumer smoke's demo arm and that leg are the verb's only oracles. It is also the only place a packed payload meets
the walkthrough at all. Today's `demo` evidence suite drives `--run-demo`, which
never touches a payload, and that suite stays as it is.

### (3) The verb contract names `demo` and tells the two walkthroughs apart {mechanical}

**Not yet applied.**

- installer/SPEC.md §The verbs: the table gains the row `demo` | *show me the
  adoption arc without touching my repository* | *the payload, in a scratch
  repository of its own*. The writes/does-not-write classifier puts `demo` among
  the verbs that do not write, as far as the adopter's tree is concerned.
  Everything it writes lives in a scratch it removes, so it takes no `--dry-run`.
  Say that in the classifier paragraph, beside `doctor` and `diff`.
- installer/SPEC.md §The consumer smoke: the arm roster gains the demo arm of
  delta 2.
- gate-sdk/SPEC.md §Consumer smoke, the walkthrough paragraph: `--run-demo` is the
  **contributor's** walkthrough. It vendors every kit root the source tree
  resolves and runs the kits' shell recipes. The adopter's walkthrough is
  installer/SPEC.md's `demo` verb. One sentence, pointing, with no mechanism
  restated.
- The `// spec:` comments at `native/src/installer/mod.rs:1` and
  `native/src/main.rs:311` say "the five adopter verbs". They are rewritten
  count-free, so a sixth verb does not falsify them.

Mechanical: the wording is settled by deltas 1 and 2.

### (4) The adopter surfaces offer the verb {mechanical}

**Not yet applied.**

- docs/index.md's try-it fence gains `npx checkwright demo` as its first command,
  with a trailing comment saying it needs no repository. The prose above the
  fence says that the demo installs nothing, and that the three lines after it are
  the real install and its reversal.
- installer/README.md §What you can run: the verb table gains the `demo` row.
- `.workflow/release-declarations.md` §Behavior changes: one bullet, `checkwright
  demo` is new and needs nothing from you.

Mechanical: the text is fixed by delta 1.
`check-fence-command-head` passes `npx` as a configured program, and the arm's
existence is what `--help`'s verb line derives from `VERBS`.

## Producers and consumers

- **The `--demo` arm (delta 1).** Producer: `installer::VERBS`, reached from
  either bootstrap by the argv rule. Its enabling configuration is none. It needs
  a payload, which every installed package carries, so no deployed configuration
  leaves it dead. Consumers:
  - the adopter at a terminal;
  - the consumer smoke's demo arm (delta 2);
  - `--help`'s `verbs:` block and the unknown-verb refusal's `adopter verbs:` line
    (`native/src/main.rs:244,259`), which both derive from `VERBS`. They are
    roster-holding readers, and they gain the name with no edit.
- **`DEMO_TMP_DIR` (delta 1).** Its reader was `--run-demo` alone, and it now has
  two. Its status is unchanged: a process-environment override, not a kit knob
  (gate-sdk/SPEC.md §Consumer smoke owns that ruling). So it joins no knob roster,
  and `check-knob-citation` has nothing to red on.
- **The exit and verdict lines (delta 1).** Their reader is the consumer smoke's
  demo arm, which reads the exit code and the `DEMO: clean` line. The
  `DEMO: FAIL` lines are read by the adopter and by nothing else. They are not
  parsed, so no field lacks a reader.
- **The demo arm (delta 2).** Producer: `run-smoke.sh`, in the arm sequence after
  the value arm. Consumer: the `installer_smoke` evidence suite, which reads the
  printed arm headers as a scenario roster in print order (evidence-kit/SPEC.md
  §Layout and configuration). A new header is a new scenario row. It is the
  roster-holding reader of this delta, and it derives from the print.
- **Point 6, every member has a satisfying value.** The one corpus a delta obliges
  per member is the profile set. The verb runs one profile, `full`, which is
  derived and always present (installer/SPEC.md §Profiles). No member is named
  and none is narrowed past.

## Existing sections updated

Rosters produced by `grep -n "Arm::\|\"--" native/src/installer/mod.rs
native/src/main.rs`, by `git grep -n -i "five \(adopter \)\?verbs"` (three sites:
`installer/SPEC.md:716`, `native/src/installer/mod.rs:1`,
`native/src/main.rs:311`), by `git grep -n "run-demo\|walkthrough" -- '*.md'`,
and by reading installer/SPEC.md §The verbs, §The install boundary, §Profiles and
§The consumer smoke, and gate-sdk/SPEC.md §Consumer smoke in full.

- `native/src/installer/mod.rs` (`VERBS`), `native/src/main.rs`
  (`TOP_LEVEL_FLAGS`), a new `native/src/installer/demo.rs`, and the shared
  walkthrough module out of `native/src/emit/demo.rs` (delta 1).
- `installer/consumer-smoke/run-smoke.sh` (delta 2).
- `.github/workflows/gates.yml`, the PowerShell install-smoke leg (delta 2).
- installer/SPEC.md §The verbs (the table and the classifier paragraph) and
  §The consumer smoke (the arm roster). gate-sdk/SPEC.md §Consumer smoke (the
  walkthrough paragraph). The count-bearing comments at
  `native/src/installer/mod.rs:1` and `native/src/main.rs:311` (delta 3).
- installer/SPEC.md:716, "The five verbs are not ops of this family", is
  rewritten count-free (delta 3).
- docs/index.md (the try-it block), installer/README.md §What you can run, and
  `.workflow/release-declarations.md` §Behavior changes (delta 4).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/installer/SPEC.md`, `docs/gate-sdk/SPEC.md`, and `docs/installer/README.md`.

## Retired spellings

- None — no name is removed: `--run-demo`, `DEMO_TMP_DIR` and the `demo` evidence
  suite all stay, and the count phrase "five verbs" is rewritten rather than
  retired as a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only.** The home page and installer
      README rows carry no grounds, and delta 3 places them in installer/SPEC.md.
- [ ] **Merged with no information lost.** Each addition re-phrases the text it
      refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `adopter-demo-verb-missing` moves to Done in the merge
      commit, at a stage before the drain stage.
- [ ] **The verb works from a package.** The consumer smoke's demo arm is green
      on a packed tarball, and it reds on a verb that leaves a scratch behind or
      writes into the invoking tree.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap that build discovers is resolved
      in that session.
