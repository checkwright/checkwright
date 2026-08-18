# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

























## 2026-08-18 scope — Which port groups are takeable at the 2026-08-18 boundary, and which selection arm composes the next increment?
- corpus: gate-sdk/checks scripts/gates.list native/src canon-kit/checks context-kit/checks delegation-kit/checks doctrine-kit/checks drift-kit/checks evidence-kit/checks guard-kit/checks lifecycle-kit/checks queue-kit/checks site-kit/checks
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 1910f7a2b3e807fb6ac9829c61fa0bec12d2c58b
- finding: 104 members scanned, 74 already ported, 3 permanently shell, 27 owed — and the partition forms 27 groups of exactly one member each, 0 undecidable. The size arm (largest set sharing one corpus derivation) is therefore EXHAUSTED, not merely unattractive: no group has two members to amortize a walk across. No blocker-retiring override is visible either — the remaining singletons queue nothing behind them; the two standing prerequisites are check-tree-terms' criterion-4 hold and the '# port-until:' spelling, both owned by cohort-held-members-port-prerequisites. So the next increment composes by the BUDGET arm, whose precondition (a --group run reporting no takeable group) is met by this very run. Sizing caveat: gate-sdk/SPEC.md tells the sizing session to read a per-member shell line count off --group, and the tool prints no such column (filed as port-budget-sizing-input-absent), so a batch sized this iteration is sized on the criterion columns plus a hand read of each declaration.
