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

**How to read a ruling recorded here.** A recorded operator ruling is closed,
and a session that meets one **does not re-litigate it** — reversing, demoting
or re-scoping one is operator-class, and a finding that appears to contradict an
objective is an escalation rather than a stage-level or lead-level decision,
however well-grounded. Only the operator reopens a closed ruling.

**Retiring a spent ruling is not reversing it**, and that distinction is what
lets the completion-time contract below be a session's ordinary work rather than
an escalation. A ruling whose subject is finished directed something that has
already happened, so deleting the record decides nothing. Reversal is the other
act — making a closed ruling stop being the rule — and it stays operator-class.

**Who may record a ruling here.** A session **records** a ruling the operator
closed; it never **authors** one. The authority is the ruling's existence, not
the recording session's role — so no roster of permitted writers is maintained,
and none needs to be. A recording names the ruling's date, and where the
mechanism has a canonical home it points there rather than restating it.

**The session that may record may retire.** Recording and retirement are the two
ends of one authority, and for the same reason: neither decides anything. What
licenses a recording is the ruling's existence; what licenses a retirement is
its discharge. Neither turns on the session's role, so a roster of permitted
retirers is as unneeded as the roster of writers. What a retirement may never do
is re-decide. Reversing, demoting or re-scoping a recorded ruling stays
operator-class, unchanged. The honest limit: this is an authoring contract, not
a gate. Nothing mechanizes *the operator closed this*, and nothing should — the
alternative is a session attesting to its own consent, which is worth less than
the rule.

**Where the grounds live.** A ruling whose mechanism already has a canonical
home is registered here with a pointer to that home rather than restated —
one owner per fact, as everywhere else. What this file *owns* is what has no
other durable home: the objectives, the rulings named below without a pointer,
and the sequence.

That is the authoring-time half. The completion-time half is two triggers, and
they are the whole rule:

- **A ruling whose subject is finished is deleted outright.** Not distilled to a
  line, and not annotated as finished. Git history holds the obsolete text and
  the motivation behind it, and the cost of going there to retrieve it is the
  accepted cost.
- **A fact that has aged is corrected where it stands.** A correction is never
  appended, and a superseded sentence is never left standing beside the sentence
  that corrects it: two readings of one fact is the defect, whichever of them
  wears the label *current*.

One authoring convention follows, and it is what makes the next application
cheap: **a ruling able to name its own discharge event says so in its own
text**, so the session that meets the event deletes rather than judges.

**A retirement's blast radius is derived, never rostered**, and stating that here
is what stops each retiring session re-discovering it. No surface owns a list of
what cites this file and none should — a maintained roster would be one more copy
to stale — so a retirement greps for its own citations. Two properties make the
grep non-obvious enough to write down. `docs/` is a **generated mirror**, so every
kit-SPEC citation appears twice and only the source is editable; regenerate rather
than hand-edit the second. And a citation naming a surviving **section** still
resolves after the ruling inside it is deleted, so the grep finds the pointer
while no gate finds the staleness — the inbound half of the class, whose design
lives in `ruling-record-condition-staleness-probe`.

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
   The trial lifecycle is part of the product, not an afterthought. *The
   adopter's* set is the one this bounds — the build graph behind the binaries
   is a different set, ruled below.
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
and gate-sdk/SPEC.md §Consumer payload.

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
which is small enough to be written twice. Everything conditional belongs on
the far side of that invoke, and today most of it is not there yet — the
measurement is recorded with the tail sequence below. Two standing obligations
follow, and they bind every unit that touches the install path: **add no new
shell-only install step**, and assume no POSIX shell. Designing the Windows
half and moving the remaining conditional steps behind the invoke are both
`powershell-installer-surface`'s — one entry owns the whole bootstrap.

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

