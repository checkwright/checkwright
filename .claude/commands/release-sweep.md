Execute the template at lifecycle-kit/templates/skills/release-sweep.md, applying the bindings below.

## Bindings

**inventory-command** — `bash drift-kit/templates/kpi-deprecated-surface.sh`,
the live-marker scan over the `CANON_KIT_DEPRECATION_MARKERS` roster
(`scripts/canon-config.sh`) — the same resolution `check-deprecation-task`
enforces between majors. An empty roster reports `n/a`: nothing to disposition.

**evidence-gate** — the evidence path is `.workflow/release-sweep-evidence.txt`,
committed on the release commit, one disposition block appended per release. No
gate over the stamp file by design (demand-gated) — wire one only if a release
ever ships with the sweep skipped.
