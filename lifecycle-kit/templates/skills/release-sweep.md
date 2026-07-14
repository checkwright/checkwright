The `release-sweep` skill — the deprecation disposition walk at a major
release boundary. Not an iteration stage: it invokes no `enter-stage.sh`, stamps
no `WORKFLOW-STATE.txt`, and can run in any session at a major. Exit condition:
every deprecation marker in the tree has a disposition line stamped for this
release, and the marked surfaces reflect the dispositions taken.

Between majors, `check-deprecation-task` (canon-kit) already holds every
deprecation marker bound to a live decommission task — this sweep forces the
standing inventory to a decision at the boundary the deprecations were promised
against, so a "remove after the next major" never silently rides into the major
after that.

## Session ritual

1. **Inventory the markers.** Walk every deprecation marker over the
   `CANON_KIT_DEPRECATION_MARKERS` roster (the same set the gate resolves and
   `kpi-deprecated-surface` trends). *<inventory-command: the step-1 inventory —
   the roster scan, or your linter's deprecation report.>*
2. **Force a disposition per entry**, one stamped line each into the release
   sweep's evidence file, the `check-lesson-disposition` contract shape at a
   release boundary — `<release> deprecation <disposition> <task-slug> — <the
   marker's lead prefix>`. The disposition set is **decommission** (remove the
   surface now and close its task this release), **carry-forward** (re-justify:
   the surface stays deprecated another cycle — the task stays live and the
   marker keeps pointing at it), or **un-deprecate** (the removal was reversed —
   delete the marker and close its task as won't-do). Clearing is not
   dispositioning: a marker whose promised-against major has *arrived* is a
   decommission or an explicit re-justification, never a silent carry.
3. **Apply the decommissions.** For each decommission, remove the surface and
   its marker in this release's changes, and move its decommission task to
   `Done`. A carry-forward leaves marker and task untouched; an un-deprecate
   deletes the marker and closes the task.
4. *<evidence-gate: the evidence path, the gate decision over the stamp file, and
   the reclaim trigger — the kit ships the disposition shape, not a gate over the
   evidence; name the stamp file's path and any gate you add.>*

The stamp file is operator evidence riding the release commit; nothing in the
kit reads it. The two named slots hold your release-process content — the
inventory command and the evidence-path/gate decision.
