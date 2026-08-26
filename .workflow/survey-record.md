# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.









































## 2026-08-26 scope — Which deferred entries have reached the recurrence threshold, and which live entries carry the heaviest inbound-citation weight
- corpus: TASK-QUEUE.md ## Deferred
- oracle: bash queue-kit/bin/queue-edges.sh (inbound aggregation) plus a per-entry awk over recurrence: date counts
- rev: 493941ded53b36e03a8b44b74d3abf60f6f52c12
- finding: SIX entries stand at or above LIFECYCLE_KIT_RECURRENCE_THRESHOLD=2: relayed-ruling-provenance-unrecorded, dead-queue-citation-report, kfric-empty-log-ambiguity, absorbed-duplicate-disposition, validate-baseline-suite-coverage, isolated-child-liveness-hook-displaces-its-report. The last of these carries a SLUG-LESS declaration (recurrence: <dates>), so an anchored slug-form grep misses it and only a per-entry awk finds it, which is recurrence-declaration-grammar-ungated's subject exactly. INBOUND AGGREGATION, the second act of the survey-engagement obligation, run here rather than omitted: platform-support-ci-matrix 12, powershell-installer-surface 9, prose-filename-citation-liveness 6, gate-binary-target-roster-widening 5, then a band of twelve at 3. The Windows install chain carries 26 of those edges across three entries and converges on one upstream blocker, which is the heaviest convergence in the queue and is what makes it rankable ahead of the citation-liveness bundle despite that bundle retiring more slugs.

## 2026-08-26 scope — How large is the crate-portability blocker that stops the gate binary compiling for x86_64-pc-windows-msvc
- corpus: native/src
- oracle: grep -rn over all 139 .rs files for std::os::unix, PermissionsExt, MetadataExt, CommandExt, OsStrExt and symlink, then git merge-base --is-ancestor of each cfg-gate commit against the measured head 261231d2
- rev: 493941ded53b36e03a8b44b74d3abf60f6f52c12
- finding: ONE un-gated site, not four. gate-binary-target-roster-widening asserts a portability pass over four modules naming gate_binary_fresh.rs:13, proc.rs:69, proc.rs:337 and install.rs:165. THREE OF THOSE FOUR ARE ALREADY PORTABLE: proc.rs:67 and :335 and install.rs:163 each sit under a cfg(unix) with a cfg(not(unix)) twin beside it, and each cfg predates the Windows measurement (proc.rs at e3abf907 2026-08-23, install.rs at da6c6645 2026-08-25, both ancestors of the measured head). The entry's three-further-uses claim was therefore a grep extrapolation past the compiler's first error batch rather than a compiler finding. THE REAL SITE is gate_binary_fresh.rs:12-17, a local is_executable with no cfg duplicating proc.rs's already-portable one, so the fix is a de-duplication rather than a portability pass. HONEST LIMIT: a compiler halts at its first error batch, so only a real cargo check for the msvc target proves the crate builds; the census above found no other un-gated site but cannot prove absence, and no local cross-check is buyable because this machine has no rustup. A Linux cargo check for the msvc target does not link and would buy both the proof and a permanent regression gate.
