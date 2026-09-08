# TRAJECTORY.md — where Checkwright is going, and what is already ruled

This file is the project's **ruling record**: the objectives the work aims at,
the decisions the operator has closed on the way there, and the sequence the
port track runs in. It is hand-authored and answers *toward what, and under
which closed rulings*. Close reads it for aged facts and fired discharge
conditions (lifecycle-kit/SPEC.md §The close-surface roster):

close-surface: TRAJECTORY.md advisory

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

**A figure recorded here is a dated record of what a decision was weighed
against, never a live claim.** The same arithmetic in a governed spec is repaired
by de-literalization; the figure here is corrected under bounded operator
authority instead. Different kinds, different repairs — one commit may owe both.

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

**The third act is correction, and the line between it and reversal is the one
the rules above leave undrawn** — ruled 2026-09-06 by the operator, relayed
by the lead session: "/close can correct invalid or remove obsolete data from
TRAJECTORY". Three acts, named and separated:

- **Correcting an aged fact** — a slug the queue has retired, a count the tree
  has moved, a pointer whose target does not carry the claim, a condition
  naming a unit or a gate that does not exist. The fact is wrong *about the
  world*, so correcting it decides nothing, and the completion-time contract
  below already directs that it be corrected where it stands. **Session-class,
  including inside the text of a ruling that is still in force.**
- **Retiring a spent ruling** — it directed something that has already happened,
  so deleting the record decides nothing. **Session-class**, as above.
- **Reversing, demoting or re-scoping** — making a closed ruling stop being the
  rule, narrowing what it reaches, or moving it down a priority order.
  **Operator-class, unchanged**, however well-grounded the finding and however
  urgent the fix.

**What makes the first act safe is that it is decidable against the tree, and
that is the test rather than the category.** A correction qualifies only where the
aged fact is one an oracle settles — a slug's presence in the live set, a count
re-derivable by a command, a pointer's target. A "fact" that is really a judgment
about whether the ruling still ought to govern is a reversal wearing a
correction's clothes, and the sentence separating them is the one worth writing:
**if the repair would change what the ruling directs, it is not a repair.** The
honest limit is the one stated above, unchanged — an authoring contract, not a
gate. What the boundary buys is that a session meeting an aged fact inside a live
ruling now has an answer other than stopping, and a session tempted to re-decide
still has to write down that it is only correcting.

**The fourth act is annotation, and it is the one the three above leave
unnamed — ruled 2026-09-07 by the operator, answered in a lead session and
relayed by the lead.** Where a closed ruling's stated **ground** turns out to
have been false when the ruling was taken, and the ruling's **conclusion is
unchanged**, the ground is annotated beside the ruling and the ruling's own text
is left standing. **Session-class**, on the same test as correction: it decides
nothing, since what the ruling directs does not move. Both alternatives were
weighed and refused. Repairing the ground where it stands rewrites a closed
ruling and hides that the ground was ever wrong — which is what a later reader
judging that ruling needs most. Leaving it unannotated is refused too, because
the next composing session re-derives the same finding and re-escalates a
question already answered.

**Why this does not contradict the never-append rule below, and the price it
accepts.** That rule governs a fact that has **aged**: the world moved, the old
reading is simply wrong now, and correcting it in place loses nothing. A ground
that was wrong at its writing is the other case, and correcting it in place
loses the only record that it was. The price is real and is the one that rule
names — two readings of one fact, which is why this act is bounded to a ground
whose conclusion survives it. A ruling whose **conclusion** does not survive is
not annotated: that is reversal, and the bullet above owns it.

**Where the grounds live.** A ruling whose mechanism already has a canonical
home is registered here with a pointer to that home rather than restated —
one owner per fact, as everywhere else. What this file *owns* is what has no
other durable home: the objectives, the rulings named below without a pointer,
and the sequence.

That is the authoring-time half. The completion-time half is two triggers — but
a ruling is tested by **mood** before it is tested by completion, and taking
those in the wrong order is what makes the newest rulings the most dangerous to
judge. A ruling that **directed work** is spent when the work lands. A ruling
that **establishes a rule** is never spent while the rule governs, however
completely the mechanism that prompted it has shipped — and such a ruling looks
retirable under a naive subject-finished read at exactly the moment its landing
commits make a retirement sweep due. The mood question costs one question and is
the one a completion test cannot ask itself.

**Relocation is a third disposition beside delete and keep.** Where a
rule-establishing ruling's content belongs when its home changes, it is retired
from here and its content **relocates** to the surface that owns the mechanism —
which is not what the completion trigger below describes, and naming it is what
stops a later session reading a relocation as a loss.

