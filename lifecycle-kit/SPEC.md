# lifecycle-kit — an evidence-stamped iteration lifecycle for stateless agent sessions

An iteration is a self-contained work cycle driven through a configurable
sequence of stages (default: `scope → [align] → build → validate → close`).
The problem the kit solves: a stateless agent session cannot be trusted to
remember, or even re-read, a process document — so the process state lives in
two governed files a gate can read, and every stage transition leaves
machine-checkable evidence.

The kit carries the generic state machine only; a consumer's stage names,
exit conditions, and ritual content are config and skill-template fill-ins.
Requires [gate-sdk](../gate-sdk/) (the gates follow its four contracts and
resolve through its registry).

## The state machine

Two governed surfaces, carrying **one axis each**:

- **The header line**, at the top of the consumer's queue file (default
  `TASK-QUEUE.md`), carrying the *slow* axis — the iteration name, and nothing
  else:

  ```
  ## Iteration: <name>
  ```

- **The evidence file** (default `.workflow/WORKFLOW-STATE.txt`): free prose,
  then a `---` separator, then one data line per stage-skill invocation:

  ```
  <iteration> <stage> <session-id> <YYYY-MM-DD> <head>
  ```

  It carries the *fast* axis too: **the last data line's `<stage>` token is the
  cursor** — the single source for "which stage is this iteration in".

  `<head>` is the abbreviated commit `bin/enter-stage.sh` read at the instant
  it wrote the stamp, or the literal `none` where it found no git work tree and
  no commit to name. It is **required**, and `none` is a value rather than an
  omission: a permanently optional field is a permanent disarm switch for the
  provenance assertion it exists to serve (§check-stage-evidence), which would
  be enforcement-first inverted — the fix shipped beside its bypass. The field
  is **appended** rather than inserted, so every positional reader of fields one
  to four is unmoved.

  **The five-field grammar is a breaking change to a shipped file format**, and
  a consumer vendoring it mid-iteration reds until they rewrite their own
  stamps: for each, `<head>` is the first parent of the commit that introduced
  that stamp line, recoverable from history, and `none` where it is not — a
  stamp whose introducing commit cannot be identified takes `none` rather than
  a guess, because a guessed provenance value is indistinguishable from a real
  one afterward. That rewrite is a **rewrite and not an introduction**, which is
  what the migration clause of the newly-introduced test (§check-stage-evidence)
  exists to say. The honest mitigation is the boundary: the first stage's reset
  truncates this file, so a consumer upgrading at an iteration boundary pays
  nothing.

The header once carried a `[stage:]` field as well. It was a second copy of a
derivable fact — every stage entry already stamps its `<stage>` — bought at one
queue write per stage entry, and kept in sync only by an assertion that existed
for no other purpose. Deriving the cursor from the stamps retires the copy, the
write, and the assertion together (derivation-first). A consumer upgrading
mid-iteration needs no migration step: every header reader strips an optional
trailing bracketed field, so a residual `[stage:]` is inert and the next
iteration-boundary reset rewrites the header without it.

Both surfaces are **single-writer and branch-scoped**: an iteration owns exactly
one branch (its home branch) and every stamp lands there, so
concurrency between operators is git branch topology, not a multi-writer state
file. The integration branch is the degenerate single-operator home; a second
concurrent operator cuts a branch at their scope entry. The merge semantics that
make this safe — the iteration-scoped surfaces resolve to the arriving branch at
a merge — are §Multi-operator semantics.

The default motion is the linear stage walk; the gate-legal shapes for leaving
it — abandon, split, reopen — are specified in §Deviation transitions, not
improvised.

**A stage owes a resume journal, and the obligation is the stage's rather than
the dispatch's.** delegation-kit owns the journal *contract* — what a session
writes into it, when, and who may delete it (delegation-kit/SPEC.md §Resume
journal — agent writes, scratch reset sweeps). What lives here is the **path**,
because only the stage machine knows the stage: `LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN`
carries a `<stage>` placeholder, and expanding it is the one derivation a
dispatching supervisor, a stage session and a stage entry all read. Nothing about
the contract is restated here and nothing about the path is stated there — the
owner-and-pointer split, held across the seam rather than inside one surface.

**The derivation is the enabling move, and it is why this was unoracled before.**
A path invented per dispatch leaves no record on disk of what was granted, so no
gate and no entry can name the file a stage owes; two dispatchers can also
disagree about where one stage's journal lives. The supervisor still spells the
path out in the prompt — an agent cannot read a knob it has no reason to look for
— but that grant is now a **restatement of a derivation** rather than its only
source, which is what makes it checkable.

**One journal per stage, appended by every session of that stage.** A stage that
runs several sessions — the implementation stage runs one per task by contract —
does not get several journals: the path is a function of the stage, so each
session appends to the file its predecessors wrote, under a heading naming
itself. Two consequences are stated because each looks like a defect until it is
read. The `DONE`-as-last-line rule survives unchanged and reads *better* at this
granularity: an earlier session's marker sitting mid-file is not a lie about that
session, and the file's last line still answers *did the session that wrote last
finish*. And the entry assertion below is satisfied by **any** session of the
predecessor stage having written, which is the correct question — the successor
needs the predecessor's reasoning, not one particular session's.

**Measured against this repo's own journals rather than asserted.** The default
`<scratch dir>/<stage>-journal.md` is the shape sessions here converged on
unaided, which is the argument for deriving it rather than inventing one — but
the convergence is not total, and the exception is the case the rule above
exists for: on an iteration whose implementation stage was split into batches,
the two batch sessions each wrote a discriminated name of their own. The
derivation overrides that: the discriminator belongs in a heading inside the
stage's journal, never in its filename, or the successor's assertion has no file
to name. **The honest limit**: two sessions of one stage running *concurrently*
append to one file with no coordination beyond append atomicity, which is
untested here and filed rather than claimed safe.

### The stamp protocol

The **arriving** stage's skill *stamps* the evidence file as its first step —
and that stamp is the whole transition, because the last stamp is the cursor.
Nothing flips. The departing session writes nothing, so no uncommitted stage
line crosses the session boundary. The entry commit stages the evidence file,
so every state-coupled gate re-fires on it: the prior stage's
machine-expressible exit is re-verified *at the entry* (`check-stage-evidence`),
and `check-stage-entry` extends that one hop back. A self-asserted "stage
complete" marker would prove a claim, not completion — the kit deliberately
has none.

**What the entry re-fires, and what it does not.** The entry commit stages the
evidence file alone, so the gates coupled to it — `check-stage-entry`,
`check-stage-evidence`, `check-evidence-manifest`, `check-trajectory-fresh`,
and any gate globbing the workflow dir — re-fire at every entry exactly as
before. Queue-only-coupled gates do not: the queue is not written at an entry,
so their re-run would be a no-op on unchanged input, and each of
them couples some *other* input that fires it on the change it actually gates.
What is genuinely given up is the incidental *periodic sweep* — the guarantee
that those gates ran at least once per stage regardless of what changed, which
could catch drift introduced out of band (a `--no-verify` commit, an edit
outside every coupled glob). The full battery at the validate stage is the
surviving sweep and the stronger one: it runs every gate, not the
queue-coupled subset. The per-entry queue re-fire was a side effect of the flip,
never the designed sweep — an accepted, costed loss.

Mid-iteration the queue file is written only for real **work-state**
transitions: promotion and naming (the first stage), the Done move riding each
amendment-merge commit, and the closing dispositions. Stage motion never
touches it.

**A stage session landing a ruling writes its provenance in the same commit as
the ruling's content.** The queue's `ruled:` declaration (queue-kit/SPEC.md
§The tag algebra) names who ruled, when, and through what channel; this section
owns *when* the session writes it, because the session is the party whose commit
is the audit artifact. Not afterwards, and not in a later pass: the relay that
carried the ruling is transport, never a store, so a ruling landed without its
declaration has already lost the only party who could attest it, and the next
session has no way to tell a relayed ruling from an invented one. Where the
relaying party stated no authority, the session **asks** rather than defaulting
to the higher one — reading an authority into a silence is the invention the
declaration exists to prevent, and inflating a relaying party's own ruling to the
authority above it freezes a decision that should have stayed re-rulable at the
relay. The obligation binds both parties and neither half discharges the other's:
the relay states the facts (`templates/lead.md`), the landing session records
them here.

The **deterministic half** of that first step — read the iteration from the
header, read the id from the `--emit-session-id` arm, append the stamp — is
mechanized by
`bin/enter-stage.sh <stage>`, the same
writer/asserter split as `gen-pre-commit.sh` ↔ `check-graph`: the skill
invokes it, **judgment stays in the skill** (what the stage means, its exit
condition, when to enter it at all), and the stage gates stay the independent
verifier. The tool takes no `--force` flag, so the compliant path is the easy
one — an operator who intends to override writes the stamp by hand, exactly
as before the tool existed. Committing the stamp remains the skill's
business, on its own — with **one** exception, stated here rather than only
where the valve is, because a session reads this rule at its entry step and the
valve's contract several sections away: an entry the one-shot pre-flight valve
admitted rewrites the valve ledger in the same motion as the stamp, so the two
commit together and the purity assertion exempts exactly that path
(§bin/enter-stage.sh, §check-stage-evidence). And **never with `--no-verify`**:
`enter-stage.sh` refuses to write
while `check-stage-entry` is red, so the hook a bypass skips is exactly the
battery that would confirm the stamp just written. A stage entry is never the
one-off-with-cause that a bypass is reserved for.

The first stage is the iteration boundary: `enter-stage.sh` *truncates* the
evidence file back to its header (git history is the permanent audit trail;
the gates only read the current iteration) and stamps under the
unnamed-iteration sentinel `—`, rewritten to the real name when the stage
names the iteration. Later stages only append.

**The no-cursor window.** Because the cursor is the last stamp, it has an empty
state the header never had — and one of its two shapes is reachable in normal
operation. The boundary truncation leaves the evidence file holding its prose
preamble and `---` with **no data line**, and the cursor is empty until the
boundary stamp lands milliseconds later; the second shape is an absent file
entirely (an unvendored or pre-upgrade consumer). Every reader of the cursor —
inside this kit and in every consuming kit that derives it — **states its own
behavior for that window** rather than inheriting whatever its parser happens
to emit. The shared derivation `lifecycle_current_stage` reports it as empty
with a *success* status, because "no cursor" is a legitimate state and not an
error; what it means is the caller's ruling, and each caller's is recorded in
its own section.

**Honest limit:** a stamp proves the stage skill was *invoked*, not that its
work was done faithfully — strictly better than skip-and-no-trace, but not
proof of done. The `<session-id>` field is **read, not hand-picked**:
the `--emit-session-id` arm prints the canonical id by a fixed derivation
order (§bin/session-id.sh), which rotates per session (including across a context
clear), and the stage skills stamp exactly what it prints, so each stage's
provenance is observed, not guessed.
`check-stage-evidence`'s invocation floor keys on `<iteration> <stage>`; it
additionally reads the session id to enforce cross-stage distinctness under
the default `stage` posture (a stage entry must carry a fresh session; the
`iteration` posture of `LIFECYCLE_KIT_SESSION_BOUNDARY` relaxes exactly this —
see §check-stage-evidence).

**Same-stage re-entry (N sibling sessions per stage).** Entering the
currently-stamped stage from a *new* session is legal and appends a fresh
stamp: `enter-stage.sh`'s idempotence guard keys on the full
`(iteration, stage, session-id, head)` tuple, so only the *same* session
re-entering **at the same commit** is a no-op — a re-entry after `HEAD` moved
appends a fresh stamp carrying the new head, which is what makes re-running the
tool the stated remedy for a stale recorded head rather than a reported no-op
that changes nothing (§check-stage-evidence). `check-stage-entry` assertion A
keys on the *predecessor* stamp,
which the stage's first entry satisfied for every sibling. So N sessions may
enter one stage — a multi-session build, or a lead's intra-stage batch split
(§templates/lead.md) — serialized by the shared index/HEAD like any concurrent
sessions; each leaves its own stamp, so per-batch provenance rides the existing
stamp grammar with no new field. A sibling's entry simply appends another stamp
naming the same stage, which leaves the cursor where it already was — there is
no once-per-stage write left to make idempotent.

**The optional lead never becomes a second state source.** An iteration may run
with a live *lead* session (§templates/lead.md) that dispatches its stage
sessions and answers their escalations so a blocked stage resumes in place
rather than restarting. The lead writes no state: every stamp
originates in a stage session through `enter-stage.sh`, exactly as above, so the
stamp protocol stays the only iteration state and a lead crash costs
nothing the tracked surfaces do not already hold. The lead is a boundary skill,
not a stage — it stamps nothing and joins no stage set, so the coverage gate
never reads it (the release-sweep precedent, §templates/lead.md).

**Honest limit on the lead's dispatch precondition.** A lead dispatches stage
N+1 on stage N's **agent completion notification**, never on an artifact — not
its commit, its stamp, a clean tree, a green battery, or a cleared `--simulate`
(§templates/lead.md). That precondition is **prose-only and human-enforced**, and
the cause is structural rather than budgetary. The signal's **producer is the
harness**, emitting it when the dispatched session's turn ends — outside every
governed tree, which is not a gap to close but the direct reason no gate can read
it; its enabling configuration is the dispatch itself, since the lead dispatches
in the background with notification (delegation-kit/SPEC.md §The delegation
model), so the producer is reachable on the ordinary path. Its **consumer is the
lead**, at the dispatch transition for stage N+1, by the lead's own in-turn wait
rather than a read. Its **truthfulness** is not the harness's to guarantee: a
dispatched session that ends its turn on still-running work emits a notification
that lies, which is what delegation-kit/SPEC.md §Operative residency exists to
prevent. The limit is recorded here rather than left to be inferred from the
absence of a gate, because an unstated version reads as an oversight for a later
session to fix by building the impossible gate. Naming it is also what routes the
enforcement duty to where it *can* be discharged: the **negative is** assertable
from the artifact side. A producer-liveness gate wired into
`LIFECYCLE_KIT_ENTRY_PREFLIGHT` (evidence-kit/SPEC.md §check-producer-liveness)
answers *is the producer still running?* independently of the signal, which is
what covers the case where the signal itself is wrong. Precedent for a
prose-only rule in the same template: the no-sibling-dispatch clause, prose for
the same reason.

**Honest limit on the lead's post-dispatch check routing.** The rule that the
lead's verify of the evidence-producing stage is a **read** of the committed
evidence rather than a re-run of the producer (§templates/lead.md, applying
delegation-kit/SPEC.md §Verify after every agent commit) is likewise
**prose-only and human-enforced**, and for the generic reason that section
states: the subject is a lead's *choice of command*, which leaves no tracked
artifact any check could read. The nearest artifact-side proxy is
`check-producer-liveness` on the entry-preflight hook (evidence-kit/SPEC.md
§check-producer-liveness), and its coverage boundary must be stated precisely or
this limit reads as closed when it is not: that gate answers *is a producer
running now*, so it covers the **concurrent** case — a producer still live when
the next entry is stamped — and not a **sequential** re-run, where a lead
re-runs the producer after it has cleanly exited, holds no lock, and reds
nothing. The two limits in this section are the same shape from opposite sides
of one dispatch: the one above is about waiting for stage N to be *over*, this
one about which command is safe *once it is*.

### Deviation transitions

The stages walk `scope → align → build → validate → close` in order by
default; the gate-legal shapes for leaving that walk are specified here, not
improvised. Each composes mechanism the kit already owns — `enter-stage.sh`'s
boundary reset, canon-kit's amendment pairing, queue-kit's tag algebra — so
**no new tooling, state, stamp grammar, or tag is introduced**, and a
harness-less consumer keeps every shape. `check-stage-entry` and the stamp
protocol bar an ad-hoc abandon, which is why each hatch is spelled against the
existing gates.

**The demote ritual** is the shared step the other shapes compose. To take a
promoted entry out of a live iteration: move it back to the deferred queue
section restoring its design-pending tag, and delete its amendment file in the
same commit. Git history preserves the design — a later scope re-promotes by
resurrecting the file from history rather than re-deriving it. The enforcement
is already on the books: canon-kit's `check-amendment-queue` reds the commit on
a deferred entry that still carries a spec ref, or an orphaned amendment left on
disk. If the validate baseline carries a scenario keyed to the demoted entry,
that scenario is re-scoped or removed in the same commit — a coverage-honesty
obligation, not a gate one (`check-evidence-baseline`'s slug-liveness passes
regardless, since a demoted entry stays a live queue task).

**Abandon** ends an iteration without a close. Disposition every active entry
explicitly — demote it (ritual above) or carry it (it stays active with its
amendment and the next iteration adopts it); sink or delete every Lessons entry
under the existing disposition rules (the first-stage entry refuses a non-empty
Lessons section, so this is already forced). Then the next `enter-stage.sh
scope` *is* the abandon: scope has no mandatory predecessor, so the entry is
gate-legal from any stage, and the boundary reset drops the dead iteration's
stamps exactly as it drops a closed one's (git history is the permanent audit
trail — the existing boundary doctrine, not a new rule). The abandon commit's
subject names the abandoned iteration; no stamp grammar changes.

**Split mid-flight** narrows a live iteration. The iteration name never changes
once set — every stamp already written carries it, and a rename-in-place is
barred because it would orphan those stamps against `check-stage-evidence`'s
name-axis agreement — its staleness assertion reds every stamp whose iteration
is not the header's. So splitting is demotion: demote the split-out subset
via the ritual and drive the remaining queue through the remaining stages; the
subset re-promotes at a later scope under its own iteration.

**Reopen after close** is barred as an in-place edit. Stamps are append-only
within an iteration and scope is the only reset, so there is no gate-legal way
to continue a closed iteration's evidence file — and no history rewrite is
sanctioned to fake one (doctrine-kit's *Re-verify volatile state before a git
history rewrite* territory — cited by name, since a rule number re-arms the
drift every later insertion causes). The sanctioned shape
is a successor iteration: a post-close defect files as a debt entry and the
follow-up iteration proceeds normally; the closed iteration's record stays
immutable.

**The interstitial mitigation** is the shape that fits *between* a close and the
next scope. A repo-local mitigation for a recurring incident may land directly in
that window while the incident's queue entry stays open for the kit-shaped form.
One cap makes it admissible: **it adds no governed name** — canon-kit's
feature/debt litmus read the other way (canon-kit/SPEC.md §The amendment
lifecycle, where a task adding any name to a governed surface is a feature and
needs an amendment). So an interstitial landing is admissible exactly when it is
debt-shaped; anything feature-shaped waits for scope. The entry stays open either
way, because the mitigation is repo-local and the entry's deliverable is not. Like
every shape here it introduces no tooling, state, stamp grammar or tag — the cap
is an existing litmus and the landing is an ordinary commit. It **widens Reopen
after close** rather than competing with it: routing a post-close defect to a debt
entry and a normal follow-up iteration is the right answer for a first occurrence
and the wrong one for the fourth, and the fourth is now countable
(queue-kit/SPEC.md §The tag algebra, the `recurrence:` declaration). The commit
accounting already holds and needs no change — an interstitial commit falls into
the *next* iteration's range and surfaces when that iteration closes
(drift-kit/SPEC.md §The published-evidence extractor).

**A parallel hotfix track is refused**, recorded here so it is not re-proposed: it
violates scope-gated intake, it contends on live stage surfaces, and most incident
fixes are feature-shaped — so a "hotfix" of them is an unreviewed iteration. The
lane this shape governs already half-existed; what was missing was the signal that
says when to use it.

**The close-merge** is the concurrent-close shape, and like the others it
composes existing mechanism (the merge-supersede rule of §Multi-operator
semantics, not new tooling). Iteration boundaries serialize on the integration
branch: the closing operator reconciles *on their iteration branch* — merges the
integration branch in, where the `merge=iteration-scoped` driver resolves the
iteration-scoped surfaces to their own (arriving) side and humans resolve the
content conflicts — re-runs the full battery green, then lands
fast-forward-only on the integration branch. The integration branch never hosts
a conflict resolution, so "arriving iteration" is always well-defined (ours on
the iteration branch) and every merged tree passed the battery post-reconcile.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `lifecycle-kit/`); its
gates are registered in the consumer's `gates.list` by name and resolve
through gate-sdk's multi-kit path (consumer gates dir first, then each kit's
`checks/`). The kit's markdown templates are laid out on the stage/boundary axis
this SPEC classifies them by: `templates/*.md` is exactly the **boundary skills**
(`lead.md`, `release-sweep.md`, `upgrade.md`, `consult.md`),
`templates/stages/*.md` exactly
the stage-class templates. Stage skills adopt `templates/stages/*.md` in one of
two modes — copied into the consumer's agent-skill directory with each named slot
overwritten, or a thin binding shim that references the template (the grammar
and the contract both modes satisfy are §templates/stages/).

The stage machine itself is config with this repo's lifecycle as the
default: copy `templates/lifecycle-config.sh` into the gates dir as
`lifecycle-config.sh` (or point `LIFECYCLE_KIT_CONFIG_FILE` elsewhere) and
set only what you override — this roster owns every knob and its default;
the template carries no second copy. The loader validates the machine
(unknown stages in the map, a waiver token colliding with a stage name, a
non-integer n-gram width, a malformed preflight entry) and exits 2 on a
malformed config — a broken machine must not gate anything. That template and
the copy it seeds are **permanently shell**, each carrying the `# no-port:`
cause of the class ruling at gate-sdk/SPEC.md §The config-seam port disposition.

**No knob carries the ruling-authority vocabulary, and this is where a reader
looking for one finds out why.** §The state machine obliges a stage session to
record who ruled a ruling it lands, and the authorities a project recognises are
its own governance roles — named on the consumer's always-loaded surface, not
here. No kit mechanism reads those values: nothing in this kit branches on them,
so a knob holding them would have `check-knob-citation` as its only reader, which
is a knob that should not exist. The grammar itself is queue-kit's
(§The tag algebra owns the slot); this kit owns only the timing.

Knob-rename compat precedent. A rename carries two obligations of different
natures, and each answers to its own threshold.

**Declaration — owed from the first release tag, unconditionally.** A tagged
release is a distribution, and whoever vendored it reads the note to learn what
moved. From the first tag onward a rename owes a tightened-gates/release-note
declaration (the deprecation-lifecycle and upgrade-path rungs).

**Compat shim and deprecation window — owed from the project's declared
general-availability posture onward, never from a tag.** While a project's own
declared stability posture is pre-general-availability, a knob rename is
compat-free: no read-the-old-name shim, no deprecation window, no queue-bound
deprecation marker. The declaration is still owed, so a consumer who vendored
early is never surprised. They are told; they are simply not carried.

The observable is a declared posture rather than an adoption count, because the
obligation is normative rather than empirical. You owe a migration path because
you promised stability, not because you counted users. A project that has
promised nothing owes nothing, and owes it the instant it promises, whether or
not anyone has installed. "The first observed external install" reads the premise
most literally and is rejected: it cannot be falsified from the tree, and it
makes the obligation turn on a fact outside the project's control. A tag was a
bad proxy for that same reason rather than one that merely aged — it measures
distribution where the obligation tracks promise, so it would have drifted under
any release history, fast or slow.

**Fail-safe direction.** A project declaring no stability posture at all is
treated as **past** the threshold: the compat obligation applies. Silence must
not read as still-pre-GA, because that is the reading under which a project that
never got round to declaring anything grants itself a permanent exemption. The
window is opened by an explicit pre-GA declaration and by nothing else.

The clause is knob-scoped, and the scoping is substantive rather than accidental:

1. **The mechanism it points at is knob-shaped.** The deprecation path it invokes
   is the queue-bound deprecation markers and the release note's `Renamed knobs`
   section, whose `old → new` / `old → ∅` grammar is specific to config names. A
   rule reaching gate names or file conventions would point at a mechanism that
   does not accept them.
2. **No class is left without a home.** A non-knob rename — a gate name, a file
   or directory convention — is structurally accommodated by the release note's
   `Behavior changes` section, whose bullet lead is defined as the changed
   surface's name (script, knob, template, or file), a definition that already
   admits each of those classes. What the note grammar does not yet carry is a
   sentence explicitly *routing* non-knob renames there, so this is a sound
   structural inference rather than an established convention. The narrow scoping
   leaves no rename without a section shaped for it, which is all this reason
   claims.
3. **Widening it would restate a neighbour.** A general "any governed name" rule
   is doctrine-tier, and that placement call belongs to doctrine rather than to a
   kit SPEC, which would otherwise settle a doctrine question from the inside and
   duplicate whatever lands there.

No gate reads this clause, and that is a ruling rather than an omission. The
predicate a gate would need is *"this commit renames a knob"*, which is not
decidable from a diff without a knob-identity model no kit has; an approximation
would fire on additions and removals alike and be valved into silence. The
clause's reader is the build-stage session performing the rename, which reads it
to decide whether it owes a shim and a deprecation marker.

A consumer points the general-availability criterion at whatever stability
declaration it maintains. The kit states the criterion and never the instance:
naming one project's channel vocabulary here would ship that project's release
posture as everyone's, and no knob is introduced to read the value either, since
the clause's reader is a human or agent rather than a gate.

- `LIFECYCLE_KIT_STAGES` — the stage roster, in order; default
  `(scope align build validate close)`.
- `LIFECYCLE_KIT_PREDECESSOR` — associative map stage → the predecessor whose
  stamp `check-stage-entry` requires; default `([align]=scope [build]=scope
  [validate]=build [close]=validate)` (`build` keys to `scope` because the
  audit stage is trigger-gated; §check-stage-entry).
- `LIFECYCLE_KIT_FIRST_STAGE` — the stage whose entry is the iteration boundary
  (§bin/enter-stage.sh truncation); default `scope`.
- `LIFECYCLE_KIT_DRAIN_STAGE` — the stage whose entry requires the active queue
  sections empty; default `validate`; empty disables the drain assertion. Must
  not be terminal: at least one `LIFECYCLE_KIT_PREDECESSOR` entry names it, or
  config load fails (the drain-exempt backstop; §check-stage-entry).
- `LIFECYCLE_KIT_ACTIVE_SECTIONS` — the queue sections the drain assertion
  reads; default `("New Features" "Technical Debt")`. Independent of
  canon-kit's `CANON_KIT_ACTIVE_SECTIONS` (read by `check-amendment-queue`'s
  misfiled-spec-ready clause) though their defaults coincide — a consumer
  retargeting one alone splits the drain assertion's view from
  `check-amendment-queue`'s, with no gate to notice. A known, accepted
  coupling: kit independence outranks unifying the knob.
- `LIFECYCLE_KIT_AUDIT_STAGE` — the trigger-gated audit stage assertion C looks
  for; default `align`; empty disables the audit machinery entirely.
- `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` — the stage whose entry assertion C blocks
  on a cross-component signal with no audit stamp; default `build` when an
  audit stage is set, else empty.
- `LIFECYCLE_KIT_WAIVER_TOKEN` — the stamp token recording the user's explicit
  audit waiver; default `<audit-stage>-waived`; must not collide with a
  stage name.
- `LIFECYCLE_KIT_AMENDMENT_GLOB` / `LIFECYCLE_KIT_ROSTER_BASENAME` — the amendment
  filename shape and the canonical-spec basename assertion C scans
  (template dirs pruned); defaults `SPEC-*.md` / `SPEC.md`.
- `LIFECYCLE_KIT_CONTRACT_TOKENS` — the amendment-body substrings assertion C
  reads as a cross-component contract signal; default `("SPEC.md" "proto/")`.
- `LIFECYCLE_KIT_SKILLS_DIR` — the agent-skill directory
  `check-stage-skill-coverage` scans; default `.claude/commands`.
