# SPEC amendment: docs-render-fidelity-gate

site-kit gains the gate that exercises the *rendered* docs artifact. GitHub
Pages renders through kramdown's GFM parser, which diverges from
github.com's cmark: consecutive fenced blocks inside one list item corrupt
the page (the second fence prints literally; a contract-skeleton line
becomes a heading), so a source-green tree shipped garbled Install sections
with no gate in the path. The faithful-artifact-verification class,
mechanized for this artifact.

## What changes

- `site-kit/checks/check-docs-render-fidelity.sh` — for every tracked
  markdown page under the docs dir (underscore-prefixed dirs excluded),
  strip front matter, render with the pinned Pages parser, and assert the
  known divergence class:
  1. **No fence leakage** — the rendered HTML's text content carries no
     literal fence marker; a leaked backtick run is the failure's
     signature regardless of which construct confused the parser.
  2. **No heading leakage** — every rendered heading element corresponds to
     a source heading line that the gate's own fence-aware scan (cmark
     rules) places *outside* any code context; a heading born from inside a
     fenced or indented block is the promoted-`#`-line failure.
- Knobs, in the kit's `<KIT>_<KNOB>` shape with this repo's layout as
  defaults: the docs dir (default `docs`), and the renderer invocation
  (default the kramdown CLI with GFM input — the parser GitHub Pages pins).
- **Fail-closed on the missing oracle**: an unresolvable renderer exits 2
  with a help line naming the dependency — a gate that cannot run its
  oracle refuses; it never silently passes. The dependency ruling: ruby
  plus the kramdown-parser-gfm gem joins the *consumer's* toolchain only
  when the consumer registers this gate — it stays outside env-probe's
  probe-set floor, and `docs/install.md`'s Requirements prose states the
  tier (SPEC-os-support.md owns that page's ruling). Registration stays the
  consumer's choice by the registry-not-array convention: a consumer with
  no published docs site never installs the dependency.
- Honest limit, stated in the gate section: this is not a full render-diff
  between parsers; it mechanizes the observed leakage class (fences,
  headings) and stays silent on divergences that corrupt neither.
- Ships per the kit-landing checklist (gate-sdk/SPEC.md §Consumer smoke):
  `good/`+`bad/` fixture pair — the bad fixture is the real bug shape, a
  list item carrying consecutive fenced blocks; the good fixture the
  indented-block restructure that fixed it — registration in this repo's
  `scripts/gates.list`, precommit tier, `# graph:` manifest coupling the
  docs tree.

## Producers and consumers

- The rendered artifact: produced by the pinned renderer invocation inside
  the gate run (hermetic — nothing is fetched; the pin is the local gem,
  named by the knob); consumed by the gate's two assertions.
- The gate: invoked by the generated pre-commit hook via the registry;
  read by committers and the fixture runner.
- site-kit's monitor boundary (site-kit/SPEC.md §The monitor boundary) is
  untouched: this is tree-verifiable and deterministic, so it lands on the
  gate side, not the monitor side.
- No new fields; the knobs above join site-kit's knob roster with their
  defaults stated in the SPEC (the coupling the sibling
  SPEC-knob-default-coupling.md gate then holds).

## Existing sections updated

- `site-kit/SPEC.md` §Layout and configuration — the knob roster gains the
  docs-dir and renderer knobs with defaults.
- `site-kit/SPEC.md` — the new gate's contract section lands beside
  §check-docs-cname-parity.
- `site-kit/README.md` — the gate joins the kit's roster line.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls site-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
