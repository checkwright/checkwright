---
title: Install
nav_order: 3
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

## Requirements

Checkwright is **Unix-first**: Linux and macOS are the supported platforms.
Windows is supported through WSL (Windows Subsystem for Linux), not natively —
the gate battery and the git hooks are Bash over a coreutils toolchain, and no
native-Windows shell path exists.

The battery leans on a small command-line toolchain; each tool below must be on
your `PATH`, and the note says what breaks without it:

<!-- toolchain:begin -->

- `bash` — every gate and both generated git hooks are Bash scripts; nothing in
  the battery runs without it.
- `git` — the gates read tracked files and the hooks fire at commit time; the
  model is git-native end to end.
- `jq` — the settings and evidence gates, and guard-kit's JSON tooling, parse
  their inputs with it.
- `awk` — the gate family's line scanning and field extraction are written in
  awk; most checks cannot run without it.
- `shellcheck` — the `check-shellcheck` meta-gate lints every shipped script,
  and a lint finding blocks the commit.

<!-- toolchain:end -->

No minimum versions are pinned here: the toolchain moves with your platform, and
a version floor baked into this page would rot. To see exactly what your machine
carries, seed a local profile with context-kit's env-probe —
`bash context-kit/bin/env-probe.sh` writes an `ENV.local.md` you keep untracked.

Publishing a docs site is an optional wider tier. A consumer that registers
site-kit's render-fidelity gate — which re-renders every page through the
GitHub Pages parser — additionally needs Ruby with the `kramdown-parser-gfm`
gem. A consumer that publishes no docs site never installs it.

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
out-of-gate behavior says so on each. This consumer-owned residue Phase A
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
