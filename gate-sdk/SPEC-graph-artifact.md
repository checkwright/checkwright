# SPEC amendment: check-graph-site-move

Publish the coupling-graph artifact on the docs site by moving it — never
copying it — out of the workflow dir: the artifact is pure generated
visualization, self-contained HTML by kit convention, and the site already
hosts gated generated projections (the enforcement map, the evidence data
page). A second committed copy would owe its own parity assertion for no
reader benefit.

## What changes

- **New knob `GATE_SDK_GRAPH_ARTIFACT`** — the path of the emitted
  coupling-graph artifact; the shipped default stays the workflow-dir
  location (`${GATE_SDK_WORKFLOW_DIR}/CHECK-GRAPH.html` resolution,
  unchanged for every existing consumer). `check-graph.sh` assertion E
  (artifact freshness) reads the knob; the gate's help text and the
  emit-command lines it prints emit the *resolved* path so the corrective
  a red run offers is always the consumer's real one.
- **gate-sdk gains the standard consumer-config seam.** `lib/gate.sh`
  auto-sources `${GATE_SDK_GATES_DIR:-scripts}/gate-sdk-config.sh` when
  present, honoring an explicit `GATE_SDK_CONFIG_FILE` first — the same
  loader shape every content-bearing kit ships. This knob is its first
  real user; the seam exists so a consumer can override any layout knob
  persistently (env-only knobs die at the end of the shell that set
  them, which is why nothing overrides them today). Bootstrap note:
  `GATE_SDK_GATES_DIR` locates the config file, so that one knob remains
  env-or-default only — a config file cannot name its own directory.
- **This repo's override**: new `scripts/gate-sdk-config.sh` sets the
  artifact path to `docs/check-graph.html`; the tracked artifact moves
  there (`git mv` + regenerate), and `docs/index.md` links it (a
  same-tree file link, inside the served site).
- **Manifest couples carry both homes.** The `# graph:` manifest of
  `check-graph.sh` is kit-shipped static text (consumers never edit
  vendored files), so it cannot read the knob: it lists both the default
  workflow-dir artifact and `docs/check-graph.html` as couples. For a
  default consumer the docs path simply never stages (an inert trigger
  pattern); for this repo the hook re-fires on the real artifact.
  Couples↔trigger parity holds because the generated hook derives from
  the same manifest.

## Producers and consumers

- Artifact producer: `check-graph.sh --emit` redirected by the operator
  (the regenerate command in CLAUDE.md and gate-sdk/README, swept to the
  new path for this repo). Consumers: assertion E (byte-compares at every
  battery/hook run) and the docs-site reader via the `docs/index.md`
  link.
- Knob producer: `scripts/gate-sdk-config.sh`, auto-sourced by
  `lib/gate.sh` — which every gate sources, so every gate-sdk check sees
  the same resolution; reader: assertion E's path resolution and the
  help-text emitter.
- `evidence-kit/smoke/install.sh` emits the artifact in a scratch
  consumer using the shipped default — no edit; the smoke keeps proving
  the zero-config path.
- Citation sweep: CLAUDE.md (regenerate command), gate-sdk/README.md,
  `.claude/commands/agent-execution.md` (names the artifact path), and
  gate-sdk/SPEC.md sections stating the artifact location.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration — the knob row and the
  config-file loader paragraph (the seam, its resolution order, the
  `GATE_SDK_GATES_DIR` bootstrap exception).
- gate-sdk/SPEC.md §check-graph — assertion E's path resolution and the
  dual-couple manifest note.
- The sibling amendment `SPEC-runner-doc.md` is reworded in the same
  unit that lands first: its rationale claimed gate-sdk deliberately
  ships no config seam; with this amendment the seam exists, and the
  runner-doc design holds for a different reason (the flipped default is
  right for every consumer, so no override is needed anywhere).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge
      (`ls gate-sdk/SPEC-graph-artifact.md` finds nothing; the sibling
      runner-doc amendment lives its own lifecycle).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (the old artifact path in this repo's surfaces); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
