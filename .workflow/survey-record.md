# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-02 scope — Which deferred entries rank into this iteration's unit set, and which pairings among them are non-independent?
- corpus: 'TASK-QUEUE.md'
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 3d04dfd5f7c2f0150d3681c2798fd87a1c5db3d4
- finding: Tier 1 (session/iteration cost): spec-brevity-residue (session/high, surface installer, only installer §The consumer smoke left on that surface), compiled-consumer-smoke-driver (iteration/high, installer), heterogeneous-agent-delegation and manual-operation-spend-channel (iteration/low). Pre-emption: disposed-findings-register carries three recurrence dates against threshold 2. No first-tier entry has a blocking inbound edge; the retired block holds only shipped brevity slices and landed companion prerequisites, no premise argued from disposed work. Supersession: the installer §The consumer smoke brevity slice and kit-log-declaration-transport's consumer-smoke leg are both reshaped by compiled-consumer-smoke-driver (driver first); disposed-findings-register pairs with nothing (searched gap inbox, file-gap, discard, disposed, disposition record over Deferred and Icebox); config-variant-battery-harness's refusal concerns kit smoke scripts, independent. Queue-flow mean-filed 3.2.
- inferred: that compiled-consumer-smoke-driver fits one iteration beside two smaller features; that kit-log-declaration-transport's smoke leg is cheaper authored in the compiled driver than in run-smoke.sh plus a port

## 2026-10-02 scope — What reads or invokes installer/consumer-smoke/run-smoke.sh and run-smoke.ps1, i.e. what a compiled driver replacing them must rewire?
- corpus: 'native/src' 'scripts' '.github' 'installer' 'gate-sdk/SPEC.md'
- oracle: git grep -n run-smoke -- native/src scripts .github installer gate-sdk/SPEC.md
- rev: 3d04dfd5f7c2f0150d3681c2798fd87a1c5db3d4
- finding: Callers: .github/workflows/gates.yml (four bash run-smoke.sh legs and one run-smoke.ps1 leg), scripts/evidence-config.knobs (EVIDENCE_KIT_RUN_installer_smoke and EVIDENCE_KIT_PARSER_installer_smoke, the latter passing run-smoke.sh to --emit parse-smoke-log), native/src/emit/parse_smoke_log.rs (reads run-smoke.sh from the directory), native/src/emit/foreign_shells.rs (spawns bash run-smoke.sh), plus installer/SPEC.md and gate-sdk/SPEC.md prose. Sizes at this rev: run-smoke.sh 2194 lines, run-smoke.ps1 239. check-install-disposition reads each kit's smoke/install.sh, not these drivers.
- inferred: none

## 2026-10-03 build — What cites into installer/SPEC.md §The consumer smoke, and which of its facts does each citer need kept?
- corpus: 'native/src' 'native/build.rs' 'native/targets.list' '.github' 'installer' 'gate-sdk/SPEC.md' 'evidence-kit/SPEC.md' 'companion' 'context-kit/SPEC.md' 'CONTRIBUTING.md' 'scripts'
- oracle: git grep -n 'The consumer smoke' -- ':!docs' ':!TASK-QUEUE.md'; git grep -n 'truth table in installer/SPEC.md' native/src
- rev: f09ab4347aeea02bbfa4389fc0dfef7ac8e61ab3
- finding: No gate parses the body: check-spec-pointer needs the heading verbatim and unique, check-citation-link the #the-consumer-smoke anchor (companion/SPEC.md). report.rs prints 'the truth table in installer/SPEC.md §The consumer smoke'. 128 spec: tags in native/src/emit/installer_smoke, 37 in gates.yml, plus build.rs, proc.rs, targets.list, host-target.sh, each naming a paragraph's fact. Specific-fact citers: installer SPEC (§init's follow-up block grammar, withholding count, hooked move, newer-verb, narrowing, demo, manifest five values and truth table, whole-tree precondition), gate-sdk (artifact hand-off, harness stand-in and 'handed a directory it did not produce', roster steering and cross-build refusal, spawn set, check-tree-terms by name, seed arm, bash-less arm), evidence-kit (one scenario per arm, exit 1/2 invisible to parse-smoke-log), companion (companion arm, defect set covers the tested claim). Stale outside the section: CONTRIBUTING.md jq row, context-kit bash-floor construct list, gate-sdk binary-less-leg residual paragraph. The section misquoted artifact.rs's declared-target line.
- inferred: the gate-sdk 1058 citation gap is read from the text, not run
