# SPEC amendment: foreign-session

**The second slice of cross-vendor delegation: a foreign run that can escalate and be answered in place.** The first slice, §The foreign-vendor run, runs one read-only audit or mechanical sweep on another vendor's coding agent and returns a report. Its stated limit is that a run is not resumable, so its only escalation channel is a report that ends the work. A foreign agent meeting a question its prompt did not settle either guesses or stops, and stopping costs the whole session's context on the re-run. This slice adds the dispatch transport the entry names as (2), with the escalation resume model, (1), riding it: open, prompt, permission request and resume, each over the vendor's machine plane.

**The model is one process per turn, with the session held by the vendor.** On the master harness a paused session holds its state in memory and a message resumes it. A vendor's headless mode exits at its turn end. Its conversation persists in the vendor's own session store, and a resume form continues it by session id. So the kit keeps no conversation. It keeps the binding, the run's key to its adapter, its mode, its base commit, its turn count and the vendor's session id, beside the clone the session works in.

- **Open** is `--foreign-run` on an adapter that also configures a resume form. Its first turn runs as today. On `OK` the clone stays and the session is recorded.
- **Prompt** is a file on standard input, at the open and at every resume.
- **A permission request is an escalation.** A headless vendor mode cannot prompt anyone, so the adapter pins a policy that never asks. An action the policy refuses reaches the agent as a failure, which it reports at its turn end like any other open question. The kit relays no approval prompt. A widening is a new run under an adapter the consumer configured wider, never a flag a resume can add.
- **Resume** is `--foreign-resume <key> <prompt-file>`: the dispatcher's answer, delivered as the next turn's prompt, in the same clone and the same vendor session.

**The escalation channel is the report, and the kit reads no prose.** Whether a turn's report is a finished result or a question is the dispatcher's judgment. Classifying it would parse an agent's prose or bind a vendor's event schema into a kit literal (gate-sdk/SPEC.md §The provenance seam). The dispatcher asks the agent to end its turn with its open questions rather than guess, the foreign side of the template's **Supervisor owns rulings; agents surface, never guess** rule.

**Measured at authoring.** These help-text reads ran no vendor turn.

- One installed vendor CLI's headless mode takes a resume form, a session id or "the most recent", with the prompt on standard input.
- Its resume form filters candidate sessions by working directory.
- Its resume form takes no sandbox flag, only a configuration override, so the resume argv cannot copy the open argv.
- Its ephemeral flag persists no session at all.
- The second installed CLI's print mode takes a conversation id to resume.

What neither help text settles is where each vendor prints its session id in a mode whose standard output is the report. That is **inferred, not run**, and the live run in the Definition of Done settles it before the knob shape is relied on.

**Foreign adapters bind to no tier class.** That was this slice's question to answer for the critique entry. §The tier binding has three readers: a definition's generated `model:`, D6 at a dispatch, and `--model-verdict` over a transcript. All three read the master harness's model ids. An adapter's argv pins its own vendor's model, so a class bound to an adapter would have no reader. The dispatcher chooses an adapter by name for the unit's class.

**Refused alternatives.**

- **A TUI relay.** It buys no resume state, which lives in the vendor's store whatever renders it, and it bets on the vendor's least-stable surface. The entry records this ground.
- **A long-lived vendor process the kit holds open across turns.** It would make the kit own a process lifetime, and a dispatched session's turn end reaps its observer (§The delegation model). Its one gain, a warm process, is what the vendor's warm resume already provides.
- **A foreign stage session in this slice.** A stage session stamps, commits and reads its own transcript for its session id. A foreign agent here commits nothing, since its commits are refused by construction, and has no transcript the stamp protocol reads. Stage dispatch stays the entry's (4), stage-contract expression, with the stamp path it needs.

## What changes

### (1) Two knobs: a resume form and a session-id marker per adapter

