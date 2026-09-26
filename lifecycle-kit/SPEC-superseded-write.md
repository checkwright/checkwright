# SPEC amendment: superseded-write

A stage session can write into a stage it no longer holds, and nothing refuses it. The attested case: a spec session committed an amendment edit on top of the align stamp while align was auditing that amendment, so the auditor reported on prose that no longer existed. Both sessions kept their rules. Spec checked `git status` before its commit and the index was clean, because a clean index does not say who holds the stage. The cursor does. The window stays open by design: under a lead, a stage session stays resumable after its successor enters. In the split posture it is the iteration's intent oracle. lifecycle-kit/templates/lead.md §The escalation protocol relays the oracle's answer to the asking session, which lands it, so the resumed predecessor owes no tracked write. Nothing holds that.

**The ruling: the workflow-state guard gains a third rule.** A dispatched stage session whose own stage is no longer the cursor's may not `Write` or `Edit` outside the scratch dir. It is the second rule's reads with one more comparison. The second rule refuses a write before the caller's stamp; this one refuses a write after the caller's stage has been left.

**Refused: a commit-time refusal in the git hook**, which is what the entry proposed first. A commit carries no session identity, and nothing a hook can read supplies one. The harness exports no per-agent id into a subagent's shell, only the root session's id, and `--emit-session-id` answers with the newest transcript, which a parallel dispatch can own (§bin/session-id.sh). A hook that cannot tell the superseded session's commit from the live stage's own commit can only refuse both or neither. The harness payload does carry the caller's identity, and the workflow-state guard already reads it.

