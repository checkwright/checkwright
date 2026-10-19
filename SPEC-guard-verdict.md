# SPEC amendment: guard-verdict

Pairs `guard-verdict-typed-operand`. No arm prints the shell guard's decision for a command a session types. A session re-verifying a guard finding hand-builds the hook's payload in a scratch file, after the guard's own expansion rule refuses the one-line spellings. Settling whether a verdict regressed from the last release has cost a tag worktree and a cargo build. This amendment adds `--guard-verdict`, an arm taking the command as an operand and printing the decision the hook returns, in the tree or in `--run-guard-tests`' sandbox, from the running binary or one the caller names. Obtaining a released binary stays manual: its per-target Release archive holds one, and no arm fetches it.

**What the tree does today**, run at spec:

- **The probes.** Every payload probe this session ran was a hand-written JSON file fed to `--hook shell-guard`, after a loop over payloads was refused by the expansion rule.
  - The ad-hoc probes ran without `GUARD_KIT_LOG` pointed at scratch, so their fall-throughs landed in the live friction log, against guard-kit/SPEC.md §Testing's ad-hoc probe rule. They were removed by hand.
  - The arm owns that redirect, so a verdict probe cannot make the slip.
- **The spellings the arm would take** were fed to the built hook as payloads, with the arm not yet existing; the guard reads only the outer text. Each exits 0 with nothing printed, left to the harness's allowlist of the front end:
  - `bash gate-sdk/bin/run-gates.sh --guard-verdict -- 'echo $(date)'`;
  - `… --guard-verdict -- "touch '\.tmp/a'"`;
  - `… --guard-verdict -- "printf x > .tmp/$'x'"`;
  - `… --guard-verdict --stdin`, with a quoted-delimiter heredoc carrying `"$(date)"`, a single quote and `${HOME}`.
- **The v0.31.0 Release** carries `checkwright-gates-0.31.0-<target>.tar.gz` for eight targets (`gh release view v0.31.0 --json assets`).
  - The `x86_64-unknown-linux-gnu` archive holds `checkwright-gates` and a `.sha256` sidecar. The binary's digest matches the sidecar, and the binary is stored without its execute bit.
- **That binary in this tree** blocks every command. Today's `scripts/guard-config.knobs` line 33 sets `GUARD_KIT_CONSUMER_CASES`, a name v0.31.0 does not declare, and the guard's refused knob read blocks.
- **That binary in a fresh `git init` sandbox** with no knob files answers today's payload grammar. It gives the same two verdicts as the current binary: fall-through for the `--guard-verdict` spelling carrying `touch '\.tmp/a'`, and the expansion block for the lead template's first-step line.
- **The decision table's runner** (`native/src/emit/run_guard_tests.rs`) already holds the pieces the arm needs, each one copy: `payload`, `classify` (the five-way ladder), `substitute` (the three substitutions), `build_sandbox`, and `decide`, which spawns `<binary> --hook shell-guard` in the sandbox and discards stderr.

**The older-binary half is settled as an operand, and fetching stays manual.**
- The arm takes the binary to ask with `--bin`, so asking v0.31.0 is a download and an extraction, never a build.
- A fetch would make the arm a network spawner. It would also be a second copy of the installer's target resolution and digest check, which the bootstraps own in shell and PowerShell (installer/SPEC.md §The gate binary).
- `--sandbox` is what makes an older binary's answer readable. In the tree, a knob a later release added refuses the older binary's knob read.

**One act depends on a sibling's landing.** `hook-binding-single-source` builds first, by the set record's order.
- **If it lands its binding module**, `--run-guard-tests`' payload builder reads that module's spellings, and the arm calls the same builder wherever that unit leaves it.
- **If it records a refusal instead**, the builder stays `run_guard_tests::payload`, made `pub(crate)`, and the arm calls it there.

Either way the arm constructs no payload of its own.

## What changes

### (1) `--guard-verdict` prints the hook's decision for a typed command {design-bearing} {user-facing: guard-verdict-typed-operand's Deliverable, "a guard-kit arm taking the command as an operand and printing the decision the hook would return, plus a way to ask an older binary the same, whose `--hook shell-guard` it already has; whether that half reads a released binary or stays manual is the amendment's to settle"}

