# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.


















## 2026-08-15 scope — Which generated-projection freshness comparators and emitters are still shell, and how does the family cut across port-blockers' advisory groups?
- corpus: docs/site-architecture.md scripts/gen-*.sh gate-sdk/bin/enforcement-map.sh context-kit/bin/footprint.sh drift-kit/bin/trajectory.sh queue-kit/bin/roadmap.sh gate-sdk/checks/check-enforcement-fresh.sh context-kit/checks/check-footprint-fresh.sh scripts/gates.list
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 5c32a65b67de678ee0632e951413f61116ee2ccd
- finding: Six generated projections, each a comparator+emitter pair. Comparators ALREADY native: check-docs-mirror-fresh, check-value-rollup-fresh, check-trajectory-fresh. Comparators still shell and criteria-clearing (c2=pair c3=precommit c7=clean, both in advisory group 1): check-enforcement-fresh (gate-sdk/checks), check-footprint-fresh (context-kit/checks). Comparator HELD on an unruled emitter design: check-roadmap-fresh (advisory group 29). Comparator blocked on a command-position binary variable: check-gate-binary-fresh. ALL SIX emitters are still shell, 971 lines total: scripts/gen-docs-mirror.sh 127, scripts/gen-value-rollup.sh 124, gate-sdk/bin/enforcement-map.sh 273, context-kit/bin/footprint.sh 129, drift-kit/bin/trajectory.sh 242, queue-kit/bin/roadmap.sh 76. Judgment: the family is one design question (what a ported emitter IS under the substrate contract, which today has no representation for one) answered once and applied six times, and it cuts ACROSS the tool's advisory groups rather than following one. Second judgment: the size arm is exhausted a FOURTH time. Advisory group 1 is the largest at 9 members but its key is libs=fail_closed globs=- with no shared glob walk, and its members couple visibly divergent corpora (CLAUDE.md, docs/footprint.md, docs/enforcement.md, scripts/git-hooks, .workflow, TASK-QUEUE.md, delegation-kit/SPEC.md) — it is the null-key residue bucket, not a shared corpus derivation, which is the adjudication gate-sdk/SPEC.md section 'The first cohort, and the rule that selects the next' reserves to the selecting session.
