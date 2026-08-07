# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.


## 2026-08-07 spec — Are the seven recorded payload-disclosure ('all-source') overclaim sites plus the H1 qualifier site still present and verbatim unchanged?
- corpus: governed markdown: repo-root *.md, installer/, docs/ minus docs/posts/ and the docs/<kit>/SPEC.md mirrors
- oracle: delegated read-only witness (worktree-isolated) re-reading each recorded site, plus a bounded phrasing grep for new in-class sites
- rev: 4baea24368dfe6abf3ac7bf4c8d1fc5f6ace574e
- finding: All 8 present and verbatim unchanged at their recorded sections: installer/README.md §What this package is (12-13) and §init (55); docs/install.md GH-Release bullet (22-24), one-shot-vendoring paragraph (47-49), §Quick start (211-213), §What a gate discloses (291-311), H1 preamble (8-9, qualifier-only, NOT in the overclaiming class); SECURITY.md §Threat boundary (49). No new in-class site found. Recovered from git show 71c584e:canon-kit/SPEC-payload-claim.md — the align-hardened corpus of SEVEN, which supersedes the queue entry's stale FOUR. The amendment's own ruling refuses a hand tally as the completeness oracle; this list is a build worklist only.
