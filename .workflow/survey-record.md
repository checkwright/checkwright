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

## 2026-08-18 spec — Which of the ten cheap-band port candidates fail criterion 4, and which fixture pairs miss an arm of their own derivation?
- corpus: the ten cheap-band declarations: gate-sdk/checks/{check-commit-msg,check-core-files,check-gate-fail-closed,check-gate-fixture-coverage,check-kit-enum,check-exec-bit,check-gate-output}.sh, canon-kit/checks/check-docs-link-convention.sh, site-kit/checks/check-docs-cname-parity.sh, evidence-kit/checks/check-battery-roster.sh, plus each member's gate-tests/<name>/{good,bad}/ pair
- oracle: read each declaration's walk and each pair's case files; criterion 4's predicate is gate-sdk/SPEC.md §The port-candidate criteria (a registry member's declaration path lies inside the corpus the gate scans as content)
- rev: 70306d88f1e7c4eccc3da25638b306593871f87b
- finding: FIVE BIND: check-gate-fail-closed (corpus IS check-*.sh), check-gate-fixture-coverage, check-kit-enum, check-gate-output (each greps a resolved declaration path as text), and check-docs-cname-parity — the last a NEW finding, invisible to assertion C because couples=docs/CNAME is one literal file while its walk defaults to SITE_KIT_SCAN_ROOT='.' and greps every tracked file. FIVE CLEAR: check-commit-msg, check-core-files, check-docs-link-convention, check-battery-roster, check-exec-bit (index MODE only, never bytes). FIXTURE-ARM GAPS, and they do NOT track the criterion-4 verdict: check-exec-bit's pair carries zero .gate paths so the descriptor-non-executable rule is exercised by no case; check-gate-fixture-coverage has no .gate case carrying '# no-fixture:'; check-kit-enum has no .gate case and its multi-kit hand-list branch fires on no live group; check-gate-output has no .gate-with-no-crate case; check-docs-cname-parity's cases both pass an explicit scan root so the default whole-tree arm has none; check-commit-msg and check-core-files leave cheap arms untested. TWO CLAIMS PROBED FALSE: the kit-root predicate is directory existence (gate-sdk/lib/gate.sh _gate_kit_roots_derived), not a *.sh glob, so porting a kit's last shell gate cannot drop a kit root — four kits already carry zero .sh under checks/; and check-gate-fail-closed's 'Retired with cause' row is about that meta-gate's assertion ABOUT a ported member, not about its own portability, so it stays a batch candidate.

## 2026-08-18 spec — Which still-shell members would declare '# port-until:' on day one, and does each have a stated ground and a live owning queue entry?
- corpus: every still-shell registered member (port-blockers --group), each candidate's own SPEC section, and TASK-QUEUE.md's live sections
- oracle: bash gate-sdk/bin/port-blockers.sh --group and bash gate-sdk/bin/port-blockers.sh, cross-read against each member's SPEC hold prose and a TASK-QUEUE.md slug search
- rev: 70306d88f1e7c4eccc3da25638b306593871f87b
- finding: FIVE HOLDERS: check-tree-terms (criterion-4 hold, ground in the queue entry, owner cohort-held-members-port-prerequisites — the only member with a specific owner); check-action-run-shell and check-shellcheck (shellcheck on PATH); check-gate-assertions (paste); check-docs-render-fidelity (ruby, via SITE_KIT_RENDERER's first element — ground stated in site-kit/SPEC.md, NOT gate-sdk's). NEGATIVE RESULT: only check-tree-terms has a specific owning entry; the other four resolve to the umbrella slug cohort-held-members-port-prerequisites, which by its own text claims exactly this class — legitimate, but the anti-rot is coarse, since a member whose own blocker lands before the umbrella closes keeps a live-but-wrong declaration. SECOND NEGATIVE: check-shellcheck has the blocker and no gate-specific SPEC sentence grounding its own hold, so it owes one before it may declare. NOT HOLDERS, excluded by name: check-reads-couples and check-gate-binary-fresh — their c7=? is port-blockers' tokenizer failing on a command-position variable naming the crate's own binary, not a substrate gap, and no SPEC states a hold for either. ALREADY PERMANENT (# no-port:, excluded): check-install-disposition, check-gate-substrate-parity, check-crate-arms — matching the tool's '3 permanently shell and excluded'.

## 2026-08-18 close — Do this iteration's four captured knowledge-friction facts have durable homes, and is any top-level governed doc stale after the eight-gate port batch?
- corpus: the four .workflow/knowledge-friction.log entries against their claimed owner sections, plus CLAUDE.md, README.md, TRAJECTORY.md, RELEASING.md, docs/site-architecture.md, docs/install.md and ROADMAP.md
- oracle: read each claimed owner section and probe every falsifiable claim: .gate count vs the measured ported-gate-members marker, gates.list line count, context-kit/lib/toolfloor.sh PROBE_SET vs the docs toolchain block, native/Cargo.toml rust-version, and each ROADMAP.md bullet against its entry's roadmap-summary: declaration
- rev: 56b905dd6fce9d794d757a4ce3664d2a06d2353e
- finding: ALL FOUR facts already have homes -- site-kit/SPEC.md 120-133 (whole-tracked-tree corpus, landed at the check-docs-cname-parity port), gate-sdk/SPEC.md 1183-1196 (a config-selecting argument is deleted, not ported) and 1225-1237 (transitively stops at the universal layers), both landed at fb6b854e, and gate-sdk/SPEC.md 7048-7053 (stage before build), which PREDATES the capture -- so the fourth log line records a fact found at its own owner rather than a re-derivation. ZERO staleness across all seven docs: ported-gate-members=82 matches the .gate count against 104 registered, the toolchain block matches PROBE_SET and the 1.71 MSRV exactly, and all nine ROADMAP.md bullets match their roadmap-summary: strings verbatim.

## 2026-08-18 close — Which of this iteration's 59 prompting patterns are genuinely ungranted mechanics, and which are already-granted commands broken by decoration?
- corpus: the 686-line .workflow/prompt-friction.log for wide-budget-batch-and-hold-declaration, read against .claude/settings.json, .claude/settings.local.json and guard-kit/lib/guard.sh + scripts/bash-guard.sh
- oracle: per pattern at 2x or above: grep the real invocation lines out of the log to establish the SHAPE, test that shape against the committed allowlist's bare-match semantics, and check guard.sh for a rule that already steers it
- rev: 56b905dd6fce9d794d757a4ce3664d2a06d2353e
- finding: DECORATION, not absent grants, is still the dominant cause: grep, ls, git show/status/add/log, echo, mkdir, du, printf and df are all granted bare and every ranked instance was chained, piped or redirected -- habit, and no grant fixes it. SIX GENUINELY ABSENT grants, none previously named: awk (also absent from GUARD_KIT_RO_BINS), the mandated 'nohup ... & echo pid=... > <key>.run' launch idiom, the bare non-gate 'checkwright' arm while its '-gates' sibling is overlay-granted, chmod with a numeric mode, 'bash */checks/check-*.sh' not reaching the .gate descriptors gates now ship as, and git worktree (overlay-only). TWO RECURRENCES: python3 scratch execution 3 -> 25+ while the bash side ran cleanly through scratch-run.sh, and heredoc journal writes 21 -> ~69. compare-settings-allow.sh reports zero redundant and zero over-broad local entries across 115 overrides. Two guard mechanics establish most of the table: guard_rule_cat_file/find_glob bail on any of && || | & < >, and guard_rule_allowlist_chain only compares a chain's lead against committed entries carrying no glob.
