Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

**ruling-config** — `.claude/agents/stage-session.md` (dispatch `subagent_type:
stage-session`); its §Ruling classes holds the escalate-vs-decide roster.
Posture: **Split** (template §The lead model). Lead and every stage — scope
included — ride Opus via the agent's `model: opus` frontmatter default, so no
stage inherits the dispatcher's tier (measured leakage was the largest budget
drain). Three stages depart from the Opus default, and by the per-batch tiering
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
- **`align`** — the lead dispatches it with a `model: sonnet` override, on the
  same stage-uniform reading `validate` takes. align's work class is
  *verification against an already-authored contract*: it audits the tree and
  the spec against each other and reports divergence, rather than generating the
  contract, which is `spec`'s work. That reading is what tiers it — the measured
  spend only says the tier is worth re-judging, never which way. It was taken
  provisionally, on the one stage carrying an adversarial-audit role, against a
  named revert signal: a missed spec defect surfacing as a build round-trip,
  one such miss outweighing the saving.
  **Verdict after the first full iteration on it (`shipped-roster-parity`,
  2026-08-02): keep.** The miss-shaped failure did not occur — align found the
  iteration's one load-bearing defect and reached the same escalation the lead
  reached, from its own evidence rather than by being told. That is the
  departure's premise holding, not merely its absence of harm. The revert
  signal stays live and unchanged; one confirming iteration retires the
  provisional label, not the watch.
  **Watch the draw, not the dollars** — a cheaper per-token rate makes a growing
  draw read flat in cost, so the trend is judged on the `cr` column of
  `.metric/stage-economics-log.txt`, never on `cost`. Reading at this verdict:
  align's cache-read fell against the preceding iterations rather than
  continuing to climb, so the growth this watch exists for did not appear in
  the tiering-down iteration. One point is not a trend — keep reading it.

scope, `spec`, and close stay on Opus, their generative and verificational
judgment being what justifies the tier. Re-judge every tier when the harness
model roster churns.

**escalation-guard** — inert, the optional-guard default (as
`guard-kit/templates/wakeup-guard.sh`, unwired in `.claude/settings.json`). A
lead-model session wires `guard-kit/templates/escalation-guard.sh` on
SendMessage per guard-kit/SPEC.md §wakeup-guard.
