# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-23 scope — How does the deferred pool rank for an undirected iteration, and which rows can land now?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: f70d1494038dedb34dc78b8f0763162c4ac90978
- finding: 17 deferred rows, none session- or iteration-class, none at the recurrence threshold (max 1 of 2). Roadmap now rows (companion-toolkit-profile, design-partner-preview) gate on a published tag and an observed install, neither tree work; v0.31.0 is the last tag, 1663 commits back. Release-gated: tarball-attestation-observed, front-door-job-observed, codex-plugin-version-pin. Upstream-gated: codex-plugin-hook-discovery (codex-cli 0.163.0 on PATH, the version probed 2026-10-21). Not installed: cursor-muse-plugin-observed. Operator-present: notification-delivery-probe. Demand-gated: benchmark-ab-experiment, hosted-attestation-service, external-gate-quality-evidence. Landable features: antigravity-hook-binding (agy 1.3.4 on PATH), agent-selection-probe, manifest-count-blank-run-bound, evaluator-root-agent-file, record-stamp-encoding-compression; awaiting a ruling: smoke-whole-tree-precondition-unscoped, verify-workflow-decoupling. Inbound edges: design-partner-preview 3, benchmark-ab-experiment 2, agent-selection-probe 1 (from evaluator-root-agent-file).
- inferred: landability of each event-gated row beyond the probes named here; whether Antigravity 1.3.4 runs plugin hooks at all (plugin/SPEC.md reads its protocol off the CLI docs)

## 2026-10-23 spec — Does agy 1.3.4 run a plugin's hooks, and with what payload, decision and failure semantics?
- corpus: SPEC-antigravity-binding.md
- oracle: four headless agy -p sessions against a workspace-local plugin whose PreToolUse and SessionStart hooks log stdin, cwd and env and answer deny, {}, empty or exit 2; agy plugin validate and install under a scratch HOME
- rev: a0888da7ab862e73d4e7ba5dbd314f9a4e9aab30
- finding: Hooks fire with cwd and PWD the plugin dir and no project-dir variable. The payload names no event and carries workspacePaths, conversationId, transcriptPath and toolCall name and args; run_command carries args.CommandLine, write_file and edit_file args.Path. Empty stdout is no decision, {} or any undecided object denies, a deny reason reaches the model, a non-zero exit fails the call. Install copies a root hooks.json and never loads hooks/hooks.json. The amendment's preamble records each run.
- inferred: an installed copy's hooks fire as the workspace copy's do; native Windows runs the command under cmd /c with Go's default escaping
