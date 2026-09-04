# SPEC amendment: file-authoring-act

Governance for **the act of bringing a file into being**, which today has no
steer, no grant and no owner — so authoring one costs an out-of-band permission
decision every time. The unit is `file-authoring-act-ungoverned`, promoted under
an **operator** ruling of 2026-09-04 relayed over the `lead-relay` channel and
already recorded on the entry; the ruling reopens the port-only run for a second
non-port unit, and TRAJECTORY.md carries that reconciliation. The authority here
is the operator's and not the lead's, and that is stated because a ruling
recorded without its authority reads at a later triage as more settled than it is.

**The re-costing is part of that ruling rather than a caveat beside it**, so this
amendment inherits no figure — not the entry's 56 of 139, not the prior close's
per-stage decomposition of 52 `cat >>` calls. Every number below was measured at
this HEAD.

## The measurement, and what is honest about it

`bash gate-sdk/bin/run-gates.sh --emit scan-prompts` over the live friction log —
149 raw fall-through lines accumulated since this iteration's boundary reset —
reads **48 prompting calls across 22 patterns**.

**It is not comparable to the entry's 56 of 139 and this amendment does not
compare them.** That figure was a close-time reading over a whole iteration; this
one is mid-iteration with two stages run, the log not yet cleared. Scaling one
into the other would manufacture exactly the inherited number the ruling forbids.
The corpus is also **live and non-stationary** — it grew from 143 to 149 lines
during the measurement, sibling sessions appending — so the reading was taken
against a frozen snapshot and is reported as *as of that snapshot*.

**The class, two ways, because the instrument's key over- and under-counts and
both directions were traced to the raw lines rather than trusted.**

