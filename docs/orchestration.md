---
title: Orchestration
nav_parent: methodology
nav_child_order: 1
---

# Agent orchestration: the verification layer beneath it

Agent orchestration frameworks answer *who works on what, and when*. They fan a
job out across agents, sequence the hand-offs, and keep many writers moving at
once. What they do not answer is *whether the work is right*. Every delegation
hop is a place where "the agent said it passed" can quietly stand in for
verification, and drift compounds per hop — so orchestration without a
verification layer parallelizes drift: wrong answers faster, with more
confidence.

Checkwright is that second answer. It makes *done* mechanically decidable, so a
supervisor (human or agent) can trust a completed unit of work without
reading all of it. That is the prerequisite the orchestration story quietly
assumes: coordination is only worth scaling once each coordinated result is
checkable. Checkwright therefore sits **beneath** an orchestrator, not beside
it — the verification layer a coordination layer stacks on. It complements your
orchestration setup; it does not replace it.

This page positions Checkwright as that verification layer. It owns no
contract — each mechanism below is owned by the kit that enforces it, cited
downward so the invariant stays in one place.

## The mechanisms

- **Verification survives the adversary.** A delegated agent runs the full gate
  battery after every commit it makes, and a dedicated gate blocks any commit
  that weakens the gates themselves — so an agent cannot silently disarm the
  checks that judge it, whether by accident or to make its own work pass. Owned
  by `delegation-kit/SPEC.md §Validate after every agent commit`.
- **Every dispatch is metered.** A per-dispatch budget guard sits in front of
  each delegated agent, so fanning work out cannot silently run away with the
  token budget — the orchestrator gets a mechanical stop, not a surprise bill.
  Owned by `delegation-kit/SPEC.md §The delegation model`.
- **Stage claims are auditable evidence, not assertions.** Each lifecycle stage
  stamps a line into a committed state file when it runs, so a stateless
  supervisor can audit *what happened* by reading the stamps rather than
  re-reading the work. Owned by `lifecycle-kit/SPEC.md §The state machine`.
- **Every run leaves a committed receipt.** A per-run evidence manifest is
  committed alongside the work, so the claim "this passed" is a durable
  artifact in the history, checkable after the fact by anyone. Owned by
  `evidence-kit/SPEC.md §Evidence manifest`.

Together these make the delegation boundary trustworthy: work crosses it with
a checkable receipt, not a promise.

## Checkwright's own coordination

Checkwright is a verification layer, not an orchestration layer — the
prerequisite for *unattended* orchestration at scale, where no human reads each
hop, and this page claims none of the coordination ground itself. What it does
carry is a small, checkable amount of its *own* coordination, named in the open:

- **The iteration lead** — a live session that dispatches an iteration's stage
  sessions as background agents and answers their batched escalations, so a
  blocked stage forwards its question and resumes in place instead of stopping
  cold. The protocol is owned by `lifecycle-kit/SPEC.md §templates/lead.md`.
- **Multi-operator semantics** — the contributor altitude: merge and conflict
  behavior for the lifecycle's single-writer state surfaces. An iteration owns
  one branch, the iteration-scoped surfaces resolve to the arriving branch at a
  merge (a derived `merge=iteration-scoped` driver set plus a parity gate), and
  a close-merge serializes boundaries on the integration branch. Owned by
  `lifecycle-kit/SPEC.md §Multi-operator semantics`.

Within one iteration that is the extent of it: a lead dispatching its own
verified stages. Across operators Checkwright still coordinates nothing — it
verifies, so that whatever does the coordinating can be trusted to have
coordinated correct work. Where the coordination layer goes next is the roadmap,
and the roadmap is not narrated here: the queue's Deferred section is that
surface, checkable in the tree rather than restated on a page that would age the
moment it was written.

## Running an iteration under a lead

The lead is a role a session *becomes*, not a stage it runs — and running no
lead is equally valid. The recommended shape is the **split posture**: the
lead rides a cheap routing tier and only the judgment-heavy work pays premium
prices. To drive an iteration under it:

