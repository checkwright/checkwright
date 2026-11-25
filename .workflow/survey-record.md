# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-11-25 scope — Which orchestrator-pairing asks (A-E, exit codes) does the tree already serve, and what does each remaining one need?
- corpus: gate-sdk/SPEC.md evidence-kit/SPEC.md native/src/sarif.rs native/src/provenance.rs native/src/emit TASK-QUEUE.md
- oracle: grep -c -i fingerprint native/src/sarif.rs; grep -rl -- '"--json"' native/src; grep -n -i 'tree\|digest' native/src/provenance.rs; grep -n 'Exit codes' gate-sdk/SPEC.md; bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 29fe7a960274416f4714628e2a4c298564868a1e
- finding: SARIF log carries verdict, commit+worktree, kit versions, member rule ids, finding-line locations; missing tree id, gate-set digest, per-result fingerprint, sub-rule id, error kind (verdict-identity-fields, verdict-lookup). --json only on --verify-gate-run (orchestrator-arm-json). Exit 0/1/2; hooks block on 2 alone (exit-code-tool-error). D: text and ghpin partly covered, seven new (adopter-content-checks). ESOM reopen names no gate. No candidate carries an inbound edge; no recurrence-threshold entry; no session- or iteration-class row.
- inferred: the adopter's out-of-tree asks were read as the lead relayed them, not re-asked; 'script' and E's 'four peer-channel defects' are unresolved by any tracked surface

## 2026-11-25 spec — Which tracked readers branch on, relay or state the gate binary's exit status 2, so that a new tool-error status 3 changes their behavior or falsifies their text?
- corpus: native/src gate-sdk/bin gate-sdk/lib gate-sdk/templates .github/workflows .claude lifecycle-kit/templates gate-sdk/SPEC.md evidence-kit/SPEC.md delegation-kit/SPEC.md installer/SPEC.md
- oracle: git grep -nE 'Some[(]2[)]|== *2[^0-9]|!= *2[^0-9]' -- native/src; git grep -nE 'ARM_UNAVAILABLE_STATUS|-eq 2|LASTEXITCODE|exit 2' -- gate-sdk/bin gate-sdk/lib .github .claude; git grep -cE 'return 2;|exit[(]2[)]' -- native/src
- rev: 5c4f2fba1ff8c3c97a6c1cdd0e264de1c49298d1
- finding: Edit owed: main.rs Arm::Emit collapse; runner.rs dispatch-harness tails and help text; run_gate_tests rc==2 HARNESS; run_consumer_smoke rc_s==2 surface-absent; stop_liveness reads producer-liveness 2 as refuse and any other exit allows; model_verdict UNKNOWN; run_validate guard; run-gates.sh and run-gates.ps1 unavailable status; lib/gate.sh fail_closed; check-skeleton.sh; stage-session.md and consult.md model-verdict branch; close.md roadmap branch; gates.yml asserts. Unchanged: runner any-nonzero FAIL, hook-member readers, enter_stage and git_hook relay, run_config_variant. Producers about 450 literal 2 plus about 1064 emit Err sites.
- inferred: kit smoke and gate-test assertions of 2 counted (46 files, 114 lines) not each read; installer/SPEC.md refusal statements sampled