- `LIFECYCLE_KIT_AGENT_FILE` — the always-loaded agent file the
  `--install-lifecycle` arm writes the registration block into and
  `check-lifecycle-registration` reads it back from; default `CLAUDE.md`
  (the `DOCTRINE_KIT_AGENT_FILE` sibling).
- `LIFECYCLE_KIT_QUEUE_FILE` / `LIFECYCLE_KIT_STATE_FILE` — the governed header and
  stamp files, defaulting through gate-sdk's `GATE_SDK_QUEUE_FILE` /
  `GATE_SDK_WORKFLOW_DIR`.
- `LIFECYCLE_KIT_SESSION_ID` — the harness-neutral stamp-id override, source 1
  of the derivation order (§bin/session-id.sh); default unset. Read from the
  process environment: the arm that reads it declares no knob roster, so this
  name does not cross the config bridge.
- `LIFECYCLE_KIT_SESSION_BOUNDARY` — `stage` or `iteration`; default `stage`.
  The knob lives on the session-span/evidence axis only (ruled; no role
  values): manual-versus-lead is the driver/role axis and rides context-kit's
  session-role signal, never this knob.
  The consumer's session-boundary posture: at `stage`, distinct stages of
  one iteration may not share a session id (§check-stage-evidence); at
  `iteration`, that cross-stage distinctness check alone is skipped —
  attribution still rides the stamps, and every other assertion holds. The
  loader validates the value alongside its machine checks and exits 2 on
  anything else. `enter-stage.sh` does not read it — stamping is
  posture-independent; `templates/lead.md` consumes it as the inline-run
  posture prose.
- `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` — the kit-owned lesson-disposition stamp
  file; default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/lesson-evidence.txt`,
  read by `check-lesson-disposition` and the boundary-reset built-in.
- `LIFECYCLE_KIT_GAP_INBOX_FILE` — the committed append-only gap inbox
  (§The committed gap inbox); default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/gap-inbox.md`, written by the `--emit-file-gap` arm,
  its `merge=union` attribute verified by `check-merge-attrs`, drained by the
  close skill and read for emptiness by `bin/enter-stage.sh`'s boundary refusal.
- `LIFECYCLE_KIT_SURVEY_RECORD_FILE` — the committed per-iteration survey record
  (§The survey record); default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/survey-record.md`, written by the
  `--emit file-survey` arm, asserted by `check-survey-record`, its headings
  printed and its body truncated by `bin/enter-stage.sh` (a kit-owned boundary built-in,
  so it does not ride `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`), and its
  `merge=iteration-scoped` attribute verified by `check-merge-attrs`. One knob,
  and no second one for the grammar, the witness commands, or an opt-out — each
  of those is contract rather than layout.
- `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` — positive integer, default `2`; the
  recorded re-filing count at which a **deferred** entry enters the scope stage's
  proposed unit set regardless of the standing directive's theme (the pre-emption
  rule, §templates/stages/). Two recorded re-filings is a third incidence of the
  same finding. It is a stated policy with a stated purpose rather than a derived
  number, and a knob for that reason — `QUEUE_KIT_ENTRY_LINE_CAP`'s posture. The
  count it is read against is the date count of the entry's `recurrence:`
  declaration (queue-kit/SPEC.md §The tag algebra owns that grammar). That rule is
  its only reader: queue-kit owns the declaration and reads no threshold,
  drift-kit reports the count and applies no verdict.
- `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS` — the consumer's `close-surface:`
  declaration surfaces beyond the resolved kit roots (§The close-surface
  roster); default `*/SPEC.md`. It deliberately does **not** default to
  `CANON_KIT_MANIFEST_FILES`: reading another kit's knob would make lifecycle-kit
  depend on canon-kit's configuration for a value the consumer already owns, and
  the one cross-kit knob read in the tree today is precedent, not a ruling. The
  declaration vocabulary is kit-owned and carries no consumer content; the roster
  is derived, never a kit literal — a kit shipping the *names* of a consumer's
  inbound surfaces would publish that consumer's private workflow.
- `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS` — array of path globs for the surfaces
  held to the no-retrieval-pointer rule (§check-scratch-citation); default the
  queue file alone (`LIFECYCLE_KIT_QUEUE_FILE`). That default is the one permanent
  surface this kit owns and where both attested firings landed, so it is
  non-vacuous in every consumer and over-reaches in none. The roster is consumer
  config and the forbidden targets are derived from the consumer's own truncate
  configuration, so no kit literal names any surface a consumer happens to have.
- `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` — extra files reset to their header at the
  iteration boundary; default empty. Adding a member also widens
  `check-scratch-citation`'s forbidden-target set, because both read
  `lifecycle_supersede_set`.
- `LIFECYCLE_KIT_BOUNDARY_PRESERVE` — keep-list of scratch-dir **basenames** the
  iteration-boundary wipe spares (§bin/enter-stage.sh); default empty, so an
  unset-knob consumer gets a clean wipe of an already-disposable surface.
  Boundary-only, and paired with the scratch dir it reads (`GATE_SDK_TMP_DIR`)
  rather than adding a directory knob of its own. **The tier split matters:** the
  `.gitkeep` exemption is a kit invariant this knob cannot unset, and this knob
  is the consumer keep-list layered on top — a reader who takes the knob for the
  wipe's only keep rule would reintroduce the tracked-file deletion.
- `LIFECYCLE_KIT_BOUNDARY_REQUIRE` — array of repo-relative files each of which
  must carry a data line naming the closing iteration before the iteration
  boundary may be crossed (§bin/enter-stage.sh); a missing member is a
  fail-closed refusal; default empty (an unconfigured consumer sees no change).
- `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK` — `0` or `1`; default `1`. At `1` the
  iteration boundary refuses while any linked worktree stands
  (§bin/enter-stage.sh). Defaulted on because a consumer that never dispatches an
  isolated agent has an empty `git worktree list` and the check is vacuous rather
  than absent; a consumer with a standing long-lived worktree turns it off here
  rather than teaching the kit its paths.
- `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` — a POSIX ERE with exactly one capture
  group, matched against a linked worktree's git lock reason, the group being
  the holder's pid (§bin/enter-stage.sh); **default empty**. Empty because a
  lock reason is one harness's vocabulary and a kit literal spelling it would
  publish it, the same seam the residue-directory omission takes; empty also
  means *no classification is configured*, so an unconfigured consumer sees
  exactly the unclassified refusal it sees today. Setting it buys a dependency
  on evidence-kit's liveness predicate; a pattern that will not compile, or one
  declaring no capture group, is a fail-closed config refusal (§lib/stages.sh).
- `LIFECYCLE_KIT_ENTRY_PREFLIGHT` — per-stage `<stage>=<command>` entries run
  alongside the built-in pre-flight (§bin/enter-stage.sh); default empty.
- `LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE` — the committed one-shot valve ledger
  whose `armed` line admits a single entry past a refusing
  `LIFECYCLE_KIT_ENTRY_PREFLIGHT` command (§bin/enter-stage.sh); **default
  empty**, meaning no valve and a final refusal, which is the behaviour every
  consumer has today. The default is off because a ledger path is one consumer's
  workflow-directory layout, and a kit literal spelling it would ship that layout
  to every adopter — the seam `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` already takes
  one knob up. Which stages may be valved is likewise not a kit literal: it is
  whatever an arming line names, bounded by the configured stage roster. A
  consumer setting it owes the ledger two things: membership in
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`, which bounds "how many times did we reach
  for the valve" at the iteration and keeps the ledger from accreting into a log
  nobody reads, and a `close-surface:` declaration (§The close-surface roster),
  without which a tracked ledger reaches no derived roster at all.
- `LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN` — the resume journal's path as a function
  of the stage, carrying a `<stage>` placeholder (§The state machine); the
  default is the scratch dir's own knob followed by `/<stage>-journal.md`, so the
  scratch dir's literal is deferred to rather than restated here. A pattern
  carrying no placeholder is a fail-closed config refusal (§lib/stages.sh).
- `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` — `0` or `1`; default `0`. At `1` a stage
  entry refuses when the cursor's stage left no journal at that derived path
  (§bin/enter-stage.sh). Defaulted **off** because the assertion reads the
  *predecessor*: switching it on mid-iteration asserts against sessions
  dispatched before the rule existed, so a consumer throws it at an iteration
  boundary, where a refusal costs a re-entry rather than a wedge.
- `LIFECYCLE_KIT_SHIM_NGRAM` — the shared-n-gram width `check-shim-restatement`
  trips at (positive integer; §check-shim-restatement); default `9`.
- `LIFECYCLE_KIT_SHIM_DEDUP_CORPUS` — that gate's corpus file list; default
  empty for the computed `CLAUDE.md`-plus-kit-templates default.

`.gitattributes` (repo root) is a consumer surface the kit writes but adds **no
knob** for: the `merge=iteration-scoped` supersede set derives from the existing
boundary-truncate knobs (`LIFECYCLE_KIT_STATE_FILE`,
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, and each `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`
member), so a reshaped truncate set flows into the attribute block by
construction — §Multi-operator semantics, §bin/install-lifecycle.sh.

## Multi-operator semantics

The contributor altitude of the coordination map: a second operator running
their own concurrent iteration on the same repo. The two narrower altitudes are
ruled elsewhere and stay untouched — sub-agents within one session
(delegation-kit's serialize-or-worktree rules) and sessions within one iteration
(§templates/lead.md: one live iteration, stages serialized through the stamp
protocol). Fork contributors are out of scope: an outside PR never stamps
state — it only passes the battery in CI.

**The topology ruling — state surfaces stay single-writer; concurrency is git
topology.** The header line, the evidence file, and every boundary-truncated
surface are *iteration-scoped*: an iteration owns exactly one branch (its home
branch), and every stamp lands there. One live iteration per branch —
the second concurrent operator cuts a branch at their scope entry; the
integration branch is the degenerate single-operator home, which is why a
single-operator repo's own dogfood changes nothing. Branch naming is prose
guidance (name the branch after the iteration), not mechanism — no knob, no
gate. Ruled out, each because it composes worse than git already does: per-operator
state files or stamp-attribution fields (multi-writer surfaces and a new stamp
grammar — operator attribution already rides the git author on every stamp
commit); a lock or lease on the integration branch (state the kit refuses to
own, where git already provides the isolation).

**The merge-supersede rule.** At any branch merge, the iteration-scoped surfaces
resolve wholesale to the *arriving* (checked-out) iteration's version — the other
side's content is per-iteration scratch the boundary doctrine already declares
dead (git history is the permanent audit trail). The supersede set is **derived,
never maintained**: it is exactly what `bin/enter-stage.sh` truncates at the
iteration boundary — `LIFECYCLE_KIT_STATE_FILE`,
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, `LIFECYCLE_KIT_SURVEY_RECORD_FILE`, and
the `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`
members — rendered by `lifecycle_supersede_set` (§lib/stages.sh). The survey
record is in the set for a reason worth stating: a carried survey describes a
tree state the arriving iteration never had, so taking the other side would
hand a stale finding a fresh-looking home (§The survey record). The queue file
is deliberately *not* in the set: its body (backlog sections, lessons) is shared
content that merges like any prose, and only its header line is iteration-scoped
— resolved by hand to the arriving iteration, with a wrong resolution going red
at the next commit because `check-stage-evidence` requires the header's name to
agree with every stamp's, and the state file already took the arriving side by
driver. Contention on the queue is lower than it reads: since the cursor left
the header, **stage motion writes no queue at all**, so the file changes only on
real work-state transitions (promotion and naming, a Done move, the closing
dispositions) rather than once per stage entry per operator.
Held-constant baselines and append-across-iterations evidence keep normal merge
semantics: their conflicts are real disagreements. The kit owns exactly one
`union`-driver surface — the committed gap inbox (§The committed gap inbox),
whose append-only bullets must survive a concurrent merge rather than supersede
— rendered into the attribute block beside the supersede set and verified by the
same gate; a consumer with its own *tracked* shared append log points it at git's
built-in `union` driver the same way — sanctioned shape, git-native. (Gitignored
per-checkout scratch — friction logs — never merges and needs no rule.)

**`.gitattributes` — the rule mechanized.** Each supersede-set path carries
`merge=iteration-scoped`; the driver definition (`git config
merge.iteration-scoped.driver true` — keep ours) is per-clone config installed by
the `--install-lifecycle` arm beside its registration block (the `install-hooks.sh`
opt-in class). Honest limit: on a clone without the driver installed the
attribute is inert and the file conflicts normally — the rule above then governs
the hand resolution, so the uninstalled path degrades to judgment, never to
silence. Writer/asserter split: the arm emits the attribute block
(marker-bounded), `check-merge-attrs` verifies it — the
`gen-pre-commit.sh` ↔ `check-graph` precedent.

**Who may stamp, at this altitude.** Unchanged: the arriving stage session
stamps,
and only on its own iteration's home branch. A session never stamps a branch whose
iteration it is not driving; cross-iteration discoveries (a lesson, a deferred
filing) land on the discoverer's own branch and reconcile at merge.

**Causal-completeness core: no new state surface.** The design adds no stamp
grammar, no queue tag, no evidence file. Every existing producer/consumer pair
(`enter-stage.sh`, the stage gates, the drift report) keeps working per-branch
unmodified. The two added surfaces each have a named reader at a named
transition: the `merge=iteration-scoped` lines are read by git's merge machinery
at a merge and by `check-merge-attrs` at pre-commit; the `merge.iteration-scoped`
driver config is read by git's merge machinery when an attributed path needs a
three-way merge (not readable by a pre-commit gate — per-clone state, its absence
the recorded honest limit above). The close-merge protocol (§Deviation
transitions) is produced by the closing operator's close session and consumed by
the integration branch's battery — the reconcile commit re-fires every
queue/state-coupled gate, which is what makes the header hand-resolution
enforceable.

## The committed gap inbox

Mid-iteration work-state writes race the stage session holding the shared git
index: a gap surfaced mid-stage has no committed place to land except the queue
file that stage session is already contending on. So mid-iteration gap *filing*
gets a committed, append-only channel of its own — distinct from the
knowledge-friction log, which stays the narrow sensor for a fact re-derived
because no doc owns it (drift-kit/SPEC.md §The knowledge-friction loop): a
*work-shaped* finding (a gap, a task, a defect) is backlog, not knowledge
friction, and routes here.

**The surface.** `.workflow/gap-inbox.md` (knob `LIFECYCLE_KIT_GAP_INBOX_FILE`,
§Layout and configuration) is a committed, append-only capture buffer. Grammar:
a `# contract:` prose header, then one `- <YYYY-MM-DD> — <gap prose>` bullet per
gap — **one shape for every filing**, with no structured slot between the date
and the prose. A slug named *inside* the prose takes queue-kit's existing
in-body citation form (queue-kit/SPEC.md §The tag algebra), single-backticked,
borrowed rather than re-spelled; it is a citation and never a field, which is
the whole distinction §check-gap-inbox-neutrality holds. Committed, not gitignored — a
per-clone buffer fragments the backlog across
operators, the finding that rules the gitignored friction log out as the channel.
What *append-only* means on this surface is a merge property, and
§Multi-operator semantics owns it: this is the kit's one `union`-driver surface.

**The affordance.** `run-gates.sh --emit file-gap [--] "<gap prose>"` (the
`--emit-kfric` pattern: repo-root anchor, config-via-env, exit 2 on an empty
argument) appends one dated bullet, seeding the contract header — byte-identical
to the line close's drain truncates back to — when the inbox does not yet exist.
It is the `--emit-file-gap` bridged arm (gate-sdk/SPEC.md §The non-gate arm),
whose declared roster is five knobs already defined in `lib/stages.sh`:
`LIFECYCLE_KIT_GAP_INBOX_FILE`, `LIFECYCLE_KIT_QUEUE_FILE`,
`LIFECYCLE_KIT_STATE_FILE`, `LIFECYCLE_KIT_STAGES` and
`LIFECYCLE_KIT_FIRST_STAGE`. The family is forced rather than chosen: the tool
resolves consumer knobs, and a hardcoded top-level flag would resolve platform
defaults while silently ignoring every override.
Its one positional is free text, so it validates that argument's **shape** to
gate-sdk/SPEC.md §The bin/-tool contract — an unrecognized leading `-` refused
at exit 2, `--` ending option processing — which is the rule this capture
affordance's own three attested firings bought, and which **survives the port
because the hazard belongs to the argument rather than to the substrate**. The
`-h`/`--help` arm does **not** survive: usage for a bridged arm lives in
`run-gates.sh`'s own help and in [README.md](README.md), so `--emit file-gap
--help` is a refusal rather than a capture. That is the one observable this
member's port moved. Stdout is the filed bullet and nothing else; the three
advisories below all ride **stderr**, so a reader pasting the returned line
never carries a question or a warning inside it. It is
advisory tooling, not a gate — no fixture pair is owed; the raw append (a bullet
line into the inbox) stays a legal fallback, the grammar being the surface's
contract, not the writer.

**Taking that port did not discharge this section, and the residue is written
rather than implied.** Three implementations carry this section's contract and
only one of them is in-crate. The capture affordance above is compiled.
`bin/enter-stage.sh` still holds the **iteration-boundary gap-inbox check** and
its close-skipped/post-close discriminator (§bin/enter-stage.sh), and it ports in
a cut of its own. `lib/stages.sh` still holds this surface's union-merge
membership (§Multi-operator semantics), and it is **permanently shell** under the
kit-library class ruling as the config bridge's sole `LIFECYCLE_KIT_*` resolver
(gate-sdk/SPEC.md §The kit-library port disposition) — so this section's contract
will not be wholly in-crate while that ruling stands. Its already-ported twin
`native/src/stages.rs` carries the same constant on the crate side.

It also **resolves the prose against the live slug set** at capture and, on a
match, raises a stderr advisory that **asks** the filer: the prose names live
entry `<slug>` — if this bullet *re-files* that finding, say so in the prose and
say why; if it merely cites, corrects, or argues against that entry, say it is
*distinct* and why, because the closing stage's drain judges the recurrence and
reads what was written. The resolution buys a prompt and nothing else: it writes
no marker, and the bullet it accompanies is the same one shape every filing gets.
The
live set is every column-0 `- **<slug>** —` entry bullet in the queue file
*outside* the fixed-spelling `## Lessons Learned` section — that is exactly
active, deferred, and a configured icebox — and it needs **no section knob of its
own** because both exclusions fall out of grammar the kit already reads. The done
section is excluded by construction: a done entry is a bare-slug line, outside the
entry grammar queue-kit/SPEC.md §The queue format defines. Lessons is excluded by
name, and must be: a lesson lead line is outside that grammar for every *gate*,
but it may legitimately be *written* in the entry shape (`- **slug** [tag] —
prose`), so a grammar-only scan would resolve a live lesson as a queue entry and
ask the filer about something that is not one. The `## Lessons Learned`
literal is fixed spelling rather than config (queue-kit/SPEC.md §The tag algebra),
and this kit already carries it — `bin/enter-stage.sh`'s boundary refusal and
`check-lesson-disposition` both scan it.
The done exclusion is the substantive half: a finding that recurs *after* its fix
landed is a new defect, not a recurrence, and files as one. Resolution is
lifecycle-kit's own scan over the
queue — the shape `check-stage-entry` assertion B already takes — never
queue-kit's `queue_live_slugs`, because reaching for it would close a cross-kit
cycle. queue-kit/SPEC.md §The queue format states that as the general rule (a kit
that cannot depend on queue-kit re-implements the predicate and both ends cite the
owner section), and drift-kit's `kpi-deferred-age` records the same accepted
residual. Two further behaviours belong to the predicate rather than to its
grammar: **longest match wins**, so the advisory names the most specific live
entry the prose reaches; and the match is **word-bounded** on `[a-z0-9-]` in both
directions, so a slug embedded in a longer hyphenated token raises nothing.

**The port reproduced that predicate rather than collapsing it onto its compiled
twin, and the reason is a corpus difference rather than a preference.** The crate
already carries `queue::live_slugs`, the compiled form of queue-kit's own
predicate, whose section scope is composed from `QUEUE_KIT_ACTIVE_SECTIONS` plus
the deferred and icebox section knobs. This section's predicate is
**grammar-scoped instead** — every column-0 entry bullet outside the
fixed-spelling Lessons section, needing no section knob of its own. Those are not
the same corpus: a consumer whose icebox is unconfigured, or whose active-section
roster differs from its heading set, gets a different live set from each. The
collapse would therefore be a verdict change on a real consumer, which is not a
thing a port may take on its own authority.

**The stated ground weakens under that substrate, and the honest note is here
rather than left for a later reader to re-derive.** "Would close a cross-kit
cycle" is a claim about a *shell* source dependency — this kit's `bin/` sourcing
queue-kit's `lib/`. Inside one binary both predicates are already compiled
together and no vendoring decision separates them, so the anti-cycle premise no
longer describes the arrangement it was written against. Whether the refusal
survives on an independent ownership ground, or retires with its premise, is not
settled by any surface here and is not settled by the port.

**Why the matcher prompts rather than decides, and why it survives at all.** A
recurrence is a claim about *what a finding is*, not about *what a string
contains*, and no syntactic tell separates "this recurred" from "this is about
that": a bullet can spell the denial out in words and still match. So the
predicate is not sharpened, it is **demoted** — its recall was never the defect,
its authority was. Deleting it would delete the only thing that can produce the
prompt, and the prompt is what reaches the one party who can answer cheaply: a
mid-build filer routinely has not read the queue, which is why the matcher
existed in the first place.

**Its honest limit.** A bullet describing a recurrence without spelling the slug
raises no advisory at all. That leaves no hole in this channel, because the drain
reads **every** bullet regardless of whether the matcher spoke — which is the
difference between a prompt and a gate, and the reason the judgment had to move
rather than the predicate get sharper.

**Three alternatives are refused, recorded so they are not re-drafted.** A
*required* recurrence-or-new argument on the affordance turns capture into an
interrogation, and refusing capture does not dissolve a finding — it pushes it
back into session context, the deferred-capture antipattern this inbox exists to
prevent; it is also asked of a party who has not read the queue. An *optional*
`--recurrence <slug>` flag invites the drain to trust a structured claim,
reinstating the same defect one layer up with a better provenance story. And
*narrowing the matcher* — anchoring the slug, exempting a bullet containing a
negation — institutionalizes the evasion: an affordance that must be phrased
around to stay accurate is miscalibrated, and the next filer does not know to
phrase around it. What all three trade against is prose, which the channel
already carries and a judge already reads.

**A fourth shape is refused — deriving post-close-ness from git — recorded so it
is not re-drafted.** The proposal: rather than reading the cursor, ask "has the
inbox been truncated since the close stamp's commit?", which separates the cases
with no marker field. It is refused on evidence, on four independent grounds.

- **It is wrong on the case that matters.** The predicate assumes the drain and
  the closing stamp are commensurable in commit order. Measured over this
  project's four most recently closed iterations they are not: the truncation is
  **never** in the stamp's commit and is **always** a later, separately authored
  one. So "truncated since the close stamp" is true of every normally-closed
  iteration, and the single case it does distinguish — a close that stamped and
  then skipped its drain — it answers *close-skipped*, handing the entering
  session a recovery naming a stage that is gone. The cursor read is right there,
  because close did run.
- **It cannot answer the other case at all.** Where close was genuinely skipped
  there is no close-stamp commit to anchor on, so the git predicate degenerates
  into "is there a close stamp?" — which is the cursor read, obtained from a file
  the tool already holds open.
- **It would be `bin/enter-stage.sh`'s first `git` invocation.** That tool shells
  out to git nowhere: every decision it takes is a read of the queue and the
  state file. Adding history-dependence to the state machine's only writer makes
  the entry decision non-hermetic, and the branch's fixture would have to
  construct commit history rather than three files.
- **Its stated virtue does not discriminate.** "Needs no marker field, so
  §check-gap-inbox-neutrality's two-field bound stays intact" is equally true of
  the cursor read, which writes nothing anywhere. The bound is untouched by both,
  so it selects neither.

**The tool writes no queue file, and that is the load-bearing constraint on the
whole channel.** This inbox exists precisely because a gap surfaced mid-stage has
no committed place to land except the queue file a stage session is already
contending on. A capture affordance that stamped a `recurrence:` declaration onto
a queue entry would do the one thing the inbox was built to prevent. The queue write
therefore belongs to the closing stage's drain, which writes the queue anyway.

**A queue entry cites this inbox as provenance, never as a locator — ruled
2026-09-02 by the operator.** The inbox is drained every close, so a sentence
naming it as a fact's *current home* — "it is in the gap inbox as of" a date, or
a parenthetical "(gap inbox," plus a date) — is broken by the next drain, while a
sentence stating that a finding *was filed there and drained from it on a date*
is history and stays true. The test is whether the sentence still resolves after
the next drain. A fact a bullet carries is restated on the entry or cited to the
surface the drain moved it to; a queue entry that must point at the bullet itself
is pointing at a surface that will not exist. The same rule binds citations of
the survey record (§The survey record), which the iteration boundary truncates.
Held by review rather than a gate: a present-tense scan over queue prose is a
tree-facing heuristic whose over-refusal cost the tree already pays elsewhere, and
the measured rate — two locators in the inbox's whole life, both caught by one
read — does not buy one.

**A dated attestation freezes the claim, never the locator — ruled 2026-09-03 by
the operator on a consult's recommendation, lead-relayed.** The test above
generalises past these two surfaces to every dated claim in queue prose. A dated
measurement or judgment — *verified 2026-08-30: the three sites are …* — is
history: true on its date, immune to later drift, and never refreshed, which is
the freeze the ruling record's dated-measurement idiom already states per
instance (TRAJECTORY.md §The closed rulings; gate-sdk/SPEC.md §The decisions this
substrate already closed). A `path:line` beside it is a locator by construction —
its only reader follows it now — so drift falsifies the literal, and a literal
position in prose is the de-literalization defect whatever date sits beside it
(doctrine-kit/DOCTRINE.md §Methodology-maintenance rules). A date is not a rev:
two closes have fallen on one calendar day, so a date under-determines the tree,
where a carried survey's witness is corpus, oracle and rev (§The survey record)
and the one frozen line range this tree keeps carries its commit
(gate-sdk/SPEC.md §port-blockers). So a locator worth keeping is de-literalized
to the name it points at, or rev-pinned where the exact span is load-bearing; the
rule reaches the bare `verified <date>` standing beside a `path:line` with no
freezing sentence, a case no surface stated before this. The consequence for the
pendency sweeps: an anchor whose sentence holds while its line moved is a finding
of the de-literalization class, fix-shaped at a drain, not of the pendency class;
an anchor resolving to different text is a false claim. Two readings were
refused. *A historical record immune to drift* makes a date a licence to carry
rotting locators, contradicts de-literalization and spec-over-precedent, and
would un-find a citation resolving to different text. *A live pointer the drift
falsifies whole* makes the dated-measurement idiom defective and invites the
recount that ruling forbids. A scanner gate over anchors is refused on the
grounds the paragraph above already gives.

**The drain's dispositions are ordered, and promotion is last — ruled 2026-08-30
by the operator, on a measured drain.** The disposition set is fix, icebox,
promote, discard; the drain tries them in that order per bullet and a promotion
states in the close commit message which earlier disposition it failed and why.
The measurement: with the set listed promotion-first, fifteen consecutive drained
bullets over two closes were promoted and none fixed or iceboxed, on a pool whose
intake had outrun its exits three to one for a fortnight — so the unordered set
was read as a promotion default, and a three-line defect cost a thirty-line
entry plus the iteration that would one day build it. **A bullet is fix-shaped
when it is debt-shaped by the interstitial litmus above — it adds no governed
name — and lands test-and-doc-complete in the drain's own commit**; that litmus
is the one this section's interstitial mitigation already applies, so no new
criterion is minted. Fixing in the drain is not the refused hotfix track: it
contends on no live stage surface (the closing stage holds the index), and
what it admits is exactly what that refusal said a hotfix is not — debt-shaped
work. It is also not an intake violation: scope gates *initiatives*, and a
defect the drain fixes in one commit adds nothing for scope to weigh. A bullet
is icebox-shaped on queue-kit/SPEC.md §The icebox tier's eligibility, read at
the drain. The alternative refused with the ruling: a gate reddening a close on
net pool growth. Enforcement-first would prefer it, but the refusal ground is
that the exits are judgments — a forced exit is a fake one — so the figure is
surfaced instead: drift-kit's `kpi-queue-net-delta` already computes it, and
the drain's commit message states its `qnet` fragment.

