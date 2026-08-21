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

## 2026-08-21 align — Do SPEC-graph-port.md and SPEC-stamp-head.md hold true against the tree, and does no sibling surface carry a claim either amendment corrects?
- corpus: gate-sdk/SPEC-graph-port.md (10 deltas), lifecycle-kit/SPEC-stamp-head.md (6 deltas), gate-sdk/checks/check-graph.sh, gate-sdk/gate-tests/check-graph-{refs,cap,theme}.test.sh, scripts/graph-vocab.sh, native/src/gates/stage_evidence.rs, lifecycle-kit/bin/enter-stage.sh, lifecycle-kit/gate-tests/rename-iteration.test.sh, gate-sdk/SPEC.md sections check-graph and The port-candidate criteria, TASK-QUEUE.md's three promoted entries, .workflow/gap-inbox.md, TRAJECTORY.md
- oracle: re-measured bash gate-sdk/checks/check-graph.sh and its two gen-pre-commit.sh spawns directly; grep -rn for each literal/negative-existence claim (929, declare -F, the four-field grammar sentence, the retired-seam symbol set) across the whole tree; read every named fixture (good/bad args, the three hermetic test drivers) byte-for-byte; diffed check-graph.sh's assertion B against delta 6's stated four branches line by line
- rev: 9d82a7ab37795a631d87ce1305ccb3ff68c526b8
- finding: Two amendments, three real defects, all inside SPEC-graph-port.md and all fixed in place: (1) delta 8 claimed assertions F/H/I already have hermetic drivers naming two files that cover only H and I, assertion F (check-graph.sh:600-605) has none; (2) H's own driver check-graph-refs.test.sh injects its fixture through the retired GATE_SDK_GRAPH_THEME seam and would trip delta 2's tripwire post-port instead of testing anything, unnoted by the widening plan; (3) 'no other file in the tree uses the pattern' (declare -F) is an unbounded negative-existence claim two other files disprove (gate-sdk/bin/port-blockers.sh, delegation-kit/lib/delegation.sh), and gate-sdk/SPEC.md already names declare -F as this tree's general convention. Every other checked claim in both amendments held: the 10/6 delta counts, assertion B's four branches verbatim, the 7-prefix graph-vocab migration, the runtime measurement (re-measured 7582/5666/207ms against claimed 7591/5685/213ms), both gap-inbox filings (graph-port-bash-spawn-residue, config-bridge-resolution-cost), the fork-refusal/feature reclassification and its explicit (not implied) composition with stage-completion-unattested in SPEC-stamp-head.md's own text, criterion 4's register gaining check-graph as the SECOND no-clearing-configuration member after check-gate-exemption-tasks, and the Existing-sections-updated roster's completeness against a tree-wide grep for the four-field grammar sentence. SPEC-stamp-head.md needed no edit.
