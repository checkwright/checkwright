# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-19 scope — Which deferred entries compose the next iteration's unit set under an undirected directive and the enhancement admission filter?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 7afb14412ac42703e93100e5d14b39374c2050ae
- finding: board of 94 deferred lead lines carries no session-class row; heterogeneous-agent-delegation is the only iteration-class row and an enhancement failing all three admission arms; no recurrence declaration carries two dates; docs/install.md's unconditional floor is bash and git, starter and prose profiles vendor only native-gate kits, so rung 4a floor-bash-hooks-front-end is the rung that takes bash off their floor (4b floor-jq-guard-lib touches only the already-conditional @guard-kit member); fail-open-arm-status-second-source (run-gates.sh ARM_UNAVAILABLE_STATUS) and kit-token-anchor-hook-for-divergence (hook emitter anchor vs --for) sit on the front-end and hook generator 4a redesigns; the substrate-parity pair shares check-gate-substrate-parity assertion F
- inferred: none — the board read, queue-flow, docs/install.md's toolchain roster and each proposed entry's cited source sites were run

## 2026-09-19 spec — Which kit_roots_rel and expand_couples callers match a repository-relative corpus, and which match kit-parent text?
- corpus: native/src
- oracle: git grep -n -e kit_roots_rel -e expand_couples -- native/src
- rev: 26192fcd1fceb6ccf2c8a0f86d30ff036a88d251
- finding: repository-class (K spelling supplied, the defect): runner.rs:747-754 --for, gates/reads_couples.rs:459,485, gates/core_files.rs:62, emit/port_blockers.rs:215, gates/gate_substrate_parity.rs:451; kit-parent-class (correct): gates/graph.rs:517-519 vocab, kit_enum.rs:107, kit_registration.rs, knob_default_coupling.rs:23, knob_citation.rs:22, docs_cmd.rs; display: emit/graph.rs:138,177, port_blockers.rs:612; git_hooks.rs:105-106 already toplevel-anchored; non-couples fs readers with the same latent mismatch: enum_sets.rs:79, close_surfaces.rs:147, gate_binary_fresh.rs:40, install_platforms.rs:206, knob_default_coupling.rs:360, pack_installer.rs:196,295, kit_enum.rs:123, knobs/evidence_kit.rs:44-49; no test nests GATE_SDK_ROOT
- inferred: none

## 2026-09-19 spec — Does check-gate-substrate-parity assertion F read .github/workflows/gates.yml clean if the knob names it?
- corpus: .github/workflows/gates.yml native/src/gates/gate_substrate_parity.rs
- oracle: GATE_SDK_NATIVE_PUBLISH_WORKFLOW=<copy of gates.yml> native/target/release/checkwright-gates check-gate-substrate-parity
- rev: 26192fcd1fceb6ccf2c8a0f86d30ff036a88d251
- finding: no: red x4 'digest computed by a consumer' on install-smoke-windows, install-smoke-macos, install-smoke-macos-intel, install-smoke-linux-arm64, each a false positive of computes_digest's substring test matching sha256sum as a word in a toolchain-presence for loop (gates.yml:275,1303,1610,1797); matrix check passes (gates.yml:1081-1082 expression); native-artifacts computes 0 in text (hash in scripts/ci-build-artifact.sh via gates.yml:1155); install-smoke-powershell computes 1 (:592) and downloads nothing
- inferred: none

## 2026-09-19 spec — What does a jq read in guard-kit's hook cost against a gate-binary spawn?
- corpus: guard-kit/lib/guard.sh scripts/bash-guard.sh
- oracle: 50-call loops: jq -r .tool_input.command; checkwright-gates --version; checkwright-gates --hook agent-dispatch-guard <payload; bash scripts/bash-guard.sh <payload with a PATH jq shim counting spawns
- rev: 26192fcd1fceb6ccf2c8a0f86d30ff036a88d251
- finding: jq read 1.68 ms/call; binary --version 0.60 ms; binary --hook agent-dispatch-guard with stdin 0.82 ms; whole hook on an allowed ls 52.4 ms with 3 jq spawns (guard_read_command, one guard_input_field, guard_allow); the library already spawns the binary once per call for --emit-knob-values (guard.sh:2075)
- inferred: the per-call figures are one 14-core Linux host's, unrepeated elsewhere

## 2026-09-19 align — Do the five adopter-floor-native-rungs amendments self-check: do their line/site citations match the current tree, and does every '## Existing sections updated' entry name the delta it belongs to?
- corpus: SPEC-binary-door.md SPEC-guard-lib-native-reads.md SPEC-fail-open-set.md SPEC-couples-anchor.md SPEC-release-workflow-set.md
- oracle: bash gate-sdk/bin/run-gates.sh, plus git grep -n on each amendment's cited symbols/line numbers against the files they name
- rev: 9408f8f30f649385fb7a02852f4b758d3ef3aaca
- finding: battery green (118/118) both before and after; binary-door, fail-open-set and release-workflow-set citations (toolfloor.rs, gate.sh, guard.sh, run-gates.sh/.ps1, gates.yml lines, gate_substrate_parity.rs, init.rs) all matched the tree byte-for-byte; couples-anchor's main table (kit_roots/kit_roots_rel callers) matched, but its filed non-couples list cited gates/kit_enum.rs:123 and knobs/evidence_kit.rs:44-49, neither of which calls kit_roots_rel — the survey oracle (git grep kit_roots_rel/expand_couples) lands on kit_enum.rs:45 and evidence_kit.rs:50, corrected in-place; guard-lib-native-reads' point 6 named only one of three toolfloor.rs:240-246 owed_names assertions jq's contributor move touches (245 undecided, 246 not-owed were unstated), and one roster bullet (guard-kit/SPEC.md :2338-2342/:2722-2725) attributed a delta-1-and-3 compound change to delta 3 alone — both amended; both fixes committed as 9408f8f3
- inferred: which gates.yml Linux runners resolve /bin/sh to dash, and whether PowerShell 5.1/7 run a ./-prefixed binary path, are the amendments' own cannot-run-before-build markers, not re-verified here
