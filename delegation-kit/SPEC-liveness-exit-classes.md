# SPEC amendment: liveness-exit-classes

Closes `turn-end-liveness-exit-two-conflation`. The SubagentStop hook reads the
gate substrate's fail-closed "could not run" as a malformed launch record, so
every worktree-isolated agent is refused at turn end.

**The mechanism was re-run first-hand at this spec stage rather than carried
from the queue entry, and the run corrected the entry on two points.** The
correction is recorded here because the entry's wording inherits a *false*
diagnosis the code itself prints.

## What changes

### (1) Reader exit 2 splits by record count, and the split is decidable from data the hook already holds

`verdict=corrupt` stops being the hook's only reading of reader exit 2: exit 2
over a **non-empty** `*.run` set stays `corrupt`, and exit 2 over an **empty**
set becomes a new verdict `unresolved` — a **diagnostic** split, not a decision
one. Both refuse: `unresolved` carries `decision=refuse`, exit 2, exactly as
`corrupt` and `red` do. **{design-bearing}**

The ground is not a judgment about appetite, it is a proof from the reader's own
contract. `check-producer-liveness` in set mode derives corruption **per
record** and aggregates *exit 2 wins over red wins over green*
(evidence-kit/SPEC.md §check-producer-liveness). Over an empty glob there is no
per-record verdict to aggregate, so the gate **cannot** return corrupt on an
empty set. A reading of `verdict=corrupt records=0` is therefore provably *not*
record corruption; it is the reader failing for a reason that has nothing to do
with any record. The hook already computes `records` by its own glob over the
same directory, so the discriminator costs no new field, no new knob and no
second reader.

**Measured, not deduced.** A worktree-isolated agent dispatched from this spec
session logged **fifteen** firings into its own worktree, every one
`live=no verdict=corrupt records=0 decision=refuse`. Fifteen of fifteen
refusals carried an empty record set. Re-derive by dispatching an isolated agent
and reading its worktree's own copy of the log rather than citing this figure.

**The record count labels the diagnosis and decides nothing — that is the
ruled shape, not the first-drafted one.** §The turn-end liveness hook records
that a capped option — "refuse only on a `verdict=red live=yes` reading
carrying at least one record, stay advisory otherwise" — was offered to the
operator and **refused**, unconditionally, with the record count deliberately
excluded as a condition on the `corrupt` arm at that same landing. This delta
does not reopen that exclusion: the record count chooses which **name** a
refusing exit-2 reading gets — `corrupt` over a non-empty set, `unresolved`
over an empty one — and it chooses nothing about whether the turn end refuses.
Both names carry `decision=refuse`. The reader's own contract still supports
the asymmetry that motivates the *name*: a record count is vacuous for `red`
(red implies a record) and not vacuous for exit 2 (exit 2 can occur with
zero) — but that asymmetry earns a diagnostic label, not a second decision
path. See the ruling below for how this delta's first draft read that
asymmetry as licensing the latter, and was corrected.

**RULED 2026-08-24 by the operator, relayed through the lead at align, and
recorded here because the deliverable this delta ships is the ruled shape, not
the one spec first drafted.** Spec's draft mapped `unresolved` to
`decision=allow`, arguing that `verdict=corrupt records=0` is provably not
record corruption and that this was a *different* move from the refused
capped variant, which made the hook advisory everywhere outside `red`. The
iteration lead set a tripwire on exactly that reasoning at align entry —
*adjacent to a refused option is exactly the shape that reads as derivable
when it is not* — requiring any edge of narrowing found later to escalate
rather than be resolved by whoever found it, however strong the grounds.

**The align audit found the narrowing, independently, and it is recorded as a
finding rather than left to be re-derived.** Checked directly against the
shipped hook (`scripts/subagent-stop-liveness.sh`), today's `verdict=corrupt`
maps to `decision=refuse` **unconditionally**, at every record count. Spec's
draft `unresolved` arm would have flipped the `records=0` case to
`decision=allow` — a real, mechanical edge of the operator's 2026-08-24
"unconditional" ruling that would have narrowed on delivery. The operator
ruled on that finding: **keep the three-way verdict split for its diagnostic
value, and map `unresolved` to `decision=refuse`, so no edge of the secured
refusal set narrows.** That is the shape landed above.

