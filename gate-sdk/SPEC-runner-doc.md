# SPEC amendment: runner-doc-off-resident

Move the fixture-runner registration signal off the always-loaded surface:
`check-kit-registration`'s runner doc becomes `README.md` by default, the
per-kit fixture-runner battery moves from `CLAUDE.md` to README's
"This repo, governed" section, and the CLAUDE.md roster is replaced by a
pointer. The roster is derivable (`gate_fixture_suites` derives it from
`*/gate-tests` on disk; CI runs that derivation), so its only structural
role is check-kit-registration assertion B's registration tripwire — which
does not need to be resident.

## What changes

- **`GATE_SDK_RUNNER_DOC` default flips to `README.md`** (from the
  always-loaded agent file). Rationale: the runner roster serves
  contributors at the moment they run the battery — a load-triggered
  README read — not every session at start; the registration tripwire
  works from any tracked doc. This dissolves the config-injection
  question the queue entry flagged: the kit default is the desired value,
  so no override needs to be emitted anywhere — a stronger answer than
  wiring the config seam the sibling amendment `SPEC-graph-artifact.md`
  gives gate-sdk (that seam exists for knobs whose right value is
  consumer-specific; the runner doc's right value is the same for every
  consumer). With both defaults on `README.md`, assertion A (registry
  row) and assertion B (fixture-runner line) read one doc; the assertions
  stay distinct.
- **README "This repo, governed" gains the fixture-runner battery**: the
  full-battery command it already carries, plus one
  `bash gate-sdk/bin/run-gate-tests.sh <kit>/gate-tests [<kit>/checks]`
  line per kit shipping `gate-tests/`, plus the consumer-gate fixtures and
  the guard-kit decision-table runner — the block CLAUDE.md carries today,
  relocated. Runnable command lines are kept (not bare registration
  markers): `check-docs-cmd` resolves them, contributors paste them, and
  assertion B's grep needs no new grammar.
- **CLAUDE.md sheds the roster**: the "before committing" block keeps
  `run-gates.sh` (the full battery) and points to README for the per-kit
  fixture runners.
- **CONTRIBUTING.md's pointer becomes true**: it already says the README
  "lists" the fixture runners — after this change README actually does.
  No CONTRIBUTING edit is expected beyond verifying the sentence reads
  correctly against the landed README section.
- **gate-sdk/SPEC.md** updates the two sites stating the old default
  (§Layout and configuration; §check-kit-registration).

## Producers and consumers

- The flipped default's producer is `check-kit-registration.sh` line-level
  fallback (`${GATE_SDK_RUNNER_DOC:-README.md}`); its consumers are the
  gate itself and `scripts/check-docs-kit-parity.sh`, which wraps it
  overriding only the registry doc — the wrapper inherits the runner-doc
  default unchanged, so both gates read README with no wrapper edit.
- The gate's producers (invocation paths) are `run-gates.sh`, the
  generated pre-commit hook, and CI — all invoke the check with no env
  override today and continue to; no configuration must be emitted
  anywhere.
- The graph manifest of `check-kit-registration.sh` couples `CLAUDE.md`;
  that couple changes to `README.md` (already present via the registry
  doc), so the hook regeneration (`gen-pre-commit.sh --write`) and the
  check-graph artifact re-emit ride the same unit. The manifest of
  `scripts/check-docs-kit-parity.sh` drops `CLAUDE.md` likewise.
- The always-loaded meter reads the shrunken CLAUDE.md; the baseline
  update is the close-stage act, not this unit's.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration — the `GATE_SDK_RUNNER_DOC`
  default value.
- gate-sdk/SPEC.md §check-kit-registration — the assertion-B doc default
  and the fixture-pair notes if they bake the old default.
- README.md §This repo, governed — gains the battery block.
- CLAUDE.md §This repo is governed by its own kits — sheds the per-kit
  lines, keeps full battery + pointer.
- Fixture pairs under `gate-sdk/gate-tests/check-kit-registration/` and
  `scripts/gate-tests/check-docs-kit-parity/` — re-point any baked
  runner-doc default.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge
      (`ls gate-sdk/SPEC-runner-doc.md` finds nothing; the sibling
      graph-artifact amendment lives its own lifecycle).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