**New gates are born native by default; shell is the exception, and it needs a
stated cause — ruled 2026-08-14.** This reverses the default recorded at
gate-sdk/SPEC.md §The port-candidate criteria, where a born-native gate was "a
design ruling rather than a default", and it is the only measure taken so far
that acts on the port's *denominator* rather than its remainder: under the prior
default every gate landed while the port ran added shell the port then owed.
The mechanism — the exception criterion that makes "with cause" operable — is
**delivered** at that same section as three closed classes with a stated cause
form each, and is not restated here. The cost it was weighed against, recorded
so it is not re-argued: a `.gate`-declared member is *omitted* on a platform
`native/targets.list` carries no artifact for, and that roster is one target, so
the flip attaches that omission to **every macOS adopter** for every new gate.
The ruling does not widen the roster; the residue is costed as
`born-native-omission-accumulation`, and the roster's own widening trigger
(stated at `native/targets.list`) is unchanged.

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

The asymmetry the ruling rests on: a wrong deferral is corrected by rewriting
one line, and a wrong tag is public.

**The honest limit.** This narrows a criterion, so it can only be wrong in one
direction — an install-path defect that genuinely does reach users late will now
read as not-firing, and no gate catches that. What bounds it is the same clause
that grounds it: the ruling is stated *for* `preview`, so the flip to `stable` at
`v1.0.0` reopens it rather than inheriting it. Nothing here narrows the operator
direction trigger, which remains available for exactly the urgent case this
paragraph declines to automate.

**Objective 4 constrains the adopter's dependency set, never the crate's build
graph — ruled 2026-08-14, and it was never otherwise.** The two sets are
different things and reading them as one is what made a prohibition appear that
had never been stated: the adopter's set is what they install and can uninstall,
which is git plus pre-compiled binaries; the crate's build graph is resolved and
compiled by contributors and CI, and no consumer ever receives, resolves or
compiles it (install-time builds from vendored source are void under objectives
1, 2 and 5). So the `native/` crate is under **no** no-external-dependencies
prohibition and never was. What the ruling does **not** touch:
the dependency **bar** the crate applies to a candidate dependency is
engineering judgment owned by gate-sdk/SPEC.md, and nothing here loosens it.

**The allowlist narrows on security grounds only — ruled 2026-08-20 by the operator.** Breadth is
kept wherever it is safe, because an allow match short-circuits the permission classifier outright,
so a broad rule actively *saves* model calls rather than merely avoiding a prompt — which is the
opposite of the premise the 2026-08-13 breadth ruling was taken on. Narrowing is warranted only
where a destructive form sits inside a blanket grant: `reset --hard`, `clean`, `push --force`, and
a bare `checkout --`. The disposition pair a narrowing chooses between — narrow the glob, or record
that the breadth is intended — is owned by guard-kit/SPEC.md §compare-settings-allow and is not
restated here.

The standing declarations live in `scripts/guard-config.sh`'s `GUARD_KIT_BREADTH_DECLARED`, one
per glob with its grounds; the criterion above is what a future narrowing question is decided
against.

**A permission-settings edit is operator-class work, and scope never promotes one — ruled
2026-08-22 by the operator.** An edit to `.claude/settings.json` is not build work and is not
queue work: it is applied by the operator, out of band, or it is not applied. A stage session may
*prepare* one — derive the diff, state its security grounds, record both on the entry — and there
its remit ends.

**The grounds are that the work is not executable where the lifecycle dispatches it**, which is
stronger than a preference about who should do it. Two independent mechanisms refuse the edit: the
auto-mode classifier denies it, and a dispatched stage session's own definition forbids changing
permission settings on any agent's say-so. So an operator ruling that resolves to "edit the
allowlist" cannot be discharged by the session sent to discharge it — it is promoted, scoped,
dispatched, and then stalls at build having spent the whole pipeline to reach a wall.

**The rejected alternative is recorded because it is the tempting one.** The build session offered
to hand its prepared diff to the supervising session to apply. That was refused: a supervising
session performing an edit its child was denied is permission laundering, and it converts the
operator's permission boundary into a formality. A session that meets the refusal and stops has
behaved correctly, and re-deferring the unlandable half is the outcome, not a failure.

