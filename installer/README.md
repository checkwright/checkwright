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

Bash, up to the boundary §The install boundary rules: the verbs are bash, and
what the bootstrap hands to the verified gate binary is Rust the same section
names. npm is the delivery vehicle, never the implementation: the `bin` entry is
a bash script, so a reader reviewing what they are about to run reads source
rather than a build product, and the linter that governs every other script in
the repository governs these too.

## Requirements

This package reaches a tree over two transports, and each carries its own
requirement. Fetched from npm it needs Node, for `npx`. Fetched as the tarball
attached to a GitHub Release it needs none — `curl`, `tar`, and `sha256sum`,
then `bash package/bin/checkwright.sh init`. Both requirements belong to a
delivery path alone: the gate battery this vendors does not use Node, no
delivery-path tool joins the toolchain roster, and the manual vendoring path
documented on the site needs neither. The toolchain the battery does assert,
with its version floors, is on the install page.

**The verbs themselves need `jq`.** `init`, `diff` and `uninstall` read JSON —
this package's own version stamp and the `checkwright.lock` manifest — and they
read it with `jq`, so every one of them refuses without it, naming the program
and the remedy (§The manifest). This is a claim about *these verbs*, separate
from the install page's toolchain block, which names `jq` for the gates in the
battery being vendored: different programs' users, and a machine can satisfy
either without the other, though a machine missing `jq` is below both.

`doctor` is the exception by design, and the exception is that it **reaches its
diagnosis** rather than that it is content: it reports a toolchain instead of
refusing before it can report one, so on a machine with no `jq` it renders its
whole report, names `jq` as missing, and tells you that is also why the manifest
could not be read. Its verdict is still **below contract**, because `jq` is a
consumer-audience member of the toolchain floor and the battery being vendored
needs it too. So the installer always knew about this dependency and `doctor`
always said so — what was wrong is that `init` runs `doctor` *last*, after the
JSON reads that need `jq`, so the one verdict that named the program was ordered
behind the refusals that could not. The precondition above is that ordering
corrected, not a fact newly discovered.

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

## The verbs

`init` makes an install; the rest manage one after it exists. The roster itself
is derived — a verb is advertised because its implementation is present under
`lib/`, so `checkwright --help` lists the directory rather than a list beside it
— and what follows is what each verb is *for*.

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

Four preconditions, and all of them **refuse** rather than warn — a partial
install is the outcome none of them may produce, so every one is checked before
a single file is written:

- **You are inside a git work tree.** The vendored source is meant to be
  committed; that is what makes it auditable rather than merely present.
- **`jq` is installed.** It sits immediately before the first JSON read rather
  than at the very top of the verb, because that is where *before the first
  read* falls: the two preconditions either side of it are about the package and
  the repository and are answerable without `jq`, while every refusal after it
  is one a `jq`-less machine would otherwise meet as a misdiagnosis. That is the
  whole of what this precondition buys, and it is worth stating as the
  deliverable rather than leaving it to be read as a side effect: the refusals
  below it — *this package carries no version stamp*, and *`checkwright.lock`
  carries a schema this build does not know* — **keep their exact text and
  recover their accuracy**. Each is the right message for the condition it
  names, and each was only ever wrong as the message a `jq`-less machine
  received, reached by a path that had nothing to do with what it said. After
  this precondition, meeting one of them means the condition it names is
  actually true.
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
what `lifecycle-kit/bin/install-lifecycle.sh` writes, which is the adopter's own
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
what keeps `recipe_needs_agent_file` from being a seeding roster — the predicate
asks whether the agent file must *exist* for a kit's starting gates, which
context-kit answers yes to while writing nothing into it; the seeding arms ask
the narrower question of which kit writes into it.

