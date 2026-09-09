# Checkwright

The activation path for **Checkwright** — a coding-agent-assisted delivery
methodology shipped as installable kits: a self-testing gate SDK for
prose/spec/config surfaces, an evidence-stamped iteration lifecycle designed
for stateless agent sessions, and token-economics-aware context management.

## What this package is

A one-shot vendoring installer. It copies pinned kit source out of its own
payload into your repository and commits it, then prints the commands that
finish the setup. What governs your tree afterwards is committed and auditable:
every gate arrives with its declaration, its `# spec:` pointer and the section
behind it, and its `good/`+`bad/` fixture pair, and a gate whose implementation
is compiled arrives as a digest-verified binary rather than as source
(gate-sdk/SPEC.md §Consumer payload, which rules that and bounds it).

What it is not: a dependency channel. Nothing resolves at your build time,
nothing is fetched after this package itself, and the installer writes no
dependency reference, no lockfile entry pointing at a registry, and no
install-time lifecycle script. Remove the installer afterwards and the tree it
vendored keeps working — it needs nothing from a package registry again.

## Implementation

Bash, up to the boundary §The install boundary rules — and that boundary is now
the **bootstrap itself**. The two bootstraps are the whole of the shell: one bash,
one PowerShell, twins rather than an original and a rewrite, because the host the
second exists for may run no POSIX shell at all. Everything they hand to the
verified gate binary is Rust, every verb included, so an install is written once
in one language instead of twice by hand in two.

npm is the delivery vehicle, never the implementation: **both** `bin` entries are
scripts, so a reader reviewing what they are about to run reads source rather than
a build product, and the linter that governs every other script in the repository
governs the bash one — the PowerShell half has no such linter here and its oracle
is its own install-smoke leg instead, which is the trade §The install boundary's
parity ruling accepts. What the two scripts amount to is small and fixed by that
section's five steps: resolve, select, verify, execute. There is no third thing a
reader has to review before running an install.

## Requirements

This package reaches a tree over two transports, and each carries its own
requirement. Fetched from npm it needs Node, for `npx`. Fetched as the tarball
attached to a GitHub Release it needs none — `curl`, `tar`, and `sha256sum`,
then `bash package/bin/checkwright.sh init`. Both requirements belong to a
delivery path alone: the gate battery this vendors does not use Node, no
delivery-path tool joins the toolchain roster, and the manual vendoring path
documented on the site needs neither. The toolchain the battery does assert,
with its version floors, is on the install page.

**The verbs themselves need no `jq`.** They read JSON — this package's own
version stamp and the `checkwright.lock` manifest — and behind the invoke they
read it with the crate's own parser, so none of them refuses for want of an
external one. The install page's toolchain block still names `jq`, and that claim
is about the **gates in the battery being vendored**: a different program's users,
and the one that survives.

**A `jq`-less machine is still refused an install, and by a better message.** `jq`
is a consumer-audience member of the toolchain floor, so `doctor` — which runs as
`init`'s last precondition, still before any file is written — blocks the install
naming the floor the battery needs. What is lost is a refusal that named a verb's
own dependency; what replaces it is the one that names the adopter's actual
problem.

`doctor` is the verb that **reaches its diagnosis** rather than refusing before it
can report one: it renders its whole report, names every floor member that is
missing or below its floor, and only then returns the verdict. That is why it is
the precondition `init` gates on, and why its ordering — last, after the manifest
and the profile are resolved — keeps a bad manifest from being reported as a
toolchain fault.

## Layout

- `bin/checkwright.sh` — the bash bootstrap (§The install boundary), and the
  package's `bin` entry.
- `bin/checkwright.ps1` — the PowerShell bootstrap (§The install boundary), the
  second `bin` entry, named `checkwright-pwsh` there. The name is not cosmetic:
  npm generates `<name>.cmd` and `<name>.ps1` shims itself, so a second entry
  also called `checkwright` would collide with the shim npm writes for the bash
  target. **Its `#!/usr/bin/env pwsh` line is load-bearing rather than
  decorative** — npm's shim generator reads the target's shebang to pick the
  interpreter, and without one it writes a shim that invokes the `.ps1`
  directly, which is precisely what a Windows host cannot do.
  **There is no `lib/`.** Every verb lives behind the invoke, in the gate binary
  the payload carries, so the package ships two bootstraps and a payload and
  nothing between them. **What decides whether a file is reachable at an installed
  `PKG_ROOT` is `package.json`'s `files` roster, not the directory listing** — and
  the roster's entries are **directories**, so both bootstraps are published by
  `bin/` and neither is named on it individually. Adding one would be a second
  declaration of what the directory entry already carries. That rule is why the
  deletion of the verb tree is spelled on the roster as well as on disk: a
  directory that stops existing has to stop being published, and the two are
  separate edits.
- `payload/` — the vendored kit source, assembled at pack time from the
  repository's own kit roots. It exists in the published tarball only, never in
  the source tree, so no second copy of any kit is checked in.
- `profiles.list` — the profile rosters (below).

## The verbs

`init` makes an install; the rest manage one after it exists. Each is a
`--`-prefixed arm of the gate binary, reached through the bootstrap's one argv
rule (§The install boundary, step 5), and **the roster's owner is the binary**: a
verb is advertised because the artifact carries it, so `checkwright --help` cannot
promise a verb this build does not implement and an unknown verb is refused by the
binary's own usage arm rather than by a roster beside it. What follows is what
each verb is *for*.

| verb | asks | reads |
| --- | --- | --- |
| `init` | vendor this profile's kits into my repository | the payload, and the manifest a previous run left |
| `doctor` | can this machine run the battery, and what is installed here? | the toolchain, and the manifest's identity fields |
| `diff` | which of the files `init` wrote have I changed? | the manifest's `files` hashes, against the tree |
| `update` | bring the install here up to the version this package carries | the manifest, then everything `init` reads |
| `uninstall` | reverse the install, keeping anything I have edited | the manifest's `files` roster, against the tree |

**The classifier is writes / does not write, and it is what decides
`--dry-run`.** `doctor` and `diff` write nothing, so neither has one and neither
would have anything to preview. Every verb that does write has one, and it means
the same thing in each: print the plan, write nothing, exit 0. That rule is
asserted behaviorally rather than gated — §The consumer smoke holds
`uninstall --dry-run` to leaving the tree object and the worktree unchanged,
which is what a flag that parsed and then wrote anyway would fail and a flag
that merely existed would pass. *The honest bound:* `init`'s and `update`'s
`--dry-run` carry the same contract and are not held to it there yet, so the
rule is asserted on one verb and documented on the other two.

**Why `diff` is a verb of its own rather than a widening of `doctor`.**
`doctor`'s exit status has exactly one owner — the toolchain contract — and
`init` gates its own precondition on it. That ownership already forced one
carve-out: §doctor reports an artifact finding *without* setting the status,
because reddening there would block the `init` re-run that is the finding's own
remedy. Drift would force the same carve-out a second time and on worse ground.
Editing a vendored file is **sanctioned** — the whole ownership contract in §The
manifest exists to protect it — so a folded-in drift report would be a permanent
expected finding on the one surface that teaches a reader to skim its last line.
And drift wants a status of its own: `0` the tree is exactly what `init` wrote,
`1` it has diverged. That is gatable in CI, and it cannot live on `doctor`
without colliding with the number the toolchain contract already owns.

A second ground, independent of the first: `doctor` runs as `init`'s
precondition, before every install, so per-file hashing there would make every
`init` pay for a computation the install redoes moments later and print the
answer twice.

Nor do the two `--dry-run`s stand in for it. They answer *what would this
payload change*, which is a different question from *what have I changed*, and
an adopter deciding whether to uninstall needs the second without running a
mutating verb at all.

## init

`checkwright init` vendors the selected profile's kit source out of this
package's own payload into your repository and commits it. One command, and
what governs your tree afterwards is committed and auditable on the terms
gate-sdk/SPEC.md §Consumer payload sets.

**No selection builds.** Not "no profile that happens to carry only shell
gates" — no selection, ever. `init` writes files and compiles nothing, so it
asks for **no build toolchain** and cannot fail on one. The only toolchain it
asks for at all is the one the battery it is vendoring will itself run, which
is what the `doctor` precondition below checks and the whole of what can block
an install. A gate whose implementation is a
compiled subcommand reaches you as a prebuilt binary picked for your platform
and checked against a published digest before it is written, which is what makes
zero build step a property of `init` itself rather than of which profile you
picked (gate-sdk/SPEC.md §Porting a gate to the binary substrate, criterion 5).

Three preconditions, and all of them **refuse** rather than warn — a partial
install is the outcome none of them may produce, so every one is checked before
a single file is written:

- **You are inside a git work tree.** The vendored source is meant to be
  committed; that is what makes it auditable rather than merely present.
- **The worktree is clean.** `init` makes one commit, and a dirty tree would
  fold your work into it, leaving a reviewer's diff wider than what was
  actually vendored. It is also what makes a run's own residue attributable:
  anything dirty at a path `init` recorded, when the run ends, was written by
  that run and by nothing else — which is what lets a run that changed nothing
  still commit what it rewrote rather than guess whose the change was.
  `--no-commit` is the valve: it writes and stages the
  files and leaves the commit to you, so an operator who wants to compose the
  change themselves has taken that guarantee on deliberately — and because it
  waives the precondition, it waives that attribution with it.
- **`doctor` passes.** The contract it holds you to is the consumer-audience
  subset of the roster — the tools the vendored battery runs, never a tool that
  only builds Checkwright (§doctor) — so a machine with no Rust toolchain is
  not below it. A machine that *is* below it blocks *before* any partial
  install, rather than halfway through one, which is why `doctor` ships in the
  same phase as `init` rather than as a later convenience.

  **This is the precondition that carries the whole toolchain refusal**, and it
  is worth saying so where a reader meets the list: every refusal `init` makes
  about the machine it is running on is this one. The refusals below it — *this
  package carries no version stamp*, and *`checkwright.lock` carries a schema
  this build does not know* — are about the package and the manifest, are reached
  by the crate's own JSON reader, and each means exactly the condition it names.

It writes the selected profile's kit directories, a `gates.list` seeded with
each kit's starting gates, the config seam files those kits need, and the
manifest. Then it makes **one commit** naming the profile and the version, and
prints a **follow-up block**: the commands that finish the setup, one per line,
each carrying its reason beside it. The commands are deliberately **not spelled
on this page** — what `init` prints is `init`'s to say, and a second copy here is
a string a rename has to be remembered to move.

**The block has a stated grammar, because something reads it.** The banner line
is exactly `next:`; each line after it that begins with whitespace and then a
non-whitespace character is one command; the block ends at the first line that is
not one of those, or at the end of the output. On a command line, everything from
the first `#` is commentary for the adopter and is read by nothing else. The
command's **target** is a repo-relative script path, so it is the first token
containing a `/` — any token before it is the interpreter the line spells — and
every token after the target beginning with `-` is a flag.

That grammar is a contract rather than a layout choice because §The consumer
smoke's follow-up arm reads it: a later edit that reflows the block reds that arm
instead of silently un-covering the pair, and a rename of either command moves
one string with nothing to keep in step. What that arm asserts against the
grammar, and what it deliberately does not, is that section's to state.

**An install's size is not bounded by the host's argv width.** `init` names every
vendored path to `git` — when it stages, and again when it asks what the run left
behind — and a large profile's roster is long enough to exceed the command line a
host will actually accept, which surfaces as `Argument list too long` and leaves
the install incomplete on exactly the profiles that vendor the most. Those calls
are issued in batches, so what a profile may carry is bounded by the payload and
never by how many characters one process may be handed. The ceiling that binds is
the host's, not POSIX's: it is a native Windows process limit that a POSIX-shaped
`getconf` reports nothing useful about, so the batch size is a fixed conservative
budget rather than a probed one.

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
which an unowned path could otherwise be written straight over your edits. The
gate binary is the single exception, and §The gate binary states it: no version
of a compiled artifact is yours, so a re-run rewrites a substituted one. A
re-run that finds
nothing to change says so and exits clean; an unchanged tree is the success
case, not an error.

**"Nothing to change" does not mean "nothing was committed".** `init` regenerates
the projections its vendored tools own on every run, so a run it classifies as a
no-op has still rewritten files — and **one commit** is a promise about what the
run leaves behind, not about which branch it took to get there. So the no-op exit
commits anything it rewrote that is not already committed, under the same message
the ordinary vendoring commit uses, and says so. Expect it to find nothing: a run
that rewrote something normally stages it and never reaches that branch at all.
The arm is there so the promise holds when it does. Under `--no-commit` the arm
is off, because the commit is yours.

**Two shapes this is deliberately not, and one ordering it must not take.** It is
not a freshness comparison that skips regenerating a projection whose inputs have
not moved: `init` does not compare freshness, and the projections are the vendored
tools' own output rather than something `init` holds an input for. It is not a
change of owner either — the projections stay `init`'s `files` entries rather than
becoming yours to regenerate, because the ownership rule in §The manifest is what
protects your edits to everything else `init` writes. And the commit on that branch
sits **inside** the exit rather than ahead of it: a commit against an empty index
exits non-zero and `init` treats a failed commit as a fatal install error, so an arm
placed before the "nothing to change" test would turn the pure idempotent path into
a false hard failure.

A payload older than the recorded install is refused as a
silent downgrade — `--force` covers that refusal too, which is what makes a
rollback a thing you asked for rather than a thing that happened to you.
`--force` means the same thing in all three places it appears — the changed-file
protection here, the downgrade refusal above, and the kept files `uninstall`
would otherwise leave behind (§uninstall): overwrite what `init` would otherwise
protect.

`--dry-run` prints the file plan and the manifest that would be written, writes
nothing, and exits 0. The contract is **end to end, not caller-deep**: a step
that runs behind the binary invoke is passed `--dry-run` and honors it there
(§The install boundary), so the plan a dry run prints is produced by the same
code the real run performs rather than by a second prediction of it.

## What init seeds

Beyond the kit directories themselves, `init` writes the least that makes the
battery green on the tree it just made.

The **config seam is derived, never listed**: a kit's consumer config is
whatever `templates/*-config.sh` it ships, and the destination is always your
gates directory under the file's own name. A kit that grows a config template
is picked up with no edit here. The class this derivation defines — both sides
of it, template and seeded copy — is **permanently shell**, ruled at
gate-sdk/SPEC.md §The config-seam port disposition on the ground stated below:
these are the files you edit, and a port would leave nothing to edit.

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

**The queue file's source is derived too, and resolved once per install rather
than once per kit.** A kit declares itself the queue format's owner by shipping
`templates/TASK-QUEUE.md`; `init` seeds from the template of the first kit in
the resolved profile's set that ships one, and writes a minimal inline skeleton
only when **no** kit in that set does. Resolving over the whole set is the
point: while the choice was made inside the per-kit loop, the first kit reached
decided it, so a kit that reads the queue and ships no template pre-empted one
that does — and the kit shipping the template was reachable in no profile at
all. Nothing declares an owner, because shipping the artifact already does. The
inline skeleton carries every `QUEUE_KIT_REQUIRED_SECTIONS` heading at that
knob's default, since it is a path a shipping profile takes rather than
defensive symmetry: a profile whose kit set reads the queue and carries no
kit shipping the template — `prose` is that shape — receives exactly that file,
and the section gate that would catch a missing heading is `on-surface`, so
nothing in the battery `init` registers would say so.

The **starting gate roster** is the subset a fresh consumer begins with, not
the kit's full roster — the same distinction gate-sdk's own README draws. A
gate whose subject you have not authored yet has nothing to read: canon-kit's
duplication gate wants a glossary, and site-kit's wants a docs host. On a tree
that has done nothing wrong those would red on day one, so they are registered
when the surface exists rather than at install. Each kit's README names the full
roster to grow into. **No lifecycle-kit gate is `zero-config`**, and the reasons
differ across its roster rather than being one reason: some read a stage
attestation only a stage session can write, others read surfaces a stage session
authors, and two — `check-lifecycle-registration` and `check-merge-attrs` — read
what the `--install-lifecycle` arm writes, which is the adopter's own
step and not a stage's. The posture is kit-wide; the reason is per gate.

**A kit's agent-file block is seeded at install iff a gate registered at install
reads it.** Seeding follows the gate, not the kit — the agent-file half of the
rule above, extended to the one other thing `init` writes on a kit's behalf.
doctrine-kit's block is seeded because `check-doctrine-registration` declares
`zero-config`, so the reader of the block is in the registry `init` writes and
the block is read on day one rather than sitting resident. lifecycle-kit's is
not, because nothing `init` registers would read it: seeding it would put
always-loaded instruction for a stage machine into your agent file and charge
every session's context for machinery nothing yet enforces. The rule is also
what keeps the agent-file *membership* predicate from being a seeding roster — it
asks whether the agent file must *exist* for a kit's starting gates, which
context-kit answers yes to while writing nothing into it; the seeding arms ask
the narrower question of which kit writes into it.

**The roster is derived from the gates themselves.** The recipe's per-kit gate
derivation, behind the invoke with everything else, is the
**whole** of a fresh consumer's registry — what a tree `init` just made will run
is read there, never inferred from a kit's full roster or from this repo's own
`gates.list` — and what it reads is each shipped gate's own
`# install: <disposition>` header line, taking every member a kit declares
`zero-config` (gate-sdk/SPEC.md §The install disposition). It carries no gate
name of its own, which `check-install-disposition`'s third assertion holds it to,
so a kit that adds a zero-config gate is picked up with no edit
there. One function unions the result over a profile's kits, and it has exactly
one caller: the registry `init` writes. The consumer smoke's monotonicity
assertion reads **that registry** rather than calling the function — it drives
this package as a black box and holds no second copy of the derivation. So the
two still share one derivation and the smoke reads it *across* the package
boundary instead of inside it, which strengthens the claim rather than loosening
it: containment is asserted over the roster an adopter of each profile actually
receives, not over what a recipe said they would. No disposition varies on
the profile today; the argument is the seam, so a roster that does vary becomes a
change to one gate rather than to a signature and every caller of it.

