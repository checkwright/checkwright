# SPEC amendment: isolation-seams

Three gaps in how delegation-kit's protocol template covers a dispatch under
`isolation: worktree`. Each is one half of a rule whose other half already ships:

- the report channel of an isolated read-only child (queue:
  `worktree-isolated-agent-report-lost-to-a-failed-peer-send`);
- the dispatcher half of isolation cost (4) (queue:
  `isolation-oracle-cost-lacks-dispatcher-clause`);
- the isolation a resume silently drops (queue:
  `isolated-dispatch-resume-loses-its-isolation`).

All three land in `delegation-kit/templates/agent-execution.md` and in
§The delegation model, which holds the grounds for the template's isolation
bullets. This consumer's own read-only agent definition takes the one child-side
clause (delta 3).

## What changes

### (1) An isolated read-only prompt states that the return value is the channel

Shape two of the queue entry's three is the ruling: the dispatcher spells the
obligation into the prompt. **{mechanical}** The template's
**A child's only upward route is a durable artifact** bullet, disposition (2),
is rewritten. **Replacement text** — **Not yet applied** — for the sentence
"**(2) Return-value only, otherwise** — a fan-out needing no mid-run channel
needs no artifact, which is the read-only carve-out above.":

> **(2) Return-value only, otherwise** — a fan-out needing no mid-run channel
> needs no artifact, which is the read-only carve-out above. Say so in the
> prompt of every isolated read-only dispatch: its report is its final message,
> or the harness's hand-back where it has one, and it sends to no peer.

**The other two shapes were weighed and declined, and the grounds go into
§The delegation model beside D3.** Replacement text — **Not yet applied** — as a
new paragraph after the "**D3 advises rather than blocks**" paragraph:

> **The return-value obligation of an isolated read-only child is carried by the
> dispatcher's prompt, not by this guard and not by the kit.** Nothing carries
> D3's warning into the child: an isolated read-only child whose report went to a
> peer name it could not resolve returned a bare `.`, twice at one close, and a
> bare `.` reads as a sweep that found nothing. The template's disposition (2) has
> the dispatcher say so in the prompt. Two other carriers were weighed. **The
> guard rewriting the prompt** was declined: it would make this guard an author of
> dispatch text rather than a judge of dispatch shape, for a class that has not
> reproduced since the harness began stating the hand-back duty in the child's
> own prompt (re-measured twice, the second time on a 46-tool-use isolated sweep).
> **The agent-type definition** is the consumer's surface, since the kit ships no
> agent-type names (§Layout and configuration, `DELEGATION_KIT_READONLY_TYPES`); a
> consumer may carry the line there as well, and this repository's read-only type
> already does in its return contract.

### (2) Isolation cost (4) gains its dispatcher half

The residency call is **the template, as one sentence on cost (4)**, matching
the dispatcher clause cost (3) already carries. **{mechanical}** A guard
assertion is declined on the ground §The delegation model already gives for
cost (3): the dispatch payload carries the prompt, never the list of oracles a
sweep will run, so a rule there would key on prompt wording. Always-loaded
residency is declined by §Operative residency's condition (a): the dispatcher
fires the trigger that loads the template. A shared classify-before-dispatch step
across (3) and (4) was also weighed. It would have to re-phrase cost (3)'s ruling
that such a sweep is "not delegable at all" to a D2-held type. The unit sets that
ruling outside its reach, so the step is not taken.

**Replacement text** — **Not yet applied** — for cost (4)'s closing sentence
"Name the gate that could not run, say why, and return; the parent's checkout has
the binary and can run it.":

> Name the gate that could not run, say why, and return. **The dispatcher's half,
> on (3)'s pattern:** a sweep whose work includes a binary-dispatched gate or arm
> is not delegable into isolation as an oracle-running sweep. Run the arm in your
> own checkout, write its output under your scratch dir, and name that file
> absolute into the main checkout in the prompt. Classify before you dispatch,
> never after you read the answer.

**Why an absolute path works where cost (3) says a gitignored surface does not.**
Cost (3) is about a path the child resolves inside its worktree, which carries no
gitignored file. A path named absolute into the main checkout reaches the main
checkout's file, the same route the resume-journal rule uses for a child's
writes. Measured: the sweep the queue entry cites finished once it was
re-dispatched with the arm's stdout materialized that way. The new sentence is
added to §The delegation model's paragraph on cost (3)'s composition as one
closing sentence — **Not yet applied**:

> Cost (4) now carries the matching dispatcher half: an oracle-running sweep is
> classified before dispatch, and the arm's output is handed over as a file named
> absolute into the main checkout, which an isolated child reads as it writes its
> journal.

### (3) A resumed isolated dispatch is a new dispatch, and the child checks its own cwd

Shape (b) is the ruling, with shape (c) as its child-side clause. **{design-bearing}**
Shape (a), the guard evaluating D2 on a resume, is **unreachable**. A resume is a
`SendMessage` call, and its `PreToolUse` payload carries the recipient and the
message, never the recipient's agent type or its isolation. The guard keeps no
record of an earlier dispatch to join them against, and the `Agent` payload it
sees at dispatch carries a `tool_use_id` but not the agent id a resume
addresses. A join would need a second, post-dispatch hook that writes a map from
agent id to dispatch shape, plus a state file, all for one class. That is declined
as the heavier shape, whatever the post-dispatch payload turns out to carry. What
that payload carries is unmeasured here, and the ruling does not rest on it.

