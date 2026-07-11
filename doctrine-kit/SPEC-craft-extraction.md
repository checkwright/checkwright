# SPEC amendment: craft-extraction

Copy-first extraction of the generic craft share of a private consumer's
handbook: agent working-style and git-operation habits, and the
dispatch/rename checklists. Per the provenance seam, everything lands
generalized — tool names become tool classes, attested incidents survive
anonymized, and identity/key material and product vocabularies stay in the
private source. Shape ruling (the question the queue entry deferred): the
habit roster earns the full doctrine-rule triple. The engineering-craft
register already *is* the lighter surface — it lives behind the link,
load-triggered, and is digest-exempt (doctrine-kit/SPEC.md
§check-doctrine-registration, assertion B), so new rules stale no consumer
digest; a second, rationale-free habits file was ruled out because a habit
stripped of its attested failure reads as style advice, and the *Under agent
work* line is the persuasion payload. Enforcement lines stay honest — the
register's "a convention, not yet a checkwright gate" precedent holds where
no gate is buildable.

## What changes

- **DOCTRINE.md §Engineering-craft rules grows rules 14–19**, each in the
  full statement / *Under agent work* / *Enforced by* triple:
  14. **Config edits are merges, not rewrites.** Edit a config/settings file
      with targeted edits, never a full-file write from a partial read; and
      validate with the format's own parser before judging a file corrupt —
      an apparent corruption is usually a parse by the wrong tool.
  15. **Re-verify volatile state before a git history rewrite.** Verify HEAD
      (`git log --oneline -3`) before an amend or squash; re-stage after a
      `git reset --soft` and verify staged content (`git show :<path>`)
      before committing — the soft reset keeps the old index snapshot; write
      a `commit -F` message file fresh in the same turn (prefer `-m` for
      short messages — a leftover file lands the wrong message with exit 0);
      rewrite the message when amending so it states the combined change.
  16. **Entering another repo's tree, read its governance first.** A
      cross-repo edit re-reads that repo's agent file / README and checks
      its branch freshness every time — its model drifts independently.
  17. **Naming: drop the qualifier the context supplies — only when every
      consumer has that context.** A name that travels into a flat namespace
      keeps its qualifier; default to the shorter form, reject the vacuous
      one.
  18. **Reuse a co-located consumer's data before designing a new path.**
      For an embedded or co-located actor, first ask whether it can read a
      co-located consumer's already-fetched data before minting a new
      stream or grant — a "which path" framing can hide "no path needed".
  19. **A resolver gate's flagged key is a fork, not a verdict.** A
      name-resolution gate that finds silent drops finds both dead config to
      remove and promised-but-unwired config to build; the two share a
      signature and only the owning SPEC distinguishes them — verify intent
      against the SPEC before sweeping.
- **guard-kit's generic ruleset gains one advisory rule** (the enforcement
  half of rule 15): `guard_advise` on `git commit --amend`, `git reset
  --soft`, and `git commit -F <path>`, steering with rule 15's checklist.
  Generic by the ruleset's own criterion — it encodes git-substrate
  behavior, no toolchain or product vocabulary; advisory posture because all
  three commands are legitimate. Ships with its decision-table row and
  usage-test cases per guard-kit's test layout.
- **delegation-kit gains `templates/dispatch-checklists.md`** — a second
  load-triggered template reached by one pointer line from
  `templates/agent-execution.md` (loaded only when a dispatch matches a
  checklist trigger, never resident). Contents, generalized: pre-dispatch
  importer survey for any deletion (grep every importer; use the consumer's
  consumer-scan tool where the toolchain ships one; zero consumers is the
  safe-to-delete signal; an out-of-scope importer rescopes the unit to
  migrate-then-keep); rename target collision check (grep the target
  namespace before the amendment is declared build-ready — a scope-stage
  step); rename dispatch brief (name where the old token is legitimately
  *mentioned* — historical `Old→New` arrows — separately from where it is
  *used*; a naive sweep collapses the record); component-list re-derivation
  (derive the dispatch set from the amendment body, never the queue entry's
  enumeration); sweep verification (re-grep in a fresh tool call after any
  bulk edit; match the gate's matcher class, not a looser one; a
  cross-language value needs emit-side and consume-side greps; prefer
  LSP find-references for a compiled-symbol sweep; a term gate has
  structural blind spots — allowlisted words, string/variant forms,
  modeling errors — the by-eye complement must cover); ownership of a
  shared removal (the dedup owner is the layer both sides can import
  without a cycle, not necessarily the domain owner); audit fan-out (the
  heavy cross-spec audit's real output is a gate backlog — mechanize the
  decidable classes, then run a light semantic pass thereafter; the width
  and journal rules are agent-execution's, cited not restated).

## Producers and consumers

- New doctrine rules: producer is this DOCTRINE.md edit, delivered by
  re-vendoring (doctrine-kit/SPEC.md §The doctrine deliverable); consumer is
  a session reaching the craft register through the installed link at its
  load triggers. Digest-exempt, so `check-doctrine-registration` needs no
  consumer digest edit and no fixture change — asserted at build by a green
  battery with CLAUDE.md untouched.
- Guard rule: producer is the consumer's existing template guard (this
  repo's `scripts/bash-guard.sh`) invoking the new lib rule on
  `PreToolUse(Bash)`; consumer is the session receiving the
  `additionalContext` steer. No new knob, no new config surface.
- Checklist template: producer is the vendored file; consumer is a
  dispatching session following agent-execution.md's pointer line at
  deletion/rename/audit dispatch time. The existing consumer binding shim
  is untouched — it reaches the new file through the template it already
  names.
- No new persistent state, stamps, or message fields anywhere in this unit.

## Existing sections updated

- doctrine-kit/DOCTRINE.md §Engineering-craft rules — rules 14–19 appended
  in register order.
- doctrine-kit/SPEC.md §The doctrine deliverable — the craft register's
  trigger description ("triggered by test and rename work") widens to name
  git-rewrite, config-edit, and dispatch moments.
- guard-kit/SPEC.md §The generic ruleset — the new rule's numbered entry;
  guard-kit/SPEC.md §Testing — its usage-test row.
- delegation-kit/SPEC.md §One template, a resident pointer — registers the
  second template and the pointer-line contract.
- delegation-kit/templates/agent-execution.md — gains the one pointer line
  (the template stays complete alone for its own protocol; the checklists
  are a reach-through, not a residency change).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls doctrine-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