**Not a reversal, demotion or re-scoping of the 2026-08-24 "unconditionally
refuse on `red` or `corrupt`" ruling — it is that ruling holding intact
against a delta that would have narrowed it.** Spec's technical grounds were
sound and are unchanged by this ruling: `records=0` at exit 2 genuinely cannot
be record corruption, given `check-producer-liveness`'s own per-record
aggregation contract (§check-producer-liveness). The ruling did not turn on
that being wrong. It turned on the unconditional refusal being worth more than
the diagnostic gain of letting one provably-empty reading allow. Spec's claim
that its draft "kept every refusal the ruling secured" was made in good faith
on those sound grounds and was simply not what the ruling decided — recorded
here so this does not read, on a later pass, as the amendment quietly walking
back a claim it once made carelessly. Two competent sessions reading the same
delta and disagreeing is exactly the case the tripwire exists for.

**The cost this ruling keeps, stated honestly rather than left for a later
session to discover and "helpfully" repair.** A worktree-isolated
`audit-sweep` dispatch is **still refused at turn end** on a binary-absent
reading: `verdict=unresolved` still means `decision=refuse`, so the refusal
`turn-end-liveness-exit-two-conflation` was filed against is unchanged by this
delta. That refusal's disposition, if any, belongs to that entry's own
subject and is not reopened here — build must not read the still-refusing
`unresolved` arm as an oversight this delta forgot to loosen.

**Why a new verdict value rather than folding onto `unavailable`.** The three
non-answering states carry different follow-up actions for a human, and — per
the ruling above — two different hook decisions as well: `unresolved`
refuses, the other two allow. A name that merged `unresolved` into
`unavailable` would cost the close-stage triage the distinction it was given
`error` for in the first place, and would also hide that one of the three
refuses while the other two do not:

| verdict | what happened | hook decision | is it actionable |
| --- | --- | --- | --- |
| `unavailable` | no reader named, or the named path is unreadable | allow | no — this tree never configured enforcement |
| `error` | a configured reader ran and gave an unmapped answer, or the `timeout` bound fired | allow | yes — this tree's enforcement is broken |
| `unresolved` | a configured, readable reader ran, exited 2, and held no record to be about | **refuse** | yes — and the fix is a *different* one: the reader could not run at all |

**The refusal's stderr must not reuse `corrupt`'s wording for `unresolved`.**
The hook's message branch is two-way today (`red` versus everything else that
refuses), and folding `unresolved` into the "everything else" arm would print
"a launch record ... does not parse" over a case where there is no record to
parse at all — exactly the false-diagnosis shape delta 3 fixes one caller
over, reintroduced here if the branch is not widened. The branch becomes
three-way: `red` names a live producer, `corrupt` names a record that does not
parse, `unresolved` names a reader that produced no reading at all — no
record and no diagnosis of one — each keeping its own `look` remedy line.

### (2) The record glob is taken after the reader runs, not before

`scripts/subagent-stop-liveness.sh` globs `records` at lines 27-29 and runs the
reader at lines 33-46; the two swap, so the count delta 1 branches on is read
**after** the reading it qualifies. **{mechanical}**

A record created between the glob and the reader would today produce
`records=0` alongside a reading that legitimately saw that record — the one
window in which delta 1's discriminator could allow a genuine corruption. Taking
the glob after the reader does not close the window (nothing but a lock could)
but it inverts which way an in-flight record errs: a record appearing during the
reader's run is then *counted*, and the reading refuses. The residual window is
one malformed record's lifetime against a battery member that reds on the next
commit, which is the bound §The turn-end liveness hook already prices the corrupt
arm at.

### (3) `gate-exec.sh` stops overwriting a true diagnosis with a false one

The front end's unresolvable-gate branch stops firing on a gate that **did**
resolve, so a binary-absent failure is reported as what it is.
**{design-bearing}**

**Probed at this stage, not asserted.** Running
`GATE_SDK_NATIVE_BIN=/nonexistent bash scripts/producer-liveness-reader.sh .tmp`
exits 2 having printed **two** stderr lines: `gate_command`'s own true one
naming the absent binary and the build command that fixes it, and then
`gate-exec`'s `check-producer-liveness resolves in none of: <dirs>`, which is
false — the gate resolved in the first directory tried.

