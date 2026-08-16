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

## 2026-08-16 scope — What does the native port's remaining corpus partition into, and where is the amortization left?
- corpus: scripts/gates.list native/src/gates gate-sdk/checks lifecycle-kit/checks queue-kit/checks canon-kit/checks context-kit/checks delegation-kit/checks doctrine-kit/checks drift-kit/checks evidence-kit/checks guard-kit/checks site-kit/checks scripts
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 813efecc19b6eabdba64e1bfc5620097074b90ba
- finding: SUPERSEDES the block above, whose judgment was wrong on the governing rule. Partition unchanged: 104 registered, 61 native modules, 45 shell, 37 groups, 34 singletons, two pairs, one 7-member group. CORRECTED judgment: the 7-member group is NOT a cohort and the SPEC already rules so -- gate-sdk/SPEC.md:2371-2379 rejects exactly this key (libs=fail_closed globs=-) because fail_closed derives no corpus, so its members share the ABSENCE of a derivation; operator-ruled 2026-08-14. Of the two pairs, one holds check-install-disposition, which SPEC.md:864 keeps on shell permanently (a declaration-and-dispatch auditor must not depend on the substrate it names), and the other holds check-docs-render-fidelity on criterion 7 (ruby). So no valid multi-member cohort remains anywhere in the 45: the shared-derivation selector selects nothing, which is a stronger finding than thin amortization and is the real ground under port-tail-cohort-batching-policy. Also recorded so it is not misread: a bare port-blockers run reports 62 undecidable, which is the already-ported members answering ? (binary substrate; no --needs), not a regression against the eighth cohort's recorded 0.

## 2026-08-16 scope — Which of the 11 gates in the three smallest-key port groups are portable now, and at what cost each?
- corpus: doctrine-kit/checks/check-doctrine-registration.sh gate-sdk/checks/check-hook-exec-bit.sh gate-sdk/checks/check-workflow-tiering.sh lifecycle-kit/checks/check-close-surfaces.sh queue-kit/checks/check-queue-prose-precondition.sh delegation-kit/checks/check-rule-citation.sh context-kit/checks/check-brevity.sh gate-sdk/checks/check-install-disposition.sh gate-sdk/checks/check-readme-roster.sh delegation-kit/checks/check-agent-tier-explicit.sh site-kit/checks/check-docs-render-fidelity.sh gate-sdk/SPEC.md lifecycle-kit/bin/close-surfaces.sh
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 813efecc19b6eabdba64e1bfc5620097074b90ba
- finding: Delegated read-only audit against the seven port-candidate criteria; two load-bearing SPEC citations re-read by the dispatching session rather than relayed. READY, no open design question, 6: check-hook-exec-bit (46 shell lines, pure scan of git ls-files -s), check-agent-tier-explicit (62), check-rule-citation (77), check-workflow-tiering (85), check-brevity (104), check-doctrine-registration (246, dearest -- two AWK section-walkers, and no heading-bounded-section helper exists in native/src yet). READY with bounded pre-work, 2: check-queue-prose-precondition (64; its named ERE-engine hold is PAID, reuses native/src/queue.rs::Sections and native/src/ere.rs) and check-readme-roster (108; clean logic but its corpus is checks/*, so its own descriptor joins the scanned set once ported -- criterion 4, discharged by a widened fixture pair on the check-tree-terms precedent). NOT in any near cohort, 3: check-close-surfaces (criterion 6 -- it wraps the unported lifecycle-kit/bin/close-surfaces.sh that close.md step 3 also calls, so it needs the check-roadmap-fresh-shaped non-gate-arm design first; lifecycle-kit/SPEC.md calls it unsized by any cohort), check-docs-render-fidelity (criterion 7, ruby+kramdown, 241 lines of render-comparison logic), check-install-disposition (not a future port at all -- gate-sdk/SPEC.md:864 keeps it shell by ruling, so it should leave the remainder census). Cross-check: none of the 11 reads a declare -A knob by key, so the associative-array bridge blocks none of them; its two real members (check-stage-entry, check-evidence-baseline) are outside this set.

## 2026-08-16 scope — How many gate members are ported, by the registry's own oracle rather than a file count?
- corpus: native/src/gates scripts/gates.list
- oracle: bash scripts/measured-claims.sh
- rev: 2bdcc4cd2b3dc680bd9f572e9a6506d6605929b9
- finding: 59 of 104, emitted as ported-gate-members. This corrects a subsidiary figure in both earlier partition blocks on this page, which said 61 native gate modules: that was a file count of native/src/gates/*.rs, which includes mod.rs and any module that is not a registered member. The load-bearing partition figures in those blocks are unaffected -- 104 registered, 45 still shell, 37 groups, 34 singletons -- and 104 minus 59 is the 45 they already state. Recorded separately because a later stage citing a ported-count should cite the oracle, not a directory listing.

## 2026-08-16 scope — What did recording the lead's rulings cost against the per-entry cap, and what does that tell the stage that designs the cap relief?
- corpus: TASK-QUEUE.md
- oracle: none
- rev: bdcc3947bef72c2d74a27b3e7ff5dd1e251a50f3
- finding: NOTE, not a re-usable survey: its grounds are this session's own experience, so re-derive before relying on it. Recording the Q2 ruling was itself a firing of entry-cap-displaces-mandated-writes, the entry the same ruling promotes. Two of the three entries owed a mandated write stood at 50/50 with zero headroom, so neither ruling could be seated without first compressing. Both were seated by the gate's first relief, compress-by-ANSWERING, and both compressions are defensible on their own terms rather than as budget-making: in waiting-rule-fourth-firing-post-fix a coupling clause appeared twice in one entry and had already been engaged and dispositioned, so the spent copy went and the quoted one stayed; in stage-stamp-ordering-unenforced the symptom paragraph asserted 'nothing caught it' where the cause paragraph below already answers which gates and why, so the assertion was replaced by a pointer. The third relief -- relocation to a linked entry -- was never reached, which matters because it is the authorization-gated one and the authority was present and would have granted it. Design input for the relief: the cheap candidate (exempt the machine-written recurrence line from the count) would NOT have helped here. Neither of these writes was a recurrence stamp; both were argued prose recording a ruling, which is exactly the content the cap is meant to protect. The candidate that would have helped is the third -- a standing self-served authorization to relocate -- or the second, treating an at-cap entry receiving a fresh ruling as a promotion signal.