**What the seam can and cannot express, measured rather than assumed**, because
the obvious use of it is the one thing it does not do. `full` is the
payload-derived maximum, so every profile's kit set is contained in it, and the
smoke's monotonicity assertion therefore means *anything a profile registers,
`full` must register too* — a gate armed "for one profile only" is not
expressible, since the moment it is armed anywhere it is owed to `full`, and
once it is owed to `full` it is simply `zero-config`. `starter` sits below every
profile for the mirror reason, so no profile may **drop** a gate-sdk
`zero-config` member either; and subtraction in the band between them has
nothing to derive from, because `check-install-disposition` forbids the recipe
a literal gate name and no gate declares
profile-varying reachability. So the parameter is **additively inert** and
subtractively undeclarable, which is why a cohort of gates a narrow profile
should receive arrives as a disposition correction on each gate instead. The
argument stays the seam for the future in which a gate does declare that
reachability; nothing declares it today.

**A disposition change reaches trees that are already installed.** `init` on an
existing consumer rewrites `gates.list` from this derivation, so a gate moving
`on-surface` → `zero-config` joins their battery on their next run, on content
they wrote before the gate existed. Their *edits* to the file are protected —
`gates.list` is claimed before it is written, like any rewritten surface — but
the roster growing is the intended behavior rather than a claim violation, and a
release carrying such a move owes them the sentence in its note.

Each kit's `smoke/install.sh` registers a **richer** roster against the scratch
tree that script builds and seeds — a superset of this one by contract rather
than by coincidence, held there by `check-install-disposition`. The two describe
two different trees, which is why the disposition is what a kit owns and the
roster is what each caller derives.

## The install boundary

One part of an install must be written in whatever language the host already
runs: the bootstrap that resolves, verifies and executes the gate binary.
Everything else is **conditional install logic**, and
TRAJECTORY.md's interpreter policy (§The closed rulings) rules that everything
conditional belongs on the far side of that invoke — written once, in Rust,
rather than twice, in bash and in the PowerShell half a native Windows install
needs. **That relocation has landed**: the bootstrap below is the whole of the
shell, and every verb is an arm of the binary it invokes. This
section states the bootstrap's job, the disposition every install step carries,
and the test that assigns one, so a step's side of the line is read off a rule
rather than re-argued per step.

**The bootstrap's job is the whole of what is written twice:**

1. resolve the package's own payload directory;
2. resolve the host to one Rust target triple;
3. read the payload's target roster and resolve the artifact and its sidecar,
   refusing a declared target whose pair is incomplete;
4. verify the artifact's SHA-256 against that sidecar;
5. execute the verified artifact, under one unconditional argv rule: a dashless
   leading token is prefixed with `--`, and everything after it is forwarded
   verbatim.

Step 5's rule is the only conditional shape either bootstrap carries, and it is
unconditional in the sense that matters — it branches on the *shape* of the first
token and never on which verb it is, so no verb table is maintained in two
languages. `checkwright init --profile starter` reaches the artifact as
`--init --profile starter`; `checkwright --help` is forwarded unchanged. Two
consequences follow, and both are improvements rather than costs: the verb
roster's owner moves from the filesystem to the binary (§The verbs), and the two
halves stop disagreeing about the verb word, which one used to forward and the
other to consume.

**Every step's disposition takes one of three values**, and the test that
assigns one is *what the step needs that the binary cannot supply at that
moment*:

- **`bootstrap`** — the step must precede the invoke because the binary cannot
  select, verify or execute itself. The steps listed above, and nothing else.
- **`behind-invoke`** — conditional install logic. Written once, in Rust. This
  is the default: the interpreter policy rules that everything conditional
  belongs on the far side of the invoke, so a step claiming `bootstrap` owes a
  reason drawn from the previous bullet and no other.
- **`retired`** — the step exists only to serve a dependency the relocation
  removes, and ceases to exist rather than moving. `init`'s `jq` preflight is
  the worked case: nothing behind the invoke reads JSON with `jq`, because the
  crate reads it with `serde_json`.

**How the port oracle reads those three values — ruled 2026-08-31 by the
operator in consult, so a cut can cite this section as its stated contract.**
The section is a two-sided port disposition. `bootstrap` is the port's
irreducible (TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence). A
file whose *whole* body is bootstrap steps declares `# no-port:` citing this paragraph, and that is the
only `no-port` cause `installer/` may carry. `behind-invoke` is a **port
obligation**: a file carrying any such step stays `owed` in
`--emit port-blockers --tree` until the step is relocated behind the invoke and
the shell that ran it is deleted. `retired` steps are deleted, not declared. **The relocation has landed, so the
obligation is discharged**: what `installer/` still carries is two bootstraps
whose whole bodies are the steps above, each declaring `# no-port:` citing this
paragraph, and nothing else.
`scripts/pack-installer.sh` is outside this section's reach and its disposition
is §The packer's, so a reader hunting the packer's port standing here is in the
wrong section.
Two readings were refused with it, so the next cut does not re-argue them:
*`behind-invoke` alone is the obligation and `bootstrap` is silent* — refused
because it leaves the one shell surface the trajectory sanctions with no cause
to declare on, so the bootstrap would read as owed forever; and *the section
answers only WHEN a step runs, not WHETHER its implementation ports* — refused
because the section's own words for `behind-invoke` are "written once, in
Rust", which is a port commitment, and a reading that voids the section's text
is wrong.

**The two bootstraps are hand-kept, and parity is held by running, not by
generation — ruled 2026-08-26.** Each half is authored in its own language
against the five steps above, and the oracle that holds them equal is a
per-**bootstrap** install-smoke leg. **Count those legs by bootstrap and never by
platform.** `.github/workflows/gates.yml` carries an install-smoke job for each
platform it measures — Linux, native Windows, and each macOS architecture — and
every one of them drives the *bash* half, the Windows one through
Git-for-Windows bash; exactly one further job drives the PowerShell half under
`pwsh`. The platform side is deliberately not given a number here: it moves
whenever the platform declaration does, and a reader counting platforms would
read a growing set of legs as covering a growing set of bootstraps, which they
never did. **The bootstrap count is two and only that number is load-bearing.**
The PowerShell leg ships with the PowerShell half, and **no other leg
substitutes for it**, because every other leg drives the other bootstrap.

**That oracle has two parts, and only one of them is in place — read this before
reading the leg as short of its own ruling.** The ruling above asks for a leg
*exercising the payload end to end*, and that phrase carries two distinct
assertions:

- **Bootstrap parity** — that the two halves answer the five steps the same way.
  The PowerShell leg discharges this for its half.
- **Payload coverage** — that a whole `checkwright init` completes through that
  bootstrap. This was owed rather than dropped, and it was owed **with the
  relocation**: while the binary carried no `init` arm, step 5 handed `init` to a
  program that did not implement it, and no leg could assert a completion no code
  path could reach. The relocation minted that arm, so a completion is reachable
  and the leg asserts one.

The two were conflated because both halves were assumed to land together. The
drift the 2026-08-26 ruling guards is between the two **bootstraps**, and *end to
end* adds **payload** coverage rather than **parity** coverage — which is why the
first part shipped alone and the second waited. Both are in place now, and the
distinction is kept because a later reader sizing a new leg needs to know which
of the two it is adding.

**What a reader compares when the two legs disagree.** Parity held by running
tells you *that* the halves differ and never *where*, and with two hand-kept
halves a red on one leg beside a green on the other is the routine case rather
than the exceptional one. So the comparison has a stated order — the five steps,
in sequence, each with the observable that distinguishes it — and a session
arriving at a divergence walks it rather than diffing two languages:

| step | what to compare | the observable that settles it |
| --- | --- | --- |
| 1 payload directory | the resolved package root, after the symlink chain | the path each half prints when the payload is absent |
| 2 host triple | the triple each half maps this host to | one half's empty string against the other's triple |
| 3 selection | which of the three outcomes each half reached | unrostered vs. verify vs. broken payload, by message and remedy, never the exit status alone |
| 4 digest | the hex compared, and its case | a mismatch on identical bytes is a case fold, not a corrupt artifact |
| 5 execution | the argv the artifact actually received | the binary's own usage refusal, which echoes what it was handed |

Step 4 is the row that most often reads as a defect and is not one: `Get-FileHash`
returns upper-case hex where the sidecar carries `sha256sum`'s lower case, so the
halves agree on the digest and disagree on its spelling. Step 3 is the row where
reading the exit status alone misleads, and it misleads in the other direction
now: two of the three outcomes refuse, so a reader comparing statuses sees one
answer where an adopter is owed two — a platform they can do nothing about, and a
payload they fix by re-downloading. Compare the message and the remedy. This is
the same economy the truth table in §The consumer smoke buys, stated once here
rather than rediscovered per divergence.

**Two host assumptions this bootstrap rests on are measured rather than assumed.**
Both were measured on a native Windows runner: `[[ -x ]]` **holds** on a freshly
`chmod +x`'d shebang script, which executes directly despite
`core.filemode=false`; and it **holds** on npm's extension-less bin shim, written
mode `-rwxr-xr-x` beside its `.cmd` and `.ps1` siblings. So neither `-x` test
needs a Windows special case and neither bootstrap carries a mode-detection
branch. They are recorded here because a later reader meeting either test will
otherwise ask the question again, and the answer cost a runner to buy.

The alternative refused is one declaration
generating both halves. Its
grounds: at five steps the generator is a third artifact — a template language,
a freshness gate and a projection-roster row — maintained for a surface small
enough to be written twice by design, and a generated twin still needs the
platform leg to prove it runs, so the generator buys no oracle the legs do not
already supply. Two hand-kept halves drifting is a *real* cost, and the leg is
the mechanism that turns that drift into a red run rather than a reading.

**Step 5 is *execute*, not *install*.** The tracked copy of the binary under the
consumer's gates directory is an install artifact with ownership semantics —
claimed against the manifest, carried in `files[]`, removed by `uninstall` — so
by the rule above it is conditional install logic and sits `behind-invoke`. The
bootstrap runs the artifact **in place, out of the payload**, where step 4 has
just verified it; a copy to a scratch path in order to run it would be a copy
with no reader. The interpreter policy's "place the matching binary" names the
job *make the binary runnable*, and its very next sentence is what settles which
half of "place" this is.

**A `behind-invoke` step may spawn `bash`, and one does.** `gen-pre-commit.sh`
does not port (gate-sdk/SPEC.md §gen-pre-commit), and `check-graph` is
`install: zero-config`, so a fresh consumer's day-one battery holds the
generated hook against `--emit` and the hook must therefore exist at install.
The step is consequently neither droppable nor portable. It is **not** stuck:
the compiled substrate already spawns `bash <emitter>` for exactly this
generator from `check-graph`'s own assertion, the port criteria clear that spawn
explicitly because `bash` is on `GATE_SDK_PROGRAM_FLOOR`
(gate-sdk/SPEC.md §lib/gate.sh) — the payload's own assumed-program set, not the
consumer-audience probe roster that `bash` also
happens to sit on (context-kit/SPEC.md §bin/env-probe) — and the arm declares it. So the step moves behind the invoke
as a declared spawn, and the *bootstrap* — which is what the interpreter
policy's standing "assume no POSIX shell" obligation binds — spawns nothing.
Recorded because the natural reading is that this step is a third class that
neither moves nor re-implements.

**`--install <op>` is the seam both bootstraps call**, specified so the two
calls are byte-identical. It is a non-gate arm of the binary
(gate-sdk/SPEC.md §The non-gate arm) and deliberately **not** a bridged one: a
bridged arm's knobs are resolved by `gate_command`, a bash front-end, and this
arm's caller is the bootstrap, which may not be assumed to be a POSIX shell at
all. So **every value the arm needs arrives as argv**, and the arm reads no kit
config and no knob. A bridged install arm would be unreachable from the half of
the boundary this section exists to make writable.

- **Grammar.** `--install <op> [--<key> <value>]…`, `<op>` from a closed set, an
  unknown `<op>` or an unknown key exiting 2.
- **Channels, because the caller is a program in two languages.** *stdout* is a
  wire: one record per line, tab-separated, `<verb><TAB><field>…`. *stderr* is
  the adopter-facing report.
- **Exit status.** `0` performed — or, under `--dry-run`, planned; `1` an
  adopter-actionable refusal; `2` usage or harness error, on
  gate-sdk/SPEC.md §Fail-closed contract's terms.
- **`--dry-run` is owed by every mutating op**, on §The verbs' existing
  classifier: print the plan, write nothing, exit 0.

The family carries two ops and neither produces a `1`: `place-artifact`'s only
failures are a bad argv and a write it could not make, and `queue-source` reads
and writes nothing, so both refuse at `2`. The status is
specified on the family rather than on the op, so an op that *can* refuse
something the adopter can act on has a status to refuse with rather than minting
one.

**`queue-source` is a read op, and the family's grammar is what it costs.**
`--install queue-source --payload <dir> --kits <kit>[,<kit>…]` answers which
template a kit set's queue is seeded from — the derivation §What init seeds
states — by emitting `queue-source<TAB><path>` when one is owed and **an empty
wire** when none is. A caller's whole reading is therefore *nonempty means
owed*, with no field to interpret. It exists because the derivation has a second
reader outside the package: the consumer smoke asserts init's queue
post-condition and must not carry a second copy of the rule to do it, so it
reads this one across the package boundary instead of inside it (§The consumer
smoke). Minting a contract for that reader was refused — the op takes this
family's existing grammar, channels and exit statuses, so the seam the
bootstraps already call is the seam the smoke calls.

**The five verbs are not ops of this family, and the reason is a channel conflict
rather than taste.** This family specifies stdout as a *wire* — tab-separated
records, one per line — while `init`'s stdout carries the adopter-facing
follow-up block whose grammar §init states and whose reader is the consumer
smoke's follow-up arm. One of the two contracts would have had to yield. So the
verbs are top-level `--`-prefixed arms resolved before the registry lookup
(§The verbs), and this family keeps the wire it was given for a caller that is a
program in two languages. The line is not where a step *runs* — every verb is
behind the invoke exactly as an op is — but which **channel contract** its output
answers to.

**The relocation's own precondition, and how it was discharged.** A step could
move behind the invoke only where the binary is reachable on every platform that
step ran on, and it was not: §The gate binary's selection table has three
outcomes, and two of them once left `init` with no binary — a host whose triple
the payload's roster does not carry, and a host with no SHA-256 hasher. Both
**proceeded**, omitting the compiled gates and declaring the omission in the
consumer's `gates.list`.

That branch never kept a battery alive on an uncovered platform: every registered
member dispatches to the binary, so an artifact-less install retained no live
member and its battery refused. What it delivered was the install and its
disclosure. Once every step of an install sits behind the invoke, the same branch
has nothing to run at all — the failure the relocation would otherwise introduce
is **not a smaller battery but a silent non-install**.

**So both outcomes became bootstrap refusals**, which is the honest form of what
those hosts were already getting:

- **An unrostered host is refused, naming the platform.** There is no adopter
  action, and saying so is better than writing a tree whose battery cannot run.
- **A POSIX host carrying neither `sha256sum` nor `shasum` is refused** rather
  than served an unverified artifact, because step 4 of the bootstrap is
  irreducible: a host that cannot hash cannot verify, and verifying before
  executing is the whole of the integrity claim. On the PowerShell half the branch
  was always vacuous — `Get-FileHash` — which is the same fact this section
  records from the other direction.

**The digest question dissolves behind the invoke rather than moving.** The binary
hashes in-process, so nothing past step 5 needs an external hasher at all; what
survives is the bootstrap's own step 4, and it survives because it cannot use the
binary to verify the binary.

## The gate binary

A gate whose implementation is a compiled subcommand needs that binary on disk
before it can run, and `init` is what puts it there. Selecting and verifying it
is the **bootstrap's** job, because the binary cannot select itself; placing it
in the consumer's tree is `init`'s, behind the invoke. Nothing builds and nothing
fetches. The selection and verification steps below sit inside the irreducible
bootstrap the vendoring ruling leaves outside the binary
(gate-sdk/SPEC.md §Porting a gate to the binary substrate), and each is
deliberately small enough to be written twice, in bash and in PowerShell.

**Platform resolution is derived and never stored.** This paragraph describes the
**bash** half; the PowerShell half answers the same question from its own runtime
and is described at the end of it. `target_of_host()` maps
`uname -s` and `uname -m` to one Rust target triple, and to the empty string on
a host that maps to none. It runs once per invocation, before anything else
happens. The result stays a local: a stored copy
would be a second source for a fact the host already answers, and it is stale
the first time a vendored tree moves between machines — the case that matters
most, since a vendored tree is shared by construction. Two fields rather than
`uname -a` because that is the smallest input that answers the question, and it
is what a PowerShell half can answer without parsing prose. Nothing reads the
kernel version, so nothing collects it.

**The map answers "which published artifact fits this host", which is why a
MinGW, MSYS or Cygwin `uname` maps to `x86_64-pc-windows-msvc`.** Those `uname`
strings report the *shell environment* — the measured runner answers
`MINGW64_NT-10.0-26100`/`x86_64` — and not the toolchain that built the artifact
the adopter is about to receive; what a Windows build leg would publish is what
that host's own `rustc` reports as its host triple, `x86_64-pc-windows-msvc`.
Before this arm those hosts matched nothing and the function returned empty.
**The arm's verdict is unchanged today, stated rather than left to be
discovered**: its reader is `select_artifact`'s roster comparison, live on every
invocation, and that comparison still fails because `native/targets.list` does not
name the triple — a target joins the roster only on a run that produced and
exercised its artifact (gate-sdk/SPEC.md §Consumer payload), and none has. It is
reachable **now** for testing through `GATE_SDK_NATIVE_TARGETS_FILE`, the same
steering knob §The consumer smoke already documents as the roster re-entry.
Whether an msvc-built binary runs on an arbitrary Windows host is a question no
run has answered, and withholding that answer until one has is that join bound
doing its job.

**The PowerShell half reads the runtime instead, and reaches the same triple.**
It asks the .NET runtime for the OS platform and the OS architecture rather than
shelling out to `uname`, because the host this half exists for need carry no
POSIX shell and therefore need carry no `uname` — the standing obligation is to
assume none. On a native Windows x64 host it resolves `x86_64-pc-windows-msvc`,
the same triple the bash half's `MINGW*`/`MSYS*`/`CYGWIN*` arm produces and for
the same stated reason: the map answers *which published artifact fits this
host*. **Its verdict is unchanged today for the same reason the bash arm's is** —
`native/targets.list` does not carry that triple, so selection still refuses the
host as unrostered. Shipping this half is not a platform claim, and
§Requirements still says what it says.

