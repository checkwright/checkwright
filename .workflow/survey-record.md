# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.
















































## 2026-08-30 scope — Does any deferred entry reach the recurrence pre-emption threshold, and is the one-date-per-declaration state a defect?
- corpus: TASK-QUEUE.md
- oracle: grep -n 'recurrence:' over the ## Deferred section, unanchored
- rev: 74018ceb47845d391c785f3f0fdbe9f159d85096
- edges: none
- finding: MEASURED: 29 declarations across 283 deferred entries, every one carrying exactly ONE date, so ZERO entries reach the threshold of 2. Zero-at-threshold is the EXPECTED steady state, not a defect: git history holds 28 multi-date declarations (up to four dates) and appending demonstrably works (2a101d24, 8708890b append rather than rewrite); an entry reaching two dates is pre-empted into a unit set, built, and leaves Deferred, so the rule consumes its own input. Traced end to end on boundary-wipe-preserve-lifetime-scope (stamped d6dee4ac, second date 679f2e9c, gone at b17dd45b). No live declaration is malformed, so the anchored/unanchored grep delta at HEAD is zero. ONE REAL EXCEPTION: dated-measurement-restatement-class attests a 2026-08-25 second instance in its own body that was never stamped (the observing close 2c312f23 wrote the prose and no declaration; the declaration was first created at 8708890b carrying only 2026-08-29), so its true date count is 2 and it SHOULD pre-empt. Distinct from recurrence-threshold-counts-dates-not-incidences (different calendar days, not same-day collapse), from recurrence-declaration-grammar-ungated (well-formed and fully visible to the prescribed oracle), from recurrence-resolver-literal-match-only and from recurrence-obligation-residency. Two false claims also found in the live recurrence-declaration-grammar-ungated entry: it misattributes its witness to 8a29e8ec when 2c312f23 created the slugless line and 8a29e8ec only extended it, and its cost line claims a live unreadable declaration when none survives.

## 2026-08-30 scope — Which deferred entries carry the most inbound citations, and does citation weight predict promotability?
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 85cd79a1612d7218f26fd684ed3a1c16668d31b7
- edges: platform-support-ci-matrix 10, native-gate-port-remaining-corpus 7, prose-filename-citation-liveness 6, powershell-installer-surface 5, then a flat 15-way tie at 3 and a 32-way tie at 2
- finding: MEASURED over 283 deferred entries: 135 live targets / 227 live edges; 77 retired targets / 166 retired edges. 157 entries (55.5 percent) carry ZERO inbound and 82 percent sit at 0 or 1, so any cut below rank 5 is arbitrary inside a tie. CITATION WEIGHT IS NOT A PROMOTABILITY PROXY HERE: a slug-token classifier hand-audited against a 1-in-7 sample puts the pool at roughly 30 percent product / 70 percent machinery (plus-or-minus 8 points, dominated by a ~42-entry ambiguous band), and the machinery 70 percent also dominates the ranking, so most high-cited entries are icebox-class by default under the 2026-08-30 ruling. Only 9 of 283 carry a roadmap tag. THE RANKING'S OWN BLIND SPOT: inbound rank buries convergence hubs. citation-liveness-family-convergence has ZERO inbound and 23 outbound while its cluster carries 41 live inbound edges across 21 entries — four times the nominal number one — and its witnessed survey finds the family is four gate touch-points rather than fourteen tickets. Cluster sums that beat any single entry: citation-liveness 41, Windows/platform 15-plus, native port 12-plus, wait/liveness 11-plus. RETIRED-BLOCK CAVEAT: the retired heuristic aliases slugs named after the gate they shipped — check-spec-pointer's 14 edges are all citations of a LIVE registered gate (scripts/gates.list) and must be discounted before ranking; battery-runner-port 13 and shell-gate-tail-port 12 are provenance citations, not premises. Only two live entries argue centrally from a retired premise: threshold-recurrence-routing-residency and close-eviction-refiles-without-checking.

