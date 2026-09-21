# SPEC amendment: install-page

`docs/install.md` is an adopter's first page after the front door, and it is 980
lines. Most of them are maintainer-tier grounds: the release channel's
mechanism, the upgrade contract's note grammar, and the platform-support
reasoning. Nothing holds a `docs/` page to a length, so the page regrows after
every trim. The operator's deliverable, by operator direction (2026-09-21, lead
session), is two things. First, an adopter page of about 150 lines: intro,
Requirements with its remedy blocks bare, Quick start, Managing, Upgrading, and
pointers to installer/SPEC.md for the grounds. It keeps the gated marker blocks
and the `install-primary:` declaration. Second, a regrowth guard whose reach is
the whole `docs/` site.

**The page today (`grep -n "^#\+ " docs/install.md`, 980 lines).**

| Section | Lines | What owns its grounds today |
| --- | --- | --- |
| intro | 6–60 | the page |
| §Requirements, with ###Where this is heading | 61–331 | split: context-kit/SPEC.md (the tool floor), docs/site-architecture.md §Generated projections (the parity contracts), installer/SPEC.md (the binary half) |
| §Quick start | 332–429 | installer/SPEC.md §init, §The install boundary |
| §Managing an install | 430–469 | installer/SPEC.md §update, §diff, §uninstall, and the page says so |
| §Vendoring the kits, with ###What a gate discloses | 470–535 | **no other owner** |
| §Reviewing the pre-commit hook before you install it | 536–580 | **no other owner** for the audit walk; gate-sdk/SPEC.md owns the tier claim it cites |
| §Running under an AGENTS.md harness | 581–625 | docs/positioning.md, already cited twice inline |
| §Versioning, with ###The release channel | 626–805 | **no other owner** |
| §The upgrade contract | 806–971 | **no other owner** |
| §Branch protection | 972–980 | the page |

**What the page must keep, because a reader reds without it.**

- **The four marker blocks**, 109 lines between them:
  - `platforms`, 27 lines, held by `check-install-platforms`;
  - `toolchain`, 64 lines, held by `check-install-toolchain`;
  - `macos-remedy` and `windows-remedy`, 9 lines each, which four
    `.github/workflows/gates.yml` legs awk out of `docs/install.md` by marker and
    execute.

  The workflow's comments cite `docs/install.md §Requirements`, so that heading
  stays.
- **`<!-- install-primary: tarball -->`**, read by `check-install-claim`, which
  reds on a zero count. It must sit under a heading matching
  `CANON_KIT_INSTALL_SECTION_RE` (`^(Quick start|Install)`), so it stays under
  `## Quick start`.
- **The `Release channel: **preview**` line** (`:634`). `check-release-channel-parity`
  reads it from `docs/install.md` by default (`DEFAULT_INSTALL_MD`,
  `native/src/gates/release_channel_parity.rs:7`), and an adopter should read the
  channel on the install page anyway. It moves into §Upgrading.

**What moves, and what that reaches.** Versioning, the release channel and the
upgrade contract together are about 345 lines, and they have no other owner. They
are cited by `docs/install.md §<heading>` in 41 single-line places
(`git grep -c "install.md §\(Versioning\|The release channel\|The upgrade contract\)"`
outside the generated mirrors), plus wrapped citations in gate-sdk/SPEC.md and
RELEASING.md:

- 30 `// spec:` pointers across four release gates and `upgrade_smoke.rs`;
- four `.gate` descriptors;
- `publish.yml`;
- the kit SPECs;
- RELEASING.md.

The upgrade gates' own error text says "(docs/install.md §The upgrade contract
owns the note grammar)". `check-spec-pointer` resolves a path-bound
`<path> §<heading>` against that file, so every one of those citations reds the
moment its heading leaves the page. The relocation is therefore one commit with
its citation sweep, and `check-spec-pointer` is its oracle.

