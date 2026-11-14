# SPEC amendment: resume-wait

Pairs `liveness-hook-false-refusals`. `subagent-stop-liveness` refuses a dispatched session's turn end while a launch record names a live producer or the harness's task view shows a running `shell` task of the session's own (§The turn-end liveness hook). On a harness that gives a session a new turn when its own background task completes, that turn end is the session's supported wait, and the refusal is false. This amendment makes the waiting rule a consumer-selectable set: today's refusal is the default, and a `resume-wait` member allows the turn end that a completion path will resume. A hand-back the session declares on its launch record is still refused while the producer is live.

**What the tree does today**, read from `native/src/hook/stop_liveness.rs` and probed at spec on Linux with the built binary:

- **A `red` reading refuses whatever the session holds.** `fire` sets `decision=refuse` on `red`, `corrupt` and `unresolved`, then refuses again on `shell_task_running`, so no own task can turn a `red` into an allow. On a resuming harness, a session that backgrounds a recorded producer and a wait on it, then ends its turn, is refused at every firing until the producer exits. The adopter's log-only run read 14 such refusals in 24 firings and no true positive.
- **An unreadable transcript counts every running `shell` element as own.** `shell_task_running` returns `true` when the child transcript is absent or unreadable, and treats an element whose `id` is absent or empty as own. A sibling's task can then refuse the session. That is read from the code, not observed.
- **The record grammar has no room for a marker.** A record `pid=1 run=k handoff=/x/h` makes `check-producer-liveness <dir>` exit 2, "carries no readable 'pid=<n> run=<key>' record". The hook would read that as `corrupt` and refuse. guard-kit's `parse_record` (`native/src/guard/host.rs`) skips the record, so rule `git_mutation_under_producer` would not block on it. The two-field record `pid=1 run=k` reads `red`, exit 1.
- **Latency.** Twenty firings over an empty record set took 29 to 32 ms each, p50 about 31 ms. Nearly all of that is the reader spawn. The adopter measured p50 76 ms and p95 182 ms on its own host against a 100 ms budget. No delta here adds a spawn: the marker read is one `stat` per marked record, and the transcript is read only on the arm whose decision it can change.

## What changes

### (1) The wait set and its knob {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling), against scope's recommendation to keep the rule: the waiting rule becomes a policy-as-choice set, `refuse` its default, a `resume-wait` member, no `off` member}

A new scalar knob, `DELEGATION_KIT_STOP_WAIT`, selects the hook's waiting policy: `refuse` or `resume-wait`. The default is `refuse`, which is today's behavior unchanged. The table validator refuses any other value at exit 2, as it refuses a `DELEGATION_KIT_REQUIRE_TIER` that is neither `on` nor `off`. At the hook, a malformed value reaches `hook::decline` through the `walk::knob_scalar` error, as a malformed `DELEGATION_KIT_STOP_LOG` does today: the turn end goes unguarded and the operator notice says so.

- **The set has no `off` member.** Each member refuses a live producer the session does not wait for. Turning the refusal off remains unwiring the hook (delta 8).
- **`resume-wait` is a claim the consumer makes about its harness, and the kit cannot check it.** It is correct only where the harness gives a dispatched session a new turn on its own background task's completion notification. Binding it under a harness that ends the session at its turn end loses the producer's result, which is the hazard the waiting rule exists against. The binding is a tracked write to the consumer's knob file, the consumer's act, as wiring the hook is.
- **Rows.** `native/src/knobs/delegation_kit.rs` gains `Row::scalar("DELEGATION_KIT_STOP_WAIT", "refuse")` beside `DELEGATION_KIT_STOP_LOG`, and its validator gains the two-value check. `native/src/hook/mod.rs` adds the knob to `subagent-stop-liveness`'s declared knob slice in `HOOKS`.
- **This repo's binding.** `scripts/delegation-config.knobs` binds `DELEGATION_KIT_STOP_WAIT = refuse` explicitly, behind its `spec:` line. Policy-as-choice: each consumer binds its own.

§Layout and configuration gains the knob's bullet beside `DELEGATION_KIT_STOP_LOG`, carrying the set, the default, the validation, and the harness claim above. **Not yet applied.**

### (2) The resume-wait decision {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): a turn end is allowed only while an own running `shell` task is live, which is the completion path; a `red` reading with no own task refuses; a live record whose `handoff=` file exists and is non-empty refuses}

