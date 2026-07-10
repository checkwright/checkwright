# SPEC amendment: canon-rename

## What changes

The kit's brand identity renames: **spec-kit → canon-kit** (from the kit's
own "canonical spec" vocabulary), resolving the collision with GitHub Spec
Kit — the most recognized name in spec-driven development, aimed at the
identical audience. Hard rename, no compatibility aliases: no release tag
exists and this repo is the only consumer, which is exactly why the rename
lands now — after tag one the upgrade contract makes a kit-dir rename a
breaking major.

The seam between brand tokens and generic vocabulary:

- **Brand tokens — renamed.** Every name derived from the kit's identity:
  the kit dir (`spec-kit/` → `canon-kit/`), the env-knob prefix
  (`SPEC_KIT_` → `CANON_KIT_`), the consumer-config filename the loader
  discovers (`spec-config.sh` → `canon-config.sh`, the `<kit>-config.sh`
  convention), and the docs-site page dir (`docs/spec-kit/` →
  `docs/canon-kit/`).
- **Generic vocabulary — kept.** Every name derived from the spec *artifact*
  discipline, not the kit's brand: the `SPEC.md` canonical-spec filename,
  the `SPEC-*.md` amendment glob, the `spec:` / `contract:` source
  directives, the `[spec:]` / `[needs-spec]` queue tags, the `check-spec-*`
  gate names, and the `lib/spec.sh` loader filename (internal to the kit
  dir; "spec" there names the discipline).

Because gate names are unchanged and `gates.list` resolves names against
each vendored kit's `checks/` dir, the registry needs no edit — the renamed
dir re-registers by name alone.

**`check-kit-ref-liveness` (consumer gate, `scripts/`)** lands in the same
unit — the removals-propagated grep made permanent: every kit-name
reference in a tracked file resolves to a kit root on disk. Reference
forms it resolves: a kit-shaped path segment (`(^|/)<name>-kit(/|$)`, plus
`gate-sdk`) must name an existing root, and a kit-prefixed knob must
resolve the way `check-docs-cmd` already resolves its knob mentions —
borrow that resolver rather than re-derive the prefix mapping, which is
not uniform (`SPEC_KIT_` but `LIFECYCLE_`). Bare prose tokens are out of
scope: the align-audit inventory showed the tree's `<name>-kit` tokens are
dominated by non-references (gate names like `check-kit-registration`,
fixture placeholders like `alpha-kit`, compounds like `per-kit`), and the
path/knob forms are the ones that dangle harmfully. Valves, each with
precedent: `gate-tests/` fixture bodies (the manifest finder already
prunes them), `docs/posts/*` and the generated `docs/evidence-data.md`
(immutable history — both cite the old brand forever), and `SPEC-*.md`
amendments (legitimately name retired and future paths). Consumer-local
by operator ruling: the hazard it guards — a kit rename or retirement
leaving dangles — is a kit-author hazard, not a consumer one, so it is
not templated into gate-sdk; ships as a check-skeleton copy in `scripts/`
with a `good/`+`bad/` fixture pair under `scripts/gate-tests/`, registered
in `gates.list` with a `# graph:` manifest (hook + graph + enforcement map
regenerate on landing). Promotion into gate-sdk stays a file move if it
ever earns generality.

## Producers and consumers

- **`CANON_KIT_*` knobs** — producer: the consumer config
  `scripts/canon-config.sh` (or env); consumer: `lib/spec.sh`'s loader and
  validation block, then every gate that sources it. The loader's discovery
  path becomes `${GATE_SDK_GATES_DIR:-scripts}/canon-config.sh`
  (today `lib/spec.sh:13`).
- **Renamed paths** — consumers are every cross-reference the battery
  already governs: `check-md-refs` (links), `check-docs-cmd` (invoked
  commands), `check-kit-registration` / the kit enum, `check-docs-kit-parity`
  (the docs/index.md row), `check-core-files` (`scripts/core-files.list`),
  the root allowlist, and CI (`.github/workflows/gates.yml` fixture step).
  The sweep is verified by the battery, not by eyeball — except the
  ungoverned sites below.
- **Ungoverned reference sites** — surfaces outside the spec manifest and
  the `spec:`-pointer scan that carry retired names, where the battery stays
  green on a dangle (audited at align): `.claude/commands/scope.md` and
  `validate.md` (prose §-citations of the SPEC),
  `.claude/commands/agent-execution.md` (the `spec-config.sh` mention and
  the fixture-runner line), `lifecycle-kit/templates/skills/release-sweep.md`
  (the brand name and `SPEC_KIT_DEPRECATION_MARKERS`),
  `scripts/graph-vocab.sh` (the kit-class vocabulary — a stale pattern
  misclassifies silently on regeneration rather than erroring), and
  `scripts/evidence-config.sh` (the fixture-suite path goes red only at the
  validate stage, not at commit). The removals-propagated grep is the net
  for these.
- **Generated projections** — the pre-commit hook, `.workflow/CHECK-GRAPH.html`,
  and `docs/enforcement.md` embed kit paths; all three regenerate after the
  sweep (`gen-pre-commit.sh --write`, `check-graph.sh --emit`,
  `enforcement-map.sh --emit`).
- **`check-kit-ref-liveness`** — producer: `gates.list` registration (the
  consumer gates dir resolves first, so no kit changes); consumers: the
  battery (`run-gates.sh`), the consumer fixture runner
  (`scripts/gate-tests`), and the pre-commit hook via its `# graph:`
  manifest. Green on the pre-rename tree too (every `spec-kit` path
  resolves until the dir moves), so it may land before, with, or after the
  sweep — but inside this unit, because the rename is what turns today's
  references into potential dangles.

## Existing sections updated

- `spec-kit/SPEC.md` (→ `canon-kit/SPEC.md`): title line and §Layout and
  configuration — the conventional vendor path and every knob name; the
  generic-vocabulary rulings above land there so the boundary is owned, not
  re-derived.
- `canon-kit/README.md`, the repo `README.md` kit map, `CLAUDE.md`'s fixture
  runner line, `CONTRIBUTING.md`: mechanical reference updates.
- Cross-kit SPEC references (drift-kit, evidence-kit, lifecycle-kit,
  context-kit, site-kit, queue-kit, gate-sdk, drift-kit/templates, and
  lifecycle-kit/templates): mechanical reference updates, propagated per
  merge step 5.
- The ungoverned reference sites listed above (`.claude/commands` skills,
  `scripts/graph-vocab.sh`, `scripts/evidence-config.sh`): mechanical
  reference updates, held by the removals grep rather than the battery.
- `docs/posts/` needs no edit: no dated post references the kit (checked at
  scope), so the immutability rule is not exercised.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every tracked file for names this
      change retired (`spec-kit`, `SPEC_KIT_`, `spec-config.sh` outside
      generic-vocab keepers and immutable surfaces: dated posts, generated
      trajectory data, fixture bodies where the name is inert content);
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
