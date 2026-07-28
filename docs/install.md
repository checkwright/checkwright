---
title: Install
nav_order: 3
---

# Install and upgrade

Checkwright is distributed git-native and vendored-committed: the kit
directories live in your repository as committed source. The gates read tracked
files, and the audit story wants the governance layer inside the reviewed tree.

The two registries stand in different relations to that, and the distinction is
worth drawing rather than blurring:

- **crates.io holds the name only, never a dependency channel.** There is
  nothing to `cargo add`.
- **npm carries a real package, and it is an installer rather than a
  dependency.** `npx checkwright init` copies pinned kit source out of its own
  payload into your tree and commits it.
- **The same payload downloads straight off the GitHub Release**, as a tarball
  and its `.sha256`, attached to every tagged release. This strengthens the
  doctrine rather than weakening it: a downloaded, checksummed, extracted
  tarball you read before running it is the most auditable form of the same
  one-shot vendoring.

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
committed, auditable source you read before you run it. Both properties are
asserted rather than claimed — the consumer smoke installs from a packed
tarball with no registry access and runs the whole path from it, and
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

- `bash` (≥ 4.0) — every gate and both generated git hooks are Bash scripts;
  nothing in the battery runs without it. Three constructs force the floor:
  associative arrays, `mapfile`, and the lowercasing case expansion, spread
  across the gate libraries, the checks, and canon-kit's prose tooling.
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

<!-- toolchain:end -->

A member is pinned only where a construct the battery actually runs forces the
pin, and each pinned member names that construct above. A floor nobody's code
forces is an aspiration, and an aspiration is what rots; the rule is what keeps
this list honest, not a promise to revisit it.

Nor are the bullets maintained beside the code. The roster lives in
`context-kit/lib/toolfloor.sh`; this block renders it, and
`check-install-toolchain` holds the two in whole-element parity — floor and
implementation token included — so the page cannot drift from what the gates
require.

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

Publishing a docs site is an optional wider tier. A consumer that registers
site-kit's render-fidelity gate — which re-renders every page through the
GitHub Pages parser — additionally needs Ruby with the `kramdown-parser-gfm`
gem. A consumer that publishes no docs site never installs it.

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
`curl … | sh` one-liner would have this page contradicting its own opening
claim: what governs your tree is meant to be source you read before you run it,
and an unreviewed remote script fed straight to a shell is that claim's
counter-pattern.

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

You pick how much to meet first. `starter` is the framework — the gate SDK on
its own. `delegation` adds the kits whose subject is the agent session itself.
`full` is everything. The progression is a containment chain, so moving up only
ever adds.

Re-running is idempotent and non-destructive: it reads the per-file hash
recorded in `checkwright.lock`, rewrites what still matches, and **reports
rather than overwrites** anything you have changed since, unless you pass
`--force`. `--dry-run` prints the file plan and the manifest and writes nothing.

`checkwright.lock` is the install-ownership record — what was installed, from
which upstream commit, and which files the installer owns. That commit field is
what lets a reviewer resolve the tree in front of them to an exact upstream
state, which is the difference between vendored source that is merely committed
and vendored source that is auditable.

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
[stage-skill modes](lifecycle-kit/SPEC.md#templatesskills).

Start with [gate-sdk](gate-sdk/index.md) — the other kits register into its
runner — then add kits in the order the [kit map](index.md#the-kits) lists them.

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
(`lifecycle-kit/templates/skills/*.md`), and the `.claude/` shims are one binding
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
for independent adoption. The first tag rides the launch announcement.

What earns each bump derives from the release note itself — its three fixed
sections (§The upgrade contract below) already declare everything phase B must
reconcile, so the floor is read off the note rather than maintained beside it:

- **Patch** — all three note sections are "None": a phase-A-only sync, fixes and
  docs that tighten nothing a consumer must reconcile.
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
`deferred:vX.Y.Z` (lifecycle-kit/SPEC.md §templates/skills/), and
those criteria stay unconsumed until a release at or above that version ships.
The next qualifying note carries them in its three sections and may not fall
below that version — so a note's floor is the higher of what its own sections
derive and what an outstanding deferral carries. `check-release-bump` reads both.

The derivable half is gated: `check-release-bump` (this repo's `scripts/`)
orders the release notes by version and reds a patch-only bump whose note
declares tightened gates, renamed knobs, or behavior changes (and fails closed
if any of the three fixed sections is absent). The major criteria stay judgment —
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
asserting the phase-A sync is deterministic and the red set stays within the
target note's declaration. [The upgrade skill](lifecycle-kit/SPEC.md#templatesskills)
is the phase-B disposition ritual a consumer runs to register the note's newly
declared gates and disposition each red.

Release notes are dated posts under `docs/posts/`. Each carries a
`release: vX.Y.Z` key in its front matter — the key that resolves a version to
its note — and three sections under fixed names:

- **Tightened gates** — one bullet per gate that landed new or got stricter, the
  gate name the bullet's lead token. A mechanical consumer reads these lead
  tokens as the release's allowed-red set: the gates a clean upgrade may turn
  red, each named here with the intent behind the move.
- **Renamed knobs** — one bullet per rename, `old → new`; a knob *removal* is
  the same residue class (own-config orphaned) and is expressed `old → ∅`.
- **Behavior changes** — one bullet per shipped change that alters what the kits
  *do* without landing or tightening a battery gate: a fail-closed convergence
  in a shared library, a runner's semantics, a skill or template behavior, a
  default's effect. The bullet's lead token is the changed surface's name (the
  script, knob, template, or file), bolded like the other sections' lead tokens;
  the rest states what moved and what, if anything, the consumer reconciles.

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

Honest limit on the third section: **Behavior-changes bullets are declared for
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
