# DOCTRINE.md — the Checkwright delivery doctrine

The cross-kit delivery rules the kits enforce piecemeal, stated once. This file
is the deliverable of [doctrine-kit](README.md): referenced in place from a
consumer's always-loaded agent file, never copy-installed — re-vendoring the kit
*is* the doctrine upgrade. Each rule lands as its statement, why it holds under
coding-agent work, and a pointer to the mechanism that enforces it; the
mechanism and its knob rosters live in the cited kit SPEC, never restated here.

The roster is in two registers. **Methodology-maintenance** rules govern how the
methodology's own surfaces stay honest; they bear on every surface edit and form
the always-loaded digest a consumer installs. **Engineering-craft** rules govern
how the work built under the methodology is written; they are load-triggered by
test and rename work and live behind the link — an application of the
load-trigger residency rule to the doctrine itself.

## Methodology-maintenance rules

1. **Content-tiering / SSOT.** Every governed surface owns exactly one content
   tier and *points to* — never restates — a fact another surface owns. A
   parallel copy is the defect; the fix is one shape: replace the slab with a
   pointer to the owner, then a gate forbids it growing back.
   *Under agent work:* a restated fact drifts the moment one copy is edited and
   the other is not, and an agent reading the stale copy inherits the drift as
   ground truth. One owner means one thing to keep true.
   *Enforced by:* the anti-restatement gate family — the comment-tier and
   manifest/prose gates in [canon-kit/SPEC.md](../canon-kit/SPEC.md).

