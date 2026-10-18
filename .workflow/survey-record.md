# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-18 scope — Which deferred entries lead an undirected unit set, by the scope ranking tiers
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit-queue-edges
- rev: c74ad706a671895d80cedce543d1f77255ee36b5
- finding: At c74ad706a the Deferred section holds 19 entries (15 event/low, 2 event/high, 2 once/low). No session/iteration-class row; no entry at the recurrence threshold of 2. Only actionable roadmap row is plugin-harness-reach (next/ecosystem); companion-toolkit-profile waits on a tag and an observed install, design-partner-preview on operator hours, benchmark-ab-experiment and hosted-attestation-service on demand. hook-contract-harness-neutral reshapes plugin-harness-reach (its guard wiring binds against the contract). grant-pwsh-word-read is the costliest actionable row (event/high), held behind the v0.32.0 bar by the 2026-10-14 split direction. drift-kit holds the most rows (4), three of them waiting on outside work.
- inferred: codex and agy skill loading not run, only their plugin subcommands seen in --help

## 2026-10-18 spec — Which kit prose lines spell the shipped hook binding (hook-contract sweep worklist)
- corpus: '*/SPEC.md' '*/README.md' README.md '*/templates/*.md'
- oracle: GATE_SDK_HARNESS_LITERALS='CLAUDE_PROJECT_DIR CLAUDE_CODE_SESSION_ID CLAUDE_CODE_CHILD_SESSION CLAUDE_CONFIG_DIR CLAUDE_PLUGIN_ROOT PreToolUse PostToolUse SessionStart SessionEnd SubagentStop UserPromptSubmit PreCompact hookSpecificOutput hookEventName additionalContext permissionDecision updatedInput systemMessage hook_event_name tool_input tool_name transcript_path stop_hook_active background_tasks agent_transcript_path last_assistant_message run_in_background subagent_type parentAgentId spawnDepth isFork tool_use' ./native/target/release/checkwright-gates --only check-harness-literal
- rev: c0fe5fbd8d8d879645e42f4244fcf52efc6db40a
- finding: 129 red lines: delegation-kit SPEC 57, guard-kit SPEC 39, lifecycle-kit SPEC 7, context-kit SPEC 7, gate-sdk SPEC 6, drift-kit SPEC 6, guard-kit README 2, delegation-kit README 2, lifecycle-kit README 1, evidence-kit SPEC 1, delegation-kit/templates/agent-execution.md 1; dispositions per gate-sdk/SPEC-hook-contract.md delta 3
- inferred: none

## 2026-10-18 align — Which section headings carry the 129 harness-literal red lines under delta 4's roster (build worklist by heading)
- corpus: '*/SPEC.md' '*/README.md' README.md '*/templates/*.md'
- oracle: check-harness-literal with delta 4's roster set in the environment, lines grouped by nearest preceding heading
- rev: 3d793261aa4a623a920e0b49edcb82ca5118f367
- finding: At 3bd711bc2 the 129 lines sit under: delegation-kit SPEC The delegation model 23, The turn-end liveness hook 22 (plus its Attribution 1, probe 1, background_tasks 2 subsections), Operative residency 2, Layout 3, Testing 1, model-verdict 1, usage.txt 1; guard-kit SPEC The rule roster 13, The shell guard 6, Testing 5, Layout 4, Consumer rules 3, intro 1, friction loop 1, generic ruleset 1, wakeup-guard 1, escalation-guard 1, reader and its views 1, rewrite security posture 1, Writing a consumer rule 1; lifecycle-kit SPEC session-id.sh 4; context-kit SPEC session-context hook 5; drift-kit SPEC stage-economics 4; gate-sdk SPEC harness-integration arm 4, run-gates 1, enforcement-map 1. The guard-kit hook-on-native-Windows section carries none.
- inferred: none