**Selection keeps three outcomes, and collapsing any two is the defect. What
changed with the relocation is that only one of them proceeds.** The
payload carries the target roster verbatim beside the artifacts
(gate-sdk/SPEC.md §Consumer payload), and the bootstrap reads it rather than
inferring support from a directory's presence:

| the host resolves to | the payload holds | outcome |
| --- | --- | --- |
| a target **not** in the roster | — | **refuse** — this platform is not in the support roster; there is no adopter action |
| a target **in** the roster | its binary and sidecar | verify, then execute |
| a target **in** the roster | nothing, or half the pair | **refuse** — the payload is broken |

The three stay told apart by **message and remedy**, never by exit status alone:
an undeclared host and a broken payload remain different answers to an adopter,
and collapsing them is the same defect this table has always named. The third row
is the whole reason the roster is read. Without it the bootstrap cannot
tell a platform that was never committed to from a platform that was committed
to and whose artifact went missing, and reading the second as the first turns a
publisher defect into a silently smaller green battery. A payload assembled with
no artifacts at all carries no `artifact/` directory, so it reads as the first
row and never as a payload whose every target went missing.

**Why the first row refuses rather than omitting.** Once every step of an install
is on the far side of the invoke, an artifact-less host has no code path at all,
so *omit and declare* has nothing to declare into and nothing to proceed with
(§The install boundary). A refusal that names the platform is the honest form of
what that host was already getting. **Both bootstraps owe this table all three
rows and now answer them identically**, which is a parity defect closed rather
than one introduced: the PowerShell half used to declare-and-stop where the bash
half declared-and-proceeded.


**The digest is verified before anything is executed.** The bootstrap computes
the artifact's SHA-256 and compares it against the sidecar that travelled with
it, and only then runs it. The ordering is the whole of it: a consumer who cannot
read the gate has nothing else standing between them and a substituted binary. A
mismatch refuses — never a warning, never a run.

`sha256sum` is tried first, then `shasum -a 256`, because stock macOS ships the
second and not the first. When **neither** resolves the bootstrap **refuses**:
nothing unverified is ever executed, and there is no path here that skips the
check. This is the only step an external hasher is still needed for — behind the
invoke the binary hashes in-process, so the placement `init` performs verifies
without one.

**That hasher resolution is the bash half's, and the PowerShell half has none of
it — a ruling, not an omission.** PowerShell carries `Get-FileHash`, so on that
half there is no hasher to resolve between, no `sha256sum`/`shasum` fallback and
no refusal to reach: the branch is vacuous there, which is
the same fact the relocation's precondition already records from the other
direction. This is the one step where the twin is simpler than the original
rather than parallel to it, and it is stated because a reader holding the two
halves side by side will otherwise read a missing branch as a defect. The one
asymmetry it does introduce is spelling, not logic: `Get-FileHash` returns
upper-case hex against a lower-case sidecar, so that half folds case before
comparing.

**The artifact is the one path the changed-file protection does not cover, and
that is what makes the digest actionable.** Everywhere else `init` writes, a file
whose recorded hash disagrees with the tree is yours: you authored the
difference, so it is reported and left alone (§init). A compiled artifact has no
version you authored for that rule to protect — a binary that fails the recorded
digest is corrupt or substituted, never edited — so `init` rewrites it from the
payload copy it just verified rather than adding it to the changed-file report.
Without the exemption the diagnosis and the remedy come apart: §doctor reports
the mismatch and names a re-run as the fix, and a re-run that classified the
binary as yours would leave it exactly where it was, reporting the same finding
forever. `--force` is therefore never needed here and means nothing extra. The
exemption is scoped to the artifact path alone: the config seam written beside it
is a file you genuinely do edit, and it is claimed like everything else.

*The honest bound, stated so no surface overclaims it.* The digest travelled
inside the same payload as the artifact, so it catches corruption and a
substitution made to the artifact alone — not a compromised publisher, which no
in-payload value can. What raises it above a self-check is that the identical
bytes are published on the Release, so a human can cross-check the value out of
band. The claim is **verified against a published digest**, never *reproducible*.

**The gate-sdk config seam rides this path and only this path.** `init` claims
`scripts/gate-sdk-config.sh` inside the branch that has selected an artifact
target, and gate-sdk ships no config-seam template for the generic seam plan to
copy. On a payload carrying no artifact that file is therefore never written and
is not a `files` entry at all — so a verb reasoning about the surfaces `init`
rewrites on every run must not assume it is present.

**The install-time omission retired with the bootstrap's one success path.**
`init` once wrote `# omitted: <name> <reason>` into the consumer's `gates.list` in
place of a member it could not install, with two reason tokens for two remedies.
Neither reason can arise any more: an unrostered host and a hasher-less host are
both refused before an install begins (§The install boundary), so an install that
happens at all installs the whole starting roster.

**What did not retire is gate-sdk's registry class.** `# omitted: <name> <reason>`
is a comment line the runner strips from the live set and reports beside its
summary (gate-sdk/SPEC.md §run-gates), and it is **reason-agnostic**: it belongs
to any consumer who omits a member for any cause of their own. Two mechanisms
shared one vocabulary, and only the installer's outcome retired — reading the
class's two best-known values out of existence with it would publish an installer
decision as a kit narrowing. `doctor` reports whatever reason it finds there and
invents no remedy for it (§doctor).


**Placement is one call, and the bootstrap makes it.** Steps 1 to 4 above are
the bootstrap's; placing the artifact is conditional install logic, so it sits
behind the invoke as `--install place-artifact` (§The install boundary). The
bootstrap runs the **payload** artifact it just verified, never the installed
copy, which on a first install does not exist yet:

```
<artifact> --install place-artifact
    --root   <absolute repo root>
    --src    <the verified payload artifact>
    --dest   <repo-relative path for the installed binary>
    --seam   <repo-relative path of <gates-dir>/gate-sdk-config.sh>
    --target <rust target triple>
    --digest <the artifact's verified SHA-256>
    [--lock  <repo-relative manifest path>]
    [--force] [--dry-run]
```

Every key has a reader inside the op: `--root` resolves every relative path,
`--src` is the copy source, `--seam` is the claimed path, `--dest` is written on
the rule below instead, `--target` and `--digest` are compared against the
manifest's `artifact` key and the on-disk copy for the skip-rewrite branch,
`--lock` supplies the recorded hash the claim compares, and `--force` and
`--dry-run` carry `init`'s existing meanings. `--lock` is optional and absent on
a first install, where nothing is claimed. Two stdout verbs come back, and each has one reader in the caller:

| verb | record | the caller's reader |
| --- | --- | --- |
| `own` | `own<TAB><path>` | the path is recorded and joins the staged set |
| `kept` | `kept<TAB><path><TAB><hash>` | the path joins the changed-file report and is carried forward at that hash |

A third verb distinguishing an unchanged write from a write is deliberately
absent: no caller reads the difference, including the `--dry-run` report, and a
field with no reader is removed.

**The install location has one owner.** The binary is written to your gates
directory beside the `gates.list` seeded there, and the op sets
`GATE_SDK_NATIVE_BIN` to that path in `<gates-dir>/gate-sdk-config.sh` — the
optional persistent config seam gate-sdk's library already sources when it
exists (gate-sdk/SPEC.md §Layout and configuration). The op creates that file
when it places an artifact, and gate-sdk ships no config template for it because
**the seam file's content is resolved at install time rather than shipped**: its
one line sets `GATE_SDK_NATIVE_BIN` to the path the artifact was actually placed
at, and that path does not exist until selection has run. A static template could
only be copied and then immediately rewritten — the same file written twice, with
the copy contributing nothing. That is a property of the value, not of how the
template seam happens to copy, so it does not expire when the copy changes.
The knob's own default is unchanged and still names the crate's build output,
because it is a **stable relative path** on purpose: the generated hook persists
the emitted argv, so a machine-specific path baked into a tracked hook would make
the graph artifact's freshness comparison machine-dependent. The seam is claimed
like any rewritten surface and then rewritten preserving every line except the
one setting `GATE_SDK_NATIVE_BIN`, seeding the two shellcheck directives only
when the file is absent — so an adopter's own knobs in that file survive every re-run.

**The non-destructive re-run is the op's too, and `--seam` is where it applies.**
The seam is claimed against the hash `--lock` records for it and left alone when
it differs, unless `--force` — it is a file you edit. `--dest` is not claimed
(§The gate binary): the copy is skipped when the recorded target, the recorded
digest and the on-disk digest all agree with `--target` and `--digest`, and made
otherwise. That is what makes a bare re-run leave the tree byte-identical and a
substituted binary get replaced, from the one test rather than two.

**Ordering is load-bearing.** The op is called after every config seam is in
place and before the pre-commit hook is generated, because the generator
resolves each member's invocation argv and a `.gate` member resolves to this
binary — the knob must name it and the file must be there. A hook generated
first would resolve a dispatch it cannot make.

Every row of the table above runs under an oracle rather than a
hand-verification with a date on it, and the rows are split across §The consumer
smoke's arms by which install can show them. The **verify-then-write** row is the
main loop's, on every profile, because the payload it packs carries a binary this
run built; the **unrostered-host** row is the binary-less leg's and the artifact
arm's undeclared-host leg's; the **broken-payload** row is the artifact arm's
alone, against a payload it mutated after extraction. Nothing here changes for that —
the behavior this section specifies is what those arms assert, not something they
added.

## doctor

`checkwright doctor` tells you whether this machine meets the toolchain the
gate battery needs, and it says so in its **exit status** rather than only in
its output: `0` meets the contract, `1` is below it. That is what lets a CI
step or `init`'s own precondition check gate on the answer without parsing a
report — and it is why a below-contract machine is caught before any partial
install rather than halfway through one.

**Which toolchain: the consumer-audience subset of the roster.** The roster
carries an audience axis (context-kit/SPEC.md §bin/env-probe), and a member
declared contributor-side is one no install path and no vendored gate reaches —
`cargo` is the case that exists. `doctor` walks the roster through that
predicate and a contributor-audience member is not probed, not rendered and
cannot set the verdict. It is left out rather than reported as informational on
purpose: `doctor` is the adopter's verb, and showing an adopter a tool they do
not need is an invitation to install it. So **`DOCTOR: clean` is a claim about
this machine as a consumer**, not about the machine — which is the narrowing
that makes the exit status usable as `init`'s precondition, since what `init`
needs to know is exactly whether the tree it is about to vendor into will run.

It has two behaviors, selected by where you run it rather than by a flag. Run
anywhere, it reports the toolchain verdict. Run inside a repository that has
been vendored into, it additionally reads `checkwright.lock` and reports the
installed release, the upstream commit it came from, the profile, and the kit
set — plus, where one was installed, the gate binary's target re-verified
against its recorded digest **in place**, and any member the registry records as
omitted, against whatever reason it carries. `doctor` invents no remedy for that
reason: the class is reason-agnostic and the two install-time tokens that once had
one are retired (§The gate binary).

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

**The identity block names the registry this tree's battery runs from**, beside
the version, commit, profile and kit set. It is the one install fact the identity
fields cannot be read off, and the one a reader most needs when the report and
the tree seem to disagree: a `doctor` that resolved the wrong file would
otherwise say nothing at all about which one it inspected. A recorded registry
missing from disk is named as such, and a manifest recording none says so — both
with the same remedy, a re-run of `init`.

`doctor` defines no floor of its own. It reads the toolchain roster from the
module that also holds the floor predicate (context-kit/SPEC.md §bin/env-probe)
and renders whatever verdict that predicate
returns, so the contract keeps one owner and this stays a display of it. Which
members are consumer-audience — and so which absences set the verdict — is read
off that roster, never listed here; §Requirements works the one case a reader
arrives with, `jq`, and says why it is below contract rather than outside it. The
roster is the binary's own rather than a copy in the tree it is inspecting:
at `init` time nothing has been vendored there yet, so a tree copy would not
exist at the moment the answer is needed.

**A residue is read apart from an install, and reported as one.** A
`checkwright.lock` carrying `files` and no `version` is not an install — it is
what `uninstall` leaves over files you had edited (§uninstall) — and `version`
is the field an install always has and a residue never does, so its absence is
the discriminator. On that reading `doctor` reports how many files remain and
that they are yours, and prints **no** identity block, no artifact check and no
registry report: every one of those is a per-install reading with nothing
left to describe. Blank identity fields would be uninformative rather than
wrong, but printing them beside a residue message is the same mixed-verdict
shape the exit-status carve-out above already refuses.

On both readings `doctor` closes by naming `diff`. It reports what was
installed, never whether the tree still matches it, and keeping the two apart is
what stops `DOCTOR: clean` from being read as a claim about the tree's contents.

A third exit status, `2`, means the question could not be answered rather than
that the answer was bad: the package carries no payload, or the manifest
carries a schema key this build does not know. A build refuses an unfamiliar
manifest rather than guessing at the shape behind it. **A usage error is that
same status by derivation rather than by a new code** — a verb refusing an
unknown argument has not judged the tree, the package or the manifest, which is
what `1` is reserved for; it could not answer.

## update

`checkwright update` is `init` with **one added precondition and its arguments
forwarded verbatim**: `checkwright.lock` must already exist, so a verb named
`update` can manage an install but never perform the first one. Every `init`
flag stays valid — `--profile`, `--force`, `--no-commit` and `--dry-run` among
them — because none of them is reimplemented here. `update` checks its one
precondition and then *becomes* `init`.

That is the whole of the difference, and it is deliberate. Upgrading is already
what a second `init` does: it compares the recorded version against the
payload's and falls through in the upgrade direction, re-reads the profile from
the manifest when none is passed, and re-applies the payload around every file
you have edited since. A separate implementation would be a second copy of all
of that, free to drift from the original. So `update` is a **name**, not a
second mechanism — one operation under two names, where the second name is the
one an adopter looks for.

It checks existence and no more. An unreadable schema, a stale downgrade, a
below-contract toolchain — each is `init`'s own precondition, one call away, and
repeating any of them here would be the copy this design exists to avoid. Being
outside a git work tree is not this verb's precondition either: `init`'s own
refusal already names the accurate remedy, so an unresolvable root falls through
to it rather than being misreported as an absent manifest.

**Residue, recorded rather than papered over.** Because the delegation is real,
most of what you see comes from `init` and says so. The one refusal `update`
owns is prefixed `checkwright update:`; every other refusal — not a git work
tree, an unknown schema, a dirty worktree, a stale downgrade, a below-contract
toolchain — arrives prefixed **`checkwright init:`**, and the
success path
reports `INIT:`, because that is literally which verb produced the line. A
refusal raised inside a shared module inherits this without a second rule: it
surfaces through the calling verb's own refusal shape, so it carries that verb's
prefix, its `help:` line and its exit code rather than a separate idiom. This is
honest rather than untidy: the prefix names the operation that actually ran, and
renaming it cosmetically would hide exactly the delegation that makes these two
verbs one. It is written down here so that a `checkwright init:` line answering
a command you typed as `update` reads as the design rather than as a bug.

## diff

`checkwright diff` answers *which of the files `init` wrote have I changed?* It
classifies every `files` entry against the tree with the **same hash comparison
`init` makes** before it rewrites anything, so the report and the protection are
never two opinions.

Three classes, two of them named apart rather than pooled. **Unchanged** is
counted only. **Changed** means the content differs from what `init` wrote — the
sanctioned case, and the one the ownership contract exists to protect.
**Missing** means a recorded path is not on disk, and it is separated
because its consequence differs: §The manifest's exit rule means the next `init`
silently drops a missing path from the roster and writes it fresh, which is
worth a warning before it happens rather than after.

**The exit status is the verdict**: `0` every recorded entry matches what `init`
wrote, `1` at least one has changed or gone missing. That is what being a verb
of its own buys — a CI step can gate on *is our vendored tree pristine?* without
parsing a report, and without borrowing a status the toolchain contract owns
(§The verbs).

Its preconditions are the ones its subject requires: inside a git work tree, a
manifest present, and a schema this build knows. It writes nothing, so it has no
`--dry-run`.

Run against a **residual** manifest, `diff` reports the survivors as `changed`.
That is neither a special case nor a defect: they are recorded at the hashes
`init` wrote and they carry your edits, so *changed* is exactly what they are.

## uninstall

`checkwright uninstall` reverses an install against the roster `init` recorded,
and against nothing else. Install, see what it does, reverse it — that is the
property the adoption story rests on rather than an ergonomic extra.

**Preconditions**, all refusing rather than warning, and all checked before
anything is removed, because a partial removal is the outcome none of them may
produce: inside a git work tree; a manifest present with a schema this build
knows, else a refusal naming `init`; and a clean worktree, for `init`'s own
reason — one commit is made, and a dirty tree would fold your work into it.
`--no-commit` is the same valve on the same terms. The parallel with `init`
stops one clause short, and the divergence is deliberate rather than an
oversight to be tidied: `init` regenerates on every run, so it needs an arm that
commits what a no-op rewrote (§init), while `uninstall` only ever removes — a
removal it declines to make writes nothing, so there is no residue for such an
arm to find.

**The removal rule is the ownership contract seen from the other side.** For
each `files` entry: hash the file, remove it while the hash still matches what
`init` recorded, and **keep and report** it when it differs. So it removes only
files this installer wrote and you have not touched, never a file you wrote —
and it needs no new data to do that, because §The manifest already records every
path at the hash `init` last wrote there. A recorded path already off the tree
is a no-op rather than an error: it left the roster when it left the tree.
`--force` removes what would otherwise be kept, meaning here exactly what §init
says it means there — overwrite what `init` would otherwise protect.

**A removal is not bounded by the host's argv width either**, on §init's rule and
for the same reason: this verb names the whole recorded roster to `git` twice, to
ask which of those paths the repository tracks and then to stage them, and a
large profile's roster overflows the command line a host will accept. Both calls
are issued in batches. The first of them is additionally read through a file
rather than a process substitution, because **its status has to survive**: every
file is already off the worktree by then, so a read that failed and returned
nothing would stage the manifest alone and commit it under a message announcing
the removal, leaving the deletions unstaged and unmentioned. That failure was
silent before it was bounded, and batching widens it from total failure to
partial, which is why the status is now captured and refused on rather than
discarded.

The gate binary needs no special case. `init` records it as an ordinary `files`
row *and* under the separate `artifact` key; the roster walk removes it like any
other row, because `artifact` is identity rather than ownership.

