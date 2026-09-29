---
title: Spec Kit
nav_parent: install
nav_child_order: 2
---

# Checkwright for Spec Kit

Add the extension from a Checkwright release, with `X.Y.Z` the release's version:

```sh
specify extension add checkwright --from https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-companion-X.Y.Z.zip
```

Then, in a clean worktree, ask your agent to run the extension's install command, `speckit.checkwright.install`. It installs the `prose` profile at the release the extension was tested with, with the Spec Kit recipe applied, and commits. The extension's `speckit.checkwright.check` command runs the battery, and an optional hook offers it after `implement`. The [extension's README](https://github.com/checkwright/checkwright/blob/master/companion/speckit/README.md) lists what it adds.

The Spec Kit recipe prunes `.specify/`, whose scripts are Spec Kit's own, governs the markdown under `specs/`, admits `Task:` as a command, the first word of each line in the task list's example block, and points the task gates at each feature's `tasks.md`, a `[USn]` label resolving to the `### User Story n` heading in the `spec.md` beside it.

The install command runs your system's line from the [install page](install.md#install), and that page's [Requirements](install.md#requirements) apply.

**Every kit instead of `prose`.** After the extension's install, or in place of it, run your system's line from the install page with:

<!-- companion-full:begin -->

```text
checkwright init --profile full --recipe speckit
```

<!-- companion-full:end -->

`full` adds every kit's gates to the gates [the overview](spec-toolkits.md#what-the-gates-catch) names. It needs `bash` 4.3 or later, which stock macOS lacks ([macOS and Linux](install.md#macos-and-linux) has the remedy). It seeds Checkwright's own task queue and evidence files under `.workflow/` beside Spec Kit's own, and writes a doctrine block into your agent file.

Spec Kit has no lifecycle layer ([companion/SPEC.md §The lifecycle layer](companion/SPEC.md#the-lifecycle-layer)).

What the gates catch, what is tested and the limits: [Spec Kit and OpenSpec](spec-toolkits.md).
