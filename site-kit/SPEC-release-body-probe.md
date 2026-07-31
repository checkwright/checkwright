# SPEC amendment: release-body-probe

Durable coverage for the one release step whose output lives entirely off the
tree: the GitHub Release body that points at the release note. `RELEASING.md`
step 6 says outright that the body "lives on the host, out of the battery's
reach", and that honesty is the gap — the step is real, its output is public and
effectively permanent, and its omission leaves no residue any gate can find.

The queue entry `release-body-host-side-unverified` is the governed surface for
the rulings this amendment rests on: monitor-arm-rather-than-close-affordance,
the seam split, the still-armed trailing-slash trap, and the release-defect rate.
This file does not restate them; it designs against them.

## What changes

A new probe arm on site-kit's `templates/site-health.yml`, folded into the
existing `probe live site` step. Nothing else about the monitor's shape moves:
the arm appends to the same `failures` collector, so the existing issue
open/update/close step consumes it unchanged.

### Delta 1 — the release-body pointer arm — **design-bearing**

For every tracked release note in the consumer's tree whose front-matter tag key
names a tag that exists on the remote, the arm asserts two properties over that
tag's Release:

- **Presence** — a Release exists for the tag, and its body contains the note's
  derived canonical URL (apex host from the CNAME file, path from the configured
  pattern). This is the assertion that kills the empty-body class.
- **Resolution** — every apex-hosted URL *as literally written in the body*
  answers 200. This is the assertion that kills the trailing-slash class.

**The two run over two different strings, and collapsing them re-opens the defect
the entry filed.** A body carrying `…/posts/<slug>/` *contains* `…/posts/<slug>`
as a substring, so a presence-only check passes the trailing-slash form; and a
probe that derived the URL itself and then resolved *that* would confirm its own
arithmetic while the body stayed wrong. Presence runs over the URL the arm
derives; resolution runs over the URLs the body actually carries.

URL extraction is **scheme-anchored over the whole body text**, never
markdown-link-aware: the shipped corpus carries both shapes — a markdown link
whose visible label repeats the URL scheme-less, and a bare URL sitting in
running prose — and only the scheme reliably separates a target from a label.
Trailing sentence punctuation is stripped from an extracted URL before it is
resolved, so a URL ending a sentence does not red on a captured period.

**The arm fails closed on an empty note set.** A glob matching no notes is a
misconfiguration and is itself a finding, which is what makes "set these knobs or
delete the block" an instruction rather than advice. Zero *released* notes is
not a finding — a repository whose notes all precede its first tag is a
legitimate pre-release state and passes. The arm prints a census line (notes
found, of those released, of those checked) on every run, so a reader of the run
log can see the arm did work rather than infer it from silence. That distinction
is the whole difference between this arm and a vacuous pass.

### Delta 2 — three consumer knobs on the probe step's `env:` — **design-bearing**

The seam the entry ruled: generic probe mechanism in the kit, the apex host from
the CNAME file as the existing probe already reads it, and the note-URL pattern
as consumer config. Three knobs carry the consumer half, each shipped in the kit
template as an editable placeholder in the established `ALT_DOMAIN` shape:

- **`RELEASE_NOTE_GLOB`** — the tracked path glob enumerating release-note files.
- **`RELEASE_NOTE_TAG_KEY`** — the front-matter key whose value is the tag a note
  belongs to. Load-bearing rather than decorative: a notes directory routinely
  holds posts that are not release notes, and the key is what separates them.
- **`RELEASE_NOTE_URL_PATH`** — the site path a note is published at, written
  with a `{slug}` token the arm substitutes with the note filename minus its
  extension. It holds the **path only**; the host is never repeated here, because
  the CNAME file stays the single source for it.

**Holding the host out of the knobs is a gate constraint, not only taste.**
`check-docs-cname-parity` runs on every commit over every tracked file and reds a
configured host alias appearing in a `://` URL, so a probe that hard-coded a site
URL would red the battery in this repository and in any consumer that configured
its aliases. The `ALT_DOMAIN` idiom — a bare hostname in `env:`, the URL built at
run time — is the shape that survives that gate, and these knobs keep it.

These are **step-level workflow env, not `SITE_KIT_*` knobs**, and are not loaded
by `lib/site.sh`. The template is copied and edited, not sourced — `ALT_DOMAIN`
already established that shape for monitor knobs — and reaching into the gate
config loader would couple the monitor to the gates dir and hard-code a vendored
kit path into a workflow whose whole distribution model is verbatim copy.

### Delta 3 — `contents: read` and a probe-step `GH_TOKEN` — **mechanical**

The workflow's `permissions:` block currently names `issues: write` alone. **A
`permissions:` block is an allowlist, not an addition**: every scope it does not
name is set to `none`, so reading the tag list and the Release bodies requires
`contents: read` declared explicitly, and the probe step requires its own
`GH_TOKEN` — today only the issue step carries one.