`guard-kit/SPEC.md`, a new section `## guard-verdict` after `## scan-prompts`. **Not yet applied.**

> `--guard-verdict [--sandbox] [--powershell] [--background] [--bin <binary>] (--stdin | [--] <command>)` prints the decision the shell guard returns for `<command>`. **The member is the gate binary's arm**, reached as `bash gate-sdk/bin/run-gates.sh --guard-verdict …`. It is spelled bare because its child runs the consumer's rule command and any binary `--bin` names (gate-sdk/SPEC.md §The non-gate arm).
>
> **What it asks.** It builds the hook payload with `--run-guard-tests`' own builder, carrying the command, `PowerShell` as the tool under `--powershell` (else `Bash`), and `run_in_background` under `--background`. It spawns `<binary> --hook shell-guard` with the payload on stdin. `<binary>` is the running binary, or the path `--bin` names, so a released binary is asked the same question: extract it from the Release's per-target archive `checkwright-gates-<version>-<target>.tar.gz`, check it against the sidecar beside it, and mark it executable. The child's `GATE_SDK_NATIVE_BIN` names the binary asked, and its `GUARD_KIT_LOG` names a file of the arm's own under `GATE_SDK_TMP_DIR`, removed once the child exits. So a verdict probe never files a synthetic command as friction.
>
> **Where it asks.**
>
> - **In the tree by default**, from the git toplevel. The answer is the one the hook returns there: the consumer's rule command, the knob files, the tracked set, the liveness records and the settings allowlist all take part.
> - **Under `--sandbox`**, from inside a sandbox `--run-guard-tests` builds, by its own builder, removed once the child exits (§Testing). There the answer is the generic ruleset's alone, the one a row of the decision table pins, and it depends on no knob file. That makes it the mode for asking a release: in the tree, a knob a later release added refuses an earlier binary's knob read, and the guard then blocks every command.
>
> The command takes the decision table's three substitutions, `@ROOT@` naming the mode's root, so a table row's command is pasted as written. It arrives as the one operand, read verbatim after `--`, or under `--stdin` as the whole of standard input with one trailing newline dropped. A quoted-delimiter heredoc carries any spelling past the guard that reads the arm's own command line.
>
> **What it prints.** Line one is the decision, in the decision table's ladder (§Testing): `block`, `advise`, `allow`, `rewrite` or `fallthrough`, or `exit<rc>` and `unknown` for an answer the ladder does not place. The child's stdout follows verbatim, then its stderr verbatim, so a block's steer and an allow's reason are read where the hook wrote them. The ladder is `--run-guard-tests`' own function.
>
> **Exit:** 0 when the child answered, whatever its decision; 2 for a usage error, a `--bin` path that is not an executable file, a sandbox that cannot be built, or a child that cannot be spawned. `-h` or `--help` first prints the usage at exit 0. Every grammar refusal prints it on stderr at exit 2:
>
> - a surplus operand;
> - a leading-`-` operand before `--`;
> - a command operand beside `--stdin`;
> - no command at all.
>
> Its knob roster is `GATE_SDK_TMP_DIR` and the shell guard's own knobs, which the child reads itself (gate-sdk/SPEC.md §The non-gate arm, a dispatching arm's declaration). **Honest limits:** in the tree an earlier binary's answer reads today's knob files, as stated above. The sandbox carries the decision table's fixture files and settings and nothing of the tree's. The arm reports what the binary asked returns, and never what a harness would do with an answer it does not recognize.
>
> The arm is `native/src/emit/guard_verdict.rs` and its `ARMS` row. Its seam cases live in `gate-tests/guard-verdict.test.sh`.

`guard-kit/SPEC.md` §Testing, the paragraph opening **Any ad-hoc invocation of the shell guard must set `GUARD_KIT_LOG` to a scratch path.**, reads as follows. **Not yet applied.**

