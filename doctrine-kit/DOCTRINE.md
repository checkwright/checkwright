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
test, rename, git-rewrite, config-edit, and dispatch work and live behind the
link — an application of the load-trigger residency rule to the doctrine itself.

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
   stays a stated manual duty rather than a noisy gate. A stated manual duty
   carries a *named cadence*, or it is a duty no session performs: the
   un-gateable class joins a tracked audit roster reviewed on a lifecycle hook,
   with event-keyed due-ness — a named observable event (a contract edit, a
   release prep, a template upgrade) beats an iteration counter no surface
   tracks. The roster is hand-curated, not derived — which classes escape a
   clean check is a judgment no tool enumerates, so Derivation-first's ladder
   lands on state-once-at-the-owner — and the roster plus its review step *are*
   the capture mechanism this carve-out owes, self-applying like the
   Gap-disposition rule's costed deferral rather than a gate; a duty with no
   cadence rots exactly like the gap that rule catches.
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

4. **Derivation-first.** A fact a tool can derive from the tree is derived at
   use time, never maintained as its own surface. The ladder: derive; else
   state once at the owner and point (the Content-tiering / SSOT rule); and a
   second copy a reader genuinely needs is a generated projection with a
   freshness gate, never a hand-maintained restatement. Whether what remains
   earns a gate is the Enforcement-first weighing. A count of a collection is
   the archetypal derivable fact: the collection is its own counting surface,
   and a stated total anywhere else is a duplicate counting surface, off by
   one at the next member.
   *Under agent work:* a hand-maintained enumeration of a derivable set is a
   copy of the tree that is stale at the next edit, and an agent trusts the
   nearer roster over the tree it summarizes; derivation deletes the drift
   axis rather than gating it.
   *Enforced by:* the derivation seams and their residual meta-checks — the
   `kit:` couple expansion with its hand-list gate in
   [gate-sdk/SPEC.md](../gate-sdk/SPEC.md) §check-kit-enum, the generated
   projections' byte-compare freshness assertions in
   [gate-sdk/SPEC.md](../gate-sdk/SPEC.md) §check-graph, and the
   bare-cardinal gate in [canon-kit/SPEC.md](../canon-kit/SPEC.md)
   §check-manifest-count.

5. **Always-loaded shape.** A rule in the always-loaded agent file is one line —
   the convention plus a pointer; its mechanism, rosters, and defaults live
   behind the pointer in the owning doc.
   *Under agent work:* the always-loaded file is read in full at every session
   start, so every line spent restating a mechanism is context tax on every
   task; a one-line pointer pays once and loads the detail only when needed.
   *Enforced by:* the brevity budget in
   [context-kit/SPEC.md](../context-kit/SPEC.md) §The brevity gate.

6. **Load-trigger residency.** The always-loaded file earns a rule only when no
   stage, skill, or tool-call trigger exists to load it; anything a trigger can
   pull lives in its owned doc behind that trigger.
   *Under agent work:* always-loaded context is the scarcest budget; a rule that
   a stage or a tool call would load anyway costs nothing to defer and everything
   to keep resident.
   *Enforced by:* the stage/skill load-triggers in
   [lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) and the tool-call hook seam in
   [guard-kit/SPEC.md](../guard-kit/SPEC.md), which give every triggered rule a home
   that loads on demand.

7. **Widest-true-tier placement.** A fact lands at the widest tier where it holds
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

8. **Oracle-first.** A check's output is its interface: run the gate instead of
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

9. **Spec-over-precedent.** A gate-enforced doc is ground truth: read the
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

