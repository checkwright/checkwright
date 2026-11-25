# SPEC amendment: exit-tool-error

Pairs `exit-code-tool-error`. Today a caller of the gate binary tells a refused request from a tool that could not finish only by parsing stderr, because both exit 2. This amendment splits that status. Exit 2 keeps the request refused as given, and a new exit 3 carries a tool or input error. A `--json` form names the error's kind, and `--run` gains a three-way verdict. The split reaches every member and every non-hook arm, the arms git calls among them. Three classes stay as they are, each for its own ground: a harness hook arm keeps 2 for both, because failing closed outweighs the split there and its only caller, the harness, blocks on 2 alone; a member whose grammar names a verdict word on 2 keeps its word; and an arm that relays a child's status passes it through (delta 2).

**What the tree does today**, each bullet with the probe that settled it:

- Every member and arm spells its error status as a literal: about 450 `return 2;` and `exit(2)` sites, with no exit type in the crate (`git grep -cE 'return 2;|exit[(]2[)]' -- native/src`).
- `Arm::Emit` members return `Result<String, String>`, and `main` maps every `Err` to exit 2. About 1,064 `Err(` sites sit behind that one collapse (`grep -n 'Arm::Emit' -A12 native/src/main.rs`).
- `--run` exits 1 on any member that did not pass, whatever its status, and 2 on its own refusals (gate-sdk/SPEC.md §run-gates, its opening paragraph).
- The front ends exit 2 on every path where no binary runs (`grep -n ARM_UNAVAILABLE_STATUS gate-sdk/bin/run-gates.sh gate-sdk/bin/run-gates.ps1`).
- The readers that branch on a 2 and would misread a 3 are in the survey record's spec block on exit-status readers (`.workflow/survey-record.md`). The one with a silent wrong outcome is the turn-end liveness hook: it refuses on its reader's exit 2 and allows on any other status, so a 3 would allow (`grep -n 'Exit(2)' -A3 native/src/hook/stop_liveness.rs`).

## What changes

### (1) The exit contract {design-bearing} {user-facing: the entry's deliverable — a refused request told from a tool error by status}

**Not yet applied**: gate-sdk/SPEC.md §Output contract's sentence "Exit codes: **0** clean, **1** violation, **2** harness/usage error." is replaced by:

> Exit codes: **0** clean, **1** violation, **2** the request refused as given, **3** a tool or input error. The line between 2 and 3 is who could fix it. Exit 2 is the caller's: an argument vector that does not parse, a knob or config value that is malformed or required and unset, a policy refusal such as a held lock, or an operand naming something absent, such as an unknown gate, a revision that does not resolve, or a run outside a git repository. Exit 3 is the tool's own: a file or store it reads cannot be read or does not parse, a write fails, a program it runs is absent or fails to start, or git fails on a repository it must read. A hook member is the exception, and keeps 2 for both (§The hook contract).

The paragraph's later sentence "A fourth exit code would make every caller branching on the status learn a new value" is rephrased to "A not-applicable status would make every caller branching on the status learn a new value", since 3 is now a status and the ground still refuses one for not applicable.

- **Why the caller-fixable line.** An outside engine retries a tool error and repairs its own request on a refusal, so the line it needs is who acts next, and the adopter's own rule draws it there. A malformed config value is the caller's because the caller wrote it. An unreadable file is the tool's even when the caller named it, because the request was well formed.
- **Why a hook member keeps 2.** The harness blocks a tool call on 2 alone (§The hook contract), so a 3 from a guard would let the call through. The split buys a hook nothing, because its one reader cannot use it. **A hook member here is a harness hook.** The arms git calls (`--git-hook`, `--measure-commit` and the forwarding launcher) split like any non-hook arm, since git blocks on any non-zero status and a 3 stays fail-closed there. A chain that forwards the pre-push status unchanged keeps blocking on either.

**Not yet applied**: gate-sdk/SPEC.md §Fail-closed contract's code comment "non-zero status -> exit 2, never a false clean" becomes "non-zero status -> exit 3, never a false clean". The paragraph's "a program that could not be run is an exit-2 refusal" becomes "a program that could not be run is an exit-3 tool error". `lib/gate.sh`'s `fail_closed` exits 3, and `templates/check-skeleton.sh` spells both statuses with the cut beside them.