> **Ask the guard for a verdict with `--guard-verdict` (§guard-verdict), which points `GUARD_KIT_LOG` at scratch itself; any other ad-hoc invocation of the shell guard must set it to a scratch path.** The logger's default is the *live* friction log, so a probe driving the guard files its synthetic commands as real friction and skews the next close's ranking. The kit's own runners all set it: the decision table into its sandbox, the `scan-prompts` fixture into its own, and the verdict arm into its own file. **No scanner is proposed:** the polluting caller is a one-off script in the gitignored scratch dir, so there is no committed corpus to scan. The convention is carried by this sentence and by the `--scratch-run` arm's echo-at-execution.

`native/src/emit/run_guard_tests.rs`. **Not yet applied.**

- `decide`'s spawn becomes `pub(crate) fn ask(bin: &str, root: &str, log: &str, payload: &str) -> Result<(i32, String, String), String>`, answering the child's code, stdout and stderr. `decide` calls it with the running binary and classifies the stdout as today.
- `classify`, `substitute`, `build_sandbox` and the payload builder become `pub(crate)`. The payload builder does so here only if `hook-binding-single-source` records a refusal, and otherwise wherever that unit leaves it.
- `ask`'s `spec:` comment reads "one ask of a binary's `--hook shell-guard` from inside `root`, shared with the verdict arm".

`native/src/emit/guard_verdict.rs`, new. **Not yet applied.**

- `KNOBS`: `GATE_SDK_TMP_DIR` and the entries of `crate::guard::host::KNOBS`.
- `run(args) -> i32` parses the grammar above, resolves the root, reads the command and substitutes it, builds the payload, and calls `ask`.
  - The root is the toplevel, or a sandbox from `build_sandbox` removed after.
  - The log file is created under `GATE_SDK_TMP_DIR` with a run-unique name and removed after.
  - It prints the class and the two streams.
- Its `#[cfg(test)]` module covers:
  - each grammar refusal and the help;
  - `--stdin` dropping one trailing newline;
  - the substitutions with each root;
  - the rendering of a class line followed by both streams.

`native/src/emit/mod.rs`: an `ARMS` row `("--guard-verdict", Arm::Run(guard_verdict::run), guard_verdict::KNOBS)`. `FENCE_SAFE_ARMS` and `READ_ONLY_ARMS` do not take it. **Not yet applied.**

`guard-kit/gate-tests/guard-verdict.test.sh`, new, on `lib/test-hermetic.sh`. Each case runs the arm through `gate_arm_run` from a `git init` sandbox of its own, with `GATE_SDK_TMP_DIR` and `GUARD_KIT_LOG` pinned into it. **Not yet applied.**

Each sandbox case takes a row of `guard-kit/guard-tests/cases.tsv` and expects its decision:

- **sandbox-block.** `--sandbox -- 'cd deploy && ls'` prints `block` first and the rule's steer after it, exit 0.
- **sandbox-allow.** `--sandbox -- 'touch .tmp/a.txt'` prints `allow` and the allow envelope.
- **sandbox-fallthrough.** `--sandbox -- 'git status'` prints `fallthrough` alone.
- **stdin.** `--sandbox --stdin`, fed `cd deploy && ls` and a newline, prints `block`.
- **substitution.** `--sandbox -- 'bash @ROOT@/scripts/check-foo.sh'` prints `rewrite`.
- **bin.** `--sandbox --bin <the running binary's path>` answers as the bare form. A `--bin` naming a non-executable file exits 2.
- **in-tree, no friction.** An in-tree fall-through leaves the pinned `GUARD_KIT_LOG` byte-identical, and leaves no file of the arm's under the scratch dir.
- **refusals.** No command, a surplus operand, `-x` before `--`, and an operand beside `--stdin` each exit 2 with the usage on stderr.


### (2) gate-sdk records what the arm spawns {mechanical}

`gate-sdk/SPEC.md` §The non-gate arm, the **Split between the arm and its subject.** bullet. After its sentence on `--run-guard-tests` it adds the following. **Not yet applied.**

> `--guard-verdict` (guard-kit/SPEC.md §guard-verdict) spawns once, as the `--hook shell-guard` member, the running binary or the one `--bin` names. It spawns `git` to build a sandbox under `--sandbox`. Through the member it spawns, in the tree, the argv `GUARD_KIT_CONSUMER_RULES_CMD` names.

### (3) The README and the release record name the arm {mechanical} {user-facing: as delta 1}

