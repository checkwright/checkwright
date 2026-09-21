# SPEC amendment: harness-moved-producer

A foreground Bash call that outruns its timeout is moved to the background by the
harness. The moved command is a live producer that no liveness record names: the
guard saw a foreground call, so rule 15 had no launch to refuse, and the session
never wrote a `.run` record because it never made a launch. This was attested
four times. In the fourth, the session ended its turn twice on the moved producer,
and the turn-end hook logged `decision=allow records=0` both times.

The queue entry (`harness-moved-background-task-unrecorded`) names two
candidates. The first widens the turn-end hook to refuse on a running harness
`shell` task in its payload's `background_tasks`, which is operator-class. The
second is a steer at the move. This amendment takes both. The steer goes in the
template (deltas 1 and 2). **The widening was granted by operator direction
(2026-09-21, lead-relayed; a direction, not a ruling)**, as delta 3. The grant
does not reopen §What `background_tasks` carries' ruling that the hook logs key
names only, and delta 3 is designed not to need it reopened.

## What changes

### (1) The template treats a moved call as the session's own unrecorded launch

A clause is added to the template's **Background + notification, never poll**
bullet, after the paragraph that ends "Spell a wait inline so it meets that
exemption; a wait behind a script or binary name is invisible to the guard's span
walk and takes the record like any launch." **{mechanical}** **Replacement
text** — **Not yet applied**:

> **A call the harness moves to the background on its timeout is a launch you
> did not make, and it is yours.** The result says so ("moved to the
> background"), and nothing recorded it, because the guard saw a foreground call.
> Prevent it: give a foreground call a timeout above the command's run, and
> launch anything that can outrun that timeout backgrounded, with its record, as
> above. If a call is moved anyway, it is a live producer from that moment. Where
> it names its own pid (a lock, a printed pid), write its `<key>.run` record from
> that pid at once. Either way, end no turn and write nothing to the tracked tree
> until its completion notification arrives.

**Probed at authoring, 2026-09-21.** A foreground call past a 3-second timeout
returned "Command did not complete within its 3s timeout and was moved to the
background (ID: …). Output is being written to: … You will be notified when it
completes." The completion notification arrived on the session's next tool round
with `status` `completed` and the exit code. So the move is announced in the
tool result the session reads, and the harness promises the notification the
clause tells the session to await.

**Honest limit, stated where it is merged.** A moved producer that names no pid
leaves a dispatched session with no in-turn primitive that ends on the
notification: there is no pid for a `kill -0` loop, and a pattern match is
refused. That session can keep working until the notification arrives, but it
cannot wait for it idle. The clause's other two halves still hold: prevention at
the call, and a record wherever a pid exists. Delta 3 holds the turn end over it
mechanically.

### (2) §The delegation model's residue statement names what the steer covers

delegation-kit/SPEC.md §The delegation model ends its launch-chokepoint paragraph
with "The residue left is a command the harness moves to the background on its
timeout: it was a foreground call when the guard saw it, it writes no record, and
no `PreToolUse` payload carries task state." **{mechanical}** **Replacement
text** — **Not yet applied**:

> The residue left is a command the harness moves to the background on its
> timeout: it was a foreground call when the guard saw it, it writes no record,
> and no `PreToolUse` payload carries task state. The template covers the
> recoverable half. It sizes a foreground call's timeout so the move does not
> happen, has a record written from a pid the moved producer names, and holds the
> turn and the tracked tree until the harness's completion notification, which
> the move itself promises. A moved producer that names no pid stays unrecorded,
> but the turn-end hook refuses a dispatched session's turn end while the
> harness's own task view shows it running (§The turn-end liveness hook). Guard
> rule 14 still cannot see it, so a tracked-tree write beside it is held only by
> the template.

### (3) The turn-end hook refuses while the harness shows a running shell task

**Operator direction, 2026-09-21 (lead-relayed): the widening is granted.**
**{design-bearing}** `subagent-stop-liveness` gains a second refusing condition,
independent of the liveness reader: the payload's `background_tasks` array holds
an entry whose `type` is `shell` and whose `status` is `running`. The hook
decides as follows:

- **Refuse** when the reader's reading refuses (`red`, `corrupt`, `unresolved` as
  today), **or** when such an entry exists. The task condition is unconditional,
  like `red`: it reads nothing from `stop_hook_active`, because its condition
  resolves when the task ends.
- An entry of `type` `subagent`, an entry with any other `status`, an absent key,
  a non-array value, and an element that is not an object all contribute nothing.
  A malformed view degrades to the record-set decision alone. This is the
  fail-open-but-loud posture's quiet half: the view is a supplement, and its
  absence must not refuse.

**What is read, and what is logged.** The hook reads two fields of each element,
`type` and `status`, to decide. It logs neither of them, nor a count, nor any new
field, so the log grammar and §What `background_tasks` carries' key-names-only
ruling are untouched. A task-held refusal is legible in the log without a new
field. The reader's `verdict` stays what the reader said, so a refusal that the
reader did not produce shows as `decision=refuse` beside a non-refusing verdict
(`green`, `unavailable`, `unstarted`, `error`), a pairing no other arm produces.
The stderr refusal names the arm: "the harness shows a background shell task of
this session still running (a call moved to the background, or a backgrounded
launch); await its completion notification before ending the turn".

**Whether a finished task leaves the array is not a premise of this design.** The
decision keys on `status` being `running`, so a finished task either leaves the
array or carries another status, and both allow. The residual is a finished task
still reporting `running`, which would hold the refusal across the session's
retries. The log exposes that without logging a value: a run of `decision=refuse`
beside a non-refusing verdict that outlasts the task's completion notification.
The first live firing measures it that way. **Honest limit:** until then the
residual is unmeasured, and a session caught in it has no in-band way out. An
operator-visible `decision=refuse` run in the log is the signature.

