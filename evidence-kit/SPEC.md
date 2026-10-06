# evidence-kit — a held-constant baseline and a committed per-run evidence manifest for validate

lifecycle-kit's stage evidence proves a stage was *invoked*; it cannot prove the stage produced its green result. evidence-kit closes that gap with three coupled surfaces, so a validate stamp is backed by a recorded, hashable verdict rather than a claim:

- a held-constant test baseline;
- a committed per-run evidence manifest;
- a codified run contract.

It is a kit of its own, not a lifecycle-kit extension, because the evidence manifest is a wire contract a future external verifier consumes. Its format is versioned, stable and hashable independent of the state machine, and a consumer that runs no iteration lifecycle can adopt the kit. lifecycle-kit integration is optional and arrives through generic knobs on its side of the seam (§lifecycle-kit integration).

## Layout and configuration

The kit is vendored beside [gate-sdk](../gate-sdk/) (required); its gates register in the consumer's `gates.list` by name and resolve through gate-sdk's multi-kit path.

Config is a **knob file**: copy `templates/evidence-config.knobs` into the gates dir as `evidence-config.knobs`, or point `EVIDENCE_KIT_KNOB_FILE` elsewhere, and set any knob below. Defaults fill what the file leaves unset. A gitignored `evidence-config.local.knobs` in the gates dir is the home for a private value a tracked file cannot carry.

The knobs are **static**: the binary resolves them in process from its own defaults table and the consumer's knob file. `bash gate-sdk/bin/run-gates.sh --emit knob-roster` prints each one with its shape and rendered default. A derived default below names the knob it reads as `${NAME}`, or as `${NAME:-<default>}` so the roster's rendered literal reads as agreement.

gate-sdk/SPEC.md §The knob file owns the grammar, the `.local` overlay, the environment-over-file precedence for a scalar, the knob reference, the declared family and the refusals. Those refusals cover a set `EVIDENCE_KIT_KNOB_FILE` that does not exist, a left-behind `evidence-config.sh` or `evidence-config.local.sh`, and a non-empty file named by the retired `EVIDENCE_KIT_CONFIG_FILE`.

The kit's table validator refuses at exit 2, so a broken config gates nothing, when:

- `EVIDENCE_KIT_PARSER`, `EVIDENCE_KIT_BASELINE_FILE`, `EVIDENCE_KIT_MANIFEST_FILE`, `EVIDENCE_KIT_QUEUE_FILE` or `EVIDENCE_KIT_DONE_SECTION` is empty;
- a suite name is not a valid `EVIDENCE_KIT_RUN_<suite>` suffix.

Knobs, this repo's surface names as defaults:

- `EVIDENCE_KIT_SUITES` — the ordered suite names; default empty. A file may splice the derived suites in with `EVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES` at the position the order wants them.
- `EVIDENCE_KIT_FIXTURE_SUITES` — the fixture suites, derived from `GATE_SDK_ROOT`, `GATE_SDK_KIT_DIRS` and `GATE_SDK_GATES_DIR`: one suite per directory carrying a `gate-tests/` tree, the kit roots in order then the gates directory. A suite is named by the directory's basename with `-` turned to `_` (gate-sdk/SPEC.md §lib/gate.sh). No reader reads the knob directly: it exists to be referenced into the roster, and a consumer who does not want the derived suites leaves the reference out.
- `EVIDENCE_KIT_RUN_<suite>` — the command that runs a suite, captured to a log under `EVIDENCE_KIT_TMP_DIR`. A **declared family**: each fixture suite has a derived member, and every other suite's member is the consumer's to set. A file or exported member of the same name replaces a derived one.
  - The derived member reads the same three inputs and `GATE_SDK_NATIVE_BIN`. It is the fixture runner over the suite's tests directory and, when one exists, its checks directory, on the binary door: `<door> --run-gate-tests <tests-dir> <checks-dir>`, the door spelled by gate-sdk's `gate_native_bin_spelled` rule.
  - The directories are spelled relative to the working directory, and the door's relative value is read against it as the binary reads every relative knob. So `--run-validate` spawns the member in the one directory both are bound to, the tree's root: the toplevel a front end changes to, or a directory below it. A directory below the tree root reads as a root of its own, its manifest included.