2. **Enforcement-first.** On any fix or redundancy finding, name the defect
   class *and* the mechanism that catches it, and land both in one unit; a green
   instance fix is the stop signal to ask what check should have caught it. When
   the gate cannot land in the same unit, the instance fix rides the gate's unit
   rather than landing bare. The mechanism is bought against a budget — gate
   count, runtime, maintenance — so the strongest form is structural: remove the
   duplication or collapse the surface so the defect class cannot recur, and no
   gate is owed. No duplication with no gate outranks duplication with a gate;
   tolerated duplication is the exception and carries its defence (the amendment
   template's sanctioned-copy rule is the model). The caveat that keeps this
   from reading gate-averse: for a drift-prone surface that must exist, the
   gate is cheap insurance and is owed — the budget argument never keeps a
   needed gate out; it keeps removable surfaces from being kept because a gate
   could watch them. The defence that excuses a *missing* gate on such a
   surface is a high false-positive rate: a noisy check breeds exemptions and
   erodes the battery's authority, so the class a check cannot decide cleanly
   stays a stated manual duty rather than a noisy gate.
   *Under agent work:* an unenforced rule is a rule the next session cannot see;
   a gate is the only carrier of intent that survives a fresh context window.
   *Enforced by:* the meta-gate contracts every gate must satisfy —
   [gate-sdk/SPEC.md](../gate-sdk/SPEC.md).

3. **De-literalization.** Prose cites names; code or the owning SPEC owns values.
   Knob defaults, shared constants, and derivable rosters are stated once at
   their owner, and prose names the knob or the roster rather than its value.
   *Under agent work:* a literal copied into prose is a second source of a
   number, and an agent that trusts the nearer copy ships the stale one.
   *Enforced by:* the bare-cardinal gate in
   [canon-kit/SPEC.md](../canon-kit/SPEC.md) §check-manifest-count.

4. **Always-loaded shape.** A rule in the always-loaded agent file is one line —
   the convention plus a pointer; its mechanism, rosters, and defaults live
   behind the pointer in the owning doc.
   *Under agent work:* the always-loaded file is read in full at every session
   start, so every line spent restating a mechanism is context tax on every
   task; a one-line pointer pays once and loads the detail only when needed.
   *Enforced by:* the brevity budget in
   [context-kit/SPEC.md](../context-kit/SPEC.md) §The brevity gate.

5. **Load-trigger residency.** The always-loaded file earns a rule only when no
   stage, skill, or tool-call trigger exists to load it; anything a trigger can
   pull lives in its owned doc behind that trigger.
   *Under agent work:* always-loaded context is the scarcest budget; a rule that
   a stage or a tool call would load anyway costs nothing to defer and everything
   to keep resident.
   *Enforced by:* the stage/skill load-triggers in
   [lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) and the tool-call hook seam in
   [guard-kit/SPEC.md](../guard-kit/SPEC.md), which give every triggered rule a home
   that loads on demand.

6. **Widest-true-tier placement.** A fact lands at the widest tier where it holds
   for every reader of that tier: kit-shipped surface when true for every
   consumer, a consumer-tracked binding or config when it names this repo's own
   choices, the local-only private brief when it cannot be published. A
   template's binding-slot grammar is the seam marker between the first two.
   *Under agent work:* a fact placed too narrow is re-derived by every reader who
   needed it wider; placed too wide, it publishes a choice or a secret that was
   never the reader's to see.
   *Enforced by:* the binding-slot grammar in
   [lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) §check-skill-binding, the seam
   marker between kit-shipped and consumer-tracked tiers.

7. **Oracle-first.** A check's output is its interface: run the gate instead of
   emulating it, and treat a red run — including a red commit attempt, where the
   generated hook runs exactly the coupled subset — as the designed feedback
   channel, not an incident. Gate source is opened to fix the gate or write its
   fixtures, never to predict its verdict.
   *Under agent work:* a deterministic check costs seconds and zero context;
   reading its source to anticipate the verdict costs more than running the whole
   battery and can still be wrong.
   *Enforced by:* the gate output contract in
   [gate-sdk/SPEC.md](../gate-sdk/SPEC.md) — the `clean` and `help:` lines are
   written to be that interface — and the targeted-run resolver reachable through
   the generated hook.

8. **Spec-over-precedent.** A gate-enforced doc is ground truth: read the
   owner, never mine git history or a prior session's artifacts for how to do
   what an owned surface already specifies. History answers *what happened*,
   never *what is correct*; precedent is consulted only where no owner exists,
   and that consultation is a doc gap to capture, not a habit to keep.
   *Under agent work:* models train on repos whose docs lag practice, so
   "imitate the last instance" is a reflex prior — and here it is inverted:
   the gates hold docs and practice convergent, so mimicking a prior session
   re-imports as house style exactly the deviation that session was gated on.
   *Enforced by:* the battery itself, which is what makes the owner doc
   trustworthy, plus the knowledge-friction loop
   ([drift-kit/SPEC.md](../drift-kit/SPEC.md) §The knowledge-friction loop) —
   a fact with no owner gets one, shrinking the space where history is the
   only oracle. No scanner sees *how* a session derived a choice; this rule
   is judgment with a capture mechanism, not a gate.

## Engineering-craft rules

9. **Spec-invariant test naming.** A test of a spec-mandated invariant encodes
   that invariant in its *name*; the SPEC's test-requirement section owns which
   invariants a test must cover. A spec-clause *comment* on the test is not a
   substitute — it duplicates what the name should carry and rots silently when
   the clause is renamed.
   *Under agent work:* a test name is the line read at every failure and kept
   across refactors; an agent scanning a failing suite reads names, not comment
   pointers, so the invariant must live where the failure surfaces it.
   *Enforced by:* the test-naming convention, and canon-kit's `check-comment-tier`
   ([canon-kit/SPEC.md](../canon-kit/SPEC.md)) — which sweeps the comment
   substitute out rather than blessing it, leaving the name as the only place to
   carry the invariant.

10. **Test from the real consumer's runtime.** Verify a contract from the runtime
   of its real consumer, never a more lenient stand-in; a failure at a higher
   test layer with no failing test at the layer below is a coverage gap in the
   lower layer, closed there first.
   *Under agent work:* an agent scripts the contract from whatever client is
   easiest to drive, and a lenient stand-in passes what the real consumer's
   stricter stack would reject — a green the production caller does not share.
   *Enforced by:* a test-layer convention, not yet a checkwright gate: a consumer
   registers its real-consumer suites in the validate battery
   ([evidence-kit/SPEC.md](../evidence-kit/SPEC.md)), where a held-constant
   baseline turns a dropped layer into a red validate.

11. **Inspectable-run discipline.** A component a test or an automation spawns
    must emit a readable log to an inspectable path — never a muted sink; on a
    failure, read that evidence before theorizing. A run you cannot inspect
    barely beats a guess.
    *Under agent work:* an agent theorizes confidently from nothing; forcing every
    spawned component to leave an inspectable artifact converts a guess into a
    read, and recorded evidence beats a reconstructed hypothesis.
    *Enforced by:* the convention, kin to delegation-kit's resume-journal
    discipline ([delegation-kit/SPEC.md](../delegation-kit/SPEC.md)) — the same
    move of making a background actor write an inspectable record rather than
    trusting its self-report.

12. **Rename is a full-surface sweep.** A rename sweeps every surface in
    lockstep — prose, fixtures, and docs, not only the compiler-checked
    identifiers; the done-gate is a text-level completeness check, not the
    type-checker, and an in-progress rename is verified by a completeness scan
    before it is called finished.
    *Under agent work:* an agent reads a green type-check as "rename done" and
    leaves the retired term alive in every surface the compiler never sees; only
    a text-level sweep closes it.
    *Enforced by:* a text-level completeness check whose term list is *consumer
    config*, never a kit literal — the same seam that keeps product vocabulary
    out of shipped mechanism (the config-via-env pattern in
    [gate-sdk/SPEC.md](../gate-sdk/SPEC.md)), so the check ships and the
    vocabulary stays with the consumer.
