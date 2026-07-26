# SPEC amendment: release-tarball-delivery-channel

A root-level amendment: it adds a delivery channel that no kit owns, and it
reaches `installer/`, `docs/`, `.github/workflows/`, and `RELEASING.md`. It
promotes the queue entry `release-tarball-delivery-channel`.

**This amendment changes more than two components' contracts**, so the
trigger-gated audit stage fires this iteration and `check-stage-entry`
assertion C will demand an audit stamp at the following stage's entry. Said here
so the downstream entry does not discover it.

**It amends a landed ruling.** The `activation-installer` amendment is merged
and deleted, so the two rulings this unit modifies are addressed at the surfaces
they landed in rather than at a file that no longer exists:

- its **§A1 phase-1 envelope** — phase 1 was `init`, `doctor`, `--dry-run`, the
  manifest, the profile roster, the packaging, and the smoke that proves the
  path. This widens it by one delivery channel over the *same* packaging. The
  phase-2 verbs (`update`, `diff`, `uninstall`, the `installer-lifecycle-verbs`
  entry) stay out, and the binding rule holds unchanged: **no manifest field is
  written whose only reader arrives in phase 2.** This unit writes no manifest
  field at all, which is the cheapest possible way to satisfy it.
- its **§B1 delivery ruling** — "bash inside, npm outside", with the corollary
  that Node is not added to `PROBE_SET`. The ruling is *extended*, not
  overturned: bash is still the implementation, npm is still a delivery vehicle,
  and it is now one of two. §A2 below states the generalization the extension
  needs. Landed homes: `installer/README.md` §Implementation and §Requirements,
  and `docs/install.md`'s opening distinction and §Requirements.

**Provenance seam.** Nothing private crosses. GitHub Releases, `gh`, and
`sha256sum` are public vocabulary; no account, maintainer identity, or release
credential is named — the job authenticates with the run's own `github.token`,
which is repository configuration and never tree state.

## What changes

Every delta carries its work class. **mechanical** — executing it demands only
oracle-running (a fixed battery, a substitution sweep, a regen command).
**design-bearing** — executing it demands generative or verificational judgment.

### A. The ruling, recorded rather than re-argued

**A1. The tarball becomes the primary channel; npm is retained as secondary.
{design-bearing}** An operator ruling. Nothing in the gate battery needs Node —
`PROBE_SET` does not carry it and the floor roster deliberately keeps it off —
so requiring Node solely to deliver a bash payload asks the adopter for a
dependency the contract does not assert. npm is **kept**, and for reasons that
do not transfer to a self-hosted script: immutable versioned artifacts, registry
integrity hashes, `npm pack` pre-inspection, and `--provenance` sigstore
attestation.

**Explicitly rejected, recorded so it is not re-proposed:** `curl -fsSL … | sh`.
`docs/install.md` argues that what governs your tree is committed, auditable
source you read before you run it; piping an unreviewed remote script into a
shell is the counter-pattern, and shipping it would have the page contradicting
its own first command. The documented flow is therefore **download → verify →
extract → run**, four steps a reader can stop after any of.

**A2. Two transports over one install model — and the floor rule that
generalizes. {design-bearing}** npm and the tarball are not two install models.
They share `init`, they share `checkwright.lock`, and they produce the same
vendored result; only the fetch differs. That is exactly why both can be sibling
jobs over one `pack` assembly, and it is why `plugin-marketplace` — a different
unit of delivery over a different subject — does not inherit this unit's
cheapness. Its own entry carries the matching negative result; do not extend the
analogy to it.

The landed §B1 corollary said *Node is not added to `PROBE_SET`*. The general
rule this unit needs, and the form the corollary takes from here on: **no
delivery-path tool joins the floor roster.** The roster asserts what the
*battery* requires, and a channel's fetch tools are not that. Concretely the
tarball path wants `curl` (or `wget`), `tar`, and `sha256sum`; `sha256sum` is
already implied by the floor's coreutils member, and the other two are named as
path requirements in prose exactly as Node is today. `context-kit/lib/toolfloor.sh`
is **unchanged**, so `check-install-toolchain` needs no re-derivation.

The asymmetry that makes the primary/secondary ordering honest is worth stating
where a reader meets it: the tarball path removes a runtime dependency, and the
npm path is the one that carries a build attestation. Neither dominates, so the
page names both properties instead of claiming the primary channel is simply
better.