- `EVIDENCE_KIT_PARSER` — a parser adapter name or a consumer command mapping a captured log to `<scenario> <pass|fail|ignore>` lines; default `exit-code`.
- `EVIDENCE_KIT_PARSER_<suite>` — a per-suite parser override with the same value grammar; a declared family with no derived member. An unset suite falls through to the global knob, and an override resolving *empty* falls through too. The parser rules follow the roster.
- `EVIDENCE_KIT_SCENARIO_GLOBS` — optional per-suite globs; configuring one arms the manifest↔disk set-equality assertion for that suite.
- `EVIDENCE_KIT_BASELINE_FILE` (default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/validate-baseline.txt`), `EVIDENCE_KIT_MANIFEST_FILE` (default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/validate-evidence.txt`), `EVIDENCE_KIT_SKIP_FILE` (default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/validate-skips.txt`). The skip file is the side-channel a consumer harness writes when it self-skips a scenario (§bin/diff-baseline.sh).
- `EVIDENCE_KIT_TMP_DIR` — the scratch dir run logs land in; default `${GATE_SDK_TMP_DIR:-.tmp}`.
- `EVIDENCE_KIT_LOCK_FILE` — the producer-liveness lock (§The producer-liveness lock), default `${EVIDENCE_KIT_TMP_DIR:-.tmp}/run-validate.lock`. It resolves *through* the scratch knob, so a consumer that moves the scratch dir moves the lock with it.
- `EVIDENCE_KIT_QUEUE_FILE` / `EVIDENCE_KIT_STATE_FILE` — the lifecycle surfaces read for the manifest's optional close-entry and stamp-coupling assertions; defaults `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}` and `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt`.
- `EVIDENCE_KIT_RUN_ID` — the evidence-line key when no lifecycle queue header names the iteration; default empty.
- `EVIDENCE_KIT_PRE_HOOK` — an optional per-suite pre-run command (projection regen, container teardown) kept on the consumer side of the spine; default empty.
- `EVIDENCE_KIT_PERMANENT_SLUGS` — blocking slugs that satisfy baseline liveness without a live queue task; default empty.
- `EVIDENCE_KIT_DONE_SECTION` — the queue section whose entries are finished, so a blocking slug found only there is stale; default `Done` (queue-kit's).
- `EVIDENCE_KIT_RUNNER_DOC` (default `README.md`, resolved against the git toplevel) — the doc whose battery-roster block `check-battery-roster` holds against the suite roster. It is gate-local: nothing in the validate run path reads it. The name mirrors gate-sdk's `GATE_SDK_RUNNER_DOC`, since in a tree vendoring both kits the two name the same physical doc for two different assertions.

**The two declared families** are `EVIDENCE_KIT_RUN_<suite>` and `EVIDENCE_KIT_PARSER_<suite>`, the second named to mirror the first (gate-sdk/SPEC.md §The knob file). A family is a *resolution set, never a roster*: the suite roster is `EVIDENCE_KIT_SUITES`, and the family answers *what is this suite's value*. `EVIDENCE_KIT_RUN_ID` is the run-id row and never a member, since a declared row the prefix spells is excluded from its family. So a suite literally named `ID` has no run member and `--run-validate` refuses it.

**Suite granularity is a floor, not a ceiling.** A suite whose runner reports per-case results carries a parser that says so, while its siblings keep the global adapter. The kit ships two parser arms a consumer names in the knob instead of authoring a script, `--emit parse-gates-log` and `--emit parse-smoke-log`. Both declare an empty knob roster and read their operands off argv, so a consumer has no enabling configuration to forget (gate-sdk/SPEC.md §The non-gate arm). A log that does not resolve is exit 2 for either.

**A value naming neither bundled adapter stays a consumer command the tools word-split and spawn** (§The evidence adapters), so an adopter points the knob anywhere. The shipped arms are reached as such values. Compiling them in as adapters would privilege them over a consumer's own and make the `--emit parse-*` arms unreachable through the path documented here.

**`parse-gates-log`** is the value `bash gate-sdk/bin/run-gates.sh --emit parse-gates-log`. It maps the verbose `run-gates` log to one scenario per registered gate. An existing gate turning red then diffs as a new failure even while a sibling gate is legitimately held red, which the whole-battery `exit-code` scenario cannot tell apart.

- It reads the two-space-indented `PASS:` / `FAIL:` tail line and its gate name, and nothing else.
- A signalled child renders in the `(exit 128+n)` shape, never as a further tail spelling.
- The battery flushes the lines in registry order, not completion order, so one run's scenario record diffs against another's (gate-sdk/SPEC.md §run-gates).

**`parse-smoke-log`** emits **one scenario per arm** of a consumer smoke: arm names, not test names. The per-suite override admits that because it names a command over a log and says nothing about what a scenario must be. The arm roster is **not listed in the parser**. It is derived from the driver, so an arm added or reworded moves the scenario set with it. That derivation makes the driver's printed headers a parsed contract rather than narration, and the driver says so where it writes them. The roster arrives in one of two forms, and every other rule here holds for both:

- **The driver form**, `bash gate-sdk/bin/run-gates.sh --emit parse-smoke-log <driver>`. The driver is the consumer's own file, so it is the arm's leading positional and the arm holds no default for it. A driver that does not resolve is exit 2 before any line of the log is judged. The roster is the smoke script's own top-level headers.
- **The log-only form**, for a compiled driver, which has no `printf` headers. Given the log alone, the arm reads the roster off the log's own `smoke-roster: <name>` lines, in order, the last naming the completion marker. Each must precede the first header, so a header or the marker ahead of the last one is exit 2. The driver prints those lines first, from the table it prints its headers from, so the two cannot differ.

The log arrives **last**, after everything the knob value spells, which §The evidence adapters' appended-log rule fixes.

**A header is a top-level `printf '<literal>\n'` with no redirect, and its name is the literal up to its parenthetical**, so a header carrying an interpolated profile still names one stable scenario. Two headers name no stable scenario and are skipped: one whose literal is empty, and one that is entirely a format specifier.

**The completion marker is positional: a driver's *last* top-level header is its completion announcement, and the ones before it are its arms.** The derivation yields the driver's clean line by construction, with no literal held in the parser.

- **A hazard with no oracle.** A header printed *after* the completion line would silently become the marker and demote the real one to an arm, so the driver carries the clause beside its clean line too.
- **Fail-closed.** Fewer than two headers, or roster lines, cannot yield an arm and a marker, so a header shape the derivation cannot read is exit 2, as is a log declaring no roster. `--run-validate`'s produced-no-result guard stands behind it.

**The suite's fail-fast shape is what makes per-arm rows assert.** The smoke exits at its first failure, so arms behind that point never print, and §bin/diff-baseline.sh's directional rule reds a baselined `pass` scenario that is red **or absent**. An early abort reds every arm behind it instead of hiding them.

**The attribution leans on fail-fast, not on the verdict, and that is its honest limit.** An arm is judged failed when the log reaches its header and neither a later header nor the run's own clean line follows. A smoke that gained a *non-fatal* failure path would read that arm as passing. The suite's verdict would still be right, because the arms behind a real abort are absent either way, and the blame would be wrong.

## Per-component contracts

### The evidence adapters

`native/src/evidence.rs` is the kit's sole holder of its adapters and of the readers its gates share. They are values and adapters only, never tool structure, and the knobs they read are the kit's table (§Layout and configuration):

- the suite's configured run command, looked up in the `EVIDENCE_KIT_RUN_` family by suite;
- the data-line filter;
- the queue-iteration, run-key and cursor readers, self-contained so the kit reads lifecycle state without a lifecycle-kit dependency.

The two axes come from two surfaces: the queue header names the iteration, and the state file's **last data line** is the stage cursor. The cursor reader answers no stage on both no-cursor shapes, an absent state file and a file truncated to its preamble with no data line yet. A caller reads an empty stage for either. Both surfaces' shapes are lifecycle-kit's (lifecycle-kit/SPEC.md §The state machine).

**The run key** is the queue header's iteration where it names one, else `EVIDENCE_KIT_RUN_ID`. lifecycle-kit's unnamed-iteration placeholder (lifecycle-kit/SPEC.md §check-stage-evidence) names none, so it never reaches a manifest line, and a run with no key is refused at the guards' exit 2 (§bin/run-validate.sh).

**The parser adapters** map a captured log to `<scenario> <pass|fail|ignore>` lines:

- `libtest` reads per-test result lines (a Rust `cargo test` suite);
- `exit-code` emits one scenario per suite, keyed off the suite command's exit status;
- any other value is a consumer command run on the log.

`exit-code` is the only adapter the suite's exit status reaches. Under `libtest` or a consumer command, two distinct non-zero exits are indistinguishable in the row. A suite that means to separate *failed* from *could-not-run* says so **in its log**, where its parser can read it.

**`libtest` ships and no suite is owed it.** Pointed at a mature crate suite, it turns one row into one row per test, over the fastest-moving surface a repository has. A renamed or deleted test is routine work, and each one reds the baseline for a non-defect. That is the hand-maintained roster §Baseline manifest refuses. What the adapter would buy is per-test absence detection, since the absent-row rule already catches a red in the suite, and absence is what churn produces. The trigger that re-opens the call is a measurement: once a suite's test names stop churning, so that a baseline row survives an iteration, `libtest` is the right answer and the rows follow a run as §Baseline manifest requires.

**Which adapter a suite gets** is the per-suite resolution: `EVIDENCE_KIT_PARSER_<suite>`, else the global `EVIDENCE_KIT_PARSER`. The dispatch sits behind it, so both callers, `--run-validate` and `--diff-baseline`, inherit per-suite parsing. The resolution is a named helper because `--run-validate`'s produced-no-result diagnostic must name the *effective* parser. Naming the global while an override produced the empty result would misreport the guard the per-gate baseline leans on.

**A consumer parser command** binds any value written against it in four ways:

- It receives the log path alone, with no suite name and no exit status. A field most parsers never read is not passed, and a consumer needing exit-code semantics for a suite leaves that suite on the global adapter.
- The value word-splits: the dispatch splits it on whitespace and spawns the words with no shell. So **no argument a value spells may contain a space**, and a value needing to pass one passes a rule that derives it instead.
- The log path is **appended after everything the value spells**: a value carrying its own operands spells them first and the log arrives last.
- Its exit status is not read: the scenario lines are its stdout whatever it exits with. A command that prints none, or cannot be spawned, leaves the produced-no-result guard to fail the run (§bin/run-validate.sh).

**A third bare-name adapter beside `exit-code` and `libtest` is refused.** A kit-shipped parser is an `--emit` arm reached through the front-end, the convention gate-sdk/SPEC.md §The knob file names, which is how a parser value reaches `parse-gates-log` and `parse-smoke-log`. A consumer whose parser the kit does not ship keeps authoring a command, and that command extends the set rather than replacing a working default, since `exit-code` is the default.

**Coverage.** Neither adapter is a gate, so their branches are covered by `gate-tests/evidence-lib.test.sh`, driven through the front-end. This repo's own wiring of the knob has a second suite, `scripts/gate-tests/evidence-parser-values.test.sh`, driven the same way. It drives the *configured value* rather than a hardcoded arm, so it notices a change that deletes a value's target.

- Both read the dispatch's answer out of `--diff-baseline`'s findings against a fixture baseline, which is what a caller can observe: a scenario the parser failed to produce reds as an absent row.
- They cover the per-suite dispatch with its global fall-through, and the absent-from-baseline triple.
- The triple's `ignore` edge carries a test of its own, since the narrow side is the half a later session widens by accident.

**A dual-held helper's parity is compared by classification, never by representation.** The comparison drives one canned corpus through each holder and compares what each classifies. One side may answer in exit codes and a stdout line and the other in an enum, and a difference of representation is no disagreement. It commits no expected file: a maintained golden is a third copy to drift, and the failure the comparison catches is one side edited without the other. context-kit's floor predicate takes the same shape (context-kit/SPEC.md §bin/env-probe), reusing the rule and no part of this kit's mechanism, and this paragraph is the rule's one home.

### Baseline manifest

Held-constant, edited by human commit only. It is a tracked checked projection of the workflow directory, so its first line is the pointer-form header `# contract: evidence-kit/SPEC.md §Baseline manifest — held-constant validate baseline: <suite> <scenario> <status> [<slug> [reproduces-at=<rev>]]` (the form ruled by gate-sdk/SPEC.md §The workflow directory, whose em-dash tail carries the line grammar). Below it, one line per known scenario, `<suite> <scenario> <status> [<slug> [reproduces-at=<rev>]]`:

- **`<slug>`**, the blocking slug, is required exactly when status is `fail` or `ignore` and forbidden when `pass`. Each slug resolves to a live queue task (the queue-file knob) or a configured permanent marker.
- **`reproduces-at=<rev>`** records a commit at which the row's red reproduces. It is allowed only after a slug, `<rev>` is 7 to 40 lowercase hex characters, and §check-evidence-baseline says when a row owes it. The token is optional, so a baseline with no flips never mentions it and an older row needs no migration.
- A `pass` row carrying `reproduces-at=` is red, and a fifth field of any other shape, or a sixth field, is a grammar error.

Tooling never writes the file: a promotion, a held-constant red recovering to pass, is a human commit, which keeps the baseline honest. Any row move also stales the suite's recorded evidence line; the deferral recipe that re-records it is §check-evidence-manifest's.

**The fail-closed rule keys on `fail`, never on non-pass.** For a scenario with no baseline row:

- observed `fail` is a new failure, so a missing `pass` row loses no enforcement;
- observed `pass` is no red. Its cost is classification: a later regression reads as a new scenario, not as a silent green;
- observed `ignore` is **silent**: an ignored test is a non-verdict, with no assertion here to converge on.

Widening to non-pass would redden a libtest consumer's newly-added `#[ignore]` test. A fail-closed appetite for absent `ignore` is a separate argued change with its own delta to this section, never a rider. The skip demotion (§bin/diff-baseline.sh) stays a baseline-row concern and does not reach absent scenarios: a skip-demoted observed `pass` absent from the baseline falls under the classification-cost rule.

So an observed set carrying no baseline rows and no observed `fail` produces **no findings and exit 0**. That is the honest answer for a suite nobody has baselined yet, and a *wrong* one for a suite name that was never a suite. The two are indistinguishable at this layer, which is the ground for the argv-shape refusal §bin/diff-baseline.sh states.

