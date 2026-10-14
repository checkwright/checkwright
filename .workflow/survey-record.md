# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-14 scope — How do the generic guard rules' variant-limit rows split between grant rules, PowerShell-reader rows and bash block/steer rules?
- corpus: guard-kit/guard-tests/variant-limits.tsv
- oracle: grep -v '^#' guard-kit/guard-tests/variant-limits.tsv | cut -f2 | sort | uniq -c
- rev: ddb2e70f8f838fbf4a95fa7c2a6b771f732ba614
- finding: 116 fallthrough rows over 17 rules; 9 are ps- transforms (PowerShell reader); 9 are rule ro_pipeline, the one auto-allow-band grant in the table; the remaining 98 are bash block/steer rules, almost all the same five transforms (sq-lead, dq-lead, esc-lead, part-lead, wrap-command) plus -arg variants on allowlist_chain, git_c_root, pgrep_self_match
- inferred: none
