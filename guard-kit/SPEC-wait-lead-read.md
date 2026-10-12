# SPEC amendment: wait-lead-read

Rule `wait_no_producer`'s clause (b) asks that the call's first statement be the wait loop, and rule `background_no_record`'s exemption (2) admits a banner or a read ahead of it, so the two rules read different shapes as a wait. Run through the hook member on this tree with the harness's background flag set, over the sandbox's one dead record:

- `until [ -f .tmp/marker ]; do sleep 30; done` blocks, and so does the same wait followed by `echo waited`;
- `echo start; until [ -f .tmp/marker ]; do sleep 30; done`, the same wait behind `ls .tmp;`, behind `grep -c x .tmp/worker.log &&` and behind `echo start |` fall through;
- in the foreground, `echo start; until [ -f .tmp/marker ]; do sleep 30; done &` falls through where the unled spelling blocks.

A file-condition wait no live producer can end therefore launches behind one banner. This amendment has the rule read its loop past the lead the exemption admits.

## What changes

### (1) Clause (b) reads the loop past a lead the exemption admits

Rule `wait_no_producer` finds its one wait loop wherever a banner or a read stands ahead of it, and declines on any other lead {design-bearing} {user-facing: the entry's Deliverable — the rule's wait-loop clause reading the loop past a lead that exemption admits, or the section stating the limit; read, a behavior change that widens a block}.

- **The lead takes the exemption's statement test, called.** Each segment ahead of the loop passes the one statement test exemption (2) holds a lead and a tail to, and none ends in a bare `&`. The rule calls that predicate and copies none of it, so the two rules cannot come to read different leads again.
- **Any other lead declines, and must.** A lead that works reaches this rule only where the call writes its own record, exemption (1): a producer launched in the canonical spelling and then awaited in the same call. Its record does not exist at `PreToolUse`, so clause (d) would hold over a producer the call is about to start, and blocking it would refuse the lawful form.
- **The tail is unchanged.** A statement after the loop has reached this rule through exemption (2) or (1) and is not read, as today.
- **No grant moves.** Rule `bounded_wait`'s clause (a) still asks that the first statement be the loop, so a led wait is granted nowhere, and that rule's read of the loop is not the one this delta changes.
  - A foreground led wait keeps falling through to a permission decision, the entry's recorded cost, which this delta does not take up.
- **One loop span still.** A second span declines as it does today, behind a lead or not.
- **The corrective's text is unchanged.**

Replacement for the `wait_no_producer` item's clause (b). **Not yet applied.**

> - **(b) Its one loop is a wait loop**: a `while`/`until` loop with exactly one balanced `do … done` span, found by the ruleset's one shell-keyword walk (rule `bare_sleep`), its body `sleep` statements and nothing else. A statement ahead of the loop is read past where it passes the statement test rule `background_no_record`'s exemption (2) holds a lead to, a banner or a read, so both rules read one shape as a wait: `echo start; until [ -f <m> ]; do sleep N; done` blocks as the bare wait does. Any other lead declines, since a call that launches its own producer ahead of the wait writes a record no `PreToolUse` read can see yet.

### (2) The rows

The led spellings are pinned in each table that holds the rule {mechanical}.

- **`guard-tests/background-cases.tsv`**, in rule `wait_no_producer`'s section, each with the flag set:
  - `echo start; until [ -f .tmp/marker ]; do sleep 30; done`, `ls .tmp; until [ -f .tmp/marker ]; do sleep 30; done` and `grep -c x .tmp/worker.log && until [ -f .tmp/marker ]; do sleep 30; done; echo waited` → block;
  - `echo start; until [ -f .tmp/a ]; do sleep 30; done; until [ -f .tmp/b ]; do sleep 30; done` → fallthrough, the second span.
- **`guard-tests/cases.tsv`**, the shell form: `echo start; until [ -f .tmp/marker ]; do sleep 5; done &` → block.
- **`gate-tests/git-mutation-under-producer.test.sh`**, beside the rule's record-set pair: the banner-led wait blocks over an empty record set and does not block beside a record naming a live PID.
- **Every row carrying a statement ahead of a `do … done` span under either backgrounding form is re-derived**, by `grep -n -e '; *until ' -e '; *while ' -e '&& *until ' -e '| *until ' guard-kit/guard-tests/background-cases.tsv guard-kit/guard-tests/cases.tsv`. The rows that lead a `grep`-condition wait keep their verdict, since clause (c) declines on them. An `allow` row that turns `block` falsifies *No grant moves* and stops the build.
- **Where `guard-spelling-variant-oracle` has landed**, the lane derives re-spellings of the new rows and holds them to `block`. The lead's banner and read are told off the skeleton, so a quoted lead word is no banner and its derived row falls through: each such pair the lane then fails takes a limit row under rule `wait_no_producer`, in this unit's commit. Where it has not, no limit row is owed.

Replacement for §Testing's `wait_no_producer` bullet *a backgrounded file-test wait → block, in either polarity and over tests joined by `&&`*. **Not yet applied.**

> - a backgrounded file-test wait → block, in either polarity, over tests joined by `&&`, and behind a banner or a read;

Where `bounded-wait-piped-loop` has landed, that bullet already ends *and piped into a reader*, and this replacement keeps the clause: *in either polarity, over tests joined by `&&`, behind a banner or a read, and piped into a reader*.

## Producers and consumers

- **The widened block.** Producer: rule `wait_no_producer`, on every backgrounded `Bash` payload carrying one loop span, under either backgrounding form. Consumer: the session, through the block and its corrective.
- **Readers of the shape**, by `grep -n -e 'first statement' guard-kit/SPEC.md`: rule `bounded_wait`'s clause (a), which is the grant's and unchanged, and this rule's clause (b), which delta 1 rewrites.
- **Instruction surfaces naming the rule**, by `git grep -n -e 'wait_no_producer' -- delegation-kit lifecycle-kit CLAUDE.md .claude`: each states that a backgrounded wait on a file condition is refused while no record names a live producer, and none promises a pass to a led one.
- **Widening.** The delta widens a block, so the readers that red are the decision tables, in both directions (§Testing), and delta 2 names the stop.
- **Knobs.** None is minted and none gains a reader.

## Existing sections updated

- `guard-kit/SPEC.md` §The rule roster — the `wait_no_producer` item's clause (b) (delta 1).
- `guard-kit/SPEC.md` §Testing — the third table's `wait_no_producer` bullet (delta 2).
- `native/src/guard/rules/liveness.rs` — the rule's loop read and its `spec:` binding (delta 1).
- `guard-kit/guard-tests/background-cases.tsv`, `guard-kit/guard-tests/cases.tsv` — the rows (delta 2).
- `guard-kit/gate-tests/git-mutation-under-producer.test.sh` — the record-set pair's led case (delta 2).
- `guard-kit/guard-tests/variant-limits.tsv` — the limit rows the led rows' re-spellings need, where that table exists (delta 2).
- `docs/guard-kit/SPEC.md` — generated, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — the `wait_no_producer` bullet gains the led wait (delta 1).

## Retired spellings

- None — the amendment widens a block and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
