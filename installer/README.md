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
- `lib/common/` — modules the verbs share. The dispatcher's roster is the
  `*.sh` files directly under `lib/`, and that glob does not descend, so a
  shared module cannot be advertised as a verb.
- `payload/` — the vendored kit source, assembled at pack time from the
  repository's own kit roots. It exists in the published tarball only, never in
  the source tree, so no second copy of any kit is checked in.
- `profiles.list` — the profile rosters (below).

## doctor

`checkwright doctor` tells you whether this machine meets the toolchain the
gate battery needs, and it says so in its **exit status** rather than only in
its output: `0` meets the contract, `1` is below it. That is what lets a CI
step or `init`'s own precondition check gate on the answer without parsing a
report — and it is why a below-contract machine is caught before any partial
install rather than halfway through one.

It has two behaviors, selected by where you run it rather than by a flag. Run
anywhere, it reports the toolchain verdict. Run inside a repository that has
been vendored into, it additionally reads `checkwright.lock` and reports the
installed release, the upstream commit it came from, the profile, and the kit
set.

`doctor` defines no floor of its own. It sources the toolchain roster out of
its own `payload/` and renders whatever verdict that roster's predicate
returns, so the contract keeps one owner and this stays a display of it. The
payload copy is the one it reads, never a copy in the tree it is inspecting:
at `init` time nothing has been vendored there yet, so a tree copy would not
exist at the moment the answer is needed.

`doctor` writes nothing, so it has no `--dry-run`. Every verb that does write
has one.

A third exit status, `2`, means the question could not be answered rather than
that the answer was bad: the package carries no payload, or the manifest
carries a schema key this build does not know. A build refuses an unfamiliar
manifest rather than guessing at the shape behind it.

## Profiles

You pick how much of the methodology to meet first. The progression is
`starter`, then `delegation`, then `full`, and it is a containment chain rather
than three unrelated menus: moving up a profile only ever adds, so nothing you
already vendored is taken away or rearranged underneath you.

- **`starter`** is the framework — the gate SDK on its own. You get a battery,
  a generated pre-commit hook, and gates that already red on real defects in
  your own tree without any configuration outside your gates directory.
- **`delegation`** adds every kit whose subject is the agent session itself:
  the stage machine a session runs, the queue it selects work from, the
  evidence a stage produces before it can close, the protocol it follows when
  it spawns, the context budget it runs inside, the permission surface it acts
  through, and the delivery doctrine it follows.
- **`full`** is everything in the payload.

`starter` and `delegation` are rosters in `profiles.list`, because neither
follows from the tree — each is a judgment about what an adopter should meet
first, and the file records the criterion behind each membership beside it.
`full` is derived instead: it is every kit root the payload carries, resolved
at run time, never a list to maintain.

## The manifest

`init` writes `checkwright.lock` at the root of the repository it vendors into,
and that file is tracked like everything else it writes. It is JSON, read with
`jq`, and it is the install-ownership record: what was installed, from which
upstream state, and which files this installer owns. `lib/common/lock.sh` is
its schema owner — the wire key, the accessors, and the hash rule live there,
so the verb that writes the manifest and the verbs that read it cannot drift
apart.

The wire key is versioned (`checkwright-lock v1`, in `CHECKWRIGHT_LOCK_SCHEMA`)
and a build that meets a key it does not know refuses rather than guessing at
the shape behind it.

| Field | Holds | Read by |
| --- | --- | --- |
| `schema` | the versioned wire key | every verb, as its first act — an unknown key is a refusal |
| `version` | the release the payload was cut at | `doctor` reports it; a re-run of `init` compares it against the payload's and refuses a silent downgrade |
| `commit` | the 40-hex commit the payload was assembled from | `doctor` prints it — it is what lets a reviewer resolve the vendored tree to an exact upstream state |
| `profile` | the profile selected | a re-run of `init` re-applies the same profile without asking again |
| `kits` | the vendored kit set | `init`'s re-run file plan, and `doctor`'s installed-set report |
| `files` | each written path with its content hash | `init`'s changed-file detection: a file whose hash still matches is rewritten, one that has changed is reported rather than overwritten |

A recorded hash is `git hash-object`, never `sha256sum`. Not a portability
detail worth burying: macOS ships `shasum` rather than `sha256sum`, and the
answer is not a new tool requirement — git is already something the toolchain
contract asserts, its object hash is content-addressed and stable, so the
manifest's integrity story stays inside the toolchain that contract already
covers.

## Docs

<https://checkwright.dev> and <https://github.com/checkwright/checkwright>.

Apache-2.0.
