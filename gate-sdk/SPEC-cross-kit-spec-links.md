# SPEC amendment: cross-kit-spec-links

**The packer's README rewrite reaches another packed kit's SPEC, so a kit README cites it relatively.** The packer rewrites a packed README's own `SPEC.md` link to the published location (§Consumer payload), but not `../<kit>/SPEC.md`, and `check-packed-links` reds that relative form, since it lands on a withheld path. So a kit README cites another kit's SPEC by a GitHub blob URL. On the docs site those citations leave the site, though the on-site mirror of the cited SPEC exists, and each carries the publisher's host in a kit file, the literal §Consumer payload refuses for the own-SPEC link. Widening the rewrite to `../<leaf>/SPEC.md[#frag]`, for a leaf the same pack packs, lets the tracked README cite relatively: the site mirror keeps the link on-site and the payload gets the published one.

**Measured at authoring.**

- The rewrite is `resolve_own_spec_links(text, leaf, base)` in `native/src/emit/pack_installer.rs:357-403`, matching the literal opener `](SPEC.md` and rewriting `](SPEC.md)` and `](SPEC.md#frag)` to `<base>/<leaf>/SPEC[#frag]`, the fragment copied verbatim. It is called once per kit in the pack loop (`:225`, via `resolve_readme_links` at `:258-271`, the base resolved above the loop at `:208`) and by `check-packed-links` in process (`native/src/gates/packed_links.rs:3`, `:166`, `:183`). The unit test `only_the_exact_own_spec_target_is_rewritten` (`pack_installer.rs:1078-1081`) asserts a `../gate-sdk/SPEC.md` link passes through.
- `grep -n 'github.com/[^)]*/blob/[^)]*SPEC.md' */README.md` finds 14 lines. Twelve are in kit-root READMEs citing another kit's SPEC, all of it gate-sdk's: the payload sentence at line 7 of `canon-kit`, `context-kit`, `delegation-kit`, `doctrine-kit`, `drift-kit`, `evidence-kit`, `guard-kit`, `lifecycle-kit` and `queue-kit`, and at `site-kit/README.md:17`, plus `guard-kit/README.md:43` and `lifecycle-kit/README.md:83`. The other two are outside the widening: `installer/README.md:7`, since `installer/` is packed whole at the package root rather than as a kit leaf, and `gate-sdk/README.md:114`, which cites `installer/SPEC.md`, no kit leaf.
- `bash gate-sdk/bin/run-gates.sh --emit kit-roots` prints the eleven kit roots; `installer/` is not one.
- The docs mirror keeps a relative link to a mirrored file relative (`native/src/emit/docs_mirror.rs`, `rewrite_target` and `is_mirrored`), so `../gate-sdk/SPEC.md#consumer-payload` in a kit README renders as a link to `docs/gate-sdk/SPEC.md` on the site with no generator change.
- `check-packed-links`' `bad/beta-kit/README.md:3` carries `../alpha-kit/SPEC.md` and `bad/expect.txt` pins its finding; `good/expect.txt` pins `2 own-SPEC link(s) resolved`.

## What changes

### (1) The rewrite takes a packed-leaf set and reaches `../<leaf>/SPEC.md`

`resolve_own_spec_links` becomes `resolve_spec_links(text, leaf, packed, base)` {design-bearing}, `packed` being the leaf names of every kit root the pack loop packs, resolved once above the loop beside the base. It rewrites, in addition to the own-SPEC forms, a link whose target is exactly `../<other>/SPEC.md` or `../<other>/SPEC.md#<fragment>`, `<other>` a member of `packed`, to `<base>/<other>/SPEC` or `<base>/<other>/SPEC#<fragment>`. A `<other>` outside `packed`, a longer path, a `./` prefix and a link title pass through untouched, as for the own form. An empty base rewrites nothing.

`check-packed-links` passes the same set, its `gate_kit_roots` leaves, the roots the pack loop's resolver yields, so the gate and the packer read one derivation. The gate's clean line reads `… {} SPEC link(s) resolved to the published location …`.

Tests and fixtures:

- The unit test `only_the_exact_own_spec_target_is_rewritten` becomes one asserting that `../<packed>/SPEC.md#x` is rewritten and `../<unpacked>/SPEC.md`, `../<packed>/SPEC.mdx` and `../<packed>/docs/SPEC.md` are not.
- `bad/beta-kit/README.md:3`'s target becomes `../omega-kit/SPEC.md`, a leaf the fixture does not pack, so A still reds on a withheld path the rewrite has no leaf for; `bad/expect.txt` follows.
- `good/beta-kit/README.md` gains `[alpha-kit's SPEC](../alpha-kit/SPEC.md#a-section)`, and `good/expect.txt`'s count becomes `3 SPEC link(s) resolved`.

### (2) The twelve kit README citations go relative

Each of the twelve kit-root citations measured above changes its target from the `https://github.com/…/blob/master/gate-sdk/SPEC.md#<frag>` form to `../gate-sdk/SPEC.md#<frag>`, its link text and fragment unchanged {mechanical}. `installer/README.md:7` and `gate-sdk/README.md:114` keep their blob links. The docs mirror is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, and `check-md-refs` resolves each new target and anchor in the tree.