**Re-measured at authoring, 2026-09-21, on HEAD 146d9336.** An `audit-sweep` was
dispatched under `isolation: worktree` to run `pwd`, `git rev-parse
--show-toplevel` and `git worktree list`. It reported its own
`.claude/worktrees/agent-<id>` tree, listed as locked. After it handed back, a
`SendMessage` resume ran the same three commands. `pwd` and the toplevel were the
**main checkout**, `git worktree list` showed the main checkout alone, and no
`worktree-agent-*` branch remained. So the resume re-enters a read-only type with
write reach on the shared branch, and the queue entry's premise holds at HEAD.

**Template, dispatcher half.** In the **A finished child is addressed by the
task-id its completion notification carried, never by its name** bullet, after
its first sentence — **Not yet applied**:

> A child dispatched under isolation comes back without it: the resume runs in
> the main checkout, with its worktree gone. Resume an isolated child only to
> have it re-emit what it already holds, as isolation cost (5)'s recovery does;
> work that needs the tree again is a new isolated dispatch.

**Template, child half**, appended to isolation cost (3)'s child-side clause —
**Not yet applied**:

> And one more check on the child's side, at every resume as well as at the
> start: dispatched under isolation, confirm that your top level is a linked
> worktree (`git rev-parse --git-dir` differs from `git rev-parse
> --git-common-dir`). If it is not, your isolation is gone: stop, say so, and
> return without running the work.

The template header's propagate list grows from three passages to four, adding
this clause: "Four passages below — the **Background + notification, never
poll** and **Findings you will act on are durable before you act on them**
bullets, and the child-side clauses of isolation cost **(3)**, the resume check
included —". The clause binds a dispatched read-only role that fires no trigger
loading the template, so it qualifies for §Operative residency on the same (a)–(c)
reading that section records for cost (3)'s blindness clause.

**Consumer side (this repository, not kit mechanism).**
`.claude/agents/audit-sweep.md` is this consumer's carrier of cost (3)'s child
clause. It takes the resume check beside that clause as a bare imperative, citing
the template's isolation cost (3), sized to that file's voice.

**Grounds**, a new paragraph in §The delegation model after the D2 composition
paragraphs — **Not yet applied**:

> **D2 is evaluated once, at dispatch, and a resume does not re-enter it.** D2
> holds a read-only claim to isolation because isolation makes the claim. A
> `SendMessage` resume of an isolated read-only child runs in the main checkout,
> its worktree gone, with the write reach D2 would have refused at dispatch.
> Nothing in the resume's `PreToolUse` payload names the recipient's type or
> isolation, and the dispatch payload carries no id to record, so the guard
> cannot see the resume. The template therefore treats a resumed isolated
> dispatch as a new dispatch for any work over the tree. It restricts the resume
> to re-emitting what the child holds, and has the child confirm its own linked
> worktree at every start. The child's check is a request, delivered where the
> child reads it (§Operative residency). What it removes is a resume running the
> work unconfined; nothing refuses a child that skips it.

## Producers and consumers

- **The prompt line (delta 1).** *Producer:* the dispatching session writing an
  isolated read-only prompt, which reads disposition (2) when it loads the
  template through the dispatch trigger. *Consumer:* the child, which reads its
  prompt. This repository's `DELEGATION_KIT_READONLY_TYPES` is `audit-sweep`
  (`scripts/delegation-config.knobs`), so the producer is reachable here.
- **The dispatcher clause (delta 2).** *Producer:* the dispatching session, as
  above. *Consumer:* the isolated child reading the named file. The file lands
  under `GATE_SDK_TMP_DIR`, which the consumer's scratch reset sweeps, so it
  needs no new reader. It is an observer's input and writes no `.run` record.
- **The resume rule and the child check (delta 3).** *Producers:* the dispatcher
  deciding whether to resume, and the child at start and at resume. *Consumers:*
  the child's own stop, and the dispatcher reading its report. Roster-holding
  readers of the template's header list: probe
  `grep -rn "Three passages\|three passages" --include=*.md .` matched the
  template header and a historical release-declaration bullet. The bullet is a
  shipped record and stays. Carriers of cost (3)'s child clause: probe
  `grep -rln "isolation cost" .claude/` matched `.claude/agents/audit-sweep.md`
  alone.
- **Citation reader.** `check-rule-citation` resolves
  `the template's **<name>** rule` citations in §The delegation model against the
  template's bullet lead-ins. No delta renames a lead-in, and the new §The
  delegation model paragraphs cite no template rule by that grammar.

## Existing sections updated

- `delegation-kit/templates/agent-execution.md` — disposition (2) (delta 1), cost (4)'s closing sentence (delta 2), the task-id bullet, cost (3)'s child clause and the header's propagate list (delta 3).
- delegation-kit/SPEC.md §The delegation model — the D3 paragraph's successor (delta 1), the cost (3) composition paragraph's closing sentence (delta 2), the D2 resume paragraph (delta 3).
- `.claude/agents/audit-sweep.md` — the resume check beside its cost (3) clause (delta 3).
- `.workflow/release-declarations.md` — one `## Behavior changes` bullet led by
  `**`delegation-kit/templates/agent-execution.md`**`: an isolated read-only prompt
  names its return channel; cost (4) gains a dispatcher half; a resumed isolated
  child is limited to re-emitting its report, and a child confirms its linked
  worktree at every start; the header's propagate list names four passages, so a
  consumer whose agent definitions restate cost (3)'s child clause propagates the
  resume check (deltas 1, 2 and 3).
- `docs/delegation-kit/SPEC.md` — the generated mirror, regenerated by its arm (all deltas).

## Retired spellings

- None — no delta renames a lead-in, a knob, a path or a tag; the header's "Three passages" becomes "Four passages", a count rather than a name.

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
