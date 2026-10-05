# SPEC amendment: manual-ops

Paired with the queue entry `manual-operation-spend-channel`. No channel attributes a session's work to repeated manual operations, so the close economics pass cannot surface a tooling opportunity. The stage-economics meter prices tokens per stage and model, the overhead meter measures governance bytes, the prompt-friction log sees only the shell calls the guard did not decide (guard-kit/SPEC.md §scan-prompts), so an Edit or Write call never reaches it, and the knowledge-friction log is self-reported. The harness transcripts the two meters already read carry every tool call: an assistant line's `message.content[]` holds `tool_use` blocks with a `name` and an `input`, the shell tools' `input.command` and the file tools' `input.file_path`. No crate module reads them today: `grep -rn "tool_use" native/src` finds nothing, the guard reading hook payloads rather than transcripts. Measured over this repository's transcripts at spec time, counts only: 3,608 Edit or Write calls on `TASK-QUEUE.md` across 589 transcripts, while queue verbs exist for most of those moves, and Bash shapes led by `grep` (27,511 calls) and the gate runner (16,808), which are ordinary work rather than candidates. So the signal is on disk, and a read needs both kinds of operation and a way to set the ordinary shapes aside.

## What changes

### (1) The manual-operation meter {design-bearing} {user-facing: the entry's operator expectation, 2026-09-27, that the economics analysis every close runs surface tooling opportunities, selected 2026-10-05}

A new section in drift-kit/SPEC.md, after §The stage-economics meter, and a new arm in `native/src/emit/manual_ops.rs` with its `ARMS` row in `native/src/emit/mod.rs`, its `KNOBS` roster and a `#[cfg(test)]` module. *Not yet applied.*

> ## The manual-operation meter
>
> `--emit manual-ops [iteration]` ranks the operations an iteration's sessions repeated by hand, so the close can see where a tool would remove work. It is advisory by the overhead meter's contract: exit is always 0, it never joins `gates.list`, a missing input is a 0-exit notice, and it writes only under `DRIFT_KIT_METRIC_DIR`.
>
> - **The sessions.** The operand names an iteration, by default the one the last stamp in `DRIFT_KIT_STATE_FILE` names. The transcripts read are those §The stage-economics meter attributes to that iteration's row keys — its stage rows, its supervision row and their fan-out — through the meter's own attribution, never a second derivation, so a transcript counts once and a lead's and a dispatched agent's work are both in scope.
> - **The operations.** Each `tool_use` block on an assistant line is one call, of one of two kinds. A **`shell`** call, from the harness's shell tools, is keyed by the prompt-friction ranking key (guard-kit/SPEC.md §scan-prompts), so the two channels name a shape alike. An **`edit`** call, from the harness's file-writing tools, is keyed by its `file_path` relative to the line's `cwd`, else to the repository's toplevel; a path under neither keys as `(outside)`, so no path off the repository is printed. Any other tool is not counted. A line the reader cannot parse is skipped, as the stage-economics reader skips it.
> - **The ranking.** Keys in `DRIFT_KIT_MANUAL_OPS_IGNORE` are dropped: the shapes a consumer counts as ordinary work. The rest rank by call count, then session count, then key, and the first `DRIFT_KIT_MANUAL_OPS_TOP` print, one line each: `<calls> <sessions> <iterations> <kind> <key> [<rows>]`, where `<rows>` names the stage-or-role values of the sessions that made the calls and `<iterations>` counts the logged iterations in which the key ranked, this one included.
> - **The log.** The printed keys are written to `DRIFT_KIT_MANUAL_OPS_LOG`, one line each, `<iteration> <kind> <key>`; a re-run replaces the iteration's lines where they stand and appends a new iteration's, so a second run over unchanged inputs writes a byte-identical log. Its one reader is this arm's `<iterations>` column: a shape that ranks iteration after iteration is the strongest tooling candidate, and transcripts age out of the sessions dir while the log persists. Counts, sessions and rows stay on stdout, under §The stage-economics meter's field rule.
> - **What it never writes.** No command text and no transcript content: a key is a shell shape or a repository-relative path. The log carries no account identifier.
> - **A candidate is never a verdict.** A high count can be the work itself, as an amendment's edits are; the reader of the ranking judges which a tool would remove (§The `/economics` skill).
>
> The tool names are the harness's transcript vocabulary, the coupling class §The stage-economics meter's fan-out row already reads; a renamed tool fails as a visible zero, never a wrong count.

