# SPEC amendment: root-dialect

The couples-anchor amendment moved the `kit:` couples readers that match
repository paths onto `walk::kit_roots()`. The entry
`kit-roots-rel-filesystem-readers` names nine remaining `kit_roots_rel` callers
and carries one explicit `**Inferred, not run:**` — *that each listed reader
misreads under a nested `GATE_SDK_ROOT`; no fixture nests it, so none was run
that way.*

**That inference is now run, and it is confirmed, widened and re-shaped.** Probed
2026-09-20 against a scratch git repo with all eleven kits copied under `vendor/`
and `scripts/` at the root, run as `GATE_SDK_ROOT=vendor/gate-sdk
GATE_SDK_GATES_DIR=scripts` against the release binary, each arm and gate paired
against the same command in this unnested tree.

## The mechanism, and why no fixture could have caught it

Read off `native/src/walk.rs:301-332`:

- `kit_roots()` spells each root **relative to the working directory**, which
  every door sets to the git toplevel. Nested, it returns `vendor/gate-sdk`,
  `vendor/canon-kit`, …
- `kit_roots_rel()` spells each root **relative to the gate-sdk root's parent**
  (`kit_roots_rel_from` takes `anchor = sdk.rsplit_once('/').0`). Nested, it
  returns the bare basenames `gate-sdk`, `canon-kit`, …
- **Unnested the two are byte-identical**, because the anchor *is* the working
  directory. Measured: `--emit kit-roots` prints `gate-sdk … site-kit` here and
  `vendor/gate-sdk … vendor/site-kit` there.

So the discriminator is exact, and it is the one this amendment states as a
rule: **a site that joins a root onto a filesystem path or a git pathspec takes
`kit_roots()`; a site that matches text naming kits from their common parent
keeps `kit_roots_rel()`.** No fixture nests the root, and unnested the wrong
choice is indistinguishable from the right one — which is why nine sites drifted
and why this amendment's last delta is a fixture that nests.

## What the probe measured

**Silently wrong — green, with a corpus that shrank to nothing.** This is the
class that matters, and the entry did not know it existed:

- `emit/enum_sets.rs:79` — nested, `--emit enum-sets` emits **zero** kit-derived
  sets (no `gate-sdk-lib`, no `*-gate-test`) and exits **0**. Its own `# spec:`
  comment at `:89-91` says the derivation *"fail-closes rather than emitting the
  silently empty set"*; the `Path::new(&lib).is_dir()` guard at `:93` skips every
  kit before the fail-closed branch at `:96` can fire, so it emits precisely the
  set the comment refuses.
- `emit/close_surfaces.rs:147` — nested, `--emit close-surfaces` emits **1** row
  against **12** unnested. Every kit-roster-sourced surface vanishes. Exit 0.
- `knobs/evidence_kit.rs:50`, through `registry::fixture_suites_in` — the derived
  `EVIDENCE_KIT_SUITES` default **changes identity**: `canon_kit, context_kit,
  delegation_kit, …` unnested; `gates, scripts, guard_tests, demo,
  installer_smoke, consumer_smoke, upgrade, agents_md_smoke` nested. Not one kit
  suite survives, and exit is 0.
- `gates/kit_registration.rs:92` — clean line goes from *"11 kit root(s) … 10
  shipping gate-tests each name a fixture-runner line"* to *"… 0 shipping
  gate-tests"*, and **PASSes** both ways.
- `gates/knob_citation.rs:22` — clean line goes from *"93 manifest file(s)"* to
  *"3 manifest file(s)"*, and **PASSes** both ways.

**Loudly wrong — fail-closed, diagnosable:** `gates/knob_default_coupling.rs`'s
second loop errors `cannot read directory gate-sdk: No such file or directory`
and exits 2.

**Narrowed rather than emptied:** `check-gate-substrate-parity`'s nested clean
line independently shows the `{root}/checks` resolve-set narrowing that
`gates/gate_binary_fresh.rs:40` and `gates/install_platforms.rs:206` carry —
*"105 in scope, 16 out of scope"* against *"121 in scope, 0 out of scope"*.

## The two findings that re-shape the unit

