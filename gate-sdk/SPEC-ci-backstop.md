# SPEC amendment: ci-backstop

## What changes

A CI workflow template, `gate-sdk/templates/gates-workflow.yml` — the
server-side backstop for the two enforcement gaps the local hook cannot
close by construction: a `--no-verify` commit, and a clone that never
opted in via `install-hooks.sh`. Scope boundary, stated where the docs
explain it: consumer-owned CI stops *bypass*; it cannot stop an agent
editing the workflow itself in the same change — verifier neutrality is
the deferred `hosted-attestation-service` rung, and this template's docs
section says so rather than overclaiming.

The template (copied out like every gate-sdk template; placeholder
discipline per spec-kit/SPEC-template-tiering.md):

- trigger: push + pull_request on the consumer's default branch;
- step 1: `bash gate-sdk/bin/run-gates.sh` — the full battery;
- step 2: a clearly-marked placeholder block the consumer fills with its
  fixture/test runners (kit fixture runners, guard tests — the consumer's
  battery list; a filled example is this repo's own workflow);
- no caching, no matrix, no third-party actions — checkout and bash only,
  so the workflow surface an agent could tamper with stays minimal and
  reviewable.

This repo wires it day-one: `.github/workflows/gates.yml` runs the
battery plus every fixture runner CLAUDE.md's battery list names, and the
two workflow files (template + instance) register in
`scripts/core-files.list` so their silent deletion is red. The instance
needs no freshness gate — a workflow invoking a retired script goes red
in CI on its own next run, which is the drift signal working as designed.

Branch-protection recipe (require the workflow's status check before
merge) lands as an install-page docs section (slot owned by
SPEC-docs-site.md), not as mechanism — GitHub settings are not a
committable surface.

Seam: mechanism only — the workflow is written fresh here and
de-hardcoded; no platform pipeline content, names, or rule sets leave the
platform.

## Producers and consumers

- Producer: GitHub Actions on push/PR, once the consumer commits its
  copied workflow (this repo commits `.github/workflows/gates.yml` at
  land time — the enabling configuration is set, not test-only).
- Consumer: the merge gate (required status check per the recipe) and the
  PR author reading the failed step's gate output — the same
  finding + `help:` lines the local hook prints.
- Inputs read: the vendored kit dirs and the consumer's `gates.list` via
  `run-gates.sh`; the placeholder block's runners. No new state or
  message fields; CI writes nothing back into the tree.

## Existing sections updated

- gate-sdk SPEC §Layout and configuration: templates/ roster gains
  `gates-workflow.yml`; §Consumer smoke unaffected (CI is not a smoke
  surface — it runs the real battery).
- gate-sdk README: one enforcement-story line (hook = local, workflow =
  server backstop, neutrality = out of scope here).
- `scripts/core-files.list`: + `.github/workflows/gates.yml`,
  + `gate-sdk/templates/gates-workflow.yml`.
- `scripts/root-allowlist.list`: + `.github` (new tracked top-level
  entry; check-root-tiering).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
