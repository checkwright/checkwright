# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.




























## 2026-08-20 scope — What does a check-graph port actually touch, and what else in the deferred pool shares that surface closely enough to ride along?
- corpus: gate-sdk/checks/check-graph.sh (632 lines), gate-sdk/bin/gen-pre-commit.sh (297), gate-sdk/SPEC.md sections check-graph + the fifth/sixth budget batch + the array-knob config bridge, scripts/graph-vocab.sh, and all 235 entries of TASK-QUEUE.md's Deferred section
- oracle: read-only audit-sweep in a worktree; findings re-probed first-hand at three load-bearing points (grep -n 'declare -F|graph_surface_layer|graph_theme_css|GATE_SDK_GRAPH_THEME' over check-graph.sh; grep -n 929 over gate-sdk/SPEC.md; bash gate-sdk/bin/port-blockers.sh --group)
- rev: d40b51581dcc44137584f86a3c813de14198d6de
- finding: The port is 929 shell lines, not the 632 the oracle prints: assertions D and E spawn gen-pre-commit.sh and byte-compare its two emissions, so the generator ports too (gate-sdk/SPEC.md:8281-8287, spawn-invisibility). A previously unscoped blocker: check-graph's consumer config crosses EXECUTABLE BASH FUNCTIONS, not values -- graph_surface_layer from graph-vocab.sh and graph_theme_css/header/footer from GATE_SDK_GRAPH_THEME, dispatched by declare -F at check-graph.sh:37,78,82,87 -- while the array-knob config bridge crosses only arrays, scalars and keyed lists into the binary's argv. So the port must either design a data-only theme contract (a breaking change to the graph-vocab seam CLAUDE.md names as the provenance doctrine's worked example) or keep a shell shim for theme emission. check-graph is the ONLY file in the tree using the declare -F sourced-hook pattern. True shared-surface riders: couples-glob-semantics-unowned (also a blocker -- check-graph assertion B reads couples= a third way, exact-token subset against trigger=, invoking no glob matcher), upgrade-smoke-graph-artifact-literal, install-step-relocation, lint-scope-hook-trigger, docs-corpus-derivation-manifest-divergence, in-crate-module-coupling-derivation, knob-shape-flip-undetected, gate-test-in-tree-invoker-ruling. NOT riders despite looking adjacent: spec-lib-dead-derivation, queue-lib-dead-derivation, born-native-omission-accumulation, born-native-flip-enforcement-gate, gate-binary-target-roster-widening, cohort-held-members-port-prerequisites, projection-trigger-witness.

## 2026-08-20 scope — Do the four theme-held threshold recurrences compose into a coherent session-mechanics iteration, and what surrounds them?
- corpus: TASK-QUEUE.md Deferred section, all 235 entries, read in full body rather than by lead line, against the guard-kit surface (guard-kit/, scripts/bash-guard.sh, guard-kit/bin/scratch-run.sh, the harness settings allowlist, GUARD_KIT_RO_BINS) and the delegation-kit surface (delegation-kit/, templates/agent-execution.md, resume journals, dispatch provenance, wait primitives, .run liveness records, worktree reclamation)
- oracle: read-only audit-sweep in a worktree; the routing clause re-read first-hand at TASK-QUEUE.md:6687-6689 and the recurrence census re-derived by anchored grep over the deferred section
- rev: d40b51581dcc44137584f86a3c813de14198d6de
- finding: 45 entries mention one of the two surfaces; 26 land their deliverable there, and they form TWO clusters that do not share a setup cost. Cluster A, dispatch/provenance/liveness integrity: turn-end-chokepoint-and-wait-primitive, session-mechanic-grants-uncommitted, delegation-provenance-floor, launch-chokepoint-liveness-record-write, subagent-stop-liveness-hook-wiring, dispatch-cited-evidence-unverified, dispatch-unreadable-target-fallback, agent-worktree-reclamation-unenforced, agent-worktree-boundary-disposition, stage-completion-unattested, stage-journal-contract-unoracled, self-revert-reminder-expectation. These are riders on each other in the strong sense: several were literally SPLIT off one another by check-queue-entry-budget headroom, and they share the .run record format and agent-execution.md's obligations. Cluster B, guard-kit prompt-friction and allowlist hygiene: guard-command-prefix-wrapper, guard-steer-grant-mismatch, guard-steer-names-absent-tool, guard-read-steer-tool-coverage, path-pinned-allow-entry-oracle, scratch-run-steer-rule, ro-bins-write-option-bypass, consumer-guard-rule-coverage, consumer-guard-rule-verification-lane, guard-ruleset-registration-lockstep, settings-allow-intended-breadth-declaration, guard-grant-review, expected-permission-mode-undeclared. Only ONE blocked-by tag in the whole set: guard-grant-review on settings-allow-intended-breadth-declaration, both Cluster B. Four entries are blocked on an operator ruling rather than on engineering: settings-allow-intended-breadth-declaration, guard-grant-review, subagent-stop-liveness-hook-wiring (a harness settings write no agent message authorizes), crate-toolchain-grant-uncommitted. scratch-citation-skill-surface-reach is a NAME-COLLISION false positive: its scratch is check-scratch-citation, not scratch-run.sh. The threshold-recurrence routing clause, verbatim at TASK-QUEUE.md:6687-6689: a third threshold recurrence routes to the operator, not to a third decline; two is where lead discretion ends.
