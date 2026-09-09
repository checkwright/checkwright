Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

**open-authorization-channel** — the lead skill's own invocation. Typing `/lead`
here *is* the operator's grant; no separate act is looked for and none is asked
for. Cardinality: **one iteration per invocation.** A second iteration wants a
second `/lead`.

**ruling-config** — `.claude/agents/stage-session.md` (dispatch `subagent_type:
stage-session`); its §Ruling classes holds the escalate-vs-decide roster.
Posture: **Split** (template §The lead model). Lead and every stage — scope
included — ride Opus via the agent's `model: opus` frontmatter default, so no
stage inherits the dispatcher's tier (measured leakage was the largest budget
drain). Three stages depart from the Opus default, and by the per-batch tiering
the template §Economics "Tier each batch to its work class" rule now names:

**The split here is by session, not by tier, and the `supervision` row is the
receipt.** The template's stated premise for splitting — what it takes the lead's
turns to be, and why they should therefore stop paying judgment-tier prices — is
the template's own §Economics (*Split the lead where the tail dominates*).

**The method is the template's** — §Economics' character limb states how the count
is taken. The threshold set here was a third such iteration, and **the measurement
is now CLOSED**: twenty-one points, one per close, the third such iteration reached
2026-09-09, and the terminal move executed at the 2026-09-10 close under an
operator ratification (2026-09-09, AskUserQuestion channel in a lead session,
lead-relayed). **The reading is that the judgment tier is confirmed — the split
here is by session, not by tier**, which is what the `supervision` row was the
receipt for.

**What settled it was the limb the count cannot see.** Twice consecutively, at the
last two closes, a stage session declined a relayed lead instruction, ran the
oracle, and corrected the lead — the second time against an instruction more
specific than the first's. A shape seen twice on the limb no count reaches is
stronger evidence than the counts were, and it confirms the tier from the direction
the counts were never able to reach. No later close records a reading, here or
anywhere; the series, its every reading and its refused premises are in git history
with `lead-split-posture-limb-unjudged`. Nothing re-opens this on one point.

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
  spend only says the tier is worth re-judging, never which way.
  The revert signal is a missed spec defect surfacing as a build round-trip. The
  trend is judged on the `cr` column of the bare `align` rows in
  `.metric/stage-economics-log.txt`, never on `cost` and never mixed with the
  `align+fanout` family; the readings taken to date are in git history with the
  retired `lead-split-posture-limb-unjudged`.

scope, `spec`, and close stay on Opus, their generative and verificational
judgment being what justifies the tier. Re-judge every tier when the harness
model roster churns.

**escalation-guard** — inert, the optional-guard default (as `--hook
wakeup-guard`, unwired in `.claude/settings.json`). A lead-model session wires
`--hook escalation-guard` on SendMessage per guard-kit/SPEC.md §wakeup-guard.
Wiring is now a `command` field naming the arm, never a template copied into
`scripts/`.
