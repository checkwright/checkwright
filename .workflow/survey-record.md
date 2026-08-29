# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.













































## 2026-08-29 scope — Which live deferred entries does the rest of the queue converge on by inbound citation sum at this boundary, and what does the retired-target block say?
- corpus: TASK-QUEUE.md ## Deferred (267 entries) plus ## Icebox (50), every inbound citation resolved live-or-retired
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 266bdfdedccc610c2425b0d7d8c89527e3d8df3c
- finding: TOP LIVE TARGETS BY INBOUND SUM: platform-support-ci-matrix 10, prose-filename-citation-liveness 6, powershell-installer-surface 5, overlay-only-oracle-grants-uncommitted 4, then a band at 3 (unqualified-section-citation-liveness, threshold-recurrence-routing-residency, session-model-identity-verification, scratch-citation-skill-surface-reach, retired-slug-live-pointer-citation, queue-entry-grammar-single-owner, guard-steer-grant-mismatch, dispatch-claim-evidentiary-tier-unmarked, companion-toolkit-profile, build-stage-tier-economics, benchmark-ab-experiment) and roughly forty at 2. TWO DELTAS since the 2026-08-28 run: powershell-installer-surface fell 6 to 5, and native-gate-port-remaining-corpus fell 3 to 2 — the ported-and-deleted port-blockers.sh tool accounting for both. WHERE THE SUM CHANGES A RANKING: unchanged and still the only place, the citation-liveness family sums to about 18 inbound across members that individually rank low, and citation-liveness-family-convergence is where that dividend is already taken. RETIRED-TARGET READ, an input and not a footnote: 76 retired targets, heaviest check-spec-pointer 14, battery-runner-port 13, shell-gate-tail-port 12, all three shipped, so an entry arguing from any of them argues from disposed work. NOT RE-BOUGHT: the 2026-08-28 close census settled that the noise bucket is empty, that all four live-pointer citations were corrected at 1dd96bd5, and that this read should be budgeted as a targeted four-to-six candidate check rather than a 170-line audit — followed exactly. NO CANDIDATE WAS REFUSED ON A SUM this pass. The open half survey-edge-aggregation-residue carries — whether the pass is owed per ranked candidate when an authority supplies the unit set — did not fire here for the THIRD consecutive boundary: this dispatch is undirected and carries no candidate list.

