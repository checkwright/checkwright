# SPEC amendment: shim-dedup

## What changes

- New lifecycle-kit gate `checks/check-shim-restatement.sh` — the duplication
  tripwire under the binding-shim grammar: no binding shim under
  `LIFECYCLE_SKILLS_DIR` may share a word n-gram of length ≥
  `LIFECYCLE_SHIM_NGRAM` with any surface in the dedup corpus
  (`LIFECYCLE_SHIM_DEDUP_CORPUS`; default: the consumer's always-loaded file
  `CLAUDE.md` plus every kit's `templates/**/*.md`, kit set from
  `gate_kit_roots`). Comparison normalizes first — lowercase, punctuation
  stripped, whitespace collapsed — so cosmetic rewording does not evade the
  tripwire. Honest limit, stated in the SPEC section: the n-gram holds the
  *copy shape* only; which tier a fact belongs to stays semantic judgment —
  a paraphrase below N words passes the gate and is still a defect to fix
  on sight (the same doctrine as check-comment-tier's floor).
- `LIFECYCLE_SHIM_NGRAM` default is calibrated at build: the smallest N with
  zero false positives on the post-rewrite corpus, with a floor of 8 words —
  a citation line (a path plus a §heading) must never fire.
- `# graph:` manifest: couples the skills dir, `CLAUDE.md`, and the kit
  template dirs; `tier=precommit`. Fixture pair per the four contracts.
- Consumer-side rewrite (this repo) rides the same unit:
  - The close shim's housekeeping binding names
    `guard-kit/templates/close-triage.md` and
    `drift-kit/templates/close-knowledge.md` as the procedure owners and
    binds only consumer residue (the local sink overlay, log locations);
    the restated triage criteria and log-walk procedure are deleted.
  - The build shim's restatements of always-loaded facts (generated-hook
    regen, public-repo hygiene, red-gate-never-bypassed, the
    skeleton+fixture contract) trim to citations; its pre-commit battery
    subset becomes a citation of the CLAUDE.md battery block plus the
    touched kit's fixture suite.
  - The scope shim's provenance-seam restatement trims to a citation.

## Producers and consumers

- Producer of the verdict: the gate, run by `run-gates.sh` and by the
  generated pre-commit hook once `gen-pre-commit.sh --write` picks up the
  manifest (both regenerated artifacts ride the landing commit).
- Consumer: the committing session — a red run names the shim, the corpus
  surface, and the shared n-gram, so the fix (delete the restatement, keep
  the citation) is mechanical.
- The rewritten shims are consumed unchanged by stage sessions via the
  Skill tool; `check-skill-binding` slot parity is unaffected — every
  binding keeps its slot name, only its body shrinks to residue+citation.

## Existing sections updated

- lifecycle-kit/SPEC.md §templates/skills/ gains the authoring rule a
  binding shim follows: bind consumer residue, cite kit-owned procedure,
  never restate it. The new §check-shim-restatement section lands beside
  §check-skill-binding.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
