# SPEC amendment: asset-names

The one-line install's two hosted scripts and the install page's step-by-step recipes each spell the Release tarball's name and its sidecar's. Nothing compares them with the release-assets declaration in gate-sdk/SPEC.md §Consumer payload, the one statement of what a Release carries. A rename that moves the declaration and the publish workflow together reds only at the next tag's `pack` job. The fetch surfaces then keep fetching a name no Release carries once the pin moves, and an adopter's one-line install breaks.

**The ruling: a gate, as a third invariant of `check-install-pin`.** The surfaces cannot derive their names at run time, because a hosted script is served verbatim and knows nothing but its pin. Generating them would make two hand-reviewed install scripts into projections to save three string literals. `check-install-pin` already reads both scripts and resolves their pin, and the declaration that binds them is the **pinned release's**, not the tree's. The pin trails the tag (§The hosted install pin), so between a rename landing and the tag, the scripts must still fetch the old names. A gate reading the tree's declaration would red exactly that correct state. Read at `v<pin>`, the invariant fires where the names really move: the commit that moves the pin.

**The seam.** The unit is repo-local. `check-install-pin` is this repository's gate, and it reads gate-sdk's declaration grammar without changing it. No kit ships the invariant.

**Refused.**

- **A new gate.** Its surfaces, its pin and its dormancy are `check-install-pin`'s, and a second gate would re-read all three.
- **A fetch-surface knob on `check-release-assets`.** That gate holds the tree's declaration against a tag's pack output, so its subject is the next release. These surfaces follow the pinned one, and one gate holding both would read two versions of one line.
- **Holding the install-smoke legs.** `.github/workflows/gates.yml`'s `install-smoke-sh-linux` and `install-smoke-pwsh-windows` spell the tarball too, to place the packed one where the page's fetch fence would put it. A leg whose name drifts from the page's install block reds at push, when that block reads a file the leg never placed, so the leg is already held by the page it runs.

**Measured at authoring.**

- **The declaration.** gate-sdk/SPEC.md carries `<!-- release-assets: checkwright-{version}.tgz checkwright-{version}.tgz.sha256 checkwright-gates-{version}-{target}.tar.gz -->`, and `git show v0.26.0:gate-sdk/SPEC.md` carries the same line. `0.26.0` is both scripts' pin.
- **The token rule.** Over docs/install.md, docs/install.sh and docs/install.ps1, the rule in delta 1 reads 13, 1 and 1 tokens. Normalized, they are `checkwright-{version}.tgz` and `checkwright-{version}.tgz.sha256` on the page and `checkwright-{version}.tgz` in each script. All are declared templates. Each script names the digest `"$cw_tgz.sha256"` or `"$tgz.sha256"`, which is no token.
- **The sidecar convention.** Every surface derives the digest's name by appending `.sha256`, as a token on the page and by string suffix in both scripts.

## What changes

### (1) Invariant C — the fetched names are the pinned release's {design-bearing}

**Not yet applied.** installer/SPEC.md §The hosted install pin re-phrases its opening and gains a third invariant:

> The one-line install's two scripts and docs/install.md's step-by-step recipes fetch one pinned release (§The dependency boundary, *The one-line install*). Each script names that release's version on one pin line: `pin='X.Y.Z'` in `docs/install.sh` and `$pin = 'X.Y.Z'` in `docs/install.ps1`. All three **fetch surfaces** spell the names of the assets they download. `check-install-pin` (this repo's `scripts/`) holds three invariants:
>
> - **Invariant A — the twins agree.** *(unchanged)*
> - **Invariant B — the pin is the newest release.** *(unchanged)*
> - **Invariant C — the fetched names are the pinned release's.** An **asset token** is a maximal run of `[A-Za-z0-9._{}$-]` containing `.tgz` or `.tar.gz`, with each variable reference in it (`$name`, `${name}`) read as `{version}`. Each fetch surface carries at least one asset token. Each token equals a template of the release-assets declaration (gate-sdk/SPEC.md §Consumer payload) as the tag `v<pin>` carries it. For each template a token matches, that declaration also carries the template with `.sha256` appended, since every surface names the digest by that suffix.
>
> C reads the declaration at the pinned tag, not in the tree. The pin trails the tag, so a rename that has landed but not shipped leaves the surfaces correctly fetching the old names. C fires in the commit that moves the pin, which must carry the surfaces' new names with it.

The fail-closed paragraph gains C's refusals. Exit 2 on:

- an unreadable page;
- a pinned tag that exists and carries no declaring doc;
- a declaring doc that carries no `release-assets` line, or more than one.

