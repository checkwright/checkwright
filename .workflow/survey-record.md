# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.














## 2026-08-14 spec — Which committed permission allow-list entries name a repo-relative .sh path that no longer resolves, and what shapes must a gate predicate NOT misfire on?
- corpus: .claude/settings.json permissions.allow (81 entries)
- oracle: jq -r '.permissions.allow[]?' .claude/settings.json, each literal .sh token tested with [[ -f ]]
- rev: 33cf820415e9bcfe544cd57be57f2a385636f591
- finding: 54 entries name a .sh path token; 46 are literal single paths, 44 resolve. 3 array entries name 2 absent paths: canon-kit/checks/check-comment-tier.sh (lines 58 and 59, bare and '*' twin) and lifecycle-kit/checks/check-stage-evidence.sh (line 46) — both native-port residue, .sh replaced by .gate, grant never repointed, accumulated one cohort apart. Five false-positive shapes a predicate must skip: 8 glob entries (*/checks/check-*.sh etc), env VAR=val bash <path> prefixing, trailing flags (gen-pre-commit.sh --write), ~35 bare non-path commands, and non-.sh path entries (.tmp/*, .workflow/*.log) whose absence is their ordinary state. jq is NOT on GATE_SDK_PROGRAM_FLOOR.

## 2026-08-14 spec — Where is scripts/pack-installer.sh invoked, how does each caller resolve its own root, and when in the caller's run does the call happen?
- corpus: whole tree: checks/, scripts/, installer/, .github/, smoke/, docs/
- oracle: grep -rn 'pack-installer' across the tree, each hit classified invocation vs prose
- rev: 33cf820415e9bcfe544cd57be57f2a385636f591
- finding: Five invocation sites, not 'every call site'. Four are in installer/consumer-smoke/run-smoke.sh (lines 64, 377, 478, 550), all using REPO resolved from BASH_SOURCE at lines 5-6; line 64 is early, 377/478/550 are minutes into the ~10-minute run (478 is the upgrade arm). The fifth is .github/workflows/publish.yml:174 (:173 is the INSTALLER_PACK_TMP_DIR= prefix line), cwd = GITHUB_WORKSPACE from a single fresh checkout, correct today. All other hits (RELEASING.md, CLAUDE.md, gate-sdk/SPEC.md x4, installer/README.md x4, docs mirror) are prose. KEY CORRECTION to the filed framing: pack-installer's dirty-worktree refusal sits EARLY within each invocation (line 44, before assembly/npm pack) — the lateness is the suite calling pack four times, not the check's position inside the script.

## 2026-08-14 spec — Does a bare corpus-primitive call site discriminate gates by corpus derivation, as the port grouping census assumed?
- corpus: all gate scripts under */checks/ and scripts/
- oracle: grep -rln 'gate_kit_roots' --include=*.sh */checks/ scripts/
- rev: 33cf820415e9bcfe544cd57be57f2a385636f591
- finding: NO — it over-selects at least as badly as couples=. gate_kit_roots alone has 10 gate callers spanning three components (canon-kit check-knob-default-coupling; gate-sdk check-gate-assertions, check-gate-binary-fresh, check-gate-fixture-coverage, check-gate-output, check-gate-substrate-parity, check-install-disposition, check-kit-enum, check-readme-roster, check-shellcheck; plus scripts/check-kit-ref-liveness.sh) which share no corpus. check-shellcheck.sh DOES call gate_kit_roots (line 22) — the scope claim that it calls no shared primitive is false; what is true is that it uses the call as a ROOT SOURCE then composes 4 fixed subdirs plus a *.sh glob, while port-blockers.sh composes the same call with 1 subdir. Same primitive, different corpus. Conclusion: the key must be set-equality over (kit-lib call set, content-glob set), not any single call.

## 2026-08-14 build — Does port-blockers.sh's tokenizer reach EOF in a clean state for the still-shell registry, and what partition does --group derive?
- corpus: the 63 still-shell members of scripts/gates.list, resolved through gate_resolve
- oracle: awk PB_SCAN with an END rule printing sq/dq/sp/hd per member; and bash gate-sdk/bin/port-blockers.sh --group
- rev: bb860dcad2ed1d20a51decf7ea45c30f6d6c8b0e
- finding: NO for 46 of 63. The dominant cause is the here-string: '<<<' is captured as a heredoc whose delimiter never recurs, so the scan skips the rest of the file. A second cause is ')' inside [[ ]] never popping the substitution frame. Both arms therefore under-report. The --group run at bb860dca reported 103 scanned, 42 groups, 3 undecidable, 40 already ported; largest group is 16 members keyed 'libs=fail_closed globs=-' whose couples= are 16 disjoint corpora (fail_closed is the fail-closed guard, gate-sdk/lib/gate.sh:23 — a kit-library call that derives no corpus), so it is a group sharing the ABSENCE of a derivation, not a cohort. Next largest are three 2-member groups: gate_msg_pattern_files (check-commit-msg + check-tree-terms; caller set empties at the port), gate_path_pruned+*.md (check-docs-render-fidelity is ruby-blocked), spec_canonical_specs (check-spec-dod-singleton + check-spec-derivable-section, both align-only). The partition itself is bounded by the truncation above, so no cohort should be cut off it until the tokenizer is repaired.

## 2026-08-14 build — What partition does --group derive once the tokenizer truncation is repaired, and does the repaired scan report an external-program blocker for any already-ported member?
- corpus: the 63 still-shell members of scripts/gates.list, plus each already-ported member's pre-port shell source recovered from history
- oracle: bash gate-sdk/bin/port-blockers.sh --group; and awk PB_SCAN over 'git show <deleting-commit>^:<kit>/checks/<member>.sh' for every .gate member, filtered against FUNCS/GATE_SDK_PROGRAM_FLOOR/builtins
- rev: f4cf49a71e8d1dc68a815e5090582edf50e17936
- finding: Repaired partition: 103 scanned, 47 groups, ZERO undecidable, 40 already ported. Supersedes the truncated run in the block above (42 groups, 3 undecidable) — that entry is kept because its numbers are the evidence the repair was bought on. The truncation had distorted the ranking itself, not just its completeness: group 1 shrinks from 16 to 14, and the pre-repair 2-member gate_msg_pattern_files group dissolves into two singletons once check-tree-terms' gate_path_pruned call becomes visible. On the repaired partition the largest group is still 'libs=fail_closed globs=-' (14, adjudicated not a cohort — fail_closed is the guard and derives no corpus), and the strongest real-derivation candidate is now check-install-disposition + check-readme-roster on 'libs=fail_closed,gate_kit_roots globs=*.gate,*.sh', both precommit/pair/clean, with criterion 4 binding on both since their corpus IS gate declarations. Ported-corpus check: NO already-ported member reports a genuine external-program blocker — all nine candidate rows are kit-library functions the port itself deleted from the shell libraries (queue_done_slugs, spec_comment_surface, spec_comment_surface_with_templates, spec_comment_whitelisted, spec_queue_slugs), so they resolve as functions at their own port rev and only look external against today's libs. check-measured-claim has no pre-port source, being born-native. No latent defect in shipped Rust from this angle.
