# SPEC amendment: wait-primitive-measurement

Takes the **measurement half** of `turn-end-chokepoint-and-wait-primitive` — and
only that half. Its first half closed 2026-08-22 into
delegation-kit/SPEC.md §The turn-end liveness probe, and its blocking-hook
variant is **sequenced behind** `subagent-stop-payload-background-tasks-read` by
the lead's 2026-08-22 ruling, which this amendment does not touch.

**The surviving question, in the entry's own words: the protocol states a hard
ordering and on this machine that ordering inverted.**
`delegation-kit/templates/agent-execution.md` rules `run_in_background` plus an
`until`-loop the sanctioned form for a single completion and names the harness's
event-stream form the wrong tool. Against that, four of one session's own
backgrounded waiters **died before their conditions went true, producers
verifiably alive**, while an event-stream call succeeded first try. A guard built
on the wrong primitive inherits its failure, which is why this is measured before
anything is built on it.

**One thing is already settled by probe rather than by measurement, and it
narrows the question rather than answering it.** The vendor tool reference for
the event-stream form states *"Exit ends the watch"* and confines the
stays-armed-to-its-deadline property to an **unbounded** command (`tail -f`,
`inotifywait -m`, `while true`); it then independently recommends
`run_in_background` with an `until`-loop for a single completion. So the
template's **ordering** is corroborated by the vendor's own guidance, and what is
unconditional in the template is its **characterization** of the event-stream
form, which the source states conditionally. The measurement's subject is
therefore **not** which form to prefer. It is: *why did four instances of the
sanctioned form die with their producers alive*, since a form that is right in
principle and unreliable in practice is worse than a named second choice.

## What changes

### (1) The instrument: a runnable, self-contained wait-primitive probe

delegation-kit gains `bin/wait-probe.sh`, a bin-tool-contract-conforming probe
that stands a known-duration producer up and exercises each candidate wait form
against it, recording one line per trial. **Design-bearing.**

It writes nothing in-tree: its scratch root is `DELEGATION_KIT_PROBE_TMP_DIR`,
defaulting under `GATE_SDK_TMP_DIR` (`.tmp/`), following `demo/run-demo.sh` and
`scripts/pack-installer.sh`, whose single-knob out-of-tree scratch is the shape
this repo already uses for a runnable artifact.

One trial is: launch a producer that sleeps a **declared** duration and then
writes a completion marker; record its PID at launch in a `<key>.run` file
exactly as `templates/agent-execution.md` mandates; arm the wait form under test;
and record **four** facts — whether the waiter exited, whether it exited
**before** the marker appeared, the waiter's own exit status, and the elapsed
wall-clock at each of the two events. The **before/after** relation is the whole
measurement: a waiter that outlives its condition is working, and one that exits
early is the observed failure.

**The producer's duration is swept, not fixed**, because a single duration cannot
distinguish the candidate causes. The declared sweep is short, medium and long
relative to the harness's own foreground timeout ceiling — the probe reads no
harness internals and asserts no ceiling, it varies the input and reports where
the behavior changes.

### (2) The four candidate causes are enumerated before the run, not after it

The probe's report classifies each early exit against a **closed** list of causes,
so a run returns a cause rather than an anecdote. **Design-bearing.**

- **(i) The waiter was reaped** — the harness or the session boundary killed the
  backgrounded shell. Tell: the waiter's exit status is a signal, and the
  producer is still alive at that instant.
- **(ii) The waiter hit a wall-clock ceiling** — a deadline the backgrounded form
  carries. Tell: early exits cluster at one duration and short trials never fail.
- **(iii) The condition was never expressible** — the `until` predicate went true,
  or could not go false, for a reason unrelated to the producer. Tell: the waiter
  exits **zero** with the marker absent, and it reproduces with no producer at
  all. This is the class the protocol's own `pgrep` warning already names, and the
  probe must be able to exonerate the primitive by finding it.
- **(iv) None of the above** — recorded as **unexplained** and reported as such. A
  closed list that cannot say *I do not know* would force a wrong attribution,
  which is the failure this enumeration exists to avoid.

**No cause is asserted in advance and none is favoured by the instrument.** The
probe measures both forms on identical producers in the same run, so neither
form's result rests on a comparison with a differently-shaped trial.

### (3) The finding's dispositions are ruled now, one per branch

Because the outcome is unknown, the **response** to each outcome is ruled here,
so the build session executes a decision rather than making one under measurement
pressure. **Design-bearing.**