### (2) The close read {mechanical} {user-facing: the entry's operator expectation, 2026-09-27, that the economics analysis every close runs surface tooling opportunities}

drift-kit/templates/economics.md chains three tools rather than two. Its step 3, after the stage-economics step. *Not yet applied.*

> 3. **Manual operations** — the same binary's `--emit manual-ops` (drift-kit/SPEC.md §The manual-operation meter): the shell shapes and hand-edited paths this iteration repeated, ranked, with how many iterations each has ranked in.

Its narrative bullet, after **Overhead share**. *Not yet applied.*

> - **Repeated manual operations.** Name the top candidates with their call and session counts, the stages they ran in, and how often they have ranked before. Say which a tool would remove and file it as a gap; a shape that is ordinary work joins `DRIFT_KIT_MANUAL_OPS_IGNORE`. A count alone is no finding.

The template's lead sentence reads *Chain the three reporting tools in order*. drift-kit/SPEC.md §The `/economics` skill's chain becomes `--emit overhead-meter` → `--emit stage-economics` → `--emit manual-ops`, and the sentence on what each contributes gains *`manual-ops` the tooling candidates*. *Not yet applied.*

### (3) The knobs {mechanical}

Three rows join drift-kit/SPEC.md §Layout and configuration after `DRIFT_KIT_FANOUT_SUFFIX`, and `native/src/knobs/drift_kit.rs`. *Not yet applied.*

> - `DRIFT_KIT_MANUAL_OPS_LOG` — the manual-operation meter's log (§The manual-operation meter); default `.metric/manual-ops-log.txt`, derived as `${DRIFT_KIT_METRIC_DIR}/manual-ops-log.txt` so a set metric dir moves it (gitignored; the meter creates the dirname).
> - `DRIFT_KIT_MANUAL_OPS_IGNORE` — array of keys the meter drops as ordinary work, matched whole against a shell shape or an edit path; default empty, ranking every key.
> - `DRIFT_KIT_MANUAL_OPS_TOP` — positive integer, default `10`: how many keys the meter prints and logs.

The table validator refuses `DRIFT_KIT_MANUAL_OPS_TOP` outside its shape, as it refuses `DRIFT_KIT_PRICE_PAGE_TIMEOUT`; that knob list's sentence in §Layout and configuration gains it.

### (4) The meter's tests {design-bearing}

The arm's unit tests cover the key derivation per kind (a shell call through the ranking key, an edit under `cwd`, under the toplevel only, and outside both) and the ignore and top filters. drift-kit/smoke/install.sh drives the arm through the front end over its own sessions dir, state file and log, with a synthetic fixture, `smoke/manual-ops-fixture.jsonl`, carrying `tool_use` blocks for a stamped session and an unparseable line, and asserts:

- the printed lines' fields and order for a fixture whose counts tie on calls and differ on sessions;
- a key in a throwaway knob file's `DRIFT_KIT_MANUAL_OPS_IGNORE` absent from the output, and `DRIFT_KIT_MANUAL_OPS_TOP` bounding it;
- the log's grammar, a re-run leaving it byte-identical, and a pre-seeded line for the key under another iteration raising its `<iterations>` to 2;
- an `(outside)` key for an edit path off the repository, and no command text in the log;
- no stamp, or no sessions dir: a 0-exit notice and no log written.

drift-kit/SPEC.md §Testing gains one sentence naming the fixture and these assertions. *Not yet applied.*

### (5) This repository's close binding {mechanical}

.claude/commands/close.md's **housekeeping** binding runs the arm after the stage-economics feed. *Not yet applied.*

> Then run `--emit manual-ops` and read its candidates as tooling opportunities: file one a tool would remove with `--emit file-gap`, and add a shape that is ordinary work to `DRIFT_KIT_MANUAL_OPS_IGNORE`.

scripts/drift-config.knobs binds `DRIFT_KIT_MANUAL_OPS_IGNORE` to this repository's read-only inspection shapes and its gate runner: `grep`, `git grep`, `git status`, `git log`, `git diff`, `git show`, `ls`, `wc`, `find`, and `bash gate-sdk/bin/run-gates.sh`. Each spelling is the ranking key's for that command: the leading binary, with its subcommand where the binary is one of the key's multi-command set (`git`, `bash` among them), so `bash gate-sdk/bin/run-gates.sh --run` keys as `bash gate-sdk/bin/run-gates.sh`.