**The ~150-line figure against the gated floor, measured.** The four blocks are
109 lines that no rewrite can shorten. They are parity- and workflow-held. So
"about 150 lines" leaves about 40 lines for the intro, Quick start, Managing and
Upgrading. **Settled by operator direction (2026-09-21, lead session): about 150
lines of prose around the gated blocks, so the page lands near 260 lines.** Build
sets docs/install.md's ceiling row at the rewritten page's actual length.

**A sanctioned option, not a requirement.** The same direction added that a long
docs page may be split into sub-pages. Build may do that here or on any page the
ratchet governs. A split page's parts each take their own ceiling row in the same
commit. The gated blocks stay on the page their readers name: the workflow legs
awk `docs/install.md`, `check-install-claim` reads a Quick start or Install
heading, and the two parity gates read `docs/install.md` by default. So a split
leaves those blocks where they are, or it repoints each reader in the same commit.

It is a root-level amendment because it spans the `docs/` site, installer/SPEC.md
(which receives the grounds), context-kit (the ratchet's stated scope), `native/`
(the pointer comments), and gate-sdk/SPEC.md and RELEASING.md (the citations).

## What changes

**Batching.** Deltas 1 and 2 are one commit, because the page cannot lose a
cited heading before its citations move. Delta 3 lands in the same commit, so the
guard's first ceiling is the rewritten page, never the 980-line one.

### (1) The grounds relocate to installer/SPEC.md under their existing headings {mechanical}

**Not yet applied.** installer/SPEC.md gains `## Versioning` (with
`### The release channel`) and `## The upgrade contract`, placed before `## Docs`.
They carry the moved text **under the same heading names**, so every citation
changes only its path, from `docs/install.md §X` to `installer/SPEC.md §X`. The
sweep covers:

- the `// spec:` comments;
- the error strings in `native/src/gates/release_bump.rs`,
  `release_declaration_parity.rs`, `release_channel_parity.rs` and
  `tightened_gates_grammar.rs`, and `native/src/emit/upgrade_smoke.rs`;
- the four `scripts/check-release-*.gate` / `check-tightened-gates-grammar.gate`
  headers;
- `.github/workflows/publish.yml`;
- gate-sdk/SPEC.md, lifecycle-kit/SPEC.md and RELEASING.md.

A bare `§The upgrade contract` (no path) resolves over the whole manifest and
needs no edit.

`installer/SPEC.md` also takes two more sections:

- `## Vendoring without the installer`, holding the manual path and
  `### What a gate discloses`;
- `## Reviewing the pre-commit hook`, holding the audit walk.

Neither has another owner, and the operator's shape points the grounds there.
§Running under an AGENTS.md harness moves nowhere: docs/positioning.md already
owns it, and the page keeps one pointer. §Branch protection becomes one sentence
in §Managing. ###Where this is heading is roadmap prose and does not survive as a
page section. Its `ported-gate-members` measured claim leaves with it. That is
safe: `check-measured-claim` has no converse assertion (canon-kit/SPEC.md
§check-measured-claim, "a key no marker names is the ordinary state"). The one
prose mention of that claim's location, docs/site-architecture.md:204, is
updated.

Mechanical: a move with a fixed citation sweep, whose worklist is
`git grep -n "install.md §"` and whose oracle is `check-spec-pointer`.

### (2) The page is rewritten at the adopter tier {design-bearing}

**Not yet applied.** docs/install.md becomes these sections, in order:

- **intro**: what the page gets you, in two short paragraphs.
- **§Requirements**: the platform and toolchain blocks and the two remedy blocks,
  bare, with one lead sentence each. The grounds are a pointer to context-kit's
  tool floor and installer/SPEC.md §Requirements.
- **§Quick start**: the `install-primary:` declaration, then the npm and tarball
  paths as commands, with `checkwright demo` first. `checkwright demo` exists
  only once SPEC-demo-verb.md lands. If that unit has not landed when this one
  does, the line is omitted, and it joins in that unit's delta 4.
- **§Managing**: `doctor`, `diff`, `update`, `uninstall`, one line each, the
  reversibility sentence, and branch protection's one sentence.
- **§Upgrading**: the `Release channel:` declaration line, the upgrade contract's
  *In brief* in adopter voice, and a pointer to installer/SPEC.md §Versioning and
  §The upgrade contract.
- **§Going further**: one line per relocated section, pointing at it.

**Voice.** Second person and imperative, and no gate internals. A sentence that
explains *why* a mechanism works is a grounds sentence, and it belongs in
installer/SPEC.md.

**Inbound links.** `git grep 'install\.md#'` returns nothing, so no markdown
anchor dangles. The whole-page pointers are
SECURITY.md:34, docs/index.md, docs/methodology.md, docs/positioning.md,
docs/releases.md, docs/gate-sdk/index.md and the posts. Their prose is read at
build against the new page, because a pointer that says "the upgrade contract on
the install page" is now wrong in fact even where it resolves. §Quick start
keeps its name, since README.md:61 cites it.

### (3) The regrowth guard widens `check-surface-ratchet` to the site by config {design-bearing}

**Not yet applied.** No new gate. context-kit's `check-surface-ratchet` already
holds any tracked file its `CONTEXT_KIT_RATCHET_PATHS` pathspecs match to a
committed ceiling. It reds on growth until the growing commit re-stamps with
`--emit always-loaded --ceiling`, and that is exactly a regrowth guard. The page
path lives in consumer config, as `gates-must-not-bind-to-document-paths`
rules.

- `scripts/context-config.knobs` gains `CONTEXT_KIT_RATCHET_PATHS[] = docs/*.md`
  and the three exclusions
  `:(exclude)docs/*/SPEC.md`, `:(exclude)docs/*/README.md` and
  `:(exclude)docs/doctrine-kit/DOCTRINE.md`. The mirrors' size is their sources'
  (canon-kit's reference-link mirror topology), so a ceiling on them would
  restate one. Under git's pathspec rules `*` crosses `/`, so `docs/*.md` reaches
  every depth. Probed: `git ls-files -- 'docs/*.md' ':(exclude)docs/*/SPEC.md'
  ':(exclude)docs/*/README.md' ':(exclude)docs/doctrine-kit/DOCTRINE.md'` returns
  **51** files: the top-level pages, the eleven kit index pages and the dated
  posts. The posts are immutable, so their ceilings never move.
- `.workflow/surface-ceiling.txt` gains 51 rows, written by `--ceiling` in the
  same commit, with docs/install.md's row at the rewritten page's length.
- context-kit/SPEC.md §The surface ratchet's **"Why these files, per file"**
  bullet is re-phrased. It no longer says the governed set is only what "a
  trigger loads whole". It says instead that the kit's case is trigger-loaded
  instruction surfaces, and that a consumer may govern any authored surface whose
  growth it wants to see in the growing commit. A public page whose reader tier
  must not regrow is the named second case.

**Point 5, the red condition.** The guard's reader is `check-surface-ratchet`,
whose red is a governed file above its row *or a governed file with no row*. The
widening adds 51 governed files, and the second clause reds all 51 until their
rows land, which is why the rows ride in this commit. The meter is not affected:
`--emit always-loaded` measures `CONTEXT_KIT_SURFACES`, and `governed()`
(`native/src/emit/always_loaded.rs:138`) adds ratchet paths to the ceiling set
alone. `--update-baseline` at close rewrites one baseline row whose surface
count grows by 51, and that is the count's own staleness witness, as designed.

**The honest limit.** A ratchet stops silent growth. It does not stop a
deliberate re-stamp, and it has no notion of reader tier. Close's brevity pass
judges re-stamps (§The surface ratchet, "close judges"). A tier detector for
docs pages is not built here, and the entry asks for none.

## Producers and consumers

- **The relocated sections (delta 1).** Readers: `check-spec-pointer`, over every
  repointed citation; the four release gates, whose messages now name
  installer/SPEC.md; and a maintainer reading why. `check-release-channel-parity`
  keeps reading the declaration line from docs/install.md, which the rewrite
  keeps in §Upgrading. Its default path and `.gate` couples are unchanged.
- **The rewritten page (delta 2).** Readers:
  - `check-install-platforms` and `check-install-toolchain`, whose blocks are
    unchanged;
  - `check-install-claim`, which needs its one declaration under a heading its
    RE matches;
  - the four workflow remedy legs;
  - `check-md-refs`;
  - `check-docs-render-fidelity`;
  - `check-door-binding` assertion C, since the page is under `docs/`.
  None of these reads a section this amendment deletes.
- **The ratchet paths and rows (delta 3).** Producer: this repo's knob file and
  `--ceiling`. Consumer: `check-surface-ratchet`. The descriptor's `couples=`
  carries `knob:CONTEXT_KIT_RATCHET_PATHS`, if it already does, or else the
  §The surface ratchet *Limit* line applies: "pathspecs outside the descriptor's
  static `couples=` are held by the battery, not the hook". Build reads the
  descriptor and states which case holds.
- **Point 6.** The ratchet corpus is the 51 files the probe enumerates, and each
  one's satisfying value is its line count at the commit, stamped by
  `--ceiling`. No member is narrowed past.

## Existing sections updated

Rosters produced by `grep -n "^#\+ " docs/install.md`, `git grep -n "install.md §"`,
`git grep 'install\.md#'`, `grep -n "install.md" .github/workflows/gates.yml`,
the 51-file pathspec probe above, and by reading context-kit/SPEC.md §The surface
ratchet, `native/src/emit/always_loaded.rs` `governed()`, and canon-kit/SPEC.md
§check-install-claim and §check-measured-claim.

- installer/SPEC.md: four new sections (delta 1).
- The citation sweep (delta 1) runs from the worklist
  `git grep -n "install.md §\(Versioning\|The release channel\|The upgrade contract\)"`,
  plus the wrapped instances `check-spec-pointer` names. Its surfaces are listed
  one per bullet below, so `check-amendment-retired-spelling` can bind each one:
- `native/src/gates/release_bump.rs` (delta 1).
- `native/src/gates/release_channel_parity.rs` (delta 1).
- `native/src/gates/release_declaration_parity.rs` (delta 1).
- `native/src/gates/tightened_gates_grammar.rs` (delta 1).
- `native/src/emit/upgrade_smoke.rs` (delta 1).
- `scripts/check-release-bump.gate` (delta 1).
- `scripts/check-release-channel-parity.gate` (delta 1).
- `scripts/check-release-declaration-parity.gate` (delta 1).
- `scripts/check-tightened-gates-grammar.gate` (delta 1).
- `.github/workflows/publish.yml` (delta 1).
- `gate-sdk/SPEC.md` (delta 1).
- `lifecycle-kit/SPEC.md` (delta 1).
- `RELEASING.md` (delta 1).
- docs/site-architecture.md:204, the measured claim's location (delta 1).
- docs/install.md (delta 2), and the whole-page pointers' prose where it names a
  section that moved (delta 2).
- `scripts/context-config.knobs`, `.workflow/surface-ceiling.txt`, and
  context-kit/SPEC.md §The surface ratchet (delta 3).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, regenerated.
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/lifecycle-kit/SPEC.md`, regenerated.
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/installer/SPEC.md` and `docs/context-kit/SPEC.md`, regenerated.

## Retired spellings

- `docs/install.md §Versioning` — the path-bound citation form; the heading now
  lives in installer/SPEC.md (delta 1).
- `docs/install.md §The upgrade contract` — the path-bound citation form (delta 1).
- `docs/install.md §The release channel` — the path-bound citation form (delta 1).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only.** The rewritten page carries no
      grounds sentence, and each one moves to installer/SPEC.md.
- [ ] **Merged with no information lost.** Every paragraph that leaves the page
      lands in its named owner or is roadmap prose explicitly dropped. Build diffs
      the old page against the union of the new page and the new sections.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `install-md-adopter-page-overgrown` moves to Done in the
      merge commit, at a stage before the drain stage.
- [ ] **Budget met.** The page carries about 150 prose lines around the gated
      blocks, per the operator direction, and its ceiling row, or each
      sub-page's row if build split it, is the landed length.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
