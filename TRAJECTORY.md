# TRAJECTORY.md — the override ledger

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

1. **The dependency floor collapses to git**, shelled out rather than embedded.
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
   Windows.

The objectives are the direction, not a claim about the tree: no user-facing
surface states the floor as reached before it is, and a platform an objective
names is declared **held** with the run that joins it, never left undeclared.
discharge: substrate-pivot  manual docs/install.md declares a git-only floor, and every platform objective 2 names reads held or supported there
ruling: the objectives  the substrate pivot  the project-trajectory pivot

## The rulings

**The customer payload withholds every kit's `SPEC.md` and `smoke/`, and a
shipped gate's `# spec:` pointer resolves to the site's SPEC mirror.** This
overrides the second shipped member of gate-sdk/SPEC.md §Consumer payload — the
SPEC section behind the pointer ships — and its restatements at docs/install.md
§What a gate discloses and installer/README.md §What this package is. Grounds:
the member's own reason, that a red gate must point at an explicable invariant,
is met by a resolvable pointer and the descriptor's one-line invariant;
canon-kit's finders prune vendored kit roots, so no consumer battery reads a
vendored SPEC; and the SPECs are the majority of the packed bytes while being
this repo's engineering record and its residual seam-leak surface. The work —
the packer exclusion, the section's amendment, the site's reference tier and the
README links — is the queue entry `payload-withholds-kit-specs`. Refused: a
generated extract of only the cited sections, which buys offline reading for a
reader the site already serves at the cost of a projection and a freshness gate.
discharge: payload-withholds-kit-specs  grep -x -- '- payload-withholds-kit-specs' TASK-QUEUE.md
ruling: the payload withholds kit SPECs  payload-withholds-kit-specs

**The compression contract at queue-kit/SPEC.md §check-queue-entry-budget is
held by authorship, and no gate that reds is owed for it.** Ruled at consult
2026-09-09 — operator-convened, lead-relayed, on the operator's own authority —
as costed and filed. This overrides the **Enforcement-first** delivery rule
(doctrine-kit/DOCTRINE.md, always-loaded at CLAUDE.md §Delivery doctrine),
under which a stated rule earns its gate in the unit that states it. Grounds:
the contract's relief is *compress by answering*, and the only machine-readable
shape a violation has — a deferred entry's counted extent falling in a commit
that also lands a ruling line — is equally the shape of that relief, so an arm
asserting it cannot tell the correct act from the defect. Recorded on this
ledger because the queue entry that carried it, `entry-compression-contract-unenforced`,
is closing, and a closed entry keeps its slug and nothing else. Refused: that
classifier under any framing that gives it an exit 1 — the two classes are
indistinguishable from the artifact, and a gate that cannot separate them
spends the correct act to catch the defect.
discharge: compression-contract-unenforceable  manual queue-kit/SPEC.md §check-queue-entry-budget records the refused classifier and its ground undated, giving the override a business-as-usual home
ruling: the 2026-09-09 consult ruling  costed and filed, no mechanism owed  entry-compression-contract-unenforced
