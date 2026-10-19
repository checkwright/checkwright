# SPEC amendment: lead-role-marker

Pairs `lead-role-marker-guard-refused`. The lead's first step writes the session-role marker with a command built on command substitution and parameter expansion (`lifecycle-kit/templates/lead.md`), and the shell guard's expansion rule refuses it, so a lead hand-inlines the binary path and its id or runs unsuppressed. The literal-path spelling is no fix for an adopter, whose binary path differs by install. This amendment has `--enter-stage --open-lead-journal`, already the lead's next command, write the marker too, so the step is one literal command and no name is added.

**What the tree does today**, read at spec:

- The marker's one producer is the template's shell line (`git grep -n -i "session.role" -- ':!docs/'`): `lifecycle-kit/templates/lead.md` lines 7 to 11. Its one reader is the session-context hook's identity match (`native/src/hook/session_context.rs`, the `CONTEXT_KIT_SESSION_ROLE_FILE` knob). The scratch sweep spares the marker by the same knob, and this repo's `LIFECYCLE_KIT_BOUNDARY_PRESERVE` spares `session-role` by name.
- Fed to the built `--hook shell-guard` as payloads, the template's line exits 2 on the expansion rule, and `bash gate-sdk/bin/run-gates.sh --enter-stage --open-lead-journal` exits 0 with nothing printed, left to the harness's allowlist.
- The opener (`open_lead_journal` in `native/src/emit/enter_stage.rs`) reads the state file and the lead journal and derives no id. The stamp path derives one in process through `crate::emit::session_id::emit` (`stamp`, the same file), behind the front-end that changes to the git toplevel.
- Cross-kit knob reads are already in the crate: `native/src/emit/drift_report.rs`, `native/src/emit/statistics.rs` and `native/src/hook/statusline.rs` each declare names of more than one kit (a scan of every `pub const KNOBS` under `native/src`).
- The arm's declared roster in code is `LIFECYCLE_KIT_*` plus five gate-sdk names, `GATE_SDK_POWERSHELL` among them since the exec-descriptor change. lifecycle-kit/SPEC.md §bin/enter-stage.sh still says four. Delta 1 rewrites that sentence, so it names the fifth while it adds the sixth.

**The choice of producer** follows the entry's Deliverable and its lean, and needs no further direction. A separate arm would mint a name and leave the lead two commands. The opener already runs as the lead's second step, already derives what the marker is keyed on through the same derivation the stamp uses, and already writes the lead's scratch. The cost is one cross-kit knob read in a lifecycle-kit arm. That has precedent, and the template already cites the knob.

**No act here depends on a sibling's landing.** No other unit of the set touches `enter_stage.rs`, `lead.md` or the marker. `hook-binding-single-source` may move how `native/src/hook/session_context.rs` reads its payload, which no delta here edits.

## What changes

### (1) The lead-journal opener writes the session-role marker {design-bearing} {user-facing: lead-role-marker-guard-refused's Deliverable, "an arm that writes the marker itself, so the lead's step is one literal command", and its lean, "`--enter-stage --open-lead-journal`, the lead's next step already, writes it, adding no name"}

`lifecycle-kit/SPEC.md` §bin/enter-stage.sh, #### The lead-journal opener. The opening sentence, "**`--open-lead-journal` is the lead's journal opener.** `[--simulate] --open-lead-journal` appends one heading to the lead journal (`<scratch>/$LIFECYCLE_KIT_LEAD_JOURNAL_FILE`), creating the file and its directory when absent:", reads as follows. **Not yet applied.**

> **`--open-lead-journal` is the lead's opener: it opens the lead journal and records the session role.** `[--simulate] --open-lead-journal` appends one heading to the lead journal (`<scratch>/$LIFECYCLE_KIT_LEAD_JOURNAL_FILE`), creating the file and its directory when absent, and writes the session-role marker (below):

After the bullet **Not stage motion.**, a new paragraph, before **The key derivation is one function**. **Not yet applied.**

