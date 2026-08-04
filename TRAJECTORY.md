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

## PRIORITY DIRECTIVE — the port track's sequence

**Ruled 2026-08-03 by the operator.** The track sequences toward **per-profile
coherence, not whole-corpus completion**: one shipped profile fully native end
to end beats a partial port spread across every profile.

1. The vendoring model — how a compiled gate arrives in a consumer tree.
2. The publish-side pipeline, `native-artifact-publish-path`.
3. A first ported cohort, chosen to make **one shipped profile** fully native
   end to end.
4. `installer-lifecycle-verbs`, with the uninstall verb.
5. `prose-profile` completion — the **earliest external-install channel** under
   the re-ruled preview cohort, not post-launch polish. Which cohort that is
   stays in the operator's local brief; this file records only that the
   sequence turns on it.
6. `companion-toolkit-profile`.

`instruction-surface-bash-focus` unblocks on a threshold rather than a date,
per its own queue entry. Surge-channel launch stays gated behind the private
brief's readiness rule, and launch-comms execution runs on its own clock under
the surface that owns distribution.
