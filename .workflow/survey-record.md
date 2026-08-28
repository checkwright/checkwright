# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.












































## 2026-08-28 scope — Does the 2026-08-27 port decomposition still hold at this boundary, and does its adopter-facing qualifier survive a probe?
- corpus: every tracked non-test .sh port-blockers.sh --tree walks (154 files), plus installer/profiles.list, installer/lib/common/profile.sh, scripts/pack-installer.sh, gate-sdk/lib/gate.sh
- oracle: bash gate-sdk/bin/port-blockers.sh --tree and bash gate-sdk/bin/port-blockers.sh --group, plus a read of the profile resolver
- rev: 62195fcb169d571b9c3c7664e4ed0a94937738c5
- finding: WITNESS HOLDS, ZERO DELTA: --tree reports 154 scanned, 12 no-port, 1 held, 141 owed, identical to rev d3361aa6, and an independent re-derivation matched the recorded by-kit split on all 14 kits. --group reports 107 scanned, 0 groups, 0 owed, 0 takeable, confirming the composer does not reach the tree remainder. TWO CORRECTIONS to the recorded block. (1) Its segment buckets sum to 146 not 141: scripts/ owed is 16 and the loose remainder is 19; the by-kit line is exact. Roles: bin 41 (5501 lines), lib 27 (5206), smoke 21 (2099), templates 20 (648), drift-kit/kpis 13 (558), scripts 16 (828), loose 3 (996). (2) Cut 2's premise that the instantiated scripts/ configs are declared no-port holds for only 6 of 10; scripts/context-config.sh, scripts/delegation-config.sh, scripts/drift-config.sh and scripts/evidence-config.sh carry no declaration and are owed, so cut 2 is 8 kit templates plus 4 undeclared scripts configs = 12 files, and the kit-template half is not covered by the 2026-08-24 provenance-seam cause at all because kit templates ride the payload while that cause is private vocabulary in a non-packed directory. THE QUALIFIER IS FALSIFIED. The recorded block says installer/profiles.list carries drift-kit in NO shipping profile and names three rosters. That reads the hand-authored rows alone and misses the fourth, derived profile. installer/lib/common/profile.sh line 9 sets PROFILE_DERIVED=full, lines 31 to 39 emit full from profile_names alongside every row profile, lines 41 to 46 resolve profile_kits full to profile_payload_kits, and lines 11 to 19 define that as every directory under payload/. scripts/pack-installer.sh lines 124 to 132 fill payload/ from gate_kit_roots_rel, and gate-sdk/lib/gate.sh lines 519 to 531 make any sibling carrying checks/ or smoke/ a kit root; drift-kit/smoke/ exists. installer/lib/init.sh lines 31 and 81 to 82 print and validate full as selectable, and docs/install.md line 274 tells adopters full is everything. So checkwright init --profile full vendors drift-kit and its 13 KPI plugins ARE adopter-facing. CONSEQUENCE: the contributor-side carve-out set shrinks from 46 files to the 33-file test and verification harness; the KPI cut moves onto the completion claim's critical path rather than off it; and the composer fork the 2026-08-27 lead ruling was left open on collapses, because largest-and-cleanest and adopter-facing now select the SAME cut. The claim was asserted independently twice, by that survey and by a fresh sweep this boundary, both by reading rows only, which is why it is recorded here with the resolver line numbers rather than as a conclusion.

## 2026-08-28 scope — Which live deferred entries does the rest of the queue converge on by inbound citation sum at this boundary, and what does the retired-target block say?
- corpus: TASK-QUEUE.md ## Deferred plus ## Icebox, every inbound citation resolved live-or-retired
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 62195fcb169d571b9c3c7664e4ed0a94937738c5
- finding: TOP LIVE TARGETS BY INBOUND SUM: platform-support-ci-matrix 10, prose-filename-citation-liveness 6, powershell-installer-surface 6, overlay-only-oracle-grants-uncommitted 4, then a band at 3 (unqualified-section-citation-liveness, session-model-identity-verification, scratch-citation-skill-surface-reach, retired-slug-live-pointer-citation, queue-entry-grammar-single-owner, native-gate-port-remaining-corpus, guard-steer-grant-mismatch, dispatch-claim-evidentiary-tier-unmarked, build-stage-tier-economics, benchmark-ab-experiment) and roughly forty at 2. powershell-installer-surface fell from 7 to 6 since the 2026-08-27 run, the retired gate-binary-target-roster-widening accounting for it. WHERE THE SUM CHANGES A RANKING, unchanged from the last run and still the only place: the citation-liveness family sums to about 18 inbound across members that individually rank low, and citation-liveness-family-convergence is where that dividend is already taken and legible. RETIRED-TARGET READ, an input and not a footnote: the heaviest retired targets are battery-runner-port 14 and check-spec-pointer 14, both shipped, so an entry arguing from either argues from disposed work; gap-resolver-mention-overcount 4, artifact-digest-mismatch-remedy-inert 4 and delegation-provenance-floor 4 follow. NO CANDIDATE WAS REFUSED ON A SUM this pass. The pass was owed and performed: this boundary is undirected, so the open question survey-edge-aggregation-residue carries, whether the pass is owed per ranked candidate when an operator supplies the unit set, does not fire here.

