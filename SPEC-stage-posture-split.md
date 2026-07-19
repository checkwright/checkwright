# SPEC amendment: stage-posture-split-tuning

<!-- Governance/workflow ruling with no owning component → repo root. Paired
     with the New-Features entry [spec: SPEC-stage-posture-split.md]. Delta only;
     merges into lifecycle-kit/SPEC.md, canon-kit/SPEC.md, lead.md §Economics,
     the scope skill, and this repo's consumer config on completion, then this
     file is deleted. -->

Two moves the `/economics` read on `derivation-by-precedent` motivated
(cache-read was ~95% of iteration burn; the levers are per-stage model class and
capping context re-accumulation, not template trimming). Both are ruled here as
one unit because they share a cause and the same dispatch surface.

## Seam ruling (settle first — the exit condition)

- **Mechanism (ships in lifecycle-kit, generic):** (1) a stage-skill template
  `lifecycle-kit/templates/skills/spec.md` for an amendment-authoring stage;
  (2) the generic **tier-differentiation rule** in `templates/lead.md`
  §Economics — a stage whose work is *mechanical oracle-running* is
  tier-downgradeable, a stage whose work is *generative judgment* is not;
  (3) SPEC prose stating that a **trigger-gated authoring stage** is a supported
  roster shape (the same class as the trigger-gated audit stage).
- **Consumer config (this repo activates; kit default unchanged):** the roster
  override and bindings in §Activation below. The **kit default roster stays the
  five-stage `scope align build validate close`** — the split is demand-gated
  (one consumer has attested it), non-breaking for existing consumers, and the
  stage machine is already extension-ready (`LIFECYCLE_KIT_STAGES` /
  `LIFECYCLE_KIT_PREDECESSOR` are config, `lib/stages.sh`).
- **Private rule content:** none. A lifecycle stage is generic methodology, not
  a term list or product constant — nothing crosses the provenance seam.

## What changes

### Move (b) — a dedicated amendment-authoring stage `spec`

New stage `spec`, positioned `scope → spec → align → build → validate → close`.
Ontology it sharpens: **scope bounds the units; `spec` authors the amendments;
align independently verifies them.** Name `spec` chosen for its tie to the
`SPEC-<feature>.md` artifact, the `[spec:]` queue tag, and canon-kit's existing
"Spec-writing" language. Rejected: `design` (collides with scope's retiring
"(design) stage" subtitle), `amend`, `draft`.

Stage properties (each a delta against `lib/stages.sh` semantics, none new
mechanism — all expressible in existing config):

- **Appends, never resets.** Only `scope` (`LIFECYCLE_KIT_FIRST_STAGE`) carries
  the unconditional iteration-boundary reset; `spec` stamps-and-appends like
  align/build/validate/close. This is the queue entry's decisive reason for a
  *new stage* over a second scope session: a second `enter-stage.sh scope` would
  re-truncate WORKFLOW-STATE, so the reset must stay uniquely scope's.
- **Trigger-gated, like the audit stage.** `spec` runs only when the iteration
  promotes ≥1 **feature** unit (an amendment to author). A **debt-only**
  iteration skips `spec` entirely (`scope → build → validate → close`), exactly
  as align is skipped when no cross-component amendment exists.
