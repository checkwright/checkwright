# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.
























## 2026-08-18 scope — Which deferred citation-liveness entries share one owning gate, and which sub-bundle do the entries themselves instruct a scope to cost together?
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 73c4ca6b470eafcb61c724f99e6bdab981aa7815
- finding: Twelve deferred entries form the dangling-citation family; five name check-spec-pointer's prose extractor as the owning surface (prose-filename-citation-liveness, unqualified-section-citation-liveness, link-wrapped-section-citation-liveness, qualified-pointer-section-ownership, spec-pointer-self-section-citation), two name check-md-refs (prose-filename-citation-liveness again, md-refs-tree-link-resolution), two resolve to a listing rather than a gate (dead-queue-citation-report and done-slug-ownership-citation-report, which say taking either costs both), and three own separate mechanisms (doctrine-rule-number-citation-liveness, kit-ref-liveness-stem-token-hole, ruling-record-condition-staleness-probe). None carries a blocked-by tag. The bundle the entries themselves instruct is the first three plus spec-pointer-self-section-citation on one extractor pass: unqualified-section-citation-liveness says a promoting scope should cost them together and may find one predicate covers both, link-wrapped says cost all three together, and spec-pointer-self says to sequence it with them rather than building a second scanner. qualified-pointer-section-ownership shares the gate name and is excluded by its own text: it resolves successfully and is wrong anyway, a comprehension problem whose entry admits an honest not-buildable outcome. stale-identifier-after-retirement names prose-filename-citation-liveness its decisive neighbour, the only subsumption claim in the set. Feature-vs-debt splits inside the bundle: prose-filename-citation-liveness states itself a feature (mints a script name and a gates.list registration) while unqualified-section-citation-liveness states itself debt (an assertion inside check-spec-pointer, minting no name), so the bundle owes an amendment for its shipped-path arm whatever the other members land as. Stated cost the bundle carries: 171 live unqualified citations held by nothing.