Every other statement of a 2 in a governed doc is reclassified by this cut. The doc roster is delta 5's probe. A statement naming both kinds of cause, such as `--install-hooks`' list, splits its list across the two statuses.

### (2) The typed fault {design-bearing}

**Not yet applied**: gate-sdk/SPEC.md §Output contract gains, after the exit codes:

> **A status other than 0 and 1 is spelled in one module.** `native/src/exit.rs` holds the error kinds and their statuses. A refusal is a `Fault` carrying its kind and its message, and a fault may also carry one line for stdout, which `main` prints before it exits; the `peer-file` collision (delta 5) is the one site that sets it. `usage` is an argument vector that does not parse, and `refused` is a well-formed request declined; both exit 2. `input` is something read that cannot be read or does not parse, and `tool` is a failed write, spawn or git call; both exit 3. An `Arm::Emit` member returns `Result<String, Fault>`, and `main` exits with the fault's status. No conversion from a bare string to a `Fault` exists, so every error site names its kind and the compiler finds the site that does not. A member or `Arm::Run` arm returns the module's named statuses, never a literal. A crate unit test reds a literal status 2 or 3 in any shipped module but `exit.rs`, with test scope dropped by derivation as the spawn-funnel test drops it (§Fail-closed contract). A member whose own grammar gives 2 a meaning beside refusal names it as a constant in `exit.rs`, with a pointer to the section that owns the meaning, and the roster test exempts it by that name. These verdict-word members keep their word grammar at 2: `--usage-verdict`'s `STALE` and `--model-verdict`'s `UNKNOWN`, each a fail-soft verdict; `--foreign-run` and `--foreign-resume`'s `FAILED` and `--foreign-run --budget`'s `STALE` and `OFF`; `FAIL(env)` on `--upgrade-smoke`, `--run-demo` and `--projection-witness`, and on the installer's `demo` and `prove`; and the "no comparison made" of `--run-config-variant` and the "nothing written" of `--queue`. Each takes 3 only where the front end already adds it, for an absent binary. **An arm that relays a child's status passes it through and is exempt from the split:** `--scratch-run`, `--with-foreign-shells` and the lesson-sink relay. A 3 from one of them may be the child's own, so a caller reads the echo, as it reads a 2 today.

- **Why no string conversion.** A default kind would compile every existing site unchanged, and every tool error behind it would keep exiting 2. The missing conversion turns the sweep into the compiler's worklist.
- **Why the verdict-word members keep their grammar.** A caller of those arms reads the word on the verdict line, and the status only to tell it apart. No outside caller reads these statuses: the adopter that asked for the split reads `--run`, `--verify-gate-run` and `--verdict-lookup` alone. A word such as `FAILED` or `FAIL(env)` spans both sides of the cut, so splitting its status would move the word's readers (the lead and host-protocol templates, the delegation README) with no reader to gain. The front end's 3 for an absent binary is the one place a status already differs, and it is a tool error by the cut.
- **Why a relay is exempt.** The arm's status is the child's, so no cut of its own causes can classify it. Only the arm's own refusals before the child starts follow the cut.
- **Why a roster test beside the type.** A gate or an `Arm::Run` arm returns an integer, which no type holds. The test is the worklist for those sites, on the routing test's precedent.

**Not yet applied**: gate-sdk/SPEC.md §The non-gate arm's paragraph **The family follows the exit contract, never the output** is rephrased so that `Arm::Emit` "maps every success to 0 and every error to its fault's status, 2 or 3". The test that admits a member to that family becomes "a member whose exit grammar is 0, 2 and 3 and no other status". In the forced-out list, each member's 2 is split by the cut, except the verdict-word members and the relays above. `--usage-verdict` keeps its 2 as `STALE`, a verdict and not an error, as `--model-verdict` keeps `UNKNOWN`. The admitted `--emit-stage-rules` reads "a missing `<stage>` is exit 2 and an unreadable doctrine file exit 3".

