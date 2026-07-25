# SPEC amendment: supply-chain-trust-baseline

A root-level amendment: the ruling is repo governance (a disclosure route, a
conduct surface, the CI action-ref form) with no single owning component, and it
reaches two kits — `gate-sdk` gains a gate and a pinned template, `site-kit`
loses an overclaiming phrase.

## What changes

Every delta carries its work class. **mechanical** — executing it demands only
oracle-running (a fixed battery, a substitution sweep, a regen command).
**design-bearing** — executing it demands generative or verificational judgment.

### A. The disclosure surface

**A1. Root `SECURITY.md`. {design-bearing}** A new tracked root file carrying
three things and no fourth: the **reporting route** (GitHub private
vulnerability reporting on this repo — the Security tab's "Report a
vulnerability", never a public issue and never an email address, which would
publish a maintainer identity the repo otherwise keeps out of tracked files),
the **response expectation** stated as an aspiration a solo maintainer can
honour, and the **supported-version rule**: pre-1.0, only the newest `v*` tag is
supported and there are no backports.

Its **threat-boundary** section is where the tiering discipline bites. The
honest limit already exists and is owned elsewhere — gate-sdk/SPEC.md
§Enforcement tiers states that consumer-owned CI stops bypass but not a
workflow self-edit, and names the hosted-attestation rung as the tamper-proof
verifier. `SECURITY.md` **cites** that section and does not restate it. What
`SECURITY.md` owns and no other surface states: that a vendored kit is bash the
consumer runs with the consumer's own privileges in their hooks and CI, that
vendoring is a copy the consumer reads before running, and that a gate is a
consistency check on a tree — not a security boundary against a committer who
already holds write access. That distinction is the file's whole payload; a
`SECURITY.md` that implies the gates defend against a hostile maintainer would
be the overclaim this iteration exists to remove.

**A2. Root `CODE_OF_CONDUCT.md`. {mechanical}** Contributor Covenant, adopted
verbatim, its enforcement contact pointing at the same private route `A1`
names rather than a second address.

**A3. Register both as governed repo-meta. {mechanical}** They join
`CONTRIBUTING.md` and `RELEASING.md` in the three registries that make a root
doc governed rather than merely present: `scripts/root-allowlist.list` (else
`check-root-tiering` reds on an unallowlisted root entry),
`scripts/core-files.list` (silent deletion goes red), and
`CANON_KIT_MANIFEST_FILES` in `scripts/canon-config.sh` (their links and
commands resolve under the doc gates like any other governed page).

**A4. Retire the superseded ops ruling. {mechanical}** The local-only ops
runbook carries a standing ruling that `SECURITY.md` is not owed yet. The
operator-approved promotion of this unit supersedes it; the runbook line is
replaced by the shipped state so the desired-state table and the tree agree.
Local-only file, no tracked diff — but leaving it stale re-litigates the ruling
at the next ops pass.

### B. Pinned action refs and the gate that holds them

**B1. Pin every mutable `uses:` ref. {mechanical}** Each `uses:` ref in the
tree resolves to a full 40-hex commit SHA, with the human-readable tag kept as
a trailing comment. Three sites, and the third is the one that matters: the two
instances under `.github/workflows/`, **and** `gate-sdk/templates/gates-workflow.yml`
— the copy-out every consumer vendors. Pinning the instances while the template
ships mutable would fix this repo and leave the drift source intact for
everyone downstream.

**B2. `check-action-pinning` — a new gate-sdk gate. {design-bearing}**

*Placement, the seam call scope left open.* It ships as **kit mechanism in
`gate-sdk/checks/`**, not as a consumer gate under the gates dir. Three reasons
converge. gate-sdk already owns a GitHub Actions artifact — `templates/gates-workflow.yml`
is its CI backstop, core-files-pinned, and gate-sdk/SPEC.md §templates/gates-workflow.yml
already rules on precisely this surface's threat model ("no third-party
actions — so the workflow surface an agent could tamper with stays minimal and
reviewable"). An immutable ref is that same invariant continued: the rule
already says *which* actions may appear, and this says the reference must not
be able to change under the consumer. Second, `B1`'s load-bearing site is a
gate-sdk-owned file, and a consumer gate cannot reach it — the kit would ship
the drift ungated. Third, the "GitHub Actions is not universal" objection is a
*reach* question, not a seam question, and gate-sdk already answers reach
questions with counted inertness rather than exclusion (a kit root without a
`smoke/` dir is skipped-and-counted; an unresolvable walk root is skipped). The
provenance seam is untouched either way: an action ref is public GitHub
vocabulary, not private rule content.