delegation-kit gains `DELEGATION_KIT_FOREIGN_RESUME` and `DELEGATION_KIT_FOREIGN_SESSION_MARKER` {mechanical} {user-facing: envelope the entry's next slice, (2) with (1) riding it, the unit selected by operator direction 2026-10-03; the arm, knob and line shapes are spec's calibration}. **Not yet applied.** Both rows join `native/src/knobs/delegation_kit.rs`, and both bullets join §Layout and configuration after `DELEGATION_KIT_FOREIGN_TIMEOUT`:

> - `DELEGATION_KIT_FOREIGN_RESUME` — each foreign adapter's resume form (§The foreign-vendor run): indexed, each element `<adapter>=<word>`, the form's argv being its words in element order, as `DELEGATION_KIT_FOREIGN_ADAPTERS`' are. A word containing `@SESSION_ID@` has the session's recorded id substituted, and `@PROMPT_FILE@` the turn's prompt file. Default empty, which leaves every adapter one-shot. The table validator refuses a malformed element and an adapter that `DELEGATION_KIT_FOREIGN_ADAPTERS` does not configure. A resume form restates its confinement, since a vendor's resume form may spell its sandbox differently from its open form.
> - `DELEGATION_KIT_FOREIGN_SESSION_MARKER` — the literal after which a turn's output carries the vendor's session id: indexed, at most one `<adapter>=<marker>` element per adapter. The id is the run of `[A-Za-z0-9._:-]` immediately after the marker's first occurrence in the turn's standard output, else its standard error. Default empty. An adapter with no marker records no id, so its resume form must not carry `@SESSION_ID@`, and the validator refuses one that does. Such a form resumes through a vendor's "most recent session in this directory" form instead, which the clone's own working directory scopes.

`templates/delegation-config.knobs` gains two commented examples after the foreign adapters' lines, under a `spec:` line of their own citing §The foreign-vendor run, with no vendor named: `# DELEGATION_KIT_FOREIGN_RESUME[] = <adapter>=@SESSION_ID@` and `# DELEGATION_KIT_FOREIGN_SESSION_MARKER[] = <adapter>=<literal>`.

### (2) `--foreign-run` opens a session on a resumable adapter

§The foreign-vendor run's step 4 and its verdict line change for an adapter `DELEGATION_KIT_FOREIGN_RESUME` names {design-bearing} {user-facing: envelope the entry's next slice, (2) with (1) riding it, the unit selected by operator direction 2026-10-03; the arm, knob and line shapes are spec's calibration}. **Not yet applied.** Step 4's replacement text:

> 4. **The cleanup, or the session.** On a one-shot adapter the clone is removed, except on a refusal, where it stays for inspection and the line names it. On a resumable adapter an `OK` turn opens a **session**. The clone stays, and `session.txt` in the run's directory records `adapter=<adapter> mode=<mode> base=<commit> turn=1 id=<id|->`. The refs the shape check compared are kept beside it as `refs.txt`, for the next turn's check. A resume form needing an id where no marker produced one opens no session: the clone is removed and the line says why.

The verdict line keeps its fields. An `OK` that opened a session carries the consequence clause `— resumable: --foreign-resume <key> <prompt-file>`, and one that could not open a session carries `— not resumable (<why>)`. That is §usage-verdict's verdict-string contract: reading, status, consequence. A run whose key holds an open session fails as a kept clone does, naming the session and `--close`, so the `FAILED` bullet's "a kept clone under the key" becomes "a kept clone or an open session under the key".

### (3) `--foreign-resume` answers a session and closes it

delegation-kit gains the compiled arm `bash gate-sdk/bin/run-gates.sh --foreign-resume <key> <prompt-file>` and its closing form `--foreign-resume <key> --close` {design-bearing} {user-facing: envelope the entry's next slice, (2) with (1) riding it, the unit selected by operator direction 2026-10-03; the arm, knob and line shapes are spec's calibration}. §The foreign-vendor run gains a subsection, **Resuming a session**. **Not yet applied.**

