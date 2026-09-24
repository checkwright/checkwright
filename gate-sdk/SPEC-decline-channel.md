# SPEC amendment: decline-channel

gate-sdk/SPEC.md §The harness-integration arm has a member that finds its own declared knob unresolved decline "loudly (`hook::decline`)". It defines "loudly" as stderr at exit 0 with no envelope, and says that on a `PreToolUse` call the harness shows that text to the operator and not to the model. **The harness shows it to nobody.** The hooks reference (`https://code.claude.com/docs/en/hooks.md`, *Exit code 0*) says stderr from a hook that exits 0 "goes to the debug log only, never the transcript, and Claude never sees it". So a declined member fails open unseen from both sides. The one reader who could repair the knob is not told that the guard stopped enforcing, and neither is the operator.

**The ruling: a decline speaks through the envelope the firing event reads, chosen by the event, and never changes the verdict.**

- **On `PreToolUse`, the envelope is the advise envelope** (`hookSpecificOutput.additionalContext`). The reference documents it as context added for the model, and the tool call proceeds through the normal permission flow when no `permissionDecision` is set. That is the envelope every `PreToolUse` member's degraded path already writes: the dispatch guard's, the budget guard's, the workflow-state guard's, and the shell guard's consumer-rule fault.
- **On every other event, the envelope is the universal `systemMessage` field**, which the reference documents as a warning shown to the user. On `Stop`/`SubagentStop` the model cannot be told without a verdict change. That event's `additionalContext` "keeps the subagent running", and its exit 2 and `decision: "block"` both refuse the stop. A decline that changed the verdict would be a block, which is what the fail-open rule exists to refuse. So the operator is told and the model is not, and the section says so.

**The front end's absent-binary decline is the same rule at its other site, so it is in this unit.** In a tree with no runnable binary, `exec_arm` in `gate-sdk/bin/run-gates.sh` and `run-gates.ps1` writes its diagnostic to stderr and exits 0 for `--hook`. That reaches the same debug log, and every hook guard in the tree is then off with nobody told. §The harness-integration arm states both declines under one fail-open rule. The stub cannot read the payload's event without a JSON reader, which the binary-less path by definition lacks, so it writes the universal `systemMessage` envelope on every event. Included by the scope oracle's decision, relayed by the lead at this spec (2026-09-24). A later scope or spec may revise it.

**Measured at authoring (2026-09-24):**

- **The callers.** `git grep -n "hook::decline" -- native/src` returns three sites, all in `native/src/hook/stop_liveness.rs` (the `SubagentStop` member). No `PreToolUse` member calls it today. The `PreToolUse` half of this ruling fixes the contract that the next member inherits, and the first such member is the stamp-before-write rule lifecycle-kit adds to the workflow-state guard this iteration.
- **The channels, read from the reference** (curl of the page above, run 2026-09-24). The exit-0 stderr sentence is under *Exit code 0*. The `PreToolUse` `additionalContext` row is under *PreToolUse decision control*. `SubagentStop`'s `additionalContext` and block semantics are under *Stop decision control* and the SubagentStop section. `systemMessage` is under *JSON output*. `hookEventName` must match the firing event, or the event-specific fields are ignored. That is under *JSON output* as well.
- **The front end's decline.** `gate-sdk/bin/run-gates.sh` `exec_arm` prints the unavailable diagnostic with `printf … >&2`, then `exit "$ARM_UNAVAILABLE_STATUS"`, which is 0 for `--hook` and `--statusline` (`FAIL_OPEN_ARMS`). `run-gates.ps1` does the same through `Write-StubError` and `exit $unavailable`. gate-sdk/SPEC.md §run-gates says of that diagnostic that "the failure is loud on both settings", which the reference's exit-0 sentence falsifies for the fail-open setting. `--run-front-end-parity` (`native/src/emit/front_end_parity.rs`) compares both stubs' stdout and stderr, and its corpus carries a "binary absent, leading --hook" case and a "binary absent, leading --statusline" case.
- **Observed in this repo's own sessions.** A `PreToolUse` advise envelope from `agent-budget-guard` reached the calling model on an allowed `Agent` call. The `SubagentStop` payload carries `hook_event_name` (`.workflow/subagent-stop-liveness.log`'s `keys=` field).

## What changes

### (1) `hook::decline` writes the firing event's envelope {design-bearing}

**Not yet applied.** In gate-sdk/SPEC.md §The harness-integration arm, the absent-binary paragraph's sentences from "A knob that cannot resolve is seen inside the member instead:" through "…on a `PreToolUse` call the harness shows that text to the operator and not to the model." become:

> A knob that cannot resolve is seen inside the member instead, and a member that finds its own declared knob unresolved declines through `hook::decline`. The decline allows the call, exits 0, and writes the envelope the firing event reads, chosen by the payload's `hook_event_name`. On `PreToolUse` that is the advise envelope (`hookSpecificOutput.additionalContext`), which reaches the session model and changes no verdict. On any other event, or where the payload does not name one, it is the universal `systemMessage` field, which reaches the operator. On a turn-end event no envelope reaches the model without refusing the stop, and a decline that refused would be the block this rule exists to avoid. The text names the member, the unresolved knob's reason, and that the rule was not enforced on this call. The same text also goes to stderr, which at exit 0 the harness writes to its debug log alone. Stderr is kept as that record and is never the channel.

In `native/src/hook/mod.rs`, `decline` takes the payload. It writes the advise envelope where `hook_event_name` is `PreToolUse`, and `{"systemMessage": <text>}` otherwise, serialized through the module's own escaper, then the text to stderr, then exits 0. Its three `stop_liveness` call sites pass the payload they already hold. A unit test holds both envelopes, the `hookEventName` of the advise one, and the exit status.

