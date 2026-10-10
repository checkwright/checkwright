# SPEC amendment: wait-body

Rule `background_no_record`'s bash exemption (2) exempts a backgrounded call from the liveness record when it carries a `do … done` span, whatever the span's body runs. A backgrounded loop that does work is then a producer launched with no record, and rule `git_mutation_under_producer` has nothing to read for it. Run through the member on this tree, each with the harness's background flag set, these fell through: `until [ -f .tmp/marker ]; do touch .tmp/marker; sleep 5; done`, which `guard-tests/background-cases.tsv` pins; `while true; do cargo test; done`; and `for f in a b; do cargo build; done`. The PowerShell exemption already holds its body to one sleep statement. A statement after the loop is the same hole: `until grep -q done .tmp/log; do sleep 2; done; cargo build` fell through too. This amendment holds the bash exemption to loop bodies that only sleep and to a tail that only reads.

## What changes

### (1) Exemption (2) is held to loops that only sleep and a tail that only reads

The bash exemption (2) of rule `background_no_record` holds only where every `do … done` span of the call has a body of `sleep` statements and nothing else, and every statement after the first loop that is not itself a loop passes rule `bounded_wait`'s clause (d) test {design-bearing} {user-facing: the entry's Deliverable for the body; for the statements after the loop, operator direction, 2026-10-10, lead-relayed, not a ruling — the body plus the statements after the loop held to the read-only test, the condition not held}.

Replacement for the item's *Exemption (2), the call is a wait loop* bullet. **Not yet applied.**

> - **Exemption (2), the call is a wait loop.** A backgrounded `until <cond>; do sleep N; done` is the waiter rule `bare_sleep` steers toward, not a producer. It is detected by that rule's `do … done` span walk, and holds only where the call does no work of its own: every span's body is `sleep` statements and nothing else, on rule `wait_no_producer`'s clause (b) test, and every statement after the first loop is another such loop or passes rule `bounded_wait`'s clause (d) test, a read-only pipeline with the banner tolerance and inert redirect targets. Anything else takes the record like any launch. **The condition is not held:** a wait on a command that reads a remote state writes nothing here, and a record would hold every commit for its lifetime. Rule `bounded_wait` draws the same waiter/producer partition to withhold a grant, so an exempted wait goes on to meet rule `wait_no_producer`, which asks whether anything can end it, and then that rule's clauses.

- **Every span, whatever its keyword.** A call of two wait loops in sequence stays exempt, and a `for` loop whose body only sleeps stays exempt with them: the test reads bodies, as the walk reads spans and never the loop keyword. A loop nested in a body is not a `sleep` statement, so a nest is not exempt.
- **The tail is the measured wait's own shape.** A wait is launched as the loop and then what the session reads once it ends, which is why rule `bounded_wait` grants a read-only tail. The exemption takes that clause's test and no second one, so a tail the grant admits is never refused here.
- **A work loop's lawful spelling is the canonical recorded launch** the block already prints, the loop standing as its `<command>`. Rule `wait_no_producer` declines on such a loop by its own body clause, so the recorded launch meets no second block.
- **One body test and one tail test.** Each is the named rule's own predicate, called, so a later change to either cannot leave a call exempt from the record and unread by the rule that owns the test.
- **The honest limit, stated in the item:** a loop whose condition is itself the work stays exempt (`until make; do sleep 1; done` fell through the member), and so does a statement standing before the first loop.

### (2) The rows

Each decision table's loop-carrying rows are re-derived {mechanical}.

- **`guard-tests/background-cases.tsv`**: the row `until [ -f .tmp/marker ]; do touch .tmp/marker; sleep 5; done` flips from `fallthrough` to `block`, and §Testing's list of rule `wait_no_producer`'s rows drops *a body running something besides `sleep`* from its fall-through pair, which that row was. A harness-form `while true; do make build; done` blocks.
- **`guard-tests/cases.tsv`**, the shell form: a work loop ending in a bare `&` blocks, and the existing `until grep -q DONE .tmp/smoke.log; do sleep 5; done &` row keeps its fall-through.
- **Every row carrying a `do … done` span under either backgrounding form is re-derived**, by `grep -n 'done' guard-kit/guard-tests/background-cases.tsv guard-kit/guard-tests/cases.tsv`; the PowerShell tables are untouched, their exemption being already this narrow.
- **The tail.** In `background-cases.tsv`, a wait followed by a producer (`until grep -q DONE .tmp/worker.log; do sleep 5; done; make build`) blocks, and the same wait followed by `echo waited` or by a `grep` keeps the verdict it has.
- **Rule `wait_no_producer`'s rows keep their verdicts**: its body clause is the test this delta reuses, unchanged.

## Producers and consumers

- **The narrowed exemption.** Producer: rule `background_no_record`'s bash arm, on every backgrounded `Bash` payload carrying a loop span. Consumer: the session, through the block; and rule `git_mutation_under_producer`, which gains a record to read for a work loop relaunched in the canonical spelling.
- **Readers of exemption (2)'s shape**, by `grep -n 'exemption (2)' guard-kit/SPEC.md`: rule `wait_no_producer`'s clause (b), which says the shape it reads *is the shape exemption (2) exempts* and becomes exactly true for a single loop; rule `bounded_wait`'s clause (0), which is indifferent to the harness flag and reads its own body clause. Neither changes a verdict.
- **Widening.** The delta widens a block, so the readers that red are the decision tables, in both directions (§Testing), and delta 2 names the flipped row.
- **Instruction surfaces naming the exemption**, by `git grep -n 'exempt' -- delegation-kit/templates/agent-execution.md .claude/agents`: each tells a session to spell a wait inline and to record a producer, and none promises the exemption to a loop that works.

## Existing sections updated

- `guard-kit/SPEC.md` §The rule roster — the `background_no_record` item's exemption (2) bullet (delta 1).
- `guard-kit/SPEC.md` §Testing — the third table's row lists (delta 2).
- `native/src/guard/rules/liveness.rs` — the exemption's test and the shared body predicate (delta 1).
- `guard-kit/guard-tests/background-cases.tsv`, `guard-kit/guard-tests/cases.tsv` — the rows (delta 2).
- `docs/guard-kit/SPEC.md` — generated, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — a bullet for the widened block (delta 1).

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
