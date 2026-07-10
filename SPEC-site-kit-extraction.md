# SPEC amendment: site-kit-extraction

A governance ruling spanning a new kit and spec-kit, so it lives at the
repo root (the template's no-owning-component placement).

## What changes

The docs/site governance cluster — accumulated as root amendments and
consumer gates — extracts to kit form. The demand criterion the deferred
entry stated (the next docs-scoped consumer gate landing) is met by
enforcement-map's emitted page and freshness gate in this same iteration.
The seam was pre-cut: conventions, host aliases, and sinks are already
consumer config, so the lift is mechanical, copy-first.

The split ruling — a split, not one kit:

- **`check-docs-link-convention` joins spec-kit**, beside check-md-refs:
  same governed doc set, same scan machinery. The gate file moves into
  spec-kit/checks/ keeping its name; its knobs move into the spec-kit
  namespace (`SPEC_KIT_LINK_*` — build enumerates the renames in the merged
  section), and the values (docs host, alias set) stay consumer config.
- **The deployment-truth pieces become `site-kit`** (a new kit root):
  `checks/check-docs-cname-parity.sh`, generalized — docs host, alias set,
  and CNAME path as `SITE_KIT_*` knobs with this repo's layout as defaults
  (`docs/CNAME`) — and `templates/site-health.yml`, the scheduled live-site
  probe generalized from this repo's workflow. Tree gates verify the tree;
  the health workflow is the monitor class (deployment truth), which is why
  it ships as a template a consumer copies, never a gate.
- **`check-docs-kit-parity` stays consumer**: it couples this monorepo's
  kit table in docs/index.md — a consumer-specific projection, not generic
  mechanism.

Kit obligations ride the extraction, per the established conventions:
site-kit README + SPEC.md (drafted at build from this outline), a fixture
pair for the gate, `smoke/install.sh` plus `smoke/violation.sh` (an alias
URL in a tracked file is a craftable violation), registration in this
repo's `gates.list`, `gate_kit_roots`/kit-registration membership, a
root-allowlist entry, and the docs/index.md kit row (check-docs-kit-parity
enforces it).

Copy-first mechanics: the scripts/ copies of the moved gates are deleted in
the same commit their kit versions land — names re-resolve through the
registry (consumer gates dir first, then each kit's checks/), so
`gates.list` itself does not change for the moves.

## Producers and consumers

- Gate producers unchanged: the generated pre-commit hook / `run-gates.sh`
  resolve the same registered names to their new homes; regeneration of the
  hook and the graph artifact rides the move (the graph manifests travel
  with the gate files).
- site-health template producer: the consumer's copied workflow on its
  schedule; consumer: the operator via the workflow's failure signal —
  unchanged behavior, new home.
- New `SITE_KIT_*` / `SPEC_KIT_LINK_*` knobs: each read by its gate at
  startup; this repo's values move from the current consumer config files
  into the renamed spellings in the same commit.

## Existing sections updated

- spec-kit/SPEC.md gains the link-convention gate's contract section
  (moved, integrated beside check-md-refs).
- site-kit/SPEC.md is written new at build; this amendment is its outline
  and the record of the split ruling.
- README kit map, CLAUDE.md fixture-runner list, and docs/index.md gain the
  site-kit row; scripts consumer config drops the retired spellings.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md site-kit/SPEC-*.md spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