The two triggers, and they are the whole rule:

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

**Two body-line declarations make that convention resolvable rather than merely
followed.** Both are declarations and not tags — a tag marks a move across a
pending/ready boundary and these mark none — and both are hand-written by the
session recording or retiring the ruling, at the moment it writes it, on the
self-naming discipline every body-line declaration inherits:

- `ruling: <name>[ <name>…]` — the ruling's own proper noun, in the words other
  surfaces use for it, plus each variant a surface has actually used. **One or
  more, appended and never rewritten.** The plural is not generosity: the largest
  measured cohort of stale citations shared exactly one invariant, the ruling's
  proper noun in a present-tense claim anchored to no file, and one site in that
  same cohort survived its own repair by writing the noun a single word
  differently. A single declared name is insufficient by construction.
- `discharge: <name> <oracle>` — for a ruling conditioned on a future event: the
  ruling's declared name and a **command** whose output settles whether the event
  has fired, never a predicate a reader interprets. A non-zero exit, or a clean
  run printing nothing, means the condition has **not** fired and the ruling
  stands; a clean run that prints means it **has**, and the ruling becomes a
  retirement candidate to judge under the mood test above. A condition no command
  can settle takes the literal operand `manual` followed by the prose condition,
  which reports as owed to judgment rather than being silently skipped.

**Existing rulings are not back-filled and no gate demands a declaration** — a
scanner cannot demand one written under judgment. A ruling carrying none is
simply one the citing-side report cannot reach, and the probe reports that hole
instead of inheriting it (lifecycle-kit/SPEC.md §The ruling-staleness probe).

**A retirement's blast radius is derived, never rostered**, and stating that here
is what stops each retiring session re-discovering it. No surface owns a list of
what cites this file and none should — a maintained roster would be one more copy
to stale — so a retirement greps for its own citations, and the declared names
above are what that grep is run over. Two properties make it non-obvious enough
to write down. `docs/` is a **generated mirror**, so every
kit-SPEC citation appears twice and only the source is editable; regenerate rather
than hand-edit the second. And the citation shape to grep for is **not** the one
the pointer suggests: a citation naming a surviving **section** still resolves
after the ruling inside it is deleted, so the grep finds the pointer while no gate
finds the staleness — but measured against a real cohort that shape is the
minority, and the majority name no file at all and carry only the ruling's proper
noun. The design of both halves lives at lifecycle-kit/SPEC.md §The
ruling-staleness probe.

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
shell-only install step**, and assume no POSIX shell. The Windows half shipped
2026-09-07; moving the remaining conditional steps behind the invoke is still
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
**delivered** at that same section as closed classes with a stated cause
form each, and is not restated here. The cost it was weighed against, recorded
so it is not re-argued: a `.gate`-declared member is *omitted* on a platform
`native/targets.list` carries no artifact for, so the flip attaches that
omission to every new gate for whatever that roster does not carry.
**It was weighed at every macOS adopter, the roster then being one target** —
that is the figure the 2026-08-14 decision rested on, and it does not move. What
the omission attaches to at any later moment is **that roster's complement**,
which narrows as the roster widens: stated as the complement rather than as a
fresh count, so this record keeps tracking the weighing instead of owing a
correction on every join. The verdict, its grounds and its date stand, the ruling
got cheaper than it was costed at rather than weaker, and recording the cost
still does the thing it was recorded to do.
The ruling does not widen the roster; that residue was costed as a queue entry
and is now **discharged** by a landed gate (2026-09-08), and the roster's own
widening trigger — the produced-and-exercised predicate stated at
`native/targets.list` — is unchanged **by this ruling**.
**This restatement is the `operator`'s, 2026-09-08, asked and answered in a
`/lead` session and lead-relayed**, and its extent is the cost sentence's *form*
and nothing else — no verdict, no ground, no date. It is recorded here rather
than in a commit message so that a reader of this paragraph can see what was
authorized without reading history.

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
implementation on every platform. Contributor-side tooling and the test harness port behind
adopter-facing work, sequenced and never blocking a cut — a priority statement that subtracts
nothing from the owed column (the 2026-08-28 ruling below). Kit `smoke/` suites and kit-resident
test runners ride the installer payload with their kit roots and land committed in adopter trees,
so they are kit mechanism on the claim like any owed file; the residue genuinely shipping to no
adopter — `installer/consumer-smoke/`, the declared `scripts/` class — takes a per-file
disposition when reached. That disposition is a **disjunction, not a default**: its branches are a
declaration and a port, and the adoption walkthrough took the porting one (2026-09-06) where
`installer/consumer-smoke/run-smoke.sh` took the declaring one.