### (3) §Consumer payload, installer/SPEC.md §The packer and the site guide state the widened rewrite

Three prose surfaces state the widened rewrite {mechanical}. **Not yet applied.**

In gate-sdk/SPEC.md §Consumer payload, the sentence "A markdown link whose target is exactly `SPEC.md`, or `SPEC.md#<fragment>`, becomes `<base>/<leaf>/SPEC` or `<base>/<leaf>/SPEC#<fragment>`." becomes:

> A markdown link whose target is exactly `SPEC.md` or `../<leaf>/SPEC.md`, either optionally followed by `#<fragment>`, becomes `<base>/<leaf>/SPEC` or `<base>/<leaf>/SPEC#<fragment>`, `<leaf>` being the linking kit's own in the first form and the named one in the second.

The first bound becomes:

> - **`<leaf>` is a kit this pack packs.** For `SPEC.md` it is the packer's own directory name for the kit it is packing, and for `../<leaf>/SPEC.md` it must be a member of the pack loop's kit set, resolved once above the loop. A link naming any other directory is not rewritten.

The paragraph's opening, "**A kit README keeps its SPEC link relative in the tree**", becomes "**A kit README keeps its SPEC links relative in the tree, its own and another kit's**", and its "the README's relative links to it would dangle" becomes "the README's relative links to any kit's SPEC would dangle".

In installer/SPEC.md §The packer, the second write's "so an own-SPEC link resolves to the published location rather than to the withheld `SPEC.md`" becomes "so a link to its own or another packed kit's `SPEC.md` resolves to the published location rather than to the withheld file".

In docs/site-architecture.md, "A kit README cites its own SPEC relatively and another kit's SPEC by blob link, since the installer payload withholds every SPEC and resolves only a README's own." becomes "A kit README cites any kit's SPEC relatively, and the installer's packer resolves each such link in the payload, which withholds every SPEC (gate-sdk/SPEC.md §Consumer payload)."

### (4) The site mirrors follow

`docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md` and the twelve kits' mirrored `docs/<kit>/README.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing deltas 2 and 3 {mechanical}.

## Producers and consumers

- **The packed-leaf set.** Producer: the pack loop's kit-root resolver, read once above the loop; `check-packed-links`' `gate_kit_roots`, the same derivation. Consumer: `resolve_spec_links`, at each README's rewrite. No other reader.
- **The widened rewrite.** Producer: `--pack-installer`, on every pack with a non-empty `GATE_SDK_SPEC_BASE_URL` (this repository's release path sets it: `scripts/gate-sdk-config.knobs`). Consumers: the packed README an adopter reads in the payload, and `check-packed-links`, which asserts over the same function's output (A: no withheld target survives; B: an empty base changes nothing).
- **A reader whose verdict moves.** `check-packed-links` A stops reddening a `../<packed>/SPEC.md` link and still reds any other link landing on a withheld path; B is unchanged. `check-docs-mirror-fresh` reds until the mirrors are regenerated (delta 4). `check-md-refs` resolves the new relative targets against the tree; `check-citation-link` reads link form and anchor, unchanged. The consumer smoke vendors by direct copy and bypasses the packer (§Consumer payload), so it reads no packed README.
- **No new knob, state or event.**

## Existing sections updated

Roster produced by `grep -rn 'resolve_own_spec_links\|own-SPEC' native/src gate-sdk installer docs/site-architecture.md`, the blob-citation grep above, and reading §Consumer payload's rewrite paragraphs.

- `native/src/emit/pack_installer.rs` — the function, its caller, its doc comments at `:255-257` and `:352-356`, and the unit tests (delta 1).
- `native/src/gates/packed_links.rs` — the import, both calls and the clean line (delta 1).
- `gate-sdk/gate-tests/check-packed-links/` — `bad/beta-kit/README.md`, `bad/expect.txt`, `bad/args`' comment, `good/beta-kit/README.md`, `good/expect.txt`, `good/args`' comment, `good/alpha-kit/README.md`'s own-SPEC line (delta 1).
- `canon-kit/README.md`, `context-kit/README.md`, `delegation-kit/README.md`, `doctrine-kit/README.md`, `drift-kit/README.md`, `evidence-kit/README.md`, `guard-kit/README.md`, `lifecycle-kit/README.md`, `queue-kit/README.md`, `site-kit/README.md` — the relative citations (delta 2).
- `gate-sdk/SPEC.md` — §Consumer payload (delta 3).
- `installer/SPEC.md` — §The packer (delta 3).
- `docs/site-architecture.md` — the section-citation rule (delta 3).
- `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md` and each `docs/<kit>/README.md` mirror of the ten READMEs above — regenerated (delta 4).

## Retired spellings

- `resolve_own_spec_links` — the rewrite's function name, which now reaches another kit's SPEC as well; replaced by `resolve_spec_links` (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
