Execute the template at lifecycle-kit/templates/release-sweep.md, applying the bindings below.

## Bindings

**inventory-command** — `bash gate-sdk/bin/run-gates.sh --emit drift-report`, read at its `deprecated surface` row: the marker count over the `CANON_KIT_DEPRECATION_MARKERS` roster (`scripts/canon-config.knobs`), the resolution `check-deprecation-task` enforces between majors. An empty roster reports `n/a`: nothing to disposition.

**evidence-gate** — the evidence path is `.workflow/release-sweep-evidence.txt`, committed on the release commit, one disposition block appended per release. No gate over the stamp file by design (demand-gated) — wire one only if a release ever ships with the sweep skipped.