**The manifest is not a `files` row, and it is disposed of explicitly.** `init`
appends `checkwright.lock` to its written set only after emitting it, so the
file never records itself. With nothing kept, it is deleted. With any entry
**kept**, it is rewritten over the survivors instead: deleting it
would disown exactly the paths the hash rule just protected, and the next `init`
would find them unrecorded, read that as *never installed*, and write straight
through your edits. That residual shape, and the argument for it, are owned by
§The manifest.

**The agent file is the one non-whole-file removal**, because it is the one
entry that is a span rather than a file (§The manifest). Two branches. Its hash
matches, meaning `init` created it and you never touched it, so it goes with
everything else. Its hash differs, so the file is kept and the **doctrine block
is trimmed out of it** — that block is prose you did not write, in the one file
whose purpose is to steer agent sessions, pointing at a doctrine file this verb
just removed, and leaving it inert there is not neutral. The trim runs through
the payload's own copy of doctrine-kit's installer (doctrine-kit/SPEC.md
§install-doctrine), so it adds no second copy of the marker strings and still
works once the vendored kit is gone. It is scoped to that span alone: a
marker-bounded block belonging to some other tool is one `init` never wrote, so
it is never a `files` entry and never this verb's to touch. The trim is left
**unstaged** — the rest of that file is yours to review.

**Directory pruning is bottom-up and removes only what is now empty.** A
directory still holding anything is left alone, whether that is a file you added
or one you edited: `uninstall` removes files it owns, never directories it
merely emptied around. A file you added inside a vendored directory is not on
the roster, so it is never removed — and `--dry-run` names it, because a
directory left behind holding only your own files is a surprise worth spending a
line on before the run rather than after.

**The hook opt-in is reported, not rewritten.** When `core.hooksPath` points
inside the gates directory that was just removed, `uninstall` prints the
`git config --unset core.hooksPath` line for you to run and does not run it. Git
config is outside the ownership roster, and a `core.hooksPath` naming a
directory that is not there is inert rather than breaking, so there is
nothing here to justify writing outside the contract.

`--dry-run` prints the plan — what would be removed, what would be kept and why,
the residual manifest if there is one, and any file of yours that will be left
behind inside a vendored directory — writes nothing, and exits 0.

**The commit** is one commit naming the profile and the version, staging the
removals and the manifest disposition. Files kept for you are never staged, on
the same reasoning that keeps `init`'s written set and its recorded roster
apart. A run with nothing to remove says so and exits 0 without narrowing the
manifest: the install is still there, so disowning it would be false.

## Profiles

You pick how much of the methodology to meet first, and *which part* of it —
`starter`, `delegation`, `prose`, `full`. Those are not four rungs on one ladder:
`delegation` and `prose` contain neither the other, because they answer different
questions about what your repository is.

**The contract is the lattice, and it always was.** Profiles are ordered
by kit-set containment, and that order is derived from the rosters rather than
declared beside them. The promise it makes, stated precisely: *moving from a
profile to one that contains it only ever adds — to the vendored tree and to the
battery you run — and profiles that contain neither the other are alternatives,
not steps.* The order is bounded: exactly one profile sits below every other,
exactly one above every other, and the one above is `full` by construction.
So nothing you already vendored is taken away or rearranged underneath you when
you move up, and a profile that is nobody's step is still a legitimate member.

- **`starter`** is the framework — the gate SDK on its own. You get a battery,
  a generated pre-commit hook, and gates that already red on real defects in
  your own tree without any configuration outside your gates directory.
- **`delegation`** adds every kit whose subject is the agent session itself:
  the stage machine a session runs, the queue it selects work from, the
  evidence a stage produces before it can close, the protocol it follows when
  it spawns, the context budget it runs inside, the permission surface it acts
  through, and the delivery doctrine it follows.
- **`prose`** adds canon-kit instead: the kit whose subject is authored
  documents, for a repository whose artifacts are prose rather than code. What
  arrives armed is link, claim, staleness and pointer governance over every
  `README.md` at any depth and your agent file — real for a documentation repo,
  since a docs tree is usually a tree of READMEs. It is deliberately **not**
  governance over `docs/*.md`: no kit spells one project's prose layout, so
  widening the corpus is your own `CANON_KIT_PROSE_SURFACE_GLOBS` line in the
  canon-kit config seam `init` already writes into your gates directory.
- **`full`** is everything in the payload.

`starter`, `delegation` and `prose` are rosters in `profiles.list`, because none
follows from the tree — each is a judgment about what an adopter should meet
first, and the file records the criterion behind each membership beside it.
`full` is derived instead: it is every kit root the payload carries, resolved
at run time, never a list to maintain. The **shape** is derived too — the order,
its bounds, and the monotonicity above are computed from the rosters and the
payload by the profile module behind the invoke, so a membership row and a
declared parent can never disagree.

## The manifest

`init` writes `checkwright.lock` at the root of the repository it vendors into,
and that file is tracked like everything else it writes. It is JSON and it is
the install-ownership record: what was installed, from which
upstream state, and which files this installer owns. One module behind the invoke
is its schema owner — the wire key, the accessors, the hash rule and the emitter
live there, so the arm that writes a manifest and the arms that read one
cannot drift apart. Being the single writer is what makes the shape a contract
rather than a convention: keys are sorted at every nesting level, and a field is
present exactly when its writer supplied one, so an omission leaves the key
**absent** rather than null or blank. `commit` is the field that rule most
recently had to be applied to: a package with no commit stamp leaves it out of
the manifest rather than writing it empty, on the same conditional footing the
`artifact` key already sat on.

**The consumer-layout constants have one owner, and it is behind the invoke.**
Where the gates directory, the agent file and the queue file sit in a vendored
tree are constants the installer module owns. A reader outside the crate — a
smoke harness, a consumer script — takes them from that module rather than
spelling a copy, a second spelling being a second thing to stale. Named here,
valued there: this is the pointer, not the roster.

**Reading it needs no external program.** The crate parses it with `serde_json`,
so a refusal an adopter meets here — *this package carries no version stamp*, or
*the manifest carries a schema this build does not know* — means exactly the
condition it names and is never a misdiagnosed missing tool.

The wire key is versioned (`checkwright-lock v1`)
and a build that meets a key it does not know refuses rather than guessing at
the shape behind it.

| Field | Holds | Read by |
| --- | --- | --- |
| `schema` | the versioned wire key | every verb, as its first act — an unknown key is a refusal |
| `version` | the release the payload was cut at | `doctor` reports it; a re-run of `init` compares it against the payload's and refuses a silent downgrade |
| `commit` | the 40-hex commit the payload was assembled from | `doctor` prints it — it is what lets a reviewer resolve the vendored tree to an exact upstream state |
| `profile` | the profile selected | a re-run of `init` re-applies the same profile without asking again |
| `kits` | the vendored kit set | `init`'s re-run file plan, and `doctor`'s installed-set report |
| `files` | `init`'s ownership roster — each path it has written, at the content hash it last wrote there, until the file leaves the tree | `init`'s changed-file detection: a file whose hash still matches is rewritten, one that has changed is reported rather than overwritten — the gate binary's row excepted, §The gate binary — and stays on the roster so the next run reads it the same way, whether or not the running release still ships that path. `uninstall` walks the same roster to decide what it may remove and what it must keep, and `diff` classifies it against the tree |
| `artifact` | the gate binary's `target` and its SHA-256 `digest`, or absent | `doctor` reports the target and re-verifies the digest in place; a re-run of `init` compares the target against this host and skips the rewrite while the digest still holds |

**Resolving one of your own seam files is an exact-path question, not a search.**
`doctor` and `uninstall` both need to know which `gates.list` — or which
`gate-sdk-config.sh` — is *yours*, because the vendored kits carry fixture trees
holding files of the same name. `files` already answers it: it records the
repo-relative path `init` wrote, and the layout constants live beside the recipe
that wrote it under them, so the resolver asks whether the manifest
records that exact path and returns it or nothing. A tail match cannot answer it,
and **no predicate over the recorded kit set repairs a tail match**: `files`
outlives `kits` by design, so a re-run at a narrower profile leaves the dropped
kits' fixture paths on the roster with nothing excluding them, and the residual
manifest below carries no `kits` key at all — there, such a predicate excludes
nothing whatever. The consumer smoke's narrowing arm is the oracle for both.

**A recorded hash is what `init` last wrote at that path** — on whichever run
last wrote it — and not the state of the tree at the end of the current run. The
two readings coincide for every path `init` rewrites and part company for exactly
one class: a path `init` left alone because you had edited it. That path stays in
`files` at the hash `init` put there, carried forward from the previous manifest
rather than recomputed, so the next run still has something to compare your
content against, still finds it different, and still leaves it alone. Editing a
file changes who may write it; it never changes whether `init` is tracking it.

**`checkwright.lock` is not on its own roster.** `init` adds the manifest to what
it has written only after emitting it, so the lock file never records itself. Any
path that walks `files` to decide what this installer owns therefore has to
dispose of the manifest explicitly: the roster will not name it.

**One entry is a span rather than a file, and it is still an ordinary entry.**
Everything above reads a `files` row as a whole file `init` wrote. The agent
file is the exception: `init` creates it only when it is absent, and thereafter
authors only the marker-bounded doctrine block *inside* it — everything outside
that span is yours and is never read or rewritten. The row is nonetheless
whole-file, hashed and compared like every other, and two things follow. Your
edit **anywhere** in the file marks the entry changed, which is a true report
rather than a defect. And that report does not stop the span from being
maintained: the block is injected before the file is claimed, so the doctrine
block keeps upgrading on every run while the rest of the file stays yours. What
survives *inside* the span across that rewrite is doctrine-kit's contract rather
than this installer's — its declared-trim round-trip and the bound on it are
`doctrine-kit/SPEC.md` §install-doctrine.

**A path leaves the roster when the file leaves the tree, and at no other
moment.** `init` owns a path because it wrote the file there, so only the file's
disappearance can end that. A release that stops shipping the path does not: the
file is still on disk, it may carry your edits, and disowning it is exactly what
would let a later release re-adding the same path write straight through them.
Neither does an `uninstall` that **kept** the file: a file kept is a file `init`
wrote that is still on disk, so its ownership has not ended and the roster must
retain it. That is the rule the residual manifest below follows, rather than a
carve-out from it.

**A relinquished path is an ordinary entry, not a state of its own.** When a
release stops shipping something `init` created, nothing visits that path on the
run, so it is carried forward at the hash `init` last wrote there — the same
carry-forward, the same `files` row, the same protection every other entry gets.
There is no *once ours, now relinquished* state for a reader to learn. Nor can
the roster grow without bound from it: the existence test is already the reaper,
so `files` is bounded by the files `init` created that still exist, which is the
ownership set itself. What `init` never does is delete, so a path no release
ships any more stays on disk and stays yours to remove.

**"Nothing visited it on the run" is a claim about the whole run, so the
carry-forward is the last pass `init` makes before it stages.** Every path `init`
writes has to be on the written set by the time that pass runs, or it carries a
path `init` rewrote seconds earlier at a hash the run has already superseded — and
a carried path is not staged, so the file is left dirty *and* the roster records a
hash the run itself has replaced. Both halves reach past the worktree: `uninstall`
and `diff` read that hash to decide whether a path is still `init`'s, so a
superseded one reads to them as your edit. The generated projections are the ones
that can reach the pass from the wrong side, which is why `init` produces them
before it rather than after.

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

**The residual manifest** is the second shape this file takes, and the exit rule
above is what produces it. When `uninstall` keeps at least one file, it rewrites
`checkwright.lock` over the survivors, and that object carries **`schema` and
`files` only**. `version`, `commit`, `profile`, `kits` and `artifact` all
describe an install this tree does not have, and a manifest asserting them would
be false. The survivors are recorded at the hashes `init` wrote and never at
yours: an entry rewritten at your hash would read as *unchanged* on the next
install and let it write straight through you, which is the defect this shape
exists to prevent — the same reason the roster names a dropped entry and an
adopter-hashed entry apart.

The wire key stays `checkwright-lock v1` here too, on the precedent just above:
an identity field an install always carries is optional within a shape a reader
already tolerates, which is a change of meaning rather than of shape. That it is
safe is a property of the existing readers, checked rather than assumed.
`init`'s downgrade refusal is guarded on a non-empty `version`, so an absent one
skips it — correct, since there is nothing to roll back. `init` re-reads
`profile` only to default it, so an absent one falls through to `starter`, which
is what a virgin tree gets and what the residue is. `doctor` reads the identity
fields for display, and rather than printing them blank it reads their absence
as the residue itself (§doctor). The honest residue is that a reader built
before this meets a residual manifest and prints blank identity fields —
uninformative, never wrong-acting.

A recorded **`files`** hash is `git hash-object`, never `sha256sum`. Not a
portability detail worth burying: macOS ships `shasum` rather than `sha256sum`,
and the answer is not a new tool requirement — git is already something the
toolchain contract asserts, its object hash is content-addressed and stable, so
the manifest's integrity story stays inside the toolchain that contract already
covers.

**It is a *filtered* `git hash-object`, and that is the definition rather than a
spelling.** The recorded-hash helper calls the command with no `--no-filters`, so
the value it records is the hash of the content *after* whatever `text`/`eol` attribute and
`core.autocrlf` setting the path's own attribute chain selects. Moving the call
to `--no-filters` would therefore not be a spelling fix. A filtered hash and a
raw hash disagree about exactly one population — an adopter whose edit is a
line-ending change — so the switch decides that `init` now notices such an edit
and refuses to overwrite it, which is a behavior change to the non-destructive
re-run on **every** platform and belongs to a unit scoped to it. It was
**refused 2026-09-05 by the lead** as a repair for the native Windows leg's
manifest disagreement (§The consumer smoke), on that ground and on one more: a
repair guessed at a mechanism no probe reproduces is a guess whatever else it
is, and a cause read carries no authority to change what a `files` hash means.

**The command has two call sites and they do not run from the same working
directory**, which is the first thing a cause read reaches for and which no
surface owned. The manifest writer records one value; the consumer smoke's
comparison value is hashed from its own scratch root (§The consumer smoke).
Neither passes `--no-filters`. Whether that difference is what a disagreement
measures is not settled here — §The consumer smoke's three-way probe exists to
answer it — but a reader holding two values is holding two invocations, not one.

**Two hash families, answering two questions, each stated where it is used.**
The rule above is scoped to `files`, and the `artifact` digest is deliberately
outside it — not because the two describe different files, but because they ask
different things about one. The two hashes are not unified. A `files` hash is
**change detection** — has the adopter edited something `init` wrote — where
collision resistance is not the property needed and staying inside git's
already-asserted toolchain is worth more. The artifact's digest is an
**integrity claim**, published for a reader to cross-check; `git hash-object`
defaults to SHA-1, and a SHA-1 supply-chain digest would undercut the one claim
§The gate binary makes.

**The artifact is on both maps, and the split is path versus digest rather than
key versus row.** The binary's *path* is an ordinary `files` row, recorded with
the same `git hash-object` hash every other entry carries, because `init` wrote
the file there and §uninstall must reverse it against the same roster — an
artifact off that roster would be a file `init` created and left behind. Only
the *digest* rides the separate `artifact` key, on the split just above. So the
map's uniformity holds where it was ever claimed: every `files` hash is still
`git hash-object`. Row and key compose on the one file — the row says whether the
tree still holds what `init` wrote, the key says whether it is the artifact that
was published — and §doctor, §uninstall and §init each read the one they mean,
§init alone reading the row as ownership for every path but this one.

A new optional top-level key is additive within the versioned wire key: a reader
that does not know it sees the same manifest it always did.

**The `artifact` key carries no path, and that is not an omission.** The install
location has exactly one owner — `GATE_SDK_NATIVE_BIN` in your
`gate-sdk-config.sh` — and that value is what the battery actually dispatches
to. A second copy under this key could disagree with the live one, so every
reader asking *where is the binary* resolves it from the same owner: §doctor
reads the knob and re-verifies whatever it finds there.

**The `files` row is not that copy, even though it is a path.** It records where
`init` wrote, which is a fact about the install and stays true when the knob is
later repointed; the knob records where the battery dispatches. They coincide on
every tree nobody has repointed, and where they part it is the row that is still
right about what `uninstall` has to reverse. The distinction is the one every
`files` row already carries, not a special case for this one.

## The packer

`scripts/pack-installer.sh` assembles the payload both transports ship — the
Release tarball and the npm package — out of this repo's own kit roots, and
npm-packs it in a scratch directory outside the worktree. This section owns that
file: its boundary, its port disposition, and the contracts its own `# spec:`
pointers cite.

**Nothing is written inside the worktree, and no second copy of any kit is ever
checked in.** The payload's kit set is derived at pack time from the roots the
battery itself enumerates, so the shipped set cannot drift from the governed
one, and the assembly happens under `INSTALLER_PACK_TMP_DIR` rather than in
tree.

**The payload is stamped with the commit it was packed from, so the tree must be
clean.** That stamp is the whole of what makes a vendored tree resolvable to an
upstream state, and a dirty tree would stamp a commit the payload does not
describe — so the packer refuses instead of stamping one. The version stamped
beside it comes from the newest reachable tag unless `--version` names one, and
never from an edit to `installer/package.json`.

**The caller names the tree it means.** `--root` takes the work-tree top level
to pack and stamp, and the value is validated to be one: silently promoting a
named subdirectory to its toplevel would be the same quiet correction the flag
exists to remove, since without it the current directory selects a tree the
caller never named. Absent `--root`, the git toplevel of the current directory
is what gets packed.

**The flag roster has exactly one tier and it is the tool itself.** `--help`
prints it on stdout at exit 0, so no doc carries a second copy to drift. Help is
adopted here on its own merits and does not extend gate-sdk/SPEC.md §The
bin/-tool contract to this repo's `scripts/`: with a flag surface this wide, an
unknown-argument refusal is too thin a discovery route to be a caller's only
one.

**The port disposition is a plain obligation, and the sequencing sentence is
that nothing sequences it.** The packer carries no `bootstrap` step — §The
install boundary's three dispositions apply to the steps of an install, and the
packer runs no install but assembles the artifact an install later consumes, so
that section's behind-invoke hold does not reach it. It ships to no adopter: the
packed set is `installer/` plus the enumerated kit roots, `scripts/` is neither,
and no adopter receives or executes this file. Nothing holds it either: its
lines sit in the reachable column of `--emit port-blockers --tree`, and a cut is
authored against this section.

