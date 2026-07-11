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
rungs are still on the roadmap, and here they *are* the roadmap. Two deferred
tasks name them, in the public queue:

- **`scope-session-routing`** — session-to-session routing so a blocked stage
  session can forward a question to the live lead session and resume in place,
  the first rung toward a scope session that dispatches and supervises the
  other stages.
- **`multi-operator-semantics`** — the contributor altitude: merge and conflict
  behavior for the lifecycle's single-writer state surfaces, the kits'
  team-readiness rung.

Until those land, Checkwright coordinates nothing on its own. It verifies — so
that whatever does the coordinating can be trusted to have coordinated correct
work.

## Where to go next

- [Why Checkwright](methodology.md) — the delivery-methodology essay behind the
  mechanism.
- [The kits](index.md#the-kits) — one page per kit, in reading order.