In delegation-kit/SPEC.md §The turn-end liveness hook, the contract sentence "…exit 0 on every other path, and no hook JSON on either." becomes:

> …exit 0 on every other path, and no hook JSON on either, except for the knob-fault decline. That decline writes the `systemMessage` envelope gate-sdk/SPEC.md §The harness-integration arm rules, so the operator learns that the turn end went unguarded.

### (2) The front end's absent-binary `--hook` decline writes a `systemMessage` envelope {design-bearing}

**Not yet applied.** In gate-sdk/SPEC.md §The harness-integration arm, the absent-binary paragraph's rule sentence "…the front-end writes the diagnostic and the remedy to stderr and exits `0`, so the guard declines rather than wedging the session." becomes:

> …the front-end exits `0`, so the guard declines rather than wedging the session. For `--hook` it writes a fixed `systemMessage` envelope on stdout, telling the operator that the tree's hook guards are not running and naming the build command. The diagnostic and its remedy go to stderr as before. A stub has no JSON reader, so it cannot choose the event's envelope the way `hook::decline` does. The universal field is the one every hook event accepts. The envelope is a literal carrying no interpolated value, so it needs no escaper, and the path-bearing diagnostic stays on stderr. `--statusline` takes no envelope, because the harness renders its stdout as the status line.

In gate-sdk/SPEC.md §run-gates, the `--hook` paragraph's "the front-end writes nothing on stdout along this path" becomes "the front-end writes nothing on stdout when the binary runs". In the `exec_arm` paragraph, "The diagnostic is written to stderr either way, so the failure is loud on both settings; only the status differs," becomes:

> The diagnostic is written to stderr on both settings. At exit 0 the harness writes stderr to its debug log alone, so the fail-open `--hook` setting also writes the `systemMessage` envelope (§The harness-integration arm), and that envelope is what makes it loud;

In `gate-sdk/bin/run-gates.sh` and `gate-sdk/bin/run-gates.ps1`, the unavailable branch prints the envelope on stdout when the arm is `--hook`, byte-identical in both stubs, before it exits. In `native/src/emit/front_end_parity.rs`, the "binary absent, leading --hook" case expects the envelope on stdout. The "binary absent, leading --statusline" case expects empty stdout.

## Producers and consumers

- **The decline envelope.**
  - Producer: `hook::decline`, reached when a member's own declared knob fails to resolve. It is reachable in every deployed configuration: a malformed value in a kit's knob file produces it.
  - Consumers: on `PreToolUse`, the session model, through the harness's context injection; on any other event, the operator, through the harness's user-facing warning. No gate reads it.
- **Its fields.** `hookEventName` is read by the harness, which ignores the event-specific fields on a mismatch, so it is always the firing event's name. `additionalContext` and `systemMessage` are each read by the party named above. No other field is added.
- **Existing readers of member stdout.** Every `PreToolUse` member already writes the advise envelope on some path, so no reader meets a new shape. The `SubagentStop` member's stdout was contracted empty. That contract is the delegation-kit sentence this delta rewrites, and the member's tests (`native/src/hook/stop_liveness.rs`) assert stdout only through `fire`, which the decline path never reaches.
- **The front end's envelope (delta 2).**
  - Producer: either stub's unavailable branch, on `--hook` with no runnable binary. A tree that vendored the kits and has no binary for its platform reaches it, which is criterion 5's omit-and-declare branch.
  - Consumer: the operator, through the harness's user-facing warning. It has one field, `systemMessage`, and the harness reads it.
  - Reader of the stubs' stdout: `--run-front-end-parity`, whose two absent-binary fail-open cases are updated.
  - `check-front-end-fail-open` reads the `FAIL_OPEN_ARMS` declaration line, which does not move.
- **Point 5.** No corpus narrows.
- **Point 6.** Two obliged sets. The decline's callers, measured above as three sites in one member, each pass their payload. The fail-open front ends are the two stubs, and each writes the same literal.

## Existing sections updated

The roster comes from `git grep -n "hook::decline\|fn decline\|no hook JSON\|declines loudly\|declines with a diagnostic\|loud on both settings\|writes nothing on stdout along this path\|binary absent, leading" -- '*.rs' '*.md' '*.sh' '*.ps1'`, run 2026-09-24.

- `gate-sdk/SPEC.md` §The harness-integration arm (deltas 1 and 2).
- `gate-sdk/SPEC.md` §run-gates (delta 2).
- `gate-sdk/bin/run-gates.sh` and `gate-sdk/bin/run-gates.ps1`, the unavailable branch (delta 2).
- `native/src/emit/front_end_parity.rs`, the two absent-binary fail-open cases (delta 2).
- `native/src/hook/mod.rs`, `decline` and its unit test (delta 1).
- `native/src/hook/stop_liveness.rs`, the three call sites and the module header's "emits no hook JSON" clause (delta 1).
- `delegation-kit/SPEC.md` §The turn-end liveness hook (delta 1).
- `native/src/runner.rs`, the `--hook` usage text's "declines with a diagnostic on stderr" clause. It becomes "declines through a hook envelope rather than blocking every guarded tool call" (deltas 1 and 2).
- `.workflow/release-declarations.md` (deltas 1 and 2). Under Behavior changes: a hook member that cannot resolve its own knob now tells the session model on `PreToolUse`, and the operator on other events. A tree with no runnable gate binary now tells the operator that its hook guards are off. Before, both wrote only to the harness's debug log.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md` and `docs/delegation-kit/SPEC.md`.

## Retired spellings

- None — the deltas rewrite a channel's prose and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the member's decline envelope and the front end's.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** The SPEC edit re-phrases the absent-binary paragraph's decline sentences, and the channel facts move with it.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `hook-decline-model-invisible` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
