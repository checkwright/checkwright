# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-03 scope — Which deferred entries lead and fill the unit set at this boundary, and what guard-kit SPEC prose remains unpassed by the brevity moves?
- corpus: TASK-QUEUE.md guard-kit/SPEC.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges; ./native/target/release/checkwright-gates --emit md-section guard-kit/SPEC.md 'The generic ruleset' | wc -w
- rev: 53fde795ecbbcd72b35d0d3d393ecd1a54eb151a
- finding: Board of 52 deferred rows. First tier: spec-brevity-residue (session/high, surface guard-kit) leads; iteration/low heterogeneous-agent-delegation and manual-operation-spend-channel sit on other surfaces and lead the next set. Guard-kit fill: write-side-steering, compound-read-classifier-reach, bash-reader-escaped-pipe. Guard-kit unpassed words: preamble 249, generic ruleset 21017, Layout and configuration 1959, Testing 6662, Out of scope 70. No recurrence at threshold; mean-filed 2.8.
- inferred: none

## 2026-10-03 build — What facts do other surfaces cite into guard-kit/SPEC.md's preamble, §Layout and configuration, §Testing (with §check-guard-registration) and §Out of scope?
- corpus: guard-kit/ native/src/ scripts/ gate-sdk/SPEC.md
- oracle: git grep -n 'guard-kit/SPEC.md §\(Testing\|Layout and configuration\|Out of scope\|check-guard-registration\)' -- ':!docs/'; grep -n '(§\(Testing\|Layout and configuration\)' guard-kit/SPEC.md
- rev: 1c9c63f34291374ded13e4b66a734f9938bf3f84
- finding: 45 spec: or prose citations: §Testing from run_guard_tests.rs (three-valued exit, declared roster and omissions, sandbox preconditions, ladder order, substitutions, IFS grammar, payload tool_name, kit-root resolution), emit/mod.rs, guard_registration.rs, the three side tables, worktree-confinement.test.sh (measured isolated toolset), smoke/install.sh (cd_compound block, Bash|PowerShell matcher, wiring from a subdir), gate-sdk/SPEC.md:956, README:68 and both knob files (consumer lane); §Layout and configuration from host.rs, knobs/guard_kit.rs, rules/mod.rs (program-bearing exclusion), compare_settings_allow.rs (empty probe set) and the knob files; §check-guard-registration from guard_registration.rs and its gate/test; inside the SPEC, §Testing's firing/non-firing obligation and the smoke's template-grant check. Nothing cites the preamble or §Out of scope.
- inferred: none
