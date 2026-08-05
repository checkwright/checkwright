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

This package reaches a tree over two transports, and each carries its own
requirement. Fetched from npm it needs Node, for `npx`. Fetched as the tarball
attached to a GitHub Release it needs none — `curl`, `tar`, and `sha256sum`,
then `bash package/bin/checkwright.sh init`. Both requirements belong to a
delivery path alone: the gate battery this vendors does not use Node, no
delivery-path tool joins the toolchain roster, and the manual vendoring path
documented on the site needs neither. The toolchain the battery does assert,
with its version floors, is on the install page.

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

**No selection builds.** Not "no profile that happens to carry only shell
gates" — no selection, ever. `init` writes files and compiles nothing, so it
asks for no toolchain and cannot fail on one. A gate whose implementation is a
compiled subcommand reaches you as a prebuilt binary picked for your platform
and checked against a published digest before it is written, which is what makes
zero build step a property of `init` itself rather than of which profile you
picked (gate-sdk/SPEC.md §Porting a gate to the binary substrate, criterion 5).

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
alone, never overwritten, unless you pass `--force`. That does not expire at the
next upgrade: a file reported as changed stays on `init`'s roster at the hash
`init` wrote there, so every later run reads it the same way and reports it
again. It does not expire when a release **stops shipping** the file either.
`init` owns what it wrote there, not what the current payload happens to carry,
so the path stays on the roster at that hash and a release that later re-adds it
meets the same protection rather than a clean slate — which is the one window in
which an unowned path could otherwise be written straight over your edits. A
re-run that finds
nothing to change says so and exits clean; an unchanged tree is the success
case, not an error. A payload older than the recorded install is refused as a
silent downgrade — `--force` covers that refusal too, which is what makes a
rollback a thing you asked for rather than a thing that happened to you.
`--force` means the same thing in both places: overwrite what `init` would
otherwise protect.

`--dry-run` prints the file plan and the manifest that would be written, writes
nothing, and exits 0. Every mutating verb has one.

## What init seeds

Beyond the kit directories themselves, `init` writes the least that makes the
battery green on the tree it just made.

The **config seam is derived, never listed**: a kit's consumer config is
whatever `templates/*-config.sh` it ships, and the destination is always your
gates directory under the file's own name. A kit that grows a config template
is picked up with no edit here.

**Everything `init` seeds takes one of two disciplines, and which one follows
from whether `init` keeps rewriting the file.** A surface `init` creates once and
then leaves to you — the queue file, the agent file, the evidence manifests, the
workflow-state file — is written **only when it is absent**, so a re-run never
disturbs a tree that has grown since. A surface `init` rewrites on every run —
every kit's config seam, and gate-sdk's `msg-patterns.list` — is **claimed
before it is written**, on the same non-destructive path as any vendored file:
the recipe *plans* the copy and `init` performs it, so your edit is compared
against the recorded hash while it is still on disk. Landing the copy first would
destroy the very evidence the comparison is made from, and these are precisely
the files whose whole purpose is to be edited by you — which is why the ordering
is stated here rather than left to each recipe.

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

## The gate binary

A gate whose implementation is a compiled subcommand needs that binary on disk
before it can run, and `init` is what puts it there. It **selects, verifies and
places** — it never builds, and it never fetches. Everything below sits inside
the irreducible bootstrap the vendoring ruling leaves outside the binary
(gate-sdk/SPEC.md §Porting a gate to the binary substrate): the binary cannot
select itself, so something must resolve the platform, verify the artifact, and
place it. Each step is deliberately small enough to be written twice, in bash
and in PowerShell.

**Platform resolution is derived and never stored.** `target_of_host()` maps
`uname -s` and `uname -m` to one Rust target triple, and to the empty string on
a host that maps to none. It runs once per `init`, after the profile's kit set
resolves and before anything is written. The result stays a local: a stored copy
would be a second source for a fact the host already answers, and it is stale
the first time a vendored tree moves between machines — the case that matters
most, since a vendored tree is shared by construction. Two fields rather than
`uname -a` because that is the smallest input that answers the question, and it
is what a PowerShell half can answer without parsing prose. Nothing reads the
kernel version, so nothing collects it.

**Selection has three outcomes, and collapsing any two is the defect.** The
payload carries the target roster verbatim beside the artifacts
(gate-sdk/SPEC.md §Consumer payload), and `init` reads it rather than inferring
support from a directory's presence:

| the host resolves to | the payload holds | outcome |
| --- | --- | --- |
| a target **not** in the roster | — | **omit and declare** — a supported outcome, not a failure |
| a target **in** the roster | its binary and sidecar | verify, then write |
| a target **in** the roster | nothing, or half the pair | **refuse** — the payload is broken |