## 2026-08-18 scope — Which deferred entries have reached the recurrence threshold, and which unported gate groups are takeable this cut?
- corpus: TASK-QUEUE.md scripts/gates.list */checks/*.gate native/src/gates/*.rs
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 73c4ca6b470eafcb61c724f99e6bdab981aa7815
- finding: Recurrence: exactly one deferred entry reaches LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2) — stage-stamp-ordering-unenforced, dates 2026-08-07 and 2026-08-16. Thirteen other entries carry a single-date recurrence declaration and are below threshold. Port: 104 members scanned, 71 already ported, 3 permanently shell, 30 groups formed and every one of them a SINGLETON, so the size arm is exhausted at this cut as it was at the last two. The budget arm's policy entry (port-tail-cohort-batching-policy) and the consumer-cohort entry (consumer-gate-port-disposition) have both left the queue through Done, so a hand-composed budget batch is the only gate-side increment available and the citations to those two slugs in TASK-QUEUE.md and TRAJECTORY.md are now dead. Separately rostered and NOT part of that 30: freshness-emitter-port-cohort's three remaining shell emitters, scripts/gen-docs-mirror.sh (127 lines), drift-kit/bin/trajectory.sh (242) and queue-kit/bin/roadmap.sh (76), 445 shell lines whose non-gate-arm design ruling is already merged and whose two Linux-side comparators, check-docs-mirror-fresh and check-trajectory-fresh, are already native. The third comparator, check-roadmap-fresh, is a held shell member on cohort-held-members-port-prerequisites whose hold ground is that it keeps queue_roadmap_entries on one shell adapter, so porting roadmap.sh without it splits an emitter/gate pair across substrates.

## 2026-08-18 scope — Which deferred citation-liveness entries share one owning gate, and which sub-bundle do the entries themselves instruct a scope to cost together?
- corpus: TASK-QUEUE.md
- oracle: bash guard-kit/bin/scratch-run.sh .tmp/verify-cluster.sh
- rev: 41ac6532ff5834916a6dd80c6c797f6882527f89
- finding: SUPERSEDES the earlier block of the same question at this iteration's scope, on provenance rather than on substance. The earlier block was written from a dispatched sweep whose completion this session cannot attest, which is the dispatch-cited-evidence-unverified class; this block re-derives the same finding first-hand and reports what survived and what did not. CONFIRMED verbatim against the entry bodies: exactly five of the twelve name check-spec-pointer (prose-filename-citation-liveness, unqualified-section-citation-liveness, link-wrapped-section-citation-liveness, qualified-pointer-section-ownership, spec-pointer-self-section-citation); none of the twelve carries a blocked-by tag; the four instructing sentences are real and correctly attributed, namely unqualified-section-citation-liveness saying a promoting scope should cost them together and may find one predicate covers both, link-wrapped-section-citation-liveness saying cost all three together, spec-pointer-self-section-citation saying sequence it with them rather than building a second scanner, and stale-identifier-after-retirement naming prose-filename-citation-liveness its decisive neighbour; and unqualified-section-citation-liveness carries the 171-live-unqualified-citations cost figure. CORRECTED: the earlier block said two entries name check-md-refs. A mention scan finds four (prose-filename, unqualified-section, link-wrapped, md-refs-tree-link-resolution). The instrument is coarser than the claim, since a mention is not an ownership assignment, so the earlier count is not refuted but is unproven; a stage acting on the md-refs attribution should read those four bodies rather than cite either number. Everything else in the earlier block stands as written.

## 2026-08-18 scope — Which deferred citation-liveness entries share one owning gate, and which sub-bundle do the entries themselves instruct a scope to cost together?
- corpus: TASK-QUEUE.md
- oracle: grep -c 'check-spec-pointer' TASK-QUEUE.md
- rev: 41ac6532ff5834916a6dd80c6c797f6882527f89
- finding: SUPERSEDES the block immediately above on ONE field only: its oracle named a .tmp/ scratch script, and .tmp is wiped at the scope boundary, so the witness it offered a later stage was unrunnable by construction. The finding is unchanged and is not restated here — read the block above for it. What changes is the cheap re-runnable half: a durable stand-in is the check-spec-pointer mention count over TASK-QUEUE.md, which was 5 entry bodies at this revision, and a stage wanting the full aggregation re-derives it by reading the twelve named bodies. Filed as a gap in its own right, since a survey record whose oracle is a boundary-wiped path is a class rather than this session's slip.

## 2026-08-18 spec — What does each of the three remaining freshness emitters emit, read, and shell out to, what knobs does it carry, and what does the crate already have to receive it?
- corpus: scripts/gen-docs-mirror.sh drift-kit/bin/trajectory.sh queue-kit/bin/roadmap.sh queue-kit/checks/check-roadmap-fresh.sh gate-sdk/checks/check-root-tiering.sh native/src/
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: ab4d690942e122c79b6d502e8e120e7b930f412c
- finding: Arms register in native/src/emit/mod.rs:18-80 as EMITTERS tuples (name, EmitFn = fn(&[String]) -> Result<String,String> at L13, bridged-knob roster); the subcommand spelling is derived, arm_name(p) = --emit-<p> at L85-87; main.rs:247-258 resolves emit::lookup before the registry and keeps arms out of --list. Registered today: footprint, close-surfaces, enforcement-map, value-rollup, queue-index. A ported comparator calls the arm in-process (gates/footprint_fresh.rs:33-37, emit::footprint::emit(&[])), while docs_mirror_fresh.rs and trajectory_fresh.rs still spawn through fresh.rs:73-81. Knobs arrive as GATE_SDK_KNOB_<NAME> (spec.rs:9-10) read by walk::knob_array/knob_map/knob_prefix/knob_scalar (walk.rs:11,25,43,74,110); queue.rs re-exports walk's. marker.rs carries read_block (L9) and write_block (L40). Per emitter: gen-docs-mirror.sh (127) emits docs/<kit>/{SPEC,README}.md plus docs/doctrine-kit/DOCTRINE.md whole-file with no marker, one knob DOCS_MIRROR_BLOB_BASE, and shells realpath -m --relative-to (GNU-only); trajectory.sh (242) emits the docs/evidence-data.md table whole-file, five knobs (DRIFT_KIT_CONFIG_FILE, DRIFT_KIT_TRAJECTORY_SURFACES, DRIFT_KIT_GATES_FILE, DRIFT_KIT_STAGES, GATE_SDK_WORKFLOW_DIR), and shells git rev-parse/cat-file/log -p -U0/show plus date -d over git's own %F strings; roadmap.sh (76) splices the ROADMAP.md marker block, four knobs (QUEUE_KIT_QUEUE_FILE, QUEUE_KIT_HORIZONS, QUEUE_KIT_ROADMAP_FILE, QUEUE_KIT_ROADMAP_MARKER), and calls queue_roadmap_entries (queue-kit/lib/queue.sh:101-147, awk, output <ntags> TAB <fieldv> TAB <slug> TAB <nsum> TAB <summary>) whose only two callers are bin/roadmap.sh:32 and checks/check-roadmap-fresh.sh:59 and which has no Rust counterpart. check-roadmap-fresh is 111 shell lines with no .gate descriptor, spawning the emitter at L99, fixtures at queue-kit/gate-tests/check-roadmap-fresh/{good,bad}/ both steering assertion A off the live emitter. check-root-tiering is 67 shell lines, no descriptor, hermetic fixtures with their own allow.list/root/args/expect.txt, positional allowlist and scan root, built-in fallback set at L26. Descriptor gap: scripts/check-trajectory-fresh.gate does not trigger on drift-kit/bin/trajectory.sh at all and check-docs-mirror-fresh.gate triggers on but does not couple scripts/gen-docs-mirror.sh. Census at this cut: 104 scanned, 30 groups all singletons, 71 ported, 3 permanently shell; check-roadmap-fresh is group 23 and check-root-tiering group 6, both c2=pair c3=precommit c7=clean.
