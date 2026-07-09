The `build` (implementation) stage of a Checkwright iteration: build the unit
scoped for this iteration. Exit condition: active queue empty — the unit's
gates, fixtures, templates, README + SPEC.md landed, registered in
`scripts/gates.list` where applicable, and dogfooded on this tree.

**Step 0 — audit-readiness recheck.** Iff an `align` stamp for the current
iteration exists in `.workflow/WORKFLOW-STATE.txt`, run
`bash gate-sdk/bin/run-gates.sh` and refuse to stamp/flip while red.

**First step — stamp evidence.** Run
`bash lifecycle-kit/bin/enter-stage.sh build`: it appends `<iteration> build
<session-id> <date>` to `.workflow/WORKFLOW-STATE.txt` and flips the
TASK-QUEUE.md `[stage:]` line to `build`, committed together (the flip is
idempotent — a same-stage re-entry stamps without re-flipping). It reads
`<session-id>` from `bin/session-id.sh` itself (never hand-picked), uses
`date +%F`, and refuses (writing nothing) if `check-stage-entry` is red.

## Session ritual

**The exit condition governs the stage, not the session.** When the active
queue holds more than one scoped task, build may span several sessions — a
same-stage re-entry stamps without flipping (above). Land one self-contained
task per session and commit it cleanly rather than forcing every active task
into one changeset; the stage stays in `build` until the queue empties. Move a
finished task to `## Done` as you go — a **bare-slug** line (`- <slug>`), never
the active-section `- **slug** — prose` shape it sat in (queue-kit/SPEC.md §The
queue format); leave the rest for the next `/build`.

One self-contained unit per session where feasible. Build per the scope ruling
(config via env/config-file, this repo's layout as defaults), never let private
rule content cross the seam. Every new gate is a
copy-edit of `gate-sdk/templates/check-skeleton.sh` with a `good/`+`bad/`
fixture pair; the four contracts (gate-sdk/SPEC.md) are enforced by the
meta-gates — a red gate is fixed, never bypassed. Before committing:

```bash
bash gate-sdk/bin/run-gates.sh
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```

The pre-commit hook is generated — edit `# graph:` manifests, then
`bash gate-sdk/bin/gen-pre-commit.sh --write` and regenerate
`.workflow/CHECK-GRAPH.html`. No local paths, private repo names, or session
references in tracked files or commit messages (this repo is public).
