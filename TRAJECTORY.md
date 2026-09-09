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
and a session that meets one **does not re-litigate it**. A finding that appears
to contradict an objective is an escalation rather than a stage-level or a
lead-level decision, however well-grounded. Only the operator may reopen a
closed ruling.

**A figure recorded here is a dated record of what a decision was weighed
against, never a live claim.** The same arithmetic in a governed spec is
repaired by de-literalization; the figure here is corrected under the first act
below. Different kinds, different repairs — one commit may owe both.

**Who may record may retire.** A session **records** a ruling that a ruling
authority closed — the operator, or the lead on its own authority, the two
CLAUDE.md §Housekeeping names — and never **authors** one; what licenses a
retirement is the ruling's discharge. Neither turns on the session's role, so no
roster of permitted writers or retirers is kept, and none is needed. A recording
names the ruling's date, its authority and its channel. **The honest limit:**
this is an authoring contract and not a gate. Nothing mechanizes *the operator
closed this*, and nothing should — the alternative is a session attesting to its
own consent, which is worth less than the rule.

**Four acts, named and separated** — the first three ruled 2026-09-06 by the
operator, the fourth 2026-09-07:

- **Correcting an aged fact** — a slug the queue has retired, a count the tree
  has moved, a pointer whose target does not carry the claim. **Session-class,
  including inside the text of a ruling still in force**, and bounded by a test
  rather than by a category: it qualifies only where an oracle settles the fact.
  **If the repair would change what the ruling directs, it is not a repair.**
- **Retiring a spent ruling** — it directed something that has already happened,
  so deleting the record decides nothing. **Session-class.** A ruling is tested
  by **mood** before it is tested by completion: one that DIRECTED WORK is spent
  when the work lands, and one that ESTABLISHES A RULE is never spent while the
  rule governs, however completely the mechanism that prompted it has shipped.
  Where such a rule's home changes it **relocates** to the surface owning that
  mechanism, which is a third disposition beside delete and keep.
- **Annotating a false ground** — where a closed ruling's stated **ground** was
  false when the ruling was taken and the ruling's **conclusion is unchanged**,
  the ground is annotated beside the ruling and the ruling's own text is left
  standing. **Session-class**, on the correction test: what the ruling directs
  does not move. A ruling whose conclusion does not survive is not annotated —
  that is the act below.
- **Reversing, demoting or re-scoping** — making a closed ruling stop being the
  rule, narrowing what it reaches, or moving it down a priority order.
  **Operator-class**, however well-grounded the finding and however urgent the
  fix.

**A correction is never appended**, and a superseded sentence is never left
standing beside the sentence that corrects it: two readings of one fact is the
defect, whichever of them wears the label *current*. The annotation act is the
one bounded exception, and it is bounded to a ground whose conclusion survives
it — correcting such a ground in place would lose the only record that it was
ever wrong.

**A retirement's blast radius is derived, never rostered.** No surface owns a
list of what cites this file and none should — a maintained roster would be one
more copy to stale — so a retirement greps for its own citations, over the
declared names below. Two properties make it non-obvious enough to write down.
`docs/` is a **generated mirror**, so every kit-SPEC citation appears twice and
only the source is editable; regenerate rather than hand-edit the second. And
the majority citation shape names no file at all and carries only the ruling's
proper noun, so a probe over citation *targets* reaches almost none of them. The
design of both halves: lifecycle-kit/SPEC.md §The ruling-staleness probe.

**What this file admits — ruled 2026-09-09 by the operator in consult.** This
record accompanies a pivot and shrinks as the pivot lands; it is never a growing
authoritative surface, and business-as-usual runs on the tooling the kits already
ship. A paragraph is admitted here only if it **names its own discharge event**
(an oracle, or `manual` with the condition) or **reverses a recorded ruling**.
Everything else has a business-as-usual home and lands there, undated: a ruling
about mechanism in the owning kit SPEC with its engineering grounds; this
project's reading of a kit template in the command binding that names the
template; a standing rule in CLAUDE.md as one line pointing at its mechanism;
and who ruled, when, through what channel, and what was refused, in the landing
commit's message — git history is the archive. The consult binding's
landing-surfaces slot states the same test from the recording side. One
authoring convention follows and is what makes the next application cheap: a
ruling able to name its own discharge event says so in its own text, so the
session that meets the event deletes rather than judges.
discharge: ruling-record-admission-test  grep -L ruling-record-shrink-to-bau TASK-QUEUE.md
ruling: ruling-record-admission-test  the admission test

**Two body-line declarations make that convention resolvable rather than merely
followed**, both hand-written by the session recording or retiring the ruling, at
the moment it writes it. Neither is a **tag**: a tag marks a move across a
pending/ready boundary and these mark none.

- `ruling: <name>[ <name>…]` — the ruling's own proper noun, in the words other
  surfaces use for it, plus each variant a surface has actually used. **One or
  more, appended and never rewritten.** A single declared name is insufficient by
  construction: the largest measured cohort of stale citations shared exactly one
  invariant, the proper noun in a present-tense claim anchored to no file, and
  one site in that same cohort survived its own repair by writing the noun a
  single word differently.
