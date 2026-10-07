# SPEC amendment: producerless-wait

A dispatched session backgrounded a wait whose condition nothing could make true, as a device to hold its turn open, and the guard granted it. Run at authoring, with no launch record under the scratch dir: the shell guard answers `permissionDecision: allow`, reason `bounded in-turn wait`, to a harness-backgrounded `until [ -f <a path nothing writes> ]; do sleep 30; done`. The session's turn end was then refused on that running task, by the turn-end hook's task-view arm, and the session sat idle until the loop was killed.

The waiting rule's clause *a wait owes a producer that will make its condition true* (delegation-kit/templates/agent-execution.md) is a request with no oracle. This amendment builds one at the launch, where the wait passes a `PreToolUse` chokepoint, and corrects the turn-end hook's stated ground for the arm that held the session.

## What changes

### (1) A guard-kit rule named `wait_no_producer` blocks a backgrounded file-test wait while no record names a live producer {design-bearing} {user-facing: operator direction 2026-10-07 on the queue entry — a mechanism where the clause failed, the choice between the hook's arm and a launch guard left to this stage}

A new row of the crate's rule table and a new item of guard-kit/SPEC.md §The rule roster, titled **Backgrounded wait with no live producer**. **Not yet applied.**

**The predicate.** The call is **blocked** when every clause holds, and the rule declines otherwise:

- **(a) The call is backgrounded**, in either of rule `background_no_record`'s two forms, the harness field or a statement-ending shell `&`.
- **(b) Its first statement is a wait loop**: a `while`/`until` loop with exactly one balanced `do … done` span, found by the ruleset's one shell-keyword walk, the shape rule `background_no_record`'s exemption (2) exempts from the record.
- **(c) The loop's condition is shell tests and nothing else**: one or more of `[ … ]`, `[[ … ]]` and `test`, each read past a leading `!`, joined by `&&` or `||`. Polarity is not read, so `until [ -f <m> ]`, `while [ ! -f <m> ]` and `until [ ! -d <d> ]` are one class.
- **(d) No `*.run` record under a `GUARD_KIT_SCRATCH_DIRS` member names a live PID**, on the per-record read rule `git_mutation_under_producer` makes: evidence-kit's grammar and PID predicate, a record that does not parse declining for itself.

**Why the class is decidable where the clause is not.** Whether *this* producer will write *this* path is a claim about the future that no chokepoint reads. What the launch does show is narrower and sufficient for the attested shape. A shell test reads local filesystem state, so whatever makes it true is a local writer. The waiting rule gives a session three local writers: a shell producer, which carries a launch record from its launch; an `Agent` child, awaited by its completion notification and never by a path on disk; and the session's caller, to whom a question is a turn end and never a wait. Only the first is lawfully awaited by a file condition, and it is visible in the record set. A backgrounded file-test wait over a record set naming no live PID therefore has no lawful producer, whatever its path.

**It blocks, where the bias is toward passing.** The departure rests on an attested firing, the ground rules `git_mutation_under_producer` and `background_no_record` take. The wrong block costs one re-issue: a session whose producer is real launches it with its record first, in the spelling rule `background_no_record` names, and the wait then passes. The wrong pass cost a held session for as long as nobody read the process table.

**The corrective names the finding and the three lawful exits**, as guard-kit/SPEC.md §The shell guard requires of every block message: launch the producer first with its liveness record, then wait; await an `Agent` child by its completion notification, which needs no wait loop; ask the caller and end the turn. It says that a wait launched to hold a turn open is the finding itself.

**Declines**, on the shared directions, and on each of these:

- a foreground call, which the foreground ceiling bounds, and which becomes the turn-end hook's subject where the harness moves it to the background;
- a condition carrying any segment that is not a shell test. A `kill -0 <pid>` condition names its producer and ends when that PID stops answering. A command condition (`grep -q`, a network client) may read a writer no local record declares, and stays rule `bounded_wait`'s to grant or withhold;
- more than one loop span, or an unbalanced `do`/`done`, as rule `bounded_wait`'s clause (a) declines.

**Placement.** Immediately after rule `background_no_record` and ahead of the auto-allow band. It must precede rule `bounded_wait`, whose arm (A) grants exactly this shape, and it follows rule `background_no_record` so a launch that owes a record meets that block first. Shells `bash` alone: its test reads bash's loop grammar, the ground rule `bare_sleep` is `bash` alone on. A PowerShell wait loop is unreached, stated as residue.

**The declaration clause** names the views the predicate reads: the views rule `background_no_record` declares for clauses (a) and (b), and rule `bounded_wait`'s condition views for (c). Build reads them off those two rules' own declarations rather than off this sentence.

