# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.


## 2026-08-07 spec — Are the seven recorded payload-disclosure ('all-source') overclaim sites plus the H1 qualifier site still present and verbatim unchanged?
- corpus: governed markdown: repo-root *.md, installer/, docs/ minus docs/posts/ and the docs/<kit>/SPEC.md mirrors
- oracle: delegated read-only witness (worktree-isolated) re-reading each recorded site, plus a bounded phrasing grep for new in-class sites
- rev: 4baea24368dfe6abf3ac7bf4c8d1fc5f6ace574e
- finding: All 8 present and verbatim unchanged at their recorded sections: installer/README.md §What this package is (12-13) and §init (55); docs/install.md GH-Release bullet (22-24), one-shot-vendoring paragraph (47-49), §Quick start (211-213), §What a gate discloses (291-311), H1 preamble (8-9, qualifier-only, NOT in the overclaiming class); SECURITY.md §Threat boundary (49). No new in-class site found. Recovered from git show 71c584e:canon-kit/SPEC-payload-claim.md — the align-hardened corpus of SEVEN, which supersedes the queue entry's stale FOUR. The amendment's own ruling refuses a hand tally as the completeness oracle; this list is a build worklist only.

## 2026-08-07 build — Does landing a .gate descriptor red an installer-smoke consumer's battery or reach an adopter's omit path?
- corpus: installer/lib/common/recipe.sh (recipe_gates, over the 11 payload kits) and gate-sdk/smoke/install.sh; the consumer registry init writes from them
- oracle: bash installer/consumer-smoke/run-smoke.sh, plus a real init + run-gates.sh against a consumer with both descriptors on disk, both .sh deleted, no binary and no crate (.tmp/exp3.sh)
- rev: 0512ba5b8cc98a28748955b6b35b9a2553e0b253
- finding: No. recipe_gates registers neither check-gate-binary-fresh nor check-gate-substrate-parity for any kit, and neither check-action-pinning nor check-action-gh-repo, so the consumer battery never runs the two meta-gates and plan_gates writes zero '# omitted:' lines. Verified: 24/24 gates green with both descriptors present. run-consumer-smoke.sh is a different registry (each kit's smoke/install.sh) and was not tested here; gate-sdk/smoke/install.sh registers neither action gate either.

## 2026-08-07 build — Which trees register a first-cohort ported member or a binary meta-gate, once the descriptors are live?
- corpus: installer/lib/common/recipe.sh recipe_gates; every kit's smoke/install.sh; scripts/gates.list
- oracle: grep over the three registry producers, plus bash gate-sdk/bin/run-consumer-smoke.sh and a binary-placed re-run of the scratch battery
- rev: f6a9b58ba13ff208d21b8802030d46a889cb7361
- finding: recipe_gates registers NONE of check-action-pinning, check-action-gh-repo, check-gate-binary-fresh, check-gate-substrate-parity, check-reads-couples — a real init adopter runs none of them, so the omit path is unaffected. gate-sdk/smoke/install.sh registers check-gate-substrate-parity and check-gate-binary-fresh (lines 18-19). site-kit/smoke/install.sh registers check-action-pinning and check-action-gh-repo (lines 15,17) under a spec: justification, because it copies templates/site-health.yml in as the only Actions-shaped surface any install writes. So run-consumer-smoke.sh is the ONLY tree where a ported member dispatches with no binary: 5 of 75 gates FAILED. Placing the host binary does not clear it — check-gate-binary-fresh needs git -C native ls-files and no consumer tree carries the crate, and assertion F then reds on the absent native/targets.list. The amendment's claim that the ported gates are not in the smoke registry at all is false.