**The roster is derived from the gates themselves.**
`recipe_gates(kit-payload-dir, profile)` in `lib/common/recipe.sh` is the
**whole** of a fresh consumer's registry — what a tree `init` just made will run
is read there, never inferred from a kit's full roster or from this repo's own
`gates.list` — and what it reads is each shipped gate's own
`# install: <disposition>` header line, taking every member a kit declares
`zero-config` (gate-sdk/SPEC.md §The install disposition). It carries no gate
name of its own, so a kit that adds a zero-config gate is picked up with no edit
here. One function unions the result over a profile's kits, `profile_gates` in
`lib/common/profile.sh`, and both the registry `init` writes and the consumer
smoke's monotonicity assertion read that one derivation. No disposition varies on
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
nothing to derive from, because §The consumer smoke's own rule forbids
`lib/common/recipe.sh` a literal gate name and no gate declares
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

`init` is bash today, and one part of it must stay whatever language the host
already runs: the bootstrap that resolves, verifies and executes the gate
binary. Everything else is **conditional install logic**, and
TRAJECTORY.md's interpreter policy (§The closed rulings) rules that everything
conditional belongs on the far side of that invoke — written once, in Rust,
rather than twice, in bash and in the PowerShell half a native Windows install
needs. This
section states the bootstrap's job, the disposition every install step carries,
and the test that assigns one, so a step's side of the line is read off a rule
rather than re-argued per step.

**The bootstrap's job is the whole of what is written twice:**

1. resolve the package's own payload directory;
2. resolve the host to one Rust target triple;
3. read the payload's target roster and resolve the artifact and its sidecar,
   refusing a declared target whose pair is incomplete;
4. verify the artifact's SHA-256 against that sidecar;
5. execute the verified artifact, forwarding argv verbatim.

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
  the worked case: nothing behind the invoke reads JSON with `jq`.

**The two bootstraps are hand-kept, and parity is held by running, not by
generation — ruled 2026-08-26.** Each half is authored in its own language
against the five steps above, and the oracle that holds them equal is the
per-platform install-smoke leg (`platform-support-ci-matrix`): a leg per
bootstrap, each exercising the payload end to end on the host that bootstrap is
for. The alternative refused is one declaration generating both halves. Its
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
(gate-sdk/SPEC.md §lib/gate.sh) — the payload's own assumed-program set, not
`context-kit/lib/toolfloor.sh`'s consumer-audience probe roster that `bash` also
happens to sit on — and the arm declares it. So the step moves behind the invoke
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

`place-artifact` is the one op today and it produces no `1`: its only failures
are a bad argv and a write it could not make, which are both `2`. The status is
specified on the family rather than on the op, so an op that *can* refuse
something the adopter can act on has a status to refuse with rather than minting
one.

**The relocation's own precondition: an uncovered platform must still install.**
A step may move behind the invoke only where the binary is reachable on every
platform that step runs on today, and today it is not. §The gate binary's
selection table has three outcomes and two of them leave `init` with no binary —
a host whose triple the payload's roster does not carry
(`substrate-unavailable`), and a host with no SHA-256 hasher
(`digest-unverifiable`). Both **proceed** today, omitting the compiled gates and
declaring the omission in the consumer's `gates.list`, and that branch is what
keeps a freshly vendored battery alive on an uncovered platform. Once
conditional install logic sits behind the invoke, the same branch has nothing to
run at all: the failure mode the relocation introduces is **not a smaller
battery but no install**, and nothing in tree asserts that a relocated step
still runs on an artifact-less host.

**The rule that yields, and it is what selects each cut:** a step is takeable
now iff it *already* runs only when an artifact was selected. Such a step costs
an artifact-less host nothing, because on that host it never ran. Two
consequences follow for the steps that are not:

- **Relocating the *unconditional* remainder of `init` is sequenced behind the
  artifact roster covering every supported platform** — the ground
  `platform-support-ci-matrix` covers and `native/targets.list` declares.
- **`digest-unverifiable` must become a refusal rather than an omission** at the
  same moment, because step 4 of the bootstrap is irreducible: a host that
  cannot hash cannot verify, and verifying before executing is the whole of the
  integrity claim. On Windows the branch is vacuous — PowerShell carries
  `Get-FileHash` — so the cost lands on a POSIX host missing both `sha256sum`
  and `shasum`.

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

