# SPEC amendment: license-text

The license text does not ship with the artifacts, and the license line differs across surfaces. `installer/package.json`'s `files` roster names no LICENSE and `installer/` holds none; npm adds a package-root LICENSE on its own only when one is there, so the package, which is also the Release tarball, ships without the text Apache-2.0 section 4(a) asks a redistribution to give. No kit directory carries one either, so a vendored kit holds none. README.md links `[Apache-2.0](LICENSE)`, installer/README.md says `Apache-2.0.`, gate-sdk/README.md says "see the repository root", which names the adopter's root once vendored, and the other READMEs say nothing. This amendment ships the text in every redistributed artifact and puts one license line on every README.

One queue entry pairs it: [license-text-and-alignment](../TASK-QUEUE.md#license-text-and-alignment). The site footer was fixed at scope and is not touched here.

**The rulings.**

- **The packer places the text; the tree keeps one copy.** The payload is assembled at pack time from the kit roots, so the packer copies the packed tree's license file to the package root and into each payload kit leaf. Eleven tracked copies would need a generator and a freshness gate to stay equal to the root; the packer already reads the commit's bytes, so it derives the copies where they ship.
- **A knob names the file.** The packer is kit mechanism generic over any publisher that redistributes kits, and its inputs are knobs rather than crate literals (installer/SPEC.md §The packer). `GATE_SDK_PAYLOAD_LICENSE` defaults to `LICENSE`, the convention. Empty places nothing, the off member for a publisher that ships no text this way. A set value naming no file tracked at the stamped commit is a refusal, since a redistribution without the text is the defect this closes.
- **The plugin keeps a tracked copy, held by its parity gate.** The marketplace installs `plugin/` from the tagged repository as a `git-subdir`, so the packer never reaches it and the root file never travels with it. `plugin/LICENSE` is a byte copy of the root file, and `check-plugin-parity` gains an assertion holding the two equal.
- **The kit README line is unlinked.** A kit README link to the text cannot survive every place the README is read. A relative `LICENSE` link dangles in the source tree, where the kit directory holds no copy and `check-md-refs` reds it. An absolute link puts the publisher's host in a kit file, which gate-sdk/SPEC.md §Consumer payload refuses for the SPEC link on the same ground. So the line names the file and where it sits.
- **Links where they survive.** The root README links the root file. installer/README.md and docs/index.md, which already carry the host, link the self-repo blob form, which `check-md-refs` resolves against the tree and the site mirror serves unchanged.
- **Out of reach, stated.** `companion/` is not redistributed on its own yet; the unit that ships it owns its text, and its READMEs take the source-tree line meanwhile. `reserve/` is the name placeholder no work develops in, so its README is untouched.
- **The seam.** Kit mechanism: the packer's placement and its knob, the vendoring step. Consumer config: this repo's knob value, which is the default. No private rule content.

**Refused.**

- **Tracked per-kit copies.** They would need a generator and a gate to track one root file, and would put eleven copies of it in the tree.
- **An absolute license link in kit READMEs.** It is the publisher's host in kit files.
- **A gate over the README line.** The obligation with a legal edge is the shipped text, which the packer's placement and the consumer smoke's assertion hold. A missing README line costs a reader one sentence, and a literal-sentence gate would be a kit literal of this repo's wording.

## What changes

### (1) The packer places the license text {design-bearing}

**Not yet applied.** installer/SPEC.md §The packer, the paragraph opening **The packer mutates packed content in exactly two places** is replaced by:

> **The packer writes into the assembly in exactly three places, and they are one roster rather than two documented and one discovered.** The stamp above edits `{asm}/package.json`. Inside the kit loop, immediately after a kit's tracked set is extracted, the packer rewrites `{asm}/payload/{leaf}/README.md` so an own-SPEC link resolves to the published location rather than to the `SPEC.md` the payload withholds. And it places the license text: the file `GATE_SDK_PAYLOAD_LICENSE` names, read from the stamped commit, at `{asm}/LICENSE` and at `{asm}/payload/{leaf}/LICENSE` for every packed kit, so the package and each vendored kit carry the text a redistribution owes. The first two read the same resolved base, which is why it is resolved once **above** the loop rather than beside the stamp below it. gate-sdk/SPEC.md §Consumer payload owns the rewrite's rule and its four bounds and gate-sdk/SPEC.md §check-packed-links owns the gate that reads its output. None reaches a tracked file: all three operate on the assembly scratch, which the pack step tears down.

The footprint's first member, *The tracked set at the stamped commit, under `installer/` and under each root the kit-root resolver yields*, gains *and the license file*. `native/src/emit/pack_installer.rs` implements the placement from `git archive <commit> -- <license>`, refuses at exit 2 when a set value names no file tracked at that commit, and places nothing when the value is empty; its `KNOBS` roster gains `GATE_SDK_PAYLOAD_LICENSE`. Unit tests hold both placements, the empty case and the refusal.

### (2) The knob {mechanical}

**Not yet applied.** gate-sdk/SPEC.md §Layout and configuration gains, after the `GATE_SDK_PAYLOAD_WITHHOLD` bullet:

> - `GATE_SDK_PAYLOAD_LICENSE` (default `LICENSE`): the repo-root-relative license file the packer places at the package root and in every packed kit root (installer/SPEC.md §The packer), so a redistributed kit carries its license text. **Explicitly empty** places nothing; a value naming no file tracked at the packed commit is a refusal.

gate-sdk/SPEC.md §Consumer payload, after the paragraph opening **What opacity does not extend to.** and its four bullets, gains:

> **Each packed kit also carries the publisher's license text**, placed by the packer from `GATE_SDK_PAYLOAD_LICENSE`, because a kit root holds none in the tree and a vendored kit is a redistribution.

`native/src/knobs/gate_sdk.rs` gains the row.

### (3) The npm roster names the file {mechanical}

**Not yet applied.** `installer/package.json`'s `files` roster gains `"LICENSE"`. npm includes a package-root LICENSE unlisted, but installer/SPEC.md §Layout rules that the roster decides what is reachable at `PKG_ROOT`, so the file is named there. installer/SPEC.md §Layout gains a bullet after `profiles.list`:

> - `LICENSE` — the license text, placed at pack time from the repository root (§The packer), never tracked here.

### (4) The consumer smoke asserts the text {mechanical}

**Not yet applied.** `installer/consumer-smoke/run-smoke.sh`, where it extracts the artifact-less tarball it packs: fail unless the extracted package carries `package/LICENSE` and one `package/payload/<leaf>/LICENSE` per `package/payload/<leaf>/README.md`, each equal to the repository's root `LICENSE`; and, after an `init` from that package, unless each vendored kit root carries `LICENSE`. installer/SPEC.md §The consumer smoke names the assertion beside its other packed-tarball assertions. The smoke packs through the real `--pack-installer` on every gates-workflow run and in the validate stage's installer suite, so the placement is exercised long before a tag.

### (5) The plugin carries a tracked copy {mechanical}

**Not yet applied.** `plugin/LICENSE` is added, a byte copy of the root `LICENSE`. plugin/SPEC.md §check-plugin-parity gains:

> - **(F) The plugin carries the license text.** `plugin/LICENSE` equals the repository's root `LICENSE` byte for byte, since the marketplace installs `plugin/` alone. A red prints the remedy, `cp LICENSE plugin/LICENSE`.

The positional form gains a trailing `license` operand, the root file to compare against, so the fixture tree carries its own. `native/src/gates/plugin_parity.rs` implements F, its `good/`/`bad/` fixture trees gain the file (`bad/` a differing copy), and its `couples=` gains `LICENSE`.

### (6) Manual vendoring copies the text {mechanical}

**Not yet applied.** installer/SPEC.md §Vendoring without the installer, step 1's second paragraph gains a closing sentence:

> Copy the repository's root `LICENSE` into the kit directory as well; the installer's payload places it there, and the source tree keeps one copy at its root.

### (7) One license line on every README {mechanical}

**Not yet applied.** Each surface below ends with a `## License` section holding one line; where the section exists its body is replaced.

- `README.md`: `Apache-2.0. The license text is [LICENSE](LICENSE).`
- `installer/README.md`: the trailing `Apache-2.0.` becomes a `## License` section: `Apache-2.0. The license text is [LICENSE](https://github.com/checkwright/checkwright/blob/master/LICENSE), shipped beside this file in the package.`
- Every kit README — the members `gate_kit_roots` yields: canon-kit, context-kit, delegation-kit, doctrine-kit, drift-kit, evidence-kit, gate-sdk, guard-kit, lifecycle-kit, queue-kit, site-kit — and `plugin/README.md`: `Apache-2.0. The license text is \`LICENSE\`, beside this file in an installed copy and at the repository root in the source tree.` gate-sdk/README.md's *Apache-2.0 — see the repository root.* is replaced by it.
- `companion/README.md`, `companion/speckit/README.md`: `Apache-2.0. The license text is \`LICENSE\` at the repository root.`
- `docs/index.md` §License: *Checkwright is Apache-2.0.* becomes *Checkwright is [Apache-2.0](https://github.com/checkwright/checkwright/blob/master/LICENSE).*, the rest of the paragraph standing.

## Producers and consumers

Probe: `git grep -n -i licen` over the tracked tree; `git ls-files '*README.md'`; the `files` roster read; `grep -n '"source"'` over `.claude-plugin/marketplace.json`; `grep -n pack-installer` over `installer/consumer-smoke/run-smoke.sh`, which packs and extracts a tarball.

- **The placed license files** (delta 1). Producer: `--pack-installer`, on every pack, under the default knob this repo leaves set. Consumers: npm's publish and the Release upload, which ship `{asm}` as the one tarball; `init`, which copies each `payload/<leaf>/` into the adopter's tree; the consumer smoke's assertion (delta 4).
- **`GATE_SDK_PAYLOAD_LICENSE`** (delta 2). Reader: the packer. Roster-holding readers: gate-sdk's static table and `--emit knob-roster`; the packer's `KNOBS` declaration, read by the knob-file derivation, `check-reads-couples` and `check-gate-substrate-parity`; `check-knob-citation`, satisfied by the SPEC bullet.
- **`plugin/LICENSE`** (delta 5). Producer: the committing session, by the copy F's remedy names. Consumer: the harness's plugin install from the pinned tag; `check-plugin-parity` F at pre-commit.
- **Red conditions.** F reds until `plugin/LICENSE` lands, so the file and the assertion land in one commit. `check-md-refs` resolves each new link (delta 7): the root README's relative link and two self-repo blob links, all to the tracked root file. The kit and plugin lines carry no link. `check-packed-links` judges only links, and the kit lines add none.
- **Every member's value** (delta 7): the README roster above is `git ls-files '*README.md'` less `docs/`, fixtures and `reserve/`, each member's line named.

## Existing sections updated

Roster probe: the probes above, plus `git grep -n "exactly two places"`.

- `installer/SPEC.md` — §The packer (delta 1); §Layout (delta 3); §Vendoring without the installer (delta 6).
- `native/src/emit/pack_installer.rs` (delta 1); `native/src/knobs/gate_sdk.rs` (delta 2).
- `gate-sdk/SPEC.md` — §Layout and configuration, §Consumer payload (delta 2).
- `installer/package.json` (delta 3); `installer/consumer-smoke/run-smoke.sh` and installer/SPEC.md §The consumer smoke (delta 4).
- `plugin/LICENSE`, `plugin/SPEC.md` §check-plugin-parity, `native/src/gates/plugin_parity.rs`, its fixture trees and its `.gate` descriptor (delta 5).
- `README.md`, `installer/README.md`, the eleven kit READMEs, `plugin/README.md`, `companion/README.md`, `companion/speckit/README.md`, `docs/index.md` (delta 7).
- `docs/installer/SPEC.md`, `docs/installer/README.md`, `docs/gate-sdk/SPEC.md`, `docs/plugin/SPEC.md`, and the `docs/<dir>/README.md` mirror of every README delta 7 edits — the generated on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1, 2, 3, 4, 5, 6 and 7).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **gate-sdk/SPEC.md §Consumer payload**: every packed kit and the package carry the license text as `LICENSE`, placed by the packer from `GATE_SDK_PAYLOAD_LICENSE` (deltas 1 and 2).

## Retired spellings

- None — every delta adds or re-phrases prose, code and files; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — `bash gate-sdk/bin/build-native.sh`, the full battery, and the gate-sdk fixture suite and the installer consumer smoke green on the landing commit. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
