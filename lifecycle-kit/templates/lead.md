The **iteration lead** — a live session that dispatches an iteration's stage
sessions and answers their escalations, so a blocked stage resumes in place
rather than restarting. It is **not a stage skill**: it invokes no
`enter-stage.sh`, stamps no evidence, flips no header, and joins no stage
roster. Its whole authority is *dispatch* and *answers*.

The lead is optional. An iteration runs correctly with no lead — each stage is
an ordinary skill invocation that stops and surfaces to the user. The lead only
changes where a stage session's escalations land: at a live lead that can
rule and resume it, instead of at a cold restart.

## The lead model

A scope session may stay live as the iteration's lead once its promotion commit
lands. It dispatches a stage session as a **background agent** whose prompt is
that stage's ordinary skill invocation (`/build`, `/validate`, …); the stage
session executes its stage skill unchanged. Every lifecycle-state write — the
flip+stamp, commits, evidence — happens **in the stage session**, never in the
lead.

Dispatch mechanics are delegation-kit's, unchanged: dispatch in the background
with notification, honor the per-dispatch budget guard, and validate after any
agent commit. Load `/agent-execution` for the protocol and follow it there — it
is not restated here.

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

## Channel design

Two channels, each with one job. Routine narration and findings go to the
**resume journal** (a pull channel — delegation-kit's journal mechanics apply
unchanged). The **message channel** carries only the escalation classes. This
is how verbosity is controlled: by channel design, not by asking a session to
be quiet.

## Policy is config, not prose

The ruling-class roster — what a stage session must escalate versus what it
decides alone — lives in the tracked agent-definition the dispatch names, never
in ad-hoc per-dispatch instructions, so there is one gated source of the policy
rather than a second one improvised per dispatch.

*<ruling-config: the tracked agent-definition the lead dispatches and the roster
it carries — its path, the subagent type the dispatch names, and where the
ruling classes are stated.>*

## Stamps are authoritative (the load-bearing invariant)

The lead writes **no** lifecycle state — no WORKFLOW-STATE stamps, no queue
header flips, no evidence files. Every stamp originates in the stage session via
`enter-stage.sh` (lifecycle-kit/SPEC.md §The state machine). An answer that
amounts to a design ruling is landed **by the stage session**, in the governed
surface it belongs to (the amendment, the queue entry), *before* the session
acts on it. The message thread is transport, never a store — so a lead crash or
a lost transcript costs nothing the tracked surfaces do not already hold.

## Economics — batch, and compact at handoff

The prompt cache's short TTL means a sporadically questioned lead pays a full
context re-warm on each cold question: stage sessions outlive the TTL and
escalations arrive on their schedule, so the lead is nearly always cold. Two
consequences:

- **Batch escalations.** The decision shape makes batching natural — a stage
  session collects its open questions and sends them in one turn.
- **Compact at handoff.** After the promotion commit lands the amendments and
  queue entries, and before the first dispatch, `/compact` the lead's context
  with an instruction that **keeps** per-amendment rationale, rejected
  alternatives,
  and the ruling-class roster, and **drops** tool output and file contents. The
  tree is re-readable and everything ruled already lives in a governed surface,
  so the lossy compact has a lead crash's bounded blast radius; the lead holds
  pointers, not state. Verify the spend afterward with delegation-kit's
  usage-verdict rather than assuming forgiveness.

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
