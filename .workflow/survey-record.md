# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-17 scope — Which deferred entry leads this boundary's unit set, by the scope rank order (cost class, roadmap, surface match)?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges; grep -n 'cost: *session\|cost: *iteration' TASK-QUEUE.md
- rev: 9dc4be9ce389dde7ab61da801a487158385096e2
- finding: Lead: inline-interpreter-substrate-census (guard-kit), premise re-verified (~224 inline python3 calls in ~9350 Bash uses since 2026-09-15). Same-surface join: scan-prompts-heredoc-grant-split (skews the census figure). install-smoke-slow-leg-residue's 31m46s Windows figure is stale (10m35s at run 35194231255). binding-intel-leg-failed-one-run-in-two unrecurred since 2026-09-08. citation-liveness-family-convergence carries the largest inbound sum but a standing not-this-window direction.
- inferred: no session-class rows; five iteration/high rows; CI leg timings (gh run 35194231255) and transcript counts are off-tree reads, re-probe rather than witness

## 2026-09-17 spec — What do the inline python3 bodies agents run compute, and which recur as unported tooling?
- corpus: harness session transcripts for this project (top-level and nested), Bash tool_use calls dated 2026-09-08 onward (33356 calls, 39 sessions); post-rewrite-arm slice 2026-09-15 onward (8112 calls)
- oracle: jq over each transcript: select .message.content[] tool_use name Bash, .input.command; python3 inline = test('(^|[;&|\n] *)python3 (- *<<|-c )'); file write = test('open\([^)]*,\s*["'\''][wa]|write_text\('); literal-rewrite markers = .replace( with no f-string/.format(/re.sub/slice (.tmp/census/extract.sh)
- rev: 0aed197ac0ebee38a42ae73c871f0c9be1002bdd
- finding: 871 inline interpreter calls since 09-08 (821 python3 heredoc, 50 python3 -c, zero node/ruby/perl -e); clusters: literal read-assert-replace-write 280 calls/26 sessions (266 tracked targets), heading-anchored splice 157/28, counting and probes 375 mostly pure reads, json 25/9, multi-file sweep 17/11, git post-processing 15/6. Recurring in 3+ sessions: the read, assert count, replace, write-back idiom (11 sessions) and a yaml safe_load check (3). Since 09-15: 172 python3 inline calls, 115 write a file, 94 of them also call .replace(. Python bodies: median 844 chars, p90 3402; all Bash commands: p90 507, 3 of 8112 over 10000. No new tool is named: the recurring computation's target, --rewrite, exists and no rule steers python to it.
- inferred: ~half of the rewrite-shaped bodies are --rewrite-coverable, sampled not audited; blockers in order: computed replacement text, per-file varying pairs, conditionals
