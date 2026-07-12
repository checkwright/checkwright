# SPEC amendment: knob-default-source-coupling

canon-kit gains the gate that closes the drift channel `check-knob-citation`
leaves open: that gate bars a knob value stated *outside* the owning SPEC,
but nothing couples the default the owning SPEC states to the `:-` fallback
in the source that actually supplies it, so the two drift silently
(surfaced verifying `GATE_SDK_GRAPH_ARTIFACT` against its gate source).

## What changes

- `canon-kit/checks/check-knob-default-coupling.sh` — invariant, for every
  kit-knob default site in kit executable source — both idioms: a
  `${PREFIX_NAME:-value}` fallback expansion, and the guarded assignment
  (`[[ -v PREFIX_NAME ]] || PREFIX_NAME=value` /
  `declare -p PREFIX_NAME &>/dev/null || PREFIX_NAME=(...)`) that is the
  dominant default form in the kits' `lib/*.sh` — whose prefix resolves to a
  vendored kit dir by the established naming convention (the dir name
  uppercased, hyphens to underscores):
  1. **Source self-agreement** — every fallback site for one knob carries
     the same literal; two sites disagreeing is drift inside the source
     before any SPEC is consulted.
  2. **SPEC agreement** — the owning kit's canonical spec states that same
     default for the knob, in the default-statement grammar
     `check-knob-citation` already parses. That grammar lives inline in
     check-knob-citation's own awk today (`_kc_default_bound` /
     `_kc_after_has_literal`), so this unit first extracts it into
     `canon-kit/lib/spec.sh` and re-points check-knob-citation at the shared
     copy — one grammar owner, never a re-implementation. An empty-string
     fallback pairs with a stated empty default.
- Calibration: non-literal defaults — any expansion, substitution, or
  arithmetic inside the fallback or the guarded assignment's value — are
  computed defaults with no single literal to couple; they are
  skipped-and-counted in the clean line, the reads-couples idiom. A knob whose owning SPEC states no default at all
  reds under assertion 2: the SPEC owns knob rosters and default values
  (the config-via-env convention), so a stated-nowhere default is the
  defect, not a valve.
- Ruling carried from filing: the graph-artifact knob's default was
  reviewed and stays site-neutral — this gate pins existing defaults where
  they are; it introduces or relocates none.
- Ships per the kit-landing checklist (gate-sdk/SPEC.md §Consumer smoke):
  `good/`+`bad/` fixture pair, registration in this repo's
  `scripts/gates.list`, precommit tier, `# graph:` manifest coupling kit
  sources to kit SPECs.

## Producers and consumers

- The scanned surface: kit executable source under the vendored kit dirs
  (the same roster the meta-gates walk); producer of the fallbacks is every
  kit author, including future kits — the gate needs no per-kit
  registration because the prefix convention is the resolver.
- The SPEC side: read through the same default-statement grammar
  `check-knob-citation` consumes, so one grammar owner serves both gates —
  a grammar change moves both in one place (`lib/spec.sh`).
- Consumers of the gate: committers via the generated pre-commit hook and
  the fixture runner. No new knobs beyond the gate's own config following
  the `<KIT>_<KNOB>` shape; no new fields.

## Existing sections updated

- `canon-kit/SPEC.md` §check-knob-citation — gains the sibling
  cross-reference (value-placement there, value-agreement here) so a reader
  finds the pair from either end.
- `canon-kit/SPEC.md` §Per-component contracts — the new gate's section
  lands in roster order.
- `canon-kit/README.md` — the gate joins the kit's gate roster line.

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