- **Mechanically** — every `>`/`>>`-suffixed row summed — **29 of 48, 60.4 %**.
- **Judgment-corrected** — **24 of 48, ~50 %**. Four `kill >` rows and one `ls >`
  row carry an incidental `2>/dev/null` on a liveness probe whose real friction
  is a later segment; one `git show >` row's own write is already granted. Against
  that, **two genuine writes hide under `mkdir`**, invisible to any write-suffixed
  key because the axis reads the **first segment only**
  (guard-kit/SPEC.md §scan-prompts' own stated limit). One row (`grep >`) could
  not be resolved to a raw line and is left unresolved rather than guessed; it
  moves the figure by at most one.

Either way the class is **the largest single one in the ranking by a wide margin**
— the runner-up row carries four — and `cat >>` alone carries **17**, more than
every other row combined. All seventeen are resume-journal appends.

**The segment-selection defect is already filed** as
`friction-key-segment-selection-unruled` and is **not re-filed here**; this
amendment cites it as the reason its own two figures differ.

## What changes

### (1) The finding that reframes the unit: rule 17 barely fires on the shape it was written for

`guard_rule_append_scratch` — rule 17, the auto-allow for an append-only write to
a gitignored target — declines on **a backtick anywhere in the raw command**
(`guard-kit/lib/guard.sh:842`, `case "$raw" in *'`'*) return 0 ;; esac`), and that
includes a backtick inside a **quoted-delimiter heredoc body** {design-bearing}.

**Measured, not argued**: of the seventeen `cat >>` journal appends in the class,
**sixteen of the sixteen locatable ones carry at least one backtick** in the
heredoc body. Journal markdown names slugs, paths and knobs in backticks as this
project's house style, so the rule written for exactly this write is defeated by
the way the write is always spelled.

**The rule is not wrong about substitution; it is wrong about where to look.**
Its ground, stated at `:840`, is that "a grant may not rest on a coverage claim
that is only mostly true" — rule 6 blocks three of the four substitution
spellings but not the output-process-substitution one, so rule 17 re-tests the raw
command itself. That reasoning is sound and survives. What does not survive is
applying it to a region where the spelling **cannot** substitute.

### (2) Clause (d) is narrowed to the region that can actually expand, on the library's own existing ruling

The backtick and substitution decline runs against `guard_skeleton "$raw" hdq`
rather than against `$raw`, so a quoted-delimiter heredoc body is blanked before
the test and an unquoted one is not {design-bearing}. Nothing else about rule 17
changes: the `>>`-only redirect test, the gitignored-target test, the
`GUARD_KIT_APPEND_BINS` roster, the single-statement test and the backgrounding
decline are all untouched.

**This is the same ruling applied one rule over, not a new one.**
`guard-kit/lib/guard.sh:350-351` already states it verbatim, for rule 6: "a
double-quoted `\"$x\"` still expands, so `dq` stays live; **a quoted-delimiter
heredoc body cannot, which is what `hdq` names**". A backtick inside `<<'EOF'` is
inert by the identical argument that makes `$x` inert there, and the `hdq`
skeleton mode that encodes it is already shipped and already used at `:352`. So
this delta mints **no mechanism, no mode and no knob** — it moves one test's
subject onto a skeleton the library already computes.

**The asymmetry is preserved deliberately.** An **unquoted** heredoc delimiter
(`<<EOF`) leaves the body live, so a backtick there is a real substitution and the
decline still fires — which is why the narrowing is `hdq` and not `hd`. Stated
because `hd` is the mode three neighbouring rules use and reaching for it here
would silently grant a body that can run a command.

**Measured payoff**: roughly seventeen of the twenty-four-to-twenty-nine class
calls, and **about 35 per cent of every prompting call in the snapshot**, removed
by one clause. That is the largest single reduction available anywhere in the
ranking and it costs no new surface.

### (3) The entry's named collision is measured at zero by construction, and saying so is the honest half

The entry warns that a steer "must not fire on the `>>`-under-scratch shape rule
17 already auto-allows, which is the shape the mandated resume-journal write
itself uses" {design-bearing}. That collision **cannot be sized from the friction
log**, and the reason is structural rather than a gap in the sweep: a call rule 17
grants exits at `guard_allow` (`guard-kit/lib/guard.sh:91-95`) **before**
`scripts/bash-guard.sh:39` reaches `guard_log_fallthrough`, so a granted call is
never logged. The measured overlap is therefore **0 by design**, not 0 by finding.

This delta records that limit rather than reporting the zero as a result, and it
is what makes delta (2) the safer move than a steer: widening the grant needs no
estimate of a population the instrument cannot see, where a steer would have to
avoid one.

**And delta (2) inverts the collision rather than navigating it.** After the
narrowing, the resume-journal append is granted where today it prompts — so the
mandated write and the governance stop pulling against each other, which is the
condition that made shape (c) look wide in the first place.

### (4) The residue after delta (2), and its shape is a create-side grant symmetric with rule 16

What delta (2) does not reach is the **create** half: `cat >` (2), `printf >` (1),
`echo >` (1), `head >` (1) and the two genuine writes hiding under `mkdir` — every
one a real-target, ungranted write to a gitignored scratch path
{design-bearing}. Rule 17 refuses all of them on one line,
`guard-kit/lib/guard.sh:865`'s `[[ "$op" == '>>' ]] || return 0`.

**The residue's answer is a create-side auto-allow under rule 17's whole clause
set, and its precedent is one rule away.** Rule 16 already auto-allows
`: > file` truncation of a gitignored target, so "destroying the prior content of
a scratch file is cheap" is a ruling this ruleset has already made; what rule 17
adds is a leading command from `GUARD_KIT_APPEND_BINS` and a heredoc body, neither
of which changes the target's disposability. The grant therefore extends to `>`
**only** where every other clause holds — gitignored target, roster-leading
command, single statement, not backgrounded, no live substitution — and the
`>>`-only test is replaced by a per-target gitignore test that was already there.

**This is a separate delta and not folded into (2) because the grounds differ.**
Delta (2) narrows a decline that was over-wide against its own stated reason;
this one **widens** what is granted, and it is a widening a reader should be able
to accept or refuse on its own.

### (5) Shape (c), the Write-tool steer, is recorded as refused-for-now with its cost

The entry's shape (c) — a guard steer routing a file-authoring composition onto
the `Write` tool, the way rules 8 through 11 route `sed`, `find`, `cat` and
`git grep` onto Read, Glob and Grep — is **not taken**, and the ground is that
deltas (2) and (4) remove its subject {design-bearing}. A steer that fires on
writes the same commit has just granted is a rule arguing with its neighbour.

Two facts are recorded so a later session reaching for it does not re-buy them:

- **The wiring exists.** `.claude/settings.json:126-158` already carries a
  `Write|Edit` `PreToolUse` matcher dispatching
  `run-gates.sh --hook workflow-state-guard`, so a Write-side rule needs no new
  matcher shape. That hook is scoped to one file — it refuses a hand-edit of
  `.workflow/WORKFLOW-STATE.txt` and names the sanctioned writer — and carries no
  general file-authoring governance.
- **A steer is not a distinct primitive.** `guard_block()`
  (`guard-kit/lib/guard.sh:81-84`) is stderr plus exit 2 for a block and a steer
  alike; the only difference is whether the corrective sentence names a
  replacement tool or says "run it yourself with `!<command>`". `guard_advise()`
  (`:86-89`) is `additionalContext` at exit 0 and never blocks; `guard_allow()`
  (`:91-101`) is `permissionDecision: "allow"` at exit 0.

**What refusing it costs, stated rather than omitted**: a composition no glob can
match — a heredoc write to a path outside scratch, say — stays ungoverned, and
deltas (2) and (4) do not reach it. The entry stays live for that residue.

### (6) The committed permission grant stays filed and is not landed here

The entry's shape (a) is a committed `permissions.allow` grant, and it is
**operator-class**: TRAJECTORY.md §The closed rulings (2026-08-22) rules a
`.claude/settings.json` edit "applied by the operator, out of band, or it is not
applied", and lets a stage session *prepare* one and no more {mechanical}.

The six-entry diff the entry already carries — `Bash(date *)`, `Bash(find *)`,
`Bash(git merge-base *)`, `Bash(git config *)`, `Bash(mkdir *)` and
`Bash(: > .workflow/subagent-stop-liveness.log)` — stays on the entry with its
security grounds, awaiting out-of-band application. **This amendment neither
applies it nor re-derives it**, and the entry remains the queue's carrier for that
obligation, which is delta (9)'s whole reason for demoting rather than closing.

The current measurement is consistent with the prepared diff and is recorded as
corroboration rather than as a change to it: `mkdir` ranks in the live log at two
calls and `find` at one, both of them named in that diff.

### (7) Shape (b), the stated habit, stays refused on the entry's own words

"A stated habit is not a mechanism" is the entry's sentence and this amendment
adds nothing to it {mechanical}. It is restated here only so the three shapes'
dispositions are readable in one place: (a) prepared and filed, (b) refused,
(c) refused-for-now with its cost, and a fourth the re-cost produced.

### (8) The fixture rows this lands with

`guard-kit/guard-tests/cases.tsv` — the 285-line `<decision>\t<command>` decision
table `guard-kit/bin/run-guard-tests.sh` runs — gains a row per new behaviour
{mechanical}:

- an **allow** for a `cat >> <gitignored> <<'EOF'` whose body carries a backtick,
  which is the sixteen-of-sixteen shape and today's decline (delta 2);
- a **fall-through** for the same write with an **unquoted** delimiter and a
  backtick in the body, which must still decline (delta 2's asymmetry);
- an **allow** for `cat > <gitignored> <<'EOF'` and for `printf > <gitignored>`
  (delta 4);
- a **fall-through** for a create-write to a **tracked** target, which no clause
  may grant (delta 4).

`:155` already carries the backtick-free `cat >> .tmp/journal.md <<'EOF'` allow,
so the new rows sit beside a live one rather than opening a family.

### (9) The queue entry is promoted, and its terminal move is a demotion rather than a Done move

`file-authoring-act-ungoverned` moves out of `## Deferred` into `## New Features`
with `[design-pending]` swapped for `[spec: SPEC-file-authoring-act.md]`
{mechanical}. Three facts, each measured:

- **The lead line must be re-wrapped, and no basename avoids it.** Measured with a
  byte count: the line is 99 columns and its base less `[design-pending] ` is 82,
  so a ref of any length overflows `QUEUE_KIT_WRAP_BUDGET=100`
  (`queue-kit/lib/queue.sh:37`). Re-wrapped, `- **file-authoring-act-ungoverned**
  [spec: SPEC-file-authoring-act.md] — writing a file has no` measures **94** and
  the remainder moves to the following line.
- **The entry does not reach `## Done`.** Its own text makes it "the queue's
  carrier for the prepared-diff obligation", which only an out-of-band operator
  application discharges (delta 6). A Done move would assert a finished deliverable
  that is not finished, and — the half canon-kit/SPEC.md §Merging an amendment
  calls the uncatchable one — a done entry is a bare slug, so **every tag it
  carries goes with it** and the carrier obligation vanishes silently. No gate
  reads either half of the Done-move contract.
- **So the terminal move is a demotion, and a demotion re-prices the entry against
  a cap a Done move would escape.** The entry is at **zero** headroom today —
  fifty lines against `check-queue-entry-budget`'s cap of fifty — so the demoting
  commit **compresses in the same motion**, answering grounds rather than dropping
  them. Neither owner states this alone: the cap is queue-kit's and the demotion is
  canon-kit's, and they meet only at an entry already at its limit.

### (10) The measurement lands in the record, not in the entry

The figures above are **not transcribed onto the queue entry** {mechanical}. The
entry is at zero headroom (delta 9), its own ruling obliges each promoting session
to re-measure rather than inherit, and a frozen mid-iteration snapshot is precisely
the kind of number a later reader would inherit if it sat there. The reading's
home is this amendment and the close's survey record; what the entry keeps is the
dated series in its ruling paragraph, unchanged.

## Producers and consumers

The amendment introduces **no new state, no new event, no new interface, no new
field and no new knob**. It changes the subject of one existing test and the
operator set of one existing clause, both inside `guard_rule_append_scratch`.

- **Producer** — `guard_rule_append_scratch` in `guard-kit/lib/guard.sh`, reached
  from `scripts/bash-guard.sh` on every `Bash` `PreToolUse` call. Its enabling
  config is the hook wiring at `.claude/settings.json:126-158`, which is live in
  this tree and is a consumer surface the kit names no path for.
- **Consumer of the grant** — the harness, reading `permissionDecision: "allow"`
  on stdout at exit 0 from `guard_allow` (`guard-kit/lib/guard.sh:91-101`). This is
  the only reader of the decision and the transition is the tool call's admission.
- **Consumer of the *absence* of a grant** — `guard_log_fallthrough`
  (`guard-kit/lib/guard.sh:103-106`), called from `scripts/bash-guard.sh:39`,
  writing one line to `GUARD_KIT_LOG`. Named because it is the reader delta (3)
  turns on: a granted call never reaches it, so widening the grant **shrinks the
  friction log's own corpus**, which is the intended effect and also the reason the
  next measurement of this class will read lower for a reason other than fewer
  writes. A close reading that figure without this sentence would mis-attribute the
  drop.
- **Consumer of the friction log** — `--emit scan-prompts` at the close stage's
  triage step, and any session running it by hand. Its ranking is advisory and
  gates nothing, which is why this whole class is invisible to the battery.
- **Consumer of the new fixture rows** — `guard-kit/bin/run-guard-tests.sh`,
  reading `cases.tsv` at every guard-suite run and comparing the decision the
  library returns against the declared one (delta 8). This is the only mechanism
  that holds the narrowing to its asymmetry: nothing else distinguishes a quoted
  from an unquoted heredoc delimiter at the grant boundary.
- **Consumer of the prepared settings diff** — the operator, out of band. Named
  because it is the reader whose absence keeps the entry alive, and therefore the
  reason delta (9)'s terminal move is a demotion (delta 6).

**No corpus is narrowed by this amendment**, so the red-condition enumeration
canon-kit/SPEC.md §The causal-completeness check point 5 binds on a narrowing does
not apply. One corpus is **widened** and its reader is named instead: the set of
calls `guard_allow` admits grows, and the reader that would catch an over-wide
grant is `cases.tsv`'s fall-through rows (delta 8), which assert that a tracked
target and an unquoted heredoc delimiter each still decline.

**The gate reader set is empty and that is the finding, not an omission.** The
friction log is advisory — guard-kit/SPEC.md §scan-prompts rules it "advisory,
triage at close, not a gate" — so **nothing reds** however far this class grows or
shrinks. That is the entry's own "invisible to every gate" cost restated at the
one place a reader might expect a gate to appear.

**Cross-component signal: this amendment's component set is one** — guard-kit
(§The generic ruleset rules 16 and 17, §scan-prompts' honest limit, and the guard
test table) — plus the queue entry. It does **not** on its own fire
`check-stage-entry` assertion C; the sibling amendment authored this session does,
and the align stamp is demanded at the next entry on that ground.

## Existing sections updated

- `guard-kit/SPEC.md §The generic ruleset`, **rule 17** — clause (d) restated as
  the narrowed test, with the `hdq` ground carried across from rule 6 rather than
  re-argued, and the unquoted-delimiter asymmetry stated as the reason the mode is
  `hdq` and not `hd` (deltas 1, 2). The redirect-operator clause is restated to
  admit the create case under the same target test, with rule 16's disposability
  ruling named as its ground (delta 4).
- `guard-kit/SPEC.md §The generic ruleset`, **rule 16** — gains the sentence that
  its truncation ruling is what rule 17's create case rests on, so the two are not
  read as independent grants of the same thing (delta 4).
- `guard-kit/SPEC.md §scan-prompts` — the honest-limit paragraph gains the
  measured consequence of the grant boundary: a granted call never reaches the log,
  so this instrument cannot size the population its neighbouring rules already
  admit, and a reading that drops after a grant widens has not measured fewer
  writes (delta 3).
<!-- update-target-exempt: its owner is the measurement preamble rather than any delta — no delta changes the first-segment rule, and the paragraph gains an attested instance of a defect another entry already owns -->
- `guard-kit/SPEC.md §scan-prompts`, the first-segment paragraph — gains this
  measurement as its second attested instance, two genuine writes having hidden
  under a leading `mkdir`; the disposition stays
  `friction-key-segment-selection-unruled`'s and is not re-ruled here.
- `guard-kit/guard-tests/cases.tsv` — four rows (delta 8).
- `TASK-QUEUE.md`, the `file-authoring-act-ungoverned` entry — moved from
  `## Deferred` to `## New Features` with `[design-pending]` swapped for this
  amendment's `[spec:]` ref and its lead line re-wrapped; it **demotes** at build
  and never reaches `## Done`, and the demoting commit compresses it against the
  cap it is already at (delta 9).
- `TRAJECTORY.md` §The closed rulings — the sentence recording
  `file-authoring-act-ungoverned` as the port-only run's second exception member
  names its own discharge, "this second member goes earlier, when that entry's
  promotion lands". This promotion is that event. The retirement is **not** a
  reversal — retiring a spent ruling is not reversing one, and the ruling schedules
  its own retirement — but it is a write to the ruling record, so it is landed on
  the lead's answer rather than on this session's reading of a scheduling clause
  (delta 9).
- The generated projections this amendment stales — the on-site `guard-kit/SPEC.md`
  mirror, and the `ROADMAP.md` block only if a `[roadmap:]` tag moves, which it
  does not. Rostered with their triggers in `docs/site-architecture.md`
  §Generated projections (all deltas — a mirror is stale the moment any of them
  lands).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone, and the
      merge holds CLAUDE.md §The provenance seam: the dated rulings and authorities
      above land in git history, never in a kit SPEC's prose.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`), the none-remain half discharged at the
      iteration rather than at the commit.
- [ ] **The narrowing was proved on the real shape, not a fixture alone** — a
      `cat >> .tmp/<journal> <<'EOF'` whose body carries a backtick is admitted with
      no permission decision, taken by running one (delta 2).
- [ ] **The asymmetry holds** — the same write with an **unquoted** delimiter and a
      backtick in the body still declines, taken by running one, because this is the
      clause a narrowing is likeliest to over-shoot (delta 2).
- [ ] **The create case grants only where every other clause holds** — a create to
      a **tracked** target declines, a create with a live substitution declines, and
      a backgrounded create declines; each taken by running one (delta 4).
- [ ] **The measured payoff was re-read after the change** — `--emit scan-prompts`
      over a fresh log shows the `cat >>` rows gone, and the reading is recorded
      **with delta (3)'s caveat attached**, so the drop is not filed as fewer writes
      (deltas 2, 3).
- [ ] **No permission-settings edit was made** — the prepared six-entry diff is
      still on the entry, unapplied, and `.claude/settings.json` is untouched by
      this work (delta 6).
- [ ] **The four fixture rows landed and the guard suite is green** — through
      `guard-kit/bin/run-guard-tests.sh`, not by reading `cases.tsv` (delta 8).
- [ ] **The promotion and the demotion both fit** — the re-wrapped lead line
      measures 94 columns, and the entry returns to Deferred with the carrier
      obligation intact and compressed against a re-measured fifty-line cap
      (delta 9).
- [ ] **The TRAJECTORY retirement was landed on a ruling, not on a reading** — the
      second-member sentence is retired only once the lead answers, and the answer
      is recorded before the edit (delta 9).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