**The seam.** The rule reads the harness's background field, bash's loop grammar and evidence-kit's record grammar: harness behavior, shell-substrate behavior and an artifact whose grammar a kit owns, the three classes §The generic ruleset admits. It mints no knob. A roster of marker paths or of producer names is consumer vocabulary and is refused, on rule `background_no_record`'s ground.

**Honest limits, stated in the item.**

- **A live record proves a producer exists, never that it writes this wait's condition.** A sibling session's producer in the shared scratch dir satisfies clause (d) for an unsatisfiable wait. The rule narrows the class to *no producer at all*, which is the attested one.
- **A producer that dies without writing the marker leaves the wait unsatisfiable after its launch.** No `PreToolUse` call sees that. The sanctioned wait on a producer is its PID's liveness, which ends with the producer, and the template says so; a marker wait beside a live record still passes here.
- **A condition spelled with a command is unreached**, as the declines state.

**Tests.** A firing and a non-firing case on each clause. Clause (d) varies the record set, which a decision-table row cannot, so its pair lives where rule `git_mutation_under_producer`'s firing arm does, the bespoke lane (`guard-kit/gate-tests/git-mutation-under-producer.test.sh`, which owns a live process and a scratch tree): a backgrounded file-test wait over an empty record set blocks, and the same call beside a record naming a live PID passes on to rule `bounded_wait`'s grant. The clauses a row can vary sit in the decision tables: a foreground file-test wait and a backgrounded `kill -0` wait are unchanged.

**Inferred, cannot run before build:** no existing decision-table row changes its expected decision under the rule — at authoring, `guard-tests/background-cases.tsv` holds one backgrounded wait row and its condition is a `grep -q`, and `guard-tests/cases.tsv`'s one file-test wait row is a foreground call; whether a table run resolves `GUARD_KIT_SCRATCH_DIRS` to a directory holding ambient live records, which would make any record-dependent row unstable there, is read off the runner once the rule exists.

### (2) The turn-end hook's task-view arm states what its condition does not resolve, and its refusal names the second exit {mechanical} {user-facing: operator direction 2026-10-07 on the queue entry — reporting nothing blocking while a running shell holds a stage session is unacceptable}

delegation-kit/SPEC.md §The turn-end liveness hook rules the task-view condition *unconditional … because it resolves when the task ends*. That ground holds for a producer and for a wait that has one. A wait nothing satisfies never ends, so the refusal it draws never lifts. **Not yet applied.**

- **The decision is unchanged.** The arm still refuses on every running `shell` element the session launched, and still ignores `stop_hook_active`. Bounding it by that field is the *refusing once* repair the section already refuses, on its own ground: a session would end its turn on its own moved call at the second try. Classing the task by its `command` is refused too, a fourth beside the section's three narrower repairs: the hook reads `type` and `status` and nothing else of the view (§What `background_tasks` carries), and a wait loop behind a script name would read as a producer.
- **The ground is restated**: the condition resolves when the task ends, *which a producer does and a wait with no producer does not*. rule `wait_no_producer` refuses that wait at its launch, for the class delta 1 names; the arm's residue is the wait that rule does not reach.
- **The refusal's stderr gains the second lawful exit**, beside awaiting the task's completion notification: *where the task is this session's own wait and nothing will make its condition true, stop it; an observer wrote nothing, so stopping it loses nothing.* The member's stub lane asserts the added wording as it asserts the rest.
- **An honest limit is added, as one reading and not a bound**: a refusal drawn by a turn end taken after the session had already delivered its report was followed by no model turn, and the session stayed held until the task was killed. §Attribution was weighed and is not available records the refusal reaching a session as stop-hook feedback; that delivery is not established for a turn end that follows a report-delivery call. The limit claims the one reading and no mechanism: whether the harness withholds the feedback there is unmeasured.

### (3) The waiting rule's template clause names its oracle, and forbids the hold device {mechanical}

delegation-kit/templates/agent-execution.md, the **Background + notification, never poll** bullet: the sentence *A wait owes a producer that will make its condition true.* is rewritten to carry two instructions and no grounds. **Not yet applied.**

- A wait owes a producer that will make its condition true, and a backgrounded wait on a file condition is refused at its launch while no launch record names a live producer (rule `wait_no_producer`).
- Never launch a wait to hold a turn open.

The grounds stay in delegation-kit/SPEC.md §Operative residency, whose *No gate is owed over the act* paragraph gains one sentence: a wait's **launch** passes a `PreToolUse` chokepoint, so the clause has an oracle for the class rule `wait_no_producer` reads, and stays a request outside it.

