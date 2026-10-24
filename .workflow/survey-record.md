# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-24 scope — Which deferred entries lead this iteration's unit set, ranked by cost tier, roadmap, surface fill and inbound edges?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges TASK-QUEUE.md
- rev: d789f920bf242321aa310100d925d92fa005726c
- finding: No Deferred row carries a session or iteration [cost:] class (all event or once), so tier one is empty. No entry reaches the recurrence threshold of 2. Tier two: companion-toolkit-profile and design-partner-preview (roadmap now) are gated on the observed install, operator hours outside a stage, so ranked out; benchmark-ab-experiment and hosted-attestation-service are later and demand-gated, ranked out. personal-adoption carries the operator's 2026-10-24 direction to take it at the next scope and names verify-workflow-decoupling (1 inbound, from personal-adoption) as the ruling it composes with first; both are installer surface, so they lead. Tier three same-surface fill: smoke-whole-tree-precondition-unscoped (installer, one recurrence). design-partner-preview and benchmark-ab-experiment carry 2 inbound each, all from within the gated adoption cluster, ranked out with it. The retired block's rows are all past-tense landed-at or split records, none directional. Supersession: verify-workflow-decoupling reshapes personal-adoption (goes first); every other name searched pairs nothing. queue-flow mean-filed 1.2 against a drain of 3.
- inferred: The two roadmap now-rows' gates (an observed install) are unmet today, read off their bodies rather than probed against docs/install-evidence.md; personal-adoption's verification half is buildable without the workflow-half export, read off its Deliverable paragraph.

## 2026-10-24 spec — Which worktree paths outside the payload footprint does the consumer smoke read from the tree under test?
- corpus: native/src/emit/installer_smoke
- oracle: grep -rn 'state.root\|porcelain' native/src/emit/installer_smoke
- rev: 877b08b0e583490d1ffc6752b39775049709dab6
- finding: Beyond the footprint the smoke reads docs/index.md, docs/speckit.md, docs/openspec.md, companion/, the GATE_SDK_NATIVE_CRATE directory's worktree bytes and the toplevel knob files. The only status read rooted at the tree under test is the preflight's whole-tree porcelain in roster.rs; no post-run clean assertion covers that tree. WORKFLOW_DIR in consumer.rs names the scratch consumer's directory, so no arm reads the tree's workflow directory.
- inferred: That no arm added after this rev reads the tree's workflow directory.

## 2026-10-24 align — Which worktree paths outside the payload footprint does the consumer smoke read, completing the spec-stage survey?
- corpus: native/src/emit/installer_smoke
- oracle: grep -n 'state.root' native/src/emit/installer_smoke/*.rs
- rev: e12c0d3e096586a9baf5f78d4c50e4a219132d7e
- finding: Beyond the spec-stage list the close-surface arm (carry.rs expected) reads each held kit's canonical spec from the tree under test, a file the payload withholds; report.rs reads the tree's status only to print a diagnostic after a failure. No arm reads the tree's workflow directory: carry.rs's WORKFLOW_DIR names a path inside the scratch consumer.
- inferred: That no read is spelled through a path variable the grep does not match.
