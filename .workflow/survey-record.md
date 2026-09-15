# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-15 scope — which deferred entries lead this iteration's unit set under scope's cost-class ranking
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: fe507943af49b7da7d91b36739bb2e5728d34436
- finding: 139 deferred lead lines; cost classes session/high 1, session/low 3, iteration/high 13, iteration/low 17; no recurrence: line reaches threshold 2. The one session/high row, couples-dynamic-root-resolution (gate-sdk), is blocked-by couples-glob-semantics-unowned (event/high), whose inbound edges are that row and depth-enumerated-glob-bound-unoracled; so the unit that moves the top row is its blocker. The three session/low rows sit on three different surfaces (canon-kit, delegation-kit, queue-kit). In the retired block, check-spec-pointer (16 inbound) is a live registered gate name, not disposed work.
- inferred: couples-dynamic-root-resolution's session/high class and its cost prose are taken as recorded, not re-priced; that settling couples= semantics can move its 26 withdrawn roots is the entry's own claim, reasoned not probed

## 2026-09-15 scope — which crate readers match the couples= manifest field, and under which glob semantics
- corpus: native/src gate-sdk/SPEC.md
- oracle: grep -rn '"couples"\|fn pattern_match\|fn path_matches_glob\|fn glob_walk\|fn staged_matches' native/src --include=*.rs
- rev: fe507943af49b7da7d91b36739bb2e5728d34436
- finding: Seven sites read the couples field. Two only print it (emit/graph.rs:177 draws edges, emit/port_blockers.rs:611 prints a row). Matching sites: check-gate-substrate-parity assertion C (gate_substrate_parity.rs:684) uses walk::pattern_match (walk.rs:472), bash string match where * and ? cross /; the trigger path (runner.rs:310 staged_matches, and emit/git_hooks.rs:233 for the hook) is the same slash-spanning form; check-reads-couples matches couples with path_matches_glob (reads_couples.rs:157), segment-count-equal with no ** arm, while its filter walk glob_walk (reads_couples.rs:305) has a ** arm; gates/graph.rs:223/482 reads couples for assertion B. gate-sdk/SPEC.md:2200 §Reading a couples= field's reach states the trigger/coverage divergence and leaves the field's intended semantics unsettled.
- inferred: that the generated hook's staged_matches splices the same bash string match runner.rs staged_matches ports is read off runner.rs:308-310's spec comment, not executed; the check-graph gate's assertion B token-subset reading is carried from the couples-glob-semantics-unowned entry body and gate-sdk/SPEC.md, not re-read in graph.rs

## 2026-09-15 spec — which fallback-branch walk reads the fallback members' literal couples leave uncovered, under the trigger matcher versus the segment-wise one
- corpus: .
- oracle: a bash probe reading each member's literal couples (knob tokens dropped, kit: expanded over gate-sdk and *-kit), the fallback read sets (SPEC.md/README.md/CLAUDE.md for manifest members, .sh/.gate/.rs for comment-surface members; gate-tests, target, node_modules, templates and docs/<kit>/ removed), each path tested with [[ path == token ]] and with an equal-slash-count variant
- rev: 033bd1c8ce507de447927bf70fd3e95cd12dfb4b
- finding: manifest members (10): 26 reads, 0 uncovered under the trigger matcher, 24 under segment-wise; comment-surface members (3, plus check-spec-pointer): 403 reads, 2 uncovered under the trigger matcher (installer/bin/checkwright.sh, installer/consumer-smoke/run-smoke.sh: .sh outside kit roots and scripts/, a genuine under-couple), 395 under segment-wise. Tracked governed sources outside gate-tests reach at most five path segments, the last rung of CANON_KIT_COMMENT_SURFACE's enumeration.
- inferred: the probe approximates the walks' prune and kit-root filters by path grep rather than running spec::manifest_files/comment_surface; the segment-wise figure uses slash-count equality plus string match, an approximation of path_matches_glob