- `discharge: <name> <oracle>` — for a ruling conditioned on a future event: the
  ruling's declared name and a **command** whose output settles whether the event
  has fired, never a predicate a reader interprets. A non-zero exit, or a clean
  run printing nothing, means the condition has **not** fired and the ruling
  stands; a clean run that prints means it **has**, and the ruling becomes a
  retirement candidate to judge under the mood test above. A condition no command
  can settle takes the literal operand `manual` followed by the prose condition,
  which reports as owed to judgment rather than being silently skipped.

**No gate demands a declaration** — a scanner cannot demand one written under
judgment. A ruling carrying none is simply one the citing-side report cannot
reach, and the probe reports that hole instead of inheriting it.

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
and the front door is where a false claim costs the most. The converse is also
a defect: a platform an objective names is declared as **held** with the run
that joins it, never left undeclared, because an undeclared platform reads to
every later session as one the project does not intend.

## The closed rulings

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
The asymmetry it rests on: a wrong deferral is corrected by rewriting one line,
and a wrong tag is public. **The honest limit** is that narrowing a criterion can
only be wrong in one direction — an install-path defect that genuinely does reach
users late now reads as not-firing, and no gate catches that. What bounds it is
the same clause that grounds it: the ruling is stated *for* `preview`, so the
channel flip at `v1.0.0` reopens it rather than inheriting it. Nothing here
narrows the operator direction trigger, which stays available for exactly the
urgent case this ruling declines to automate.
discharge: release-trigger-reads-narrowly  grep -F 'Release channel: **stable**' docs/install.md
ruling: the security-or-supply-chain trigger reads narrowly  the release policy's narrow trigger

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
discharge: no-gate-is-permanently-shell  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: no gate is permanently shell  what port complete means

**That predicate has an oracle, and it is not the one that measures the battery.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` reports the port disposition of every tracked non-test
`.sh`, and its trailer's owed count reaching zero *is* the sentence above: at zero, every remaining
script either carries a stated cause or is gone. The registry arms beside it walk `gates.list` and
answer for the gate battery alone — so a session reading zero owed **there** is reading a finished
battery and not a finished port, which is the misreading this predicate invited for as long as it
had no evaluator of its own. Mechanism and bounds: gate-sdk/SPEC.md §port-blockers.
discharge: the-port-predicates-oracle  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'

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
discharge: scripts-no-port-vocabulary-half  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: the vocabulary half generalises  the scripts no-port class

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
discharge: the-ports-completion-predicate-is-literal  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: the port's completion predicate is literal  no contributor-side subtraction

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
section above. When the condition below fires the ordering is habit and the record is retired.
discharge: machinery-class-icebox-default  manual three consecutive closes each reading qnet at or below zero
ruling: the machinery-class icebox default  the witness discriminator  machinery-class by default

**A section is a cut's outer bound, never its minimum — ruled 2026-09-03 by the operator,
lead-relayed from a consult.** A reading of the composer recorded on `native-gate-port-remaining-corpus`,
owned with its grounds at gate-sdk/SPEC.md §Porting a gate to the binary substrate: a section whose
own text sequences one member behind a named unit cuts its unblocked remainder behind that section's
amendment, the sequenced member staying owed rather than held. Refused with it: whole-section-or-nothing,
a hold declaration on the remainder (a held file leaves the owed column), and re-homing the remainder's
`# spec:` to make the section a singleton.
discharge: a-section-is-a-cuts-outer-bound  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: a section is a cut's outer bound

**A consumer's plugin on a kit seam is owed like any file; ruling (1) reaches the seam alone — ruled
2026-09-03 by the operator, lead-relayed from a consult.** The composer's ruling (1) protects an
extension point's resolution, execution and env contract, and its own surfaces bound it to files whose
whole documented purpose is to be edited; a file this repo names as a knob's value is what the seam
resolves, so the literal predicate above governs it unopposed. Owned with its grounds at
gate-sdk/SPEC.md §Porting a gate to the binary substrate. Refused with it: reading ruling (1) as a
contributor-side exemption under another name, and a seam-citing `# no-port:` on files holding no
seam content class.
discharge: a-plugin-on-a-kit-seam-is-owed  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: a consumer's plugin on a kit seam is owed like any file

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
discharge: a-port-cut-is-sized-to-one-build-window  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: a port cut is sized to fill one build window

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
lands whole, its gate included, and this paragraph is deleted with it.
discharge: kit-spec-provenance-seam  grep -L kit-spec-provenance-seam-sweep-remainder TASK-QUEUE.md
ruling: a kit SPEC carries mechanism only  this project's provenance never ships