- **Predecessor: `spec → scope`.** `spec` is added to `LIFECYCLE_KIT_PREDECESSOR`
  with value `scope`. `spec` is **not** named as any stage's mandatory
  predecessor — the same calibration `check-stage-entry` applies to the
  trigger-gated audit stage (SPEC §check-stage-entry: "the predecessor map
  deliberately omits the trigger-gated audit stage as anyone's predecessor").
  So `align → scope` and `build → scope` are **unchanged**; inserting `spec`
  adds one edge and retitles none.

**The split of labor (the economics win).** The seam is the pre-promotion ruling
this very iteration exercised ("the proposed unit set is escalated for ruling
before promotion"):

- **scope** does the exploratory half — reset+stamp, survey tree + deferred
  queue, GitHub boundary sweep, premise re-verification, propose the unit set,
  escalate for ruling; then, once ruled, **name the iteration** and **promote
  debt** units (cheap tag moves, no amendment). scope authors **no** feature
  amendment and writes **no** `[spec:]` ref. It then ends — dropping the whole
  sweep context at the session boundary.
- **spec** starts fresh (fresh cache, and a tier the lead chooses) and does the
  generative half — author the feature amendment(s), promote the feature
  entries (`[spec: <ref>]` + the amendment file in one commit, satisfying
  canon-kit's bidirectional rule), recommend the next stage. Because it is a new
  session, the causal-completeness authoring no longer re-reads scope's sweep
  context through every authoring turn — the ~5.3M cache-read cost the
  `/economics` read attributed to scope.

**Trigger enforcement.** `spec`'s "run me when a feature was promoted" trigger is
**procedural** (scope's next-stage recommendation), **backstopped by the
existing gate**: a feature entry carries `[spec: <ref>]` only when `spec` has
authored the file, and `check-amendment-queue`'s bidirectional rule reds any
`[spec:]` ref that resolves to no file — so a skipped `spec` cannot ship a
feature without its amendment. A *mechanized* `spec`-trigger assertion (a
check-stage-entry sibling to assertion C) is **considered and deferred**: the
bidirectional gate already closes the hole, and align's own audit trigger is the
precedent for "procedural + one gated backstop" over a bespoke per-stage
assertion. Flagged for align to confirm the backstop is sufficient.

### Move (a) — validate dispatched at a lower model tier

Today every stage rides Opus via `.claude/agents/stage-session.md`'s
`model: opus` frontmatter default, ruled in `.claude/commands/lead.md` so no
stage inherits the dispatcher's tier. validate's rows are **mechanical
oracle-running** (run the battery, report — measured out=7,764 on cr=759k for
one session), low generative judgment, so a cheaper class serves. The lead
dispatches the validate stage session with a `model` override to a cheaper tier.

**Out of scope for downgrade, on record:** scope, `spec`, and align. Their
value is generative/verificational judgment (align is cross-spec verification
that prevents build struggles; scope and `spec` author and bound), not glue —
the judgment justifies the tier. The generic form of this ruling is the
tier-differentiation rule in the Mechanism list above; validate's specific
assignment is this repo's consumer binding.

## Producers and consumers (causal completeness)

New state/interface: the stage name `spec`, its skill, its predecessor edge,
its dispatch tier for validate.

- **`spec` stage name.** *Producer:* `LIFECYCLE_KIT_STAGES` gains `spec`
  (this repo: `scripts/lifecycle-config.sh`). *Consumers/readers:*
  `enter-stage.sh` (validates membership; appends since `spec ≠ FIRST_STAGE`);
  `check-stage-evidence` (stamp-grammar + name-axis); `check-stage-entry`
  assertion A (predecessor scan); `lifecycle_registration_block` →
  the CLAUDE.md registration block (regenerated by `install-lifecycle.sh`,
  byte-asserted by `check-lifecycle-registration`); `check-stage-skill-coverage`
  (every roster stage must have a bound skill); `check-skill-binding` (the
  binding resolves to a template).
- **`/spec` skill.** *Producer:* `.claude/commands/spec.md` binding →
  `lifecycle-kit/templates/skills/spec.md` template. *Consumer:* the spec stage
  session that invokes it, whose first step is its own append-stamp.
- **`spec → scope` predecessor edge.** *Producer:* `LIFECYCLE_KIT_PREDECESSOR`
  (this repo config). *Consumer:* `check-stage-entry` assertion A (a `spec`
  entry requires a `scope` stamp).
- **validate dispatch tier.** *Producer:* the lead's dispatch `model` override
  for validate, ruled in `.claude/commands/lead.md` (ruling-config) under the
  generic rule in `templates/lead.md` §Economics. *Consumer:* the validate stage
  session running on the cheaper tier. *Reader:* no code path — a dispatch-time
  lead decision, like every other model-tier choice.

Every new field has a named reader; no field is introduced without one.

## Existing sections updated

At merge, integrate (do not append) into:

- **`lifecycle-kit/SPEC.md`** — §The state machine (roster is N stages; `spec`
  appends, only `scope` resets); §check-stage-entry (predecessor map gains
  `spec → scope`; `spec` omitted as a mandatory predecessor, same calibration as
  the audit stage; note the deferred mechanized-trigger option and its
  bidirectional-gate backstop); §templates/skills/ (the new `spec` template and
  its contract); §Layout and configuration (the roster/predecessor as
  consumer-config; this repo activates).
- **`lifecycle-kit/templates/lead.md`** — §Economics (the generic
  tier-differentiation rule; validate downgradeable, scope/`spec`/align not);
  §The lead model (the dispatch roster now includes `spec`).
- **`lifecycle-kit/templates/skills/scope.md`** — scope's job shrinks to the
  exploratory half + naming + debt promotion; amendment authoring and feature
  promotion move to `spec`. The "set the iteration name" step stays scope's; the
  "author design amendments / promote" language moves to the new `spec` template.
- **`canon-kit/SPEC.md`** — §The amendment lifecycle: "Spec-writing is a
  scope-stage activity" → a **`spec`-stage** activity (build sessions still never
  author specs); the feature/debt litmus still runs at filing in scope's triage,
  but the amendment is *authored* in `spec`.
- **This repo's consumer surfaces (Activation, below).**

## Activation (this repo's consumer config — the build worklist)

- `scripts/lifecycle-config.sh`: set `LIFECYCLE_KIT_STAGES=(scope spec align
  build validate close)` and `LIFECYCLE_KIT_PREDECESSOR` adding `[spec]=scope`
  (align/build/validate/close edges unchanged).
- `.claude/commands/spec.md`: bind the `/spec` skill to the kit template.
- `.claude/commands/lead.md`: carve validate out of the blanket Opus pin — the
  lead dispatches validate at the cheaper tier per the tier-differentiation rule.
- Regenerate the derived projections and re-gate their freshness:
  `install-lifecycle.sh` (the CLAUDE.md registration block → six stages),
  the pre-commit hook + `docs/check-graph.html` if the manifest moved, the
  enforcement map (`docs/enforcement.md`) if a stage-coupled class changed, and
  the render-fidelity projections (`docs/lifecycle-kit/*`).
- `docs/orchestration.md` §Running an iteration under a lead: the per-stage
  branch layout now spans six stages (`spec` trigger-gated).

## Definition of Done

- [ ] **Causal completeness** — `spec` (name, skill, predecessor edge) and the
      validate tier each have a named, reachable producer and named consumers;
      no field lacks a reader.
- [ ] **Merged with no information lost** — each addition lands in its proper
      canonical section (lifecycle-kit/SPEC.md, canon-kit/SPEC.md, lead.md,
      scope.md); the merged specs read as one coherent document.
- [ ] **Amendment deleted** — this file removed on merge; none remain
      (`ls SPEC-*.md` at repo root empty).
- [ ] **Removals propagated** — grep every spec/doc for scope's retired
      "author amendments"/"(design) stage" framing; nothing dangles.
- [ ] **Roster in lockstep** — `check-lifecycle-registration`,
      `check-stage-skill-coverage`, `check-skill-binding`, the enforcement map,
      and the docs projections all green against the six-stage machine.
- [ ] **Gaps filed** — any cross-component gap surfaced in build is resolved
      that session, not deferred.
