The `build` (extraction) stage of a Checkwright iteration: extract the kit
scoped for this iteration. Exit condition: active queue empty — the kit's
gates, fixtures, templates, README + SPEC.md landed, registered in
`scripts/gates.list` where applicable, and dogfooded on this tree.

**Step 0 — audit-readiness recheck.** Iff an `align` stamp for the current
iteration exists in `.workflow/WORKFLOW-STATE.txt`, run
`bash gate-sdk/bin/run-gates.sh` and refuse to stamp/flip while red.

**First step — stamp evidence.** Append `<iteration> build <session-id>
<date>` to `.workflow/WORKFLOW-STATE.txt`; flip the TASK-QUEUE.md `[stage:]`
line to `build` in the same commit (flip only on arrival — a same-stage
re-entry stamps without flipping). Take `<session-id>` from
`bash lifecycle-kit/bin/session-id.sh` (it reads the id from the newest
transcript — never hand-pick it); `<date>` is `date +%F`.

## Session ritual

**The exit condition governs the stage, not the session.** When the active
queue holds more than one scoped task, build may span several sessions — a
same-stage re-entry stamps without flipping (above). Land one self-contained
task per session and commit it cleanly rather than forcing every active task
into one changeset; the stage stays in `build` until the queue empties. Move a
finished task to `## Done` as you go; leave the rest for the next `/build`.

One kit extraction per session where feasible. Copy the platform mechanism,
de-hardcode per the scope ruling (config via env/config-file, platform values
as defaults), never let rule content cross the seam. Every new gate is a
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