**A cut owes the binary's reachability at both callers, and the two are not in
the same position.** `consumer-smoke/run-smoke.sh` builds the gate binary before
it reaches any of its pack call sites, on the host it runs on, so a compiled
form is already reachable there — the macOS install-smoke legs included, though
not by building: since the platform-evidence merge those consume the build
legs' artifacts through `INSTALLER_SMOKE_ARTIFACTS_DIR` and adopt a compiled
binary from the hand-off. The release publish
workflow is not: it packs in a job with its own checkout and no build step,
holding the downloaded per-target artifacts and no built binary. So a cut must
make the binary reachable in that job — by building it there, or by resolving
`GATE_SDK_NATIVE_BIN` onto the artifact the build legs already produced — and
that lands in the same unit as the cut rather than after it.
**Of those two routes the first is the one a cut takes, and the second is
refused.** A cut builds the binary in that job. `gate-sdk/SPEC.md` §Consumer
payload's *builds nothing itself* is bounded to the payload artifacts and does
not reach a binary built there for tooling; a `cargo build` in `pack:` compiles
bytes that job checked out at the tag, so it leaves intact the
checkout-plus-bash tamper floor the workflow's own header states; and
build-then-pack in one job is rehearsed on every CI run rather than first
exercised by a real publish. Resolving `GATE_SDK_NATIVE_BIN` onto a downloaded
artifact is refused instead: nothing in the tree digest-verifies against an
independent source, so it would make the one job that assembles and stamps the
published tarball execute bytes it did not check out.
`pack-installer-cut-to-a-non-gate-arm` carries the disposition, the cost and
the grounds. So the obligation above is work a cut can take, and what remains
between here and a cut is scheduling rather than a blocker.

**The disposition is stated here rather than in a kit SPEC, and the ground is
the provenance seam.** gate-sdk is a kit, vendored into every adopter's tree. A
kit SPEC section governing the disposition of a file that lives in this repo's
`scripts/` — a file no adopter receives, and whose existence no adopter can
verify — inverts the kit/consumer layering and publishes a rule about a private
tool as kit mechanism. Independently, gate-sdk/SPEC.md §Consumer payload bounds
its own reach to what a gate ships, and a port disposition for a release
assembler is not a disclosure rule; the payload-content rules that section
states about the packer stay exactly where they are, because already describing
a file is not owning it. This surface is repo-root-governed with no owning kit,
which is the governance class the packer is in, and it already hosts this shape
for the harness §The consumer smoke governs — a repo-private tool that rides no
payload, whose non-shipping status is established by citing the packer. Ruled
`lead, own-authority` 2026-09-06, the seam ground primary.

## The consumer smoke

`consumer-smoke/run-smoke.sh` is the acceptor for everything above, and it is
registered as a validate suite so a bit-rotted activation path is a red
validate rather than a discovery at announcement.

**The port disposition — `run-smoke.sh` is declared `no-port`, ruled 2026-08-31
by the operator in consult.** It is this repo's acceptance harness and rides no
payload: `scripts/pack-installer.sh` assembles both transports out of the kit
roots and never out of `installer/consumer-smoke/`, so no adopter receives it
and no adopter path executes it — the ground gate-sdk/SPEC.md §Consumer smoke,
*The port disposition* declares the kit `smoke/` class on at its leg 3, reached
one step further here for a harness the payload does not even carry. The 2026-08-28
predicate ruling (TRAJECTORY.md §The closed rulings) named this file as one of
the two that ship to no adopter and declined to turn that ground into a
*class*; this is a per-file disposition under the case-by-case residue rule that
ruling left standing, and it mints no class. The declaration sits in the file's
own header and cites this paragraph. What it does **not** say: nothing here
reaches `installer/bin/`, whose disposition §The install boundary states.

**The smoke packs the tree it lives in, by construction — the current directory
does not select it.** The script resolves that tree from its own path and hands
it to `scripts/pack-installer.sh --root` at every one of its pack call sites, so
the packed tree and the asserted tree are the same tree whatever directory you
invoke from. A clone's copy invoked by absolute path, a second
checkout, a linked worktree: all pack the tree the script belongs to. What
`--root` promises the caller, and the packer's single-tier flag roster, are
§The packer's — this section cites that contract rather than hosting a copy of
it.

*The former invocation requirement is retired, not merely unstated.* Until
`--root` existed, this section carried a standing rule — "run it with the
current directory inside the tree under test", with `env -C <clone>` as the
remedy — because the packer resolved its root from the current directory while
the smoke resolved its own from its script path, and nothing detected the
disagreement: the run packed the *invoking* tree and printed an ordinary success
for source the clone never contributed. That rule is now false rather than
merely unnecessary — following it changes nothing, and a reader who restores it
is guarding a hole that `--root` closed. The `PACK:` line names its resolved
root for exactly this reason, so a reviewer scanning smoke output can see which
tree the run described instead of inferring it from a twelve-character hash.

**The tree must be clean, and the suite says so before it spends ten minutes
proving it.** The smoke asserts that tree is clean in its preflight, beside the
tool checks and before the first build step. The packer keeps its own
dirty-worktree refusal — the preflight removes the common case but not the real
one, since the suite packs four separate times across a ~10-minute run and a
concurrent edit mid-run trips a check the caller did not choose the moment for.
That late refusal names the root it resolved and states that the check is
per-invocation, so it reads as a precondition checked at an awkward moment
rather than as a broken installer.

It builds the host gate binary, packs the package around it, installs it **from
the resulting tarball with
`--offline`**, and drives a scratch consumer once per profile: `init`, then the **follow-up
arm** — every command `init` printed in its block must resolve to an executable
path in the payload just written, with every flag it names accepted — then the
battery must be green, then the manifest must agree with the tree it describes
file by file, then the **queue post-condition** — a profile whose kit set reads
the queue file must have one, satisfying `check-queue-sections`, and a profile
whose kit set does not must have none — then a re-run must leave the tree object
identical, then `doctor`
must exit 0 and name the installed profile, and then the **value arm** — the
consumer authors one page of markdown carrying one real defect, a mistyped
relative link in a `README.md`, and the battery's verdict on it is recorded
before the link is corrected and the battery must be green again. The fix is the
link and never the corpus: the tree the second run sees is byte-identical to the
first apart from the typo, so the green is the defect being gone rather than the
scan having narrowed, and a profile green on the first run must still be green
on the second. The arm restores the consumer to the commit it found, so the
reversal below still runs against the tree `init` wrote.

**The follow-up arm asserts what `init` printed, not a copy of it.** `init` ends
by telling the adopter to run two commands, and until this arm nothing anywhere
checked that either one resolved: they are `printf` format literals in installer
shell source, which no markdown-fence corpus reaches. The cheap assertion — write
the expected pair into the smoke and compare — mints exactly the defect it would
be closing, so the arm instead **parses the pair out of the output the invocation
already captured**, under the grammar §init states. A rename moves the printed
string and the arm follows it with no edit here. Three assertions per extracted
command, in this order because each makes the next meaningful:

- **The block is present and non-empty.** Zero commands extracted is a red, never
  a skip: an assertion over an empty set passes vacuously, so a reflowed banner
  would otherwise turn full coverage into silent zero coverage.
- **The target resolves and is executable** inside the consumer the arm was handed.
- **Each flag the line names is accepted by that target.** `--install-hooks` is
  the live case: a path that still resolves while its flag does not is precisely
  the half a path check alone misses. The probe is the target's **own refusal
  behavior**, and it is two-sided so it cannot pass vacuously — a negative control
  runs the line with that flag replaced by a token derived from it and guaranteed
  unknown, and the refusal it earns *by name* is what the positive run of the line
  as printed is then measured against. No refusal string is spelled in the smoke:
  the expected one is the control's own line with the sentinel substituted back.

*The flag half is unconditional, and the branch that once skipped it is gone.*
It was a parameter of this arm while an install could complete with no artifact:
asking a front-end with nothing to dispatch to whether a flag is one it accepts
got the absent-binary refusal whatever the flag was, so the answer would have
been an artifact of the leg rather than a fact about the flag. Selection has one
success path now, so there is no such install left to make — every consumer this
arm is handed was written by a verb that ran, and a verb runs only past a
verified artifact. The parameter and its one non-green value retired together
with the leg that passed it.

*Which `init` invocation it rides, and why it is not the obvious one.* It rides
the first `init --profile` call, never the idempotent re-run below: that re-run's
no-op branch prints no banner at all, so an arm placed there would assert over an
empty block on every profile and pass by vacuity — the exact hole the first
assertion closes, arriving through the back door.

*The probe runs in a throwaway copy of the consumer, and that is not thrift.*
Asserting acceptance through the target's refusal behavior means **running the
printed command**, and the live flag wires the clone's `core.hooksPath`. Run in
the consumer itself, it would put a generated pre-commit hook in front of every
later arm's commit — including the value arm's, which commits a deliberately
defective page and expects the battery to catch it rather than the commit to be
blocked. The arm would then decide the thing it exists to observe. The copy is
made from the payload just installed, so the probe still runs against the tree
under test and never against this repo's own.

*Its verdict classes are this file's existing line, not a second one.* A printed
command that does not resolve, or a flag the target refuses, is a statement about
the payload `init` just wrote: `fail` at exit 1. A block the arm could not read
at all — `init` succeeded and printed nothing the grammar matches, or a command
line names no path — is a failure of this harness's own construction: `blocked`
at exit 2. What the arm does not assert is that the commands *succeed*; the
battery arm two steps down already owns that, and conflating them would make one
red unreadable as either.

**A disagreeing manifest arm reports what it found, in place, before it fails.**
On the failure path only, the arm prints a bounded fact set about the
disagreement and only then picks its verdict — the class of which is settled
below. It has to be the arm and
not a step beside it: this script mktemps its scratch under `trap cleanup EXIT`
and `fail` exits 1, so the consumer whose manifest disagreed is torn down before
anything later could open its `checkwright.lock`. The standing alternative was a
CI step standing up a *second* consumer by the same route — a duplicate of this
install path, maintained in YAML, unrunnable locally and free to drift from the
check it diagnoses, which it did twice (below). The arm already holds the
failing state, so removing the duplication beats maintaining it. On a green leg
none of this runs and the arm prints the `manifest: N file(s) agree with the
tree` line it always printed.

**It samples at most three paths, and none of them is chosen by arrival.** The
first disagreeing path is one, because it is what a reader would have opened
anyway.
The second is the **artifact's `files` row** — §The manifest's ordinary row for
the binary, recorded with the same `git hash-object` hash every entry carries —
whenever the manifest records an `artifact` key and that row is in the
disagreeing set. First-come alone is not a sample but an accident, and here the
accident has a direction: round 12's first disagreeing path was a `.md`, so a
report keyed on it would have shown the end-of-line-shaped case and never the
one that kills it. The artifact row is the discriminating case precisely because
git classifies its blob binary and converts a binary blob under no
configuration. Where the manifest records no artifact key the second sample is
absent and the report says so rather than printing a blank, and so are the other
two ways it can fail to resolve: a manifest recording an artifact no config seam
names a path for, and an artifact row that is not in the disagreeing set. No arm
of this smoke now produces the first of those three — an install that ran placed
a verified artifact — so it is kept as the report's own fail-soft rather than as
a case with a witness, which is the honest standing for a branch whose subject
the relocation removed.

The third is the **witness for the exit-2 verdict** — the first disagreeing
entry whose `want` or `got` failed the operand shape test, carried out of the
loop as the same `<path><TAB><want><TAB><got>` tuple every disagreement is
recorded as. It is appended last and deduplicated **on the whole tuple**, so a
run cannot exit 2 on an entry whose *bytes* this report did not print. The
artifact row above it dedups on the same predicate for the same reason: a path
is what a reader looks a row up by, and bytes are what the verdict was computed
on, so a path match is no evidence that the two rows are one row.

Whether the witness coincides with a sample already chosen is therefore a
**check this report performs and prints**, never a claim it makes, and it has
three outcomes:

- **byte-equal** — the witness row is one of the samples already chosen. The
  report says so, in the shape the artifact row already uses, because the
  reader's question is the same one ("is this the row the verdict is about?")
  and a silent collapse answers it wrongly. Under the tuple predicate that
  sentence is now earned rather than asserted.
- **path-equal, bytes differ** — both blocks are printed, the witness block
  labelled as the row the verdict below is computed from, and the report states
  plainly that two decompositions of one recorded entry disagree. That is a
  statement about **this harness** and not about the consumer's tree.
- **absent** — the witness row is not among the samples, and joins them.

Under the former path-only predicate the middle outcome was indistinguishable
from the first, and it is the outcome the attested round-18 run silently took.
The header states **how many** disagreements failed
that test, and that count is what says whether the one printed witness row is
representative: one malformed entry out of hundreds is a statement about that
path, all of them a statement about the capture step every entry runs through,
and those two readings send a reader to different places.

> **A run-wide verdict states which entry earned it, or it is not readable.**
> Where an arm reports a bounded sample and decides on an unbounded scan, the
> deciding entry joins the sample. A flag recording only *that* something tripped
> it hands the reader a verdict and withholds its subject.

The witness is the first such entry rather than every one, on the sampler's own
standing ground: the report is a bounded sample by design, and a report that
prints hundreds of rows is a report nobody reads. Keeping the other two members
is deliberate — they answer different questions from the verdict's row, and the
artifact row in particular is the discriminating case the binary-conversion
argument above rests on. A sampler narrowed to the verdict's row alone would buy
the verdict's legibility with the discriminating case.

**Six values per sampled path, and the truth table that reads them.** Each is
labelled, printed plain, rendered twice — by `printf '%q'` and as an **octet
dump** beside its own length — and judged by the decomposed shape test, with the
call that produced it and that call's standard error. **Two of the six are the
operands the failing comparison used, printed out of the variables it read; the
other four are re-reads**, and the split is the block's whole architecture rather
than a presentational choice — a value that reaches the comparison mangled and
re-reads clean is invisible to a report that asks again, and that is not a
hypothetical: the first form of this report re-read `want`, and saw nothing on a
leg where every entry disagreed.

- **`want`** — **held.** `files[P]` as the arm's own loop holds it: the loop
  reads the manifest line whole with `IFS= read -r line` and takes `want` off it
  by parameter expansion, so this is what that split produced and one of the two
  values the failing comparison actually used. It is deliberately **not**
  re-read through a second channel.
- **`got`** — **held.** The value the arm's own
  `got="$(git hash-object -- "$C/P")"` assigned, carried out of the loop on the
  failure branch: the comparison's other operand, not a description of one. It
  is the second half of the correction `want` already carries, and the reason it
  is spelled out here is that applying that correction to one operand of two is
  precisely how the remaining asymmetry stayed invisible.
- **`reread`** — a re-read. `git hash-object -- "$C/P"` run again at report time
  from the smoke's own current directory. This is the value the label `got`
  printed before the held operand existed; it is renamed rather than dropped, so
  that a held value and a re-read of the same call can never be read as one
  thing — a single label covering both is how the asymmetry hid.
- **`own`** — a re-read. `git -C "$C" hash-object -- "P"`: the same command in
  the repository context the installer's own recorded-hash helper runs in, which
  is the only thing the two call sites differ by.
- **`raw`** — a re-read. `git hash-object --no-filters -- "$C/P"`: the file's
  bytes with the attribute mechanism removed. `--no-filters` ignores attributes
  entirely, so this value depends on no repository, which is what makes it the
  fixed point the other re-reads are measured against.
- **`wantalt`** — a re-read, and the only one that reads the *manifest*.
  `jq -r --arg p "P" '.files[$p]' "$LOCK"`: the same key out of the same lock,
  straight into a command substitution. `want` was the one value in the block
  with no independent producer — `got` already has three — and it arrives through
  a `jq` line render, a tab delimiter and a `read` split, any of which could
  mangle it. This control shares none of them, so `want != wantalt` places the
  defect in that pipeline and the two octet dumps name the byte, while
  `want == wantalt` with identical dumps exonerates the pipeline outright and
  leaves the matcher as the only remaining subject.

*A held value's `call` line states where the value was read from, and is
permitted to state nothing more.* It names the variable and the read that
assigned it. It may **not** assert that the value is the one the failing
comparison used and not a second read of it: that is an identity between two
decompositions of one recorded string, no call site can check it, and the round
that exposed the defect printed exactly that assertion beside a value the
verdict decomposed differently. An unverifiable assertion there is worse than
none, because the `call` line is what licenses a reader to treat the report as
adjudicating the verdict — it converts a live defect into positive evidence of
health, which is how the series read clean. The identity claim lives in the
coincidence outcome above, where it is checked. A re-read value's `call` line is
untouched by this rule: it prints the command that produced the value, which a
reader can re-run, and it asserts nothing beyond that.

**The manifest line is read whole and split in the shell, which is what makes
the stream itself observable.** The loop reads with `IFS= read -r line` and
derives `path` and `want` by parameter expansion, so the bytes the channel
delivered exist in a variable before anything splits them. Reading with
`IFS=$'\t' read -r path want` makes the split the same operation that consumes
the evidence: there is then no point at which the delivered line is a value the
report could print, and eighteen rounds read the *value* side because nothing
ever held the *stream* side.

*The split is not merely equivalent to the read it replaces — it is less
normalizing, which is the second reason to take it.* Tab is an **IFS
whitespace** character, so `IFS=$'\t' read` collapses runs of tabs and strips
trailing ones. Probed rather than reasoned about: on the line `a\t\tb\t` the
read yields `path=a want=b` while the expansion yields `path=a want=$'\tb\t'`.
For the two-field, single-tab line `jq -r '"\(.key)\t\(.value)"'` actually emits
the two agree byte for byte — including on a CR-terminated line, where neither
strips the CR, and on a path carrying a space, which `IFS=$'\t'` leaves alone.
Where they differ is on an **anomalous** line, and there the read normalizes the
anomaly away before anything can observe it. That is the anti-normalization rule
below, reached at the read.

