# SPEC amendment: spec-pointer

The pointer-resolver that spec-kit/SPEC.md §check-comment-tier records as
missing: `check-comment-tier` blesses a `spec:` / `contract:` directive by
shape but never checks that its target resolves, so a renamed or deleted
heading leaves every inbound pointer dangling, caught only on review.
Surfaced 2026-07-07 dogfooding the comment-tier sweep — the coupling value
the sweep leaned on is only as good as the pointers being live.

## What changes

- **New gate `spec-kit/checks/check-spec-pointer.sh`** (skeleton-derived,
  four contracts, `good/`+`bad/` fixtures). Invariant, **forward direction
  only**: every pointer directive on a governed surface resolves —
  - the directive set is exactly what `check-comment-tier` blesses by
    shape: full-line `spec:` / `contract:` comments on governed sources,
    plus the workflow-file `# contract:` headers;
  - target grammar `<path> [§<heading>]`: `<path>` (repo-relative) must
    exist as a tracked file; when a `§<heading>` fragment is present, the
    file must contain a matching markdown heading (matched via
    `lib/spec.sh`'s section-regex builders — the same adapters the other
    spec gates use; `check-graph`'s asset-href resolution is the
    precedent). A pointer without `§` resolves file-only.
  - Red on: nonexistent target file, or named heading absent from the
    target. Fail-closed: an unparsable directive that matched the pointer
    shape is red, not skipped.
- **Ruled out (for now): the reverse direction** — flagging a requirement
  with no inbound pointer as uncovered code. It needs a "what counts as a
  requirement" notion (every `###`? tagged only?) that risks false
  positives, violating the cheap-and-FP-free bar for gates. If wanted
  later it is a separate task with its own design ruling; this amendment
  deliberately does not reserve syntax for it.
- **Governed-surface discovery reuses `lib/spec.sh`** (same source set as
  `check-comment-tier`); no new config knob — the gate inherits spec-kit's
  existing `SPEC_KIT_*` configuration.
- **Tier: precommit**; `# graph:` manifest lists its couplings; registered
  in this repo's `scripts/gates.list`.

## Producers and consumers

- **Producer:** the generated pre-commit hook / `run-gates.sh` — reachable
  in every consumer whose `gates.list` names the gate; no enabling config
  beyond registration (governed sources already exist wherever spec-kit is
  vendored).
- **Consumer:** the committing operator/agent via the gate output contract.
- **Fields:** a pointer's `<path>` is read by the existence check; its
  `§<heading>` by the heading match — both at the gate's single scan
  transition. No new persistent state.
- **Interaction with `check-comment-tier`:** unchanged directive shapes —
  this gate adds resolution on top; the tier gate keeps owning shape. The
  tripwire note in §check-comment-tier ("until a pointer-resolver lands")
  is retired at merge.

## Existing sections updated

At merge into spec-kit/SPEC.md:
- new `### check-spec-pointer` per-component contract section (invariant +
  calibration as above);
- §check-comment-tier's tripwire sentence about the missing
  pointer-resolver is replaced with a cite of the new gate;
- the layout section's gate inventory count/list gains the gate.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