> **The opener writes the session-role marker** context-kit's session-context hook reads (context-kit/SPEC.md §The session-context hook): the one line `lead <id>` to the path `CONTEXT_KIT_SESSION_ROLE_FILE` names, replacing what the file held and creating its directory. `<id>` is §bin/session-id.sh's derivation, run in process as the stamp runs it. The write runs on every opener run that passes the surplus refusal and finds the state file, the idempotent journal no-op included, since a resumed lead may hold a new id. A marker already holding the line is reported and left alone. Where the derivation answers nothing, the marker is skipped with a stderr line naming the derivation's refusal, and the journal is still opened at exit 0: the marker only suppresses executor-facing output, and a lead without it loses that alone. `--simulate` relays the line and path it would write, or the skip, and writes nothing. A marker that cannot be written exits 2, as a journal that cannot be written does. **Honest limit:** where the derivation falls to the newest transcript and picks a just-finished subagent's, the marker names no top-level session and the suppression no-ops (§bin/session-id.sh, context-kit/SPEC.md §The session-context hook).

The same section's exit-contract bullet: "0 opened, or a reported no-op" reads "0 opened, or a reported no-op, with the marker written, left alone or skipped". **Not yet applied.**

The same section's test paragraph, "The hermetic cases live in `gate-tests/`: creation, …": after "the `none` key", it reads "and the marker: written under a pinned `LIFECYCLE_KIT_SESSION_ID`, rewritten on the journal no-op under a new id, left alone when it already holds the line, skipped at exit 0 with the journal opened where the derivation fails, and relayed but unwritten under `--simulate`". **Not yet applied.**

`lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the paragraph opening **The declared roster is the prefix family `LIFECYCLE_KIT_*`, plus four names outside it.** Its first sentence reads "**The declared roster is the prefix family `LIFECYCLE_KIT_*`, plus six names outside it.**", and the bullet list becomes the following. The sentence before the list, "Each dispatched gate inherits the invoking environment unchanged and resolves its own knobs. They are:", stays. **Not yet applied.**

> - `GATE_SDK_PRUNE_DIRS` and `GATE_SDK_PRUNE_EXTRA_DIRS`, which are `check-stage-entry`'s;
> - `GATE_SDK_KIT_DIRS`, the kit roots whose `checks/` directories the two pre-flight members resolve in;
> - `GATE_SDK_POWERSHELL`, the host a pre-flight member declared `# exec:` with a `.ps1` script starts under;
> - `GATE_SDK_TMP_DIR`, the scratch dir: the temp state file's home, the boundary wipe's subject and the resume journal's parent;
> - `CONTEXT_KIT_SESSION_ROLE_FILE`, the session-role marker the lead-journal opener writes.

`lifecycle-kit/SPEC.md` §bin/session-id.sh. **Not yet applied.**

- The paragraph opening **Newest-file selection is the documented single-operator assumption**: its last two sentences, "So a lead deriving its own id here for its session-role marker (§templates/lead.md) can name the wrong session. A lead in that position verifies the printed id against its own transcript before writing the marker.", read "So the lead-journal opener, deriving the lead's id for its session-role marker (§bin/enter-stage.sh), can name a subagent's session, which costs the lead its suppression and nothing else."
- **Callers.**: "The one direct caller is a lead writing its own session-role marker (§templates/lead.md), which the front-end route serves." reads "The lead-journal opener derives it in process for the lead's session-role marker. No template calls the arm directly; the front-end route serves the consumer smoke."
- The paragraph opening **The process cwd is an input to source 3**: "The front-end route serves `templates/lead.md` and the consumer smoke, both at the repo root where the two agree." reads "The front-end route serves the consumer smoke at the repo root, where the two agree."
- The paragraph opening **The kit's templates name this arm as the id's source.**: "but the stage templates and `templates/lead.md` cite it, and one of them invokes it directly. So a change to its invocation edits kit `templates/*.md`" reads "but the stage templates cite it as the stamp's id. So a change to its invocation edits kit `templates/*.md`".

`native/src/emit/enter_stage.rs`. **Not yet applied.**

- `KNOBS` gains `CONTEXT_KIT_SESSION_ROLE_FILE`, and its `spec:` comment reads "the whole `LIFECYCLE_KIT_*` family this arm reads, plus the gate-sdk names outside it that the arm and the pre-flight gates it dispatches read, and the session-role marker's knob the lead-journal opener writes through".
- `Cfg` gains the marker path, read with `walk::knob_scalar("CONTEXT_KIT_SESSION_ROLE_FILE")`.
- `open_lead_journal` writes the marker as delta 1's paragraph states, after the journal: a function of its own derives the id through `crate::emit::session_id::emit`, compares the file's content, and writes, reports the no-change, or reports the skip. Under `Say { sim: true }` it reports through `say.out`. Its `spec:` comment names §bin/enter-stage.sh. The no-op return moves below the marker write, so the no-op reaches it.
- `usage`: the `--open-lead-journal` line reads "(open the lead journal under a heading keyed on the cursor, and write the session-role marker)".

