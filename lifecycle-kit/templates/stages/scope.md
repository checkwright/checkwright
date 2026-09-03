The `scope` stage of an iteration — the exploratory half of design. Identify and
bound the iteration's units, promote them into the active queue — debt always,
features only where scope also authors (below) — and suggest the
iteration name; author the promoted features' design amendments here **unless
the roster splits out a dedicated authoring stage** (below), which then owns
them. Exit condition: *<exit-condition: your scope exit
condition — e.g. no design-pending tag left in the active queue; your
amendment-readiness gate green>*.

**First step — reset + stamp evidence.** Run lifecycle-kit's
`bin/enter-stage.sh scope`. `scope` is the iteration boundary, so the tool
*resets* the evidence file: it truncates `.workflow/WORKFLOW-STATE.txt` back
to its header (dropping the prior iteration's stamps — git history is the
permanent audit trail; the gates only ever read the current iteration), stamps
`— scope <session-id> <date> <head>` under the unnamed-iteration sentinel, and sets
the queue header to `## Iteration: —`. It reads `<session-id>`
from the `--emit-session-id` arm itself (the newest transcript — never
hand-picked), uses `date +%F`, and refuses (writing nothing) if `check-stage-entry` is red.
On a refusal, **do not force the entry** — escalate to the lead (where one
exists and this is not a standalone session) and stop; a refused entry is a gate
verdict to resolve at its source, never to override.
*<evidence-reset: reset any per-iteration evidence file your validate stage
writes that the tool does not already truncate — the tool truncates
WORKFLOW-STATE, the lesson-evidence file, and every `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`
member, in the same commit as this stamp.>* A new
iteration stays `—` until this skill names it (below); only `scope` resets,
later stages append. Honest limit: the stamp proves the skill was *invoked*,
not that the work was done faithfully — it forces deliberate invocation and an
audit trail, nothing more.

**Second step — take the carried gap bullets, if the entry reported any.** The
entry admits a boundary whose gap inbox holds bullets the closing stage could
not have drained, and prints them as an advisory naming them this session's
intake (lifecycle-kit/SPEC.md §The committed gap inbox owns why it admits rather
than refuses). Each carried bullet gets **exactly one** disposition — promoted to
a queue entry, fixed inline this session, or discarded with cause in the commit
message — after which this session truncates the inbox to its `# contract:`
header **in the same commit**. That is the drain's own disposition set, run by
the stage that can now legally run it; no linked-and-skipped middle state, per
the gap-disposition rule, and deleting a bullet without a disposition is not a
drain.
Two properties are obliged rather than suggested. The disposition happens
**after the stamp**, in-stage — that is the whole point of admitting the entry,
since a pre-stamp queue write is attributed to a stage that has not started. And
a promoted entry's provenance sentence carries the **bullet's own date** and
names the iteration whose close generated it, because the finding's disposition
lands in this iteration's ledger while the finding belongs to the last one; a
record that is late and says so is what that trade buys.

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
honor) is a feature: its design amendment is authored — in this stage unless
the roster splits authoring into a dedicated stage (below), however
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

A **deferred** entry whose recurrence count has reached
`LIFECYCLE_KIT_RECURRENCE_THRESHOLD` enters the proposed unit set **regardless of
theme**, and rides that same escalation. The directive still bounds the survey;
what it may no longer do is silently outrank a counted recurrence. The count is
the number of dates on the entry's `recurrence:` declaration
(queue-kit/SPEC.md §The tag algebra) — one anchored grep over the deferred
section, no tool and no queue-kit dependency. Each of those dates is a **judged**
recurrence, stamped by a session that read the grounds rather than derived from a
string match, so a count reaching the threshold is a signal to act on rather than
a number to sanity-check first. It is scoped to deferred entries
because promotion is the decision it forces; an entry already active is being
built. **The collision is decided, not resolved in the theme's favour**: the rule
does not promote, it puts the unit in front of the authority this stage already
escalates to. An automatic promotion would be a second intake path around
scope-gated intake, and the failure this rule answers was never that the ruling
went the wrong way — it was that the collision never reached anyone.

**Weigh the iteration's cost before opening it — the economic composition test.**
A lone sub-threshold unit may not justify a whole iteration's fixed cost: either
bundle related-surface deferred entries into the iteration, or argue the unit is
significant enough to stand alone. The cost principle — amortize a fixed setup
cost across a shared-surface batch — is the lead's, owned by
lifecycle-kit/templates/lead.md §Economics and applied there to dispatch
batching; scope applies the same principle one axis over, to iteration
composition. Cite it and weigh it here; do not re-derive the economics.

