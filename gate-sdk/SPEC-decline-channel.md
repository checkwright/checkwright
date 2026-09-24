# SPEC amendment: decline-channel

gate-sdk/SPEC.md §The harness-integration arm has a member that finds its own declared knob unresolved decline "loudly (`hook::decline`)". It defines "loudly" as stderr at exit 0 with no envelope, and says that on a `PreToolUse` call the harness shows that text to the operator and not to the model. **The harness shows it to nobody.** The hooks reference (`https://code.claude.com/docs/en/hooks.md`, *Exit code 0*) says stderr from a hook that exits 0 "goes to the debug log only, never the transcript, and Claude never sees it". So a declined member fails open unseen from both sides. The one reader who could repair the knob is not told that the guard stopped enforcing, and neither is the operator.

**The ruling: a decline speaks through the envelope the firing event reads, chosen by the event, and never changes the verdict.**

- **On `PreToolUse`, the envelope is the advise envelope** (`hookSpecificOutput.additionalContext`). The reference documents it as context added for the model, and the tool call proceeds through the normal permission flow when no `permissionDecision` is set. That is the envelope every `PreToolUse` member's degraded path already writes: the dispatch guard's, the budget guard's, the workflow-state guard's, and the shell guard's consumer-rule fault.
- **On every other event, the envelope is the universal `systemMessage` field**, which the reference documents as a warning shown to the user. On `Stop`/`SubagentStop` the model cannot be told without a verdict change. That event's `additionalContext` "keeps the subagent running", and its exit 2 and `decision: "block"` both refuse the stop. A decline that changed the verdict would be a block, which is what the fail-open rule exists to refuse. So the operator is told and the model is not, and the section says so.

**Measured at authoring (2026-09-24):**

- **The callers.** `git grep -n "hook::decline" -- native/src` returns three sites, all in `native/src/hook/stop_liveness.rs` (the `SubagentStop` member). No `PreToolUse` member calls it today. The `PreToolUse` half of this ruling fixes the contract that the next member inherits, and the first such member is the stamp-before-write rule lifecycle-kit adds to the workflow-state guard this iteration.
- **The channels, read from the reference** (curl of the page above, run 2026-09-24). The exit-0 stderr sentence is under *Exit code 0*. The `PreToolUse` `additionalContext` row is under *PreToolUse decision control*. `SubagentStop`'s `additionalContext` and block semantics are under *Stop decision control* and the SubagentStop section. `systemMessage` is under *JSON output*. `hookEventName` must match the firing event, or the event-specific fields are ignored. That is under *JSON output* as well.
- **Observed in this repo's own sessions.** A `PreToolUse` advise envelope from `agent-budget-guard` reached the calling model on an allowed `Agent` call. The `SubagentStop` payload carries `hook_event_name` (`.workflow/subagent-stop-liveness.log`'s `keys=` field).

## What changes

### (1) `hook::decline` writes the firing event's envelope {design-bearing}

**Not yet applied.** In gate-sdk/SPEC.md §The harness-integration arm, the absent-binary paragraph's sentences from "A knob that cannot resolve is seen inside the member instead:" through "…on a `PreToolUse` call the harness shows that text to the operator and not to the model." become:

> A knob that cannot resolve is seen inside the member instead, and a member that finds its own declared knob unresolved declines through `hook::decline`. The decline allows the call, exits 0, and writes the envelope the firing event reads, chosen by the payload's `hook_event_name`. On `PreToolUse` that is the advise envelope (`hookSpecificOutput.additionalContext`), which reaches the session model and changes no verdict. On any other event, or where the payload does not name one, it is the universal `systemMessage` field, which reaches the operator. On a turn-end event no envelope reaches the model without refusing the stop, and a decline that refused would be the block this rule exists to avoid. The text names the member, the unresolved knob's reason, and that the rule was not enforced on this call. The same text also goes to stderr, which at exit 0 the harness writes to its debug log alone. Stderr is kept as that record and is never the channel.

In `native/src/hook/mod.rs`, `decline` takes the payload. It writes the advise envelope where `hook_event_name` is `PreToolUse`, and `{"systemMessage": <text>}` otherwise, serialized through the module's own escaper, then the text to stderr, then exits 0. Its three `stop_liveness` call sites pass the payload they already hold. A unit test holds both envelopes, the `hookEventName` of the advise one, and the exit status.

In delegation-kit/SPEC.md §The turn-end liveness hook, the contract sentence "…exit 0 on every other path, and no hook JSON on either." becomes:

> …exit 0 on every other path, and no hook JSON on either, except for the knob-fault decline. That decline writes the `systemMessage` envelope gate-sdk/SPEC.md §The harness-integration arm rules, so the operator learns that the turn end went unguarded.

## Producers and consumers

- **The decline envelope.**
  - Producer: `hook::decline`, reached when a member's own declared knob fails to resolve. It is reachable in every deployed configuration: a malformed value in a kit's knob file produces it.
  - Consumers: on `PreToolUse`, the session model, through the harness's context injection; on any other event, the operator, through the harness's user-facing warning. No gate reads it.
- **Its fields.** `hookEventName` is read by the harness, which ignores the event-specific fields on a mismatch, so it is always the firing event's name. `additionalContext` and `systemMessage` are each read by the party named above. No other field is added.
- **Existing readers of member stdout.** Every `PreToolUse` member already writes the advise envelope on some path, so no reader meets a new shape. The `SubagentStop` member's stdout was contracted empty. That contract is the delegation-kit sentence this delta rewrites, and the member's tests (`native/src/hook/stop_liveness.rs`) assert stdout only through `fire`, which the decline path never reaches.
- **Point 5.** No corpus narrows.
- **Point 6.** The obliged set is the decline's callers, measured above as three sites in one member. Each passes its payload.

## Existing sections updated

The roster comes from `git grep -n "hook::decline\|fn decline\|no hook JSON\|declines loudly\|declines with a diagnostic" -- '*.rs' '*.md'`, run 2026-09-24.

- `gate-sdk/SPEC.md` §The harness-integration arm (delta 1).
- `native/src/hook/mod.rs`, `decline` and its unit test (delta 1).
- `native/src/hook/stop_liveness.rs`, the three call sites and the module header's "emits no hook JSON" clause (delta 1).
- `delegation-kit/SPEC.md` §The turn-end liveness hook (delta 1).
- `native/src/runner.rs`, the `--hook` usage text's "declines with a diagnostic on stderr" clause. It becomes "declines, telling the session through the event's envelope where the member can run, and on stderr where the binary is absent" (delta 1).
- `.workflow/release-declarations.md` (delta 1). Under Behavior changes: a hook member that cannot resolve its own knob now tells the session model on `PreToolUse`, and the operator on other events, instead of writing to the harness's debug log alone.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md` and `docs/delegation-kit/SPEC.md`.

## Retired spellings

- None — the delta rewrites a channel's prose and retires no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the decline envelope.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** The SPEC edit re-phrases the absent-binary paragraph's decline sentences, and the channel facts move with it.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `hook-decline-model-invisible` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