**A stated cause reaches only the members it names, and an always-loaded manifest is never a
cut boundary — ruled 2026-09-05 by the operator in consult.** Mechanism at gate-sdk/SPEC.md
§Porting a gate to the binary substrate, beside the outer-bound ruling it extends. Refused: the
*as a whole* reading of a section whose cause touches two of three members, which held 139
standalone lines behind a library they never source; and composing three files sharing no
subject as one cut under CLAUDE.md §Housekeeping, which is averaging grounds under a manifest
that owns no mechanism. Re-homing a `# spec:` that points at a manifest is a defect correction
and not the re-homing the 2026-09-03 ruling refused, which was a re-pointing away from a SPEC
section that already owned the file.
discharge: a-stated-cause-reaches-only-the-members-it-names  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: a stated cause reaches only the members it names

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
discharge: icebox-default-over-a-live-trigger  manual with the machinery-class icebox default above, whose condition it shares
ruling: the machinery-class icebox default over a live trigger

**The same default governs close's OWN captures, not only the bullets it drains — ruled `lead,
own-authority` 2026-09-08, relayed in-session on a close-stage escalation. It EXTENDS the ruling
above to a second template clause and decides nothing new.** The close template's *Where close's own
captures file* clause reads Deferred always; where it and this project's machinery-class
icebox-by-default appear to disagree, the class default governs, so a close's own machinery-class
capture takes the icebox disposition.
**The ground is the paragraph above's, deliberately and not by economy:** CLAUDE.md's class default
is stated unqualified and scopes to the finding's CLASS rather than to the draining act, so the
2026-08-30 conjunctive test reads across to this clause exactly as it reads across to the trigger
clause. **A second and sound ground was offered and is NOT recorded as the ground** — that consumer
policy overriding a generic kit default is the provenance seam working, which is true on the facts
and is why no kit change is owed here either. Recorded as *the* ground it would re-expose the flank
the paragraph above deliberately closed, being a specificity argument in different clothes. Nothing
is owed in lifecycle-kit: its close template is kit mechanism shipped generic and this repo's
close-command file is only a binding shim naming it.
**Why both paragraphs stand rather than one, and it is a finding about this record rather than about
either session:** the escalating session and the lead each reached this answer without the paragraph
above being cited. This record is searchable by ruling name and by discharge oracle, and neither
index reaches *which surface wins when the close template and the class default conflict* — so the
extension sits beside its sibling, which is the cheapest repair a record with no third index has.
discharge: icebox-default-over-closes-own-captures  manual with the machinery-class icebox default above, whose condition it shares
ruling: the machinery-class icebox default over close's own captures

## PRIORITY DIRECTIVE — the port track's sequence

**Ruled 2026-08-09 by the operator, and it is the track's top priority: complete
the native gate port, ASAP.** Everything portable ports — the gates, the runners
and the install-lifecycle scripts alike. Surviving shell is **residue justified
case by case, never a protected category**, so "it is orchestration" is not by
itself an answer for a script that stays. The load-bearing ground is
cross-platform cost rather than the port's wall-clock win, which is small: a
shell dependency buys Windows support by maintaining PowerShell duplicates,
paying the dual-maintenance tax along a second axis, where a native binary is one
implementation on every platform. **This supersedes the 2026-08-03/06
per-profile-coherence sequencing rule for the port's scope** — what that rule
refused was a *roster-census* justification, and the elimination of a permanent
duplicate is different in kind. **No port-candidate criterion survives as an
eligibility gate** (gate-sdk/SPEC.md §The port-candidate criteria): the problems
those criteria name are engineering work the port owes, not exclusions it may
take.
discharge: complete-the-native-gate-port  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: complete the native gate port  the port track's top priority  the 2026-08-09 directive

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
discharge: port-first-run  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: the port-first run  the port-cut-first clause  port-first

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
may be reached for. **That bound was reached for once, by `behind-invoke-relocation` (2026-09-09),
and once is all it holds** — read in consult that day, `consult, own-authority`: the leg it schedules
is the sequence's last member, so no own-iteration leg remains to reach for, and a later reach is
the exception paragraph the warning named rather than a second use of this reading.
discharge: own-iteration-leg-reading  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: the own-iteration-leg reading  the whole-window clause

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
discharge: no-cut-ruling-ground-annotation  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: the 2026-09-06 no-cut ruling's annotated ground

**A sequence remainder of one is not a finished port, and reading it as one is the misreading
this paragraph exists to stop — operator-ruled 2026-08-25.** The completion predicate is the one
§The closed rulings names, `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree`'s owed count reaching zero, and it stands
over the whole tracked shell tree rather than over this sequence. Both readings are correct and
neither absorbs the other: the sequence is what is ordered next, the owed count is what says
*done*. No figure is carried here, because a count restated is a count that ages — run the
oracle. The corpus behind that count and its decomposition are
`native-gate-port-remaining-corpus`'s, which is why the sequence's last member being blocked
never means the port has nothing to do.
discharge: sequence-remainder-is-not-a-finished-port  bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree | grep -F ', 0 owed'
ruling: a sequence remainder of one is not a finished port