**The map answers "which published artifact fits this host", which is why a
MinGW, MSYS or Cygwin `uname` maps to `x86_64-pc-windows-msvc`.** Those `uname`
strings report the *shell environment* — the measured runner answers
`MINGW64_NT-10.0-26100`/`x86_64` — and not the toolchain that built the artifact
the adopter is about to receive; what a Windows build leg would publish is what
that host's own `rustc` reports as its host triple, `x86_64-pc-windows-msvc`.
Before this arm those hosts matched nothing and the function returned empty.
**The arm's verdict is unchanged today, stated rather than left to be
discovered**: its reader is `select_artifact`'s roster comparison, live on every
install, and that comparison still fails because `native/targets.list` does not
name the triple — a target joins the roster only on a run that produced and
exercised its artifact (gate-sdk/SPEC.md §Consumer payload), and none has. It is
reachable **now** for testing through `GATE_SDK_NATIVE_TARGETS_FILE`, the same
steering knob §The consumer smoke already documents as the roster re-entry.
Whether an msvc-built binary runs on an arbitrary Windows host is a question no
run has answered, and withholding that answer until one has is that join bound
doing its job.

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

A third token would need a third remedy to earn its place, and one was
**proposed and refused on exactly that rule**: splitting `substrate-unavailable`
into *host-unmapped* (the `uname` pair maps to no triple) and
*target-not-published* (it maps to one the roster does not carry), which
`select_artifact` collapses into a single token today. Both cases have the
**same** remedy — there is no adopter action, the platform is not in the support
roster — so no token is minted and the collapse stands. Which members are
affected is derived from the payload, never maintained here: a starting-roster
gate the payload declares as `<kit>/checks/<name>.gate` — and does not also ship
as a shell script — is one that dispatches to the binary. `run-gates.sh` reports
the count and remedy on a line of its own beside its summary
(gate-sdk/SPEC.md §run-gates), and `doctor` reports it against the reason that
caused it.

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
run built; the **omit-and-declare** row is the binary-less leg's and the artifact
arm's undeclared-host leg's; the **refusal** row is the artifact arm's alone,
against a payload it mutated after extraction. Nothing here changes for that —
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

**The identity block names the registry this tree's battery runs from**, beside
the version, commit, profile and kit set. It is the one install fact the identity
fields cannot be read off, and the one a reader most needs when the report and
the tree seem to disagree: a `doctor` that resolved the wrong file would
otherwise say nothing at all about which one it inspected. A recorded registry
missing from disk is named as such, and a manifest recording none says so — both
with the same remedy, a re-run of `init`.

`doctor` defines no floor of its own. It sources the toolchain roster out of
its own `payload/` and renders whatever verdict that roster's predicate
returns, so the contract keeps one owner and this stays a display of it. Which
members are consumer-audience — and so which absences set the verdict — is read
off that roster, never listed here; §Requirements works the one case a reader
arrives with, `jq`, and says why it is below contract rather than outside it. The
payload copy is the one it reads, never a copy in the tree it is inspecting:
at `init` time nothing has been vendored there yet, so a tree copy would not
exist at the moment the answer is needed.

**A residue is read apart from an install, and reported as one.** A
`checkwright.lock` carrying `files` and no `version` is not an install — it is
what `uninstall` leaves over files you had edited (§uninstall) — and `version`
is the field an install always has and a residue never does, so its absence is
the discriminator. On that reading `doctor` reports how many files remain and
that they are yours, and prints **no** identity block, no artifact check and no
omitted-member report: every one of those is a per-install reading with nothing
left to describe. Blank identity fields would be uninformative rather than
wrong, but printing them beside a residue message is the same mixed-verdict
shape the exit-status carve-out above already refuses.

On both readings `doctor` closes by naming `diff`. It reports what was
installed, never whether the tree still matches it, and keeping the two apart is
what stops `DOCTOR: clean` from being read as a claim about the tree's contents.

