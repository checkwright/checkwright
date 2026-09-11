# TRAJECTORY.md — where Checkwright is going, and what is already ruled

This file is the project's **ruling record**: the objectives the work aims at
and the decisions the operator has closed on the way there. It is hand-authored
and answers *toward what, and under which closed rulings*. Close reads it for aged facts and fired discharge
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
discharge: ruling-record-admission-test  grep -q ruling-record-shrink-to-bau TASK-QUEUE.md || echo discharged
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
discharge: kit-spec-provenance-seam  grep -q kit-spec-provenance-seam-sweep-remainder TASK-QUEUE.md || echo discharged
ruling: a kit SPEC carries mechanism only  this project's provenance never ships