> ### Resuming a session
>
> `bash gate-sdk/bin/run-gates.sh --foreign-resume <key> <prompt-file>` runs the next turn of the session under `<GATE_SDK_TMP_DIR>/foreign/<key>/`. It reads `session.txt`, spawns the adapter's resume form from `DELEGATION_KIT_FOREIGN_RESUME` exactly as step 2 spawns an open, in the kept clone with the prompt file on standard input, and bounds it by `DELEGATION_KIT_FOREIGN_TIMEOUT`. Before the spawn, the previous turn's `report.txt` and `stderr.txt` move to `report.<n>.txt` and `stderr.<n>.txt`, `<n>` the turn they came from, so every turn's return survives.
>
> The shape is checked as step 3 checks it, against `refs.txt` from the open and, in `sweep` mode, regenerating `change.patch` against the session's `base`, so the patch is always the session's whole change. Then:
>
> - **`OK`** — the turn counter advances, and the line carries the resumable clause again.
> - **`REFUSED`** — the session ends. `session.txt` is removed, so nothing can resume it, and the clone stays as a refusal's evidence.
> - **`FAILED`** — the turn counter advances, and the session stays open: a vendor window that refused the turn may be retried after it resets. A missing `session.txt`, an unknown key, an unreadable prompt file or an adapter the knob no longer configures fails without a spawn.
>
> The line is `foreign-resume: adapter=<adapter> mode=<mode> key=<key> turn=<n> exit=<status> report=<path|none> patch=<path|none> -> <VERDICT>`, on the run's exit codes.
>
> `--foreign-resume <key> --close` ends a session the dispatcher is done with. It removes the clone, `session.txt` and `refs.txt`, keeps every report and the patch, and prints `foreign-resume: key=<key> -> CLOSED` at exit 0. A key with no open session is `FAILED` at exit 2. A session never closed is reclaimed with the scratch dir at the consumer's work-unit boundary. A malformed argv is a shape refusal at exit 2, and `--` ends option processing.

Registration matches `--foreign-run`'s:

- the arm row joins `native/src/emit/mod.rs`'s table, with declared knobs `DELEGATION_KIT_FOREIGN_ADAPTERS`, `DELEGATION_KIT_FOREIGN_RESUME`, `DELEGATION_KIT_FOREIGN_SESSION_MARKER`, `DELEGATION_KIT_FOREIGN_TIMEOUT` and `GATE_SDK_TMP_DIR`;
- `--foreign-resume` joins the crate's network-spawner list there, which a unit test holds disjoint from the fence-safe set;
- its usage line joins `native/src/runner.rs`'s help, and delegation-kit/README.md's **Use** block gains `"$gates" --foreign-resume <key> <prompt-file>  # the next turn of a session a resumable adapter opened: exit 0 OK, 1 REFUSED, 2 FAILED; --close ends it` after the `--foreign-run` line;
- gate-sdk/SPEC.md §The non-gate arm's spawner rosters gain it beside `--foreign-run`.

`--foreign-run`'s declared knobs gain the two from delta 1, in its arm row and in §The foreign-vendor run's closing sentence naming them.

### (4) §The foreign-vendor run states the transport model and its limits

§The foreign-vendor run gains the transport model, and its honest limits are rewritten {design-bearing}. **Not yet applied.** After the section's second paragraph:

> **A resumable session is one process per turn, its conversation held by the vendor.** A vendor's headless mode exits at its turn end and keeps the conversation in its own session store, which its resume form continues. So the kit holds no conversation, only the binding beside the clone. **A permission request travels as an escalation.** The adapter pins a policy that never prompts, an action it refuses reaches the agent as a failure, and the agent reports it at its turn end like any open question. A widening is a new run under an adapter configured wider, never a flag a resume adds. **The report is the escalation channel, and the kit classifies none.** Whether a turn ended on a result or a question is the dispatcher's reading. The dispatcher answers a question with `--foreign-resume`, and the agent continues in the same clone and the same vendor session.

The honest limit **No resume.** is replaced:

> - **Resume is the vendor's.** A session resumes only as far as its vendor's store keeps it: an ephemeral open persists nothing to resume, and a vendor that expires a session fails the next turn. A resume form that does not restate the open form's confinement runs the next turn under the vendor's default policy.
> - **A turn outlives a foreground call.** The timeout's default exceeds a foreground call's ceiling, so a long turn is launched backgrounded with its liveness record (templates/agent-execution.md, **Background + notification, never poll**). While that record names a live turn, the guard blocks every tracked-tree mutation in the checkout (guard-kit/SPEC.md §The generic ruleset, rule `git_mutation_under_producer`), so the dispatcher commits nothing until the turn ends.

The **What returns, and how it lands** list's model bullet gains: "No adapter binds to a tier class: the tier binding's readers all read the master harness's model ids, so the dispatcher picks an adapter by name for the unit's class."

### (5) The protocol template carries the resume

delegation-kit/templates/agent-execution.md's **A foreign-vendor run is mechanical work returned through files** bullet: the closing sentence "No guard fires on it and it cannot be resumed, so size it to finish in one run." is replaced {mechanical} {user-facing: envelope the entry's next slice, (2) with (1) riding it, the unit selected by operator direction 2026-10-03; the wording is spec's calibration}. **Not yet applied.**

> No guard fires on it. Ask the agent to end its turn with its open questions rather than guess. Where your consumer configures the adapter's resume form, answer a question with `--foreign-resume <key> <answer-file>`, which continues the same session in the same clone, and end the session with `--foreign-resume <key> --close` once you hold the result. Otherwise size the unit to finish in one run. A turn can outlast a foreground call, so launch a long one backgrounded with its liveness record and commit nothing while it runs.

The bullet's lead-in is unchanged.

### (6) The executor's crate tests take the session

The executor module's tests gain the session's rows, driven against a throwaway repository with stub adapters {design-bearing}. **Not yet applied.**

- **Open.** A resumable adapter's `OK` keeps the clone and writes `session.txt` and `refs.txt`. A marker in standard output, and one only in standard error, each record the id. A resume form carrying `@SESSION_ID@` with no id captured opens no session and removes the clone.
- **Resume.** `@SESSION_ID@` and `@PROMPT_FILE@` are substituted. The previous turn's report is rotated, and the turn counter advances. A sweep's patch covers the session's whole change across two turns.
- **The resume verdicts.** A resume whose stub commits is `REFUSED` and removes `session.txt`. A non-zero resume is `FAILED` and keeps the session. A resume of an unknown key fails without a spawn.
- **Close and collision.** `--close` keeps every report. A new `--foreign-run` under an open session's key fails.
- **The validator.** It refuses a resume element for an unconfigured adapter, a second marker for one adapter, and `@SESSION_ID@` with no marker.

The stubs use programs the crate's tests already spawn, as the first slice's do. §Testing names the rows.

## Producers and consumers

- **`DELEGATION_KIT_FOREIGN_RESUME` and `DELEGATION_KIT_FOREIGN_SESSION_MARKER`.**
  - Producer: a consumer's knob file, here the private overlay `scripts/delegation-config.local.knobs`, which `.gitignore` covers. Its two adapters today carry the vendor's ephemeral flag, so neither can resume. The build adds a third, resumable adapter there once the operator confirms its argv (Definition of Done, *a live run*), and leaves the two one-shot: the close review's standing direction names the existing review adapter (the `second-vendor-review` roster block foreign-review lands), and a resume form on it would open a session at every close.
  - Consumers: the two arms, the table and its validator in `native/src/knobs/delegation_kit.rs`, `--emit knob-roster`, `check-knob-citation`, `check-knob-default-coupling` (the two "Default empty") and the template's example lines.