Under `resume-wait` the decision is taken as follows. `refuse` keeps today's decision exactly.

| reading | the session holds | decision under `resume-wait` |
| --- | --- | --- |
| `red` | an own running `shell` task, and no record's `handoff=` names an existing non-empty file | allow: the own task is the completion path |
| `red` | no own running `shell` task | **refuse**: a detached producer nothing will resume the session for |
| `red` | any record's `handoff=` names an existing non-empty file | **refuse**: a declared hand-back over a live producer |
| `corrupt` | — | **refuse**, as under `refuse` |
| `unresolved` | — | **refuse once**, as under `refuse`; allows when the loop guard is true |
| `green`, `unavailable`, `unstarted`, `error` | anything | allow |

- **The task view refuses nothing under `resume-wait`.** Its role there is to *enable* an allow on `red`: an own running `shell` task is the completion path the harness resumes the session on. A turn end with an own task running and a non-refusing verdict allows. That is the turn end the adopter's run refused falsely.
- **`red` keeps ignoring the loop guard**, on its existing ground: the condition ends when the producer ends.
- **A harness helper allows on every arm**, unchanged.
- **The marker is read only on `red`.** On any other reading no record names a live producer, so a declared hand-back has nothing to refuse. The read is set-level: `red` says some record names a live producer, and the marker says some record declares a hand-back. The hook decides no record's liveness itself (delta 3 gives the ground).
- **The transcript is read only on `red`**, and only when the view holds a running `shell` element. No other arm's decision depends on ownership under `resume-wait`.

The hook section's contract line, its opening subject sentence and its table gain the member's arms: the table as a second decision column, or as a second table below it. Which form is calibration at merge. **Not yet applied.**

### (3) The `handoff=` field and the one parser {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): the marker is a launch-record field `handoff=<path>`, read at stop time as a file that exists and is non-empty; the grammar widens across evidence-kit, guard-kit and delegation-kit with one shared parser in the crate's evidence module}

**The grammar.** evidence-kit/SPEC.md §The producer-liveness lock's record becomes `pid=<n> run=<key>[ handoff=<path>]`, one line with fields in that fixed order. One whitespace byte separates each pair of fields, and the line ends in a newline. `<path>` is absolute by the platform's own test (`Path::is_absolute`), non-empty, UTF-8, and free of whitespace. A relative or empty path, a path that is not UTF-8, a repeated or reordered field, or any field other than these three does not parse, and the record reads corrupt as before. §176's "Both fields have named readers and nothing else is carried" is re-phrased to name the third field's reader, the turn-end hook under `resume-wait`. The start-timestamp refusal stays.

- **Writers.** The lock writer (`run_validate.rs`) and the wait-primitive probe (`wait_probe.rs`) never write the field. A session following `templates/agent-execution.md` writes the two-field form, and the template is unchanged: the field is written by a consumer's own work engine, and the template's rule still binds an agent. The run-validate lock read ignores the field. A lock carrying one still parses, because the grammar is one grammar.
- **The one parser.** `evidence::lock_read` parses the widened grammar, and `LockRead::Held` gains `handoff: Option<String>`. guard-kit's `parse_record` is deleted. `GuardHost::live_runs` reads each record through `evidence::lock_read`, skipping `Absent` and `Corrupt` as it skips an unparseable line today. Without the move, the guard would skip a handoff-bearing record whose producer is live. That fails open on rule `git_mutation_under_producer`. The hook's marker read calls the same `lock_read` over the `*.run` listing it already takes for `runs`.
- **The marker test.** A record's marker is present when its `handoff=` path names an existing regular file of non-zero length, following links. The test is one `metadata` call per marked record. A directory, a missing path, an empty file and a read error are not a marker.
- **Liveness stays the reader's.** The hook parses records for the marker and decides no producer's liveness in process. The reader is overridable (`DELEGATION_KIT_LIVENESS_CMD`). An in-process PID test beside it would be a second liveness predicate that an override's reader could contradict. That is the split §315's one-code-path ground refuses. The reuse paragraph (§309) is re-phrased: the hook copies no grammar and calls the grammar's one parser in process for the marker, and the reading stays the spawned reader's.
- **guard-kit.** Rule `background_no_record` detects a record by its redirect target (`.run` under the scratch dir), not by its contents, so it is unchanged. The canonical grant spelling (`echo "pid=$! run=<key>" > …`, guard-kit/SPEC.md §The generic ruleset) stays exact. A launch spelling a `handoff=` field is a spelling the grant does not hold, so it costs a permission decision. The engine that writes the field is not an agent under this guard. guard-kit/SPEC.md §The generic ruleset's kit-owned-artifact bullet and §The rule roster's `git_mutation_under_producer` cite the widened grammar.
- **Messages.** The reader's "carries no readable 'pid=<n> run=<key>' record" (`producer_liveness.rs`, both set and argument modes) and the lock refusal in `run_validate.rs` name the widened grammar as `pid=<n> run=<key>[ handoff=<path>]`.

