# SPEC amendment: release-body

RELEASING.md step 6, filling in the GitHub Release body, is the only release
step whose product never touches the tree. It was skipped at the `v0.25.0` cut,
and the only thing that noticed was the next day's `site-health` monitor. This
amendment chooses among the three shapes the paired entry weighed. The body is
now **composed from the tagged tree inside the `release` job that creates the
Release**, so a cut cannot publish a Release without its pointer. Sited at the
repo root because the two surfaces it edits, `RELEASING.md` and
`.github/workflows/publish.yml`, are repo-root governed and belong to no kit.

## What changes

### (1) The `release` job writes the body it used to leave empty

The step that creates the Release resolves the tag's release note from the
tagged tree and passes a composed body with `--notes-file`. It no longer passes
`--notes ""`. **{design-bearing}**

- **The note.** Exactly one tracked `docs/posts/*.md` whose opening front-matter
  fence carries `release: <tag>`. Reading the key only inside that fence matches
  how `site-health.yml`'s release-body arm reads it (site-kit/SPEC.md
  §templates/site-health.yml), so a key quoted in a note's body is not mistaken
  for the note's own tag. If zero notes or more than one match, the step
  **fails before creating the Release**, naming the tag and the count. A tag
  with no note is the defect this unit exists to catch, and it should be caught
  at the cut.
- **The URL.** `https://` plus the host in `docs/CNAME`, plus `/posts/<slug>`,
  where `<slug>` is the note's filename minus `.md`. It has no trailing slash:
  the site serves the bare form and 404s the slashed one, as RELEASING.md step
  6 already warns.
- **The body.** The pointer sentence the earlier Releases carry, *"Full release
  note, including the allowed-red set and every behavior change:"*, then a blank
  line, then the URL. It is a pointer, never a copy of the note, which keeps the
  runbook's single-home rule.
- **A re-run.** When the Release already exists, the step rewrites the body only
  if the existing body lacks the URL. That repairs an empty body and leaves a
  hand-edited one that still points at the note alone.
- **The checkout.** The job checks out nothing today. It gains a pinned
  `actions/checkout` restricted by `sparse-checkout` to `docs/posts` and
  `docs/CNAME`. Its `contents: write` scope already covers the read.

**Why this shape and not the other two.** Having the release-sweep skill emit
the body as a copy-ready artifact keeps the hand step, and the hand step is
what was skipped. Moving the monitor's cadence toward the cut shortens the
window without closing it. The objection the entry raised against generation
was that it puts release-note text on a CI path the battery never runs. That
objection weighs less than it seems, because the text is one fixed sentence and
a URL derived from in-tree facts. The note's own prose never leaves the tree.
And a failure in the step is a red `publish` run, which the closing session is
already watching under step 5, the same hour, not the next day.

### (2) Step 6 becomes a verification

RELEASING.md step 6 stops being a write. The body is composed by the job step
5 watches, and step 6 opens the Release and follows its link. The monitor
paragraph stays: it is still the backstop for a body someone edits after the
cut. **{mechanical}**

### (3) The derivation's three values are this repo's config, cross-checked by the monitor

The note glob, the front-matter key and the URL path are set as step-level env
on the `release` step. The names are `site-health.yml`'s: `RELEASE_NOTE_GLOB`,
`RELEASE_NOTE_TAG_KEY` and `RELEASE_NOTE_URL_PATH`. The values are the ones that
file sets. **{mechanical}**

Two files in one repo now hold the same three values. That is tolerated, not
gated, because the pair has an oracle already. The monitor derives the URL from
its own copy of the values and asserts that each Release body **contains** it.
A publish copy that drifts writes a body the monitor reds on its next run. A
freshness gate over two workflow files would duplicate that check at a tier
that cannot see the host anyway. `site-health.yml` is a copied site-kit
template. `publish.yml` is this repo's own file, and no kit ships it, so none of
the three values crosses into kit mechanism.

## Producers and consumers

