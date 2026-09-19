# TRAJECTORY.md — the override ledger

<!-- undeclared-condition-exempt: the record's contract, defining what a ruling is; it directs no work and carries no condition of its own -->
This file carries two things and shrinks toward empty: the **objectives** of a
running pivot, and the operator's **rulings** — each an override of a
business-as-usual instruction until that instruction is updated. The terms are
lifecycle-kit/SPEC.md §The steering vocabulary; only the operator rules, and only
through `/consult`. Business as usual runs on the kits' own tooling, and a
decision with a business-as-usual home lands there, undated (the consult
binding's landing-surfaces slot). Close reads this file for fired discharge
conditions (lifecycle-kit/SPEC.md §The close-surface roster):

close-surface: TRAJECTORY.md advisory

It is not [ROADMAP.md](ROADMAP.md), a generated projection of the queue's
`[roadmap:]` tags that answers *what is next*; this file answers *toward what,
and under which overrides*.

**Three acts.** *Correcting an aged fact* inside a paragraph (a retired slug, a
moved count) is any session's, where an oracle settles the fact and the
paragraph directs nothing different afterwards. *Retiring on discharge* is any
session's: when a paragraph's `discharge:` oracle prints, or its `manual`
condition is met, the paragraph is deleted, not judged. *Reversing, narrowing or
re-ranking* a ruling is the operator's, through `/consult`, however well-grounded
the finding — contrary evidence is escalated, never annotated in place.

Each paragraph carries two declarations the ruling-staleness probe reads
(lifecycle-kit/SPEC.md §The ruling-staleness probe): `ruling: <name>` — the noun
other surfaces use for it, one or more, appended never rewritten — and
`discharge: <name> <oracle>`, a command whose printed output means the condition
fired, or `manual` followed by the condition.

## The objectives

The pivot, ruled by the operator: **port the battery to native binaries, and
reduce what a consumer must have to git alone.** The gate corpus is ported; the
objectives stay as the ground the remaining queue work is ranked against.

1. **The dependency floor collapses to git**, shelled out rather than embedded:
   git is the only unconditional member. A program a selected kit or arm spawns
   is that kit's declared requirement, probed only where it is selected, and is
   not the floor; the profiles a first install reaches (starter, prose) resolve
   to git alone, and no user-facing surface says "git alone" of a profile whose
   resolved floor is wider.
2. **All major operating systems, Windows included**; a bash-only install path
   fails the objective.
3. **Opacity is a goal**: a gate's source is withheld so it is executed rather
   than analysed by the agents it holds. This reversed the gate-dispatch seam's
   earlier no-opacity stance.
4. **Footprint is a first-class cost**: install, try, uninstall without growing
   the adopter's code base or dependency set.
5. **Non-technical adopters are a design constraint**: no step assumes a
   developer machine or a toolchain.
6. **The script-interpreter surface shrinks to the unavoidable**, and what is
   unavoidable is dual-implementable — bash for Linux and macOS, PowerShell for
   Windows — where the script stands between an adopter and the install or the
   battery. A script that guards a shell the harness already runs is written in
   that shell on every host: guard-kit's hook is bash everywhere because the
   harness runs its `Bash` tool, and its hook commands, under Git Bash on
   Windows, so a session with a call to guard already has the shell the guard
   needs. "PowerShell for Windows" for a guard means a guard over the harness's
   `PowerShell` tool, which is separate work, not a twin of the bash reader.

The objectives are the direction, not a claim about the tree: no user-facing
surface states the floor as reached before it is, and a platform an objective
names is declared **held** with the run that joins it, never left undeclared.
discharge: substrate-pivot  manual docs/install.md declares git as the only unconditional floor member, every other member conditional on a named kit or arm, and the starter and prose profiles resolving to git alone; and every platform objective 2 names reads held or supported there
ruling: the objectives  the substrate pivot  the project-trajectory pivot

## The rulings

**While the project is unlaunched, an enhancement enters an iteration only where
it cuts time-to-first-value, closes a trust or supply-chain gap, or produces
external proof; every other enhancement stays Deferred.** This overrides
lifecycle-kit/templates/stages/scope.md's pool ranking (**Rank the pool by what
deferral costs and what landing buys**), which admits whatever it ranks: the
filter runs ahead of the rank, and the rank orders what passes. It reaches
enhancements only — work adding no capability was never in its scope and needs no
exception recorded for it. The objectives above qualify under the first two arms
(install, try, uninstall on a git-only floor is time-to-first-value; a
digest-verified prebuilt payload is trust work), so a scope session weighs their
rungs' ordering, never their admissibility. It admits an explicit operator
exception, carried with its reason on the queue entry it admits. Grounds: the
bottleneck is observed external installs, not internal completeness. Refused: keeping the rule on the local-only brief, where it bound scope
through an untracked read and nothing ever checked its discharge.
discharge: enhancement-admission-filter  manual five external installs have been observed
ruling: the enhancement admission filter  the unlaunched enhancement rule

