# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.






































## 2026-08-25 scope — Does the citation-liveness deferred family converge on one mechanism, and what does closing it cost?
- corpus: TASK-QUEUE.md ## Deferred + ## Icebox; scripts/gates.list; native/src/gates/
- oracle: bash queue-kit/bin/queue-edges.sh; grep -n over TASK-QUEUE.md for citation/cite/liveness
- rev: ccac82f81f45ff07d5b5cbee6f8446465f070fa6
- finding: FOUR extension points, not one resolver and not N fixes; 14 members (12 Deferred, 2 Icebox); no member blocks on an operator-class fork. (A) check-spec-pointer (native/src/gates/spec_pointer.rs) absorbs prose-filename-citation-liveness:1228, unqualified-section-citation-liveness:1802, link-wrapped-section-citation-liveness:2764, spec-pointer-self-section-citation:3258 as ONE resolves-to-nothing predicate (the entries themselves say the guard WINDOW, not the citation form, is the variable), plus spec-section-title-collision:4423 and qualified-pointer-section-ownership:3471 as two harder separate predicates on the same gate — the latter self-declares an honest not-buildable a permitted outcome. (B) check-queue-slug-liveness (native/src/gates/queue_slug_liveness.rs) takes ~2 assertions for retired-slug-live-pointer-citation:6337 and queue-status-parenthetical-liveness:6373, plus TWO report-only deliverables riding bin/queue-edges.sh's existing resolution — dead-queue-citation-report:2450 and done-slug-ownership-citation-report:5177, both explicitly report-not-gate by the SPEC's own reference-vs-membership ruling. (C) check-docs-cmd (native/src/gates/docs_cmd.rs) widens from fenced-only to inline spans for cited-script-path-liveness-inline:5942 and stale-identifier-after-retirement:5495 — likely one ticket, but which gate holds it is itself an open ruling in the second entry. (D) guard-rule-number-not-citable-outside-kit:6467 and guard-rule-number-intra-kit-citations-ungated:6575 are an island: NO gate resolves rule-number citations, 111 intra-kit citations measured at the last renumber, so a new gate is owed and the false-positive budget is the unknown. Icebox members doctrine-rule-number-citation-liveness:7879 and false-ground-citation-propagation:7880 are one-liners whose bodies could not be read, so their membership is title-only and UNVERIFIED — and DOCTRINE.md carries no 'rule N' form today, so the first may be speculative. EXCLUDED with cause: scratch-citation-skill-surface-reach (self-disclaims, a glob-coverage gap), kit-ref-liveness-stem-token-hole (env-knob tokens, already checked), fixture-assertion-liveness, survey-oracle-liveness-unasserted. ADJACENT, unfolded: amendment-landing-citation-assertions:2055 and amendment-owner-position-citation:2090 share the shape but ride check-amendment-queue over a corpus deleted at merge. Size floor: 4 gate-touch points, ~8-10 new assertions, 2 report outputs, ALL native-crate (born-native default; every gate named is already a native module).
