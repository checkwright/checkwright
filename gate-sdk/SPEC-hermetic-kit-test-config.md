# SPEC amendment: hermetic-kit-test-config

## What changes

**The exposure (verified this scope; counts re-verified at align).** Every
kit lib resolves its consumer config as `<KIT>_CONFIG_FILE` env-wins, else its
per-kit config basename under `${GATE_SDK_GATES_DIR:-scripts}/`
**cwd-relative** (`lifecycle-config.sh`, `gate-sdk-config.sh`, … — each kit's
SPEC owns the name). `run-gate-tests.sh` runs fixture pairs with
`cd` into the case dir (hermetic by construction — and some fixtures ship
their own cwd-relative config deliberately), but runs bespoke unit tests
(`gate-tests/*.test.sh`) with the invoker's cwd — repo root in this repo's
battery — so every gate invoked inside a unit test silently inherits this
repo's consumer config. Attested instance: 689cd9c (check-stage-evidence's
strict-boundary cases greened under the consumer's `iteration` posture).
15 of 30 bespoke tests carry no pin of any form today; the pins that exist
are ad hoc — a `_CONFIG_FILE` export, a `GATE_SDK_GATES_DIR` redirect, or a
top-level `cd` — and each covers only the surface it names.

**Fix: one shared bootstrap, not thirty pins.** New lib
`gate-sdk/lib/test-hermetic.sh`, sourced as the first act of every bespoke
`gate-tests/*.test.sh`:

- Derives the kit roster from its own location (subdirectories of gate-sdk's
  parent named `gate-sdk` or `*-kit`), so it needs no config to bootstrap —
  the roster covers kits whose config loader lives in `bin/` scripts
  (context-kit, drift-kit) as well as the nine `lib/` loaders.
- For each kit, exports `<KIT>_CONFIG_FILE` (name uppercased, `-` → `_`)
  pointing at one shared empty file, created idempotently at
  `${TMPDIR:-/tmp}/gate-sdk-hermetic-empty.sh` with `: >` — no trap needed
  (a test's own `trap … EXIT` must not be clobbered), no growth (fixed path,
  always empty), and the file exists because the strict loader shape
  (lifecycle-kit's: six of the eleven) fails closed (exit 2) on a
  set-but-missing `_CONFIG_FILE`; the `${VAR:-default}`-shape loaders
  (gate-sdk's: the other five) skip a missing file silently per their own
  SPECs, so the shared existing file gives both shapes the same no-op source.
- A test that deliberately exercises config behavior overrides *after*
  sourcing — a later assignment or a per-invocation env prefix (the
  check-prose-enum style) wins by ordering. No opt-in flag needed.

Ruled against the two alternatives: **per-test literal pins** (the original
filing's shape; the queue entry now carries this ruling) pin only the test's own kit, but a gate sources several kits'
libs — gate-sdk's, doctrine-kit's, and guard-kit's cwd defaults would stay
live — and thirty copies of pin boilerplate is maintained duplication;
**runner-level pinning** would cover only runner-mediated runs, leaving a
directly-invoked `bash <kit>/gate-tests/x.test.sh` (the dev loop where a
wrong green misleads a build session) exposed, and offers nothing a gate can
enforce.

**Sweep.** Every existing bespoke test (30 files across 8 kits) gains the
source line; 689cd9c's ad-hoc `LIFECYCLE_KIT_CONFIG_FILE` pin is replaced by
it (dedup). Per-case config overrides inside tests are kept — they now
override a hermetic baseline instead of a leaky one.

**Enforcement: new meta-gate `check-test-hermetic`** (gate-sdk, skeleton +
fixture pair per the four contracts):

- Invariant: every `gate-tests/*.test.sh` under the configured kit dirs
  either contains a source of `lib/test-hermetic.sh` or carries an explicit
  `# hermetic-exempt: <reason>` marker line (the valve for a test that
  proves hermeticity otherwise).
- Tier `precommit`; `# graph:` couples the gate-tests trees (dir=many).
- Registration ripple: `scripts/gates.list`, regenerate the pre-commit hook
  (`gen-pre-commit.sh --write`), the enforcement map, the coupling graph,
  and the docs mirror (gate-sdk SPEC changes) — the standard generated-
  projection set.

## Producers and consumers

- **Producer:** each bespoke test sourcing `test-hermetic.sh` at start (the
  source line the sweep adds is the enabling config, present in every
  shipped test — enforced, not assumed, by the meta-gate).
- **Consumers:** every kit config loader reading its `<KIT>_CONFIG_FILE` env
  var (the nine `lib/` loaders and the `bin/`-resident loaders); each reads
  the shared empty file and falls through to kit defaults — the exact
  contract a unit test asserts against.
- **Fields:** the exported env vars, reader named above; the empty file,
  read (as a no-op source) by the same loaders. The meta-gate's marker line
  `# hermetic-exempt:` is read by `check-test-hermetic` alone.

## Existing sections updated

- `gate-sdk/SPEC.md §run-gate-tests` — states the unit-test hermeticity
  contract (bespoke tests run on kit defaults; the bootstrap is how) and
  that fixture-pair hermeticity comes from `cd` into the case dir.
- `gate-sdk/SPEC.md` gains §check-test-hermetic (invariant, valve,
  calibration) and a §lib/test-hermetic.sh home for the roster-derivation
  and empty-file mechanics (knob-free by design — a config-pinning tool
  cannot itself be configured by the surface it pins).
- `gate-sdk/SPEC.md §Consumer smoke` kit-landing checklist — a new kit's
  bespoke test sources the bootstrap (one clause; the meta-gate enforces).

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
      retired (the ad-hoc pin comment in check-stage-evidence.test.sh);
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
