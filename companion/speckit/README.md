# Checkwright for Spec Kit

This extension puts [Checkwright](https://checkwright.dev)'s gates over a Spec Kit repository. Once installed, a broken link, a dangling section citation, an unclosed fence or a documented command that runs a missing script fails at commit and in CI instead of reaching review.

## Install

Add the extension from a Checkwright release, with `X.Y.Z` the release's version:

```sh
specify extension add checkwright --from https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-companion-X.Y.Z.zip
```

Then, in a clean worktree, ask your agent to run the extension's install command, `speckit.checkwright.install`.

## What it adds

- **`speckit.checkwright.install`** installs Checkwright's prose profile with the one-line install, `init --profile prose`, pinned to the release this extension version was tested with. It then applies the Spec Kit recipe in `recipe/`, which prunes `.specify/` from the gates, governs the markdown under `specs/`, and drops the one gate a Spec Kit task list trips. It runs the battery and commits.
- **`speckit.checkwright.check`** runs the battery and reports each red gate with its finding and remedy.
- **An optional `after_implement` hook** that offers to run the check once an implementation finishes.

The install needs git, and on Windows Git for Windows. It makes two commits: the install, then the recipe.

## Learn more

- [Spec Kit and OpenSpec](https://checkwright.dev/spec-toolkits.html): what the gates catch in a spec tree, which versions are tested, and the limits.
- [The Checkwright repository](https://github.com/checkwright/checkwright), where this extension lives under `companion/speckit/`.
