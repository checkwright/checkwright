# SPEC amendment: lead-journal-arm

§The journal arm reaches a stage session's journal through that session's stamp, and its honest limits record that a supervising session's journal has no stamp, so the arm does not reach it. A lead therefore appends by a hand-spelled shell redirect, the shape the manual-ops meter ranked on the queue entry. This amendment gives the arm a second form that names the lead journal by its knob and reads no stamp.

**Ruled out: the shape on `DRIFT_KIT_MANUAL_OPS_IGNORE`.** That removes the row from the meter and leaves the redirect, so every lead finding still costs a hand-spelled call, and the count that would say so is gone.

## What changes

### (1) `--emit journal --lead` appends to the lead journal {design-bearing} {user-facing: operator direction 2026-10-08 on the queue entry — the unit is in this iteration's set, and the entry leaves the arm-form-or-ignore choice to this stage}

**Not yet applied.** A second form of the existing arm, on `--foreign-resume <key> --close`'s precedent of a flag that selects a form: no arm-table row and no front-end case arm.

`run-gates.sh --emit journal --lead [--] "<text>"` appends its operand to `<scratch>/$LIFECYCLE_KIT_LEAD_JOURNAL_FILE`, the file `--enter-stage --open-lead-journal` opens (§bin/enter-stage.sh), and with no operand appends its standard input.

- **No stamp is read and no id is derived.** The path is the knob's, so the form takes no walk and the newest-transcript race of the stage form does not reach it.
- **The write** is the stage form's: verbatim, one append, a closing newline added where it lacks one, nothing prefixed.
- **Stdout** is one line, `journal: <path> lead`, the anchored path. It carries no id field, since the form derived none.
- **Refusals, exit 2, nothing written.** The stage form's argv refusals, unchanged: an empty or whitespace-only text, a second operand, an unrecognized leading `-`, and a linked worktree. And two of its own: an absent journal file, and a file whose last segment is disposed, its last non-empty line being `DISPOSED`. Each names the remedy, `--enter-stage --open-lead-journal` first. A line appended after the mark would turn a discharged segment back into an undisposed one that the opener then keeps and the boundary advisory then reports.
- **`--lead` is recognized only as the first argument**, so a text that begins with it is passed after `--`.

Its declared roster gains `LIFECYCLE_KIT_LEAD_JOURNAL_FILE` and `GATE_SDK_TMP_DIR`. The disposed read is the segment reader the opener and the boundary advisory already share (§bin/enter-stage.sh), called a third time rather than copied.

**Honest limit.** The flag is the caller's claim to be the supervising session, and nothing checks it: a stage session that passes `--lead` writes the lead's file. The redirect it replaces checked nothing either, and an identity read would bring back the race the form exists to avoid, a lead with a live child deriving the child's id.

### (2) The lead template instructs the form, and the arm's honest limit is rewritten {mechanical}

**Not yet applied.**

- `lifecycle-kit/templates/lead.md`, the sentence after the opener instruction: append to your journal with `--emit journal --lead "<finding>"`.
- `lifecycle-kit/SPEC.md` §The journal arm: the lead sentence gains the second form, and the honest-limits sentence "A supervising session's journal has no stamp, so the arm does not reach it" is replaced by the limit delta 1 states. The section's last paragraph keeps the raw append as the legal fallback for both forms.
- `lifecycle-kit/README.md`: the form beside the stage form it already lists.

## Producers and consumers

- **The `--lead` form.** *Producer:* a lead session, instructed by `templates/lead.md`. *Enabling config:* none beyond the defaults; `LIFECYCLE_KIT_LEAD_JOURNAL_FILE` defaults to a scratch-root name and this repo leaves it there. *Consumers of what it writes:* the readers the lead journal already has, the resumed lead, the opener's segment reader and the boundary's undisposed advisory. The write adds no line shape those readers parse: they read headings and the `DISPOSED` mark, and the form writes neither.
- **The stdout line.** Its reader is the calling lead, which reads the path.
- **Roster-holding readers of the arm's argv,** by `git grep -n 'emit journal' -- ':!docs'`: `lifecycle-kit/SPEC.md` (§The stage-machine adapters and §The journal arm), `lifecycle-kit/README.md`, `delegation-kit/templates/agent-execution.md`, `.claude/agents/stage-session.md`, `.workflow/release-declarations.md`, the entry note in `native/src/emit/enter_stage.rs` and the usage string in `native/src/emit/journal.rs`. Each instructs or describes the stage form, which is unchanged; the usage string alone gains the second form.
- **Roster-holding readers of the arm's declared knobs:** the arm's own row in the crate's arm table, from which `--emit knob-roster` derives.
- **No corpus is narrowed and no enumerable corpus is obliged member by member**, so causal-completeness points 5 and 6 bind nothing here.

**Measured at authoring:** in the one live lead transcript read, every append redirect names the lead journal (16 occurrences of `>> <path>`, one target). The manual-ops meter logs keys and never text, so the share across the eight iterations it counted is not recoverable from it.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §The journal arm — the second form, its refusals, its stdout line, the declared roster and the rewritten honest limit (deltas 1 and 2).
- `native/src/emit/journal.rs` — the form and its usage string (delta 1).
- `lifecycle-kit/gate-tests/journal-arm.test.sh` — the hermetic cases: an append after an open heading, standard input, an absent file refused, a disposed last segment refused, the worktree refusal, and `--` before a text that begins with `--lead` (delta 1).
- `lifecycle-kit/templates/lead.md`, `lifecycle-kit/README.md` (delta 2).
- `.workflow/release-declarations.md` — a row for the form (delta 1).
- `docs/lifecycle-kit/` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).

## Retired spellings

- None — no delta of this amendment retires a name; delta 2 rewrites one honest-limit sentence in place.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Queue entry done** — `--queue done lead-journal-append-arm` in the merge commit, before the stage that drains the queue.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