It also **warns at the point of capture**, reading the cursor
(`lifecycle_current_stage`) to say which consequence the filer is buying: an
ordinary filing is told the bullet blocks the next first-stage entry until the
drain, and a filing made while the cursor sits at the **last configured stage**
is told that once that stage finishes, none is left to drain it. The warning
goes to stderr so the stamped bullet stays the tool's stdout contract. It is a
warning and not a refusal deliberately: refusing capture does not dissolve a
real finding, it pushes it back into session context — the deferred-capture
antipattern this inbox exists to prevent.

**The drain re-verifies; capture does not.** A bullet's prose is a claim made at
capture speed, and this channel is built to keep capture cheap, so nothing
upstream established it — a bullet can assert a mechanism its filer inferred
rather than ran, and a false premise is paid by whoever drains it. The drain
therefore names, per bullet, the claim its disposition turns on and the command
that establishes it, runs that command, and records in the same commit which
bullets were re-verified and what fell (§templates/stages/ carries the step). The
grounds are attested rather than argued: of the bullets re-verified at one
boundary, two were false at their central claim and each fell to a single
command, and a later bullet carried two false premises at once — one caught by
the filer re-reading the source before the drain, the other only by the draining
session's own probes. The step that reliably ran was the drain.

**Two capture-time shapes are refused, recorded so they are not re-drafted.** A
filing-time prompt for the establishing command, and a grammar separating
observed fact from inferred mechanism, both add friction at the moment this inbox
exists to keep cheap, and both bill it to the party least able to pay — the
mid-stage filer whose finding otherwise stays in session context. That is the
deferred-capture antipattern the channel was built against, and it is not traded
away to fix a different failure mode. The affordance gains no prompt and no
grammar; §check-gap-inbox-neutrality keeps the bullet's fields at two.

**Merge semantics.** The inbox carries `merge=union` (git-native, so no per-clone
driver registration), not the keep-ours `merge=iteration-scoped` the
boundary-truncated surfaces carry: an iteration-scoped surface is per-iteration
scratch superseded at the boundary, but a gap filed on either side of a
concurrent merge must survive. The installer emits the line and
`check-merge-attrs` verifies it (§bin/install-lifecycle.sh, §check-merge-attrs).

**The boundary check: one detector, two dispositions.**
`bin/enter-stage.sh`'s first-stage (iteration-boundary) entry is unchanged in
**detection** — it still fires on any `- ` bullet in the inbox — and
**discriminates** on which of two dispositions it takes, because the drain and
the filing window do not coincide.

- **Close-skipped** — the closing iteration's cursor never reached the last
  configured stage. The entry **refuses** (exit 1, the untriaged bullets
  printed, nothing written — the same refusal contract as the non-empty Lessons
  section), with the one recovery that applies: run the closing stage's
  gap-drain step, truncate the inbox, re-enter. A stage was skipped, so the
  recovery is to run it, and no gap outlives its iteration untriaged (the
  gap-disposition rule: costed and filed, never flagged-and-skipped).
- **Post-close** — the cursor sits at the last configured stage, so the closing
  stage has run and none is coming back. The entry is **admitted**: the bullets
  print on stderr as an advisory naming them this iteration's first-stage
  intake, and the stamp proceeds. Deleting a bullet without a disposition is
  still not a drain.

The discriminator is the cursor read `lifecycle_closing_stage_reached`
(§lib/stages.sh), the same predicate the `--emit-file-gap` arm warns from at
capture, so a filer told "none is left to drain it" is told so by the same test
that later admits the bullet. **That agreement now holds inside each substrate
and not across them**: the boundary check here is shell and the capture warning
is compiled, so until this tool ports the two are lookalikes rather than one
hoisted predicate. **This check and its discriminator are §The committed gap
inbox's surviving shell half**, stated here so a later cut selector meets the
fact where it works rather than in the other section. **Two edges take the post-close
disposition**, both following precedents the script already carries: a closing
iteration that was never named (the `—` placeholder) has no close to have
skipped — the guard `LIFECYCLE_KIT_BOUNDARY_REQUIRE` applies one block down for
the same reason — and an inbox holding bullets with no cursor at all, a fresh
consumer's first boundary, is that case too.

**No second detector is added for the post-close window**: the existing refusal
already detects it, and what was missing was the message's actionability. That
ruling is **kept and re-read** rather than retired — the two-disposition shape
satisfies it more exactly than the two-recovery message did, since one detector
still fires and what the discriminator picks is only what to do about it.

**Why admission, and not merely a better-worded refusal.** A refusal that picks
the right message still leaves the queue write where this project's own history
puts it: made **before any stamp exists, by a session that has entered no
stage**, and committed as though by a stage that has not started. Routing a
post-close finding into the first stage's ordinary intake is satisfiable only
*inside* that stage — the intake is its — so the entry has to succeed for the
finding to reach it. What the change buys is that the disposition becomes an
ordinary in-stage queue write in the stage that writes the queue anyway.

**What the admission costs, and why the loss is smaller than it looks.** The
invariant "no gap outlives its iteration untriaged" is not weakened, because a
post-close bullet never had a drainer in the iteration it was filed in — that is
the defect, and no message could have supplied one. On admission the bullet
stops being post-close: it becomes an ordinary mid-iteration bullet of the
**new** iteration, and it therefore acquires the drainer it never had, that
iteration's mandatory close drain. The first stage is directed to take it
earlier because it is writing the queue anyway; if it does not, close does. That
forcing function is the existing one rather than a new one.

**The honest limit, stated because it is real.** The finding's *disposition*
then lands in the next iteration's ledger, which cannot be repaired: the finding
postdates its own iteration's close, so no iteration-correct ledger position
exists for it. What is repaired is legibility — a promoted entry's provenance
sentence carries the bullet's own date and names the iteration whose close
generated it, so a reader sees where the finding came from even though its
disposition sits one iteration later. A record that is late and says so is
strictly better than one that is late and silent.

That check is the inbox's
forcing function — branch-conditional as of the two dispositions above, refusing
on the close-skipped branch and obliging an in-stage disposition on the other —
so the inbox declares itself on the close-surface roster
(§The close-surface roster) here, where the forcing function is documented:

close-surface: .workflow/gap-inbox.md forced=lifecycle-kit/SPEC.md §bin/enter-stage.sh

**Producers and consumers.** Producer: any mid-iteration session (lead or stage)
via `--emit file-gap` — the knob default makes the channel live everywhere the
kit is vendored. Consumers, **two**, one per disposition of the boundary check.
The close skill's drain step (§templates/stages/) dispositions every bullet —
promoted to a deferred `[design-pending]` entry, fixed inline that session, or
discarded with cause in the close commit message — then truncates the inbox to
its header. The **first stage's intake step** (§templates/stages/) takes the
bullets the boundary carried, with the same disposition set and the same
truncate-in-the-same-commit rule, run after its stamp; it is reachable at every
boundary with no enabling config, and it exists because a post-close bullet has
no drainer in the iteration that filed it. The boundary check reads emptiness at
the next first-stage entry as the backstop for both.

**A `recurrence:` date records a session's judgment that a finding re-occurred,
made by reading the bullet's prose. A slug appearing in a bullet is an input to
that judgment and never a verdict; no mechanism produces the declaration.**

That drain step is the declaration's only **mechanized** producer
(queue-kit/SPEC.md §The tag algebra), and it is the stage that must not skip the
judgment: on a bullet it judges a recurrence it stamps the date onto that entry's
declaration — creating the line when absent, appending the date when present —
**in addition to** its ordinary disposition, never instead of it. It is reachable
at every close with no enabling config, since the drain is already mandatory and
the boundary refusal already forces it.

The drain is *not* the only producer, and that path is **ruled: sanctioned, and
obliged.** A session that observes a recurrence outside the capture channel
stamps one directly, in the commit it is already making. The obligation attaches
to the **judgment**, not to the channel the observation arrived through — the
failure the counter exists to end is a recurrence *seen and not recorded*, and a
merely permitted write is one a session under pressure correctly declines, which
would answer the question in form and leave it open in substance.

**A lead cannot discharge that obligation as written, and the channel already
supplies what it can.** §templates/lead.md forbids the lead every queue write, so
a lead that judges a recurrence has exactly one channel — this inbox — and a
lead's judgment is made at the boundary, which is precisely where the check above
stands. The discharge is therefore *stated* rather than invented: **a lead
discharges the obligation by filing the judgment and its grounds into the
bullet's prose**, and the stamp is made by the session that may write the queue,
judging from that prose. That is the shape the drain already has — the bullet's
prose is the grounds the judge reads — so no new authority and no new producer is
created. What the two dispositions above supply is the route's far end: a
boundary-filed lead judgment reaches a judge rather than a refusal.
This settles the **authority** half only; the **reach** half — stages that may
write the queue but never load this rule — is untouched here.

*Not forbidden*, on three independent grounds. It would strand a class by
construction: this drain runs once, early in close, while close's own later steps
— audits, lesson disposition, staleness reads, release disposition — necessarily
generate findings that postdate it, leaving the one stage downstream of every
drain no legal channel at all. It is already being obeyed at a recorded cost, and
a rule whose disciplined observance produces a known-wrong count is the wrong
rule. And it would red roughly a third of its own precedent — the ground on which
the provenance gate below is refused, a refusal made to keep this route open.

*Not mechanized*, which is foreclosed rather than weighed: **no mechanism
produces the declaration** is the ruling stated above, and mechanizing this path
reverses it. Independently, the only available mechanism is the capture-time
matcher, whose honest limit above runs in the **under-counting** direction, so
mechanizing on it would dress a known-lossy predicate as a count.

*Prospective only, operator-ruled 2026-08-17, and this is the ruling's own reach
rather than a narrowing of it.* No session backfills a date onto a judgment an
earlier session declined to stamp. The trade below is what forecloses it: this
rule concedes re-derivability and pays with **auditability by inspection**, so a
date is legible only because the prose it was judged from sits in the same diff —
and for a past decline the judging session is gone and no contemporaneous prose
can be put there. A backfilled date would carry neither property, which is
precisely the artifact this section refuses to ship. The counter-argument is
recorded rather than buried: refutation two above rests on the declines having
produced a known-wrong count, and that count stays wrong. A rule change fixes the
count forward by removing what produced the error; retro-scoring is a different
act on a worse evidence base. Stated here so the reach is read rather than
re-derived.

Stamping is **idempotent per (slug, date)** — two filings of one slug on
one day record one date, the day being the only resolution the bullet's own
grammar has, and inventing a finer one would claim precision this channel does not
carry.

**What the judgment rule costs, and what pays for it.** The property the rule
declines to offer is **re-derivability**: a date is *not* reproducible by
re-running any predicate over the queue, so a reader cannot audit the count
without trusting the session that stamped it, and claiming otherwise would be the
dishonest half of the trade. What
pays for it is **auditability by inspection** — the drain stamps the declaration
in the *same commit* that truncates the inbox, so the judgment and the prose it
was made from sit in one diff and a reader auditing a date reads that commit and
sees the bullet the judge read. That same-commit rule is therefore **load-bearing
rather than incidental**: it is the audit artifact. The **invariant** is the
judgment and its grounds landing in one commit; truncating the inbox is one way
of satisfying it and not the thing itself. A direct stamp truncates nothing, so
it discharges the same obligation the only way it can — **the observation is
written into the entry, beside the declaration, in the stamping commit** — and a
reader auditing a direct date finds the grounds there exactly as with a drained
one. The same-commit rule is also what makes the
stamp a **mandated write** in queue-kit's sense — the class
queue-kit/SPEC.md §check-queue-entry-budget defines, and the section a stamping
session reads for its relief when that gate's per-entry cap blocks the stamp;
that grounds write is the same mandated write as the date it accompanies, its
grounds half rather than a discretionary addition, so it claims the same relief.
Neither the class nor the relief is restated here; this section is only the
contract that mandates the write. Re-running a matcher is
replaced by reading the grounds, which is the correct trade when the thing
recorded is a judgment.

**What the sanction adds is judges.** It adds no *new* class of unverifiability —
re-derivability is conceded above for the drain's own stamps — but the population
that may stamp widens from one stage to every session, and some of those sessions
judge their own tool use, which is a session grading itself. No gate can hold
that and none is proposed; the counterweight is the grounds obligation above,
which makes a thin judgment visibly thin.

**A provenance gate was drafted here and is refused, recorded so it is not
re-drafted.** Requiring every commit that adds a `recurrence:` date to co-stage
the inbox looks like the enforced replacement for re-derivability. It is not: a
back-test over this project's own history redded roughly one in three of the
commits that had ever stamped one, and it would foreclose the direct-stamp path
above by gating exactly the route the ruling there opens — which makes this
refusal retrospectively correct rather than speculative. A gate that
reds a third of the precedent it was derived from is measuring the rule, not the
tree.

Each bullet's two fields have named readers: the date feeds close's staleness
judgment and becomes the stamped recurrence date, and the prose is the
disposition body, the grounds the drain judges the recurrence from, the claim the
drain re-verifies, and — via the capture-time advisory on stderr — the field the
filer is asked to write the claim into. There is no third field, which is the point:
§check-gap-inbox-neutrality keeps it that way.

## The survey record

A stage that dispatches a survey — a census, a cohort sweep, a roster built by
applying a criterion set to a corpus — buys a finding that lives in the
dispatching session's context and dies with it. The next stage needing the same
roster has no artifact to read, so it dispatches the same survey again. Neither
session is undisciplined: each is correct in isolation, and the cost is
structural. This surface is where the expensive half of a survey is carried
across the stage boundary.

**The decomposition that makes carrying safe.** A carried finding has a
staleness problem a re-derivation does not: a census written at scope and read
at build is correct only while the tree it censused holds still, and the stages
between them are exactly when that tree moves. Sizing that window per *kind* of
survey is not makeable — it depends on what the next stage does, which the
author cannot know. So it is answered mechanically per record instead. A survey
worth carrying ran an **oracle** (the oracle-first doctrine demands it), and
that splits it in two: a **cheap, re-runnable half** — the oracle's verdict —
and an **expensive, judgment half** — the reading laid over that verdict against
a criterion set. The re-derivation re-buys both; this surface carries the
expensive half and re-runs the cheap one.

> **A carried survey is a citation with a falsifiable staleness witness, never a
> substitute for the oracle. The consuming stage re-runs the oracle and diffs the
> surveyed corpus since the recorded revision; if both hold, the recorded
> judgment stands and is cited rather than re-bought. If either moved, the stage
> dispatches only the delta.**

A survey therefore stays true exactly as long as its witness holds, and the
witness is checkable in two commands. A queue entry cites a block here as
provenance and never as a locator, since the boundary truncates it — the rule
and its test are §The committed gap inbox's.

**The surface.** `.workflow/survey-record.md` (knob
`LIFECYCLE_KIT_SURVEY_RECORD_FILE`, §Layout and configuration) is a committed
per-iteration record, append-only within the iteration. Grammar: a `# contract:`
prose header, then one block per survey:

```
## <YYYY-MM-DD> <stage> — <the one-line question this survey answered>
- corpus: <git pathspec the survey covered>
- oracle: <the command whose verdict grounds it, or the literal `none`>
- rev: <full commit sha the survey was taken at>
- edges: <the inbound-citation sum per candidate this survey ranked, or the literal `none`>
- finding: <the judgment, in prose>
```

Five fields, and each earns its place by being read at a named transition —
`corpus` and `rev` by the diff, `oracle` by the re-run, `edges` by the next
boundary's ranking, `finding` by the consuming session. The heading is the
discovery key and it states the *question*, because a later stage searches by
the question it is about to ask, not by the corpus it has not yet chosen.

**`edges` sits fourth, between the witness and the judgment, and the position is
not arbitrary.** `corpus`, `oracle` and `rev` are the *witness* — the three
strings the two-command re-use protocol consumes — and they stay contiguous.
`edges` and `finding` are the *judgment half*: an input to a ranking and the
ranking's reading. Appending `edges` after `finding` would put the field a
reader wants at a glance behind the longest line in the block.

**`edges` is obliged on every block, with the literal `none` legal — the second
reader of `oracle:`'s convention rather than a new one.** A field present only
on ranking blocks reintroduces exactly the "absent, or taken and dropped?"
ambiguity this field exists to remove, and the grammar is rigidly positional
precisely so that question never has to be asked. A field obliging every block
to write `edges: n/a — not a ranking survey` closes the ambiguity by making
every non-ranking block carry a *declaration*, which is the ceremony this
surface refuses. `edges: none` is one word, asserts nothing about a pass having
run, and is true of every survey that ranked no candidates; an *empty* value is
the silent form and is refused, exactly as `oracle`'s is. When the value is not
`none` it carries one inbound-citation sum per candidate the survey ranked, in
the spelling scope already writes — `<slug> <n>`, comma-separated, with any
caveat the sum needs. Free prose, deliberately: what makes the field usable is
that it is *addressable*, not that it is parseable.

The sum itself is bought at scope (queue-kit/SPEC.md §The queue-edges arm) and
was, before this field, recorded only inside `finding:` if at all — present but
unaddressable, so the next boundary could not tell "no sum was taken" from "the
sum is in there somewhere" and re-bought an aggregation the last boundary had
already paid for. The field is what makes it readable instead of re-buyable.

**No field for "how long this stays true."** Deliberately absent: an author
cannot know it, and a field carrying a guess would be read as a warrant.
`corpus` + `rev` + `oracle` let the *reader* compute it, which is the whole
ruling above.

**Every field's git-object-shaped tokens are real, not the `rev` field's alone.**
`rev` is machine-stamped and probed, but the other four are free prose an author
writes — and a fabricated short hash in `corpus`, put there to make a dated
census read as precise, is an attested failure of exactly this surface. Its
class — *an identifier you did not read is not a citation* — is owned by
delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps.
So a word-bounded run of 7-40 lowercase hex
carrying at least one `a`-`f`, in **any** of `corpus`, `oracle`, `edges` or
`finding`, must name a real object in this repository. A block that carries such a token on
purpose — an illustrative sha in an `oracle:` command, a fixture literal — takes
a valve line inside the block:

```
<!-- survey-token-exempt: <why this token names no object> -->
```

The **reason is mandatory**, and a valve without one is a finding that does
**not** exempt: a malformed valve must not buy the skip it failed to justify.
The valve is per **block**, not per token, because a block that legitimately
carries one illustrative sha usually carries its siblings too, and a per-token
valve would put more markup in the record than record.

**Append-only within the iteration, never edited in place.** A survey that
turned out wrong is superseded by a later block answering the same question, not
by a correction to the old one — the record is evidence of what was believed
when, and rewriting it destroys the only thing that makes a stale finding
diagnosable after the fact.

**The witness — the re-use protocol.** A session about to buy a survey reads the
record first and, for any block whose heading answers its question, runs the
witness:

1. **Corpus still?** `git diff --quiet <rev>..HEAD -- <corpus>` — clean means no
   commit since the survey touched anything it covered.
2. **Oracle still?** Re-run `<oracle>` and compare its verdict to the one the
   finding was written against.

**The witness is five strings, and that is what makes a finding portable.** It is
not a property of the record file: the protocol's whole input is `corpus`,
`oracle`, `rev`, `edges` and `finding`, each a short string, and both commands
run from HEAD. Copied onto another surface the witness is *more* durable than it
is here, because the copy is not truncated.

Both hold → **cite the record; do not re-buy the survey.** Either moved →
**dispatch only the delta**, the dispatch prompt naming the record block and the
diff, so the child re-surveys what changed rather than the corpus. The
asymmetry is what makes this safe to ship: a false *stale* costs one
re-derivation — exactly today's cost, so the mechanism's worst case is the
status quo — while a false *fresh* would need a change touching neither the
corpus nor the oracle's verdict, in which case the finding is in fact still
true. The mechanism can only degrade *toward* the behavior it replaces.

**The honest limit.** A survey whose grounds are *not* an oracle — a judgment
over prose, a reading of history — records `oracle: none`, and that block is a
**note, not a re-usable survey**: readable for orientation, re-derived before it
is relied on. Stating that costs one line per record and is the difference
between a mechanism and a false-assurance surface. An *empty* oracle is the
silent form of the same thing and is refused (§check-survey-record).

*Ruled out: a timestamp freshness window.* "Trust a survey under N hours old" is
the per-kind taxonomy in disguise — wrong in both directions (a one-minute-old
survey is stale if the tree moved; a week-old one is fine if it did not) and it
buys nothing the two-command witness does not. *Ruled out: auto-invalidating
every block on any commit* — too coarse to leave the mechanism any use; the
`corpus` pathspec exists precisely to make invalidation proportionate.

**The affordance.** `bash gate-sdk/bin/run-gates.sh --emit file-survey [--]
"<question>" "<corpus>" "<oracle>" "<edges>" "<finding>"` appends one block,
seeding the contract header when the record does not yet exist. It is a bridged
non-gate arm (gate-sdk/SPEC.md §The non-gate arm) declaring two reads,
`LIFECYCLE_KIT_SURVEY_RECORD_FILE` and `LIFECYCLE_KIT_STATE_FILE`; the family is
forced rather than chosen, since the arm resolves consumer knobs and a hardcoded
flag would resolve platform defaults while silently ignoring every override. It
keeps the repo-root anchor — a relative record path names the same file from any
subdirectory, falling back to the working directory outside a repository — exit 2
on a missing or empty argument, and the free-text argument-shape contract of
gate-sdk/SPEC.md §The bin/-tool contract, whose refusal here scans **every**
positional, since five slots make arity no protection at all. Advisory tooling,
not a gate — the raw append stays a legal fallback, the grammar being the
surface's contract rather than the writer.

**The shape refusal crossed the port and the help arm did not**, and the split is
the hazard's rather than the substrate's. A flag captured into a committed surface
at exit 0 is a property of free text reaching a *capture* tool, attested three
times, so it does not retire when the tool stops being a `bin/` script:
a positional beginning with `-` that is not preceded by `--` is a refusal at exit
2, on every slot, and `--` still ends option processing. Usage, by contrast,
belongs to the substrate and lives in the front-end's own help, so
`--emit file-survey --help` is a **refusal**, never a capture.

The arm **does not default the `edges` slot**. An omitted fifth argument is the
arity misuse it already refuses, which is the behavior wanted: a session
that forgot the field is told at filing time, by the producer, rather than at
commit time by the gate.

It **stamps `rev` and the date itself and derives `<stage>` from the cursor**
(the crate's `stages::current_stage` read over `LIFECYCLE_KIT_STATE_FILE`, which
is the only reason that knob sits on the arm's roster), which is the load-bearing
decision in the tool:
`rev` is the field the entire re-use protocol turns on and exactly the field an
author would get wrong — a short sha, the rev they *started* at, or none.
Machine-stamping it is how the mechanism avoids failing silently when someone
forgets, which is the failure shape that rules out author-supplied conventions
elsewhere in this kit. A tree with no `HEAD` commit cannot ground a witness, so
that is a refusal (exit 2) rather than a blank field; a tree with no cursor yet
stamps the never-named `—` the queue header already uses.

The arm **deliberately does not** inherit `--emit-file-gap`'s slug resolution. A
survey's prose routinely names queue slugs as its subject, and that resolver scans whole
prose, so it would stamp a recurrence declaration onto the survey's subject.
Adding no resolver here is a decision, not an omission.

**A permanent surface carries the finding, never a pointer into here.** A surface
outside the boundary-truncated set — a queue entry, a SPEC section — must not
promise a reader retrievable content in this record: the boundary reset below
empties it, so the pointer resolves to nothing one iteration after it is written,
and the finding survives only in the evicting commit. It inlines the finding
together with the block's five witness fields instead, which is what keeps the
finding *re-usable* rather than merely readable. Naming this record as a
**subject** is unaffected — "the survey record is per-iteration scratch" promises
nobody a retrieval — and that distinction is exactly what
§check-scratch-citation's red condition is calibrated against.

**Commit-pinning the citation is wrong by construction, not merely awkward**, and
it is the close a reader reaches for first. The capture arm stamps `rev` as
HEAD *at filing time*, which precedes the commit that lands the block — so
`git show <rev>:<record>` reads a blob that does not contain the block being
cited, and it fails silently by printing a record without it. The sha a pin would
need is the *landing* commit, which no field carries and which is precisely the
class of value this section already rules an author gets wrong.

**The affordance that makes inlining one command.**
`bash gate-sdk/bin/run-gates.sh --emit cite-survey [--] "<heading-substring>"`
selects the one block whose `## ` heading contains the
substring and writes it to stdout as an inline-ready snippet — the heading
rendered `**Carried survey — <heading>**` and all
five fields in record order, off the same field set the block grammar above
defines. It refuses (exit 2) on no match, on an ambiguous match, and on an absent
record, rather than guessing: the author asked for one finding, and
a silently-chosen sibling would be pasted onto a permanent surface as if it were
the one they read. The no-match refusal prints the record's headings and the
ambiguous one prints every match, because narrowing is what the author needs
back. It follows the capture arm exactly — the repo-root anchor, exit 2 on a
missing or empty argument, and — per
gate-sdk/SPEC.md §The bin/-tool contract — the shape contract for its one
free-text positional, its help arm retired on the same ground. Its declared read
is `LIFECYCLE_KIT_SURVEY_RECORD_FILE` **alone**: it derives no stage and stamps no
rev, so its sibling's second knob is deliberately off its roster. It is advisory
tooling, not a gate, the same disposition its sibling carries. Its coverage by the
shape contract is a census find rather than a firing: it carries
`--emit-file-gap`'s exact single-argument shape and had simply never been run with a
flag, and while it writes nothing, its help behavior was the same misleading
error — the half of that finding the port discharges outright.

**Four shell files implement this section and the 2026-09-01 port cut took two of
them**, so the section is not discharged and that is written here rather than
left to be inferred. The two that moved are the affordances above, each of which
declared this section in its own `# spec:` header. The two that did not are
reachable by no stated-contract cut selecting on this section, because each
declares a different one: `bin/enter-stage.sh` carries this section's **read
trigger** — the entry report that prints the record's headings and never its
findings — together with the boundary truncation below, and declares
§bin/enter-stage.sh, so it ports in a different cut; `lib/stages.sh` carries
`LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS`, the surface list the no-retrieval-pointer
rule above reads, and is header-declared `no-port` as the config
bridge's sole resolver for the `LIFECYCLE_KIT_*` knobs (gate-sdk/SPEC.md §The
kit-library port disposition). That last is structural rather than a sizing
judgment, so **this section's contract will not be wholly in-crate while that
class ruling stands.** The general shape outlives the cut: a stated-contract cut
ports the files that *declare* a section, not every file that *implements* it,
and the two sets come apart wherever a shared entry point or a config library
carries one clause of another section's contract.

**What a consumer with no binary for its host loses is the two affordances and
not the surface**, which is the whole of the port's residual here. The raw append
is already ruled the sanctioned fallback, `check-survey-record` is already
compiled and stays the assertion, and the read trigger and the boundary
truncation live in `bin/enter-stage.sh`, which the paragraph above shows stays
shell. So such a consumer still files surveys by hand, still has them asserted,
still has them printed at every stage entry and still has them truncated at the
boundary; what it goes without is one convenience and one witness hint.