*Name.* Not `check-workflow-*`. In this tree "workflow" already means the
`.workflow/` directory — `check-workflow-tiering` gates it and
`GATE_SDK_WORKFLOW_DIR` configures it — so a second `check-workflow-*` gate
over `.github/workflows/` would collide on the reader's only disambiguator.

*Invariant.* Every `uses:` ref in a scanned YAML file is immutable: either a
full 40-hex commit SHA, or a repo-local ref (a `./`-prefixed path to an in-repo
composite action, which git already pins by checkout). A tag or branch ref
(`@v5`, `@main`) reds.

*Scan set, derived not rostered.* A whole-tree walk over tracked `*.yml` /
`*.yaml` via `gate_find`, which prunes `gate-tests` through the shared
`GATE_SDK_PRUNE_DIRS` — so the `bad/` fixture cannot red the whole-tree run,
and both `.github/workflows/` and the shipped template are covered with no
roster to drift. **This gate introduces no new knob**: the scan set is derived
and the prune set is the shared one. The fixture pair reaches a synthetic tree
through the positional `check-action-pinning.sh [scan-root]`, the form
`check-root-tiering` already uses. A tree with no matching file exits clean with
a zero count — the counted-inertness that makes kit placement correct.

*What it deliberately does not assert.* It does not require the trailing
version comment, and it does not verify that a present comment names the tag
the SHA actually points at. Verifying that needs a network call, which breaks
the hermetic-gate contract; requiring an unverifiable comment would be exactly
the trivially-true proxy gate-sdk/SPEC.md §When a gate earns its place bars —
heading presence while the content drifts, manufacturing false confidence. The
comment is a convention the gate leaves to review.

*Gate contracts.* `precommit` tier; a `# graph:` manifest coupling the scanned
YAML surface; the good/bad fixture pair; the output and fail-closed contracts
per gate-sdk/SPEC.md §The gate model. Registration in `scripts/gates.list` and a
row in `gate-sdk/README.md`'s roster block, which `check-readme-roster` holds
bidirectionally against `gate-sdk/checks/`.

**B3. The regen tail. {mechanical}** A new gate in a kit moves a fixed set of
generated projections, each of which names its own regen command on a red. The
pre-commit hook (`gen-pre-commit.sh --write`), the graph artifact
(`check-graph.sh --emit > docs/check-graph.html`), the enforcement map
(`enforcement-map.sh --emit > docs/enforcement.md`), the on-site SPEC mirror
(`gen-docs-mirror.sh --write`, which also carries `B4`'s SPEC edit and `C1`'s),
the footprint (new files carry token cost), and the value rollup that joins the
last two. Executing this is running commands until the battery is green.

**B4. The honest limit on the pin. {design-bearing}** Dependabot *security
updates* are off on this repo by deliberate ruling, so a SHA pin does not
auto-refresh: the pin buys immutability, not currency. gate-sdk/SPEC.md
§templates/gates-workflow.yml states what the pin is for — tag mutation under a
consumer, the tamper axis — and states that refreshing a pin is a manual act at
release time, so a reader does not mistake a pinned ref for a maintained one.
Stating this is what keeps `B1` from becoming its own overclaim.

### C. The de-claim

**C1. Strike the bare "pinned Pages parser". {mechanical}** The phrase asserts
a pin the same kit explicitly declines to own: site-kit/SPEC.md
§check-docs-render-fidelity's parser-version limit says the exact-version pin
"stays a consumer's deliberate `SITE_KIT_RENDERER` override, not kit-run
machinery". Four sites carry the contradicting phrase — `site-kit/SPEC.md`'s
invariant sentence, `README.md`'s site-kit table row, and two in
`docs/site-kit/index.md`, one of which sits a few lines from the link to the
very limit it contradicts. All four are struck to name the parser without
claiming it is pinned. `docs/site-kit/SPEC.md` is the generated mirror and
follows from `B3`'s regen, never a hand edit.

Scope ruled two sites; the two `docs/site-kit/index.md` occurrences are the
same phrase and the same contradiction, and striking half would ship the
inconsistency on the public site. No pin is added anywhere: the CI gem install
stays unversioned, deliberately.

### D. What the vendored hooks execute

**D1. A hook-review section in `docs/install.md`. {design-bearing}** The
install page names `install-hooks.sh` and stops there, so a reader deciding
whether to run this repo's shell in their own hooks has nothing to read. The
new section is an audit account, and its value is entirely in being complete
rather than reassuring:

- **What the hook is.** A generated file — the triggered subset of the
  registered battery, emitted from the per-gate `# graph:` manifests. It is
  tracked, so a reader reads exactly what will run, and `check-graph` holds it
  byte-fresh against its emitter.
