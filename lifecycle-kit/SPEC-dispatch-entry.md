# SPEC amendment: dispatch-entry

A stage session can land its work commits before it runs its own `--enter-stage`, and nothing reds it. §check-stage-evidence catches a stamp committed after the work it preceded, never work that precedes the stamp. A same-stage re-entry finds the cursor already on its stage, so no entry gate reads the missing stamp either. The section's last ordering paragraph states this as the one case its assertions cannot reach: the interval between two stamps holds legitimate and illegitimate work alike, and nothing in the tree separates them. The recurrence is measured in `git log`: one iteration's second build batch merged two amendments, then stamped at the second merge's head. The iteration before carried the same shape, and both sessions ran on the cheaper tier.

**The ruling: the lead declares the boundary, and a commit-time gate reads it.** The entry offered two remedies. One is an oracle tying a session's work commits to its own stamp, with the tie left open because commits carry no session id. The other is a stamp-first line on the dispatch path the cheaper tier reads. The line is refused because the path already carries it: every stage template's first step is the stamp, and the build template adds "Every session still stamps". Both recurrences happened with that text in context, so another line buys nothing measured. The oracle is taken, and the tie is the **dispatch**, not the session. The party that knows a stage session is starting and has not yet entered is the lead that dispatched it. So the lead writes a scratch marker at dispatch. The dispatched session's stamp removes it. A commit made while a marker line stands, adding no stamp, reds. The section's own analysis rules this in rather than out. It names a declared boundary as the only thing that can separate the interval, and the dispatch marker is that boundary, declared by the one party that holds it.

**Refused: deriving the committing session's id at commit time.** Under this harness the id resolves in a commit hook (measured 2026-09-22: `--emit session-id` from this spec session printed its own stamp's id). It still fails as an oracle on sanctioned work. A stage session's worker agents are children of the same top-level session, laid out flat beside it, so a committing worker reads as an unstamped session. The boundary skills (`consult`, `release-sweep`) commit and never stamp. And the tie would be one harness's environment, where the marker is the kit's own.

**What stays prose.** A stage session run with no lead has no dispatch to declare, so no marker exists and the gate is inert. That path keeps the template's stamp-first step and the limit §check-stage-evidence states. The attested recurrences both ran under a lead.

## What changes

### (1) §bin/enter-stage.sh: the dispatch marker and its two writers {design-bearing}

**Not yet applied.** The grammar adds `--enter-stage --dispatch <stage>` and `--enter-stage --dispatch-withdraw <stage>`, both with exactly one operand and neither taking `--simulate`. Add a paragraph after the `--open-lead-journal` paragraph:

> **`--dispatch` — the lead's declaration that a stage session is starting.** `--dispatch <stage>` runs the `--simulate <stage>` pre-flight and, only when it would proceed, appends one line `<stage>` to the dispatch marker (`<scratch>/$LIFECYCLE_KIT_DISPATCH_MARKER_FILE`), creating it when absent. A refusal writes nothing and exits 1 with the simulate's output, so the one command both gates the dispatch and declares it. Every stamp this tool writes, the boundary reset's included, removes one line naming the stamped stage and deletes the file once it is empty. A marker with no such line is left as it is. `--dispatch-withdraw <stage>` removes one line naming `<stage>`, for a dispatched session that ended without entering, and is a reported no-op when there is none. Neither form writes anything tracked. The marker is lifecycle-kit's scratch, like the lead journal, and never lifecycle state: no stage reads it for the cursor, and the gate below reads it for one commit-time question. `check-dispatch-entry` is its reader.

In the surplus-argument paragraph, "`--rename` exactly one `<name>` and `--open-lead-journal` none" becomes "`--rename` exactly one `<name>`, `--dispatch` and `--dispatch-withdraw` exactly one `<stage>`, and `--open-lead-journal` none". In the wipe paragraph, "an ordinary stage entry appends and touches no scratch" becomes "an ordinary stage entry appends and wipes nothing".

### (2) §Layout and configuration: the marker knob {mechanical}

**Not yet applied.** After the `LIFECYCLE_KIT_LEAD_JOURNAL_FILE` bullet, add:

> - `LIFECYCLE_KIT_DISPATCH_MARKER_FILE` — the dispatch marker (§bin/enter-stage.sh); default `stage-dispatch.txt`, a **scratch-root name** resolved inside the scratch dir `GATE_SDK_TMP_DIR` names. Written by `--dispatch` and by every stamp, read by `check-dispatch-entry`. A scalar on the lead journal's ground, and not spared by the boundary wipe: the boundary stamp that would spare it is the stamp that removes its line.

The kit's static knob table gains the row, and the `--enter-stage` arm's roster already covers it through the `LIFECYCLE_KIT_*` family.

### (3) A new gate, check-dispatch-entry {design-bearing}

**Not yet applied.** Add a section after §check-stamp-subject:

> ### check-dispatch-entry
>
> Invariant: while the dispatch marker holds a line, a commit adds a stamp. The marker's lines are stage sessions a lead dispatched that have not yet entered (§bin/enter-stage.sh), so a commit made under one is work landing ahead of its session's stamp. That is the ordering case §check-stage-evidence states its own assertions cannot reach.
>
> **What the gate reads.** A `tier=commit-msg` gate, so it runs once per commit and skips the whole-tree battery. A pre-commit gate would run in the battery too, and a stage session runs the battery before its stamp (the build template's step 0). It reads the marker and the added stamps, and the added stamps are §check-stamp-subject's read, so the two gates never disagree about what a commit adds. **Red** when the marker holds a line and the commit adds no stamp. It names the outstanding stages and prints the remedy: run `--enter-stage <stage>` and commit the stamp on its own first, or, for a lead whose dispatched session ended without entering, `--enter-stage --dispatch-withdraw <stage>`.
>
> - **Skips, clean.** No marker, or an empty one. A no-argument run. A commit whose staged path set is exactly the gap inbox (`LIFECYCLE_KIT_GAP_INBOX_FILE`), which is the lead's one sanctioned commit (§templates/lead.md) and carries no stage work.
> - **Fail-closed.** An unreadable marker, a missing message file and an argument count the forms below do not admit all exit 2.
>
> **Honest limits.** With no lead nothing writes the marker, so the gate is inert and the case stays prose. A lead dispatching two batches in parallel holds two lines. A sibling that has entered and commits before the other has entered reds, and a retry after that entry clears it, since the second batch stamps first. A session that stamps and then lands work under another stage's name is outside this gate. The stamp's stage and the commit's content are §check-stamp-subject's and review's.
>
> The fixture form takes the marker, the staged and `HEAD` state-file blobs and a staged-path list as further arguments, so the pair runs hermetically: `bad/` is a work commit under a standing `build` line, `good/` the same commit adding the build stamp. `gate-tests/check-dispatch-entry.test.sh` covers the gap-inbox skip, the absent marker, the parallel two-line case and the argument edges.

**Batch condition, with `stamp-subject-merge-carve-out-unruled` (`SPEC-merge-stamps.md`).** That entry widens §check-stamp-subject's added-stamp read to exclude lines a merge parent carries. If it lands in this batch or an earlier one, this gate calls the same read, merge parents included, from one shared function in the gate modules. If this entry lands first, the gate reads the prior set from `HEAD`'s version alone, as §check-stamp-subject does today, and the later merge moves both gates onto the shared read in one commit.

The gate is born native (`native/src/gates/dispatch_entry.rs`, a `.gate` descriptor at `tier=commit-msg`, the fixture pair), is registered in `scripts/gates.list` and in `lifecycle-kit/README.md`'s gate list and `lifecycle-kit/smoke/install.sh`, and the commit-msg hook is regenerated with `--emit git-hooks --write`.

### (4) §check-stage-evidence: the unreachable ordering case is reached under a lead {design-bearing}

**Not yet applied.** In the paragraph beginning "**The one ordering case neither assertion above reaches**", replace the closing sentences from "Closing the remaining case is therefore a **composition**" to the end with:

> The declared boundary is the dispatch: a lead declares each stage session it starts, and `check-dispatch-entry` reds a commit landing under an undischarged declaration. That closes the case wherever a lead dispatches. A session run with no lead declares nothing, and there the case stays as stated. The per-stage output-surface roster stays refused on the grounds above, recorded so the next reader does not re-derive it.

### (5) templates/lead.md: the dispatch declares {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md`:

- The opening paragraph's "running `--enter-stage` only in its non-stamping `--simulate` and `--open-lead-journal` forms" becomes "running `--enter-stage` only in its non-stamping forms: `--simulate`, `--dispatch`, `--dispatch-withdraw` and `--open-lead-journal`".
- In §The lead model, the sentence ending "gates every dispatch cheaply first with `--enter-stage --simulate <next stage>`" becomes "gates every stage-session dispatch with `--enter-stage --dispatch <next stage>`, which runs that read and, on a clear verdict, declares the dispatch (lifecycle-kit/SPEC.md §bin/enter-stage.sh). A dispatched session that ends without entering is withdrawn with `--dispatch-withdraw <stage>`."
- §Stamps are authoritative's first sentence adds: "The dispatch marker is scratch, not lifecycle state."

The other `--simulate` reads in the template (the drain read in §Mechanical floor, the checks that follow a completion notification) stay `--simulate`: they declare no dispatch.

### (6) The pending release declarations name the gate, the forms and the knob {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`: a `Tightened gates` bullet for `check-dispatch-entry` (new, lifecycle-kit, commit-msg tier: while a lead-dispatched stage session has not entered, a commit that adds no stamp reds; register it, regenerate hooks, and dispatch through `--enter-stage --dispatch`), and a `Behavior changes` bullet for the two `--enter-stage` forms and `LIFECYCLE_KIT_DISPATCH_MARKER_FILE`.

## Producers and consumers

- **The dispatch marker** (deltas 1 and 2). Produced by `--dispatch`, which the lead template makes the lead's dispatch step (delta 5), so it is live wherever a consumer runs the lead. Removed by every stamp and by `--dispatch-withdraw`. Consumed by `check-dispatch-entry` at each commit, by a file read. Each line carries one field, the stage, and the gate's red message and the stamp's removal both read it.
- **The gate's verdict** (delta 3). Its consumer is the committing session, through the commit-msg hook's output and help line. The hook is generated from the descriptor, so registration is the enabling config, and this repo registers it in the same unit.
- **Roster-holding readers of the new names.** `scripts/gates.list`, the commit-msg hook's generated roster, `lifecycle-kit/README.md`'s gate list and `lifecycle-kit/smoke/install.sh` hold gate rosters (delta 3). The kit's static knob table and `--emit knob-roster` hold the knob (delta 2). `check-gate-assertions` reads the new section's assertions against markers in the module, and `check-comment-tier` any `spec:` directive the module mints.
- **Point 5.** Not reached; no corpus narrows.
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -e "open-lead-journal" -e "touches no scratch" -e "one ordering case" -e "LEAD_JOURNAL_FILE" -- lifecycle-kit native/src scripts .workflow`, run 2026-09-22.

- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the grammar sentence, the surplus-argument paragraph, the wipe paragraph and a new `--dispatch` paragraph (delta 1).
- `lifecycle-kit/SPEC.md` §Layout and configuration, a knob bullet (delta 2).
- `lifecycle-kit/SPEC.md`, a new §check-dispatch-entry (delta 3).
- `lifecycle-kit/SPEC.md` §check-stage-evidence, the ordering-case paragraph (delta 4).
- `native/src/emit/enter_stage.rs`, the two forms, the stamp-time removal and their tests (delta 1).
- `native/src/knobs/lifecycle_kit.rs`, the knob row (delta 2).
- `native/src/gates/dispatch_entry.rs`, `native/src/gates/mod.rs`, `lifecycle-kit/checks/check-dispatch-entry.gate`, its fixture pair and `lifecycle-kit/gate-tests/check-dispatch-entry.test.sh` (delta 3).
- `scripts/gates.list`, `scripts/git-hooks/commit-msg`, `lifecycle-kit/README.md` and `lifecycle-kit/smoke/install.sh` (delta 3).
- `lifecycle-kit/templates/lead.md`, the opening paragraph, §The lead model and §Stamps are authoritative (delta 5).
- `.workflow/release-declarations.md` (delta 6).

## Retired spellings

- None — no delta retires a name; `--simulate` keeps every use it has.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the marker, the two forms, the knob and the gate.
- [ ] **Instruction surfaces: instruction only.** The lead-template edits carry the instruction; the grounds sit in §bin/enter-stage.sh and §check-dispatch-entry.
- [ ] **Merged with no information lost.** §check-stage-evidence's ordering paragraph is re-phrased, and its refused roster keeps its grounds.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `build-work-before-entry-stamp` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
