---
title: Spec Kit and OpenSpec
nav_parent: install
nav_child_order: 1
---

# Gating specs that Spec Kit or OpenSpec writes

Checkwright's `prose` profile governs a repository of documents. This page fits it to a spec tree that [Spec Kit](https://github.com/github/spec-kit) or [OpenSpec](https://github.com/Fission-AI/OpenSpec) writes. Each toolkit gets a recipe: a few lines of gate configuration and, for Spec Kit, one dropped gate, applied once after the install.

## What the gates catch

With the recipe applied, four kinds of defect in your specs fail at commit and in CI:

- a relative link to a missing file, or an anchored link to a heading that is not there, caught by `check-md-refs`;
- a section citation such as `specs/001-login/spec.md §Assumptions` naming a heading that is not there, or one title used twice in a spec, caught by `check-spec-pointer`; <!-- citation-link-exempt: an example of the citation form the gate reads, not a citation -->
- a code fence that is never closed, caught by `check-spec-fence-balance`;
- a documented command that runs a script your repository does not have, caught by `check-docs-cmd`.

A citation's path is read from the repository root, so write `specs/001-login/spec.md §Assumptions`, not `spec.md §Assumptions`. The second form is skipped rather than checked. <!-- citation-link-exempt: examples of the two citation forms, not citations -->

## Spec Kit

Add the extension from a Checkwright release, with `X.Y.Z` the release's version:

```sh
specify extension add checkwright --from https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-companion-X.Y.Z.zip
```

Then, in a clean worktree, ask your agent to run the extension's install command, `speckit.checkwright.install`. It installs the `prose` profile at the release the extension was tested with, runs the recipe block below with the Spec Kit recipe, and commits. The extension's `speckit.checkwright.check` command runs the battery, and an optional hook offers it after `implement`. The [extension's README](https://github.com/checkwright/checkwright/blob/master/companion/speckit/README.md) lists what it adds.

The Spec Kit recipe prunes `.specify/`, whose scripts are Spec Kit's own, governs the markdown under `specs/`, and drops `check-fence-command-head`, which reds on the task list's example block of `Task:` lines.

## OpenSpec

Install the `prose` profile from your repository's root, in a clean worktree. On macOS and Linux:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --profile prose
```

On Windows, use the script-block form on the [install page](install.md#windows), with `init --profile prose`. Run the commands `init` prints.

Then fetch the OpenSpec recipe from the repository at the release you installed, with `X.Y.Z` the version `init` printed:

```sh
v=X.Y.Z
recipe="$(mktemp -d)"
for f in canon-config.knobs unregister.list; do
  curl -fsSLo "$recipe/$f" "https://raw.githubusercontent.com/checkwright/checkwright/v$v/companion/openspec/recipe/$f"
done
```

and run the recipe block below. The recipe governs the markdown under `openspec/` and exempts `openspec/changes/` from the rule against history words, since a change's delta records what a requirement said before.

**Keep each title unique within a spec.** OpenSpec specs often repeat a scenario title under two requirements. A citation of the second title would reach the first, so `check-spec-pointer` reds the repeat. Rename one apart, for instance *Idle timeout, web* and *Idle timeout, API*.

## The recipe block

From the repository root, in `sh` (on Windows, Git for Windows' `sh`), with `recipe` set to the recipe directory, run:

<!-- companion-recipe:begin -->

```sh
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

It appends the recipe's knob lines to your gate configuration, drops the gates the recipe names from `scripts/gates.list`, regenerates the pre-commit hook and the coupling graph, and runs the battery. On green it commits the `scripts/` directory; on red it stops before the commit and each red gate names its fix.

## What is tested

Every push installs the profile on a tree in each toolkit's layout, applies the recipe with the block above, and asserts the battery green. It then plants each of the four defects and asserts the gate that owns it reds. The same run installs the extension with Spec Kit's own command and validates the OpenSpec tree with OpenSpec's own validator. The versions tested are pinned in [`companion/toolkits.list`](https://github.com/checkwright/checkwright/blob/master/companion/toolkits.list). The recipes, the fixture trees and their design are under [`companion/`](companion/SPEC.md).

## Limits

- Spec Kit ships no validator, so the Spec Kit test tree follows the structure of its templates at the pinned version, and nothing checks it further.
- The extension's two commands and its hook are instructions your agent follows. No test runs an agent through them; the tests run the recipe block they contain.
- The Windows route is documented and not run.
- A pinned version says nothing about the next one.
