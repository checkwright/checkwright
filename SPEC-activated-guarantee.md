# SPEC amendment: activated-guarantee

Pairs `activated-guarantee-route`. The front door's guarantee is demonstrated only by `demo`, which installs `full` into a scratch repository of its own (installer/SPEC.md §demo), while a first install takes `starter`, gate-sdk alone, and no verb tells the adopter what their own tree then enforces. `doctor` answers whether the machine runs the battery and what is installed (installer/SPEC.md §doctor). This amendment adds the verb that makes `demo`'s claim on the adopter's own tree and reports what that tree configures, enforces and leaves inactive. It also gives the time to that proof a home in drift-kit's install-observation record.

**The reader it is built for includes an evaluating agent** (operator direction 2026-10-17, lead-relayed, not a ruling): an adopter may be a coding agent selecting a tool on a person's behalf, or a person deciding on an agent's recommendation. So the proof runs non-interactively and ends in a verdict a machine parses, and an agent can report "caught on my own tree" with its time to proof.

**The refused alternative: widening `doctor`.** `doctor`'s exit status is the toolchain contract `init` gates its precondition on, and `doctor` runs before every install. The proof needs a status of its own, held or not, which a CI step or an agent gates on and which would collide with the toolchain's. It also runs the battery twice in a clone, which `init`'s precondition call must not pay. These are the two grounds installer/SPEC.md §The verbs gives for `diff`, and they hold here unchanged. **Nor a mode of `demo`:** `demo` fixes its own profile and never reads the invoking tree, and both properties are what make it safe to run before an install.

## What changes

### (1) The installer gains `prove` {design-bearing} {user-facing: activated-guarantee-route's Deliverable, the verb-or-doctor form it leaves to this stage, and the operator direction of 2026-10-17 making an evaluating agent a first-class reader, both lead-relayed}

New section, `installer/SPEC.md` §prove, after §demo. **Not yet applied.**

> ## prove
>
> `checkwright prove` answers *is a done claim caught on my own tree, and what here is enforced?* It makes the claim `demo`'s act 3 makes (§demo) in a clone of your `HEAD`, runs your own battery over it, and reports what the install configures, enforces and leaves inactive. It writes nothing to your worktree, index, refs or object store.
>
> **It takes no operand.** `-h`/`--help` answers on its own (§The verbs), and any other word is refused with the usage line and exit 2. It reads nothing from stdin and spawns every child with stdin closed and `GIT_TERMINAL_PROMPT=0`, so it runs to its verdict under a shell with no terminal, an evaluating agent's included. It reads no payload, since everything it reads is the tree's, so the placed binary runs it as `--prove` as well as a bootstrap does.
>
> **Preconditions, each refusing at exit 2:** a git work tree git answers for, refused as §init refuses one; a `HEAD` naming a commit; a `checkwright.lock` at `HEAD` carrying a `version` and a schema this build knows, else a refusal naming `init` and its commit; and a gate binary at `HEAD` at the path `GATE_SDK_NATIVE_BIN` names there.
>
> **The run.**
>
> 1. **The clone.** Under the base `demo` uses (§demo, step 1), `git clone --local --no-checkout` makes a clone of the repository and checks `HEAD`'s commit out detached, with `demo`'s local identity and maintenance settings. It makes the scratch directory `init` makes (§What init seeds), which git does not carry. A drop guard removes it on every exit path. Every reading below is of that clone, except the hook state, which is per clone and read from yours.
> 2. **The report.** `configured` names the profile, the kit count and the registry's member count, from the lock and the registry it records. `enforced` names where the battery runs without being asked: `at commit`, `doctor`'s hook-state line over your repository, and `in CI`, the workflow `init` seeds where `HEAD` carries it, else `none seeded`. `inactive` names each disarmed member, by `doctor`'s reader (§doctor), and the hooks when they are not opted in. Where your worktree differs from `HEAD`, a `note:` line says the run read `HEAD` alone.
> 3. **The battery at `HEAD`.** The clone's door runs `--run`, and its summary is reported. A red battery does not stop the run.
> 4. **The claim.** It resolves the queue and state files as a battery run at the clone's root resolves `LIFECYCLE_KIT_QUEUE_FILE` and `LIFECYCLE_KIT_STATE_FILE`, falling back to the paths `init` seeds. `demo`'s claim is written over them and committed with `--no-verify`, and the battery runs again. The claim is **caught** when the battery exits non-zero and a reddened block names the claim's iteration, `demo`'s rule, each block quoted back through its excerpt rule. A tree keeping neither file, or one the claim cannot be staged in, has no claim to catch.
> 5. **Time to proof.** On a catch, the whole minutes from the committer time of the oldest commit reachable from `HEAD` that added `checkwright.lock` to the run's own clock.
>
> **The verdict is the exit status, and the last line of stdout states it** in a grammar a crate test holds:
>
> - **0** — `PROVE: held — caught at <head>; time to proof <N> min`, `<head>` the abbreviated commit and `<N>` a decimal integer.
> - **1** — `PROVE: not held — <cause>`: `the claim was not caught`, or `this tree keeps no <file> to claim a task done in`, followed by a `help:` line naming a profile or `--with-kit` selection that keeps a task queue and stage state, which `init --help` lists.
> - **2** — `PROVE: FAIL(env) — <cause>` on stderr, for a refused precondition, a clone that could not be made, a failed `git`, or a door that could not be spawned.
>
> **What it spawns.** The verb spawns `git` and the clone's door, and nothing else, so it adds no member to the tool-floor roster. The battery spawns whatever the registered gates spawn, which is the installed selection's own floor.
>
> **Its oracle** is the consumer smoke's prove arm (§The consumer smoke).
>
> **Honest limits.** It proves `HEAD`, never uncommitted work, and a gate reading a file git ignores reads that file's absence in the clone. A gate red for another reason whose block prints the queue header would read as a catch. A tree vendored without the installer carries no lock and is refused. The Windows route runs the same code and is measured by no leg.

