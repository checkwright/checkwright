# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-22 scope — Which deferred entries rank for this iteration under the admission filter, and what supersedes what?
- corpus: TASK-QUEUE.md
- oracle: checkwright-gates --emit queue-edges
- rev: 3bd53f251e99e185e412603858e239c94bb2624b
- finding: No session/iteration-class row passes the filter (heterogeneous-agent-delegation is excluded). Lead candidate: install-path-developer-first (event/high, TTFV arm). Same-surface first-contact bundle: docs-link-red-remedy-first, install-smoke-powershell-demo-runs-before-bash-strip, install-smoke-leg-names-mix-two-axes (reshaped by the lead unit: docs/install.md leg status moves off the adopter path), amendment-refusal-acceptance-parity (installer smoke + installer/SPEC.md). Most-rows surface: canon-kit (8). No entry reaches the recurrence threshold of 2. queue-flow mean-filed 3.0.
- inferred: admission-filter verdicts on roadmap rungs read from their bodies, not re-argued

## 2026-09-22 align — Does SPEC-install-per-os.md's roster and quoted current-tree text match the tree, and does its docs/install.md restructuring wire cleanly against installer/SPEC.md, installer/README.md, docs/index.md, docs/site-architecture.md and the install-failure template?
- corpus: . ':!docs/posts' ':!TASK-QUEUE.md'
- oracle: git grep -n -e "install.md §Requirements" -e "npx checkwright" -e "Quick start" -e "remedy:begin" -- . ':!docs/posts' ':!TASK-QUEUE.md'
- rev: 2da35ac9515c1ec4daa09364a7fff7d22a9b2435
- finding: Roster complete against the grep; delta 7's four gates.yml line citations (382, 1246, 1268, 1582-1584) and docs/site-architecture.md:58 verified to actually name a remedy block rather than the platform/toolchain block. Quoted current-tree paragraphs (installer/SPEC.md's dependency-boundary recipe paragraph at line 15, its Requirements sentence at line 34, its install-page requirement-blocks paragraph and Windows-remedy bullet at lines 38/42) match the tree byte-for-byte. No other §Requirements citation in the tree names a remedy block outside the ones delta 7 already retargets. No defect found standing alone; the cross-amendment findings are filed under the install-smoke-leg-names block.
- inferred: none

## 2026-09-22 align — Does SPEC-install-smoke-leg-names.md's rename table, retired-spelling block and roster match the tree, and is it consistent with sibling SPEC-install-per-os.md on their shared docs/install.md contingency?
- corpus: . ':!docs/posts'
- oracle: git grep -n -E "install-smoke" -- . ':!docs/posts'
- rev: 2da35ac9515c1ec4daa09364a7fff7d22a9b2435
- finding: All six job keys in .github/workflows/gates.yml (install-smoke, install-smoke-windows, install-smoke-macos, install-smoke-macos-intel, install-smoke-linux-arm64, install-smoke-powershell) match the rename table's today column. The two retired paragraphs (installer/SPEC.md:160's bootstrap-parity paragraph, gates.yml's PowerShell-leg comment block at 519-526, and the arm64 NON-decision paragraph at 1803-1809) exist verbatim at the quoted lead-in text. Found and fixed at this align session (commit 2da35ac9): delta 1's contingency wrongly framed sibling install-path-developer-first as possibly moving the platform block off docs/install.md entirely, when that sibling's own ruling keeps the block on the same file, only repositioned to end-of-page Requirements; and a literal rename-every-queue-mention reading would have corrupted install-smoke-leg-names-mix-two-axes's own family-wide problem statement in TASK-QUEUE.md (the backticked bare install-smoke referring to all six legs, not the baseline leg). Both amendment passages now resolve; no further cross-amendment conflict found.
- inferred: none

## 2026-09-22 align — Do canon-kit/SPEC-unwrap-declarations.md and canon-kit/SPEC-fence-run-home.md's quoted current-tree text and rosters match the tree?
- corpus: canon-kit native scripts
- oracle: git grep -n -e "CANON_KIT_UNWRAP_" -e "md_unwrapped::" -- canon-kit native scripts
- rev: 2da35ac9515c1ec4daa09364a7fff7d22a9b2435
- finding: unwrap-declarations' target bullet (canon-kit/SPEC.md:113), block-scanner paragraph (line 544) and md-unwrap paragraph (line 550) match the amendment's quoted current text exactly; its native/src roster (md_unwrapped.rs, emit/md_unwrap.rs, knobs/canon_kit.rs, gates/mod.rs) and scripts/canon-config.knobs are present as the amendment expects, with no row yet for the new declaration-lead knob delta 1 introduces (not yet applied, as stated). A second oracle, git grep -n -E "^\\s*(ruling|discharge|close-surface):" -- '*.md', confirms TRAJECTORY.md is the sole tracked file carrying live ruling:/discharge: line-start declarations outside template/grammar examples (lifecycle-kit/SPEC.md's own grammar block, .claude/commands/close.md's close-surface directives), and its discharge:/ruling: lines carry the two-space hard breaks the amendment says stay. fence-run-home's target sentence (canon-kit/SPEC.md:649, item 4, ending "is the consumer's act.") matches exactly. No defect found in either amendment.
- inferred: none
