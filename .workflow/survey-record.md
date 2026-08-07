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
