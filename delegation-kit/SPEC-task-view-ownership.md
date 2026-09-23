# SPEC amendment: task-view-ownership

`subagent-stop-liveness` refuses a dispatched session's turn end while the payload's `background_tasks` shows any running `shell` element. The view is wider than the emitting session. So a child holding no shell task of its own is refused on a task its dispatcher holds, and the refusal lasts as long as that task. The commonest foreign task is the dispatcher's own wait on that child, which makes the stall a circular wait.

**Measured at authoring.** A dispatched session held one backgrounded observer loop and dispatched a child that made no tool call. The child's own log took nine `verdict=green records=0 decision=refuse` lines in 24 seconds, each one the task-view arm. The child quoted the arm's refusal text. It was released 28 seconds after launch while the dispatcher's loop was still running, so the task ending did not release it. What did release it is not read here. The harness's consecutive-block cap fits the count, but that is **inferred, not read**. So §What `background_tasks` carries' reading that the view "spans the emitting agent's own tree" is too narrow. It spans at least the emitter's dispatcher's tasks. Whether it spans a sibling's tasks too was not tested.

**The ownership key is not in the view. It is in the emitter's own transcript.** No view element carries a pid or owner (§What `background_tasks` carries), so no scoping can be read off the view alone. The payload's `agent_transcript_path` key is present on every logged firing (the `keys` column). The emitting session's transcript records every shell task that session launched, under the harness's task id. Both ways a task comes to exist were probed. A backgrounded launch's tool result carries `backgroundTaskId` and the id in its text. A foreground call the harness moved to the background on its timeout carries the same key beside `timedOutAfterMs`. The refused probe child's transcript carried neither its dispatcher's task id nor any `backgroundTaskId`.

**Operator direction, 2026-09-23 (lead-relayed; a direction, not a ruling): scope the arm to the emitting session's own launches, falling back to the unscoped refusal.** The operator was told this narrows the unconditional refusal set the 2026-09-21 direction granted, and chose it knowing that. The two-sided live check in delta 1, and stopping to escalate if its own-task side fails, are part of the direction.

**Ruled out.** *Refuse once* (read `stop_hook_active`, as `unresolved` does) keeps the foreign refusal and also lets a session end its turn on its **own** moved call at the second try. That weakens the case the arm exists for in order to bound a case it was never meant to cover. *Retiring the arm* gives up the moved call, which leaves the record set blind to it again (§The probe is asymmetric, the residue). *Logging an owner field* adds nothing: the decision needs the key, the log does not, and a value log is refused on its own grounds.

## What changes

### (1) The task-view condition counts only the emitting session's own launches

`native/src/hook/stop_liveness.rs` narrows `shell_task_running` from any running `shell` element to one the emitting session launched. **{design-bearing}**

- **Ownership.** A running `shell` element is **the emitter's own** when its `id` occurs as a **whole token** in the file the payload's `agent_transcript_path` names. A whole token is bounded on both sides by a character outside `[A-Za-z0-9_-]`, or by the file's start or end. An element whose `id` is absent or not a string counts as own.
- **The fallback never fails open.** Every running `shell` element counts, which is today's decision, when `agent_transcript_path` is absent, is not a string, or names a file that cannot be read. Scoping is only ever a narrowing off a key the hook actually holds.
- **Read lazily, once.** The transcript is read only when the view holds at least one running `shell` element, and at most once per firing. The common firing, whose view holds none, reads nothing new.
- **Nothing new is observable.** The hook logs nothing from the transcript or the view. The log grammar, its field list, the `TASK_REFUSAL` text and the exit contract stay as they are. The file header's "a running harness shell task" becomes "a running harness shell task of its own".
- **Cases (module tests).** Six rows are added beside the existing five, which stay as they are. The existing rows carry no `agent_transcript_path`, so they now pin the fallback. The new rows:
  - `own` — the id is in a scratch transcript: refuse.
  - `foreign` — the id is not in it: allow, stderr empty.
  - `transcript-absent` — the key is missing: refuse.
  - `transcript-unreadable` — the path names no file: refuse.
  - `id-inside-a-longer-token` — only the whole-token rule tells this apart: allow.
  - `id-absent` — the element has no `id`: refuse.

  Each row keeps the existing assertion that the log line carries no value of the view.
- **Live acceptance, two-sided, logged to the build journal.** Two claims below are **inferred, not run**: that a view element's `id` for a shell task is the same token as `backgroundTaskId`, and that `agent_transcript_path` names the emitter's own transcript. The case suite cannot settle either, because both are harness facts, so the build settles them live after the binary is built:
  - **(a) Foreign.** The dispatching session holds a backgrounded observer loop and dispatches a no-tool child as a declared mutating type without isolation, so the child's firings land in the main checkout's log. The child's stop must log `decision=allow`, with no `decision=refuse` beside a non-refusing verdict in its window.
  - **(b) Own.** A child makes a foreground call that outruns a short timeout, so the harness moves it to the background, and then ends its turn. Its stop must log `decision=refuse` beside `verdict=green` until the moved call's completion notification.

  If (b) allows, the id premise is false. The delta then does not land as written: build stops and escalates, because the arm would fail open on exactly the case it exists for.

### (2) The hook's section states the view's scope and why it differs from the record set's

§The turn-end liveness hook is updated at the three passages that describe the task view. **{mechanical}**

**Replacement text** — **Not yet applied**:

