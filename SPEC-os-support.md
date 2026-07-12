# SPEC amendment: os-support-statement

A docs-site ruling with no owning kit, so this amendment lives at the repo
root. `docs/install.md` gains the requirements section it lacks, and the
derive-vs-restate fork the queue entry posed is ruled here.

## What changes

- `docs/install.md` gains a `## Requirements` section ahead of the vendor
  steps:
  - **The OS ruling** — Checkwright is Unix-first: Linux and macOS are the
    supported platforms; Windows runs via WSL, not natively. The reason is
    stated, not asserted: the gate battery and the git hooks are bash over a
    coreutils toolchain, and no native-Windows shell path exists.
  - **The toolchain list** — an annotated bullet list, one tool per bullet:
    the tool name as a code token followed by a purpose clause (what breaks
    without it). The name set is exactly env-probe's probe set
    (context-kit/SPEC.md §bin/env-probe); the purpose clauses are hand
    prose.
  - **Version policy** — no minimum versions are baked into the page:
    versions are probe-owned, and the section points the reader at seeding
    a local profile with context-kit's env-probe (the ENV.local.md
    mechanism) to see what their box actually carries.
  - **The optional docs-site tier** — prose *outside* the parity-gated
    list: a consumer that registers site-kit's render-fidelity gate
    (SPEC-render-fidelity.md) additionally needs ruby with the
    kramdown-parser-gfm gem; a consumer that does not publish a docs site
    never installs it. This keeps the tiered-honest-claim shape the
    positioning page uses (SPEC-harness-positioning.md): the floor is the
    probe set, the docs-site dependency is a named wider tier.

- **The fork ruling: parity gate over a generated fragment.** The tool
  *names* are derivable from env-probe's probe set, but the per-tool purpose
  clauses are hand content no derivation owns — a generated fragment would
  either drop them or grow a purpose roster inside a probe script where it
  has no business. So the list stays hand-authored and a gate holds the
  derivable part: `scripts/check-install-toolchain.sh`, a consumer gate
  asserting name-set equality between env-probe's probe set (read from the
  script's own array, the single owner) and the code-token tool names of
  the Requirements section's bullet list — both directions: a probed tool
  missing from the list and a listed tool absent from the probe set both
  red. Registered in `scripts/gates.list`, precommit tier, `good/`+`bad/`
  fixtures under `scripts/gate-tests/`, `# graph:` manifest coupling
  `docs/install.md` to `context-kit/bin/env-probe.sh`.

## Producers and consumers

- The probe set: already produced and owned by `context-kit/bin/env-probe.sh`
  (its in-source array, spec-pointed); the new gate becomes its second
  reader. No new interface on context-kit — the gate reads the array
  declaration from the tracked source the same way the SPEC's own pointer
  does, so a probe-set edit reds the docs list without any emitter handshake.
- The Requirements section: produced by the build session; read by
  evaluating adopters and by the new gate (the list span only).
- No new knobs, no new fields.

## Existing sections updated

- `docs/install.md` — the page's opening now routes a reader through
  Requirements before the vendor walkthrough; the upgrade-contract section
  is untouched.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md` at the repo root).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
