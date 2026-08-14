---
title: Install
nav_order: 3
---

# Install and upgrade

Checkwright is distributed git-native and vendored-committed: the kit
directories live in your repository as committed source, with the one exception
§What a gate discloses draws below — a gate whose implementation is compiled
arrives as a verified binary instead. The gates read tracked files, and the audit
story wants the governance layer inside the reviewed tree.

The two registries stand in different relations to that, and the distinction is
worth drawing rather than blurring:

- **crates.io holds the name only, never a dependency channel.** There is
  nothing to `cargo add`.
- **npm carries a real package, and it is an installer rather than a
  dependency.** `npx checkwright init` copies pinned kit source out of its own
  payload into your tree and commits it.
- **The same payload downloads straight off the GitHub Release**, as a tarball
  and its `.sha256`, attached to every tagged release. This strengthens the
  doctrine rather than weakening it: a tarball you download, verify against its
  published digest, and extract before anything runs is the most auditable form
  of the same one-shot vendoring.

The last two are two transports over one install model, not two products. Both
reach the same `init`, write the same `checkwright.lock`, and leave the same
vendored result; only the fetch differs. The tarball is the primary path because
it removes a runtime dependency. That sentence is the reader-facing tier of a
declared claim: its machine-readable tier is the `install-primary:` declaration
under §Quick start, and
[canon-kit/SPEC.md](canon-kit/SPEC.md#check-install-claim) owns what binds the
two. npm is retained because it is the path that
carries a build attestation — `npm publish --provenance` has the release runner
sign a statement of what built the package, which a self-hosted asset cannot
offer. Neither dominates, so both properties are named rather than one being
called better.

An installer is not a dependency channel, and the difference is mechanical
rather than a matter of framing. Nothing resolves at *your* build time. The
payload ships inside the published tarball, so nothing is fetched after the
package itself. And nothing the installer writes is a resolvable reference —
no `dependencies` entry, no lockfile pointing at a registry, no submodule, no
install-time lifecycle script. Uninstall the package afterwards and the tree it
vendored keeps working, needing nothing from a registry ever again.

That is what a one-shot vendoring means, and it is why the doctrine survives the
installer rather than being repealed by it: what governs your tree is still
committed and auditable, on the terms §What a gate discloses sets out below.
Both properties are asserted rather than claimed — the consumer smoke installs
from a packed tarball with no registry access and runs the whole path from it, and
`check-installer-no-deps` reds the package the moment it declares a dependency
field or an install-time script.

Before you vendor, the [footprint page](footprint.md) measures what each kit
adds to a consumer's context budget — the always-loaded and load-triggered cost
per kit, so the adoption decision weighs a number rather than a guess.

## Requirements

Checkwright is **Unix-first**, and specifically **GNU-first**: the engine is
portable to any Unix that presents a GNU userland on `PATH`, which Linux
distributions do out of the box. Windows runs it through WSL (Windows Subsystem
for Linux), not natively — the gate battery and the git hooks are Bash scripts
and no native-Windows shell path exists.

macOS runs it too, but as an adopter action rather than something the stock
system delivers. Stock macOS ships bash 3.2 over a BSD userland whose `sort`,
`date`, and `stat` reject the flags the gates pass. Install GNU bash together
with coreutils and gawk, then put them ahead of `/usr/bin` on `PATH`. That last
clause is the honest limit: the requirements below assert what `PATH` actually
resolves, so a Mac carrying Homebrew coreutils that is not `PATH`-ordered
reports below contract — correctly, since BSD `sort` is what the gates would
invoke.

The battery leans on a small command-line toolchain; each tool below must be on
your `PATH`, and the note says what breaks without it:

<!-- toolchain:begin -->

- `bash` (≥ 4.3) — every gate and both generated git hooks are Bash scripts;
  nothing in the battery runs without it. The floor is the highest construct the
  battery runs: a nameref (`local -n`) in the gate library every check sources.
  Associative arrays, `mapfile`, and the lowercasing case expansion are more
  widespread but only reach 4.0.
- `git` — the gates read tracked files and the hooks fire at commit time; the
  model is git-native end to end.
- `jq` — the settings and evidence gates, and guard-kit's JSON tooling, parse
  their inputs with it.
- `awk` (GNU) — the gate family's line scanning and field extraction are written
  in awk; most checks cannot run without it. GNU awk specifically: the 3-argument
  `match()` in `check-gate-assertions` is a gawk extension.
- `sort` (coreutils) — the battery's file plumbing assumes GNU coreutils, and
  `sort` is the member standing for that family. The binding construct is
  `realpath --relative-to` in the gate library every check sources; the release,
  drift and usage tooling reach for `sort -V`, `date -d` and `stat -c` besides.
  No BSD equivalent carries those flags.
- `shellcheck` — the `check-shellcheck` meta-gate runs
  [ShellCheck](https://www.shellcheck.net/) over every shipped script, and a
  lint finding blocks the commit.
- `cargo` (≥ 1.56, @contributor) — a **contributor** requirement with **no install-time role at
  all**: the `native/` crate carries the gate implementations that dispatch to a
  binary subcommand, and the floor is the crate's `edition = "2021"`. Gates in
  this repo now dispatch there, so a contributor builds the binary
  (`bash gate-sdk/bin/build-native.sh`) **before
  committing** — `cargo test` compiles a different artifact and does not
  discharge it; CI builds, lints and tests the crate every run. Installing
  Checkwright never builds it and never will. The binary reaches an adopter as a
  prebuilt Release asset for a declared target, digest-verified before `init`
  writes it, and where no asset matches your host `init` omits the ported gates
  from your registry and records why — so no install path asks you for Rust. That
  publish path builds and attaches those assets from the tag itself, so what any
  one release carries is read off its own Release page rather than asserted here.
  A gate on that substrate shells out to git at runtime and embeds nothing.

<!-- toolchain:end -->

A member is pinned only where a construct the battery actually runs forces the
pin, and each pinned member names that construct above. A floor nobody's code
forces is an aspiration, and an aspiration is what rots; the rule is what keeps
this list honest, not a promise to revisit it.

A bullet whose parenthetical carries an `@` token names the **audience** that
floor belongs to, and `@contributor` is the only one there is: a tool nothing on
an install path reaches, required of someone building Checkwright rather than of
someone running it. `checkwright doctor` reads that field and leaves such a
member out of its verdict entirely, so an unmarked bullet is the whole floor an
adopter's machine is held to.

Nor are the bullets maintained beside the code. The roster lives in
`context-kit/lib/toolfloor.sh`; this block renders it, and
`check-install-toolchain` holds the two in whole-element parity — floor,
implementation token and audience included — so the page cannot drift from what
the gates require.

To see where your machine stands against it, seed a local profile with
context-kit's env-probe — `bash context-kit/bin/env-probe.sh` writes an
`ENV.local.md` you keep untracked. It reports each tool's version *and* its
verdict against the contract, so the profile answers whether this box qualifies,
not only what it carries.

Some requirements belong to an **install path** rather than to the battery, and
no delivery-path tool joins the roster above. That roster asserts what the
*battery* requires; how the payload reached your machine is not that, so the
three paths carry their requirements here in prose instead. The **Release
tarball** wants `curl` (or `wget`) plus `tar` and `sha256sum` — a GNU userland
already has them, `sha256sum` being a coreutils member the roster asserts
anyway. The **`npx` installer** wants Node. **Manual vendoring** wants nothing
beyond the roster. Nothing in the gate battery uses Node on any of the three, so
a consumer who would rather not add it takes either of the other two paths and
loses nothing.

Two requirements belong to the installer *itself* — to `init`, whichever
transport delivered it — and they are stated here for the same reason: they are
not what the battery asserts, so the roster above would be the wrong place to
claim them.

- **A GNU `sort`.** `init`'s upgrade path compares two versions with `sort -V`,
  and `context-kit/bin/env-probe.sh` uses the same flag inside the floor
  predicate. The second is the sharper edge: a stock BSD or macOS userland can
  fail the probe that exists to tell you whether your box qualifies, in the same
  way the thing it diagnoses would fail. So the GNU-first instruction above is
  the install path's requirement too, not the battery's alone. It narrows on its
  own once these steps move behind a compiled binary.
- **`sha256sum` or `shasum`.** `init` verifies a prebuilt gate binary against
  its published digest before writing it, and it will take either hasher —
  `shasum` is there because stock macOS ships it instead. Neither present is not
  a failed install: the affected gates are omitted and declared rather than
  written unverified, and `init` tells you which and why.

Publishing a docs site is an optional wider tier. A consumer that registers
site-kit's render-fidelity gate — which re-renders every page through the
GitHub Pages parser — additionally needs Ruby with the `kramdown-parser-gfm`
gem. A consumer that publishes no docs site never installs it.

### Where this is heading

Everything above is what the tree requires today, and today is what you install
against. The direction is a separate thing and is stated separately, because a
requirements page that quotes a floor it has not reached costs you the install
it was written to make easy.

The direction: the battery moves off shell onto compiled gates. Those gates ship
prebuilt rather than built on your machine, and the floor they aim at is **git
alone** — git shelled out, never embedded. What survives is one small bootstrap
that has to resolve your platform before any binary can run. It is deliberately
small enough to exist twice, which is what would make a native Windows path
possible where the roster above can only offer WSL.

<!-- measured: ported-gate-members=42 -->
That direction is now underway rather than announced: 42 gates in the battery
dispatch to the compiled binary today, and the rest is still shell. The
requirements above are what the shell gates actually invoke, so they stand until
the gates that invoke them do not. When a requirement drops it drops from that
list — the list is where you will see it, not this paragraph.

## Quick start

<!-- install-primary: tarball -->

From a clean git repository. Pick a version off the
[releases](https://github.com/checkwright/checkwright/releases) page and
substitute it for `X.Y.Z`. Four steps — download, then verify, then extract,
then run — any of which you may stop after:

```bash
cw="$(mktemp -d)"   # unpack outside the repository — see the note below
curl -fsSL -o "$cw/checkwright-X.Y.Z.tgz" \
  https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-X.Y.Z.tgz
curl -fsSL -o "$cw/checkwright-X.Y.Z.tgz.sha256" \
  https://github.com/checkwright/checkwright/releases/download/vX.Y.Z/checkwright-X.Y.Z.tgz.sha256
( cd "$cw" && sha256sum -c checkwright-X.Y.Z.tgz.sha256 && tar -xzf checkwright-X.Y.Z.tgz )

bash "$cw/package/bin/checkwright.sh" init  # from your repository root
bash gate-sdk/bin/install-hooks.sh          # opt this clone into the generated hook
bash gate-sdk/bin/run-gates.sh              # the battery, green on what was just vendored
```

Unpack outside the repository rather than inside it. `init` refuses a worktree
that is not clean, and an extracted `package/` sitting in your root is untracked
content that makes it exactly that — so a tarball unpacked in place blocks the
install it was downloaded for.

Four commands rather than one piped into a shell is deliberate. A
`curl … | sh` one-liner would have this page contradicting the claim its own
intro makes about what governs your tree, since an unreviewed remote script fed
straight to a shell is that claim's counter-pattern.

The checksum is worth understanding for what it does not cover. It travels from
the same origin over the same encrypted session as the tarball, so verifying it
catches a corrupted or truncated download. It is not evidence that the release
host was uncompromised; the property carrying that is a build attestation, which
is what the npm channel's `--provenance` mints and this channel does not.

The `package/` prefix is `npm pack`'s doing, and it is named here rather than
left to be discovered. npm builds the asset on the release runner; consuming it
needs no Node.

If Node is already on your machine the same install is one command —
`npx checkwright init`, then the two `gate-sdk` lines above. Same payload, same
`init`, same `checkwright.lock`; only the fetch differs.

`init` vendors the selected profile's kit directories and writes a `gates.list`
seeded with each kit's starting gates, alongside the config seam those kits
need. Then it makes **one commit** naming the profile and the version.

Its three preconditions all refuse rather than warn. You must be inside a git
work tree. The worktree must be clean, with `--no-commit` as the valve for an
operator who wants to stage the vendoring themselves. And the toolchain must
meet the contract — which `checkwright doctor` decides *before* any partial
install, rather than halfway through one.

You pick how much to meet first, and which part. `starter` is the framework:
the gate SDK on its own. `delegation` adds the kits whose subject is the agent
session itself. `prose` adds canon-kit instead, for a repository whose artifacts
are documents. Its battery reads every `README.md` at any depth plus your agent
file; widening it to the rest of your docs tree is one line in the canon-kit
config seam. `full` is everything. The contract is kit-set containment rather
than a chain, so moving to a profile that contains yours only ever adds. Two
profiles containing neither the other, as `delegation` and `prose` do, are
alternatives rather than steps.

Re-running is idempotent and non-destructive: it reads the per-file hash
recorded in `checkwright.lock`, rewrites what still matches, and **reports
rather than overwrites** anything you have changed since, unless you pass
`--force`. Every file `init` rewrites is claimed before it is written, so that
covers each kit's config seam and the commit-message patterns — the files you are
most expected to edit — as well as the vendored kit source. Nor does the
protection expire: `init` owns a path because it wrote the file there, so the
path keeps its recorded hash until the file leaves your tree, across later
upgrades and across a release that stops shipping it. `--dry-run` prints the file
plan and the manifest and writes nothing.

`checkwright.lock` is the install-ownership record — what was installed, from
which upstream commit, and which files the installer owns. That commit field is
what lets a reviewer resolve the tree in front of them to an exact upstream
state, which is the difference between vendored source that is merely committed
and vendored source that is auditable.

## Managing an install

`init` makes an install. Three more verbs manage one once it exists. Each
answers in its **exit status** as well as its output, so a CI step can gate on
the answer without parsing a report. What follows is what each verb is for;
the mechanism behind it lives in the installer's own README
(`installer/README.md` in the repository).

- **`checkwright update`** brings the install here up to the version the package
  you are running carries. One added precondition separates it from `init`:
  `checkwright.lock` must already exist, so `update` can manage an install but
  never make the first one. Every `init` flag stays valid, `--dry-run` included.
  That thinness is the design — `update` names an operation `init` already
  performs instead of reimplementing it, so the two cannot drift apart. It
  automates phase A of §The upgrade contract below. Phase B is still yours.
- **`checkwright diff`** tells you which of the files `init` wrote you have
  changed. It writes nothing. Exit `0` means the tree is exactly what `init`
  wrote; `1` means at least one file has changed or gone missing — so *is our
  vendored tree pristine?* becomes a CI check rather than a review habit. Your
  edits are reported, never corrected. Editing a vendored file is sanctioned,
  and this is the verb that tells you where you have done it.
- **`checkwright uninstall`** reverses the install.

**Reversibility, stated plainly, because it is the property you are
evaluating.** `uninstall` removes the files `init` wrote into paths you left
untouched, then makes one commit. It reads the same per-file hashes `init`
recorded, so a file you have edited since is **kept and reported** rather than
removed. It never removes a file you wrote. Where anything is kept,
`checkwright.lock` is narrowed over the survivors rather than deleted, so a
later `init` goes on protecting them instead of writing through your edits.
`--dry-run` prints the plan first, down to which of your own files would be left
behind inside a vendored directory.

That reversal is asserted rather than promised. The consumer smoke installs each
profile into a scratch repository and then reverses it, holding that the
repository's git tree object afterwards is **identical to the one it had before
`init` ran** — so an evaluation you decide against leaves nothing behind, and a
file the installer wrote but failed to record would fail that assertion rather
than survive it.

## Vendoring the kits

The installers above do this on your behalf; the manual path stays supported and
is worth reading whether or not you take it. It is the audit story — the account
of what lands in your tree, and the reason nothing above has to be taken on
trust.

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
[stage-skill modes](lifecycle-kit/SPEC.md#templatesstages).

Start with [gate-sdk](gate-sdk/index.md) — the other kits register into its
runner — then add kits in the order the [kit map](index.md#the-kits) lists them.

### What a gate discloses

Worth knowing before you adopt, because it qualifies the opening claim on this
page. A gate whose implementation is a compiled binary ships four things and
withholds one. It ships its declaration. It ships its `# spec:` pointer with the
specification section behind it. It ships its `good/`+`bad/` fixture pair. It
ships the binary itself, verified against a published digest before anything is
written to your tree. What it does not ship is the implementation source.
[gate-sdk/SPEC.md](gate-sdk/SPEC.md#consumer-payload) is where that is ruled and
bounded.

Most of the battery is shell you can open and follow line by line. That is a
statement about the corpus as it stands, not about the contract: what a gate
discloses is set by the rule above, whatever the shape of the shipped set on the
day you vendor it.

The reason is narrow and worth stating as narrowly as it holds. A coding agent
told to make your battery green will, if it can, read the gate blocking it and
edit its way around the rule instead of fixing what the rule caught. Withholding
the implementation raises the cost of analysing a gate relative to the cost of
running it. It does not make the rule a secret. A binary can be taken apart. The
fixture pair shows you what passes and what fails. The specification section
states the invariant on purpose. What you keep is the ability to run the gate
and to hold it to its own fixtures, which is what this page means by verifying
rather than trusting.

## Reviewing the pre-commit hook before you install it

Step 4 above points a clone's git hooks at bash from this tree, which is worth
deciding with the files open. The account below aims at completeness over
reassurance.

**What the hook is.** A generated file. `gate-sdk/bin/gen-pre-commit.sh` emits
it from the per-gate `# graph:` manifests, so the hook carries the *triggered
subset* of your registered battery: each gate fires under the path globs its
own manifest declares, and the gates outside that subset run only in the full
battery. The emitted hook is tracked, so the diff you review is what will run,
and `check-graph` holds it byte-fresh against its emitter — a hand-edited hook
that has drifted from the manifests reddens rather than diverging quietly.

The same emitter writes a second tracked hook, `commit-msg`, from the gates
registered at that tier, and `check-graph` holds it fresh by the same
byte-comparison. Installing points your clone at the hooks directory, so both
hooks are what you are reviewing; everything this section says of the
pre-commit hook holds of the commit-msg one.

**What installing it changes in your clone.** `gate-sdk/bin/install-hooks.sh`
has three effects, and an audit of what the script touches wants all three:

- `core.hooksPath` is repointed at the kit's hooks directory. That config write
  is what makes the generated hook fire at commit time.
- `blame.ignoreRevsFile` is set to `.git-blame-ignore-revs`, but only when the
  repo carries that file; a repo without one is left untouched.
- `check-identity` runs once at opt-in, so a wrong-identity or wrong-remote
  mapping surfaces before your first commit. The gate's exit status becomes the
  installer's, so a failing identity check is visible to whatever ran it.

**How to review before running.** The hook and every gate it invokes are
tracked bash under the vendored kit directories and your own gates directory.
Nothing is fetched at install time or at run time, so a vendoring or upgrade
diff shows the whole of what you are agreeing to execute.

**How to disable it.** Per commit, `git commit --no-verify` skips the hook. Per
clone, `git config --unset core.hooksPath` removes it. Both are supported
positions and neither is an escape:
[gate-sdk/SPEC.md](gate-sdk/SPEC.md#enforcement-tiers) rules the local hook a
latency optimization whose guarantee lives in the CI tier, so skipping it costs
an earlier signal and nothing beyond that, provided the outer tier exists.

## Running under an AGENTS.md harness

Checkwright defaults to `CLAUDE.md` as the always-loaded agent file, but no kit
mechanism resolves that file by literal — each reads its kit's knob. A consumer
whose harness reads `AGENTS.md` (or any other always-loaded agent file) runs
every kit mechanism by pointing those knobs at that file. This path is not just
asserted: context-kit ships `smoke/agents-md.sh`, which stands up an `AGENTS.md`
consumer, sets the knobs below, and runs the full battery green (see
[the tiered compatibility claim](positioning.md#the-tiered-compatibility-claim)).

Set the agent-file knobs in your kit config seams, each to your agent file:

- `GATE_SDK_AGENT_FILE` — the root-tiering allowlist's agent-file entry
  (gate-sdk).
- `LIFECYCLE_KIT_AGENT_FILE` — the lifecycle registration + shim-restatement
  corpus (lifecycle-kit).
- `DOCTRINE_KIT_AGENT_FILE` — the always-loaded doctrine block's host
  (doctrine-kit).
- `CONTEXT_KIT_SURFACES` and `CONTEXT_KIT_BREVITY_FILE` — the measured
  always-loaded surface and the brevity target (context-kit).
- `CANON_KIT_MANIFEST_FILES` — the prose manifest that must govern the agent
  file (canon-kit).

The kit-injected always-loaded blocks (each kit's `<!-- kit:begin -->` /
`<!-- kit:end -->` markers) land in whichever file `CONTEXT_KIT_SURFACES` names,
so they inject into your agent file, not `CLAUDE.md`. The stage skills need no
Claude shim grammar: the skill templates are plain markdown executed by path
(`lifecycle-kit/templates/stages/*.md`), and the `.claude/` shims are one binding
of that mechanism, not the mechanism itself — an `AGENTS.md` harness runs a stage
by invoking its template directly.

Two honest limits:

- **Settings stay Claude-Code-native.** The settings pins, the session-context
  hook wiring, and memory-off enforcement remain Claude Code's — no standard
  cross-harness settings surface exists to port them to. This is the residue
  [the compatibility claim](positioning.md#the-tiered-compatibility-claim) names
  as harness-native.
- **Generated trigger lists carry default literals.** The generated pre-commit
  hook's per-gate trigger lists come from the gates' `# graph:` manifests, which
  carry the default `CLAUDE.md` literal. A nondefault agent file means adjusting
  the affected `# graph:` trigger lines and regenerating the hook, or relying on
  full-battery runs (`bash gate-sdk/bin/run-gates.sh`), which read the knobs and
  are agent-file-agnostic.

## Versioning

The repository carries one semver line, applied as git tags, with the kits
moving in lockstep: a kit earns its own version only if it is ever split out
for independent adoption.

### The release channel

Release channel: **preview**

The two admissible values are `preview` and `stable`. That line is prose a reader
sees and a token a gate reads — read off a line of its own rather than a bracketed
tag, the same one-line-declaration shape queue-kit's `roadmap-summary:` uses and
for the same reason.

The channel is a statement about **audience and support expectations**, not a
second artifact stream. While it reads `preview` the version line is 0.x, and
breaking changes ride minors under the pre-1.0 qualifier below. The tag rhythm
under `preview` is an artifact of internal iteration rather than a stability
signal: the tags are preview-channel iteration artifacts, and a launch
announcement is a separate, later event. The channel flips to `stable` at
`v1.0.0`, the same deliberate cut this section calls the first stability promise —
now with a surface that says so before a reader infers it from tag density.

**Mechanized on two surfaces, on two different tiers.**
`.github/workflows/publish.yml`'s `release` job creates the GitHub Release, and
while the channel is `preview` that creation carries `--prerelease`, so every
Release page states the posture without a human remembering to. That is the
**creating** posture, held by a gate. The **accumulated** posture — the flag on
every Release already published — is held by a monitor instead, because host
state is out of a precommit gate's reach by construction. Neither subsumes the
other: the creating step can be correct forever while an older Release
contradicts it, which is exactly how the drift below went unnoticed.

Nothing else in the publish path changes. In particular the npm
publish carries no `--tag`, and its absence is **load-bearing configuration**
rather than an omission: a non-default dist-tag would make §Quick start's
one-command install resolve to nothing until a reader learned to append the
channel, trading an honest signal for a time-to-first-value regression on the
front door. That trade was put to the operator in those terms and declined. It
becomes the right call only once §Quick start stops promising a bare one-command
install, or once a stable line exists to hold `latest` while preview moves off it.
Neither holds today. Nothing forecloses the change either:
`scripts/pack-installer.sh`'s version regex already admits a prerelease suffix.

**The same preference decides invariant C the other way, which is why the two
sit side by side.** Both trades weigh an honest channel signal against a reader
who follows a default pointer, and the dist-tag lost because `npm install`'s
default tag *is* the documented front door — §Quick start resolves it. No
documented install path resolves the GitHub Latest pointer at all: §Quick start
runs the installer, and every explicit download URL below names its version. The
front door is not on that path, so the cost that declined the dist-tag change is
absent here. Same preference, opposite outcome. That is not a reversal; the
dist-tag's own re-entry condition above is untouched.

Three invariants hold the declaration, so it cannot become a comment. The first
two are gate-held and read files off disk; the third is monitor-held and reads
the host, and it is stated after the gate's own contract below rather than
inside it. `check-release-channel-parity` (this repo's `scripts/`) owns A and B:

- **Invariant A — channel ⟷ publish posture.** The channel and the prerelease
  posture of the Release-creating step in `.github/workflows/publish.yml` agree:
  `preview` demands `--prerelease` on that invocation, `stable` demands its
  absence.
- **Invariant B — channel ⟷ version line.** The channel agrees with the version
  line, read from the newest tag by creator date: while that version is `0.x` the
  channel must be `preview`; from `v1.0.0` onward it must be `stable`.

B is not optional polish. The channel is *derived* from the version line, so
without B a `v1.0.0` could ship with `preview` declared and `--prerelease` still
set — A passes because both of its surfaces agree, and both are wrong. B also
protects a second consumer: lifecycle-kit's knob-rename compat precedent reads
this declaration as its general-availability threshold, so a stale `preview` would
hold that compat window open past GA.

The gate fails closed rather than passing whenever it cannot find a surface it
needs. Exit 2, never a clean line, on each of:

- a declaration line that is missing or duplicated;
- a channel value it does not recognize;
- a `publish.yml` with no recognizable Release-creating step;
- a newest tag that does not parse as semver.

A repository with **no tags at all** has no version line, so B is dormant there
while A still asserts. The gate *reports* that dormancy in its clean output, so a
reader can tell "checked and agreeing" from "nothing to check".

- **Invariant C — channel ⟷ published Release history.** Every published
  Release's prerelease flag agrees with the channel its own version line
  implies: a `0.x` tag carries the flag, a `1.x`-or-later tag does not.

C is A and B composed rather than a new policy. B already rules that a version
line implies a channel; A already rules that `preview` implies the flag. The flag
on *any* Release therefore follows from that Release's own tag. B only ever got
checked against the newest tag because the gate had one version line to read;
C is the same statement evaluated over the whole published history. That makes
the desired host state **derived rather than stored** — there is no tag-to-flag
roster to maintain anywhere, and the rule keeps working across the `v1.0.0` flip
with no edit, because at that flip the newest tag stops being `0.x` on its own.

**C is held by a monitor, not by the battery** — the release-channel arm of
`.github/workflows/site-health.yml`, which reads the Release list from the API
on its schedule and files a `site-health` issue naming every Release whose flag
disagrees with its own version line. A precommit gate cannot reach host state,
and weakening the tier to reach it is the one change this design refuses; this
is the same tier ruling `RELEASING.md` step 6 already makes for the Release
body, over the same objects.

**What this means for a reader, stated because it is what you see.** Every
release this project has published is `0.x`, so under C **no Release is
Latest**: the repo front page shows no green Latest badge and the releases list
labels every entry `Pre-release`. Resolving the pointer itself yields no release
— the API's `releases/latest` endpoint answers 404, while the browser URL
redirects to the releases list. That is the honest presentation of a channel
whose tags are preview-channel iteration artifacts. **Take a release from the
releases list, or name an explicit version — never resolve the Latest
pointer**, which advertises no release at all while the line is `0.x`. Nothing
documented here depends on it: §Quick start runs the installer and every
download URL below carries its own version.

What earns each bump derives from the release note itself — its
declaration-bearing sections (§The upgrade contract below) already declare
everything phase B must reconcile, so the floor is read off the note rather than
maintained beside it:

- **Patch** — every declaration-bearing section is "None": a phase-A-only sync,
  fixes and docs that tighten nothing a consumer must reconcile.
- **Minor** — any section is non-empty: the release carries phase-B work —
  a new or stricter gate, a knob rename riding its deprecation path, or a
  behavior change a consumer's tree may depend on (blind-upgrade-safe is exactly
  what such a change breaks, the same reasoning that floors the other two).
- **Major** — a decommission: a release that *removes* a deprecated surface
  (a release-sweep disposition executed as decommission), or any change the
  two-phase upgrade contract cannot reconcile from the note alone. This
  criterion is absolute: a decommission earns a major even while the line is
  0.x, outranking the pre-1.0 qualifier below rather than riding a minor.
  Majors are where the deprecation promises come due — the release-sweep
  constraint that no marker rides into the next major undispositioned binds
  here.
- **Pre-1.0 qualifier** — while the line is 0.x, breaking changes *other than
  decommissions* may ride minors (the semver 0.x convention), each still
  declared in the note; a decommission still earns a major (above), and that
  is what keeps release-sweep's no-marker-rides-past-the-major constraint
  anchored while 0.x. `v1.0.0` is the first stability promise and is cut
  deliberately, never earned mechanically.

The floor has a **second input**: a note also inherits the floor of any
outstanding deferred release. When an iteration's criteria were met but the
release was held back, its disposition line records the earned version as
`deferred:vX.Y.Z` (lifecycle-kit/SPEC.md §templates/stages/), and
those criteria stay unconsumed until a release at or above that version ships.
The next qualifying note carries them in its declaration-bearing sections and may not fall
below that version — so a note's floor is the higher of what its own sections
derive and what an outstanding deferral carries. `check-release-bump` reads both.

The derivable half is gated: `check-release-bump` (this repo's `scripts/`)
orders the release notes by version and reds a patch-only bump whose note
declares tightened gates, renamed knobs, or behavior changes (and fails closed
if any fixed section is absent). That presence assertion binds the **newest**
note only, so adding a fixed section to the roster above costs no historical
backfill across the published corpus. The major criteria stay judgment —
a decommission is a semantic fact no section grammar carries — so the gate
holds only the floor.

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

Two shipped tools carry this contract. [The upgrade smoke](gate-sdk/SPEC.md#upgrade-smoke)
is its executable proof — it drives both phases against a scratch consumer,
asserting the phase-A sync is deterministic and the red set stays within `TO`'s
tightened-gates declaration — the target note's when `TO` is a tag, and the
declaration surface in `TO`'s own tree when it is
not. [The upgrade skill](lifecycle-kit/SPEC.md#templatesupgrademd)
is the phase-B disposition ritual a consumer runs to register the note's newly
declared gates and disposition each red.

Release notes are dated posts under `docs/posts/`. Each carries a
`release: vX.Y.Z` key in its front matter — the key that resolves a version to
its note — and these sections under fixed names. This list is the roster: **In
brief** is the human read, and the sections after it are the
**declaration-bearing** ones a mechanical consumer reconciles. Prose elsewhere
names those classes rather than counting their members, so adding a section
here does not silently falsify a sentence somewhere else.

- **In brief** — placed first, immediately after the opener and ahead of
  Tightened gates. Three to five bullets of plain language, each answering *what
  you get* or *whether you must act*. A bullet lead here is a plain phrase, never
  a gate or knob name. The declaration-bearing sections below already carry those
  tokens; this section exists to be readable without them. **In brief has no
  "None" form**, unlike every section below it. A release with nothing worth
  saying to a human is a patch, and says that in one bullet. It bears no
  declaration. It feeds no bump criterion and no allowed-red set, and nothing
  reads it but the human upgrader. Its position ahead of the migration detail is
  deliberate: the opener's one-sentence slot is a lede, and in practice it
  summarizes the engineering instead of answering whether the reader must act.

  What holds it is `check-release-bump`'s presence assertion, and that assertion
  **binds a note under composition** — the newest note whose declared version
  carries no tag yet. A note published before this section existed is history. It
  is not retro-fitted with a fabricated summary, and the assertion goes dormant
  on it, reporting which state it is in rather than going quiet. The
  predicate is `check-tightened-gates-note-parity`'s, adopted so that release
  state is read one way across the corpus. Its residual is that sibling's too: a
  note authored and drained inside a single commit is never seen under
  composition, so what carries the section into existence is the split
  choreography the release runbook prescribes, whose chrome skeleton holds its
  slot.
- **Tightened gates** — one bullet per gate that landed new or got stricter, the
  gate name the bullet's lead token. A mechanical consumer reads these lead
  tokens as the release's allowed-red set: the gates a clean upgrade may turn
  red, each named here with the intent behind the move. The token has one
  canonical spelling. It is a **backticked, unbolded** bare gate name, directly
  after the bullet marker (``- `check-foo` ``, then the prose). Backticks because
  the token is a code identifier and this tree spells identifiers in backticks;
  unbolded because bold is a rendering choice carrying no semantics, and it
  collides with the one token a machine reads. So the section resolves to an
  explicit empty set (a "None" body) or to a non-empty token set. Nothing else.
  A non-"None" section that yields no token is a **defect in the release** rather
  than a declaration of nothing: the parse would otherwise compile a note naming
  several gates into an empty allowed-red set, passing vacuously on a green
  battery and failing with a false message on a red one.
  `check-tightened-gates-grammar` (this repo's `scripts/`) holds that over the
  whole corpus, with no version cutoff — `GATE_SDK_UPGRADE_FROM` and
  `GATE_SDK_UPGRADE_TO` make any historical pair a supported run, so any note may
  be the one the smoke resolves. Registry membership is deliberately not
  asserted. A gate renamed or retired since a note shipped would make membership
  false about history without the record being wrong.

  **A sibling gate makes a separate claim, and neither is the other's coverage.**
  The grammar gate holds each note's section *well-formed*.
  `check-tightened-gates-note-parity` (also this repo's `scripts/`) holds a note
  *under composition* **equal to the declaration surface it was composed from**,
  as set equality in both directions — both, because each direction costs
  something different: a name on the surface and missing from the note is a gate
  that tightened and shipped undeclared, licensing a red the upgrade smoke would
  wave through, while a name in the note and missing from the surface declares a
  gate that never tightened and sends consumers hunting a reconcile that does not
  exist. It arms on a note whose declared version carries no tag yet and disarms
  once tagged, reporting its dormancy rather than letting a drained surface read
  as verification. It rides `gate-sdk/lib/declaration.sh`'s parsers, never a
  private one. That is what stops it passing while the smoke reads a different
  token set from the same bytes.

  **A declaration precedes its release.** Because the upgrade smoke's untagged
  arm reads a working tree's tightened-gates declaration surface rather than a
  note (gate-sdk/SPEC.md §upgrade-smoke), an allowed-red set is owed from the
  moment a gate is landed or tightened, not from the tag. Running the smoke
  against your checkwright clone at an untagged `TO` is what reads it.
- **Renamed knobs** — one bullet per rename, `old → new`; a knob *removal* is
  the same residue class (own-config orphaned) and is expressed `old → ∅`.
- **Behavior changes** — one bullet per shipped change that alters what the kits
  *do* without landing or tightening a battery gate: a fail-closed convergence
  in a shared library, a runner's semantics, a skill or template behavior, a
  default's effect. The bullet's lead token is the changed surface's name (the
  script, knob, template, or file), bolded; the rest states what moved and what,
  if anything, the consumer reconciles. This section keeps its own spelling — its
  lead tokens are legitimately prose phrases rather than identifiers, so the
  Tightened-gates rule above does not reach it.

"None" is a valid body for any of the three sections and must be stated, not
omitted — a release that tightens nothing, renames nothing, and changes no
out-of-gate behavior says so on each. Its form is a bare "None."; add a
trailing clause only where it rules out a near-miss the reader would otherwise
mis-classify (an advisory KPI that never joins the gate registry, knobs added
but not renamed). A clause that only restates the heading's own negation is a
restatement to delete. This consumer-owned residue Phase A
cannot touch (gates you have shadowed, templates you have copied out, knob
renames in your own config, behavior your tree depends on) is that note's
checklist.

Those four residue classes map onto the three sections by design: **shadowed
gates** → Tightened gates; **own-config knob renames and removals** → Renamed
knobs; **copied-out templates and depended-on behavior** → Behavior changes. The
copied-out-template class earns no section of its own because a template you have
copied out that then changed *is* depended-on behavior diverging from your copy —
it is behavior-folded, not dropped. Four classes, three sections, by that
folding.

Honest limit on Behavior changes: **its bullets are declared for
the human upgrader, not smoke-asserted.** A non-gate change cannot red the
battery, so the upgrade smoke's containment assertion (defined over battery
reds) does not read this section — it fixes where such changes are stated and
`check-release-bump` fixes that they are stated, but neither makes them
executable. Tightened-gates bullets stay the mechanical allowed-red set;
behavior-changes bullets are reconciled by reading.

## Branch protection

The pre-commit hook is a local backstop a contributor can bypass. Server-side
enforcement makes the battery a required status check: run the gate battery in
CI on every pull request, and mark that check required in your host's
branch-protection settings so a red battery blocks the merge. Keep the
verifier neutral — enforcement that an author can edit is not enforcement.

Back to the [kit map](index.md#the-kits) or [why Checkwright](methodology.md).