**Two raw stream lines, printed once per report rather than once per sampled
path.** The **first line of the stream**, whatever it is, and the **raw line of
the first disagreeing entry**. Both, because they answer different questions:
the first says whether the channel delivers CRLF at all and is unconditional on
any comparison, which makes it the cheapest possible statement about the stream;
the second says whether *this* entry's bytes are what the verdict claims, which
is the question the verdict is actually about. Carrying only one of the two
buys half the witness for the same round. They are facts about the run and not
about a path, so they sit outside the per-path block for the reason the worktree
and config witnesses do, and they ride as **named operands** rather than being
re-read, for the reason a held value exists at all: a value re-read is a second
observation and cannot testify about the first. Their `tests` line is not a
finding — a raw line is a path, a tab and a hash, so it fails the hash shape test
by construction — and the `octets` line, produced through no construct that
could normalize, is what they are printed for.

Two values cannot separate *one side filtered* from *the bytes changed*, and no
number of re-reads alone can separate either from *the comparison was handed
something else*; six can, and the reading rule lives here rather than with
whoever reads the log next. The first three rows read the **hashing**, the next
two read the **comparison**, the sixth is the catch-all on any value's shape,
and the last three read the **instrument** rather than the operand:

| observation | reading |
| --- | --- |
| `want == own == raw` and `reread != raw` | the read side's context applies a filter the write side's does not; the defect is at `run-smoke.sh`'s call site and the two-call-site narrowing is confirmed |
| `want == own == reread` | the hashes agree and the arm could not have failed on this path — the disagreement is in the comparison, not in the hashing (whether they also equal `raw` says only *why* they agree: equal, no context filters at all; unequal, both contexts filter identically — neither changes the reading) |
| `own == reread == raw` and `want != raw` | the bytes on disk are not the bytes `init` hashed; the porcelain below and the artifact control say which |
| `got != reread` | the value the comparison used is not the value the same call yields now, so the mangling happens at capture time inside the loop; the two octet dumps name the byte and its position |
| `want == got`, byte-equal, both held | the comparison received two equal values and reported them unequal, which bash cannot do — so the pairing is wrong and `bad_hash` associated one entry's `want` with another entry's `got`; read the sampled path against the loop's own echo order |
| any of the six fails the shape test | a stray byte, a truncation or a refusal will each do it and the value is then not a hash; `len` names a truncation outright, the octet dump shows a stray byte, and the captured standard error names the refusal. Read it against the row below first, which is the one case where that reading does not hold. The arm acts on this row too, and that is the one row it acts on: a refused operand makes the run **exit 2** rather than 1. Read it with the header's count: **one** entry failing the shape test is a statement about the sampled path, **all** of them a statement about the capture step every entry runs through |
| `shape` fails while `len40` and `class` are both clean | two matchers in one shell disagree about one variable — the ERE engine or its locale is the subject, not the value; the operand is a hash and the harness refused it anyway |
| `%q` renders bare while `octets` shows a byte outside `0-9a-f` | `printf '%q'` is not a byte rendering on this host, so **every** prior round's "bare rendering" reading is weakened to what quoting alone establishes; re-read rounds 12 to 17 against the octet dump before citing them |
| `want != wantalt` | the manifest pipeline mangles between the lock and the comparison — the `jq` line render, the tab, or `read`'s splitting; the two octet dumps name the byte and the position |
| the first stream line's `octets` end `0d 0a`, or `0d` once the newline is stripped | the channel delivers CRLF into the manifest stream at all, before any entry is compared. This is the one row that reads the **stream** rather than a value, and the decision rule below names what each dump selects |
| the disagreeing entry's raw line against that same entry's `want` | separates *the carriage return arrived in the stream* from *it was introduced at the split*: a `0d` in both puts it in the stream, a `0d` in `want` alone puts it at or after the split, and neither carrying one puts the fault in the verdict's decomposition rather than in the value |
| the coincidence outcome reads **path-equal, bytes differ** | the report's witness row and an earlier sample carry one path and two different tuples, so one recorded entry has two decompositions and they disagree. The subject is **this harness**, not the consumer, and the two blocks printed under that path are the evidence. What it suspends is bounded, because the disagreeing blocks are exactly the ones carried in through the trailing `${bad_hash[@]+…}` operand: every row read off a **sample block's** `want` or `got` is suspended for that run, while `stream1`, `badline`, the witness row and the header's counts arrive as scalar operands and `reread`, `own`, `raw` and `wantalt` are commands the report re-runs — none of those is implicated, and a sample whose probes returned a hash at all proves its own path field crossed intact. A repair chosen off the unimplicated operands is sound; one chosen off a sample block's tuple is not |

The instrument rows exist because a report rendered through one subsystem cannot
adjudicate a disagreement between two, and every row above them reads the
operand — which is why seventeen rounds of value-side reading could not close
this leg's series.

**Two renderings, and only one of them can carry a byte claim.** `printf '%q'`
is a **shell-quoting** renderer: it emits a string bash can re-read to the same
value. For the class that motivated it — a value carrying a trailing carriage
return, which compares unequal and *prints* equal — quoting and byte-exactness
coincide, `$'…\r'`, and that is still exactly why the rendering is neither
decoration nor optional. They are not the same property. A rendering that quotes
nothing establishes that bash's formatter found nothing worth quoting, and
**not** that the value holds forty bytes drawn from `0-9a-f`. The `octets` line
is the one that carries the byte claim: the value's bytes in hexadecimal,
produced by piping it through `od` with no interpretation and through no
construct that could itself normalize — no `echo`, whose escape handling is
shell-dependent, and no re-quoting. Both are printed, because a disagreement
between them is itself a finding and because every round from 12 through 17 is
recorded in `%q`'s terms, so dropping it would make that record unreproducible
against a later run.

**The shape test's parts print beside the value they judged, and the class half
deliberately uses a different matcher.** The `tests` line carries `len40` —
whether `${#v}` is exactly 40 — `class`, and `shape`. `class` is computed by
`leftover="${v//[0-9a-f]/}"`, the residue after deleting every acceptable
character, which is exactly the offending set; where it is non-empty the line
names that residue and the **zero-based index of the first offending character**,
so the reader gets a position and not only a set. `shape` is the composite
`[[ "$v" =~ ^[0-9a-f]{40}$ ]]` verdict itself. The class test is a parameter
expansion over a **glob bracket expression** while the composite is an **ERE**
evaluated by the platform's `regcomp`/`regexec` — two independent
implementations of "is this character acceptable" — and that is the whole
mechanism: using `=~` for the decomposition would inherit whatever the composite
suffers and print an agreement that means nothing.

*The decomposition is an observation and never a verdict, and that refusal is
load-bearing.* The composite `=~` stays the thing that selects the exit code.
Moving the decider onto the decomposed test would green this leg on a
**hypothesis** — that the ERE engine is the faulty subsystem — before any round
has shown which subsystem is faulty, and it would destroy the disagreement that
is the entire evidence. It is the same trap the anti-normalization rule below
closes, reached from a new direction: normalizing the operand and swapping in a
matcher that happens to accept it are one move under two spellings.

**A refused operand is a precondition of this harness, so it exits 2 — never
the manifest verdict at 1.** `starter: N of N manifest entries disagree with the
tree` at exit 1 is a statement *about the consumer*: the tree `init` wrote no
longer matches what `init` recorded. If instead a value reached the comparison
malformed, nothing about the consumer has been established and the honest code
is the harness-precondition one — the code every preflight refusal in this
script already takes, from a missing tool to a crate that will not compile,
reached one step inward to the arm's own operands. **The 2 rests on a second
ground as well, and it has to be said rather than left to the first.** A matcher
that refuses a well-formed hash is a harness precondition every bit as much as a
mangled operand is, and it is likewise no finding about the consumer's tree — so
the code is the same 2 in both cases, and the refusal below is what tells the two
apart.
The arm therefore shape-tests both operands on the failure branch, and after the
report has printed it routes a refused one through `blocked` at exit 2 instead
of `fail` at exit 1. **It fires after the report, not at the first bad value**,
because diagnosing that value is exactly what the report exists for; refusing
early would trade the report for the verdict and lose the more valuable half.
That ordering argument assumed all along that the report the refusal points at
contains the refusal's subject, and the witness sample above is what makes it
true rather than hoped for. The refusal itself names that subject: the path, which
of `want` and `got` the shape test refused (or both), how many disagreements it
refused, and the **decomposed verdict for each refused operand** — so the verdict
is readable from the last line alone, by a reader who never scrolls up.
*It reports the test's verdict, and never declares the operand malformed*, because
those are different claims and the second can be false while the first is true:
where the decomposition reads `len40=yes class=clean`, the operand **is** a hash
and what happened is that this harness's matcher refused one. The last line
carries the decomposition precisely so its reader can tell those apart instead of
taking away a statement about the value that no one observed.
This does not make the shape row above redundant: that row tells a **reader**
what a refusal means, and this makes the **arm** act on it. A report a
human reads and a verdict a suite reads are two consumers, and deleting either
for the other is the mistake.

*Who actually reads the 2, and the honest limit that has to be stated with it.*
The verdict class is read by CI's job verdict and by a human reading the log — and
by **nothing else**. `scripts/evidence-config.sh` gives the `installer_smoke`
suite the `parse-smoke-log` parser, and that custom-parser path never references
the process status it is handed: the evidence row is derived from the log's arm
headers alone, so **exit 1 and exit 2 are indistinguishable to it**. The
distinction is real and it is invisible to `--run-validate`'s evidence. Nothing
above changes that or depends on it; it is stated here because the natural
reading of a refusal this specific is that a suite consumes its class, and that
reading is false.

**What the arm must not do is normalize.** Trimming the operands to their
40-hex core before comparing would turn a mangled-operand red green, and it is
the one repair this arm is closed to: it would make the instrument accept a
broken producer silently — enforcement inverted, the defect absorbed by the
detector — and it would make a subsequent green unreadable as evidence, because
such a green fails to distinguish *the tree matches* from *the tree matches once
the arm discards whatever reached it*. The refusal above is the honest form of the
same observation: an operand that is not a hash stops the run instead of being
repaired into one.

*The same rule reaches the report's own **decompositions**, and not only a
value's two renderings.* Where two independent readings of one recorded string
disagree, the disagreement is printed and named — never resolved in favour of
either. The obvious repair for the round-18 divergence below was one
decomposition helper that both the report and the verdict call, and it is
**refused** on the ground the shape test's own two matchers already stand on:
two independent implementations reading one variable are what make their
disagreement observable instead of absorbed. Unifying the two splits would
delete the only signal that anything is wrong while changing nothing about the
value that reaches the comparison — normalizing an operand and unifying the
readings that disagree about it are one move under two spellings.

*And it reaches the **read**, which is where it was needed longest without being
stated.* A read that collapses tab runs and strips trailing tabs deletes a class
of anomaly before any probe can see it, so the arm would be normalizing its own
input while refusing to normalize its operands — the rule held on one side of
the split and not the other. The loop therefore reads the line whole and splits
it in the shell, as the manifest arm's description above states.

*The one byte the read does own is the line **terminator**, and the exception is
stated rather than assumed.* A terminator is a property of the channel a record
travels over and not of the record, so a line delivered CRLF carries a byte that
was never in the manifest — and the values on that line are hashes `init` took
from `git`, which cannot end in one. The loop therefore drops **one** trailing
carriage return before it splits, keeps the raw line unstripped for the two
evidence operands the report prints, and **declares the count** it dropped, on
the green path as well as the red. That declaration is what separates this from
the repair the rule closes the arm to: trimming an operand to its 40-hex core
deletes the evidence and reports nothing, while a per-line terminator strip that
prints how many of how many lines carried one leaves a value that genuinely
ended in a carriage return visible as a count that does not fit the channel's
shape. Exactly one is taken, so a doubled carriage return still reaches the
shape test and still fails it.

Once per failing profile, and outside the per-path block because each is a fact
about the run rather than about a path, the report also prints the consumer's
`git status --porcelain` and `git log -1 --stat`, truncated — the direct witness
for the third row, since a worktree that has diverged from what `init` committed
is the one shape that explains a disagreement set containing a binary;
`core.autocrlf`, `core.eol` and `core.safecrlf` **with their origins** in both
repositories — the mechanism behind the first row, where a value alone would not
say which file set it; and `git check-attr -a` for each sampled path in both
repositories, the other half of that mechanism, since an attribute reaches a
path the config does not. Nothing is printed for completeness: every value has a
reader in the table above or in the sentence that introduces it.

*The smoke-repository half of that attribute lookup answers only when the
consumer lives inside the smoke's own repository — and it never does, because
the consumer is created under `INSTALLER_SMOKE_TMP_DIR`.* git refuses the lookup
outright for a path outside the repository it is asked, so the report prints that
refusal labelled as a refusal rather than as an attribute report, and the
refusal is itself the reading: the path's attribute chain is reachable from the
consumer alone.

**The artifact digest is a filter-free control on the content question.** When
the manifest records an `artifact` key, the report recomputes the artifact's
SHA-256 from the tree — with `sha256sum`, already one of its own preflight
tools — and prints it beside the recorded `artifact.digest`. It costs
nothing new and settles the content question outright, because SHA-256 over the
file is taken by no git filter and in no repository context: equal digests mean
the artifact's bytes are exactly the bytes `init` published, so a `files`-row
disagreement on that same path is not a content change, and it cannot be an
end-of-line conversion either, because git converts no blob it classifies
binary. Both surviving content hypotheses die on one line. It is a **control**,
so a matching digest is not free to be read as an assertion about any other
path: it speaks for the artifact alone.

*The honest limit, stated rather than gated.* The report is failure-path code
and the only host that executes it is the host that fails, so no green run
exercises it and no fixture pair can. What holds it is shape rather than
coverage: it is a straight-line sequence of prints with no branch that can
change the arm's verdict, and the `fail` that follows is the one that stood
before it. A report that cannot alter the verdict is one a stale line can only
make less useful, never wrong-acting. It carries no knob for the same class of
reason: a report enabled by a configuration no CI job sets is the dead-producer
shape, and the one host that needs this one is the host nobody is standing at.

**What the native Windows leg has measured so far.** That leg is
`continue-on-error` and reports rather than judges; these are its findings,
recorded here because they are what the next rider of the leg would otherwise
re-buy. Rounds 12 (run `33782234328`, head `32f73806`), 13 (run `33963571906`,
head `c4850072`, the first round the report above ran on) and 14 (run
`34002192468`, head `a5b6907b`) and 15 (run `34054512420`, head `bf1fc722`, the
first round with **both** operands held) all fail the same arm on the same
profile, 12 and 13 at `starter: 477 of 477 manifest entries disagree with the
tree`, 14 and 15 at `476 of 476` — the count moved with the payload, not with
the defect. *In rounds 12 to 14, the value labelled `got` in the log is the
value this section now calls `reread`: those rounds ran before the held operand
existed.*

- Every entry reads `manifest hash disagrees with the tree` and none reads
  `manifest names a file that is not there` — those are the arm's two branches,
  so the failure is genuinely a hash disagreement rather than a missing path.
  It also means every recorded path exists, since the existence test is what the
  other branch reports. In round 14 the disagreement lines run with **nothing
  interleaved**, so `git hash-object` emitted no standard error across any of
  the calls behind them.
- **Rounds 13, 14 and 15 printed identical values on both samples** — the `.md`
  and `scripts/checkwright-gates.exe`, which is in the disagreeing set every
  round and is what all three sampled as the artifact row — every value a clean
  40-hex that `%q` rendered unquoted.
  `reread == own` retires the process-context asymmetry that was this leg's
  standing narrowing; `raw` equal to the rest retires every end-of-line
  hypothesis about the file's content; and the artifact control read
  `recorded` and `recomputed` equal, so the binary's bytes are exactly the bytes
  `init` published. Round 14 adds `want` **as the loop held it** to that set and
  round 15 adds `got`, so in round 15 all five values on both samples are one
  string — `affdbceb…d982` on `gate-sdk/README.md`, `4159af89…f942e` on the
  `.exe` — and `%q` renders every one of the ten unquoted. *Read that last clause
  under the bound the two-renderings paragraph above now states: it establishes
  what quoting alone can, and no round before 18 held an octet dump to establish
  more.*
- **The consumer's `git status --porcelain` printed nothing**, so the tree holds
  what `init` committed, and no round carries a `fatal` line from any hash
  call, so no per-path refusal is open. Round 14's only `fatal` lines are the
  two the attribute lookup is *expected* to produce, git refusing a path outside
  the repository it was asked.
- Every round read `core.autocrlf` as `true` in the system gitconfig and `false`
  in the user's — the same two origins in the consumer and in the checkout, so
  the effective value is `false` on both sides — with `core.eol` and
  `core.safecrlf` unset and no attribute reported for either sampled path.
- The other host facts: `core.symlinks true`, `core.longpaths` unset,
  `core.filemode false`, git 2.55.0.windows.5, bash 5.3.15, and the failing
  profile is `starter` — the profile that vendors one kit. It is also the
  **first** profile the loop drives, so no profile has passed this arm on that
  host ahead of it. The consumer's own battery passed, `All 11 gates passed`,
  immediately before the manifest arm failed.

*What round 15 settled, and the one thing it opened.* Both operands are now
printed as the comparison received them, and they are **byte-equal**. That is
the table's fifth row, reached on evidence rather than by elimination: two equal
values were compared and reported unequal, which bash cannot do, so the pairing
`bad_hash` recorded is wrong rather than the hashing. Rows one, two and three
are all retired by the same observation — every value including `raw` agrees, so
no context filters and no byte moved — and the carriage-return candidate rounds
12 to 14 left standing is **falsified**, because `%q` would have quoted one and
did not. The diagnostic series is closed: the defect is in the comparison's
operand pairing, and no further round narrows it.

*The verdict and the report named different entries, and that is round 15's own
finding.* The run exited **2**, not 1 — `a manifest comparison operand is not 40
lowercase hex` — while both sampled entries' operands are well-formed 40-hex.
Both statements are true because they are about different rows: `malformed` is a
**run-wide flag** any one of the 476 disagreements can set, and the report
samples the *first* disagreeing path plus the artifact row. So the entry that
earned the exit-2 verdict was, by construction, one the report could not print.
That is not a hypothesis about this host — it followed from the arm's own shape,
and it meant the honest-code rule above bought a verdict at the cost of the value
that justifies it. **That finding is drained** into the witness sample and the
operand-naming refusal above: the arm now carries the deciding entry into the
report and names it in the refusal, so a round after this repair reads its own
verdict's row. Nothing else about rounds 12 to 15 moves — what they measured is
what is recorded here.