## Producers and consumers

- **The arm** (delta 1): produced by the close binding's housekeeping (delta 5), which this repository runs at every close, and by `/economics` (delta 2) or any session ad hoc. All three knobs have working defaults, so the shipped configuration enables it.
- **The stdout ranking** (delta 1): read by the close's housekeeping and by the `/economics` narrative's bullet (delta 2). Each column has that reader: `calls` and `sessions` rank and size a candidate, `rows` names where it ran, `iterations` how persistent it is.
- **The log** (delta 1): written and read by the arm alone, its `<iterations>` column. `iteration`, `kind` and `key` are each read there; no count is logged, since no reader takes one.
- **The attribution** (delta 1): the stage-economics meter's transcript-to-row-key assignment, which `native/src/emit/stage_economics.rs` computes inside its passes (`supervision_pass`, `fanout_pass`) today. It is exposed for this arm's call and the meter keeps its output, which drift-kit/smoke/install.sh's stage-economics fixtures hold.
- **The shell key** (delta 1): `ranking_key` in `native/src/emit/scan_prompts.rs`, called unchanged; guard-kit's contract is read, not changed.
- **The knobs** (delta 3): read by the arm; their roster readers are `--emit knob-roster`, which `check-knob-citation` resolves knob citations against and `check-knob-default-coupling` couples a scalar default through, and the table validator.
- **Roster-holding readers of the new arm**, each red when it is missing: the `ARMS` table's own unit test, which resolves each row to its file and reds a file with no test module (gate-sdk/SPEC.md §The non-gate arm); this repository's `check-crate-arms`, which runs the crate's tests; and `check-reads-couples` and `check-gate-substrate-parity`, which read a row's declared knobs. drift-kit/README.md's arm list and `/economics` sentence are hand prose no gate holds, and `.workflow/release-declarations.md` takes the arm and its knobs under its *Behavior changes* and *Knob changes* sections. The on-site mirrors of drift-kit's SPEC and README regenerate by the command their freshness gate prints. The arm writes under `.metric/`, already gitignored and outside the close-surface roster, so no `.gitignore` line and no `close-surface:` declaration is owed.
- **Corpus narrowing:** none.

## Existing sections updated

- drift-kit/SPEC.md §The manual-operation meter, new (delta 1); §The `/economics` skill (delta 2); §Layout and configuration, three knob rows and the validator sentence (delta 3); §Testing (delta 4).
- `native/src/emit/manual_ops.rs`, new; `native/src/emit/mod.rs`, the `ARMS` row; `native/src/emit/stage_economics.rs`, the attribution exposed (delta 1).
- drift-kit/templates/economics.md — the chain step and the narrative bullet (delta 2).
- `native/src/knobs/drift_kit.rs` — the three rows and the integer validator (delta 3).
- drift-kit/smoke/install.sh and `drift-kit/smoke/manual-ops-fixture.jsonl`, new (delta 4).
- .claude/commands/close.md, scripts/drift-config.knobs (delta 5).
- drift-kit/README.md's arm list and its `/economics` sentence, which names the two-step chain `overhead-meter → stage-economics` and takes the third step (delta 2), `.workflow/release-declarations.md`, and the drift-kit SPEC and README mirrors (all deltas).
- `.workflow/surface-ceiling.txt` — the rows of drift-kit/templates/economics.md (delta 2) and .claude/commands/close.md (delta 5) re-stamped where they grow, and drift-kit/SPEC.md's row re-stamped where the merge grows it past its ceiling, before `drift-kit-tail-brevity` passes the file and re-stamps it again (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; the template's two-step chain is rephrased, not renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, its log and its knobs.
- [ ] **Instruction surfaces: instruction only** — the template step, its bullet and the binding sentence carry no grounds; §The manual-operation meter carries them.
- [ ] **Merged with no information lost** — the new section reads as one document with its sibling meters, and the `/economics` sentence is rephrased rather than appended to.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared.
- [ ] **Gaps filed** — any shape the first run here surfaces as a tooling candidate is filed, not fixed in this unit.
- [ ] **Built and run** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, `--run-consumer-smoke` (drift-kit's assertions run there), and the guard-kit fixture suites, whose `scan_prompts.rs` key the arm now calls.
- [ ] **Entry moved** — `--queue done manual-operation-spend-channel` in the build batch that lands this, a stage before the drain stage; no remote oracle gates it.
