The `scope` (design) stage of an iteration. Identify tasks, author design
amendments for the items being promoted, promote them into the active queue,
suggest the iteration name. Exit condition: *<exit-condition: your scope exit
condition — e.g. no design-pending tag left in the active queue; your
amendment-readiness gate green>*.

**First step — reset + stamp evidence.** Run lifecycle-kit's
`bin/enter-stage.sh scope`. `scope` is the iteration boundary, so the tool
*resets* the evidence file: it truncates `.workflow/WORKFLOW-STATE.txt` back
to its header (dropping the prior iteration's stamps — git history is the
permanent audit trail; the gates only ever read the current iteration), stamps
`— scope <session-id> <date>` under the unnamed-iteration sentinel, and sets
the queue header to `## Iteration: —`. It reads `<session-id>`
from `bin/session-id.sh` itself (the newest transcript — never hand-picked),
uses `date +%F`, and refuses (writing nothing) if `check-stage-entry` is red.
*<evidence-reset: reset any per-iteration evidence file your validate stage
writes that the tool does not already truncate — the tool truncates
WORKFLOW-STATE, the lesson-evidence file, and every `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`
member, in the same commit as this stamp.>* A new
iteration stays `—` until this skill names it (below); only `scope` resets,
later stages append. Honest limit: the stamp proves the skill was *invoked*,
not that the work was done faithfully — it forces deliberate invocation and an
audit trail, nothing more.

## Session ritual

*<ritual: your scope ritual: index the governing docs before reading them
whole; update design-ahead projections (diagrams, models) in scope itself
behind a gate-safe PROPOSED marker so review happens a stage before the code;
ask clarifying questions about scope before proposing; name the grammar owner
of each governed surface this stage writes — queue-entry tags, amendment
refs — so a session reads the owner instead of re-deriving the grammar from
a gate's source.>*

**Triage every task at filing — feature vs debt by the new-names litmus.** A
task that adds any name to a governed surface (a script, a config knob, a
file or directory convention, a tag, a contract another component must
honor) is a feature: author its design amendment in this stage, however
small the diff looks. Debt converges behavior on names the specs already
carry and needs no amendment. The tell for misfiling: a design ruling
longer than a few lines drafted into a queue entry is an amendment inlined
where no gate can see it — move it into one.

**A standing directive is a theme, not a unit list.** A directive received from
a lead or operator at iteration open bounds this survey — it names the theme,
never the units. The intake sweep and the premise re-verification run regardless,
and the proposed unit set is escalated for ruling before promotion: to the lead
under the split posture, to the user directly under none — the stage's ordinary
stop. The destination changes; the proposal step does not.

Before declaring an amendment ready, verify causal completeness — every new
state, event, or interface the amendment introduces names its **producer**
(what code path or trigger creates it — and that the producer's enabling
config is actually emitted everywhere it must be), its **consumer** (what
receives it, by what mechanism), updates any **existing integration prose**
describing the prior flow, and gives every new field a **named reader** (a
field with no named reader should be removed). Survey those readers across the
**whole component set**, never a hand-picked subset, and never silence a
probe's stderr — a `2>/dev/null` on a path grep reads a bad path as "no
reader", manufacturing the false negative that hides a cross-component reader.
A premise inherited from a
queued task ("clean/mechanical", "already filed", "dead code") is a dated
hypothesis — re-verify it against the current tree before building on it.

When done, **set the iteration name without waiting for confirmation** and
inform the user: replace the `—` placeholder in the queue header AND update
the WORKFLOW-STATE scope stamp to match (`check-stage-evidence` requires
every stamp's iteration to match the header's, so they ride in one commit).
The header carries the *name axis alone* — there is no stage field to set, and
no stage field to advance. The *arriving* stage's skill moves the cursor by
stamping, as that skill's first step. Invoking the next skill is the
stage-advance approval (the name itself needs none).

Close by **recommending the next stage**: the trigger-gated audit stage when
one of its triggers fired this session, otherwise the build stage. The
cross-component trigger is mechanical — an amendment on disk spanning ≥2
component dirs makes `check-stage-entry` assertion C demand the audit stamp
(or a user-ruled waiver) at build entry — so a scope that promoted such an
amendment should say so rather than let build discover it.

With the promotion commit landed, the iteration named, and the next stage
recommended, present the consumer's **hand-off** — how this consumer carries a
closed scope into the remaining stages: *<handoff: the consumer's start-sequence
choice at this boundary — driving the rest under a lead versus steering the
stages by hand, or a plain "no lead — run the stages by hand" line for a
lead-less or harness-less consumer; point at the consumer's documented
start sequence by citation, never restating its steps here>*.
