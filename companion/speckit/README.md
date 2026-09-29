# Checkwright for Spec Kit

This extension puts [Checkwright](https://checkwright.dev)'s gates over a Spec Kit repository. Once installed, a broken link, a dangling section citation, an unclosed fence, a documented command that runs a missing script or a ticked task naming a file the repository does not have fails CI instead of reaching review, and a pre-commit hook catches it early.

## Install

Add the extension from a Checkwright release, with `X.Y.Z` the release's version:

```sh
specify extension add checkwright --from https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-companion-X.Y.Z.zip
```

Then, in a clean worktree, ask your agent to run the extension's install command, `speckit.checkwright.install`.

## What it adds

- **`speckit.checkwright.install`** installs Checkwright's `prose` profile, or `full` when you ask for it, with the one-line install, `init --profile prose --recipe speckit`, pinned to the release this extension version was tested with, with the Spec Kit recipe applied, which prunes `.specify/` from the gates, governs the markdown under `specs/`, admits the `Task:` lines of a Spec Kit task list's example block, and holds each feature's `tasks.md` to the tree and to its `spec.md`'s user stories. It commits and runs the battery. `full` adds every kit's gates, needs `bash` 4.3 or later, and seeds Checkwright's task queue and evidence files beside Spec Kit's; the command says so before it runs.
- **`speckit.checkwright.check`** runs the battery and reports each red gate with its finding and remedy.
- **An optional `after_implement` hook** that offers to run the check once an implementation finishes.

The install needs git, and on Windows Git for Windows. It makes one commit.

## Learn more

- [Checkwright for Spec Kit](https://checkwright.dev/speckit.html): the install, the `full` line that installs every kit, and a link on to what the gates catch in a spec tree, what makes CI the guarantee, which versions are tested, and the limits.
- [The Checkwright repository](https://github.com/checkwright/checkwright), where this extension lives under `companion/speckit/`.

## License

Licensed under Apache-2.0, with the text in `LICENSE` beside this file in an installed copy and at the repository root in the source tree.
