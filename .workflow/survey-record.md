# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.





















































## 2026-09-03 scope — Which specification section is the next well-formed port cut, and which of its candidates are ready?
- corpus: every tracked non-test *.sh the port oracle reports owed, grouped by its own '# spec:' declaration
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: d18df7d6e2f3abb1b412b526ffae369a24fa7a28
- edges: native-gate-port-remaining-corpus 8, platform-support-ci-matrix 12, powershell-installer-surface 6, kit-library-port-residue 0, overhead-meter-measures-the-lead 0, iteration-scoping-clause-date-ambiguity 0
- finding: 45 owed over 109 scanned (64 no-port, 0 held). Grouped by owning section the corpus is almost all singletons: only FIVE sections hold more than one owed file. context-kit/SPEC.md §Testing (3 files) is out — its own text at context-kit/SPEC.md:1243-1246 declares that group 'blocked as a whole' by index-tests/toolfloor-cases.sh. installer/README.md holds nine one-file sections (init 416, uninstall 214, doctor 164, recipe 106, profile 91, lock 75, diff 66, update 34, digest 18) and is powershell-installer-surface's behind-invoke territory. CLAUDE.md §Housekeeping holds three (pack-installer 196, run-demo 96, checkwright.sh 53) and is not a kit SPEC section. That leaves context-kit §bin/env-probe (env-probe.sh 141 unblocked, toolfloor.sh 58 sequenced behind the installer relocation) and evidence-kit §Layout and configuration (the two parsers, 66 lines together). SIX candidates were surveyed for readiness against their owning sections and against native/src: ALL SIX READ READY, none NEEDS-RULING. Three findings a later stage should not re-buy. (i) lifecycle-kit/SPEC.md §bin/session-id.sh carries an explicit dated operator ruling — 'owed, unblocked and takeable, and deferred once for want of a host, ruled 2026-09-03, lead-relayed' — naming the drop from the last iteration as NOT a property of the file; it is the strongest-grounded candidate in the corpus and wants only a host. (ii) native/src/marker.rs::install_block already holds inject_marker_block's contract byte-for-byte, and native/src/gates/install_toolchain.rs already parses toolfloor.sh's roster as text, so an env-probe cut inherits both and owes only the floor-check verdict logic, tool-version probing and OS/package-manager detection. (iii) overhead-meter.sh and session-id.sh derive session8 DIFFERENTLY — the meter takes the basename's first 8 chars with no agent- strip and scans one flat tier, where session-id.sh strips agent- and scans two tiers, and drift-kit/SPEC.md documents the sibling stage-economics meter as using the STRIPPED form; the exposure is narrow (the default argument-less invocation only sees flat-tier bare-uuid names) but the two owning SPEC sections are silent on the divergence, so a cut on either file meets an unowned design question. A guard-kit §scratch-run cut carries an UNRECORDED cost the SPEC does not state: its whole security argument rests on a committed allowlist entry naming its fixed path, so a port changing the invocation shape needs a grant ADDED, which the 2026-08-29 carve-out does not cover (that carve-out is for grants a cut DELETES) and which is therefore operator-class under the 2026-08-22 bar.

## 2026-09-03 scope — Do this boundary's five carried gap bullets already have owners in the live queue?
- corpus: TASK-QUEUE.md live slug set (active, deferred, icebox) against the five bullets in .workflow/gap-inbox.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: d18df7d6e2f3abb1b412b526ffae369a24fa7a28
- edges: none
- finding: THREE of five already have owners, and two of those bullets assert the opposite in their own prose — so the capture-time matcher's honest limit is measured here rather than argued. The overhead-meter bullet asserts 'DISTINCT from every live entry: no queue entry names the overhead meter's session attribution'; overhead-meter-measures-the-lead (filed 2026-08-15) names exactly it, probes both halves the same way, and its design-pending open call IS the bullet's cross-kit question. The prompt-friction bullet is the THIRD arrival of prompt-ranking-ungrantable-shape-class, whose body already records a 2026-08-28 arrival that 're-derived this diagnosis without finding this entry'; same three arms declared wrong, same cat-append top row, same close-triage refusal text cited. The post-push bullet's load-bearing premise — that recording a post-push instrument reading costs a push — is the false dichotomy LEAD-RULED FALSE 2026-09-01 on observation-predicate-entry-cannot-drain-in-its-own-iteration, whose text says in as many words that a session meeting it next should not re-derive it. The generalisation for a later drain: a bullet's own DISTINCT-from claim is capture-speed prose and is worth exactly one grep against the live slug set, because the two false ones here each named a DIFFERENT sibling correctly while missing the entry that owns the class — the failure is retrieval of the owner, not carelessness about siblings. Only the archaeology-residue bullet and the iteration-scoping-clause bullet have no owner.

