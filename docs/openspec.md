---
title: OpenSpec
nav_parent: install
nav_child_order: 3
---

# Checkwright for OpenSpec

Install from your repository's root, in a clean worktree, with your system's line on the [install page](install.md#install) and these arguments. Run the commands `init` prints.

<!-- companion-install:begin -->

```text
checkwright init --profile prose --recipe openspec
```

<!-- companion-install:end -->

The recipe governs the markdown under `openspec/`, exempts `openspec/changes/` from the rule against history words, since a change's delta records what a requirement said before, and points `check-task-path-claim` at each in-flight change's `tasks.md`. The install records it, so an upgrade re-applies it with the release it was tested against.

**Keep each title unique within a spec.** OpenSpec specs often repeat a scenario title under two requirements. A citation of the second title would reach the first, so `check-spec-pointer` reds the repeat. Rename one apart, for instance *Idle timeout, web* and *Idle timeout, API*.

**Every kit instead of `prose`.** The second line installs `full`:

<!-- companion-full:begin -->

```text
checkwright init --profile full --recipe openspec --recipe openspec-lifecycle
```

<!-- companion-full:end -->

It costs what the [Spec Kit](speckit.md) `full` line costs: `bash` 4.3 or later, and Checkwright's own workflow files beside OpenSpec's. The line also applies the lifecycle layer, which lets lifecycle-kit's stage machine read OpenSpec's in-flight changes: once you run the stage machine, a change touching two capabilities owes the align stage before build.

What the gates catch, what is tested and the limits: [Spec Kit and OpenSpec](spec-toolkits.md).
