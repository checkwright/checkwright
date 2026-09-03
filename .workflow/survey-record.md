# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.




















































## 2026-09-02 scope — Which stated-contract port cuts are unblocked and takeable now, and which of them is the corpus's own dependency structure pointing at?
- corpus: '*.sh' ':!*.test.sh'
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 558264504c1506ab4180006b0c91149a4b90592e
- edges: native-gate-port-remaining-corpus 8 inbound (kit-library-port-residue, iteration-scoping-clause-date-ambiguity, spec-lib-dead-derivation, ruling-accretion-outgrows-the-entry-cap x4, record-stamp-encoding-compression), one of which the retired block flags as citing a retired owner slug; threshold entries ranked beside it, each 1 inbound — dated-measurement-restatement-class 1, stage-journal-absence-caught-only-downstream 1, cardinal-notation-splits-gate-reach 1, observation-predicate-entry-cannot-drain-in-its-own-iteration 1. The cuts themselves are contract sections rather than slugs, so their weight lands on the one standing entry, as at the prior boundary.
- finding: 48 owed, down 3 from the 51 the 2026-09-02 record at 4942ae172b943ec84c0c05c6b867baffbbd049fb measured; that record's witness was run and it MOVED by exactly its own two recommendations (upgrade-smoke.sh, run-usage-tests.sh, run-trend-tests.sh deleted, five incidental edits), so only the delta was re-bought. THE CORPUS IS OVERWHELMINGLY SINGLETONS AND OVERWHELMINGLY UNBLOCKED — 27 well-formed unblocked stated-contract cuts, which the prior record's group-first framing understated by reporting only the multi-member groups. Blocked, each on a ground read rather than inferred: context-kit §Testing (283) by its own SPEC's "that group is blocked as a whole by index-tests/toolfloor-cases.sh"; context-kit §bin/env-probe (199) dragged by toolfloor.sh; installer/README.md §The install boundary (1184, a legitimate cut contract since the 2026-08-31 operator ruling) ordered behind platform-support-ci-matrix's Windows leg; gate-sdk §lib/test-hermetic.sh (37) behind hermetic-bin-suffix-pin-placement; gate-sdk §lib/inject.sh (80) structurally uncuttable, three sourcers in three sections. Three owed files still have NO specification owner (demo/run-demo.sh, scripts/pack-installer.sh, installer/bin/checkwright.sh all declare CLAUDE.md §Housekeeping); checkwright.sh's header is now confirmed stale by direct read rather than inferred, TASK-QUEUE.md stating "bin/checkwright.sh collapses into the bootstrap". THE RANKING IS LEVERAGE AND SHARED SURFACE, NEVER SIZE, a size-ordered composer being refused: gate-sdk §lib/declaration.sh (59) is the one cut whose unblocking the LAST iteration BOUGHT and left uncashed, and its compiled counterpart already exists (native/src/declaration.rs); lifecycle-kit §bin/install-lifecycle.sh (36) and doctrine-kit §install-doctrine (194) are two of inject.sh's three shell sourcers, taking it from three blockers to one, the third being installer-blocked; doctrine-kit §stage-rules (51) and lifecycle-kit §bin/session-id.sh (75) pair with those two on their own kits' surfaces, and session-id.sh additionally has enter-stage.sh (617, the corpus's largest) as its only owed caller. THE ONE RISK IN THAT SET WAS PROBED AND DISSOLVED: porting two inject.sh sourcers does not duplicate inject.sh in-crate, because native/src/marker.rs already says in its own header "The shell library keeps its own copy for its remaining shell callers; this is the compiled counterpart, not its retirement". ECONOMICS, measured off drift-kit/bin/stage-economics.sh rather than assumed: the last three port iterations cost 105, 131 and 133 dollars and retired 2, 2 and 3 files, so roughly 44 dollars a file against a fixed per-iteration floor near 60 — which is the arithmetic arguing a WIDER bundle, packaging being explicitly free under the 2026-08-30 ruling, and which prices the remaining 48 owed at roughly 16 iterations and 2000 dollars at the observed rate. TWO RULE AMBIGUITIES REMAIN UNRULED, both re-escalated unchanged from the prior record and verified open by a zero-diff TRAJECTORY.md since that rev: whether a section whose remainder is externally sequenced may be cut partially, and whether the two consumer-side parser plugins on evidence-kit's EVIDENCE_KIT_PARSER seam port at all — 66 owed lines the completion predicate counts and nothing says how to discharge. Every scripts/ candidate was held out of the recommendation for that second reason.

