# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.






























## 2026-08-22 scope — Which governed public prose asserts the enforcement core's implementation substrate in a form the current port measurement falsifies or overstates?
- corpus: README.md, CONTRIBUTING.md, SECURITY.md, RELEASING.md, docs/*.md (hand-authored pages only; docs/<kit>/ is a generated mirror), every kit-root SPEC.md and README.md, installer/README.md
- oracle: bash gate-sdk/bin/port-blockers.sh --group (2026-08-22: 104 scanned, 96 ported, 3 permanently shell, 5 held) read against each candidate sentence; every hit re-verified first-hand at this scope with a whitespace-tolerant grep
- rev: 0fc1e509ea982a8396708ef3fb443df900fc0b9a
- finding: TWELVE findings across ELEVEN distinct files, not the three the gap bullets named. FALSE (9): README.md:9; docs/methodology.md:45; docs/positioning.md:48; docs/index.md:62; docs/install.md:83 ('every gate and both generated git hooks are Bash scripts'); docs/install.md:92 ('written in awk'); gate-sdk/SPEC.md:5 and gate-sdk/README.md:10 (the kit's own definition of its central noun -- 'A gate is a small shell script'); lifecycle-kit/SPEC.md:1912 ('an errored awk capture', for a gate ported to native/src/gates/lifecycle_registration.rs). OVERSTATED (4, reason stale but claim standing): gate-sdk/SPEC.md:210 ('every gate ... sources the library'); SECURITY.md:42,44 ('A vendored kit is bash' / 'execute that shell as you'); installer/README.md:26-27 ('the one language the rest of the tree is written in'); CONTRIBUTING.md:57-58, which overstates the OTHER way ('Gates in this tree dispatch to a compiled subcommand', unqualified, against 8 still-shell members). SURVIVES: docs/install.md:193-198, measured-marker-gated and correct. The four kit-SPEC/README sites carry generated docs/ mirror copies -- regen, never hand-edit. TWO DESIGN CONSTRAINTS the sweep bought and no bullet named. (1) The claims are LINE-WRAPPED: a first single-line grep pass failed to resolve docs/index.md:62, CONTRIBUTING.md:57 and installer/README.md:26 because each phrase spans a newline -- the same failure class as the lead's retracted hyphen miss, so a line-keyed scanner is blind by construction and the predicate must normalize whitespace across lines. (2) Against the proposed literal roster, 9 of 12 are caught and 3 are structurally missed: lifecycle-kit/SPEC.md:1912 (bare token 'awk', not a rostered phrase), SECURITY.md:42 (paraphrase), CONTRIBUTING.md:57 (the inverse overstatement, which no stale-shell roster can ever fire on). 'every gate' as a roster member is very noisy -- dozens of non-substrate uses -- and needs triage or exclusion.
