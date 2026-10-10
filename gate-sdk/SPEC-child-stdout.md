# SPEC amendment: child-stdout

gate-sdk/SPEC.md §The non-gate arm states *A departed stdout reader changes no status* for every subcommand and arm, and an arm that hands a child the binary's own stdout does not keep it. Run on Linux: `--scratch-run` on a script printing 20000 lines exits 0 with a reader and 141 piped into `true`; a script that sleeps, prints one line and exits 7 exits 141 with the reader gone and 7 with it present; a script that prints nothing and exits 7 exits 7 either way. `--run-validate` with a pre-hook that prints one line exits 2 on *pre-hook failed* with the reader gone and runs its suites with one. This amendment bounds the rule to the process's own writes and says what an arm relays.

## What changes

### (1) The rule's reach is the process's own writes

The rule's lead sentence and its *Out of reach* bullet in gate-sdk/SPEC.md §The non-gate arm are rewritten so the rule covers the writes this process makes, and a bullet states what an arm does with a child it handed the inherited stdout {design-bearing}. No behaviour changes: the text is brought to what the binary does.

Replacement for the lead sentence. **Not yet applied.**

> **A departed stdout reader changes no status of the process's own.** For every subcommand and arm the binary answers, a write this process makes to stdout that fails because its reader has gone is dropped, and so is every later one. The process runs on, exits with the status it would have returned with the reader present, and prints no panic trace.

Replacement for the *Out of reach* bullet. **Not yet applied.**

> - **A child handed the inherited stdout answers for its own writes.** An arm that spawns one (`Sink::Inherit` in `native/src/proc.rs`) relays or reads the child's status as it returns it. A child that is a build of this binary (the running executable, the binary `GATE_SDK_NATIVE_BIN` names, the packaged artifact) keeps the rule, since the rule is held where that build starts. Any other child meets the departed reader as its platform delivers it, on Unix a death by the pipe signal, and the arm cannot tell the status that child would have returned. So an arm that relays its child's status returns the death, and an arm that reads the status as a verdict reads the death as that verdict: `--run-validate` refuses on a pre-hook that died printing.
> - **Routing the child through a pipe the process drains is refused.** The child's two streams would no longer share one description, so its stdout would reach a merged reader later than the stderr written after it, the reordering §run-gates refuses for a gate's own report. A descendant holding the pipe open past the child's exit would hold the arm with it, and a child started from a terminal would lose it.
> - **Starting the child with the pipe signal ignored is refused.** Its failed write then returns an error the child handles its own way, a shell script under `set -e` exiting on it, so the status still changes, with a diagnostic added. It has no Windows form.
> - **Out of reach.** A closed stderr is outside the rule.

The call sites behind the bullet, by `grep -rn "Sink::Inherit" native/src` and each site's program read off its line. A child that is a build of this binary: `emit/hook_launcher.rs`, `installer/demo.rs`, `emit/install_hooks.rs`'s identity call and `emit/queue_verbs.rs`'s post-check. Any other child: `emit/scratch_run.rs`, `emit/run_validate.rs`'s pre-hook, `emit/run_consumer_smoke.rs`, `emit/demo.rs`'s kit installer, `emit/install_hooks.rs`'s consumer shadow and `emit/foreign_shells.rs`. `run_to_in` is the one spawn in `native/src/proc.rs` that leaves a child's stdout unset, by `grep -n "stdout(" native/src/proc.rs` read against its spawn functions.

### (2) The holder names both halves

The holder bullet of the same section gains the two child cases, and `native/tests/closed_reader.rs` gains one {design-bearing}.

- **A child that is a build of this binary** is already held: the `--queue` move's post-check spawns the running binary on the inherited stdout, and the two `--queue` cases keep their status with the reader gone. The bullet says so; the test needs no edit.
- **Any other child** takes a new case, Unix legs only, on `--scratch-run` over two scripts it writes into the scratch directory's `.tmp`, which it creates, since `--scratch-run` refuses a target outside `GATE_SDK_TMP_DIR` and the harness's scratch has none. One prints nothing and exits on a code of its own, and the arm returns that code with the reader gone, on an empty stderr. The other prints after its reader has gone, and the arm returns the status the pipe signal's death maps to in `exit_code`. The second assertion is the bound, written so that a later change making the rule total reds here and brings this section with it.

Sentence added to the holder bullet. **Not yet applied.**

> A `--queue` move's post-check is the child that is this binary, and on the Unix legs a `--scratch-run` child holds the other half: its own exit code where it wrote nothing, the pipe signal's status where it wrote.

## Producers and consumers

- **The relayed death.** Producer: a child other than this binary writing to the inherited stdout after its reader closed, reached by an arm of delta 1's second list piped into a filter that exits first. Consumer: the invoking shell or session, and a pipeline under `pipefail`, which reads the arm's status.
- **Readers of the rule's text.** `grep -rn "departed.\(stdout.\)\?reader\|reader that has gone\|reader has already gone" gate-sdk/SPEC.md native/src native/tests` finds §The non-gate arm, the crate's `spec:` comments, the pointer sentence in §Output contract and the header of `native/tests/closed_reader.rs`. The pointer sentence speaks of a gate's verdict, which is the process's own status, and stands.
- **The new case's reach.** `--scratch-run` resolves an interpreter for the script it is handed (guard-kit/SPEC.md §scratch-run); the case runs where the crate's Unix legs carry one.

**Inferred, cannot run before build:** what a child other than this binary returns on Windows when it writes to a departed reader, where no pipe signal exists — no Windows host is reachable from this tree, so the case is Unix-only and the bullet names the platform's own delivery without stating the Windows status.

## Existing sections updated

- `gate-sdk/SPEC.md` §The non-gate arm — the lead sentence, the child bullets and the holder sentence (deltas 1 and 2).
- `native/tests/closed_reader.rs` — the Unix case and its header (delta 2).
- `.workflow/release-declarations.md` — the *every subcommand and arm of the gate binary* bullet, narrowed to the process's own writes: an arm that hands a child the binary's own stdout returns that child's status, which on Unix is 141 where the child writes after its reader has gone (delta 1).
- `docs/gate-sdk/SPEC.md` — generated, stale once either delta lands (all deltas).

## Retired spellings

- None — no delta removes or renames a name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **The probe re-run** — the three `--scratch-run` readings at the head hold on the built binary.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Entry moved** — `inherited-stdout-child-status` moves to Done in the merge commit, which lands before the drain stage.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