The cause is a control-flow seam rather than a wrong message. `gate_command`
(gate-sdk/lib/gate.sh) handles a `.gate` member whose binary is absent by
`exit 2` **from inside itself**, and `gate-exec.sh` calls it through a
**process substitution** (`mapfile -t argv < <(gate_command …)`), so that exit
kills only the subshell. `mapfile` then binds an empty array, and the
`${#argv[@]} -gt 0` test — whose only modelled cause of emptiness is
`gate_command`'s `return 1` — re-diagnoses a harness error as a resolution
failure. The front end reads the two causes through one observable, which is the
same conflation this amendment is named for, one caller up.

The repair keeps both causes distinguishable at the call site: `gate-exec.sh`
captures `gate_command`'s output **and its status** (a command substitution, not
a process substitution), and branches — status 1 is the genuine
resolves-in-no-check-dir refusal and keeps today's message; status 2 is a
failure `gate_command` has **already** named on stderr, and the front end
propagates it without adding a second, contradicting sentence.

**This is the delta that reaches past this repo.** The two prior deltas are the
hook's; this one is the shared front end every `LIFECYCLE_KIT_ENTRY_PREFLIGHT`
entry and every name-addressed gate call in this tree goes through, so its
blast radius is every consumer of that pattern and not the turn-end hook alone.

### (4) The worktree/binary seam is ruled by declaration, taking candidate 3's second horn

`templates/agent-execution.md` gains a stated limit: inside a worktree-isolated
dispatch, **binary-dispatched gates do not resolve**, so an isolated agent may
not read a gate verdict as available and may not repair one by building the
crate. **{design-bearing}**

The queue entry's third candidate offered two horns — build the crate at
dispatch entry, or state in agent briefs that binary-dispatched gates are
unavailable under isolation. The second is taken and the first is refused with
cause: an 11.69s build on every dispatch is a real cost paid on every isolated
agent whether or not it ever reaches a gate, and there is no hook that can reach
*into* a worktree that does not exist yet at `PreToolUse(Agent)` time.

**Probed at this stage.** A live worktree-isolated dispatch's checkout carried
no `native/target` at all — the crate's build output is gitignored, so a fresh
worktree checkout has none. That is the defect's precondition, confirmed by
looking rather than inferred from `.gitignore`.

**The declaration earns a second clause from an attested behaviour, and this
half is new.** The isolated agent dispatched at this stage, dispatched
**read-only**, reported that it built the native binary to escape the refusal
loop. That report is carried at the tier it was received — the agent's own claim
about its own run, unverifiable here because the evidence went with the
reclaimed worktree; what **is** verified is that the main checkout's binary
mtime was unchanged and `git status` stayed clean, so nothing crossed into the
main tree. The behaviour is the point regardless of who did it: a refusal whose
message names no reachable remedy invites a read-only agent to take a mutating
one. So the declaration states the limit **and** names the lawful response —
report the gate as unavailable and return — rather than leaving an agent to
invent one.

### (5) The isolated-agent firing class is declared unreachable by the log's own named reader

§The turn-end liveness hook's `decision` field bullet gains the honest limit
that its named reader — the close-stage triage — **cannot see a
worktree-isolated agent's firings at all**. **{design-bearing}**

`DELEGATION_KIT_STOP_LOG` defaults to a path under
`${GATE_SDK_WORKFLOW_DIR:-.workflow}`, resolved against the *writing session's*
cwd. An isolated agent's cwd is its own worktree, so its lines land in that
worktree's `.workflow/` and are destroyed with it at reclamation. Observed at
this stage: fifteen `decision=refuse` lines existed in the child's worktree and
none of them exists now.

This is a causal-completeness break in the **existing** contract rather than
one this amendment introduces, which is why it is declared here instead of
being repaired here: `decision` was given a named reader at a named transition
(the close-surface drain) and a whole class of firings never reaches it —
precisely the class this amendment is about. The field's claim that a refusal is
*countable* is true only of main-checkout firings.

**Not repaired here, and the reason is a seam.** Pointing the knob at an
absolute main-checkout path would make one consumer's log reachable and would
also bake a machine path into a consumer surface; deriving the main checkout
from inside a worktree is possible — `git rev-parse --git-common-dir` answers it
vendor-neutrally, probed at this stage — but that is a mechanism this unit did
not scope. It is filed as a costed deferred entry rather than flagged and
skipped.

## Producers and consumers

**The new verdict value `unresolved` (delta 1).**

