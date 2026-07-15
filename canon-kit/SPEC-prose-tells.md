# SPEC amendment: prose-tells

## What changes

A new gate, `check-prose-tells` (`canon-kit/checks/check-prose-tells.sh`,
skeleton-derived, satisfying the four contracts of gate-sdk/SPEC.md §The gate
model, with a `good/`+`bad/` fixture pair under
`canon-kit/gate-tests/check-prose-tells/`): the mechanical subset of the
AI-prose tells — the machine-detectable writing patterns that mark
agent-authored reader-facing prose — over consumer-configured markdown
surfaces. Judgment-dependent tells (voice, argument shape, hedging) are out of
scope permanently: this gate never grows heuristics that would make it
probabilistic; the non-mechanical read stays human (this iteration: a one-time
validate pass).

Assertions, each mechanical and threshold-gated (a paragraph is a blank-line
delimited block; a section is a `##`-headed span):

- **A. Em-dash density** — a paragraph carrying more than
  `CANON_KIT_PROSE_TELL_EMDASH_MAX` (default `2`) em-dashes.
- **B. Throat-clearing phrases** — any case-insensitive match from
  `CANON_KIT_PROSE_TELL_PHRASES`, an array with a bundled generic-English
  default (`It's worth noting`, `It is worth noting`, `It's important to
  note`, `That said`, `Needless to say`, `It goes without saying`); a
  consumer overrides or extends. Generic English is kit-shippable (the
  `CANON_KIT_TEMPORAL_MARKERS` precedent); a consumer's own vocabulary never
  becomes a kit literal.
- **C. Contrast cadence** — the "not X — it's Y" shape (a `not …` clause
  resolved across an em-dash or `, but` into `it's`/`it is`), more than
  `CANON_KIT_PROSE_TELL_CONTRAST_MAX` (default `1`) times per section.
- **D. Undefined abbreviations** — an all-caps token of length ≥ 3 used in a
  file that never expands it (no parenthesized expansion at any occurrence)
  and absent from `CANON_KIT_PROSE_TELL_ABBR_ALLOW`, an array with a bundled
  default of universal tokens (`API`, `CLI`, `URL`, `HTML`, `CSS`, `JSON`,
  `YAML`, `CI`, `SDK`, `SSO`, `DNS`, `HTTPS`); a consumer extends.
- **E. Sentence-rhythm variance** — a paragraph of at least
  `CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES` (default `4`) sentences whose
  sentence word-count coefficient of variation falls below
  `CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN` (default `0.25`) — metronomic cadence.
- **F. Tricolon density** — more than `CANON_KIT_PROSE_TELL_TRICOLON_MAX`
  (default `2`) `A, B, and C` list-of-three constructions per section.

Exact detection regexes are implementation, owned by the gate source; the
fixture pair is the executable statement of each boundary (`bad/` trips every
assertion, `good/` passes all). Thresholds are calibrated at build against
this repo's docs set before registration and may move; the defaults above are
the design intent.

**Scope knob** — `CANON_KIT_PROSE_TELL_GLOBS`, an array of repo-root-relative
globs, default empty ⇒ the gate passes with nothing scanned. Which surfaces
carry reader-facing prose is the consumer's editorial posture (the
`QUEUE_KIT_PROSE_SURFACE_GLOBS` precedent), and per the provenance seam a
consumer's scoping never lands as a kit literal. This repo opts in the
hand-authored docs living pages (top-level `docs/*.md`), excluding the
generated kit mirror (`docs/<kit>/`) and the immutable `docs/posts/` (a gate
that forces edits to immutable pages contradicts their immutability).

**Valve** — `prose-tell-exempt: <reason>` as an HTML comment on or directly
above the offending paragraph exempts that paragraph only; the reason is
mandatory (the `comment-tier-exempt:` convention — a deliberate stylistic
choice carries its cause in-line, and a reasonless valve is red).

## Producers and consumers

- **Producer**: the consumer's gate battery — registration by name in
  `scripts/gates.list`, a `# graph:` manifest, then the regenerated pre-commit
  hook and graph artifact (`gen-pre-commit.sh --write`,
  `check-graph.sh --emit`). Enabling config actually set: this repo's
  `scripts/canon-config.sh` gains `CANON_KIT_PROSE_TELL_GLOBS` in the same
  build unit — without it the gate is a deliberate green no-op, which is the
  correct unconfigured-consumer behavior, not a dead producer.
- **Consumers**: the committing session reads the red verdict and fixes the
  prose or lands a reasoned valve (oracle-first); `run-gate-tests.sh` consumes
  the fixture pair; the enforcement map regenerates with the new class row.
- **Field readers**: every `CANON_KIT_PROSE_TELL_*` knob is read by
  `check-prose-tells` at scan time and by no other component; the roster
  entries follow the knob-citation discipline (`check-knob-citation`,
  `check-knob-default-coupling`).

## Existing sections updated

- canon-kit/SPEC.md §Layout and configuration — knob roster gains the
  `CANON_KIT_PROSE_TELL_*` entries above.
- canon-kit/SPEC.md §Per-component contracts — new §check-prose-tells section
  (assertion set, scope knob, valve, fixture statement).
- canon-kit/README.md — gate roster row.
- Registration side, all mechanical and freshness-gated: `scripts/gates.list`,
  the regenerated pre-commit hook, `docs/check-graph.html`,
  `docs/enforcement.md`, the value rollup, and the docs mirror.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