`native/src/runner.rs`, the `--enter-stage` help: "`--open-lead-journal` opens the lead journal under a heading keyed on the stage cursor, stamping nothing" reads "`--open-lead-journal` opens the lead journal under a heading keyed on the stage cursor and writes the session-role marker, stamping nothing". **Not yet applied.**

`lifecycle-kit/gate-tests/lead-journal-open.test.sh`. **Not yet applied.**

- `run_open` keeps its environment, and each marker case passes `LIFECYCLE_KIT_SESSION_ID` through `gate_env`. Source 1 wins over the harness's variables, which reach the test unchanged.
- **marker-write.** On `create`, with `LIFECYCLE_KIT_SESSION_ID=abcdef0123`: `scratch/session-role` is exactly `lead abcdef01`.
- **marker-noop-rewrite.** The journal no-op re-run with `LIFECYCLE_KIT_SESSION_ID=12345678ab` leaves the journal's bytes and makes the marker `lead 12345678`.
- **marker-same.** A third run with the same id reports that the marker already holds the line and leaves its bytes.
- **marker-skip.** With `LIFECYCLE_KIT_SESSION_ID`, `CLAUDE_CODE_SESSION_ID` and `CLAUDE_CODE_CHILD_SESSION` unset and `LIFECYCLE_KIT_SESSIONS_DIR` naming an empty sandbox directory, the opener exits 0, the journal holds its heading, no marker exists, and the output names the skip.
- **marker-simulate.** `--simulate --open-lead-journal` with a pinned id relays `lead <id>` and the marker path under the prefix and writes no marker.
- **marker-surplus.** Each surplus case also asserts no marker.
- The header's `spec:` comment and the closing `echo` name the marker.

### (2) The lead's first step is the one opener command {mechanical} {user-facing: lead-role-marker-guard-refused's Deliverable, "so the lead's step is one literal command"}

`lifecycle-kit/templates/lead.md`, from **First step — record the session role.** through the paragraph opening "Then open your resume journal", that is the first-step paragraph, its fenced block, the paragraph under it and the journal paragraph, reads as follows. **Not yet applied.**

> **First step — open your journal and record the session role.** Run `--enter-stage --open-lead-journal` on the gate binary `GATE_SDK_NATIVE_BIN` names. It opens your resume journal, and writes `lead <id>` to the session-role marker (`CONTEXT_KIT_SESSION_ROLE_FILE`), which the session-context hook reads on each re-fire to suppress its executor-facing steps for this session only (context-kit/SPEC.md §The session-context hook). Run it again after a resume. Never overwrite the journal; append to it with `--emit journal --lead "<finding>"`.

`lifecycle-kit/SPEC.md` §templates/lead.md, the paragraph opening **The lead's first step writes the session-role marker, opens its journal and reads the model roster.**: its first three sentences read "**The lead's first step opens its journal and reads the model roster.** The journal is opened through `--enter-stage --open-lead-journal` (§bin/enter-stage.sh), whose keyed heading exempts the live lead's segment from the boundary's undisposed advisory, and which writes the session-role marker context-kit's hook reads (context-kit/SPEC.md §The session-context hook). A literal command is what the step needs: the shell guard refuses an expansion (guard-kit/SPEC.md §The rule roster, rule `expansion`), and a lead's binary path differs by install." The roster sentence stays. **Not yet applied.**

`lifecycle-kit/README.md`, the paragraph under the command block: "reach for it directly only where a session writes an id itself, as `templates/lead.md`'s session-role marker step does." reads "reach for it directly only where a session writes an id itself." **Not yet applied.**

### (3) context-kit names the opener as the marker's producer {mechanical}

`context-kit/SPEC.md` §The session-context hook, **The session-role signal.** **Not yet applied.**