`guard-kit/README.md`, the command block under `## Test`, a second line, and a sentence under the block. **Not yet applied.**

> ```sh
> "$gates" --guard-verdict [--sandbox] -- '<command>'    # the decision the shell guard returns for one command
> ```
>
> `--guard-verdict --sandbox --bin <binary>` asks another build, such as a release's binary from its per-target archive, the same question in the decision table's sandbox ([guard-kit/SPEC.md §guard-verdict](SPEC.md#guard-verdict)).

`.workflow/release-declarations.md`, `## Behavior changes`, a new bullet. **Not yet applied.**

> - **guard-kit `--guard-verdict`** — a new arm prints the decision the shell guard returns for a command you give it as an operand or on stdin: the decision word, then the hook's own output. Its child's friction log is a scratch file, so the probe is never filed as friction. `--sandbox` asks in the decision table's sandbox rather than your tree, and `--bin <binary>` asks another build, such as an earlier release's binary taken from its Release archive. Nothing to do.

## Producers and consumers

- **The arm (delta 1).**
  - Producer: a session, through the front end, the caller this arm owes (gate-sdk/SPEC.md §The non-gate arm). The guard-kit §Testing sentence delta 1 rewrites directs a session there, and the seam test drives it.
  - Consumer of its output: the session reading the decision line and the streams.
  - Its one new interface field is the decision line, read by that session and by the seam test's assertions. The streams are the hook's own.
- **The child (delta 1).** `--hook shell-guard` of the binary asked, whose payload grammar is the binding table's (gate-sdk/SPEC.md §The hook contract). Its knob reads are its own, declared on the arm's roster.
  - Measured above: an earlier binary reads the payload the shared builder makes.
- **The shared pieces (delta 1).**
  - `ask`, `classify`, `substitute`, `build_sandbox` and the payload builder gain a second caller, and none changes behavior for its first: `decide` keeps its answer.
  - The roster-holding readers of the arm table, found by `git grep -n "ARMS\b" -- native/src` and gate-sdk/SPEC.md §The non-gate arm:
    - the arm-table unit test, which resolves the row's file and needs its test module;
    - `check-crate-arms`;
    - `installer/SPEC.md` §The front door's verbs, whose pinned arm set which reads only front-door pages, none of which this amendment edits;
    - the no-subcommand roster `main` prints, derived.
- **The log redirect (delta 1).** Its consumer is the hook's fall-through logger, which then writes a file the arm removes, so `--emit scan-prompts` and the close's friction ranking read no probe.
- **Point 5:** no delta narrows a corpus.
- **Point 6:** no delta obliges the members of a corpus.

## Existing sections updated

The roster came from `git grep -n -e "--hook shell-guard" -e "GUARD_KIT_LOG" -- ':!docs/' ':!TASK-QUEUE.md'`, from `git grep -n "fn decide\|fn payload\|fn classify\|fn substitute\|fn build_sandbox" -- native/src`, and from the survey of the arm table's roster-holding readers above.

- `guard-kit/SPEC.md`, the new §guard-verdict and §Testing's ad-hoc probe paragraph (delta 1).
- `native/src/emit/run_guard_tests.rs`, `native/src/emit/guard_verdict.rs`, `native/src/emit/mod.rs` (delta 1).
- `guard-kit/gate-tests/guard-verdict.test.sh` (delta 1).
- `gate-sdk/SPEC.md` §The non-gate arm (delta 2).
- `guard-kit/README.md`, `.workflow/release-declarations.md` (delta 3).
- `docs/guard-kit/SPEC.md` and `docs/gate-sdk/SPEC.md`, the generated mirrors (all deltas), and `docs/guard-kit/README.md`, the mirror of delta 3's README edit (`docs/generated-projections.md` rosters the README mirrors beside the SPEC ones).

## Retired spellings

- None — no name is retired. The runner's functions keep their names and gain a visibility.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **A release asked** — `--guard-verdict --sandbox --bin <the v0.31.0 binary>` run on the five `bounded-write-backslash-grant` spellings, its answers recorded where that entry's validate reads them.
- [ ] **Suites green** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-guard-tests`, `--run-gate-tests guard-kit/gate-tests guard-kit/checks`, and the battery.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, at a stage before the drain stage.
