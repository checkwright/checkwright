# SPEC amendment: native-hook-dispatch

The generated pre-commit hook is 578 lines of POSIX sh that match every gate's triggers with `case` and spawn per block, so every commit on native Windows runs Git for Windows' sh, and on Windows on Arm that sh is x86-64 under emulation. The bash-driven consumer smoke on the two `install-smoke-sh-windows` legs pays the same emulation at the scale of a whole suite: the arm64 leg took 21 minutes against 12 for x64 on the run for `e9c02acd`. This amendment moves the hook's dispatch into the gate binary, leaving each hook a two-line handoff, retires both sh Windows legs, and carries their Windows coverage on the PowerShell legs.

Two queue entries pair it: [native-hook-dispatch](TASK-QUEUE.md#native-hook-dispatch) and [worktree-crate-commit-red](TASK-QUEUE.md#worktree-crate-commit-red), whose hook half lands in the new arm (delta 4) and whose test half is settled as a contract (delta 5).

**The rulings.**

- **The dispatch moves, the selection does not change.** The hook today is a baked copy of what `--for` already computes live: effective triggers, `kit:` expansion, derived couples, `mode=staged` operands, one matcher. The new arm calls that selector over the staged set and filters to `tier=precommit`, so the hook and `--for` agree by construction. A baked copy held equal by a byte-freshness assertion is replaced by no copy (removing the duplication outranks gating it).
- **The four grounds that refused a handoff shim no longer hold**, and §gen-pre-commit's own sentence said when they would stop holding: once no member dispatches to `.sh`. Probe: `git ls-files '*/checks/*'` outside `gate-tests/` lists 123 descriptors, all `.gate`, and `scripts/gates.list`'s 142 live rows name no `.sh`. Ground one, a host with no published artifact losing its whole hook, is void: an unrostered host is refused before any install (installer/SPEC.md §Hosts refused at the bootstrap), so no installed tree has a hook and no binary. Ground two, resolution on every commit, is the cost `--for` already pays, and it replaces a spawn per trigger block. Ground three, assertion D losing its subject, keeps its subject, the committed handoff; what it loses is the drift between manifests and a baked copy, which no longer exists. Ground four, the `gen=manual` round-trip, is delta 3's open question.
- **The hook's strings are kept byte for byte.** Three readers parse them: the consumer smoke's bash-less arm, the PowerShell leg's hook step, and gate-sdk's kit smoke. The arm prints `<hook>: <N> gate(s) passed.`, `<hook>: <gate> failed (see above).` and the bypass line exactly as the sh wrapper does, so no reader moves.
- **The hook stays sh on native Windows unless a probe shows otherwise.** *Inferred, not run:* Git for Windows resolves a hook as the extension-less file first and runs a script through the interpreter its shebang names, looked up by basename, so a tracked hook that `/bin/sh` runs on Linux and macOS is run by its bundled sh on Windows. A `pre-commit.exe` is found only when no extension-less file exists, and a tracked, byte-stable hooks dir cannot carry a per-platform binary. Delta 6 probes it before the docs state it. What the handoff buys either way is one sh start per commit in place of hundreds of `case` blocks and subshells.
- **A linked worktree's hook sees the work tree git implied.** Probed this session in a scratch repository with a linked worktree: git exports an absolute `GIT_DIR` with no `GIT_WORK_TREE` to a pre-commit hook in a linked worktree, and none in the main checkout. Under that environment `git -C native ls-files` takes `native/` as the work-tree top and prints root-relative paths, and `git -C native hash-object -- <those paths>` fails with *could not open*, exit 128, where the main checkout's same pipeline exits 0. That is the `check-gate-binary-fresh` red the queue entry reported. The arm pins `GIT_WORK_TREE` to the directory git ran the hook in, which is what git's own rule already means when `GIT_DIR` is set alone, so every member's `git -C <subdir>` resolves as it does in the main checkout.
- **A crate commit lands from the main checkout; that is the contract, not a defect.** `check-crate-arms` refuses rather than builds in a linked worktree, and delegation-kit's isolation cost (4) forbids a build inside isolation. So no isolated session can hold a binary built from source the main checkout never built, and a crate-source commit made there would red `check-gate-binary-fresh` too. The refusal stays, its help names the real remedy, and the crate's tests pin it instead of redding on it (delta 5).
- **Windows coverage moves to the PowerShell legs under a PowerShell driver.** The bash-hosted checks that ran only on the x64 sh leg move to `crate-tests-windows`' x64 entry, and the POSIX bootstrap keeps a one-step witness on each Windows host (delta 6). The driver's reach is delta 7's open question.
- **The seam.** Kit mechanism: the arm, the emitter's handoff, the work-tree pin, the refusal contract, the docs' per-shell statement. Consumer config: none new. The hooks dir, the binary path and the registry are existing knobs. This repo's CI legs and its acceptance harness are the publisher's own, and ship to no adopter.

**Refused.**

- **`--run --for <staged>` as the hook.** The run arm's worker pool, timing file and `FAIL:` report serve a battery reader. The hook's serial, first-failure report with its summary line is what the committing session and three smoke readers parse. So the arm reuses the selector and the member dispatch, not the run arm's output.
- **A per-clone `pre-commit.exe` hard link or copy of the binary.** It would be untracked, per-host and stale after every update, and it needs an argv[0] dispatch nothing else uses. It is reopened only if delta 6's probe finds a tracked form.
- **Stripping git's locators in the arm.** A partial-path commit hands the hook a temporary index through `GIT_INDEX_FILE`, and `check-gate-binary-fresh` must read it (§check-gate-binary-fresh, *A partial-path commit cannot split a port*). Pinning adds the one variable git left implicit and removes none.
- **Building in a linked worktree on a hook run.** It would reopen isolation cost (4) for every isolated session.
- **Splitting the Windows smoke job.** The queue entry records the operator direction against it.

## What changes

### (1) The `--git-hook` arm {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md gains a section **§git-hook** after §gen-pre-commit:

> ### git-hook
>
> The binary's `--git-hook` arm is what each generated hook hands off to: `<bin> --git-hook pre-commit`, and `<bin> --git-hook commit-msg <message-file>`. It is an `Arm::Run`: 0 every selected member passed, 1 a member failed, 2 it could not run (outside a repository, an unreadable registry, a `commit-msg` with no message-file operand). No operand, an unknown hook name or an extra operand prints `usage: --git-hook pre-commit | commit-msg <message-file>` at exit 2. The name is `git-hook` because a bare `hook` is the harness family (§The harness-integration arm), matching `--emit git-hooks`.
>
> **Before any member runs, it pins the work tree git implied.** Where `GIT_DIR` is set and `GIT_WORK_TREE` is not, as git leaves them for a hook in a linked worktree, it sets `GIT_WORK_TREE` to its working directory, which git has made the work-tree top. That is the value git's own rule already gives, so a member running `git -C <subdir>` resolves the repository as it does in the main checkout. It removes no variable: a partial-path commit's temporary index reaches every member through `GIT_INDEX_FILE` (§check-gate-binary-fresh).
>
> **`pre-commit` runs the triggered subset.** The staged set is `git diff --cached --name-only -z --diff-filter=ACMR`, and an empty set exits 0 printing nothing. The members are `--for`'s selection over that set (§run-gates) restricted to `tier=precommit`, in registry order. A `mode=staged` member receives the staged paths its triggers match that are regular files, as operands. **`commit-msg` runs every `tier=commit-msg` member**, each passed the message file.
>
> Each selected member runs through the run arm's one member dispatch (§run-gates): consumer-first resolution, a `.gate` re-exec of this binary as a child and a `.sh` declaration spawned. Members run **serially, stopping at the first failure**. Output is captured per member. A failing member's output is reprinted, then a blank line, `<hook>: <gate> failed (see above).` and `  Bypass once (use sparingly): git commit --no-verify`, exit 1. With `GATE_SDK_VERBOSE` set, each passing member's output is reprinted with `  PASS: <gate>`. After the last member it prints `<hook>: <N> gate(s) passed.` and exits 0. A registered member resolving nowhere is a failure at commit, as in a battery.
>
> The members read their knobs from the knob files when they run, and the arm passes them no knob environment. Its own knob roster is `GATE_SDK_GATES_DIR` and `GATE_SDK_KIT_DIRS`, held to what the module reads by the crate's arm-knob test. Its caller is git, through the generated hooks, so it is no harness-integration arm and does not fail open: an absent binary fails the handoff's `exec` and refuses the commit, as a missing member did in a baked hook, and `git commit --no-verify` is the bypass.

The arm's module registers in `native/src/emit/mod.rs`'s arm table. Unit tests cover the tier filter, the staged-operand filter, the empty staged set, the first-failure stop, both summary lines, the usage refusals, and the pin: a scratch repository with a linked worktree, where a member-shaped `git -C <subdir> ls-files | hash-object` pipeline succeeds under the pinned environment and fails under git's unpinned one.

### (2) The emitter writes a handoff {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md §gen-pre-commit is rewritten. Its first paragraph is replaced by:

> Emits the git hooks, each a two-line handoff to the gate binary's `--git-hook` arm (§git-hook). `<hooks-dir>/pre-commit` runs `exec <bin> --git-hook pre-commit`. `<hooks-dir>/commit-msg`, written only when a `tier=commit-msg` member is registered, runs `exec <bin> --git-hook commit-msg "$1"`, passing the prospective-message file git supplies. Which gates a commit runs is read from the `# graph:` manifests at commit time, so a manifest, registry or knob edit needs no regeneration. The emission carries no timestamp, so the committed hooks are byte-stable.

The paragraph opening **The hooks are POSIX sh** is replaced by delta 6's text. The **emitter** paragraph and the `None` paragraph stand. The **Each emission rule has one crate source** list is replaced by three bullets:

> - **The binary path** — the resolved `GATE_SDK_NATIVE_BIN`, quoted as an argv element is (below), which each hook execs, and which its header names as the door for regeneration (`--emit git-hooks --write`) and the per-clone opt-in (`--install-hooks`). The binary is named rather than the front-end because it is the one door a starter or prose adopter holds with no `bash`.
> - **The commit-msg conditional** — `registry::members` over `<gates-dir>/gates.list` filtered by `tier=commit-msg`, read through the check dirs anchored at the repository root.
> - **Header** — each hook's fixed comment text, carried as a literal in the module.

The emitter's knob roster becomes `GATE_SDK_HOOKS_DIR`, `GATE_SDK_NATIVE_BIN` and `GATE_SDK_KIT_DIRS`; `registry::EVERY_COUPLES_KNOB` leaves it. The paragraph opening **An emitted trigger set is knob-derived wherever a walk is** keeps its rule for `couples=` and replaces its ground: a knob edit moves the member's selection by `--for` and the hook arm and its edges in the graph artifact, and no longer stales a hook. The paragraph opening **Assertions D and E both compute their emissions in process** stands.

The paragraph opening **The hook's shape is ruled: a two-line** and its four bullets, and the paragraph opening **So the baked per-gate argv list is retained**, are replaced by:

> **The hook is a handoff because no member dispatches to shell.** A baked per-gate argv list was kept while members still resolved to `.sh`, so that a host with no binary kept its shell gates. An unrostered host is now refused before any install (installer/SPEC.md §Hosts refused at the bootstrap), so every installed hook has its binary. Selecting live reads the same manifests the bake read, at the cost `--for` already pays, and removes the copy the freshness assertion existed to hold equal.

The paragraph opening **The hook carries no knob environment** is replaced by §git-hook's knob sentence, cited. The paragraph opening **Both hooks carry the quiet-green wrapper** is replaced by one sentence: *The quiet-green wrapper is the arm's (§git-hook), so it has one implementation for both hooks.* The paragraph opening **A `kit:<glob>` token is emitted expanded** is deleted: the arm expands `kit:` at commit time through the selector. The paragraphs opening **The staged set picks the members**, **Regeneration follows staging**, **Pinning `GATE_SDK_NATIVE_BIN`**, **No trigger widening is owed** and **Emitted argv elements are quoted deterministically** stand.

`native/src/emit/git_hooks.rs` emits the two handoffs and drops `block`, `manual_regions`, the matcher splice and the `run_gate`/`hook_fail` literals. Its unit test emitting the pre-commit hook under every profile in `installer/profiles.list` stands. `scripts/git-hooks/pre-commit` and `scripts/git-hooks/commit-msg` are regenerated.

### (3) `gen=manual` — open, escalated to the lead {design-bearing}

**Not yet applied; the act turns on the lead's answer.** A one-line hook has no place for a manual region. Probe: `git grep -l gen=manual` names no member in this tree. The flag appears only in `check-graph-tree.test.sh`, `gate-sdk/gate-tests/check-graph/good/SPEC-example-gate.md`, the emitter, gate-sdk/SPEC.md and one release post.

- **If retired:** gate-sdk/SPEC.md §The `# graph:` manifest drops `[gen=manual]` from the grammar line and its bullet. `check-graph` assertion A reds a manifest carrying `gen=` and names the migration: a `trigger=*` shell member that reads the staged set itself does whatever a manual region did. §check-graph's key list drops `gen`. The tree test's round-trip cases go, and the example manifest drops the field. `.workflow/release-declarations.md` gains a Behavior changes bullet naming the removal and the migration.
- **If preserved:** a `gen=manual` member's region moves from the hook to `<hooks-dir>/pre-commit.d/<gate>.sh`. `--emit git-hooks --write` moves an existing region there once and writes the placeholder for a member without one. The arm runs that file through `sh` in place of dispatching the member, under the same wrapper, and `check-graph` assertion D holds a registered `gen=manual` member to having that file. The contract stays POSIX sh, so a `gen=manual` consumer keeps a shell dependency on every host.

### (4) The linked-worktree hook half {mechanical}

**Not yet applied.** Delta 1's pin is the fix. gate-sdk/SPEC.md §check-gate-binary-fresh gains one sentence after the paragraph opening **The two axes read different tree states**: *In a linked worktree git runs the hook with `GIT_DIR` set alone, and the stamp's `git -C <crate>` calls resolve only because the hook arm pins the work tree (§git-hook); a caller reaching `fresh::source_stamp` under a hook some other way inherits the same obligation.* Applied at promotion: the worktree-crate-commit-red entry's inferred marker is replaced by this session's probe result (the rulings above).

### (5) A crate commit lands from the main checkout {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md §check-crate-arms, the paragraph opening **In a linked worktree the cache is also read from the main checkout** gains, after its last sentence:

> **That refusal is the contract for a crate-source commit made in a linked worktree**, which lands from the main checkout. The main checkout's record answers only for source it built, and delegation-kit's isolation cost (4) forbids the build that would answer for anything else. So the refusal's help names that remedy, and names no re-run that cannot turn it green.

`native/src/gates/crate_arms.rs`'s second help line reads: *a crate-source change lands from the main checkout, whose build and green run answer for it; a worktree holding only source the main checkout recorded is read from that record.* In `native/src/gates/mod.rs`, `every_registry_member_declares_the_roots_it_walks` and `every_registry_member_declares_the_programs_it_spawns` take `check-crate-arms`' fixture cases in a linked worktree (`walk::main_checkout_root()` is `Some`) as the refusal. Each such case must exit 2 with the refusal's first line, and is then counted and printed as pinned rather than observed. Every other exit fails the test as today, and the main checkout observes the cases as today. A crate unit test pins the refusal directly: a linked worktree with no record exits 2 and runs no cargo.

### (6) The Windows shell, probed, and the docs {design-bearing}

**Not yet applied.** Before the docs change, build adds a probe step to `install-smoke-pwsh-windows` on each matrix entry. It runs after the hook step, in a scratch repository whose hooks dir holds the generated handoff. It renames every `sh.exe` under the Git for Windows install root away and commits, then restores them, and prints whether the hook ran. *Inferred, not run:* the commit fails because git cannot start the hook's interpreter.

- **If the hook runs with no sh** (the inference fails): build stops and escalates. The hook's form on Windows is then a design question this amendment did not settle, and the docs below are not written.
- **If it fails:** the probe step stays as the standing witness, asserting the failure, and the docs state it. gate-sdk/SPEC.md §gen-pre-commit's paragraph opening **The hooks are POSIX sh** is replaced by:

> **Each hook is two lines of POSIX sh, and sh execs the binary once.** On Linux and macOS `/bin/sh` runs it, adding no `bash`; the `gates` workflow's Linux leg prints what `/bin/sh` resolves to, `dash` there, so its green is a strict-shell proof. On native Windows git runs a script hook through the sh Git for Windows bundles, whatever `PATH` carries, so that sh is the hook's one requirement there and `bash` on `PATH` is not. `install-smoke-pwsh-windows` commits through the hooks with every bash stripped from `PATH`, once clean and once refused by gate name, and its probe step witnesses that the hook cannot start with the bundled sh renamed away. A MinGit host, whose distribution may omit sh, is not claimed. On Windows on Arm that sh is an x86-64 build under emulation, which the handoff reduces to one start per commit.

The paragraph opening `install-smoke-pwsh-windows` holds the Windows claim is folded into it and deleted.

docs/install.md: the Requirements row for Git for Windows reads *`git`, and the sh it bundles, which runs the pre-commit hook; `bash` where a row above owes it*. §Windows gains after its **You need** paragraph: *The pre-commit hook needs no `bash` on your `PATH`: git runs it through the sh Git for Windows bundles, and it hands off to the gate binary. The battery needs no shell: run the gate binary with `--run`, or `gate-sdk/bin/run-gates.ps1` from PowerShell. On Windows on Arm, Git for Windows' sh runs under emulation, so each commit pays one emulated start.* The shell-gate sentence of §Writing your own shell gates stands.

installer/SPEC.md §Requirements' **What runs without `bash`** bullet opens: *Both generated git hooks are two-line POSIX sh handing off to the gate binary, and the unix install bootstrap is POSIX sh; all run under `/bin/sh`, and on native Windows the hooks run under the sh Git for Windows bundles.* The rest stands. installer/SPEC.md §Reviewing the pre-commit hook: **What the hook is** becomes *A generated two-line POSIX sh file that execs the digest-verified gate binary's `--git-hook` arm. The arm reads the per-gate `# graph:` manifests at commit time and runs the triggered subset of your registered battery; the gates outside that subset run only in the full battery. The hook is tracked, so the diff you review is what will run, and `check-graph` holds it byte-fresh against its emitter.* **How to review before running** replaces *The hook is tracked POSIX sh in your gates directory, and every gate it invokes* with *The hook is tracked and two lines long, and every gate the binary runs for it*. Its opening sentence's *points a clone's git hooks at POSIX sh from this tree* becomes *points a clone's git hooks at a generated handoff to the gate binary*.

### (7) The sh Windows legs retire; the PowerShell driver — reach open, escalated to the lead {design-bearing}

**Not yet applied.** `.github/workflows/gates.yml` deletes the `install-smoke-sh-windows` and `install-smoke-sh-windows-arm64` jobs. Moves that do not turn on the open question:

- The x64 leg's bash-hosted steps move to `crate-tests-windows`, guarded `if: matrix.target == 'x86_64-pc-windows-msvc'` and placed after its host-binary build: front-end parity (`--run-front-end-parity`), the guard decision table (`--run-guard-tests`), the scratch runner under both PowerShells, and the producer-liveness rows over Windows process ids. Each keeps its commands and assertions. Their host was an arbitrary leg that happened to carry Git Bash and the binary, and `crate-tests-windows` carries both. The leg-naming readers below are re-pointed at it.
- The POSIX bootstrap keeps a witness on each Windows host: one `shell: bash` step on each `install-smoke-pwsh-windows` matrix entry runs `sh installer/bin/checkwright.sh init --profile <lattice minimum>` from the packed tarball in a scratch consumer, asserting the manifest records the host triple's artifact. installer/SPEC.md §Platform resolution's last sentence names that step as the POSIX half's ARM64 witness in place of the retired leg.
- `installer/consumer-smoke/run-smoke.sh` is declared unix-hosted. Its preflight refuses a `MINGW*`/`MSYS*`/`CYGWIN*` host at exit 2, naming `run-smoke.ps1`, and its bash-less arm's native-Windows skip branch and that branch's comment are deleted. installer/SPEC.md §The consumer smoke's bullet opening **A native Windows host skips the arm** is replaced by: *The suite is unix-hosted and refuses a native Windows host; the PowerShell legs carry Windows (below).*

The driver is `installer/consumer-smoke/run-smoke.ps1`, run by each `install-smoke-pwsh-windows` matrix entry after its existing steps, under `pwsh`. It takes the same per-file `no-port` disposition as `run-smoke.sh`, on §The consumer smoke's ground, declared in its header. It prints one header line per arm. Its reach turns on the lead's answer:

- **If the five named arms** (the queue entry's *init, battery, hooks, upgrade, uninstall*): for each profile in `installer/profiles.list` plus `full`, in a fresh consumer, it runs `init --profile` through `installer/bin/checkwright.ps1` from the packed tarball. It runs the battery through the binary `GATE_SDK_NATIVE_BIN` names with `--run`, green, leaving the worktree clean. It runs the printed `--install-hooks` line, then commits once clean, reading `pre-commit: <N> gate(s) passed.`, and once refused, reading `pre-commit: <gate> failed (see above).` with the gate held to the consumer's registry. It runs `uninstall`, and the tree object must equal the pre-init one. Bash is stripped from `PATH` for every profile whose own `doctor` report owes none, and left for the others. Once, at the lattice minimum, the upgrade arm packs the tree at the next patch version, runs `update` from that tarball, and asserts the lock's version moved and the battery is green. The arms `run-smoke.sh` runs and this driver does not are no longer run on native Windows. They stay on the four unix legs: seed, demo, companion, plan parity, artifact-less refusal, download, toolchain-free, jq-less, cross-version and newer-verb reversal, seam, narrowing, artifact, withholding, follow-up, doctor, value. installer/SPEC.md §The consumer smoke gains a paragraph naming the driver, its arms, and that list.
- **If full parity:** the driver carries every arm `run-smoke.sh` runs, header for header, and installer/SPEC.md §The consumer smoke states that each arm has a bash and a PowerShell spelling held to one header roster.

`.github/workflows/gates.yml`'s `install-smoke-pwsh-windows` timeout is re-set from the first green run's measured duration plus the margin the other install-smoke legs carry.

### (8) The shell matcher retires {mechanical}

**Not yet applied.** With no splice, `gate_staged_matches` in `gate-sdk/lib/gate.sh` has no caller but the crate's cross-substrate comparison, so it is deleted. `native/src/runner.rs`' comparison test becomes unit cases on `staged_matches` over the same inputs: `*` crossing `/`, the unquoted-pattern semantics, a leading `*`, an exact path. gate-sdk/SPEC.md:

- §The `# graph:` manifest, the paragraph opening **The field has one matcher**: *The generated hook's `staged_matches` is spliced from `gate_staged_matches` in `lib/gate.sh`. Its `case "$_gsm_f" in $_gsm_pat)` leaves the pattern unquoted under a standing `# shellcheck disable=SC2254`. That is POSIX `case` matching* becomes *The matcher is `runner::staged_matches`, which reads a pattern as POSIX `case` matching does*. The XCU citation stands. The **Do not "fix" the unquoting** sentence becomes *Do not narrow `*` to one segment: it is the semantics, and narrowing it would break every trigger in the tree at once.*
- §lib/gate.sh: the `gate_staged_matches` bullet is deleted, and the sentence counting the library's POSIX sh bodies drops the spliced one.
- §run-gates, the paragraph opening **The match is one matcher**: its first two sentences become *The match is one matcher, `runner::staged_matches`, and the hook arm calls this selector (§git-hook), so what the hook runs for a staged path and what `--for` runs for it cannot diverge.* Its remaining sentences on `trigger=*`, `mode=staged` and `kit:` stand, re-phrased from reproducing hook behavior to stating the selector's own.
- §The port-candidate criteria, the sub-bullet opening **The road is open to a duplication a port creates**: the `gate_staged_matches` instance is past tense, the twin removed with the splice.
- §check-graph, the paragraph opening **Assertion D's emission-failure arm** is deleted: the emitter reads no library.

delegation-kit/SPEC.md, the paragraph opening **A consumer whose crate-arms trigger did not reach this member**: its sentence describing bash's unquoted `[[ str == pat ]]` becomes *a `couples=` pattern is matched as POSIX `case` matching does (gate-sdk/SPEC.md §The `# graph:` manifest), so `*` matches `/`*. The paragraph opening **Those semantics are gate-sdk's and not this hook's** becomes: *Those semantics are gate-sdk's and not this hook's, so they are cited rather than restated: the matcher has one owner, and the rule governs every gate's `couples=` in every kit.*

### (9) The readers of the hook's text and the retired legs {mechanical}

**Not yet applied.**

- gate-sdk/SPEC.md §check-graph's invariant sentence: *the pre-commit hook is the faithful generated projection of the manifests* becomes *each committed hook is its emitter's handoff*. Its **Dual-couple manifest** paragraph's closing clause becomes *the graph artifact re-fires on whichever a consumer publishes to*. `native/src/gates/graph.rs`' assertion D help names the handoff.
- gate-sdk/SPEC.md §check-graph's tree-test paragraph (the one naming `check-graph-tree.test.sh`) and the test itself: the stale-hook, absent-hook and stale-commit-msg cases run against the handoff. The `gen=manual` cases follow delta 3.
- `native/src/gates/reads_couples.rs`' test comparing `--for` against the pre-commit emission is rewritten against the arm's selection function. The emission carries no trigger to compare against.
- `native/src/runner.rs`' `--for` usage line drops *exactly as the generated hook would*, and the hook arm cites the selector instead.
- gate-sdk/SPEC.md: the front-end parity sentence naming *the binding Windows install-smoke leg* names `crate-tests-windows`' x64 entry. The sentence in §The path-dialect contract opening *The Windows leg runs the installer smoke* names the PowerShell legs, whose driver runs the scratch consumer's battery through the shipped executable. The with-foreign-shells honest limit's clause *such as the generated hooks git runs directly* stands, since the handoff is still a `/bin/sh` script.
- guard-kit/SPEC.md: *the `--run-guard-tests` arm on the binding native-Windows install-smoke leg* names `crate-tests-windows`' x64 entry, and so does the producer-liveness oracle sentence.
- installer/SPEC.md: every sentence naming *both Windows install-smoke legs*, *Windows install-smoke legs* or the retired leg keys names `install-smoke-pwsh-windows`, or `crate-tests-windows`' x64 entry for the moved steps.
- `native/runners.list` and `native/targets.list` comments citing the sh Windows legs name the PowerShell leg.
- docs/site-architecture.md: the graph-artifact bullet's *because a knob edit moves a hook's triggers* becomes *because a knob edit moves the graph artifact's triggers*, and its *Regenerate the hooks first … then the artifact* keeps the order. The roster line naming *its `install-smoke-sh-windows` leg* names the PowerShell leg's roster read. The windows-remedy readers line names `install-smoke-pwsh-windows` alone. The line saying the generated pre-commit hook bakes each gate's resolved argv becomes *bakes the resolved binary path*.
- docs/positioning.md: the sentence saying the generated hook's per-gate trigger lists carry manifest default literals is re-phrased to the graph artifact, which still does.
- The kit READMEs whose install step says the manifests put gates *in the generated pre-commit hook, written by `--emit git-hooks --write`*, or tells the adopter to *regenerate the hook + graph artifacts*, are re-phrased: the hook runs the precommit-tier gates their manifests trigger, read at commit. `--emit git-hooks --write` writes it once, and again only to add the commit-msg hook when a first `tier=commit-msg` gate is registered. gate-sdk/README.md's **adding a gate to a hook is manifest-only** clause stands.
- CLAUDE.md's hook line becomes *The pre-commit hook is **generated** — never hand-edit `scripts/git-hooks/pre-commit`; it hands off to the gate binary, which reads the `# graph:` manifests at commit time.* The rest stands.
- `.github/workflows/gates.yml`: the `install-smoke-sh-linux` step **name the shell the hooks run under** keeps its command and cites the rewritten paragraph. The PowerShell leg's hook step drops *the hooks stay POSIX sh on native Windows* and *git ran the sh hooks*, naming the handoff.

## Producers and consumers

Probes: `git grep -n` over the tracked tree for `gate_staged_matches`, `gen=manual`, `install-smoke-sh-windows`, `git-hooks`, `run_gate`, `staged_matches` and `POSIX sh`; the workflow read for the job keys and the steps each Windows job runs; `grep -n '"-C"' native/src` (76 lines across 35 modules) for git calls from a subdirectory; the scratch-repository probe of the hook environment named in the rulings.

- **The `--git-hook` arm** (delta 1). Producers: the two generated hooks, which git runs on every commit in a clone that took `--install-hooks`, the line `init`'s follow-up block prints and this repo's clones take. Consumers: the committing session, through the output contract. The strings' parsing readers are `installer/consumer-smoke/run-smoke.sh`'s bash-less arm, `.github/workflows/gates.yml`'s PowerShell hook step, `gate-sdk/smoke/install.sh` and the new `run-smoke.ps1`. Every one reads a string this amendment keeps. Roster-holding readers: the arm table in `native/src/emit/mod.rs`, and the crate's arm-knob test, which reds on an undeclared read.
- **The work-tree pin** (delta 1). Producer: the arm, on a hook environment carrying `GIT_DIR` alone. Consumers: every member's git child. `fresh::source_stamp` (both callers, `check-gate-binary-fresh` and `check-crate-arms`' cache key) and `check-crate-arms`' untracked probe are the two attested ones, and any of those `-C` sites a member reaches is covered by the same pin.
- **The handoff hooks** (delta 2). Producer: `--emit git-hooks --write`, run by `init` (and so `update`), a kit smoke, and a contributor. Consumers: git; `check-graph` assertion D, which compares bytes; `check-hook-exec-bit`, which keeps holding the mode. **Red conditions:** D reds each committed hook until it is regenerated in the landing commit; `check-hook-exec-bit` reds a handoff committed without its mode.
- **The narrowed emitter reads** (delta 2). The emitter stops reading manifests and the matcher body. A reader whose verdict turned on the hook carrying triggers loses its subject: `reads_couples.rs`' comparison test (rewritten, delta 9) and assertion D's emission-failure arm (deleted, delta 8). Neither reds on finding none: the first compares two selections, and the second fires only on an emission error.
- **The refusal contract** (delta 5). Producer: `check-crate-arms` in a linked worktree on a cache miss. Consumers: the committing session, through the rewritten help; the two registry-coverage tests, which assert exit 2 and the refusal line for that member's cases in a linked worktree and fail on anything else, so a refusal that stops firing is not silently observed.
- **The Windows probe step** (delta 6). Producer: each `install-smoke-pwsh-windows` matrix entry. Consumers: the build session, whose next act turns on it, and afterwards the standing witness of gate-sdk/SPEC.md §gen-pre-commit's Windows sentence.
- **`run-smoke.ps1`** (delta 7). Producer: each `install-smoke-pwsh-windows` matrix entry. Consumers: the run's verdict. The installer_smoke validate parser reads `run-smoke.sh`'s headers alone and does not run on Windows, so it gains no reader.
- **The moved steps and the bootstrap witness** (delta 7). Producer: `crate-tests-windows`' x64 entry, and each PowerShell matrix entry. Consumers: the run's verdict and the leg-naming prose re-pointed in delta 9. `check-action-job-ref` reads leg keys in prose through `GATE_SDK_JOB_REF_PATTERNS`, and reds a surviving mention of a deleted job, which the roster below names.
- **Every member's value** (delta 7). The profile set is `installer/profiles.list`'s rows plus `full`, read at run time rather than listed here, so a profile added later is covered with no edit.

## Existing sections updated

Roster probe: the probes above, plus `git grep -n -i "hook" -- '*/SPEC.md' '*/README.md' docs/install.md docs/site-architecture.md docs/positioning.md CLAUDE.md`, each hit read for the hook's shape or the retired legs.

- `gate-sdk/SPEC.md` — new §git-hook (delta 1); §gen-pre-commit (deltas 2 and 6); §The `# graph:` manifest (deltas 3 and 8); §check-gate-binary-fresh (delta 4); §check-crate-arms (delta 5); §lib/gate.sh, §run-gates, §The port-candidate criteria (delta 8); §check-graph (deltas 3, 8 and 9); the front-end parity and Windows-leg sentences (delta 9).
- `native/src/emit/mod.rs` — the arm table row, and the arm's new module beside it (delta 1).
- `native/src/emit/git_hooks.rs` (delta 2).
- `native/src/gates/crate_arms.rs` (delta 5).
- `native/src/gates/mod.rs` (delta 5).
- `native/src/runner.rs` (deltas 8 and 9).
- `native/src/gates/graph.rs` (delta 9).
- `native/src/gates/reads_couples.rs` (delta 9).
- `scripts/git-hooks/pre-commit` — regenerated (delta 2).
- `scripts/git-hooks/commit-msg` — regenerated (delta 2).
- `gate-sdk/lib/gate.sh` (delta 8).
- `gate-sdk/gate-tests/check-graph-tree.test.sh` (deltas 3 and 9).
- `gate-sdk/gate-tests/check-graph/good/SPEC-example-gate.md` (delta 3).
- `delegation-kit/SPEC.md` (delta 8).
- `guard-kit/SPEC.md` (delta 9).
- `installer/SPEC.md` — §Requirements, §Reviewing the pre-commit hook (delta 6); §The consumer smoke, §Platform resolution (delta 7); every Windows-leg sentence (delta 9).
- `installer/consumer-smoke/run-smoke.sh` (delta 7).
- `installer/consumer-smoke/run-smoke.ps1` — new (delta 7).
- `.github/workflows/gates.yml` (deltas 6, 7 and 9).
- `docs/install.md` (delta 6).
- `docs/site-architecture.md` (delta 9).
- `docs/positioning.md` (delta 9).
- `native/runners.list` (delta 9).
- `native/targets.list` (delta 9).
- `canon-kit/README.md` (delta 9).
- `context-kit/README.md` (delta 9).
- `delegation-kit/README.md` (delta 9).
- `doctrine-kit/README.md` (delta 9).
- `evidence-kit/README.md` (delta 9).
- `lifecycle-kit/README.md` (delta 9).
- `queue-kit/README.md` (delta 9).
- `site-kit/README.md` (delta 9).
- `CLAUDE.md` (delta 9).
- `TASK-QUEUE.md` — the entry naming the live CI leg `install-smoke-sh-windows` re-points at the PowerShell leg, and the two paired entries move at merge (delta 7).
- `docs/gate-sdk/SPEC.md` — the generated on-site mirror, as every `docs/<kit>/` page below, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `docs/installer/SPEC.md` — mirror (deltas 6, 7 and 9).
- `docs/delegation-kit/SPEC.md` — mirror (delta 8).
- `docs/guard-kit/SPEC.md` — mirror (delta 9).
- `docs/<kit>/README.md` — the mirror of every README delta 9 edits (delta 9).
- `docs/check-graph.html` — regenerated with the hooks (delta 2).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **gate-sdk/SPEC.md §gen-pre-commit**: the generated hooks become a two-line handoff to the gate binary's `--git-hook` arm, which reads the manifests at commit time, so a manifest or registry edit no longer needs a regeneration, and `update` rewrites an adopter's hooks. Delta 3's outcome adds its own bullet (deltas 2 and 3).
<!-- update-target-exempt: a carried survey's frozen finding, truncated at the next scope boundary rather than edited -->
- `.workflow/survey-record.md`

## Retired spellings

- `gate_staged_matches` — the shell matcher the hook spliced, deleted with its splice (delta 8).
- `install-smoke-sh-windows` — the x64 sh Windows job, deleted (delta 7).
- `install-smoke-sh-windows-arm64` — the arm64 sh Windows job, deleted (delta 7).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — `bash gate-sdk/bin/build-native.sh`, the full battery, the gate-sdk, delegation-kit and guard-kit fixture suites, the gate-sdk kit smoke and the installer consumer smoke green on the landing commit; the Windows hook, probe, driver and moved steps green on the mid-iteration push's CI run, whose run id is the entries' evidence. Both entries move to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), and not before that run is green.
