# SPEC amendment: product-statement

Checkwright is described two ways. README.md and docs/index.md open on *Verification for coding-agent delivery* and the verification-layer sentence. Five other surfaces carry the older *coding-agent-assisted delivery methodology* wording: the GitHub About, `installer/package.json`'s description, `reserve/crates/Cargo.toml`'s description, `docs/_config.yml`'s `description`, and the openers of `installer/README.md` and `reserve/crates/README.md`. The `docs/_config.yml` value is the meta description of every page that sets none of its own, through `docs/_layouts/default.html`. No surface owns the statement.

**The ruling: one source file, a generated projection, a freshness gate, and a monitor arm for the one copy the tree cannot hold.**

- **The source** is a tracked `scripts/product-statement.conf`, in the `key value…` grammar `scripts/identity.conf` uses. It holds two keys, the category line and the summary sentence. Today's README and index wording is the text, since it is the statement the front door already makes.
- **The copies are generated**, each in the rendering its format needs. `--emit product-statement --write` writes them, and `check-product-statement-fresh` holds them at commit.
- **Two openers stop restating instead of being generated.** `installer/README.md` and `reserve/crates/README.md` describe their own package and link the project, where they now characterize it. Removing a copy outranks gating it.
- **The About is a network read and a GitHub write**, so a precommit gate cannot reach it. It sits on the monitor side of site-kit/SPEC.md §The monitor boundary: this repo's `site-health.yml` copy gains a consumer-local arm comparing it with the generated plain rendering. It is set once by hand, under OPS.local.md's account step.

**Refused.**

- **README.md as the source.** A page that is both the master and a rendering leaves no way to tell an edit to the statement from an edit to the page.
- **Jekyll data with Liquid in docs/index.md.** It would remove one copy and leave README.md, which GitHub renders with no Liquid, as a second hand-kept one.
- **A monitor over npm and crates.io.** npm's description is `installer/package.json`'s at the published tag, so it trails any statement edit until the next release, and a monitor comparing it with the tree would red on every such edit. crates.io carries the reservation's description until the reservation is republished, which is filed as a gap.
- **Generating into the excluded surfaces.** `CLAUDE.md`'s first line tells an agent what the repository holds, and the methodology wording still says that truly. The release-note opener in `RELEASING.md` is a note's reserved framing, published inside immutable posts. Neither states the product to a reader choosing it.

**Measured at authoring.**

- `gh api repos/checkwright/checkwright --jq .description` returns *A coding-agent-assisted delivery methodology as installable kits: …*.
- `git grep -n -i "installable kits\|coding-agent-assisted\|verification layer"` finds the seven tracked copies named above, plus the exclusions.
- `docs/_layouts/default.html` line 9 renders `{{ page.description | default: site.description }}`.
- The statement's plain rendering runs to 236 characters.

## What changes

### (1) The source and its two renderings {design-bearing}

**Not yet applied.** `scripts/product-statement.conf`:

```text
# The product statement — docs/site-architecture.md §Generated projections and their freshness gates.
# 'key value…'; '#' comments and blanks ignored. Edit here, then run
# bash gate-sdk/bin/run-gates.sh --emit product-statement --write
category Verification for coding-agent delivery.
summary Checkwright is the verification layer under agent orchestration: spec drift, skipped stages, and unsupported *done* claims become failing checks before a merge, instead of review findings after one.
```