### (3) The runner's three-way verdict {design-bearing} {user-facing: the entry's deliverable — an errored battery told from a failing one}

**Not yet applied**: gate-sdk/SPEC.md §run-gates' sentence "Exit 0 only when every member passed, and 1 on any member failure, whatever code the member returned." is replaced by:

> The run's verdict is `pass`, `fail` or `error`, and it exits 0, 1 or 3. It is `fail` where any member exited 1. Otherwise it is `error` where any member exited with another non-zero status, resolved nowhere, or could not be started. Otherwise it is `pass`. A run that also fails is `fail`, because a violation found is a finding whatever else errored. The run's own refusals stay 2, and its own tool errors are 3.

- **Why error is its own verdict.** A member that crashed has not judged the tree. Read as a failure, its run would be stored as a judged verdict and served for that tree until the gate set changed. As `error` it stores nothing, so the next call reruns (the verdict store, which `verdict-lookup` adds to gate-sdk/SPEC.md §run-gates). A hook or CI reader blocks on any non-zero status, so neither sees a change.
- **Why fail outranks error.** A violation one member printed is true whatever another member did, and a caller fixing it first loses nothing.

The tail `(dispatch harness error, exit 2)` becomes `(dispatch harness error, exit 3)`, in the tail list and wherever a test asserts it. A member that could not be started is a tool error. §run-gates' `GATE_SDK_SARIF_FILE` paragraph sentence "The statuses stay 0, 1 and 2, because a hook or CI reader blocks on any non-zero: not applicable rides a result kind, never a fourth code." becomes "The statuses are the run's verdict (above). A hook or CI reader blocks on any non-zero status, and not applicable rides a result kind, never a status." A SARIF path the arm cannot write is exit 3, and a `GATE_SDK_SARIF_RESULTS` value outside its set stays exit 2. A SARIF result keeps its kind: an errored member is a `fail` result, as it is today. The log is unchanged and its contract takes no bump.

### (4) The error document {design-bearing} {user-facing: the entry's deliverable — the error's kind in the machine form}

**Not yet applied**: gate-sdk/SPEC.md §The non-gate arm's sentence "On the arm's refusal status it prints no document." is replaced by:

> Where the arm itself refuses or fails, at exit 2 or 3, it prints the **error document**, `{"contract":"<its version>","error":"<kind>"}`, its kind one of `usage`, `refused`, `input` and `tool` (§Output contract), and the diagnostic goes to stderr as without the option. A caller reads the status, then the document. The kind names the class of the request's fault, and the message stays on stderr, outside every contract. An arm whose document carries a verdict of its own prints that document where it judged, whatever the status: `--run --json` at exit 3 for a run whose verdict is `error` prints the run document, which the arm's own failures never do, and a caller tells the two by the `error` key against the `verdict` key.

- **Why only under `--json`.** The adopter reads the status and the `--json` document, never stderr. A kind on stderr or on a hook's envelope would be minted for no reader, so neither carries one.

**Not yet applied**: in evidence-kit/SPEC.md §verify-gate-run, the sentence "On exit 2 the arm prints no document, and the refusal goes to stderr as it does without the option, so a caller reads the status before it parses." is replaced by "On exit 2 or 3 the arm prints the error document (gate-sdk/SPEC.md §The non-gate arm)." The `contract` row becomes `1.1.0`, and a golden `error.json` joins the two, an unreadable Statement file at exit 3. The arm's own statuses split by the cut: an argument vector or an empty `EVIDENCE_KIT_STATEMENT_CMD` with no positional is 2, and a Statement that cannot be read or does not parse is 3. A Statement that fails check 1's shape, or names a `contract` major this binary does not know, is an `input` error at 3, since it is not a Statement this reader can judge. A `HEAD` naming no commit stays a `refused` at 2, since the caller fixes it by committing, as the cut treats a run outside a repository. A Statement that passes check 1 and fails a later check stays 1.

### (5) The readers, front ends and templates {mechanical}

Each reader that branches on a 2 follows the cut.