**No gate is permanently shell, and a spawned program is a dependency, never a port
exclusion — ruled 2026-08-23 by the operator.** The "permanently shell" class (gate-sdk/SPEC.md
§The port-candidate criteria, exception class (a)) and the "held behind a sub-project" reading of
criterion 7 are both retired, with their refutations recorded where each stood: the shell auditor
already trusts the binary's `--list`, absence and staleness of the binary are the fail-closed
contract's and `check-gate-binary-fresh`'s to catch, and a wrapper that spawns cargo, shellcheck or
a renderer moves the dependency floor not at all. Every shell gate ported, and the registry oracle
reads none owed over the battery it walks.
**What "port complete" means is ruled with it**: the battery runs from the hook to the binary with
no bash in between, the install bootstrap is the shim ruled by `installer/README.md` §The install
boundary, written twice (`powershell-installer-surface`), and every remaining non-test `.sh`
outside it either carries a
stated `no-port` cause or is deleted. The adopter-facing
residue is the bootstrap alone: the generated pre-commit hook shim needs no PowerShell twin, because
git runs hooks through the `sh` Git for Windows ships, so a two-line shim invoking the binary is one
implementation on every platform. Contributor-side tooling and the test harness ship to no adopter
and port opportunistically, never as a gate on the claim.

**That predicate has an oracle, and it is not the one that measures the battery.**
`bash gate-sdk/bin/port-blockers.sh --tree` reports the port disposition of every tracked non-test
`.sh`, and its trailer's owed count reaching zero *is* the sentence above: at zero, every remaining
script either carries a stated cause or is gone. The registry arms beside it walk `gates.list` and
answer for the gate battery alone — so a session reading zero owed **there** is reading a finished
battery and not a finished port, which is the misreading this predicate invited for as long as it
had no evaluator of its own. Mechanism and bounds: gate-sdk/SPEC.md §port-blockers.

**The deferred pool's exits are close's to take — ruled 2026-08-23 by the operator, on a measured
8:1 intake-to-exit ratio.** Three things change, each owned where its mechanism lives: the icebox
age floor drops to seven days (`scripts/queue-config.sh`); a finding whose cost opens in the low
class and names no live trigger may be filed **directly** as a one-line icebox entry, and a trigger
that is launch-gated counts as dormant rather than live (queue-kit/SPEC.md §The icebox tier); and
close's existing wontfix disposition — the bare `## Done` line — is the ordinary exit for an entry
a landed unit or a closed ruling has mooted, taken by close under those two criteria rather than
escalated. The icebox compresses and the wontfix line retires; neither needs a new section. What
stays operator-class is reversing a `[roadmap:]`-tagged entry, which no drain may retire.

**`check-crate-arms` stays whole and caches — ruled 2026-08-23.** The alternative, clippy at
commit and tests at validate, was refused: CI runs the battery and the fixture suites, not
validate, so the narrowing would have moved the test arm out of CI's reach. The source-stamp
cache is owned at gate-sdk/SPEC.md §check-crate-arms.

**A native Windows CI leg joins `platform-support-ci-matrix`, ordered ahead of the macOS leg —
ruled 2026-08-26 by the operator.** This reopens the 2026-08-25 narrowing of that entry's
platform set to macOS alone, and the ground is a named adopter rather than a plan: a native
Windows project is ready to adopt, the first member of the population objective 2 was ruled for.
The roster rule is untouched — a Windows triple joins `native/targets.list` only once that leg
has produced and exercised its artifact — which is exactly why the leg is what is ordered. The
alternative refused: leaving native Windows to WSL, which serves the adopter today and is the
interim path until the leg is green, but reaches no evaluator objective 2 names.

**The two bootstraps are hand-kept, held in parity by the per-platform install-smoke legs —
ruled 2026-08-26 by the operator**, closing `powershell-installer-surface`'s fork 2. The
mechanism and the refused generated-twins alternative are owned at `installer/README.md`
§The install boundary.

## PRIORITY DIRECTIVE — the port track's sequence

