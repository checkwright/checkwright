---
title: Orchestration
nav_order: 3
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
supervisor — human or agent — can trust a completed unit of work without
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

## What is built, and what is roadmap

Checkwright is the verification layer **today** — a facilitator for
orchestration, and specifically the prerequisite for *unattended* orchestration
at scale, where no human reads each hop. It is not itself an orchestration
layer, and this page claims none of that ground.

The honesty is structural, not a disclaimer: Checkwright's own coordination
rungs are named in the open, and their state is checkable.

- **The iteration lead** — landed. A live session that dispatches an
  iteration's stage sessions as background agents and answers their batched
  escalations, so a blocked stage forwards its question and resumes in place
  instead of stopping cold. The orchestration protocol is owned by
  `lifecycle-kit/SPEC.md §templates/lead.md`.
- **`multi-operator-semantics`** — deferred, in the public queue: the
  contributor altitude — merge and conflict behavior for the lifecycle's
  single-writer state surfaces, the kits' team-readiness rung.

Within one iteration that is the extent of Checkwright's own coordination: a
lead dispatching its own verified stages. Across operators it still
coordinates nothing — it verifies, so that whatever does the coordinating can
be trusted to have coordinated correct work.

## Running an iteration under a lead

The lead is a role a session *becomes*, not a stage it runs. Start the
iteration with `/scope` in a live session; once its promotion commit lands, that
same session takes the lead role and dispatches the remaining stages — `/align`,
`/build`, `/validate`, `/close` — each as a background stage session running its
skill unchanged. The lead writes no lifecycle state of its own: every flip,
stamp, and commit stays in the stage session, so the history reads identically
whether or not a lead drove it. A blocked stage session forwards its question
and resumes in place rather than restarting cold, and anything outside its
ruling roster reaches the operator. Running no lead is equally valid — each
stage is then an ordinary skill invocation that surfaces to the operator
directly. The start sequence, the escalation shape, and the ruling-class
boundary are owned by `lifecycle-kit/SPEC.md §templates/lead.md`.

## Where to go next

- [Why Checkwright](methodology.md) — the delivery-methodology essay behind the
  mechanism.
- [The kits](index.md#the-kits) — one page per kit, in reading order.