## 2026-09-03 align — Do SPEC-session-id-cut.md and SPEC-env-probe-cut.md's factual claims hold against the tree, and does either amendment's roster miss a cross-component surface the cut's deleted literal reaches?
- corpus: lifecycle-kit/SPEC-session-id-cut.md, context-kit/SPEC-env-probe-cut.md, and every tree surface either cites (settings.json, TASK-QUEUE.md, templates/, smoke/, gate-sdk/SPEC.md, context-kit/SPEC.md, native/src/gates/{docs_cmd,template_copy_parity,queue_entry_budget,release_bump}.rs, canon-kit/lib/spec.sh, scripts/canon-config.sh, installer/lib/{doctor.sh,common/recipe.sh}, index-tests/toolfloor-cases.sh) plus a tree-wide grep for both cut scripts' literal names
- oracle: bash gate-sdk/bin/run-gates.sh (full battery) plus per-claim literal probes (grep, wc -l, awk length, the gate binaries' own source read for each cited gate's exact scope)
- rev: 882e0b72fd88bcad9b5fec17daf9de23f73428ff
- edges: none
- finding: SPEC-env-probe-cut.md held on every probed claim — line counts (141/58), the settings.json grant count (1, sole line 38), the criterion-6 unless-clause and evidence-kit/SPEC.md §lib/evidence.sh citation (gate-sdk/SPEC.md:2057-2058), installer/lib/doctor.sh:66-70's four tool_floor_* call sites, recipe_seed's templates/*-config.sh glob (installer/lib/common/recipe.sh:14), check-template-copy-parity's four declared-surface classes (func/case/lib/knob — none matches a bare `bash "$RUN_GATES" --emit env-probe` line), the uncomparable verdict's two conditions (toolfloor.sh:37) and the sort::coreutils golden case (index-tests/toolfloor-cases.sh:20), and gate-sdk/SPEC.md §lib/inject.sh's still-current two-sourcer text (which the amendment correctly plans to correct at merge, deleting the false 'Windows leg' clause). SPEC-session-id-cut.md had three self-audit defects, all fixed in this repo's own tree at this rev (delta count now 13, was 12): (1) its DoD omitted an explicit bullet regenerating scripts/git-hooks/pre-commit and docs/check-graph.html, which delta (11)'s own prose names as owed and which no gate catches (no measured: marker exists for the tree-shell-owed key) — sibling amendment already carried this bullet; (2) delta (10)'s citation-site sentence wrongly named context-kit among the kits carrying a literal '§bin/session-id.sh' citation (grepped tree-wide: zero hits in context-kit) and overclaimed docs-mirror doubling for all seven listed sites when only the four SPEC.md sites are mirrored surfaces (docs/site-architecture.md §The on-site SPEC mirror is SPEC/README/DOCTRINE only — templates/, TASK-QUEUE.md and smoke/ are not mirrored); (3) context-kit/SPEC.md §The session-context hook names 'session-id.sh' five times in plain prose (lines 298-346, none formatted as a §-citation so none among the counted citation sites) describing lifecycle-kit's tool, untouched by either amendment's roster — added as delta (13) with its roster bullet. Every one of /spec's six batch pointers (toolfloor.sh staying owed/sequenced; the half-discharge of inject.sh's blocker set; the new --toolfloor-parity lane; the two staled generated-projections with no measured: marker; check-docs-cmd's and check-template-copy-parity's stated non-reach; the two preserve-behaviour hard calls; the host line-budget figures) verified true by direct probe rather than trusted — including the host figures, re-derived from check-queue-entry-budget's own counting algorithm (extent minus one line per present declaration grammar) rather than hand-counted: native-gate-port-remaining-corpus measures to exactly 50/50 (lines 15-65), kit-library-port-residue to exactly 48/50 (lines 66-114, 2 headroom). Full battery green (108/108) after the fixes, committed at this rev.
