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
(`lifecycle-kit/templates/skills/spec.md`); a default-roster scope that authors
follows it here.

A premise inherited from a queued task ("clean/mechanical", "already filed",
"dead code") is a dated hypothesis — re-verify it against the current tree
before building on it, whether or not this stage goes on to author.

When done, **set the iteration name without waiting for confirmation** and
inform the user: replace the `—` placeholder in the queue header AND update
the WORKFLOW-STATE scope stamp to match (`check-stage-evidence` requires
every stamp's iteration to match the header's, so they ride in one commit).
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
