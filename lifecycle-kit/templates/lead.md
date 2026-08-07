The **iteration lead** — a live session that dispatches an iteration's stage
sessions and answers their escalations, so a blocked stage resumes in place
rather than restarting. It is **not a stage skill**: it invokes no
`enter-stage.sh`, stamps no evidence, moves no cursor, and joins no stage
roster. Its whole authority is *dispatch* and *answers* — and, at iteration open, the
operator's selection **directive**, relayed to scope verbatim, never authored.

The lead is optional. An iteration runs correctly with no lead — each stage is
an ordinary skill invocation that stops and surfaces to the user. The lead only
changes where a stage session's escalations land: at a live lead that can
rule and resume it, instead of at a cold restart.

**First step — record the session role.** Write `lead <id>` — `<id>` from
`bash lifecycle-kit/bin/session-id.sh` — to the session-role marker
(`CONTEXT_KIT_SESSION_ROLE_FILE`, default
`${GATE_SDK_TMP_DIR:-.tmp}/session-role`):

```bash
mkdir -p "${GATE_SDK_TMP_DIR:-.tmp}" && \
  echo "lead $(bash lifecycle-kit/bin/session-id.sh)" \
  > "${CONTEXT_KIT_SESSION_ROLE_FILE:-${GATE_SDK_TMP_DIR:-.tmp}/session-role}"
```

The session-context hook reads it on each re-fire and suppresses its
executor-facing steps for this session only — the id match scopes the signal,
so no other session inherits it and a stale marker self-invalidates
(context-kit/SPEC.md §The session-context hook owns the grammar and limits).
Skipping the step costs nothing but the suppression. If the Bash environment
lacks the harness session uuid, the id derivation's newest-transcript fallback
can mis-pick a just-finished subagent's id — verify before writing the marker
(lifecycle-kit/SPEC.md §bin/session-id.sh owns the limit).

## The lead model

The lead takes one of two postures; which one — and the model tier each
session rides — is standing dispatch policy (the ruling-config slot below):

- **Unified** — the scope session stays live as the iteration's lead once its
  promotion commit lands. One session holds both scope's judgment and the
  dispatch loop, so every orchestration turn rides whatever tier scope ran on.
- **Split** — a session on a cheaper routing tier takes the lead role first
  and dispatches scope itself as a stage session on the judgment tier, keeping
  that agent resumable afterward as the iteration's **intent oracle**. The
  lead's own turns are routing work — dispatch, result ingestion, budget
  verdicts — and stop paying judgment-tier prices. The oracle is a fresh
  dispatch under delegation-kit's **Never dispatch a fork to narrow a child**
  rule (`templates/agent-execution.md`), of which this posture is the named
  tier-split instance: the narrowing at stake here is the model tier, and a
  fork inherits the dispatcher's.

Under either posture the lead dispatches a stage session as a **background
agent** whose prompt is that stage's ordinary skill invocation (`/build`,
`/validate`, …); the stage session executes its stage skill unchanged. Every
lifecycle-state write — the entry stamp, commits, evidence — happens **in the
stage session**, never in the lead.

Dispatch mechanics are delegation-kit's, unchanged: dispatch in the background
with notification, honor the per-dispatch budget guard, and verify after any
agent commit. The guard blocks only on PAUSE (STALE and OK advise); a deliberate
override rides the `.claude/settings.local.json` env block — delegation-kit/SPEC.md
§The delegation model. Load `/agent-execution` for the protocol and follow it
there — it is not restated here.

**When the dispatched stage is the evidence-producing stage, the lead's verify
is a read of the committed evidence — never a re-run of the producer.** This is
the lifecycle instance of delegation-kit's no-producer-re-run rule
(delegation-kit/SPEC.md §Verify after every agent commit, which owns the generic
rule and why re-running is safe for work and unsafe for evidence); only a stage
roster knows which stage that is, which is why the instance lands here. Both
misroutes are attested and the harm splits between them: re-running the whole
gate battery writes nothing under the workflow directory, so it is inert on the
evidence and merely wasted; re-running the evidence producer, the manifest's
sole writer, mutates or duplicates the committed record. Read the manifest the
stage committed and judge it.

