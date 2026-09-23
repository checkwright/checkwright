# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-24 scope — Which deferred entries lead and fill the next unit set, under the rank tiers and the enhancement admission filter?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: c3b7754ac4fb6e28288f602841e5b76e3bf7aeab
- finding: 40 deferred entries, no session class; iteration class is heterogeneous-agent-delegation (filter-excluded 2026-09-19) and subagent-stop-phantom-firing, which leads. No entry reaches LIFECYCLE_KIT_RECURRENCE_THRESHOLD 2. Lead surface delegation-kit has no other admissible row (credential-swap demand-gated, session-model-identity and consult-tier no arm, consult-tier blocked). queue-flow mean-filed 4.6, so fill continues with the widest surface, guard-kit (7 rows), whose admissible hub is shell-guard-native-shell (adopter-constraint work, operator direction 2026-09-23). queue-edges: no retired-block row cites a guard-kit or delegation-kit candidate as disposed; guard-rule-number pair cites each other; shell-guard-native-shell cites guard-powershell-tool-unguarded.
- inferred: each filter verdict below is a judgment against TRAJECTORY.md's three arms, not a command's output; shell-guard-native-shell's slice count is reasoned from the 27-rule, 2381-line lib/guard.sh, not measured

## 2026-09-24 scope — Which deferred or iceboxed entries does shell-guard-native-shell moot or reshape?
- corpus: TASK-QUEUE.md guard-kit
- oracle: grep -nE 'bash-guard|guard\.sh|PowerShell|Git Bash|consumer rule|SubagentStop|liveness hook|scratch-run|rule 23' TASK-QUEUE.md
- rev: c3b7754ac4fb6e28288f602841e5b76e3bf7aeab
- finding: Icebox: no hit. Mooted: guard-powershell-tool-unguarded (its own DISTINCT paragraph predicts it). Reshaped, shell-guard first: guard-rule23-worktree-scratch (rule 23's match is re-implemented), guard-declares-class-correspondence-ungated (the guard_skeleton call sites move), guard-rule-number-intra-kit-citations-ungated and guard-rule-number-not-citable-outside-kit (the 111-citation corpus sits in SPEC, lib/guard.sh and the test tables the port rewrites), side-effect-free-read-arms (its steering rules land in the new rule model). Independent: settings-hook-command-path-gate, foreign-toolchain-docker-legs (though a PowerShell reader raises its cost), install-hosted-one-liner. guard-kit/SPEC.md §The hook on native Windows refuses both a PowerShell twin and a native hook front, and §The guard framework declares lib/guard.sh permanently shell on the consumer-rule seam; the direction reverses both.
- inferred: the reshaped verdicts assume the design moves the ruleset off lib/guard.sh, which the entry leaves open (binary or library)

## 2026-09-24 spec — Which tracked surfaces read, spawn or cite guard-kit's shell guard (lib/guard.sh, templates/bash-guard.sh, GUARD_KIT_LIB, --guard-json, --guard-lib-parity, run-guard-tests, the guard_* primitives) and guard-kit rule numbers?
- corpus: .
- oracle: git grep -n -E 'lib/guard\.sh|bash-guard|GUARD_KIT_LIB|guard-json|guard-lib-parity|run-guard-tests|guard_rule_|[Rr]ules? [0-9]+'
- rev: e3c8a407a19d5b54b6059d8fcb6b5e8c5db18ea4
- finding: Code readers: native/src/emit/run_guard_tests.rs (spawns the bash template), main.rs (--guard-json, --guard-lib-parity), guard.rs (five twins + json_arm; scan_prompts and compare_settings_allow call the twins), gates/guard_registration.rs (+ .gate couples, fixtures, .test.sh), hook/mod.rs HOOKS table, emit/overhead_meter.rs bash-guard literal, installer/consumer-smoke/run-smoke.sh, gate-sdk/gate-tests/run-gates-linked-worktree.test.sh, guard-kit/smoke/install.sh, .claude/settings.json and guard-kit/templates/settings-hooks.json wiring. init vendors kits generically and seeds no guard copy. Intra-kit rule-number tokens ~525 (SPEC 325, lib 109, cases.tsv 67). Out-of-kit guard rule-number cites: delegation-kit/SPEC.md (~25, rules 13-15, 19, 27), agent-execution.md, lifecycle-kit and evidence-kit SPECs one each, native/src emit comments, scripts/guard-config.knobs, TASK-QUEUE.md; the qualified form 'guard(-kit) rule N' has no false positive; bare 'rule N' outside the kit also hits doctrine and canon-kit rules. The spelling rule backtick-name form has no current use tree-wide.
- inferred: each hit's reader-vs-prose-vs-record classification is a reading judgment; which out-of-kit bare 'rule N' hits cite guard-kit (vs doctrine or canon-kit rules) is judged from context

## 2026-09-24 align — Which non-guard-kit tracked surfaces cite a guard rule by number, and how many rule-N tokens do they carry (delta 1's out-of-kit sweep count)?
- corpus: delegation-kit/SPEC.md, delegation-kit/templates/agent-execution.md, lifecycle-kit/SPEC.md, evidence-kit/SPEC.md, native/src/emit/scan_prompts.rs, native/src/emit/wait_probe.rs, native/src/knobs/guard_kit.rs, native/src/main.rs, scripts/guard-config.knobs, TASK-QUEUE.md
- oracle: git grep -o -E '[Rr]ules? [0-9]+' -- <each file>, counted per file and summed
- rev: 6cf58b78e90010a5e3b6fdde85a48ad734a02469
- finding: delegation-kit/SPEC.md 35, agent-execution.md 1, lifecycle-kit/SPEC.md 1, evidence-kit/SPEC.md 1, scan_prompts.rs 4, wait_probe.rs 2, guard_kit.rs 1, main.rs 1, guard-config.knobs 1, TASK-QUEUE.md 7 (its two icebox one-liners plus live-entry citations) = 54 total out-of-kit tokens, not the ~40 SPEC-shell-guard.md originally estimated at authoring; corrected in the amendment at align (2026-09-24)
- inferred: none
