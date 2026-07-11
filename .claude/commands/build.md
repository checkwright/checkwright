Execute the template at lifecycle-kit/templates/skills/build.md, applying the bindings below.

## Bindings

**consistency-gate** — the full gate battery, `bash gate-sdk/bin/run-gates.sh`.

**ritual** — land one self-contained task per session and commit it cleanly
rather than forcing every active task into one changeset; the stage stays in
`build` until the queue empties. Move a finished task to `## Done` as you go —
a bare-slug line (`- <slug>`), never the active-section `- **slug** — prose`
shape (queue-kit/SPEC.md §The queue format); leave the rest for the next
`/build`. A completed task's amendment merges per canon-kit/SPEC.md §Merging
an amendment (on task completion). Build to the scope ruling — config surface
with this repo's layout as defaults, the provenance seam uncrossed. The
gate-authoring contract, the generated-artifact regeneration, the whole
pre-commit battery to run first (with the touched kit's fixture suite), and
public-repo hygiene are always-loaded under CLAUDE.md §This repo is governed by
its own kits — followed there, never restated in this shim.
