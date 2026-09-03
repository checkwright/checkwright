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
