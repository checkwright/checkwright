# SPEC amendment: stdout-reader

The gate binary panics when its stdout reader has gone before a write: a subcommand or arm piped into a filter that exits early dies in the standard print macro with a panic trace and a status that is none of its own. Run with the reader already closed, `--list`, `--emit-queue-edges`, `--emit-knob-roster`, `--source-stamp` and the member `check-exec-bit` each exit 101 on the trace. This amendment states the rule and its holder.

## What changes

### (1) A departed stdout reader changes no status

A new paragraph in gate-sdk/SPEC.md §The non-gate arm, placed after the exit-contract paragraph and its member list, stating the rule below for every subcommand and arm the binary answers, and one sentence in §Output contract pointing a gate's reader at it {design-bearing} {user-facing: the entry's Deliverable, an arm whose reader has gone exits with no panic trace on a status that still tells a landed write from a refused one}.

- **The rule.** A write to stdout that fails because its reader has gone is dropped, and so is every later one. The process runs on and exits with the status it would have returned with the reader present, and prints no panic trace.
- **It is the process's rule, held once where the binary starts, never per arm.** The crate's print sites number in the thousands, and a rule per site is one every new arm has to remember.
- **Dying at the first failed write is refused.** A fixed status there cannot tell a move that landed from one refused after its first line, and it loses `--queue`'s 1, the move written under a red post-check, exactly when the post-check line is the write that fails. A gate's verdict is its status, and a verdict that changes with the reader's patience is no verdict. Restoring the signal's default disposition is refused on the same ground and one more: it has no Windows form.
- **Only a departed reader is dropped.** Any other stdout failure, a full disk under a redirect among them, keeps the behaviour it has: a document cut short by one must not exit clean.
- **Stated cost.** An arm piped into a filter that exits early runs to its end unread, the battery under `--run` being the long instance. A caller that needs the whole document reads its own reader's status.
- **Out of reach.** A closed stderr is outside the rule.

Sentence for §Output contract. **Not yet applied.**

> The status is the verdict whether or not stdout was read: a reader that has gone changes neither (§The non-gate arm).

### (2) The direct stdout handles follow the rule

Each site writing through a stdout handle of its own, rather than a print macro, drops a departed-reader failure as delta 1 states {mechanical}. Probe: `grep -rn "io::stdout()" native/src`, seven sites. Five already discard every write result and need no edit: `runner.rs`, `emit/rewrite.rs`, `emit/scratch_run.rs`, `emit/queue_verbs.rs`'s flush and `hook/shell_guard.rs`. Two do not:

- `emit/lesson_sink.rs` turns a failed re-emit of the sink's output into a refusal, so a sink that ran reads as one that could not. It returns the sink's own status when the failure is a departed reader, and keeps the refusal for any other.
- `gates/mod.rs`, the observation block's writer, panics on a failed write. It drops a departed-reader failure and keeps the panic for any other, since a truncated observation block read as whole would pass a member's declared roots unobserved.

### (3) A crate test holds it on every platform leg

A crate test runs the built binary with a stdout whose reader is already closed and asserts the status and an empty stderr {design-bearing}. It is a crate test rather than a shell suite because the failure has a Windows form, which only the crate's Windows legs can run (§check-crate-arms). Its cases, each on input it builds:

- a member whose verdict is clean exits 0, and one whose verdict is a finding exits 1;
- an `--emit-` arm that prints a document exits 0;
- a `--queue` move on a scratch queue lands its write and returns the status the same move returns with a reader, under a green post-check and under a red one;
- a refused `--queue` move still exits 2.

## Producers and consumers

- **The dropped write.** Producer: any stdout write of the process after its reader closed, reached by a pipeline whose downstream exits first; `--queue done <slug>` into a one-line filter is the observed instance. Consumer of the resulting status: the invoking shell or session, and a pipeline under `pipefail`, which now reads the arm's own status where it read 101.
- **Readers of a status that changes.** A caller that treated any non-zero status of a piped arm as failure now sees 0 where the arm succeeded unread. Probe: `grep -rn "101" gate-sdk/SPEC.md gate-sdk/lib gate-sdk/bin`, which finds no reader keyed on the panic status. `check-pipe-membership` (§check-pipe-membership) holds shell files whose producer is a shell builtin and is unaffected.
- **Harness-integration arms.** A hook member whose harness has gone drops its envelope and returns its protocol status, which no caller is left to read.
- **Every member's satisfying value (point 6).** The obliged corpus is the stdout write sites. The print-macro sites take delta 1's one mechanism. The seven direct handles are enumerated in delta 2, five satisfied as they stand and two with their change named.

**Inferred, cannot run before build:** that Windows reports a closed reader under the error kind Unix does, so one mechanism and one test body serve both — no Windows host is reachable from this tree, and delta 3's test on the Windows legs of the closing push is the oracle.

## Existing sections updated

- `gate-sdk/SPEC.md` §The non-gate arm — the rule (delta 1).
- `gate-sdk/SPEC.md` §Output contract — the pointer sentence (delta 1).
- `native/src/main.rs` — where the process holds the rule (delta 1).
- `native/src/emit/lesson_sink.rs`, `native/src/gates/mod.rs` — the two handles (delta 2).
- `docs/gate-sdk/SPEC.md` — generated, stale once delta 1 lands (all deltas).
- `.workflow/release-declarations.md` — a Behavior-changes bullet, since a piped arm's status changes for an adopter (delta 1).

## Retired spellings

- None — no delta removes or renames a name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **The probe re-run** — the five arms named at the head exit their own status with an empty stderr under a closed reader.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Entry moved** — `arm-stdout-close-panic` moves to Done in the merge commit, which lands before the drain stage. The Windows legs first run delta 3's test on the closing push, whose watched runs read it; a red there is a finding of that push, and the set's push need stays the closing push alone.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
