# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

























## 2026-08-18 scope — Which port groups are takeable at the 2026-08-18 boundary, and which selection arm composes the next increment?
- corpus: gate-sdk/checks scripts/gates.list native/src canon-kit/checks context-kit/checks delegation-kit/checks doctrine-kit/checks drift-kit/checks evidence-kit/checks guard-kit/checks lifecycle-kit/checks queue-kit/checks site-kit/checks
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 1910f7a2b3e807fb6ac9829c61fa0bec12d2c58b
- finding: 104 members scanned, 74 already ported, 3 permanently shell, 27 owed — and the partition forms 27 groups of exactly one member each, 0 undecidable. The size arm (largest set sharing one corpus derivation) is therefore EXHAUSTED, not merely unattractive: no group has two members to amortize a walk across. No blocker-retiring override is visible either — the remaining singletons queue nothing behind them; the two standing prerequisites are check-tree-terms' criterion-4 hold and the '# port-until:' spelling, both owned by cohort-held-members-port-prerequisites. So the next increment composes by the BUDGET arm, whose precondition (a --group run reporting no takeable group) is met by this very run. Sizing caveat: gate-sdk/SPEC.md tells the sizing session to read a per-member shell line count off --group, and the tool prints no such column (filed as port-budget-sizing-input-absent), so a batch sized this iteration is sized on the criterion columns plus a hand read of each declaration.

## 2026-08-18 scope — How large is each of the 27 owed port members, the per-member cost the budget arm asks for and port-blockers does not print?
- corpus: gate-sdk/checks canon-kit/checks context-kit/checks delegation-kit/checks doctrine-kit/checks drift-kit/checks evidence-kit/checks guard-kit/checks lifecycle-kit/checks queue-kit/checks site-kit/checks
- oracle: bash guard-kit/bin/scratch-run.sh .tmp/owed-lines.sh (wc -l over each owed member's .sh; the script is scratch and dies at the next boundary reset, so re-derive it as: locate each owed member's checks/<name>.sh and count its lines)
- rev: 963af8000dd7bdcd7654a47cdab8cc54ac509f86
- finding: Owed shell totals ~3849 lines across 27 members, and the distribution is strongly skewed. Cheapest first: check-commit-msg 51, check-shellcheck 56, check-tree-terms 66, check-core-files 76, check-docs-link-convention 79, check-docs-cname-parity 81, check-gate-fail-closed 85, check-battery-roster 87, check-gate-fixture-coverage 93, check-kit-enum 95, check-exec-bit 96, check-gate-output 103, check-evidence-manifest 105, check-gate-exemption-tasks 106, check-gate-binary-fresh 107, check-amendment-queue 117, check-template-copy-parity 127, check-spec-embedded-source 134, check-identity 135, check-gate-tamper 140, check-gate-assertions 148, check-prose-tells 208, check-reads-couples 219, check-action-run-shell 222, check-knob-default-coupling 240, check-docs-render-fidelity 241, check-graph 632. Reading it against the criterion columns from the same cut: check-shellcheck and check-action-run-shell carry c7=shellcheck, check-docs-render-fidelity c7=ruby, check-gate-assertions c7=paste and c3=align-only, check-reads-couples and check-gate-binary-fresh c7=?, and check-tree-terms carries a criterion-4 hold that is design work rather than a port. Everything else is c7=clean. So the takeable cheap band is the nine clean members from 76 to 96 lines plus check-commit-msg at 51 — about 800 lines and no design fork among them — while check-graph alone is 632 lines and 27 library functions and is the corpus's single largest piece.
