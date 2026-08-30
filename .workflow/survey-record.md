# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.















































## 2026-08-30 scope — Which of the 102 owed port-blocker files are engineering ports and which are undecided dispositions?
- corpus: ':(top)' — every tracked non-test .sh the --tree arm scans
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 3b35e2c009966e627d6b3ca4b40c0a1559f62bb8
- edges: platform-support-ci-matrix 9, native-gate-port-remaining-corpus 8, prose-filename-citation-liveness 6, powershell-installer-surface 5, then a tail at 3
- finding: 102 owed of 139 scanned. 29 of the 102 (28 percent of the completion predicate) are DISPOSITION questions, not engineering: 18 kit templates/*.sh, 6 scripts/ installed copies of those same templates, 5 remaining scripts/*-config.sh. Both halves are already half-declared, which is what makes the class visible: drift-kit's two templates carry no-port while the other 18 do not, and 6 of 11 scripts/ configs carry the 2026-08-24 no-port cause while 5 of the same class do not. The queue's kit-config-template-port-disposition (10 files) and harness-template-port-disposition (17 files) together name 27 of the 29; the two my census adds are scripts/drift-config.sh and scripts/enum-sets.sh, which no entry reaches. The engineering remainder is dominated by lib and bin: guard.sh 1243, run-smoke.sh 851, gate.sh 709, enter-stage.sh 617, spec.sh 568, stage-economics.sh 465, init.sh 416, run-gates.sh 328. Smoke class probed clean: every smoke/install.sh and smoke/violation.sh that exists now carries no-port under the 2026-08-30 ruling; the one residue is context-kit/smoke/agents-md.sh, a third smoke member the ruling's letter does not reach, so widening to it is operator-class.
