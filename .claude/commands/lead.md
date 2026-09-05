Execute the template at lifecycle-kit/templates/lead.md, applying the bindings below.

## Bindings

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

**The method is the template's** — §Economics' character limb states how the
count is taken. **What is set here is the threshold:** a third such iteration.
One iteration is one point; do not flip on it either way.

**Every reading is the entry's, not this file's.** The cost series, the ruled-alone
counts, the counters and the refused premises live on
`lead-tier-split-premise-unamended`; close records each iteration's reading there,
and this file carries no datum.

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
  `align+fanout` family; the readings and their conclusions live on the entry
  named above.

scope, `spec`, and close stay on Opus, their generative and verificational
judgment being what justifies the tier. Re-judge every tier when the harness
model roster churns.

**escalation-guard** — inert, the optional-guard default (as `--hook
wakeup-guard`, unwired in `.claude/settings.json`). A lead-model session wires
`--hook escalation-guard` on SendMessage per guard-kit/SPEC.md §wakeup-guard.
Wiring is now a `command` field naming the arm, never a template copied into
`scripts/`.