Its absence does not announce itself. GitHub masks an unauthorized read as an
absent resource, so the failure arrives as an HTTP 404 that reads like "no such
Release" — the same permission signature the sibling entry
`release-credential-precondition-scope-vs-permission` documents from the
`v0.19.0` release. On a public repository the declaration is redundant today and
the arm would appear to work; at a private-repo consumer it is the difference
between the arm working and the arm 404ing on every note. Executing the delta is
a two-line pin; knowing it is owed is the design.

**No gate anywhere parses a `permissions:` block**, so this delta has no oracle
and is landed on the reading alone. Stated because a build session that assumed
a green battery meant a verified permission would be assuming wrong.

### Delta 4 — the tag list comes from the API, never `git tag` — **design-bearing**

`actions/checkout` defaults to `fetch-depth: 1` and fetches no tags, so a
`git tag`-driven arm would find zero released notes and report green forever —
the exact vacuous-pass class the `v0.19.0` note closed elsewhere in this
repository. The arm reads the tag list over the API, paginated so a consumer past
the first page is not silently truncated.

The tag list is genuinely needed and is not replaceable by a per-note
Release-by-tag lookup: a 404 on that call cannot distinguish "the tag was never
pushed" — a legitimate skip for a note whose release is deferred — from "the tag
exists and its Release is missing", a real finding. Conflating them either reds
every deferred note or hides a missing Release.

### Delta 5 — the `# enforce:` marker and the enforcement map — **mechanical**

The marker names what the monitor covers, and the arm widens that, so the marker
text changes in **both** files. Only one of them projects: `enforcement-map.sh`
prunes template paths, a deliberate dormancy ruling gate-sdk/SPEC.md states, so
the repository's own `.github/workflows/` copy owns the live row and the kit
template's marker projects nothing. The consequence for build is asymmetric —
editing the repository copy's marker makes `docs/enforcement.md` stale and
`check-enforcement-fresh` reds until it is regenerated, while editing the
template's marker reds nothing and is owed anyway, because divergent markers are
gated by nothing at all.

The class stays `monitor`. No new tier, no `gates.list` entry, no fixture pair,
and no `# graph:` manifest — the arm is not a gate, so neither the pre-commit
hook nor the check-graph artifact goes stale.

### Delta 6 — this repository's copy of the workflow — **mechanical**

`.github/workflows/site-health.yml` is a consumer copy that already diverges from
the template by comment wording and its `ALT_DOMAIN` value. It takes the same arm
with this repository's knob values filled in. **No parity or freshness gate
exists between the two files** — not `check-template-copy-parity`, which pairs
`.sh` templates against the gates dir — so the mirroring is done by hand and a
missed half is caught by nothing. That is why it is its own delta rather than a
clause on Delta 1.

### Delta 7 — the site-kit spec and README — **design-bearing**

`site-kit/SPEC.md` §templates/site-health.yml gains the arm, its three knobs, the
fail-closed-on-empty-glob rule, and the coverage limit below. §Layout and
configuration gains the sentence separating the monitor's step env from the
`SITE_KIT_*` knobs `lib/site.sh` resolves. §The monitor boundary gains the
refinement below. `site-kit/README.md` updates its paragraph on the template and
its Install step 4. `site-kit/SPEC.md` is byte-mirrored under `docs/`, so the
mirror is regenerated in the same unit or `check-docs-mirror-fresh` reds.

**The boundary refinement, which is the design content of this delta.** The
section draws the gate/monitor line at *whether a commit caused the failure* — a
probe reds on DNS, an incident, a stalled renewal, none of them commit-shaped.
This arm does not fit that framing: its failure cause **is** commit-shaped in
spirit, a session that skipped a step, and it is still not gateable. The line
that actually holds is **where the asserted object lives**: a gate asserts over
the tree, and the Release body is host state no checkout contains. Stating the
line that way keeps the section true of every arm instead of true of the original
five and awkward about this one.

**Stated coverage limit.** The arm is driven from the tree's notes, so it is
total over note→Release and silent on the reverse: a Release carrying no note
anywhere in the tree reds nowhere. The deliverable is the note's pointer, and the
population that has ever gone wrong is the pointer's.

### Delta 8 — `RELEASING.md` step 6 — **design-bearing**

Step 6's closing sentence — that the body is out of the battery's reach, so the
hand-check "is the only thing standing between a typo and a dead link in a
permanent artifact" — stops being true once the arm ships, and a step whose
stated reason has quietly expired is how a false premise survives to the next
session. The step keeps its hand-verification, because the monitor is next-day
and issue-shaped while the release session is the only actor who can fix the body
before anyone reads it, and gains the named backstop with its honest latency.

## Producers and consumers

