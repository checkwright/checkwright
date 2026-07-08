# SPEC amendment: template-tiering

## What changes

Ruling (scope 2026-07-09, resolves the template-exemption question logged
for adoption): **kit SPECs travel** — the install contract is
vendor-the-whole-kit-directory, SPEC.md included, so a copied-out
template's `spec:` pointer resolves against the vendored kit path in the
consumer tree. The blanket `/templates/` exclusion in
`spec_governed_sources()` therefore over-exempts, and it splits per gate:

- **`check-comment-tier` governs `templates/`.** After the header
  thinning below, every full-line comment a template carries is a legal
  tier class (a directive, or the placeholder `spec:` line — a spec
  pointer by *shape*, which is all this gate checks). spec-kit's own
  stubs then pass the comment-restraint gate on merit, not via exemption
  — the adoption-optics fix. Precedent: the same per-gate lift landed for
  `check-shellcheck` on templates.
- **`check-spec-pointer` keeps the exemption, re-grounded.** A template's
  `spec: <your SPEC> §check-<area>` line is a placeholder — unresolvable
  *by design* until the consumer fills it in; resolution-checking
  templates is checking the consumer's homework in the kit's tree. The
  SPEC states this rationale (placeholders-by-design, not
  "SPECs don't travel").

Mechanism: `spec_governed_sources()` gains a variant that keeps
`templates/` paths — `spec_governed_sources_with_templates()` — and
`check-comment-tier` switches to it; every other caller is untouched. No
new knob: which finder a gate uses is kit contract, not consumer config
(a consumer wanting the old behavior shadows the gate, the established
escape hatch).

Header thinning (the doctrine half, riding in this amendment): installer
and pedagogy prose in template headers — copy-into-your-gates-dir,
wire-under-PreToolUse, this-is-a-reference-skeleton narration — moves to
its single source: the owning kit's SPEC/README sections that already
carry the doctrine (e.g. gate-sdk/SPEC.md §The gate model), with the
header keeping only the `# graph:`/`# spec:` directive lines and the
placeholder scaffolding the consumer fills in. A docs page may cite those
sections; it restates nothing (SPEC-docs-site.md's tiering ruling).

## Producers and consumers

- Producer: `check-comment-tier` over the widened source set, at
  pre-commit and in `run-gates.sh` — active immediately on land (the gate
  is already registered).
- Consumer: the committing session — findings on template comments now
  name real lines to fix instead of being invisible.
- Inputs read: `spec_governed_sources_with_templates()` output; no new
  state or message fields. The new finder's only caller at land is
  `check-comment-tier` (a finder with no caller is removed, per causal
  completeness — it ships wired).
- Fixture pair: `check-comment-tier`'s existing fixtures gain a
  template-path case (good: thinned header passes; bad: narration
  comment under `templates/` is red).

## Existing sections updated

- spec-kit SPEC §lib/spec.sh: the finder table gains the variant and the
  per-gate split rationale.
- spec-kit SPEC §check-comment-tier: governed-set sentence updated
  (templates included); §check-spec-pointer: exemption rationale replaced
  with placeholders-by-design.
- Each kit's `templates/` headers thinned in the same change (gate-sdk,
  lifecycle-kit, queue-kit, spec-kit, guard-kit, delegation-kit,
  context-kit, drift-kit, evidence-kit — wherever narration exceeds the
  directive lines), receiving SPEC/README sections verified to already
  own the moved fact.

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
