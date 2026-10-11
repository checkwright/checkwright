# SPEC amendment: wait-lead

Rule `background_no_record`'s bash exemption (2) holds a wait loop's body and the statements after the loop, and reads neither a statement ahead of the first loop nor a loop's condition. A backgrounded call whose work stands in either place is then a producer launched with no record, and rule `git_mutation_under_producer` has nothing to read for it. Run through the hook member on this tree, each with the harness's background flag set, these fell through:

- a lead: `make build; until grep -q DONE .tmp/worker.log; do sleep 5; done`, the same wait behind `sort -o out.txt in.txt;` and behind `bash worker.sh;`, and `make build & until grep -q DONE .tmp/worker.log; do sleep 5; done`;
- a condition: `until sort -c -o out.txt data.txt; do sleep 5; done; echo waited`, which `guard-tests/background-cases.tsv` pins, `until grep -q DONE .tmp/worker.log > out.txt; do sleep 5; done`, `until find . -name x -delete; do sleep 5; done` and `until make build & grep -q DONE .tmp/worker.log; do sleep 1; done`;
- a keyword read off an argument: `until grep -q DONE .tmp/worker.log; do sleep 5; done; echo for; make build; until grep -q X .tmp/worker.log; do sleep 5; done`, where the same call without `echo for;` blocks.

This amendment holds the lead to the tail's test and the condition to the writes its text shows, and states why a build in the condition cannot be told from a remote read.

## What changes

### (1) A statement ahead of the first loop takes the tail's test

Exemption (2) holds every statement outside a loop, ahead of the first as after it, to rule `bounded_wait`'s clause (d) test and to the statement-ending-`&` test, and a loop opens only at a keyword leading a statement {design-bearing} {user-facing: the entry's Deliverable — the exemption holding a leading statement to the test its tail already takes, a behavior change that widens a block}.

- **One statement test, called for both places.** The lead calls the predicate the tail calls and copies none of it, so a repair to either reaches both.
  - Where `wait-exemption-redirect-tail` has landed, the lead takes its repaired tail test as it stands.
  - Where it has not, the lead takes the tail test as this tree has it, and that unit's repair then reaches the lead through the one call, with no edit here.
- **Where a loop opens.** `until`, `while` or `for` opens a loop only as a statement's command word. The same word as an argument opens none, so the statements between it and the real keyword are lead or tail statements and are read.
  - Where `loop-keyword-as-argument` has landed, the opening keyword is read by the command-position test that unit gives the shared walk.
  - Where it has not, this delta's build writes that test for the three opening keywords, and that unit extends it to `do` and `done`.
- **No grant moves.** Rule `bounded_wait`'s arm (A) asks that the call's first statement be the loop, so every call it grants carries no lead, and rule `wait_no_producer` asks the same of its clause (b).

Replacement for the *Exemption (2), the call is a wait loop* bullet and its sub-bullets that deltas 1 and 2 touch; the sub-bullets not shown are unchanged. **Not yet applied.**

