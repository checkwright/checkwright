---
title: Install
nav_order: 5
---

# Install and upgrade

Checkwright is distributed git-native and vendored-committed: you copy the kit
directories into your repository and commit them. The gates read tracked files,
and the audit story wants the governance layer inside the reviewed tree — so
the package registries hold the name only, never a dependency channel. There is
nothing to `npm install` or `cargo add`.

Before you vendor, the [footprint page](footprint.md) measures what each kit
adds to a consumer's context budget — the always-loaded and load-triggered cost
per kit, so the adoption decision weighs a number rather than a guess.

## Vendoring the kits

Each kit is a self-contained top-level directory. To adopt one, copy it into
your repo root and wire it in:

1. Copy the kit directory (for example `gate-sdk/`) into your repository.
2. Register the gates it ships in your `gates.list`, where the kit ships gates.
3. Point the kit at your layout through its external configuration — consumers
   never edit vendored kit files, so configuration always lives outside them.
4. Opt each clone into the generated pre-commit hook with
   `bash gate-sdk/bin/install-hooks.sh`.

Where a kit ships adoptable skills, take each as a binding shim by default — a
one-line directive that references the vendored template, so a re-vendor reaches
it and its gates hold the shim thin. Copying the template and filling its slots
is the sanctioned fork: kept for legitimate structural divergence, but you then
own its prose and an upgrade won't reach it. The shipping kit's SPEC owns the
shim↔template contract — see lifecycle-kit's
[stage-skill modes](lifecycle-kit/SPEC.md#templatesskills).

Start with [gate-sdk](gate-sdk/index.md) — the other kits register into its
runner — then add kits in the order the [kit map](index.md#the-kits) lists them.

## Versioning

The repository carries one semver line, applied as git tags, with the kits
moving in lockstep: a kit earns its own version only if it is ever split out
for independent adoption. The first tag rides the launch announcement.

## The upgrade contract

An upgrade runs in two phases.

**Phase A — deterministic.** Replace the vendored kit directories wholesale at
the target tag. Because consumers never edit kit files, this sync loses
nothing. Then regenerate the generated artifacts (the pre-commit hook and the
graph projection).

**Phase B — gate-driven.** Run the full battery. The set of gates that go red
*is* your migration worklist: each red gate names the surface that moved, and
the release note supplies the intent behind the move. Reconcile the red set and
you are current.

Release notes are dated posts. Each carries a tightened-gates section and a
renamed-knobs section — the consumer-owned residue Phase A cannot touch
(gates you have shadowed, templates you have copied out, knob renames in your
own config) is that note's checklist.

## Branch protection

The pre-commit hook is a local backstop a contributor can bypass. Server-side
enforcement makes the battery a required status check: run the gate battery in
CI on every pull request, and mark that check required in your host's
branch-protection settings so a red battery blocks the merge. Keep the
verifier neutral — enforcement that an author can edit is not enforcement.

Back to the [kit map](index.md#the-kits) or [why Checkwright](methodology.md).