*Round 17 is the first round under that repair, and it REOPENS the series rather
than confirming the pairing.* Run `34108112152`, job `101697744618`, read by
close at the push it already watches. The repair delivered exactly what it was
built for: the verdict's own row is printed, the refusal names it, and the report
says so in as many words — *the witness row `gate-sdk/README.md` is one of the
samples already chosen, so the verdict row and that sample coincide*. What that
row then shows is the reopening. **All five values on it are the same
40-lowercase-hex string, and `printf '%q'` quotes none of them.** Yet the same
run refuses at the harness precondition, *the `want` operand on
`gate-sdk/README.md` is not 40 lowercase hex*, and counts **493 of 493**
disagreeing entries failing that test. Every entry in the profile, not a subset.

That is a stronger and different fact than round 15's. Round 15 saw one equality
test contradict its operands and read it as a pairing defect. Round 17 has the
**shape** test contradicting the same variable's own `%q` rendering, on the very
entry the verdict is about, for the whole manifest at once. A wrong pairing
cannot produce it: `[[ "$want" =~ ^[0-9a-f]{40}$ ]]` reads one variable, so there
is no second operand to mis-pair, and the value it read is the value printed
beside it. The next round's question is therefore about that one variable — what
`read -r want` put in it, and what each subsystem that reads it then makes of it
— and not about which entry got compared with which.

*The consequence round 17 has for every round before it.* The observation is not
about any operand — it is a disagreement between **two subsystems reading one
variable**, bash's `printf` on one side and the platform's `regcomp`/`regexec`
behind `[[ =~ ]]` on the other. That is what a report rendering through only one
of them cannot adjudicate, and it is why seventeen rounds of value-side reading
could not close the series. It also weakens the record above rather than only
extending it: rounds 12 through 17 read a bare `%q` rendering as evidence that no
byte was out of place, and `%q` is a shell-quoting renderer, so what those
readings establish is that bash's formatter found nothing worth quoting. The
carriage-return candidate stays falsified — quoting and byte-exactness do
coincide for exactly that class — but *no round before 18 carries an octet dump*,
so no round before 18 has established what bytes any of those values held.

*What round 18 must show to be terminal, stated before it is read so the reading
cannot be shaped to fit.* The block now prints, per value, an octet dump beside
the `%q` rendering, `len40` and `class` beside the composite `shape` verdict, and
`wantalt` beside `want`. Three partitions follow, and between them they cover
every hypothesis round 17 leaves open: an octet outside `0-9a-f` names a mangled
value and the round is about the pipeline or the producer; `len40` and `class`
clean with `shape` failing names the matcher, and the subject is the ERE engine
or its locale; and `want` against `wantalt` separates a mangling introduced by
the manifest pipeline from one introduced anywhere else. Which of them the round
actually shows is not predicted here, and nothing above claims the leg passes.

*Who reads the next round, and the trap that would otherwise swallow it.*
`install-smoke-windows` is a job of the `gates` workflow, which runs on every
push to master, so the reading has an owner inside the ordinary stage set and
costs no push of its own: **close**, at the push it already makes and watches.
The reading is an explicit, **job-keyed** act — read that job's log, never the
workflow's conclusion. The job is `continue-on-error: true`, so the two verdicts
come apart, and that is measured rather than predicted: runs `34002192468` and
`34054512420` both concluded `success` while `install-smoke-windows` concluded
`failure`. A session that watched the run to green and inferred the leg from
that verdict would read a passing workflow as a passing leg. What gets recorded
here afterwards is what was observed, by whoever observed it — this section
predicts no content for it.

**Rounds 18 through 20 are compressed to what still binds — observed 2026-09-07
and 2026-09-08, runs `34142337941`, `34212264301` and `34245261556`; the
derivations are in git history and this block is the whole of what survives
them.**

*The established fact.* The carriage return is on the manifest line as the loop
reads it, **before any split**: `stream1` and `badline` — the first line of the
stream and the raw line of the first disagreeing entry — end `… 63 0d`, and
`want` decomposes as `len40=no class=dirty[residue=$'\r' first=40]`. The count
was **493 of 493**, and a whole-manifest count is the signature of a suffix on
the reader rather than real hash divergence: the two operands were never
compared as hashes at all. The byte was invisible to `printf '%q'` for six
rounds, which is why the octet dump exists.

*Three things were refuted, and re-running any of them is the mistake to avoid.*
**`wantalt` is not a discriminator** — it reads one value, so its capture holds
exactly one terminator and always consumes it, and it comes back clean whichever
end of the stream put the byte there. It never separated the producer from the
channel, and reading it as though it did is what aimed round 19's repair at the
channel. **The channel swap did not fix it** — replacing the loop's process
substitution with a command substitution and a here-string moved the count from
493 to 492 and no further, which prices a capture's effect on the *terminator* at
exactly one line, the line whose terminator the capture itself consumes.
**The argv round trip is exonerated** — the report's two decompositions of one
entry disagree, and the `${bad_hash[@]+"${bad_hash[@]}"}` transport was
reproduced on bash 5.3 against a CRLF stream, including with `IFS` set to a
carriage return, with both decompositions agreeing. That divergence is
unexplained, host-specific, and carried as its own queue entry rather than here;
it suspends the array-sourced sample blocks and nothing else.

*The repair that selected, and why it is at the read.* Production and channel are
both out of reach — the producer is unidentified, and the channel was swapped
once to no effect — so the repair lands where the byte is unambiguously not part
of the record. The read drops one trailing carriage return **as a terminator**
before it splits, holds the raw line unstripped for the report's two evidence
operands, and declares the count it dropped on the green path as well as the red.
The terminator exception stated with the anti-normalization rule above owns why
that is not the trimming this arm is closed to. Its coverage is **per-read and
correctly narrow**, which round 21 then measured rather than assumed.

**Round 21 read the first job to run under that repair, and the repair works —
read 2026-09-08 off run `34267324532`, job `102199861062`, head `adb7379f`.**
The strip fired on **492 of 493 lines carrying a carriage return**, and `mismatch`
came back **0**: the manifest hash arm PASSES, having reddened at 493 and then 492
*disagreeing entries* in the two prior rounds. That class is closed.

*The two 492s are different quantities, and their agreeing is the corroboration.*
Round 20 counted 492 entries disagreeing; round 21 counts 492 lines arriving with
the byte, and zero disagreeing. The same line is missing from both counts — the
last one, whose terminator the capture consumes — so the set that carried the byte
and the set that used to disagree are the same set. Nothing was assumed to make
those line up; they were counted by different code in different rounds.