- **The turn-end liveness hook.** `native/src/hook/stop_liveness.rs` reads its reader's exit 3 as it reads 2, splitting by record count into `corrupt` and `unresolved`, and both refuse. **Not yet applied**: delegation-kit/SPEC.md §The turn-end liveness hook's "reader exit 2" becomes "reader exit 2 or 3". `check-producer-liveness` exits 3 for a record it cannot read or parse, and 2 for an argument it refuses.
- **The fixture runner.** `--run-gate-tests` reads a case's exit 2 or 3 as a harness error, never as a logic failure. Its own grammar becomes 0 clean, 1 a logic failure, 2 an argument it refuses, and 3 a case or fixture it could not run. **Not yet applied** at gate-sdk/SPEC.md §run-gate-tests' three-valued exit, which is rephrased four-valued.
- **The consumer smoke.** `--run-consumer-smoke` keeps reading a probed gate's 2 as a surface the consumer configures as absent, which is a refusal under the cut. A 3 is a smoke failure.
- **The front ends.** `gate-sdk/bin/run-gates.sh` and `run-gates.ps1` carry two statuses where they carry one: `ARM_REFUSED_STATUS`, 2, for the run outside a git repository, and `ARM_UNAVAILABLE_STATUS`, 3, for an absent or unexecutable binary and a repository git refuses or skips. Both are 0 for a fail-open arm. **Not yet applied** at gate-sdk/SPEC.md §run-gates' paragraphs on the front end's refusals and on `exec_arm`'s variant. `--run-front-end-parity`'s corpus expectations move with them. This is the change only the Windows legs run, which the entry's mid-iteration push carries.
- **The model verdict.** `--model-verdict`'s statuses are unchanged: `UNKNOWN` is a fail-soft verdict at 2, declared in `exit.rs` beside `STALE` (delta 2). Reached through the front end, it can now exit 3 on an absent binary. So the two instruction surfaces branching on it, `.claude/agents/stage-session.md` and `lifecycle-kit/templates/consult.md`, read "on exit 2 or 3". The close template's roadmap step reads a horizon the consumer has not configured, which stays 2, so it is unchanged.
- **The validate arm.** `--run-validate` exits 2 for a refused invocation and 3 for a run that could not start. **Not yet applied** at evidence-kit/SPEC.md §bin/run-validate.sh and at gate-sdk/SPEC.md §The non-gate arm's forced-out list.
- **The config-variant rehearsal.** `--run-config-variant` runs `--run` twice and accepts only a status of 0 or 1, aborting the comparison on any other (`native/src/emit/run_config_variant.rs`, `battery_in`). Under delta 3 a battery whose failing members exit other than 1 now exits 3, and a rehearsed variant that makes members fail at their first knob read is such a battery. The arm reads 0, 1 and 3 as a battery that ran, and reads 2, the run's refusal, as no comparison made. **Not yet applied** at gate-sdk/SPEC.md §run-config-variant, whose rows for such members still read `red`.
- **The queue post-check.** `--queue` maps any non-zero `--run` status to its own 1, the move written under a battery that is not green (`native/src/emit/queue_verbs.rs`). That reading stands for a 3, so the arm is unchanged, and queue-kit/SPEC.md §The queue verbs says "not green" where it says "red".
- **The installer smoke.** `native/src/emit/installer_smoke/` asserts verb refusals at 2 (`moves.rs`, `personal.rs`, `profiles.rs`). Each follows its producer by the cut: an unparseable block read at `uninstall` is 3, a usage error stays 2.
- **The instruction surfaces beside the model verdict.** `delegation-kit/templates/agent-execution.md`, `lifecycle-kit/templates/lead.md`, `lifecycle-kit/templates/host-protocol.md` and `delegation-kit/README.md` carry the exit numbers or verdict words of `--usage-verdict` and `--foreign-run`. They change only where a ruling on those members' statuses moves one. delegation-kit/SPEC.md §usage-verdict's front-end paragraph, where an absent binary reads as `STALE` at 2, gains "or 3" as the model verdict's does.
- **The peer-channel collision.** `--emit peer-file` refusing a filing whose id exists already exits 2 and prints the existing id on stdout, as a filing prints its own. The caller reads the id and treats the ask as filed. The message on stderr says the same. The collision carries no error kind of its own, since `peer-file` has no `--json` form and the status and the one stdout line are its whole machine form. **Not yet applied** at lifecycle-kit/SPEC.md §The peer request channel, in the `peer-file` bullet, as the rule a caller reads:
  > A collision is exit 2 with exactly one line on stdout, the existing request's id. Every other exit-2 path of `peer-file`, and every other refusal or error of the arm, writes nothing to stdout. A caller reading the status and the stdout therefore tells a collision from every other refusal, and treats the id as an ask already filed. A second refusal that printed to stdout would break that reading, and the route then is a `--json` form naming the kind `collision`, never a second stdout shape.

  This kind adds no push, since the arm runs on every leg. The rule is held by one unit test and one fixture. The unit test is `a_collision_is_exit_2_with_the_id_alone_on_stdout` in `native/src/emit/peer_channel.rs`'s test module: it files one request at a fixed filing time, files the same ask again at that time, and asserts through the arm's dispatch the status 2, a stdout of exactly the id and a newline, and an empty stdout on each of the arm's other refusals and errors (an unbound channel, a malformed body, a peer outside its grammar, and a write that fails at exit 3). The fixture is a new leg in `lifecycle-kit/gate-tests/peer-channel.test.sh`, which today merges stderr into stdout in its `arm` helper; the leg captures the two streams apart and asserts the same two facts on the binary, refiling in a short bounded loop until a pair lands in one second, since two filings straddling a second boundary are two requests, so the contract is held at the process boundary as the adopter reads it.
