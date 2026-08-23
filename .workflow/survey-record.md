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

## 2026-08-23 validate — Which bespoke gate-tests/*.test.sh sandbox by cwd-relative .tmp/.workflow, and which of those pin the path knobs themselves rather than relying on the ambient default staying relative?
- corpus: every */gate-tests/*.test.sh in the tree
- oracle: grep -ln 'mkdir -p .*\.workflow\|mkdir -p .*\.tmp\|/\.tmp\|/\.workflow' */gate-tests/*.test.sh  vs  grep -l 'GATE_SDK_TMP_DIR=\|GATE_SDK_WORKFLOW_DIR=' */gate-tests/*.test.sh — the set difference is the exposed cohort
- rev: 5849144ddcef50bef28ec14df4db3f400bcf4e91
- finding: 16 bespoke tests build cwd-relative .tmp/.workflow sandboxes; 11 pin GATE_SDK_TMP_DIR or GATE_SDK_WORKFLOW_DIR explicitly, so pinning is already the majority idiom. The 7 that do not — canon-kit/check-comment-tier, evidence-kit/producer-lock, and lifecycle-kit's check-stage-entry, check-merge-attrs, check-survey-record, check-stage-evidence, check-close-surfaces — are isolated only by the ambient value of those knobs happening to be relative. lib/test-hermetic.sh pins <KIT>_CONFIG_FILE and nothing else, so it does not cover this. Probed, not inferred: with GATE_SDK_TMP_DIR absolute and a live lock at that path, producer-lock.test.sh reds on 4 assertions; with GATE_SDK_WORKFLOW_DIR absolute it reads a foreign manifest and reds on 4.