The citation arm deliberately does **not** rewrite the citing surface. The author
chooses where
the finding belongs and how much of the `finding` prose to carry; a tool that
spliced would need a marker block, which would make a hand-written citation a
second-class form of the very thing this rule is trying to make ordinary.

**How this differs from the gap inbox, which it sits beside.** The two are
sibling committed per-iteration surfaces with deliberately *opposite* semantics
on both axes, and a reader who meets them apart will assume symmetry and get
both wrong:

- **Merge.** The gap inbox is `merge=union` — a gap filed on either side of a
  concurrent merge must survive. The survey record is `merge=iteration-scoped`
  (keep-ours): a survey from the other side describes a tree state this clone
  never had, so surviving the merge is the hazard, not the save.
- **Boundary.** The gap inbox *refuses* the iteration boundary while it holds
  bullets, because every gap owes a disposition. The survey record is
  boundary-**truncated and never refuses**: a survey owes nobody a disposition,
  being scratch whose whole lifetime was the iteration that just ended
  (§bin/enter-stage.sh).

That per-iteration lifetime is delivered by the boundary reset rather than by a
judgment — which is precisely the window the design needed sized. The record
resets as a **kit built-in member**, not through the consumer's
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` array, by the same rule
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` follows: a defaulted bash array is
*replaced* when a consumer assigns it, so shipping the record as a default
member would silently lose the reset in every consumer that sets the knob for
its own reasons.

Two homes were ruled out, each on grounds that decide it alone. **The resume
journal** (delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset
sweeps) is crash recovery for the
*writing* session, discoverable by its per-session name — a hand-off surface
must be discoverable by content, by a session that never knew the writer's name
— and it lives in untracked scratch with a second reclaimer, context-kit's
session-context hook, whose age guard is shorter than an iteration
(context-kit/SPEC.md §The session-context hook); a scope-stage journal can be
swept before the stage that would read it ever runs, silently and
time-dependently. **The gap inbox** has the right tier and the wrong semantics
on both axes above: routing surveys there would make the boundary refuse on
residue nobody owes a disposition for.

**Producers and consumers.** Producer: any session that bought a survey, via
`--emit file-survey` through the battery front-end (the raw append the sanctioned
fallback); the knob default makes the channel live everywhere the kit is vendored, so the deployed
configuration that must be set is none. Consumer: the next stage session, at its
`bin/enter-stage.sh` entry — which prints the record's headings, the questions
and never the findings — and at the moment it is about to dispatch a survey,
where the witness applies. Each field's reader at its transition: `corpus` and
`rev` by the consuming session's `git diff` at the pre-dispatch check, `oracle`
by the same session at the same transition to re-run, `finding` by that session
only *after* the witness holds (which is why the entry report prints headings and
not findings), the `## ` heading by `bin/enter-stage.sh` at every stage entry and
by `check-survey-record` at commit time as the block delimiter. Third reader, at
the iteration boundary: the first-stage truncate, which reads the surface as a
whole and discards it.

**Nothing is produced on the delegation side.** No journal contract moves, no
dispatch shape changes, no obligation lands on a dispatched child. The producer
is the *parent*, which is where delegation-kit's durable-before-you-act rule
already puts the write; this surface gives that existing obligation a home for
one shape of finding rather than adding an obligation. It is also **not** the
durable escalation artifact delegation-kit/SPEC.md §The delegation model
prescribes as a child's upward route: that one is mid-run, dispatcher-minted,
per-dispatch and written by the child, where this is a hand-off, single-path,
per-iteration and written by the parent after the child returns. Different
writer, reader, lifetime and discovery key — they must not be collapsed.

**Open, and shared with the gap inbox:** who commits an append filed by a
session that is not the stage session. This surface inherits that question and
does not answer it; the exposure is smaller, since a survey lost to an
uncommitted append costs a re-derivation, which is today's cost.

## The close-surface roster

Close reads a set of **inbound triage surfaces** — capture logs, harvest sinks,
the gap inbox, the queue's Lessons section. Enumerated only as prose, that set
has no closure: a surface close never reads leaves no trace anywhere, and a
sixth inbox added without being named costs nothing at the moment of the mistake
and everything afterwards. The roster replaces the enumeration with a
derivation, and gives each surface a mode so a skip is a *visible* judgment
rather than an invisible omission.

**The declaration.** A surface declares itself in the section that already owns
it — one full-line directive, the same shape and altitude as canon-kit's `spec:`
and `contract:` directives (the derivation skips fenced blocks, so the grammar
is quotable where it is specified — `check-spec-pointer`'s carve-out, for the
same reason):

```
close-surface: <path> <mode> [reclaim=<command>]
```

- `<path>` — repo-relative, or a `<file>#<section>` locator when the surface is
  a section of a larger file; the fragment is the heading in anchor form (spaces
  as `-`), since `<path>` is the line's first whitespace-delimited token.
- `<mode>` — exactly one of:
  - `forced=<owner-path> §<section>` — a structural forcing function exists and
    the citation names it. The gap inbox's is the iteration-boundary entry
    refusal; the Lessons section's is that refusal's sibling assertion.
  - `advisory` — no forcing function. Close reads it by procedure, and a skip is
    **sanctioned and visible** rather than undetected. An advisory surface is not
    a lesser surface; it is one whose skip is a judgment someone may audit.
- `reclaim=<command>` — required when `<path>` is a capture-tier (gitignored)
  member, naming the drain that empties it. It runs to end of line, so it is
  written last. The runtime-artifact lifecycle rule already demands a paired
  reclaim path for every write path; this is where that pairing becomes
  machine-readable.

Declaration lives with the owner, never in a central list: a central list is a
second source that drifts from the surface it names, and the one-owner rule puts
the fact where the surface is defined.

A `forced=` declaration belongs on a **manifest surface**, and that is
load-bearing rather than incidental — it is what gives the citation its
resolver. `check-spec-pointer`'s prose-citation pass sweeps the manifest set for
a free-prose `<path>.md §<heading>` citation and resolves it in prefix mode; a
`forced=` citation *is* that shape, so it resolves today with no new code. The
restriction binds `forced=` only, not the directive as such: an `advisory`
declaration carries no citation and so needs no resolver, which is what lets a
consumer-owned capture surface declare itself in the binding that owns it
(a stage-skill binding is not a manifest surface). The honest limit follows from
the same seam — a `forced=` declaration authored outside the manifest set would
carry an unresolved citation, and no gate here catches that, because the
resolver is canon-kit's and lifecycle-kit does not depend on canon-kit.

**The derivation** is the `close-surfaces` emit arm
(§The close-surfaces emit arm), never a maintained registry. Two
sources, unioned: every `close-surface:` declaration across the resolved kit
roots and the consumer's configured declaration surfaces; and every **gitignored
member of the workflow directory** — capture-tier by definition (gate-sdk/SPEC.md
§The workflow directory), therefore close-inbound by definition. The second
source is the closure that makes the roster fail loudly: a capture surface added
with no declaration appears as `(undeclared)` rather than not appearing at all.
The roster reports the hole instead of inheriting it, which is the whole
difference between a derived roster and a maintained one.

**Ruled out: shrinking the roster by merging the two capture logs.** The obvious
way to make close's inbox count smaller is to merge the two friction capture logs
behind one file with a type column. It is ruled out, on this roster's own
evidence:

- The logs are owned by **different kits**, and the dependency runs one way only
  — the drift-kit KPI already reaches into guard-kit through the shared kit-root
  resolution, so guard-kit cannot depend back without a cycle. A merged log has
  no legal owner short of the base gate framework, which is not a friction sink.
- Their producers are not the same kind of act: one is a **harness hook
  fallthrough** writing raw command text at the moment of a prompt, undated and
  ungrammared; the other is a **deliberate structured capture**. A type column
  would not unify them, it would document that they were never one stream.
- Their consumers are disjoint — allowlist-filtering and pattern-ranking on one
  side, doc-owner remediation on the other. Every consumer would filter by type
  first, re-deriving the two logs at read time, which is the tell that the merge
  moves the split rather than removing it.
- Their reclaim moments are independent whole-file truncations. Sharing one file
  makes each sweep's drain erase the other type's untriaged lines.

The complaint the merge reached for is real — the two frictions compete for one
triage attention and were ranked against each other by nothing — and the roster
answers it directly: both appear on one derived roster, with modes, which is what
"ranked against each other" needs. Merging the files was the proxy, not the thing.

## Testing

Every gate ships the ordinary `good/`+`bad/` fixture pair, and the scenarios one
pair cannot hold live in `gate-tests/*.test.sh` scenario runners. What departs
from the plain fixture-pair default, and what no single gate's subsection below
can own, is how those runners **reach** their gate.

A runner that drives a gate names that gate, never a substrate: it resolves through
`gate_run <name> <checks-dir> <args>` (gate-sdk/SPEC.md §run-gate-tests), which
dispatches through `gate_command` exactly as the fixture harness does. A member
that ports to a compiled subcommand therefore leaves its scenario coverage
running unchanged — the property a path-named runner does not have, since the
`checks/<name>.sh` it holds stops existing in the motion that lands the
descriptor. For a case that differs by a knob rather than by argv, `gate_env
NAME=VALUE` sets that one case's environment in the caller's subshell;
`check-stage-evidence`'s session-boundary posture cases are the worked instance.

Every gate-driving runner in this kit resolves through `gate_run` rather than a
held `checks/<name>.sh` path, and the payoff has now been collected **twice**:
`check-stage-entry`'s runner needed no edit when that gate ported, and
`check-close-surfaces`' sandbox-repo runner needed none when this one did —
each already named the gate. Two attestations rather than one exception, which
is the stronger claim this rule was written to earn.

The rest exercise `bin/` tools, which are advisory tooling with no gate to
dispatch and so have nothing to say about reach.

`gate-tests/gap-inbox-route.test.sh` is one of those, and it is named because it
closes a hole rather than adding coverage to a covered path: the
iteration-boundary gap-inbox check had **no fixture at all**, so both its refusal
and its recovery text were unpinned while being the thing an entering session
acts on. It drives `bin/enter-stage.sh` against a sandboxed queue, state file and
inbox — the harness `gate-tests/boundary-scratch-wipe.test.sh` established — and
pins the branch both ways: the close-skipped case refuses with the drain recovery
and writes nothing, the post-close case stamps and carries the bullets with its
advisory, the never-named and no-cursor edges take the post-close branch, and
`--simulate` reports each branch with its recovery relayed and writes nothing. No
fixture *pair* is owed: this is a `bin/` tool, not a gate.

## Per-component contracts

### lib/stages.sh

The sourced config loader: consumer config first, defaults fill what it left
unset (an explicitly empty value disables a knob where the contract says so),
then validation. Also owns the shared state adapters
(`lifecycle_header`, `lifecycle_header_iter`, `lifecycle_current_stage`,
`lifecycle_stage_known`) — both gates must read the two axes identically, and
a shared adapter removes that drift axis.

**It is permanently shell and declares so in its own header**, as the config
bridge's sole resolver for the `LIFECYCLE_KIT_*` knobs — gate-sdk/SPEC.md §The
kit-library port disposition rules the class and gate-sdk/SPEC.md §lib/gate.sh
states the ground. `LIFECYCLE_KIT_PREDECESSOR` crosses the bridge from here as
the tree's live keyed-knob instance, which is this member's sharpest form of it.

`lifecycle_current_stage <state-file>` is the **cursor derivation**: the last
data line's `<stage>` token, the read `bin/enter-stage.sh` already performed
inline, hoisted so every lifecycle reader shares one definition of "current
stage". It prints empty and returns *success* for both no-cursor shapes (§The
state machine) — an absent file and a file with no data line — because "no
cursor" is a legitimate state rather than a parse failure; its two callers
each rule on what it means for them (§check-stage-entry, §check-stage-evidence).
`lifecycle_header_iter` keeps stripping an optional trailing bracketed field:
that strip is now **residual-field healing**, letting a pre-upgrade header
still carrying `[stage:]` read as the bare iteration name. The cross-kit
readers deliberately do *not* call this helper — each derives the cursor
itself from a path it already configures, so no consumer kit gains a
lifecycle-kit dependency.

`lifecycle_closing_stage_reached [<state-file>]` is the **closing-stage
predicate** built on that cursor: success when the cursor equals the last member
of `LIFECYCLE_KIT_STAGES`, failure otherwise — including for both no-cursor
shapes, since a cursor that does not exist has not reached anything. It is
hoisted rather than spelled at each site because its **two callers must agree by
construction**: the `--emit-file-gap` arm reads it for the capture-time warning
that tells a filer which consequence they are buying, and `bin/enter-stage.sh`
reads it at the iteration-boundary gap-inbox check to choose between refusing and
admitting (§The committed gap inbox). **Since that arm ported, the guarantee
holds inside each substrate and is lost across them** — the crate composes the
same test from `stages::current_stage` and the last `LIFECYCLE_KIT_STAGES`
element, so the two agree by lookalike until `bin/enter-stage.sh` ports, at which
point the hoisting's original guarantee returns without further work. A filer warned that "none is left to drain
it" is warned by the very test that later admits the bullet, rather than by a
lookalike that can drift from it. **No knob is minted and none is possible**: the
last configured stage is already `LIFECYCLE_KIT_STAGES`'s last member, so the
predicate is config-derived in every consumer with nothing left to configure. It
is a predicate over the cursor and must not acquire a third caller silently —
`lifecycle_current_stage` remains the general reader.

The loader also owns `lifecycle_registration_block`,
which renders the resident registration block (§bin/install-lifecycle.sh) from the
live config. Three more renderers follow the same
writer/asserter shape for the merge-attribute surface: `lifecycle_supersede_set`
prints the derived iteration-scoped supersede set (the state file, the two
kit-owned built-ins — the lesson-evidence file and the survey record — and each
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member: exactly
what `bin/enter-stage.sh` truncates at the boundary); `lifecycle_union_set`
prints the derived union-merge set (the gap inbox — §The committed gap inbox);
and `lifecycle_merge_attrs_block` renders the supersede set as
`<path> merge=iteration-scoped` lines and the union set as `<path> merge=union`
lines (§Multi-operator semantics). The writer/asserter shape those four express
is live and unchanged — it holds between the `--install-lifecycle` arm and the
gates that assert what it writes (§check-merge-attrs,
§check-lifecycle-registration), in one substrate.

**All four are shell holders with no caller outside this file, kept rather than
deleted with this cut.** The 2026-09-03 port of §bin/install-lifecycle.sh
moved the writer in-crate and closed their caller set on itself: nothing outside
`lib/stages.sh` names any of the four, and inside it `lifecycle_merge_attrs_block`
is the sole caller of `lifecycle_supersede_set` and `lifecycle_union_set` — which
is why they leave together or not at all. The compiled
counterparts `crate::stages::registration_block` and
`crate::stages::merge_attrs_block` are what the arm and the gates read.
An empty caller set makes criterion 6's dead-twin road **available** and does not
make it that cut's to take: these are documented members of *this* section, that
cut's stated contract was §bin/install-lifecycle.sh, and a section is a cut's
outer bound (gate-sdk/SPEC.md §Porting a gate to the binary substrate; §The
port-candidate criteria, criterion 6). So they stay — but **not as an `owed`
entry of their own**: `--tree` reads a disposition off the whole file's header
block, and this file's header already declares **no-port**, permanently, as the
config bridge's sole `LIFECYCLE_KIT_*` resolver (stated three paragraphs above) —
a declaration this cut neither made nor could narrow to four functions, since the
oracle has no function-level column, only a file one. These four are dead code
sitting inside a `no-port` file, not an owed or held file of their own; they take
no `# port-until:` because that field is file-scoped too and would misdescribe
the file around them. The cut that next takes §lib/stages.sh's own contract
removes all four in one motion. Written here so that cut does not rediscover the
caller set from scratch (lead-ruled 2026-09-03, own-authority; filed to the gap
inbox with its probe). `lifecycle_stage_journal <stage>` is the **journal-path
derivation**:
`LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN` with `<stage>` expanded, hoisted here for
the same reason the cursor is — three readers must name one file or the
assertion checks a path nobody was asked to write. Its readers are
`bin/enter-stage.sh` at the entry assertion, a dispatching supervisor deriving
the path it grants, and the dispatched stage session deriving where to write
(§The state machine).