Replacement for `installer/SPEC.md` §The verbs, a new table row after `demo`'s, and the classifier paragraph's first sentence. **Not yet applied.**

> | `prove` | is a done claim caught on my own tree, and what here is enforced? | `HEAD`, through a clone of its own, and the hook state of this clone |

> **The classifier is writes / does not write, and it decides `--dry-run`.** `doctor` and `diff` write nothing, and `demo` and `prove` write only into a scratch repository each makes and removes (§demo, §prove), so none of the four takes one.

The verb also joins `VERBS` in `native/src/installer/mod.rs` and `TOP_LEVEL_FLAGS` in `native/src/main.rs`, and takes no `FLAGS` row.

### (2) `init` names the proof after its follow-up block {mechanical} {user-facing: as delta 1}

After the `next:` block, on every run that prints it, `init` prints one unindented line: `then: <door> --prove reports what this tree enforces and whether a task claimed done with no evidence is caught here.`, `<door>` spelled as the block's commands spell it. The line sits outside the block, so neither the consumer smoke's follow-up arm nor the PowerShell leg runs it, and a `not held` on `starter` reds neither.

Replacement for `installer/SPEC.md` §init, the sentence of the paragraph beginning **It writes the kit directories** that opens "It makes **one commit**". **Not yet applied.**

> It makes **one commit** naming the profile and the version, then prints a **follow-up block**: the commands that finish the setup, each with its reason. A `then:` line after the block names `prove` (§prove), the step that shows the install working on this tree.

### (3) The consumer smoke gains the prove arm {design-bearing}

New paragraph in `installer/SPEC.md` §The consumer smoke, after the demo arm's. **Not yet applied.**

> **The prove arm** runs after the demo arm, in a fresh consumer `init` wrote at the lattice minimum, with `DEMO_TMP_DIR` pointed at a directory the harness owns. It runs `checkwright prove` through the installed package's entry point with stdin closed, and asserts, each as its own named failure line:
>
> - exit 1 and a last line opening `PROVE: not held — this tree keeps no`;
> - after an `init` re-run at the payload-derived profile, exit 0 and a last line matching the held grammar (§prove);
> - the placed binary's `--prove` reaching the same verdict, so the payload-free path is held;
> - the consumer's tree object and `git status --porcelain` identical before and after each run, and the `DEMO_TMP_DIR` directory empty afterwards;
> - `checkwright prove extra` refused at exit 2 with the usage line.

The arm is a new scenario header in `native/src/emit/installer_smoke/`, beside the demo arm's.

### (4) The install-observation record gains the proof {design-bearing} {user-facing: activated-guarantee-route's Deliverable, "with time to that proof as its metric"}

Replacement for `drift-kit/SPEC.md` §The install-observation record, the grammar block, a new bullet after `<ttfg>`'s, and the key-rule list. **Not yet applied.**

> ```
> <date> install <id> <profile> <floor> <ttfg>
> <date> proof   <id> <ttfp>
> <date> red     <id> <gate> <verdict> <disposition> <behaviour>
> <date> checkin <id> <day> <kit> <retained>
> ```

> - `<ttfp>` — a non-negative integer, minutes from the first install command to the first proof the consumer's activation check reported held. An install with no `proof` line reached none, so the kind needs no sentinel: the `install` line already records the install.

> - `install` and `proof` each **replace on `<id>`**. One install has one first-green time and one first proof; a re-measure corrects it rather than adding a second.

The interface block gains `--emit file-install proof <id> <ttfp>`, and `<ttfp>` joins the closed-set refusals. **A new kind, not a field on `install`**, because a field would change a shipped arity and make an install filed before its proof wait for one; a kind is additive, and a record written before it reads unchanged.

Replacement for `drift-kit/SPEC.md` §The install-evidence projection, the Installs bullet. **Not yet applied.**

> - **Installs** — observed; how many reached a first green, with the min, median and max time-to-first-green over those; how many reached a held proof, with the min, median and max time-to-proof over those. An install whose `<ttfg>` is the no-green sentinel is outside the first population, and one with no `proof` line outside the second. A `proof` line whose `<id>` has no install line contributes nothing.

