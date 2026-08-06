# TRAJECTORY.md — where Checkwright is going, and what is already ruled

This file is the project's **ruling record**: the objectives the work aims at,
the decisions the operator has closed on the way there, and the sequence the
port track runs in. It is hand-authored and answers *toward what, and under
which closed rulings*.

**It is not [ROADMAP.md](ROADMAP.md), and the two must not be merged.**
ROADMAP is a *generated* projection of `TASK-QUEUE.md`'s curated `[roadmap:]`
tags and answers *what is next*; its marker block is overwritten by its
generator, so hand-authored content placed inside it is destroyed. Read them
together: the objectives here are what the queue's ordering is trying to buy.

**How to read a ruling recorded here.** A recorded operator ruling is closed.
A session that meets one **re-verifies facts against the tree and does not
re-litigate the ruling** — reversing, demoting or re-scoping one is
operator-class, and a finding that appears to contradict an objective is an
escalation rather than a stage-level or lead-level decision, however
well-grounded. Only the operator reopens a closed ruling.

**Who may record a ruling here.** A session **records** a ruling the operator
closed; it never **authors** one. The authority is the ruling's existence, not
the recording session's role — so no roster of permitted writers is maintained,
and none needs to be. A recording names the ruling's date, and where the
mechanism has a canonical home it points there rather than restating it.
Reversing, demoting or re-scoping a recorded ruling stays operator-class,
unchanged. The honest limit: this is an authoring contract, not a gate. Nothing
mechanizes *the operator closed this*, and nothing should — the alternative is a
session attesting to its own consent, which is worth less than the rule.

**Where the grounds live.** A ruling whose mechanism already has a canonical
home is registered here with a pointer to that home rather than restated —
one owner per fact, as everywhere else. What this file *owns* is what has no
other durable home: the objectives, the rulings named below without a pointer,
and the sequence.

## The objectives

**Ruled 2026-08-03 by the operator as a project-trajectory pivot.** This
section is the durable record of it, and it is the ground every ruling below
stands on; a session weighing an alternative weighs it against these, not
against the constraint set any particular component was built under.

The pivot: **port the battery to native binaries, and reduce what a consumer
must have to git alone.**

1. **The dependency floor collapses to git.** What a consumer needs today —
   bash, awk, the GNU userland — stops being acceptable. git stays, shelled out
   rather than embedded.
2. **All major operating systems, Windows included.** A bash-only install path
   fails native Windows, so it fails the objective.
3. **Opacity is a goal, not a side effect.** Withholding a gate's
   implementation source is wanted: it favours *execution* of a gate over
   *analysis* of it by the coding agents the gate exists to hold. This reverses
   the direction the gate-dispatch seam was argued under, where opacity was
   explicitly not claimed.
4. **Footprint is a first-class cost.** An adopter installs, tries, and
   uninstalls without growing their managed code base or their dependency set.
   The trial lifecycle is part of the product, not an afterthought.
5. **Non-technical adopters are a design constraint.** A prose-profile consumer
   will not install a toolchain and may not have one. Any step that assumes a
   developer machine excludes them.
6. **The script-interpreter surface shrinks to the unavoidable.** Where an
   interpreter is genuinely unavoidable it must be dual-implementable — bash
   for Linux and macOS, PowerShell for Windows — and everything else moves into
   the binary.

Objective 3 is the one that reverses a prior ruling rather than adding to it.
Objectives 1, 2 and 5 are jointly what voids building from vendored source at
install time.

**What the objectives are not.** They are the direction, not a claim about the
tree. No user-facing surface may state the dependency floor they aim at as
though it were reached — a requirements page claiming git-only today is false,
and the front door is where a false claim costs the most.

## The closed rulings

**The substrate language is Rust — ruled 2026-08-02, final.** The alternative
weighed and refused is Go. The refusal grounds, and why binary size is not a
matter of taste under objective 4, are recorded at
gate-sdk/SPEC.md §The decisions this substrate already closed, which owns them
because the component that depends on them must be readable alone.

**A bash portability floor was costed and rejected — closed, not deferred.**
What was costed, what the blast radius was, and why the floor cannot deliver
the reach it would be bought for are recorded in the same section. The figures
there are a dated measurement rather than a live claim: a recount is a step
toward re-deciding.

