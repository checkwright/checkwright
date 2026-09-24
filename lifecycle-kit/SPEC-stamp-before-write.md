# SPEC amendment: stamp-before-write

A lead-dispatched stage session can do its whole batch without running `--enter-stage`. This happened at `native-shell-guard` build b5, a cheaper-tier session. No stamp records it, and its dispatch-marker line stayed unconsumed until the successor withdrew it. The work rode a sibling's stamp, and the per-session audit trail lost it. `check-dispatch-entry` is the one mechanical reader of the case, and it is a commit-msg gate by design (lifecycle-kit/SPEC.md §check-dispatch-entry). It fires at a commit and never at a tree write. The same session deleted `scripts/bash-guard.sh` before the hook swap, which opened a fail-open window for every session sharing the tree.

**The ruling: the workflow-state guard gains a second rule. A dispatched stage session writes nothing before its stamp.** The refusal is keyed on **who is calling**: the payload's `agent_type` names a configured stage-session type, and its `agent_id` carries no stamp. It is not keyed on the dispatch marker, which is the design the queue entry inferred and left pending on exactly this ground. A marker-keyed refusal reaches every session sharing the tree while any line is outstanding. That includes the lead, a stamped sibling in a parallel batch, and every subagent a stamped session dispatches. It also misses a dispatch the lead never declared. Keying on the caller reaches the dispatched stage sessions and nothing else. It needs no marker at all, and it leaves `check-dispatch-entry`'s commit-time assertion standing beside it.

**The rule rides the existing member and not a new one.** The workflow-state guard is already a lifecycle-kit `PreToolUse(Write|Edit)` member, registered in this tree and in guard-kit's wiring template. Its first rule is the other half of the same property, that lifecycle state has one writer and the stamp comes first. A new member would need a new settings registration in every adopter's operator-owned settings file, for no separation a reader would use.

**Measured at authoring (2026-09-24):**

- **The id a stage session stamps.** `--emit-session-id` derives a dispatched child's id from its subagent transcript, `agent-<id>.jsonl`, by stripping `agent-` and taking eight characters (lifecycle-kit/SPEC.md §bin/session-id.sh). This session's transcript records `"agentId":"a9ba9909f687031fe"` and it stamped `a9ba9909` (`.workflow/WORKFLOW-STATE.txt`).
- **What a subagent's hook payload carries.** `.workflow/subagent-stop-liveness.log`'s `keys=` field shows `agent_id`, `agent_type` and `session_id` on a `SubagentStop` payload, with `session=` the lead's uuid. So `session_id` does not identify a dispatched session, and `agent_id` does. On `PreToolUse` the field's presence is measured too: `agent-dispatch-guard` reads `agent_id` to detect a nested dispatch (`native/src/hook/dispatch.rs:56`), and its nested-dispatch advisory fired on this spec session's own `Agent` call. The hooks reference (`https://code.claude.com/docs/en/hooks.md`, *common input fields*) states that tool events fired inside a subagent carry `agent_id` and `agent_type`.
- **This repo's stage-session type.** `.claude/agents/stage-session.md` declares `name: stage-session`, the type the lead dispatches (`scripts/delegation-config.knobs` rosters it as mutating).

**Inferred, cannot run before build:** a `PreToolUse` payload's `agent_id` equals the calling subagent's transcript `agentId`, so it normalizes to that session's stamp id — no registered member records a `PreToolUse` payload's field values, and delta 1 lands the first reader that could; build observes it on a dispatched session's `Write` before the refusal is trusted.

## What changes

### (1) The workflow-state guard refuses a dispatched stage session's write before its stamp {design-bearing}

**Not yet applied.** In lifecycle-kit/SPEC.md §check-dispatch-entry, after the honest-limits paragraph:

> **The write-time half rides the workflow-state guard.** That `PreToolUse(Write|Edit)` member (§check-stage-evidence) carries a second rule. It blocks a write when the payload's `agent_type` is a member of `LIFECYCLE_KIT_STAGE_SESSION_TYPES` and no data line of the state file carries the caller's id in its session field. The caller's id is the payload's `agent_id` normalized by §bin/session-id.sh's rule, so it is the id that session's own entry stamps. The block names the remedy, `--enter-stage <stage>` first, and why: its stamp is what records the session's work. A caller with no `agent_type` is not a dispatched session and the rule is silent. That covers a top-level session, the lead and a stage run by hand. A subagent a stamped session dispatches carries its own type, so it is out of the roster.
>
> **It keys on the caller, never on the marker.** A marker-keyed refusal would reach every session sharing the tree while any line is outstanding: the lead, a parallel sibling that has entered, and that sibling's own subagents. It would still miss a dispatch nobody declared. The marker keeps its one reader, this gate, and the two halves together cover the write and the commit.
>
> **Degradation costs this rule alone.** An unresolvable roster or an unreadable state file declines through `hook::decline` (gate-sdk/SPEC.md §The harness-integration arm), and the first rule still runs. An empty roster is the not-adopted case and makes the rule inert, not a decline.
>
> **Honest limits.** The rule reaches `Write` and `Edit` and nothing else. An unstamped session's shell write is not refused. The first `Write`/`Edit` it attempts is, and its commit meets this gate. Refusing the `Bash` tool wholesale is refused: the build template's step 0 runs its battery before the stamp. Classifying a shell command as a tree write is the shell guard's machinery, not this member's. The id derivation's newest-transcript race can stamp a parallel sibling's id (§bin/session-id.sh). The session it skipped is then refused and clears the refusal by entering again, which appends a stamp under its own id.

