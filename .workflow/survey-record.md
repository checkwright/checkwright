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
