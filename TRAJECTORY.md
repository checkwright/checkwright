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

**New gates are born native by default; shell is the exception, and it needs a
stated cause — ruled 2026-08-14.** This reverses the default recorded at
gate-sdk/SPEC.md §The port-candidate criteria, where a born-native gate was "a
design ruling rather than a default", and it is the only measure taken so far
that acts on the port's *denominator* rather than its remainder: under the prior
default every gate landed while the port ran added shell the port then owed.
The mechanism — the exception criterion that makes "with cause" operable — is
**delivered** at that same section as three closed classes with a stated cause
form each, and is not restated here. Ruled with criterion 5's cost in hand and against the recording
session's own lean, which is why it is recorded rather than argued: a
`.gate`-declared member is *omitted* on a platform `native/targets.list` carries
no artifact for, that roster is one target today, and the flip therefore attaches
that omission to every new gate. **Re-affirmed the same day on corrected
evidence, recorded because the first ruling was taken on an understated one.**
The uncovered platform set was put to the operator as narrow and Windows-shaped;
`native/targets.list` ships a single target, so the set is **every macOS adopter**
today, for whom omit-and-declare is already the normal path. The operator
re-affirmed the flip against that cost, unchanged. This records what the ruling
was weighed against — it is not a re-opening, and the ruling stands as ruled.
What the ruling does not do is widen the
roster; the residue it creates is costed and filed as
`born-native-omission-accumulation`, and the roster's own widening trigger
(`platform-support-ci-matrix` landing a CI leg) is unchanged.

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

**Objective 4 constrains the adopter's dependency set, never the crate's build
graph — ruled 2026-08-14, and it was never otherwise.** The two sets are
different things and reading them as one is what made a prohibition appear that
had never been stated: the adopter's set is what they install and can uninstall,
which is git plus pre-compiled binaries; the crate's build graph is resolved and
compiled by contributors and CI, and no consumer ever receives, resolves or
compiles it (install-time builds from vendored source are void under objectives
1, 2 and 5). So the `native/` crate is under **no** no-external-dependencies
prohibition and never was. Recorded because the false reading was not merely
held — it was cited as a *ground* in four places in gate-sdk/SPEC.md and encoded
in a passing test before an operator correction caught it, which is the cost of
leaving an objective's scope to be inferred. What the ruling does **not** touch:
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

This is recorded rather than left where it was decided, and that is the whole point of the entry:
the 2026-08-13 ruling **selected** this question's disposition and never **stated** the direction,
so for a week the answer lived in no governed surface at all — only a drained gap bullet and a
commit message, which spec-over-precedent says is not ground truth. The ruling is discharged when
the re-derived grant set lands; the entry carrying that work is **`guard-grant-review`**.

**The graph gate's consumer config becomes a data-only contract, and the sourced-function seam is
retired — ruled 2026-08-20 by the operator.** `check-graph` is the last takeable member of the
port, and the only one whose consumer configuration crosses **executable bash functions** rather
than values: `graph_surface_layer` from `GATE_SDK_GRAPH_VOCAB`, plus `graph_theme_css`,
`graph_theme_header` and `graph_theme_footer` from `GATE_SDK_GRAPH_THEME`, each dispatched through
`declare -F`. A compiled binary cannot be handed a bash function, and the array-knob config bridge
carries arrays, scalars and keyed lists into argv and nothing else — so the port cannot be built
without deciding what replaces the hooks. Two alternatives were costed and refused: a **shell shim**
kept for theme emission alone preserves the seam and adds a shell-only install step the interpreter
policy above forbids outright, and **shipping the artifact unthemed** is cheapest and regresses a
shipped capability, which is not a port.

**The cost is a breaking change, and it is stated rather than softened.** It narrows asserted
behavior for every consumer carrying a `graph-theme.sh`, and it re-cuts the seam
CLAUDE.md §The provenance seam names as its own worked example — so the replacement is bound to keep
that doctrine true: the vocabulary and the theme stay the consumer's, and only their *form* moves
from executable to declarative. Designing that form is the port unit's work. **The ruling is
discharged when an amendment lands the contract at gate-sdk/SPEC.md §check-graph**, after which
this record is deleted rather than annotated.

Two questions the ruling deliberately leaves open, recorded here so the authoring stage receives
them instead of rediscovering them. `couples=` is read three ways across the tree and `check-graph`
assertion B is the third — **`couples-glob-semantics-unowned`** owns that, and the crate already
carries two matchers with nothing anywhere to say which side a port should reach for. And
`check-graph`'s **criterion-4 self-reference disposition is unruled rather than cleared**: the
member was kept out of every budget batch on the non-gate-arm ground, before the criteria checklist
was ever run against it.

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

**What this supersedes, and why the supersession is principled rather than a
change of mind.** The 2026-08-03 sequencing rule and its 2026-08-06 re-ruling —
that the track runs toward per-profile coherence rather than whole-corpus
completion, measured at the adopter's floor and never at a roster substrate
census — is superseded for the port's scope. What that rule refused was a
*roster-census* justification: completing a substrate tally for its own sake.
The case that carries now is different in kind, the elimination of a permanent
duplicate. **No port-candidate criterion survives as an eligibility gate on
which gates may be ported** (gate-sdk/SPEC.md §The port-candidate criteria):
the technical problems those criteria name — self-referential parity, a rule
depending on an external program — are engineering work the port owes, not
exclusions it may take.

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

**The consumer's own gates port into `native/` — ruled 2026-08-14.** The gates
this repo declares under `scripts/` become subcommands of the existing
multi-call binary rather than a second consumer-owned crate or a sanctioned
shell block, so no part of the remaining corpus is now without a destination.
The ruling's grounds and the cost it accepts — an adopter's binary carries
subcommands implementing another project's repo rules, which they can never
register — were the queue entry's, and that entry has since been **discharged
and retired**: gate-sdk/SPEC.md §The consumer remainder cohort records the last
tranche, after which `scripts/` keeps no gate script at all. Recorded here
because it binds every remaining cohort's ordering and the entry that owned it
was always going to leave the queue.

What the sequence names next, after the port: `companion-toolkit-profile`.

`instruction-surface-bash-focus` unblocks on a threshold rather than a date,
per its own queue entry. Surge-channel launch stays gated behind the private
brief's readiness rule, and launch-comms execution runs on its own clock under
the surface that owns distribution.