**Replacement text** — **Not yet applied**:

- §The turn-end liveness hook's contract sentence becomes "exit 2 on `red`,
  `corrupt` or `unresolved`, **or while the payload's `background_tasks` shows a
  running `shell` task**, exit 0 on every other path, and no hook JSON on either".
- The reading table gains a closing paragraph:

  > **The harness's task view is a second refusing condition, beside the reading
  > rather than in it.** An element of `background_tasks` with `type` `shell` and
  > `status` `running` refuses whatever the reader said. It covers the producer no
  > launch record names — a call the harness moved to the background on its
  > timeout — which §What `background_tasks` carries measured the view as
  > enumerating, since the harness launched it. It supplements the record set and
  > substitutes for nothing: a detached producer the view does not enumerate is
  > still the record set's. It logs nothing of the view.

- §What `background_tasks` carries' sentence "So the blocking hook **cannot
  substitute** the harness's view for the `*.run` record set" keeps its ruling, and
  the paragraph's closing "Supplementing is the most it could do …" becomes
  "Supplementing is the most it could do, and the hook now does: it refuses on a
  running `shell` element (§The turn-end liveness hook) and logs none of the
  view's values."

**Test.** `native/src/hook/stop_liveness.rs`'s case table (the hook's fixture
lane, which drives `fire` over payloads) gains four rows. Each has a green reader
and an empty record set, and they differ in `background_tasks`:

- a running `shell` element → exit 2, `decision=refuse`, `verdict=green`, stderr
  naming the task arm;
- the same element with status `completed` → exit 0;
- a running `subagent` element → exit 0;
- `background_tasks` a string → exit 0.

A fifth row pairs the running `shell` element with `stop_hook_active` true and
wants exit 2, which pins the arm as unconditional.

## Producers and consumers

- **The task-view refusal (delta 3).** *Producer:* `subagent-stop-liveness` at
  every `SubagentStop`, registered in this repo's `.claude/settings.json`.
  *Enabling fact:* §What `background_tasks` carries measured the key as present
  and populated, and its `shell` elements as carrying `type` and `status`. *Fields
  read:* `type` and `status`, each read at the decision; nothing is written.
  *Consumer:* the dispatched session, which receives the stderr as stop-hook
  feedback at a real turn end. An intermediate firing that exits 2 delivers
  nothing (§The turn-end liveness hook), so the frequency with which a running
  task refuses there costs nothing. That settles one half of the queue entry's
  unmeasured pair from an existing measurement. *Point 5:* the refusal set only
  widens, and the log's one reader of `decision`, a triage reader, is monotone in
  it.
- **Waits under the sanctioned primitive.** A session's own backgrounded wait loop
  is a running `shell` task, so a turn end during it refuses. That is the waiting
  rule itself (a turn never ends on work still running), and intermediate firings
  cost nothing, so it needs no exemption.

- **The clause (delta 1).** *Producer:* the template, which a dispatching session
  loads through the dispatch trigger. A dispatched session receives the same
  obligation through its always-loaded restatement of this bullet, where the
  consumer carries one (§Operative residency); this repository's two
  `.claude/agents/` definitions restate the waiting rule. *Consumer:* the session whose call was moved. The field it
  reads is the tool result's move line, which it already holds. No gate reads the
  clause, and the obligation is a request (§Operative residency's "operative is
  not obeyed").
- **Carriers of the waiting rule (propagate obligation).** The template header
  obliges propagation when the **Background + notification, never poll** rule
  changes. Probe: `grep -rln "never poll\|recorded PID\|kill -0" .claude/`
  (the bullet name wraps across lines in both carriers, so the probe keys on the
  rule's phrasing) matched `.claude/agents/audit-sweep.md` and `.claude/agents/stage-session.md`.
  Each takes one sentence in its own voice: a call moved to the background is your
  own producer; record it from any pid it names, and end no turn on it.
- **Record readers.** A record written from a moved producer's pid is an ordinary
  `<key>.run` record, read unchanged by `check-producer-liveness`, guard-kit rule
  14 and the turn-end hook. No reader changes.

## Existing sections updated

- `delegation-kit/templates/agent-execution.md` — the **Background + notification, never poll** bullet (delta 1).
- `.claude/agents/audit-sweep.md` and `.claude/agents/stage-session.md` — the propagated sentence (delta 1).
- delegation-kit/SPEC.md §The delegation model — the residue sentence (delta 2).
- `.workflow/release-declarations.md` — one `## Behavior changes` bullet led by
  `**`delegation-kit/templates/agent-execution.md`**`: a call moved to the
  background on its timeout is the session's own producer; size timeouts, record
  from a named pid, and hold the turn to its notification. A consumer whose agent
  definitions restate the waiting rule propagates the sentence (delta 1).
- delegation-kit/SPEC.md §The turn-end liveness hook — the contract sentence and the task-view paragraph; §What `background_tasks` carries — the supplement sentence (delta 3).
- `native/src/hook/stop_liveness.rs` — the task-view condition, the refusal text and five case rows (delta 3).
- `.workflow/release-declarations.md` — a second `## Behavior changes` bullet led by
  `**`subagent-stop-liveness`**`: the turn-end hook also refuses while the
  harness's `background_tasks` shows a running `shell` task, a call moved to the
  background included; the log grammar is unchanged, and such a refusal reads as
  `decision=refuse` beside a non-refusing verdict (delta 3).
- `docs/delegation-kit/SPEC.md` — the generated mirror, regenerated by its arm (all deltas).

## Retired spellings

- None — no name, knob, path or tag is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
