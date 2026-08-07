# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-08-07 build — which surfaces does landing a new gate actually stale in this repo?
- corpus: docs/ scripts/gates.list scripts/git-hooks/ */README.md
- oracle: bash gate-sdk/bin/run-gates.sh
- rev: 645326c77f4bb0dc209c5f6a7fc3abc9a7733fb2
- finding: docs/site-architecture.md §Generated projections names six regen commands (docs mirror, enforcement map, footprint, value rollup, graph artifact, generated hooks) and all six were needed. Three more surfaces red that it does not name, each found by the battery rather than by the roster: the owning kit README's gate-roster block (check-readme-roster), the release note under composition, whose Tightened-gates bullets must match the tightened-gates declaration surface (check-tightened-gates-note-parity), and check-gate-tamper, which forces any non-meta-layer path — here .gitattributes — into a separate commit from the gate file. Battery count moved 96 to 97.

## 2026-08-07 build — which surfaces does landing a new gate actually stale in this repo?
- corpus: docs/ scripts/gates.list scripts/git-hooks/ */README.md */gate-tests/ */checks/
- oracle: bash gate-sdk/bin/run-gates.sh
- rev: 18ae4008790a71175311f8a2ef09625532c96a12
- finding: Supersedes the block above, whose witness went stale when this batch's own commits moved docs/. Same answer plus two surfaces the first pass had not yet met. Beyond the six regen commands docs/site-architecture.md §Generated projections names: the owning kit README's gate-roster block; the release note under composition, whose Tightened-gates bullets must match .workflow/tightened-gates.txt; check-gate-tamper, which forces a non-meta-layer path into a separate commit from the gate file; the fixture pair of any EXISTING gate whose derived set the new surface joins, which the whole-tree battery cannot catch because the repo's real file is correct while the fixture builds its own; and check-comment-tier, if the new surface is workflow-dir markdown. Battery count 96 to 97.