The validator roster is fail-closed on **shape, not merely on presence**, and
`LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` states the rule the others follow: a
non-empty pattern must compile as a POSIX ERE and must declare a capture group,
each refused with its own message. Compilation is read off bash's own `[[ =~ ]]`
status — 2 for a pattern it could not compile, 0 and 1 both meaning it compiled
— rather than off a second regex engine that could disagree with the one the
consumer will actually run against. The refusal exists because the failure it
replaces is silent: an uncompilable pattern matches nothing and a group-less one
captures nothing, and either would classify every worktree unclassified while
looking configured. **The probe's status is captured in a condition context**, and
that is a contract of this loader rather than a spelling: a probe designed to
return non-zero on a routine non-match, run as a bare command, aborts every
`set -e` caller that sources this file. The attested cost of getting it wrong was
a consumer with a pattern configured being unable to re-emit its own derived
surfaces, silently and at exit 1 — that caller was `bin/install-lifecycle.sh`,
which ported on 2026-09-03, and no surviving shell sourcer in this tree
(`bin/enter-stage.sh`) runs under `set -e`. The contract binds
the next one that does, which is why it is stated as a property of the loader
rather than as a note about one caller.
Exercised in `smoke/` with a pattern actually set, the empty default never
reaching the branch. `LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN` takes the same
treatment for the same reason: a pattern with no `<stage>` placeholder names one
file for every stage, so the entry assertion would read some other session's
journal and **pass** on it — a wrong answer, not a missing one, which is why it
is refused rather than tolerated. `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` takes the
`0|1` arm shape `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK` already has.
`LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE` is resolved here and deliberately gains
**no** validator arm, which is the roster's rule applied rather than an omission
from it: its value is a path that need not exist — header-only is the valve
ledger's resting state (§bin/enter-stage.sh) — so there is no shape a load-time
check could refuse. The two fail-closed refusals that surface *are* the ledger's,
and they live in the writer, where the file is actually read: a loader that
parsed the ledger would refuse every entry on a malformed one, including entries
that never asked whether a valve was armed.
The derived supersede set **is read past the merge-attribute pair**, and those
readers take the compiled `crate::stages::supersede_set` rather than this shell
holder: `check-scratch-citation`, which forbids a permanent surface pointing a
retriever at any of its members, and `check-stage-evidence`. So a consumer adding a
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member gets citation enforcement over it with no
second roster to keep in step. Values and adapters only, never
gate structure (gate-sdk's `lib/gate.sh` rule).

### bin/session-id.sh

Prints the canonical stamp id so a stage skill reads it rather than guessing.
**The member is the gate binary's `--emit-session-id` arm**, reached through the
generic composer as `bash gate-sdk/bin/run-gates.sh --emit session-id`. **The
heading is a section name, not a file name** — no `.sh` driver stands behind it,
and it keeps this spelling because the citations pointing here resolve against
it. It is not a gate — no
`gates.list` row, no `.gate` descriptor, no fixture pair — but a bridged-arm
table member (gate-sdk/SPEC.md §The non-gate arm), whose contract is a
**document**: one normalized id on stdout and exit 0, or a diagnostic on stderr
and exit 2.

The id derives by a fixed source order, first hit wins, every source ending in
the same normalization — strip a leading `agent-` token if present, then take
the first 8 characters:

1. `LIFECYCLE_KIT_SESSION_ID` — a harness-neutral consumer override: a consumer
   whose harness exposes a session identity by any means wires it here.
2. `CLAUDE_CODE_SESSION_ID` — the shipped default source, harness-specific by
   nature: this harness exports the current session's transcript uuid into every
   Bash environment, identifying the session directly rather than inferring it
   from file mtimes. Taken directly here only when `CLAUDE_CODE_CHILD_SESSION`
   is unset; when the flag is set a lead-dispatched stage session
   (§templates/lead.md) may see the *lead's* uuid here, so source 3 verifies
   the flag before trusting it and can route back to this uuid.
3. The newest transcript under the sessions dir (default
   `<config-home>/projects/<cwd-slug>` — `$CLAUDE_CONFIG_DIR` or `~/.claude`,
   and the cwd with every non-alphanumeric char mapped to `-`; override
   `LIFECYCLE_KIT_SESSIONS_DIR`), the top-level glob widened with
   `<dir>/*/subagents/*.jsonl` so a dispatched session with neither env var
   still resolves without a per-dispatch override. Newest-file selection is the
   documented single-operator assumption (one live session per project tree) —
   and the widened glob makes it bite *within* one session too: a top-level
   session that reaches this source (its Bash environment lacking the source-2
   uuid) right after one of its subagents finishes picks that subagent's
   transcript when it out-mtimes the session's own, so a lead deriving its own
   id here (the session-role marker, §templates/lead.md) can name the wrong
   session and misfire the suppression — a lead in that position verifies the
   printed id against its own transcript before writing the marker. A
   dispatched child (source 2 skipped, `CLAUDE_CODE_SESSION_ID` carrying the
   lead's uuid) narrows this scan to `<dir>/<lead-uuid>/subagents/*.jsonl`
   alone, excluding the lead's own top-level transcript — concurrently written,
   and able to out-mtime the dispatched session's — from the candidate set. The
   flag is verified, not trusted, because this harness sets it in top-level
   sessions too: a non-empty narrowed scan is a genuine child (newest subagent
   transcript wins); an empty scan with `<dir>/<lead-uuid>.jsonl` present marks
   the flag spurious — a genuine child's transcript lives under `subagents/`
   while it runs, so an empty scan plus a top-level transcript for the env uuid
   means the uuid names a live top-level session, and the derivation falls back
   to `CLAUDE_CODE_SESSION_ID` (source 2's answer). An empty scan with no such
   top-level transcript exits 2 — only a wrong sessions dir or a broken layout
   still reaches it. Two races are accepted: a genuine child stamping before
   its transcript's first write would fall back to the lead's uuid (theoretical
   — a child's transcript has its first writes by the time it can run a tool
   call), and a spurious-flagged session that dispatched subagents earlier in
   the same session stamps the newest subagent's id (a provenance smudge, not a
   correctness break, unchanged from the prior trusting behavior). An absent dir
   or transcript exits 2.

Invoked internally by `enter-stage.sh` for the `<session-id>` field; the stage
skills reach it through `enter-stage.sh` rather than calling it themselves. The
one session that calls it directly is a lead writing its own session-role marker
(§templates/lead.md), which the front-end route serves.

**The declared knob roster is empty, and it must be.** `lib/stages.sh` defines
neither `LIFECYCLE_KIT_SESSION_ID` nor `LIFECYCLE_KIT_SESSIONS_DIR`, and a
bridged arm declaring a knob its owning kit's library does not define is the
config bridge's undeclared-knob refusal (gate-sdk/SPEC.md §lib/gate.sh) — the
arm would fail-close on every invocation. Table membership is nonetheless what
makes the arm *reachable*, so the row exists with an empty roster, the second
such member after `--emit-md-section`. Adding the two defaults to `lib/stages.sh`
so the names could be declared is a **behaviour widening rather than a port** and
was refused on that ground: the driver sourced no library, so neither name has
ever resolved from a `LIFECYCLE_KIT_CONFIG_FILE`, and bridging one would make a
config file start working that does not today.

**Both names reach the arm anyway, and so do the harness's, because the bridge
*adds* to the environment rather than replacing it.** `gate_command` and
`exec_arm` both compose `env <resolved knobs> <binary> <arm>` (gate-sdk/SPEC.md
§lib/gate.sh, §run-gates), never an `env -i`, so `LIFECYCLE_KIT_SESSION_ID`,
`LIFECYCLE_KIT_SESSIONS_DIR`, `CLAUDE_CODE_SESSION_ID`,
`CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CONFIG_DIR` and `HOME` reach it exactly as
they reached the driver. §The non-gate arm's rule that *a default the deleted
shell driver held inline moves into the owning kit's library in the same cut*
does not bind here: its ground is a **declared** knob resolving empty through the
bridge, and this arm declares none.

**The process cwd is an input to source 3, so `enter-stage.sh` reaches the binary
directly rather than through the front-end.** `bin/run-gates.sh` cds to the git
toplevel before dispatch (gate-sdk/SPEC.md §run-gates), which would change the
sessions-dir slug under any other cwd and turn a non-repository cwd from working
into exit 2 — so the sole production caller invokes `gate_native_bin`'s
`--emit-session-id` directly, which §The non-gate arm sanctions in as many words:
what the class forbids is a second entry point into the emission path, not a
caller. The front-end route stays available and is what `templates/lead.md` and
the consumer smoke use, both standing at the repo root where the two agree.
Pinning the sessions dir to the toplevel would be an *arguable repair* inside a
port, and was refused as one. One spelling is load-bearing so a later rewrite
does not silently change it: bash's `pwd` prints the **logical** path it carries
in `PWD` where `std::env::current_dir()` returns the physical one, so the arm
reads `PWD` where it is set and the crate's crosser answers otherwise.

**The caller's absent-binary refusal is the port's one added surface.**
`enter-stage.sh` reads the id before it dispatches any gate, so nothing else in
that run would have reported an unbuilt binary first; it checks `gate_native_bin`
is executable and refuses with the build command, in the shape §lib/gate.sh's own
two readers use. Neither the derivation nor either exit status moved otherwise —
the port was held against the deleted driver over the derivation order's five
axes, all three refusal texts, a cross-tier mtime tie, a broken symlink, a
dotfile and four cwd axes, with no difference in output or status.
**In-envelope rather than a widening — ruled 2026-09-03 by the lead on its own
authority.** The cut *opens* this failure mode rather than inheriting it: the
deleted driver was a vendored shell script, present wherever the kit was, where
the arm needs a **built** binary — so the refusal covers a case that did not
exist before the cut, and a faithful port owes the case it created. Recorded as a
lead ruling and not an operator-blessed one: the operator was asked whether they
read the added refusal as a widening and had not answered when the cut closed.

**The port disposition: taken 2026-09-03**, hosted on
`native-gate-port-remaining-corpus`, which discharges the deferral this section
recorded — the member was dropped once from
`declaration-install-and-stage-helper-cuts` for want of a host rather than for
any property of the file, and a host is all it ever needed. Exactly **one**
`.claude/settings.json` allow entry named the deleted path, and the deleting
commit carries its deletion — the count probed rather than assumed, per the
2026-08-29 settings-grant carve-out on that entry. No grant is added: the
post-port
invocation is covered by the committed `Bash(bash gate-sdk/bin/run-gates.sh)`
entries. The cut created no twin — the derivation sources no kit library, and
the shell caller set emptied — so no parity oracle is owed.

**One question this cut deliberately leaves open**, so a reader does not take the
silence for a discharge: `LIFECYCLE_KIT_SESSION_ID` is documented on §Layout and
configuration as a kit knob but is environment-only in practice, and whether it
should become bridged or be redocumented is filed to the committed gap inbox
rather than ruled here. It outlives the port in either direction.

Two facts a session taking this cut should not re-derive. Its sole production
caller is `bin/enter-stage.sh`, which resolves the id internally — but the kit's
own stage templates and `templates/lead.md` name this helper as the id's source
and one of them **invokes it directly**, so a port edits kit `templates/*.md` and
stales whatever projection reads them. And it is the one member of that iteration's
candidate set whose path a committed permission grant names, so the 2026-08-29
settings-grant carve-out on `native-gate-port-remaining-corpus` is exercised
**once** here — the count probed rather than assumed, as that ruling requires.

### bin/enter-stage.sh

The deterministic writer for a stage transition: `enter-stage.sh <stage>`
appends the invocation stamp, reading the `--emit-session-id` arm for
the id — never an argument, so the no-hand-picking rule rides into the tool.
It is the **sole production writer of `<head>`** (§The state machine), read as
`git rev-parse --short HEAD` in the state file's own work tree at the instant of
the append, and `none` where that yields nothing — so the field is produced by
the live path every stage template's first step already invokes rather than only
under test. It takes no enabling knob: the field is unconditional, which is what
refusing an optional spelling buys.
`<stage>` must be a configured stage; anything else is a usage error (exit 2).
Its positionals are membership-validated rather than free text, so it owes no
leading-`-` refusal (gate-sdk/SPEC.md §The bin/-tool contract) — but it owes the
help half, and `-h`/`--help` as the first argument prints usage on stdout and
exits 0. The measured cost of not owing it is in this tool's own history: a
session hunting for the rename mode below ran `--help`, got
`'--help' is not a lifecycle stage`, and worked around a contract for three
sessions that the usage text would have settled in one command.
An ordinary stage **writes the evidence file only** — the appended stamp *is*
the transition, since the last stamp is the cursor, so the queue file is not
touched and need not be committed. The first stage
(`LIFECYCLE_KIT_FIRST_STAGE`)
performs the iteration-boundary reset instead — truncating the state file and
every file in `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` back to its contract header and
restarting the header at the unnamed-iteration form. That header rewrite is
also where a residual pre-upgrade `[stage:]` field is dropped, so a consumer
that vendored the cursor extraction mid-iteration heals at its next boundary
without a migration step. `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`
is a generic per-iteration reset knob — no consumer surface is named in
the kit; a downstream kit whose per-iteration file must start each cycle from
its contract header adds itself here, as evidence-kit's manifest does. The two
kit-owned surfaces — `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` and
`LIFECYCLE_KIT_SURVEY_RECORD_FILE` (§The survey record) — reset by the same rule
as **built-in members**: the kit owns those surfaces, so they do not ride the
consumer knob (git history keeps the retired stamps and the retired surveys).

That written-surface set is also the **source of the stamp-commit purity
assertion's exemption** (§check-stage-evidence): the boundary reset is the one
legitimate reason a stamp commit touches anything besides the state file, so the
assertion reads the members this reset already resolves rather than a roster
minted to describe it — which is why widening the reset cannot leave the
assertion behind.

**What counts as the header, for the truncate.** The retained run is the
member's leading blank and `#` **comment** lines, stopping at the first data
line *or* at a markdown `## ` section heading, whichever comes first. The second
half of that predicate is load-bearing rather than defensive: on a markdown
surface whose blocks *are* `## ` headings — the survey record — a bare
"keep every leading `#` line" rule reads the first block's heading as part of
the header and carries one stale survey across the boundary, with the record's
own read trigger then advertising it. The failure is silent, survives a green
battery, and is invisible to any fixture that does not run a real boundary
entry, which is why the boundary behavior is exercised end-to-end in
`gate-tests/` rather than reasoned about.

The boundary reset additionally **wipes the scratch dir** (`GATE_SDK_TMP_DIR`),
deleting every member whose basename is neither `.gitkeep` nor a
`LIFECYCLE_KIT_BOUNDARY_PRESERVE` entry — at any depth the wipe reaches — and
naming the wiped set in its report the way it already names the truncated set.
Truncate and wipe share the boundary trigger and the report line and **nothing
else**: truncate rewrites a *tracked* file down to its `# contract:` header, so
the file survives with an empty body; the wipe *deletes* untracked scratch
members outright. `LIFECYCLE_KIT_BOUNDARY_PRESERVE` is therefore a keep-list for
the delete and never a
truncate target. The wipe runs last — after the truncate loop and after
enter-stage removes its own temp files — so those temporaries are never
candidates, and it is **boundary-only**: an ordinary stage entry appends and
touches no scratch. It is unconditional over the directory because scratch is
disposable *by definition*; a consumer's persistent measurement trends live
outside it by retention contract. A delete that fails because a preserved
basename sits inside an otherwise-doomed subdirectory is noise, never an abort.

`.gitkeep` is a **kit invariant, not configuration** — the consumer cannot unset
it. The kit must not delete a file the consumer tracks: doing so removes a
tracked file at the boundary and dirties the tree at the very moment the
first-stage session commits the reset. The exemption is *not* about the directory
surviving (the tool `mkdir -p`s the scratch dir, and the wipe removes members,
not the dir); it is recorded with its real reason so a later reader does not
retire it as redundant, and it is generic-consumer mechanism rather than a fact
about any one checkout — a consumer that gitignores its scratch dir wholesale
never exercises it. Shipping the exemption as `LIFECYCLE_KIT_BOUNDARY_PRESERVE`'s
*default* is **ruled out**: a defaulted bash array is replaced, not merged, when a
consumer assigns it, so protection would decrease as configuration increases and
any consumer setting that knob for its own reasons would silently lose the
exemption. A
git-aware "spare any tracked file" rule is **ruled out** too — it makes
filesystem behavior git-dependent for one case, and it would spare any tracked
file parked in scratch, re-opening the accumulation the wipe exists to close.

The scratch dir has a **second reclaimer, and the two do not overlap.**
context-kit's session-context hook sweeps the same directory at *every* session
start, age-guarded precisely so a concurrent same-checkout session's in-flight
scratch survives (context-kit/SPEC.md §The session-context hook). The boundary
wipe is deliberately **not** age-guarded: it fires once, at the iteration
transition, where the only scratch a consumer means to carry across is by
definition named in `LIFECYCLE_KIT_BOUNDARY_PRESERVE` — an age guard there would
leave the previous
iteration's fresh residue behind, which is the whole thing being reclaimed. Two
triggers, two postures, one directory; neither mechanism reads the other.

The boundary entry also
**refuses outright when `## Lessons Learned` is non-empty** (exit 1, the
untriaged entries printed, nothing written — the same refusal contract as the
built-in pre-flight): an untriaged lesson must not cross into the next
iteration, so no `[attend]` injection (queue-kit §The queue-index arm) can
outlive the iteration that filed it. Its **gap-inbox check
(`LIFECYCLE_KIT_GAP_INBOX_FILE`) is one detector with two dispositions**
(§The committed gap inbox owns the design): it fires on any bullet, then reads
`lifecycle_closing_stage_reached` to choose. A **close-skipped** boundary
refuses on the same contract as the Lessons check, so a mid-iteration gap the
close skill did not drain cannot cross untriaged; a **post-close** one admits
the entry and prints the bullets as a stderr advisory naming them the entering
stage's intake, because no stage of the closing iteration is coming back for
them. An absent inbox has no bullets and passes either way.
**The survey record, its neighbour in the boundary reset, deliberately does
not refuse** (§The survey record): it is truncated like any built-in member, and
a non-empty one is never a blocker, because a survey owes nobody a disposition —
it is scratch whose whole lifetime was the iteration just ending. The two
surfaces sit one line apart in this tool with opposite semantics, so the
asymmetry is stated rather than left to be inferred. The entry additionally
carries the record's **read trigger**: when the record is non-empty the report
prints its `## ` headings — the questions the iteration's prior surveys answered,
never their findings, since printing a possibly-stale judgment ahead of its
witness is the failure the witness exists to prevent. It rides the tool every
stage already invokes as its first step, so it adds no invocation point and no
schedule, and it lands at the one moment a stage session is guaranteed to be
looking. **This trigger and the truncation above are the survey record's
surviving shell half**: the two capture affordances ported onto the binary in the
2026-09-01 cut, this file declares §bin/enter-stage.sh rather than that section
and so ports in a different cut, and a later cut selector meets that fact here,
where it works, rather than only in the other section (§The survey record).

The boundary entry also **refuses when any linked worktree still stands** (exit
1, each path printed, nothing written — the same refusal contract as the two
above), gated by `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK`. The predicate is
deliberately a property of the boundary rather than a path: **at an iteration
boundary no linked worktree should be live**, an in-flight dispatch being
something that must not straddle a boundary and everything else being residue.
So it fires on every entry of `git worktree list` beyond the main checkout — a
harness's isolated-agent tree, a leaked per-ref worktree, any future producer —
and no knob names a residue directory, because a kit default spelling one
harness's layout would publish it. It is read off `git worktree list` and
**never off `git status`**: a gitignored worktree leaves the status clean while
it still stands, so a status-derived check reports success on exactly the state
it exists to catch. `--simulate` relays the would-be refusal the way it does for
lessons, and a tree that is not a git checkout at all skips the check rather than
failing on it.

**Each linked worktree carries a liveness class, and the signal is git's own
rather than one this kit invents.** A worktree held by a live process is
`locked`, and `git worktree list --porcelain` prints that lock's **reason** —
which, for at least one harness, is a liveness record naming the holder's **pid**
and its process **start time**; the same string sits on disk at
`.git/worktrees/<name>/locked`. Measured rather than assumed, and re-measured at
each stage that rested on it: the start field equals that process's own
`/proc/<pid>/stat` field 22, so the reason carries a PID-reuse guard rather than
decoration — a record strictly richer than the `pid=<n> run=<key>` grammar
evidence-kit reads for backgrounded shell producers
(evidence-kit/SPEC.md §check-producer-liveness). The reap that had no designable
mechanism now has one, and nothing about the signal is minted here.

**The reason's format is consumer vocabulary, taking the same disposition the
residue directory takes one paragraph up.**
`LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` is a POSIX ERE with exactly one capture
group, matched against a lock reason, the group being the holder's pid; the
kit's default is **empty**, so an unconfigured consumer classifies nothing and
sees exactly the behaviour above. The kit ships the mechanism — read the
porcelain, apply the pattern, probe the pid — and the consumer ships the
pattern, the same split `check-graph` takes over its vocabulary
(CLAUDE.md §The provenance seam).

| observation | class |
| --- | --- |
| locked, reason matches, captured pid alive | **live** |
| locked, reason matches, captured pid dead | **orphaned** |
| locked, reason does not match the pattern | **unclassified** |
| not locked at all | **orphaned** |

With the pattern unset every linked worktree is **unclassified**, an unlocked one
included: an empty pattern reads as *no classification is configured* rather than
as a pattern matching everything, which is what keeps the default additive.

**Liveness is `ek_pid_alive` and never a second predicate** — the probe the
`.run` path already uses, so how liveness is decided stays settled in one place
(evidence-kit/SPEC.md §check-producer-liveness). The dependency is **bought by
the knob**: `bin/enter-stage.sh` sources evidence-kit's library only when the
pattern is non-empty, so a consumer that configures none owes no second vendored
kit, and the hard `gate-sdk` dependency stated below gains no sibling for
everyone. A pattern set with that library unreachable is exit 2 — the same
fail-closed direction §lib/stages.sh takes on a malformed pattern, and never a
silent everything-unclassified.

**One capture group and not two.** The start-time field is matched and
deliberately not captured. Parity is the first ground — the `.run` record grammar
carries no start-time guard either, so capturing one here would make the worktree
path stricter than the record path for no stated reason — and the error direction
is the decisive one: a stranded worktree whose pid has been reused classifies
**live**, and a live classification refuses and says *wait*, never authorising a
removal. Dropping the guard errs toward refusing, which is the direction a
fail-closed boundary wants. That safety is a function of what **reads** the
class, so the strengthening is filed rather than banked — the moment anything
reaps on the classification instead of printing it, the argument inverts.

**The refusal set does not narrow; the class changes the remedy.** Both classes
still refuse — a live worktree because an in-flight dispatch must not straddle
the boundary, an orphaned one because residue must be cleared before it is
crossed — and the refusal prints one line per path carrying its class:

- **live** — the holding pid is named and the guidance is *wait, then re-enter*.
  A force-removal is actively wrong here and is not offered.
- **orphaned** — the holder is gone, so the lock states a fact that has become
  false. This is the class force-removal exists for and it is named only here.
  Git requires **`--force` twice** to remove a *locked* worktree; once suffices
  only for an unlocked dirty one, and the single-force spelling this guidance
  used to carry was wrong for exactly the class it was aimed at.
- **unclassified** — today's reap guidance verbatim, which is also the whole
  behaviour an unconfigured consumer sees.

**The reap stays a session act, on a ground the experiment did not discharge.**
The earlier ground — that this kit does not remove a directory whose liveness it
cannot establish — is **retired**, liveness now being establishable. What stands
in its place is untouched by that discharge: a worktree can hold commits existing
nowhere else, so a wipe is destructive rather than merely wasteful. Liveness
answers *is anyone working here*; it does not answer *would removing this lose
anything*. What was missing at every attested firing was in any case not a reap
but anything that *told* a session there was residue.

**The loss question is answered mechanically rather than left to the session.**
For every non-live path the refusal reports two facts read at the moment of
refusal — whether the worktree's tree is dirty, and how many commits its `HEAD`
carries that are unreachable from the main checkout's `HEAD` — and says so
plainly when a path is clean **and** commitless, the case where removal is
lossless by construction. Two git reads, no vendor vocabulary, and exactly the
two facts a session used to re-derive by hand per path. This **replaces** a
generic help line rather than sitting beside it: the retired line said harness
residue is expected rather than evidence the child wrote, and that a hand-run
`git status --porcelain` inside the worktree tells a stray write from an unfired
reclamation — the same question, answered by hand, over the same paths. Keeping
both would leave two lines teaching two ways to learn one fact, which is the
residue this classification exists to remove, applied to its own refusal text.

**Away from the boundary the same scan runs as an advisory.** Every non-boundary
stage entry reads the worktrees and reports **orphaned paths only**, with the
same loss report and the same reap guidance, as a stderr advisory that never
refuses and never suppresses the stamp. It is safe only because the class
exists: an unclassified mid-iteration report would name every in-flight
dispatch, which mid-iteration is the **normal** state whenever a supervisor has
work out, so it would either cry wolf at every entry or push a session to refuse
legitimate work. A live worktree is reported nowhere here, there being nothing
for the entering session to do about it, and with the pattern unset nothing is
classifiable and no advisory is emitted at all.

Two bounds are stated rather than banked, and the advisory moved one of them. The
ceiling was **per iteration**, not per dispatch, with residue accumulating
unseen inside an iteration; that blind spot is closed — residue is now surfaced
at every stage entry — but **surfacing is not sweeping**, so accumulation within
an iteration is visible rather than prevented and the reap remains a session act.
The second bound is unchanged and still open: the check **cannot see a dangling
branch ref** whose worktree is already gone — `git worktree list` does not report
one, and the branch-name pattern that would is one harness's vocabulary — so the
refusal's guidance names the branch half explicitly and the reaping session
removes the ref with the directory. No gate reads any of this:
`check-stage-entry`'s three
assertions are unchanged and the boundary-precondition family has deliberately
never had a gate sibling (§check-stage-entry states the predecessor-map omission
for the same class of reason), and a gate over tree state cannot see a worktree
at all. The grounds for refusing the earlier, sweep-shaped
alternative — a reap in the dispatch guard — are delegation-kit's
(delegation-kit/SPEC.md §The delegation model).

The boundary entry additionally **refuses
when a `LIFECYCLE_KIT_BOUNDARY_REQUIRE` member lacks a disposition line for the
closing iteration** (exit 1, nothing written, the same refusal contract): each
member must carry a data line whose first token is the closing iteration's name,
so a consumer wiring its release-disposition evidence here makes the close-stage
disposition a mechanical boundary precondition rather than a decorative stamp.
The check is **value-agnostic by construction** — it tests the first token only
and never parses the value field — so a disposition grammar gaining a value (as
it did with `deferred:<version>`, §templates/stages/) needs no
widening here; recorded so a future value addition does not re-derive it.
Fail-closed: a member that does not exist on disk is a refusal naming the path. A
never-named (`—`) closing iteration has nothing to disposition and skips the
check. `--simulate` relays the would-be refusal the way it does for lessons. The
require-check runs after the Lessons refusal and before the boundary truncation,
so a member that is also a `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` file is verified by
the same boundary that then consumes it. **Pre-flight,
not enforcement:** before writing, it runs the built-in `check-stage-entry`
for the entered stage plus each `LIFECYCLE_KIT_ENTRY_PREFLIGHT` command whose
stage key matches, and refuses
(exit 1, findings printed, no writes) when any is red.
The `LIFECYCLE_KIT_ENTRY_PREFLIGHT` half of that refusal is **conditional, and
that half alone**: the one-shot valve below admits a single entry past it. A
valve is not the `--force` flag this tool still refuses — it is armed in a
committed file, carries a mandatory written reason, and valves one arm rather
than the tool. The hand-off keeps the
same `<queue> <state>` argv it always had, but **the temp file swapped sides**:
because the cursor is the last stamp, the candidate transition now lives in a
temp *state* file under `${GATE_SDK_TMP_DIR}` carrying the not-yet-written
stamp, while the live queue passes through untouched (the boundary reset, which
does rewrite the header, passes a temp queue as well). The refusal is advisory
in the same sense the gate is at commit time (no `--force`, so the easy path is
the compliant one — and the valve does not reinstate one: it is reached by
committing a file, not by typing a word).
`LIFECYCLE_KIT_ENTRY_PREFLIGHT` is a generic per-stage hook
— no consumer surface is named in the kit; a downstream kit whose gate is the
real precondition for a stage wires itself here (as evidence-kit's manifest gate
does for close entry), turning a would-be pre-commit deadlock into a loud
refusal at the entry. **Each entry's `<command>` is split on whitespace and
exec'd as argv with no interpreter word prepended**, so the configured path rides
its own exec bit: a consumer wiring a gate here configures the gate's *invocation*
and not its declaration file, and a configured path that stops being executable —
a member ported to a binary substrate leaving a data-file descriptor behind — is
a stage-entry breakage rather than a stale reference.

**The one-shot pre-flight valve admits one entry past a refusing pre-flight
command.** `LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE` names a committed **valve
ledger**; its default is the empty string, so no valve exists, every pre-flight
refusal is final, and an unconfigured consumer sees exactly the behaviour above.
The ledger's grammar is the one this kit's evidence files already use — a
`# contract:` pointer header, then one data line per arming:

```
<iteration> <stage> armed|used <reason...>
```

`<reason>` runs to end of line and is **mandatory**. It is what makes the
one-shot a documented one, so a line without one is not a weaker arming; it is
not an arming. It is free text authored by a consumer's own session: the kit
specifies that it must be non-empty and never what it may say. **There is no
date field, and its absence is a decision** — the ledger is truncated at the
boundary, so every line belongs to one iteration by construction and the state
file already dates that iteration's stages; a date here would be a second,
drift-capable copy of a fact another surface owns.
The iteration matched is the **queue header's**, not the stamp's:
the ledger is truncated at the iteration boundary, so its lines belong to the
iteration currently open, while the first stage's stamp carries the unnamed
placeholder rather than a name any arming could have been written against.

When a `LIFECYCLE_KIT_ENTRY_PREFLIGHT` command refuses **and** the ledger carries
an `armed` line for the entering iteration and stage, the entry is **admitted**
instead of refused and that line's state token is rewritten `armed` → `used`.
**The admission is loud**: the report relays the pre-flight command's own
findings — the text the refusal would have printed — says the valve admitted
them, prints the reason, and prints how many `used` lines this iteration already
carried before this one. The entry then proceeds and stamps as normal.

**What the valve is for, stated in its own contract because the ruling makes the
documentation part of the deliverable.** Exactly one deadlock: a stage whose
entry pre-flight is refused by a precondition only a *later* stage can clear, so
the stage chartered to clear it is the stage the pre-flight is refusing.
**Reaching for it twice in one iteration is the failure rather than a supported
mode** — a valve gets reached for whenever the refused stage is inconvenient, and
that prediction is what this contract has to answer. What answers it is not a
prohibition but a **count at the moment of the act**: the admission report prints
this iteration's prior `used` count, so the second reach announces itself to the
session taking it, in its own transcript, before it proceeds. That is the shape
the predecessor-journal escape below already takes — an evadable assertion whose
value is that the deviation becomes deliberate and written instead of silent, at
the one moment someone is looking. The refusal's `help:` line therefore names the
valve **and that single sanctioned cause together**, so it cannot read as a
generic bypass.

**The honest limit is stated with it rather than left to be inferred from it.** A
session can arm its own valve. The valve is bypassable in exactly the sense the
journal escape is, and claiming otherwise would be a stronger claim than the
evidence carries. What it buys is that a bypass leaves a committed artifact with
a written reason, a named obligation on the closing stage (§templates/stages/),
and a count that makes the second one visible.

**Four narrowings, each a consequence of an existing ruling rather than a
carve-out.**

- **This arm only.** The valve does not reach the built-in `check-stage-entry`
  pre-flight (which asserts the state machine's own stamp-protocol invariants),
  the predecessor-journal assertion (which has its own named escape below), or
  any iteration-boundary refusal — the Lessons check, the gap-inbox check, the
  linked-worktree check, `LIFECYCLE_KIT_BOUNDARY_REQUIRE` — each of which guards
  against work leaking across an iteration boundary and each of which already
  states a recovery. `LIFECYCLE_KIT_ENTRY_PREFLIGHT` is the **consumer-wired**
  arm, and a consumer-wired precondition is the only one whose deadlock a
  consumer can reach at all.
- **One line per admission.** Only the **first** matching `armed` line is
  consumed, and one entry consumes at most one however many of its stage's
  pre-flight commands refused — the valve admits the *entry*, not a command. So
  arming twice does not admit twice, and the second line is still there for the
  closing stage to see.
- **Iteration *and* stage must match.** An arming aimed at another stage or left
  from another iteration never admits. Both halves are asserted even though the
  boundary truncation should make the cross-iteration case unreachable, because
  that truncation is a knob a consumer may decline to set, and an assertion
  resting on another consumer's configuration is not an assertion.
- **The idempotent no-op consumes nothing.** A re-entry whose stamp is already
  the last line exits 0 before the pre-flight runs at all, so a crashed-and-
  resumed session cannot spend a second valve line on the same transition.

**Two fail-closed refusals (exit 2, nothing written):** a data line with fewer
than four whitespace-separated fields, and a state token that is neither `armed`
nor `used`. A ledger that cannot be parsed makes *is it armed?* unanswerable, and
**both** silent branches are wrong there — admitting hides a malformed arming,
refusing hides a valid one. The ledger is read only where that question is
actually asked, which is at a pre-flight refusal: parsing it at every entry would
let a malformed ledger wedge entries that never needed a valve, a wider refusal
than the fail-closed arm was ruled for.

**A configured path that does not exist is *not armed*, not an error**, because
header-only is the ledger's resting state and requiring the file would oblige
every consumer setting the knob to create one; the failure direction is then a
refusal, which is the safe one. So that a typo'd path cannot masquerade as a
never-armed valve, **the refusal message names the configured path** and says
which of the two cases it is — absent, or present and carrying no matching
arming.

**Two writers, and the split is contract.** The arming session writes every line
(§templates/stages/); this tool writes **none** — it rewrites the state token of
one line that already exists. So the ledger's line set is the arming session's
alone and this tool can only ever narrow what is admissible, which is what makes
a two-writer surface safe here. The rewrite rides the **write**, not the match:
the boundary refusals run after the pre-flight loop, so an entry that matched an
arming can still refuse, and a refused entry must neither report that it was
admitted nor spend a line saying so.

**A consumer wiring the knob owes its ledger a `close-surface:` declaration**
(§The close-surface roster). The ledger is *tracked*, so `check-close-surfaces`'s
undeclared-surface arm — whose second source is the workflow directory's
*gitignored* members — never reaches it, and undeclared it would simply not
appear on the roster the closing stage's inbound-triage sweep recomputes. No
`reclaim=` field is owed: that is a capture-tier member's obligation. The kit
names no path here, the ledger's location being consumer configuration and a kit
literal spelling one consumer's workflow-directory layout being the seam this
knob's empty default already holds.

**The predecessor-journal assertion runs in that same pre-flight**, gated by
`LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` and defaulting off. At `1`, an entry
refuses when the stage the cursor names left no journal at its derived path
(§The state machine) — **exit 1, the expected path printed, nothing written**,
the boundary-precondition family's contract, and `--simulate` relays the
would-be refusal rather than taking it. Its inputs are both already read here:
the cursor, and the pattern expanded against it. Three narrowings, each a
consequence of an existing ruling rather than a carve-out:

- **Existence and non-emptiness, never the `DONE` marker.** delegation-kit rules
  the marker redundant wherever the supervisor consumed the agent's return, its
  signal being reserved for a cold read. A stage entry is not a cold read — the
  entering session exists *because* the previous stage returned — so asserting
  the marker would mint an obligation its owning section already retires at
  exactly this transition. Absent and empty are told apart in the message,
  because they are different mistakes.
- **The in-iteration predecessor only.** The boundary reset wipes the scratch dir
  at the first stage's entry and `LIFECYCLE_KIT_BOUNDARY_PRESERVE` deliberately
  does not keep **stage** journals, so the first stage of an iteration has no
  predecessor journal by construction and is never asserted against. The
  qualifier is load-bearing rather than pedantic: a *supervising* session's
  journal is a legitimate keep-list member (§templates/lead.md) and is not a
  stage journal, so keeping one leaves this assertion's construction untouched —
  the bullet below is that same distinction reached from the other side.
- **Stages only.** A supervising session's own journal is not a stage journal: it
  has no stamp, so the cursor cannot name it, and a second roster to reach it is
  a surface this assertion did not buy.

**The refusal is evadable, and saying so is the point rather than a caveat on
it.** Its help text names one way forward — write the missing journal yourself,
stating that the predecessor left none — and an oracle over an artifact the
asserted-against session can itself create is bypassable by definition. What the
mechanism buys is not that a journal exists; it is that its **absence becomes
deliberate and written** instead of silent and unnoticed, which is the defect
exactly: the finding this closes was not that a session refused to journal, it
was that nobody knew one had not. A session that writes "the predecessor left
none" has produced the recovery record the channel exists for, at the one moment
someone is looking. The escape is also what keeps the refusal off the deadlock
class: a refusal with no reachable exit would wedge the lifecycle behind a
session that has already ended and cannot be asked to fix anything.

**Default `0`, and the switch is thrown at a boundary rather than mid-iteration.**
`REQUIRE=1` asserts against the *predecessor*, so enabling it inside a running
iteration lands it underneath stages already dispatched before the rule existed,
and puts the first enforced firing inside an iteration rather than at its
boundary — where a refusal costs one stage re-entry instead of a wedge. A rule
whose first live firing needs its own escape has been scheduled badly rather than
designed badly. **`--simulate <stage>` — the read-only preflight mode:**
it runs everything a real entry runs up to the write — config load and stage
validation, header parse, session-id derivation, the idempotence probe (a
would-be no-op is reported as such and exits 0), the candidate-stamp temp
state build, `check-stage-entry`, every matching `LIFECYCLE_KIT_ENTRY_PREFLIGHT`
entry and — where one of those refuses — the valve lookup, which names the line
a real entry would consume and the reason it carries, leaves the ledger
byte-identical and holds the mode at exit 0 because the real entry would proceed;
then the predecessor-journal assertion, and — at an iteration boundary — the
Lessons check, the gap-inbox check, the linked-worktree check and every
`LIFECYCLE_KIT_BOUNDARY_REQUIRE` member, in the order a real entry runs
them; then it stops: no stamp, no boundary truncation, the temp files removed.
That roster is stated in full because a roster that stops short of the refusals
the code runs is read as a guarantee the mode does not give. Every output
line is prefixed `enter-stage (simulate):` so a transcript can never read as
a stamp. Exit 0 = the real entry would proceed (or no-op); exit 1 = it would
refuse, with the refusing check's output relayed line-by-line; exit 2 =
usage/config error, as a real entry.

**Every refusal's recovery is relayed too, and that is the mode's contract
rather than a courtesy.** A refusal's `help:` line is its actionable half — the
real refusal's own design rests on that (§The committed gap inbox) — so each one
prints under `--simulate` as well, prefixed like every other simulate line. The
mode's designed consumer is the lead (§templates/lead.md), which gates an
expensive dispatch on it rather than hand-deriving prior-stage completeness;
relaying a verdict while withholding the one line that resolves it hands that
consumer the refusal and keeps the recovery, which is measured rather than
supposed — a lead reading a simulated boundary refusal escalated a question the
withheld line answers verbatim. The **gap-inbox check's post-close disposition
reports the branch it would take**: that the entry would proceed, naming how
many bullets it would carry into the entering stage's intake, since "would
refuse" and "would proceed carrying work" are different answers to the question
the lead is asking. Not a gate — exercised in `smoke/`
beside the existing enter-stage coverage (would-pass, would-refuse,
would-no-op, nothing written).

**`--rename <name>` — the mechanized iteration rename.** Naming an iteration is
a **two-surface** write — the queue header's placeholder and column 1 of the
first stage's stamp — which `check-stage-evidence` requires to agree, so it rides
this writer rather than a second tool. `[--simulate] --rename <name>` sets the
header to `## Iteration: <name>` and rewrites column 1 of **every** data line in
the state file. Every line rather than only the last is correct by the boundary
invariant, not by convenience: the first-stage entry truncates the state file, so
every line below the separator belongs to the current iteration by construction —
which is exactly what `check-stage-evidence` asserts — and the whole-file rewrite
therefore also heals a half-landed hand-rename. **It is not stage motion**: no
stamp is appended and no stage token is written, so the cursor is untouched and
"stage motion never writes the queue" is unweakened. The precedent for this tool
writing the queue header at all is the boundary reset, which already does.

Before writing, the mode asserts its **columns-2-to-last witness** — fields 2
through `NF` of every data line (stage, session id, date, head, and any field a
later grammar appends) identical before and after. It reads to the end of the
line rather than to a pinned column because a field riding *outside* the witness
could be dropped or corrupted by the rename with neither the tool nor its test
noticing, which is exactly what the four-field spelling did to `<head>` the
moment that field landed; an explicit column-5 check was refused because it
re-hardcodes the arity the gap came from. This
is the content predicate applied by the **writer**, where it is cheap and exact,
rather than by the `Write`/`Edit` guard: a `PreToolUse` hook sees only *proposed*
content, so proving "no stage token moved" there means reconstructing the
pre-edit file and diffing field-wise inside a hook — the same computation with
less information and no way to refuse cleanly. **Pre-flight, the same contract as
the stamp path:** the candidate header and candidate state file are built as
temporaries, `check-stage-evidence` runs against them, and a non-zero exit
refuses with the gate's output relayed and nothing written. That gate is
resolved through gate-sdk's `gate_command` rather than named by script path, so
the arm names a gate and never a substrate: the resolved argv is prefix-shaped,
so the two positionals ride it unchanged, and an argv the bridge refused to build
is exit 2 — the dispatcher's own verdict — never a rename pre-flighted by a check
that did not run. The built-in `check-stage-entry` pre-flight above resolves the
same way, so both arms of this tool name a gate and neither names a substrate.
**Refusals** (exit 2,
nothing written): `<name>` empty; `<name>` equal to the unnamed placeholder,
which only the boundary reset may write — checked ahead of the grammar that would
also reject it, so the message names the owning writer instead of reporting a
malformed name; and `<name>` outside the queue slug grammar `[a-z0-9][a-z0-9-]*`,
where whitespace is the corrupting case, since column 1 is whitespace-delimited
and a two-word name silently shifts every field of every stamp.
**Idempotent** in the stamp path's sense: a header and every column 1 already
reading `<name>` reports and exits 0 without writing. `--simulate --rename`
relays what would change, prefixed `enter-stage (simulate):`, and writes nothing.
The report names both written files and says to commit them together, which makes
the one-commit coupling a property of the writer instead of a line of prose in
the calling stage template.

One reader is **invalidated** by a rename rather than served by it, and it is the
reason the placeholder refusal exists: `LIFECYCLE_KIT_BOUNDARY_REQUIRE`'s check
matches the closing iteration's name against the first token of a disposition
line and reds on finding *none*, so a rename landing after the close stage has
stamped its disposition reds the next boundary. Renaming an iteration that has
already been dispositioned is a rename to redo at the disposition surface too;
un-naming one is refused outright.

**Ruled out: a separate `rename-iteration.sh` tool.** It would add a second
sanctioned writer of the state file, and one-writer is the property the
`Write`/`Edit` guard's own block message asserts and the whole reason that guard
exists. Advisory tooling like `--simulate`, so no fixture pair is owed; the
hermetic cases — both surfaces rewritten, fields 2-4 proved unchanged, the
half-landed heal, each refusal, the idempotent no-op, and `--simulate --rename`
writing nothing — live in `gate-tests/`, because a rename cannot be exercised
against a live checkout's own queue the way `smoke/` exercises the stamp path.

**Idempotent:** if the
state file already ends with a stamp for the same `<iteration> <stage> <id>`,
it reports and exits 0 without appending, so a crashed-and-resumed session
re-runs its entry step safely. It reads the `lib/stages.sh` knobs
(`LIFECYCLE_KIT_QUEUE_FILE`, `LIFECYCLE_KIT_STATE_FILE`, `LIFECYCLE_KIT_STAGES`,
`LIFECYCLE_KIT_FIRST_STAGE`, `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`,
`LIFECYCLE_KIT_BOUNDARY_PRESERVE`,
`LIFECYCLE_KIT_BOUNDARY_REQUIRE`, `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`,
`LIFECYCLE_KIT_SURVEY_RECORD_FILE`, `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE`,
`LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN`, `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE`,
`LIFECYCLE_KIT_ENTRY_PREFLIGHT`, and `LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE`).

**This tool depends on `gate-sdk/lib/gate.sh`, and the dependency is stated
rather than absorbed.** It is sourced at load — from `GATE_SDK_ROOT`, defaulting
to the sibling `../gate-sdk` — not inside the one arm that dispatches a gate,
because a missing library that surfaces halfway through a rename is worse than
one that surfaces before the tool does anything. The dependency is precedented
inside the kit, where every `checks/` member sources exactly that library, but it
is new in `bin/`: a tree that vendored lifecycle-kit without gate-sdk cannot run
the stamp writer at all. The alternative — a second
dispatch resolver written into `bin/` — is the duplicate the shared substrate
exists to remove, so the dependency is taken. Advisory tooling,
not a gate: no fixture pair is owed; it is exercised end-to-end in
`smoke/install.sh` — including the boundary require-check scenarios (a member
naming the closing iteration passes; a member missing the line, a member absent
from disk, and a never-named closing iteration each take their branch).

### bin/install-lifecycle.sh

`bash gate-sdk/bin/run-gates.sh --install-lifecycle [agent-file]` writes the
resident registration block
into the always-loaded agent file (`LIFECYCLE_KIT_AGENT_FILE`, default
`CLAUDE.md`; the positional override points a smoke or fixture at a scratch
tree without touching consumer config), idempotently. **The `###
bin/install-lifecycle.sh` heading is the section name, not a file name** — the
in-SPEC citations resolve against it, and the shell tool it was named after
ported on 2026-09-03. The block is bounded by
fixed marker lines (`<!-- lifecycle-kit:begin -->` … `<!-- lifecycle-kit:end -->`);
a run replaces the content between the markers when present and appends the
block when absent, so re-running never duplicates. A begin marker without its
end is a malformed target (exit 2, rather than guess the bounds); the agent
file must already exist — the arm edits an always-loaded file, it does
not mint one — so a missing target is exit 2. The marker insert/replace itself
is not this member's code: it rides `crate::marker`'s installer writer, the
compiled half of gate-sdk's `lib/inject.sh` (gate-sdk/SPEC.md §lib/inject.sh),
so no second replace path exists to drift.

**It is a bridged `Arm::Run` member** (gate-sdk/SPEC.md §The non-gate arm),
reached by its own `bin/run-gates.sh` front-end branch rather than through the
`--emit <name>` composer, because its contract is an **action with an exit
status** — it mutates two files and writes one git config key, printing narration
on stdout — and `Arm::Emit` collapses every error to 2. **The obvious alternative
is an op of the `--install <op>` family, and it is refused on that family's own
stated terms**: installer/README.md §The install boundary rules that arm
deliberately unbridged, reading no kit config and no knob, because its caller is
the bootstrap and may not be assumed to be a POSIX shell. This member's whole job
is to render blocks derived from **resolved kit config**, so an unbridged op
would have to take all eight knobs on argv from a caller with no way to resolve
them. Recorded as refused rather than unconsidered, because the name collision
makes it the first place a reader looks. Its declared roster is the union of what
the two renderers read, taken from the gates that already declare those knobs
rather than re-derived: `LIFECYCLE_KIT_AGENT_FILE`, `LIFECYCLE_KIT_STAGES` and
`LIFECYCLE_KIT_QUEUE_FILE` from `check-lifecycle-registration`, and
`LIFECYCLE_KIT_STATE_FILE`, `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`,
`LIFECYCLE_KIT_SURVEY_RECORD_FILE`, `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` and
`LIFECYCLE_KIT_GAP_INBOX_FILE` from `check-merge-attrs` — **eight**. A hardcoded
top-level flag would resolve platform defaults and silently ignore every consumer
override, which is not a calibration between two workable shapes but the
difference between working and appearing to.

**The `[agent-file]` positional ports unchanged, and the test that decides it was
run rather than assumed.** §The non-gate arm's distinguishing test makes an
argument *unportable* when it redirects something the front-end has already
resolved from the tree's own config before the exec — it arrives a process too
late and would silently change nothing. This positional is the other kind: it is
not a selector for where configuration comes from, it **is the file the rule
writes into**, read from the arm's own argv and overriding the bridged default.
The near miss is one line away and is an env var rather than a positional:
`smoke/install.sh` runs the arm under `LIFECYCLE_KIT_CONFIG_FILE=lock-stages.sh`,
a genuine config-file selector — and it keeps working, because `gate_knob_env`
resolves the eight knobs by sourcing `lib/stages.sh` in a subshell that inherits
the caller's environment, so the redirection happens *inside* the resolution
rather than arriving after it.

The block is pointer-only, its roster derived: `crate::stages::registration_block`
renders the one line that the repo runs the
state machine on `LIFECYCLE_KIT_QUEUE_FILE`, the stage roster as skill
invocations (`/<stage>` for each `LIFECYCLE_KIT_STAGES` member), and the
markdown link to the kit SPEC — never stage prose, and never a hand-listed
roster, so a consumer's reshaped stage set flows into the block by
construction. The arm and `check-lifecycle-registration` share that one
renderer, so the emitted block and the block the gate certifies cannot
diverge.

The same run performs two further steps for the multi-operator merge surface
(§Multi-operator semantics). **The merge-attribute step** injects a
marker-bounded block (`# lifecycle-kit:merge:begin` … `# lifecycle-kit:merge:end`,
the same installer writer again) into `.gitattributes` (repo root) rendered from
`crate::stages::merge_attrs_block` — one `merge=iteration-scoped` line per
supersede
member (keep-ours) and one `merge=union` line per union member (the gap inbox,
git-native) — so a reshaped supersede or union set flows into the attribute lines
by construction and `check-merge-attrs` certifies the same rendering. **The
writer/asserter split survives the port and its two implementations collapse to
one**: before 2026-09-03 the writer was shell and the asserter was crate, deriving
the same lines through two implementations held together by nothing but
`smoke/install.sh`; they now compose the same two set derivations in one
substrate. Unlike the agent file, the arm legitimately **mints
`.gitattributes` when absent** (it is not an always-loaded file the consumer
authored) — two adjacent file writes with opposite absent-file dispositions, which
is precisely the shape a port unifies by accident. **The driver-config step**
registers the keep-ours driver — `git
config merge.iteration-scoped.driver true` — per-clone (the `install-hooks.sh`
opt-in class); a non-repo cwd degrades to a printed skip, never a hard failure,
leaving the `.gitattributes` attribute inert until a clone installs the driver.
The skip goes to **stderr** and the two action lines to **stdout**, and that split
is load-bearing: the arm is machine-drivable and a finding on stdout is a finding
in a caller's data stream.
The union attribute needs no such step — `merge=union` is git-native, so its
line is live the moment `.gitattributes` carries it.

**The entry point requires a repository, and that removed a silent mis-write
rather than narrowing a graceful degradation.** `bash gate-sdk/bin/run-gates.sh`
cds to `git rev-parse --show-toplevel` and refuses outside a repository
(gate-sdk/SPEC.md §run-gates: *every entry point cds to the toplevel before
resolving paths*), so a non-repo cwd now exits 2 with nothing written. Read that
as a repair, not a loss: both install targets are **repo-root-relative by this
kit's own config**, so the shell tool run outside a repository did not degrade
gracefully — it wrote two files into whatever directory it happened to be in and
exited 0. The soft-skip property above is the **driver-config step's**, and it is
intact: when the arm runs, a cwd whose repository has no driver registered still
prints its skip to stderr at exit 0. Stated in these terms deliberately, because
"a narrowing we accepted" invites a later session to try to restore a behaviour
that was wrong (lead-ruled 2026-09-03, own-authority).

Advisory tooling, not a gate: no fixture pair is owed; every step is exercised
end-to-end in `smoke/install.sh`, which is the member's only caller. **Criterion
2's discharge was the `# no-fixture:` road** — the same cases, both substrates,
while both implementations existed — bought once at port time over a fourteen-case
scenario compared on exit status, both output streams and the **bytes of both
written files**: a fresh agent file, a re-run, a staled block, a missing agent
file, a begin marker without its end, a fresh scratch repo, a re-run of that, the
`[agent-file]` positional, and the `LIFECYCLE_KIT_CONFIG_FILE=lock-stages.sh`
case. Three deltas came out of that run and each is recorded rather than smoothed
away: the non-repo entry-point refusal above (two of the three, one case and one
assertion); and the malformed-marker refusal's prefix, which reads
`install-lifecycle: <file>: begin marker present but end marker missing` where the
shell named a helper function that has no compiled counterpart to name — the exit
status is 2 on both sides and only the diagnostic wording moved. The compiled writer's third
divergence, its whole-line marker-presence test (gate-sdk/SPEC.md §lib/inject.sh),
produced **no** delta here and is unreachable in all fourteen cases: it is
recorded at the module that owns it rather than counted against this scenario. **Criterion 5's residual bites at
adoption rather than during use**, which is unusual: a vendored consumer on a host
the artifact roster does not cover cannot install or refresh its registration
block and merge attributes, and that block is what a consumer writes on day one.
It is accepted on the class's stated terms and on one narrowing fact —
`check-lifecycle-registration` and `check-merge-attrs` are themselves compiled, so
a host with no artifact does not run the gates that would demand the block either,
losing the writer and the asserter together rather than being held to a standard
it cannot meet.

### The close-surfaces emit arm

Prints the derived close-surface roster (§The close-surface roster), one row per
surface, tab-separated `<path>	<mode>	<reclaim>	<owner>`, sorted by path. A
field with nothing declared is `-`; an owner-less row is a capture surface source
2 found with no declaration, whose mode reads `(undeclared)`. The mode is echoed
verbatim — a malformed one is passed through for `check-close-surfaces` to rule
on, so the derivation never silently repairs what the gate exists to catch. An
empty roster prints nothing and succeeds: a resolved-empty derivation is an
answer, never an error.

A **non-gate arm** (gate-sdk/SPEC.md §The non-gate arm), invoked as
`bash gate-sdk/bin/run-gates.sh --emit close-surfaces [scan-root]` — the
front-end that resolves the arm's bridged knobs in front of it. Its two callers
are `check-close-surfaces`, which reaches the derivation **in process** rather
than spawning anything, and close's own inbound-triage sweep. Nothing stores the
roster and nothing must: its whole value is that it is recomputed at the moment
close reads it, so a capture surface added yesterday appears today.

The declaration surfaces are the resolved kit roots' `LIFECYCLE_KIT_ROSTER_BASENAME`
files plus every `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS` match (gate-sdk's kit-root
resolution, consumer-first with kit shadowing — the order every kit registry
already uses); duplicates collapse. Follows the affordance contract, with the
`cd` a compiled arm cannot take becoming a **computed base**: the scan-root
argument, else the repo toplevel, with every path globbed, ignored and printed
relative to it. Config-via-env across the bridge; and the three exit-2 causes —
a non-repo base, an unreadable declaration surface, a `git check-ignore` that
could not decide — become the arm's error return, which the front-end and the
in-process caller both surface. Advisory tooling, no fixture pair owed — the
gate below is what blocks, and §The non-gate arm is the second, now-structural
ground for the same verdict: an arm returning a document has no pass and no
fail to fixture.

The sort is by path, with the **whole row** as its tie-break, and that is
observable rather than incidental: one path may be declared on two surfaces, a
duplication the derivation deliberately does not collapse (it collapses
duplicate *surfaces*, not duplicate declarations). A path-only sort would leave
the tie order at the sorter's discretion and churn the gate's error order for no
edit.

### check-close-surfaces

Three assertions, over the derived roster: (A) **no undeclared surface** — every
capture-tier workflow-dir member carries a declaration; (B) **every declaration
carries a mode**, and a `forced=` mode's citation is *well-formed* — a
repo-relative `<path>.md` followed by `§<section>`; (C) **every capture-tier
declaration names a reclaim command**.

Assertion B is shape-only, and resolution is somebody else's job already:
`check-spec-pointer`'s prose pass resolves a `forced=` citation on any manifest
surface (§The close-surface roster). Taking the presence-and-shape half here is
not a preference — canon-kit's heading resolver is defined *inside its own gate*
rather than exported from a library, and the port to the binary substrate did not
change that: it is a private function of the compiled member, so there is still
no resolver a second gate could call. Reaching it would mean either copying
the resolver, which canon-kit's own tiering rule bans, or making lifecycle-kit
depend on canon-kit — the same ownership-cycle argument that rules out the log
merge. The honest arrangement is a pair of gates independently reading one
surface, each asserting what it owns.

The gate reads the roster by **calling the derivation in process**
(§The close-surfaces emit arm),
so a roster it could not derive is fail-closed (exit
2), as is an unreadable declaration surface. The in-process call is the point
rather than an optimization: it makes "the derivation and the gate can never
disagree" structural instead of conventional. It also means the descriptor
acquires a **source coupling** — the gate module and the emit module it reaches
transitively join `couples=` beside the surfaces already named, or the gate
stays registered, green, and never triggered on the edit that broke it.

An optional scan-root argument (passed through to the derivation as its base) is
the fixture
capability: `git check-ignore` never reports a *tracked* path, and a fixture file
must be tracked to survive a clone, so a case dir cannot hold a capture-tier
member. The `good/`+`bad/` pair therefore covers assertion B on a consumer-set
`LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS`, and the bespoke
`gate-tests/check-close-surfaces.test.sh` builds sandbox repos with a real
`.gitignore` for assertions A and C — the `check-exec-bit` precedent. Both move
with the member across a substrate change rather than being re-authored: the
pair is keyed by gate *name*, and the scenario runner dispatches by name through
`gate_run` (§Testing). Tier
`precommit`; the `# graph:` manifest couples the workflow dir, the
declaration surfaces and the two crate modules.

The three assertions carry `assertion A:` / `assertion B:` / `assertion C:`
markers in the implementing member's own source, in whatever comment leader its
substrate spells. That is not decoration: `check-gate-assertions` set-matches
the enumeration above against those markers and reds on an **empty** set, so a
port that drops them leaves the gate looking unasserted.

Calibration and honest limit: the gate asserts the roster is **complete and
moded**, never that close actually *read* a surface. Reading is a session act
with no mechanical residue short of a per-surface disposition stamp, which is the
heavier design this deliberately does not take: the marking converts an invisible
omission into a sanctioned one, and a stamp would convert it into a gated one at
the cost of a stamp-per-surface-per-iteration ritual. The lighter disposition
comes first because an advisory surface's skip is often correct, and gating a
correct action is the failure mode the gap-inbox refusal already demonstrated.
The un-gateable half rides the Enforcement-first carve-out's cadence — a class on
the consumer's close-stage audit roster (doctrine-kit/DOCTRINE.md
§Methodology-maintenance rules).

### check-lifecycle-registration

Invariant: the configured agent file (`LIFECYCLE_KIT_AGENT_FILE`) carries a
lifecycle-kit marker block whose inner content byte-matches the block
regenerated from the live stage machine (`lifecycle_registration_block`). The
block is derived from the machine, and a reshaped machine — a renamed or
reordered stage, a relocated queue file — or a hand-edit stales it *by
construction*, on the exact path the kit advertises (reshape the config,
re-run the installer): the drift-prone-generated-surface case where a gate is
owed (the enforcement-first weighing). The freshness posture is
`check-doctrine-registration`'s, byte-strict like `check-docs-mirror-fresh`.

A missing block is a finding with the install remedy; a block present but out
of lockstep is a finding printing the diff and the regenerate remedy (both
exit 1). The stale-block report prints **every** hunk: the format and the
uncapped contract are the crate renderer's, whose home is
gate-sdk/SPEC.md §The diff renderer since it stopped being this gate's private
mechanism, and this gate is its one live consumer. Resolution fails closed: a missing agent file, a begin marker without
its end, or an errored marker capture is exit 2 — a half-written or unreadable
target must not pass as clean. The gate satisfies the four gate-sdk contracts
(gate-sdk/SPEC.md §The gate model): the single `LIFECYCLE-REGISTRATION: clean`
line and a `help:` remedy on each finding path (output); exit 2 on an
unreadable target (fail-closed); a `good/`+`bad/` fixture pair under
`gate-tests/` (byte-lockstep-clean and stale-block) plus a sibling `*.test.sh`
for the block-absent, unpaired-marker, and agent-missing cases the one-pair
harness cannot hold (fixture-pair); and registration in this repo's
`gates.list` where its own always-loaded file is the scan target (self-lint).
Positional form `check-lifecycle-registration.sh [agent-file]` points the
fixtures at a synthetic agent file. Its `# graph:` manifest couples the agent
file and `lib/stages.sh` — the config that feeds the block — so an edit to
either re-fires the gate.

### check-stage-evidence

Invariant: the evidence file's stamps are well-formed and every one of them
belongs to the header's iteration. Each stage skill appends a stamp as its
first step, and since the last stamp *is* the current stage, "the current
stage has a matching stamp" holds by construction — so the gate does not
assert it, because a tautology is not an invariant. What it asserts is the
**name-axis agreement** between the two surfaces, carried by the staleness
assertion: every data line's iteration must be the header's. That assertion is
what forces the first stage to rewrite its `—` bootstrap stamp once it names
the iteration, and it is the sole remaining enforcer of header↔stamp
agreement, not a bystander to it.

The gate's one no-cursor ruling: a state file that exists but carries **no
stamp** is a red, with the same shape as the missing-file message. The
window is legitimate *inside* `enter-stage.sh`'s boundary reset, which stamps
in the same motion; by commit time an unstamped file means no stage was ever
invoked, which is precisely what this gate exists to reject — and with the
stage axis off the header, nothing else would have caught it (an empty file
gives the grammar and staleness passes nothing to reject).

The stamp file is additionally kept provably bounded: every data line
must be grammatically well-formed — exactly five fields, stage ∈ the
configured stage set plus the waiver token (a stamp token but never a header
stage), date `YYYY-MM-DD`, and `<head>` either the literal `none` or a
lowercase hex abbreviation of 7 to 40 characters (§The state machine owns the
field); every data line's iteration must be the current
one, stale lines from a prior iteration are rejected; and the `—`
unnamed-iteration sentinel may appear only while the header itself is unnamed.
It also reads the `<session-id>` field (which it once ignored) for one
cross-stage invariant, active at the default `stage` posture of
`LIFECYCLE_KIT_SESSION_BOUNDARY` (§Layout and configuration): within the
current iteration, two *different* stages may not share one session id — a
stage entry is a context boundary and demands a fresh session, so a duplicate
(e.g. build == validate) is a self-reported skip and fails. The rule constrains
*cross-stage* sharing only: **same-stage re-entries are in-contract** — a
multi-session build, or a lead's N sibling batch sessions of one stage
(§The state machine), may share or rotate ids freely, a sanctioned pattern
rather than merely unpunished. This owner-doc statement is the home of the rule
the gate's own distinctness message echoes. Waiver-token stamps are exempt
(never a stage, so never in the map). At the
`iteration` posture the gate skips only this distinctness map; stamp grammar,
staleness, and sentinel scoping hold
identically, and a reused session id remains on the audit trail — it just
stops failing the gate.

Calibration: the `—` sentinel is the bootstrap name for a new iteration
before the first stage names it. Any stage past the first carrying `—` in the
header is rejected (the unnamed-iteration guard, which reads the name axis from
the header and the stage axis from the cursor — it works only because the two
axes stay independently sourced); admitting `—` at every stage — an
attested bug — would let an unnamed iteration reach the final stage
undetected, so the allowance is stage-scoped, not global. The data section
begins after the first `---` separator; prose above it is not validated
line-by-line. Argument mode `$1 $2` (queue, state) with configured defaults
makes the gate fixture-capable; the sentinel-scoping interplay that exceeds
one good/bad pair is covered by `gate-tests/check-stage-evidence.test.sh`, which
is also where the two assertions below are exercised — each needs a real git
history, which no static fixture pair can carry.

**The stamp-provenance assertion.** A stamp proves the stage skill was invoked;
this assertion is what makes it prove the invocation came **first**. For every
**newly introduced** stamp, inside a git work tree: **`<head>` must name
`HEAD`** — the recorded abbreviation, at least seven characters, must be a
prefix of the full `HEAD` sha, so a consumer's `core.abbrev` never enters the
contract.

A stamp is **newly introduced** when no data line in `HEAD`'s version of the
state file carries the same `<session-id> <head>` pair. Identity is that pair
rather than the whole line because `bin/enter-stage.sh --rename` rewrites
column 1 of every data line and must not read as re-introducing all of them.
**The migration clause** applies that same reasoning to the one other bulk
rewrite this format has: a `HEAD`-version data line carrying only **four**
fields matches any working-tree line with the same `<session-id>`, so the
one-time five-field migration (§The state machine) is a rewrite and not an
introduction. Without it the migration commit — and every consumer's own
rewrite, which that section names as the recovery — would red against an
assertion no historical stamp can satisfy. The clause is self-retiring:
four-field lines cease to exist once migrated, and the boundary reset truncates
the file within one iteration.

What it catches, without a history walk and without a per-stage surface roster:
a session that stamps first and commits the stamp *after* its work commits has
moved `HEAD` between the write and the commit, so the recorded head is stale and
the gate reds naming both commits. **Same-stage re-entry is in reach by shape
rather than by exemption** — a second session's stamp records its own `HEAD` and
is committed on it, and the assertion never compares two stamps to each other.

**Four inertness conditions, all stated, because an unstated one reads as
coverage.** The assertion does not run when

- **(a)** the state file's directory lies in no git work tree, or in a different
  one from the configured surfaces — a vendored tree under test, a sandbox;
- **(b)** the file handed to the gate is not *this* work tree's own configured
  state file — its repo-root-relative path is not `LIFECYCLE_KIT_STATE_FILE`.
  "Inside a work tree" alone is not the discriminator the amendment behind this
  section assumed it was: a `gate-tests/` fixture tree sits inside the repo like
  everything else, and firing there would assert a recorded head against a repo
  whose history the fixture's stamps were never taken in. That is also what
  keeps this gate's argument mode fixture-capable at no cost to the assertion;
- **(c)** the state file is not tracked in `HEAD`, because there is then no
  prior version to diff against and "newly introduced" is unanswerable rather
  than false — a consumer's first state-file commit;
- **(d)** no stamp is newly introduced, which is every battery run that stamps
  nothing.

On the live file, `none` on a newly introduced stamp is a **red**, and that is
what keeps the inertness from being a disarm. The residual is `git rm --cached`
of the state file, which would restore condition (c): the same class as
`--no-verify`, and closed by neither.

**The stamp-commit purity assertion.** The provenance assertion alone is
defeated by a session that writes the stamp and its work into **one** commit:
`HEAD` has not moved, so the recorded head is current. The complement closes it.
Where the state file is among the staged paths *and* introduces a stamp, the
staged path set must contain **only**:

- the state file — for any stage's stamp; and additionally
- `LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE` — for **any** stage's stamp, because an
  admitting entry rewrites the valve ledger's state token in the same motion as
  the stamp (§bin/enter-stage.sh); and additionally
- the queue file, `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`,
  `LIFECYCLE_KIT_SURVEY_RECORD_FILE`, the members of
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`, and `LIFECYCLE_KIT_GAP_INBOX_FILE` — for
  the **first stage**'s stamp only, because the iteration-boundary reset
  legitimately writes all of them in one motion (§bin/enter-stage.sh).

**The set is one predicate, stated as itself: the paths `bin/enter-stage.sh`
writes at *this* entry.** Every member above derives from that and none is
minted. The boundary-reset members are scoped to the first stage because the
boundary reset is the only entry that writes them; the valve ledger is scoped to
none because an admitting entry is not the first stage's and, more to the point,
because the scoping it needs rides **membership** rather than a stage name — a
non-admitting entry leaves the ledger untouched and therefore unstaged, so the
exemption is unreachable except at the one branch that earns it. Widening the
set can only remove violations and narrowing it can only add them, so a config
change here is safe to reason about by inspection in both directions. Gating on
the *staged* set rather than on the working tree is what keeps a sibling
session's staged file out of this verdict: a stamp nothing is committing is not
a stamp this commit introduces. It mechanizes a sentence every stage template
already carries — *commit the stamp on its own* — which is enforcement-first
applied to prose that had no gate behind it, and the one exception that sentence
now carries (an admitted entry commits the valve ledger with the stamp) is this
set's second bullet rather than a weakening of the rule.

**The concurrent-session false fire is real and its remedy is cheap.** Where a
repo shares its git index between sessions, a sibling committing between a
stamp's write and its commit moves `HEAD` and reds the stamp. The remedy is to
re-run `bin/enter-stage.sh`, which appends a fresh stamp at the current `HEAD` —
a same-stage re-entry, in-contract, and cheaper than any weakening that would
admit the case the assertion exists to catch. The tool's idempotence guard reads
the head for exactly this reason (§The state machine).

Honest limit: the stamp proves the stage skill was *invoked*, never that it
produced its green result — a validate stamp says validate ran, not that the
suites passed. That gap is closed by evidence-kit, which commits a per-run
evidence manifest (a suite verdict per line) and, via the optional
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` integration, couples a close entry to
the full green block.

**The one ordering case neither assertion above reaches**, and it is structural
rather than a gap in their design: a session that does its work, commits it, and
only *then* runs `bin/enter-stage.sh`. The stamp legitimately records the
post-work `HEAD`, the stamp commit is pure, both assertions pass — and the work
still preceded the mark. The interval between stage X-1's stamp and stage X's
holds X-1's legitimate work and X's illegitimate pre-stamp work, and nothing in
the tree separates them. Only a *declared* boundary can, and there are two
candidates: a per-stage output-surface roster, refused because "a stage's own
output surfaces" is not derivable from the tree — a build stage's are
approximately everything — and a configured one false-fires when coarse and
misses when narrow; or an **exit mark**, which this kit does not have and which
no reading of an entry stamp can supply. Closing the remaining case is therefore
a **composition** with a deliverable that mints an exit mark, not more work
inside these assertions. Recording that is what stops the next reader
re-deriving the refused roster.

What the assertions do close is the generalization that *every entry-time
assertion is satisfiable retroactively*, which fails for this one: the recorded
head is bound to a commit that already existed when the stamp was written, and a
stamp introduced later is checked against the `HEAD` of its own introduction, so
re-satisfying it means re-stamping — which is a fresh, honest stamp rather than
a forged one.

**The uncommitted window, and what closes it.** This gate — and every other one
that would catch a hand-written stamp — fires at *commit*. The cursor, though, is
the working tree's last stamp, and every reader reads the working tree. So a
session that hand-edits the state file and simply never commits moves the stage
cursor for its entire life and is caught by nothing. Content-based detection
narrows this and does not close it: the `<head>` field makes a hand-written stamp
name the `HEAD` it claims to have been taken at, so it is not byte-identical to
`bin/enter-stage.sh`'s for free — but a hand that reads `git rev-parse --short
HEAD` writes an identical line, and a stamp that is never committed is never
checked at all.

The `workflow-state-guard` harness-integration arm
(`bash gate-sdk/bin/run-gates.sh --hook workflow-state-guard`) closes the part of
that window an agent tooling actually passes through. It is a
`PreToolUse(Write|Edit)` hook (register it on the alternation matcher; guard-kit's
wiring template carries the block) that refuses a write whose target **resolves**
to the state file, naming `bin/enter-stage.sh` as the sanctioned writer. Resolved,
not textual: an absolute path, a `./` prefix and a route through a symlinked
directory are one file, and a textual comparison catches only the spelling it was
written against. It resolves the path and advises or blocks through the same
channels every harness-integration arm uses (gate-sdk/SPEC.md §The non-gate arm),
sourcing no kit library of its own. The `GATE_SDK_WORKFLOW_DIR` indirection is
honored, so a consumer who relocated the workflow directory gets a guard that
follows it. No knob is added: there is no value to configure.

The residual is stated because an unstated residual reads as a closed hole. The
guard reaches agent `Write`/`Edit` tool calls and nothing else — not a `Bash`
redirect, not `--no-verify` at the commit that would have caught the result, not a
human editing the file outside the agent tooling, and not any future writer that
is not one of those tool calls. It narrows the window to the writers the harness
routes through a hook; it does not close it. And it is inert until wired: a
consumer who omits the settings-json block from guard-kit's wiring template gets a
hook that enforces nothing while looking like it does, because the arm only ever
runs when the harness's `PreToolUse(Write|Edit)` hook invokes it.

### check-stage-entry

Invariant: the stage being *entered* re-verifies its prior stage's static
exit, extending the invocation-stamp floor `check-stage-evidence` provides
for the current stage one hop back (a shared surface, a distinct invariant:
*stamp grammar + name-axis agreement* there, *prior-stage invoked +
entered-stage static exit* here). It reads the entered stage from the **cursor**
— the state file's last stamp — and the iteration from the header, both through
the shared `lifecycle_current_stage`/`header_iter` derivations, which take
positional fields one to three. That is why the `<head>` field appended in
§The state machine moved nothing here, and it is recorded rather than left
unstated so the next grammar change knows this reader was checked and cleared
rather than overlooked. An empty
cursor is unreachable by construction here and stays a hard parse error rather
than a disarm: `enter-stage.sh` hands the gate a temp state file that always
carries the candidate stamp, and at commit time the entry commit stages that
same stamp. It owns three assertions, (A)
prerequisite-stamp ordering — for an entered stage X the file carries a
stamp for X's configured mandatory predecessor, which closes the "jumped
straight to the last stage with no prior stamp" hole its sibling — which no
longer asserts any stage coverage at all — cannot see; (B) drain-entry
queue-empty —
a drain-stage entry requires the configured active queue sections to carry
no top-level `- ` entry, catching entry-on-incomplete-build, with one modeled
residue class: an entry whose **lead line** carries `[drain-exempt: <reason>]`
(syntax: queue-kit/SPEC.md §The tag algebra) is skipped at drain-stage entry —
a drain-spanning feature whose remaining half *is* drain-stage work. The
reason must be non-empty (empty is malformed, red); it is echoed in the
refusal/clean detail, the audit trail its semantic reader. The **backstop**:
at entry to every drain *successor* — each stage whose
`LIFECYCLE_KIT_PREDECESSOR` value is the drain stage; the map is many-to-one,
so every match backstops and a tagged entry drains by whichever successor is
entered first — assertion B runs with **no** exemption: nothing may remain
active, tagged or not. So untagged entries drain by drain-stage entry, tagged
entries by successor entry. A roster whose drain stage is terminal (zero
successors) is refused fail-closed at config load (§lib/stages.sh's
validator): an exemption with no reachable backstop would be permanent.
Ruled-but-unpromoted work is never exempt residue — it files as Deferred
`[design-pending]` for a later scope's promotion (deferred-filing is the model
for designed-but-unscheduled work); and (C)
audit-trigger — an audit-entry-stage header carrying a cross-component
amendment signal but no `<iter> <audit-stage>` stamp demands either that
stamp or an explicit recorded waiver line, mechanizing the audit stage's
self-reported cross-component trigger so a missed trigger cannot silently
skip the audit. The signal reads the on-disk amendment tree (cwd-relative,
gate-sdk prune set applied, and `templates/` paths excluded — a shipped
`SPEC-amendment.md` skeleton is a copyable stub, not a live amendment, the
same exclusion canon-kit's finders apply): it fires when amendment files span
≥2 component dirs, OR when a single amendment's component set — its own dir ∪
the contract-surface tokens in its body that resolve to a roster dir — is ≥2.
A **roster dir** is a directory holding `LIFECYCLE_KIT_ROSTER_BASENAME`, and
the roster test screens the *body tokens* alone: an amendment file's own
directory is a component wherever it sits, so a second component arrives
either from a second amendment file anywhere, or from a body token that
resolves to a roster dir.
The
waiver rides the same file the stamps do (auditable) and is written only on
an explicit user ruling — never self-issued by the entering session; it
satisfies only assertion C (assertion A's predecessor scan matches the audit
stage exactly, so a waiver is never read as an audit *stamp*).

Calibration: the predecessor map deliberately omits a **trigger-gated stage**
as anyone's mandatory predecessor — the audit stage, and equally a trigger-gated
*authoring* stage where a roster splits one out (§templates/stages/) — because
demanding its stamp before a successor would false-fire on an iteration that
legitimately skipped it (an amendment-free iteration runs no audit; a debt-only
one runs no authoring stage); the build→align re-check when align *did* run is
the build skill's step-0 procedural precondition, not this gate. A trigger-gated
authoring stage takes **no assertion-C sibling**, a mechanized trigger considered
and deferred: its trigger is procedural (the prior stage's next-stage
recommendation) and already backstopped by canon-kit's bidirectional
amendment-pairing rule — a feature entry carries a `[spec:]` ref only when the
amendment exists on disk, so a skipped authoring stage cannot ship a feature
without its amendment. The disanalogy with assertion C is decisive: C mechanizes
because the *audit* it gates is otherwise-unverifiable judgment, whereas an
authoring stage's *output* — the amendment — is otherwise-verified by that
bidirectional rule, so a process assertion there would only duplicate the
on-disk-amendment signal or smear canon-kit's feature-section grammar into this
gate. Assertion C's honest limit: it
approximates "changes ≥2 components' *contracts*" with "touches or names ≥2
component surfaces" — it can over-demand (the cheap waiver valve absorbs
that) and can under-detect a purely semantic cross-component impact; it
converts a silent skip into a stamp, a recorded waiver, or a narrow
false-negative, strictly better than self-report.

**The gate dispatches to the binary substrate** — `checks/check-stage-entry.gate`
to `native/src/gates/stage_entry.rs`, the shell script deleted — and the port
asserts nothing new: the three assertions, their calibration and assertion C's
honest limit are exactly as stated above. The port was held on the config
bridge's want of a key channel, since the predecessor map is read **by key**
(gate-sdk/SPEC.md §lib/gate.sh, the keyed arm that retired the hold). Two
consequences are worth stating where a reader of this gate will look for them.
Its `couples=` **widens** to reach the amendment and roster corpora assertion C
scans, which is correct coupling rather than a concession: an amendment landing
anywhere genuinely changes what C sees, and the shell form's
`# reads-couples-exempt:` markers were excusing that rather than expressing it.
And its two whole-tree scans are declared to `--reads` as one root under two
**filter-knob** names, `LIFECYCLE_KIT_ROSTER_BASENAME` and
`LIFECYCLE_KIT_AMENDMENT_GLOB` — the channel gate-sdk/SPEC.md §check-reads-couples
grew for it, so the port **answers** that assertion where the shell member was
exempt from it. **Honest limit on the crate-side declaration:** neither fixture
case reaches assertion C, so the crate's declared-roots unit test holds over an
empty observation for this member and the scans' coverage rests on the
behavioral test below and on the live battery.

The good/bad pair covers
assertion A; `gate-tests/check-stage-entry.test.sh` covers B and C over nine
sandbox scenarios (untagged residue red, tagged residue at drain entry green,
empty-reason tag red, tagged residue at successor entry red; two-dir
amendments ±waiver, a single-amendment cross-component body, a
single-component amendment, and a `templates/` stub that must not fabricate a
second component). Suite *runs* and other
non-static exits are not re-runnable as pre-commit gates and stay
human-judged at the stage approval; the prerequisite-stamp floor is their
mechanical residual.

### check-stage-skill-coverage

Invariant: the configured stage set and the skills dir (`LIFECYCLE_KIT_SKILLS_DIR`,
default `.claude/commands`; override with the first argument) cover each other,
both directions. Forward: every `LIFECYCLE_KIT_STAGES` member has a `<stage>.md`
skill file — a stage with no skill cannot be entered. Reverse: every skill file
that invokes `enter-stage.sh` names a live stage in the token it passes. The
`enter-stage.sh` invocation is the mechanical marker separating a stage skill
from an ordinary one, so a retired stage's orphan skill (its `.md` still
invoking a now-unknown stage) reddens without false-flagging a non-stage skill
like `/agent-execution`, which never invokes `enter-stage.sh`. A skills dir that
does not exist is fail-closed (exit 2). The `# graph:` couples the skills dir at
`tier=precommit`; the whole-tree `run-gates.sh` battery backstops a stage-set
edit (`lifecycle-config.sh`), which is not itself in the coupled surface.

### check-skill-binding

Invariant: every skill under `LIFECYCLE_KIT_SKILLS_DIR` (default `.claude/commands`;
override with the first argument) that carries a binding directive — `Execute
the template at <path>, applying the bindings below.` — (a) names a template
file that exists and (b) binds exactly that template's slot set: an unbound slot
is red, an orphan binding naming no slot is red. A skill with no directive is
not read, so a copy-and-specialize skill carrying no such line is untouched —
the same directive-as-selector mechanism `check-stage-skill-coverage` uses on
`enter-stage.sh`. A bound skill need not be a stage skill: `/agent-execution`
binds a delegation-kit template, which the gate accepts unchanged (the resolved
template path may point at any kit). Template slots are the `*<slot-name: …>*` opening
tokens; a shim's bindings are the `**slot-name** —` lead lines under
`## Bindings`; the directive's template path resolves relative to the current
directory (the tree root at pre-commit). A skills dir that does not exist is
fail-closed (exit 2). The `# graph:` couples the skills dir, the stage-template
dir, each boundary skill (`lead.md`, `release-sweep.md`, `upgrade.md`,
`consult.md`), and each out-of-tree bound template (e.g. `delegation-kit/templates/agent-execution.md`)
at `tier=precommit`, so a slot added to a template or a binding changed in a shim
fires the gate. The boundary skills are coupled **by name** because this couple
carries no kit-wide `templates/*.md` glob: a bindable template left off it is one
whose edits fire nothing, and the gate stays green while the coupling is gone.
`upgrade.md` is listed even where no consumer binds it — the entry is about the
template's edits re-triggering the gate, and a consumer that does bind it
inherits the manifest. The good/bad pair drives the unbound-slot case;
`gate-tests/check-skill-binding.test.sh` covers the orphan-binding,
missing-template, and skip (no-directive / no-slots) cases the one pair cannot.

### check-shim-restatement

Invariant: no binding shim under `LIFECYCLE_KIT_SKILLS_DIR` shares a normalized word
n-gram of length ≥ `LIFECYCLE_KIT_SHIM_NGRAM` with any surface in the dedup corpus
`LIFECYCLE_KIT_SHIM_DEDUP_CORPUS` — the duplication tripwire under the same
directive-as-selector rule `check-skill-binding` uses (a file with no `Execute
the template …` directive is not a shim and is not read). The corpus defaults to
the consumer's always-loaded agent file (`LIFECYCLE_KIT_AGENT_FILE`, default
`CLAUDE.md`) plus every kit's `templates/**/*.md`
(kit set from `gate_kit_roots`); an explicit `LIFECYCLE_KIT_SHIM_DEDUP_CORPUS` or
positional corpus arguments override it — the latter is the hermetic-fixture
affordance. Comparison normalizes first — lowercase, punctuation stripped to a
word boundary, whitespace collapsed — so cosmetic rewording does not evade the
tripwire; a resolved corpus that yields no n-grams is fail-closed (exit 2), never
a false clean.

