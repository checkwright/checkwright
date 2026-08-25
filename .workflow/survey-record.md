# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.







































## 2026-08-25 scope — Is the native port's tail actually blocked, and what does unblocking it cost?
- corpus: native/targets.list; .github/workflows/publish.yml; scripts/pack-installer.sh; installer/lib/init.sh; TASK-QUEUE.md ## Deferred
- oracle: bash gate-sdk/bin/port-blockers.sh --tree; gh release view v0.25.0 --json assets; grep -n 'runs-on|matrix:' .github/workflows/
- rev: bea34101ce3d1bfcf8f0d6d3233bd26c172e20c1
- finding: BLOCKED, and the blocker is two lines per target. THE CHAIN, each hop probed not cited: platform-support-ci-matrix -> gate-binary-target-roster-widening -> powershell-installer-surface fork 2 (the middle hop is native/targets.list's own stated widening trigger; the last is the lead ruling recorded at TASK-QUEUE.md:1050 under criterion 5's omit-and-declare branch). FOUR premises re-verified, TWO of them now FALSE. (1) FALSE: platform-support-ci-matrix says 'no matrix: key anywhere, every leg ubuntu-latest'. publish.yml:75-123 is a roster-driven build matrix with runs-on: matrix.runner, a roster-to-runner map at :54-56, per-target upload at :116-123, a merge-and-normalize step at :133-176, and pack-installer.sh:143-148 loops the roster into payload/artifact/<target>/. The multi-target pipeline is BUILT and exercised; widening is ONE targets.list line plus ONE runner-map line per triple. (2) FALSE-BY-FIRING: the same entry's second un-defer trigger is 'native-gate-binary-port reaching distribution ... the release boundary'. Release v0.25.0 (2026-08-23) publishes checkwright-gates-x86_64-unknown-linux-gnu and its .sha256 sidecar. The trigger has fired; the entry is no longer demand-gated on that arm. (3) gate-binary-target-roster-widening's design fork -- cross-compilation versus a matrix of runners -- is settled in practice by the landed workflow, which is the matrix of runners; what remains open is which triples and whether each needs a native runner. (4) installer/lib/init.sh:98 target_of_host() already names FOUR triples (linux x86_64/aarch64, darwin x86_64/arm64) against a one-triple roster, so THREE host classes the installer can name take the omit-and-declare branch on every install today. SEPARATE AND UNRESOLVED: --tree reads 153/12/1/140 owed while TRAJECTORY.md PRIORITY DIRECTIVE says the tail is ONE member; the tail sentence describes the battery-plus-bootstrap sequence and the 140 is the tree predicate that section itself names as the completion oracle. Two readings of one word, escalated rather than resolved here.