delegation-kit/SPEC.md's "two-field" and "one-line `pid=<n> run=<key>`" phrasings (§47, §738) are re-phrased to the widened grammar, the optional field named once. §53's filename convention is unchanged. §Attribution's "carries no writer identity" stays true: `handoff=` names a file and no writer. **Not yet applied.**

### (4) The unreadable transcript, reviewed per member {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): under `resume-wait` an unreadable transcript shows no task as own, so the turn end refuses; under `refuse` the count-all branch stays, with the review recorded}

- **Under `resume-wait`, an absent or unreadable transcript shows no task as own.** On `red` the session holds no completion path the hook can see, and the turn end refuses. An element whose `id` is absent, not a string, or empty also counts as *not* own. Under this member, ownership is what *licenses* an allow, so the test errs toward the refusal. That is the reverse of `refuse`, where ownership *licenses* a refusal.
- **Under `refuse`, the count-all branch stays.** The review is recorded in the hook section's ownership paragraph (§279, §283): an unreadable transcript counts every running `shell` element as own. Dropping that would fail open on the session's own moved call, which is the case the task view exists for. The false refusal the branch can produce, a sibling's task refusing the session, needs the harness to fire with the transcript unreadable. No firing has shown that, and the refusal still names its lawful exits. **The branch was weighed and kept, not overlooked.**

§305's unreadable-payload paragraph gains the member split: an unreadable transcript leaves the condition unscoped under `refuse`, and under `resume-wait` it withholds the allow. **Not yet applied.**

### (5) The member's honest limits {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): two honest limits stated in the SPEC, that a sibling's marked record refuses this session and that an undeclared hand-back passes}

A new paragraph group in the hook section states each limit:

- **A sibling's marked record refuses this session.** The record set is shared (*The shared scratch dir is the subject*), and the marker read is set-level. A record another session marked as a hand-back refuses every session's turn end while any producer is live.
- **An undeclared hand-back passes.** No contracted payload key tells a final hand-back from a stop the harness will resume (§295-297). A session that hands back with its own task running and declares no marker is allowed. Its dispatcher then holds a possibly interim report while the producer writes on. That is the unobservable-producer hazard (§The delegation model), and it is the member's price.
- **Above `records=1`, the marked record may not be the live one.** `red` says some record names a live producer, and the marker says some record is marked. With two or more records, a marked record whose producer exited, beside a live unmarked one, refuses. Deleting the exited producer's record clears it. That is the ordinary retraction (*The ordinary escape is not the operator's*).
- **An own task the harness never completes holds the session, with no refusal to reach it.** A wait with no producer that will make its condition true is an own running `shell` task. Under `resume-wait` it allows the turn end, and the harness holds the session until someone stops the task. That is the reverse case lifecycle-kit's lead template inspects. Under `refuse` the hook refused that turn end and named stopping the task as its exit. guard-kit rule `wait_no_producer` still refuses the wait at its launch, for the class it reads.
- **The completion path is uncontracted harness behavior.** It rests on the resumed-harness reading of §295-297: one child, one harness build. A harness revision that stopped resuming a session would turn every `resume-wait` allow into a lost result. No gate here detects that, and only a fresh firing catches it.
- **An unrelated own task licenses the turn end.** An own running `shell` task that is not a wait on the live producer still allows on `red`. The session resumes when that task completes, and its next turn end reads the set again. So a detached producer is bounded by the session's next turn end, not by the turn end that left it.

**Not yet applied.**

### (6) The log's `wait=` field and the marker refusal arm {design-bearing}

