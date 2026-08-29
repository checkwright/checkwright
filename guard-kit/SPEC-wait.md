# SPEC amendment: the in-turn wait's grant

Rules `wait-loop-grant-lost-its-carrier`. The lead took the unit on 2026-08-29 and
deliberately left the **shape** unruled, assigning it here with an instruction —
cost all three candidates and take **the narrowest that covers the measured
class** — and two hard bounds: the deliverable is **guard-kit code**, and the
2026-08-22 operator bar on promoting a permission-settings edit stands.

The entry's history, its three recurrences and its discharged precondition are
settled and are not re-derived. What this amendment adds is the fourth
measurement the ruling ordered, two probes that falsify premises the shape choice
would otherwise have rested on, and the shape.

## The fourth datum, and the first premise it falsifies

Taken at spec, oracle `bash guard-kit/bin/scan-prompts.sh` plus a grep over the
friction log, and filed to the survey record before it was acted on. The log held
162 fall-throughs, 54 prompting calls across 18 patterns. **The wait-loop class is
four prompting calls across three ranked patterns, and not one of them is
`while kill -0 "$pid" …`** — the exact spelling the entry's cost line, rule 17's
non-target paragraph and delegation-kit's mandate are all written on. The four:

1. `until grep -q '<pattern>' <file>; do sleep 20; done; echo …`
2. `until grep -q '<pattern>' <file> 2>/dev/null; do sleep 30; done; grep … <file>`
3. `until gh run view <id> --json status --jq .status 2>/dev/null | grep -q completed; do sleep 30; done; gh run view …`
4. `while pgrep -f '<pattern>' >/dev/null 2>&1; do sleep 20; done; echo …; grep … <file>`

Per the ruling, this is **one sample and not a rate**, and the entry is not
re-priced off it.

**Why the `kill -0` spelling is absent is structural, not incidental, and it is
the probe that matters.** Rule 6 does not *prompt* on
`while kill -0 "$pid" 2>/dev/null; do sleep 5; done` — it **blocks** it, exit 2,
because rule 6 keeps `dq` live so a double-quoted expansion stays visible. And
`guard_block` exits before `guard_log_fallthrough` runs, so that spelling **cannot
appear in the friction log at all**. The entry's cost line — *"rule 6 decides the
mandated `kill -0 "$pid"` loop out of band on every call — sixty decisions"* — is
therefore **false by construction**, and the 19/60/6/8 figures are counts of
something else. Two independent confirmations: the trace, and the measured zero.
The correction lands on the entry with this promotion.

**The second falsified premise, and it is the one that decides the shape.** Every
one of the four measured calls is a **compound** — a wait loop followed by the
thing the session does once the wait ends. That is not sloppiness; it is the
mandated shape ("report only results you hold"). A grant admitting only a lone
loop statement would cover **zero** of the measured class — precisely the failure
rule 17's own non-target paragraph warns of, *"a guard on the wrong primitive
inherits its failure."*

## The three shapes, costed

**(a) Narrowing rule 6 — refused on sufficiency, before cost.** Rule 6 *blocks*;
narrowing it converts a block into a **fall-through**, which is the out-of-band
decision this unit exists to remove. It cannot deliver the grant on its own, and
it would need shape (b) beside it regardless. Its cost is also the worst of the
three: rules 12, 13, 14, 15, 21 and 22 each decline on the ground that rule 6 has
already blocked an expansion, so narrowing rule 6 re-opens holes six other rules
deliberately do not close.

**(c) A raw-command exemption — refused, and it is ambiguous three ways.** Reading
1, relaxing the **raw-command carve-out** every auto-allow rule shares (a
substitution or backtick in the *raw* command declines outright), contradicts the
safety argument the carve-out exists for — *an auto-allow may not rest on a
coverage claim that is only mostly true*. Reading 2, a new roster knob of exempt
command literals, meets rule 15's own no-knob seam refusal: a roster of this shape
is the consumer vocabulary the provenance seam keeps out of a kit. Reading 3, an
entry in the committed settings' `permissions.allow`, is **doubly out** — it is
what the 2026-08-22 operator bar forbids, *and* it is provably ineffective, since
a `Bash(…)` glob cannot match a loop condition, which is this entry's own founding
premise.

**(b) A new generic rule — TAKEN.** It is the only shape that emits a grant, and
its cost is the smallest of the three.

## What makes it landable, probed first because it gates everything

**guard-kit already has an allow path; nothing is minted and the operator bar is
not reached.** `guard_allow` emits
`{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"allow",…}}`
and exits 0 — the envelope that suppresses the prompt — and it has three live
callers already (rules 16, 17, 18), with `guard_rewrite` as its `updatedInput`
sibling. The hook's registration in the tracked settings is a `PreToolUse`/`Bash`
matcher on the guard script, and **a new rule changes no line of it**. The
deliverable is guard-kit code, as the bound requires.