## 2026-08-28 spec — What does the native crate already carry for a port-blockers arm, and what does the port owe?
- corpus: native/src/**/*.rs (43.6k lines) plus gate-sdk/bin/port-blockers.sh, gate-sdk/lib/gate.sh, gate-sdk/bin/run-gates.sh
- oracle: read of native/src/emit/mod.rs, registry.rs, walk.rs, proc.rs, main.rs, gates/mod.rs, gates/gate_exemption_tasks.rs, gates/gate_fixture_coverage.rs plus exhaustive grep for a bash tokenizer
- rev: 4c8532004930f8ff794811f2a621628918a60b6f
- finding: REUSABLE AS-IS: the bridged-arm table native/src/emit/mod.rs:61 (const slice of (flag, Arm, knobs); Arm::Emit is fn(&[String])->Result<String,String> and main.rs:345-354 maps Ok to stdout/exit 0 and Err to stderr/exit 2, which covers port-blockers' whole exit contract); registry.rs:12-92 all pub (members, resolve, resolve_dirs, manifest_fields, field, expand_couples); walk.rs:10-30 prune resolution; walk.rs:35 pub fn tracked_shell_tree, byte-identical to the shell --tree corpus rule, sole caller gates/gate_exemption_tasks.rs:381; gates/mod.rs:1770 pub fn needs, in-process, replacing the --needs spawn; proc.rs spawn wrappers with three PRODUCTION bash spawns (evidence.rs:136, gates/graph.rs:376, runner.rs:732). NEEDS PROMOTION: header_block at gates/gate_exemption_tasks.rs:284, private, 12 lines, one call site; fixture-dir resolution at gates/gate_fixture_coverage.rs:8-16 plus an inlined dirs list at :33-41, which resolves ABSOLUTE kit roots where the report needs repo-relative ones, so a shared helper must take roots as a parameter. DOES NOT EXIST: no bash tokenizer of any kind (verified exhaustively; the nearest, reads_couples.rs:33 command_position, is a single-line byte scan with no quoting state), estimated 350-500 Rust lines against 179 lines of awk at port-blockers.sh:179-357; the disposition triple; a GATE_SDK_PROGRAM_FLOOR reader (grep over native/src returns nothing). TOTAL OWED ~825-1075 lines, landing near action_run_shell.rs (1052), the crate's largest. TWO FINDINGS THAT CHANGED THE DESIGN: (1) walk::tracked_shell_tree DEGRADES to Ok(empty) on a non-git tree (walk.rs:41-42) where --tree REFUSES exit 2 (port-blockers.sh:83-86), so reusing it verbatim would print 0 owed - the completion predicate - where the shell refuses; and it must NOT be changed, because check-gate-exemption-tasks requires the degrade by contract. (2) knob_program (port-blockers.sh:430-439) resolves an ARBITRARY knob discovered at scan time, which a static knob roster cannot express - answered by the union-sentinel precedent at gates/mod.rs:1725 (EVERY_FILTER_KNOB), whose stated ground is verbatim this case.

## 2026-08-28 spec — What in the tree must move when gate-sdk/bin/port-blockers.sh is deleted?
- corpus: every tracked file (git grep port-blockers, 24 files, 52 hits in gate-sdk/SPEC.md alone) plus every .gate descriptor, every # graph: manifest, scripts/core-files.list, scripts/gates.list, installer/, demo/, .github/
- oracle: git grep -n port-blockers; git grep -c; read of gate-sdk/smoke/install.sh, scripts/measured-claims.sh, scripts/pack-installer.sh, docs/site-architecture.md, canon-kit/checks/check-measured-claim.gate, gate-sdk/checks/check-gate-exemption-tasks.gate
- rev: 4c8532004930f8ff794811f2a621628918a60b6f
- finding: THREE INVOCATION SITES ONLY: scripts/measured-claims.sh:38 (the only machine reader, suffix-parses the --tree trailer's owed count), gate-sdk/smoke/install.sh:146/:182/:222, and two dead permission grants at .claude/settings.json:35-36. ZERO .gate descriptor and ZERO # graph: manifest names either file; every coupling is by glob, so canon-kit/checks/check-measured-claim.gate's kit:*.sh reaches the tool today and would NOT reach native/src after the port. VERIFIED ABSENT: demo/, installer/lib/, .github/, scripts/core-files.list, scripts/gates.list, scripts/git-hooks/pre-commit, every gate-tests fixture, Makefile. IT DOES RIDE THE PAYLOAD: scripts/pack-installer.sh:124-133 packs whole kit roots by tracked path with no per-file filter. SMOKE: one file, two blocks, gate-sdk/smoke/install.sh:129-171 (registry arms; plants a one-member registry and pins the tokenizer's two repaired shapes - a here-string at :142 and a ) inside [[ ]] at :138 with a genuine case pattern at :141 as the negative control) and :173-249 (--tree; six-file plant, exact trailer DELTAS at :236-243). NO fixture pair exists or is owed. STRUCTURALLY UNCOVERED: the .gate --needs path and its fail-closed branch, both arms' undecidable counters, knob_program resolution, the --group unkeyed row, all three exclusion counters, and both exit-2 refusals. CI BLIND SPOT: .github/workflows has zero hits and never runs the consumer smoke, so this coverage is validate-time only. check-gate-exemption-tasks: deletion is BENIGN (it asserts only that existing slugs are live) and DISCHARGES the entry's Done blocker - the file is the tree's ONLY live # port-until:. Generated fan-out: five kit SPEC/README mirrors, the two generated hooks (whose baked values are resolved by RUNNING the emitter), docs/check-graph.html, docs/enforcement.md, docs/value.md; docs/footprint.md verified NOT in it. gate-sdk/README.md:36-50 rosters the tool in a hand-authored bin/ list that check-readme-roster does NOT reach - that roster has no gate behind it. NO GATE CATCHES A MISSED PROSE PATH: check-docs-cmd reaches only FENCED repo-relative paths and this tool is cited in backticks throughout, so the ~14-file prose sweep must be enumerated rather than run.

## 2026-08-28 spec — Which of the 33 disputed test-harness files ship to an adopter?
- corpus: the 33 owed test-and-verification files (21 kit smoke/ members, 8 kit-resident test runners, 4 walkthrough drivers), plus scripts/pack-installer.sh, installer/package.json, installer/lib/init.sh, installer/lib/common/profile.sh
- oracle: read of the pack-and-vendor chain: pack_tracked archives each kit root WHOLE into payload/, installer/package.json's files roster ships payload/ and excludes consumer-smoke/, init.sh's vendor loop copies every payload file of a selected kit into the adopter's repo and commits it, and profile.sh derives full as every payload kit
- rev: a1984310d05559567cc6921b7d82c394ea0507cd
- finding: 31 of 33 SHIP: every kit smoke/ file and kit-resident runner rides the payload with its kit root and lands committed in adopter trees, so they are adopter-facing kit mechanism; only demo/run-demo.sh (demo/ is never packed) and installer/consumer-smoke/run-smoke.sh (absent from the npm files roster) ship to no adopter. This extends the drift-kit full-profile falsification to the smoke/runner classes and is the measured ground of the 2026-08-28 operator ruling's refusal of a contributor-side no-port class.

## 2026-08-28 close — Are the queue's retired-target inbound citations mostly tool false positives, and how many read as live pointers?
- corpus: All 76 retired targets and 170 citing pairs from queue-edges.sh over TASK-QUEUE.md, cross-checked against the queue's full git history
- oracle: bash queue-kit/bin/queue-edges.sh TASK-QUEUE.md, then git log -p --all -- TASK-QUEUE.md per target for the lead-line grammar
- rev: b8781386728e196d12f1a39cca0f59c0ae593cc8
- finding: THE FALSE-POSITIVE PREMISE IS FALSE — noise bucket EMPTY. All 76 retired targets genuinely held a top-level entry lead line with real tag grammar at some point. The sharpest counter-cases are the ones that most look like noise: check-spec-pointer was a [needs-spec] entry before it shipped and became a gate, and battery-runner-port / shell-gate-tail-port were entries that reached Done and were then RE-USED as the labels of the iterations they spawned, which is exactly why they read as bare iteration names to a reader who does not check history. queue-edges.sh's retired-set derivation is PRECISE; do not discount its output as over-matching. Of the 170 citing pairs, 166 are already qualified or benign — including the pervasive 'DISTINCT from X, whose subject is Y' idiom, which this corpus uses for live and retired siblings alike and which therefore misleads nobody, so a future sweep should not count it. Only 4 read as live pointers, all corrected 2026-08-28 at 1dd96bd5. No [blocked-by:] tag anywhere points at a retired slug — the highest-severity variant of the class is absent. WHAT A LATER STAGE SHOULD TAKE FROM THIS: the retired block is small-signal and high-noise-looking, so budget the read as a targeted check of ~4-6 candidates rather than a 170-line audit, and screen candidates by whether the citing prose lacks ANY disposal qualifier anywhere in its entry body rather than in the tool's truncated one-line snippet.