- **`session.txt`.** Producer: an `OK` open on a resumable adapter. Reader: `--foreign-resume`. Each field has its reader there: `adapter` picks the resume form, `mode` picks the shape check, `base` is the sweep patch's base, `turn` names the rotated reports, and `id` fills `@SESSION_ID@`. Its absence is the closed state, which a new `--foreign-run` reads as a free key.
- **`refs.txt`.** Producer: the open. Reader: each resume's committed check.
- **`report.<n>.txt` and `stderr.<n>.txt`.** Producer: a resume, rotating the previous turn's files. Reader: the dispatcher, and a person diagnosing a session.
- **The resumable consequence clause.** Producer: an `OK` on either arm. Reader: the dispatcher, which reads its next command off it.
- **`--foreign-resume`'s verdict line.** Reader: the dispatcher, routing on its exit and opening `report=` and `patch=`. `turn` names which rotated file holds an earlier turn.
- **The kept clone.** Readers: the next resume, `--close`, and the iteration boundary's scratch reset, which removes it with the scratch dir.
- **Enabling config.** No tracked configuration can set a resume form, since it names a vendor's program. The producer is reachable outside the crate tests through the private overlay, once the operator confirms the adapter.

## Existing sections updated

- `delegation-kit/SPEC.md`: §The foreign-vendor run, step 4, the verdict line and the `FAILED` bullet (delta 2), the new **Resuming a session** subsection and the declared-knob sentence (delta 3), and the transport model and honest limits (delta 4); §Layout and configuration (delta 1); §Testing (delta 6).
- `delegation-kit/README.md`, the **Use** block (delta 3).
- `native/src/knobs/delegation_kit.rs` (delta 1).
- `native/src/emit/foreign_run.rs` (deltas 2 and 3) and its tests (delta 6).
- `native/src/emit/mod.rs` and `native/src/runner.rs` (delta 3).
- `gate-sdk/SPEC.md` §The non-gate arm, the spawner rosters (delta 3).
- `delegation-kit/templates/delegation-config.knobs` (delta 1).
- `delegation-kit/templates/agent-execution.md` (delta 5).
- `docs/delegation-kit/SPEC.md`, `docs/delegation-kit/README.md` and `docs/gate-sdk/SPEC.md`: the generated mirrors (all deltas).

The roster came from `git grep -n 'foreign-run\|foreign_run\|FOREIGN_ADAPTERS' -- ':!docs/' ':!TASK-QUEUE.md' ':!.workflow/'` over the tracked tree, which names the first slice's registration sites, and from its knob's readers.

## Retired spellings

- None — the amendment adds an arm and two knobs, and the honest limit it rewrites carried no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the knobs, `session.txt`, `refs.txt`, the rotated reports, the arm and the consequence clause.
- [ ] **Instruction surfaces: instruction only** — the template bullet carries the route; the model and its limits sit in §The foreign-vendor run.
- [ ] **Merged with no information lost** — §The foreign-vendor run reads as one document, its one-shot behavior unchanged for an adapter with no resume form.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **A live run** — granted: an operator grant, 2026-10-03, relayed by the lead, iteration-scoped and unspent at authoring. Build spends it and archives it in the spending commit's message. The grant covers one resumable codex adapter in the private overlay, added beside the two one-shot adapters rather than converting either, non-ephemeral, in the read-only sandbox, with a resume form and a session-id marker, and about three small foreign turns. One read-only audit opens a session whose prompt asks the agent to end its first turn with a question. One `--foreign-resume` answers it to `OK`, and `--close` ends the session. The landing commit's message quotes the three verdict lines with the adapter name elided. A live run contradicting the inferred session-id placement corrects delta 1's marker rule before the slice lands.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry is demoted, not done** — its deliverable is the corpus of slices, so the landing commit returns `heterogeneous-agent-delegation` to the deferred section with `--queue demote`, a stage before the drain stage. It drops the `[spec:]` tag, records this slice as landed and names the next, (3) the budget oracle or (4) stage-contract expression (canon-kit/SPEC.md §Merging an amendment, step 4). The same commit rewords the entry's body to fit queue-kit's per-entry cap.
