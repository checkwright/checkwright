---
title: Spec toolkits
nav_order: 4
nav_id: toolkits
nav_suffix: spec=companion/SPEC.md
---

# Spec toolkits: gating what Spec Kit and OpenSpec write

Checkwright's `prose` profile governs a repository of documents. A recipe fits it to a spec tree that [Spec Kit](https://github.com/github/spec-kit) or [OpenSpec](https://github.com/Fission-AI/OpenSpec) writes: a few lines of gate configuration, applied with the install. Each toolkit has two lines: `prose`, the default, and `full`, every kit. Each toolkit has a page: [Spec Kit](speckit.md), whose extension installs Checkwright with the recipe applied, and [OpenSpec](openspec.md), whose recipe you pass to `init`.

## What the gates catch

With the recipe applied, four kinds of defect in your specs fail CI, and a pre-commit hook catches them early:

- a relative link to a missing file, or an anchored link to a heading that is not there, caught by `check-md-refs`;
- a section citation such as `specs/001-login/spec.md §Assumptions` naming a heading that is not there, caught by `check-spec-pointer`; <!-- citation-link-exempt: an example of the citation form the gate reads, not a citation -->
- a code fence that is never closed, caught by `check-spec-fence-balance`;
- a documented command that runs a script your repository does not have, caught by `check-docs-cmd`.

A citation's path is read from the repository root, so write `specs/001-login/spec.md §Assumptions`, not `spec.md §Assumptions`. The second form is skipped rather than checked. <!-- citation-link-exempt: examples of the two citation forms, not citations -->

`git commit --no-verify` skips the hook, so the guarantee is CI: `init` commits a workflow that runs the battery, and once its check is required, a red battery blocks the merge ([Requiring the CI check](install.md#requiring-the-ci-check)).

## What the gates check against your code

The task gates read your task lists against the tree. A task you ticked that names a file or directory the repository does not have reds `check-task-path-claim`. On Spec Kit, a task labelled with a user story the spec beside it does not define, such as `[US3]` with no User Story 3, reds `check-task-label-resolution`. Both check that a path or a story exists, not that the code does what the task says.

## Which checks each toolkit gets

`prose`: the default line arms and tests it, and `full` keeps it. `full`: only the `full` line does. `opt-in`: an overlap you add beside the toolkit's own check ([the value bar](companion/SPEC.md#the-two-tiers)). `toolkit`: the toolkit's own check, which Checkwright leaves to it.

<!-- support-table:begin -->

| Check | Spec Kit | OpenSpec |
| --- | --- | --- |
| `check-docs-cmd` | prose | prose |
| `check-md-refs` | prose | prose |
| `check-spec-fence-balance` | prose | prose |
| `check-spec-pointer` | prose | prose |
| `check-stage-entry` | — | full |
| `check-task-label-resolution` | prose | — |
| `check-task-path-claim` | prose | prose |
| `openspec validate --all --strict --no-interactive` | — | toolkit |

<!-- support-table:end -->

## What is tested

Every push to Checkwright's repository installs the profile on a tree in each toolkit's layout, applies the recipe through the toolkit's documented install line, and asserts the battery green. On Spec Kit it also installs `full` with the recipe and asserts the battery green and each defect caught. It then plants each defect and asserts the gate that owns it reds. The same run installs the extension with Spec Kit's own command and validates the OpenSpec tree with OpenSpec's own validator. On OpenSpec it also applies the lifecycle layer to a `full` install and asserts that a change touching two capabilities demands the align stage. The versions tested are pinned in [`companion/toolkits.list`](https://github.com/checkwright/checkwright/blob/master/companion/toolkits.list). The recipes, the fixture trees and their design are under [`companion/`](companion/SPEC.md).

## Limits

- Spec Kit ships no validator, so the Spec Kit test tree follows the structure of its templates at the pinned version, and nothing checks it further.
- The extension's two commands and its hook are instructions your agent follows. No test runs an agent through them; the tests run both install lines the install command contains.
- A required check stops a skipped hook, not an author who edits the workflow.
- The Windows route is documented and not run.
- A pinned version says nothing about the next one.
