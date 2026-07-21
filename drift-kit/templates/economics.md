CONSUMER BINDING — create `.claude/commands/economics.md` as a binding shim
naming this template (a header `Execute the template at <path>, applying the
bindings below.` then a `## Bindings` section) and bind its one slot — the model
posture the report weighs. `/economics` is a post-iteration reporting ritual,
not a lifecycle stage: it moves no cursor and stamps nothing (drift-kit/SPEC.md
§The `/economics` skill), so it is outside the stage roster and never a gate.

Run it at close (the close skill may invoke it) or ad hoc. It answers, for the
iteration just run: what did it cost, where did the cost land, and was the model
posture worth it. Chain the two reporting tools in order, then read their
output into one narrative — do not paste the raw tool dumps:

1. **Governance overhead** — `bash drift-kit/bin/overhead-meter.sh` on the
   closing session (drift-kit/SPEC.md §The overhead meter): the governance-versus-task
   byte proxy for this session.
2. **Stage economics** — `bash drift-kit/bin/stage-economics.sh` (drift-kit/SPEC.md
   §The stage-economics meter): real spend by stage × model × iteration, priced
   through the consumer price table. `cr` (cache-read) is the headline burn field —
   accumulated-context cache-read, not model choice, is the dominant draw, so lead
   the cost story with it. A missing price table degrades cost to `n/a` and the
   tokens still report.

Compose the narrative from those two surfaces:

- **Cost by stage.** From the stage-economics rows, name the priciest stages and
  their token shape; call out `cr` explicitly as the lever, since it dominates.
- **Supervision.** Under a split-lead posture the lead's dispatch, verification,
  and battery burn carries no stage stamp, so it lands in its own row rather than
  inside any stage's total (drift-kit/SPEC.md §The stage-economics meter, the reserved
  `supervision` value). Report it as a named line item beside the per-stage
  bullets — never folded into them — and say plainly that per-stage figures
  exclude it. If the run's caveat named an apportionment key (one lead spanning
  several iterations), repeat the key so the reader can discount the row.
- **Overhead share.** From the overhead meter, state what fraction of the
  closing session was governance versus task work — the methodology's own cost.
- **Posture verdict.** Weigh *<posture: the model posture this iteration ran —
  which model class rode which stages, and where the consumer's ruling records it
  — so the report can judge whether that posture earned its per-stage cost against
  the stage-economics rows.>* against the priced rows: did the stages that rode
  the expensive class earn it, or would a cheaper class have served? State the
  finding; the numbers, not a prior, settle it.

Every emitting surface is advisory (exit 0, no gate): a missing input is a
notice, never a failure. The tools write only under the gitignored metric dir and
emit no account identifiers, so the report carries none either.
