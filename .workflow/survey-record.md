# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-11-27 scope — Which deferred entries rank into uninstall-containment-pass, and do any of them cite, supersede or reshape each other?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 9e383706fa3ff027e3b4755d9921042ad1420578
- finding: Read over the Deferred and Icebox sections at the scope stamp. No session or iteration cost rows; the roadmap rows are gated on the observed install or are the service. Every promoted entry has zero inbound edges, and none pairs for supersession: uninstall, user.lock, append_attempt, is_utilization, resets_at, is_iso_day, subject-path, release-assets, binary-only and attestation were searched. is_iso_day has 8 callers across native/src/emit, so a calendar-valid fix reaches them all.
- inferred: none
