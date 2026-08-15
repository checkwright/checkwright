# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.


















## 2026-08-15 scope — Which generated-projection freshness comparators and emitters are still shell, and how does the family cut across port-blockers' advisory groups?
- corpus: docs/site-architecture.md scripts/gen-*.sh gate-sdk/bin/enforcement-map.sh context-kit/bin/footprint.sh drift-kit/bin/trajectory.sh queue-kit/bin/roadmap.sh gate-sdk/checks/check-enforcement-fresh.sh context-kit/checks/check-footprint-fresh.sh scripts/gates.list
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 5c32a65b67de678ee0632e951413f61116ee2ccd
- finding: Six generated projections, each a comparator+emitter pair. Comparators ALREADY native: check-docs-mirror-fresh, check-value-rollup-fresh, check-trajectory-fresh. Comparators still shell and criteria-clearing (c2=pair c3=precommit c7=clean, both in advisory group 1): check-enforcement-fresh (gate-sdk/checks), check-footprint-fresh (context-kit/checks). Comparator HELD on an unruled emitter design: check-roadmap-fresh (advisory group 29). Comparator blocked on a command-position binary variable: check-gate-binary-fresh. ALL SIX emitters are still shell, 971 lines total: scripts/gen-docs-mirror.sh 127, scripts/gen-value-rollup.sh 124, gate-sdk/bin/enforcement-map.sh 273, context-kit/bin/footprint.sh 129, drift-kit/bin/trajectory.sh 242, queue-kit/bin/roadmap.sh 76. Judgment: the family is one design question (what a ported emitter IS under the substrate contract, which today has no representation for one) answered once and applied six times, and it cuts ACROSS the tool's advisory groups rather than following one. Second judgment: the size arm is exhausted a FOURTH time. Advisory group 1 is the largest at 9 members but its key is libs=fail_closed globs=- with no shared glob walk, and its members couple visibly divergent corpora (CLAUDE.md, docs/footprint.md, docs/enforcement.md, scripts/git-hooks, .workflow, TASK-QUEUE.md, delegation-kit/SPEC.md) — it is the null-key residue bucket, not a shared corpus derivation, which is the adjudication gate-sdk/SPEC.md section 'The first cohort, and the rule that selects the next' reserves to the selecting session.

## 2026-08-16 spec — Do the three freshness-emitter-substrate amendments (SPEC-emitter-substrate.md, SPEC-liveness-record.md, SPEC-jq-floor.md) hold self-consistent and tree-consistent, and does the align trigger genuinely fire?
- corpus: gate-sdk/SPEC-emitter-substrate.md delegation-kit/SPEC-liveness-record.md installer/SPEC-jq-floor.md gate-sdk/SPEC.md context-kit/SPEC.md delegation-kit/SPEC.md evidence-kit/SPEC.md guard-kit/SPEC.md guard-kit/lib/guard.sh gate-sdk/lib/inject.sh installer/lib/init.sh installer/lib/common/lock.sh installer/README.md lifecycle-kit/checks/check-stage-entry.sh
- oracle: bash gate-sdk/bin/run-gates.sh (104/104 green, before and after the amendment edits below); literal grep+read against check-stage-entry.sh assertion C and against each amendment's cited line numbers/section headers
- rev: 58499fdc086dd7bfbe0b0c1a61129f5ca44ec586
- finding: Trigger confirmed genuine: check-stage-entry.sh assertion C fires purely on amendment dir count (amend_dirs >= 2; three component dirs here), independent of any content signal. installer-jq-silent-degradation's corrected premise verified byte-accurate: init.sh:54/56/76-78, lock.sh:14, doctor.sh:79-80, README.md Requirements/doctor sections all match as cited. Three findings resolved in-amendment (mechanics, no envelope change): (1) SPEC-emitter-substrate.md's roster omitted gate-sdk/SPEC.md's existing enforcement-map/check-enforcement-fresh sections and context-kit/SPEC.md's check-footprint-fresh section + Layout file-tree listing, all of which describe the pre-port shell-spawn mechanism and need real content changes at build -- added. (2) SPEC-emitter-substrate.md sec6 falsely claimed the new Rust marker-writer 'preserves inject.sh's contract'; gate-sdk/lib/inject.sh:25-29 actually appends on an absent marker, not refuses -- corrected to state the write half deliberately tightens/diverges from the shell original rather than preserving it. (3) SPEC-liveness-record.md sec5 claimed 'exactly two' restatement carriers (stage-session.md, audit-sweep.md) but guard-kit/lib/guard.sh's guard_rule_pgrep_self_match corrective message (mirrored in guard-kit/SPEC.md rule 12) states the same pre-widening narrow clause and is a live third carrier, distinct from waiter-loop-condition-predicate-gap's firing-predicate concern -- added as a third carrier throughout sec5, roster, producers/consumers and DoD. All edits confirmed green on full battery re-run after landing.
