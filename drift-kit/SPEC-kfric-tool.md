# SPEC amendment: kfric-tool

## What changes

- New tool `drift-kit/bin/kfric.sh "<fact>" "<surface>"` — the capture
  affordance for the knowledge-friction loop: appends one line in the
  existing capture grammar, `<date> <fact> ← <surface>` (date from
  `date +%F`), to the knowledge-friction log (`DRIFT_KIT_KNOWLEDGE_LOG` —
  the existing knob and default; no new knob). Both arguments required
  and non-empty, else a usage message and exit 2; the log's parent
  directory is created if missing.
- Why a tool and not the documented raw append: the raw form is a shell
  redirect (`printf ... >> <log>`), which no allowlist glob can suppress
  safely — a mid-pattern wildcard opens the command-injection shape the
  bash guard exists to catch, and a decorated write trips the guard's
  decoration rule regardless. The helper takes the fact as an argument
  with no caller-side redirect, so the invocation is a safe end-wildcard
  prefix-glob allowlist entry, and it stamps the grammar so the format
  stops being hand-typed. Capture must be prompt-free — a permission
  prompt at the capture moment is deferred capture, which is no capture.
- Raw append stays legal: the grammar, not the writer, is the log's
  contract — both consumers below read lines, not provenance.
- Consumer residue (this repo) rides the unit: the committed settings
  allowlist gains the prefix glob for the tool, and the CLAUDE.md
  knowledge-friction bullet names the tool instead of spelling the raw
  append.

## Producers and consumers

- Producer: any session at the re-derivation moment — the loop's capture
  step, unchanged in trigger, cheaper in mechanics.
- Consumers, both existing and unchanged: the close-stage triage walks
  the log (`drift-kit/templates/close-knowledge.md`), and
  `kpi-knowledge-friction` counts its lines. The tool only stamps the
  grammar they already parse.

## Existing sections updated

- drift-kit/SPEC.md §The knowledge-friction loop: the capture step names
  `bin/kfric.sh` as the affordance (raw append remains the fallback).
- drift-kit/SPEC.md bin roster (the kit's component list) gains the tool.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
