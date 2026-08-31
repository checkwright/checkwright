# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

















































## 2026-08-31 scope — Which owed cohort is composer-conforming at HEAD, and how does the port's owed column decompose after the 2026-08-31 consult rulings?
- corpus: every tracked non-test .sh (139 files); TASK-QUEUE.md ## Deferred; native/src/emit/mod.rs; .claude/settings.json
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree at d6e006ea; bash queue-kit/bin/queue-edges.sh; bash scripts/gate-exec.sh check-queue-entry-budget
- rev: 668b146ae1ca80bd6587f1464a7cb3addfdd1986
- edges: harness-template-port-residue 0 inbound; kit-library-port-residue 0 inbound; native-gate-port-remaining-corpus 8; powershell-installer-surface 5; platform-support-ci-matrix; RETIRED-BLOCK FINDING: kit-library-port-residue sequences toolfloor.sh behind installer-boundary-behind-invoke-port-reading and consumer-smoke-runner-port-disposition, BOTH of which reached ## Done at d6e006ea, so that sequencing sentence argues from two disposed premises
- finding: MEASURED at d6e006ea: 139 scanned, 64 no-port, 0 held, 75 owed / 7838 lines. SUPERSEDES the 2026-08-31 close decomposition (76/8689 at 80035780): the only delta is installer/consumer-smoke/run-smoke.sh (851 lines) leaving by per-file declaration, so ZERO lines were ported since that reading. By class: kit bin 38 files ~5045 lines (64 percent of the owed column, unchanged across three readings), installer runtime 10/1237, harness+hook templates and their scripts/ copies 12/613, kit lib 6/292, scripts/ 6/367, misc (demo, context-kit smoke, index-tests) 3/284. COMPOSER-CONFORMING CANDIDATES, against the 2026-08-28 rule of one specification section and the one amendment it needs: (1) harness-template-port-residue — 12 files 613 lines, section gate-sdk/SPEC.md §The non-gate arm, arm-kind question OPERATOR-RULED 2026-08-31, discharges a TRAJECTORY paragraph on landing, members re-verified file-by-file against this oracle and unchanged from the 2026-08-30 filing. (2) installer runtime — 1237 lines, section installer/README.md §The install boundary, whose step-value reading was ruled 2026-08-31, but entangled with design-pending powershell-installer-surface and a red Windows leg. (3) kit-library-port-residue FAILS the composer by its own text: its six members do not resolve together and it is 'one entry owning a residue rather than one cut'. (4) kit bin is the mass and has no settled arm-kind ruling for any sub-cohort. PREMISES RE-VERIFIED for candidate 1: native/src/emit/mod.rs BRIDGED_ARMS holds emitters plus the battery runner and no stdin/hook-envelope arm (true); .claude/settings.json:61 carries the allow grant naming delegation-kit/templates/usage-poller.sh (true, check-settings-paths corpus); seven more members are wired as hook command fields at lines 120-171 which check-settings-paths does not read (true). ONE FACT THE ENTRY DOES NOT CARRY: two of the wired hook commands, scripts/session-context.sh (137) and scripts/bash-guard.sh (40), are DECLARED no-port and stay shell, so the cut ports 12 of 14 harness members and the substrate is mixed by design.

## 2026-08-31 scope — Which deferred entries reach LIFECYCLE_KIT_RECURRENCE_THRESHOLD, and what does the pre-emption rule collide with this iteration?
- corpus: TASK-QUEUE.md ## Deferred, 284 entries
- oracle: grep -n '^  recurrence: ' TASK-QUEUE.md, anchored; threshold read from lifecycle-kit/SPEC.md line 550 (default 2)
- rev: 668b146ae1ca80bd6587f1464a7cb3addfdd1986
- edges: subagent-liveness-log-unattributed-refusal 1 inbound; dated-measurement-restatement-class 0 inbound; the former's fix site scripts/subagent-stop-liveness.sh is a MEMBER of the proposed port cut, so the two units converge on one file
- finding: MEASURED at 668b146a: 32 recurrence declarations across the pool. THIRTY carry exactly one date; TWO reach the threshold of 2. (1) dated-measurement-restatement-class 2026-08-25 2026-08-29 — already met threshold 2026-08-30 and promotion was DECLINED by the lead on own authority as machinery-class with no unit set reaching it; the entry carries that ruled line. (2) subagent-liveness-log-unattributed-refusal 2026-08-28 2026-08-31 — a FOURTH measurement at the last close, 440 events and 19 unattributed refusals, volume moved 11-of-172 to 21-of-366 to 19-of-440 while readability did not move at all. THE STRUCTURAL FACT A LATER SESSION WOULD RE-BUY: that entry's fix site is scripts/subagent-stop-liveness.sh (91 lines), which is one of the twelve members of harness-template-port-residue and is DELETED when its template ports — so a fix written into it outside the cut is written into a file the cut removes. The entry also has ZERO lines of headroom under the 50-line cap, so it cannot absorb another paragraph without eviction. COLLISION: the port-only run (TRAJECTORY.md, operator 2026-08-31) makes every iteration a port cut with no yield, while the scope template's threshold rule puts a counted recurrence in front of the escalation authority regardless of theme. The rule does not promote; it forbids the collision going unseen, and the decision is not scope's.

## 2026-08-31 align — Which tracked surfaces name any of harness-template-port-residue's twelve deleted basenames, beyond gate-sdk/SPEC-hook-arm.md delta 10's authored three?
- corpus: whole tracked tree, grep for the 12 member basenames (agent-dispatch-guard.sh agent-budget-guard.sh workflow-state-guard.sh subagent-stop-liveness.sh escalation-guard.sh wakeup-guard.sh usage-poller.sh statusline-usage.sh)
- oracle: grep -rn '<basename>' . (excluding .git), one term per basename, at HEAD 4e00dfea
- rev: 4e00dfea62a7e551ef3de17e7de8547129e54ec0
- edges: none
- finding: MEASURED at 4e00dfea62a7e551ef3de17e7de8547129e54ec0: 32 files hit across the 8 grepped basenames (4 members with a scripts/-copy pair need no basename grep, matched via the pair's own name). Every hit reconciles to either an already-covered SPEC/README section (delta's Existing-sections-updated roster), a test harness delta 11 already retires, or a docs/ mirror delta 12's generic regen covers -- EXCEPT four, none previously named: delegation-kit/smoke/install.sh:49,71 (invokes templates/usage-poller.sh and templates/subagent-stop-liveness.sh directly, part of this repo's own validate battery via run-consumer-smoke.sh), guard-kit/smoke/install.sh:9 (cp templates/wakeup-guard.sh into the scratch consumer, unguarded), delegation-kit/templates/delegation-config.sh:6 (a shellcheck-disable comment naming templates/agent-dispatch-guard.sh), .claude/commands/lead.md:95-98 (prose naming guard-kit/templates/wakeup-guard.sh and escalation-guard.sh as what a lead session wires). All four landed in the amendment's delta 10 at the 2026-08-31 align audit (harness-hook-arm-port), which also corrects delta 7's --statusline env-var claim (DELEGATION_KIT_USAGE_HISTORY dropped, no reader in the shipped producer -- filed to gap-inbox separately, unrelated pre-existing drift). The six-member channel table (delta 2), the sentinel/knob-union mechanism (delta 5, verified against native/src/emit/mod.rs), the guard.sh:258 fail-open precedent (delta 3), and every quantitative claim (613 total lines, 168 owed at six members, 67 owed at eleven) were independently verified against source and matched exactly -- no further correction needed there.