**git is the sole runtime dependency, and it is shelled out rather than
embedded.** This is the concrete content of objective 1 and the claim that
must not weaken into a statement about the *build*. "Buildable from source,
needing only git" is a different and weaker claim, about what a contributor
needs; the ruling is about what a consumer's machine must carry at run time.

**The payload ships a prebuilt gate binary, selected by platform. The consumer
builds nothing, installs no toolchain, and receives no gate implementation
source.** The condition this satisfies and the mechanism it satisfies it by are
owned by gate-sdk/SPEC.md §Porting a gate to the binary substrate (criterion 5)
and gate-sdk/SPEC.md §Consumer payload. `native-artifact-publish-path` produces
and publishes the artifacts; `native-artifact-install-path` places and verifies
them.

**Building from vendored crate source at install time is void.** It was ruled
once and is recorded here rather than deleted, so the next session reaching for
the cheap answer finds it already costed: it adds a Rust toolchain to the
dependency floor objective 1 is collapsing, it is unreachable for the
non-technical adopter objective 5 admits, and it ships the very source
objective 3 wants withheld.

**The interpreter policy.** Something outside the binary must run first,
because the binary cannot select itself — that bootstrap is the irreducible
interpreter surface. Objective 6 binds its *shape* rather than its existence:
its whole job is resolve the platform, place the matching binary, invoke it,
which is small enough to be written twice. Everything conditional lives on the
far side of that invoke. Two standing obligations follow, and they bind every
unit that touches the install path: **add no new shell-only install step**, and
assume no POSIX shell. Designing the Windows half is
`powershell-installer-surface`'s work; moving the remaining shell steps behind
the invoke is `install-step-relocation`'s.

**Opacity is taken on deliberately, and it carries an obligation.** A consumer
who cannot read the gate has only the publisher's word for what it does, so the
integrity story stops being garnish and becomes the whole of what replaced
reading the source. *Ruled: ship the achievable floor and claim nothing beyond
it.* The floor is a published per-target digest verified before the artifact is
written. A genuinely reproducible build is a larger program and the queue holds
its ground as `tarball-build-attestation`; the pivot changes what that entry is
worth rather than what it says. The bound this puts on prose is exact and is
not a hedge to be softened later: a governed surface may say *verified against
a published digest* and may **not** say *reproducible*. What opacity buys, what
it does not extend to, and why the claim is raised cost of analysis rather than
confidentiality are owned by gate-sdk/SPEC.md §Consumer payload.

**The first tag that publishes binaries is the operator's call, not the
cadence's — ruled 2026-08-04.** Every close already dispositions the release
boundary against its criteria and may defer; none of that machinery changes.
What is added is narrow and one-shot: **the close that would cut the first tag
carrying prebuilt gate binaries as Release assets escalates to the operator
before tagging, whatever the criteria returned** — the escalation is owed on a
*release* verdict as much as on a deferral, because a cadence trigger firing is
not consent. Two things make this a ruling rather than a preference. The
elapsed-time trigger is a timing device and was never meant to decide whether
the project starts shipping executables, so leaving the two coupled means a date
decides it. And the coupling is invisible until the day it fires: a disposition
line that merely *names* the situation is read by a session under no obligation
to stop. The ruling is spent once that tag is cut; ordinary cadence governs
every tag after it.

**That tag is cut at `native-first-port-cohort`'s close — ruled 2026-08-06.**
The escalation the ruling above owes the operator is **answered in advance**:
consent is on record here, so the close that cuts this tag owes no second ask.
That ruling still spends itself at the tag, exactly as it says.

What this settles beyond timing is the first cohort's shape.
`native-gate-binary-port` lands its **implementations** and **holds its `.gate`
descriptors** for a follow-up unit gated on the tag. The ground is measured
rather than argued: a vendored descriptor with no binary behind it takes down a
freshly installed consumer's battery — `gate-sdk/bin/run-consumer-smoke.sh` reds
on it, as it did at the reverted slice-1 port — which gate-sdk/SPEC.md §Consumer
smoke already rules the deliberate outcome rather than a defect, and criterion 5
of §The port-candidate criteria states that no adopter can reach a prebuilt
binary until this tag is cut. **This is not the deferral the PRIORITY DIRECTIVE
below refuses.** That refusal is of deferring the *track*, and the track
advances: both cohort members' rules ship now as compiled subcommands, proved
byte-identical against the shell gates they replace. What waits is one two-line
declaration per member, on a scheduled tag rather than on a further ruling.

