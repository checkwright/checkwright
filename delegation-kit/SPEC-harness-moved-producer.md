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
second is a steer at the move. **This amendment rules the steer, in the template.
The widening is escalated and is not part of this amendment unless it is granted**,
in which case a delta is added here (§Pending ruling).

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
the call, and a record wherever a pid exists. The rest stays residue until the
escalated hook question is ruled.

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
> and nothing refuses a turn end over it.

## Pending ruling

**Escalated to the operator through the lead, 2026-09-21: whether the turn-end
hook's refusal set widens to a running harness `shell` task in `background_tasks`.**
The facts the ruling would rest on, as they stand:

- §What `background_tasks` carries measured the array as the harness's
  live-children view, with `shell` entries carrying `id`, `type`, `status`,
  `description` and `command` and no pid. The array is a supplement to the record
  set, never a substitute.
- An intermediate `SubagentStop` that exits 2 delivers nothing to the session
  (§The turn-end liveness hook), so how often a running task would refuse at an
  intermediate firing costs nothing. That half of the entry's unmeasured pair is
  already settled by an existing measurement.
- Whether a finished task leaves the array is still unmeasured. The hook logs key
  names only, and measuring the values reopens §What `background_tasks` carries'
  ruling to log no values.

If the widening is granted, it is a new delta here: an eighth reading or a
refusal arm, plus a fixture row, plus a `## Behavior changes` bullet.

## Producers and consumers

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
