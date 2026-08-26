# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.








































## 2026-08-26 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2), and does the stage template's prescribed anchored-grep oracle find all of them?
- corpus: TASK-QUEUE.md's Deferred section, lines 17-8297, 304 entries
- oracle: grep -n 'recurrence:' TASK-QUEUE.md, UNANCHORED, then count ISO dates per hit and confirm the line number is below the Icebox header
- rev: 4b35bc295be2d12e8c1bb3501f20459646ecab92
- finding: SEVEN entries are at threshold: relayed-ruling-provenance-unrecorded, dead-queue-citation-report, kfric-empty-log-ambiguity, absorbed-duplicate-disposition, validate-baseline-suite-coverage, pack-installer-vendors-untracked-scratch, isolated-child-liveness-hook-displaces-its-report. 26 recurrence declarations exist in all. CRITICAL SECOND FINDING: the anchored grep the stage template prescribes would have MISSED the seventh. TASK-QUEUE.md:7934 reads 'recurrence: 2026-08-25 2026-08-26' with NO slug, violating queue-kit/SPEC.md's grammar, and no gate validates it — the only reader, native/src/gates/queue_entry_budget.rs is_recurrence(), is a budget-discount heuristic that accepts the slugless form. Use an UNANCHORED grep until a gate exists.

## 2026-08-26 scope — Does citation-liveness-family-convergence's 2026-08-25 survey still hold, and is it citable at this boundary?
- corpus: TASK-QUEUE.md Deferred + Icebox, scripts/gates.list, native/src/gates/ — the survey's own corpus, re-run as a delta against its rev pin 457148bd
- oracle: git diff --stat 457148bd..HEAD over the three corpus halves, then grep each of the 14 named members for liveness, then read the native/src/gates/mod.rs delta for gate additions or removals
- rev: 8a53b2a64105de92eb79c09ef31c61c0ab09e804
- finding: WITNESS HOLDS — the survey is CITABLE at this boundary, delta-checked rather than assumed. Both corpus halves MOVED since the pin (TASK-QUEUE.md +953 lines, three files under native/src/gates/), so a blind citation would have been out of contract. The delta: all FOURTEEN named members still resolve live (12 Deferred + 2 Icebox); the mod.rs delta is ONE env knob added to check-stage-evidence's roster (LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE) with no gate added or removed; and none of the four gate-touch points (check-spec-pointer, check-queue-slug-liveness, check-docs-cmd, the guard rule-number island) changed. The 953-line queue churn removed no member. So the finding stands as written: 14 members collapse to 4 gate-touch points, size floor 8-10 new assertions plus 2 report outputs, all native-crate. Operator-ruled 2026-08-26 to stay DEFERRED in favour of the Windows CI leg; this record exists so the next boundary inherits a live witness instead of re-buying the sweep, which is what the entry's own cost line says re-buying costs.

## 2026-08-26 build — What in the consumer-smoke execution path breaks on a native Windows host under Git-for-Windows bash, and how far does GATE_SDK_NATIVE_TARGETS_FILE steering actually reach?
- corpus: installer/consumer-smoke/run-smoke.sh, scripts/pack-installer.sh, gate-sdk/bin/build-native.sh, gate-sdk/lib/gate.sh, installer/bin/checkwright.sh and all of installer/lib, read in full at build 2026-08-26
- oracle: Three commands, each cheap and each still the witness. One: grep -rn -e exe installer gate-sdk/lib/gate.sh gate-sdk/bin/build-native.sh native/build.rs returns no .exe handling. Two: git ls-files -s piped to grep for mode 120000 returns exactly one tracked symlink, under gate-sdk/gate-tests. Three: listing *.sh under any kit checks directory returns only gate-tests fixture doubles, so every production check is .gate-dispatched to the binary.
- rev: 0a764e5492dd1bd12493eb6a8afe59bf1bc3a037
- finding: TWO HARD BLOCKS, both source-side and neither reachable from CI configuration. (1) No .exe suffix anywhere: cargo emits checkwright-gates.exe on Windows while GATE_SDK_NATIVE_BIN names it suffix-less, so build-native.sh refuses right after a successful cargo build, and because every production check dispatches through gate_native_bin the omission blocks the whole battery on that platform rather than only the smoke preflight. (2) installer/lib/init.sh target_of_host maps Linux and Darwin alone, so a Windows consumer takes the omit-and-declare branch whatever the payload declares. STEERING REACH, measured: GATE_SDK_NATIVE_TARGETS_FILE reaches exactly two readers, run-smoke.sh's own roster preflight and pack-installer.sh's artifacts branch which copies the steered file verbatim into the payload. It reaches nothing inside the installed consumer, which reads the payload copy and derives its target from target_of_host alone, so the steered run does not stay coherent end to end. SETTLED SEPARATELY: the smoke never runs this repo's own battery, only the scratch consumer's, because all three run-gates.sh invocations sit behind a cd into the consumer dir. UNMEASURABLE WITHOUT A RUN, and therefore what the CI leg is for: the npm bin-shim shape on Windows and whether it satisfies a -x test, exec-bit semantics on freshly written shebang scripts, whether the one tracked symlink survives git archive piped to tar, the 260-char path ceiling, and whether cargo's remap-path-prefix matches a Windows-spelled CARGO_HOME.