A third exit status, `2`, means the question could not be answered rather than
that the answer was bad: the package carries no payload, or the manifest
carries a schema key this build does not know. A build refuses an unfamiliar
manifest rather than guessing at the shape behind it. **A missing program is that
same status by derivation rather than by a new code** — a verb refusing because
`jq` is absent has not judged the tree, the package or the manifest, which is
what `1` is reserved for; it could not answer. It is also the status the
misdiagnosing refusals already exited with, so no caller's exit-code handling
changes and only the message does.

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
toolchain, an absent `jq` — arrives prefixed **`checkwright init:`**, and the
success path
reports `INIT:`, because that is literally which verb produced the line. The
`jq` refusal inherits this without a second rule: it is raised from `lock.sh` but
surfaces through the calling verb's own `die`, so it carries that verb's prefix,
its `help:` line and its exit code rather than a separate idiom. This is
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
payload by `lib/common/profile.sh`, so a membership row and a declared parent
can never disagree.

## The manifest

`init` writes `checkwright.lock` at the root of the repository it vendors into,
and that file is tracked like everything else it writes. It is JSON, read with
`jq`, and it is the install-ownership record: what was installed, from which
upstream state, and which files this installer owns. `lib/common/lock.sh` is
its schema owner — the wire key, the accessors, the hash rule and the emitter
live there, so the two verbs that write a manifest and the verbs that read one
cannot drift apart. Being the single writer is what makes the shape a contract
rather than a convention: keys are sorted at every nesting level, and a field is
present exactly when its writer supplied one, so an omission leaves the key
**absent** rather than null or blank. `commit` is the field that rule most
recently had to be applied to: a package with no commit stamp leaves it out of
the manifest rather than writing it empty, on the same conditional footing the
`artifact` key already sat on. Until the `jq` precondition landed ahead of it, an
empty `commit` was ambiguous between *no stamp* and *no `jq`* and was masked by
statement order alone; with `jq`'s absence refused up front the field is
decidable, and the writer's existing omission rule settles it with no new one.

Reading it needs `jq`, and that is a dependency of the verbs rather than of the
file: `lock.sh` declares it once, beside the wire format it is a fact about, and
each JSON-reading verb calls that declaration before its first read. So a machine
without `jq` is told which program is missing, instead of being told the package
has no version stamp or the manifest has an unknown schema — refusals that are
correct for the conditions they name and were never correct for this one.

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
| `files` | `init`'s ownership roster — each path it has written, at the content hash it last wrote there, until the file leaves the tree | `init`'s changed-file detection: a file whose hash still matches is rewritten, one that has changed is reported rather than overwritten, and stays on the roster so the next run reads it the same way, whether or not the running release still ships that path. `uninstall` walks the same roster to decide what it may remove and what it must keep, and `diff` classifies it against the tree |
| `artifact` | the gate binary's `target` and its SHA-256 `digest`, or absent | `doctor` reports the target and re-verifies the digest in place; a re-run of `init` compares the target against this host and skips the rewrite while the digest still holds |

**Resolving one of your own seam files is an exact-path question, not a search.**
`doctor` and `uninstall` both need to know which `gates.list` — or which
`gate-sdk-config.sh` — is *yours*, because the vendored kits carry fixture trees
holding files of the same name. `files` already answers it: it records the
repo-relative path `init` wrote, and `lib/common/recipe.sh` owns the gates-dir
constant `init` wrote it under, so `lock_own_file` asks whether the manifest
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
`git hash-object`. Row and key compose on the one file — the row says whether you
have edited it, the key says whether it is the artifact that was published — and
§doctor and §uninstall each read the one they mean.

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

## The consumer smoke

`consumer-smoke/run-smoke.sh` is the acceptor for everything above, and it is
registered as a validate suite so a bit-rotted activation path is a red
validate rather than a discovery at announcement.

