Execute the template at lifecycle-kit/templates/skills/build.md, applying the bindings below.

## Bindings

**consistency-gate** — the full gate battery, `bash gate-sdk/bin/run-gates.sh`.

**ritual** — land one self-contained task per session and commit it cleanly
rather than forcing every active task into one changeset; the stage stays in
`build` until the queue empties. Move a finished task to `## Done` as you go —
a bare-slug line (`- <slug>`), never the active-section `- **slug** — prose`
shape (queue-kit/SPEC.md §The queue format); leave the rest for the next
`/build`. Build per the scope ruling (config via env/config-file, this repo's
layout as defaults), never letting private rule content cross the seam. Every
new gate is a copy-edit of `gate-sdk/templates/check-skeleton.sh` with a
`good/`+`bad/` fixture pair; the four contracts (gate-sdk/SPEC.md) are enforced
by the meta-gates — a red gate is fixed, never bypassed. Before committing:

```bash
bash gate-sdk/bin/run-gates.sh
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```

The pre-commit hook is generated — edit `# graph:` manifests, then
`bash gate-sdk/bin/gen-pre-commit.sh --write` and regenerate
`.workflow/CHECK-GRAPH.html`. No local paths, private repo names, or session
references in tracked files or commit messages (this repo is public).