**A row is a claim about one scenario, and a suite's scenarios come from its parser.** The row asserts that *that scenario* is held at the recorded status, and a scenario is whatever the suite's configured parser emits (§The evidence adapters).

- A suite carrying a single scenario has a baseline that asserts about the suite as a whole and nothing finer: **adequate** where the suite's arms are not independently meaningful, and **empty** where they are.
- Finer coverage is bought by **configuring a parser**, never by hand-authoring rows. The rows follow the parser's output, so they are recorded from a run rather than maintained against one, and under-coverage is a parser question, not a row-count one.
- A gate `gates.list` declares `# unregistered:` (gate-sdk/SPEC.md §Layout and configuration) is not a scenario of the gates suite. Its row leaves in the deregistering commit, since a row left behind is an absent `pass` scenario and reads as a new failure (§bin/diff-baseline.sh).
- Prose quoting a row copies that claim, bound through §The baseline-claims arm.

**An `exit-code` suite baselined at `fail` asserts nothing at all.** The suite has one scenario, its whole verdict. Any non-zero matches the baselined `fail`, and a zero is an unpromoted recovery, which is not a red either, so every outcome reads clean. That is no coverage wearing a verdict. The remedy is the rule above: give the suite a parser, so its arms become scenarios a `pass` row can hold.

**Which task a slug names, when more than one could.** The slug names the **standing** unpaid price the row was written to hold visible, never the topmost cause of the latest run. A transient condition that also fails the scenario (a polluted corpus, a half-applied fix, a dirty worktree) is diagnosed in its own entry and leaves this row alone, on two grounds:

- a slug that changes identity whenever someone cleans the tree is not held-constant;
- re-attributing to the transient makes every clean-tree run read as an unpromoted recovery and the next dirty one as a new failure. §bin/diff-baseline.sh splits on the row's status and cannot see which of two stacked causes produced it.

### Evidence manifest

Committed, written once per run. The file header is a `# contract: evidence-manifest v1` line, the versioned wire format the deferred hosted-attestation service consumes as its attestation payload. Each data line is `<iteration> <suite> sha256=<log-hash> pass=<n> fail=<n> ignore=<n> verdict=<clean|new-failures> <date>`, and a run supersedes that iteration's prior line for every suite it ran. `<log-hash>` is 64 lowercase hex characters, each `<n>` is decimal digits, and `<date>` is the `YYYY-MM-DD` digit shape, held as a shape and never parsed as a calendar date.

- The captured log stays uncommitted under the tmp dir. Its digest pins the log the counts came from, and `<suite>` names the suite. Neither pins the command behind that key, so a payload reader that must bind it owes a new format version, never a reinterpretation of `sha256=`.
- The iteration key scopes the line, so the boundary-truncate knob can clear the manifest at the start of the next iteration.

**The spine touches this file only after its last suite has run**, and that is contract. The `--run-validate` arm accumulates its rows in a batch file under the tmp dir and folds them in as a single write. The fold drops this iteration's prior line for each suite the run covered and re-appends the batch in configured-suite order. The write **publishes by rename**, which is what keeps the torn read §The producer-liveness lock calls unreachable out of reach. The lock's destructor claims the batch file too, so an aborted run leaves no orphan under the scratch dir.

The single fold leaves a suite free to sit anywhere in the roster even when its own precondition is a clean worktree. A spine writing per suite dirties the tree before such a suite's turn, so every full run reddens it for its roster position alone. Pinning it to the front of the roster is mitigation, not a fix: nothing asserts the position, so a second such suite re-breaks it silently.

**Line order follows the configured roster, not run history.** A repeat run rewrites this iteration's rows where they already were, so a run whose counts and date are unchanged leaves the file byte-identical. A concurrent or repeated producer therefore surfaces no diff without content behind it.

**An aborted run writes nothing.** It leaves the manifest as it found it, which is what the abort's own diagnostic claims. A partial manifest is inadmissible anyway: §check-evidence-manifest's close-entry assertion wants a clean line for every configured suite.

**A run killed from outside lands in the same place and prints nothing**: a `timeout`, a SIGTERM, a cancelled session. The fold is the only write, so a kill at the roster's last suite discards every earlier suite's clean row as a kill at its first one does. A caller therefore leaves the spine unbounded, and a deadline short enough to fire is the caller's defect.

**The header is a wire-format version marker, not a doc pointer.** gate-sdk/SPEC.md §The workflow directory rules that as one of the two payload forms a checked projection may carry, and this section is the statement that form requires. `check-evidence-manifest` owns it, asserting the first line is `# contract: <version>`. A consumer that also runs canon-kit's `check-spec-pointer` over its workflow dir whitelists **this** file there (`CANON_KIT_COMMENT_WHITELIST`), since a version marker resolves as no path. The baseline is pointer-form and needs no whitelist entry.

### The producer-liveness lock

Uncommitted, under `EVIDENCE_KIT_LOCK_FILE`. A stage stamp proves invocation and an evidence line proves a green result, but neither can say a producer is *still running*: a file read at an instant cannot carry that. The manifest's own guarantees do not reach it either. §check-evidence-manifest's assertion A asserts suite-roster completeness at a close cursor, and the spine's single fold makes a torn read unreachable. The gap is liveness alone, and the lock is the artifact that closes it.

**The record** is one line, `pid=<n> run=<key>`, where `<key>` is the evidence-line key the run-key reader yields. `<n>` is a positive decimal with no leading zero, one whitespace byte separates the fields, `<key>` carries no whitespace, and the line ends in a newline. Anything else does not parse, an empty file and an unterminated line included. Both fields have named readers and nothing else is carried. A start timestamp is refused: under a PID-liveness stale policy it has no reader.

**The grammar has a second writer class, so a change to the record shape has both callers in view.** A session that backgrounds a shell child writes a launch-time liveness record in this same one-line form. Whoever arrives after the session dies can then still ask whether the orphan is writing (delegation-kit/SPEC.md §The delegation model owns that rule). The two writers share the grammar and the predicate below, and nothing else. The atomic create-exclusive claim is this lock's alone, since a launcher recording its own child's PID has no second claimant to exclude.

**The lock is held if and only if the recorded PID is alive**, so a leaked lock self-invalidates. An **age-based TTL is refused**: a long validate run outlives any honest TTL, and a long run is the case the lock exists for. A TTL short enough to reclaim a crashed run promptly declares a healthy long run dead, restoring the false green the lock removes.

**The liveness predicate** is the one all three readers share. A pid is held when any leg says held, and the reading it must never give is a false **free**: a producer running under another uid exists but cannot be signalled. A pid that is not a positive decimal with no leading zero names no process and reads gone before any leg is asked.

- **unix.** Signal 0 is the cheap existence probe: the predicate calls `kill(2)` and reads `EPERM` as held and `ESRCH` as gone. A pid past the width of the platform's pid type reads gone unprobed.
- **Windows, the native leg, asked first.** It opens the process for limited query. A process that has not exited, or whose exit code cannot be read, is held. An open denied for access is held too, the `EPERM` reading carried over. Any other answer is gone.
- **Windows, the MSYS legs behind it.** The shell's `kill -0` builtin runs first, whose exit status conflates the two readings. `ps -p` is the fallback, where any evidence of existence means held.

The native leg exists because a Windows record names a pid in one of two namespaces: an MSYS shell's, from a Git Bash launch, or Windows's own, from a PowerShell launch. The MSYS legs read a live Windows pid as gone, which the native-Windows leg's liveness step measured. Reading `/proc` to confirm process *identity* is refused separately: it is unportable, and the OS-reach constraint (gate-sdk/SPEC.md §The adopter constraints) makes a Linux-only predicate a cost.

**A reader's own pid is never a held reading.** No reader is the producer its record names: the writer reads before it claims, and its release compares pids without the predicate. A record naming the reader's own pid therefore names a recycled id, and the producer it recorded is gone.

- **unix.** One namespace, so the predicate answers gone without a probe.
- **Windows.** The reader's pid is Windows's own, while a record may name an MSYS pid of the same number. Only the native leg is skipped, and the MSYS legs still answer.

**On a non-unix build an absent `ps` leaves the predicate unable to answer, and every reader, the writer's own claim included, refuses rather than reads free.** `ps` is reached only when the native leg and `kill -0` both read gone. Without the fallback, a process that exists but cannot be signalled is indistinguishable from one that is gone. Reading the unanswerable case as free would restore the false free the fallback closes. The refusal takes the guards' exit 2 wherever the reader has one.

**PID reuse is a named, accepted residual.** A recycled PID yields a false *held* reading, which refuses a stage entry that could have proceeded. That direction fails closed and costs one file deletion to clear, against a defect that would cost the next session an evidence file changing underneath it. Two further cases are instances of the same residual, with the same direction and clearance:

- the writer's own refusal reads the identical predicate, so a recycled PID makes `--run-validate` over-refuse;
- on Windows, an MSYS pid that numerically matches a live Windows process reads held through the native leg;
- on Windows, each MSYS leg is a spawned process holding an MSYS pid of its own for the length of its probe, so a record naming that pid reads held through that leg.

The reader's own pid is the one recycling the residual does not cover (above).

**The claim is atomic create-exclusive, never check-then-write**, and the asserted property is two-part: it succeeds for exactly one producer, *and* the record publishes whole. The idiom builds the record in a temp file under the same scratch dir, then `ln` it into place, which fails if the target exists. `mkdir` and a `set -C` redirect are atomic on the first half only: each leaves a window where the lock exists and its record does not. Because the record publishes whole, a reader never interprets a partial lock or an empty one, so an unparseable lock is corruption and fails closed rather than reading free.

