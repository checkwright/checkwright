# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-26 scope — How many words does each gate-sdk/SPEC.md subsection left on spec-brevity-residue carry, and which cut makes a slice near the prior slice's size?
- corpus: gate-sdk/SPEC.md
- oracle: awk '/^###? /{if(h)print h, w; h=$0; w=0} {w+=NF} END{print h, w}' gate-sdk/SPEC.md
- rev: d219c61ef0d71142c55ca28767945df5113855a6
- finding: At d219c61e the file holds 174455 words. Left on the residue: The non-gate arm 6612, The port-candidate criteria 7035, and Per-component contracts about 107k across 61 subsections, the largest run-gates 7960, port-blockers 7952, check-gate-substrate-parity 8616, lib/gate.sh 5814, upgrade-smoke 5368. The native-substrate cut (non-gate arm, port-candidate criteria, port-blockers 7952, build-native 1661, check-crate-arms 2410) totals about 25.7k, beside the porting-records slice's about 27k.
- inferred: none

## 2026-09-26 build — Which facts of gate-sdk/SPEC.md §port-blockers, §check-crate-arms and §build-native does a tracked file outside the SPEC cite, so a brevity pass must keep them?
- corpus: . ':!gate-sdk/SPEC.md' ':!docs/gate-sdk/SPEC.md'
- oracle: git grep -n -E '§(port-blockers|check-crate-arms|build-native)' -- ':!gate-sdk/SPEC.md' ':!docs/gate-sdk/SPEC.md'
- rev: 4be5bde2a5d0433962fad91c324c319896c28df4
- finding: Code spec: lines in native/src/bashscan.rs, emit/port_blockers.rs, walk.rs, gates/crate_arms.rs, gates/mod.rs, proc.rs, registry.rs, gate-sdk/bin/build-native.sh and gate-sdk/smoke/install.sh cite the tokenizer rules (here-string whole, double-bracket-scoped pop, EOF balance blind), the header-block disposition read and its fault projection, the grouping key factors, lines= placement, the crate-presence predicate, the source-stamp cache and worktree read, both arms running, the fixture arm and locator strip, the rustc empty field, the three declared programs, the no-walk-root corpus, the build's cwd resolution, consumer-case message, remap, verification, BN_TARGET and the port disposition. canon-kit/SPEC.md cites §port-blockers for command position; scripts/measured-claims.sh for the --tree trailer.
- inferred: none