## 2026-08-30 scope — How does the port's owed column decompose, and which cut moves the completion predicate fastest?
- corpus: TASK-QUEUE.md .github/workflows/gates.yml gate-sdk/ installer/ scripts/ native/
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 8181fc422ef06c2168034f1d8c63f283e9453a69
- edges: platform-support-ci-matrix 10, native-gate-port-remaining-corpus 7, powershell-installer-surface 5
- finding: MEASURED at rev 6e345565: 102 owed files / 12450 lines; 37 no-port / 3100 lines. By class: kit bin 38 files 5045 lines, kit lib 16/3588, installer 11/2088, kit templates 18/615, scripts 16/830, misc 3/284. Nothing owed under native/ or docs/. The battery is 108 of 108 ported and NO shell gate declaration exists anywhere in the tree, so the registry arm reads zero owed while the tree arm reads 102 — the two-axis misreading the oracle exists to prevent. ZERO of the 102 falls under the 2026-08-30 smoke ruling: it is fully discharged, every smoke recipe and both harness members already declared, so every remaining subtraction needs a new ruling or a real port. THE FASTEST CUT is the declaration cohort — config templates 11 files 113 lines, harness templates 17/965, kit libs 16/3588 — 44 files, 43 percent of the owed column, 37 percent of the lines, essentially no Rust, paired enforcement-first with no-port-cause-validation-scoped-to-registry so the 44 new headers are shape-checked. Its blocker is THREE separate rulings, not engineering, and each entry forbids averaging its ground. Highest independent value: guard-kit/lib/guard.sh alone, one file at 1243 lines (10 percent of owed lines) on a per-bash-call hot path, blocked by nothing. gate-sdk/bin plus lib is structurally LAST: lib/gate.sh is the sole knob resolver and criterion 6 forbids a second producer. installer/ (2088 lines) is blocked behind the Windows leg AND on an unresolved reading of installer/README.md §The install boundary, whose behind-invoke default never says the words no-port or port-blockers. PREMISE CORRECTIONS: kit-config-template-port-disposition names three scripts siblings where four are owed; harness-template-port-disposition says about 255 scripts lines where 374 are owed; kit-lib-port-disposition-cohort cites kit-smoke-port-disposition-cohort which no longer exists and its never-dispositioned headline is stale (two kit libs now carry declarations); powershell-installer-surface points at an empty ## Done; entry-compression-contract-unenforced says ruling-accretion-outgrows-the-entry-cap was dropped when it is live; native-gate-port-remaining-corpus sits AT the 50-line entry cap so any new ruling on it evicts grounds; born-native-flip-enforcement-gate's whole design premise (a new shell gate being indistinguishable from an unported one) has dissolved now that zero shell gates exist.

## 2026-08-30 build — Which shell sites derive a root with a non-crossing 'cd … && pwd', and which of them compose two roots by string arithmetic?
- corpus: every tracked *.sh / *.gate / *.yml in the tree
- oracle: grep -rn pwd --include=*.sh --include=*.gate --include=*.yml . filtered against 'pwd -P'; then grep -rn realpath over the same set
- rev: ef65956bf692b0745c9ce048a0ad418c6538e408
- edges: a fixture tree under gate-tests/check-path-dialect deliberately writes both a good cd-only form and a bad logical-readback form, so two hits are the gate's own corpus and not findings; test-hermetic.sh and the *.test.sh family sit outside every CI path that reaches Windows
- finding: About forty sites take the non-crossing 'cd … && pwd' shape and all are benign under the consumption predicate — they cd and read files under the result. Exactly one composes two roots by string arithmetic: gate-sdk/bin/gen-pre-commit.sh, whose 'realpath --relative-to' was also the tree's ONLY unguarded realpath (every other call site carries '2>/dev/null || printf'). That single intersection is the whole Windows exposure, which is why the repair is one script rather than a sweep.

## 2026-08-31 close — After the declaration cohort, how does the port's owed column decompose at HEAD, and what real-port throughput has the track shown since the completion predicate landed?
- corpus: every tracked non-test .sh (139 files) plus git history since 2026-08-14 for deleted non-test .sh
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree at 80035780; git log --diff-filter=D --numstat -- '*.sh' excluding *.test.sh
- rev: 80035780e0f08c815da3c8a82c7fb9690fbfe628
- edges: native-gate-port-remaining-corpus, installer-boundary-behind-invoke-port-reading, consumer-smoke-runner-port-disposition, harness-template-port-residue, kit-library-port-residue, platform-support-ci-matrix, powershell-installer-surface
- finding: MEASURED at 80035780: 76 owed files / 8689 lines; 63 no-port; 0 held. SUPERSEDES the 2026-08-30 decomposition (102/12450 at 6e345565): the declaration cohort retired 26 files / 3761 lines from the owed column by declaration, and NOT ONE LINE of kit bin/ moved. By class today: kit bin 38 files 5045 lines (58 percent, unchanged), installer runtime 10/1237, installer/consumer-smoke/run-smoke.sh 1/851, harness+hook templates and their scripts/ copies 12/613, kit lib 6/292, scripts/ 6/367, misc (demo, context-kit smoke and index-tests) 3/284. The three largest single files are run-smoke.sh 851, lifecycle-kit/bin/enter-stage.sh 617, drift-kit/bin/stage-economics.sh 464. gate-sdk/bin (run-gates 327, upgrade-smoke 239, run-gate-tests 188, build-native 110, install-hooks 59 = 923) stays structurally last behind lib/gate.sh's no-port sole-resolver status. THROUGHPUT: non-test shell retired per day peaked 2026-08-15..19 at 1126, 1527, 556, 1362, 1885 lines while the battery ported; since the --tree predicate landed 2026-08-24 only two real ports shipped in eight days and roughly nine iterations — port-blockers.sh 581 (08-28) and the drift KPI contract 713 (08-29), about 1300 lines — the other iterations bought the Windows leg (three), the installer repair, the msys dialect migration and the composer/declaration rulings. So the true port rate over the last week is roughly one port iteration in three, at 600-700 lines each, against 8689 owed lines whose members are heavier than the gates were (argv and env contracts, test harnesses, the installer runtime). Naive projection: 10-14 port-shaped iterations; at the observed mix that is 4-6 weeks of calendar, at a port-only mix roughly two. Three of the subtractions are RULINGS not engineering: run-smoke.sh (851) is one disposition call, the installer reading (1237) is one section clause, the harness arm-kind (613) is one contract decision.
