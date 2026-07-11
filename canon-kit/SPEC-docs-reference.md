# SPEC amendment: docs-reference-routing

How a reader reaches full reference from the deliberately-lite docs site.
Today kit pages name `SPEC.md` as bare inline code with no link, so the
served site dead-ends. Ruling: point, don't generate — the repo tree is the
reference tier, cited from the site by absolute GitHub links; on-site
generated SPEC projections stay demand-gated (SPEC-internal relative links
would need rewriting at emit — a real emitter, not a copy — and no demand
attests that cost yet).

## What changes

- **The reference-link grammar** (owned post-merge by canon-kit/SPEC.md
  beside §check-docs-link-convention): a docs-site page cites an in-repo
  reference artifact with an absolute link
  `https://github.com/checkwright/checkwright/blob/master/<path>[#anchor]`
  — anchored when citing a section (the downward-citation shape extended
  off-site). The ref is pinned to `master` permanently: the site is
  living documentation of the current tree (dated posts are the immutable
  register), and a vendored consumer reads its own vendored copy, so
  tag-pinned reference links buy staleness, not stability. Tag-pinned
  reference copies become a question only if the upgrade-path or
  launch-comms rungs surface real demand.
- **Each kit docs page gains a reference block**: an anchored link to the
  kit's `SPEC.md` (and `README.md` where the page cites it), replacing
  the bare inline-code mentions. `docs/methodology.md` / `docs/install.md`
  adopt the same grammar where they name in-repo artifacts.
- **`check-md-refs` gains a self-repo blob-link pass**: an absolute link
  whose normalized prefix matches this repo's `origin` remote (both
  `https://` and `git@` remote forms normalize to the same repo identity
  — derived, never a configured literal, so the kit ships no repo name
  and the provenance seam holds) is resolved locally: the `<path>` must
  be git-tracked, and the `#anchor` must match a heading in that file
  under the docs-host slugging rules (lowercase, spaces to hyphens,
  punctuation dropped — implemented beside the section adapters in
  `lib/spec.sh`). A repo with no `origin` skips the pass (nothing to
  match — the links cannot be identified as self-repo). Gate-economy:
  owed — the surface must exist once the site points off-tree, and a
  renamed heading or moved SPEC dangles every reference link silently;
  the check is local (no network), riding a gate that already walks
  every doc.

## Producers and consumers

- Link producers: the docs pages (hand-authored prose). Consumers: the
  site reader (follows to GitHub's rendered view) and the new
  `check-md-refs` pass at its scan transition.
- The self-repo identity's producer is `git remote get-url origin` at
  gate runtime; its reader is the link classifier in the same pass. CI's
  checkout sets `origin`, the generated hook runs in the clone —
  both producers emit it; the no-remote skip covers a bare scratch
  consumer (the fixture pair covers both branches).
- The slugging adapter's consumer is the anchor resolver at the same
  transition; no persistent state anywhere.
- `check-docs-link-convention` is untouched: it owns relative-link
  *shape* inside the docs tree; absolute reference links are the new
  pass's charge — the split is stated where both are specified.

## Existing sections updated

- canon-kit/SPEC.md §check-md-refs — the new pass, the identity
  derivation, the skip rule.
- canon-kit/SPEC.md §check-docs-link-convention — one sentence naming the
  charge split with the new pass.
- The kit docs pages under `docs/` — the actual link edits (build work,
  enumerated by grepping the bare `SPEC.md` inline-code mentions).

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