**That predicate has an oracle, and it is not the one that measures the battery.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` reports the port disposition of every tracked non-test
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

**`scripts/`'s `# no-port:` class generalises on its vocabulary half alone — ruled 2026-08-24 by
the operator**, and recorded here at the 2026-08-31 drain because the causes that cite this ruling
by date had only a commit message to resolve against. `scripts/measured-claims.sh`'s cause has two
halves. The **vocabulary** half generalises: a file holding this repo's private claim vocabulary —
the content classes CLAUDE.md §The provenance seam names — declares, and a mechanism file stays
owed. The half about `scripts/` riding no installer payload deliberately does **not**, because the
2026-08-14 ruling above accepted the payload cost for mechanism. Membership is re-derived per file
against the seam's classes rather than swept by filename, and the ruling's own caution is the
asymmetry: over-declaring excuses a file from the port and mis-sizes a governed completion
predicate with nothing red to catch it, while under-declaring is visible and cheap. No roster is
kept here — each declaring file's `# no-port:` cause states its own instance and the owed set is
the port oracle's answer. Untouched by the 2026-08-30 config-seam ruling, which reaches part of
the left-owed set on an **edit-seam** ground the vocabulary question never asked; gate-sdk/SPEC.md
§The config-seam port disposition states the two grounds as cumulative.

**The two bootstraps are hand-kept, held in parity by a per-bootstrap install-smoke leg —
ruled 2026-08-26 by the operator**, closing `powershell-installer-surface`'s fork 2. The
mechanism, the refused generated-twins alternative, why the legs are counted by bootstrap
and not by platform, and the oracle's own two-part split — bootstrap parity, which the
PowerShell leg discharges for its half, against payload coverage, which is owed with the
relocation — are all owned at `installer/README.md` §The install boundary.

**The port's completion predicate is literal — `--tree`'s owed count reaching zero, over the whole
derived corpus, with no contributor-side subtraction — ruled 2026-08-28 by the operator**,
answering the question scope escalated as
`port-completion-predicate-contributor-side-accounting`; the operator ruled on a consulted
session's recommendation whose citations the lead relayed unverified. The carve-out
sentence in the 2026-08-23 ruling above never subtracted from the owed column: it scopes the
**adopter-facing** residue claim and sets sequencing, and each file it covers, when reached, ports,
leaves the tree with the surface it drives, or takes a per-file declared disposition under the
case-by-case residue rule below. Two alternatives were refused with the ruling. **A
contributor-side `# no-port:` class**: the field declares permanence while the carve-out's own text
says those files do port, so the declaration would negate the ruling it cites; its class ground —
ships to no adopter — was measured false for 31 of the disputed 33 files (kit `smoke/` suites and
kit-resident runners ride the payload; only `demo/run-demo.sh` and
`installer/consumer-smoke/run-smoke.sh` do not), and the 2026-08-24 `scripts/` declarations already
deliberately decline that ground. No mechanism is missing — a cause is free text — only a standing
ruling that would make one true, and none does. **Superseded for the `smoke/` class alone,
2026-08-30**, on exactly the condition that last sentence names — a standing ruling now exists
where none did. Two correct rulings in sequence rather than a mistake: this refusal was right on
the evidence it had. The distinction the later one turns on is gate-sdk/SPEC.md §Consumer smoke,
*The port disposition*'s, cited and not restated here. Untouched: kit-resident runners, which that
ruling does not reach, and the corpus alternative next. **Narrowing the predicate's corpus**: its
defensible content is already the corpus rule (the `*.test.sh` suffix and the prune set,
gate-sdk/SPEC.md §port-blockers), and widening the exclusion would let *port complete* be claimed
while a full-profile adopter's vendored tree still runs bash — a false claim at the front door, the
cost §What the objectives are not names.