- The contract sentence: "exit 2 on `red`, `corrupt` or `unresolved`, or while the payload's `background_tasks` shows a running `shell` task **the emitting session launched**, exit 0 on every other path, and no hook JSON on either".
- The paragraph opening **The harness's task view is a second refusing condition** is rewritten so that its first two sentences read:

  > **The harness's task view is a second refusing condition, beside the reading rather than in it, and it counts only the emitting session's own launches.** A `background_tasks` element with `type` `shell` and `status` `running` refuses whatever the reader said when its `id` occurs as a whole token in the transcript the payload's `agent_transcript_path` names. That is where the harness records every shell task a session launches, a moved call included, and where the view itself carries no owner. An absent or unreadable transcript counts every running `shell` element, so the scoping narrows only off a key the hook holds and never fails open.

  The rest of that paragraph stands. "It logs nothing of the view" becomes "It logs nothing of the view or the transcript."
- A new paragraph follows it:

  > **The view is scoped where the record set is shared, and the difference is deliberate.** The record set binds every session (the shared-scratch-dir paragraph below), because a record declares a producer, and observers write none. The view makes no producer/observer split, and it spans more than the emitter: a child holding no task of its own was refused on its dispatcher's. The commonest foreign element in a dispatch is the dispatcher's own wait on that child, so an unscoped view turns that wait into a circular one. A dispatcher's recorded producer still refuses the child through the record set. **Honest limit:** the transcript is harness state that no contract covers, the same footing as the payload. A revision that stopped writing task ids there would make every element foreign and fail the arm open on the session's own moved call. The record set is untouched by that, and only a re-run of the two-sided live check catches it.
- In the paragraph opening **An unreadable payload does not disable enforcement**, the clause "and the task-view condition above is read from the payload, so an unreadable one drops it and leaves the record-set decision" becomes "and the task-view condition above is read from the payload, so an unreadable one drops it and leaves the record-set decision; an unreadable **transcript** leaves the condition unscoped rather than dropping it".

### (3) §What `background_tasks` carries corrects its scope claim

The measured claim is corrected to what the two measurements support. **{mechanical}**

**Replacement text** — **Not yet applied**:

- In **It is a live-children enumeration, and it is populated**, the sentence beginning "It spans the emitting agent's own tree rather than its direct children only" becomes:

  > It is wider than the emitting agent: the emitting session itself, its backgrounded shell task and its dispatched grandchild all appeared with `status` `running`, the count rising as the grandchild started. A later read found a child holding no task of its own refused on a running shell task its dispatcher held. So the view spans at least the emitter's dispatcher's tasks. Whether it spans a sibling's too was not tested.
- **No entry carries a pid** gains a closing sentence:

  > Ownership is read off the emitting session's transcript instead (§The turn-end liveness hook), which the harness writes each launched task's id into, so the join needs no value of the view beyond `id`.

## Producers and consumers

- **The ownership test (delta 1).** Producer: the harness. It writes `agent_transcript_path` on every `SubagentStop` payload, and the key is present on every line of `.workflow/subagent-stop-liveness.log`'s `keys` column. It writes the task id into that transcript at a backgrounded launch and at a timeout move (both probed). Enabling config: the hook's registration, which this repo's `.claude/settings.json` carries. Consumer: `shell_task_running`, by a file read. The refusal set narrows, so point 5 applies. The case rows in `stop_liveness.rs` assert an exact exit per row, and none flips: every existing row carries no transcript key, so each takes the fallback and keeps its exit. `scripts/gate-tests/subagent-stop-reader.test.sh` sends no `background_tasks`, so no verdict of it moves. The close-stage triage reads `decision=refuse` beside a non-refusing verdict as a task-held refusal. That reading keeps its meaning, and after delta 1 it only ever names an own task.
- **No field, knob or name is added.** No log field and no stderr change, so no field needs a reader. No knob: scoping is a correctness repair and not a calibration a consumer selects. The seam: all of it is kit mechanism in delegation-kit's hook member. No rule content is private, and no consumer config is involved.
- **The `TASK_REFUSAL` text's reader is the refused session.** It keeps "a background shell task of this session", which becomes true without an edit.

## Existing sections updated

Roster probe: `git grep -n -i "background_tasks\|task view\|task-view\|agent_transcript_path\|shell task"` over the tracked tree, minus the queue and `.workflow/`, run at authoring.

- `native/src/hook/stop_liveness.rs` — `shell_task_running`, its call site, the file header and the case module (delta 1).
- `delegation-kit/SPEC.md` — §The turn-end liveness hook (delta 2) and §What `background_tasks` carries (delta 3).
- `docs/delegation-kit/SPEC.md` — the generated on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- None — no delta retires a name. The scope claim is rewritten in place, and no identifier, knob or field is removed.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the ownership test.
- [ ] **Instruction surfaces: instruction only** — no template or agent definition changes. The refusal text is unchanged.
- [ ] **Merged with no information lost** — deltas 2 and 3 re-phrase the passages they refine. The measured stall and the ruled-out alternatives move into §The turn-end liveness hook's prose.
- [ ] **Live acceptance recorded** — both sides of delta 1's two-sided check pass, and the result is written into the merged SPEC as measured. That settles both inferred premises, and the inferred markers go with it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none retired, as the block above declares.
- [ ] **Entry moved** — the paired entry moves to Done in the build stage's merge commit, before the drain stage.
- [ ] **Gaps filed** — cross-component gaps found during the work are filed as debt tasks.