**Ruled 2026-08-09 by the operator, and it is the track's top priority: complete
the native gate port, ASAP.** Everything portable ports — the gates, the runners
and the install-lifecycle scripts alike. Surviving shell is **residue justified
case by case, never a protected category**, so "it is orchestration" is not by
itself an answer for a script that stays.

**Two grounds, and the second is the stronger one.** First, permanent dual
maintenance: two spellings of every gate is a cost with no end date, and a
cross-substrate parity gate earns its keep during a *transition* while as a
steady state it only protects a duplicate that should not exist. Second,
**cross-platform cost** — a shell dependency means Windows support is bought by
maintaining PowerShell duplicates, the same dual-maintenance tax paid again
along a different axis, where a native binary is one implementation across every
platform. The second ground is load-bearing precisely because it does not depend
on any timing measurement being right: the port's wall-clock win is small, and
`native-gate-port-remaining-corpus` records it measured rather than assumed.

**The known irreducible, recorded so it is not mistaken for non-compliance.**
Something has to run before the binary exists on the machine, and fetching and
executing the first artifact cannot itself be that artifact. So the **bootstrap**
— in shell, or absorbed by the transport, since the payload already ships as an
npm package and a Release tarball and npm can carry per-platform binaries — is
the one place a shell dependency may be unavoidable in the strict sense. This
records the residue; designing it is the unit's work, not this file's.

**This supersedes the 2026-08-03/06 per-profile-coherence sequencing rule for the
port's scope.** What that rule refused was a *roster-census* justification; the
case that carries here is different in kind, the elimination of a permanent
duplicate. **No port-candidate criterion survives as an eligibility gate**
(gate-sdk/SPEC.md §The port-candidate criteria): the problems those criteria
name are engineering work the port owes, not exclusions it may take.

**Both install paths ship, and the disclosure boundary is untouched.** An adopter
takes a **pre-compiled binary**, or builds from source — where *from source*
means a developer clones the **public repository** and builds it there. Source
does not enter the installer payload. Objective 3 therefore stands, so does the
closed ruling below it voiding a build from vendored crate source at install
time, and so does `check-gate-substrate-parity` assertion E, which structurally
refuses an implementation source inside a vendoring kit root. **Operator-answered
2026-08-09**, against the reading that the second path reopens any of the three:
it reopens none, because what a developer builds from is the artifact that is
public already. Recorded because the question is worth asking once and expensive
to ask twice — the two paths sound like a disclosure change and are not one.

**The port's tail — ruled 2026-08-23.** This names the **sequence** `battery-runner-port`,
`shell-gate-tail-port`, then the bootstrap, and nothing else: the first two landed, so the
sequence's remainder is **one** member, `powershell-installer-surface`, which owns the Windows
half and the relocation of every conditional install step behind the invoke. Its honest size,
measured 2026-08-24: the "resolve the platform, place the binary, invoke it" shape is roughly
eighty lines of `installer/lib/init.sh`, and the roughly three hundred and fifty beside it —
kit-source vendoring, manifest and lock I/O, registry and queue seeding, the commit flow — are
conditional install logic not yet behind the invoke. After the port: `companion-toolkit-profile`.

**A sequence remainder of one is not a finished port, and reading it as one is the misreading
this paragraph exists to stop — operator-ruled 2026-08-25.** The completion predicate is the one
§The closed rulings names, `port-blockers.sh --tree`'s owed count reaching zero, and it stands
over the whole tracked shell tree rather than over this sequence. Both readings are correct and
neither absorbs the other: the sequence is what is ordered next, the owed count is what says
*done*. No figure is carried here, because a count restated is a count that ages — run the
oracle. The corpus behind that count and its decomposition are
`native-gate-port-remaining-corpus`'s, which is why the sequence's last member being blocked
never means the port has nothing to do.

`instruction-surface-bash-focus` unblocks on a threshold rather than a date,
per its own queue entry. Surge-channel launch stays gated behind the private
brief's readiness rule, and launch-comms execution runs on its own clock under
the surface that owns distribution.
