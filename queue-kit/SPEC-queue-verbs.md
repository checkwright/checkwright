# SPEC amendment: queue-verbs

The queue has gates and read arms and no write command, so every move an entry makes is a hand edit the gates check only afterwards, and an entry's life is rebuilt from git by hand. This amendment adds a family of write verbs on the gate binary, each one move or stamp, each followed by the queue's own gates, and a read arm printing one entry's section transitions. It pairs [queue-write-side-verb](../TASK-QUEUE.md#queue-write-side-verb). [recurrence-declaration-grammar-ungated](../TASK-QUEUE.md#recurrence-declaration-grammar-ungated) is debt in the same iteration and shares delta 7's parser.

**The rulings.**

- **One arm, a verb operand.** The verbs are `--queue <verb>`, an `Arm::Run` member whose first operand selects the verb, on `--hook`'s and `--wait-probe`'s shape (gate-sdk/SPEC.md §The non-gate arm). One arm keeps one declared read roster and one exit contract. Each verb is still its own allowlist line for a consumer granting narrowly: `Bash(<door> --queue promote *)` grants that verb alone.
- **Each verb is a checked text transform, then the queue's gates.** A verb computes the new file, checks its own postcondition on the shared adapters' live and done sets, and writes only when that holds (the queue-migrate precedent, §The queue-migrate arm). It then runs every gate coupling to the queue file, the selection `--for <queue-file>` already makes (gate-sdk/SPEC.md §run-gates), and its status carries that verdict. Measured at authoring: `run-gates.sh --for TASK-QUEUE.md` ran 40 gates, green, in 2.7 s.
- **A verb moves an entry; it never decides whether the caller may.** Who may write the queue, and when, stays lifecycle-kit's (lifecycle-kit/SPEC.md §The state machine and §The committed gap inbox). A mid-iteration session still files a gap rather than calling a verb.
- **A verb never commits and never stages.** The move lands in the commit the session is already making, which is where every same-commit rule the moves carry already puts it.
- **The history walk is shared, and two verbs read it.** Restoring a demoted entry's position and board tags, and a thawed entry's body, both need the revision before the move that took the entry out. The walk that finds that move is the one `--emit queue-history` prints, so the read arm and the two verbs share one adapter.
- **The allowlist ground is already met here.** This repository's settings grant `Bash(bash gate-sdk/bin/run-gates.sh *)` and the binary door, and guard-kit's recommended allowlist carries both (guard-kit/templates/settings-allow.json). So every verb is granted on landing and no settings edit is owed. The narrow per-verb line matters to a consumer holding a narrower grant.

**Refused.**

- **A write mode on an existing read arm.** `--emit` collapses every error to 2, and a verb's 1, a move written with a red post-check, is the status its caller acts on.
- **A verb that refuses on a red post-check and writes nothing.** A Done move leaves every `[blocked-by:]` on the moved slug stranded, and clearing each one owes a prose rephrase in the same commit (§check-task-names). A refuse-on-red verb could never make that move.
- **Editing the settings allowlist.** Nothing needs it (above), and a delegated session only prepares such a diff.

## What changes

### (1) The `--queue` arm

**Not yet applied.** Add `--queue <verb> <operand>…` to the arm table as an `Arm::Run` member, with its module at `native/src/emit/queue_verbs.rs` and its contract as below {design-bearing}.

- **Exit status.** 0: the move is written and the post-check is green. 1: the move is written and the post-check is red; its findings are printed, and they are the session's worklist before commit. 2: nothing is written, whether for a usage error, an unknown verb, a refusal named below or an unreadable queue file. An absent binary exits 2 at the front end, the default gate-sdk/SPEC.md §run-gates gives every arm whose status a session reads, so the front end needs no edit.
- **Output.** One line per act on stdout, naming the slug, the section it left and the one it entered, and each tag the move dropped or added. Then the post-check's own output.
- **The write.** The whole file is written to a sibling temporary file and renamed over the queue file, so an interrupted verb leaves the old file or the new one.
- **The postcondition.** Computed on `live_slugs` and `done_slugs` (§The shared queue adapters) before and after the transform. Each verb states its own below; a transform that breaks it is refused with nothing written, and the refusal names the difference. This is the check that catches a wrong extent.
- **The post-check.** The arm spawns its own executable as `--run --gates-dir <gates-dir> --for <queue-file>`, the argv the front end composes for `--for`, and reads its status. It spawns rather than calling in process, the rule for a dispatching arm (gate-sdk/SPEC.md §The non-gate arm), so a member that panics cannot take the verb with it.
- **Declared reads.** `QUEUE_KIT_QUEUE_FILE`, `QUEUE_KIT_ACTIVE_SECTIONS`, `QUEUE_KIT_DEFERRED_SECTION`, `QUEUE_KIT_ICEBOX_SECTION`, `QUEUE_KIT_DONE_SECTION`, and, as a dispatching arm declares its callee's reads, the runner's `crate::runner::KNOBS` with the `EVERY_REGISTERED_KNOB` sentinel.
- **Scope of an operand.** Every verb takes a top-level `###` entry. `done` and `recur` also take a `####` sub-task. Any other verb given a sub-task refuses and names `split`.
- **The front end.** A bare `Arm::Run` flag falls through `bin/run-gates.sh`'s `-*)` case unchanged. The runner's usage block gains the line `run-gates.sh --queue <verb> <slug> [args]  move or stamp one queue entry, then run the queue's gates` (delta 9).

