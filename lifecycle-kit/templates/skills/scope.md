The `scope` (design) stage of an iteration. Identify tasks, author design
amendments for the items being promoted, promote them into the active queue,
suggest the iteration name. Exit condition: *<your scope exit condition — e.g.
no design-pending tag left in the active queue; your amendment-readiness gate
green>*.

**First step — reset + stamp evidence.** `scope` is the iteration boundary, so
it *resets* the evidence file: **truncate `.workflow/WORKFLOW-STATE.txt` back
to its header** (dropping the prior iteration's stamps — git history is the
permanent audit trail; the gates only ever read the current iteration)*<, and
reset any other per-iteration evidence files your validate stage writes, in
the same reset commit>*. Then append one stamp line
`<iteration> scope <session-id> <date>`, and set the queue header to
`## Iteration: <name>  [stage: scope]` (naming the iteration + flipping the
stage) — the arriving-stage flip the lifecycle mandates, here also naming the
new iteration. A new iteration is `—` until `scope` names it, so the scope
stamp keys on `—` (e.g. `— scope ab12cd34 2026-06-06`). The later stage skills
**append**; only `scope` truncates. This is what `check-stage-evidence`
requires on a stage advance. Honest limit: the stamp proves the skill was
*invoked*, not that the work was done faithfully — it forces deliberate
invocation and an audit trail, nothing more.

## Session ritual

*<Your scope ritual: index the governing docs before reading them whole;
update design-ahead projections (diagrams, models) in scope itself behind a
gate-safe PROPOSED marker so review happens a stage before the code; ask
clarifying questions about scope before proposing.>*

**Triage every task at filing — feature vs debt by the new-names litmus.** A
task that adds any name to a governed surface (a script, a config knob, a
file or directory convention, a tag, a contract another component must
honor) is a feature: author its design amendment in this stage, however
small the diff looks. Debt converges behavior on names the specs already
carry and needs no amendment. The tell for misfiling: a design ruling
longer than a few lines drafted into a queue entry is an amendment inlined
where no gate can see it — move it into one.

Before declaring an amendment ready, verify causal completeness — every new
state, event, or interface the amendment introduces names its **producer**
(what code path or trigger creates it — and that the producer's enabling
config is actually emitted everywhere it must be), its **consumer** (what
receives it, by what mechanism), updates any **existing integration prose**
describing the prior flow, and gives every new field a **named reader** (a
field with no named reader should be removed). A premise inherited from a
queued task ("clean/mechanical", "already filed", "dead code") is a dated
hypothesis — re-verify it against the current tree before building on it.

When done, **set the iteration name without waiting for confirmation** and
inform the user: replace the `—` placeholder in the queue header AND update
the WORKFLOW-STATE scope stamp to match (`check-stage-evidence` requires
header and stamp to agree, so they ride in one commit). This sets only the
*name*. **Do not flip the `[stage:]` line here** — the *arriving* stage's
skill flips it to its own stage as that skill's first step, committed
atomically with its WORKFLOW-STATE stamp. Invoking the next skill is the
stage-advance approval (the name itself needs none).
