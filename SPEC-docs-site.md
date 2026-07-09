# SPEC amendment: docs-site

## What changes

A public docs site under `docs/` — plain Markdown, zero build toolchain,
served by GitHub Pages from the `docs/` directory on master with a `CNAME`
file (`docs.vasyltretiakov.dev`; 301 to the product domain at launch). No
site-generator config ships; GitHub Pages' default rendering is the
contract, revisited only on a concrete rendering failure. Every page joins
the governed doc set by explicit wiring, not by default: the link/narration
gate family reads `spec_manifest_files` (spec-kit/lib/spec.sh), whose
default set is canonical SPECs plus `README.md`/`CLAUDE.md` — `docs/` pages
enter only when `scripts/spec-config.sh` sets `SPEC_KIT_MANIFEST_FILES`,
and that knob *replaces* the default set (explicit-globs mode), so the
config enumerates the prior members plus the `docs/` globs (build resolves
the exact globs; the finders' `templates/` exclusion must survive the
switch). `check-tree-terms` needs no wiring — it already scans every
tracked file.

Layout (new directory convention, repo-root governance — no owning kit):

- `docs/index.md` — orientation: what Checkwright is, the kit map (links to
  per-kit pages), the reading order. Owns *sequencing*, cites contracts.
- `docs/kits/<kit>.md` — one page per kit root: what it is, install
  (vendor-whole), quick-start; cites the kit's README/SPEC for contracts.
- `docs/install.md` — repo-level install and upgrade: the vendor-whole
  flow across kits, the two-phase upgrade contract (below), and the
  branch-protection recipe section gate-sdk/SPEC-ci-backstop.md cites.
- `docs/demo.md` — the walkthrough page (the `docs/kits/` sibling slot
  SPEC-demo-walkthrough.md cites; that amendment owns its content).
- `docs/methodology.md` — the delivery-methodology essay surface.
- `docs/evidence.md` — the drift-trajectory evidence page (see
  drift-kit/SPEC-trajectory.md; this amendment owns only its slot).
- `docs/evidence-data.md` — the generated trajectory table (see
  drift-kit/SPEC-trajectory.md; this amendment owns only its slot).
- `docs/posts/YYYY-MM-DD-<slug>.md` — dated artifacts; the announcement
  post is the first.

Three page classes with different drift contracts:

- **Living pages** (everything outside `posts/`) — fully governed: link
  resolution, command/knob resolution (spec-kit/SPEC-docs-cmd.md), bare
  cardinals, temporal narration, fence balance, kit↔page parity.
- **Dated posts** (`docs/posts/`) — immutable after publication; dated
  narrative is their nature, so the class is exempt from
  `check-manifest-temporal` (section/path exemption via the existing
  `SPEC_KIT_TEMPORAL_EXEMPT_SECTIONS`-style consumer config, extended with
  a path form if the current knob cannot express it — build resolves
  against the knob's real shape) while link and command resolution still
  apply. Post-publication edits are a new dated post, not a rewrite.
- **Generated data** (`docs/evidence-data.md`) — emitted by a tool and
  byte-pinned by a freshness gate, so a hand edit is red by construction;
  narration-gate applicability resolves at build (exempted only if the
  generated content trips a prose gate).

Content tiering ruling (extends spec-kit's star topology with a docs tier):
docs pages own orientation, sequencing, and pedagogy; kit READMEs own
install/quick-start for their kit; SPECs own contracts. A docs page cites
downward, never restates an invariant — the anti-restatement doctrine
applies to docs as to comments.

Kit↔page parity (new consumer gate, wrapper not mechanism):
`scripts/check-docs-kit-parity.sh` registered in `gates.list`, a thin
wrapper invoking gate-sdk's existing `check-kit-registration.sh` with
`docs/index.md` as the registry-doc argument — every kit root must carry a
`](<kit>/)`-style link row in the docs index (assertion A); the wrapper
passes only the registry-doc argument, so assertion B redundantly re-checks
CLAUDE.md (harmless, noted). No gate-sdk change. The wrapper ships a
`good/`+`bad/` fixture pair under `scripts/gate-tests/check-docs-kit-parity/`
(a synthetic registry doc missing a kit row), per
check-gate-fixture-coverage.

The announcement post's canonical copy lives in `docs/posts/`; external
publication (newsletter/blog platforms) is a copy that links back — the
repo copy is the one under gates.

Versioning and upgrade contract (ruling, scope 2026-07-09): repo-level
semver via git tags, kits in lockstep — a kit gets its own version only
if it is split out; the first tag (`v0.1.0`) rides the announcement.
Distribution stays git-native and vendored-committed: the gates read
tracked files and the audit story needs the governance layer inside the
reviewed tree, so the package registries remain namespace reservations,
never a dependency channel. The install page documents upgrades as two
phases — (A) deterministic: replace the vendored kit directories
wholesale at the target tag (consumers never edit kit files, so the
sync is lossless) and regenerate the generated artifacts; (B)
gate-driven: run the full battery and reconcile the red set, which *is*
the migration worklist, reading the release note for intent. Release
notes are dated posts, each carrying tightened-gates and renamed-knobs
sections; the consumer-owned residue phase A never touches — shadowed
gates, copied-out templates, knob renames in consumer config — is the
release note's checklist. Upgrade tooling (the tag-N→N+1 upgrade smoke,
an optional thin installer CLI) is the deferred `upgrade-path` rung.

Seam: docs describe mechanism with placeholder vocabulary only (the
graph-vocab pattern); no platform rule content, term lists, or private
names — `check-tree-terms` already fences this mechanically and governs
`docs/` as part of the tree.

## Producers and consumers

- Producer of pages: build sessions of this iteration; thereafter any
  session touching a governed surface a docs page cites (parity and link
  gates make a stale page red at that commit).
- Producer of the parity verdict: the pre-commit hook and `run-gates.sh`
  once `check-docs-kit-parity` is added to `scripts/gates.list`.
- Consumer of pages: adopters via GitHub Pages; the committing session for
  gate findings (which name the kit root missing from `docs/index.md`).
- Inputs read by the wrapper: `gate_kit_roots_rel` (via the wrapped gate),
  `docs/index.md`. No new state or message fields.
- `CNAME` consumer: GitHub Pages' domain binding; no repo code reads it.

## Existing sections updated

- README.md gains a docs-site link line ("docs live at …") at merge.
- CLAUDE.md Housekeeping gains the `docs/` convention line (living pages vs
  dated posts, cite-never-restate) at merge — CLAUDE.md is the canonical
  home for repo-level conventions, since `docs/` is not a kit.
- `scripts/gates.list` registers `check-docs-kit-parity`; regenerate the
  pre-commit hook and CHECK-GRAPH artifact on land (the wrapper carries its
  own `# graph:` manifest line coupling `docs/index.md` to the kit roots).
- `scripts/spec-config.sh`: `SPEC_KIT_MANIFEST_FILES` set as above (the
  prior default members plus the `docs/` globs).
- `scripts/root-allowlist.list`: + `docs` (new tracked top-level entry;
  check-root-tiering).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
