Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

**ruling-config** — `.claude/agents/stage-session.md` (dispatch `subagent_type:
stage-session`); its §Ruling classes holds the escalate-vs-decide roster.
Posture: **Split** (template §The lead model). Lead and every stage — scope
included — ride Opus via the agent's `model: opus` frontmatter default, so no
stage inherits the dispatcher's tier (measured leakage was the largest budget
drain). Two stages depart from the Opus default, and by the per-batch tiering
the template §Economics "Tier each batch to its work class" rule now names:

- **`validate`** — the lead dispatches it with a `model: sonnet` override.
  validate's batches are uniformly *mechanical oracle-running* (run the battery,
  report), so the stage collapses to a single stage-uniform-mechanical default —
  the degenerate case of per-batch tiering, not a bound per-stage roster.
- **`build`** — tiered **per batch**, not stage-uniform. The lead reads the
  work-class labels of a batch's deltas (via the `[spec:]` amendments its entries
  point at) and pins `model: sonnet` on a batch whose deltas are **all
  mechanical**, leaving a batch carrying **any design-bearing** delta on the Opus
  judgment default. build is the divergent case the per-stage rule could not
  express — the template §Economics "Tier each batch" rule owns the worked
  example.

scope, `spec`, align, and close stay on Opus, their generative and
verificational judgment being what justifies the tier. Re-judge every tier when
the harness model roster churns.

**escalation-guard** — inert, the optional-guard default (as
`guard-kit/templates/wakeup-guard.sh`, unwired in `.claude/settings.json`). A
lead-model session wires `guard-kit/templates/escalation-guard.sh` on
SendMessage per guard-kit/SPEC.md §wakeup-guard.