**The smoke packs the tree it lives in, by construction — the current directory
does not select it.** The script resolves that tree from its own path and hands
it to `scripts/pack-installer.sh --root` at every one of its pack call sites, so
the packed tree and the asserted tree are the same tree whatever directory you
invoke from. A clone's copy invoked by absolute path, a second
checkout, a linked worktree: all pack the tree the script belongs to. The
packer's flag roster has exactly one tier and it is the script itself —
`bash scripts/pack-installer.sh --help` prints it on stdout at exit 0 — so no
doc carries a second copy to drift.

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
`--offline`**, and drives a scratch consumer once per profile: `init`, then the
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
a second roster to maintain beside `recipe_gates`, and the claim is about the
battery rather than about a member of it. Nor does either name a profile, for
the same reason the lattice assertions do not — which profiles catch a prose
defect follows from the rosters, and spelling it here would be that derivation
copied out. What the arm turns from a claim into an assertion is the one the
install page makes hardest to check: an install that is green, idempotent and
reversible is still worth nothing until it catches something.

That is the **coverage** claim, and it speaks only for a covered platform. What
an adopter on an uncovered one gets is a different claim — **disclosure** — and
it has its own leg, because folding the two into one verdict is what let a cohort
of ports pass per member while emptying a value class for every binary-less
install.

**The binary-less leg** installs one profile from a payload packed with no
artifact, and asserts that every member the install loses is recorded
`# omitted: <name> <reason>` in the consumer's own `gates.list` — against the set
that payload dispatches to a binary, derived rather than spelled — and that the
count is **non-zero**. The count is the half that matters: without it the
completeness assertion passes vacuously on a payload that dispatches nothing,
which is exactly the reading under which N individually runnable members
discharged a cohort that left such a consumer with no markdown-link governance at
all (gate-sdk/SPEC.md §The port-candidate criteria, criterion 5). The leg
deliberately does **not** assert that the planted defect goes uncaught: pinning a
missing capability as expected behavior would make the hole permanent the moment
it closes. It names a profile where the value claim above names none, and the
difference is real — this is a scoping choice about which install to run, not a
derivation of which profiles catch what. The profile it names is the one whose
roster criterion the binary-gated class empties.

It also asserts the profile lattice
against the installed payload, in four parts: every named kit resolves in the
payload; the derived order has **exactly one minimum and exactly one maximum**,
so the lattice is bounded; that maximum is the payload-derived profile; and
**gate rosters are monotone** — for every comparable pair, the smaller profile's
gate set is contained in the larger's. The fourth is the one that earns
`recipe_gates`' profile argument: what you experience is the battery, not the
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
side effect. It is also the assertion form of the claim the install page makes:
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

**The `jq`-less arm** asserts what a machine without `jq` is told. It drives
`init` on a tree with no manifest — the case that used to surface as *this
package carries no version stamp* — then makes an ordinary install and drives
`init`, `diff` and `uninstall` against it with `jq` gone, the cases that used to
surface as *carries a schema this build does not know*. Each must refuse, **name
`jq`**, carry a `help:` line and exit 2. Naming the program is the whole
assertion, and asserting the exit status alone would have been worthless: these
verbs already exited 2 before the precondition existed, so a status-only arm
would have passed against the very defect being fixed. `doctor` is asserted on
the other side of the boundary, and the boundary is *reaching the diagnosis*
rather than the exit code: it must render its below-contract verdict, name `jq`
as missing in the toolchain block, **and** say `jq` is why the manifest could not
be read. Exit 1 is the correct verdict there, and asserting exit 0 would have
been asserting the opposite of the contract — `jq` is a floor member, so a
machine without it is below contract and `doctor` is the verb that says so.

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
manifest when none is passed, and `claim()` re-applying the payload around a file
that has changed since `init` wrote it — left alone, reported, and still the
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

