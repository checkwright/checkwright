# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.







































## 2026-08-25 scope — Is the native port's tail actually blocked, and what does unblocking it cost?
- corpus: native/targets.list; .github/workflows/publish.yml; scripts/pack-installer.sh; installer/lib/init.sh; TASK-QUEUE.md ## Deferred
- oracle: bash gate-sdk/bin/port-blockers.sh --tree; gh release view v0.25.0 --json assets; grep -n 'runs-on|matrix:' .github/workflows/
- rev: b2bc0ec588d3301211170818e96eba41efe324c4
- finding: BLOCKED, and the blocker is two lines per target. THE CHAIN, each hop probed not cited: platform-support-ci-matrix -> gate-binary-target-roster-widening -> powershell-installer-surface fork 2 (the middle hop is native/targets.list's own stated widening trigger; the last is the lead ruling recorded at TASK-QUEUE.md:1050 under criterion 5's omit-and-declare branch). FOUR premises re-verified, TWO of them now FALSE. (1) FALSE: platform-support-ci-matrix says 'no matrix: key anywhere, every leg ubuntu-latest'. publish.yml:75-123 is a roster-driven build matrix with runs-on: matrix.runner, a roster-to-runner map at :54-56, per-target upload at :116-123, a merge-and-normalize step at :133-176, and pack-installer.sh:143-148 loops the roster into payload/artifact/<target>/. The multi-target pipeline is BUILT and exercised; widening is ONE targets.list line plus ONE runner-map line per triple. (2) FALSE-BY-FIRING: the same entry's second un-defer trigger is 'native-gate-binary-port reaching distribution ... the release boundary'. Release v0.25.0 (2026-08-23) publishes checkwright-gates-x86_64-unknown-linux-gnu and its .sha256 sidecar. The trigger has fired; the entry is no longer demand-gated on that arm. (3) gate-binary-target-roster-widening's design fork -- cross-compilation versus a matrix of runners -- is settled in practice by the landed workflow, which is the matrix of runners; what remains open is which triples and whether each needs a native runner. (4) installer/lib/init.sh:98 target_of_host() already names FOUR triples (linux x86_64/aarch64, darwin x86_64/arm64) against a one-triple roster, so THREE host classes the installer can name take the omit-and-declare branch on every install today. SEPARATE AND UNRESOLVED: --tree reads 153/12/1/140 owed while TRAJECTORY.md PRIORITY DIRECTIVE says the tail is ONE member; the tail sentence describes the battery-plus-bootstrap sequence and the 140 is the tree predicate that section itself names as the completion oracle. Two readings of one word, escalated rather than resolved here.

## 2026-08-25 build — What would a per-platform CI leg cost, and can any triple beyond x86_64-unknown-linux-gnu satisfy the roster's join criterion today?
- corpus: GitHub-hosted runner labels; the macos-15-arm64 image manifest; .github/workflows/*; installer/consumer-smoke/run-smoke.sh; .workflow/validate-baseline.txt
- oracle: actions/runner-images README + image manifest via WebFetch; gh repo view --json visibility; a full local run of 'bash installer/consumer-smoke/run-smoke.sh' on a clean tree
- rev: b1487df0831753e0205ebb9736d3bfa2987769b5
- finding: Runners exist and are free (repo is PUBLIC): ubuntu-latest, ubuntu-24.04-arm, macos-latest/macos-26 (arm64), macos-15-intel/macos-26-intel (x64); macos-13 is RETIRED and no longer a label. The macos-15-arm64 image ships bash 3.2.57, jq, node, git and cargo/rustc 1.97.1 preinstalled, Homebrew — and NO GNU coreutils and NO gawk, so a macOS leg must brew-install bash/coreutils/gawk and PATH-order them, which is precisely the adopter action docs/install.md Requirements documents and nothing has ever run. No CI job on any platform runs an install smoke today (gates.yml is one ubuntu-latest job: kramdown gem, build-native, battery, derived fixture suites, guard tests). A clean-tree local run of the installer smoke passes every profile, the download arm, the toolchain-free arm and the jq-less arm, and fails at exactly one place: the binary-less leg, on 'run-gates: scripts/gates.list names no gates' — the ruled failure baselined at .workflow/validate-baseline.txt:93 against binary-less-dispatch-loop-retirement. So a naive CI leg running that script bare is RED on day one on the one platform that already has evidence. Zero triples satisfy gate-sdk/SPEC.md Consumer payload's join criterion today, because no green run has produced or exercised an artifact anywhere but x86_64-unknown-linux-gnu.