**The log line gains `wait=<refuse|resume-wait>`**, after `runs` and before `decision`, beside the fields the decision reads. Its reader is the close-stage triage. One append-only log spans a knob change, as it spans the commit landing enforcement (the `decision` field's ground). Read without `wait=`, a `verdict=red decision=allow` row from a session's own firing would be unexplained. Under `refuse` it marks a helper alone. Under `resume-wait` it marks a helper or a completion-path allow, and `wait=` is how the triage tells which rule produced it. **Honest limit:** under `resume-wait` the row does not separate a helper from a completion-path allow. The helper's marks are unlogged under the key-names-only ruling, and the triage reads a helper's 30-second cadence instead. The open field set (§347) takes the field without any reader changing. The real-reader lane's field assertions gain it.

**The two `resume-wait` refusals on `red` share a row and a remedy**, so they get no field of their own: `wait=resume-wait verdict=red decision=refuse`. Both are a live producer the session may not leave. **The refusal message gains a marker arm.** On `red` under `resume-wait`:

- **with a marker:** it states that a launch record declares a hand-back while a producer is live. It names the marked records, and says the hand-back waits for the producer.
- **without one:** it states that no running task of the session's own will resume it. It names both lawful exits: wait for the producer in-turn on its own artifact, or background a wait on it so that its completion resumes the session.

Both arms also name the existing exit: delete a record whose producer has exited. Neither names deleting or emptying the handoff file. That would retract the session's own declaration to pass the hook, which is the honour-system escape the hook exists against.

The `red` and `corrupt` messages under `refuse` are unchanged. The three-way branch paragraph (§359) gains the marker arm. **Not yet applied.**

### (7) The delegation-model and hook paragraphs re-phrased {design-bearing}

Each is re-phrased in place, not appended to. Each **Not yet applied.**

- **§The delegation model, the waiting rule (§21-27).** A dispatched role still waits in-turn by default. A consumer whose harness gives a session a new turn on its own task's completion may bind `resume-wait`, which makes that turn end the session's wait. The rule's invariant, that the return value is the contract, is unchanged. Under that member, the declared hand-back is the return value, and the hook holds it until the producer is gone.
- **The unconditional paragraph (§235).** Under `resume-wait` the refusal is no longer unconditional on `red`. This narrowing stands on its own ground: a completion path the harness owns resumes the session. It is not a safety margin. It coincides with the capped variant in part, since both leave the task view refusing nothing, and the paragraph says so as its own rule requires.
- **The task-view paragraph (§279).** Under `resume-wait` the arm enables an allow and refuses nothing. Under `refuse` it is unchanged.
- **The delivered-report paragraphs (§295-297).** Under `resume-wait`, the refusal on a declared hand-back reaches a session whose hand-back is its turn end. A harness that holds such a session after the hand-back is the harness the member is written for, and the marker is how the session tells the hook which stop is final.
- **§Attribution.** The grammar gains a field and still carries no writer identity. That paragraph's ruling stands.

### (8) The no-knob line {design-bearing}

§365's "**There is no knob, and unwiring the hook is the valve.**" no longer holds: `DELEGATION_KIT_STOP_WAIT` is a knob. What the line protects stays true: **no knob turns the refusal off**, and no per-dispatch override exists. The line is re-phrased to say that. The act depends on whether `hook-observe-mode` lands in the same build batch. Each **Not yet applied.**

- **With it in the batch:** the line names both consumer-side levers, the wait set and the `--observe` registration argv. It states that neither is a per-dispatch override, and that unwiring stays the valve that removes the hook. That entry's amendment owns the observe half.
- **Without it:** the line names the wait set alone, re-phrased as above. That entry's later landing adds the observe half.

delegation-kit/README.md step 6's "There is no knob: unwiring is the valve" is re-phrased the same way, and names `DELEGATION_KIT_STOP_WAIT` and its default.

### (9) Cases, the release declaration and the mirrors {mechanical}

Each **Not yet applied.**

- **The stub lane** (`stop_liveness.rs` tests): one case per row of delta 2's table. Also: an unreadable transcript under each member; an element with an empty `id` under each member; a marker naming a missing file, an empty file and a directory, each not a marker; a helper allowing under `resume-wait` on `red`; `refuse` unchanged on every existing case with the knob explicit; and the `wait=` field on every line.
- **The real-reader lane** (`scripts/gate-tests/subagent-stop-reader.test.sh`): a record carrying `handoff=` naming a non-empty file, read by the real reader as `red` and not `corrupt`, refused under `resume-wait`. The same record with its file emptied, beside an own running task in the payload, allowed.
- **The grammar's cases.** `evidence.rs` and `producer_liveness.rs` gain parse cases: a third field accepted, and a relative path, an empty path, an unknown field, a repeated field, a reordered field and two separators refused. guard-kit's `a_record_line_parses_only_in_its_one_grammar` moves to the shared parser's cases. `git-mutation-under-producer.test.sh` gains a handoff-bearing live record that blocks.
- **The knob's cases.** The validator refuses `off`, an empty value and a mixed-case `Refuse`. The hook declines on a malformed value with the operator notice.
- **The release declaration** (`.workflow/release-declarations.md`, `## Behavior changes`): `subagent-stop-liveness` takes `DELEGATION_KIT_STOP_WAIT=resume-wait`, which allows a turn end whose own background task the harness will resume, and the launch record takes an optional `handoff=<path>` field.
- **The mirrors**: `docs/delegation-kit/SPEC.md`, `docs/evidence-kit/SPEC.md` and `docs/guard-kit/SPEC.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **The knob** (delta 1). Producer: the consumer's knob file, or the default. Consumers: the hook's `run`, through `walk::knob_scalar`; the table validator; the `HOOKS` slice, which declares the read.
- **The `handoff=` field** (delta 3). Producer: a consumer's work engine, which writes it into a launch record. No kit writer writes it. Consumers: `evidence::lock_read`, and through it `check-producer-liveness` (set and argument modes), the run-validate lock read, guard-kit's `live_runs` and the hook's marker read. The stage-entry preflight reads through `check-producer-liveness` and meets no change.
- **The marker test** (deltas 2 and 3). Producer: the handoff file the engine writes. Consumer: the hook under `resume-wait`, on `red`.
- **The `wait=` field** (delta 6). Producer: `fire`. Consumer: the close-stage triage.
- **The refusal's marker arm** (delta 6). Producer: `refusal`. Consumer: the refused session, through stop-hook feedback.

The grammar's readers and writers were found by `grep -rn "run=" --include=*.sh --include=*.ps1 --include=*.rs . | grep "pid="`, and the parsers by `grep -rn "parse_lock_line\|parse_record\|lock_read" native/src`. No shell or PowerShell lib carries a copy (`grep -rln "pid=" */lib */bin` is empty). The knob's reads were found by `grep -rn "DELEGATION_KIT_STOP_LOG"`.

## Existing sections updated

- `delegation-kit/SPEC.md` §The delegation model (deltas 3 and 7), §The turn-end liveness hook (deltas 2 to 8), §Attribution was weighed and is not available (delta 7), §bin/wait-probe's record-shape paragraph (delta 3), §Layout and configuration (delta 1).
- `delegation-kit/README.md` step 6 (delta 8).
- `evidence-kit/SPEC.md` §The producer-liveness lock, §check-producer-liveness (delta 3).
- `guard-kit/SPEC.md` §The generic ruleset's kit-owned-artifact bullet and §The rule roster's `git_mutation_under_producer` (delta 3).
- `native/src/guard/host.rs`, `parse_record` and its test, replaced by the shared parser (deltas 3 and 9).
- `native/src/evidence.rs`, `native/src/gates/producer_liveness.rs`, `native/src/emit/run_validate.rs` (delta 3); `native/src/hook/stop_liveness.rs` (deltas 2, 4 and 6); `native/src/knobs/delegation_kit.rs`, `native/src/hook/mod.rs` (delta 1); and their unit tests (delta 9).
- `scripts/delegation-config.knobs` (delta 1); `scripts/gate-tests/subagent-stop-reader.test.sh`, `guard-kit/gate-tests/git-mutation-under-producer.test.sh` (delta 9).
- `.workflow/release-declarations.md` (delta 9).
- `docs/delegation-kit/SPEC.md`, `docs/evidence-kit/SPEC.md`, `docs/guard-kit/SPEC.md`, the generated mirrors (all deltas).

## Retired spellings

- `parse_record` — guard-kit's private copy of the record grammar in `native/src/guard/host.rs`, retired by delta 3 in favour of `evidence::lock_read`.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and the fixture suites of delegation-kit, evidence-kit and guard-kit green. No remote run observes this entry, so its Done move lands before the drain stage, in the build batch that lands it.
