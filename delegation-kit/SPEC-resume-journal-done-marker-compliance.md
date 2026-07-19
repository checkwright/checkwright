# SPEC amendment: resume-journal-done-marker-compliance

<!-- Owning component: delegation-kit (owns §Resume journal — agent writes,
     supervisor deletes). Pairs with the queue entry tagged
     [spec: SPEC-resume-journal-done-marker-compliance.md]. Rescoping-only: it
     corrects the interpretation of an existing signal, adds no name. -->

## What changes

The resume-journal recovery contract's `DONE`-absence clause is rescoped. Today
delegation-kit/SPEC.md §Resume journal states unconditionally: "A journal present
*without* `DONE` means that unit was interrupted — resume from it." In practice
agents routinely complete without appending `DONE`, so read universally the
clause **false-reads a completed run as interrupted**.

The corrected contract keys the recovery signal on **whether the supervisor
consumed the agent's return** — delegation-kit's own supervisor/agent vocabulary,
no lifecycle-specific terms:

- **Return consumed.** When the supervisor received the agent's return message and
  ran its post-commit verification (§Validate after every agent commit — re-run
  the gates plus the validate battery over the agent's commit), that return **plus
  the verification is the recovery contract**: it attests completion directly, so
  the `DONE` marker is **redundant** and a journal without `DONE` does **not** imply
  interruption. This is the ordinary completion path.
- **Cold journal read.** When the supervisor finds a journal but never consumed a
  return — the agent's session died before returning (a background agent whose
  sandbox died, a crash, a timeout) — the marker is the only signal, and here the
  original reading holds: **no `DONE` = interrupted, resume from it.** This
  cold-read case is what the marker exists for.

The inline-findings requirement is **unchanged** and now reads coherently: a
finding lands in the journal *inline as it is confirmed* because in the cold-read
arm the return did not survive, so a pointer-only journal would make a would-be
`DONE` lie about recoverability. The marker stays a real signal exactly where the
return is absent, and is redundant exactly where the return is present.

## Producers and consumers

Rescoping-only — no new state, event, field, or interface; the amendment
re-conditions the reader of an existing signal.

- **The `DONE` marker.** Producer: the mutating agent, appending it on success —
  unchanged. Consumer: the supervisor, on recovery. What changes is the
  **consumer's interpretation rule** — the supervisor reads `DONE`-absence as
  "interrupted" **only in the cold-read arm** (no return consumed); in the
  return-consumed arm it reads completion from the return plus its post-commit
  verification and does not consult the marker.
- **The consumed return + post-commit verification** is the recovery signal for
  the ordinary path. Producer: the agent's return message and the commit it
  produced; consumer: the supervisor, which already runs the
  §Validate-after-every-agent-commit checkpoint — no new mechanism, the
  verification the supervisor performs anyway is named as the completion attest.

## Existing sections updated

- **delegation-kit/SPEC.md §Resume journal — agent writes, supervisor deletes**
  (the last sentence, ~:144-145) — rescope the `DONE`-absence clause to the two
  arms above; keep the inline-findings rule, now framed as the cold-read arm's
  insurance.
- **delegation-kit/templates/agent-execution.md** (~:52-53) — the operational
  template copy restates the same clause ("A journal still present *without* a
  `DONE` marker means that unit was interrupted — resume from it"); rescope it
  identically so the load-triggered protocol and its owning SPEC stay in lockstep.
- **docs/delegation-kit/SPEC.md** — generated mirror of the edited SPEC;
  regenerate with `bash scripts/gen-docs-mirror.sh --write`
  (`check-docs-mirror-fresh` byte-gates its freshness).

**Reviewed and deliberately not edited** — `.claude/agents/stage-session.md`
§Standing dispatch policy *points at* delegation-kit/SPEC.md §Resume journal for
the journal mechanics; it restates no invariant, so the pointer stays valid with
no edit. Per the ruling this amendment is plain (b) — **removal/rescoping only**:
it does **not** enforce a non-skippable `DONE` append, and it does **not**
mechanize the supervisor's post-commit completeness verification into a gate.

<!-- Note for close: the ruling codifies the supervisor's manual post-commit
     completeness verification as the recovery contract. If that manual step is
     later judged worth retiring, mechanizing it (a return/commit-parity check)
     is its own costed unit — it widens this unit's envelope beyond scope's
     filing, so it is filed at close, not folded in here. -->

## Definition of Done

- [ ] **Causal completeness** — rescoping-only; the re-conditioned reader (the
      supervisor's `DONE`-absence interpretation) and the named completion signal
      (consumed return + post-commit verification) are stated, and the change is
      propagated to every surface restating the clause.
- [ ] **Merged with no information lost** — the two-arm recovery contract
      integrated into delegation-kit/SPEC.md §Resume journal (not appended) and
      mirrored into the template copy; both read coherently for a reader who never
      saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      delegation-kit (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `git grep` for the `DONE`-absence clause confirms
      every restatement (SPEC, template, docs mirror) carries the rescoped form.
- [ ] **Gaps filed** — the mechanization follow-up noted above filed at close as
      its own costed unit if the manual step is judged worth retiring; any other
      cross-component gap filed as debt.
