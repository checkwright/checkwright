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

## init

`checkwright init` vendors the selected profile's kit source out of this
package's own payload into your repository and commits it. One command, and
what governs your tree afterwards is committed source you can read.

Three preconditions, and all three **refuse** rather than warn — a partial
install is the outcome none of them may produce, so every one is checked before
a single file is written:

- **You are inside a git work tree.** The vendored source is meant to be
  committed; that is what makes it auditable rather than merely present.
- **The worktree is clean.** `init` makes one commit, and a dirty tree would
  fold your work into it, leaving a reviewer's diff wider than what was
  actually vendored. `--no-commit` is the valve: it writes and stages the
  files and leaves the commit to you, so an operator who wants to compose the
  change themselves has taken that guarantee on deliberately.
- **`doctor` passes.** A below-contract toolchain blocks *before* any partial
  install, rather than halfway through one — which is why `doctor` ships in the
  same phase as `init` rather than as a later convenience.

It writes the selected profile's kit directories, a `gates.list` seeded with
each kit's starting gates, the config seam files those kits need, and the
manifest. Then it makes **one commit** naming the profile and the version, and
prints the two commands that finish the setup — `gate-sdk/bin/install-hooks.sh`
to opt this clone into the generated hook, and `gate-sdk/bin/run-gates.sh` to
run the battery.

**Re-running is idempotent and non-destructive.** A second `init` reads each
recorded hash from the manifest: a file whose hash still matches is `init`'s to
rewrite, and one that has changed since is **yours** — it is reported and left
alone, never overwritten, unless you pass `--force`. A re-run that finds
nothing to change says so and exits clean; an unchanged tree is the success
case, not an error. A payload older than the recorded install is refused as a
silent downgrade.

`--dry-run` prints the file plan and the manifest that would be written, writes
nothing, and exits 0. Every mutating verb has one.

## What init seeds

Beyond the kit directories themselves, `init` writes the least that makes the
battery green on the tree it just made.

The **config seam is derived, never listed**: a kit's consumer config is
whatever `templates/*-config.sh` it ships, and the destination is always your
gates directory under the file's own name. A kit that grows a config template
is picked up with no edit here.

The **starting gate roster** is the subset a fresh consumer begins with, not
the kit's full roster — the same distinction gate-sdk's own README draws. A
gate whose subject you have not authored yet has nothing to read: canon-kit's
duplication gate wants a glossary, site-kit's wants a docs host, and
lifecycle-kit's two want a stage attestation only a stage session can write. On
a tree that has done nothing wrong those would red on day one, so they are
registered when the surface exists rather than at install. Each kit's README
names the full roster to grow into.

*The honest limit.* That per-kit starting roster is knowledge this installer
holds, and each kit's `smoke/install.sh` holds a second encoding of the same
fact for its own scratch consumer. The consumer smoke catches the drift that
matters — a roster naming a gate that fails to resolve reds it — but not a
kit that adds a zero-config gate this roster never learns about. Collapsing the
two into one per-kit source is filed as queued work, not papered over here.

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