**`init-claim-stickiness` enters through the next scope's standing directive —
ruled 2026-08-04.** It is neither left to win an undirected survey nor filed
straight into an active section. The defect: `init`'s non-destructive re-run
guarantee survives exactly one upgrade and then inverts, because the
adopter-edited file drops out of the new manifest's tracked roster, so the
*next* upgrade overwrites it with no changed-file report at all. Reproduced end
to end across three packed versions at build, in the shipped activation surface,
and the only reason it is not already live is that no adopter has upgraded twice
— the exposure grows per release rather than per day. **The sequencing is part
of the ruling: the spec pass on installer/README.md §The manifest comes before
any code.** The candidate fix changes what a recorded hash *means* for a file
`init` did not write that run, which is a contract change rather than a repair,
and settling it in code first would be settling it by implementation.
**Discharged 2026-08-05.** The scope this named ran under an undirected directive
that did not carry the unit; the operator ruled the silence a relay gap rather
than a re-disposition, and the unit entered as the `install-claim-contract`
iteration. The ruling is spent — a later scope owes it nothing.

**The release policy's security-or-supply-chain trigger reads narrowly — ruled
2026-08-05.** It fires on a vulnerability or a compromised artifact. An
**install-path data-loss repair does not fire it**, however severe, and however
plainly the cost lands on whoever upgrades before it ships.
`install-claim-contract` is the case that settles it: a bare `init` at the same
version, with no `--force`, silently reverted the config template of every kit
that ships one, gate-sdk's `msg-patterns.list`, and a declared doctrine trim —
and the criterion
still does not fire. The reason is the channel. `docs/install.md` §Versioning
declares it `preview` with an audience it describes as internal iteration, so
*reaching users late* — the entire cost that trigger exists to avoid — has no
measured population to land on.

Two things make this a ruling rather than a preference. It was reached twice and
settled neither time: the `ruling-capture-contracts` close raised it, routed it
to the operator, and received no direction; the `install-claim-contract` close
reached the identical question on a broader instance of the same defect class. A
question two closes have spent real judgment on and neither could close will
re-fire on the next install-path integrity fix, and each firing spends that
judgment again for an outcome the cadence floor usually reaches anyway. Second,
the asymmetry the deferral rests on is worth recording once: a wrong deferral is
corrected by rewriting one line, and a wrong tag is public.

**The honest limit.** This narrows a criterion, so it can only be wrong in one
direction — an install-path defect that genuinely does reach users late will now
read as not-firing, and no gate catches that. What bounds it is the same clause
that grounds it: the ruling is stated *for* `preview`, so the flip to `stable` at
`v1.0.0` reopens it rather than inheriting it. Nothing here narrows the operator
direction trigger, which remains available for exactly the urgent case this
paragraph declines to automate.

**The next iteration's subject is delegation burn, and the meter grows to see
it — ruled 2026-08-06 by the operator.** Two halves, ruled together. The
iteration takes **token waste as its subject rather than any one filed entry**,
sub-agent burn included; and `bin/stage-economics.sh` is extended to price the
fan-out subtree, so the reduction is measured rather than asserted.