## 2026-08-29 scope — For each of the four carried gap bullets, does the queue already carry an entry whose subject is the same, so that promotion would duplicate a filing?
- corpus: TASK-QUEUE.md ## Deferred plus ## Icebox, read as full entry bodies rather than lead lines; 17 named near-neighbour slugs plus a repo-wide grep for the subject vocabulary
- oracle: delegated audit-sweep under worktree isolation reading each candidate body in full, then verifying each bullet's own DISTINCT-from assertion against the named entry rather than trusting it
- rev: 266bdfdedccc610c2425b0d7d8c89527e3d8df3c
- finding: THREE DISTINCT, ONE DUPLICATE. Bullet 1 (read-only dispatch type cannot see close's gitignored capture surfaces) is DISTINCT and must name worktree-isolated-dispatch-cannot-reach-the-main-checkout and worktree-isolated-agent-report-lost-to-a-failed-peer-send: the first is the WRITE direction, an isolated child's own capture appends lost on reclamation, this is the READ direction, an audit dispatched to triage pre-existing gitignored content that a linked worktree never carries; no entry anywhere mentions DELEGATION_KIT_READONLY_TYPES. Bullet 2 (kfric second field names the owed home rather than the surface read from) is DISTINCT with NO existing owner: kfric-capture-unverified-assertion's axis is whether the FACT is verified, kfric-obligation-residency's is that capture never happens at all, and neither body mentions the field's direction. Bullet 4 (baseline self-certification unasserted) is DISTINCT and its own two disclaimers verify TRUE: baseline-move-stales-evidence-line owns the stale-evidence direction and check-evidence-manifest ALREADY REFUSES on it, so it is mechanized where this is not, and baseline-row-prose-coupling-gate is prose-vs-file consistency; gate-tests-suite-identity-in-evidence and stage-completion-unattested are thematically adjacent with no subject overlap. BULLET 3 IS A DUPLICATE of prompt-ranking-ungrantable-shape-class, filed by the IMMEDIATELY PRECEDING close: same structural claim that the matcher breaks on composition so no glob can match a composed form, and the same two proposed dispositions, partition the ranking composed-vs-bare or state the ceiling in the owning doc. The bullet checked its distinctness only against inline-body-interpreter-grant-absent and never against the actual near-duplicate. WHAT A LATER SESSION SHOULD TAKE: a gap bullet's own DISTINCT-from list is evidence of what its author checked, never of what exists — three of these four were right and the fourth omitted the one entry that mattered, so the dedup sweep is owed on the whole neighbourhood rather than on the bullet's nominated set.

## 2026-08-29 spec — What does the wait-loop grant class actually measure at this iteration, and is it still the kill -0 shape the entry cost line was written on?
- corpus: .workflow/prompt-friction.log (UNTRACKED local capture — the witness corpus-diff command does NOT apply to this block; re-run the oracle instead)
- oracle: bash guard-kit/bin/scan-prompts.sh, plus grep -nE 'until |while |kill -0|sleep ' .workflow/prompt-friction.log
- rev: 3923411694dc202a7fd967ed3547a0d682b5e407
- finding: FOURTH DATUM, taken at spec on an uncleared log spanning the previous close plus this iteration to date: 162 logged fall-throughs, 54 prompting calls across 18 patterns. The wait-loop class is 4 prompting calls across 3 ranked patterns (until x3, while x1). ZERO are the kill -0 pid form — the exact form the entry cost line, guard rule 17 and the standing dispatch policy are all written on. The four measured commands are: two 'until grep -q PATTERN FILE; do sleep N; done' waits on an artifact, one 'until gh run view ID ... | grep -q completed; do sleep N; done', and one 'while pgrep -f PATTERN; do sleep N; done' — the last being the pattern-match liveness form the dispatch policy explicitly forbids. So the measured class is NOT kill-0-shaped; it is 'a wait loop whose condition is an arbitrary command', strictly wider than any grant keyed on kill -0 would cover. A fifth ranked pattern, pgrep -af used to check liveness without a loop, prompted once more.

## 2026-08-29 spec — What obligations does porting the drift-kit KPI corpus to the binary substrate actually carry, and what does the fan-out stale?
- corpus: drift-kit/ plus native/src/emit/ plus gate-sdk/SPEC.md sections Porting a gate to the binary substrate, The non-gate arm and port-blockers, plus every .gate descriptor and docs projection naming a KPI
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree (141 owed / 153 scanned / 12 no-port at this rev), plus git ls-files over drift-kit and a grep for no-port and port-until headers
- rev: daebc9a21443304ede4b47d5ab00083a55c045a3
- finding: NO .gate DESCRIPTOR, NO gates.list REGISTRATION, NO FIXTURE PAIR IS OWED — the seven port-candidate criteria are ruled out of scope for a plain script. Registration is exactly two edits: a pub mod line in native/src/emit/mod.rs and one BRIDGED_ARMS tuple. The arm must be a BRIDGED arm, not a top-level flag: the front-end composes --emit-NAME from its --emit NAME operand, and a hardcoded top-level flag receives no consumer override at all. THE SHELL ORIGINAL IS DELETED, never left beside the port — corpus membership is git ls-files, so a surviving file must carry a marker, and a marker DECLARES IT IS NOT PORTING. Only never-porting files declare. THE HARDEST DESIGN PROBLEM: the plugin env contract is built on compgen over the DRIFT_KIT_ namespace SPECIFICALLY so there is no fixed export list to drift out of parity, but a bridged arm receives only its DECLARED knobs — so a naive port replaces a derivation with a maintained roster. Two in-tree solutions exist: the union sentinel at-every-registered-knob, minted for this exact shape, and the prefix-family form. THE SHARPEST BLOCKER: deleting the 13 kpis/kpi-*.sh reds EVERY LINE of drift-kit/templates/kpis.list under check-template-registry-parity, whose assertion B refuses a registry line naming no shipped file and which has no notion of a native-dispatched plugin. check-settings-paths also reds on two dead grants in the committed settings. Four of the 18 are contested: lib/drift.sh is the config bridge's only knob resolver and three already-compiled arms source it; both templates/ files are consumer content; smoke/install.sh is 41 percent of the corpus and the kit's only behavioural oracle. Fan-out that fires: check-gate-binary-fresh (discharged only by build-native.sh, never by cargo test), check-crate-arms, check-docs-mirror-fresh, check-settings-paths, and tree-shell-owed in measured-claims; check-footprint-fresh does NOT fire. Newest precedent amendment to copy: gate-sdk/SPEC-port-blockers-arm.md at 187696c4, 11 deltas, work-class labels spelled in curly braces at the end of each lead sentence.

## 2026-08-29 spec — Which root call sites are dialect-exposed, and what actually produced the observed MSYS failure string?
- corpus: every show-toplevel occurrence tree-wide plus native/src/walk.rs, gate-sdk/lib/gate.sh root derivation, installer/bin and installer/lib root entry, and .github/workflows/gates.yml's Windows leg
- oracle: grep -rn show-toplevel over the tree excluding .git, read against native/src/walk.rs abs_against and normalize_abs and against gate-sdk/lib/gate.sh's GATE_KIT_ROOTS_HERE derivation
- rev: daebc9a21443304ede4b47d5ab00083a55c045a3
- finding: THE ENTRY'S DIAGNOSIS IS FALSIFIED AND THE DEFECT IS IN RUST, NOT IN SHELL. The unaccounted third step is native/src/walk.rs: cwd() is std env current_dir(), which on the windows-msvc leg returns a backslash spelling; abs_against tests absoluteness with starts_with slash, a POSIX-only predicate a drive-lettered path fails; normalize_abs splits on slash only and then UNCONDITIONALLY PREPENDS A POSIX SLASH; registry.rs appends the checks segment. That reconstruction reproduces the observed string character for character, which neither of the entry's two candidate mechanisms does. The shell side behaved CORRECTLY — gate.sh deliberately emits kit roots RELATIVE to PWD so no absolute path is baked into the tracked hook. COROLLARY THAT INVERTS THE OBVIOUS INFERENCE: porting a file to Rust does NOT retire its dialect exposure, because the crate composes paths with String and format! rather than with Path, and in this case the port CREATED the defect. THE JUDGING PREDICATE THE CONTRACT NEEDS: the pwd fallback confers nothing, since it fires only when git FAILS and on MSYS git SUCCEEDS in the wrong dialect — what separates safe from exposed is CONSUMPTION, so a root consumed only by cd is dialect-tolerant and a root consumed by concatenation is not. All four drift-kit bin tools are cd-only and are therefore the cheapest possible migration, but drift-report.sh carries a SECOND root, KIT, which IS a concatenation site. The gate-tests bucket does NOT fall out entirely: roughly three of nine are concatenation sites. Census re-derived: 52 code occurrences of which 48 are real call sites, and the eight-roots figure counts TOP-LEVEL DIRECTORIES, not root variables, of which there are four. NO SPEC OWNS ROOT DIALECT: the tree's only normative sentence states shape and mechanism and never dialect. And there is NO CI ORACLE — the Windows leg dies before the battery runs and the Ubuntu leg only cross-compiles.

## 2026-08-29 align — Do the four amendments' factual claims about the tree (line numbers, grant counts, code shapes, work-class labels, precedent citations, roster completeness) still hold against the current tree?
- corpus: drift-kit/SPEC-kpi-port.md gate-sdk/SPEC-dialect.md guard-kit/SPEC-wait.md lifecycle-kit/SPEC-edge-sum.md, plus every surface each amendment's Producers/consumers and Existing-sections-updated rosters cite (native/src/walk.rs, native/src/gates/template_registry_parity.rs, native/src/emit/enforcement_map.rs, native/src/emit/mod.rs, native/src/proc.rs, native/src/gates/survey_record.rs, .claude/settings.json, guard-kit/lib/guard.sh, guard-kit/SPEC.md, drift-kit/README.md, drift-kit/SPEC.md, README.md, lifecycle-kit/SPEC.md, CLAUDE.md, TASK-QUEUE.md)
- oracle: direct grep/Read verification per claim (--emit port-blockers --tree; grep -n drift-kit .claude/settings.json; grep -n show-toplevel; grep -c rule-19..22 over the four named guard-kit files; grep -rn survey-engagement; grep for on_path fixture pair; git log -S for the eight-roots phrase origin) — no single oracle
- rev: b3e641c754f9fa785a9af095196312ee53955c94
- finding: ALL SIX RULINGS HOLD, unchanged, against a direct re-check. WORK-CLASS LABELS MATCH THE DELTA BODIES EXACTLY on all four amendments (SPEC-kpi-port.md 1-7/8-10, SPEC-dialect.md 1-4/5-6, SPEC-wait.md 1,2,4,5/3, SPEC-edge-sum.md 1,4,6/2,3,5) — no mislabelled delta found. THE SETTINGS GRANT LINE NUMBERS ARE EXACT: grep -n drift-kit .claude/settings.json returns lines 19-26 verbatim as SPEC-kpi-port.md's table states, split 19-20/25-26 dying and 21-24 surviving. Rule-number placement claims in SPEC-wait.md (rule 6 is expansion, blocking a quoted pid; rule 12 is pgrep_self_match; rule 13 is bare_sleep with the corrective spelling; rule 15 is background_no_record; rules 17/18 are append/ro_pipeline auto-allow band) verified against guard.sh's actual dispatch order and SPEC.md's own numbered prose; the roughly-thirty-citation estimate for the four shifted rules (19-22) reconciles to 29 when scoped to exactly those four rules over exactly the four named files (a wider 16-23 scope over the whole guard-kit tree gives 66, which is not what the amendment measured). SPEC-dialect.md's walk.rs diagnosis (abs_against's starts_with-slash test, normalize_abs's split-on-slash-then-unconditional-prepend) reproduces byte-for-byte against the current source. drift-report.sh's KIT/BASH_SOURCE concatenation at line 11 and the three drift-kit bin tools single-cd REPO_ROOT consumption both confirmed line-exact. SPEC-edge-sum.md's four-argument CLAUDE.md citation, the four-key gate array, and the three-block good fixture (plain/none/valved) all confirmed. ONE GAP FOUND AND FIXED IN-SESSION (not a lead escalation, inside the amendment's own envelope): SPEC-kpi-port.md's roster named context-kit/SPEC.md and both session-context.sh copies as the surfaces reading drift-report.sh as a script path but missed drift-kit/README.md's own install step and Use section, plus README.md's drift-kit table row, all three going stale the moment delta 9 deletes the shell driver; the live precedent (context-kit/README.md's emit-footprint line, queue-kit's already-ported table row) made the fix mechanical, landed directly in the amendment's roster and Definition-of-Done at b3e641c7. NO OTHER GAP SURVIVED THE SWEEP: the msys-dialect-migration entry's own eight-roots-equals-root-variables gloss traces by git log -S to a genuine historical meaning of eight KIT DIRECTORIES (context-kit delegation-kit drift-kit gate-sdk installer lifecycle-kit native scripts, commit 1ab1bad2), so SPEC-dialect.md delta 6's planned correction of that gloss is itself correct rather than a new drift to flag.
