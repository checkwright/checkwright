# SPEC amendment: msgfile

At `door-binding-sweep`'s build, a second batch session ran
`git commit -F .tmp/commit-msg.txt`, picked up the first batch's leftover file,
and landed the first batch's whole message on its own commit at exit 0. `.tmp/`
survives across the sibling sessions one stage dispatches, so a conventional
message-file name is shared state between them. The queue entry
(`sibling-stage-sessions-collide-on-a-shared-scratch-commit-message-file`) asks
this stage to rule between two remedies: a lifecycle-kit path derivation for the
file, or a doctrine line obliging a session to read back what it committed.

**Ruled: the doctrine line.** It is written as a re-phrase of the doctrine rule
that already governs `git commit -F`, rather than as a new rule. The message
travels in the command, so no file exists for a sibling to share, and a read-back
covers the case where a file is still used. The rule spans doctrine-kit, which
owns it, and guard-kit, whose advisory prints it, so this amendment sits at the
repo root.

## What changes

### (1) Doctrine rule 18 carries the message in the command

doctrine-kit/DOCTRINE.md rule 18 (*Re-verify volatile state before a git history
rewrite*) already says to "write any `git commit -F` message file fresh in the
same turn (prefer `-m` for a short message — a leftover file lands the wrong
message with exit 0)". **{mechanical}** The attested session broke that clause,
with guard-kit rule 21's advisory printing it at the call, so the clause is
strengthened rather than duplicated. **Replacement text** — **Not yet applied** —
for that clause:

> carry the commit message in the command — `-m`, or `-F -` fed by a quoted
> heredoc — rather than in a message file, because scratch outlives the session
> that wrote it and a sibling session committing from the same name lands the
> wrong message with exit 0; where a file is unavoidable, write it in the same
> command as the commit and read the landed message back
> (`git log -1 --format=%B`)

The rule's *Under agent work* paragraph gains one clause after "a rewrite acts on
what the working tree *now* holds, not what the transcript last recorded" —
**Not yet applied**: "and a message file under scratch is state another session
may have written last".

**The path derivation was weighed and declined, on the case it would have to
fix.** Deriving the file's name the way the resume journal's path is derived
would give it a lifecycle-kit name, and that name is per stage: several sessions
of one stage **append to one journal** by design (lifecycle-kit/SPEC.md §The state
machine). The attested collision was two batches of one stage, so a stage-derived
name hands both of them the same file. A name keyed on the session id would
separate them, but it would mint a lifecycle-kit path convention for a file the
protocol does not need.

### (2) Guard rule 21's advisory prints the new clause

guard-kit rule 21 (`guard_rule_git_rewrite`) prints rule 18's checklist when a
commit carries `--amend`, `-F` or `--file`, or on a `git reset --soft`.
**{mechanical}** Its trigger is unchanged. Its text follows delta 1:

- guard-kit/SPEC.md §The generic ruleset, rule 21 — "write any `commit -F` message
  file fresh this turn" becomes "carry the message in the command (`-m`, or
  `-F -` from a heredoc), and read a file-borne message back after the commit" —
  **Not yet applied**.
- `guard-kit/lib/guard.sh`, the `guard_advise` string in `guard_rule_git_rewrite` —
  "write any 'git commit -F' message file fresh this turn — prefer '-m' for a
  short message, since a leftover file lands the wrong message with exit 0"
  becomes "carry the message in the command ('-m', or '-F -' from a heredoc) —
  a scratch message file may be another session's, and a leftover lands the wrong
  message with exit 0; if you must use a file, write it in this same command and
  read the result back with 'git log -1 --format=%B'".

## Producers and consumers

- **The doctrine clause (delta 1).** *Producer:* doctrine-kit's DOCTRINE.md, which
  this repository reads through the vendored doctrine and the pointer in its
  agent file. *Consumer:* any committing session, at the commit. Rule 18 carries
  no `Digest:` trailer (the digest set is the maintenance rules, probe
  `grep -n "Digest:" doctrine-kit/DOCTRINE.md`), so `check-doctrine-registration`'s
  assertion F, which compares digest bullets, has nothing to compare here.
- **The advisory (delta 2).** *Producer:* `guard_rule_git_rewrite` on every Bash
  `PreToolUse` call a wired bash guard sees. *Consumer:* the session about to
  commit. The guard test lane compares verdicts, not message text. Probe:
  `grep -rn "commit -F\|--file" guard-kit/guard-tests/` matched two `advise` rows
  (`cases.tsv` 65 and 379), and both keep their verdict, since the trigger is
  unchanged.
- **Surfaces carrying the old clause.** Probe:
  `grep -rn "message file fresh\|prefer .-m." --include=*.md --include=*.sh --include=*.rs --include=*.tsv .`
  matched doctrine-kit/DOCTRINE.md, guard-kit/SPEC.md, `guard-kit/lib/guard.sh` and
  the two generated `docs/` mirrors.

## Existing sections updated

- doctrine-kit/DOCTRINE.md rule 18 — the clause and the *Under agent work* addition (delta 1).
- guard-kit/SPEC.md §The generic ruleset, rule 21 — the paraphrased checklist (delta 2).
- `guard-kit/lib/guard.sh` — the rule-21 advisory string (delta 2).
- `.workflow/release-declarations.md` — one `## Behavior changes` bullet led by
  `**doctrine-kit rule 18 and guard-kit rule 21**`: the commit message travels in
  the command, and a file-borne message is read back; nothing to change unless you
  copied the doctrine out (deltas 1 and 2).
- `docs/doctrine-kit/DOCTRINE.md` and `docs/guard-kit/SPEC.md` — the generated mirrors, regenerated by their arm (all deltas).

## Retired spellings

- None — no name, knob, path or tag is retired; a clause's wording changes.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