- **The sanctioned form is reliable and the four deaths reduce to (iii).** The
  ordering stands **unchanged**; the measurement lands as evidence in
  §Operative residency and the template's *Which primitive* paragraph gains the
  observed failure mode plus the tell that identifies it, because a rule whose
  attested counter-examples are unexplained is a rule its readers will discount.
- **The sanctioned form is unreliable under a stated condition — (i) or (ii).**
  The ordering becomes **conditional** and the condition is stated: the template
  names the bound and names the fallback for a wait that exceeds it. Every reader
  in §Existing sections updated moves in the same unit, guard rule 13's corrective
  included, because a corrective naming a primitive the measurement just bounded
  is the enforcement-first defect this repo ranks above gating it.
- **The event-stream form is the reliable one for the mandated shape.** The
  ordering **inverts**, and the template's unconditional characterization of that
  form is corrected to the vendor's conditional one — the stays-armed property
  attaching to an unbounded command rather than to the form. This branch is
  stated with the others rather than argued down, and it is the branch the
  fifteenth firing's evidence points at.
- **The result is machine-specific.** Recorded as such and the ordering stands,
  with the probe retained as the reproducer a second machine runs. This is a real
  branch: `ENV.local.md` is this repo's own acknowledgement that machine-shaped
  facts exist and are not doctrine.

**Every branch lands the measurement.** None of the four ends in *no change and no
record*, which is the outcome a measurement bought and then discarded would have.

### (4) The measurement is durable before it is acted on

The run's output lands in `.workflow/wait-primitive-evidence.txt` — capture-tier,
gitignored, advisory — and declares itself on the close-surface roster with its
own reclaim path. **Mechanical.**

close-surface: .workflow/wait-primitive-evidence.txt advisory reclaim=: > .workflow/wait-primitive-evidence.txt

`advisory` rather than `forced`, on the reasoning
§The turn-end liveness probe's log already takes: nothing refuses a close that
skips it, and a visible skip is the honest mode for a probe. The **finding** — the
branch delta (3) selects and its grounds — lands in the SPEC, not in the log; the
log is evidence, never the record.

### (5) The blocking-hook variant is untouched, and the boundary is restated

Nothing here builds, designs or costs a blocking `SubagentStop` hook. **Mechanical.**

§The turn-end liveness probe already rules that a hook that blocks is a
**separate authorization**, and the lead ruled 2026-08-22 that the variant is not
sought this iteration because `subagent-stop-payload-background-tasks-read` may
collapse what such a hook should read. Restated as a delta rather than left as a
silence, because this amendment's subject is the same entry's other half and a
reader could take the promotion as reaching both.

## Producers and consumers

**`bin/wait-probe.sh`** (new bin tool).
*Producer:* invoked by hand, by the build session executing this amendment — a
runnable artifact with no trigger, exactly as `demo/run-demo.sh` and
`gate-sdk/bin/port-blockers.sh` are. It is deliberately **not** wired into the
battery: it launches processes and sleeps for its declared durations, so a gate
tier would put a multi-second sleep on every commit for a measurement bought once.
*Consumers:* the **build session**, reading the per-trial lines to select delta
(3)'s branch; and any later session on a second machine, reading the same tool as
the reproducer delta (3)'s fourth branch names.
*Enabling config:* `DELEGATION_KIT_PROBE_TMP_DIR`, defaulted in the tool itself
rather than in a library, since no gate dispatches it and it therefore crosses no
config bridge — the distinction §lib/gate.sh draws between a bridged knob and a
tool's own default.

**The per-trial line** (new record; every field has a named reader, and the field
set is closed at these).

- **`form`** — which wait form the trial armed. Read by the build session at the
  comparison, and it is the only field that distinguishes the two candidates.
- **`producer_ms`** — the producer's declared duration. Read at the sweep
  analysis, to answer cause (ii): a ceiling shows as a threshold in this field.
- **`waiter_exit`** — the waiter's exit status. Read at the classification, to
  separate cause (i) — a signal status — from cause (iii) — a clean zero with the
  marker absent.
- **`marker_at_ms`**, **`waiter_at_ms`** — elapsed wall-clock at the marker's
  appearance and at the waiter's exit. Read **together and only together**, at the
  classification: their **order** is the measurement and neither alone says
  anything.
- **`producer_alive_at_exit`** — whether the recorded PID still answered
  `kill -0` when the waiter exited. Read at the classification to establish the
  *producers verifiably alive* condition the entry reports, on the recorded PID
  and never a process-table pattern, which is the protocol's own rule and applies
  to the instrument that measures it.
