# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.






















































## 2026-09-03 scope — Which spec section's owed shell files are takeable as the next port cut, and which are sequenced behind a named unit?
- corpus: the 43 files the tree oracle reports owed, joined to each file's own '# spec:' declared section (tracked non-test *.sh)
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 10351659b174f070b8d1259da64d12882f8968e3
- edges: native-gate-port-remaining-corpus 8, platform-support-ci-matrix 12, powershell-installer-surface 6, kit-library-port-residue 0; the five recurrence-threshold deferred entries sum 1,1,1,0,0
- finding: 17 of 43 owed files are SEQUENCED, and 15 of those trace to one unit. Behind powershell-installer-surface's behind-invoke relocation: installer/lib/*.sh (9), doctrine-kit/bin/install-doctrine.sh, gate-sdk/lib/inject.sh (behind install-doctrine.sh), context-kit/lib/toolfloor.sh, and context-kit SPEC section Testing's three members (run-index-tests.sh, toolfloor-cases.sh, smoke/agents-md.sh), which that section declares blocked as a whole because toolfloor-cases.sh sources toolfloor.sh. Beside them installer/bin/checkwright.sh collapses into the bootstrap, and gate-sdk/lib/test-hermetic.sh waits on the live defect hermetic-bin-suffix-pin-placement. The remaining 26 are unblocked, every one in a singleton or near-singleton section, so no group amortizes a walk and the stated-contract composer selects on shape rather than on size. TWO SPEC-VERSUS-ORACLE ALARMS WERE RAISED AND BOTH DISSOLVED ON PROBE: context-kit SPEC line 1296's 'The runner itself stays shell' is a sequencing sentence whose own clause names the blocked-as-a-whole group, not a permanent declaration, so the absent no-port tag is correct; and the toolfloor.sh sequencing sentence living in Layout and configuration rather than in bin/env-probe is exactly where the 2026-09-03 ruling already cites it. Neither is a defect and neither was filed. THE CHAIN WORTH CARRYING: installer README line 494 sequences the relocation's unconditional remainder behind the artifact roster covering every supported platform, whose entry's remaining consequences wait on a first-observed-green Windows leg, red on one cause for rounds 7 through 11.