### (2) The single-revision verbs

**Not yet applied.** Five verbs read nothing but the current file: `promote`, `done`, `clear-done`, `icebox` and `recur` {design-bearing}.

- **`promote <slug> <section> [--spec <file>]`.** The entry leaves the deferred section and becomes the last entry of `<section>`, which must name one of `QUEUE_KIT_ACTIVE_SECTIONS`. Its `[cost:]` and `[surface:]` tags are dropped, the checksum §check-deferred-board-tags assertion B holds. `--spec` writes `[spec: <file>]` first on the tag line, creating the tag line where the entry had none. Every other tag stays. Postcondition: the live set is unchanged. Refused: a slug not in the deferred section, and a `<section>` that is not active.
- **`done <slug>`.** The entry's extent is removed from whichever task section holds it, and `- <slug>` becomes the last line of the done section. Every `[<slug>] (#<slug>)` in the file is rewritten to `` `<slug>` ``, the retired citation (§The tag algebra). Postcondition: the slug leaves the live set and joins the done set, and the live set is otherwise unchanged. Refused: a `[roadmap:]`-tagged entry in the deferred section or the icebox, since no drain may retire one (§The icebox tier). An active roadmap entry that landed takes the move.
- **`clear-done`.** Every bullet under the done section is removed, and the heading stays. Postcondition: the done set is empty and the live set is unchanged. This is close's "then clear Done" (lifecycle-kit/templates/stages/close.md, step 5). The entry's operation set does not name it; see the escalation in the stage report.
- **`icebox <slug> <sentence>`.** The entry becomes `### <slug>`, a blank line and `<sentence>`, as the last entry of the icebox section, every tag dropped (§The icebox tier's grammar). Postcondition: the live set is unchanged. Refused: no icebox configured; a slug not in the deferred section; a sentence that is empty or spans a line break; and an entry carrying `[roadmap:]` or `[not-icebox-eligible:]`, the two exclusions a tag declares. The judged limbs of eligibility stay the closing stage's, read off `--icebox-candidates`.
- **`recur <slug> [<date>]`.** `<date>` defaults to the local today, the day `--emit queue-index` reads (§The queue-index arm). It is appended to the entry's `[recurrence:]` array, which is created at the end of the tag line when absent. Postcondition: the live and done sets are unchanged. A date equal to the array's last element writes nothing and exits 0, since the stamp is idempotent per slug and date (close.md step 2). Refused: an array or a `<date>` that delta 7's parser rejects; a date earlier than the array's last element, because dates are appended in order; and a slug found only in the done section, which is a new defect and not a recurrence (lifecycle-kit/SPEC.md §The committed gap inbox).

### (3) The transition walk and the `queue-history` arm

**Not yet applied.** Add `queue::transitions(file, slug)` to the shared adapters and an `--emit queue-history <slug> [<queue-file>]` arm that prints it {design-bearing}.

- **A transition** is a commit at which the slug's **place** changed. A place is the task section heading its entry, the done section when a bare done line carries it, or `(absent)`. Each transition carries the commit, its committer date, its subject, and the place before and after.
- **The walk** is entry-history's. It pipes commits touching the queue file, newest first, through the one `git cat-file --batch` child. It reads a revision older than the heading grammar through the migration arm, and it takes its bound from the live and retired sets (§check-queue-entry-budget). It stops at the first revision, walking back, in which the slug is absent after being present; the move out of that revision is the filing. A slug neither live nor retired is refused before any blob is read. The walker is extracted from `native/src/emit/entry_history.rs` into the adapter, so the two arms read history one way.
- **The report** is the slug on its own line, then one indented row per transition, oldest first: `<YYYY-MM-DD> <short-sha> <from> -> <to>  <subject>`. Oldest first, because the question it answers is the entry's story, filed to promoted to done, and entry-history's newest-first order answers a different one, the most recent fall.
- **Exit** 0 with the report, 2 on a usage error. There is no 1, on entry-history's ground. Declared reads: the queue file and the four section knobs `done_slugs` and the task sections read.
- **Degradations**, inherited from the retired-set derivation: a file outside a git work tree, or no `git`, yields no transitions and prints the slug alone; a shallow clone under-claims.
- **Why a separate arm.** entry-history reports falls in counted size, four fields, no verdict. A section transition is a different question over the same walk, and folding it in would give that arm two output grammars (§The queue-counts arm's refusal).

### (4) The history-reading verbs: `demote` and `thaw`

**Not yet applied.** Two verbs restore what an earlier move took, read off delta 3's walk {design-bearing}.

- **`demote <slug> [--cost <class>] [--surface <entry>]`.** The inverse of `promote`, and the move canon-kit/SPEC.md §Merging an amendment step 4 names. The entry leaves its active section for the deferred section, dropping `[spec:]` and `[drain-exempt:]`. It finds the newest transition from the deferred section into an active one, and reads the revision before that commit:
  - **Board tags.** The `[cost:]` and `[surface:]` values the entry carried there are restored. A flag overrides its tag.
  - **Position.** The entry lands after the entry that preceded it there. Where that predecessor is no longer in the deferred section, the walk tries the one before it, and so on. With none left, the entry leads the section.
  - **No such transition** (the entry was filed straight into an active section, or history is unavailable): both flags are required, and the entry becomes the section's last. Without them the verb refuses.
  - Postcondition: the live set is unchanged.
- **`thaw <slug> [--cost <class>] [--surface <entry>] [--date <date>]`.** The icebox's return route, taken on a judged recurrence (§The icebox tier). It finds the newest transition from the deferred section into the icebox and reads the revision before it:
  - **Restored.** The entry's whole extent there — heading, tag line and body — replaces the one-sentence form, so the recovery §The icebox tier makes mandatory before a ruling is done by the move itself.
  - **Position.** The predecessor rule above.
  - **Tags.** A flag overrides the restored board tag. `<date>`, defaulting to the local today, is appended to the `[recurrence:]` array under `recur`'s rules, since the return is a recurrence.
  - Refused with nothing written: no such transition, because there is no body to restore.
  - Postcondition: the live set is unchanged.

### (5) The `split` verb

**Not yet applied.** `split <parent> <child> <parent-sentence>`, with the child's tag line and body on stdin, performs the mechanical half of an authorized split (§check-queue-entry-budget) {design-bearing}.

- `### <child>` and the stdin text are inserted directly after the parent's extent, in the parent's section, and `<parent-sentence>` becomes the parent's last paragraph.
- Refused: `<child>` is not a valid slug, or is already live or done; the stdin text carries no `[<parent>] (#<parent>)` link; `<parent-sentence>` carries no `[<child>] (#<child>)` link. The two links are the bidirectional citation that section requires, so the verb cannot write a split that fails it.
- Postcondition: the live set gains exactly `<child>`.
- The authorization stays with the session that grants it. The verb cannot see a grant, and the section's refusal to let a blocked session split its own entry is unchanged.

### (6) The ownership sentences, rewritten

**Not yet applied.** Seven sentences in four specs state that no queue-mutating tool exists, or say how a move a verb now makes is made. Each is rewritten, not appended to {design-bearing}.

- queue-kit/SPEC.md §The queue-index arm, the sentence "No queue-mutating tool is added — `--extent <slug>` already yields the line range an eviction deletes." becomes: "The arm writes nothing. The moves an extent bounds are §The queue verbs'."
- queue-kit/SPEC.md §The icebox tier's conserved-move bullet and §check-task-names' Done-move paragraph: "the binary's `--rewrite` arm does it in one call" becomes "`--queue done` makes it with the move".
- gate-sdk/SPEC.md §The non-gate arm: "That caller is load-bearing, because the queue-index arm's refusal to ship a queue-mutating tool rests on it." is deleted. The paragraph's point, that a query arm's caller may be a session, stands without it. `--queue` joins that section's roster of `Arm::Run` members forced out of the `--emit-` family, on its 1.
- canon-kit/SPEC.md §Merging an amendment, step 4: the Done move is `--queue done <slug>`, and the demotion is `--queue demote <slug>`, which recovers the position and the board tags from the promoting commit that the step tells a session to read.
- lifecycle-kit/SPEC.md §The committed gap inbox ("That drain step is the declaration's only **mechanized** producer") and queue-kit/SPEC.md §The tag algebra ("the closing stage's gap-inbox drain is its only mechanized producer"): every stamp, the drain's and a direct one alike, is written with `--queue recur`, so the verb is the declaration's one writer and the judgment stays the session's.

The canonical home for deltas 1 to 5 is a new queue-kit/SPEC.md §The queue verbs, after §The lesson-sink arm, and a §The queue-history arm beside §The queue-edges arm. §Layout and configuration's roll of non-gate arms outside `gates.list` names both.

### (7) The recurrence parser

**Not yet applied.** Add `queue::recurrence_array(tag_line)`: the `[recurrence:]` value split on `,`, each token trimmed, every token a calendar-valid `YYYY-MM-DD` (month 01 to 12, day within its month, Gregorian leap years). It returns the dates in order, or the first token that fails. An empty value fails {design-bearing}.

- `recur` and `thaw` refuse on a failing array and on a failing `<date>`.
- `recurrence_dates` keeps its contract for its readers, the valid dates with a bad token dropped, so no reader's count moves here.
- **It depends on whether the debt sibling lands in the same batch.**
  - If [recurrence-declaration-grammar-ungated](../TASK-QUEUE.md#recurrence-declaration-grammar-ungated) lands in the same build batch, the parser lands once with both callers: `recur`, and that entry's `check-queue-hygiene` axis refusing a malformed token.
  - If it lands in a later batch, this batch lands the parser with `recur` and `thaw` as its callers, and the debt entry adds the axis as a third, on the same function.
- Measured at authoring: every `[recurrence:]` tag line in `TASK-QUEUE.md` parses. `grep -o '\[recurrence:[^]]*\]'` matches two malformed spellings, and both are backticked prose in that debt entry's own body rather than tags.

### (8) The instruction surfaces

**Not yet applied.** Each surface that instructs a move names its verb, replacing the hand-edit instruction rather than adding a second one {mechanical}.

- lifecycle-kit/templates/stages/close.md, step 2: "append the bullet's date to that entry's `[recurrence:]` array (creating the tag when absent)" becomes "stamp it with `--queue recur <slug> <bullet-date>`".
- close.md, step 5: "moves to the done section as a bare slug in this stage's queue commit, every link to it rewritten to a backticked slug" becomes "moves with `--queue done <slug>` in this stage's queue commit", and "Then clear Done." becomes "Then clear Done with `--queue clear-done`." (the second only if delta 2's `clear-done` is kept).
- lifecycle-kit/templates/stages/spec.md, the promotion paragraph: the pairing commit promotes with `--queue promote <slug> <section> --spec <file>`.
- lifecycle-kit/templates/stages/scope.md, the debt promotion paragraph: `--queue promote <slug> <section>`.
- lifecycle-kit/templates/stages/build.md, the opening paragraph's Done move: `--queue done`, or `--queue demote` for an entry that outlives its amendment.
- `.claude/commands/close.md`, the backlog-eviction bullet: evict with `--queue icebox <slug> <sentence>`, and rule wontfix with `--queue done <slug>`.
- `CLAUDE.md`, the recurrence-stamping line: "takes today's date on its entry's `[recurrence:]` array" becomes "is stamped with `--queue recur <slug>`". The line keeps its one-line shape.

### (9) Usage, tests and the README

**Not yet applied.** {mechanical}

- `native/src/runner.rs`'s usage block gains the `--queue` line (delta 1).
- queue-kit/README.md's arm listing gains one line per verb and the `queue-history` line.
- Unit tests in `native/src/emit/queue_verbs.rs` cover each verb's transform, postcondition and refusals over a queue built in the test. A test module in the history arm's file covers the report grammar (the non-gate arm's test obligation).
- `queue-kit/gate-tests/queue-verbs.test.sh`, a bespoke test on entry-history's precedent: throwaway repositories whose commit history promotes, evicts and moves an entry, then `demote` and `thaw` restoring position, tags and body, `queue-history`'s rows over the same history, a verb's 1 on a planted post-check red, and its 2 with the file unchanged.

## Producers and consumers

- **A verb's write.**
  - Producer: a session running `--queue <verb>` through the front end or the door, at the transitions delta 8's surfaces name: promotion at scope and spec, the Done move and demotion at build, the drain's stamp, eviction, the moot sweep and the Done clear at close, and a direct recurrence stamp in any session that may write the queue.
  - Consumers: the post-check battery in the same call; the session's own commit; every queue reader afterwards, unchanged, since the verbs write the grammar the readers already parse.
  - Red conditions: the post-check's, which are the gates' own.
- **The exit status.** Read by the calling session. 1 is fix-forward before commit, and 2 means the file is untouched.
- **A transition row.** Producer: `queue::transitions`. Consumers: the `queue-history` report, whose reader is a session reconstructing an entry's life; `demote`, which reads the newest deferred-to-active row's commit; `thaw`, which reads the newest deferred-to-icebox row's commit. Every field has a reader: the commit is opened or read by the two verbs; the date and subject are read by the session; the two places decide which row the verbs select.
- **`recurrence_array`.** Callers `recur` and `thaw`, and the debt sibling's hygiene axis (delta 7 states both landing orders).
- **Roster-holding readers of the minted names.**
  - The arm table's unit test resolves each row's function to a file with a test module (delta 9).
  - `check-reads-couples` and `check-gate-substrate-parity` read the declared roster (delta 1).
  - `check-fence-command-head` and `check-docs-cmd` read the README's new command lines, whose head is the `"$gates"` door the file's existing lines use (delta 9).
  - `check-shim-restatement` reads `.claude/commands/close.md` against the templates. The edits there are verb names, not template sentences.

## Existing sections updated

Roster probe: `git grep -n -i "queue-mutating\|mutating tool\|arm does it in one call\|clear Done\|Done move"` over tracked `*.md` outside `docs/` and the queue, plus `git grep -n "recurrence"` over lifecycle-kit/SPEC.md, `grep -n "entry-history\|lesson-sink"` over queue-kit/README.md and gate-sdk/SPEC.md, and `grep -n "icebox"` over `.claude/commands/`.

- `native/src/emit/queue_verbs.rs`, `native/src/emit/mod.rs` — the module and the arm-table row (deltas 1, 2, 4 and 5).
- `native/src/queue.rs` — `transitions` and `recurrence_array` (deltas 3 and 7).
- `native/src/emit/entry_history.rs` — its walker moves into the shared adapter (delta 3).
- `native/src/emit/queue_history.rs` — the read arm (delta 3).
- `queue-kit/SPEC.md` — §The queue verbs and §The queue-history arm added; §The queue-index arm, §The icebox tier, §check-task-names, §The shared queue adapters, §Layout and configuration and §check-queue-entry-budget's split paragraph rewritten (deltas 1, 2, 3, 4, 5, 6 and 7).
- `gate-sdk/SPEC.md` — §The non-gate arm (delta 6).
- `canon-kit/SPEC.md` — §Merging an amendment, step 4 (delta 6).
- `lifecycle-kit/SPEC.md` — §The committed gap inbox, the producer paragraph (delta 6).
- `lifecycle-kit/templates/stages/close.md`, `lifecycle-kit/templates/stages/spec.md`, `lifecycle-kit/templates/stages/scope.md`, `lifecycle-kit/templates/stages/build.md`, `.claude/commands/close.md`, `CLAUDE.md` (delta 8).
- `native/src/runner.rs`, `queue-kit/README.md`, `queue-kit/gate-tests/queue-verbs.test.sh` (delta 9).
- `docs/queue-kit/SPEC.md`, `docs/queue-kit/README.md`, `docs/gate-sdk/SPEC.md`, `docs/canon-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md` — the generated on-site mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- None — no delta retires a name; delta 6 rewrites sentences that stated an absence, and every name they carry stays live.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Run** — `bash gate-sdk/bin/build-native.sh`, the full battery, every kit fixture suite the crate change reaches, and `queue-kit/gate-tests/queue-verbs.test.sh` green. The build session makes its own queue moves for this iteration with the verbs, the first live use.
- [ ] **The entry closes with the build.** The merge moves [queue-write-side-verb](../TASK-QUEUE.md#queue-write-side-verb) to Done, with `--queue done`, in the build commit that deletes this file, before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