- **The installer.** The bootstraps and the adopter verbs keep 1 for an adopter-actionable refusal, a dirty tree, a missing identity or a downgrade among them (installer/SPEC.md §The verbs, its *Exit status* bullet). The rest split by the cut: an unsupported platform, a malformed argument and an unwritable target the caller named are 2, and a download, a digest mismatch and a failed write are 3. `doctor` and `diff` read "the question could not be answered" as 3 where a file or manifest is unreadable, and a usage error no longer shares that status. `demo` and `prove` keep `FAIL(env)` at 2 (delta 2). The ground is that a person runs the installer and no adopter branches on its status, so moving the 1 class to 2 is churn with no reader.
- **The arms git calls.** `--git-hook`, `--measure-commit` and the forwarding launcher split by the cut, and each statement of their 2 in gate-sdk/SPEC.md §git-hook and §measure-commit follows its cause. Git blocks on any non-zero status, so a refusal at 2 and a tool error at 3 both refuse the commit. **Not yet applied** at those two sections. **Not yet applied** at each installer/SPEC.md statement the delta 5 probe finds. The CI action reads any non-zero status as a failure and is unchanged.
- **CI.** `.github/workflows/gates.yml` asserts that read a status follow the cut. Its stale comment on the unavailable status is corrected.

Roster probe for the doc statements, over the tracked tree: `git grep -nE 'exit 2|exits 2|at exit 2|exit status 2|exit .2.' -- '*.md' ':!docs/**' ':!TASK-QUEUE.md'`. Each hit is classified by delta 1's cut. A hit naming a hook member's 2 stands. For the 114 test asserts across 46 files, the probe is `git grep -lE '(-eq|==|!=) *2|exit 2' -- '*.test.sh' '*smoke*'`, and each assert follows its producer.

## Producers and consumers