- The **Producer.** bullet reads: "lifecycle-kit's lead-journal opener, `--enter-stage --open-lead-journal`, run as `/lead`'s first step, writes one line `lead <id>`, `<id>` being the `--emit-session-id` derivation's value, to `CONTEXT_KIT_SESSION_ROLE_FILE` (gitignored scratch, default `${GATE_SDK_TMP_DIR:-.tmp}/session-role`) (lifecycle-kit/SPEC.md §bin/enter-stage.sh)."
- The third accepted limit, opening "The producer inherits `--emit-session-id`'s dependency": "The producer inherits" reads "The opener inherits", and the rest stays.

`context-kit/SPEC.md` §Layout and configuration, the `CONTEXT_KIT_SESSION_ROLE_FILE` bullet: "the session-role marker `/lead` writes" reads "the session-role marker lifecycle-kit's lead-journal opener writes". **Not yet applied.**

### (4) The release record declares the change {mechanical}

`.workflow/release-declarations.md`, `## Behavior changes`, a new bullet. **Not yet applied.**

> - **lifecycle-kit `--enter-stage --open-lead-journal`, lifecycle-kit/templates/lead.md** — the opener now also writes the session-role marker, `lead <id>` to the file `CONTEXT_KIT_SESSION_ROLE_FILE` names, deriving the id as a stamp does, and skips it with a note where no id derives. The lead's first step is that one command, where it was a shell line the shell guard refused for its expansions. Nothing to do; a lead shim of yours that copied the old line can drop it.

## Producers and consumers

- **The marker write (delta 1).**
  - Producer: `open_lead_journal`, reached as `--enter-stage --open-lead-journal` through `bash gate-sdk/bin/run-gates.sh`, which the lead template's first step runs (delta 2). No knob enables it. It runs wherever the opener runs.
  - Consumer: the session-context hook's identity match, reading the file `CONTEXT_KIT_SESSION_ROLE_FILE` names on each fire (`native/src/hook/session_context.rs`). The one field, the id, is read by that match. The line's grammar is unchanged, so the reader does not move.
  - Roster-holding readers of the surfaces: the scratch sweep's spare and this repo's `LIFECYCLE_KIT_BOUNDARY_PRESERVE` name the file by knob and by name, and neither moves. The arm's `KNOBS` roster gains the knob, and the SPEC roster it mirrors gains it in delta 1.
- **The id derivation's new caller (delta 1).** `crate::emit::session_id::emit`, already the stamp's. Its callers paragraph is rewritten under delta 1, and the arm's empty roster does not move.
- **The opener's new output lines (delta 1).** Read by the lead that ran it, and by `lead-journal-open.test.sh`'s new cases. No gate parses the opener's output.
- **Point 5:** no delta narrows a corpus. The template's `$(…)` line leaves the template, and no reader counts the template's commands.
- **Point 6:** no delta obliges the members of a corpus.

## Existing sections updated

The roster came from `git grep -n -i -E "session.role|SESSION_ROLE" -- ':!TASK-QUEUE.md' ':!docs/'`, `git grep -n -l -e "open-lead-journal" -- ':!TASK-QUEUE.md' ':!docs/'` and `git grep -n -E "emit session-id|emit-session-id" -- ':!docs/'` over the tracked tree.

- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the declared roster and #### The lead-journal opener; §bin/session-id.sh, four paragraphs (delta 1).
- `native/src/emit/enter_stage.rs`, `native/src/runner.rs` (delta 1).
- `lifecycle-kit/gate-tests/lead-journal-open.test.sh` (delta 1).
- `lifecycle-kit/templates/lead.md`, the first step; `lifecycle-kit/SPEC.md` §templates/lead.md; `lifecycle-kit/README.md` (delta 2).
- `context-kit/SPEC.md` §The session-context hook and §Layout and configuration (delta 3).
- `.workflow/release-declarations.md` (delta 4).
- `docs/lifecycle-kit/SPEC.md` and `docs/context-kit/SPEC.md`, the generated mirrors (all deltas).

## Retired spellings

- None — no name is retired. The opener keeps its spelling, and the template's shell line is prose, not a name another surface cites.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The step run as written** — the new first step, fed to the built `--hook shell-guard` as a payload, does not block, and run at the repo root it writes the marker the session-context hook then matches.
- [ ] **Suites green** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-gate-tests lifecycle-kit/gate-tests lifecycle-kit/checks`, context-kit's suites, and the battery.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, at a stage before the drain stage.