**The arm's findings.** Producer: the `probe live site` step's script, reached on
the workflow's daily `schedule` cron and on `workflow_dispatch` — the same two
triggers the existing arms ride, with no new trigger and no enabling config
beyond Delta 3's permission. Consumer: the existing `file or clear the
site-health issue` step, by the `steps.probe.outputs.status` and
`steps.probe.outputs.report` outputs it already reads. **The arm introduces no
new output, no new step, and therefore no new field** — its findings are strings
appended to the `failures` array the report is assembled from. Final reader: the
operator, in the fenced report block of the `site-health` issue that step opens
or updates; the recovery path closes the issue on a green run, unchanged.

**The three knobs.** Producer: the consumer, editing the copied workflow at
install time — the `ALT_DOMAIN` mechanism exactly, not a repository secret or
variable, so nothing outside the file has to be provisioned for the arm to run.
Consumer, one named reader each, one transition each, inside the probe script:
`RELEASE_NOTE_GLOB` is read by the note enumeration; `RELEASE_NOTE_TAG_KEY` by
the front-matter tag extraction; `RELEASE_NOTE_URL_PATH` by the canonical-URL
derivation feeding the presence assertion alone. No knob is read at a second
transition, and none is populated anywhere else.

**The apex host.** Producer: the CNAME file, already read at the top of the probe
step and already the single gated source of truth. Consumer: the arm, for both
the derived canonical URL and the filter selecting which of a body's URLs it
resolves. No second copy is introduced.

**`permissions: contents: read`.** Producer: the `permissions:` block in both the
kit template and this repository's copy — the enabling config a deployed
configuration actually sets, not a test-only one. Consumer: the `gh` calls in the
probe step, and `actions/checkout`, at run time.

**Existing gates that already reach the new code**, recorded so build neither
re-derives them nor works around them:

- `check-action-run-shell` extracts every literal `run:` block and runs
  ShellCheck at `-S warning` under the step's dialect, so the arm's bash is
  lint-governed like a script. It also **fails closed** rather than skipping on a
  folded scalar, an explicit indent indicator, a YAML alias, or an unbalanced
  `${{` on a body line — so the arm stays a plain `run: |` literal block and
  keeps expression syntax out of its body.
- `check-action-gh-repo` requires a job invoking `gh` to carry a checkout ordered
  before the call, set `GH_REPO`, or pass `--repo` everywhere. The probe step's
  checkout is already the job's first step, so the arm's `gh` calls satisfy it as
  written and no redundant `--repo` is owed.
- `check-docs-cname-parity`, as Delta 2 states.

**Cost per run.** Two API calls plus one per released note, and one resolution
request per apex URL in each body — linear in release count, on a daily
schedule, well inside the authenticated rate limit. Deliberately uncapped and
unknobbed: the two shipped defects were found in old releases, which is precisely
the population a "newest N only" bound would stop probing.

## Existing sections updated

- `site-kit/SPEC.md` §templates/site-health.yml — the arm, its knobs, the
  empty-glob rule, the coverage limit (Delta 7, describing Deltas 1-2).
- `site-kit/SPEC.md` §Layout and configuration — the step-env versus
  `SITE_KIT_*` separation (Delta 7, describing Delta 2).
- `site-kit/SPEC.md` §The monitor boundary — the tree-versus-host refinement
  (Delta 7).
- `site-kit/README.md` — the template paragraph and Install step 4 (Delta 7).
- `site-kit/templates/site-health.yml` — the header comment block, which
  enumerates what the probe covers and what a consumer must set or delete
  (Deltas 1-3, 5).
- `.github/workflows/site-health.yml` — the same, with this repository's values
  (Deltas 1-3, 5, 6).
- `RELEASING.md` step 6 — the expired "out of the battery's reach" reasoning
  (Delta 8).

Three generated projections go stale on the above and are regenerated in the same
unit rather than left to a later gate run: the enforcement map (Delta 5), the
docs mirror of `site-kit/SPEC.md` (Delta 7), and the value rollup, which couples
`.github/workflows/*.yml` and so re-fires on Delta 6.

## Ruled out

Recorded so build does not re-litigate them under time pressure.

- **A close-stage affordance.** Ruled out on the queue entry: an affordance
  cannot catch its own omission, since the failure mode is a body never filled —
  the case where the session skipped the step and would skip the affordance too.
- **A hermetic gate.** The body is host state; no checkout contains it. This is
  the boundary Delta 7 restates, not an exception to it.
- **A zero-knob variant** — walk the Releases, require each body to carry at
  least one apex URL and all of them to resolve. Cheaper and knob-free, but it
  cannot pair a Release with *its own* note, so a body pointing at the wrong post
  passes clean, and it cannot see a released note whose Release is missing
  entirely. The entry ruled the tree-driven design, naming the note's existence
  in the tree as one of the two queryable halves.
- **Presence-only, or resolution of a self-derived URL.** Each alone passes one
  of the two shipped defects; the reasoning is in Delta 1.
- **`git tag` for the tag list.** The `fetch-depth: 1` vacuous pass, Delta 4.
- **A separate workflow step for the arm.** It would need its own output pair and
  a second issue-body assembly, duplicating the signal path to gain nothing; the
  existing step's collector already reaches the issue.
- **Sourcing `lib/site.sh` for the knobs.** Delta 2 — it couples the monitor to
  the gate config surface and hard-codes a vendored kit path into a copied
  workflow.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls site-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Both copies carry the arm** — the kit template and this repository's
      workflow, mirrored by hand because no gate compares them (Delta 6).
- [ ] **The arm is exercised, not assumed** — dispatched once after landing, with
      the census line in the run log showing a non-zero checked count. An arm
      that has only ever been reasoned about is the vacuous pass this design
      spends most of its length avoiding.