## What changes

### (1) A new generic rule: auto-allow a bounded in-turn wait

A new member of the generic ruleset, placed in the auto-allow band and emitting
`guard_allow` when **every** clause below holds, falling through untouched
otherwise. Every clause is a safety argument, and `permissionDecision: allow`
blesses the *whole call*, so the rule must bound the command rather than merely
recognize the shape it keys on. {design-bearing} That is rule 17's
stated discipline and this rule inherits it.

- **(0) No statement-ending `&` anywhere in the command.** A backgrounding launch
  is rule 15's subject, and a compound that *launches* something beside its wait
  is a producer, not a waiter. This clause refuses the attested
  launch-plus-poll form on its own terms rather than by relying on an earlier
  rule's ordering — belt and braces, and the safety argument is better for not
  being an ordering accident. The rule is **indifferent to the harness's
  background flag**, which backgrounds the whole call without changing its text:
  a backgrounded wait is the mandated form and rule 15 already exempts it from the
  record advisory on the ground that a waiter is not a producer.
- **(a) The first statement is a `while`/`until` loop with a balanced
  `do … done` span**, detected with `_guard_loop_span` — the ruleset's single
  shell-keyword walk, already shared by rules 13 and 15. A second dialect is
  refused; one walk, three readers.
- **(b) The loop body contains nothing but `sleep`.** This is the rule's safety
  core. A loop body that runs anything else is unbounded work executing an
  unbounded number of times under a grant, and no clause elsewhere would bound it.
- **(c) The loop condition is read-only.** The condition runs once per iteration,
  unboundedly often, so it is held to the same test rule 18 applies to a
  pipeline: every segment leads with a `GUARD_KIT_RO_BINS` member, with the shell
  test forms (`[ … ]`, `[[ … ]]`, `test`) and `kill -0` admitted as conditions in
  their own right, and every redirect inert (`/dev/null` or an fd-dup).
- **(d) Every statement after the loop passes rule 18's read-only-pipeline
  test.** This is the clause the measurement bought: it is what makes the grant
  cover the compound the class is actually written in, and it grants nothing rule
  18 would not have granted standing alone.
- **(e) Conservative decline on anything unmodelled** — a command or process
  substitution or a backtick anywhere in the **raw** command, or a redirect target
  surviving normalization with a quote on it. The auto-allow band's shared
  carve-out, adopted unchanged rather than reasoned about afresh.

**Two forms this rule deliberately does not grant, stated so the silence is not
read as an oversight.**

- **`pgrep -f <pattern>` as a condition.** It is a *pattern match*, and the
  standing dispatch policy forbids matching a pattern where a recorded PID is
  available. One of the four measured calls uses it. Granting it would bless a
  form the methodology refuses, so it keeps prompting — which is the rule working,
  not a coverage gap.
- **`gh` as a condition.** Waiting on a CI run has a sanctioned blocking primitive
  (`gh run watch`); admitting `gh` here would both widen a roster into a network
  tool and grant a workaround for a primitive that already exists. One measured
  call uses it, and it keeps prompting for that reason.

So the rule covers two of the four measured calls, and the two it declines it
declines **on purpose and by name**. Coverage of the class is not the target;
coverage of the *sanctioned* class is.

### (2) Placement in the dispatch order, and the ordering argument

The rule dispatches **after `guard_rule_ro_pipeline` and before
`guard_rule_allowlist_chain`** — the tail of the auto-allow band.
The placement is the rule's second safety argument, and each constraint below was
checked against the decision table rather than assumed. {design-bearing}

It must sit **after rule 12** (the `pgrep`/`pkill` self-match block), or the
attested launch-plus-poll row flips from `block` to `allow` and a never-exiting
waiter is blessed; clause (0) refuses that row independently, and both holding is
the intent. **After rules 8, 9, 10 and 13**, each of which states in its own
section that it is placed before both auto-allow rules. **After rule 15**, so a
launch that owes a liveness record still meets the advisory before anything
grants it — the same partition rule 17 clause (0) draws. **Before rule 19**, which
would otherwise block a decorated loop first.

Placement at the tail of the band rather than at its head is also the cheap
choice: it renumbers four existing rules instead of eight, roughly halving the
citation sweep below.

### (3) The renumbering sweep

Every `rule <N>` citation for the shifted rules moves — a find-and-replace over a
known corpus, in descending order so no rename collides. {mechanical}