`LIFECYCLE_KIT_SHIM_NGRAM` is calibrated to the smallest window with zero false
positives on the post-rewrite corpus, with a floor of 8 words so a citation line
(a path plus a §heading) never fires — this repo's default is 9, the width at
which the 8-word §heading `This repo is governed by its own kits` stops
tripping. Honest limit: the n-gram holds the *copy shape* only. Which tier a
fact belongs to stays semantic judgment — a paraphrase below N words passes the
gate and is still a defect to fix on sight (the same doctrine as
check-comment-tier's floor). The `# graph:` couples the skills dir, `CLAUDE.md`,
and the kit template dirs at `tier=precommit`, so editing a shim or a corpus
surface fires the gate. Because the corpus find recurses but the `kit:templates/*.md`
couple does not, the one kit template *sub*directory it misses —
`lifecycle-kit/templates/stages/*.md`, the stage-skill templates each stage shim
binds — is coupled explicitly beside it; a shim's own template is its likeliest
collision surface, so it must re-trigger the gate. The boundary skills need no
such entry: they sit at `templates/` root, already inside the plain couple. A red run names the
shim, the corpus surface, and the shared n-gram, so the fix (delete the
restatement, keep a citation) is mechanical. The good/bad pair drives the plain restatement/clean split;
`gate-tests/check-shim-restatement.test.sh` covers the no-directive skip, the
short-corpus fail-closed, and the below-N paraphrase the one pair cannot.

### check-lesson-disposition

Invariant: every `## Lessons Learned` entry present at HEAD and absent from the
worktree leaves a well-formed disposition stamp in
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` — a lesson cannot be cleared without a
recorded rule/task/harvest/discard call. The evidence home is a stamped file,
not the commit body, because the battery runs at pre-commit when no commit
message exists yet, so only a file is mechanically decidable (the
`check-stage-evidence` fail-closed precedent). Each data line is
``<iteration> lesson <rule <file> | task <slug> | harvest <tag> | discard <reason>> — <lead-line prefix>``;
the `` — `` separates the disposition from the
lead-line prefix that joins it to the removed entry (a stored prefix matches a
removed entry when it is a leading substring of that entry's normalized lead
line). Both grammar (each line well-formed, a known disposition kind) and
per-entry matching (every removal has a stamp) hold; an unreadable evidence
surface is fail-closed (exit 2). Shape validation only: `harvest <tag>` is not
checked against `QUEUE_KIT_LESSON_TAGS` — that would cross-couple the kits'
configs; the close skill and `check-tag-lead-line` hold the vocabulary.

Calibration: diffing HEAD against the worktree is fixture-hostile (a committed
fixture has HEAD == worktree, so the removal case has no static representation —
the `check-task-conservation` precedent), so the gate takes optional
`[queue-head] [queue-worktree] [evidence-file]` override args and its good/bad
pair drives all three hermetically (the `check-trajectory-fresh` synthetic-args
precedent); `gate-tests/check-lesson-disposition.test.sh` covers the malformed
grammar, still-present-not-removed, and prefix-join cases the one pair cannot.
The `# graph:` couples the queue file and the evidence file at
`tier=precommit`.

### check-merge-attrs

Invariant, over two derived sets: bidirectional set-parity between the derived
iteration-scoped supersede set (`lifecycle_supersede_set`, §lib/stages.sh) and
the paths carrying `merge=iteration-scoped` in the consumer's `.gitattributes`
(default `.gitattributes`; override with the first argument), plus **forward-only**
parity between the derived union set (`lifecycle_union_set` — the gap inbox) and
the paths carrying `merge=union`. The iteration-scoped forward direction — a
supersede-set path with no `merge=iteration-scoped` line — catches an
unmechanized rule (a merge would silently take the wrong side on that surface).
Its reverse direction is the safety edge: a `merge=iteration-scoped` attribute on
a path *outside* the derived set silently discards merge content on a real
surface, so a smuggled line is red, not config. The gate scans every
`merge=iteration-scoped` line in the file — inside the installer's marker block
or not — so the reverse edge holds against a hand-added line anywhere. The union
set is **forward-only by design** — a union member with no `merge=union` line is
red (a filed gap would be silently dropped at a concurrent merge), but a
`merge=union` line *outside* the derived set is **not** flagged: `merge=union` is
a git-native driver a consumer's own tracked append log legitimately carries
(§Multi-operator semantics), so there is no smuggling to catch. A missing
`.gitattributes` reports every derived surface as unmechanized (exit 1, the
install remedy); an unreadable one, an empty supersede set (a lifecycle always
owns at least its state file and its two kit-owned built-ins, the
lesson-evidence file and the survey record), or an empty union set (it
always owns at least its gap inbox), is fail-closed (exit 2).

The gate satisfies the four gate-sdk contracts (gate-sdk/SPEC.md §The gate model):
the single `MERGE-ATTRS: clean` line and a `help:` remedy on the finding path
(output); exit 2 on an unreadable target or an empty derived set (fail-closed); a
`good/`+`bad/` fixture pair under `gate-tests/` — the good case the default
state+lesson attribution plus the gap-inbox union line, the bad case a smuggled
reverse-edge line — plus `gate-tests/check-merge-attrs.test.sh` for the
iteration-scoped forward-missing and missing-file findings, the union
forward-missing finding, the union no-reverse-edge case (a `merge=union` line
outside the derived set stays clean), and a real two-branch merge in a sandbox
repo that asserts the keep-ours driver resolves an attributed surface to the
arriving side (fixture-pair); and
registration in this repo's `gates.list` where its own `.gitattributes` is the
scan target (self-lint). Its `# graph:` couples `.gitattributes` and
`lib/stages.sh` (the config the supersede set derives from) at `tier=precommit`;
a reshaped `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` in the consumer config is backstopped
by the whole-tree `run-gates.sh` battery (the `check-stage-skill-coverage`
precedent).

### check-survey-record

Invariant: every block in the survey record (§The survey record) carries a whole
witness. Over each `## ` block: all five keys present, in order, one per line
(and no sixth key, no stray line); `corpus` non-empty; `oracle` and `edges`
non-empty, where the literal `none` is legal and is the honest form and an
*empty* value is the silent form and is refused; and `rev` a full 40-hex sha
naming a commit that
exists — the assertion that catches the short-sha and wrong-rev cases, and the
reason the field is machine-stamped rather than author-supplied. **And every
git-object-shaped token in the other four non-`rev` fields names an object that
exists** (§The survey record), unless the block carries the valve.

`edges` is asserted on the same two footings as `oracle` — present, and
non-empty with `none` legal — because it answers the same question in its own
dimension: an absent key and an empty value are both the silent form of "no sum
was taken", and the field exists precisely so a later boundary can tell that
apart from a sum it can read.

**The widened arm reuses the `rev` arm's probe over a wider input, and its two
asymmetries with that arm are deliberate.** The mechanism was already here and
already trusted — `git cat-file -e` — pointed at the one field the attested
fabrication did not use. Its input is every field but `rev`, `edges` included:
a sum whose caveat pastes a sha is a citation like any other, so the arm covers
it by construction rather than by a second arm. First asymmetry: the widened arm
accepts **any object type**, where `rev` demands `^{commit}`. A sha naming a blob
or a tree is a real citation, and demanding a commit outside `rev` would red a
legitimate one. Second: the token shape requires at least one `a`-`f`, where
`rev` takes any 40-hex string. That is false-positive control rather than rigour — a 7-plus
run of bare digits in prose is a count, a compact date or a byte figure far more
often than a sha — and its honest limit is that an all-digit short sha is not
probed at all. `rev` itself is **excluded** from the widened scan: it has the
stricter arm above, and reporting it twice would say one thing in two voices.
The heading is not scanned either; it is a question, not a field.

**The corpus stops at this record, and the stopping point is argued rather than
convenient.** A survey block's fields are short and structured, so a
word-bounded hex run in one is a citation far more often than an accident. The
same arm over the queue, or over every governed prose surface, would meet
ordinary hex-looking English and fixture data — and a gate that cries wolf
trains its readers to bypass it (gate-sdk/SPEC.md §When a gate earns its place).
The wider sweep is filed and costed rather than built here.

**The valve is a markdown comment on a surface `check-comment-tier` does not
scan, and that was probed rather than assumed.** A `survey-token-exempt` line
added to this repo's live record produced no comment-tier violation, so the
spelling is **not** registered on canon-kit's directive roster: a registration
would be dead configuration, and it would couple one kit's gate to another kit's
block grammar. A consumer that widens the comment-tier corpus to include the
record has the universal `comment-tier-exempt:` escape already. What *does* have
to know the spelling is this gate's own parser, which would otherwise read the
valve as the stray line its block grammar forbids — the half the hermetic
fixture pair can assert, since it needs no probe.

An absent record, and a record truncated to its header, are **clean and counted
inert**: the surface is optional, and a consumer that never files a survey must
not carry a red gate.

Enforcement-first is why the gate ships in the same unit as the surface rather
than as follow-up debt: a block missing its witness is *silently unusable*,
which is the exact failure class the surface exists to close.

**Bare drives the configured record with the full assertion set; an explicit
file argument drives it hermetically — grammar only, with no rev-existence
probe.** That split is forced by portability rather than convenience: a fixture's
`rev` names no commit in the tree the fixture is copied into, so a pair asserting
existence would go red in every consumer that vendored the kit. Bare also
degrades to grammar-only outside a git repository, where there is nothing to
resolve a sha against. The clean line names which mode ran, so a grammar-only
pass can never be read as a verified one.

The gate satisfies the four gate-sdk contracts (gate-sdk/SPEC.md §The gate
model): the single `SURVEY-RECORD: clean` line and a `help:` remedy naming the
grammar and the `--emit file-survey` arm on the finding path (output); exit 2 on
an unreadable or explicitly-named-but-missing record and on a failed parse
(fail-closed); a `good/`+`bad/` fixture pair under `gate-tests/` driven through
the hermetic argument — the good case a three-block record including an
`oracle: none` note, a valved block, one block carrying a real per-candidate sum
and two carrying `edges: none`, the bad case a short sha, an empty
oracle, a block with its `oracle` line missing, an **empty** `edges` and a block
whose `edges` line is **missing** — which are different findings with different
remedies, one "write `none`" and one "the grammar grew a field" — and a
reasonless valve — plus
`gate-tests/check-survey-record.test.sh` for the half the pair cannot hold,
which is everything the probe decides: both arms of the rev-existence probe in a
sandbox repo (a rev naming a real commit, and a well-formed 40-hex rev naming
nothing), both arms of the widened token probe, the valve exempting its block,
a reasonless valve that reds *and still probes*, and the two inert shapes
(fixture-pair); and registration in this repo's
`gates.list`, where its scan target is this repo's own record (self-lint). Its
`# graph:` couples the record at `tier=precommit`.

*Not gated, and stated so the gate is not mistaken for more than it is:* whether
a `finding` is **true**, and whether a session holding a fresh record actually
read it. The first is unmechanizable; the second leaves no tracked artifact —
the class delegation-kit/SPEC.md §Operative residency rules no gate is owed for.
The entry report's read trigger (§bin/enter-stage.sh) is the affordance in place
of an oracle there, and it is weaker than one by construction.

### check-scratch-citation

Invariant: no surface in `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS` carries a
**retrieval pointer** to a path in the derived boundary-truncated set. The rule
and why inlining is the right close are §The survey record's; this section owns
the red condition.

**The forbidden-target set is derived, never maintained** — it is
`crate::stages::supersede_set` (§lib/stages.sh states the derivation and its
shell holder), already the single derivation behind the `.gitattributes` block
the `--install-lifecycle` arm writes and `check-merge-attrs`'s parity check.
This gate reads it too, which is why a consumer widening its truncate
configuration gets citation enforcement for free.

**Retrieval-pointer position, stated as the red condition.** A supersede-set path
that is either **(a)** in markdown link-target position — the parenthesized half
of a `[text]`-plus-parentheses pair — or **(b)** preceded
by a colon and whitespace and followed by nothing but a closing quote, comma,
period, bracket or end of paragraph — the attested form, *"Full finding and its
two-command witness"* followed by the path. A bare path elsewhere in prose is a
**mention** and is clean; that is the whole calibration, and it is why this is
specified as a position rather than as a pattern.

**The scan joins a paragraph's wrapped continuation lines before testing (b), or
it is blind on its own worked example.** A queue file wraps prose to a budget, so
a colon and the path it introduces routinely land on different physical lines —
which is exactly what the attested instance did. The join is
`check-spec-pointer`'s prose-extraction shape (canon-kit/SPEC.md
§check-spec-pointer), adopted rather
than re-decided: concatenate a blank-line-delimited paragraph, match over the
join, and map a hit back to its physical line through per-line start offsets. A
scanner that is right on an unwrapped fixture and silently blind on the wrapped
case it was written for is the failure this forecloses, which is why the `bad/`
case carries a wrapped instance beside the single-line and markdown-link forms.