The third row is the whole reason the roster is read. Without it `init` cannot
tell a platform that was never committed to from a platform that was committed
to and whose artifact went missing, and reading the second as the first turns a
publisher defect into a silently smaller green battery. A payload assembled with
no artifacts at all carries no `artifact/` directory, so it reads as the first
row and never as a payload whose every target went missing. The refusal is the
one place this path fails an install, and it belongs there: a missing artifact
for a **declared** target is a defect the adopter cannot act on and must not
inherit silently.

**The digest is verified before anything is written.** `init` computes the
artifact's SHA-256 and compares it against the sidecar that travelled with it,
and only then writes. The ordering is the whole of it: a consumer who cannot
read the gate has nothing else standing between them and a substituted binary,
and a post-write check has already put it on disk. A mismatch refuses — never a
warning, never a write.

`sha256sum` is tried first, then `shasum -a 256`, because stock macOS ships the
second and not the first. When **neither** resolves the install proceeds and the
artifact is omitted rather than written unverified. Both halves are load-bearing:
never write what was not verified, and never fail an install over something the
adopter did not choose.

*The honest bound, stated so no surface overclaims it.* The digest travelled
inside the same payload as the artifact, so it catches corruption and a
substitution made to the artifact alone — not a compromised publisher, which no
in-payload value can. What raises it above a self-check is that the identical
bytes are published on the Release, so a human can cross-check the value out of
band. The claim is **verified against a published digest**, never *reproducible*.

**Omission is declared and counted.** An omitted member rides the registry
rather than a new file: `init` writes `# omitted: <name> <reason>` into the
consumer's `gates.list` in place of the member's name — a comment line the
runner already strips from the live set. The record then sits in the consumer's
tracked history where a reviewer reads it, instead of scrolling past in
install-time stdout, and a re-run on a machine that has since gained a hasher
converts it back into a live member with no hand edit. Two reason tokens,
because there are two remedies:

- `substrate-unavailable` — this host's platform has no declared artifact. There
  is no adopter action; the platform is not in the support roster.
- `digest-unverifiable` — an artifact exists but no hasher does. Install
  `sha256sum` or `shasum` and re-run `init`.

A third token would need a third remedy to earn its place. Which members are
affected is derived from the payload, never maintained here: a starting-roster
gate the payload declares as `<kit>/checks/<name>.gate` — and does not also ship
as a shell script — is one that dispatches to the binary. `run-gates.sh` reports
the count and remedy on a line of its own beside its summary
(gate-sdk/SPEC.md §run-gates), and `doctor` reports it against the reason that
caused it.

**The install location has one owner.** The binary is written to your gates
directory beside the `gates.list` seeded there, and `init` sets
`GATE_SDK_NATIVE_BIN` to that path in `<gates-dir>/gate-sdk-config.sh` — the
optional persistent config seam gate-sdk's library already sources when it
exists (gate-sdk/SPEC.md §Layout and configuration). `init` creates that file
when it places an artifact; gate-sdk ships no config template, and adding one
would be worse than writing the file here, since the template seam copies
unconditionally and would overwrite an adopter's own overrides on every re-run.
The knob's own default is unchanged and still names the crate's build output,
because it is a **stable relative path** on purpose: the generated hook persists
the emitted argv, so a machine-specific path baked into a tracked hook would make
the graph artifact's freshness comparison machine-dependent.

**Ordering is load-bearing.** The config seam and the binary are both in place
before the pre-commit hook is generated, because the generator resolves each
member's invocation argv and a `.gate` member resolves to this binary. A hook
generated first would resolve a dispatch it cannot make.

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
set — plus, where one was installed, the gate binary's target re-verified
against its recorded digest **in place**, and any omitted members against the
reason that caused them (§The gate binary).

Those last two **report without setting the exit status**, and the asymmetry is
deliberate rather than lenient. The status is the toolchain contract, and `init`
gates its own precondition on it — so reddening here for a swapped or missing
binary would block the `init` re-run that is the finding's own remedy. A binary
that cannot be dispatched to is caught where it is dispatched from: the battery
treats it as a harness error rather than a skip.

Not setting the status is not the same as staying quiet about it, and the
verdict line says which state it is in. A run carrying an artifact finding
reports the toolchain clean **and names the finding**, rather than signing off
as plainly clean — a `doctor` that printed `DIGEST MISMATCH` above and `clean`
below would be the surface teaching a reader to disbelieve the last line.

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
| `files` | `init`'s ownership roster — each path it has written, at the content hash it last wrote there, until the file leaves the tree | `init`'s changed-file detection: a file whose hash still matches is rewritten, one that has changed is reported rather than overwritten, and stays on the roster so the next run reads it the same way, whether or not the running release still ships that path |
| `artifact` | the gate binary's `target` and its SHA-256 `digest`, or absent | `doctor` reports the target and re-verifies the digest in place; a re-run of `init` compares the target against this host and skips the rewrite while the digest still holds |

