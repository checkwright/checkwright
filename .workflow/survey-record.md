# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-24 scope — Which deferred entries and carried bullets rank into this iteration's unit set?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: d276f025dc8b3643d7966eb1b879bbe94597b78e
- finding: Deferred and Icebox read with the three carried bullets of foreign-harness-write-path-pass' close filed in: no session/iteration cost class and no recurrence-threshold entry; mean-filed 1.2. The now-horizon roadmap entries wait on the observed install, not tree work. The set is npx-native-windows-route (feature), front-door-npm-visibility-wait and prove-front-door-restoration (debt); supersession none over install.md, page-length, front-door, checkwright-pwsh, checkwright.sh, prove, overlap, npx, npm, selection-probe.
- inferred: B1 shapes A (pwsh-7 dependency) and B (env-sh shebang reach) are both unmeasured on a Windows runner

## 2026-10-24 spec — Which surfaces read installer/bin/checkwright.sh's shebang, or name /bin/sh as the interpreter the bootstrap runs under?
- corpus: .
- oracle: git grep -n 'bin/checkwright.sh\|checkwright\.sh'; grep -n '/bin/sh' installer/SPEC.md .github/workflows/gates.yml; grep -rln shebang native/src
- rev: d100f51479ae5ddcb671ebf22e4b78e7a31aeb1c
- finding: Outside the docs mirrors and dated posts: only npm's bin link reads the shebang; every other route runs 'sh <path>' (docs/install.sh, docs/manual-install.md unix-install, installer/action.yml via install.sh, the gates.yml POSIX-bootstrap step, the consumer smoke's extracted entry). No crate code reads the bootstrap's shebang. installer/SPEC.md names /bin/sh for the bootstrap at §Implementation (two passages), §Requirements' What runs without bash, §Layout's bin/checkwright.sh bullet and §The consumer smoke's bash-less arm; gates.yml's Linux step 'name the shell the unix bootstrap runs under' prints readlink -f /bin/sh.
- inferred: none
