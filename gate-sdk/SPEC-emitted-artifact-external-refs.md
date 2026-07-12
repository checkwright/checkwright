# SPEC amendment: emitted-artifact-external-refs

## What changes

`check-graph`'s artifact assertions gain an **external-reference assertion**
over the emitted HTML, mechanizing the self-contained-artifacts convention's
external half (its stated honest limit: absolute `://` URLs and JS `import`
statements pass unseen, caught by review, not the gate).

- **Assertion.** In the emitted artifact, every absolute (`://`-carrying) URL
  appearing as an `href=`/`src=` attribute value, and every ESM `import`
  specifier, must prefix-match the allowed set; any other external reference
  is red, the finding naming the URL and the artifact.
- **Kit-sanctioned seed (mechanism, not config).** The pinned-major mermaid
  ESM import prefix — the one sanctioned external reference, which
  `emit_graph` itself emits — is always allowed; a consumer cannot
  accidentally lock the kit's own emission out.
- **New knob.** `GATE_SDK_GRAPH_EXTERNAL_REFS` (space-separated URL prefixes,
  default empty) — consumer-sanctioned additional prefixes, joining the knob
  roster in gate-sdk/SPEC.md §Layout and configuration. The consumer case that
  motivates it: `graph_theme_header`/`graph_theme_footer` chrome may emit
  absolute site links (this repo's theme links its docs host and its GitHub
  repo), and those are consumer rule content the kit must not hardcode — the
  `graph-vocab` seam pattern.
- **Scope and honest limit.** The scan covers `href`/`src` attribute values
  and ESM import specifiers. Residual vectors — CSS `url()`, a `fetch()` in
  inline script — stay review-caught; the updated §check-graph honest-limit
  sentence names that residue. XML-namespace attribute values (`xmlns=`) are
  neither `href` nor `src` and stay out of scope.
- **Fixture.** The check-graph fixture pair gains a bad case whose theme emits
  an un-allowlisted absolute reference; the good case covers the seeded
  mermaid import plus a knob-allowed prefix.

## Producers and consumers

- **External references** — producers: `emit_graph` (the mermaid import) and
  the consumer's theme override functions (chrome links). Consumer: the new
  assertion inside `check-graph`'s existing artifact pass (both artifact
  homes, the workflow-dir default and the docs republication, exactly as the
  freshness byte-compare already walks them).
- **`GATE_SDK_GRAPH_EXTERNAL_REFS`** — producer: the consumer's gate config
  (the `<KIT>_<KNOB>` env shape); this repo sets its two chrome prefixes
  beside its other gate-sdk knobs. Reader: `check-graph`'s external-ref
  assertion at scan time — the single named reader.
- **The red** — producer: `check-graph` at precommit tier; consumer: the
  committing session via the battery/hook.

## Existing sections updated

- gate-sdk/SPEC.md §check-graph: the sanctioned-exception paragraph (mermaid
  as "the only sanctioned external reference" and the asserts-relative-only
  honest limit) is rewritten around the new assertion — the exception becomes
  the seeded allowlist entry, and the honest limit narrows to the residual
  vectors above.
- gate-sdk/SPEC.md §Layout and configuration: knob roster gains
  `GATE_SDK_GRAPH_EXTERNAL_REFS` with its default (the owning SPEC owns knob
  defaults).
- CLAUDE.md §Conventions established in gate-sdk: the self-contained-artifacts
  bullet's "one sanctioned exception and its honest limit" pointer stays
  valid — the pointed-at section changes, the bullet does not.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