- *Producer:* `templates/subagent-stop-liveness.sh` and this repo's copy
  `scripts/subagent-stop-liveness.sh`, on the reader-exit-2 arm when the hook's
  own `*.run` glob is empty. **Enabling config actually set:** the arm needs no
  new configuration — it fires on the existing `DELEGATION_KIT_LIVENESS_CMD`
  path this repo already sets to `scripts/producer-liveness-reader.sh`, so it is
  live in this tree the moment the script lands, not test-only.
- *Consumer:* two, at two transitions. (a) The **hook's own decision**, in the
  same run: `decision=refuse`, exit 2, with a stderr reason distinct from
  `corrupt`'s (the three-way message branch above) — the diagnosis changes,
  the decision does not, per the 2026-08-24 ruling. (b) The **close-stage
  triage** at the close-surface drain, reading
  `.workflow/subagent-stop-liveness.log` before clearing it
  (lifecycle-kit/SPEC.md §The close-surface roster), which reads it to
  distinguish a broken reader (`error`) from one that could not run at all
  (`unresolved`) — different fixes, which is what earns the third name, even
  though both now refuse identically to `corrupt` at the hook's own decision.
- *Named reader for every field:* no field is added. `verdict` gains a value and
  keeps its reader; `records`, `live` and `decision` keep theirs unchanged. The
  grammar's field list and its order are untouched, so the space-delimited parse
  the close-stage reader uses does not move.

**The reordered glob (delta 2)** introduces no state. Its producer and consumer
are the same script and the same run; it changes which of two existing values is
read first.

**The status-carrying call in `gate-exec.sh` (delta 3).**

- *Producer:* `gate_command` (gate-sdk/lib/gate.sh), which already emits both
  signals — `return 1` for a gate resolving in no check dir, `exit 2` having
  written its own stderr for a `.gate` member whose binary is absent or whose
  knob bridge refused. Nothing new is produced; an existing signal stops being
  discarded.
- *Consumer:* `scripts/gate-exec.sh`, at the resolution step, before `exec`.
  Its own consumers are unchanged and are every caller that reaches a gate by
  name: this repo's `LIFECYCLE_KIT_ENTRY_PREFLIGHT` roster and
  `scripts/producer-liveness-reader.sh`.
- *Named reader:* the **committing or entering session**, through stderr. The
  reader that made this delta necessary is the one that was being lied to.

**The declared limit (deltas 4 and 5)** introduces no state, event or interface.
Both are prose obligations on surfaces that already exist, and their consumer is
the reader of those surfaces — a dispatched agent loading
`templates/agent-execution.md` at delta 4, the close-stage triage at delta 5.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 1, as ruled, **narrows nothing** — the RULING note above is the record
of why the first draft did and this shape does not. It is analysed anyway
because it changes what a verdict reader observes (a new label and a new
stderr message on an existing refusal), and the point-5 rule is about the
argument being made rather than about the author's confidence in it:

- `delegation-kit/gate-tests/subagent-stop-liveness.test.sh` — reds when a
  driven verdict arm's **exit code or `decision` column** differs from the
  asserted one. **Monotone for every existing case**: the fixture's `corrupt`
  arm already drives a **non-empty** run dir (`pid=1 run=k`, written before the
  case loop), so its exit code and decision are untouched by this delta. A new
  empty-dir case is still owed — it drives reader exit 2 over zero records and
  asserts `verdict=unresolved`, `decision=refuse`, exit 2, and the three-way
  branch's own distinct stderr wording, never the `corrupt` arm's "does not
  parse" text.
- `scripts/gate-tests/subagent-stop-reader.test.sh` — reds when the configured
  reader's arm does not produce the asserted verdict. **Monotone for its two
  existing cases**: `clean` fires over a truly empty directory through the
  real, resolvable gate and gets `verdict=green` (the gate answers zero
  records cleanly; it never reaches exit 2), and `live` fires over one
  live-pid record and gets `verdict=red` — neither exercises exit 2 today, so
  neither changes under this delta. A **new** case is owed, and it is the one
  this whole amendment is about: fire through the real reader with
  `GATE_SDK_NATIVE_BIN=/nonexistent` (the exact probe spec ran) over an empty
  run dir, and assert `verdict=unresolved decision=refuse` — **not**
  `unavailable`, which would misreport a resolvable, configured reader as one
  that was never wired at all.
