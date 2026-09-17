# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-17 scope — Which deferred entries rank first for the iteration after install-smoke-slow-leg (undirected scope)?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: ab319f794ff4927460668e14ee0ab03862e1912d
- finding: Deferred board: no session-class rows; sole iteration/high is binding-intel-leg-failed-one-run-in-two (.github), intel leg green in the last 20 master gates runs with scratch witness landed. No recurrence count reaches threshold 2. Validate red-holding cluster (validate-hold-rule-admits-iteration-caused-red, validate-tier-premise-mechanical-only, baseline-move-stales-evidence-line) is cross-cited and was the prior scope's named later set; .github CI-leg cluster (smoke-leg-crate-build-uncached, shellcheck-analyser-version-unpinned-in-ci, held-ci-leg-failure-reddens-a-binding-one) shares binding-intel's surface.
- inferred: binding-intel-leg-failed-one-run-in-two's iteration class is stale: its residual is observation of an unrecurred firing, not a build

## 2026-09-17 align — Do the hold-cause and tier-discovery amendments (3ad5ff58) wire cleanly against the current tree, and does their Existing-sections-updated roster miss a touched surface?
- corpus: SPEC-hold-cause.md, SPEC-tier-discovery.md, evidence-kit/{SPEC.md,README.md}, native/src/{evidence.rs,stages.rs,gates/evidence_baseline.rs}, lifecycle-kit/{SPEC.md,templates/stages/validate.md,templates/stages/close.md,templates/lead.md}, .claude/{commands/validate.md,commands/lead.md,agents/stage-session.md}, TASK-QUEUE.md, .github/workflows/gates.yml
- oracle: targeted grep/read at each replacement-text site plus bash gate-sdk/bin/run-gates.sh (full battery, green) and --only check-amendment-retired-spelling (green)
- rev: af1950b6bca5e0bafb7af2d6d93937441c0c9798
- finding: every checkable technical/cross-component claim in both amendments verified true against the tree; two amendment-only defects found and fixed in this session -- hold-cause delta 3 never edited close.md's valve-disposition step to carry reproduces-at onto the row it lands, though Producers-and-consumers already asserted that behavior (added the replacement text and roster entry); tier-discovery delta 3 miscounted the lead.md validate bullet's sentences ('second and third' where only one exists after the first, an em-dash appositive not a sentence break) -- corrected. No other roster gaps found
- inferred: the remaining replacement-text targets are byte-accurate now, but a tree edit between here and build could still drift a site; build should re-probe each target before applying rather than trusting this audit's snapshot