- **`class`** — the cause from delta (2)'s closed list, `unexplained` included.
  Read by the build session at branch selection.

No field is carried that this list does not name a reader for; in particular the
producer's own PID is **not** logged, because the `<key>.run` record already holds
it for the lifetime the wait needs and a second copy in an advisory log would be a
PID with no reader after the trial ends.

**`.workflow/wait-primitive-evidence.txt`** (new capture-tier file).
*Producer:* `bin/wait-probe.sh`, appending one line per trial.
*Consumers:* the build session at branch selection; the close stage through the
close-surface roster, at the reclaim step, which is the reader that keeps the file
from becoming untracked-and-unignored residue.
*Red condition:* none of its own. Two existing readers do have one and both are
named because a new file under `.workflow/` is exactly what they police:
`check-workflow-tiering` reds on a member that is **neither tracked nor ignored**
— so the file must be gitignored, which the delta's roster line and the existing
`.gitignore` capture-tier pattern together satisfy — and lifecycle-kit's
close-surface reader reds on a declared surface whose reclaim path does not
resolve. Neither is monotone in a violation set and neither is cleared by
inspection; both are cleared by running them.

**Existing prose describing the prior flow.** The stated ordering has **four**
carriers in this tree, surveyed across the whole component set rather than a
hand-picked subset and with stderr unsilenced on every probe: the template's
*Which primitive* paragraph, §Operative residency's bare-imperative copy, guard
rule 13's block message, and this repo's own always-loaded agent definition. The
template itself rules that the residency copy is **sanctioned rather than drift**
and that a change here **propagates**, so delta (3)'s branches move all four or
none. They are inventoried below.

## Existing sections updated

Each names the delta that owns it.

- **delegation-kit/templates/agent-execution.md, the *Background + notification,
  never poll* bullet** — its **Which primitive** paragraph is the surface under
  measurement. The vendor-conditional correction and whichever branch delta (3)
  selects land here (deltas 1, 3). The paragraph's own instruction that a change
  here propagates to the residency copy is what makes the next entry mandatory
  rather than optional.
- **delegation-kit/SPEC.md §Operative residency** — the bare-imperative copy of
  the same rule, which the template names as sanctioned and requires to move with
  it (delta 3).
- **guard-kit/SPEC.md §The generic ruleset, rule 13** — the bare-foreground-`sleep`
  block, whose corrective **names the backgrounded condition wait and the harness's
  event-stream form by name**. Its own section already records what such a literal
  costs — portability, not privacy — and delta (3)'s second and third branches
  change which form the corrective names (delta 3).
- **guard-kit/lib/guard.sh, `guard_rule_bare_sleep`** — the block message itself,
  which is where that corrective's text lives (delta 3).
- **delegation-kit/SPEC.md §Layout and configuration** — `DELEGATION_KIT_PROBE_TMP_DIR`
  joins the knob roster with its default (delta 1).
- **delegation-kit/SPEC.md** — a new `### bin/wait-probe` section under
  §Per-component contracts, stating the tool's contract, its trial grammar, its
  closed cause list and its honest limit (deltas 1, 2).
- **lifecycle-kit/SPEC.md §The close-surface roster** — reached through the
  declaration delta (4) emits rather than by an edit here; the roster is derived
  from the declarations, so this entry exists to record that the derivation was
  checked and re-run rather than assumed (delta 4).
- **.gitignore** — the evidence file joins the capture tier, which is what keeps
  `check-workflow-tiering` green (delta 4).
- **CLAUDE.md §Agent execution** and this repo's agent definition — re-read at
  merge against delta (3)'s selected branch; the resident line is a pointer today,
  so an edit is expected only on the branch that changes the rule rather than its
  evidence (delta 3).
- **TASK-QUEUE.md** — `turn-end-chokepoint-and-wait-primitive` demotes or moves at
  the terminal step according to which half remains: the blocking variant stays
  sequenced, so the measurement's completion does not empty the entry
  (deltas 3, 5).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), discharged at the iteration
      rather than at the commit.
- [ ] **Removals propagated** — grepped every spec, template and agent definition
      for the ordering's four carriers; none states a rule the selected branch
      retired.
- [ ] **The finding is durable before it is acted on** — the trial lines are
      written and the branch is selected from them, in that order, which is the
      same rule the template binds a dispatched session to and which this unit is
      measuring the other half of.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