The measurement that prompted it is recorded where it belongs — in the queue
entries it grounds, never restated here: `fork-dispatch-prohibition` (a
prohibition the operator ruled and designed 2026-08-03, never armed, now on its
third firing with the cost priced), `stage-fanout-burn-unbilled` (the meter
prices stamped stage sessions only, so a stage's fan-out bills to no row), and
`cross-stage-census-duplication` (consecutive stages buying one roster twice).
All three were filed 2026-08-06 from an operator consultation that priced the
last eight iterations' transcripts.

**The two halves are ordered by the second, and that ordering is a consequence
rather than a second ruling.** A waste-reduction iteration that cannot price the
tier it targets can only assert its result, and the fan-out tier is exactly
where close-over-close variance now sits — so the meter extension is what makes
the rest falsifiable. Scope still owns the cut.

**The honest limit.** *All* token waste sources is a direction, not a scope. The
unbounded reading is unsatisfiable in one iteration and no gate bounds it; what
bounds it is scope's ordinary sizing against the measured entries above, and a
source found but not cut is a costed Deferred entry like any other.

**The cut is five units — ruled 2026-08-06 by the operator at the
`delegation-burn-reduction` scope.** The ruling above left this slot open —
*Scope still owns the cut* — and this records what filled it, in build order:

1. `stage-fanout-burn-unbilled`, first, by the ordering the ruling above fixed.
2. `fork-dispatch-prohibition` and 3. `subagent-parent-addressing`, as one batch:
   both are enforced at the single PreToolUse dispatch chokepoint, so one hook
   edit and one doctrine-template edit carry two doctrines.
4. `cross-stage-census-duplication`.
5. `read-only-fanout-unenforceable` — **the operator's own extension, taken over
   scope's recommendation to leave it deferred.**

Point 5 is the half worth recording, because it crosses the theme this ruling
set. That entry's cost is a correctness cost — an unreviewed fan-out commit on
the shared branch — not a burn cost, which is why scope proposed against it. What
bought it was marginal cost rather than theme: it is a further doctrine on a hook
the batch above opens regardless, and the surface is open exactly once. A later
session reading the theme alone would find unit 5 anomalous and re-derive the
argument; it is written down here instead.

All five are features by the new-names litmus, so `spec` authors their amendments
and pairs them in — scope promoted none of them.

## PRIORITY DIRECTIVE — the port track's sequence

**Ruled 2026-08-03 by the operator.** The track sequences toward **per-profile
coherence, not whole-corpus completion**: one profile coherent at the adopter's
floor — install, get value, uninstall — beats a partial port spread across every
profile.

1. The vendoring model — how a compiled gate arrives in a consumer tree.
2. The publish-side pipeline, `native-artifact-publish-path`.
3. A first ported cohort, chosen by the port-candidate criteria
   (gate-sdk/SPEC.md §The port-candidate criteria) **wherever the qualifying
   gates live — never to complete a profile**. Re-ruled 2026-08-06; below.
4. `installer-lifecycle-verbs`, with the uninstall verb.
5. `prose-profile` completion — the **earliest external-install channel** under
   the re-ruled preview cohort, not post-launch polish. Which cohort that is
   stays in the operator's local brief; this file records only that the
   sequence turns on it.
6. `companion-toolkit-profile`.

**Step 3 is re-ruled 2026-08-06 by the operator: the first cohort is selected by
the criteria, not by a profile.** The 2026-08-03 ruling itself — *per-profile
coherence, not whole-corpus completion* — survives intact; what the same ruling
settles on 2026-08-06 is **where that coherence is measured: at the adopter's
floor — install, get value, uninstall, on the dependency floor objective 1
names — and never at a roster substrate census.** The preamble above illustrates
the floor reading, which is why it must not be read as a roster census.

The grounds are structural, not a matter of appetite. While criterion 4 stands,
**no** profile can be fully native by census, first cohort or last: every
profile carries gate-sdk (`installer/profiles.list` — "gate-sdk is forced in",
and `full` derives every kit root), its roster is meta-gate-dominated, and a
consumer's own gates keep the shell hatch, so the meta-gates' subject stays
shell indefinitely. `starter` is the case that shows it — gate-sdk alone, whose
zero-config roster (`installer/lib/common/recipe.sh`) held exactly one gate
clearing all six criteria when it was counted at the ruling. A step 3 that had
to complete a profile was therefore unsatisfiable
as written, and reading the endpoint as a census would re-import that one rung
later.

Two alternatives were refused, and the difference between them is the part worth
keeping. **Relaxing criterion 4 is refused for the first cohort only**, with a
named re-entry condition: the criterion-clearing corpus exhausted *and* the
parity oracle held off the shell substrate. That is a sequencing refusal rather
than a permanent one — criterion 4 is what stops the parity proof being
self-referential, and the first cohort is the worst moment to weaken the oracle.
**Deferring the track until the old clause could be met is refused outright:** a
second port waits on no build and no further ruling (gate-sdk/SPEC.md §What is
retained, and where the second port stands), and the publication control such
a deferral would buy already exists as the operator-gated first tag ruled
2026-08-04 above.

`instruction-surface-bash-focus` unblocks on a threshold rather than a date,
per its own queue entry. Surge-channel launch stays gated behind the private
brief's readiness rule, and launch-comms execution runs on its own clock under
the surface that owns distribution.