> - **Exemption (2), the call is a wait loop.** A backgrounded `until <cond>; do sleep N; done` is the waiter rule `bare_sleep` steers toward, not a producer. It is detected by that rule's `do … done` span walk, and holds only where the call does no work its text shows. Every span's body is `sleep` statements and nothing else, on rule `wait_no_producer`'s clause (b) test. Every statement outside a loop, ahead of the first or after it, is another such loop or passes rule `bounded_wait`'s clause (d) test, a read-only pipeline with the banner tolerance and inert redirect targets. Every condition statement carries inert redirect targets only. No statement but a loop itself ends in a bare `&`, which launches a child of its own whatever word leads it. And the declared-forms reader clears the whole call. Any other call owes the record. Rule `bounded_wait` draws the same waiter/producer partition to withhold a grant, so an exempted wait goes on to meet rule `wait_no_producer`, which asks whether anything can end it, and then that rule's clauses.
>     - **A loop opens at a keyword leading a statement.** `until`, `while` or `for` as a statement's command word opens a loop, and as an argument opens none: in `echo for; make build; until <cond>; do sleep N; done` the build is a statement ahead of the loop, and is read as one. A `for` header carries no command and is not read.
>     - **The lead and the tail are the measured wait's own shape**: a banner or a read, the loop, and then what the session reads once it ends, which is why rule `bounded_wait` grants a read-only tail. The declared-forms reader runs once over the whole call, as that rule's does, so a call the grant admits is never refused here.
>     - **The condition is held to the writes its text shows, and not to the read-only roster.** A redirect to a named file, a statement-ending `&` and a roster member's declared write or execute form each owe the record, as in `until sort -c -o <out> <in>; do sleep N; done`. Roster membership is not asked, because what a wait lawfully polls reaches past any read-only roster: a client reading a remote state writes nothing here, and a record would hold every commit for its lifetime.
>     - **The bare `&` is a third test, this rule's own.** The compound split does not cut on a lone `&`, the harness's measured class (§The reader and its views). So `sleep 5 & cargo build` is one segment led by `sleep`, and `echo waited & make build` one the banner tolerance admits. The exemption therefore applies the shell form's statement-ending-`&` predicate to every statement of the call but a loop itself. The two named predicates are not narrowed, so no other rule's verdict moves.
>     - **The honest limit:** a condition led by a command outside the read-only roster is not read, so a loop whose condition is itself the work stays exempt (`until make; do sleep 1; done`). The text does not tell a build from a remote reader, and the roster of names that would is consumer vocabulary, refused on the seam ground below. The pass is not silent: rule `bounded_wait`'s clause (c) withholds its grant from such a condition and no allow entry matches a loop, so the call falls through to the harness's permission decision.

### (2) A condition is held to the writes its text shows

