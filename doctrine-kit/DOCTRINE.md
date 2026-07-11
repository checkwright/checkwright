# DOCTRINE.md — the Checkwright delivery doctrine

The cross-kit delivery rules the kits enforce piecemeal, stated once. This file
is the deliverable of [doctrine-kit](README.md): referenced in place from a
consumer's always-loaded agent file, never copy-installed — re-vendoring the kit
*is* the doctrine upgrade. Each rule lands as its statement, why it holds under
coding-agent work, and a pointer to the mechanism that enforces it; the
mechanism and its knob rosters live in the cited kit SPEC, never restated here.

## The rules

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
   rather than landing bare.
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