**1. The caller set is 19, not 9.** `git grep -n kit_roots_rel native/src` returns
nineteen call sites. Five the entry does not name are measurably affected —
`kit_registration.rs:92`, `knob_citation.rs:22`, `docs_cmd.rs:50`,
`kit_ref_liveness.rs:159`, and `registry.rs:104` through `fixture_suites`. Four
more the entry never examines — `gate_tamper.rs:194`, `shim_restatement.rs:106`,
`emit/graph.rs:143`, and `gates/kit_enum.rs:45`. The last of those is **dormant**
rather than measurably wrong: this repo's `gates.list` carries no multi-kit
hand-listed `couples=` group large enough to reach its git-pathspec half, so the
nested probe's clean line reads the same *"0 multi-kit hand-list group(s)
complete"* both ways rather than a shrunk one — the defect follows from delta 1's
rule, not from a run that caught it, which is exactly the residue a static read
against a stated rule exists to close where a probe cannot. The entry's roster is
a subset, and its `[cost: event/low]` was priced against it.

**2. The call is per-USE, not per-site.** The entry frames the work as *"a
per-site call on whether it matches repository paths or kit-parent text"*.
Measured, at least five sites take **both** off one `kit_roots_rel()` call, so
moving a site wholesale would break its text half:

- `gates/kit_registration.rs` — `:132` `registry_text.contains("]({root}/")` is a
  **text** match against a README markdown link; `:137` `format!("{}/gate-tests/",
  r)` is a **git pathspec**. One call, two dialects. This single site is why the
  nested clean line reads *0 shipping gate-tests* while assertion A still passes.
- `gates/knob_citation.rs:20-36` and `gates/knob_default_coupling.rs:21-35`
  (`prefix_pairs`) — each derives a SCREAMING_SNAKE prefix from the **basename**,
  which is correct in either dialect, while carrying `kr` itself as the pair's
  path element, which is not.
- `gates/docs_cmd.rs:50-58` — pushes `root` into `roots` (a path, consumed by
  `defined_knobs`) and a basename-derived prefix, in one loop.
- `gates/kit_enum.rs` — `:107`'s `kit_roots.iter().any(|r| r == root)` matches a
  `couples=` hand-listed root name against the enumerated set, a **text** match;
  `:124`'s `format!("{}/{}", r, glob)`, fed straight to `git ls-files`, is a **git
  pathspec**. One call, two dialects, the same shape as `kit_registration.rs`
  above — and the same silent direction: an empty `git ls-files` result is read
  as *this root doesn't carry the group* (`:139`), not as a violation, so a
  nested mismatch would make every root's pathspec return empty and the check
  would pass with nothing checked, the way `kit_registration.rs` does today.
  Currently dormant only because no live `couples=` group is large enough to
  reach the loop body.

So the delta shape is **split the mixed reads, then re-point only the path use** —
not *re-point the caller*.

