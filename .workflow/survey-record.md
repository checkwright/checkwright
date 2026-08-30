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
