# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-19 scope — Which deferred entries lead and fill this iteration's unit set under the scope pool ranking and the enhancement admission filter?
- corpus: TASK-QUEUE.md native/src/installer
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges; bash gate-sdk/bin/run-gates.sh --emit queue-flow
- rev: f298358a2cf2159e9efb5f555437c4c145ccae71
- finding: Only session/iteration-class row is heterogeneous-agent-delegation (iteration/low), excluded by the enhancement admission filter: operator budget headroom is none of its three arms. No recurrence declaration carries two dates. Objective-advancing lead: floor-bash-install-bootstrap (installer). Installer-surface fill: hasher-shasum-fallback-unexercised, init-dry-run-plan-parity (agent-file arm (c) discharged: init.rs predicts it), install-smoke-intel-mac-bash-path-assert-vacuous (same macOS install-smoke legs); release-binary-archive-versioned-name joined by operator direction (bootstrap asset lookup). queue-flow mean-filed 3.6. queue-edges shows no hub among candidates.
- inferred: that init-dry-run-plan-parity divergences (a) and (b) still hold: dry_seed_paths in native/src/installer/init.rs predicts evidence-kit and lifecycle-kit seeds unconditionally while recipe.rs seeds them only when absent; no run on an installed consumer was made

## 2026-09-19 spec — Which tracked surfaces invoke or name the unix install bootstrap's language, and which read the Release's per-target gate binary names?
- corpus: installer .github/workflows docs/install.md context-kit/SPEC.md gate-sdk/SPEC.md RELEASING.md scripts/gate-tests/check-install-platforms
- oracle: git grep -n -i -E 'bash (half|bootstrap)|checkwright[.]sh'; git grep -n -F -e per-target -e 'gates-<target>'
- rev: 2aaf937bc64a222c3ba1093d2fb49790f34ebd44
- finding: Bootstrap invokers spelling bash: run-smoke.sh 917,933,964,1254,1302,1329,1331,1340,1506 and the bash-less arm entry at 1132; docs/install.md:341; installer/README.md:53; installer/SPEC.md:46. Language labels: installer/SPEC.md sections Implementation, Layout, The install boundary, The gate binary, The consumer smoke; checkwright.sh:2; checkwright.ps1:127; gates.yml:703; both check-install-platforms fixture headers; context-kit/SPEC.md:516-518; docs/install.md 161-167 and 305-313. Release per-target name readers: publish.yml pack loop and comments and release job comment, gate-sdk/SPEC.md Consumer payload Release paragraph, RELEASING.md procedure step 5, installer/SPEC.md gate-binary honest bound. No bootstrap, smoke or gate reads a Release asset.
- inferred: that the ubuntu-latest runner's /bin/sh is dash
