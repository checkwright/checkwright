Execute the template at lifecycle-kit/templates/skills/scope.md, applying the bindings below.

## Bindings

**exit-condition** — the design is written down (this session's plan or a SPEC
draft) and the seam is ruled: what ships as mechanism, what stays private rule
content, what becomes consumer config.

**evidence-reset** — nothing to reset by hand here:
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` (`scripts/lifecycle-config.sh`) lists
`.workflow/validate-evidence.txt`, so `enter-stage.sh scope` truncates it —
together with the lesson-evidence file — in the same stamp commit.

**ritual** — read `BRIEF.local.md` (local-only brief); decide the unit's
layout, config surface, and worklist; name the iteration after the unit. Hold
the provenance seam per CLAUDE.md §The provenance seam (never cross it) and
config-via-env per CLAUDE.md §Conventions established in gate-sdk. The amendment
lifecycle the template's triage step invokes is canon-kit/SPEC.md §The
amendment lifecycle; the queue-entry grammar the promotion step writes
against is queue-kit/SPEC.md §The tag algebra, and `[spec:]` ref
resolution is canon-kit/SPEC.md §check-amendment-queue.

**handoff** — lay out the two branches per docs/orchestration.md §Running an
iteration under a lead: compact this session then `/lead` to drive the rest, or
skip the lead and steer each stage by hand, consulting the compacted scope
session when a stage asks. That page is the sequence's single source (owned by
lifecycle-kit/SPEC.md §templates/lead.md) — send the reader there, do not
transcribe its steps.