### B. The publish workflow

**B1. A `release` job, sibling to `npm`. {design-bearing}** `.github/workflows/publish.yml`
gains one job. Its shape is dictated by that file's own header, which is
load-bearing and not up for revision here: `pack` assembles and stamps the
tarball once and uploads it as the run's artifact, and **every delivery channel
is a sibling job that `needs: pack` and consumes that one artifact**. So:

- `needs: pack`, downloading the `installer-package` artifact — the *same* bytes
  the `npm` job publishes. One assembly, two channels, nothing to disagree about.
- `permissions: contents: write`, and nothing more. The file's top-level
  `permissions: {}` with per-job widening is the established shape.
- **No `environment:` gate.** The `npm` job carries `environment: npm-publish`
  because publishing to a public registry is irreversible and the operator
  attaches approval there. Attaching assets to this repository's own Release is
  neither irreversible nor external, and gating it would hold the primary
  channel behind the secondary channel's approval — inverting the ordering `A1`
  just established.
- **No new `uses:` ref.** The job runs `gh` from the runner image with
  `GH_TOKEN: ${{ github.token }}` — the shape `site-health.yml` already uses. A
  fourth action would owe a fourth pinned SHA and widen the tamper surface
  `check-action-pinning` exists to bound, for no capability the CLI lacks.
- **The job creates the Release if absent and attaches both assets**, so the
  assets are on the Release from the moment it exists. `C1` reworks the runbook
  step that assumed the operator creates it first.
- Its `run:` body routes every GitHub expression through `env:`, per this repo's
  standing convention — and it is a third block under the sibling unit's
  `check-action-run-shell`, so it must land clean at `-S warning`.

**The three `uses:` SHA pins and the job split are untouched.**

**B2. The assets, and what the checksum actually buys. {design-bearing}** Two
assets: the `.tgz` from the `pack` artifact, and a `.sha256` file over it,
generated with `sha256sum` in the same job.

The honest limit, stated in the same breath as the instruction rather than
omitted: the checksum and the tarball are served from **the same origin over the
same TLS session**, so it proves the download was not corrupted or truncated —
it does **not** prove the release host was not compromised. The property that
would prove that is a build attestation, and that is precisely what the npm
channel's `--provenance` carries and this one does not. Stating the asymmetry is
what keeps `A1`'s "primary" from becoming an overclaim, and it is the second
concrete reason npm is retained. A signed attestation on this channel is a
separate initiative, filed rather than smuggled in.

**B3. Node moves from the adopter to the runner. {mechanical}** `scripts/pack-installer.sh`
runs `npm pack` to *produce* the tarball, so npm is still needed to build the
asset — on the release runner, which already has it. Consuming the asset needs
no Node at all. The `package/` prefix `npm pack` puts inside the tarball is
therefore visible to a reader who has never used npm, so `C2`'s documented steps
name it explicitly instead of leaving it to be discovered:
`tar -xzf checkwright-<version>.tgz` yields `package/`, and the entry point is
`bash package/bin/checkwright.sh init`.

### C. The documented surfaces

**C1. `RELEASING.md`. {design-bearing}** Step 5 (watch the publish workflow)
gains the second channel: the run now publishes to npm *and* attaches the
Release assets, and both are watched to green. Step 6 changes character rather
than content — the Release already exists with its assets by the time the
operator reaches it, so the step becomes **fill in the Release body** rather
than create the Release. Everything that step already pins survives verbatim:
the body points at the post's `https://checkwright.dev/` URL, **without a
trailing slash**, and opening the link once is a named verification, because the
body lives on the host and out of the battery's reach.

**C2. `docs/install.md`. {design-bearing}** Four edits, and the second is the
one that carries the design.

- **§Quick start is reordered**: the tarball flow first, as four commands
  (download, verify, extract, run), with `npx checkwright init` retained
  immediately below as the one-liner for a reader who already has Node. Both
  reach the same `init`, and the page says so rather than implying two products.
- **§Requirements' path-requirement paragraph is rewritten.** It currently reads
  "One requirement belongs to a path rather than to the battery: the installer
  below needs Node, for `npx`." That sentence is now false as the page's primary
  claim. It becomes a three-path statement — tarball (curl/wget + tar, which a
  GNU userland already has), npx (Node), manual vendoring (nothing) — carrying
  `A2`'s rule that none of them joins the floor roster, and pointing at the
  roster's real owner rather than restating it.