Where `v<pin>` does not resolve, as in a shallow checkout, C is dormant beside B, and the clean line says so.

The positional form becomes `check-install-pin [install-sh install-ps1 install-md pinned-doc [version]]`. With it, the pinned declaration is read from the file `pinned-doc`, so a fixture holds still as the tags move.

§The hosted install pin gains a closing **Honest limits** paragraph:

> A name assembled from parts, such as `"checkwright-" + $v + ".tgz"`, is no token, so C cannot see it. The install-smoke legs spell the tarball as well and are left out, because a leg that places a name the page's install block does not read reds that leg at push.

### (2) The implementation {design-bearing}

**Not yet applied.**

- `native/src/gates/install_pin.rs` gains C.
  - It reads the page at `docs/install.md` by default.
  - It reads the pinned declaration through `git show v<pin>:<doc>`, where `<doc>` is `check-release-assets`' default declaring doc, over the one program resolver.
  - It parses that doc with `release_assets`' declaration reader, made `pub(crate)` rather than spelled twice.
- Output follows gate-sdk/SPEC.md §Output contract.
  - A finding names the surface, its line and the token, and which reading failed: an undeclared name, a surface with no token, or a matched template whose sidecar the declaration lacks.
  - The `help:` line names the two remedies: spell the pinned release's name, or, in the commit moving the pin, spell the new release's.
  - The clean line adds the token count and the pinned declaration's version.
- `scripts/check-install-pin.gate` couples `docs/install.md` and `gate-sdk/SPEC.md`, and its `# spec:` line names C.
- `scripts/gate-tests/check-install-pin/` takes the new positional form.
  - `good/` carries a page and two scripts spelling declared names, and a pinned doc declaring them.
  - `bad/` carries a script spelling `checkwright-$v.tar.gz`, a page with no token, and a pinned doc whose tarball template has no sidecar template.
- The module's unit tests hold the tokenizer: a path-joined token, a backslash-joined one, a `${name}` form, and a `$tgz.sha256` suffix that is no token.

The act is the same whichever order this and `customer-docs-quality-standard` land in. That entry rebuilds the page and folds its recipes into collapsed blocks, but C reads the page's raw text, where the recipes' tokens stand either way.

### (3) The declaration's two readers point at each other {mechanical}

**Not yet applied.** gate-sdk/SPEC.md §check-release-assets' honest-limits paragraph replaces its last sentence with:

> The surfaces that spell an asset name to fetch it, the install page's recipes and the hosted install scripts, are held to the pinned release's declaration by installer/SPEC.md §The hosted install pin, invariant C.

docs/site-architecture.md's hosted-install-scripts row adds invariant C to what `check-install-pin` holds.

## Producers and consumers

- **The asset token.**
  - Producer: a hand edit of a fetch surface.
  - Consumer: `check-install-pin` at every commit. It is precommit-tier and registered, and its couples gain the page and the declaring doc.
  - Red condition: a token outside the pinned declaration, a surface with none, or a matched template with no sidecar template.
  - Each member's value: the three surfaces and their normalized tokens under *Measured at authoring*. The install-smoke legs are excluded, on the ground delta 1 states.
- **The pinned declaration.**
  - Producer: the release that `v<pin>` names, whose gate-sdk/SPEC.md line `check-release-assets` held at that tag's `pack`.
  - Consumer: C.
  - Red condition: exit 2 when the tag carries the doc with no line or two lines; dormant where the tag does not resolve, as B already is.

## Existing sections updated

Roster probes: `git grep -n "check-install-pin"` and `git grep -n "release-assets"` over the tracked tree, for the sites that name the gate or the declaration's readers.

- `installer/SPEC.md` — §The hosted install pin (delta 1).
- `native/src/gates/install_pin.rs`, `native/src/gates/release_assets.rs`'s declaration reader, `scripts/check-install-pin.gate`, `scripts/gate-tests/check-install-pin/` (delta 2).
- `gate-sdk/SPEC.md` — §check-release-assets' honest limits (delta 3).
- `docs/site-architecture.md` — the hosted-install-scripts row (delta 3).
- `docs/installer/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/check-graph.html`, `scripts/git-hooks/pre-commit` — the generated mirrors, graph and hook, regenerated by the command each freshness gate prints (deltas 1, 2 and 3).

## Retired spellings

- None — the unit adds an invariant to an existing gate and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **C runs live** — the full battery green with C reading `v0.26.0`'s declaration and counting the three surfaces' tokens on its clean line, and the fixture pair green. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`). It needs no remote read. The commit-time run is its oracle, and the gates workflow's battery, checked out at full depth, only repeats it.
