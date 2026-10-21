# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-21 scope — Which production std::env::temp_dir() sites does run-scratch-temp-residue route?
- corpus: native/src
- oracle: grep -rn 'temp_dir()' native/src
- rev: 97a8d0a3a2078a932cdeb35ed3780143775f9733
- finding: 10 sites in 7 files, a hit counted only above its file's first top-level #[cfg(test)], usage_tests.rs and the #![cfg(test)] settings_pins_parity.rs excluded: emit/front_end_parity.rs:617, emit/installer_smoke/masked.rs:270, emit/installer_smoke/roster.rs:178, emit/run_guard_tests.rs:485, emit/run_index_tests.rs:312, gates/action_run_shell.rs:194, proc.rs:528,738,1083,1250
- inferred: whether proc.rs:528/738/1083/1250 sit in a test-only item under its indented #[cfg(test)] at line 76 — read at build

## 2026-10-21 scope — How does the deferred pool rank for an undirected iteration, and which rows can land now?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 97a8d0a3a2078a932cdeb35ed3780143775f9733
- finding: 22 deferred rows; one session-class row (run-scratch-temp-residue, gate-sdk); none at the recurrence threshold (max 1 of 2); no shortlisted candidate has inbound edges. Event-gated now: tarball-attestation-observed, front-door-job-observed, codex-plugin-version-pin, companion-toolkit-profile (published tag, v0.32.0 deferred); design-partner-preview, external-gate-quality-evidence, benchmark-ab-experiment, hosted-attestation-service (external evidence or demand); codex-plugin-hook-discovery (codex-cli 0.163.0 hooks/list still empty beside plugin.json, probed); cursor-muse-plugin-observed (neither CLI on PATH); notification-delivery-probe (operator present). Landable: run-scratch-temp-residue, tmp-dir-prune-coupling (debt); manifest-count-blank-run-bound, releases-page-table, evaluator-root-agent-file, antigravity-hook-binding, record-stamp-encoding-compression, agent-selection-probe (features); smoke-whole-tree-precondition-unscoped and verify-workflow-decoupling (await a ruling); wait-loop-trailing-read-block premise falsified (date not in GUARD_KIT_RO_BINS).
- inferred: landability of each event-gated row beyond the probes named here

## 2026-10-21 build — Which shipped crate sites read the system temp dir after run-scratch-temp-residue?
- corpus: native/src
- oracle: cargo test --release --manifest-path native/Cargo.toml no_shipped_module_reads_the_system_temp_dir_outside_its_declared_sites
- rev: 35bcfac08fd25a25841ba9e2abc59f733381e89b
- finding: 4 sites, spelling env::temp_dir (the fn-pointer form included): proc.rs temp_base (the one routed read), emit/installer_smoke/roster.rs and masked.rs (installer/SPEC.md The consumer smoke), installer/demo.rs (installer/SPEC.md demo). The scope survey's 'temp_dir()' grep missed gates/fence_run.rs and installer/demo.rs, both unwrap_or_else(std::env::temp_dir).
- inferred: none