**Per-line escape hatch:** a `scratch-citation-exempt:` tag on the line above the
hit, this repo's established opt-out shape, for a surface that must quote a dead
citation verbatim in order to describe it. Its reader is this gate's scanner, at
the transition where it has matched a pointer and is deciding whether to record a
finding. *Stated so it is not mistaken for more than it is:* the class is real and
permanent — any entry documenting a citation defect must quote one — but the tree
carries no live user today, because the entry that motivated the tag completed and
its prose left the queue with it. The `bad/`+`good/` fixture pair is where the
hatch is exercised.

The gate satisfies the four gate-sdk contracts: the single
`SCRATCH-CITATION: clean` line naming how many surfaces were scanned, and a
`help:` remedy naming the `--emit cite-survey` arm and the exempt tag on the
finding path (output); exit 2 on an empty derived target set and on a failed parse
(fail-closed); a `good/`+`bad/` pair whose good case copies the mention forms live
entries actually use — a path opening a wrapped line after a full stop, a
prepositional mention, a colon-introduced pointer at a **non**-member path, and a
tagged dead citation — because a rule that reds any of those is wrong and the
fixture is what says so (fixture-pair); and registration in this repo's
`gates.list`, where its surfaces are this repo's own queue and spec set
(self-lint).

*Not gated:* whether an inlined finding is faithful to the block it came from.
Nothing tracked relates the two once the record is truncated, and inventing a
provenance field would re-open the per-iteration lifetime the record's design
turns on.

