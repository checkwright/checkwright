# SPEC amendment: enforcement-map

## What changes

An emitted, kit-first map of every check surface — kit → governed surface →
enforcement class — published as a docs page. Never hand-maintained: a
hand-written table is exactly the restated projection the copy-gates exist
to catch, so the page derives from registries.

New emitter **`bin/enforcement-map.sh --emit`** (check-graph's sibling)
writing `docs/enforcement.md`, plus gate
**`checks/check-enforcement-fresh.sh`** asserting the artifact byte-matches
the emitter's current output (the check-graph / trajectory-freshness
byte-compare pattern), `# graph:`-trigger-coupled to every class registry so
a registry change re-fires it.

The class spectrum and its parseable sources — the design ruling is that
every class derives from a registry that already exists, except the monitor
class, which gains a declaration:

- **Blocking gates, by tier** — `gates.list` plus each gate's `# graph:`
  `tier=` field (precommit / commit-msg / align-only); owning kit comes
  from the same name-resolution walk the runner uses (`gate_kit_roots`);
  a consumer-dir gate groups as the consumer's.
- **Advisory KPIs** — the drift-kit `kpis.list` registry (advisory by that
  kit's standing ruling, never gates).
- **Guards** — PreToolUse hook entries in the tracked harness settings
  file (parseable JSON), mapped to the guard scripts they name.
- **Session warnings** — SessionStart hook entries, same source.
- **Validate suites** — evidence-kit's suite registry.
- **Monitors** — the one class with no parseable home today: a non-gate
  surface declares itself with a one-line marker,
  `# enforce: class=monitor <surface free-text>`, which the emitter greps
  (this repo's first carrier: the site-health workflow — deployment truth,
  not tree truth). The `# enforce:` grammar is the new name; only the
  monitor class needs it today, and a future uncovered class reuses it
  rather than growing a bespoke registry.

The emitted page owns the **enforcement-class taxonomy** prose, which today
is spread across kit SPECs and owned nowhere; the emitter's preamble heredoc
is its single source — the SPEC documents the emitter contract and cites the
page for the taxonomy. `docs/enforcement.md` joins the governed docs set by
the existing manifest glob; `docs/index.md` links it as the adoption page
("what does each kit enforce, and how hard" — the evaluating adopter's
first question).

Cross-kit reads are ruled acceptable: the emitter is a reporting surface
(the drift-report precedent already reads across kits), and it degrades
per-class — a consumer missing a registry gets that section absent, so a
gates-only consumer still gets its gate map.

## Producers and consumers

- Emitter producer: the operator/CI regenerating on registry change —
  reachable because the freshness gate rides the generated pre-commit hook
  and its corrective names the regeneration command (the check-graph
  contract exactly).
- Consumers: the evaluating adopter (the docs page) and
  check-enforcement-fresh (byte compare — the whole artifact is its read).
- Per-registry fields: name and tier/class read at emit time into the
  page's rows; the `# enforce:` marker's class token selects the section
  and its free-text becomes the surface cell — every field has that one
  emit-time reader.

## Existing sections updated

- §Per-component contracts: two new sections (emitter contract, freshness
  gate) beside check-graph's.
- CLAUDE.md's regeneration instructions and the consumer README commands
  gain the emit line at build (same motion as CHECK-GRAPH regeneration).
- Registration in this repo's `gates.list`; fixture pair for the freshness
  gate; `enforcement-map.test.sh` covers the degraded-registry paths
  (missing kpis.list, no hooks file) fixtures cannot reach from this tree.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