The writer-side refusal (§bin/run-validate.sh) is what makes the atomicity *required*. A read-then-claim would let two producers both observe a clear lock and both claim, the second's record overwriting the first's. That is the two-producer case the refusal closes, and it defeats conditional release too: "still ours" cannot be answered from a record another producer overwrote. The claim's success *is* the check, so there is no interval between them to lose.

**The release is conditional: remove only if the lock is still ours.** The writer compares the recorded PID against its own and removes nothing on a mismatch. The property is the comparison, whatever mechanism runs it. With an unconditional release, whichever of two producers exits first deletes the survivor's lock, and a preflight then reads free with a producer still live.

Atomicity does not make the condition redundant. The case the condition closes is a lock removed by a path *other than* the reclaim below: an operator deleting an apparently-stuck lock, or a future code path. Producer A is still alive and unaware, producer B claims the freed slot, and A's unconditional release would delete B's live lock. A producer correctly identified as stale is not that case, because its release runs synchronously within its own exit and a `SIGKILL` skips the release rather than deferring it. Atomicity makes "still ours" *answerable*; conditional release acts on the answer.

**The reclaim path** a runtime artifact owes is three layers, and all three are asserted:

1. the writer's destructor, which covers every exit path the spine has;
2. the readers' PID-liveness predicate, which makes a leaked file inert;
3. the consumer's scratch-boundary wipe, which removes it.

The one honest limit is that a destructor does not run on a `SIGKILL`, and that residual is inert through the second and third layers. No close-surface declaration is owed: that obligation attaches to capture-tier members of the workflow directory, and this lock lives in the scratch tier.

**evidence-kit owns the lock at both ends; lifecycle-kit contributes only the hook it already ships.** The lock is a property of the *producer's run*, and this kit owns the producer and the scratch directory it lives under. `LIFECYCLE_KIT_ENTRY_PREFLIGHT` is a generic per-stage hook naming no evidence surface, so a second evidence-kit gate on that roster adds no cross-kit dependency. The consumer wires it as it wires the manifest gate (§lifecycle-kit integration).

A lock held by lifecycle-kit is refused. That kit would have to know this one's scratch knob, a downward dependency onto one producer kit. It would also have to model every producer's lock rather than offer one hook any producer's gate can hang from.

### bin/run-validate.sh

The codified spine, bounded by the producer-liveness lock. In order:

1. the guards, then the claim;
2. per suite, the optional pre-hook, then the suite run foreground, its log parsed and diffed per-scenario against the baseline's suite slice;
3. one evidence line per suite, whose verdict is `clean` unless the diff finds a new failure.

The lines batch under the tmp dir and reach the manifest in the single fold §Evidence manifest rules, so a suite never runs against a tree the spine has already written to. The spine never edits the baseline, never retries, and surfaces a non-zero suite exit verbatim. A log with no parseable result is a run failure, not an empty diff.

It is not a gate. It is a tool exercised end-to-end in `smoke/`, with the lock's own behavior pinned by `gate-tests/producer-lock.test.sh` and the pre-hook's ordering and abort by `gate-tests/pre-hook.test.sh`.

**It is the arm-table member `--run-validate`**, reached through `bash gate-sdk/bin/run-gates.sh --run-validate` and dispatched to `native/src/emit/run_validate.rs`. The heading keeps the tool's file name because sibling sections and code directives cite it.

**The exit contract is three-state, which is why the spelling is its own rather than `--emit-`:**

- 0, every suite clean;
- 1, a suite recorded `new-failures`: the verdict;
- 2, the guards' code: the run could not start at all, which a held or unclaimable lock joins.

The `--emit-` family maps onto `exit(0)` for a document and `exit(2)` for a failure, so it can never return 1. An `--emit-` spelling would rewrite the verdict to the refusal code, making *a suite regressed* indistinguishable from *the run could not start* on the tool whose product is that distinction, and nothing in the battery would report it. gate-sdk/SPEC.md §The bin/-tool contract states the rule, and its §The non-gate arm owns the family test and this member's placement in the class. A refusal is a start-time verdict about the world, not a result, so it takes the guards' code and not the verdict's.

**It takes no positional argument at all.** Its whole input is the resolved `EVIDENCE_KIT_*` knobs, so the argv-shape refusal and the `--` escape have no free text to bind on. The `-h`/`--help` arm lives in the front-end, as it does for every member of the class. Any argument is refused at exit 2 before the lock is claimed, so a `--help` handed to the member starts no run. That settles this one member and rules nothing for another tool taking no positionals.

**The declared knob roster is twelve names:** `EVIDENCE_KIT_SUITES`, `EVIDENCE_KIT_RUN_*`, `EVIDENCE_KIT_PARSER`, `EVIDENCE_KIT_PARSER_*`, `EVIDENCE_KIT_BASELINE_FILE`, `EVIDENCE_KIT_MANIFEST_FILE`, `EVIDENCE_KIT_SKIP_FILE`, `EVIDENCE_KIT_QUEUE_FILE`, `EVIDENCE_KIT_TMP_DIR`, `EVIDENCE_KIT_LOCK_FILE`, `EVIDENCE_KIT_RUN_ID` and `EVIDENCE_KIT_PRE_HOOK`. It is a declared roster rather than a set of hardcoded flags because every value is the kit table's, resolved against the consumer's knob file. A hardcoded flag would resolve platform defaults and silently ignore every consumer override. Two of the names are the kit's declared families (gate-sdk/SPEC.md §The non-gate arm).

**Two consumer seams word-split and spawn by design, and neither is an implementation detail:**

- A `EVIDENCE_KIT_PARSER_<suite>` value that names neither bundled adapter is a consumer command the arm spawns against the log (§Layout and configuration states why it is never short-circuited).
- `EVIDENCE_KIT_PRE_HOOK` runs once per suite, before it, with the suite as operand. A failing pre-hook aborts the run at exit 2 with no evidence appended. That ordering is contract, because it keeps a refused run from writing a line.

So the arm's spawned-program set is the consumer's roster and not the arm's: `bash`, each suite's own run command, and whatever the parser and pre-hook values name.

**`sha256sum` is outside the spawn set, and `ps` is outside it on unix.** The per-suite log's digest is computed in-crate (`native/src/sha256.rs`). Its hex encoding is byte-compatible with `sha256sum`'s first field, so a manifest line written from either digest source reads the same. `ps` stays only on a non-unix build, reached through the pid predicate's fallback leg. §The producer-liveness lock rules that leg the content of the rule rather than incidental spelling (gate-sdk/SPEC.md §The port-candidate criteria, criterion 7).

**The claim's placement is asserted, not left to the implementer.** It sits after the preflight guards and the scratch directory's creation, since a run that refuses to start must not claim. It sits before the batch file is created, so no evidence work happens outside the lock's cover.

**Release is a destructor**, `impl Drop`, conditional on the record still being ours (§The producer-liveness lock owns both properties and their grounds). A tail-line release would leak the lock on every failure path, because the arm returns from many guard and fail-closed sites besides its terminal one. That population matters most, since a crashed run is when a stale lock appears. The property is reclaim on every exit path, the one `--enter-stage`'s `impl Drop for Scratch` (`native/src/emit/enter_stage.rs`) carries for its temp files. The **batch file joins that same destructor**, so a run that dies between the last suite and the fold leaves no batch behind.

**The front-end resolves the tree, so a caller in a scratch directory owes two things.** `bin/run-gates.sh` changes to the git toplevel and refuses outside a repository, and it resolves the binary through `GATE_SDK_NATIVE_BIN`, whose default is repo-relative. A harness driving this arm against a scratch tree therefore makes that tree its own toplevel (`git init`) and crosses an **absolute** `GATE_SDK_NATIVE_BIN`. A tree that is merely a subdirectory of a repository resolves every relative knob against the *enclosing* tree while still exiting 0, the failure those two steps prevent. `gate-tests/producer-lock.test.sh`, `gate-tests/pre-hook.test.sh` and `smoke/install.sh` are the in-tree harnesses that do it.

**It refuses to start while a live lock is held**, and the refusal is the atomic claim's failure branch rather than a second mechanism: a claim that succeeds found no holder. On a failed claim it reads the existing lock, and each refusal below takes the guards' exit 2:

- A **live** PID refuses immediately, naming the blocking run key, with no reclaim attempted.
- A **dead** PID, or a lock that has vanished since the claim failed, is reclaimed by removing the lock and retrying **exactly once**. A second failure refuses rather than loops, which resolves the two-contender stale case without an unbounded retry: both contenders may remove and relink, exactly one `ln` succeeds, and the loser's re-read finds a live PID.
- A lock that does not parse refuses outright.
- On a non-unix build where `ps` is absent the lock cannot be classified, and that refuses too. A holder that cannot be read as free must not be reclaimed, which is the readers' own disposition applied to the writer.

The writer checks the lock because the entry-side reader cannot see a producer that never enters a stage. A session can run this tool without entering one, so an entry-side red alone would leave two producers able to race the manifest with every stage entry green. A lock the producer itself does not check is not a mutex.

### bin/diff-baseline.sh

The situational runtime diff, not a precommit gate: it takes captured logs as arguments, parses each, and diffs against the baseline slice per-scenario.

