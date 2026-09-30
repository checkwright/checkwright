# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-30 scope — Which gates declare install: zero-config, per kit, and which canon-kit members read adopter-authored surfaces?
- corpus: '*/checks/*.gate'
- oracle: grep -l '^# install: zero-config' */checks/*.gate
- rev: cbf56b626d695330096cab5856cf14400c5b8ad3
- finding: 54 zero-config gates: canon-kit 26, gate-sdk 15, queue-kit 6, evidence-kit 2, and one each in context-kit, delegation-kit, doctrine-kit, guard-kit, site-kit. check-comment-tier's descriptor couples *.sh with no surface restriction by default. gate-sdk/SPEC.md §The install disposition defines zero-config as every read surface being one init writes and the adopter does not author.
- inferred: Which of the 28 non-canon-kit zero-config members read adopter-authored content is unsurveyed; the canon-kit readers of adopter .sh and READMEs are the entry's own re-verification, and a fresh prose install reddening on a plain adopter comment is unrun (init needs a packed payload).

## 2026-09-30 scope — Which deferred entries lead this iteration's unit set by cost class, inbound citations and supersession?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: cbf56b626d695330096cab5856cf14400c5b8ad3
- finding: Tier one: session/high canon-kit-zero-config-rejudge and spec-brevity-residue; iteration/high gate-customer-value-audit and compiled-consumer-smoke-driver; iteration/low heterogeneous-agent-delegation, manual-operation-spend-channel, roadmap-horizon-lag-detector, lessons-learned-channel-audit. No deferred entry reaches the recurrence threshold of 2. Inbound: companion-toolkit-profile 5, heterogeneous-agent-delegation 4, gate-customer-value-audit 3 (one from the rejudge). Supersession: seeded-ci-gates-on-surface and companion-install-tier reshaped by the rejudge (same install-disposition definition); release-step-number-cites should precede any RELEASING.md step reorder. queue-flow mean-filed 4.2; gate-sdk carries the most board rows (13).
- inferred: The canon-kit brevity slice's word total (about 11.3k over eight sections) is an awk word count, not a check-prose-bounds reading; which sections the rejudge edits is reasoned from its deliverable.

## 2026-10-01 spec — Which zero-config gates red on ordinary adopter-authored content in a fresh install, per profile, and which assert a house policy rather than a defect?
- corpus: '*/checks/*.gate' native/src/gates native/src/knobs installer
- oracle: run .tmp probes: pack the payload with --pack-installer, init each profile into a scratch repo holding plain adopter files, run the battery
- rev: bc9ab0de8e574ce2bcbc79c4364062cb72c5c683
- finding: Probed: starter reds check-path-dialect (a rev-parse root) and check-tree-terms (a /home/<name> path); prose adds check-comment-tier, check-manifest-temporal, check-md-refs, check-manifest-count, check-fence-command-head (npm install fence), check-spec-pointer (a duplicate heading), check-spec-dod-singleton (a SPEC.md with no DoD) and check-amendment-queue (an unrelated SPEC-notes.md); delegation's check-brevity exits 2 on an adopter CLAUDE.md lacking its section; a bare repo is green in every profile. By reading: 13 hold adopter-authored content to a policy a plain file breaks (canon-kit's manifest-temporal, manifest-count, fence-command-head, spec-pointer, comment-tier, spec-dod-singleton, spec-derivable-section, spec-embedded-source, amendment-queue; tree-terms, commit-msg, path-dialect, brevity), 6 red adopter content only on a defect (md-refs, docs-cmd, tracking-claim, spec-fence-balance, pendency-contradiction, pipe-membership), and 35 read init-written, kit or knob-armed surfaces or methodology constructs only; check-knob-default-coupling can only skip in a vendored tree. check-action-pinning and -permissions pass the seeded gates.yml and assert a policy.
- inferred: The per-member defect-versus-policy classification of the 54 is a code read by two sweeps, not a run per member; only the probed content classes were executed; check-comment-tier fired on a plain .sh comment in probe 1 only.