**The composed body.** Producer: the `release` job, triggered by a pushed `v*`
tag (`.github/workflows/publish.yml`'s `on: push: tags`), which this repo
deploys. Consumers: a reader of the Release page, who follows the URL, and the
`site-health` release-body arm, which reads it daily for presence and
resolution. Every part has a reader. The pointer sentence is read by the human,
and the URL by the human and by both monitor assertions.

**The failure.** A missing or ambiguous note fails the step. The run is read
by the closing session's `gh run watch` under RELEASING.md step 5, which
already requires both jobs green before continuing.

**Roster-holding readers of `publish.yml`'s release step.** The probe is
`git grep -n 'gh release create'` over the tracked tree.

- `check-release-channel-parity` (`native/src/gates/release_channel_parity.rs`)
  finds the Release-creating step by the line containing `gh release create`,
  and requires `--prerelease` on it. The new invocation keeps both on one line.
  Build runs that gate, which is the oracle for this.
- `check-action-gh-repo` reads the `GH_REPO` env. The comment beside it says
  *"this job checks out nothing"*, which delta 1 makes false, so it is rewritten.
  `GH_REPO` stays, because `gh` in a sparse checkout still benefits from an
  explicit target. Build confirms that against the gate.
- `check-action-pinning` requires the new `actions/checkout` to be pinned by
  SHA, at the pin `gates.yml` already uses.
- `check-action-permissions` is unaffected, since the job already declares
  `contents: write`.

**Point 5.** No corpus is narrowed.

The parity gate's recognizer was read at this stage
(`release_channel_parity.rs`, the `create_step` filter). It collects every line
containing `gh release create` and passes when any one of them carries
`--prerelease`. So swapping `--notes ""` for `--notes-file` on that line keeps
it green, and build still runs the gate on the edited file.

**Inferred, cannot run before build:** — the job runs only on a pushed tag. The
premise waiting on it is that a `sparse-checkout` of `docs/posts` and
`docs/CNAME` places `docs/CNAME` at that path. The first tagged run settles it,
and a failure there is loud (delta 1).

Unaffected, recorded as read: docs/install.md §The release channel's sentence
about *"the same tier ruling `RELEASING.md` step 6 already makes for the Release
body"* stays true, because the monitor still backstops the body.

## Existing sections updated

- `.github/workflows/publish.yml`, the `release` job: the checkout step, the
  step env, the note resolution, and the create or edit. The comments reading
  "the operator fills in the body afterwards" and "this job checks out nothing"
  are rewritten. (deltas 1 and 3)
- `RELEASING.md` §The procedure step 6, rewritten as below. (delta 2)
- `RELEASING.md` §The procedure step 5, the sentence naming what `release`
  attaches, now names the body as well. (delta 1)

### Replacement text

**RELEASING.md §The procedure step 6. Not yet applied.** Replace the step's body
with

> 6. **Verify the GitHub Release body.** Step 5's `release` job created the
>    Release with its body already written: one pointer sentence and the post's
>    `https://checkwright.dev/` URL, derived from the note whose `release:` key
>    names the tag. The post is the note's single home, and the Release is a
>    pointer to it. A tag with no note, or with two, fails that job before the
>    Release exists, so step 5's watch is where a missing note surfaces. Open the
>    Release and follow the link. The body lives on the host, out of the
>    battery's reach, so its backstop is a monitor rather than a gate:
>    `site-health.yml`'s release-body arm asserts daily that each note's Release
>    body carries that URL, and separately that every apex URL the body carries
>    resolves (site-kit/SPEC.md §templates/site-health.yml). It is what catches a
>    body edited after the cut.

## Retired spellings

- None — no name, path or token is retired; `--notes ""` is an argument value in
  one workflow line, replaced in place.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — step 6's text carries the
      instruction, and this amendment carries the grounds.
- [ ] **Merged with no information lost** — step 6 re-phrased, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap discovered during the work is filed.