- **What installing it does to the clone.** Not one config write but three
  effects: `core.hooksPath` is repointed, `blame.ignoreRevsFile` is set when the
  repo carries that file, and `check-identity` is run once at opt-in so a
  wrong-identity mapping surfaces before the first commit. A reader auditing
  "what does this script touch" needs all three; naming only the first would be
  the same class of half-truth as `C1`.
- **How to review before running.** The hook and every gate it invokes are
  tracked bash under the kit and gates dirs; nothing is fetched at install or
  at run time.
- **How to disable.** Per commit, the bypass valve; per clone,
  `git config --unset core.hooksPath`. Both are supported positions, not
  escapes — gate-sdk/SPEC.md §Enforcement tiers already rules the local hook a
  latency optimization whose guarantee lives in the CI tier.

## Producers and consumers

No new state, event, or message is introduced — this unit adds one gate, three
governed documents, and a pinned literal form. The causal chains that exist:

- **The disclosure route (`A1`).** *Producer*: GitHub's private vulnerability
  reporting on this repo, whose enabling setting is recorded as `enabled` in the
  local ops runbook's desired-state table — a live route, verified this session,
  not a named producer no configuration sets. *Consumer*: an external reporter
  reaching it from the repo's Security tab, which GitHub renders **because** the
  file sits at the repo root under that exact basename; root placement is the
  mechanism, not a convention. `A2`'s enforcement contact is a second reader of
  the same route, which is why it points at it rather than introducing an
  address of its own.

- **The pinned ref form (`B1`).** *Producer*: a maintainer editing a workflow or
  the shipped template. *Consumers*: two, and both must exist or the pin rots.
  The Actions runner resolves the SHA at run time — the functional reader. And
  `check-action-pinning` reads the same literal at commit and CI time — the
  reader that keeps `B1` from silently regressing on the next workflow edit.
  A pin with only the first reader is a convention; `B2` is what makes it a
  contract, which is the enforcement-first pairing this unit is filed under.

- **`check-action-pinning`'s own wiring (`B2`).** *Producer*: registration in
  `scripts/gates.list`, which is what makes `run-gates.sh` resolve and run it —
  a gate file on disk that no registry names runs nowhere. *Consumers*: the full
  battery, the generated pre-commit hook (through its `# graph:` manifest and
  `precommit` tier), and the CI backstop, which runs both the battery and the
  fixture suite. The `# graph:` manifest has a further reader: `check-graph`'s
  artifact and the enforcement map both project from it, which is why `B3` is a
  delta and not housekeeping.

- **The struck phrase (`C1`).** *Producer*: the hand edit in `site-kit/SPEC.md`,
  `README.md`, and `docs/site-kit/index.md`. *Consumer*: `gen-docs-mirror.sh`,
  which regenerates `docs/site-kit/SPEC.md` from the kit SPEC, with
  `check-docs-mirror-fresh` byte-gating the result — so the mirror site is a
  derived reader of the edit, never a fifth place to edit.

- **The registry rows (`A3`).** *Producers*: the three list/config files.
  *Consumers*, one per registry and each a different gate:
  `check-root-tiering` reads the root allowlist, `check-core-files` reads the
  core-files list, and the canon-kit doc gates read the manifest globs. Each row
  has a named reader; a root doc added to none of the three would red the first
  and be ungoverned by the other two.

## Existing sections updated

- **gate-sdk/SPEC.md** — a `### check-action-pinning` section under the
  per-component contracts, carrying the invariant, the derived scan set, the
  no-new-knob note, and the deliberate non-assertion from `B2`.
  §templates/gates-workflow.yml gains `B4`'s pin rationale and its refresh
  limit beside the existing "no third-party actions" ruling, which is the
  sentence the new rule extends.
- **site-kit/SPEC.md** — `C1`'s strike in the §check-docs-render-fidelity
  invariant sentence. The parser-version limit two hundred lines below is
  already correct and is not touched; this edit makes the opening agree with it.
- **gate-sdk/README.md** — the roster row for the new gate.
- **README.md** — `C1`'s strike in the site-kit table row.
- **docs/install.md** — `D1`'s hook-review section, placed after the vendoring
  steps where `install-hooks.sh` is first named.
- **docs/site-kit/index.md** — `C1`'s two strikes.
- **CLAUDE.md §Housekeeping** — the governed-repo-meta sentence enumerates
  `CONTRIBUTING.md`, the `.github/` templates, and `RELEASING.md`; the two new
  root docs join that enumeration so the always-loaded tier names the surface
  it governs. One clause extended, not a new rule line.
- **The local ops runbook** — `A4`.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: no bare "pinned Pages parser"
      survives anywhere, mirror included.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
