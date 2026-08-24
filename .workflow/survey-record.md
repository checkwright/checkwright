# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.




































## 2026-08-24 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2 judged recurrence dates), and what is each one's hold history?
- corpus: TASK-QUEUE.md ## Deferred, 239 entries
- oracle: grep -n 'recurrence:' TASK-QUEUE.md, then read each entry's body
- rev: 6914b91aa184e397c421510cd5a43ab88d57890a
- finding: Five entries at or over threshold: kfric-empty-log-ambiguity (2: 08-17, 08-23; held 2026-08-24 by lead on the surface criterion), close-entry-baseline-bootstrap-deadlock (2: 08-12, 08-18; SEVEN holds, the sixth operator-ruled 2026-08-23), agent-worktree-reclamation-unenforced (3: 08-19, 08-22, 08-24; four firings, held 2026-08-24), delegation-provenance-floor (2: 08-18, 08-19; five holds, the second operator-ruled 2026-08-22), icebox-candidate-eligibility-unapplied (2: 08-21, 08-23; held 2026-08-24 on a rate-collapse ground the 2026-08-24 gap bullet now contests). No sixth entry carries two dates. Every declaration line was read in full rather than counted by matcher.

## 2026-08-24 scope — Which live deferred entries converge on one shared surface, ranked by summed inbound citation edges?
- corpus: TASK-QUEUE.md, every live slug with at least one inbound edge
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 6914b91aa184e397c421510cd5a43ab88d57890a
- finding: The largest convergent cluster is CITATION LIVENESS on canon-kit's check-spec-pointer / check-docs-cmd surface: prose-filename-citation-liveness carries the queue's maximum 5 inbound edges, and 13 live entries sit on the same surface at lines 1199 (prose-filename-citation-liveness), 1773 (unqualified-section-citation-liveness), 2421 (dead-queue-citation-report), 2729 (link-wrapped-section-citation-liveness), 2776 (ruling-record-condition-staleness-probe), 3223 (spec-pointer-self-section-citation), 3427 (qualified-pointer-section-ownership), 4430 (spec-section-title-collision), 5235 (done-slug-ownership-citation-report), 5553 (stale-identifier-after-retirement), 6064 (cited-script-path-liveness-inline), 6540 (retired-slug-live-pointer-citation), 6576 (queue-status-parenthetical-liveness), plus amendment-roster-omission-detection in the icebox. Runners-up by inbound count: delegation-provenance-floor 4, benchmark-ab-experiment 4, powershell-installer-surface 3, platform-support-ci-matrix 3, session-model-identity-verification 3, guard-steer-grant-mismatch 3, overlay-only-oracle-grants-uncommitted 3, native-gate-port-remaining-corpus 3, dispatch-cited-evidence-unverified 3.

## 2026-08-24 spec — If a stale .claude/worktrees/<id>/ copy of the whole repo were present, which readers in this tree would descend into it and read that second copy as tree content?
- corpus: every scripts/gates.list member, every native/src/gates/mod.rs registry entry, every tracked kit bin/ and lib/ script plus scripts/, and every consumer-config glob array in scripts/*-config.sh
- oracle: read each reader's walk and classify: pruned via GATE_PRUNE_DIRS (shell gate_find/GATE_GREP_EXCLUDES/gate_path_pruned, native path_pruned with the knob declared) | immune-by-tracking (git ls-files corpus) | immune-by-glob (a single-level glob cannot cross /) | EXPOSED
- rev: 1392591c8d692ee2e9e11eeeb1ba7fdb9683dd62
- finding: EXACTLY TWO exposed readers, both verified by direct read at this stage: context-kit/bin/md-index.sh:72-76 (find over REPO_ROOT excluding only */target/*, */.git/*, */node_modules/*) and context-kit/bin/pub-index.sh:50,71 (a PRUNE array of those three plus */dist/*, */build/*). Neither carries the worktrees leaf, so both index a second full copy of every governed markdown surface. Everything else is pruned, immune-by-tracking or immune-by-glob. 37 of 108 native registry members declare GATE_PRUNE_DIRS and ZERO walk without declaring it. All three shell walk adapters read one shared array. So the standing premise that the .claude/worktrees/ exclusion is per-caller rather than central is TRUE ONLY IN A NARROWER SENSE: central and correctly bridged for the gate substrate, per-caller and stale for the two context-kit index tools.