In lifecycle-kit/SPEC.md §check-stage-evidence, the `workflow-state-guard` paragraph's last sentence "No knob is added: there is no value to configure." becomes:

> Its first rule adds no knob, because there is no value to configure. Its second rule, a dispatched stage session's write before its stamp, reads `LIFECYCLE_KIT_STAGE_SESSION_TYPES` and is stated at §check-dispatch-entry.

### (2) `LIFECYCLE_KIT_STAGE_SESSION_TYPES` names the dispatched stage-session types {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §Layout and configuration, after the `LIFECYCLE_KIT_DISPATCH_MARKER_FILE` bullet:

> - `LIFECYCLE_KIT_STAGE_SESSION_TYPES` — the agent types a lead dispatches stage sessions as, read by the workflow-state guard's stamp-before-write rule (§check-dispatch-entry). The default is empty, which makes the rule inert. A type is the harness's name for an agent definition, and the kit ships none.

The kit's static table declares the array with an empty default (`native/src/knobs/lifecycle_kit.rs`). `lifecycle-kit/templates/lifecycle-config.knobs` carries it commented. This repo binds `LIFECYCLE_KIT_STAGE_SESSION_TYPES[] = stage-session` in `scripts/lifecycle-config.knobs`. The workflow-state guard's `HOOKS` row declares `LIFECYCLE_KIT_STAGE_SESSION_TYPES` and `LIFECYCLE_KIT_STATE_FILE` beside `GATE_SDK_WORKFLOW_DIR`.

In `lifecycle-kit/README.md`, step 5's hook sentence gains: "With `LIFECYCLE_KIT_STAGE_SESSION_TYPES` set, it also refuses a dispatched stage session's write before that session's stamp."

## Producers and consumers

- **The refusal (delta 1).**
  - Producer: the workflow-state guard on a `Write`/`Edit` call, registered in this tree's settings and in guard-kit's wiring template. The enabling config is the roster this repo binds in delta 2.
  - Consumer: the calling session. The harness feeds a `PreToolUse` block's stderr back to the model, and the block names the remedy.
- **The decline (delta 1).** Its channel is the one `hook::decline` writes. This iteration's decline-channel amendment makes that the advise envelope on `PreToolUse`. The act here is the same call whether or not that amendment lands in the same build batch; only the channel the session sees differs.
- **The knob (delta 2).** Its reader is the guard's second rule. As a declared knob it has three roster-holding readers: the kit's static table, `check-knob-citation`, and the `EVERY_HOOK_KNOB` derivation through the `HOOKS` row. Each is listed as an update target.
- **Fields read.** `agent_type` and `agent_id` from the payload, and each state-file data line's session field. Nothing new is written.
- **Point 5.** No corpus narrows.
- **Point 6.** The obliged members are this repo's stage sessions, all dispatched as `stage-session`. Each one's satisfying value is its own stamp, which its first step writes.
- **Sibling unit.** The waiver-writer amendment's waiver-bearing marker line is invisible to this rule, which reads no marker.

## Existing sections updated

The roster comes from `git grep -n "workflow-state-guard\|workflow_state\|No knob is added\|LIFECYCLE_KIT_DISPATCH_MARKER_FILE" -- '*.md' '*.rs' '*.knobs' ':!docs/*'`, run 2026-09-24.

- `lifecycle-kit/SPEC.md` §check-dispatch-entry and §check-stage-evidence (delta 1).
- `lifecycle-kit/SPEC.md` §Layout and configuration (delta 2).
- `native/src/hook/workflow_state.rs`, the second rule, its per-rule decline and its unit tests (delta 1).
- `native/src/hook/mod.rs`, the member's `HOOKS` row (delta 2).
- `native/src/knobs/lifecycle_kit.rs`, `lifecycle-kit/templates/lifecycle-config.knobs` and `scripts/lifecycle-config.knobs` (delta 2).
- `lifecycle-kit/README.md`, step 5 (delta 2).
- `.workflow/release-declarations.md` (deltas 1 and 2). Under Behavior changes: with `LIFECYCLE_KIT_STAGE_SESSION_TYPES` set, the workflow-state guard refuses a dispatched stage session's `Write`/`Edit` before that session's stamp.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/lifecycle-kit/SPEC.md` and `docs/lifecycle-kit/README.md`.

## Retired spellings

- None — the deltas add a rule and a knob, and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the refusal and the knob.
- [ ] **Instruction surfaces: instruction only.** The README edit carries the act.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines, and the marker-keyed alternative's grounds move into §check-dispatch-entry.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `unstamped-session-tree-edit` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Cannot-run claim discharged.** The `agent_id` equality is observed on a dispatched session's `Write` and the marker is deleted, or the rule is narrowed to what was observed.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
