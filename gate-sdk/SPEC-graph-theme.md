# SPEC amendment: check-graph theme seam (graph-theme)

The emitted coupling-graph page renders in a bespoke stylesheet that reads
as foreign beside every other page on the consumer's docs host, because the
raw HTML artifact bypasses the site generator's layout. Parity means
inlining the host theme's visual tokens into the emission — the
self-contained-artifact rule bars referencing the site stylesheet — and the
theme is consumer-specific, so the emitter must not hardcode it: the fix is
a theme-injection seam fed by consumer config, the graph-vocab pattern,
never a kit literal.

## What changes

- **`emit_graph` grows a theme seam.** A new knob `GATE_SDK_GRAPH_THEME`
  (default: `graph-theme.sh` in the gates dir, mirroring
  `GATE_SDK_GRAPH_VOCAB`) names a consumer file sourced when present at
  every emission. It may define three optional override functions — the
  function-override pattern `graph_surface_layer` already uses:
  - `graph_theme_css` — emits the `<style>` element's body, replacing the
    kit's default stylesheet;
  - `graph_theme_header` — emits an HTML fragment directly after `<body>`
    (site chrome above the kit header);
  - `graph_theme_footer` — emits a fragment directly before `</body>`.
  An absent file or an undefined function falls back to the kit default,
  and a themeless consumer's output stays byte-identical to today's.
- **Determinism.** Assertion E's in-memory emission and the `--emit` a
  consumer redirects into the artifact resolve the same theme path, so the
  byte-compare stays deterministic; the artifact remains generated-only,
  never hand-edited — a styling change lands in the theme file (or the
  emitter) and is regenerated.
- **Self-containment unchanged.** Injected content is inline; a theme
  emitting a relative asset href must resolve under the artifact dir or
  assertion F is red — the existing gate already polices the
  link-the-site-stylesheet shortcut into inlining.
- **Dark-mode ruling.** The kit default keeps its light+dark scheme; the
  disposition moves to the theme owner. This repo's theme ships both
  schemes: the emitted mermaid init already keys on
  `prefers-color-scheme`, so chrome that ignored the query would clash
  with a dark-rendered graph on the same page.
- **Fixture coverage.** check-graph's gate-tests grow a themed case — a
  theme file whose injected marker provably lands in the emission and
  byte-compares fresh — beside the existing pair, which stays the
  themeless case (gate-sdk/SPEC.md §Fixture-pair discipline).
- **Consumer worklist (this repo).** `scripts/graph-theme.sh` supplies the
  docs-host parity values: the tokens, header fragment, and footer fragment
  are derived from the site layout the docs-site-chrome unit lands (its
  amendment owns the chrome contract, including the `checkwright-theme`
  localStorage key the injected chrome honors — hence that unit builds
  first); then `docs/check-graph.html` is regenerated. The theme values
  are consumer config by seam discipline — this repo's are public site
  identity, committable in `scripts/`, but the kit default stays
  brand-free.

## Producers and consumers

- Theme file: produced as consumer config at the knob's path (this repo:
  `scripts/graph-theme.sh`, in-tree so every clone and CI resolve it);
  consumed by `emit_graph` at emission — both assertion E's compare and
  the redirected `--emit` — the single read transition.
- The three functions: each read once at emission, at its injection point
  (style body, post-`<body>`, pre-`</body>`); a function the consumer does
  not define has the kit default as its reader-visible value, so no
  injection point is ever dangling.
- Knob: `GATE_SDK_GRAPH_THEME` joins the §Layout and configuration roster;
  read by check-graph.sh alone. No hook or enforcement-map change — the
  gate's manifest, tier, and registration are untouched.

## Existing sections updated

- gate-sdk/SPEC.md §check-graph — the artifact description gains the theme
  seam, its fallback rule, and the determinism note.
- gate-sdk/SPEC.md §Layout and configuration — the knob roster row.
- The consumer regeneration instructions (this repo's CLAUDE.md) are
  unchanged — same command, same artifact path.

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
