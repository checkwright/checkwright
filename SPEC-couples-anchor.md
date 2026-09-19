# SPEC amendment: couples-anchor

A `kit:<glob>` couples token is expanded by `registry::expand_couples` against a
list of kit roots its caller passes, and the callers pass two different
spellings. `native/src/walk.rs` holds both:

- `kit_roots()` (`:301-306`) spells each root relative to the working directory,
  which every door sets to the repository toplevel.
- `kit_roots_rel()` (`:308-324`) spells it relative to the gate-sdk root's parent.

The two agree when the kits are vendored at the repository root, which is where
`init` vendors them and where every fixture and test in the tree puts them. They
part when an adopter hand-vendors under a subdirectory, e.g.
`GATE_SDK_ROOT=tools/gate-sdk`. Then `kit_roots()` spells `tools/canon-kit` and
`kit_roots_rel()` spells `canon-kit`.

The hook emitter anchors at the toplevel (`native/src/emit/git_hooks.rs:105-106`),
because its globs match staged paths, which git spells repo-relative
(gate-sdk/SPEC.md §gen-pre-commit, *Check dirs*). `--for`'s selector passes the
other spelling (`native/src/runner.rs:747-754`) and matches the result against the
paths its caller gave, which are repo-relative too. So under a subdirectory
vendoring, `--for tools/canon-kit/SPEC.md` selects nothing the hook would run.
gate-sdk/SPEC.md §run-gates already rules that a bug: "a divergence between what
the hook would run for a staged path and what `--for` runs for the same path is a
bug against this contract."

**The rule this amendment states is the one the survey found the readers already
split along.** An expanded glob or a spelled root is either matched against a
path read from the tree, or matched against text that spells kits from their
common parent. The first kind takes the repository spelling; the second keeps the
kit-parent spelling. Every caller was read and classed:

| reader | matched against | class |
|---|---|---|
| `runner.rs:747-754` (`--for`) | the caller's paths, at the toplevel | repository |
| `gates/reads_couples.rs:459,485` | `git ls-files` output (`:387-399`) | repository |
| `emit/port_blockers.rs:215` | check dirs resolved off disk (`:207-209` states they are repo-relative) | repository |
| `gates/gate_substrate_parity.rs:451` | `{root}/checks` dirs resolved off disk | repository |
| `gates/core_files.rs:62` | the `kit:` lines of the core-files manifest, each tested on disk and with `git ls-files` (`:102-106`) | repository |
| `emit/git_hooks.rs:105-106` | staged paths | repository, already |
| `gates/graph.rs:517-519` | `scripts/graph-vocab.knobs` layer rules (`canon-kit/:k_canon`) | kit-parent, correct |
| `emit/graph.rs:138,177` | nothing: node and edge labels in the graph artifact | display |
| `emit/port_blockers.rs:612` | nothing: a printed `couples=` line | display |

The five repository-class readers passing the kit-parent spelling are the defect.
`gate_substrate_parity.rs` matches its expansion (`:681`) against declarations it
resolved through the same wrong spelling (`:506`), so its pairing is
self-consistent and only its directory resolution is exposed.

It is a gate-sdk amendment: `walk`, the runner, the emitters and the three gates
are gate-sdk's.

The tree does not already do this. The four readers call `kit_roots_rel`, and no
test builds a `GATE_SDK_ROOT` under a subdirectory: the fixtures at
`native/src/registry.rs:534` and `native/src/gates/reads_couples.rs:631` both put
`gate-sdk` directly under the synthetic repository.

## What changes

**Batching.** Deltas 1 and 2 land in one commit, because delta 2's test is the
proof of delta 1.

### (1) A repository-matching reader takes the repository spelling {design-bearing}

**Not yet applied.** `--for`'s selector, `check-reads-couples`,
`check-core-files`' `kit:` expansion, the port-blockers check-dir resolution and
`check-gate-substrate-parity`'s check-dir resolution read `walk::kit_roots()`. `check-graph`'s vocabulary match keeps `kit_roots_rel()`,
because the vocabulary it matches is kit-parent prose.

The display readers take the repository spelling too, so the graph artifact names
the globs the hook actually matches. Nothing reads their output as a path, so the
change is safe either way, and one spelling across the expansion's path-matching
and display readers leaves the kit-parent spelling with prose readers alone.

