# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-16 scope — Which deferred entries lead this iteration's unit set, by deferral cost then roadmap, with their inbound edges summed?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 645359233c5547bb451938c5c056b228919bc330
- finding: Deferred and Icebox read; intake lanes empty. No session-class row; iteration-class rows are crate-test-unremaps-binary (gate-sdk, tree-workable) and heterogeneous-agent-delegation (delegation-kit, remainder grant-bound live runs). No entry reaches the recurrence threshold (record-stamp-encoding-compression holds 1). Live inbound edges peak at 2, none on a gate-sdk row. Tree-workable roadmap row: custom-rust-gates (next/ecosystem, gate-sdk); companion-toolkit-profile and design-partner-preview stay gated on a tag and an observed install. gate-sdk carries the most board rows (5): crate-test-unremaps-binary, custom-rust-gates, windows-shell-gate-dispatch, exception-list-reader-split, case-pin-outranked-by-env. windows-shell-gate-dispatch is reshaped by custom-rust-gates (both edit registry::invocation's consumer-member start on native Windows); the rest independent.
- inferred: none

## 2026-10-16 spec — Which crate sites read the exec target class or a shell member's start, and how does each answer an extensionless executable target and a Windows bash start?
- corpus: native/src native/tests gate-sdk/lib
- oracle: git grep -nE 'Substrate::(Exec|Shell|Compiled)|exec_target|exec_field|is_powershell_body|EXEC_NEEDS' -- native/src native/tests gate-sdk/lib
- rev: a0f8f3699270f6a7d8e186a4608933750790a15b
- finding: Class readers that move: registry::invocation (order and arm), gate_substrate_parity K (class, tracked, clean segment), port_blockers EXEC_NEEDS, gate_assertions marker read. Unaffected: psscriptanalyzer, fixture_coverage, gate_output, gate_tamper, reads_couples, binary_fresh. Several readers match Substrate non-exhaustively, so a fourth variant would route silently. The shell start reaches two native spawners: proc::dispatch via invocation (raw Command::new, no System32 rejection) and run_gate_tests via lib/gate.sh gate_command's one-element .sh argv. No test or workflow dispatches a .sh member on Windows. Landed in gate-sdk/SPEC-member-start.md.
- inferred: none

## 2026-10-16 align — Which tracked surfaces still state an exec target as PowerShell-only or an adopter as unable to author a compiled gate, beyond the member-start roster as spec filed it?
- corpus: gate-sdk/SPEC.md docs gate-sdk/README.md native/src native/tests
- oracle: git grep -n -E 'compiled gate|exec descriptor|exec member|# exec:' -- . ':!docs/gate-sdk/SPEC.md'
- rev: 23ac737f48be180c23b88a820b978b23571823a4
- finding: Beyond spec's roster: SPEC.md check-shellcheck paragraph 'The gate ships to adopters' (bold-split phrase missed the -F probe) and the conservation row's family clause, the graph-manifest K sentence, the field-refusal list, the gate_command bullet and the port-blockers compiled-member paragraph. Landed in gate-sdk/SPEC-member-start.md roster; also three ordinal sentence anchors rewritten to opening words, and the runner's shell-start key set to the .sh extension since registry::substrate answers shell for every non-.gate path.
- inferred: none