**The deferred pool's intake is triaged at the drain, and machinery findings are dormant by
default — ruled 2026-08-30 by the operator in a consult, on a re-measured pool.** The 2026-08-23
ruling above bought its exits once — the pool fell from 252 to 228 on the day — and then intake
resumed at the prior slope: 227 to 284 entries across the eleven days to this ruling, 181 filed
against 64 removed, with the last two drains promoting fifteen of fifteen bullets and iceboxing
none. The intake's dominant class was measured, not assumed: ten of the last drain's ten entries
concern the lifecycle machinery governing this repo — journals, stamps, boundary wipes, entry
caps, tag positions — and by slug roughly three deferred entries in five do, against one in six
on the product the objectives above name. Three things are ruled. **The drain's dispositions are
ordered, promotion last** — mechanism at lifecycle-kit/SPEC.md §The committed gap inbox, which
also records the refused net-growth gate. **A finding about the delivery machinery itself —
the lifecycle, queue, guard or drift tooling as it governs this tree, as opposed to the kits as
product — that blocks no stage entry and no push is icebox-class by default**: it lands as one
line under queue-kit/SPEC.md §The icebox tier's grammar and returns on a real recurrence by the
conserved route that tier already has. **That class has a witness discriminator, ruled
2026-08-30 by the operator when the role clause just above proved to under-determine it in a
repo that dogfoods its own kits: a finding is machinery-class when its only demand witness is
this repo's own delivery process, and product when an adopter-facing claim — the install path, a
gate's verdict, the payload, a front-door statement — witnesses or would witness it, wherever
the fix lands.** Two readings were refused with it, and their grounds are what make the
discriminator non-obvious. Reading the class by *where the fix lands* alone empties this ruling's
own measured class, since the drained ten were mostly fixes landing in shipped kit code, and a
reading that voids its ruling's evidence base is wrong. Reading it by *subject* alone misbins an
adopter-reaching defect that dogfooding happened to find, the MSYS resolver being the worked
instance. The discriminator qualifies this default and retires with it on the discharge event
below, rather than outliving the paragraph it exists to make operable.
The ground is the objectives: every dollar an iteration
spends on the machinery's self-findings is a dollar the port track and the Windows leg do not
get, at seventy to one hundred and eighty dollars an iteration measured over the last eight, and
the machinery has been finding defects in itself faster than any iteration retires them. The
alternative refused: a per-iteration meta-work budget or ratio. It would need a classifier no
gate can honestly run, and the icebox default reaches the same end with a mechanism that
already exists. **The pool's net motion is stated at every close** rather than re-derived at a
consult — the `qnet` figure, owned by drift-kit/SPEC.md `kpi-queue-net-delta`, is written into
the drain's commit message; an enforcing gate on it was refused, grounds at the lifecycle-kit
section above. Discharge event for this paragraph: three consecutive closes each reading
`qnet` at or below zero, at which point the ordering is habit and the record is retired.

**A queue entry cites the gap inbox and the survey record as provenance, never as a locator —
ruled 2026-09-02 by the operator in consult.** Both surfaces are drained or truncated by the
lifecycle, so a pointer into either as a fact's current home is broken by the next boundary; the
rule, its resolves-after-the-next-drain test and the refused gate are owned at lifecycle-kit/SPEC.md
§The committed gap inbox. The alternative refused: banning every mention, which would strip filing
provenance from most of the deferred pool to remove a class measured at two instances.

**A dated attestation freezes the claim, never the locator — ruled 2026-09-03 by the operator,
lead-relayed from a consult.** A `path:line` in queue prose is live whatever date stands beside it,
and a dated measurement is history whatever moved since; the rule, its test and its consequence for
the pendency sweeps are owned at lifecycle-kit/SPEC.md §The committed gap inbox, beside the
2026-09-02 ruling it generalises. The alternatives refused: reading the date as freezing the locator
too, which licenses rotting anchors, and reading the locator's drift as falsifying the dated claim,
which would have every dated figure in this file re-counted — the step toward re-deciding the
bash-floor ruling above forbids.

**A section is a cut's outer bound, never its minimum — ruled 2026-09-03 by the operator,
lead-relayed from a consult.** A reading of the composer recorded on `native-gate-port-remaining-corpus`,
owned with its grounds at gate-sdk/SPEC.md §Porting a gate to the binary substrate: a section whose
own text sequences one member behind a named unit cuts its unblocked remainder behind that section's
amendment, the sequenced member staying owed rather than held. Refused with it: whole-section-or-nothing,
a hold declaration on the remainder (a held file leaves the owed column), and re-homing the remainder's
`# spec:` to make the section a singleton.

**A consumer's plugin on a kit seam is owed like any file; ruling (1) reaches the seam alone — ruled
2026-09-03 by the operator, lead-relayed from a consult.** The composer's ruling (1) protects an
extension point's resolution, execution and env contract, and its own surfaces bound it to files whose
whole documented purpose is to be edited; a file this repo names as a knob's value is what the seam
resolves, so the literal predicate above governs it unopposed. Owned with its grounds at
gate-sdk/SPEC.md §Porting a gate to the binary substrate. Refused with it: reading ruling (1) as a
contributor-side exemption under another name, and a seam-citing `# no-port:` on files holding no
seam content class.