Measured at this placement: roughly thirty citations across `guard-kit/SPEC.md`,
`guard-kit/lib/guard.sh`, `guard-kit/guard-tests/cases.tsv` and
`guard-kit/bin/run-guard-tests.sh`. **Nothing outside guard-kit cites a rule
number except queue prose**, and no generated projection carries one:
`docs/enforcement.md`'s Guards rows are per **hook script**, derived from the
settings file's `PreToolUse` array, so a new rule produces no row change — and
`docs/value.md`'s rollup, which reads that page, does not move either.

**The sweep has no oracle, and that is a known, filed condition rather than a
discovery** (`guard-rule-number-intra-kit-citations-ungated`). Its recorded
precedent is the sharp one: the rule-17 amendment enumerated six sites and the
sweep found roughly five times that. So the build's instruction is to sweep by
`grep -rn 'rule 1[6-9]\|rule 2[0-3]' guard-kit/` and count, never by working from
a list this amendment supplies.

### (4) The decision table gains the rule's rows, and every existing loop row is re-derived

`guard-kit/guard-tests/cases.tsv` and `background-cases.tsv` gain rows for each
clause — the grant, and one refusal per clause so no clause can be deleted
without a red. The table *is* guard-kit's fixture pair — the kit ships no
`good/`+`bad/` directories by design — so calibrating these rows is calibrating
the gate. {design-bearing}

**The re-derivation duty is non-negotiable and non-monotone.** Every pre-existing
row whose command carries a loop has its expected verdict re-derived by running
the table, never cleared by inspection — the ruleset's own stated rule for a
change of this shape. Two rows are known to flip from `fallthrough` to `allow`
(the `until grep -q` row in each table); **one row must stay `block`** — the
launch-plus-poll row — and it is the regression this delta exists to protect.

Driven by `bash guard-kit/bin/run-guard-tests.sh`, which is an **evidence-kit
validate suite and not a gate**, so a red here surfaces at validate rather than at
commit. The build batch runs it explicitly.

### (5) Rule 13's corrective spells a form rule 6 blocks, and it moves

Rule 13's block message steers toward the sanctioned wait and spells it
`while kill -0 "$pid" 2>/dev/null; do sleep N; done`. **Rule 6 dispatches before
rule 13 and blocks that exact spelling**, so the corrective can never be followed
as written: a session that obeys rule 13 is blocked by rule 6, and the ruleset
argues with itself. The message keeps its reasoning and its polarity clause and
moves to the spelling that is actually reachable — a literal PID, which is what
the session read out of the `.run` record in the first place, and which this
amendment's rule then grants. The message is the corrective, and a corrective
naming an unreachable form is the defect being fixed here rather than a wording
preference. {design-bearing}

This is the one place where the mandate/guard mismatch below is closed *inside*
guard-kit. The rest of it is not this unit's, and is escalated rather than taken.

## Producers and consumers

**The new state is one verdict on one class of command.** No message, no field, no
file, no configuration knob — which is why this section is short and why the
`edges`-style field walk does not apply.

- **Producer, named and reachable:** the new rule function in
  `guard-kit/lib/guard.sh`, dispatched from `guard_generic_rules` (delta 2). Its
  enabling configuration is the `PreToolUse`/`Bash` hook registration that already
  exists in the tracked settings and that this amendment does not touch — so the
  producer is reachable in every deployed configuration that runs the guard at
  all, and unreachable in exactly the configurations where no rule fires.
- **Consumer, named, by what mechanism:** the harness's `PreToolUse` permission
  layer, which reads `hookSpecificOutput.permissionDecision` off the rule's stdout
  and suppresses the prompt. The mechanism is `guard_allow`'s existing envelope,
  already consumed at this transition by rules 16, 17 and 18 — this rule adds a
  fourth caller of a path with three live ones, not a new channel.
- **Second consumer, at a different transition:** `guard-kit/bin/scan-prompts.sh`,
  which ranks the friction log. A granted call **never reaches the log**, because
  `guard_allow` exits before `guard_log_fallthrough`. So the rule's effect is
  observable as a fall in this class's ranked count at the next boundary, and that
  is the measurement close should take rather than a fresh argument.

**Red conditions, named rather than subjects.** Delta 2 inserts a verdict into an
ordered dispatch, which **narrows** what reaches every later rule — so the check's
point 5 binds and each reader is enumerated by what makes it *red*, not by what it
is about:

- **The decision table** (`run-guard-tests.sh`) — reds on any row whose observed
  verdict differs from its expected column. **Non-monotone in both directions**: a
  new allow *upstream* of a later rule turns that rule's rows from `block` to
  `allow` without any of those rows changing. Not clearable by inspection; delta 4
  re-derives by running.
