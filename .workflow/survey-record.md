# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-27 scope — Which deferred entries join the ruled companion-toolkit-profile slice this iteration, by the scope ranking tiers and inbound edges
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: e1832eb24f89e09f4d30a1d62e0859f20011da5e
- finding: Corpus is the Deferred and Icebox sections. Tier-1 session/iteration rows: spec-brevity-residue (session/high, lifecycle-kit) joins by the join rule; one-motion-commit-race-remains-open (session/low, CLAUDE.md) and the iteration/low rows sit on surfaces the set does not carry. Same-surface (lifecycle-kit) fill: lead-writes-during-live-stage, consult-inbox with roadmap-horizon-motion-unowned (consult-inbox first), tier-only-rank-out-unruled (needs a ruling). No deferred entry reaches the recurrence threshold of 2. companion-toolkit-profile has 2 inbound edges, neither blocking; no retired-block row cites it.
- inferred: none

## 2026-09-27 spec — How many section-citation lines outside fences do the rendered docs pages, the kit READMEs and the kit SPECs carry, and in which forms
- corpus: docs '*/README.md' '*/SPEC.md'
- oracle: git ls-files 'docs/*.md' 'docs/*/index.md' '*/README.md' '*/SPEC.md' | grep -v -e '^docs/posts/' -e gate-tests -e '^reserve/' -e '^docs/[^/]*/README.md' -e '^docs/[^/]*/SPEC.md' | xargs awk 'FNR==1{fen=0} /^[[:space:]]*```/{fen=!fen; next} !fen && /§/ {n[FILENAME]++} END{for (k in n) print n[k], k}'
- rev: a0cffa9b5c4f8db3c96d65ba49dcc7f9ada79cb1
- finding: Docs pages 46 lines (site-architecture 16, orchestration 8, install 7, enforcement 6 already links and generated, ddd 5, positioning 3, releases 1); the twelve top-level READMEs 52 (drift-kit 11, lifecycle-kit 10, delegation-kit 6, gate-sdk 4, guard-kit 4, installer 4, evidence-kit 3, context-kit 2, queue-kit 2, site-kit 2, canon-kit 1, doctrine-kit 1); the kit SPECs 2,129. Forms seen: whole-code-span path citation, bare path citation, backticked path then section, link then section, path-less same-page, SPEC with no extension, possessive. Only docs/install.md opens <details> regions (two, both markdown=1 with a summary and no heading).
- inferred: none
