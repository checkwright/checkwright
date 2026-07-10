# SPEC amendment: count-shapes

Closes the three restatement shapes `check-manifest-count` misses
(queue: `manifest-count-shapes`). The gate's invariant is unchanged — a
pinned total for a growing governed collection is a second source no gate
reads — this widens the matched grammar to the shapes prose actually
reaches for.

## What changes

Three new match branches in `check-manifest-count`'s scan, sharing the
existing cardinal grammar, exemption ladder, and output contract:

1. **Wedged modifier** — `<cardinal> <wedge> <collection-noun>` where the
   wedge is 1..`SPEC_KIT_COUNT_WEDGE_WORDS` (new knob, default `2`)
   intervening words (`nine generic rules`, `three governed surfaces`-class
   phrasing; the adjacency-only grammar is what let a stale
   `the nine generic rules` pass). A wedge containing a **partitive marker**
   (`of`, `out of`) exempts the match — `three of the twelve gates`,
   `nine out of ten kits` are proportions, and in a partitive neither
   cardinal is a restated total, so the denominator side
   (`<of|out of> [the] <cardinal> <noun>`) is exempted by the same
   prefix test. Partitive markers are fixed mechanism (generic English,
   the `all but` precedent), not config.
2. **Noun-then-range** — `<collection-noun> <d+>-<d+>` (`rules 1-8`,
   `gates 1-42`): pins *both* endpoints of an ordered collection and rots
   on every append. Same exemption ladder (inline code, fences, per-site
   `manifest-count-exempt:`).
3. **`rules` joins `SPEC_KIT_COUNT_COLLECTIONS`** — guard-kit's generic
   ruleset is exactly the growing-collection class the default list
   enumerates; the omission is why the motivating hit was invisible.

**Allowed-phrases default retired.** `contracts` has never been a scanned
collection noun, so the shipped default `("the four contracts")` is dead
config that *documented* an adjacency miss instead of fixing it — the gate
could never have fired on that phrase. The default becomes `()` (empty);
the knob and its containment semantics stay (it is the valve for a
genuinely fixed named set whose noun *is* scanned). The SPEC's example
text updates accordingly.

Calibration at build: sweep the manifest set with the widened grammar and
disposition every new hit by the existing ladder (reword to cite the
owning collection, or site-exempt with reason) before the gate lands red.

## Producers and consumers

- Producer: the generated pre-commit hook / `run-gates.sh`, unchanged.
- Consumer: the committing operator via the existing output contract; each
  new branch reports `file:line` plus the matched span at the same single
  scan transition.
- `SPEC_KIT_COUNT_WEDGE_WORDS` — read by the scan when assembling the
  wedged-modifier regex; consumer-overridable in `spec-config.sh`;
  default `2` is set by the kit loader, so the branch is live in every
  deployed configuration.
- The count grammar (cardinal alternation + noun list + range shape)
  factors into `lib/spec.sh` as a shared adapter — consumed here and by
  the sibling comment-count extension (`spec-kit/SPEC-comment-count.md`),
  so the consumer vocabulary (`SPEC_KIT_COUNT_COLLECTIONS`) enters once.

## Existing sections updated

- §check-manifest-count: grammar paragraph gains the wedge and range
  branches and the partitive exemption; the allowed-phrases sentence loses
  the `the four contracts` default and its "shipped vocabulary" rationale;
  the knob list gains `SPEC_KIT_COUNT_WEDGE_WORDS`.
- §Layout and configuration: knob list gains `SPEC_KIT_COUNT_WEDGE_WORDS`;
  `SPEC_KIT_COUNT_COLLECTIONS` default gains `rules`;
  `SPEC_KIT_COUNT_ALLOWED_PHRASES` default becomes empty.
- §lib/spec.sh: names the shared count-grammar adapter.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (`the four contracts` as an allowlist default); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Fixture pair per shape** — bad: wedged modifier, noun-then-range,
      a `rules` hit; good: partitive idioms, comparator/threshold, wedge
      beyond the window.