### check-gap-inbox-neutrality

Invariant: the gap inbox (§The committed gap inbox) records observations, not
conclusions. Two assertions over every non-blank line below the `# contract:`
header:

- **A — grammar.** Every line is `- <YYYY-MM-DD> — <prose>` with non-empty
  prose. Nothing gated this before, even though the raw append is an explicitly
  legal fallback, so a malformed bullet was found by the drain reading it an
  iteration later.
- **B — no interposed verdict.** No bullet's prose opens with the retired
  ``recurrence of `<slug>`:`` form. This is the class assertion, and its value is
  that it catches a conclusion reaching the capture surface **by any producer** —
  a stale vendored affordance, a hand append copying an older bullet's shape —
  rather than only the affordance this kit ships. The `help:` line teaches the
  rule rather than the regex: say *why* you believe the bullet re-files an entry,
  in the prose, and let the drain judge.

An absent inbox is **clean, not fail-closed** — never having filed a gap is a
legal state for a fresh consumer, unlike §check-lifecycle-registration's missing
agent file, which is an install that did not finish. Fail-closed applies to the
scanner's own exit status, per gate-sdk's contract.

**Bare drives the configured inbox; an explicit file argument drives it
hermetically**, the §check-survey-record precedent, which is what lets the
fixture pair run against a copied inbox.

**Why the tool-side contract is pinned separately.** This gate reads the surface
and cannot see which producer wrote a line, so it cannot hold the affordance to
its own contract. `gate-tests/file-gap-recurrence.test.sh` is what does that. The
two together are the enforcement: the fixture-runner catches the producer
regressing, the gate catches a verdict arriving by any other route.

The gate satisfies the four gate-sdk contracts (gate-sdk/SPEC.md §The gate
model): the single `GAP-INBOX-NEUTRALITY: clean` line and a `help:` remedy on the
finding path (output); exit 2 on an unreadable or explicitly-named-but-missing
inbox and on a failed parse (fail-closed); a `good/`+`bad/` fixture pair under
`gate-tests/` driven through the hermetic argument — the good case a record whose
prose both re-files and disclaims an entry in words, the bad case a retired
marker beside two broken bullets (fixture-pair); and registration in this repo's
`gates.list`, where its scan target is this repo's own inbox (self-lint). Its
`# graph:` couples the inbox at `tier=precommit`.

*Not gated, and stated so the gate is not mistaken for more than it is:* whether
a bullet's prose states the recurrence claim **at all**, and whether the claim is
**right**. Assertion B removes the one shape that pre-empts the judge; it cannot
compel a filer to answer the advisory, and the judgment itself is the drain's
(§templates/stages/), which is where the amendment deliberately put it.

### templates/stages/

The stage-skill templates (`scope`/`align`/`build`/`validate`/`close`) carry
the generic stage spine — the stamp first step (performed by invoking
`enter-stage.sh <stage>` and stating in one line what it does), each stage's
trigger/ordering rules, and its stage-local doctrine — with **named slots**
where the consumer's rule content goes. The templates are the owned surface:
this section states the contract a consumer skill must satisfy and never
restates what a template carries.

Alongside the default-roster templates the kit ships **`spec.md`**, an optional
**amendment-authoring** stage template — the generative half of design, split
out from `scope` on a roster that carries a dedicated authoring stage (the
ontology: scope bounds the units, the authoring stage authors the amendments,
the audit stage independently verifies them). It is a full stage (it invokes
`enter-stage.sh` and stamps), **trigger-gated exactly like the audit stage**: it
runs only when an iteration promotes a feature to author, it **appends** rather
than resets (only the first stage resets the evidence file), and it takes `scope`
as its predecessor without being named any stage's mandatory predecessor
(§check-stage-entry's trigger-gated-stage calibration). The kit **default roster
does not bind it** — the split is demand-gated and non-breaking; a consumer
activates it through `LIFECYCLE_KIT_STAGES` / `LIFECYCLE_KIT_PREDECESSOR`
(§Layout and configuration). This is the supported roster shape for a
trigger-gated authoring stage, the same class as the trigger-gated audit stage.
The generic authoring how-to `spec.md` single-sources — causal completeness and
canon-kit's bidirectional queue pairing — is the content `scope`'s conditional
authoring step points at, so a default-roster `scope` that still authors reads it
there.

The directory holds the **stage-class** template set, not any one consumer's
roster: it ships six templates while `LIFECYCLE_KIT_STAGES` defaults to five,
because `spec.md` serves the split-authoring roster alone. Nothing derives a
roster from this listing — the glob buys a legible layout, not a derivation.

A consumer skill adopts a template in one of two modes; either way the executed
skill states in one line what the stamp step does and supplies every
slot's content:

- **Consume-by-reference (the default)** — the consumer skill is a thin
  **binding shim** whose body is a single directive line, `Execute the template
  at <repo-relative path>, applying the bindings below.`, followed by a
  `## Bindings` section with exactly one entry per template slot. The template
  stays the executed surface; the shim carries only consumer content, so generic
  doctrine has one owner and never drifts across a copy. This is the documented
  default because it tracks the kit: a re-vendor reaches the template, and
  `check-skill-binding` + `check-shim-restatement` hold the shim to a thin
  reference. This repo dogfoods it (`.claude/commands/*.md`).
- **Copy-and-specialize (the sanctioned fork)** — the template is copied into
  the consumer's skills dir and each slot overwritten in place; self-contained
  and legible, structure copied not imported, so the skill stands alone
  (gate-sdk's check-skeleton shape). It is a fork with its consequence owned:
  you own the ritual prose, an upgrade's re-vendor does not reach it, and the
  shim gates do not cover it. It is kept deliberately — the blessed escape hatch
  that keeps legitimate structural divergence (different stages, a reshaped
  machine) visible and contained, and the harness-agnostic floor the
  upgrade smoke assumes; removing it would drive forks into edits of the
  vendored template, which break Phase-A upgrade determinism with no gate to
  catch them.

A consumer reaching for the fork to express *prose* divergence rather than
structural divergence signals the slot vocabulary is too thin; the fix is
richer slots pulling those cases back under shim protection, not more copying.
No gate or telemetry watches for it — which mode a consumer picks is their tree,
not this one.

**Named slots (template grammar).** Each consumer placeholder is a named slot
`*<slot-name: guidance>*` — `slot-name` matches `[a-z][a-z0-9-]*`, is unique
within its template, and precedes the `:` and the guidance a copy-editor or
shim author replaces. A copy-and-specialize consumer overwrites the whole
`*<…>*` span; a shim binds the slot by name in a `## Bindings` entry
`**slot-name** — <consumer content>` (multi-line content indents under its lead
line), and carries nothing else — doctrine restated from the template in a shim
is the defect the reference mode removes. `check-skill-binding` holds the
shim↔template slot parity.

**Authoring rule (a binding shim binds residue, cites procedure, restates
nothing).** A binding a slot supplies carries only what is local to this
consumer — the residue: which surfaces to sweep, which config knobs, which log
sinks. Procedure and always-loaded fact that a kit template or the consumer's
`CLAUDE.md` already owns are named by a citation (a path plus a §heading), never
copied into the shim: a shim is loaded on every stage invocation, so a
restatement there is a per-session token tax on a fact with an owner, and it
drifts the moment the owner changes. `check-shim-restatement` is the tripwire
for the copy shape; the tier judgment (residue vs owned fact) stays the author's.

**The `validate` template carries the valve's arming step, and the `close`
template carries its disposition step** (§bin/enter-stage.sh owns the valve's
contract; both templates point at it rather than restating it). A validate that
ends on a **deliberately accepted** red — a suite whose failure is understood and
is not a regression from this iteration's diff — arms the valve for the closing
stage and commits the ledger, rather than stopping for an operator round-trip.

**Arming is not a queue write, and that distinction is what makes it validate's
move to take.** A validate may not pre-empt the closing stage by writing the
queue: a mid-iteration queue edit is what the gap inbox exists to prevent. Arming
the valve is an evidence-adjacent record on a surface validate already writes at
this exact point in its ritual, beside the evidence manifest and the baseline
diff. The move the machine lacked was never a queue edit; it was a **hand-off**,
and the ledger is the artifact that carries it. **The reason field is that
hand-off's payload**, so validate writes what close needs — which suite, what the
red is, and why it is accepted rather than fixed — and close reads it as the
input to the task it is about to file.

**A `used` line is a close-stage obligation, not a free pass.** For every `used`
line in the closing iteration, close **files the blocking task and lands the
baseline row that names it, in that session**. That is the deadlock's actual
resolution rather than a courtesy: what makes the valve one-shot in substance and
not merely in mechanism is that the entry it bought is spent making the next
iteration's pre-flight pass without one. A close that enters through the valve
and files nothing has moved the deadlock forward by one iteration at the cost of
a record. The filed task **inlines** the finding rather than pointing at the
ledger, the ledger being truncated at the next boundary — the same close the
retrieval-pointer rule requires on independent grounds (§check-scratch-citation).
**A residual `armed` line is dispositioned in the same step and with the same
weight**: it means a session expected a refusal that never came, or armed the
wrong stage, and either is a fact about the iteration worth one line rather than
a file the boundary quietly truncates. The horizon is the iteration, because the
ledger is truncated at the boundary — so "how many times did we reach for the
valve" has a bounded, committed answer for exactly as long as anyone can act on
it.

The `close` template carries a **release-disposition step**: every close
dispositions the iteration at the release boundary — reading the consumer's
`release-policy` slot and either executing its release procedure or stamping an
explicit no-release line into the consumer-named disposition-evidence file
(`<iteration> release <version|none|deferred:<version>> — <basis>`, the
`check-lesson-disposition` contract shape at the release boundary).

The third value carries a release the criteria **earn** but an operator ruling
holds back, which `none` ("nothing to release") cannot express — a reader forced
to tell the two apart by parsing basis prose has no mechanical signal at all. It
is `deferred:<version>` and not a bare `deferred` because the thing that must
survive is the *earned bump level*; a bare token drops it and the next release
re-derives which floor it inherits. The producer derives `<version>` as the
version the criteria *would have shipped as* had the release not been held: the
bump the note's upgrade-contract sections floor, applied over the newest
already-released note — never the next version the project happens to reach.
Stating that rule is what keeps the field mechanically derivable rather than an
operator's guess, since without a defined scale the discharge comparison has
none. The criteria themselves stay in the basis prose and are not structured
fields: the release note's upgrade-contract sections already own them, and a
structured list here would be a second copy of that surface — the line carries
the *level*, the note owns the *criteria*.

**Outstanding-deferral is derived, never tracked.** A `deferred:<version>` line
is **outstanding** until a later line dispositions a release at or above
`<version>`; that later line **discharges** it. Nothing records discharge
separately — the release actually happening *is* the discharge, so there is no
second state to drift. This is what keeps a consumer's gate over the value
low-false-positive: a deferral cannot linger past the release that consumes it,
and one that genuinely has not been consumed *should* keep firing.

A consumer deriving that outstanding set reads the disposition file as
**history ∪ live** — not replacement, not fallback — the same reader
drift-kit/SPEC.md §The stage-economics meter applies to the stage stamps and
drift-kit's `trajectory` emit arm already ships. The file is typically a `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member,
so a carrying line survives only in committed history and a live-only reader sees
nothing; conversely a history-only reader is blind at exactly the moment that
matters most for a precommit gate — the pre-commit of the very close commit
writing the `deferred:` line, when that line is live and not yet committed.
The live arm covers the uncommitted tail, the history arm covers everything
truncation has taken, and the union costs nothing because a line in both arms is
the same line. Truncation-immunity is a property of the *reader*, and every
reader of a truncated evidence file needs it.

**The kit wires no gate over the value**, consistent with the release-sweep stamp
file: the disposition file is operator evidence riding the release commit, the
kit defines the value and the outstanding/discharged derivation, and a consumer
may gate it. That split is the provenance seam — the *grammar* is generic
lifecycle mechanism, the *bump criteria* being carried are consumer release
policy, so the kit ships no list of criteria. The step runs after the surface-mutating
close steps and before the brevity pass, since the note is itself such a write;
silence is not a disposition. The `release-policy` slot carries the consumer's
procedure and criteria by citation, the disposition-evidence path, and any
boundary-only sub-procedure such as a major-only deprecation sweep; a consumer
with no release process binds a plain `none`-every-iteration line. The
disposition line's mechanical reader is `enter-stage.sh`'s boundary require-check
(§bin/enter-stage.sh, `LIFECYCLE_KIT_BOUNDARY_REQUIRE`) when a consumer wires the
file into that knob.

### templates/release-sweep.md

A **boundary skill**, not a stage — which is why it sits at `templates/` root
beside `lead.md` rather than among the stage templates: it invokes no
`enter-stage.sh` and stamps no state, so `check-stage-skill-coverage` never reads
it (it governs only the configured stage set). It is the deprecation disposition
walk at a major — invoked from
close's release-disposition step when the derived bump is a major — forcing every
marker on the `CANON_KIT_DEPRECATION_MARKERS` roster to a stamped disposition —
decommission, carry-forward, or un-deprecate — the `check-lesson-disposition`
contract shape at a release boundary. canon-kit's `check-deprecation-task` holds
each marker bound to a live task between majors; this sweep forces the standing
inventory to a decision at the boundary the deprecations were promised against.
It carries **named slots** (`inventory-command`, `evidence-gate`), so like the
stage skills it adopts the binding-shim grammar (§templates/stages/) and
`check-skill-binding` holds the slot pairing; the stamp file is operator
evidence riding the release commit — the kit wires no gate over it (a consumer
may, through `evidence-gate`).

### templates/upgrade.md

`upgrade.md` is a **boundary skill** too, at `templates/` root rather than among
the stage templates: it invokes no `enter-stage.sh` and stamps no state, so
`check-stage-skill-coverage` never reads it. It is the phase-B disposition walk a consumer runs when moving
their vendored kits from one release to the next — the judgment half of the
two-phase upgrade contract whose deterministic half (the wholesale kit-sync) and
whose executable proof both live in gate-sdk (gate-sdk/SPEC.md §upgrade-smoke),
against the release-note grammar docs/install.md §The upgrade contract owns. Its
ritual registers the target note's newly-declared gates (a new gate's only
delivery channel to an upgrading consumer — the phase-A sync never re-runs the
installer), dispositions each red gate (fix-the-tree or exempt-with-cause, never
a weakened gate), and closes on the semantic-residual audit: the upgrade is the
cadence at which a consumer judges the ungateable third of a template-slot change
— the shim fill that clears both `check-skill-binding` (slot-set drift) and
`check-shim-restatement` (verbatim copy) yet duplicates what the new slot now
means to own. It carries named slots (`gates-list`, `disposition-evidence`), so
like the stage skills it adopts the binding-shim grammar (§templates/stages/) and
`check-skill-binding` holds the slot pairing when a consumer binds it. This repo
binds no command for it — the repo is the kit source, never a vendored consumer,
so it never upgrades itself; the template ships for consumers and the upgrade
smoke exercises the mechanics it narrates.

### templates/lead.md

The **iteration lead** template — an optional live session that dispatches an
iteration's stage sessions and answers their escalations, closing the
restart-cost of a stage that would otherwise stop and surface to the user cold
(§The state machine). Like `release-sweep.md` it is a **boundary skill, not a
stage**: it invokes no `enter-stage.sh` and joins no stage set, so
`check-stage-skill-coverage` never reads it. Like release-sweep it carries
named slots, so it adopts the binding-shim grammar (§templates/stages/) — a
consumer copies-and-specializes it or binds it through a thin shim, and
`check-skill-binding` holds the slot pairing either way (this repo's
`.claude/commands/lead.md` shim).

The template owns the orchestration protocol whole: the two lead postures
(**unified** — the scope session stays live as the lead, one session holding
judgment and dispatch on one model tier; **split** — a routing-tier lead
dispatches scope as a stage session on the judgment tier and keeps it
resumable as the iteration's *intent oracle*, ruling machinery questions
itself, forwarding intent questions to the oracle with the working-state
excerpt each turns on, and falling back to the governed surfaces — then the
operator — when the oracle is gone; posture and tier assignment are standing
dispatch policy in the ruling-config slot), the lead model (dispatch a
stage session as a background agent whose prompt is that stage's ordinary skill
invocation, with the inline-run posture sentence reading
`LIFECYCLE_KIT_SESSION_BOUNDARY` — inline stage runs banned under `stage`,
the sanctioned blocked-dispatch fallback under `iteration`; and the
completion-notification dispatch precondition, with the prompt-answered-signal
trap and the gating-versus-liveness boundary that keeps it from reading as a
reversal of the hand-derivation corollary below — its honest limit is recorded
in §The state machine and it is standing lead policy under the
policy-is-config rule), the
opening-an-iteration contract (the lead never selects the unit set — it relays
the operator's standing directive, a theme bounding scope's survey and never a
slug list, verbatim in the scope dispatch, and routes scope's proposed set back
as an ordinary escalation; selection is scope's contract, and a lead-authored
menu pre-empts the premise re-verification), the four-header
escalation block (Question / Options / Recommendation / Evidence) together with
the one class the lead never rules under either posture — reversing, demoting or
re-scoping a recorded operator ruling or a stated objective is operator-class and
is relayed, carved out of the derivable-from-the-governed-surfaces routing rule
because a session holding contrary evidence reads the surface carrying the ruling
as stale rather than as closed, the split-channel design (routine narration to the
resume journal, escalations to the message channel), the compact economics —
the split-where-the-tail-dominates rule, the unified posture's handoff compact,
and operator-suggested compacts at the acceptance
boundaries that pay under the cold-wakes-times-compressible-residue rule —
with the dispatch-granularity rule (the roster derived from every unit the
iteration promoted rather than from the amendment set, then batch units
sharing a kit or SPEC surface, split on a model-tier change or a
delegation-kit split trigger) and
the lead-owns-batching clause (an intra-stage batch split is N sibling stage
sessions the lead dispatches and verifies — each a same-stage re-entry,
§The state machine — and a stage session never dispatches a sibling stage
session), and
the stamps-authoritative invariant carried from §The state machine as the
design's load-bearing rule — with its two corollaries: the lead never
hand-derives prior-stage completeness from WORKFLOW-STATE or the git log (it
dispatches and trusts `enter-stage.sh`'s fail-closed refusal, or gates an
expensive dispatch with `--simulate`, and reads that same drain-entry verdict
**before** declaring a stage's batches complete rather than after,
§bin/enter-stage.sh), and a ruling
whose acting session is not imminent is filed to a durable governed surface
in the moment it is made. An earlier second assertion over the drain
assertion's own population is **ruled out** on record: a batching roster has
dropped a promoted unit in practice and `check-stage-entry` named
it at the refused entry, so the defect is detected late — at the price of one
wasted dispatch — rather than undetected, and a duplicate reading of one fact
would buy only the timing the simulate read already buys for free.
The template also carries the lead's first step —
writing the session-role marker context-kit's hook reads
(context-kit/SPEC.md §The session-context hook).
**Both of the lead's scratch artifacts outlive the iteration the boundary wipe
reclaims, and that is one fact rather than two exceptions.** A lead session is
live *at* the boundary — it files a boundary judgment there for the entering
session's intake — so its session-role marker and its own resume journal alike
have a live session's lifetime rather than the iteration's, and a consumer that
names either keeps it on `LIFECYCLE_KIT_BOUNDARY_PRESERVE` (§bin/enter-stage.sh).
The contrast with §templates/consult.md is deliberate and reached from the
opposite direction: a consult session may span the boundary too and still takes
**no** preserve entry, because its rulings are discharged into a commit as each
closes, so a preserved journal would outlive the session that can interpret it. A
*stage* session's journal takes none either, being spent by the boundary it is
reclaimed at. So the question a keep-list candidate answers is **where its
content is discharged**, not which session class wrote it — and a lead's journal
is the case where the answer is *nowhere yet*, the channel still being read. Dispatch safety is not re-owned — it inherits
delegation-kit's protocol by citation (delegation-kit/SPEC.md §The delegation
model: background dispatch, the per-dispatch budget guard, verify after any
agent commit) — with one lifecycle **instance** the generic rule cannot state
because it would have to name a stage: when the dispatched stage is the
evidence-producing one, the lead's verify is a read of the committed evidence
manifest rather than a re-run of its producer, the harm splitting between an
inert battery re-run and a destructive producer re-run (delegation-kit/SPEC.md
§Verify after every agent commit owns the generic rule and the split; its honest
limit here is recorded in §The state machine). Consumer residue stays in named slots — the tracked
agent-definition carrying the standing dispatch policy the dispatch names (the
ruling-class roster and everything else true of every dispatch, not improvised
per prompt), and whether the consumer wires the optional escalation-shape guard
(guard-kit/SPEC.md §wakeup-guard) or leaves it inert.

### templates/consult.md

The **operator strategy session** template, and a **boundary skill** of the class
§Layout and configuration enumerates: like `lead.md` it invokes no
`enter-stage.sh`, stamps nothing, and joins no stage set, so
`check-stage-skill-coverage` never reads it. The classification is what makes it
cost no new mechanism. A consultation may run before an iteration opens, between
stages, or across an iteration boundary, and a template with no cursor is at odds
with none of those. It carries named slots, so it adopts the
binding-shim grammar (§templates/stages/) and `check-skill-binding` holds the
slot pairing.

*A seventh stage was ruled out.* It would need a predecessor entry in
`LIFECYCLE_KIT_STAGES`, a fixed position in the walk, and a stamp — none of which
a session that may precede or span an iteration has — and it would make
`check-stage-entry` assertion A demand a consult stamp on every iteration that
legitimately holds no consultation.

**What the template owns** is a landing contract: the session's exit condition is
that every ruling the operator closed has reached a governed surface, every
refused alternative is recorded with its grounds, and every always-loaded surface
those rulings stale is corrected, flagged, or filed. Two mechanisms are what make
that reachable rather than aspirational, and both are stated in the ritual. A
ruling lands **at the moment it closes**, not at exit, so an interrupted session
loses at most the ruling in flight rather than the session's whole output. And
the session journals while it cannot commit, on the durability rule at its owning
tier (delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset
sweeps), which is what converts a shared index held by a live stage session from
data loss into latency. The journal's reclaim is the existing boundary wipe of
the scratch dir (§bin/enter-stage.sh) and no `LIFECYCLE_KIT_BOUNDARY_PRESERVE`
entry is added — a preserved journal would outlive the session that can interpret
it, its content having been discharged into a commit at exit.

**Write authority is not this template's to grant.** Who may record a ruling on a
consumer's ruling record is stated by that record, at the widest tier true for
every reader of it — every session that closes a ruling with the operator faces
the question, not only a consult session. What the skill adds on top is
obligation: elsewhere recording a closed ruling is permitted, here a consultation
that did not record one has not exited.

Dispatch safety is inherited by citation rather than re-owned
(delegation-kit/templates/agent-execution.md), on the same rule
§templates/lead.md follows — a copied protocol is a second content tier that
drifts. The template does state **tier selection** in its own right, because the
hazard is sharper here than for a stage session: an unselected dispatch inherits
the dispatcher's tier, and a consultation runs at the judgment tier while most of
what it dispatches is read-only research, so an unselected fan-out buys the most
expensive tier for the cheapest work.

Consumer residue stays in exactly **two named slots**, and the split is the
provenance seam rather than a convenience — `entry-reading` (the surfaces a
consultation reads on entry) and `landing-surfaces` (the surfaces a closed ruling
may land on, by ruling class). A consumer's entry set reaches its private
context, and a kit literal naming it would publish a private surface; its landing
set names that consumer's own governance layout, and a kit shipping those names
would ship one project's layout as everyone's. What stays kit mechanism is the
shape: a consultation has an entry read-set and an exit landing-set, and every
closed ruling reaches the landing set before the session ends. Each slot has a
named reader — the entry step and the exit check respectively — and
`check-skill-binding` requires the shim to bind exactly that set, so an unbound
slot or an orphan binding is caught mechanically rather than by review.