**Stage N+1 is dispatched on stage N's agent completion notification — never on
its commit, its stamp, a clean tree, a green battery, or a cleared
`--simulate`.** Completion is a fact about a *session*; every one of those others
is a fact about an *artifact*, and no artifact distinguishes "finished" from
"still writing". The attested failure is exactly that gap: a lead confirmed the
validate stage's commit had landed with complete evidence, the tree clean, the
battery green and a simulated close entry cleared — then dispatched close into a
still-running `run-validate`. Every check passed mid-write, because the producer
commits its evidence and keeps going, so the terminal commit existing is fully
compatible with the process still executing.

**Any prompt-answered signal is a start signal, never a completion one.** The
same lead read an operator's note about having just answered a stalled permission
prompt as the stage being finished. It means the opposite: an approval prompt
gates a command **starting**, so such a note timestamps a beginning. Stated here
rather than left as incident lore, because the misreading is available to any
lead on any harness that prompts, and it reads as good news at exactly the moment
the lead wants good news.

The lead never hand-derives prior-stage completeness — reading WORKFLOW-STATE
or the git log to decide whether a dispatch may proceed re-derives what the
machinery already rules on. It dispatches and trusts `enter-stage.sh`'s
fail-closed refusal (relayed in the stage session's report), or gates an
expensive dispatch cheaply first with `enter-stage.sh --simulate <stage>`
(lifecycle-kit/SPEC.md §bin/enter-stage.sh) — oracle-first made concrete.

**That rule and the precondition above answer different questions, and reading
them as one makes the pair unusable.** This one is a *gating* rule: *may stage
N+1 proceed?* — a question about preconditions the machinery owns, which
`--simulate` answers correctly and cheaply and which the lead must not re-derive
by hand. The precondition is a *liveness* rule: *is stage N over?* — which no
instantaneous read answers at all. The incident is the proof they are two
questions rather than one stated twice: the lead followed this rule faithfully,
ran `--simulate`, and the simulated entry cleared mid-write. A rule told to trust
an instantaneous read is not at fault for failing to answer a question about
duration; it was never asked. `--simulate` keeps its whole job. What it never
was, and must now be said not to be, is evidence that the prior stage is over.

One qualification, because `--simulate` runs every matching entry-preflight
command: where a consumer wires a producer-liveness gate onto that hook,
`--simulate` inherits it and the specific mid-write clearing above stops
happening. That is a real narrowing rather than a repeal — an instantaneous read
is still instantaneous, so a producer starting a moment later is still unseen,
and the gate sees only producers that claim a lock. Completion stays a fact about
a session; what the gate contributes is to shrink the window in which being wrong
about it goes undetected.

**Relay, never assert.** The lead manages on *optimal* rather than extensive
context, so on any topic it has not mastered it acknowledges and relays — it
does not hand down a tree fact as a ruling. The asymmetry is the reason: a
stage session writes lifecycle state and is held to oracle-first, fixture
pairs and a validate battery, while the lead writes no state and so has no
verification discipline, yet its rulings steer what stage sessions land. A
claim reasoned from one narrow grep and delivered as instruction spends a
stage session's work against it, and nothing reds. So a factual claim travels
as a claim with its provenance attached ("read off X, unverified"), leaving
the stage session — which holds the oracle — to run it. What the lead rules
alone is scope, envelope and priority: the things no gate can decide and no
grep can answer.

Whether the lead may ever run a stage *inline* is the consumer's
session-boundary posture (`LIFECYCLE_KIT_SESSION_BOUNDARY`,
lifecycle-kit/SPEC.md §Layout and configuration). Under the strict posture
(`stage`) the lead may never run a stage inline for an iteration it already
stamped — its session id is spent, and an inline run would be exactly the
self-reported skip `check-stage-evidence` exists to catch. Under the relaxed
posture (`iteration`) an inline stage run is the sanctioned fallback when
dispatch is blocked (e.g. the budget guard), the stamp recording the shared id
honestly.

## Opening an iteration

The lead never selects the iteration's unit set. Selection is the **scope**
stage's contract — the survey, the intake boundary sweep, and the
re-verification of each queued premise against the current tree are scope's
first job, and a lead-made list pre-empts exactly the half that catches a stale
premise. The designed path runs end to end already: scope surveys and proposes,
a queue change escalates, the operator rules.

Before dispatching scope, the lead obtains the operator's **standing directive**
for the iteration — a theme bounding scope's survey, never a slug list — and
passes it in the scope dispatch prompt **verbatim**. The directive varies per
iteration, so per the policy-is-config rule it rides the dispatch prompt, not the
agent definition. Absent a directive, the lead dispatches scope undirected: scope
surveys and recommends either way.

Scope's proposed unit set returns as an ordinary four-header escalation, and the
lead routes it like any scope/queue change — ruled by the operator, or by the
lead only where the answer is derivable from the governed surfaces (the routing
rule the escalation protocol below already states).

The anti-pattern, named: a lead-authored menu restates the operator's own queue
view from staler data, costs a round trip, and skips the premise re-verification
that has already caught a false filed premise in practice.

## The escalation protocol

A stage session that hits a question inside its ruling classes ends its turn
with a **decision-shaped escalation block** and batches every open question into
that one turn-end rather than forwarding singly. The block's four headers, each
read by the lead at its answer transition:

- **Question** — what is blocked.
- **Options** — the choice set.
- **Recommendation** — the default the lead can rubber-stamp.
- **Evidence** — what the session already verified.

The lead answers by messaging the paused session, which resumes in place with
its working state intact. That resume — not a cold restart — is the cost
asymmetry the lead exists to close.

Under the split posture the lead **routes** an escalation before answering it.
A question the iteration's machinery already governs — which fixture, which
surface, ordering, a helper's shape — the lead rules itself. A question about
the iteration's *intent* — a scope boundary, an amendment's envelope, a seam
ruling — is forwarded to the intent oracle and the oracle's answer relayed
back. A forwarded question carries the excerpt of current working state it
turns on: the oracle holds the iteration's intent, not the build's unfolding,
so the lead supplies what the question needs read. When the oracle cannot be
resumed, the lead answers from the governed surfaces the rulings already live
in — the amendments, the queue entries — and a question not derivable there
goes to the operator, never substituted by the lead's own judgment.

**One class the lead never rules, under either posture.** Reversing, demoting
or re-scoping a **recorded operator ruling** or a stated objective is
operator-class: the lead relays it, however well-grounded the escalating
session's finding and however urgent the fix. It is carved out precisely
because it reads as derivable — the ruling is written in a governed surface,
so a session holding contrary evidence takes that surface for stale rather than
for closed, and the routing rule above then hands the lead a decision it may not
make. The escalating session is right to escalate and right about its evidence;
what it may spend the evidence on is the operator's reconsideration, never the
reversal itself.

## Channel design

Two channels, each with one job. Routine narration and findings go to the
**resume journal** (a pull channel — delegation-kit's journal mechanics own
the rest). The **message channel** carries only the escalation classes. This
is how verbosity is controlled: by channel design, not by asking a session to
be quiet.

A pull channel is read, never consumed: the lead **does not delete** a stage
session's journal, at the post-commit checkpoint or anywhere else. A resumable
session is still writing to it, and deleting it is how a lead destroys the
channel it is meant to keep reading for the rest of the unit. It is swept with
the rest of the scratch dir at the iteration boundary
(delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps).

## Policy is config, not prose

All *standing* dispatch policy — everything true of every dispatch, not the
ruling-class roster alone — lives in the tracked agent-definition the dispatch
names, never in ad-hoc per-dispatch instructions, so there is one gated source
of the policy rather than a second one improvised per dispatch. Journal
mechanics, environment wiring, the shared-index caution, the escalation shape,
and the escalate-versus-decide roster are all standing; a dispatch prompt
carries only what varies per dispatch — the stage skill to invoke, the batch's
task slugs, and pointers specific to that batch. The tell that content is
misplaced: the same sentence appearing in two dispatch prompts. The
agent-definition points at its owning docs rather than restating them
(content-tiering) — it cites delegation-kit's resume-journal mechanics, say,
never transcribes them.

The rule cuts both ways: policy binding the **lead itself** is standing too, and
its tracked source is this template rather than the agent definition it
dispatches. The completion-notification dispatch precondition (§The lead model)
is standing policy of exactly that kind, named here so it is stated once and not
re-improvised in each dispatch prompt.

*<ruling-config: the tracked agent-definition the lead dispatches and the roster
it carries — its path, the subagent type the dispatch names, and where the
ruling classes are stated.>*

## Stamps are authoritative (the load-bearing invariant)

The lead writes **no** lifecycle state — no WORKFLOW-STATE stamps, no queue
writes, no evidence files. Every stamp originates in the stage session via
`enter-stage.sh` (lifecycle-kit/SPEC.md §The state machine). Lead-does-stamping
is ruled out, not merely omitted: it breaks this invariant, and under the
`stage` posture of `LIFECYCLE_KIT_SESSION_BOUNDARY` a lead stamp is exactly the
self-reported skip `check-stage-evidence` exists to catch.

**The lead stamping nothing is not the batch stamping nothing**, and the two
read alike from here. A dispatched batch stamps on entry like any stage session:
an intra-stage split makes the second batch a *session*, not a re-entry to
suppress, and a sibling stamp naming a stage the cursor already sits on moves
nothing — the stage skill owns that rule and its gate tolerance
(lifecycle-kit/templates/stages/build.md). Directing a batch not to stamp is
therefore not conflict-avoidance; it silently spends the per-session audit trail
the stamp exists to provide, and no later session can repair it, because
backdating a stamp falsifies the trail rather than restoring it.

An answer that
amounts to a design ruling is landed **by the stage session**, in the governed
surface it belongs to (the amendment, the queue entry), *before* the session
acts on it — and a ruling whose acting session is **not imminent** is filed to
a durable governed surface (a queue entry, an amendment) in the moment it is
made, because "the stage session lands it" holds only when that session
exists. The message thread is transport, never a store — so a lead crash or
a lost transcript costs nothing the tracked surfaces do not already hold.

## Economics — batch, and compact where it pays

The prompt cache's short TTL means a sporadically questioned lead pays a full
context re-warm on each cold question: stage sessions outlive the TTL and
escalations arrive on their schedule, so the lead is nearly always cold. Two
consequences:

- **Batch dispatches by shared surface.** Both naive granularities are ruled
  out: whole-queue-in-one rides past every split trigger the delegation protocol
  names, and one-dispatch-per-task pays context setup times N while buying no
  parallelism (committing agents serialize on the shared git index regardless).
  Batch units that share a kit or SPEC surface into one dispatch, where derived
  context is actually common; split where the model tier changes or a
  delegation-kit split trigger fires — per-batch model tiering is the dominant
  window lever, not token counts.
  The set being batched is **every unit the iteration promoted** — every
  top-level entry in the configured active queue sections, debt units as much as
  feature units — never the amendment set. A debt unit converges an
  implementation on existing spec and mints no governed name, so it carries no
  `[spec:]` ref; keying the roster on amendments makes exactly the
  amendment-free half of the queue invisible, and it disappears silently because
  the units that vanish are the ones that left no artifact to miss. Derive the
  roster from the queue, which is the record of what was promoted, and re-read
  it rather than carrying a count.
- **Tier each batch to its work class.** The lead reads the **work-class**
  labels of the deltas in a batch — via the `[spec:]` amendments the batch's
  entries point at, where `/spec` emits one `{mechanical | design-bearing}` tag
  per delta — and tiers the batch by their aggregate. A batch whose deltas are
  **all mechanical** (oracle-running: a fixed verification battery, a
  rename/merge sweep, a mechanical pin — low generative judgment) is
  **tier-downgradeable**: a cheaper model serves it, and the dispatcher pins the
  cheaper tier with a `model` override on that batch's dispatch. A batch carrying
  **any design-bearing** delta (generative or verificational judgment — bounding
  a unit set, authoring an amendment, cross-spec audit, non-obvious
  implementation) **stays on the judgment tier**: the judgment is exactly what
  the tier buys, and downgrading it trades a large correctness risk for a small
  window saving. Class → live model is mapped at dispatch time, where the roster
  dependency already belongs (agent-execution.md, same bullet). There is no
  standing per-stage classification to bind: the batch's labels decide at
  dispatch time, so a stage-uniform class (validate, uniformly mechanical) is a
  **collapsed default**, not a bound roster — while a stage whose batches diverge
  (build — a one-line hermeticity pin beside a new KPI plugin) tiers each batch
  from its deltas' labels, which a per-stage rule could not express. Re-judge
  every assignment when the harness model roster churns.
- **An intra-stage batch split is the lead's to own.** When a stage's work
  splits into batches, those batches are **N sibling stage sessions the lead
  dispatches and verifies** — each entering through `enter-stage.sh` as a
  same-stage re-entry (lifecycle-kit/SPEC.md §The state machine: N sessions may
  enter one stage, each leaving its own stamp and the cursor staying put). A stage session **never dispatches a sibling stage session**: a stage
  that sub-dispatches its own batches nests a second supervisor at the lead's
  tier, hidden from its budget and context accounting — the redundancy the split
  posture exists to remove, and the clause is a dispatched stage's authority to
  refuse. Read-only fan-outs inside a stage stay sanctioned (the delegation
  nudge; CLAUDE.md §Agent execution). The batching *criteria* are the
  shared-surface rule above, unchanged; this adds only the owner — the lead
  serializes sibling batches that share a surface and may parallelize those that
  do not, subject to the shared-index discipline.
- **Batch escalations.** The decision shape makes batching natural — a stage
  session collects its open questions and sends them in one turn.
- **Split the lead where the tail dominates.** Most of a lead's turns are
  tail — dispatch, result ingestion, budget verdicts — and under the unified
  posture each one re-reads a cache carrying scope's whole working context at
  judgment-tier prices. The split posture moves that recurring cost to the
  routing tier and concentrates judgment-tier spend where it pays: scope
  itself, plus one oracle turn per forwarded intent question — a cold context
  re-warm at worst, bounded and per-question instead of per-turn. Assume the
  oracle is cold: escalations arrive on the stages' schedule, so the TTL
  arithmetic that leaves a lead nearly always cold (above) applies to the
  oracle unchanged.
- **Compact at handoff (unified posture).** After the promotion commit lands the amendments and
  queue entries, and before the first dispatch, `/compact` the lead's context
  with an instruction that **keeps** per-amendment rationale, rejected
  alternatives,
  and the ruling-class roster, and **drops** tool output and file contents. The
  tree is re-readable and everything ruled already lives in a governed surface,
  so the lossy compact has a lead crash's bounded blast radius; the lead holds
  pointers, not state. Verify the spend afterward with delegation-kit's
  usage-verdict rather than assuming forgiveness.
- **Suggest a compact at the paying acceptance boundaries.** After a stage
  session's work is accepted — its commits verified, its rulings landed in
  governed surfaces — and before the next dispatch, the lead *suggests* a
  compact to the operator. Compaction is operator-invoked; the lead can only
  recommend (the honest limit), so the suggestion is one line in the lead's
  acceptance message, never a new mechanism. Suggest where it pays, not
  blanket: a compact pays when the remaining cold wakes times the compressible
  residue exceed one context re-read — the early acceptance boundaries, with
  the most residue accreted and the most wakes still ahead, pay, while the
  late ones do not warrant the operator interruption. This is the rule, not a
  stage roster; a consumer derives its own paying boundaries from its stage
  set. The keep-instruction is the handoff bullet's, unchanged.

Cache-keepalive pinging is ruled out: at batched escalation rates the idle
re-warm pings cost more than the cold reads they avoid, burn the shared budget
window, and invite idle-turn drift.

## Mechanical floor

Prompts request; guards enforce. The four-header escalation shape has an
optional guard-kit mechanical floor — a SendMessage guard registered in the
stage session that advises when an outbound escalation to the lead lacks the
decision-shape headers (guard-kit/SPEC.md §wakeup-guard). The header grammar is
kit mechanism; the ruling-class roster stays consumer config (above).

*<escalation-guard: whether this consumer wires the optional guard-kit
SendMessage guard for its stage sessions, or leaves it inert — the mechanical
floor and where its opt-in lives.>*

The floor under the batching-roster rule is `check-stage-entry`'s drain-entry
assertion: the drain stage refuses to be entered while the active queue still
carries entries, so a dropped unit surfaces as a **refused entry** at the next
stage rather than never. That is a backstop, not the working signal — it costs
a dispatch to learn from. Read it early and cheaply instead: the `--simulate`
read §The lead model already names for gating an expensive dispatch, aimed at
the drain stage, returns the same verdict with no session spent. Run it
**before** declaring a stage's batches complete, not after — a simulate run
that follows the declaration confirms the drop instead of preventing it, and a
roster has already lost a promoted unit in practice. Keying the roster on the
wrong set is only one way to lose a unit: a plain miscount against the queue
loses one just as quietly, and neither failure reddens on its own. That is why
the rule is to re-read the queue rather than to key it differently.

The floor's honest limit belongs with it: it fires on the queue's *residue*, so
it catches a dropped unit and says nothing about a unit batched onto the wrong
surface or tiered wrongly. Those stay prompt-side — the ordinary
prompts-request/guards-enforce split rather than a gap.
