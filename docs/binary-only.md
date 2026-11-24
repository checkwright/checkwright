---
title: Binary-only CI
nav_parent: install
nav_child_order: 4
description: Running Checkwright's gates in a read-only CI container from the release binary alone, with no vendored kit and no init.
---

# Running gates from the binary alone

## When this page applies

This page is for a CI job holding the gate binary and a checkout of your repository, and nothing else: no `init` has run and no kit is vendored in the tree. The checkout may be read-only. A tree `init` installed runs its battery through the CI action instead ([`installer/SPEC.md` §The CI action](installer/SPEC.md#the-ci-action)).

Both forms below are held by [`gate-sdk/gate-tests/binary-only-recipe.test.sh`](https://github.com/checkwright/checkwright/blob/master/gate-sdk/gate-tests/binary-only-recipe.test.sh), which runs them on a scratch checkout holding no kit and reds when either stops doing what this page says.

## Getting the binary

Download and verify the Release tarball as [Installing step by step](manual-install.md) does, into `$cw`, outside the checkout, and run no `init`. The binary is `package/payload/artifact/<target>/checkwright-gates`, `<target>` naming your system ([`installer/SPEC.md` §Layout](installer/SPEC.md#layout)); [Requirements](requirements.md#supported-systems) lists the systems a binary is published for. The commands below take it from `cwg`, set here for a Linux runner on x86_64:

```sh
cwg="$cw/package/payload/artifact/x86_64-unknown-linux-gnu/checkwright-gates"
```

## Running one gate

From the checkout's root:

```sh
"$cwg" check-spec-fence-balance
```

This direct form runs one gate compiled into the binary, with no registry, no descriptor and no knob file. Exit `0` is clean, `1` a finding, and `2` a gate that could not run ([`gate-sdk/SPEC.md` §Fail-closed contract](gate-sdk/SPEC.md#fail-closed-contract)). It writes nothing, so it runs on a read-only checkout, and it prints its verdict with no Static Analysis Results Interchange Format (SARIF) log.

These gates need neither a kit nor a knob:

- `check-spec-dod-singleton`: no `SPEC.md` carries its Definition of Done twice, or none ([`canon-kit/SPEC.md` §check-spec-dod-singleton](canon-kit/SPEC.md#check-spec-dod-singleton)).
- `check-amendment-update-target`: each amendment's update targets cite deltas it defines ([`canon-kit/SPEC.md` §check-amendment-update-target](canon-kit/SPEC.md#check-amendment-update-target)).
- `check-amendment-retired-spelling`: a spelling an amendment retires survives only where its roster says ([`canon-kit/SPEC.md` §check-amendment-retired-spelling](canon-kit/SPEC.md#check-amendment-retired-spelling)).
- `check-spec-fence-balance`: no governed markdown file ends inside an open code fence ([`canon-kit/SPEC.md` §check-spec-fence-balance](canon-kit/SPEC.md#check-spec-fence-balance)).

A gate that scans for YAML takes the directory to scan as its argument, so `"$cwg" check-action-pinning ci` reads `ci` and nothing outside it ([`gate-sdk/SPEC.md` §check-action-pinning](gate-sdk/SPEC.md#check-action-pinning)).

## Running a set with a SARIF verdict

A set runs through a registry, and the registry can sit outside the checkout. Make a directory there holding a `gates.list`, one gate name per line, and beside it each member's `.gate` descriptor, copied from the tarball's `package/payload/<kit>/checks/`:

```sh
gates="$RUNNER_TEMP/gates" \
  && mkdir -p "$gates" \
  && printf '%s\n' check-spec-dod-singleton check-spec-fence-balance > "$gates/gates.list" \
  && cp "$cw/package/payload/canon-kit/checks/check-spec-dod-singleton.gate" \
       "$cw/package/payload/canon-kit/checks/check-spec-fence-balance.gate" "$gates/" \
  && GATE_SDK_SARIF_FILE="$RUNNER_TEMP/gates.sarif" GATE_SDK_TMP_DIR="$RUNNER_TEMP/gates-tmp" \
     "$cwg" --run --gates-dir "$gates"
```

`$RUNNER_TEMP` is GitHub Actions' scratch directory; any directory outside the checkout serves. Both paths the run writes are absolute and outside the checkout, so the checkout's status stays empty and the SARIF log is where your upload step reads it ([`gate-sdk/SPEC.md` §run-gates](gate-sdk/SPEC.md#run-gates)).

The descriptor is the cost of this form: one file copied per member. A name with no descriptor beside it fails as unresolved even when the binary carries that gate, because the descriptor is where the member's manifest, its install disposition and the knob that arms it are declared, and a run that skipped them would report a disarmed gate as coverage.

## Configuring a gate without writing to the tree

A gate reads its kit's knobs from the file `<KIT>_KNOB_FILE` names, such as `CANON_KIT_KNOB_FILE` for the gates above. It may name a file the checkout tracks or one outside it, and a name that resolves to no file is exit 2 rather than a silent default ([`gate-sdk/SPEC.md` §The knob file](gate-sdk/SPEC.md#the-knob-file)).

A top-level directory holding `checks/` or `smoke/` reads as a kit, and a gate that leaves a kit's own files out of its scan, `check-spec-dod-singleton` among them, skips it. Where such a directory is yours, set `GATE_SDK_KIT_DIRS` to `gate-sdk` alone so it is scanned like the rest of the tree ([`gate-sdk/SPEC.md` §Layout and configuration](gate-sdk/SPEC.md#layout-and-configuration)).

## What the container must provide

- **git**, the one tool every host needs ([`gate-sdk/SPEC.md` §The adopter constraints](gate-sdk/SPEC.md#the-adopter-constraints)).
- **A git worktree** for `check-amendment-retired-spelling` where an amendment declares a spelling, since it lists the tree through git. Outside one it exits 2.
- **A checkout git trusts.** git refuses a repository owned by a user other than the job's as dubious ownership. The remedy is git's own, the [`safe.directory`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-safedirectory) setting; this is git's documented behavior, not measured here.
- **A task queue for `check-amendment-queue`.** It exits 2 on a tree with no `TASK-QUEUE.md`, so don't run it on one.

## Which files a gate reads

A gate that reads CI workflows or commit messages is opt-in: registering it is your choice, and `init` registers none of them ([`gate-sdk/SPEC.md` §The install disposition](gate-sdk/SPEC.md#the-install-disposition)). An action gate reads the directory you hand it, and a commit-message gate reads only the message being committed, handed to it by the `commit-msg` hook.

A gate in a registry scans from the checkout's root, since a `gates.list` line carries no argument. To scope an action gate there, leave it out and register a script of your own under a name of its own, which runs the direct form with your directory.
