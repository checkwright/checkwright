# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.



















































## 2026-09-01 scope — Which owed shell files group behind one stated-contract section, and which of those groups is takeable now?
- corpus: tracked non-test *.sh (the port-blockers --tree corpus) x every kit SPEC.md section
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 9845f2c350a93a29ed055138da5826537783a4e1
- edges: native-gate-port-remaining-corpus 7
- finding: 58 owed. Only five groups of 2+ owed files sit behind ONE stated-contract section; every other owed file is a singleton or is already governed elsewhere. Membership was read off each file's own '# spec:' header, which is authoritative over inference. (1) context-kit SS-Index-first reading: md-index.sh 85, md-section.sh 63, pub-index.sh 79, lib/pub-lang/rust.sh 26, lib/pub-lang/ts.sh 32 = 5 files, 285 lines. All five declare that section in their own headers. Clause (1) is live and this is the corpus's cleanest instance: pub-index.sh is a dispatcher over a consumer-first registry (CONTEXT_KIT_PUB_LANG_DIR shadowing, CONTEXT_KIT_PUB_LANGS derived from the shipped roster at run time), so the seam survives and only the two bundled extractors move in-crate, the disposition kit-library-port-residue already states is ruled for them. Taking it discharges 2 of that entry's 6 members, which it says wait on this exact resolver. lib/context.sh stays no-port as the sole CONTEXT_KIT_PRUNE_DIRS resolver, so the cut reads it across the bridge exactly as the queue-kit cut settled. PROBED: the crate already holds section.rs (heading-bounded section walk) and walk.rs (pruned walk over the bridged prune set), which are md-section.sh's and both walkers' hardest mechanisms. (2) lifecycle-kit SS-The survey record: file-survey.sh 69, cite-survey.sh 80 = 2 files, 149 lines. Both declare the section; no extension seam; no blocker found. PROBED: check-survey-record is already native (survey_record.rs), so the crate already parses the block grammar the two tools stamp. file-gap.sh is NOT a member -- it declares SS-The committed gap inbox. (3) context-kit SS-Testing: run-index-tests.sh 91, index-tests/toolfloor-cases.sh 49, smoke/agents-md.sh 139 = 3 files, 279 lines. BLOCKED as a whole section: toolfloor-cases.sh exercises lib/toolfloor.sh's floor predicate, and that library is sequenced behind the installer's behind-invoke relocation (powershell-installer-surface). PROBED: install_toolchain.rs holds only the docs roster parity, not the version/uncomparable predicate, so the crate cannot supply it today. (4) delegation-kit SS-Testing: run-usage-tests.sh 226, run-trend-tests.sh 63 -- clean, but inverts sequence, since both subjects (usage-verdict.sh, usage-trend.sh) are still shell singletons. (5) evidence-kit SS-Layout and configuration parser seam: parse-gates-log.sh 15, parse-installer-smoke-log.sh 51 = 66 lines -- clause (1) sensitive and its amendment is undesigned: how a compiled arm spells as an EVIDENCE_KIT_PARSER value (a command string) has no answer in tree. All 10 installer/ files sit behind uniquely-named README sections and are behind-invoke blocked; scripts/pack-installer.sh and demo/run-demo.sh both cite CLAUDE.md SS-Housekeeping, an omnibus section that fails 'the one amendment that section needs'.

## 2026-09-01 scope — Does the lifecycle-kit SS-The survey record cut port every implementation of that section's contract?
- corpus: lifecycle-kit/**/*.sh x the literal 'The survey record'
- oracle: grep -rn 'The survey record' --include=*.sh . ; bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 6f9fdcd276f66a4ada1bec66af94e46a166ed57c
- edges: none
- finding: NO -- the cut leaves a two-file residue, and one half of it is PERMANENT. Raised by the lead off headers and RUN here as a defect claim; it holds, and is sharper than stated. Four shell files implement this section. Two are the cut: bin/file-survey.sh (owed 69) and bin/cite-survey.sh (owed 80), both header-declaring the section. Two are not, and neither is reachable by this cut because the composer selects by STATED CONTRACT and each declares a different section in its own header. (1) bin/enter-stage.sh:610 implements the read trigger and the boundary truncation, and its header at line 2 declares SS-bin/enter-stage.sh; the file is owed at 617 lines and ports in a DIFFERENT cut. (2) lib/stages.sh:71 implements LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS, the no-retrieval-pointer surface list, and its header at line 2 declares SS-lib/stages.sh; that file is header-declared no-port under the 2026-08-30 kit-library class ruling, as the config bridge's sole resolver for the LIFECYCLE_KIT_* knobs, which is structural and not a sizing judgment. So this section's contract will NEVER be wholly in-crate while that class ruling stands, and the amendment must state the residue rather than imply the cut discharges the section. The assertion half is already ported: check-survey-record is native at native/src/gates/survey_record.rs. Two gate-tests also cite the section and are outside the port corpus by the *.test.sh suffix rule. THE GENERAL SHAPE, which outlives this cut: a stated-contract cut ports the files that DECLARE its section, not every file that IMPLEMENTS it, and the two sets come apart wherever a shared entry point or a config library carries one clause of another section's contract. A later cut selector should expect this rather than treat it as this section's peculiarity.
