---
title: Spec Kit and OpenSpec
nav_parent: install
nav_child_order: 1
---

# Gating specs that Spec Kit or OpenSpec writes

Checkwright's `prose` profile governs a repository of documents. This page fits it to a spec tree that [Spec Kit](https://github.com/github/spec-kit) or [OpenSpec](https://github.com/Fission-AI/OpenSpec) writes. Each toolkit gets a recipe: a few lines of gate configuration and, for Spec Kit, one dropped gate, applied with the install.

## What the gates catch

With the recipe applied, four kinds of defect in your specs fail CI, and a pre-commit hook catches them early:

- a relative link to a missing file, or an anchored link to a heading that is not there, caught by `check-md-refs`;
- a section citation such as `specs/001-login/spec.md §Assumptions` naming a heading that is not there, or one title used twice in a spec, caught by `check-spec-pointer`; <!-- citation-link-exempt: an example of the citation form the gate reads, not a citation -->
- a code fence that is never closed, caught by `check-spec-fence-balance`;
- a documented command that runs a script your repository does not have, caught by `check-docs-cmd`.

A citation's path is read from the repository root, so write `specs/001-login/spec.md §Assumptions`, not `spec.md §Assumptions`. The second form is skipped rather than checked. <!-- citation-link-exempt: examples of the two citation forms, not citations -->

These four are document hygiene: they hold your specs together as documents. None of them checks that your code does what a spec says.

`git commit --no-verify` skips the hook, so the guarantee is CI: `init` commits a workflow that runs the battery, and once its check is required, a red battery blocks the merge ([Requiring the CI check](install.md#requiring-the-ci-check)).

## Spec Kit

Add the extension from a Checkwright release, with `X.Y.Z` the release's version:

```sh
specify extension add checkwright --from https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-companion-X.Y.Z.zip
```

Then, in a clean worktree, ask your agent to run the extension's install command, `speckit.checkwright.install`. It installs the `prose` profile at the release the extension was tested with, with the Spec Kit recipe applied, and commits. The extension's `speckit.checkwright.check` command runs the battery, and an optional hook offers it after `implement`. The [extension's README](https://github.com/checkwright/checkwright/blob/master/companion/speckit/README.md) lists what it adds.

The Spec Kit recipe prunes `.specify/`, whose scripts are Spec Kit's own, governs the markdown under `specs/`, and drops `check-fence-command-head`, which reds on the task list's example block of `Task:` lines.

## OpenSpec

Install from your repository's root, in a clean worktree, with your system's line on the [install page](install.md#install) and these arguments. Run the commands `init` prints.

<!-- companion-install:begin -->

```text
checkwright init --profile prose --recipe openspec
```

<!-- companion-install:end -->

The recipe governs the markdown under `openspec/` and exempts `openspec/changes/` from the rule against history words, since a change's delta records what a requirement said before. The install records it, so an upgrade re-applies it with the release it was tested against.

**Keep each title unique within a spec.** OpenSpec specs often repeat a scenario title under two requirements. A citation of the second title would reach the first, so `check-spec-pointer` reds the repeat. Rename one apart, for instance *Idle timeout, web* and *Idle timeout, API*.

Running lifecycle-kit's stage machine as well? Install `full` instead of `prose`, with the lifecycle layer beside the recipe:

<!-- companion-full:begin -->

```text
checkwright init --profile full --recipe openspec --recipe openspec-lifecycle
```

<!-- companion-full:end -->

The layer lets the stage machine read OpenSpec's in-flight changes, so a change touching two capabilities owes the align stage before build. Spec Kit has no lifecycle layer ([companion/SPEC.md §The lifecycle layer](companion/SPEC.md#the-lifecycle-layer)).

## What is tested

Every push to Checkwright's repository installs the profile on a tree in each toolkit's layout, applies the recipe through the install line above, and asserts the battery green. It then plants each of the four defects and asserts the gate that owns it reds. The same run installs the extension with Spec Kit's own command and validates the OpenSpec tree with OpenSpec's own validator. On OpenSpec it also applies the lifecycle layer to a `full` install and asserts that a change touching two capabilities demands the align stage. The versions tested are pinned in [`companion/toolkits.list`](https://github.com/checkwright/checkwright/blob/master/companion/toolkits.list). The recipes, the fixture trees and their design are under [`companion/`](companion/SPEC.md).

## Limits

- Spec Kit ships no validator, so the Spec Kit test tree follows the structure of its templates at the pinned version, and nothing checks it further.
- The extension's two commands and its hook are instructions your agent follows. No test runs an agent through them; the tests run the install line the install command contains.
- A required check stops a skipped hook, not an author who edits the workflow.
- The Windows route is documented and not run.
- A pinned version says nothing about the next one.