- **The opening two-registry distinction is extended, not rewritten.** The
  crates.io half stays verbatim ("nothing to `cargo add`"). The npm half keeps
  its installer-not-a-dependency argument; a third bullet states that the same
  payload is downloadable as a Release asset, which strengthens rather than
  weakens the doctrine — a downloaded, checksummed, extracted tarball you read
  before running is the *most* auditable form of the same one-shot vendoring.
- **§Vendoring the kits' claim to be the Node-free path is corrected.** It says
  of manual vendoring "it is the path that needs no Node" — true when there was
  one installer path and false the moment there are two. The audit found this
  surface; the three edits above do not reach it, and the DoD's
  no-unqualified-Node-claim item is not satisfied without it. The section keeps
  its real subject: manual vendoring stays the **audit story**, the account of
  what lands in your tree. It simply stops claiming the Node-free property as
  its own, since the tarball path now has it too.

**C3. `installer/README.md`. {mechanical}** §Requirements carries the landed §B1
corollary in its narrow form ("The installer path needs Node, for `npx`") and is
updated to `A2`'s two-transport statement. §Implementation is **unchanged** —
bash inside, npm outside is still true; npm is simply no longer the only outside.

**C4. `installer/consumer-smoke/run-smoke.sh` — a download arm. {design-bearing}**
The smoke's existing arm packs, installs the tarball through `npm install
--offline`, and drives `init` per profile — which is what proves the payload
ships inside the tarball. The new arm proves the **Node-free** claim, and it can
only do that by not using Node:

- extract the packed tarball with `tar -xzf` into a scratch dir;
- verify the `sha256sum` of the tarball against a digest computed in the smoke,
  which exercises `B2`'s verification step rather than only documenting it;
- run `bash <extracted>/package/bin/checkwright.sh init` in a scratch consumer
  with **`npm` and `node` masked off `PATH`**, so a latent Node dependency fails
  the smoke instead of passing silently on a machine that happens to have Node;
- assert the same post-conditions the npm arm asserts — green battery, manifest
  agrees with the tree, idempotent re-run, `doctor` clean.

**Scope: one profile, not the per-profile loop.** The npm arm drives `init` once
per profile because what it proves — the payload resolves and every profile's
kit set is present — is profile-dependent. What *this* arm proves is
transport-independent: the same payload reached the tree without Node. Re-running
it per profile would re-assert the npm arm's property at triple the smoke's cost
and prove the Node-free claim no harder. It runs against **`full`**, the profile
whose payload is largest and whose `doctor` sources the widest toolchain roster,
so a Node dependency latent in any kit's path is reachable. Stated here because
it is a cost decision, and an unstated one gets re-made at build.

The masking is the whole value of the arm. Without it the two arms differ only
in how the bytes arrived, and the claim the channel exists to make would be
untested. Note the smoke's preflight currently *requires* `npm` and `node` on
`PATH` for the pack step; that requirement stays for packing and is masked only
around the download arm's `init`.

`INSTALLER_SMOKE_TMP_DIR` remains the smoke's only knob — the new arm writes
under the same scratch root, so a run still leaves the worktree untouched.

**C5. The site's install page is a governed doc, not a free edit. {mechanical}**
`docs/install.md` is in the spec manifest, so its links and commands resolve
under the doc gates: `check-docs-cmd`, `check-md-refs`,
`check-docs-link-convention`, and `check-docs-render-fidelity` all read the
edited page, and `check-install-toolchain` holds its toolchain block in
whole-element parity with `context-kit/lib/toolfloor.sh` — which `A2` leaves
byte-identical, so that gate needs nothing.

### D. The regen tail

**D1. {mechanical}** New tracked content moves the footprint and the value
rollup; the docs mirror regenerates if any kit SPEC is touched. Each names its
own regen command on a red. No new gate is added by this unit, so the pre-commit
hook, the graph artifact, and the enforcement map move only if the sibling units
move them.

## Producers and consumers

- **The Release assets (`B1`, `B2`).** *Producer*: the `release` job in
  `.github/workflows/publish.yml`, triggered by a `v[0-9]*` tag push — the only
  trigger the file accepts, and one that fires on this iteration's own
  `v0.16.1`, so the producer is exercised rather than theoretical. Its input is
  the `installer-package` artifact `pack` uploads; its enabling configuration is
  the job's `contents: write` permission plus the run's own `github.token`,
  neither of which is optional repository state that could be unset. *Consumers*,
  three and each named: an adopter following §Quick start's four commands; the
  smoke's download arm (`C4`), which is the automated reader that keeps the
  channel from rotting between releases; and `RELEASING.md` step 6's operator,
  who finds the Release already present with its assets attached.

- **The `.sha256` asset (`B2`).** *Producer*: `sha256sum` in the same job.
  *Consumers*: the adopter's `sha256sum -c` at §Quick start step 2, and the
  smoke's verification in `C4`. A checksum with only a documented reader would
  rot at the first format change; the smoke is the reader that fails loudly.

- **The Node-free entry point (`B3`).** *Producer*: `scripts/pack-installer.sh`,
  which already assembles `bin/checkwright.sh` into the payload — no change to
  the pack script is required, which is what makes this channel cheap.
  *Consumer*: `bash package/bin/checkwright.sh`, invoked directly from the
  extracted tarball. The transition where the claim is read is `C4`'s masked
  `PATH`: that is the point at which "needs no Node" stops being a sentence and
  becomes an assertion.

  **Verified at the audit stage rather than assumed**, because "the payload
  already works without Node" is the hypothesis this whole unit rests on:
  `installer/bin/checkwright.sh` takes its package root from `BASH_SOURCE`
  through the symlink chain, not from any npm-supplied variable, and no
  `npm_*`, `node`, or `npx` reference exists anywhere under `installer/bin/` or
  `installer/lib/`. `installer/package.json`'s `files` array carries `bin/`, so
  the entry point is inside the packed tarball. `C4`'s arm therefore lands as an
  assertion of a property the tree already has — not as a discovery that the
  entry point needs reworking first.

- **No new manifest field.** `checkwright.lock`'s six fields are written by
  `init`, which is transport-agnostic; the channel is upstream of it. This is the
  landed §A1 binding rule discharged by construction — there is no field whose
  only reader would arrive in phase 2 because there is no new field.

- **No new state in the tree.** The assets live on the release host, outside the
  battery's reach — the same disposition as a Release body, and the reason `C1`
  keeps the manual link verification rather than proposing a gate for it.

## Existing sections updated

- **`.github/workflows/publish.yml`** — the `release` job from `B1`. The header
  comment's job-split rule is the authority for the shape and is **not** edited;
  the three `uses:` SHA pins are untouched.
- **`RELEASING.md` §The procedure** — steps 5 and 6 per `C1`.
- **`docs/install.md`** — the opening two-registry distinction, §Requirements'
  path-requirement paragraph, §Quick start, and §Vendoring the kits' Node-free
  claim, per `C2`.
- **`installer/README.md` §Requirements** — per `C3`; §Implementation unchanged.
- **`installer/consumer-smoke/run-smoke.sh`** — the download arm, `C4`. It is the
  evidence-kit `installer_smoke` validate suite, so the arm is re-run at every
  validate stage rather than only at release.
- **`installer/README.md` §The consumer smoke** — owned by `C4`: the arm roster
  this section describes gains the download arm, so the prose and the script
  agree. Its "the offline tarball install is the load-bearing one" paragraph
  gains the second load-bearing arm rather than being displaced by it — the two
  assert different properties (no registry resolution; no Node).
- **`docs/footprint.md`, `docs/value.md`** — generated projections, regenerated
  through their owning commands per `D1`.
- **This repo's release note for the shipping version** — a Behavior-changes
  bullet naming the new primary channel, per docs/install.md §The upgrade
  contract; authored at close.

Deliberately **not** updated: `context-kit/lib/toolfloor.sh` and the toolchain
block it feeds (`A2`), and `scripts/pack-installer.sh` (`B3`) — both named here
so a build session does not adopt them as orphans on its own authority.

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
      retired; nothing dangles. Specifically: no surface still says the installer
      path needs Node as an unqualified claim.
- [ ] **The Node-free claim is asserted, not stated** — the smoke's download arm
      runs `init` with `npm` and `node` masked off `PATH`, and reds if either is
      reached.
- [ ] **`PROBE_SET` is unchanged** — no delivery-path tool joined the floor
      roster (`A2`).
- [ ] **The publish workflow's job split and `uses:` pins are byte-unchanged
      apart from the added sibling job.**
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