1. **`/lead` in a fresh session on the routing tier.** The lead role writes no
   lifecycle state of its own; every flip, stamp, and commit stays in the stage
   sessions it dispatches, so the history reads identically whether or not a
   lead drove it.
2. **The lead dispatches `/scope` as a background stage session on the
   judgment tier.** Scope formalizes the iteration — authoring the design
   amendments, promoting the queue entries — and lands the promotion commit. Its
   agent stays resumable afterward as the iteration's **intent oracle**.
3. **The lead dispatches each remaining stage** — `/align`, `/build`,
   `/validate`, `/close` — as a background stage session that runs its skill
   unchanged, on the tier the stage-session agent definition pins.
4. **A blocked stage escalates to the lead and resumes in place** rather than
   restarting cold. The lead rules machinery questions itself; a question
   about the iteration's *intent* it forwards — with the working-state excerpt
   the question turns on — to the scope oracle and relays the answer back.
   Anything outside the ruling roster reaches the operator.
5. **If the oracle cannot be resumed,** the lead answers from the governed
   surfaces the rulings already live in — the amendments, the queue entries —
   and hands anything not derivable there to the operator.

The earlier **unified posture** — `/scope` in a live session, `/compact`, then
`/lead` in that same session — remains valid and simpler when tiering is not a
concern; its trade-off is that every orchestration turn re-reads scope's
context at scope's prices. Prefer to stay hands-on? Skip the lead and run each
stage manually as an ordinary skill invocation, consulting the scope session
when a stage raises a question. The posture definitions, the compaction
instruction, the escalation shape, and the ruling-class boundary are owned by
`lifecycle-kit/SPEC.md §templates/lead.md`.

## Two operators, one governed tree

A lead drives one iteration. When a second operator wants to work the same repo
at the same time, the coordination is git topology, not a new mechanism — every
rule below is owned by `lifecycle-kit/SPEC.md §Multi-operator semantics`, and
this walkthrough only tells the story.

Picture two iterations live at once — say *surface-trust* and *docs-polish*.

1. **Each iteration owns one branch.** The first operator's iteration lives on
   its home branch; the second operator cuts a branch at their own `/scope`
   entry. The integration branch is just the degenerate single-operator home,
   which is why a solo repo's own dogfooding needs none of this. Every flip and
   stamp an iteration makes lands on its own branch — the state surfaces stay
   single-writer even while two operators move.
2. **Each branch stamps its own state.** `surface-trust`'s stage sessions flip
   its header and append its evidence on its branch; `docs-polish` does the same
   on its. Neither reads the other's state, so there is no shared file to race
   on — operator attribution rides the git author on each flip commit, no new
   stamp grammar required.
3. **At a merge, the iteration-scoped surfaces resolve to the arriving side.**
   When one branch merges into another, the header line, the evidence file, and
   the other boundary-truncated surfaces take the checked-out iteration's
   version wholesale — the other side's copy is per-iteration scratch the
   boundary doctrine already declares dead (git history keeps the audit trail).
   The set is *derived* from what the iteration boundary already truncates, and
   mechanized as a `merge=iteration-scoped` driver plus a parity gate, so it
   never drifts from a hand-maintained list. Shared backlog and lessons in the
   queue body merge like any prose; only its one header line is resolved by
   hand — and a wrong resolution reddens at the next commit, because the stage
   evidence gate demands header↔stamp agreement.
4. **Closes serialize at the integration branch.** Each operator closes on their
   own iteration branch, reconciling by merging the integration branch in, where
   the driver resolves the state surfaces to their arriving iteration; the
   close-merges land one after another on the integration branch. No lock, no
   lease — the serialization is the merge order git already imposes.

The through-line: concurrency is isolation git already provides, and
verification is unchanged on every branch. Whatever an operator's branch merges,
it merged a tree that passed the battery — so two operators parallelize
*checked* work, never drift.

## Where to go next

- [Why Checkwright](methodology.md) — the delivery-methodology essay behind the
  mechanism.
- [The kits](index.md#the-kits) — one page per kit, in reading order.