**The grammar.** Each key occurs exactly once, and no other key is admitted. A value may carry `*…*` emphasis and no other markdown: `` ` ``, `[`, `]`, `<`, `>` and `|` are refused, so the plain rendering loses nothing.

**The renderings.**

- **markdown:** `**<category>** <summary>`.
- **plain:** `<category> <summary>`, with every `*` dropped.

### (2) The sites and the emit arm {design-bearing}

**Not yet applied.** `--emit product-statement` prints each site's expected text, one `<site>: <text>` line apiece. `--write` rewrites each site in place and touches nothing else in the file.

| Site | What is written |
|---|---|
| `README.md` | the markdown rendering, between `<!-- product-statement:begin -->` and `<!-- product-statement:end -->`, with a blank line inside each marker |
| `docs/index.md` | the same block |
| `docs/_config.yml` | the `description:` line's value, as a YAML double-quoted scalar |
| `installer/package.json` | the `description` member's value, as a JSON string |
| `reserve/crates/Cargo.toml` | the `description` key's value, as a TOML basic string |

Every site takes the plain rendering except the two markdown blocks. `"` and `\` are escaped in all three formats.

The arm lives in the crate's emit table beside `--emit value-rollup`, the precedent for a repo-local projection. Its rendering is one function the gate calls in process.

**The hand edits riding it.**

- README.md's statement block becomes its own paragraph. The sentence after it, *It ships as installable kits: …*, becomes the next.
- `installer/README.md`'s opener becomes *The activation path for **Checkwright**, whose kits are listed in the [kit reference](https://checkwright.dev/kits.html).*
- `reserve/crates/README.md`'s opener becomes *This crate name is reserved for **Checkwright** (<https://checkwright.dev>).*

The act is the same whichever order this and [customer-docs-quality-standard](TASK-QUEUE.md#customer-docs-quality-standard) land in. That entry rewrites README.md's and docs/index.md's §Try it first and §Install and docs/index.md's §Start here, none of which holds the statement's line.

### (3) The gate: `check-product-statement-fresh` {design-bearing}

**Not yet applied.** Born native, repo-local, specified in docs/site-architecture.md §Generated projections and their freshness gates:

- `scripts/check-product-statement-fresh.gate` carries:
  - `# graph: couples=scripts/product-statement.conf,README.md,docs/index.md,docs/_config.yml,installer/package.json,reserve/crates/Cargo.toml dir=one valve=none tier=precommit`;
  - a `# projection:` line naming the five sites;
  - the `# spec:` line.
- `native/src/gates/product_statement_fresh.rs` compares each site with the arm's emission in process. A stale site is one finding naming it, and the `help:` line names `bash gate-sdk/bin/run-gates.sh --emit product-statement --write`.
- **Exit 2:**
  - an unreadable source or site;
  - a source-grammar fault: a missing, repeated or unknown key, or a refused character;
  - a site whose block or field is absent or occurs twice, since `--write` could not place it.
- The `good/`+`bad/` pair runs in a positional form, `check-product-statement-fresh [source site…]`.
  - `good/` holds a source and five fresh sites.
  - `bad/` holds a stale README block, a stale JSON description and a TOML description left in the retired wording.
- Unit tests hold the three escapers and the markdown and plain renderings.
- `scripts/gates.list` registers it.

docs/site-architecture.md §Generated projections and their freshness gates gains a row, keyed `<!-- projection: check-product-statement-fresh -->`:

> **The product statement** — `scripts/product-statement.conf` is the one statement of what Checkwright is. `bash gate-sdk/bin/run-gates.sh --emit product-statement --write` renders it into five sites: README.md's and docs/index.md's `product-statement` blocks in markdown, and in plain text `docs/_config.yml`'s `description` (every page's default meta description), `installer/package.json`'s `description` (npm's listing, at the next release) and `reserve/crates/Cargo.toml`'s. `check-product-statement-fresh` holds all five. The repository's GitHub About is the sixth copy and is out of a gate's reach, so `site-health`'s About arm holds it (below).

### (4) The About arm {design-bearing}

**Not yet applied.** `.github/workflows/site-health.yml`, this repo's copy and not site-kit's template, gains an arm in its probe step:

- It reads `gh api 'repos/{owner}/{repo}' --jq .description`, asserting on the call's own exit status as the tag-list call does.
- It compares the result with `installer/package.json`'s `description`, read with `jq -r`. That field is the plain rendering, held fresh by delta 3's gate, so the arm carries no second renderer.
- A difference is a finding. It prints both texts, and the remedy: a `gh repo edit --description` under OPS.local.md's account step.
- A failed call is a finding. An empty About is a difference like any other.
- It prints a census line naming whether the arm compared, and its outcome.
- The workflow's `# enforce:` marker gains *the repository About against the product statement* in its arm list, so the enforcement map projects the arm.

The arm is consumer-local by site-kit/SPEC.md §templates/site-health.yml's arm-roster ruling: the product-statement source is this repo's convention, which no kit ships. So site-kit's template is untouched.

docs/site-architecture.md's new row (delta 3) closes with the arm: *The About is set by hand, under OPS.local.md's account step, and site-health's About arm compares it daily with `installer/package.json`'s description and files the site-health issue on a difference.*

## Producers and consumers

- **`scripts/product-statement.conf`.**
  - Producer: a hand edit.
  - Consumers: `--emit product-statement`, which runs when a maintainer runs it; `check-product-statement-fresh` at every commit that touches the source or a site.
  - Every field has a reader: `category` and `summary` both reach every rendering.
- **The five generated sites.**
  - Producer: the emit arm's `--write`.
  - Consumers:
    - the gate;
    - GitHub's renderer for README.md;
    - Jekyll for docs/index.md and `site.description`, whose reader is `docs/_layouts/default.html`;
    - npm's registry at the next publish, which reads the packed `package.json`;
    - crates.io at a republish of the reservation, which no step here performs (filed as a gap).
  - Red condition: a site differing from its rendering.
  - Each member's value: the table in delta 2.
- **The `product-statement` marker pair**, a new marker name.
  - Readers: the emit arm and the gate.
  - Roster-holding readers: `check-projection-roster` holds the `# projection:` header to the site-architecture row's key. Markers are HTML comments, which check-docs-render-fidelity renders as nothing.
  - `check-front-door-verbs` reads README.md and docs/index.md for code spans alone, and the block carries none.
- **The About arm.**
  - Producer of its verdict: the scheduled `site-health` run. It is never hand-dispatched, so its first reading arrives on the schedule after the closing push.
  - Consumer: the `site-health` issue, which the arm opens or updates.

## Existing sections updated

Roster probe: `git grep -n -i "installable kits\|coding-agent-assisted\|verification layer"` over the tracked tree, for the copies. Each is either rostered here or excluded under **Refused**.

- `scripts/product-statement.conf` (delta 1).
- `README.md`, `docs/index.md`, `docs/_config.yml`, `installer/package.json`, `reserve/crates/Cargo.toml`, `installer/README.md`, `reserve/crates/README.md`, and the emit table in `native/src/emit/mod.rs` with the arm's module (delta 2).
- `native/src/gates/product_statement_fresh.rs`, `native/src/gates/mod.rs`, `scripts/check-product-statement-fresh.gate`, `scripts/gate-tests/check-product-statement-fresh/`, `scripts/gates.list` (delta 3).
- `docs/site-architecture.md` — §Generated projections and their freshness gates, the new row (deltas 3 and 4).
- `.github/workflows/site-health.yml` — the probe step's About arm and the `# enforce:` marker (delta 4).
- `docs/installer/README.md`, `docs/enforcement.md`, `docs/value.md`, `docs/check-graph.html`, `scripts/git-hooks/pre-commit`, `.workflow/surface-ceiling.txt` — the generated mirror and the new-gate fan-out, each regenerated by the command its gate prints (deltas 2, 3 and 4).

## Retired spellings

- None — the retired wording is prose, not a name, and delta 2 rewrites every tracked copy the roster probe finds.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment remains for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **One statement live** — the full battery green with `check-product-statement-fresh` registered, and the About set to the plain rendering by `gh repo edit --description` under OPS.local.md's account step, read back with `gh api repos/checkwright/checkwright --jq .description`. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`). The About arm's first scheduled run is read at close if one has run, and otherwise is left to its own issue channel.