- `check-producer-liveness` — **untouched**: no delta changes the gate, its
  exit classes, or the `pid=<n> run=<key>` grammar. Reds on a live pid or a
  malformed record exactly as today.
- guard-kit rule 14 — **untouched**: it reads records one at a time and takes no
  reading from this hook. The rule-14/hook divergence recorded on both surfaces
  is *unchanged in substance* and gains one clause, which is why both sides are
  update targets below.
- `check-workflow-tiering` — reds on a `.workflow/` member that is neither
  tracked nor ignored. **Monotone and cleared by inspection**: no delta adds,
  removes or renames a `.workflow/` member.

## Existing sections updated

- `delegation-kit/SPEC.md` §The turn-end liveness hook (template) — the
  five-row verdict table gains `unresolved` as a sixth reading that refuses
  identically to `corrupt`; the `corrupt`/`unresolved` split is stated as a
  record-count-driven **diagnosis**, never a record-count-driven **decision**;
  the "**`records=0` is not a clause**" paragraph is extended rather than
  rewritten into falsity, since the 2026-08-24 ruling keeps it true of exit 2
  as well as of `red` — record count decides no refusal anywhere in this hook,
  before or after this delta — and the refused-capped-variant disclosure is
  extended to this second coincidence, now resolved in the ruling's favour
  (deltas 1 and 2).
- `delegation-kit/SPEC.md` §The turn-end liveness hook (template), the
  per-field bullet list — the `verdict` and `verdict=error` bullets name the
  third actionable state, and the `decision` bullet takes delta 5's honest
  limit (deltas 1 and 5).
- `delegation-kit/SPEC.md` §The turn-end liveness hook (template), the
  two-lane testing paragraph — both lanes move together, as that paragraph
  itself requires (delta 1).
- `delegation-kit/templates/subagent-stop-liveness.sh` — the shipped template,
  which is the surface a consumer copies; the consumer copy
  `scripts/subagent-stop-liveness.sh` moves with it or the seam that hid the
  dead default for an iteration re-opens (deltas 1 and 2).
- `delegation-kit/templates/agent-execution.md` — the isolated-dispatch limit
  and the lawful response to it (delta 4).
- `guard-kit/SPEC.md` §The generic ruleset, rule 14 — the recorded divergence
  from the hook's `corrupt` disposition is restated from rule 14's side, since
  the hook now names a `records=0` sub-case (`unresolved`) that rule 14's
  per-record read has no analogue for, even though neither side's decision
  changes — the divergence is unchanged in substance, gaining one clause
  (delta 1).
- `evidence-kit/SPEC.md` §check-producer-liveness — the aggregation contract
  (*exit 2 wins over red wins over green*) gains the consequence delta 1 rests
  on, stated where the contract lives: the aggregation **cannot** yield exit 2
  over an empty record set (delta 1).
- `gate-sdk/SPEC.md` §Fail-closed contract — the contract's own account of a
  `.gate` member with an absent binary states that the refusal is raised inside
  `gate_command` and that a caller reading it through a process substitution
  observes only an empty argv (delta 3).
- `gate-sdk/SPEC.md` §lib/gate.sh — the `gate_command` entry states its two
  distinguishable failure signals and that a caller must capture status to tell
  them apart (delta 3).
- `evidence-kit/SPEC.md` §check-evidence-manifest — the section that owns the
  consumer-side front-end pattern, and therefore owns what that front end must
  do with a resolution failure it did not cause (delta 3).
<!-- update-target-exempt: a consistency re-read owned by no delta — the sibling amendment SPEC-agent-id-doubt.md owns every edit to that bullet, and claiming it here would give one bullet two owners across two amendments -->
- `delegation-kit/SPEC.md` §What `background_tasks` carries — no content change;
  re-read at merge to confirm the `agent_id`/`session_id` claims it shares with
  the `session` bullet stay consistent with the sibling amendment editing that
  bullet.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), discharged at the iteration
      rather than at this commit, three amendments being in flight for it.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Both test lanes moved together** — the kit's stub-driven
      `gate-tests/subagent-stop-liveness.test.sh` and the consumer's
      `scripts/gate-tests/subagent-stop-reader.test.sh`, per the narrowing
      analysis above.
- [ ] **Re-derived, not cited** — the fifteen-of-fifteen figure and the
      two-stderr-line probe re-run against the tree at build, not carried from
      this file.