**The first hop's clean-worktree assertion carries a tripwire**, for the same
reason the binary-less leg's disclosure count does: the assertion is evidence
only over a hop that rewrote something. This arm's payload is artifact-free over
the lattice minimum, and for as long as that kit set dispatched no member to the
binary, `init` seeded no omission, the hop rewrote nothing beyond the manifest,
and the assertion passed for a reason unrelated to correctness — which is how a
defect leaving that very worktree dirty reached the tree under a green
assertion. So the hop asserts a **non-zero** count of omission-declaring lines in
the consumer's own registry *before* asserting the worktree is clean, reading the
registry rather than the run's output because the registry is what a later run
and a reviewer both read, and treating an absent registry as a count of zero
rather than as a reason to abort. Its failure names the remedy — **re-scope the
arm onto a profile whose kit set ships a member `init` seeds and dispatches to
the binary, never drop the assertion.** Its scope is the first hop; the second
hop's clean-worktree assertion carries no tripwire, and widening it is a separate
judgment.

**The seam arm** covers the two surfaces `init` rewrites on every run — a
`templates/*-config.sh` destination and gate-sdk's `msg-patterns.list` — which no
arm above reaches. That is not an oversight in those arms: the upgrade arm's
subject is an ordinary vendored file on the plain `copy_in`/`claim` path, so they
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
shell block it replaced — the assertions themselves did not move. The **omission**
outcome is not thereby lost — it is what the binary-less leg above is for, and
there the members recorded `# omitted:` are exactly the ones the consumer's own
vendored tree implements as a binary subcommand. That set is derived rather than
spelled, which is what keeps the assertion alive: a literal would read as green
on a tree where nothing has ported and stop asserting on the first tree where
something has.

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
**off** the roster omits and declares and exits clean, a **tampered** artifact
refuses with the consumer's tree object unchanged and no manifest written, and a
**declared** target whose artifact is gone refuses rather than omitting. All
three are bootstrap outcomes, decided before the invoke, so the arm is unmoved
by the placement step going behind it: the two refusals are step 3's and step
4's, and the omission is step 2's. The last
two are what keep the three outcomes of §The gate binary's table from collapsing
into each other. Mutating an extracted package rather than adding a flag to the
publishing path is deliberate: it leaves the publisher no way to ship a payload
with a hole in it.

`pack-installer.sh` gains nothing from this: the **smoke** builds and hands it a
directory, while the publishing path still never builds, so a locally built
binary can still never substitute for a released one.

*The honest limit, recorded because it is a fact about today's roster rather
than a property of the design.* A single host build satisfies `--artifacts` only
while `native/targets.list` declares this host alone — pack refuses a roster
target no leg built, and refusing is right, because a payload missing a declared
target is broken rather than narrower. The moment the roster declares a second
target the build step blocks rather than passing, naming its own re-entry: steer
its pack at a narrowed roster through `GATE_SDK_NATIVE_TARGETS_FILE`
(gate-sdk/SPEC.md §Layout and configuration) so the smoke commits to the host
alone while the published payload still commits to all of them, or give the step
a cross-compiling build. Neither is built ahead of the second target. Note where
that block now falls: the build precedes the main pack, so a second declared
target stops the whole smoke rather than only its last arm — which is the honest
consequence of the main payload carrying the artifact, not a regression.

Its only knob is `INSTALLER_SMOKE_TMP_DIR`, and it writes nothing inside the
worktree: the crate's build output lands in gitignored build space and the
artifact directory it assembles lives under the smoke's own scratch, so the
clean-worktree precondition keeps its meaning. It needs that clean worktree,
because the pack step refuses to stamp a commit the payload does not match. It
needs `cargo` and `rustc` too, alongside the tools every other arm needs, and
refuses at the same preflight when either is missing — a machine that cannot
compile the crate has not falsified the install path.

**What it costs to run, because the precondition above is only expensive if you
know that.** Re-measured 2026-08-13 on one developer machine, against
the composition this section describes — four profiles × install + battery +
value arm, the binary-less leg, four packs, an npm install and one release build
of the crate — the whole smoke took **272 seconds**. The 2026-08-09 measurement
it replaces was 227 over a composition with one arm fewer, taken twice
independently inside a full `evidence-kit/bin/run-validate.sh` whose 24 suites
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
