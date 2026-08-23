# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

































## 2026-08-23 scope — how many Deferred cost fields open with a token the --icebox-candidates cost filter recognizes (low/zero/bounded/cosmetic)?
- corpus: TASK-QUEUE.md ## Deferred, every 'Cost while deferred:' line
- oracle: grep -oE 'Cost while deferred:.. [a-z]+' TASK-QUEUE.md, take the trailing word, sort | uniq -c | sort -rn
- rev: 87b6436f0330bf192b4311d2b3c68e2af36487c1
- finding: 238 cost lines; recognized openers total about 40 (low 35, zero 5), the rest free prose (the 44, every 35, a 32, paid 12, one 8, an 8, each 6, silent 5, small 4, compounding 4, ...). So roughly 83% of the pool is undecidable by the cost filter, confirming the 2026-08-23 gap bullet.

## 2026-08-23 scope — what is the port's non-gate shell corpus, and does the port oracle count it?
- corpus: every non-test .sh in the tree, and scripts/gates.list
- oracle: find . -name '*.sh' outside .git/ and native/target/, minus tests/smoke/fixtures paths, piped to wc -l; then bash gate-sdk/bin/port-blockers.sh --group trailer
- rev: 87b6436f0330bf192b4311d2b3c68e2af36487c1
- finding: 13990 non-test shell lines in the tree. The oracle scans 106 gates.list members and reads '6 still owed, 6 takeable'. It counts NEITHER the two kit-shipped unregistered shell gates (canon-kit/checks/check-surface-duplication.sh, evidence-kit/checks/check-producer-liveness.sh, both present, neither in gates.list) NOR any non-gate shell. So the owed count is a gate-corpus count, not the PRIORITY DIRECTIVE's corpus.