**Refused: registering the guard on `Bash` too.** Every adopter's operator-owned settings file would need a new registration, and a shell command is not a path-shaped write this member can classify. Classifying a shell command as a tree write is the shell guard's machinery (§check-dispatch-entry's honest limits already say so for the second rule).

**What stays open is the shell.** A superseded session can still commit, through `git commit`, work it had already written before its successor entered, and a shell redirect writes a tracked file with no `Write` call. The first case needs an edit left uncommitted across the session's own report. A dispatched session may not end a turn on unfinished work (delegation-kit/templates/agent-execution.md), and the lead dispatches the successor only on the predecessor's completion notification (lifecycle-kit/templates/lead.md). So the residue needs a rule already broken at the predecessor's turn end.

**Measured at authoring (2026-09-26):**

- **No commit-time identity.** In a dispatched stage session's shell, `env` carries `CLAUDE_CODE_SESSION_ID` set to the root session's id and no agent id. `bash gate-sdk/bin/run-gates.sh --emit-session-id` returned the id of a grandchild the session had just dispatched, not the session's own stamped id.
- **The second rule's reads.** `native/src/hook/workflow_state.rs` `stamp_before_write` reads `LIFECYCLE_KIT_STAGE_SESSION_TYPES`, the payload's `agent_type` and `agent_id`, normalizes the id with `crate::sessions::normalize`, and reads the state file through `LIFECYCLE_KIT_STATE_FILE`. The new rule needs those reads plus `stages::current_stage` and the scratch dir.
- **This repo's roster is live.** `scripts/lifecycle-config.knobs` sets `LIFECYCLE_KIT_STAGE_SESSION_TYPES[] = stage-session`, and `.claude/settings.json` registers `--hook workflow-state-guard` on `Write|Edit`.
- **The attested case.** The gap bullet drained at the 2026-08-31 close (`d696756b`) records spec's `1aebbccd` landing on align's stamp `352baa5e`.

## What changes

### (1) The third rule {design-bearing}

**Not yet applied.** In `native/src/hook/workflow_state.rs`, once `stamp_before_write` finds the caller stamped, it runs the superseded-stage check before allowing:

- The caller's stage is the stage field of the last data line carrying the caller's normalized id whose stage is a `LIFECYCLE_KIT_STAGES` member.
- The cursor is `stages::current_stage` over the same state text.
- When the two differ, a write whose target resolves inside the scratch dir `GATE_SDK_TMP_DIR` names is allowed. The resolution is the module's own `resolve`, so a spelling cannot slip past it. Every other target is blocked.
- A cursor that is not a configured stage, or a scratch dir that cannot be resolved, declines through `hook::decline`, as the second rule's degradations do.

The block message names the caller's stage and the cursor's, and says:

> this '<type>' session (<id>) entered '<caller-stage>', and the cursor has moved to '<cursor-stage>': a later stage holds the tree. Answer through your report, and the live stage session lands the change. Your resume journal under the scratch dir stays writable.

`lifecycle-kit/gate-tests/stamp-before-write.test.sh` gains three cases on its sandbox, whose state file already stamps `scope` then `build`: the scope-stamped caller writing `notes.md` is blocked with both stage names in the message, the same caller writing under the scratch dir is allowed, and the build-stamped caller writing `notes.md` is still allowed.

### (2) The contract text {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §check-dispatch-entry, after the paragraph opening "**The write-time half rides the workflow-state guard.**", add:

> **A third rule refuses a write after the caller's stage has been left.** A roster-typed caller whose stamped stage is not the cursor's is blocked from writing outside the scratch dir that `GATE_SDK_TMP_DIR` names. The caller's stage is its last stamp's stage, so a same-stage sibling is never superseded, and at the `iteration` posture a session that entered a later stage holds the cursor. The rule exists because a stage session stays resumable after its successor enters, as the lead's intent oracle or to answer a routed question. The answer lands through the live stage session (§templates/lead.md), and a clean index says nothing about who holds the stage. The scratch dir stays writable, so the resumed session keeps its resume journal. A commit-time refusal is refused: a commit carries no session identity, the harness exports none into a subagent's shell, and `--emit-session-id` names the newest transcript (§bin/session-id.sh). A cursor that is not a configured stage, or an unresolvable scratch dir, declines. Its hermetic cases live in `gate-tests/stamp-before-write.test.sh`.

In the same section's **Honest limits** paragraph (the one opening "The rule reaches `Write` and `Edit` and nothing else."), "An unstamped session's shell write is not refused." becomes:

> An unstamped or superseded session's shell write is not refused, so neither is a superseded session's `git commit` of work it wrote before its successor entered. That needs work left uncommitted across the session's own turn end, which the dispatch policy already forbids.

The section's earlier **Honest limits** sentence "A session that stamps and then lands work under another stage's name is outside this gate." becomes:

> A session that stamps and then lands work under another stage's name is outside this gate. Its `Write` and `Edit` calls meet the workflow-state guard's third rule once its stage is left.

In §check-stage-evidence, "Its second rule, a dispatched stage session's write before its stamp, reads `LIFECYCLE_KIT_STAGE_SESSION_TYPES` and is stated at §check-dispatch-entry." becomes:

> Its second and third rules, a dispatched stage session's write before its stamp and after its stage is left, read `LIFECYCLE_KIT_STAGE_SESSION_TYPES` and are stated at §check-dispatch-entry.

In §Layout and configuration, the `LIFECYCLE_KIT_STAGE_SESSION_TYPES` line's "read by the workflow-state guard's stamp-before-write rule (§check-dispatch-entry). The default is empty, which makes the rule inert." becomes:

> read by the workflow-state guard's stamp-before-write and superseded-stage rules (§check-dispatch-entry). The default is empty, which makes both rules inert.

## Producers and consumers

- **The superseded-stage block** (delta 1). Producer: the workflow-state guard on a `PreToolUse(Write|Edit)` call. Enabled where a consumer registers the guard and sets `LIFECYCLE_KIT_STAGE_SESSION_TYPES`, as this repo does (measured above). Consumer: the calling stage session, through the harness's block channel. It answers through its report, and the lead relays the answer to the live stage session.
- **Reads.** `agent_type`, `agent_id`, `LIFECYCLE_KIT_STAGE_SESSION_TYPES` and `LIFECYCLE_KIT_STATE_FILE` are already the second rule's. `LIFECYCLE_KIT_STAGES` gives the caller-stage filter and `GATE_SDK_TMP_DIR` the exemption. Both are already on the kit's and gate-sdk's knob tables. The member's declared read roster in `native/src/hook/mod.rs` (gate-sdk/SPEC.md §The non-gate arm) carries only the first two today and gains both.
- **Roster readers.** `docs/enforcement.md` lists the member by name, event and matcher, and none of those change. guard-kit's settings template registers the member unchanged.
- **Point 5.** No corpus narrows.
- **Point 6.** Not obliged.

## Existing sections updated

Roster from `grep -n "stamp-before-write\|second rule\|workflow-state guard" lifecycle-kit/SPEC.md` and `grep -rln "workflow-state-guard\|workflow_state" native/src docs/enforcement.md guard-kit/templates`, run 2026-09-26.

- `native/src/hook/workflow_state.rs`, the member's read roster in `native/src/hook/mod.rs`, and `lifecycle-kit/gate-tests/stamp-before-write.test.sh` (delta 1).
- lifecycle-kit/SPEC.md §check-dispatch-entry, §check-stage-evidence and §Layout and configuration (delta 2).
- The on-site mirror of `lifecycle-kit/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (delta 2).
- `.workflow/release-declarations.md`: one Behavior changes bullet naming `--hook workflow-state-guard` (delta 1). A dispatched stage session whose stage the cursor has left is refused `Write` and `Edit` outside the scratch dir.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the third rule.
- [ ] **Instruction surfaces: instruction only.** No template changes. The block message carries the act, and the grounds sit in delta 2's SPEC text.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines. The one added paragraph has no passage to rewrite.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `stage-cursor-unread-by-index-check` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