- A baseline `pass` scenario red-or-absent is a new failure.
- A baseline `fail` or `ignore` scenario running green is an unpromoted recovery.
- An observed `fail` with no baseline row at all is a new failure (§Baseline manifest's fail-closed rule, which keys on `fail` alone).

The split is per-scenario, so a regression and a recovery cannot net to zero. The shared diff returns non-zero the moment a new failure fires, which is also how `--run-validate` derives its verdict.

**The skip channel.** The tool reads the skip side-channel (`EVIDENCE_KIT_SKIP_FILE`, truncated per run) and demotes a self-skipped scenario from pass before the pass/fail branch, so a self-skip cannot masquerade as a pass. The kit ships no producer for the skip file: a *consumer harness* that self-skips a scenario writes it (§Layout and configuration). A skip record is a `<suite> <scenario>` line. A final line is read with or without its newline, in the skip file and in a parser's output alike.

**Each argument group is `<suite> <logfile> [<status>]`, and the status is optional only where the parser can do without it.** A log-parsing suite derives its scenarios from the log, so the pair form is complete for it. An `exit-code` suite's verdict *is* the status and appears nowhere in the log, so a group naming one without a status is **refused at exit 2** rather than run. Assuming success there would report pass for every log the tool is ever handed, clearing reds it structurally cannot observe. That is the fail-closed reading of §Baseline manifest's rule applied to the tool's own input.

The optional third token is unambiguous rather than heuristic: a suite name suffixes `EVIDENCE_KIT_RUN_<suite>`, so it is a shell identifier and can never be all digits, and a status can never be anything else. `--run-validate` does not go through this path: it holds each suite's status at the point it ran it, and passes it to the parser dispatch itself.

**It is the arm-table member `--diff-baseline`**, reached through `bash gate-sdk/bin/run-gates.sh --diff-baseline` and dispatched to `native/src/emit/diff_baseline.rs`. The heading keeps the tool's file name because the compiled diff's own directive and sibling sections cite it. The kit ships no `bin/` directory.

**It is an `Arm::Run` although it prints a report.** What excludes `--emit-` is the **exit contract**, not the document test. The emitting family collapses to `{0, 2}`. Here 1 is the verdict, *NEW failures against the baseline*, and 2 is the refusal: a bad argument shape, an unreadable log, or an `exit-code` suite named without its status. Collapsed, a real regression would be indistinguishable from *the tool was called wrong*, on a tool whose one functional caller is a CI leg that reads nothing but the status.

**The declared knob roster is five names:** `EVIDENCE_KIT_BASELINE_FILE`, `EVIDENCE_KIT_SKIP_FILE`, `EVIDENCE_KIT_TMP_DIR`, `EVIDENCE_KIT_PARSER` and the `EVIDENCE_KIT_PARSER_*` family, all declared on the same forced-family test its sibling states. **It declares no suite roster and needs none**: its suites arrive on argv, one group at a time, which is what distinguishes it from the spine.

**The argv-shape half of gate-sdk/SPEC.md §The bin/-tool contract binds here**, in three behaviours:

- a positional beginning with `-` that is not a recognized option is refused, usage on stderr, exit 2;
- `--` is the escape that admits one as free text;
- the `-h`/`--help` arm is the front-end's, where the class keeps usage.

The ground is the exit 0 §Baseline manifest's rule gives an observed set with no baseline rows and no observed `fail`. A check of arity alone absorbs a first argument of `--help` as a *suite name* and proceeds against whatever the second argument is. The tool then prints `diff-baseline: clean`, and the CI leg that reads its status goes green.

**Two orderings interact, and the naive one breaks the disambiguation.** The shape refusal is applied to a positional **before** the group parser consumes it, so `-x` in a suite slot is refused rather than resolved as a parser name. The all-digit test stays **inside** the group, because a status is not free text and a leading `-` cannot appear in it. The other order would refuse a negative-looking token where a well-formed group was meant.

### The baseline-claims arm

`--emit baseline-claims` prints each baseline row as a `measured:` oracle line, `baseline-<suite>--<scenario>`⇥`<status>`, in the file's order. Each part is lowercased with every character outside `[a-z0-9]` turned to a hyphen, so the key is slug-shaped. A consumer whose governed prose quotes a row names the arm in `CANON_KIT_MEASURED_CLAIMS_CMD`, or appends its output to its own emitter, and binds each quoting sentence with `<!-- measured: <key>=<status> -->`. canon-kit's `check-measured-claim` then reds the sentence the day the row moves (canon-kit/SPEC.md §check-measured-claim), and a claim class under `check-unmarked-claim` makes the marker owed rather than voluntary.

**A sentence recording a *retired* row cannot carry a marker**, since the oracle contradicts it or emits no such key. It takes that gate's `unmarked-claim-exempt:` valve naming the retired row. A tense predicate is refused, because tense is not decidable. Two rows mapping to one key, an unreadable file or a row short of three fields exit 2, the collision naming both rows.

**The arm reads the baseline alone and declares no canon-kit knob**, so naming it in a canon-kit command knob cannot recurse (canon-kit/SPEC.md §The shared spec adapters). A new canon-kit gate reading this kit's file was refused: it would be a second mechanism for the class `check-measured-claim` and `check-unmarked-claim` already hold, and a canon-kit member reading another kit's knob.

### check-evidence-baseline

Invariant: the held-constant baseline stays grammatical and honest. `checks/check-evidence-baseline.gate` declares it, with its rule in `native/src/gates/evidence_baseline.rs`. It asserts:

- **Shape** — every row is `<suite> <scenario> <status> [<slug> [reproduces-at=<rev>]]`.
- **Blocking-slug liveness** — every `fail`/`ignore` slug resolves to a live queue task or a permanent marker. A slug present only under the `EVIDENCE_KIT_DONE_SECTION` heading is stale and red.
- **Manifest↔disk set equality** — for every suite carrying a configured scenario glob, a baseline scenario with no matching file is red, and so is a file with no baseline line.
- **Suite coverage** — every suite in `EVIDENCE_KIT_SUITES` (§Layout and configuration) carries at least one row, and every row names a suite in it.
- **Flip causation** — a row held red that passed when the iteration opened carries `reproduces-at=<rev>` naming a commit at or before the iteration-start commit.

Argument mode `$1 $2 $3` (baseline, queue, state), each defaulting to its configured value, makes the gate fixture-capable. `gate-tests/check-evidence-baseline.test.sh` covers the liveness, coverage and flip branches beyond the one good/bad pair.

**A filed task that matches a red is not what caused it.** A regression this iteration introduced can match a red someone filed earlier, and a hold keyed on the match commits the regression as expected. So a hold on a row that passed at the iteration start (the first state-file stamp's head, lifecycle-kit/SPEC.md §The state machine) must say where the red reproduces, and that commit must be outside the iteration. The gate checks the commit's position and **not** the reproduction, so a false commit still passes. What the rule buys is that holding a regression takes a claim the diff shows, rather than a match nobody wrote down.

**The flip assertion's corpus and branches.** A flip is a `(suite, scenario)` pair that is `pass` in the baseline as committed at the iteration-start commit and `fail` or `ignore` now.

- A scenario missing from that prior baseline is never a flip: a new scenario red from the start can be a new test for an old defect, and widening to it is a separate argued change.
- A flip row with no token is red. So is any token whose `<rev>` resolves to no commit, or is neither the start commit nor one of its ancestors.
- The resolution and ancestry checks cover every token, not only flip rows. A token valid in an earlier iteration stays valid, because each iteration starts after the one before.
- The prior baseline is read from git at the start commit. A path absent there makes every row new, so there are no flips, and any other git failure is fail-closed (exit 2).

**The assertion is off wherever there is no iteration-start commit**: every case lifecycle-kit/SPEC.md §The state machine lists, a clone that cannot resolve the commit included. Resolution and ancestry go off with it, while the token's shape stays a grammar check, and the clean line says which way it went. A shallow clone is one such case, so a consumer whose CI clones shallow gets the check only from the local hook.

**`EVIDENCE_KIT_SCENARIO_GLOBS` is a keyed knob, read by key** (gate-sdk/SPEC.md §The knob file). A consumer configuring no scenario glob resolves the kit default, an empty map, so its whole battery crosses the *empty* arm and a defect in the keyed read would pass it. The non-empty arm is exercised by the coverage case in this gate's own behavioral test, which is the load-bearing evidence for it.

**A scenario glob expands unpruned, because its corpus lives where the prune set points.** A suite's scenarios are its test cases, and a kit keeps those in its tests directory, which the default prune set carries (gate-sdk/SPEC.md §Layout and configuration). Every other corpus reader bounds its `**` descent by the prune set (gate-sdk/SPEC.md §The port-candidate criteria). Doing so here would drop each scenario a `**` glob reaches through that directory, and red the set-equality arm on a tree with nothing wrong in it. The cost is the race that rule exists for: a `**` glob here stats every entry of a build directory it descends into, and a build deleting one mid-walk is exit 2. A glob whose first component is literal never descends into one.

**Suite coverage is a derived obligation, not a maintained roster.** Nothing in the gate enumerates suites: the roster is the configured one, so a suite added there acquires the obligation with no edit anywhere else, and a suite removed drops it. The fail-closed rule §Baseline manifest states still catches a rowless suite going **red**. This arm closes what that rule cannot, a suite **silently ceasing to run**, and it takes both directions because that failure has two shapes:

- A suite **renamed** under a config edit leaves the new name rowless. The forward direction, a configured suite with no row, reds it.
- A suite **dropped** from the roster leaves only its rows, which nothing runs and nothing reads. The reverse direction, a row naming no configured suite, reds it, once per suite rather than per row. The fix is deleting the rows in the commit that drops the suite.

The arm is the enforcement half of §Baseline manifest's granularity rule.

**Two branches sit at the ends of that roster, and only one of them is clean.**

- A suite set that **will not resolve**, where a baseline file exists, is **fail-closed (exit 2)**: a config the gate cannot judge is not a clean run (gate-sdk/SPEC.md §Fail-closed contract).
- A consumer configuring **no** suites at all disarms the arm, both directions, at a **declared early-out**. The reverse direction goes too because `--diff-baseline` takes its suite as an argument and reads rows without consulting the roster. A consumer running it by hand with no roster reads rows this gate has nothing to compare against.

The early-out is the shape §check-evidence-manifest's no-cursor branch takes: a gate with nothing to say says so at a named branch, never by falling through a live assertion. The clean line reports the configured suite count, which tells a reader which of the two branches a green run took.

### check-evidence-manifest

Invariant: the evidence manifest is well-formed and, where lifecycle drives the tree, coupled to the stage machine. `checks/check-evidence-manifest.gate` (`precommit`, binary-dispatched). It owns three assertions:

- **(A) close-entry** — a `close` cursor requires the full green block: a `verdict=clean` line for every configured suite, dated on or after the iteration's earliest validate stamp.
- **(B) grammar** — every line is the eight-field manifest shape and carries the current iteration. A foreign iteration line means the boundary truncation was skipped.
- **(C) stamp-coupling** — a validate stamp demands at least one evidence line. It arms only once the cursor has advanced past `validate`, since the entry stamp legitimately precedes the suites.

A red (B) suppresses A and C. An empty cursor disarms A and C entirely, so a consumer running no lifecycle keeps only the grammar floor. The cursor is empty when the queue header names no iteration or the state file yields no stage, and an absent or unreadable file of either reads that way rather than refusing. Both no-cursor shapes (§The evidence adapters) disarm at a *declared* early-out. An empty stage falling through two live assertions would read as a green gate in exactly the window where the gate has nothing to say.

Argument mode `$1 $2 $3` (manifest, queue, state): each positional reaches the rule as argv and overrides its knob there. The good/bad pair carries no `args` and reaches the rule through the three path knobs, the branch the production battery takes. `gate-tests/check-evidence-manifest.test.sh` covers the positional arm and owns the close-entry and stamp-coupling assertions.

**Moving a baseline row stales every evidence line recorded before the move.** A line's verdict is relative to the baseline live when its suite ran, and the manifest records that verdict rather than recomputing it. Take a known red deferred rather than fixed, its scenario moved from `pass` to a slug-carrying `fail`/`ignore`. The suite's recorded line still carries the old baseline's non-clean verdict, so (A) keeps refusing the close entry with "no clean evidence line" though the task and the row both landed. The deferral is three steps, in order, before the close entry is stamped:

1. **File the blocking task** the row will name, so the slug resolves to a live queue entry (§check-evidence-baseline's liveness).
2. **Move the baseline row** to `fail`/`ignore <slug>`, with `reproduces-at=<rev>` where §check-evidence-baseline's flip assertion binds it, by human commit (§Baseline manifest).
3. **Re-run the suite** with `--run-validate` and commit the fresh evidence line, which the moved baseline now diffs clean.

Both gates' help text names step 3: stopping after step 2 repeats the refusal for a reason neither landed change names.

**A pre-flight entry names this gate, never its declaration path.** `LIFECYCLE_KIT_ENTRY_PREFLIGHT` (lifecycle-kit/SPEC.md §bin/enter-stage.sh) is exec'd with **no interpreter word**, so an entry's first token rides its own exec bit. A `.gate` descriptor is a non-executable data file: an entry naming one fails the exec, the non-zero exit refuses the entry, and the stage cannot be entered at all. No gate catches it, since a pre-flight entry is an ordinary shell-out that no gate's reader enumeration covers.

So every entry names `<front-end> --only <gate>`, the battery front-end's form, with `run-gates.sh` riding its own exec bit.

- The knob is not taught to resolve a name, which would be a kit-contract change to lifecycle-kit.
- A consumer-side resolver script buys nothing beside that form: `--only` resolves the same name through the same check dirs and already carries the resolution-failure obligation below.
- The form covers the consumer's whole pre-flight roster, still-shell members included. An entry naming a path breaks the day its member ports, and re-pointing it costs nothing, because the same resolution finds the path the entry held.

**Each entry's argument rides that form through a `--` separator, mandatory on every entry.** `--enter-stage` appends `<queue>` and `<state>` to every entry's argv, and `--only` would consume each appended token as a gate name. So an entry spelled without `--` refuses, whether or not it carries an argument of its own. The grammar, its single-member bound and the sole-name resolution are gate-sdk/SPEC.md §run-gates', cited rather than restated. The appended pair lands inside the forwarded argv: `check-producer-liveness` ignores every argument past the first, and this gate reads the pair as its queue and state positionals.

**A pre-flight roster reaches a gate by *declaration*, a different claim from battery membership.** An entry-hook gate can be absent from `gates.list` by design (§check-producer-liveness rules why), so a selector resolving only registry members could not express the roster at all. That is the ground for §run-gates' sole-name widening.

**Owning the front end means owning what it does with a resolution failure it did not cause.** `gate_command` has two failure signals, told apart only by its **status** (gate-sdk/SPEC.md §lib/gate.sh). One is `return 1`, for a member resolving in no check dir. The other is an `exit` 2, already named on stderr, for a harness error such as a `.gate` member whose binary is absent. Both hand the caller an empty argv. A front end that reads the argv for emptiness reports *resolves in none of* over a gate that resolved and merely could not be built. That turns a *build the binary* problem into a *this gate does not exist* problem for every caller of the roster at once. So the obligation is the front end's, not the pre-flight entry's:

- keep the resolver's status rather than reading its argv for emptiness;
- name the resolves-in-no-check-dir case itself;
- on any other non-zero status, propagate the refusal **without adding a second sentence**, since the reason is already on stderr.

**The `--only` arm discharges it.** A member resolving nowhere is reported as *listed in … but resolves in none of*, and a member that resolved and could not be run as *dispatch harness error, exit 2*. One difference is real and not load-bearing at this caller: a bash front end calling `gate_command` exits **2** on a resolution failure, where the `--only` run exits with the aggregate's non-zero status. The pre-flight caller reads zero versus non-zero and nothing else, so both refuse the entry identically.

**The data-line helper is evidence-kit's own primitive, and the crate carries a same-named one that is a different rule.** `evidence::data_lines` filters comment and blank lines and nothing else. The crate's `stages::data_lines`, the lifecycle state file's reader, takes only the lines *below a `---` separator*. Bound here it would compile, pass a thin fixture, and silently drop every manifest line in a file with no separator. So the gate keeps evidence-kit's own reader (`native/src/evidence.rs`), the independence from lifecycle-kit §The evidence adapters states.

### check-battery-roster

`checks/check-battery-roster.gate` (`precommit`, binary-dispatched). Invariant: the configured runner doc's battery-roster block holds name-set parity with `EVIDENCE_KIT_SUITES`, both directions. The suite roster is machine-owned and the doc block is a hand copy of it. This is the `check-readme-roster` fork (gate-sdk/SPEC.md §check-readme-roster) applied to the validate battery: the register stays a human-read list carrying per-line annotation prose an emitter would have to invent, and a gate holds it honest rather than generating it.

**The marker vocabulary** follows that gate's in shape. The doc wraps its register in `<!-- battery-roster:begin -->` / `<!-- battery-roster:end -->` markers, which may carry leading indentation: the scan trims surrounding whitespace before matching, since a README nests the block inside a list item. Outside the markers nothing is scanned, so the same command appearing elsewhere in the doc for a different rhetorical job neither satisfies nor violates the gate. That is why the block, not the whole doc, is the unit.

Inside the markers a **roster line** is a line whose content begins with a command word and carries one more word:

- The command word is a bare lowercase word (`bash …`, `cargo …`) or the gate binary's door. The interpreter is not part of the grammar, so a suite whose runner is not a shell script is rosterable without widening a literal each time.
- The door is any word gate-sdk's `gate_native_bin_spelled` rule returns unchanged: `./`- or `../`-led, or rooted, a drive-rooted Windows path included. That is the shape the derived fixture-suite member takes. A bare relative path is not a door, since the rule would prefix it.
- The fenced-block delimiters and any prose fail the match by starting with neither.
- A trailing `#` annotation clause is prose the gate never reads.

A suite's **documented invocation** is `EVIDENCE_KIT_RUN_<suite>` normalized by stripping a leading `env` token and any leading `VAR=value` assignments, with or without that token. The run environment is a validate-harness concern and not something a contributor types: this repo's `gates` suite runs under gate-sdk's `GATE_SDK_VERBOSE` knob, whose value that kit's SPEC owns, to emit the per-gate tails its parser reads. What remains is compared as an exact string, whitespace-collapsed on both sides so the block's annotation alignment carries no meaning.

Two assertions over the suite set versus the roster set:

- **(A) every suite is documented** — a member of `EVIDENCE_KIT_SUITES` whose normalized invocation matches no roster line is red;
- **(B) every roster line resolves to a suite** — a roster line whose command matches no suite's normalized invocation is red, so a retired suite cannot leave a stale line telling a contributor to run a command that is not a configured suite.

Each finding names the suite (A) or the command and its line (B), and the doc. A suite with no `EVIDENCE_KIT_RUN_<suite>` configured has no documented invocation to compare and is passed over: `--run-validate` already exits 2 on it, and reporting it here would send the reader to the doc to fix a config bug.

**The overlap with `check-kit-registration` assertion B is deliberate** (gate-sdk/SPEC.md §check-kit-registration). That assertion requires every kit root with tracked `gate-tests/` files to have a runner-doc line naming `<kit>/gate-tests`. Where a consumer's config derives exactly those roots into `EVIDENCE_KIT_SUITES` through `EVIDENCE_KIT_FIXTURE_SUITES`, assertion (A) here is a superset of that arm. B is kept on a dependency direction: a gate-sdk gate may not require this kit's config. gate-sdk's enforcement-map emitter reads the suite roster where a consumer has one, but an assertion cannot, having no honest verdict when it is absent. So B is the arm that survives a gate-sdk-only adoption, the more common shape. Both sections say so, each naming the other. One omission reported from both sides is a duplicate finding, not a contradiction: the two name different sets in their output, a kit root and a suite.

Config: `EVIDENCE_KIT_RUNNER_DOC` (§Layout and configuration). The positional form `check-battery-roster [runner-doc]` overrides it against a hermetic fixture tree, the sibling meta-gates' shape. There is no empty-knob valve: a consumer keeping no runner doc opts out by not registering the gate in its `gates.list`, gate-sdk's registry opt-out shape. Each of these is a misconfiguration and fail-closed (exit 2), never a false clean:

- a configured doc that does not exist;
- a doc carrying no marker block;
- an empty suite roster;
- a non-repo cwd with no positional argument.

`gate-tests/check-battery-roster.test.sh` covers the fail-closed branches and the normalization arms beyond the one good/bad pair. It dispatches through `gate_run` rather than by script path, the invocation shape that survives a substrate move (gate-sdk/SPEC.md §lib/test-hermetic.sh).

**The suite roster and the run family are two knobs of different kinds.** `EVIDENCE_KIT_SUITES` is the roster. `EVIDENCE_KIT_RUN_*` is a **prefix family**, a resolution set the gate looks names up in and never enumerates. Enumerating it would publish `EVIDENCE_KIT_RUN_ID` as a suite, the case behind gate-sdk/SPEC.md §lib/gate.sh's rule that a prefix is a resolution set and never a roster.

### check-producer-liveness

Invariant: no stage entry while the evidence producer is still running. `checks/check-producer-liveness.gate` declares it, with its rule in `native/src/gates/producer_liveness.rs` and the two library readers it shares with the `--run-validate` arm in `native/src/evidence.rs`. It reads `EVIDENCE_KIT_LOCK_FILE`:

- **green** when the lock is absent or names a dead PID, the clean line saying so where that PID is the reader's own (§The producer-liveness lock);
- **red** when it names a live one, printing the blocking run key, so the operator can tell *wait for that run* from *reclaim a lock whose owner is gone*. On a non-unix build the red line also names the leg that answered and the reader's pid: a held reading there has three legs over two pid namespaces, and only the line can say which one a recycled id reached;
- **exit 2** when the lock cannot be read or does not parse. The claim publishes the record whole (§The producer-liveness lock), so an unparseable lock is corruption and never a free reading.

The descriptor couples only `knob:EVIDENCE_KIT_LOCK_FILE`: set mode's directory is the invoker's argument. A consumer's scratch directory is gitignored, so no commit stages a record for a trigger to see.

It is its own gate rather than a fourth assertion on `check-evidence-manifest`. That gate's charter is manifest *content* (the close-entry green block, the grammar, the stamp coupling), and liveness is a different class, which earns its own fixture pair.

Argument mode `check-producer-liveness [lock-file]` makes it fixture-capable and is how the entry hook points it at the lock (§lifecycle-kit integration). It is named as a gate, since the declaring substrate is not part of the grammar. Extra arguments are ignored, so the hook's trailing `<queue> <state>` argv passes through harmlessly.

**Its subject is a record, not this lock.** Any file in §The producer-liveness lock's `pid=<n> run=<key>` grammar is a legal subject, whoever wrote it. The second writer class is the launch-time liveness record a session places when it backgrounds a shell child (delegation-kit/SPEC.md §The delegation model). Pointed at one, this gate answers *is that producer still running* on the same exit contract, so that rule adds a reader without adding a gate.

**The pid predicate has a named caller outside the `.run` path.** `--enter-stage` classifies a linked git worktree as live or orphaned by extracting the holding process's pid from the worktree's git **lock reason** and calling this predicate on it (lifecycle-kit/SPEC.md §bin/enter-stage.sh). The record it reads is git's, not this kit's grammar, so the *gate* does not reach it. The **predicate** is shared so that how liveness is decided has one holder, and a second lifecycle surface cannot drift from this one.

- The call is in-crate and made **only when the caller's lock-reason pattern is configured**. With the pattern unset every worktree is unclassified and the predicate is never asked, so the whole of the guard sits at the call site.
- The `EPERM`-is-held reading carries over: a holder running under another uid reads **alive** rather than free.
- The accepted PID-reuse residual carries over: a recycled pid reads **live**. The worktree caller then refuses and says wait, the same fail-closed direction.

**Set mode: a directory argument quantifies that verdict over a whole record set.** Pointed at a directory, the gate reads every `*.run` file in it. That is the naming convention delegation-kit/SPEC.md §The delegation model gives the launch-time record, which makes the set a glob rather than a path a reader must be told. The per-record verdict, the exit contract and the PID predicate are the ones above and are **not re-decided**. The aggregation rule is the mode's only new decision: **exit 2 wins over red wins over green**, so one corrupt record is never averaged away by nine clean ones.

- An empty directory is green, the verdict the absent-lock case already takes.
- A directory whose records all name dead PIDs is green.
- Any live PID reds, naming **every** blocking record and run key, so a reader waits on the set rather than discovering it one entry at a time.
- Only `*.run` is read. A stray file in the same directory is not a record and its unreadability is not corruption, which is the point of giving the record a suffix.

**Set mode cannot exit 2 over an empty record set, and a caller rests on it.** Corruption is derived **per record**, so an empty glob offers no per-record verdict to aggregate and the empty directory takes green unconditionally. A caller that observes exit 2 alongside a zero record count is therefore reading something that is **not** record corruption: the gate failed to run at all, before it read a record. A caller may branch on that without re-deriving it (delegation-kit/SPEC.md §The turn-end liveness hook is the one that does).

**The single-path mode is left exactly as it is.** `EVIDENCE_KIT_LOCK_FILE` has one path and one writer, and routing it through a directory would be the generalization that breaks the case that already works. The two modes are told apart by the argument being a directory, not by a flag: the caller already knows which it holds.

**The `.run` suffix is deliberately not `.lock`.** A lock is claimed and released by one owner and its absence means *free*: `EVIDENCE_KIT_LOCK_FILE` is one and keeps its name. A launch record is a **statement of fact left behind**, and its absence means *nothing was recorded*, never *nothing is running*. Two meanings, two suffixes.

**It belongs on the entry hook and not in a `gates.list` battery.** Its subject is a transition, *is a producer in flight right now*, where every battery member's subject is tree state. A consumer whose validate roster includes its own gate battery (this repo's does: the `gates` suite *is* the battery) would have `--run-validate` invoke this gate while holding the lock it just claimed. Every validate run would red against its own record. So the kit ships the gate registered nowhere and wired at the entry, which is also the honest reading of its argument mode: it takes the lock path because its caller is a stage entry, not a whole-tree sweep.

An entry-hook caller has to be able to name a gate registered nowhere, so the battery front-end's `--only` resolves a sole name against the check dirs rather than the registry alone (gate-sdk/SPEC.md §run-gates). The port oracle's two registry arms walk `gates.list` and never select this member, so the fixture pair and this kit's smoke stand in for the dispatch proof those arms give (gate-sdk/SPEC.md §The port-candidate criteria).

**Coverage.** The fixture pair carries the two static verdicts, a dead PID and a live one. Its `bad/` case names PID 1, the one PID a checked-in fixture can assert the liveness of on every platform. The predicate's `EPERM` reading is what makes that reliable: under the builtin's exit status alone, an unprivileged run reads init as dead and the case would silently invert. `gate-tests/producer-lock.test.sh` covers everything the pair cannot hold:

- a live PID the test itself owns, the unparseable-lock exit, and the writer-side behavior;
- set mode's four verdicts and the suffix bound. A fixture dir holds exactly one `good/` and one `bad/` case, so four verdicts and an aggregation rule between them cannot be a pair. Three of the four also need a multi-record directory, and the red case needs a live PID the pair could only reach as init.

**The dead side of any such scenario is bought with a PID no platform can issue.** `2147483646` sits above every platform's `pid_max` ceiling, so a `.run` record or a lock naming it reads dead. Nothing is spawned, and there is no race against a reaped PID being recycled. Both witnesses spell that literal: `gate-tests/producer-lock.test.sh` for the record and lock scenarios, and the compiled gate's own unit assertion for the predicate.

**On a non-unix build it is a wrapper, and the requirement lives in the library, not the gate's own text.** There the pid predicate tries the `kill -0` builtin through `bash -c` and falls back to `ps -p`. So that build's registry row declares `bash`, and `ps` for the fallback leg. Both sit on the program floor and so go uncounted. `ps` is there because it is POSIX-mandated, and the program roster's parity test requires an adopter-side spawn to sit on the floor or the probe roster (gate-sdk/SPEC.md §The program roster). Floor membership waives no refusal: the absent-`ps` refusal below stands. On unix the predicate is one `kill(2)` call and the row declares nothing. The Windows native leg that runs ahead of the two is in-process calls and spawns nothing, so the declared set is unchanged. The lock reader spawns nothing on either.

**On a non-unix build an absent `ps` refuses at exit 2.** The refusal is on the fallback leg only, because a `kill -0` that answers never reaches the program. It is a deliberate divergence from the shell form the gate ported from, not parity with it. That form read the missing program as *not alive* and printed a clean line, the *clean because the program was missing* vacuity gate-sdk/SPEC.md §Fail-closed contract exists to close. **Its cost is real and bounded**: on a machine with no `ps`, a lock naming a *dead* PID refuses rather than printing clean, because `kill -0` fails with `ESRCH` and the disambiguator is gone. That is honest, since without `ps` nothing can tell `ESRCH` from `EPERM`, and `ps` is present on busybox, macOS and every Linux.

**A port can empty a dual-held helper's caller set, and this gate's two readers are the instance.** The pid predicate and the lock reader each had a shell holder beside the compiled one for as long as `--run-validate` was a shell tool calling both. A standing cross-substrate comparison held each pair equal (§The evidence adapters), and its discriminating case was **PID 1**, which `kill -0` alone reads as dead. Porting that arm (§bin/run-validate.sh) removed the one shell consumer, so the shell forms retired and the comparison retired with them: a comparison with one holder can only skip (gate-sdk/SPEC.md §The non-gate arm). A dual holding is a statement about a caller set at a moment, not a permanent classification.

## lifecycle-kit integration

Integration is two generic knobs on lifecycle-kit's side of the seam, each naming no evidence surface in that kit. The coupling lives entirely in the consumer's config and `check-evidence-manifest`'s optional assertions.

`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` lists the files `--enter-stage` truncates back to their `# contract:` header at the iteration boundary, as it resets the state file. A consumer sets it to the evidence manifest, so a new iteration starts with a manifest carrying only its contract header. That is what makes assertion (B)'s foreign-iteration test able to catch a skipped truncation.

`LIFECYCLE_KIT_ENTRY_PREFLIGHT` carries **both** of this kit's entry-side gates, at the stage keys the paragraphs below name. `--enter-stage` runs each matching entry against the candidate temp state file, the prospective stamp appended, and the live queue. It appends that `<queue> <state>` argv to whatever the entry names, and a non-zero exit refuses the entry with nothing written.

**`check-evidence-manifest` is wired at `close=`**, its command naming the *gate* rather than a path, and the manifest after it: `close=<name-resolving front end> check-evidence-manifest <manifest>`. §check-evidence-manifest owns the front end and the trap an entry naming a declaration path falls into. Assertion (A)'s close-entry green-block check then fires *before* the stamp is written, so missing evidence is a refusal at the entry, pointing at `--run-validate`. Unwired, it is a self-referential deadlock at pre-commit: the `gates` suite that would produce the evidence re-runs this same red gate against the already-stamped cursor. The wiring is belt-and-braces behind the validate skill's `--run-validate` step, not a replacement for it. For a consumer that wires it, assertion (A)'s enforcement point moves one step earlier, from commit to entry.

**`check-producer-liveness` is wired at every stage key in set mode**, each entry pointed at the consumer's scratch **directory**: `<stage>=<front end> check-producer-liveness <scratch-dir>`, the same name-resolving form. The subject is any recorded producer, and any stage can leave one. A stage that ended its turn on a backgrounded `gh run watch` leaves a record no lock-pointed entry names. Two keys carry the one-producer cases: `close=`, a lead dispatching close into a still-running producer, and `validate=`, a second validate batch entering while a first batch's `--run-validate` is live. The full roster costs one gate invocation per stage entry against a directory that is usually empty. Which keys a consumer wires is config, not asserted kit behavior.

**The lock-pointed entries stay beside the set entries rather than being replaced by them.** Set mode reads `*.run` and `EVIDENCE_KIT_LOCK_FILE` keeps `.lock` (§check-producer-liveness), so the directory pass **cannot see `--run-validate`'s own lock**. Replacing the lock entries would trade the coverage set mode adds for the coverage they already had. A consumer whose producer publishes a lock keeps that lock's entry and adds the directory.

**What this wiring is honest about: it detects, and it detects late.** An orphan is found at the *next* entry, so the turns between the orphaning and that entry are already spent. If the orphan's stage is the iteration's last, no entry follows it at all. The preventing half is a `PreToolUse` rule over the harm rather than the act (guard-kit/SPEC.md §The generic ruleset, guard-kit rule `git_mutation_under_producer`); this entry is the backstop behind it.

**The read-only `--simulate` mode inherits this gate with no extra wiring.** It runs every matching preflight entry, so a lead gating an expensive dispatch with a simulated entry sees a live producer. It does **not** make a dispatch rule redundant. A simulated entry remains an instantaneous read, so a producer that starts a second later is still unseen, and the gate covers only producers that claim the lock or leave a record. A lead dispatching on artifact state is still dispatching on artifact state, with one more artifact. A dispatch rule governs what the lead waits *for*; this gate narrows what survives being wrong about it.

The validate stage records evidence on a commit later than the entry stamp (assertion C's arming): the stamp proves invocation at entry, the evidence line proves the green result once the suites have run.

## Producers and consumers

- **Evidence line** — produced by `--run-validate` per suite verdict; consumed by `check-evidence-manifest` (A/B/C), by close-stage entry via that gate, and, forward, by the hosted-attestation payload. Every field has a reader there: iteration (A/C scoping), suite + verdict + counts (A's green-block test), sha256 (audit pinning of the producing log), date (A's stamp-ordering floor).
- **Suite roster** (`EVIDENCE_KIT_SUITES` + `EVIDENCE_KIT_RUN_<suite>`) — produced by consumer config, with the fixture suites and their run members derived by the kit's table. Consumed by `--run-validate` (what to run), by `check-evidence-manifest` (A's green block), by `check-battery-roster` (the doc-parity compare), and by gate-sdk's enforcement-map emitter, which drops its section on an empty roster so evidence-kit stays optional. Every one of them resolves it through the crate's knob table rather than parsing the file, so a derived suite is visible to all of them with no second parse to keep in step (gate-sdk/SPEC.md §The knob file).
- **Producer-liveness lock** — produced by `--run-validate` at the claim point, which sits on the ordinary path: the validate stage runs it, and it is the only writer of the manifest. Its enabling config carries a default in the kit's table, so it resolves in every deployed configuration rather than only under a test harness. Consumed by `check-producer-liveness` through the entry-preflight hook and, with no extra wiring, by that hook's read-only simulate mode. Both fields have named readers at named transitions:
  - `pid` at three: the gate at the stage-entry transition, `--run-validate` at run start on a failed claim (the refusal), and its destructor at release, compared against its own pid to answer *is this still ours*;
  - `run key` at the two transitions where a refusal line is composed, whose reader is the operator standing at that refusal choosing between waiting and reclaiming.

  Reclaimed by the three layers §The producer-liveness lock lists.
- **Baseline line** — produced by human commits (initial seed, promotions); consumed by `--diff-baseline` (the per-scenario diff), `check-evidence-baseline` (grammar, liveness, coverage, flip causation) and `--emit baseline-claims` (the oracle lines below). Its `reproduces-at=<rev>` token is produced by a human commit too. That is a validate session's, after reproducing a red at or before the iteration-start commit, or the closing stage's, landing the row a `used` valve line owes with the commit the valve reason names. The flip assertion alone consumes it: `--diff-baseline`, `--run-validate` and `--emit baseline-claims` read fields one to three and ignore it.
- **Baseline oracle line** — produced by `--emit baseline-claims` per run of the command that names it; consumed by canon-kit's `check-measured-claim` arms A and B through a consumer's `CANON_KIT_MEASURED_CLAIMS_CMD`, which read the key and the status at each marker. The blocking slug and `reproduces-at=` are not printed, since no prose claim reads them.
- **Skip record** — produced by a consumer harness that self-skips a scenario; consumed by `--diff-baseline`. An absent file means no skips.
- **Per-suite parser override** — produced by consumer config (`EVIDENCE_KIT_PARSER_<suite>`); read by the per-suite parser resolution, and so by the parser dispatch and `--run-validate`'s effective-parser diagnostic, reaching both spine tools.
- **Scenario line** — produced by the resolved parser from the captured log; consumed by the per-scenario diff and by the evidence line's `pass=`/`fail=`/`ignore=` counts. Per-gate granularity changes the line's population, not its shape, so those readers are unchanged.
- **Truncation** — produced by `--enter-stage` at the scope boundary reading `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`; consumed by assertion (B)'s foreign-iteration test, which is what makes skipping it visible.