`walk::kit_roots_rel`'s `# spec:` comment names the anchor "the couples globs
share". That stops being true. It is rewritten to name its remaining readers: the
ones matching kit-parent-relative text.

**Point 5, each narrowed or moved reader's red condition.** At the root layout the
two spellings are identical, so every reader's verdict on this tree and every
fixture is byte-unchanged. Under a subdirectory:

- `--for` selects the members the hook runs, where it selected none. It is not a
  gate and has no red.
- `check-reads-couples` reds when a read root has no covering couples glob. Its
  globs now match the `git ls-files` corpus they are compared with, so the change
  removes false reds and adds none.
- `check-core-files` reds on a manifest path that does not exist or is untracked.
  A `kit:` line now spells the path that exists, so the change removes false reds.
- The two resolution readers find check dirs that exist, where they found none.
  Neither reds on *finding none*, and a found member can only add evidence.

### (2) A subdirectory vendoring is tested {mechanical}

**Not yet applied.** A crate unit test builds a scratch repository with
`tools/gate-sdk` and a sibling `tools/alpha-kit` carrying a gate whose couples is
`kit:SPEC.md`. It asserts three things:

- The pre-commit emission's glob for that member is `tools/alpha-kit/SPEC.md`.
- `--for tools/alpha-kit/SPEC.md` selects that member.
- `check-reads-couples` resolves the member's covering glob against the scratch
  tree's `git ls-files`.

This is the fixture the filing said no suite carries. It is a unit test rather
than a `good/`+`bad/` pair because no gate's verdict changes, and the selector it
proves is not a gate.

## Producers and consumers

- **The spelling each reader passes (delta 1).** Producer: `walk::kit_roots` and
  `walk::kit_roots_rel`, both unchanged. Consumers: the eight readers in the table
  above, each named with what it matches against.
- **Point 2, rosters on the surface.** No roster reds on a spelling, since neither
  function mints a name.
- **Out of this unit, and filed.** Other `kit_roots_rel` callers resolve paths off
  disk or through a git pathspec and are not couples-expansion readers:
  `emit/enum_sets.rs:79`, `emit/close_surfaces.rs:147`,
  `gates/gate_binary_fresh.rs:40`, `gates/install_platforms.rs:206`,
  `gates/knob_default_coupling.rs:360`, `emit/pack_installer.rs:196,295`,
  `gates/kit_enum.rs:123`, and evidence-kit's derived suite default
  (`knobs/evidence_kit.rs:44-49`). They carry the same latent mismatch under a
  subdirectory vendoring. That class is filed to the gap inbox at this stage,
  costed, rather than folded in.

## Existing sections updated

Rosters produced by `git grep -n "kit_roots_rel\|expand_couples" -- native/src`,
each caller read for what it matches.

- `gate-sdk/SPEC.md` §gen-pre-commit, *Check dirs*: the sentence contrasting the
  hook's anchor with "the spelling the other expansion readers pass" is rewritten.
  Every reader matching a tree path now passes the hook's spelling (delta 1).
- `gate-sdk/SPEC.md` §run-gates: the divergence sentence gains the anchor. The
  hook and `--for` expand `kit:` against one spelling of the kit roots (delta 1).
- `gate-sdk/SPEC.md` §Layout and configuration, where the three kit-root
  spellings are described (`:339-341`, "the anchor the couples globs share"): the
  couples globs take the working-directory spelling, and the kit-parent spelling
  is named as the prose matchers' (delta 1).
- `gate-sdk/SPEC.md` §check-core-files: "`walk::kit_roots_rel` is the one
  derivation of the roots" names `walk::kit_roots`, the one spelling this reader
  takes. The root set is still derived in one place (delta 1).
- `native/src/walk.rs`, `native/src/runner.rs`, `native/src/emit/graph.rs`,
  `native/src/emit/port_blockers.rs`, `native/src/gates/reads_couples.rs`,
  `native/src/gates/core_files.rs`, `native/src/gates/gate_substrate_parity.rs`
  (delta 1), and the new unit test (delta 2).
<!-- update-target-exempt: generated mirror of the kit SPEC, regenerated by its freshness gate's printed command, never hand-edited -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — no name is added or removed; four readers switch between two existing
  functions.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `kit-token-anchor-hook-for-divergence` moves to Done in the
      merge commit, a stage before the drain stage.
- [ ] **Gaps filed** — the non-couples filesystem readers are filed at spec; any
      cross-component gap build discovers is resolved that session.
