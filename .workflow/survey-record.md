# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-11-25 scope — Which orchestrator-pairing asks (A-E, exit codes) does the tree already serve, and what does each remaining one need?
- corpus: gate-sdk/SPEC.md evidence-kit/SPEC.md native/src/sarif.rs native/src/provenance.rs native/src/emit TASK-QUEUE.md
- oracle: grep -c -i fingerprint native/src/sarif.rs; grep -rl -- '"--json"' native/src; grep -n -i 'tree\|digest' native/src/provenance.rs; grep -n 'Exit codes' gate-sdk/SPEC.md; bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 29fe7a960274416f4714628e2a4c298564868a1e
- finding: SARIF log carries verdict, commit+worktree, kit versions, member rule ids, finding-line locations; missing tree id, gate-set digest, per-result fingerprint, sub-rule id, error kind (verdict-identity-fields, verdict-lookup). --json only on --verify-gate-run (orchestrator-arm-json). Exit 0/1/2; hooks block on 2 alone (exit-code-tool-error). D: text and ghpin partly covered, seven new (adopter-content-checks). ESOM reopen names no gate. No candidate carries an inbound edge; no recurrence-threshold entry; no session- or iteration-class row.
- inferred: the adopter's out-of-tree asks were read as the lead relayed them, not re-asked; 'script' and E's 'four peer-channel defects' are unresolved by any tracked surface
