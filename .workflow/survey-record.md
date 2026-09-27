# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-27 scope — Which deferred entries gate the catalog submission, and what does the rest of the queue say about them?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 12600154cf63fd0398ba1ee810dcb33e4130a9ad
- finding: Over the deferred section and icebox. Preconditions: windows-fresh-fixture-stub then crate-tests-windows-flip (blocked-by edge), linux-glibc-artifacts, catalog-landing-docs-polish, spec-toolkits-guarantee; design-partner-preview's observed install also gates it (operator hours). Supersession: musl-smoke-build-wrapper and musl-dev-binary reshaped by linux-glibc-artifacts; install-platform-release-gap reshaped by it. foreign-spec-lifecycle-unowned does not reach the prose profile (gate-sdk + canon-kit), so it is outside the catalog slice.
- inferred: none