10. **Gap disposition.** A gap a session surfaces but will not close this
   session — a coverage hole, a design defect, an escaping class — is *costed*,
   not merely flagged: the session works out how the gap could be closed and
   lands that analysis where the next session finds it, as a new queue entry or
   an enrichment of the standing deferred entry that owns it. A bare
   flag-and-skip is the defect. This is the knowledge-friction loop one altitude
   up — that loop captures re-derived *facts* with no owner; this rule captures
   the *design and coverage gaps* a session leaves behind. It fills the space
   between two neighbours: Spec-over-precedent captures a doc gap met during
   precedent consultation, Enforcement-first governs a fix being *landed*, and
   neither governs a gap being *deferred*.
   *Under agent work:* a fresh context window cannot see a gap the prior session
   only reasoned about; an unwritten deferral is one the next session
   re-discovers from scratch, and the costed remedy is the only carrier of the
   analysis across the reset.
   *Enforced by:* judgment with a capture mechanism, not a gate — whether a
   session costed the remedy is not machine-decidable (the Enforcement-first
   false-positive carve-out, on Spec-over-precedent's model). It rides the
   queue's Deferred section for design gaps and the knowledge-friction log
   ([drift-kit/SPEC.md](../drift-kit/SPEC.md) §The knowledge-friction loop) for
   fact gaps.

## Engineering-craft rules

11. **Spec-invariant test naming.** A test of a spec-mandated invariant encodes
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

12. **Test from the real consumer's runtime.** Verify a contract from the runtime
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

13. **Inspectable-run discipline.** A component a test or an automation spawns
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

14. **Rename is a full-surface sweep.** A rename sweeps every surface in
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

15. **Config edits are merges, not rewrites.** Edit a config or settings file
    with targeted, string-scoped edits, never a full-file write reconstructed
    from a partial read — a whole-file write built from part of the file
    silently drops everything the write did not carry. And validate an
    apparently broken file with the format's own parser before calling it
    corrupt: an apparent corruption is usually a parse by the wrong tool, not
    damaged content.
    *Under agent work:* an agent reads the top of a settings file, writes the
    whole file back from that fragment, and the keys it never read vanish with
    no diff to flag them; or it declares a file corrupt off a wrong-tool parse
    and "repairs" a file that was already valid.
    *Enforced by:* a convention, not yet a checkwright gate — the harness's Edit
    tool is the mechanism (an exact-string replace that fails loudly on a
    missing or ambiguous match, keeping the write scoped to what was read); no
    gate asserts a given edit was a merge rather than a rewrite.

16. **Re-verify volatile state before a git history rewrite.** Verify HEAD
    (`git log --oneline -3`) before an amend or squash; after a `git reset
    --soft`, re-stage and verify the staged content (`git show :<path>`) before
    committing — the soft reset keeps the old index snapshot; write any `git
    commit -F` message file fresh in the same turn (prefer `-m` for a short
    message — a leftover file lands the wrong message with exit 0); and rewrite
    the message when amending so it states the combined change.
    *Under agent work:* the git index and HEAD are mutable state an agent
    reasons about from a stale in-context snapshot, and a rewrite acts on what
    the working tree *now* holds, not what the transcript last recorded — so the
    verification must be a fresh read, never a remembered one.
    *Enforced by:* guard-kit's advisory guard rule on `git commit --amend`, `git
    reset --soft`, and `git commit -F`
    ([guard-kit/SPEC.md](../guard-kit/SPEC.md) §The generic ruleset), which
    surfaces this checklist at the moment the rewrite is dispatched — advisory
    because each command is legitimate.

17. **Entering another repo's tree, read its governance first.** A cross-repo
    edit re-reads that repo's agent file and README and checks its branch
    freshness every time — a second repo's model drifts independently of this
    one's, so a remembered version of its rules is a stale premise.
    *Under agent work:* an agent carries the governing rules of the repo it was
    last in into the next one, importing the wrong house style; only a fresh
    read of the entered tree's own governance corrects the prior.
    *Enforced by:* a convention, not yet a checkwright gate — another repo's
    tree is outside this tree's gate horizon, so no local check can assert its
    governance was reread.

18. **Naming: drop the qualifier the context supplies — only when every consumer
    has that context.** A name that travels into a flat namespace keeps its
    qualifier; default to the shorter form and reject the vacuous one, but a
    name that loses the context which disambiguated it must carry the qualifier
    with it.
    *Under agent work:* an agent copies a short name out of the scope that gave
    it meaning into a flat namespace where the qualifier is load-bearing, and
    the collision surfaces far from the rename; "does every reader still have
    the context" is the discriminator, not shortness.
    *Enforced by:* a convention, not yet a checkwright gate — whether a
    qualifier is redundant *for every consumer* is a judgment a scanner cannot
    decide; the de-literalization instinct, a name sized to its widest reader,
    is its written form.

19. **Reuse a co-located consumer's data before designing a new path.** For an
    embedded or co-located actor, first ask whether it can read a co-located
    consumer's already-fetched data before minting a new stream, grant, or
    fetch — a "which path" framing can hide a "no path needed" answer.
    *Under agent work:* an agent asked "how should X get this data" designs a
    new channel because the question presupposes one, and never asks whether the
    data is already in reach; the cheaper reuse stays invisible unless the
    framing is challenged.
    *Enforced by:* a convention, not yet a checkwright gate — a data-path choice
    leaves no artifact a scanner reads; it is design-review judgment, kin to
    reaching for an existing owner before minting a second source.

20. **A resolver gate's flagged key is a fork, not a verdict.** A name-resolution
    gate that finds a silent drop has found either dead config to remove *or*
    promised-but-unwired config to build — one signature, opposite fixes; only
    the owning SPEC distinguishes them, so verify intent against the SPEC before
    sweeping the key away.
    *Under agent work:* an agent reads a gate finding as an instruction ("remove
    the flagged key") and deletes config the SPEC promised but the wiring never
    delivered — turning a build-it signal into a delete-it action; the gate saw
    the drop, not the intent.
    *Enforced by:* the name-resolution gates that surface the drop — the
    couples/reads walkers and reference-liveness sweeps in
    [gate-sdk/SPEC.md](../gate-sdk/SPEC.md) — but the fork itself is resolved
    against the owning SPEC by judgment; the gate reports the dangling key,
    never which way it should be closed.
