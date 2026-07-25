# SPEC amendment: per-batch-tiering

Component: **lifecycle-kit** (both changed surfaces are lifecycle-kit
templates). This reconciles the lead binding's model-tiering unit from the
*stage* to the *batch*, and adds the label that makes a batch tierable — a
per-delta **work-class** tag on `/spec`'s output. It is **single-component**: it
changes lifecycle-kit's `/spec` output contract and `lead.md` §Economics only,
and *cites* delegation-kit's unit-shape doctrine
(`delegation-kit/templates/agent-execution.md`, "Match the dispatched model and
effort to the unit's shape") without changing it. The taxonomy's economic
principle stays owned in delegation-kit; this amendment adds a lifecycle-kit
label that **applies** it and a lifecycle-kit consumer that **reads** it, so the
cross-component align-audit stays disarmed.

## What changes

**Delta A — `/spec` emits a per-delta work-class label. `[design-bearing]`**
`/spec`'s output contract (the amendment it authors) gains one requirement: in
the amendment's §What changes, every delta carries a work-class tag — either
**mechanical** or **design-bearing**. A delta is **mechanical** when executing
it demands only oracle-running — running a fixed verification battery, a
rename/merge sweep, a mechanical pin — with low generative judgment. It is
**design-bearing** when executing it demands generative or verificational
judgment — authoring a contract, a cross-spec audit, a non-obvious
implementation. The label records what the delta *demands*, never a model name:
a baked model name is drift by construction against a churning roster, and a
spec-time model *recommendation* would attach to a batch the lead has not cut
yet (the lead cuts batches at build). The two values apply delegation-kit's
existing unit-shape distinction — "a read-heavy or mechanical unit … rides a
cheaper model class; a unit carrying design judgment stays on the supervisor's
class" — at authoring time. This is judgment spec holds and the lead does not:
spec knows what each delta demands, the lead knows only what the queue entry
says. (This amendment's own two deltas are the worked example, each labelled
here — and both are design-bearing, so this batch correctly stays on the
judgment tier.) A *mixed* amendment is the sharper case: this iteration's build
amendment carried a design-bearing Delta A beside a mechanical Delta B and a
mechanical hermeticity rider — three deltas, two classes — which per-batch
tiering can act on and per-stage tiering cannot.

**Delta B — `lead.md` §Economics tiers the batch, not the stage. `[design-bearing]`**
The §Economics bullet currently headed "Tier each stage to its work class" is
reconciled to tier the **batch**. The lead reads the work-class labels of the
deltas in a batch — via the `[spec:]` amendments the batch's entries point at —
and a batch whose deltas are **all mechanical** is tier-downgradeable (the lead
pins the cheaper tier with a `model` override on that batch's dispatch), while a
batch carrying **any design-bearing** delta stays on the judgment tier. Class →
live model is mapped at dispatch time, where the roster dependency already
belongs (agent-execution.md, same bullet). This resolves the live
self-contradiction in the current text: the bullet just above already calls
"per-batch model tiering … the dominant window lever," which presupposes a
per-batch tier the "tier each stage" rule never provides. A stage whose batches
share one class (validate, uniformly mechanical) collapses to a stage-wide
default as the degenerate case; a stage whose batches diverge (build — a
one-line hermeticity pin beside a new KPI plugin) tiers each batch from its
deltas' labels, which the per-stage rule could not express.

## Producers and consumers

New interface: **the per-delta work-class label** — an enumerated value,
`{mechanical | design-bearing}`, one per delta in an amendment's §What changes.

- **Producer** — `/spec` (lifecycle-kit `templates/skills/spec.md`), which emits
  the label into every amendment's §What changes as it authors. Enabling config:
  **none** — it is a skill-mandated output, always emitted; there is no knob to
  unset, so no deployed configuration can leave the producer dark.
- **Consumer** — the lead (lifecycle-kit `templates/lead.md` §Economics), which
  reads the labels at **batch-cut time** and maps the batch's aggregate class to
  a live model tier. The named reader is the lead's per-batch tier decision; the
  transition it is read at is the choice of `model` override on that batch's
  stage-session dispatch.
- **Every field has a named reader** — the label is a single enumerated value;
  its one reader is the lead's per-batch tier decision above. No field is left
  unread.

## Existing sections updated

- **`lead.md` §Economics, the "Tier each stage to its work class" bullet** — is
  rewritten per Delta B. Its clause "Which stages fall on each side is the
  consumer's binding (the ruling-config slot)" is reframed: the batch's labels
  decide at dispatch time, so there is no standing per-stage classification to
  bind — a stage-uniform class is a collapsed default, not a bound roster. The
  neighbouring "split where the model tier changes" bullet is unchanged; it
  already presupposes the per-batch tier this delta now supplies.
- **`spec.md` output contract** — gains Delta A's label requirement, authored as
  a paragraph in the amendment-authoring ritual, beside the causal-completeness
  contract it already carries.
- **Consumer integration — `.claude/commands/lead.md` (this repo's
  ruling-config binding)** — its per-stage roster ("validate → sonnet;
  scope/`spec`/align/build/close stay Opus") is reconciled by **build** when it
  implements this amendment: `validate` stays a stage-uniform-mechanical default
  (correct under per-batch as the collapsed case), and **`build`** moves to
  per-batch tiering from its deltas' labels — the case batch 2a (mechanical,
  belonged on the cheaper tier) and batch 2b (design-bearing) proved the
  per-stage rule could not express. This shim is named here for causal
  completeness; it is **consumer config, not a second kit component**, so this
  amendment stays single-component and does not arm the align-audit.

No wire-contract delta; no embedded source.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical surface (spec.md's output contract; lead.md §Economics),
      not appended; the merged templates read as one coherent document a reader
      who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every surface for the per-stage tiering
      phrasing this change retires (`lead.md`'s "Tier each stage", the shim's
      per-stage roster); nothing dangles.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed
      as a debt task (a build-time causal gap is resolved that session, not
      deferred).