*Which of the pinned outcomes this is, and the one qualification it earns.* It is
the first — the manifest arm passing with the leg reddening later. The strip
reached the operands, so the second outcome is excluded; the stream still carries
the byte at 492 lines, so the third is excluded too. The pin read that first
outcome as meaning "whatever follows is a new subject", and the qualification is
that *subject* and *cause* came apart: the leg now fails at
`run-smoke.sh`'s kits-roster assertion, on `manifest kits (gate-sdk\r) differ from
the profile roster (gate-sdk)`. A different assertion, a different reader — and
the same byte. Whoever writes the next pin states which of the two it means.

*What that second reader settles, and it is what twenty rounds could not.*
`mapfile -t lock_kits < <(jq -r '.kits[]' "$LOCK")` is a **multi-line capture
through a process substitution**: no tab split, no `read` loop, a different
builtin, reading the same lock a little further down the same function — and it
carries the carriage return.
Set beside `wantalt`, a single-value capture that reads clean, the pair
discriminates what neither did alone: **the byte is a per-line terminator in that
`jq` stream's multi-line output on this host, reaching every multi-line reader of
it and invisible to every single-value capture.** That is a statement about
coverage, and it is established. It is *not* a producer claim: nothing here
separates `jq`'s output mode from the channel, and none is asserted.

*What is owed, and it is not owed here.* Every multi-line reader of that stream
needs the same terminator handling the repaired loop got. The kits read above is
one; the installer's own copies are filed separately. That work is routed to the
gap inbox rather than taken at this close — the read was close's to buy and the
repair is not, and no host this repo can reach reproduces the byte.

*Reproduced, which is why the numbers above are not one run's accident.* Run
`34270944900`, job `102212080750`, head `7329b319` — an independent job on the next
commit — reports the same **492 of 493**, the same absent disagreement report, and
the same kits-roster failure. Two runs agreeing is what licenses reading the count
as a property of the host rather than of a round.

*What the next red round selects, pinned ahead of it.* The kits assertion passing
with the leg reddening further on means another unrepaired multi-line reader, and
the next one is found the same way. The kits assertion still reddening after a
terminator-aware read means the byte reaches somewhere this account does not
cover. The manifest arm reddening again at any count means the strip regressed,
and the count says at which reader. Nothing here predicts which, and nothing here
claims the leg passes.

*The free log read is gated on the run, not on the job, and the way past that is
a different endpoint.* `gh run view <id> --log` refuses with `run <id> is still
in progress; logs will be available when it is complete` even when the Windows
job itself has finished, so a close reading a job-keyed leg would otherwise wait
on whichever macOS sibling finishes last — `install-smoke-macos` or
`install-smoke-macos-intel`, neither of which this leg has any dependency on.
The sibling is deliberately not named singly here: the run's tail is whichever
macOS leg is slowest on the day, and a second one arriving is exactly how a
named-sibling claim goes quietly wrong.
`gh api repos/<owner>/<repo>/actions/jobs/<job-id>/logs` answers for a
finished job while its run is still going, and that is the call to make; the job
id comes from `gh run view <id> --json jobs`. Measured at round 15.

*Why the CI diagnostic that used to stand beside this leg is gone.* A
`read one manifest disagreement in place` step stood up its own consumer and
printed none of the five things it existed to print, twice, each time for a
reason the arm cannot have. It installed at `--profile full` where the smoke's
failing check is on `starter`, so it reproduced a different run; and the full
profile is what met `init-vendor-staging-argv-overflow` — `git` refusing an
over-wide argv while staging the vendored set — so it bailed at its own early
guard. The arm inherits the right profile by construction, running inside
whichever profile failed, so there is no profile to select and none to get
wrong. **Deleting that step repairs no part of the overflow and must not be read
as repairing it**: `init-vendor-staging-argv-overflow` owns one `git`
invocation's argv width, a native-Windows adopter on the full profile still
cannot install, and that entry stands untouched. What the deletion removes is
this leg's *dependence* on the full profile, not the defect.

**The payload every profile installs carries a real gate binary**, because the
value claim is a claim about the product an adopter receives. The smoke compiles
the crate for the host target, emits the digest sidecar beside it and hands the
pair to `pack-installer.sh --artifacts` before the loop starts, so each profile's
battery is the battery a covered platform actually gets. It was not always so,
and the accident is worth naming rather than quietly fixed: a main loop packing
no artifact makes *every* profile an uncovered-platform install, so a value
verdict measured there is a verdict about the omitted battery wearing the
shipped one's name. It costs no new dependency — the preflight already requires
`cargo` and `rustc`.

*A host-built artifact is a harness stand-in, and is labelled one where the rule
it stands beside is cited.* gate-sdk/SPEC.md §Consumer payload rules that the
payload carries a prebuilt binary per declared target, *built by the release and
never from a working tree*. The smoke builds from a working tree because it has
no Release to draw on. That is a liberty this harness takes, not the payload rule
relaxing: the publishing path still cannot build, and `pack-installer.sh` is
still handed a directory it did not produce.

**The value claim is asserted over the loop, not inside it**, and it is two
sentences: some profile catches the defect at all, and some profile *below* the
payload-derived maximum catches it. The second is the load-bearing one — a
defect only `full` catches is not value an adopter can choose, it is value they
have to take everything for. Neither sentence names a gate: naming one would be
a second roster to maintain beside the recipe's own derivation, and the claim is about the
battery rather than about a member of it. Nor does either name a profile, for
the same reason the lattice assertions do not — which profiles catch a prose
defect follows from the rosters, and spelling it here would be that derivation
copied out. What the arm turns from a claim into an assertion is the one the
install page makes hardest to check: an install that is green, idempotent and
reversible is still worth nothing until it catches something.

That is the **coverage** claim, and it speaks only for a covered platform. What
an adopter on an uncovered one gets is a different claim — a **refusal** — because
with every step of an install behind the invoke there is nothing left for an
artifact-less host to proceed into. It has its own leg, since folding it into the
coverage verdict is what let a cohort of ports pass per member while emptying a
value class for every such install.

**The artifact-less refusal leg** drives a payload the packer itself produced
with no artifact directory, and that is what no other leg reaches: the artifact
arm's two refusals are driven against a payload the smoke mutated by hand, so
without this leg nothing asserts that the *publishing path's* own artifact-free
output refuses rather than proceeding. It asserts a refusal that **wrote
nothing** — the tampered leg's shape, and the assertion that tells a refusal apart
from a warn-then-install — naming the platform and carrying the remedy line that
tells this outcome from the broken-payload one. It then asserts the **same**
refusal for `doctor`, for `diff` and for a bare invocation, because the refusal is
the bootstrap's and precedes every verb: a leg asserting only `init` would pass on
a bootstrap that had grown a per-verb branch, which is exactly what the branchless
argv rule forbids. It names a profile where the value claim above names none, and
the difference is real — this is a scoping choice about which invocation to make,
not a derivation of which profiles catch what. That the refusal is reached before
the profile is ever read is itself part of what the leg says.

It also asserts the profile lattice
against the installed payload, in four parts: every named kit resolves in the
payload; the derived order has **exactly one minimum and exactly one maximum**,
so the lattice is bounded; that maximum is the payload-derived profile; and
**gate rosters are monotone** — for every comparable pair, the smaller profile's
gate set is contained in the larger's. The fourth is the one that earns
the recipe derivation's profile argument: what you experience is the battery, not the
directory list, so "moving up only ever adds" is a claim about gates, and
kit-set containment stops implying it the moment a roster varies by profile.
Nothing here counts profiles; a fourth is admitted exactly when it fits.

**The reversal arm** then runs on that same consumer, so every profile is
installed *and* reversed. In order: `diff` must exit 0 and report the freshly
installed tree clean; `uninstall --dry-run` must name a non-zero removal count
while leaving the tree object and `git status` exactly as they were; then
`uninstall`, after which the consumer's tree object must equal the one it had
**before `init` ran**, the worktree must be clean, and no `checkwright.lock` may
remain. The `--dry-run` step is where the writes/does-not-write rule in §The
verbs is asserted for `uninstall`, and asserted behaviorally: a flag that parsed
and then wrote anyway fails it, where a flag that merely existed would pass. It
is the only verb this arm holds to that rule — §The verbs states the bound.

**The tree-object equality is the load-bearing assertion, and it proves more
than `uninstall`.** Nothing else here asserts that the manifest covers
*everything* `init` wrote — the per-profile check runs the other direction,
every recorded entry against the tree. A file `init` wrote and failed to record
survives the removal and breaks this equality, so the arm closes that hole as a
side effect — for a *first* `init` here, and for an upgrade hop's own write set
in the cross-version reversal arm below. It is also the assertion form of the claim the install page makes:
an install an evaluator can reverse.

The equality holds for a reason rather than by luck. The surfaces `init` seeds
and then leaves alone — the queue file, the agent file, the evidence manifests,
the workflow state — are recorded at `init`'s own hashes, and this consumer
never edits them, so the ordinary hash rule removes them. That same rule keeps
them on a tree where an adopter *has* grown them. No special case either way,
which is why the arm needs none.

A second arm drives the **download transport**: it verifies the packed tarball
against a digest the smoke computes, extracts it with `tar` rather than npm,
and runs the same `init`, the same post-conditions and the same reversal with
`npm` and `node` **masked off `PATH`** — so `diff` and `uninstall` are proved
Node-free by the arm that already exists, at no extra pack cost. It runs against `full` alone rather than per profile —
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

**The toolchain-free arm** points that same mask at `cargo` and `rustc`, and
drives `doctor` and then a full `init` on a consumer that has neither. It
asserts that `doctor` exits 0 and reports clean, that it names neither masked
tool in its report — a contributor-audience member is omitted from the adopter's
verdict, not shown as informational (§doctor) — and then, through the same
post-conditions every other arm runs, that `init` succeeds and the vendored
battery is green. `doctor` is asserted before `init` as well as inside it,
because `init` reads only its exit status: the standalone verdict is what an
adopter meets first and what every later refusal is downstream of.

That arm exists because nothing else here could see the defect it holds out.
The smoke's own preflight *requires* `cargo` and `rustc`, legitimately — it
builds the binary the payload carries — so every arm above drives `doctor` and
`init` on a machine that has them. **Masking is per-arm**, which is exactly what
lets this arm exist without weakening that preflight: the host requirement is
unchanged and the build step still gets the real tools. What the arm asserts is
sharpened by that payload rather than compromised by it — it installs a
*prebuilt* artifact with no toolchain on `PATH`, which is the claim the port
directive rests on: an adopter takes the pre-compiled path, and nothing on it
compiles. The mechanism is the
existing Node-free mask rather than a new facility, and it is reused rather than
replaced by a knob for a reason: `INSTALLER_SMOKE_TMP_DIR` stays the smoke's only
knob, a knob that suppressed a roster member would be a second, test-only
audience axis whose production behavior no adopter ever exercises, and a masked
`PATH` is what a machine with no Rust toolchain actually is.

**The `jq`-less arm** asserts what a machine without `jq` is told, and the claim
it makes is that the verbs **do not need `jq` at all**. Nothing behind the invoke
reads JSON with it — the crate reads it with `serde_json` — so the arm's oracle is
a verb that runs clean where a verb shelling out to `jq` fails: on this `PATH` the
two are distinguishable and nowhere else are they. That is what the arm is for,
and it is why the arm outlives the `jq` preflight it was built around.

So the arm splits its verbs by what each one actually meets. `diff` and
`uninstall --dry-run` run no `doctor` precondition, so on this machine they both
read the manifest and reach their answer: each must **exit 0** and say nothing
about `jq`, which is the arm's positive evidence that the read itself is
`jq`-free. `init` is the one verb that still refuses, and not for a reason of its
own — `jq` is a consumer-audience member of the toolchain floor, and `init`'s
last precondition is `doctor`'s verdict. So it must exit 1, name **the toolchain
floor** as the reason, render the report that says which member is missing, and
leave no manifest behind: the refusal an adopter meets here is the floor's,
delivered before anything is written, rather than a JSON reader's. That is
asserted twice, on a tree with and without a manifest, because a verb that had
grown a `jq`-shaped manifest read would answer differently in the two cases.
`doctor` itself is asserted directly as well, and the difference is what each
shows: `init`'s refusal proves the floor is a precondition, and `doctor`'s own run
proves it **reaches its whole report** rather than refusing somewhere ahead of it.
Exit 1 is the correct verdict there and asserting exit 0 would have been asserting
the opposite of the contract.

The gap that arm closes was total, and it is why this defect could be filed
twice: the smoke's preflight requires `jq` and this harness reads every manifest
assertion above with it, so nothing in this tree had ever exercised a `jq`-less
install. **Masking is per-arm** here for a reason the other two masks do not
have — a mask on the harness's own `PATH` would disarm the assertions rather
than the installer, so the mask rides the verb's `PATH` alone and the preflight
is unchanged.

**This mask is by absence, and that is a different instrument from the other
two** — the distinction is which question is being asked, not a matter of taste,
and it is written down because the failing-shim idiom is what a later reader will
reach for. The Node-free and toolchain-free arms ask whether the payload ever
*reaches* a program, so a shim that fails loudly and names itself is exactly the
right tool. This arm asks what a machine *without* `jq` is told, and a shim is a
`jq` that is present: `command -v jq` — the precondition's own predicate —
resolves it, the precondition never fires, and the verbs run straight into the
misdiagnosis the arm exists to catch. So the arm builds a directory of links to
every program on `PATH` except `jq` and runs the verbs against that. It is
derived from the live `PATH` rather than from a maintained list of the programs
these verbs happen to use, so it cannot fall out of date the way such a list
would, and the mask is proved in **both** directions: `jq` must resolve to
nothing, and a control program must still resolve, since a farm that failed to
populate would fail every verb for a reason that has nothing to do with `jq` and
pass this arm on the wrong refusal.

**The upgrade arm** drives a cross-version run, because every arm above installs
at one version and re-runs at that same one. It packs a second tarball a patch
version higher, installs `starter` from the first, has the adopter edit and
commit two vendored files, then runs the second package's `init` with no flags at
all. What only that reaches: the manifest's version comparison falling *through*
in the upgrade direction rather than refusing, the profile re-read from the
manifest when none is passed, and the ownership claim re-applying the payload
around a file that has changed since `init` wrote it — left alone, reported, and still the
adopter's afterwards. The roster is asserted directly rather than only through
that effect, because two different manifests leave the same intact file on this
hop and neither survives the next: an entry dropped altogether reads as *never
installed* on the following run, and an entry recorded at the adopter's own hash
reads as *unchanged*. Both would let the next `init` claim the path, so both are
named apart.

The second edited file is the **relinquish subject**, and it is what makes the arm
reach §The manifest's exit rule at all. The pack step assembles every version from
one worktree, so two hops would otherwise carry byte-identical payloads and no
path would ever leave a kit's shipped set — the arm therefore deletes that path
from the *extracted package's own* `payload/` before the hop runs, rather than
through a flag on the publishing path, which leaves the publisher no way to ship a
payload with a hole in it. The subject is chosen against a criterion rather than
by taste: a `starter`-kit payload file `init` records in `files` that no `init`
step and neither generated projection reads, so dropping it exercises the roster's
exit condition and nothing else. On this hop the file must be untouched on disk
and must **still be on the roster at the hash `init` wrote there** — disowning it
here is exactly what would arm the next release to write through the adopter's
edits.

It then chains a **third** version onto the same consumer with no fresh edit,
because one hop only shows the protection starting. The second hop is where it
either persists or inverts, and nothing above reaches it: the already-edited,
already-reported file must still be the adopter's and must still be reported, and
the manifest the first hop wrote must still carry it. It is also the hop whose
payload **re-adds** the relinquished path, which is where the ownership rule pays:
that path must meet the carried claim, be refused, be reported, and keep `init`'s
own hash on the roster. Without the exit rule this hop overwrites it silently, so
the defect is reproduced end to end rather than argued about. Each version is
derived from the one packed before it and the arm refuses to run unless the
derivation is strictly higher, so neither hop can quietly turn into a second test
of the downgrade refusal.

**The first hop's clean-worktree assertion carries a tripwire**, because the
assertion is evidence only over a hop that rewrote something. It once passed for
a reason unrelated to correctness — the arm's payload was artifact-free over the
lattice minimum, that kit set dispatched no member to the binary, and the hop
rewrote nothing beyond the manifest — which is how a defect leaving that very
worktree dirty reached the tree under a green assertion. **The tripwire's subject
moved with the relocation and the tripwire did not.** It counted the omissions
that artifact-free payload declared; every payload now carries a verified
artifact, because a payload without one refuses at the bootstrap and these hops
assert manifest behavior only a completed install reaches. So the hop asserts a
**non-zero** count of *live* registry members and a **placed artifact** in the
consumer's own manifest *before* asserting the worktree is clean — the same claim
over the class delta the relocation left standing rather than the one it emptied,
and a stronger reading, since those members are the ones that ran rather than the
ones that could not. It reads the registry rather than the run's output because
the registry is what a later run and a reviewer both read, and its failure names
the remedy — **repair the hop, never drop the assertion.** Its scope is the first
hop; the second hop's clean-worktree assertion carries no tripwire, and widening
it is a separate judgment.

**Every cross-version pack carries the artifact directory the main pack used**,
for the same reason: an artifact-free payload does not install at all, so a hop
that packed without one would assert a refusal where the arm needs a completion.
The bytes are the ones the build step already staged, so the hops differ in
version and in the relinquish this arm performs, and in nothing else.

**The cross-version reversal arm** reverses a consumer that crossed all three of
those versions, because every other reversal in the suite runs on a consumer
that has only ever met one version's payload — each of them reverses a *first*
`init` — and the upgrade arm asserts no reversal at all. It is its own scratch
consumer at the lattice minimum with **no
adopter edit**: it re-runs the upgrade arm's two already-extracted packages with
no flags, so it costs no pack, and the edited case belongs to the protection
branch below, since an adopter edit is what tree-object equality cannot host. It
then drives the same `diff`, `uninstall --dry-run` and `uninstall` the reversal
arm does, with the **latest** package's verb — what an adopter holds after an
upgrade — against a roster three versions old, and the tree object must come
back to the one the consumer had before its first `init`.

*What that reaches is narrower than a recorded hash moving, and the limit is the
harness's rather than the assertion's.* `pack-installer.sh` assembles every
version from one worktree, so no vendored path's content differs between hops
and the arm does not exercise a hash that changed under a path that stayed; the
shape changes it does cross are path additions and removals. What
it does exercise, and nothing else here does: a tree whose payload lost a path
on one hop and regained it on the next must still be **wholly removable**, and
the roster must cover an **upgrade** hop's write set — a rewritten manifest,
regenerated projections, whatever a future release adds. The arm proves its own
premise before it reverses, in the tripwire idiom above: the relinquish must
still be in effect, the manifest must record the third version, and the
relinquished path must still be on the roster. Each failure names the re-scope,
never a licence to drop the assertion.

**The seam arm** covers the two surfaces `init` rewrites on every run — a
`templates/*-config.sh` destination and gate-sdk's `msg-patterns.list` — which no
arm above reaches. That is not an oversight in those arms: the upgrade arm's
subject is an ordinary vendored file on the plain claim-and-copy path, so they
exercise a different file class and stay true without covering this one. This arm
re-runs at the **same version with no flags**, which is the whole point — the
class needs no upgrade and no `--force`, so an arm that only ran across versions
would attribute it to a path it does not live on. It is its own scratch consumer
at the **maximum** profile — the only profile whose kit set is fixed by the
payload rather than by a roster judgment, so both surfaces it edits are present
by construction rather than by a membership row that may be revised — and its own
consumer because an adopter edit inside the per-profile loop would break the
file-by-file agreement that loop exists to assert. After the adopter edits and commits both, the re-run
must leave both byte-identical, name both as changed, and still record `init`'s
hash for each rather than the adopter's. It reuses the already-installed package,
so it costs no second pack.

**The protection branch chains onto that consumer** rather than onto the
reversal arm, because an adopter edit is exactly the case tree-object equality
cannot host — the edit is what breaks the equality. This arm already has two
edited, committed vendored files, which is precisely the case that reaches
`uninstall`'s keep branch. So after its own assertions it runs `diff`, which
must exit **1** and name both files; then `uninstall`, which must keep both,
report both, remove every other recorded file, and leave a `checkwright.lock`
carrying `schema` and a `files` map of exactly those two paths **at `init`'s
hashes, not the adopter's** — the same apart-naming the upgrade arm uses and for
the same reason, since an entry dropped reads as never-installed and an entry at
the adopter's hash reads as unchanged, and either would let the next `init`
write through them.

That residual object is asserted for **shape**, not only for field presence,
because the single-writer contract in §The manifest is exactly what an accessor
cannot check: a missing key and a present-but-null key both read back as the
empty string. So the assertions run on the captured manifest text itself —
`has("artifact")` must be false, so the omitted flag left the key absent rather
than null, and re-piping the object through a recursive sort must reproduce it
byte for byte, so the sort reached every nesting level rather than the top one.
Both run here because this is the one call site that ever emits the
no-identity, no-artifact shape; the per-profile loop's fresh install already
exercises the other writer field by field.

**The narrowing arm** is the only arm that moves a consumer *down* the lattice:
it installs the maximum profile and re-runs `init` at the minimum. Every other
re-run holds the profile fixed, so none reaches the state where `files` outlives
`kits` — and that state is not exotic, it is the ordinary consequence of the
carry-forward rule, which keeps every once-vendored path on the roster while the
recorded kit set shrinks. For every seam path the narrowed manifest records it
asserts that the path still resolves to the consumer's **own** file, and that a
`kits`-stripped copy of that same manifest — the residual shape — resolves it the
same way; then that `doctor` names the consumer's own `gates.list` as the
registry it inspected. It asserts its own premise too: at least one seam path
checked, and at least one of those genuinely shadowed by a vendored fixture tree,
or the resolver has nothing to be ambiguous about and a green result would mean
only that the payload changed shape.

*Its honest limit.* Only the recorded paths are asserted on, and the config seam
is written on the artifact placement path alone — so on a payload carrying no
prebuilt binary the arm would resolve the registry and not the seam. It runs
against the main payload, which carries one, so both are checked today; what the
arm still refuses to do is *demand* the seam, because that would fail it on the
payload rather than on the resolver. The residual shape is what keeps it biting
either way: with no `kits` key, a tail match returns a vendored fixture for
`gates.list` too.

**Every arm above rides §The gate binary's placement outcome**, because the
payload they install carries this run's artifact: the binary is written, an
`artifact` key reaches the manifest, nothing is omitted, and each profile's
battery dispatches its `.gate` members through the placed path. That outcome is
now the `--install place-artifact` op's rather than an inline branch of `init`,
so the same arms are also what asserts the first relocated step behaves as the
shell block it replaced — the assertions themselves did not move. What the
**omission** outcome became is a refusal, and the leg that used to assert the
omission asserts that refusal: an artifact-less payload is now told apart from a
covered one before any verb runs, rather than after an install that silently kept
none of its battery.

**The binary is built rather than fabricated**, which is what makes the placement
branch worth exercising at all. A stand-in with a matching digest would drive the
same code while leaving the one thing most likely to break — the real build's
digest agreeing with what `init` verifies before writing — covered by nothing. So
the smoke asserts the target `init` selected against what `rustc` independently
reports this host to be, the recorded digest against the one the build step
emitted, the binary executable at the path the config seam names, and both that
path and the seam on the manifest roster.

**The artifact arm is what remains once the main payload carries the binary**:
the outcomes a single install cannot show. It takes its own extraction of that
same tarball and mutates the copy, rather than packing a second time — a host
**off** the roster refuses naming the platform and offering no adopter action, a
**tampered** artifact refuses with the consumer's tree object unchanged and no
manifest written, and a **declared** target whose artifact is gone refuses as a
publisher defect the adopter can act on. All three are bootstrap outcomes,
decided before the invoke, so the arm is unmoved by the placement step going
behind it. Two of the three now refuse, so the arm asserts each one's own
**message and remedy** and then asserts that the two **differ** — the exit status
is not what tells them apart, and an arm that read it alone would see one answer
where an adopter must be given two: one they can do nothing about, and one they
fix by re-downloading. That comparison is what keeps §The gate binary's three
outcomes from collapsing into each other. Mutating an extracted package rather
than adding a flag to the publishing path is deliberate: it leaves the publisher
no way to ship a payload with a hole in it.

`pack-installer.sh` gains nothing from this: the **smoke** builds and hands it a
directory, while the publishing path still never builds, so a locally built
binary can still never substitute for a released one.

**The smoke steers its own roster, and that is what a second roster line stopped
costing.** A single host build satisfies `--artifacts` only while the roster
declares this host alone — pack refuses a roster target no leg built, and
refusing is right, because a payload missing a declared target is broken rather
than narrower. Two exits from that were on the table once the roster was about
to grow: narrow the smoke's roster through `GATE_SDK_NATIVE_TARGETS_FILE`
(gate-sdk/SPEC.md §Layout and configuration), or give the build step a
cross-compiling build. **The steering is built and the cross-build is refused**,
and the refusal is on grounds already in the tree rather than on cost:
`native/targets.list` refuses a cross-build because it would publish an artifact
no run has ever executed, which is the same bound the roster's own join
predicate rests on.
**That guard is unconditional where its stated ground is not, and a reader owes
the difference.** It refuses a roster wider than this host *before* reaching the
artifact hand-off branch and without consulting it, so a run pointed at a
complete producer hand-off — which could satisfy `--artifacts` for every
declared target — refuses byte-identically to one holding nothing, naming a
host-build remedy that does not describe the hand-off path. The guard is
therefore stricter than the reason it gives. Nothing is blocked by it today:
every caller either steers at its own host or lets the smoke self-steer.

So the smoke derives a one-line roster from this host's triple and points that
knob at it **unless the caller has already set it**. Three consequences, each
stated because the cheap reading loses one of them. A caller that steers
explicitly keeps its own roster, so the override branch stays a live path rather
than a fixture-only one. A local `bash installer/consumer-smoke/run-smoke.sh`
survives a multi-line shipped roster instead of blocking on it, which is what
made the second roster line affordable at all. And the *incidental* assertion
the unsteered Linux leg used to carry — that the shipped roster is packable,
true only by the coincidence that its one line was that leg's host — is gone;
what replaces it is stronger and deliberate, the `native-artifacts` producer
building every declared platform on every run, so the shipped roster's
producibility is asserted directly.

**The narrowing removes one reader's subject, and that reader gets a planted
witness rather than an argument.** Pack's refusal of a *declared target no leg
built* is the one verdict here that reds on finding a target instead of on
finding none, so a roster narrowed to the host makes it unreachable on every
ordinary path — where every other affected reader is monotone under the
narrowing and clears by inspection. The pack arm therefore plants that case
explicitly, packing against a roster carrying a second target the artifact
directory has nothing for, and asserts both the refusal and its cause; its log
line is `pack: a declared target with no artifact directory refused, not packed
narrower`. Read it as distinct from the artifact arm's `declared target with no
artifact: refused, not omitted`, which is `init` refusing a broken *payload*
after the pack succeeded — same shape of mistake, two different verbs, two
witnesses.

**A platform leg's steered roster makes the artifact branch live on that host,
and that is the fact a dormancy argument gets wrong.** A per-platform
install-smoke leg steers `GATE_SDK_NATIVE_TARGETS_FILE` at a single-host roster,
the packer copies that roster into the payload verbatim, and `target_of_host()`
maps the leg's host to a triple — so the roster comparison matches and
`select_artifact` takes its artifact-present branch on a platform
`native/targets.list` does not declare. A claim that code below that branch is
dormant *because the shipped roster carries no such line* is therefore false on
every platform leg, and a site costed dormant on that ground is a live site.

It takes two knobs. `INSTALLER_SMOKE_TMP_DIR` is the scratch base, and the smoke
writes nothing inside the worktree: the crate's build output lands in gitignored
build space and the artifact directory it assembles lives under that scratch, so
the clean-worktree precondition keeps its meaning. It needs that clean worktree,
because the pack step refuses to stamp a commit the payload does not match.

`INSTALLER_SMOKE_ARTIFACTS_DIR` is the **artifact hand-off**, and it is the
difference between a run that exercises a release-shaped artifact and one that
exercises a harness stand-in. Set it to a `<dir>/<target>/` tree a producer
already filled and the smoke installs *those* bytes: it builds nothing, and it
recomputes nothing — the sidecar that arrives is the sidecar the producer
emitted beside the bytes, so gate-sdk/SPEC.md §Consumer payload's one-producer
rule reaches across the new hop unbroken. Its live setters are the macOS
install-smoke legs in `.github/workflows/gates.yml` — one per macOS
architecture, each a deployed configuration rather than a test-only one; outside
those legs it is unset and the smoke's behaviour is what it always was. Read the
setter set as "the legs consuming the producer's upload" rather than as a named
leg: a platform joining or leaving the declaration moves it, and the hand-off is
what those legs have in common.

`cargo` and `rustc` join the preflight alongside the tools every other arm
needs, and refuse there when either is missing — a machine that cannot compile
the crate has not falsified the install path. That refusal relaxes **on the
hand-off path alone**: a host handed a prebuilt artifact was never asked to
compile anything, so refusing it for a missing compiler would refuse the exact
case the knob exists to serve. Where `rustc` is absent, the sole target
directory in the hand-off answers the host triple `rustc` would have; two
directories and no `rustc` is refused rather than guessed.

**What it costs to run, because the precondition above is only expensive if you
know that.** Re-measured 2026-08-13 on one developer machine, against
the composition this section describes — four profiles × install + battery +
value arm, the binary-less leg, four packs, an npm install and one release build
of the crate — the whole smoke took **272 seconds**. The 2026-08-09 measurement
it replaces was 227 over a composition with one arm fewer, taken twice
independently inside a full `--run-validate` run whose 24 suites
finished in about 536.
So it fits one ordinary foreground invocation and needs no session-window
planning, and the clean worktree it holds is held for minutes rather than for a
working session. That is a **dated measurement on one machine, not a live claim**
— it is recorded because sessions kept re-deriving it by wall clock and one
estimated it an order of magnitude high; re-measure rather than trust the number
if the decision turns on it.

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
