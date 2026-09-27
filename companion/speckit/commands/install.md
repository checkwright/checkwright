---
description: "Install Checkwright's gates and apply the Spec Kit recipe"
---

## User Input

$ARGUMENTS

## Steps

Work from the repository root, in a clean worktree. The install makes one commit and refuses uncommitted changes, so ask the user to commit or stash them first.

**Install the release this extension was tested with.** Read `extension.version` in `.specify/extensions/checkwright/extension.yml` and put it in place of `X.Y.Z` in the line for the host. On macOS and Linux:

```sh
curl -fsSL https://checkwright.dev/install.sh | CHECKWRIGHT_VERSION=X.Y.Z sh -s -- init --profile prose
```

On Windows, in PowerShell:

```powershell
$env:CHECKWRIGHT_VERSION = 'X.Y.Z'; & ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --profile prose
```

**Finish the setup.** Run each command `init` printed under `next:`.

**Apply the recipe.** Run this block from the repository root in `sh`, which on Windows is Git for Windows' `sh`. It appends the recipe's knob lines to the gate config, drops the gate the recipe names, regenerates the hook and the coupling graph, runs the battery and commits:

<!-- companion-recipe:begin -->

```sh
recipe=.specify/extensions/checkwright/recipe
(
  set -e
  gates=./scripts/checkwright-gates
  for f in "$recipe"/*.knobs; do
    if [ -f "$f" ]; then sed '/^#/d' "$f" >> "scripts/${f##*/}"; fi
  done
  awk 'FILENAME == ARGV[1] { if ($0 !~ /^#/) drop[$0] = 1; next } !($0 in drop)' \
    "$recipe/unregister.list" scripts/gates.list > scripts/gates.list.new
  mv scripts/gates.list.new scripts/gates.list
  "$gates" --emit git-hooks --write
  "$gates" --emit graph > scripts/CHECK-GRAPH.html
  "$gates" --run
  git add scripts
  git commit -q -m "chore: apply the Checkwright recipe"
)
```

<!-- companion-recipe:end -->

**Report the result.** On a green battery, say so and name the commit. On a red one, list each failing gate with its finding and its `help:` line, and stop.

Never edit a file under a vendored kit directory, drop a gate the recipe does not name, or commit with `--no-verify`.
