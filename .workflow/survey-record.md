# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-21 scope — Which Deferred entries lead this iteration's unit set under the pool ranking and the enhancement admission filter?
- corpus: TASK-QUEUE.md
- oracle: native/target/release/checkwright-gates --emit queue-edges
- rev: 1efc048ab9e588b88fb52ea037dfbe20c16ae304
- finding: Tier 1 (session/iteration class): only heterogeneous-agent-delegation, filter-excluded. Tier 2 (roadmap): companion-toolkit-profile, gate-authoring-sdk-surface, plugin-marketplace, benchmark-ab-experiment, hosted-attestation-service filter-excluded; design-partner-preview is operator hours. No recurrence at threshold 2. Surface rows: gate-sdk 19, canon-kit 12, queue-kit 11, lifecycle-kit 8, .github 8. gate-sdk admitted 16 (drop gate-authoring-sdk-surface, gate-tamper-exemption-reader-substrate, config-variant-battery-harness); 15 promoted, gap-capture-argv-prompt-friction escalated. Retired block: provenance and DISTINCT only. mean-filed 2.4.
- inferred: none

## 2026-09-21 scope — Which Deferred entries lead this iteration's unit set under the pool ranking and the enhancement admission filter?
- corpus: TASK-QUEUE.md
- oracle: native/target/release/checkwright-gates --emit queue-edges
- rev: e0e662b396fda3509c720bb038faf66d5f2ac161
- finding: Supersedes the rev-1efc048a block (queue touched since). Tier 1 (session/iteration class): only heterogeneous-agent-delegation, filter-excluded. Tier 2 (roadmap): all filter-excluded; design-partner-preview is operator hours. No recurrence at threshold 2. Surface rows: gate-sdk 19, canon-kit 12, queue-kit 11, lifecycle-kit 8, .github 8. gate-sdk: 15 promoted; held Deferred by the filter: gate-authoring-sdk-surface, gate-tamper-exemption-reader-substrate (blocked on it), config-variant-battery-harness, gap-capture-argv-prompt-friction (--from arm excluded). Retired block: provenance and DISTINCT only. mean-filed 2.4, drain 15.
- inferred: none

## 2026-09-21 spec — Which consumer-path literals do kit-shipped gate descriptors freeze in couples= and trigger=, by class?
- corpus: *.gate
- oracle: git ls-files '*.gate' | grep -v gate-tests | grep -v '^scripts/' | xargs head -qn1
- rev: 3c8e67a6eef33cedc0492d87b02f944ada6b34cb
- finding: Kit-shipped descriptors outside gate-tests and scripts: TASK-QUEUE.md 24 occurrences, gates-dir globs under scripts/ 26, scripts/gates.list 9, .workflow/* 6, CLAUDE.md 6, SPEC-*.md 5 plus SPEC.md 1, docs/ paths 14, .claude/ paths 10, named .workflow/ files 11, platform-fixed .github/ .gitignore .gitattributes SECURITY.md 18, kit-own template paths 3, the rest one each. 17 scripts/ descriptors are consumer-owned and out of the corpus. Classes and the per-member rule: gate-sdk/SPEC-doc-path-tokens.md.
- inferred: none

## 2026-09-21 spec — Where does a compiled gate's in-crate module coupling discriminate: transitive closure or one hop?
- corpus: native/src
- oracle: static read of crate:: use-crate and super:: references over native/src, closed per gate module under a stop set
- rev: 3c8e67a6eef33cedc0492d87b02f944ada6b34cb
- finding: Stop set walk.rs proc.rs registry.rs gates/mod.rs: transitive closure median 29 modules, max 36, a core of about 28 modules reached by 113 of 131 gate modules. One hop plus the direct imports of any emit/ module reached, with emit/mod.rs and knobs/mod.rs added to the stop set: median 2, max 7, and it reproduces check-value-rollup-fresh's hand-found set. 78 of 130 members with a descriptor name none of their one-hop set.
- inferred: none
