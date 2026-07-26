# Checkwright

The activation path for **Checkwright** — a coding-agent-assisted delivery
methodology shipped as installable kits: a self-testing gate SDK for
prose/spec/config surfaces, an evidence-stamped iteration lifecycle designed
for stateless agent sessions, and token-economics-aware context management.

## What this package is

A one-shot vendoring installer. It copies pinned kit source out of its own
payload into your repository and commits it, then prints the commands that
finish the setup. What governs your tree afterwards is committed, auditable
source you read before you run it.

What it is not: a dependency channel. Nothing resolves at your build time,
nothing is fetched after this package itself, and the installer writes no
dependency reference, no lockfile entry pointing at a registry, and no
install-time lifecycle script. Remove the installer afterwards and the tree it
vendored keeps working — it needs nothing from a package registry again.

## Implementation

Bash. npm is the delivery vehicle, never the implementation: the `bin` entry is
a bash script, so a reader reviewing what they are about to run reads the one
language the rest of the tree is written in, and the linter that governs every
other script in the repository governs these too.

## Requirements

The installer path needs Node, for `npx`. That requirement belongs to this
delivery path alone — the gate battery it vendors does not use Node, and the
manual vendoring path documented on the site needs none. The toolchain the
battery does assert, with its version floors, is on the install page.

## Layout

- `bin/checkwright.sh` — the verb dispatcher, and the package's `bin` entry.
- `lib/` — one file per verb; the dispatcher's roster is this directory.
- `payload/` — the vendored kit source, assembled at pack time from the
  repository's own kit roots. It exists in the published tarball only, never in
  the source tree, so no second copy of any kit is checked in.

## Docs

<https://checkwright.dev> and <https://github.com/checkwright/checkwright>.

Apache-2.0.