## 2026-09-03 spec — Which queue entries qualify as [spec:] hosts for the five port-cut subjects, and how many tags does each host's lead line admit?
- corpus: TASK-QUEUE.md (9373 lines, all sections) + gate-sdk/SPEC.md 1956-2010 + queue-kit/SPEC.md The tag algebra + canon-kit/SPEC.md check-amendment-queue
- oracle: grep -n over each basename and contract-section spelling, each hit mapped to its enclosing '^- \*\*' lead line and '^## ' section; lead-line widths measured directly; native/src/gates/amendment_queue.rs read for the parser's arity
- rev: 0d67bfbf7da9a9d3e80e1f965aa84f25693f5d3b
- edges: gate-sdk/SPEC.md's one-tag rule is grounded ONLY on the composer entry's 66 fixed columns and is stated as if general
- finding: Exactly TWO entries qualify as hosts for any of the five subjects: kit-library-port-residue (declaration.sh as an owned member; install-lifecycle.sh and install-doctrine.sh in one sentence as inject.sh's blockers) and the composer native-gate-port-remaining-corpus. session-id.sh has NO qualifying host; stage-rules.sh is named NOWHERE in the queue. Every other grep hit is a passing mention or a slug coincidence. The parser takes a Vec of refs per line, so at-most-one-tag is a WIDTH constraint, not a parser one, and the width is per-entry: kit-library-port-residue's fixed lead-line part is 30 columns, not 63, because trailing prose reflows and a tags-only lead line is legal.

## 2026-09-03 align — Do the three declaration-install-and-stage-helper-cuts amendments (SPEC-declaration-cut.md, SPEC-install-lifecycle.md, SPEC-stage-rules-cut.md) hold up against themselves and against the live tree?
- corpus: the three amendment files in full, plus every file/line/count each cites: gate-sdk/SPEC.md, lifecycle-kit/SPEC.md, doctrine-kit/SPEC.md, context-kit/SPEC.md, TASK-QUEUE.md lines 15-118, native/src/{main.rs,stages.rs,marker.rs,gates/{merge_attrs,lifecycle_registration}.rs}, .claude/settings.json, docs/ mirrors, installer/README.md, gate-sdk/lib/gate.sh, smoke/install.sh
- oracle: direct reads/greps of every cited file/line plus check-queue-wrap, check-queue-entry-budget arithmetic (wc -c/awk length), --emit port-blockers --tree, three isolated audit-sweep subagents (one per cut) each checking a numbered claim list
- rev: 6295da2be1854500ed87d694ba935e15704c201f
- edges: each amendment's basename/column arithmetic, its 'Existing sections updated' roster vs its own deltas, and its docs-mirror-path claims were the edges that actually diverged; the substantive design claims (criterion-6 collapse, argv distinguishing test, inject.sh two-sourcer correction, dropped-members-still-owed) all held
- finding: 27 checkable tree-facing claims verified (9 per cut A, 11 per cut B, 9 per cut E — one item audited on both A and B/E's overlap), 3 real defects found and fixed, all narrative/arithmetic and none changing a ruling, a Definition of Done item, or a demote/host verdict: (1) install-lifecycle's roster still said 'one line of headroom' after delta 11 corrected the figure to zero; (2) stage-rules-cut's basename arithmetic used 22 chars/97 cols for its own filename where wc -c reads 23/98 (verdict unchanged, still under the 100-col cap); (3) stage-rules-cut cited 'docs/doctrine-kit/index.md' three times as the mirror carrying the invocation text where the actual mirror is 'docs/doctrine-kit/README.md' (index.md is a distinct hand-authored nav page with no such text); install-lifecycle's 'six further sites' in lifecycle-kit/SPEC.md undercounted its own roster by one (the set -e caller sentence at SPEC.md:1719); and an unsupported 'second worked instance' ordinal on the --install <op> refusal claim, with no first instance findable anywhere in the tree, was dropped rather than asserted a number for. Landed at 6295da2b.
