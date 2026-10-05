---
title: Spec Kit
nav_parent: toolkits
nav_child_order: 1
---

# Checkwright for Spec Kit

Add the extension from a Checkwright release, with `X.Y.Z` the release's version:

```sh
specify extension add checkwright --from https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-companion-X.Y.Z.zip
```

Then, in a clean worktree, ask your agent to run the extension's install command, `speckit.checkwright.install`. It installs `prose`, or `complement` or `full` when you ask for one, at the release the extension was tested with, with the Spec Kit recipe applied, and commits. The extension's `speckit.checkwright.check` command runs the battery, and an optional hook offers it after `implement`. The [extension's README](https://github.com/checkwright/checkwright/blob/master/companion/speckit/README.md) lists what it adds.

The Spec Kit recipe prunes `.specify/`, whose scripts are Spec Kit's own, governs the markdown under `specs/`, admits `Task:` as a command, the first word of each line in the task list's example block, and points the task gates at each feature's `tasks.md`, a `[USn]` label resolving to the `### User Story n` heading in the `spec.md` beside it.

The install command runs your system's line from the [install page](install.md#install), and the install's [Requirements](requirements.md) apply.

**Every kit Spec Kit does not replace.** Ask the install command for `complement`, or run your system's line from the install page with:

<!-- companion-complement:begin -->

```text
checkwright init --profile full --without-kit lifecycle-kit --without-kit queue-kit --without-kit doctrine-kit --recipe speckit
```

<!-- companion-complement:end -->

`complement` leaves out each kit whose job Spec Kit already does. [`companion/exclusions.list`](https://github.com/checkwright/checkwright/blob/master/companion/exclusions.list) records the Spec Kit surface each one defers to, and the line carries one `--without-kit` per kit. It needs `bash` 4.3 or later and seeds evidence files under `.workflow/`, beside Spec Kit's own. Drop a kit's `--without-kit` to adopt it later.

**Every kit.** Ask the install command for `full`, or run your system's line from the install page with:

<!-- companion-full:begin -->

```text
checkwright init --profile full --recipe speckit
```

<!-- companion-full:end -->

`full` adds every kit's gates to the gates [the overview](spec-toolkits.md#what-the-gates-catch) names. It needs `bash` 4.3 or later, which stock macOS lacks ([macOS and Linux](install.md#macos-and-linux) has the remedy). It seeds Checkwright's own task queue and evidence files under `.workflow/` beside Spec Kit's own, and writes a doctrine block into your agent file.

Spec Kit has no lifecycle layer ([companion/SPEC.md §The lifecycle layer](companion/SPEC.md#the-lifecycle-layer)).

What the gates catch, what is tested and the limits: [Spec toolkits](spec-toolkits.md).
