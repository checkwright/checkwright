# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.











## 2026-08-13 scope — What must the owed POSIX ERE engine actually implement — which of the nine ERE-owing gates need substitution or span extraction over a CONSUMER pattern, rather than only a match test?
- corpus: queue-kit/checks/check-queue-prose-precondition.sh canon-kit/checks/check-install-claim.sh canon-kit/checks/check-payload-claim.sh canon-kit/checks/check-manifest-temporal.sh canon-kit/checks/check-spec-derivable-section.sh canon-kit/checks/check-deprecation-task.sh gate-sdk/checks/check-tree-terms.sh gate-sdk/checks/check-commit-msg.sh context-kit/checks/check-brevity.sh
- oracle: grep -n '~ |match(|gsub(|sub(|grep -E' over the nine files, then read each consumer knob to its use site
- rev: 116f0d864b5abed916f58f69a1c292722da53bc9
- finding: Eight of the nine use their consumer-configured pattern ONLY as a match test: check-install-claim (:110 head ~ sectre, :115, :118), check-manifest-temporal (:64 low ~ markers[i]), check-queue-prose-precondition (:27 b ~ trig), check-payload-claim (consumer input is a claim vocabulary from a command, not a regex), check-tree-terms and check-commit-msg (both grep -EnHf over consumer pattern files - whole-line match, no -o), check-brevity (CONTEXT_KIT_BREVITY_POINTER_RE, awk-interpreted). Only check-deprecation-task needs more: :29 pipe-joins CANON_KIT_DEPRECATION_MARKERS unescaped and :56 does match()+substr(RSTART,RLENGTH), i.e. span extraction over an arbitrary consumer pattern. Every gsub/sub in the set runs over a pattern baked literally into its awk source (e.g. check-manifest-temporal:61, check-queue-prose-precondition:25-26), which a Rust port hand-compiles - the same ground that correctly screened check-comment-tier out via join_alt's bracket-escaped literals. NET: the engine is a POSIX ERE matcher with leftmost-longest span reporting. No substitution engine and no capture-group replacement is owed by any of the nine. This is materially smaller than the union recorded on cohort-held-members-port-prerequisites, and it lowers the cost of the largest single piece of work the port has named.

## 2026-08-13 scope — Completing the previous block's enumeration: is check-spec-derivable-section, the ninth ERE-owing gate, a tester or does it need more?
- corpus: canon-kit/checks/check-spec-derivable-section.sh
- oracle: grep -n '~ |sub(|CANON_KIT_' canon-kit/checks/check-spec-derivable-section.sh
- rev: 116f0d864b5abed916f58f69a1c292722da53bc9
- finding: Tester. Line 55 uses the consumer knob CANON_KIT_DERIVABLE_POINTER_REGEX as '$0 ~ pointer', a match test; its two sub() calls at :47 strip baked literals from a heading. This is the member the previous block listed in its corpus but did not name in its finding, so the 'eight of the nine are testers' claim was one member short of its evidence when written and is now complete: the eight testers are check-install-claim, check-manifest-temporal, check-queue-prose-precondition, check-payload-claim, check-tree-terms, check-commit-msg, check-brevity and check-spec-derivable-section; check-deprecation-task is the sole member owing span extraction over a consumer pattern. The previous block's conclusion is unchanged - it was under-evidenced in its enumeration, not wrong.