It is a root-level amendment because it spans `native/` (the nineteen sites and
the fixture) and gate-sdk (§The path-dialect contract and §lib/gate.sh, which own
the two spellings' documented meanings).

**The tree does not already do this.** The probe above is the evidence; every
listed site reads `kit_roots_rel` at HEAD.

## What changes

**Batching.** Delta 1 lands alone — it is the rule every later delta cites.
Deltas 2 and 3 land in one commit: a split caller and its re-pointed half are one
edit. Delta 4 lands last and is the only thing that can prove 2 and 3, so it does
not land before them.

### (1) The path-dialect contract names which spelling each kind of reader takes {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md §The path-dialect contract states the rule
the two functions' `# spec:` comments today only imply:

> A root joined onto a path that is opened, statted, walked, or handed to git as
> a pathspec takes `kit_roots()`, the working-directory spelling, because every
> door sets the working directory to the repository toplevel. A root matched
> against text that names kits from their common parent — a couples field, a
> markdown link target, a knob prefix — takes `kit_roots_rel()`. The two are
> identical when the gate-sdk root is a direct child of the toplevel, which is
> every fixture and this repository, so the choice is unobservable there and must
> be made from the reader's kind rather than from a passing run.

`walk::kit_roots()` and `walk::kit_roots_rel()`'s own `# spec:` comments are
re-phrased to point at that rule rather than each restating half of it.

**Why a stated rule and not nineteen corrected call sites alone.** Enforcement-
first: the correction without the rule is nineteen edits and no reason the
twentieth reader chooses right. The rule is what delta 4 makes executable.

### (2) The mixed callers are split {design-bearing}

**Not yet applied.** Each of the five sites that takes both dialects off one call
is split so that each dialect has its own binding, and neither is derived from the
other:

- **`gates/kit_registration.rs`** — assertion A's `](<root>/` match keeps the
  rel spelling; assertion B's `<root>/gate-tests/` pathspec takes `kit_roots()`.
  The two lists are index-aligned by construction, since both derive from the same
  root order.
- **`gates/knob_citation.rs`** and **`gates/knob_default_coupling.rs`**'s
  `prefix_pairs` — the prefix stays derived from the basename, which is
  dialect-free, and the pair's path element takes `kit_roots()`.
- **`gates/docs_cmd.rs:50-58`** — `roots` takes `kit_roots()`; `prefixes` stays
  basename-derived.

**A basename is not a third dialect.** Every one of these reads
`r.rsplit('/').next()`, which returns the same token from either spelling. Stated
so the split does not look like it has three cases when it has two.

### (3) Every path-consuming reader takes the working-directory spelling {mechanical}

**Not yet applied.** The remaining sites whose whole use is a filesystem path or a
git pathspec move to `kit_roots()`:

`emit/enum_sets.rs:79`; `emit/close_surfaces.rs:147`;
`emit/pack_installer.rs:196` (its `is_dir` test and `pack_tracked` pathspec) and
`:295` (`footprint`'s pathspec); `registry.rs:86` `fixture_suites_in`, whose
`{base}/gate-tests` and `{base}/checks` `is_dir` tests are the path use, reached
from `registry.rs:104` and `knobs/evidence_kit.rs:50`;
`gates/knob_default_coupling.rs:360` (`find_files(Path::new(kr))`);
`gates/gate_binary_fresh.rs:40` and `gates/install_platforms.rs:206` (their
`{root}/checks` resolve dirs); `gates/shim_restatement.rs:117`
(`{root}/templates`); and `gates/gate_tamper.rs:194-198`, whose prefix is matched
against **staged git paths**, which are repository-relative — a site the entry
does not name and which the dialect rule settles the same way.

`registry.rs:86`'s parameter is renamed with the change; it would otherwise name
the dialect it no longer takes. The binding is local to that signature, so the
rename reaches no surface outside it (§Retired spellings).

**Three sites stay as they are, and are named so a later reader does not re-open
them:** `emit/graph.rs:143` feeding `registry::expand_couples`, which the
couples-anchor amendment already ruled a text match; `gates/kit_ref_liveness.rs:164-165`,
whose `live` set and `prefixes` are both basename-derived text; and
`toolfloor.rs:280`, a test reading basenames.

Mechanical: every site's dialect is decided by delta 1's rule, and delta 4 is the
oracle for whether the decision was applied.

### (4) A fixture nests the gate-sdk root {design-bearing}

**Not yet applied.** `check-kit-roots-dialect` (gate-sdk, native substrate, with
the `good/`+`bad/` fixture pair the four contracts require) runs the arms and
gates above against a fixture tree whose kits sit under a subdirectory, and
asserts each produces the **same corpus size** it produces against the same tree
vendored at the root.

**`check-kit-enum` needs a populated fixture, not a count comparison alone.**
Every other listed gate already has live content in this repo's own tree to
compare a nested count against a flat one; `check-kit-enum`'s dormant path half
(delta 2) has none — its clean line reads `0` either way regardless of whether
the split landed. The fixture's `gates.list` therefore carries one multi-kit
`couples=` hand-list group (`>=2` roots sharing a glob) so the git-pathspec half
has something to find, and the nested/flat comparison is over a non-zero count.

- **`good/`** — kits at `vendor/`, `GATE_SDK_ROOT=vendor/gate-sdk`. Every arm's
  counted output matches the flat tree's.
- **`bad/`** — the same tree with one reader deliberately left on the rel
  spelling. The gate reds naming the arm and the two counts.

**Point 5 — the red condition, and why it is a count and not a diff.** The
failure mode this unit exists to close is a corpus that silently shrank, not one
that changed content: `--emit enum-sets` went to zero rows and exited 0, and two
gates printed a clean line with a smaller number in it. A count comparison across
the two layouts catches exactly that, and catches it on the arms whose output is
too tree-dependent to diff. The gate prints both counts on green as well as red,
so a fixture that packed nothing is visible.

**This is the first fixture in the battery that nests the root**, which is the
whole reason nine sites drifted, and it is what makes delta 1's rule enforced
rather than merely written.

## Producers and consumers

- **The dialect rule (delta 1).** Producer: gate-sdk/SPEC.md §The path-dialect
  contract. Consumers: `walk::kit_roots` and `walk::kit_roots_rel`'s `# spec:`
  pointers, which `check-spec-pointer` resolves; every author of a twentieth
  reader; and `check-kit-roots-dialect`, which is its executable form.
- **`walk::kit_roots()`'s widened caller set (deltas 2 and 3).** Producer:
  `walk.rs`, unchanged in behaviour — this amendment adds no function and changes
  no return value. Consumers: the fourteen re-pointed or split sites. Its enabling
  configuration is `GATE_SDK_ROOT` and `GATE_SDK_KIT_DIRS`, which `init` writes
  into every consumer's knob file — deployed, not test-only, and a nested value is
  reachable today by an adopter who hand-vendors under a subdirectory.
- **`registry::fixture_suites_in`'s renamed parameter (delta 3).** Consumers:
  `registry.rs:104` and `knobs/evidence_kit.rs:50`, both of which this delta
  re-points in the same commit, so the rename has no unmoved caller.
- **`check-kit-roots-dialect` (delta 4).** Producer: its `.gate` descriptor in
  gate-sdk's `checks/`, registered in `gates.list`. Consumers: the battery, the
  generated pre-commit hook through its `# graph:` manifest (coupling `walk.rs`
  and each re-pointed module), and its fixture pair.

**Point 6 — every member of the nineteen-site corpus has a satisfying value.**
Fourteen sites take `kit_roots()` by deltas 2 and 3 (five of them as the path half
of a split). Three stay on `kit_roots_rel()` and are named in delta 3 with the
ground for each. The remaining two are `walk.rs`'s own definitions and
`gates/reads_couples.rs`' tests, which call the `_from` variants (one exception,
`reads_couples.rs:727`, calls the bare function directly but under a
test-resolved knob env exercising the dialect itself rather than reading it) with
an explicitly resolved root and take neither dialect by default. No site is
narrowed past unnamed.

## Existing sections updated

Rosters produced by `git grep -n kit_roots_rel native/src` (19 call sites), by the
nested-versus-flat probe described above, and by reading `native/src/walk.rs`,
gate-sdk/SPEC.md §The path-dialect contract and §Layout and configuration.

- `gate-sdk/SPEC.md` §The path-dialect contract: the dialect rule (delta 1).
- `gate-sdk/SPEC.md` §lib/gate.sh: the `kit_roots` / `kit_roots_rel` /
  `kit_roots_abs` spelling roster, which today describes three spellings without
  saying which reader takes which (delta 1).
- `native/src/walk.rs` — the two `# spec:` comments (delta 1).
- `native/src/gates/kit_registration.rs`, `knob_citation.rs`,
  `knob_default_coupling.rs`, `docs_cmd.rs`, `kit_enum.rs` (delta 2).
- `native/src/emit/enum_sets.rs`, `close_surfaces.rs`, `pack_installer.rs`;
  `native/src/registry.rs`; `native/src/knobs/evidence_kit.rs`;
  `native/src/gates/gate_binary_fresh.rs`, `install_platforms.rs`,
  `shim_restatement.rs`, `gate_tamper.rs` (delta 3).
- `scripts/gates.list`, a new `gate-sdk/checks/check-kit-roots-dialect.gate`, its
  native module and its `good/`+`bad/` fixture pair; gate-sdk/README.md's gate
  roster block (delta 4).
- `TASK-QUEUE.md`'s `kit-roots-rel-filesystem-readers` entry: its nine-site
  roster, which named a subset (delta 3). Its **cost class** is already corrected
  to `event/high` by operator direction at spec, in the entry's cost prose rather
  than a board tag — a promotion drops the cost board tag and an active entry
  carrying one reds (queue-kit/SPEC.md §check-deferred-board-tags), so the prose
  is both the class's home and the source a later demotion would class the tag
  from.
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md`, `docs/enforcement.md`,
  `scripts/CHECK-GRAPH.html`, `scripts/git-hooks/pre-commit`.

## Retired spellings

- None — no name is removed. `walk::kit_roots_rel` keeps its name and its three
  remaining callers; what changes is which sites *call* it. The one identifier
  this amendment renames is `registry::fixture_suites_in`'s parameter, whose
  spelling is a local binding no surface outside `registry.rs:86` can name, so it
  retires nothing a reader elsewhere could still be reading.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `kit-roots-rel-filesystem-readers` moves to Done in the
      merge commit, a stage before the drain stage, with its roster and cost
      corrected to the measured nineteen sites.
- [ ] **The silent class is closed** — under the nested fixture, `--emit
      enum-sets`, `--emit close-surfaces` and the derived `EVIDENCE_KIT_SUITES`
      produce the same corpus they produce flat, and
      `check-kit-registration` and `check-knob-citation` print the same counts.
- [ ] **The rule is enforced, not written** — `check-kit-roots-dialect` reds on a
      reader left on the wrong spelling.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed** — any cross-component gap build discovers is resolved that
      session.
