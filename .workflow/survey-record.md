# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-24 scope — Which deferred entries rank into this undirected iteration's unit set under the enhancement admission filter?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 5df489d3168185459aac7539e003eb3ed28a955c
- finding: No session/iteration-class row passes the filter (heterogeneous-agent-delegation excluded 2026-09-19); no roadmap row passes it. Lead: settings-hook-command-path-gate (event/high, 1 recurrence, 2 inbound: enforcement-map-member-owner, unstamped-session-tree-edit), bundling both carried gap bullets (hook::decline envelope, workflow run: path liveness) and enforcement-map-member-owner; lifecycle-kit fill: split-posture-waiver-writer, unstamped-session-tree-edit. Filter-held or blocked: config-variant-battery-harness, foreign-toolchain-docker-legs, gate-authoring-sdk-surface, gate-tamper-exemption-reader-substrate, companion-toolkit-profile, local-only-files-write-back-untriggered, consult-tier-declaration.
- inferred: none

## 2026-09-24 spec — Which harness hook channels reach the model or the operator without changing a hook's verdict, per event?
- corpus: https://code.claude.com/docs/en/hooks.md (the harness hooks reference)
- oracle: curl -sL https://code.claude.com/docs/en/hooks.md | grep -n 'exits 0 goes to the debug log\|additionalContext\|systemMessage'
- rev: ede96ac28432fff101035ef0d71f8ec6e24eefd3
- finding: Exit-0 stderr goes to the debug log only, seen by neither model nor operator (section Exit code 0). PreToolUse hookSpecificOutput.additionalContext reaches the model and sets no permission decision. On Stop/SubagentStop, additionalContext keeps the subagent running and exit 2 or decision block refuse the stop, so no model channel leaves the verdict unchanged there. systemMessage is a universal field shown to the user. hookEventName must match the firing event or the event-specific fields are ignored. SubagentStop exit 2 does block.
- inferred: none
