# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-26 scope — How many words does each gate-sdk/SPEC.md subsection left on spec-brevity-residue carry, and which cut makes a slice near the prior slice's size?
- corpus: gate-sdk/SPEC.md
- oracle: awk '/^###? /{if(h)print h, w; h=$0; w=0} {w+=NF} END{print h, w}' gate-sdk/SPEC.md
- rev: d219c61ef0d71142c55ca28767945df5113855a6
- finding: At d219c61e the file holds 174455 words. Left on the residue: The non-gate arm 6612, The port-candidate criteria 7035, and Per-component contracts about 107k across 61 subsections, the largest run-gates 7960, port-blockers 7952, check-gate-substrate-parity 8616, lib/gate.sh 5814, upgrade-smoke 5368. The native-substrate cut (non-gate arm, port-candidate criteria, port-blockers 7952, build-native 1661, check-crate-arms 2410) totals about 25.7k, beside the porting-records slice's about 27k.
- inferred: none