**A port cut is sized to fill one build window, never to a unit count — ruled 2026-09-03 by the
operator in consult, on measured overhead.** The mechanism is owned at gate-sdk/SPEC.md §The first
cohort, and the rule that selects the next (the budget's floor). The measurements, dated here rather
than carried live: the overhead meter read a 66 % governance share over the ten sessions to this
ruling; the stage-economics meter over 134 iterations put build near one fifth of an iteration's
spend and scope, close and supervision together near three fifths; and the account's seven-day
window closed each of the four weeks before this ruling between 77 and 97 %, so the cap is the
window and not the iteration count. Two alternatives were refused with it. A per-iteration
**unit-count target** (ten or twelve units) measures nothing the window measures and nothing the
record supports: no KPI reads units per iteration or cost per unit, and the one wide cut on record —
eight members in one iteration — was the cheapest per member and the dearest in total. The width
is bought inside the cut first; what fills the window's remainder is the port-first run's below.
The cost-per-unit meter that would let the next sizing be read rather than judged is queue work
filed from this consult.

**A kit SPEC carries mechanism only; this project's provenance never ships — ruled 2026-09-03 by
the operator in consult.** A dated operator stamp (`ruled <date> by the operator`), a pointer into
this file, and a refused alternative's grounds are this project's ruling history, and the kit SPECs
ride the installer payload and the public site whole, so every such line lands in a consumer tree
where it resolves to nothing and reads as mechanism. Provenance lives here and in git history,
which CLAUDE.md §The provenance seam now names as a class; a SPEC states the rule and its
engineering grounds, undated. The measure, taken 2026-09-03: thirty-seven dated stamps and fifty
pointers to this file across six kit SPECs, gate-sdk's carrying twenty-four and forty-three. How
the class arrived, recorded so the fix is aimed at its cause: this file's own recording rule sends a
ruling with a canonical home to that home by pointer, and sessions read that as licence to write
the ruling *into* the kit SPEC with its date and its refusals, so the SPEC became the ruling's home
and this file the index — the seam's content classes named vocabulary and never provenance, and no
gate held it. The alternative refused: leaving the class in place as design rationale, which
publishes private history as mechanism and ships pointers a consumer cannot follow. The sweep and
the gate that holds the seam afterwards are one queue unit, filed from this consult —
`kit-spec-provenance-seam-sweep-remainder`, the surface that survives after scope split that filing
by kit. The 2026-09-05 direction that sent it beside the test-harness cut is **spent** — that
iteration ran and joined it, and a second joined it again — and its same-surface ground went with
it, gate-sdk being swept. **Re-grounded 2026-09-06 by the operator on product-class with a live
trigger alone**, the accretion being self-attesting: the class grows from cuts that never touch
provenance. Scope sizes it against the window and may split it by kit. Discharge event: that entry
lands whole, its gate included, at which point this paragraph is deleted.

**No governed file is exempt from brevity; the close pass walks every file that grew — ruled
2026-09-05 by the operator in consult.** The close-stage brevity pass had measured `CLAUDE.md` alone
and exempted on-demand files by its own text, so it reported green through a month in which the
queue, the ruling record and the kit SPECs multiplied while `CLAUDE.md` grew seven lines. The
exemption was never the operator's. Mechanism: context-kit/SPEC.md §The always-loaded meter's
`--growth` arm and §The close-stage brevity pass; doctrine-kit/DOCTRINE.md rule 5 states the
principle. Refused: a gate reddening a close on net growth, on the 2026-08-30 grounds that the exits
are judgments; the figure is stated at every close and read at the consult.

**The queue's `ruled:` declaration line is retired; a ruling's provenance is stated inline —
ruled 2026-09-05 by the operator in consult.** Mechanism at queue-kit/SPEC.md §The tag algebra and
§check-queue-entry-budget assertion (D). The ground: the line had no reader but the budget gate's
own one-line discount, the entry body already carried authority, date and channel inline, and on
the host entry five of six such lines were counted against the cap that had just forced a
compression — provenance restated in a second tier, displacing the task it annotated. Refused:
keeping the line as an audit record, which no auditor reads in preference to the sentence beside
it; and a validity gate for it, which would have gated a grammar worth deleting.

**A stated cause reaches only the members it names, and an always-loaded manifest is never a
cut boundary — ruled 2026-09-05 by the operator in consult.** Mechanism at gate-sdk/SPEC.md
§Porting a gate to the binary substrate, beside the outer-bound ruling it extends. Refused: the
*as a whole* reading of a section whose cause touches two of three members, which held 139
standalone lines behind a library they never source; and composing three files sharing no
subject as one cut under CLAUDE.md §Housekeeping, which is averaging grounds under a manifest
that owns no mechanism. Re-homing a `# spec:` that points at a manifest is a defect correction
and not the re-homing the 2026-09-03 ruling refused, which was a re-pointing away from a SPEC
section that already owned the file.

**The "add the useful grant" directive is discharged by falsification, and no grant was owed —
ruled 2026-09-06 by the operator, relayed by the lead session.** The directive answered a proposal
whose premise measures false: the standing prompt-guard rule has granted the resume-journal append
since 2026-09-04, and both shapes the remaining residue could take are already answered by the tree,
so no work was owed and neither survivor folds into an iteration. What earns the line is the general
form — a directive answering a proposal is discharged when the proposal's premise measures false,
and such a discharge leaves no landed unit and no queue trace, so a later session reading only the
directive re-derives the whole investigation that falsified it. The surviving two-shape residue
stays queued and deferred on `append-grant-decline-cause-unlogged`, which owns the measurement.
ruling: add the useful grant

**The lead's invocation is itself the open authorization, and one invocation grants exactly one
iteration — ruled 2026-09-06 by the operator, in a direct in-session reply to a lead session's
authorization ask.** Verbatim: *"when I issue `/lead undirected` I grant you the right to open one
iteration. Not two, not three, but one."* Two limbs, separate and both load-bearing. **The
invocation is the grant** — a lead that has been invoked holds its authorization already and asks
for nothing further, so §Opening an iteration's *obtained explicitly and separately* is corrected
by this: an invocation is explicit without being separate. **The grant is bounded at one open** —
the iteration after it takes a fresh invocation, and the bound belongs to the **grant** rather than
to the boundary that §Closing an iteration attaches it to today.

The ground is a measured failure and not a preference: a lead opened a second iteration on one
grant, and the instruction surface is what let it. §Opening names no channel by which authorization
arrives and no cardinality at all, so a lead reads that its own invocation authorized nothing and
learns the bound only an iteration later, at the far end. The operator directed the correction with
the ruling — *"Correct your instructions if not clear"* — and it was owed rather than taken here
because it is two edits on two tiers. **Both landed 2026-09-06 and this paragraph's mechanism has
relocated to them, on the instruction this paragraph itself carried.** The generic half is
`lifecycle-kit/templates/lead.md` §Opening an iteration; the consumer half is the
**`open-authorization-channel`** binding in `.claude/commands/lead.md`, a **newly minted** slot and
not the `ruling-config` one this sentence named while the mechanism was still owed — ruled `lead,
own-authority` 2026-09-06 on the spec session's escalation, since `ruling-config` declares a
different subject and policy binding the lead itself belongs to the template. **Correcting a
mechanism sentence this paragraph had already marked transitional is the last act of the
relocation, not a reversal of anything closed:** what is closed is the ruling's two limbs, their
date and their channel, and those stand exactly as recorded above.
ruling: the lead-invocation open grant

**Where this project's own class default and the close template's icebox trigger test appear to
disagree, the class default governs — ruled `lead, own-authority` 2026-09-07, through the lead's
message channel at that iteration's close.** So a machinery-class gap bullet takes the icebox
disposition **even when it names a live trigger**. First applied to the stale-DoD-arithmetic bullet,
which became the icebox entry `dod-size-figure-stales-in-iteration` — that entry is the ruling's
first application and not its content. **The ground is not the one the escalating session
offered**, and recording the weak form would invite the counter that retires it: *the default is
the more specific rule* fails, because a default is ordinarily the yielding rule and a live trigger
would then outrank it. What settles it is the 2026-08-30 ruling's own text. Its witness
discriminator was recorded **precisely because** the role clause above it "proved to
under-determine it in a repo that dogfoods its own kits" — this case, not an analogue of it — and
its test is **conjunctive**: machinery-class by the discriminator, blocks no stage entry, blocks no
push. There is no live-trigger limb in it at all, so a bullet whose trigger fired satisfies the
conjunction on its own terms and the default applies without outranking anything. The template's
"naming no live trigger" is kit-generic phrasing that stays true for a consumer that does **not**
dogfood its own kits, so the two never collide once the class ruling is read as answering the case
the kit phrasing under-determines. Nothing is owed in lifecycle-kit/SPEC.md: the rule is this
project's reading of its own default, and the provenance seam keeps it here.
Discharge: with the 2026-08-30 machinery default it qualifies.
ruling: the machinery-class icebox default over a live trigger

**A hotfix's "minimal" is measured against the failure modes the change itself creates, never
against the smallest diff — ruled `lead, own-authority` 2026-09-07, through the lead's message
channel at that iteration's build.** The operator-ruled hotfix carve-out admits a fix that is
"minimal and test-and-doc-complete in one commit" (CLAUDE.md §Delivery doctrine, scope-gated
intake). The uninstall argv hotfix reached past its two named call sites, moving one of them to a
file-mediated read so its exit status survives, and that reach is INSIDE the carve-out. **The
ground is specific and licenses no convenient extra:** batching that site alone would have
introduced a silent PARTIAL-roster case the single-invocation defect cannot produce — one
invocation is all-or-nothing, while an early batch succeeding and a later one failing stages
part of the set just as silently — so reading a real status there repairs a failure mode it would
otherwise have created. **Refused:** taking the two sites literally and shipping that case, which
would have made the hotfix's own diff the source of a new defect. Recorded at the close because
it was ruled after batch A returned and batch B held the shared index.
Discharge: none — it reads a standing carve-out rather than directing an act.
ruling: the hotfix-minimality reach

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
conditional install logic not yet behind the invoke. After the port: **one deferred-pool triage
iteration**, then `companion-toolkit-profile`.

**The port-first run — ruled 2026-09-05 by the operator in consult, superseding the 2026-08-31
port-only run and the three paragraphs that qualified it.** The port stays the track's top
priority, and every iteration carries a port cut while the ruled composer can reach one: the cut
is selected first, sized to fill the build window, and never displaced by a joined unit. What
changes is what fills the window beside it. The port-only run barred every other deferred entry
until the oracle read zero owed, and its measured outcome is what retires it: sixteen of the
seventeen iterations since it was ruled shipped a cut, the owed column fell to 23 files and 2679
lines — 1722 behind the install boundary and two remote-oracle CI legs no cut can buy, 956
reachable, four fifths of those in one section family — while the deferred pool rose from 283 to
292 and the icebox from 52 to 84. Its last iteration bought one cause read at full ceremony, six
stage sessions and a resume for one deferred entry that returned to Deferred, at a governance
share the overhead meter read between 64 and 75 per cent across the run's ten sessions. That is
the run buying the single-unit iteration the kit's own composition test refuses, and the ruling
record accreting an exception paragraph every time a session hit the bar — three in five days.
Quality and efficiency are both the project's objectives; a priority is not a licence to spend
the second on the first. **The rule:** scope composes the iteration under
lifecycle-kit/templates/stages/scope.md's economic composition test, which the port-only run had
overridden — the port cut first, then deferred entries filling the window's remainder, and the
whole window when the reachable column is empty or the port's next act is an own-iteration leg. A
joined unit qualifies on one of three grounds, each an existing class and none a new classifier,
ordered so when the window cannot hold every candidate: **port-critical** — its discharge is the
stated precondition of an owed file's cut, the test being the owning section's own sequencing
sentence and never a session's judgment of usefulness; it rides inside the cut it unblocks when it
fits (the hermetic suffix pin was the instance) and is its own iteration only when it cannot (the
Windows and macOS legs); **same-surface** — it edits a kit or SPEC section the cut edits, so the
ceremony and the context are already paid; or **product-class with a live trigger** by the
2026-08-30 witness discriminator, which `[roadmap:]`-tagged entries and entries at the recurrence
threshold satisfy — a threshold member is proposed once under this ground, never re-escalated as a
set already answered. Close's drain runs its 2026-08-30 disposition order unqualified — fix,
icebox, promote last — so `promotes nothing` retires with the run; machinery-class findings stay
icebox-by-default under the ruling above, untouched. **Refused:** continuing the port-only run,
whose reachable column is one more window, after which every iteration is an own-iteration leg or
another exception paragraph; a dedicated relief iteration, which pays a full ceremony floor for
units that each fit beside a cut; and a per-iteration meta-work ratio, refused again on the
2026-08-30 grounds. **The health triad this ruling is scored on, stated
by the operator with it:** a healthy project shows a decreasing queue, instruction surfaces that
get better and cheaper, and falling governance overhead, and on 2026-09-05 every one of the three
read the opposite way. Each has an owner already — `kpi-queue-net-delta`'s `qnet`,
the always-loaded meter's delta and its `--growth` arm over every governed prose file, and
`kpi-overhead`'s share — so close states all three in the drain's commit message beside the `qnet`
fragment it already writes, with the cause when one moves the wrong way, and the next consult reads
them against this ruling's expectation: qnet at or below zero, governed prose flat or shrinking,
the share falling.
An enforcing gate stays refused on the 2026-08-30 grounds; the shape is reopened if none of the
three has moved over five closes. **The pool is still
triaged after the port, as one iteration** that iceboxes by class and retires what a landed unit
or a closed ruling has mooted; joining units under the grounds above is work on the pool, not that
triage. Discharge event: the oracle's owed count reads zero, at which point the port-cut-first
clause is spent and this paragraph is deleted; the composition test is the kit's and stays.

**`citation-liveness-family-convergence` is carved out of the machinery-class icebox-by-default bar
in the paragraph above — ruled 2026-09-08 by the operator, answered directly in a lead session and
relayed by the lead.** The bar stands for the class; this one member leaves it.
**What the carve-out buys is admissibility at a future scope and nothing more, and reading it as an
admission is reading past its own limit.** The operator answered that same boundary's unit-set
question separately and did not admit this family to that window. So the carve-out removes a
standing bar; it composes no iteration and outranks no composition test.
**It is a carve-out from the bar rather than a fourth joining ground**, and the distinction is
load-bearing: the lead's 2026-09-08 reading that the family meets none of the three grounds was put
to the operator and not overruled, so a scope still has to find the entry a ground on the day it
proposes it. Minting a fourth would have made every machinery-class entry arguable, which is the
outcome the 2026-08-30 discriminator exists to prevent.
**The grounds are the entry's own and are not restated here** — it carries the bought survey, its
four gate touch points, its measured inbound sum and its cost field, and a copy here would be a
second reading of one fact aging on its own clock.
discharge: citation-liveness-family-carve-out grep -L citation-liveness-family-convergence TASK-QUEUE.md
ruling: citation-liveness-family-carve-out

**The whole-window clause describes what fills an iteration composed AROUND a cut; scheduling an
own-iteration leg as the iteration is what that phrase names — ruled 2026-09-07 by the operator,
through the AskUserQuestion channel in a lead session and relayed by the lead.** The clause above
sends the whole window to deferred entries when "the port's next act is an own-iteration leg", and
read as barring the leg itself the clause would make its own discharge event unreachable — the owed
count cannot fall while the one unit every held file waits on is unschedulable. It is a reading of
the clause and **not** a reversal of the 2026-09-06 application on that entry's clause (8), which
stands for that iteration. Two passages carry it and are recorded together because weighing one
without the other is what makes this re-derivable: the port-critical ground above says such a unit
"is its own iteration only when it cannot" ride inside a cut, **naming the Windows and macOS legs**
as the instances — that is the text the reading stands on; and the refusal of the port-only run
warns that its reachable column is "one more window, after which every iteration is an own-iteration
leg or another exception paragraph", which is not repealed here and bounds how often this reading
may be reached for. Discharge: with the port-cut-first clause above, which this reads.

**The 2026-09-06 no-cut ruling on `native-gate-port-remaining-corpus` reached
the right conclusion from a ground the disjunction above does not bear —
annotated 2026-09-07 by the operator, answered in a lead session and relayed by
the lead, under the fourth act at the head of this file.** That ruling reads
*empty-column, both fire*, and it is clause (8) on the entry, where its own text
stands unedited. **The first limb is false, and its falsity did not age into
place — the same members were held on the day the ruling was taken.** The
reachable column is not empty; what is true of its members is that every one is
**held** by a stated precondition in its owning section, which is a different
fact with a different consequence — a held column can be emptied by one ruling,
an absent one cannot. No figure is carried here for the reason the paragraph
below already gives: run the oracle. **The second limb fires alone**, and it
carries the conclusion by itself: the port's next act is
`powershell-installer-surface`, an own-iteration leg. **So the ruling's
conclusion is unchanged** — the disjunction asks for one limb, no cut was
takeable, and the window was the deferred pool's, exactly as ruled.

**A sequence remainder of one is not a finished port, and reading it as one is the misreading
this paragraph exists to stop — operator-ruled 2026-08-25.** The completion predicate is the one
§The closed rulings names, `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree`'s owed count reaching zero, and it stands
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
