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

## 2026-08-23 close — Which --icebox-candidates rows are actually icebox-eligible under queue-kit/SPEC.md's three conjuncts, and which conjunct holds each survivor?
- corpus: TASK-QUEUE.md ## Deferred, the 27 rows the arm returned at the widened seven-day age floor
- oracle: extract each candidate's extent (lead line to the next same-indent bullet); flag it if the extent holds a [roadmap: tag, a recurrence: line, or ANY live slug matched as a bare word against the set of every '- **<slug>**' lead line under New Features / Technical Debt / Deferred / Icebox — matching bare words, not only backticked ones, since a citation in plain prose is a trigger too
- rev: b5fd0e587a2fbdc9d5e22cf87bb711749d9af791
- finding: 27 candidates, 19 evictable and 8 held. The 8 split 3 / 5 by conjunct: three by a live [roadmap:] tag (plugin-marketplace, benchmark-ab-experiment, hosted-attestation-service) which the arm should have filtered and does not; five by naming a live unbuilt slug — build-stage-tier-economics, gate-tamper-exemption-reader-substrate, absorbed-duplicate-disposition, baseline-row-prose-coupling-gate, tracked-to-untracked-pointer-scope. Zero were held by a recurrence: line. TWO READINGS WORTH THE CARRY. First, the bare-word match matters: a backtick-only regex missed nothing here but would have, because entries cite siblings in plain prose as often as in code spans — run it bare-word or the eviction is unsound. Second, one of the five (baseline-row-prose-coupling-gate) is held by a slug that is itself ICEBOXED (gate-spec-claim-assertion-parity), which is correct under the tier's own rule that an iceboxed slug is live and unbuilt and a legal blocker target — so an eviction sweep cannot treat the icebox as terminal, and the trigger graph reaches into it.

## 2026-08-23 close — Does every <kit>/{checks,bin,lib}/*.sh path literal appearing in governed prose resolve on disk?
- corpus: every tracked *.md that is governed prose, excluding the generated docs/<kit>/ mirrors
- oracle: regex each <kit-or-root>/(checks|bin|lib)/*.sh literal out of the corpus, unique them, and existence-test each one — the corpus-wide form the capability-pendency-after-landing roster row mandates, NEVER a grep restricted to the current iteration's deletion list, which is blind to residue by construction
- rev: b5fd0e587a2fbdc9d5e22cf87bb711749d9af791
- finding: 74 unique path literals, 21 non-resolving, and ALL 21 CLEARED on inspection — the probe is genuinely dry rather than unarmed. The 21 fall into six exempt shapes, and the shape list is the reusable half: dated docs/posts/ entries narrating a past port's before/after mapping (past-state records by design); TASK-QUEUE.md prose correctly past-tense about a deleted script; two entries that log WRONG paths a session guessed as measured defect evidence (allowlist-path-existence-unchecked, kit-bin-entry-point-unrostered) where nonexistence is the point; x.sh-style illustrative placeholders; one deliberately-broken gate-test fixture that is the intended-red case for check-kit-ref-liveness; and one design-pending future deliverable correctly framed as unbuilt. THE CARRY: a later sweep should not re-derive those six exemptions from scratch — they are stable categories, and a naive existence test reports 21 findings where there are zero. Note also that this run confirms the 2026-08-13 residue the seventh sweep found (which six consecutive sweeps had missed) is now cleaned, so the corpus-wide form has been run to completion at least twice.