Owed with it: `native/src/emit/file_install.rs` and `native/src/emit/install_evidence.rs` and their crate cases, the usage line in `drift-kit/README.md`, and `docs/install-evidence.md`, regenerated with its freshness gate's printed command.

### (5) The front door names the path and the proof {mechanical} {user-facing: as delta 1}

- `docs/install.md` §Install's opening paragraph gains the recommended path in one sentence: `demo` to see the guarantee, the install line, the commands `init` prints, then `checkwright prove` to see it on your tree. A new subsection after §Choosing a profile, *Prove it on your tree*, says what `prove` makes and reads, that it writes nothing and asks nothing, that its last line is the verdict with the time since the install commit, and that `starter` is not held and the line names what adds a task queue.
- The `text` prompt in §With another coding agent gains a last sentence: run `checkwright prove` from the repository root and report its last line.
- `installer/README.md`'s verb table gains `prove`'s row, held equal to `VERBS` by `check-front-door-verbs` invariant A, and its `--dry-run` sentence names `prove` beside `demo`. `docs/maintenance.md`'s verb list gains a `checkwright prove` bullet.

**The release this rides.** `prove` on a front-door page is an advertised verb, held to the pinned release by `check-front-door-verbs` and admitted while pending (installer/SPEC.md §The front door's verbs). With the v0.32.0 cut the set record grants, the advertisements stand. Without a cut, close withdraws them or files the release, as that admission requires.

## Producers and consumers

- **The `prove` verb (delta 1).**
  - Producer: the adopter or an evaluating agent, through any bootstrap route or the placed binary, pointed at by `init`'s `then:` line (delta 2) and the install page (delta 5). No enabling config: the verb reads the tree `init` wrote.
  - Consumers: the adopter and an evaluating agent, through the exit status and the verdict line; the observer filing `proof` (delta 4); the prove arm (delta 3).
  - Readers it reuses rather than copies, from `grep -n "fn hooks_line\|fn tree_disarmed_block" native/src/installer/doctor.rs` and `grep -n "fn claim\|fn scratch_base" native/src/installer/demo.rs`: `doctor`'s hook-state and disarmed readers and `demo`'s claim and scratch base, each called from one module rather than re-implemented.
  - Roster-holding readers of the minted verb, from `grep -rln '"--demo"' native/src` and `grep -rln "checkwright demo" --include=*.md .`: `VERBS` and `TOP_LEVEL_FLAGS`; `installer/README.md`'s verb table, which invariant A holds equal to `VERBS`; the front-door pages invariant B reads; and the crate test holding every parser and `usage:` line to `FLAGS`, which `prove` joins with no flag.
  - Fields: `<head>`, read by the adopter or agent to know which revision held; `<N>`, read by the observer as `<ttfp>`'s measurement and by an agent as its time to proof; `<cause>` and the `help:` line, read by the adopter on a `not held`.
- **The `then:` line (delta 2).** Producer: `init`, on every run printing the follow-up block. Consumer: the adopter or agent who ran `init`. The follow-up arm and the PowerShell leg parse the block alone, which the line's indentation ends (§init's block grammar).
- **The `proof` kind (delta 4).** Producer: the observer at an observed install, filing with `--author` for the author seat's own. Consumer: the install-evidence projection's next emission, and `check-install-evidence-fresh` through the committed page. `<id>` is read by the projection's join; `<ttfp>` by its time-to-proof statistics.
- **Point 5:** no delta narrows a corpus. The projection's Installs block gains columns, and the freshness gate byte-compares the regenerated page, which delta 4 lands.
- **No member corpus is obliged**, so point 6 adds nothing.

## Existing sections updated

The roster below came from `grep -n "demo\|doctor" installer/SPEC.md installer/README.md docs/install.md docs/maintenance.md`, `grep -n "ttfg\|Installs" drift-kit/SPEC.md drift-kit/README.md` and `grep -rn '"--demo"' native/src` over the tracked tree.

- `installer/SPEC.md` §The verbs, a new §prove (delta 1).
- `native/src/installer/mod.rs`, a new `native/src/installer/prove.rs` and `native/src/main.rs` (delta 1).
- `installer/SPEC.md` §init and `native/src/installer/init.rs` (delta 2).
- `installer/SPEC.md` §The consumer smoke and `native/src/emit/installer_smoke/` (delta 3).
- `drift-kit/SPEC.md` §The install-observation record and §The install-evidence projection, `native/src/emit/file_install.rs`, `native/src/emit/install_evidence.rs`, `drift-kit/README.md` and `docs/install-evidence.md` (delta 4).
- `docs/install.md`, `installer/README.md` and `docs/maintenance.md` (delta 5).
- `docs/installer/SPEC.md`, `docs/drift-kit/SPEC.md`, `docs/installer/README.md` and `docs/drift-kit/README.md`, the generated mirrors (all deltas).

## Retired spellings

- None — no delta retires a spelling; the verbs, the record's kinds and the projection's labels keep theirs.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred), and the PowerShell leg's coverage of `prove` filed as a gap bullet.
- [ ] **Fixture suites** — the installer consumer smoke, with the prove arm, and the drift-kit crate cases run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
