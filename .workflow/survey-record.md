# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.



















## 2026-08-16 scope — How many deferred entries stand at the per-entry line cap with zero headroom?
- corpus: TASK-QUEUE.md
- oracle: awk '/^- [*][*]/{if(s){print n" "s} s=$2; gsub(/[*]/,"",s); n=0} s{n++} END{if(s)print n" "s}' TASK-QUEUE.md | sort -rn
- rev: 06f2e4627897373ac9bbd3efa3c632db0a3a552a
- finding: 13 deferred entries stand at exactly 50/50 lines (QUEUE_KIT_ENTRY_LINE_CAP), with zero headroom: stage-stamp-ordering-unenforced, settings-allow-intended-breadth-declaration, ruling-record-condition-staleness-probe, recurrence-drain-input-widening, probe-evidence-sufficiency, heterogeneous-agent-delegation, entry-headroom-unexposed, entry-cap-displaces-mandated-writes, deferred-release-declaration-accumulation, companion-toolkit-profile, cardinal-notation-splits-gate-reach, build-stage-tier-economics, batch-split-stamp-ownership. Two more sit at 49 (native-gate-port-remaining-corpus, contributor-writeback-disposition). The oracle was calibrated against queue-index.sh --icebox-candidates, which reports the same counts for the three rows they share (19/45/24). Judgment: entry-cap-displaces-mandated-writes' structural claim -- that the entries nearest the cap are the most-recurring ones -- holds on this census: three of the four entries at the recurrence threshold are in the zero-headroom set, so the next recurrence stamp on any of them displaces argued content by construction.

## 2026-08-16 scope — How many deferred entries stand at the per-entry line cap with zero headroom?
- corpus: TASK-QUEUE.md
- oracle: awk '/^- [*][*]/{if(s){print n" "s} s=$2; gsub(/[*]/,"",s); n=0} s{n++} END{if(s)print n" "s}' TASK-QUEUE.md | sort -rn
- rev: 06f2e4627897373ac9bbd3efa3c632db0a3a552a
- finding: SUPERSEDES the block above, whose last sentence miscounted. Census unchanged: 13 deferred entries stand at exactly 50/50 lines (QUEUE_KIT_ENTRY_LINE_CAP): stage-stamp-ordering-unenforced, settings-allow-intended-breadth-declaration, ruling-record-condition-staleness-probe, recurrence-drain-input-widening, probe-evidence-sufficiency, heterogeneous-agent-delegation, entry-headroom-unexposed, entry-cap-displaces-mandated-writes, deferred-release-declaration-accumulation, companion-toolkit-profile, cardinal-notation-splits-gate-reach, build-stage-tier-economics, batch-split-stamp-ownership; two more at 49 (native-gate-port-remaining-corpus, contributor-writeback-disposition). CORRECTED judgment, counted rather than asserted: of the four entries at the recurrence threshold, TWO are at zero headroom (stage-stamp-ordering-unenforced 50, entry-cap-displaces-mandated-writes 50), one has two lines (waiting-rule-fourth-firing-post-fix 48) and one is comfortable (icebox-worklist-roadmap-blind 37). So entry-cap-displaces-mandated-writes' structural claim -- the entries nearest the cap are the most-recurring ones -- holds for half this cohort rather than for it wholesale, and the sharper reading is that the two entries whose next recurrence stamp must displace argued content are the cap entry itself and the stamp-ordering entry.

## 2026-08-16 scope — What does the native port's remaining corpus partition into, and where is the amortization left?
- corpus: scripts/gates.list native/src/gates gate-sdk/checks lifecycle-kit/checks queue-kit/checks canon-kit/checks context-kit/checks delegation-kit/checks doctrine-kit/checks drift-kit/checks evidence-kit/checks guard-kit/checks site-kit/checks scripts
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 06f2e4627897373ac9bbd3efa3c632db0a3a552a
- finding: 104 gates registered, 61 native gate modules, 45 still shell. --group partitions the 45 into 37 groups by shared lib + glob key: 34 singletons, two 2-member groups, one 7-member group. The 7-member group's key is the weakest possible dependency signature (libs=fail_closed, globs=-): check-doctrine-registration, check-hook-exec-bit, check-workflow-tiering, check-close-surfaces, check-queue-prose-precondition, check-rule-citation, check-brevity -- 686 shell lines between them, every member c2=pair c3=precommit c7=clean. The two pairs are check-install-disposition + check-readme-roster (libs adds gate_kit_roots) and check-docs-render-fidelity + check-agent-tier-explicit (libs adds gate_path_pruned), the first of that last pair carrying c7=ruby. Judgment: shared-derivation amortization survives for exactly one group, so cohort composition for the remaining 34 singletons has to be justified by iteration budget rather than by a shared prerequisite -- the finding port-tail-cohort-batching-policy now owns.
