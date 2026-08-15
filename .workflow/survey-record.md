# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.
















## 2026-08-14 scope — After the ninth cohort, which still-shell registry members share a real corpus derivation — and does port-blockers --group see it?
- corpus: the 60 unported members of scripts/gates.list (104 registered, 44 ported)
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 1c157a12dd3f4a0d224382458f8375b695dfa85f
- finding: 45 groups: 1x14, 2x2, 42 singletons. The 14-member group (libs=fail_closed globs=-) is operator-ruled not a cohort; each 2-member group holds one member that stays shell. So the size arm is exhausted again and the tenth cohort needs the documented blocker-retiring override. BUT the tool's key is exact-set equality over libs, and it split one real shared derivation into three singletons: groups 8, 13 and 14 (check-release-bump, check-tightened-gates-note-parity, check-tightened-gates-grammar) all draw only from gate-sdk/lib/declaration.sh (decl_section_bullets, decl_section_tokens, decl_record_tokens, section_bullets, collect_dispositions) and all three walk docs/posts/*.md with the identical nullglob idiom; check-release-bump.sh:79 states the sharing in its own spec line. Union of the three keys = one 59-line library + one walk. All three are consumer-declared under scripts/. lib/declaration.sh keeps a consumer outside the family (gate-sdk/bin/upgrade-smoke.sh), so criterion 6 leaves it dual, as spec_manifest_files is.

## 2026-08-14 scope — What blocks each of the 13 consumer-declared gates (scripts/check-*.sh) from porting into native/, and which are takeable today?
- corpus: scripts/check-*.sh (13 members, all still shell — ls scripts/*.gate is empty)
- oracle: bash gate-sdk/bin/port-blockers.sh --group, filtered to the 13 scripts/ members
- rev: 1c157a12dd3f4a0d224382458f8375b695dfa85f
- finding: All 13 clear c2=pair and c3=precommit. Held on an unported emitter (3): check-trajectory-fresh, check-docs-mirror-fresh, check-value-rollup-fresh. c7 undetermined (1): check-docs-kit-parity. Takeable now (9): check-installer-no-deps (c7=jq; the reader is paid — native/Cargo.toml carries serde_json, MSRV 1.71), check-release-channel-parity, check-kit-ref-liveness, check-release-bump, check-tightened-gates-note-parity, check-tightened-gates-grammar, check-docs-nav-reachable, check-install-toolchain, check-npm-publish-spec. Shell line counts: 46/101/126/129/85/57/130/122/137/60/37/49/47. The tranche entry's own figures (63 of 103, then 62 of 104) are stale: 60 of 104 unported after the ninth cohort. What every one of them owes once, not per member: check-gate-substrate-parity assertion B prints the owning kit's gate_kit_roots basename and a crate unit test asserts owner/checks/name.gate exists — scripts/ is not a kit root and the descriptor would be scripts/name.gate, so the owner column has no representation for a consumer-declared member.

## 2026-08-15 align — After the decl-cohort + consumer-first-mover amendments, does gate-sdk's README roster, root README's kit table, the born-native convention, gate-sdk/SPEC.md's first-cohort claims, amendment-file hygiene, and the lifecycle stage roster still hold against the tree?
- corpus: gate-sdk/README.md gate-roster block vs gate-sdk/checks/; root README.md kit table vs top-level kit dirs; every .sh gate added since commit f1360f00 for a stated shell-stays cause; gate-sdk/SPEC.md 'first cohort' claims vs native/src/gates/mod.rs; every SPEC-*.md amendment file repo-wide; scripts/lifecycle-config.sh's LIFECYCLE_KIT_STAGES vs CLAUDE.md's stage list
- oracle: diff each README roster/table against its live corpus by grep/ls; git log for .sh gates added since f1360f00 and check each for a stated cause in gate-sdk/SPEC.md; grep native/src/gates/mod.rs for the first-cohort gates' registration; ls gate-sdk/SPEC-*.md and repo-wide SPEC-*.md; diff the two stage lists
- rev: f60d40c378fa616bd198c1a92423014eac8af6d2
- finding: Clean on all six: 33-gate README roster matches checks/ 1:1; 10-kit README table matches top-level dirs with no mismatch either direction; no .sh gate added since f1360f00, the two nearest predate it and both carry a stated cause; check-action-pinning/check-action-gh-repo match native/src/gates/mod.rs:74,81; exactly the two expected amendment files exist repo-wide, no strays; LIFECYCLE_KIT_STAGES=(scope spec align build validate close) matches CLAUDE.md verbatim. No drift found this iteration.
