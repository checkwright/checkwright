---
title: OpenSpec
nav_parent: toolkits
nav_child_order: 2
---

# Checkwright for OpenSpec

Install from your repository's root, in a clean worktree, with your system's line on the [install page](install.md#install) and these arguments. Run the commands `init` prints.

<!-- companion-install:begin -->

```text
checkwright init --profile prose --recipe openspec
```

<!-- companion-install:end -->

The recipe governs the markdown under `openspec/`, exempts `openspec/changes/` from the rule against history words once you register it, since a change's delta records what a requirement said before, and points `check-task-path-claim` at each in-flight change's `tasks.md`. The install records it, so an upgrade re-applies it with the release it was tested against.

**Keep each title unique within a spec.** OpenSpec specs often repeat a scenario title under two requirements. A citation of the second title would reach the first, so set `CANON_KIT_SPEC_POINTER_TITLE_ONCE = on` in `canon-config.knobs` to have `check-spec-pointer` red the repeat. Rename one apart, for instance *Idle timeout, web* and *Idle timeout, API*.

**Every kit OpenSpec does not replace.** The second line installs `complement`:

<!-- companion-complement:begin -->

```text
checkwright init --profile full --without-kit lifecycle-kit --without-kit queue-kit --without-kit doctrine-kit --recipe openspec
```

<!-- companion-complement:end -->

`complement` leaves out each kit whose job OpenSpec already does. [`companion/exclusions.list`](https://github.com/checkwright/checkwright/blob/master/companion/exclusions.list) records the OpenSpec surface each one defers to, and the line carries one `--without-kit` per kit. It needs `bash` 4.3 or later and seeds evidence files under `.workflow/`, beside OpenSpec's own. Drop a kit's `--without-kit` to adopt it later. It applies no lifecycle layer, since it leaves the stage machine to OpenSpec.

**Every kit.** The third line installs `full`:

<!-- companion-full:begin -->

```text
checkwright init --profile full --recipe openspec --recipe openspec-lifecycle
```

<!-- companion-full:end -->

It costs what the [Spec Kit](speckit.md) `full` line costs: `bash` 4.3 or later, and Checkwright's own workflow files beside OpenSpec's. The line also applies the lifecycle layer, which lets lifecycle-kit's stage machine read OpenSpec's in-flight changes: once you run the stage machine, a change touching two capabilities owes the align stage before build.

What the gates catch, what is tested and the limits: [Spec toolkits](spec-toolkits.md).
