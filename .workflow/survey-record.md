# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.















































## 2026-08-30 scope — Which of the 102 owed port-blocker files are engineering ports and which are undecided dispositions?
- corpus: ':(top)' — every tracked non-test .sh the --tree arm scans
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 3b35e2c009966e627d6b3ca4b40c0a1559f62bb8
- edges: platform-support-ci-matrix 9, native-gate-port-remaining-corpus 8, prose-filename-citation-liveness 6, powershell-installer-surface 5, then a tail at 3
- finding: 102 owed of 139 scanned. 29 of the 102 (28 percent of the completion predicate) are DISPOSITION questions, not engineering: 18 kit templates/*.sh, 6 scripts/ installed copies of those same templates, 5 remaining scripts/*-config.sh. Both halves are already half-declared, which is what makes the class visible: drift-kit's two templates carry no-port while the other 18 do not, and 6 of 11 scripts/ configs carry the 2026-08-24 no-port cause while 5 of the same class do not. The queue's kit-config-template-port-disposition (10 files) and harness-template-port-disposition (17 files) together name 27 of the 29; the two my census adds are scripts/drift-config.sh and scripts/enum-sets.sh, which no entry reaches. The engineering remainder is dominated by lib and bin: guard.sh 1243, run-smoke.sh 851, gate.sh 709, enter-stage.sh 617, spec.sh 568, stage-economics.sh 465, init.sh 416, run-gates.sh 328. Smoke class probed clean: every smoke/install.sh and smoke/violation.sh that exists now carries no-port under the 2026-08-30 ruling; the one residue is context-kit/smoke/agents-md.sh, a third smoke member the ruling's letter does not reach, so widening to it is operator-class.

## 2026-08-30 spec — Which root call sites in this tree are dialect-exposed and uncrossed under gate-sdk/SPEC.md §The path-dialect contract, per substrate?
- corpus: every tracked shell source plus native/src/**/*.rs; four producer classes, not just show-toplevel: git rev-parse --show-toplevel / --git-dir / --git-common-dir, std::env::current_dir(), std::fs::canonicalize(), env!("CARGO_MANIFEST_DIR")
- oracle: grep -rn over the four producer forms, then per-site consumption tracing against the contract's predicate (two dispatched read-only sweeps, shell and Rust, plus my own verification of every count reported here)
- rev: 66b41e052e405c54f7d0f9896855056e7e255320
- edges: shell: installer/lib 5, gate-sdk/gate-tests 5, context-kit/bin 4; Rust: gates/ 16, emit/ 3; shared helpers with >1 caller: scan_root 3, fresh::toplevel 2, walk::kit_roots_abs 14, installer lock_path 4
- finding: 60 producer occurrences across 54 files (shell 33/32, Rust 27/22), of which 30 are dialect-exposed AND uncrossed today: 17 shell, 13 Rust. NOT ONE exposed shell site is crossed at production. Six shell sites consume the same root by BOTH cd and concatenation, so they need the two-line idiom. pwd -P is load-bearing and plain pwd is a no-op: bash's cd with an absolute arg sets PWD from the arg, only -P calls getcwd. Two in-tree sites already write the idiom (scripts/producer-liveness-reader.sh:16, scripts/pack-installer.sh:55-56), neither as a dialect measure. No shell dialect normalizer exists in any kit's lib/. Three contract clauses are falsified by the census: the declared dialect is per-substrate (walk::normalize_abs keeps the drive letter and must, since std::path cannot resolve /c/repo), consumption is local only WITHIN a file (installer/lib/common/lock.sh lock_path() concatenates whatever root it is handed and is the sole exposure path for installer/lib/update.sh), and cd && pwd crosses only with -P. Highest-severity single site: native/src/gates/memory_off.rs:31-34. Second normalizer found: native/src/gates/stage_evidence.rs norm() (lines 37-49) reimplements normalize_abs POSIX-only. All 8 non-walk.rs env!(CARGO_MANIFEST_DIR) sites are Path-wrapped and clear by construction; the 2 bare-&str ones (main.rs:375, gate_binary_fresh.rs:206) are test-only and -C-consumed. Two prose-only false positives a gate must not red: gate-sdk/gate-tests/check-graph-tree.test.sh:8 and check-template-copy-parity.test.sh:4 name the producer in a header comment.