Exemption (2) holds each loop's condition to three of the tail's four tests — inert redirect targets, no statement-ending `&`, and the declared-forms reader — and not to read-only roster membership {design-bearing} {user-facing: the entry's Deliverable — a condition that is a write told from one that reads remote state, and the section stating why a build cannot be; a behavior change that widens a block}.

- **Why the fourth test is not asked.** Holding the condition to the roster would refuse every wait on a command the roster does not carry, a remote client or a project's own status command, and the record it would then owe holds every commit for the wait's lifetime. That is the ground the section already states for leaving the condition unread, and it stands for the membership test alone.
- **Why a build is not told apart.** `make` and a remote client are each a command word outside the roster with no redirect and no declared form. Telling them apart takes a roster of producer names or of remote readers, which the rule's *No knob is minted* paragraph refuses as consumer vocabulary. So the build in a condition stays the stated limit, with its bound named: no grant, and a permission decision.
- **Refused: reading a gitignored redirect target as inert in a condition.** A wait that caches each poll into scratch would stay exempt, but the tail reads such a target as a write, and one redirect predicate for the whole call is what keeps a call exempt here from being a call a later reader of that predicate refuses. Such a wait pipes its poll (`<client> | grep -q <state>`) or takes the record.
- **Refused: reading a `GUARD_KIT_WRITE_BINS` member in a condition as a write.** `until mkdir <lock> 2>/dev/null; do sleep N; done` is a lock wait, and a record for it would hold every commit while it waits on the lock's holder. It would also give that knob a second reader.
- **No grant moves.** Rule `bounded_wait`'s clause (c) holds a condition to the read-only test with every redirect inert, its clause (0) refuses a statement-ending `&`, and its declared-forms reader runs over the whole call, so every condition arm (A) grants passes this delta's three tests.

The replacement text is delta 1's block: its *condition* and *honest limit* sub-bullets and the main bullet's condition sentence are this delta's.

### (3) The rows

Each decision table's loop-carrying rows are re-derived {mechanical}.

- **`guard-tests/background-cases.tsv`**, existing rows:
  - `until sort -c -o out.txt data.txt; do sleep 5; done; echo waited` flips from `fallthrough` to `block`;
  - `until make build; do sleep 1; done` keeps its `fallthrough`, now the limit's pinned row.
- **The same table, new rows, a firing and a non-firing case per place:**
  - a lead that works, `make build; until grep -q DONE .tmp/worker.log; do sleep 5; done`, a declared write form as the lead, and a lead ending in a bare `&` → block;
  - a banner lead and a read-only lead ahead of the same wait → fallthrough;
  - `echo for; make build;` ahead of the wait, and between two waits → block;
  - a condition carrying a redirect to a named file, and one carrying a statement-ending `&` → block;
  - a condition that pipes a command outside the roster into `grep -q` → fallthrough.
- **`guard-tests/cases.tsv`**, the shell form: a working lead ahead of a wait closed by its own `&` → block.
- **Every row carrying a `do … done` span under either backgrounding form is re-derived**, by `grep -n 'done' guard-kit/guard-tests/background-cases.tsv guard-kit/guard-tests/cases.tsv`. An `allow` row that turns `block` falsifies the two *No grant moves* claims and stops the build. The PowerShell tables are untouched: their exemption admits one loop as the whole skeleton.
- **The table's comment block** above the exemption's rows drops *The condition is not held* for the held and unheld halves.

Replacement for §Testing's two third-table bullets that name the loop shapes. **Not yet applied.**

> - a loop that works, under either keyword, a nest, a producer or a declared write form ahead of the wait or after it, a condition carrying a write its text shows, and a statement other than a loop ending in a bare `&` → block;
> - a `for` loop that only sleeps, a banner or a read ahead of the wait, and a loop whose condition is a command outside the read-only roster → fallthrough, the exemption's stated reach and limit;

## Producers and consumers

- **The narrowed exemption.** Producer: rule `background_no_record`'s bash arm, on every backgrounded `Bash` payload carrying a loop span, under either backgrounding form. Consumer: the session, through the block and its corrective, whose text is unchanged; and rule `git_mutation_under_producer`, which gains a record to read for a call relaunched in the canonical spelling.
- **Readers of exemption (2)'s shape**, by `grep -n 'exemption (2)\|already exempts' guard-kit/SPEC.md`: rule `wait_no_producer`'s clause (b) and rule `bounded_wait`'s clause (0). Each reads its own clauses, and neither changes a verdict.
- **Widening.** Both deltas widen a block, so the readers that red are the decision tables, in both directions (§Testing). Delta 3 names the flipped row and the claim an `allow` row would falsify.
- **Statements of the limit this amendment moves**, by `git grep -n -e 'condition is the work' -e 'condition is not held' -e 'before the first loop' -e 'condition is itself the work'`: `guard-kit/SPEC.md` at the exemption bullet, its honest-limit sub-bullet and §Testing; `guard-kit/guard-tests/background-cases.tsv`'s comment; `.workflow/release-declarations.md`'s bullet for the body-and-tail change; and the generated `docs/guard-kit/SPEC.md`.
- **Instruction surfaces naming the exemption**, by `git grep -n -e 'wait loop' -- delegation-kit/SPEC.md delegation-kit/templates .claude/agents lifecycle-kit/templates`: each tells a session to spell a wait inline and to record a producer. None promises the exemption to a call that works ahead of its loop or in its condition, and delegation-kit/SPEC.md §The delegation model's sentence that the guard reads a wait's condition for its form stays true.
- **Knobs.** None is minted and none gains a reader: the declared-forms reader already reads `GUARD_KIT_RO_BINS` and `GUARD_KIT_RO_FORMS` for this rule.

## Existing sections updated

- `guard-kit/SPEC.md` §The rule roster — the `background_no_record` item's exemption (2) bullet and its sub-bullets (deltas 1 and 2).
- `guard-kit/SPEC.md` §Testing — the third table's row lists (delta 3).
- `native/src/guard/rules/liveness.rs` — the exemption's test and its `spec:` binding (deltas 1 and 2).
- `guard-kit/guard-tests/background-cases.tsv`, `guard-kit/guard-tests/cases.tsv` — the rows and the comment block (delta 3).
- `docs/guard-kit/SPEC.md` — generated, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — a bullet for the widened block, and the body-and-tail bullet's *the loop condition is not held* clause rewritten to what delta 2 leaves unheld (deltas 1 and 2).

## Retired spellings

- None — the amendment narrows an exemption and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