**A recorded hash is what `init` last wrote at that path** — on whichever run
last wrote it — and not the state of the tree at the end of the current run. The
two readings coincide for every path `init` rewrites and part company for exactly
one class: a path `init` left alone because you had edited it. That path stays in
`files` at the hash `init` put there, carried forward from the previous manifest
rather than recomputed, so the next run still has something to compare your
content against, still finds it different, and still leaves it alone. Editing a
file changes who may write it; it never changes whether `init` is tracking it.

**A path leaves the roster when the file leaves the tree, and at no other
moment.** `init` owns a path because it wrote the file there, so only the file's
disappearance can end that. A release that stops shipping the path does not: the
file is still on disk, it may carry your edits, and disowning it is exactly what
would let a later release re-adding the same path write straight through them.

**A relinquished path is an ordinary entry, not a state of its own.** When a
release stops shipping something `init` created, nothing visits that path on the
run, so it is carried forward at the hash `init` last wrote there — the same
carry-forward, the same `files` row, the same protection every other entry gets.
There is no *once ours, now relinquished* state for a reader to learn. Nor can
the roster grow without bound from it: the existence test is already the reaper,
so `files` is bounded by the files `init` created that still exist, which is the
ownership set itself. What `init` never does is delete, so a path no release
ships any more stays on disk and stays yours to remove.

The hash carried forward is the one `init` wrote, not the one this payload would
have written. Either would protect the file, so protection does not decide it —
the **revert** does. Restore the file to what `init` put there and it is `init`'s
to rewrite again, which works only when the recorded hash is what `init` actually
wrote; recording an intended write would report the path as changed forever and
would record a write that never happened.

That is a change of meaning, not of shape, so the wire key stays
`checkwright-lock v1`. A reader built before it meets one of these entries, finds
a hash that disagrees with the tree, and does exactly what it does today: reports
the file changed and leaves it alone. A retained relinquished entry passes the
same test even more quietly — that reader's payload does not ship the path, so it
never asks whether it may write there and does nothing with the entry at all. The
old behavior on the new data is the behavior the new meaning wants, so there is
nothing for a version key to protect.

A recorded **`files`** hash is `git hash-object`, never `sha256sum`. Not a
portability detail worth burying: macOS ships `shasum` rather than `sha256sum`,
and the answer is not a new tool requirement — git is already something the
toolchain contract asserts, its object hash is content-addressed and stable, so
the manifest's integrity story stays inside the toolchain that contract already
covers.

**Two hash families, on two classes of file, each stated where it is used.** The
scope above is exactly `files`, and `artifact` is deliberately outside it. The
two hashes answer different questions and are not unified. A `files` hash is
**change detection** — has the adopter edited something `init` wrote — where
collision resistance is not the property needed and staying inside git's
already-asserted toolchain is worth more. The artifact's digest is an
**integrity claim**, published for a reader to cross-check; `git hash-object`
defaults to SHA-1, and a SHA-1 supply-chain digest would undercut the one claim
§The gate binary makes.

That is also why `artifact` is its own key rather than a `files` row. A `files`
entry means *hashed with `git hash-object`, rewritten when unmodified*, and this
one is hashed with SHA-256 against a published value and rewritten on a
different rule — so a `files` row would put two hash families on one map. The
uniformity that map is held to is the hash *function* across every entry, which
is why the argument survives the paragraphs above: it never rested on an entry
agreeing with the tree. The consumer smoke does assert that agreement entry by
entry, against a **freshly initialized** consumer, where no adopter edit exists
for a carried-forward hash to come from. The
binary is still written by `init` and still tracked. A new optional top-level key
is additive within the versioned wire key: a reader that does not know it sees
the same manifest it always did.

**The path is absent, and that is not an omission.** The install location has
exactly one owner — `GATE_SDK_NATIVE_BIN` in your `gate-sdk-config.sh` — and
that value is what the battery actually dispatches to. A stored copy could
disagree with the live one, so every reader that wants the path resolves it from
the same owner.

## The consumer smoke

`consumer-smoke/run-smoke.sh` is the acceptor for everything above, and it is
registered as a validate suite so a bit-rotted activation path is a red
validate rather than a discovery at announcement.

It packs the package, installs it **from the resulting tarball with
`--offline`**, and drives a scratch consumer once per profile: `init`, then the
battery must be green, then the manifest must agree with the tree it describes
file by file, then a re-run must leave the tree object identical, then `doctor`
must exit 0 and name the installed profile. It also asserts the profile
invariant against the installed payload — every named kit resolves, the
containment chain holds, and there are at most three profiles.

