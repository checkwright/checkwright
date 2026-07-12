# SPEC amendment: resident-trigger

## What changes

lifecycle-kit gains the agent-file injector that gives the stage skills their
resident load-trigger — today install step 3 adopts the skills into the
skills dir but never touches the always-loaded agent file, so the consumer
hand-authors the breadcrumb or the skills sit undiscovered.

**Injector ruling: generated marker block, not a paste-in snippet** — a
parity gate needs a generated target (the doctrine-kit precedent). New
`bin/install-lifecycle.sh` (lifecycle-kit): idempotently (re)writes a block
bounded by `<!-- lifecycle-kit:begin -->` / `<!-- lifecycle-kit:end -->` into
the agent file named by a new knob `LIFECYCLE_KIT_AGENT_FILE` (default
`CLAUDE.md`; the `DOCTRINE_KIT_AGENT_FILE` precedent), appending the block
when absent, replacing it in place when present, refusing on an unpaired
marker.

**Block content ruling: pointer-only, roster derived.** The block carries
the Load-trigger-residency minimum: one line that the repo runs
lifecycle-kit's iteration state machine on the queue file, the stage roster
with its skill invocations (`/scope` … `/close`), and the SPEC link — never
stage prose. The roster is *derived*: the injector sources
`lib/stages.sh` (which honors `LIFECYCLE_KIT_CONFIG_FILE`), so a consumer's
reshaped stage set, queue file, and skills dir flow into the block and a
hand-listed roster never exists.

**Shared-mechanism ruling.** The marker-bounded insert/replace is extracted
into a gate-sdk lib helper (new `lib/inject.sh`, one function taking file,
begin marker, end marker, and block content on stdin);
`doctrine-kit/bin/install-doctrine.sh` refactors onto it in the same unit —
Enforcement-first's structural form: no second copy of the awk replace
logic, so no gate is owed on their divergence. The unified
every-kit-blocks assembler stays unbuilt: the shared lib *is* the
deduplication it promised; a third injector reopens the question.

**Parity gate.** New `check-lifecycle-registration` (lifecycle-kit/checks/):
regenerates the block from the live config and byte-compares it against the
agent file's marker block — an edited, stale, or missing block reddens
(the `check-doctrine-registration` freshness posture, byte-strict like
`check-docs-mirror-fresh`). Registered in this repo's `gates.list`, `# graph:`
manifest coupling the agent file and the lifecycle config, `good/`+`bad/`
fixture pair per the gate-sdk skeleton contract.

**Dogfood.** This repo's `CLAUDE.md` §"This repo is governed by its own
kits" hand-authors the stage-machine paragraph today; the build replaces its
stage-roster sentences with the generated block (hand-authored context that
is not roster/pointer stays outside the markers), re-runs the injector, and
re-checks the brevity budget.

## Producers and consumers

- **Producer:** the consumer's install (README gains step: run
  `bin/install-lifecycle.sh`) and every re-run after reshaping the machine;
  the injector is the only writer of the block.
- **Consumer:** every agent session — the block is always-loaded, and its
  named reader is the session deciding when to invoke a stage skill; the
  gate is the block's second reader (freshness).
- **Knob reader:** `LIFECYCLE_KIT_AGENT_FILE` is read by the injector and
  the parity gate (both resolve it through the config chain); default set in
  `lib/stages.sh` beside the sibling knobs.
- **Existing integration prose:** lifecycle-kit/README.md §Install gains the
  injector step beside step 3; lifecycle-kit/SPEC.md gains the
  §install-lifecycle and §check-lifecycle-registration contracts and the
  knob joins §Layout and configuration's roster.

## Existing sections updated

- `lifecycle-kit/README.md` §Install — new injector step.
- `lifecycle-kit/SPEC.md` — new component contracts; knob roster.
- `gate-sdk/SPEC.md` — the lib section gains the inject helper's contract.
- `doctrine-kit/SPEC.md` §install-doctrine — updated to cite the shared
  helper it now rides.
- `scripts/gates.list`, generated pre-commit hook, `docs/check-graph.html`,
  `docs/enforcement.md` — regenerated for the new gate.
- This repo's `CLAUDE.md` — block installed (dogfood).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — install-doctrine's inlined replace logic
      retired; nothing cites it.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
