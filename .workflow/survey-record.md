# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.




















































## 2026-09-02 scope — Which owned specification section is the next well-formed port cut, and what does each candidate group unblock?
- corpus: '*.sh' ':!*.test.sh'
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 4942ae172b943ec84c0c05c6b867baffbbd049fb
- edges: native-gate-port-remaining-corpus 8 inbound (kit-library-port-residue, iteration-scoping-clause-date-ambiguity, spec-lib-dead-derivation, ruling-accretion-outgrows-the-entry-cap x4, record-stamp-encoding-compression); no retired-slug citations in that block. The cut candidates are contract sections rather than slugs, so all inbound weight lands on this one standing entry.
- finding: 51 owed, grouped by the owning specification section each file's own '# spec:' header declares. Six groups of two or more, the rest singletons. UNBLOCKED AND WELL-FORMED: delegation-kit/SPEC.md §Testing (run-usage-tests.sh 226 + run-trend-tests.sh 63 = 289 lines) — one stated contract, the hermetic ambient-strip-plus-poison-export runner discipline, not a directory; guard-kit/bin/run-guard-tests.sh is the same SHAPE but declares guard-kit's own §Testing and is NOT a member. HIGHEST LEVERAGE: gate-sdk/SPEC.md §upgrade-smoke (upgrade-smoke.sh 239) — the only cut in the corpus that unblocks another owed member, and the unblock was PROBED not assumed: a grep for sourcers of gate-sdk/lib/declaration.sh over every non-crate .sh returns upgrade-smoke.sh plus two *.test.sh files, and the test suffix is outside the oracle's corpus, so this cut empties the shell caller set kit-library-port-residue names as declaration.sh's own stated release test (59 further lines). BLOCKED, each on a stated ground read rather than inferred: context-kit §bin/env-probe's toolfloor.sh and the whole of context-kit §Testing sit behind the installer behind-invoke relocation, and context-kit/SPEC.md §Testing says that group is blocked as a whole; installer/README.md §The install boundary is 1184 lines across nine files and is operator-ruled 2026-08-31 as a cut's stated contract, but it is powershell-installer-surface's and is ordered behind platform-support-ci-matrix's Windows leg. THREE OWED FILES HAVE NO SPECIFICATION OWNER — scripts/pack-installer.sh, demo/run-demo.sh and installer/bin/checkwright.sh all declare CLAUDE.md §Housekeeping, which is the always-loaded manifest and cannot hold a port contract, so they are uncuttable by stated contract until a section owns them; checkwright.sh looks like a stale header, since powershell-installer-surface already homes it in the bootstrap. STRUCTURAL FACT ABOUT THE COMPOSER: gate-sdk/lib/inject.sh's three shell sourcers declare three DIFFERENT sections, so no single stated-contract cut can ever clear it. TWO RULE AMBIGUITIES THE CORPUS DOES NOT SETTLE, both escalated: whether a section whose remainder is externally sequenced may be cut partially, which would make env-probe.sh a legal 141-line cut; and whether scripts/parse-gates-log.sh plus scripts/parse-installer-smoke-log.sh — consumer-side tier-1 plugins on evidence-kit's parser seam — port at all, since ruling (1) says a cut narrows the port and never an extension point while the completion predicate admits no subtraction.