**Scope's censuses are the ones later stages most often re-buy, so record
them.** A roster built by applying a criterion set to a corpus dies with this
session's context unless it is written down, and the stage that needs it next
has no artifact to read. File it, and read the record before dispatching a
survey of your own: lifecycle-kit/SPEC.md §The survey record owns the block
grammar, the two-command witness, and the rule that a carried survey is cited
only while that witness holds.

**Authoring the promoted features' amendments is scope's — unless the roster
splits out a dedicated authoring stage.** Where it does, that stage authors the
amendment(s) and pairs the feature entries into the queue, and scope keeps only
the exploratory half: survey, propose, name, and debt promotion — ending at the
boundary so the survey context drops before authoring begins (that drop is the
split's whole economics). Feature promotion is not a separable step that could
fall to either stage: writing the amendment *is* promoting the entry
(canon-kit/SPEC.md §The amendment lifecycle, the bidirectional rule), so the
stage that authors is necessarily the stage that promotes. Either way the
detailed authoring how-to — causal
completeness (every new field's producer, consumer, and named reader, surveyed
across the whole component set) and the bidirectional queue pairing — is
single-sourced in the authoring stage's template
(`lifecycle-kit/templates/stages/spec.md`); a default-roster scope that authors
follows it here.

A premise inherited from a queued task ("clean/mechanical", "already filed",
"dead code") is a dated hypothesis — re-verify it against the current tree
before building on it, whether or not this stage goes on to author.

That re-verifies the entry's *own* claims — which is the half a survey already
reads. The other half is what the **rest of the queue** says about that entry:
its inbound citations, summed. Reading sibling entries one at a time never
produces that sum, so a unit that several siblings separately converge on,
subsume, or block against can be read carefully and still ranked low — the
failure is aggregation, not retrieval, and no amount of following
cross-references fixes it. So before ranking a candidate, aggregate its inbound
edges (queue-kit/SPEC.md §The queue-edges arm) and read what cites it. **The sum
lands in the survey record's `edges` field** (lifecycle-kit/SPEC.md §The survey
record), which is what makes it readable at the next boundary instead of dying
with this session — a sum recorded only inside the finding's prose is present
and unaddressable, and the next boundary re-buys it. **Read the
same output's retired block, which is an input to this ranking and not a
footnote to it:** a citation resolving to a slug that was once live and is not
now points at *disposed* work, so an entry arguing from one is arguing from a
premise that has already been settled or shipped — and it reads exactly like a
live one until the block names it. That is the failure this stage is on record
for committing, against its own survey. The
promotion dividend lives in the total and in no single entry, which is also why
splitting an entry is safe only against that total: a split scatters an entry's
weight across siblings, and the sum is what puts it back together.

When done, **set the iteration name without waiting for confirmation** and
inform the user: run `bash lifecycle-kit/bin/enter-stage.sh --rename <name>`,
which writes both surfaces — the queue header and column 1 of every stamp — in
one motion and names them for the one commit they ride in. Never edit either by
hand: `check-stage-evidence` requires every stamp's iteration to match the
header's, and the state file has one sanctioned writer.
The header carries the *name axis alone* — there is no stage field to set, and
no stage field to advance. The *arriving* stage's skill moves the cursor by
stamping, as that skill's first step. Invoking the next skill is the
stage-advance approval (the name itself needs none).

Close by **recommending the next stage**: where the roster splits out a
dedicated authoring stage and this iteration promoted a feature, that stage
(it authors the promoted amendments); otherwise the trigger-gated audit stage
when one of its triggers fired this session, else the build stage. The
cross-component trigger is mechanical — an amendment on disk spanning ≥2
component dirs makes `check-stage-entry` assertion C demand the audit stamp
(or a user-ruled waiver) at the next stage's entry — so whichever stage
promoted such an amendment should say so rather than let the entry discover it.

With this stage's promotion commit landed (debt, plus the features only where
scope authored them), the iteration named, and the next stage
recommended, present the consumer's **hand-off** — how this consumer carries a
closed scope into the remaining stages: *<handoff: the consumer's start-sequence
choice at this boundary — driving the rest under a lead versus steering the
stages by hand, or a plain "no lead — run the stages by hand" line for a
lead-less or harness-less consumer; point at the consumer's documented
start sequence by citation, never restating its steps here>*.
