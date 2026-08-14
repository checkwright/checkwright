# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.















## 2026-08-14 scope — Does the corpus-derivation cohort rule still yield a takeable multi-member cohort for the ninth port cohort?
- corpus: scripts/gates.list */checks scripts/check-*.sh native/src
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: af9c22ee55cfb9407d9443c8ab0fe6f930ded737
- finding: No. 47 groups form. Group 1 (14 members, key libs=fail_closed globs=-) is operator-ruled 2026-08-14 NOT a cohort. Group 2 (2) loses check-install-disposition to meta-gate conservation; group 3 (2) loses check-docs-render-fidelity to a ruby criterion-7 hold. Groups 4-47 are 44 singletons. Largest takeable derivation group is therefore 1 member, and the SPEC states no fallback for the degenerate case. The rule's documented override still applies: a cohort retiring a blocker several later cohorts queue behind outranks a larger one.

## 2026-08-14 scope — Which registered gates require jq, a program outside GATE_SDK_PROGRAM_FLOOR, and does porting them retire jq from the consumer floor?
- corpus: scripts/gates.list */checks scripts/check-*.sh installer/lib gate-sdk/lib/gate.sh
- oracle: bash gate-sdk/bin/port-blockers.sh
- rev: af9c22ee55cfb9407d9443c8ab0fe6f930ded737
- finding: Exactly four registered gates carry a jq criterion-7 requirement: check-installer-no-deps (scripts/, seam-blocked by consumer-gate-port-disposition), check-memory-off (context-kit, also held on criterion 2), check-settings-paths and check-settings-pins (context-kit). Porting all four retires jq from the gate battery but NOT from the consumer floor: installer/lib/init.sh lines 54,55,76,77,78 shell to jq -r with 2>/dev/null, so a jq-less machine silently gets empty version, commit and lock values; diff.sh, doctor.sh, uninstall.sh and common/lock.sh do the same, and guard-kit lib/guard.sh has the filed guard-advise-jq-dependency instance of the identical silent-degradation shape.
