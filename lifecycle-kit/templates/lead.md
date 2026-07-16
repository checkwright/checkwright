The **iteration lead** — a live session that dispatches an iteration's stage
sessions and answers their escalations, so a blocked stage resumes in place
rather than restarting. It is **not a stage skill**: it invokes no
`enter-stage.sh`, stamps no evidence, flips no header, and joins no stage
roster. Its whole authority is *dispatch* and *answers*.

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
Skipping the step costs nothing but the suppression.

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
  verdicts — and stop paying judgment-tier prices. The oracle must be a fresh
  dispatch, never a fork: a fork inherits the dispatcher's model, which is
  exactly the tier split this posture exists to make.

Under either posture the lead dispatches a stage session as a **background
agent** whose prompt is that stage's ordinary skill invocation (`/build`,
`/validate`, …); the stage session executes its stage skill unchanged. Every
lifecycle-state write — the flip+stamp, commits, evidence — happens **in the
stage session**, never in the lead.

Dispatch mechanics are delegation-kit's, unchanged: dispatch in the background
with notification, honor the per-dispatch budget guard, and validate after any
agent commit. Load `/agent-execution` for the protocol and follow it there — it
is not restated here.

The lead never hand-derives prior-stage completeness — reading WORKFLOW-STATE
or the git log to decide whether a dispatch may proceed re-derives what the
machinery already rules on. It dispatches and trusts `enter-stage.sh`'s
fail-closed refusal (relayed in the stage session's report), or gates an
expensive dispatch cheaply first with `enter-stage.sh --simulate <stage>`
(lifecycle-kit/SPEC.md §bin/enter-stage.sh) — oracle-first made concrete.

Whether the lead may ever run a stage *inline* is the consumer's
session-boundary posture (`LIFECYCLE_KIT_SESSION_BOUNDARY`,
lifecycle-kit/SPEC.md §Layout and configuration). Under the strict posture
(`stage`) the lead may never run a stage inline for an iteration it already
stamped — its session id is spent, and an inline run would be exactly the
self-reported skip `check-stage-evidence` exists to catch. Under the relaxed
posture (`iteration`) an inline stage run is the sanctioned fallback when
dispatch is blocked (e.g. the budget guard), the stamp recording the shared id
honestly.

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

## Channel design

Two channels, each with one job. Routine narration and findings go to the
**resume journal** (a pull channel — delegation-kit's journal mechanics apply
unchanged). The **message channel** carries only the escalation classes. This
is how verbosity is controlled: by channel design, not by asking a session to
be quiet.

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

*<ruling-config: the tracked agent-definition the lead dispatches and the roster
it carries — its path, the subagent type the dispatch names, and where the
ruling classes are stated.>*

## Stamps are authoritative (the load-bearing invariant)

The lead writes **no** lifecycle state — no WORKFLOW-STATE stamps, no queue
header flips, no evidence files. Every stamp originates in the stage session via
`enter-stage.sh` (lifecycle-kit/SPEC.md §The state machine). Lead-does-stamping
is ruled out, not merely omitted: it breaks this invariant, and under the
`stage` posture of `LIFECYCLE_KIT_SESSION_BOUNDARY` a lead stamp is exactly the
self-reported skip `check-stage-evidence` exists to catch. An answer that
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
  session's work is accepted — its commits validated, its rulings landed in
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

## Mechanical floor — the escalation-shape guard

Prompts request; guards enforce. The four-header escalation shape has an
optional guard-kit mechanical floor — a SendMessage guard registered in the
stage session that advises when an outbound escalation to the lead lacks the
decision-shape headers (guard-kit/SPEC.md §wakeup-guard). The header grammar is
kit mechanism; the ruling-class roster stays consumer config (above).

*<escalation-guard: whether this consumer wires the optional guard-kit
SendMessage guard for its stage sessions, or leaves it inert — the mechanical
floor and where its opt-in lives.>*
