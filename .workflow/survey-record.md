# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-04 scope — Which canon-kit/SPEC.md sections carry the check-prose-bounds findings, and where does the next brevity slice cut?
- corpus: canon-kit/SPEC.md
- oracle: CANON_KIT_PROSE_BOUND_CEILING_FILE= bash gate-sdk/bin/run-gates.sh --only check-prose-bounds
- rev: c93ba5339865ab370e15e9db7ef45f14e34af399
- finding: 96 findings, all from §check-provenance-seam on (line 438): prose-enum 15, measured-claim 13, unmarked-claim 8, docs-cmd 8, surface-duplication 7, knob-citation 7, install-claim 7, tracking-claim 6, fence-run 6, provenance-seam 5, the rest 1 to 3 each. The unpassed remainder is about 25.9k of the file's 40.1k words. Recommended slice: §check-provenance-seam through §check-surface-duplication, 55 findings in about 9.5k words, the size of the last slice; §check-comment-tier on carries the other 41 in about 16.5k words. delegation-kit/SPEC.md reads 0, so the entry's surface moves to canon-kit.
- inferred: word counts by line span were summed with awk, not an oracle

## 2026-10-04 scope — Which deferred entries rank into this iteration's unit set, and does any carry aggregated inbound weight?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: c93ba5339865ab370e15e9db7ef45f14e34af399
- finding: No entry aggregates more than 3 inbound edges (design-partner-preview 3); none reaches the recurrence threshold of 2 dates. Tier one: spec-brevity-residue (session/high) leads, its next slice canon-kit; update-notice-probe-contract (session/low, installer) cannot join a canon-kit set without a second component, so it leads the next set; iteration/low installer-smoke-live-upstream, audit-trigger-mirror-component, heterogeneous-agent-delegation and manual-operation-spend-channel likewise. Canon-kit fill: manifest-finder-untracked-walk and spec-pointer-bare-section-mark, both canon-kit gates and both owing an amendment. Board rows: gate-sdk 7, installer 5, drift-kit and lifecycle-kit 4. Supersession: the brevity slice reshaped by manifest-finder-untracked-walk, whose per-member statements may land in sections it rewrites (§check-prose-enum, §check-knob-citation), so it is applied last; spec-pointer-bare-section-mark independent, §check-spec-pointer lying outside the slice; installer-smoke-live-upstream and tarball-attestation-observed reshaped by update-notice-probe-contract and install-attestation-binding.
- inferred: per-entry supersession was read by grep over deliverable names, not by an oracle; the stage each candidate triggers is judged from its deliverable

## 2026-10-04 build — Which facts do other surfaces cite into canon-kit/SPEC.md's claim-gate sections, §check-provenance-seam through §check-surface-duplication?
- corpus: . ':!docs/'
- oracle: git grep -n -E '§(check-provenance-seam|check-measured-claim|check-unmarked-claim|check-prose-enum|check-knob-citation|check-knob-default-coupling|check-surface-duplication)' -- ':!docs/'
- rev: a84b58577551a0bc94d6910bc1f8185f1a2d87b4
- finding: 173 citation sites across 34 files, mostly one-line spec: glosses in native/src naming a mechanism fact; prose citers are canon-kit/SPEC.md (16), gate-sdk/SPEC.md (7), evidence-kit/SPEC.md (2) and the audit roster. The brevity pass kept every heading verbatim and every cited fact: provenance-seam's honest-limit roster and disarmed default, measured-claim's marker spans and gate-substrates key, unmarked-claim's coverage-only rule and wrap boundary, prose-enum's in-process refusal and identifier boundary, knob-citation's code-point reach, knob-default-coupling unchanged, surface-duplication's registry-oracle exclusion and fixture-plus-smoke oracle.
- inferred: none
