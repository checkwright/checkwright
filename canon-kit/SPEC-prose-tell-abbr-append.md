# SPEC amendment: prose-tell-abbr-append

`CANON_KIT_PROSE_TELL_ABBR_ALLOW` and its sibling
`CANON_KIT_PROSE_TELL_PHRASES` are replace-not-append: a consumer adding one
token must copy the kit's entire bundled set verbatim, because assigning the
array overwrites the default rather than unioning with it. This repo's
`scripts/canon-config.sh` carries the full 12-token bundled abbreviation set
reproduced solely to append four local tokens — a literal duplication that
silently diverges if the bundled set ever changes. §Layout and configuration
even promises "A consumer extends either array with its own vocabulary" — an
extension the semantics do not actually provide; that sentence is part of the
defect.

## What changes

1. **`CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA`** and
   **`CANON_KIT_PROSE_TELL_PHRASES_EXTRA`** (arrays, default empty) —
   `lib/spec.sh` appends each to its base array after the base defaults
   resolve: the effective set is base (the bundled default, unless the
   consumer replaced it) plus extra. The base arrays keep replace semantics —
   the narrowing valve stands: a consumer that wants a bundled member *gone*
   still replaces the base array.

2. **Consumer side (this repo)** — `scripts/canon-config.sh` drops the copied
   12-token prefix; `CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA` carries only the
   four local tokens (`SPEC`, `KPI`, `README`, `CNAME`). The comment
   explaining the restatement dies with the restatement. No local
   `PHRASES_EXTRA` value exists today; the sibling knob ships for symmetry —
   asymmetric semantics between the two arrays the same gate reads would be
   its own trap — and the fixture pair is its exerciser.

**Ruling** — candidate (a) union semantics over candidate (b) a freshness
gate coupling consumer copies to the kit default: enforcement-first, removing
the duplication outranks gating it. The generalized check class the queue
entry named (consumer-config-restates-kit-default) is dissolved for these two
arrays by removal. Other bundled-default arrays (`CANON_KIT_TEMPORAL_MARKERS`
et al.) adopt the `_EXTRA` convention on demand — no live consumer
duplication exists for them today, and a preemptive sweep is speculative
generality.

**Provenance seam unchanged** — the extra tokens are consumer vocabulary in
consumer config; the kit ships only the generic-English bundled sets.

## Producers and consumers

- **The `_EXTRA` arrays** — produced by consumer config (this repo:
  `scripts/canon-config.sh`, the deployed enabling config for the
  abbreviation knob; the phrases knob's default-empty state is a no-op merge);
  read at scan time by `check-prose-tells` through the lib merge. The
  §check-prose-tells single-reader sentence ("every `CANON_KIT_PROSE_TELL_*`
  field is read by `check-prose-tells` at scan time and no other component")
  extends to the pair unchanged.
- **The merged effective sets** — produced by `lib/spec.sh` at source time;
  consumed by assertion B (throat-clearing phrases) and assertion D
  (undefined abbreviations).

## Existing sections updated

- **§Layout and configuration** — the "A consumer extends either array with
  its own vocabulary" sentence becomes the `_EXTRA` mechanics: extension via
  the `_EXTRA` knobs, replacement of the base array as the narrowing valve.
- **§check-prose-tells** — assertions B and D read the merged sets; the knob
  roster sentence names the pair.

## Tests

`check-prose-tells` gate-test additions: an `_EXTRA`-supplied abbreviation
passes assertion D with the base set left at its default (no restatement); an
`_EXTRA`-supplied phrase trips assertion B.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
