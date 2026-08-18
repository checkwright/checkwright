# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.


























## 2026-08-18 scope — Outside the native-gate-port track, which deferred entries are the strongest next-iteration candidates, and which cluster onto a shared surface?
- corpus: TASK-QUEUE.md's 223 Deferred + 19 Icebox entries, read via inbound-citation degree
- oracle: bash queue-kit/bin/queue-edges.sh for degree, then full-body reads of every top-inbound slug against its 'Cost while deferred' clause
- rev: eee198e5724f18068d9d50aaf6fa44c16e959fa7
- finding: THREE CLUSTERS carry the corpus. (1) CITATION/POINTER LIVENESS — prose-filename-citation-liveness (:1274), unqualified-section-citation-liveness (:2109), link-wrapped-section-citation-liveness (:3158), spec-pointer-self-section-citation (:3713), qualified-pointer-section-ownership (:3917), stale-identifier-after-retirement (:6338), adjacent ruling-record-condition-staleness-probe (:3232). All six extend check-spec-pointer's already-native prose-citation extractor; link-wrapped's own body says a promoting scope should cost three of them together because the guard's adjacency window, not the citation form, is the variable. Mostly DEBT, one FEATURE edge (prose-filename mints a script name), one possibly-unbuildable member (qualified-pointer, whose body admits 'not buildable' as a permitted outcome). Cost is MEASURED: 171 live unqualified citations, 902/1774 directive pointers plus 247 prose citations with unverified ownership, one dead intra-file citation already published to the public docs mirror. (2) DELEGATION / DISPATCH PROVENANCE / WAIT PRIMITIVE — turn-end-chokepoint-and-wait-primitive (:5920, THIRTEEN attested firings, one costing a whole suite's evidence), subagent-stop-liveness-hook-wiring (:5971), launch-chokepoint-liveness-record-write (:6472), dispatch-cited-evidence-unverified (:3894), dispatch-unreadable-target-fallback (:4579), delegation-provenance-floor (:5131, fired twice, the second on 2026-08-18, two of three relayed claims later falsified), handoff-premise-reverification-placement (:5178). The middle four are ONE design question seen from four angles. launch-chokepoint is cleanly buildable after a one-command probe. subagent-stop-liveness-hook-wiring is NOT buildable by a stage session at all: it needs an operator-authorized .claude/settings.json hook write, so it is an operator-ask rather than an in-iteration deliverable. (3) SESSION-MODEL-IDENTITY — session-model-identity-verification (:2737) plus its dependents consult-tier-declaration (:2757, blocked-by it) and co-authored-by-trailer-attribution (:2772); FEATURE, self-labelled as wanting spec rather than a debt promotion. REJECTED with cause: stage-stamp-ordering-unenforced (declined four times, twice by the operator, on an unresolved design fork); batch-split-stamp-ownership (entangled with that same fork, self-described as doctrine drift rather than breakage); benchmark-ab-experiment / build-stage-tier-economics / design-partner-preview and heterogeneous-agent-delegation / companion-toolkit-profile (forward-looking design space, no measured recurring cost).

## 2026-08-18 scope — What does each of the 14 takeable port members cost, and what is the defensible fifth budget batch?
- corpus: the 14 takeable members' shell declarations, their gate-tests good/bad pairs, and gate-sdk/SPEC.md's porting procedure plus its meta-gate conservation table
- oracle: bash gate-sdk/bin/port-blockers.sh --group for the takeable set and each member's libs= key, wc -l per declaration, then a hazard read of each declaration against criterion 4 (self-reference), criterion 7 (external program) and fixture-arm coverage
- rev: 33afcd6936d07bd31d41158b4d64ede8f4f0d1ab
- finding: CHEAPEST-FIRST, lines then hazards: check-gate-fixture-coverage 93 (self-referential, real pair); check-gate-output 103 (self-referential, pair already carries a .gate case); check-evidence-manifest 105 (NOT self-referential, trivial line-parsing, but its three helpers are local to evidence-kit and have no Rust precedent — first port into that kit); check-gate-binary-fresh 107 (c7=? but its only non-floor program is the crate's own --source-stamp arm, already shipped; gate-sdk/SPEC.md's conservation row states it reads declaration PATHS as a set and never a gate's source, so criterion 4 clears); check-amendment-queue 117 (clean, one ~30-line awk state machine); check-template-copy-parity 127 (corpus confirmed clear of criterion 4 by the SPEC table; the risk is reproducing declared_surface()'s hand-rolled case/esac awk faithfully); check-identity 135 (cheapest LOGIC of the whole set, git only, 3 local helpers); check-gate-tamper 140 (ZERO shared-lib fan-out, all six helpers local; criterion 4 clears — its corpus glob is scripts/check-*.sh + scripts/check-*.gate and its own declaration lives at delegation-kit/checks/, and scripts/ currently holds NO check-*.sh at all, only .gate descriptors, which the scope re-probed directly); check-gate-exemption-tasks 157 (self-referential by design, two real awk state machines, and DELIBERATELY EXCLUDED from the fourth batch for contract churn — portable, not cheap); check-prose-tells 208 (six statistical assertion branches incl. floating-point variance, two shared awk generators, and a THIN fixture: one case per side for six branches — widen before porting); check-reads-couples 219 (the SPEC's own hardest worked case: self-referential, c7=?, largest fan-out at 9 shared + 5 local, .gate branch under-exercised); check-knob-default-coupling 240 (two intricate awk programs plus a hidden shared generator, incidentally self-referential via kit:*.sh); check-graph 632 (27 shared functions, emits a self-contained HTML+mermaid artifact, deepest self-reference — needs a non-gate emit arm DESIGNED before porting, per the generated-projection freshness family precedent). RECOMMENDED FIFTH BATCH, 7 members inside the fourth cut's ruled 6-8 width, none failing criterion 4 or 7 on the SPEC's own terms: check-gate-fixture-coverage, check-gate-output, check-evidence-manifest, check-gate-binary-fresh, check-amendment-queue, check-identity, check-gate-tamper. Optional eighth: check-template-copy-parity. check-graph should be its OWN iteration — bundling an HTML emitter with 27-function fan-out into a budget batch violates the never-as-one-cohort property the batch mechanism rests on. UNVERIFIED, carried as such: whether check-spec-embedded-source's CANON_KIT_EMBED_LANGS includes a shell kind (which would put gate declarations in its content-scanned set), and whether the three shared awk generators have callers beyond the two gates using them.