A second arm drives the **download transport**: it verifies the packed tarball
against a digest the smoke computes, extracts it with `tar` rather than npm,
and runs the same `init` and the same post-conditions with `npm` and `node`
**masked off `PATH`**. It runs against `full` alone rather than per profile —
what the per-profile loop proves (the payload resolves and each profile's kit
set is present) is profile-dependent, while what this arm proves (the same
payload reached the tree without Node) is transport-independent, and `full` is
the largest payload with the widest `doctor` toolchain read. The smoke's
preflight still requires `npm` and `node`, because packing needs them; the
masking is around this arm's `init` only.

Those transport arms carry two load-bearing properties, neither displacing the
other. The offline tarball install is what turns "this is a one-shot vendoring installer,
not a dependency channel" from a claim into an assertion: a package that
installs and runs with no registry access after the fetch is not resolving
anything on your behalf. The masked-`PATH` arm is what turns "the Release
tarball needs no Node" from a sentence into an assertion — without the masking
the two arms would differ only in how the bytes arrived, and a latent Node
dependency would pass silently on any machine that happens to have Node.

The mask is itself asserted rather than assumed: before the arm runs, each
masked name must resolve to the shim, so a `PATH` that quietly failed to shadow
the real interpreter reds instead of passing. Shims rather than deletion,
because dropping every `PATH` entry carrying `node` would take `/usr/bin` with
it wherever Node is installed there. The residue is that a payload merely
*probing* for a Node binary still finds a name; one that *runs* it fails loudly
and says which name it reached.

**The upgrade arm** drives a cross-version run, because every arm above installs
at one version and re-runs at that same one. It packs a second tarball a patch
version higher, installs `starter` from the first, has the adopter edit and
commit a vendored file, then runs the second package's `init` with no flags at
all. What only that reaches: the manifest's version comparison falling *through*
in the upgrade direction rather than refusing, the profile re-read from the
manifest when none is passed, and `claim()` re-applying the payload around a file
that has changed since `init` wrote it — left alone, reported, and still the
adopter's afterwards.

It then chains a **third** version onto the same consumer with no fresh edit,
because one hop only shows the protection starting. The second hop is where it
either persists or inverts, and nothing above reaches it: the already-edited,
already-reported file must still be the adopter's and must still be reported, and
the manifest the first hop wrote must still carry it. Each version is derived from
the one packed before it and the arm refuses to run unless the derivation is
strictly higher, so neither hop can quietly turn into a second test of the
downgrade refusal.

**The artifact arm rides the per-profile post-conditions**, taking whichever of
§The gate binary's two outcomes the payload and the host actually produce. The
smoke packs with no artifacts, so today it is always the **omission** arm: no
binary is written, no `artifact` key reaches the manifest, and the registry
carries no live member dispatching to a binary. Asserting that is not
ceremony — it is what reds if `init` ever fabricates an artifact record from a
payload that carried none, which is the failure mode that would make the
manifest lie about what is on disk. The **placement** arm — the target matching
the host's resolution, the digest matching the payload sidecar, the binary on
disk at the seam's path and executable — is asserted by the same lines the
moment a run packs artifacts.

*The honest limit, and it is a real one.* Because the smoke packs no artifacts,
the placement half of that arm is unexercised by any automated suite. The write
path is verified by hand against a real built binary, which is evidence with a
date on it rather than a standing assertion. Closing it means the smoke building
or fetching a binary to pack, which is queued work rather than something this
section papers over.

Its only knob is `INSTALLER_SMOKE_TMP_DIR`, and it writes nothing inside the
worktree. It needs a clean worktree, because the pack step refuses to stamp a
commit the payload does not match.

*Why the payload has no gate of its own.* The obvious sibling check would assert
that the packed payload matches the repository's kit roots. It is deliberately
not a gate: the payload exists only at pack time, so the gate would have to run
`npm pack` at commit time — putting a network-capable toolchain in the
pre-commit path and breaking the hermeticity every other gate keeps. The
property is covered here instead, in the tier that can afford it: the pack step
derives the payload from the same kit-root set the battery itself resolves, and
the smoke then runs `doctor` from the payload for *every* profile, so a payload
missing the kit `doctor` sources its toolchain roster from fails the smallest
profile rather than passing unnoticed. The honest residue is that no assertion
compares the two sets element by element — a kit dropped from the payload that
nothing in the smoke's path reads would not be caught here.

## Docs

<https://checkwright.dev> and <https://github.com/checkwright/checkwright>.

Apache-2.0.