- **Exit 3.** Producers: every member and non-hook arm, through `exit.rs`, on a delta 1 tool cause. The front ends on a binary that is absent or will not run. Reachable in every deployed configuration, since a missing file or a failed spawn needs no setting. Consumers: the outside engine, which retries on 3 *(inferred: the adopter's reader is out of tree, its need adopter-stated)*. In-tree: the liveness hook, the fixture runner, the consumer smoke and the instruction surfaces of delta 5.
- **The error document.** Producer: every `--json` arm where it refuses or fails itself, at 2 or 3: `--verify-gate-run`, and the `--run --json` and `--verdict-lookup` arms that `orchestrator-arm-json` and `verdict-lookup` add to gate-sdk/SPEC.md §run-gates. Consumer: the outside engine reading `error` *(inferred, as above)*, and each arm's golden test.
- **The collision's stdout id.** Producer: `--emit peer-file`, where its id exists already, in every deployed configuration that binds a peer channel. Consumer: the adopter's peer-filing caller, which reads the status and the one line and takes the ask as filed (adopter-stated, with the stdout rule above as its condition). In tree: the unit test and the fixture leg of delta 5.
- **The run's error verdict.** Producer: the `--run` arm. Consumers: the verdict store, which records no `error` run, `--run --json`'s `verdict`, and every reader of the run's status. A hook and CI block on any non-zero status and are unchanged.
- **Point 2, roster-holding readers.** `--emit parse-gates-log` keys on the verdict line's prefixes, which are unchanged, and holds the tails in its own test, which moves with delta 3. `check-front-end-fail-open` reads only `FAIL_OPEN_ARMS`, which is unchanged. `--run-front-end-parity`'s corpus holds each exit path's status, and moves with delta 5.
- **Point 4, every field's reader.** `error` is the only new key, and the outside engine reads it. The four kind words are its value set. The in-tree reader of `usage` against `refused` is each golden test. Their split is minted for the adopter's named need (the entry's ask), with the inference marked above.
- **Point 5, narrowing.** A reader treating 2 as "could not run" narrows, because a tool error now arrives as 3. Every in-tree reader the survey found is listed in delta 5, and each one's red or refuse condition moves to "2 or 3" where it meant could-not-run. An out-of-tree caller branching on 2 misses 3. The operator direction weighed that cost against an in-house adopter set, and the release note's Behavior changes section names it.
- **Point 6, the corpus obligation.** Every literal 2 site, every `Err` behind the emit collapse, and every doc statement of a 2 is obliged. Each one's value is the delta 1 cut applied to its cause: a caller-fixable cause is 2, the tool's own is 3, and a hook member's is 2. The sites are enumerable but too many to name here. The compiler (delta 2) and the roster test name every crate site at build. The doc probe (delta 5) names every statement.

## Existing sections updated

Roster probe: `git grep -nE 'exit 2|exits 2|return 2;|exit[(]2[)]|ARM_UNAVAILABLE_STATUS|dispatch harness error|harness/usage error' -- . ':!docs/**' ':!TASK-QUEUE.md'`, over the tracked tree.

- `gate-sdk/SPEC.md`: §Output contract (deltas 1 and 2), §Fail-closed contract (delta 1), §The non-gate arm (deltas 2 and 4), §run-gates (deltas 3 and 5), §run-gate-tests (delta 5), and every other statement of a 2 the probe finds (delta 1).
- `evidence-kit/SPEC.md`: §verify-gate-run (delta 4), §bin/run-validate.sh (delta 5), §import-gate-run (delta 1), §check-producer-liveness (delta 5), and the `--only` pre-flight paragraph carrying the dispatch tail (delta 3).
- `delegation-kit/SPEC.md`: §The turn-end liveness hook, §model-verdict and every other statement of a 2 (deltas 1 and 5).
- `lifecycle-kit/SPEC.md`: §The peer request channel and every other statement of a 2 (deltas 1 and 5).
- `installer/SPEC.md`: each refusal statement the probe finds (delta 5).
- `canon-kit/SPEC.md`: each statement of a 2 the probe finds (delta 1).
- `guard-kit/SPEC.md`: each statement of a 2 the probe finds (delta 1). A hook member's 2 stands.
- `context-kit/SPEC.md`: each statement of a 2 the probe finds (delta 1). A hook member's 2 stands.
- `queue-kit/SPEC.md`: each statement of a 2 the probe finds (delta 1).
- `drift-kit/SPEC.md`: each statement of a 2 the probe finds (delta 1).
- `site-kit/SPEC.md`: each statement of a 2 the probe finds (delta 1).
- `doctrine-kit/SPEC.md`: each statement of a 2 the probe finds (delta 1).
- Every other tracked doc the probe finds, the kit READMEs and templates among them: each statement of a 2 (delta 1).
- `native/src/exit.rs`: new, the kinds, the statuses, the declared verdict constants and the roster test (delta 2).
- `native/src/main.rs`: the emit collapse (delta 2).
- `native/src/runner.rs`: the three-way verdict, the dispatch tails and the usage text (delta 3).
- `native/src/emit/parse_gates_log.rs`: its tail test (delta 3).
- `native/src/emit/verify_gate_run.rs`: the error document and the split statuses (delta 4).
- `native/src/hook/stop_liveness.rs`: the reader's 3 refuses (delta 5).
- `native/src/gates/producer_liveness.rs`: the split statuses (delta 5).
- `native/src/emit/run_gate_tests.rs`: the case's 3 and its own grammar (delta 5).
- `native/src/emit/run_validate.rs`: the split statuses (delta 5).
- `native/src/hook/model_verdict.rs`: `UNKNOWN` as a declared verdict constant (delta 2).
- `native/src/emit/peer_channel.rs`: the collision's id on stdout, and the unit test that pins the rule (delta 5).
- `lifecycle-kit/gate-tests/peer-channel.test.sh`: the stream-separated collision leg (delta 5).
- `native/tests/exec_dispatch.rs`: the asserted dispatch tail (delta 3).
- `native/src`, every other module the roster test or the compiler names: delta 2.
- `docs/contracts/verify-gate-run/v1/error.json`: new, and the two existing goldens' `contract` (delta 4).
- `gate-sdk/bin/run-gates.sh`: the two statuses (delta 5).
- `gate-sdk/bin/run-gates.ps1`: the two statuses, the twin (delta 5).
- `gate-sdk/lib/gate.sh`: `fail_closed` (delta 1).
- `gate-sdk/templates/check-skeleton.sh`: the skeleton's statuses (delta 1).
- `gate-sdk/gate-tests/run-dispatch-streams.test.sh`: the asserted dispatch tail (delta 3).
- Every other test or smoke file the delta 5 probe finds: the asserted statuses (delta 5).
- `.github/workflows/gates.yml`: its status asserts and comment (delta 5).
- `.claude/agents/stage-session.md`: the exit branch (delta 5).
- `lifecycle-kit/templates/consult.md`: the exit branch (delta 5).
- `native/src/emit/run_config_variant.rs` and gate-sdk/SPEC.md §run-config-variant: the battery's accepted statuses and the variant rows (delta 5).
- `queue-kit/SPEC.md` §The queue verbs: "red" becomes "not green" for the post-check's 1 (delta 5).
- `native/src/emit/installer_smoke/`: the asserted statuses (delta 5).
- `delegation-kit/SPEC.md` §usage-verdict: the front end's absent-binary status, "or 3" (delta 5).
- `docs/gate-sdk/SPEC.md`: the generated mirror (all deltas).
- `docs/evidence-kit/SPEC.md`: the generated mirror (all deltas).
- `docs/llms-full.txt`, and every other generated SPEC mirror: regenerated (all deltas).

## Retired spellings

- `harness/usage error` — the 2 it named is split into a refusal and a tool error (delta 1).
- `dispatch harness error, exit 2` — a member that could not be started exits 3 (delta 3).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for exit 3, the error document and the error verdict.
- [ ] **Instruction surfaces: instruction only**: the two template and agent branches change their status only.
- [ ] **Merged with no information lost**: each addition rephrases the canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted**: this file is removed on merge (`ls SPEC-*.md`).
- [ ] **Removals propagated**: `## Retired spellings` holds.
- [ ] **Swept by the oracle**: the crate builds with no string-to-fault conversion, the roster test is green, and the delta 5 doc probe's hits are each classified.
- [ ] **Readers moved**: the liveness hook refuses on its reader's 3, and `--run-gate-tests` reads a case's 3 as a harness error, each under a unit test.
- [ ] **Pushed once mid-iteration**: the Windows legs run `--run-front-end-parity` green on the moved statuses.
- [ ] **Entry moved**: `exit-code-tool-error` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed**: cross-component gaps discovered during the work are filed as debt tasks.
