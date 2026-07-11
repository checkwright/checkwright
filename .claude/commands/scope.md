Execute the template at lifecycle-kit/templates/skills/scope.md, applying the bindings below.

## Bindings

**exit-condition** — the design is written down (this session's plan or a SPEC
draft) and the seam is ruled: what ships as mechanism, what stays private rule
content, what becomes consumer config.

**evidence-reset** — nothing to reset by hand here:
`LIFECYCLE_BOUNDARY_TRUNCATE` (`scripts/lifecycle-stages.sh`) lists
`.workflow/validate-evidence.txt`, so `enter-stage.sh scope` truncates it —
together with the lesson-evidence file — in the same stamp commit.

**ritual** — read `BRIEF.local.md` (local-only brief); decide the unit's
layout, config surface, and worklist; name the iteration after the unit.
Private rule content never lands here — term lists, vocabularies, glossary
bodies become optional consumer config, never kit literals; this repo's layout
stays as the defaults under `<KIT>_<KNOB>` env/config knobs. The amendment
lifecycle the template's triage step invokes is canon-kit/SPEC.md §The
amendment lifecycle; the queue-entry grammar the promotion step writes
against is queue-kit/SPEC.md §The tag algebra, and `[spec:]` ref
resolution is canon-kit/SPEC.md §check-amendment-queue.