- **`check-template-copy-parity`** — reds on a divergence between
  `guard-kit/templates/bash-guard.sh` and this repo's `scripts/bash-guard.sh` that
  carries no divergence marker. Fires if the rule lands in one and not the other.
  Monotone; clear by inspection once both move.
- **`check-comment-tier`** — reds on a non-directive comment. The new rule's
  `# spec:` line is a one-line binding; monotone.
- **`check-docs-mirror-fresh`** — reds on a byte difference between
  `guard-kit/SPEC.md` and its `docs/` mirror. Fires on all deltas touching the
  SPEC; clears on regeneration.
- **`check-enforcement-fresh` and `check-value-rollup-fresh`** — red on a
  difference between the emitted map/rollup and the committed page. **Cleared by
  inspection, and the inspection is stated rather than assumed:** both derive
  their Guards rows from the settings file's `PreToolUse` array, one row per hook
  *script*. A rule inside a script that is already rostered changes no row, and no
  count either page carries moves.
- **`check-exec-bit`, `check-smoke-entry-guard`** — unaffected; no file is added
  or removed and no template roster changes.

## Existing sections updated

- guard-kit/SPEC.md §The generic ruleset — the new rule's own numbered entry with
  its five clauses and its two stated non-targets (delta 1); the placement
  paragraph and the four shifted rule numbers (deltas 2, 3); rule 13's corrective
  text (delta 5).
- guard-kit/SPEC.md, rule 17's **non-target paragraph** — it records the mandated
  in-turn wait as a *stated non-target* awaiting a measurement, and says
  explicitly that *"what remains is the grant itself"*. That paragraph is
  discharged by this amendment and is rewritten to cite the rule rather than to
  hold the refusal (delta 1). Its assertion that rule 6 *"decides the mandated
  spelling out of band on every call"* is the falsified premise above and does not
  survive the rewrite.
- guard-kit/SPEC.md §The generic ruleset, rule 15's exemption (2) — it names the
  backgrounded wait loop as *"the sanctioned primitive rule 13 steers toward"*.
  With a grant in place, the exemption's relationship to it is stated once rather
  than left for a reader to infer (delta 1).
- guard-kit/SPEC.md — the new rule's **honest limit**, which is inherited rather
  than new: its clause (a) predicate is `_guard_loop_span`, which is
  command-text-shaped, so a wait loop inside a script body is invisible to it
  exactly as it is to rule 15. That makes
  `wait-loop-exemption-blind-behind-a-script-name` a shared-mechanism sibling of
  this rule rather than a rule-15-only defect, and the cross-reference is owed
  here (delta 1).
- `guard-kit/lib/guard.sh` and `guard-kit/templates/bash-guard.sh` /
  `scripts/bash-guard.sh` — the rule, its dispatch line, and the copy-parity pair
  (deltas 1, 2, 5).
- `guard-kit/guard-tests/cases.tsv`, `guard-kit/guard-tests/background-cases.tsv`
  (delta 4).
- `docs/guard-kit/SPEC.md`, `docs/guard-kit/README.md` — generated mirrors, stale
  the moment any delta lands (`all deltas`); regenerated by the command
  `check-docs-mirror-fresh` prints on red.

The one target this amendment deliberately leaves alone, and why the silence is
recorded rather than left to be read as an omission: delegation-kit's agent-execution
template spells the mandated wait with a quoted PID variable in four places, and rule 6
blocks that spelling. Closing that gap means either widening rule 6 past the measured
class or editing a delegation-kit template, and each crosses a bound the lead set on
this unit — so it is escalated and filed rather than adopted on this session's authority.

<!-- update-target-exempt: closing the mandate/guard spelling mismatch crosses a bound the lead set on this unit, so it is escalated and filed rather than owned by a delta here -->
- delegation-kit/templates/agent-execution.md — **not updated by this amendment**;
  see the paragraph above and the gap filed alongside it.

## Definition of Done

- [ ] **Causal completeness** — the one new verdict has a named, reachable
      producer (the rule, dispatched from `guard_generic_rules`) and two named
      consumers at two named transitions (the harness's permission layer; the
      friction-log ranking, by absence).
- [ ] **Merged with no information lost** — the rule integrated into §The generic
      ruleset in dispatch order, rule 17's non-target paragraph rewritten rather
      than appended to, and the falsified rule-6 premise removed from it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      guard-kit (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every `rule <N>` citation for the four shifted
      rules re-derived by grep and count, never from a list; rule 13's old
      spelling gone from the corrective.
- [ ] **The decision table re-derived by running, not by inspection** — both
      tables green under `bash guard-kit/bin/run-guard-tests.sh`, with the
      launch-plus-poll row still `block`.
- [ ] **Gaps filed** — the mandate/guard spelling mismatch filed to the gap inbox
      with its two candidate closes and the bound each crosses.