**The carriers propagate by grep, at the merge.** The clause is one of the bullet's restated imperatives (§Operative residency), so each carrier takes the rewrite in its own voice. At authoring, `git grep -l 'owes a producer'` over the tracked tree, the generated `docs/` mirror and the queue apart, names the template, `.claude/agents/stage-session.md`, `.claude/agents/audit-sweep.md` and one `.workflow/release-declarations.md` row; build re-runs the grep rather than trusting this list.

**Deltas 2 and 3 land in delta 1's batch or after it, and cite the rule qualified by its kit there.** `check-guard-registration` assertion D reads the kit-qualified citation outside guard-kit's tree and reds one naming a rule the roster lacks, so this amendment writes the bare form until delta 1 puts the name on the roster. The text deltas 2 and 3 land on delegation-kit's surfaces and on the carriers takes the qualified form, which that gate then resolves.

## Producers and consumers

- **The block (delta 1).** *Producer:* the shell guard's rule walk on every `PreToolUse` call the consumer's `Bash` matcher hands it; the enabling config is the guard's registration, which this repo carries. *Consumer:* the calling session, through the block message. *Roster-holding readers of the rule roster,* by `git grep -l 'pgrep_self_match'` over the tracked tree: guard-kit/SPEC.md's roster and `native/src/guard/rules/mod.rs`' rule table, which `check-guard-registration` holds in lockstep by name; the decision tables under `guard-kit/guard-tests/`; `docs/guard-kit/SPEC.md`, the generated mirror; and `.workflow/release-declarations.md`, which takes a Behavior changes row for a new block.
- **The record-set read (delta 1, clause (d)).** *Producer of the records:* a session's canonical recorded launch. *Reader:* this rule, through the read rule `git_mutation_under_producer` already makes, so no second parse of evidence-kit's grammar is minted.
- **The refusal text (delta 2).** *Producer:* the `subagent-stop-liveness` member's task-held refusal. *Consumer:* the refused session, on stderr; and the member's stub lane, which asserts the wording.
- **The template clause (delta 3).** *Producer:* a dispatching session's `/agent-execution` load. *Consumers:* that session, and each carrier's bound role at the tier it always loads.
- **No corpus is narrowed and no enumerable corpus is obliged member by member**, so causal-completeness points 5 and 6 bind nothing here. Delta 1 widens a block: the one reader whose red condition it could move is a decision-table row expecting `allow` for a backgrounded file-test wait, and the marker on delta 1 carries what authoring could and could not read of that.

## Existing sections updated

- `guard-kit/SPEC.md` §The rule roster — the new roster item, in dispatch order after rule `background_no_record` (delta 1); rule `background_no_record`'s exemption (2), whose exempted wait now meets this rule before rule `bounded_wait`'s clauses (delta 1); rule `bounded_wait`'s placement paragraph, which names the rules it sits after (delta 1).
- `guard-kit/SPEC.md` §Testing — the record-dependent pair's home beside rule `git_mutation_under_producer`'s firing arm, and the layout tree's comment on `gate-tests/git-mutation-under-producer.test.sh` (delta 1).
- `native/src/guard/rules/mod.rs` and the rule's module under `native/src/guard/rules/` — the table row and its test (delta 1).
- `guard-kit/guard-tests/` — the rows a decision table can vary, and `guard-kit/gate-tests/git-mutation-under-producer.test.sh` for the pair it cannot (delta 1).
- `delegation-kit/SPEC.md` §The turn-end liveness hook — the task-view paragraph's ground, the added refused repair (its heading's count), the added exit and the honest limit (delta 2); §Operative residency — the one sentence on the launch chokepoint (delta 3).
- `native/src/hook/stop_liveness.rs` — the task-held refusal's text and the stub lane's assertion on it (delta 2).
- `delegation-kit/templates/agent-execution.md` and the carriers the merge-time grep names (delta 3).
- `.workflow/release-declarations.md` — a Behavior changes row for the new block and for the refusal's added exit (deltas 1 and 2).
- `docs/guard-kit/SPEC.md`, `docs/delegation-kit/SPEC.md` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).

## Retired spellings

- None — no delta of this amendment retires a name; delta 2 and delta 3 reword a message and a clause in place.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **The marker discharged** — delta 1's cannot-run claim is read off the table runner once the rule exists.
- [ ] **Amendment deleted** — this file removed on merge, in the batch that merges the last of its deltas, before the stage that drains the queue; none remain at the root (`ls SPEC-*.md`).
- [ ] **Queue entry moved to Done** — `--queue done producerless-wait-recurred`, in that same commit and before the stage that drains the queue.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
