# SPEC amendment: published-release-channel-flag-unheld

The declared release channel governs two surfaces and reaches neither of the
ones a reader actually sees. `check-release-channel-parity` holds the
Release-*creating* invocation (invariant A) and the newest version line
(invariant B); nothing holds the **Releases already on the host**. The result is
live and widens per release: `.github/workflows/publish.yml:311` passes
`--prerelease` unconditionally, so every tag cut since the channel was
mechanized is a prerelease, and GitHub resolves *Latest* to the newest release
that is **not** — v0.21.0, from before the mechanization. The repo front page
and `https://.../releases/latest` therefore advertise a release predating the
prebuilt gate binary, the uninstall verb, and the install-claim repair.

**Re-verified at authoring rather than taken from the entry.**
`gh release list` returns v0.22.0 `Pre-release` above v0.21.0 `Latest`, and
`gh api repos/{owner}/{repo}/releases/latest` returns v0.21.0 with
`prerelease=false`. The corpus scope surveyed is unchanged since its recorded
rev, so the survey record's finding is cited rather than re-bought
(`.workflow/survey-record.md`, 2026-08-08 scope).

**Nothing in the tree is false, which is why nothing reds.** An unsilenced sweep
for readers of the Latest pointer across the whole component set — every tracked
`*.sh`, `*.md`, `*.yml`, `*.json`, `*.ps1` — finds none. `site-health.yml`'s
release-body arm reads `releases/tags/<tag>`, explicitly. The README badge reads
`github/v/tag`, so it shows v0.22.0 correctly. `docs/install.md` links the
releases *list* and explicit `releases/download/vX.Y.Z/` URLs. The exposure is
entirely host-side and reader-side, which is exactly the class the queue entry
names: the tree can be perfectly self-consistent while the front door advertises
the wrong thing.

**The design this amendment owes, stated before its deltas.** The cheap reading
— flip the newest Release to non-prerelease so Latest tracks it — is refused,
because it contradicts invariant A and guts the mechanization
`docs/install.md` §The release channel calls load-bearing. What is settled here
instead is what a `preview`-channel project should present as current *at all*,
and the answer falls out of a rule the project already ruled rather than a new
policy.

## What changes

**1. Invariant B becomes a per-release property; that is invariant C.**
*(design-bearing)* Invariant B already rules that a `0.x` version line demands
channel `preview` and `v1.0.0` onward demands `stable`, and invariant A already
rules that `preview` demands the prerelease flag on creation. Compose them and
the flag on **any** Release is determined by that Release's own version line:

> **Invariant C — channel ⟷ published Release history.** Every published
> Release's prerelease flag agrees with the channel its own version line
> implies: a `0.x` tag carries the flag, a `1.x`-or-later tag does not.

Nothing here is a new decision. B was always a per-release statement that only
ever got checked against *the newest tag*, because the gate had one version line
to read; C is B evaluated over the accumulated history the queue entry says the
declaration "was meant to govern". Stating it this way is what makes the desired
host state **derived rather than stored** — there is no roster of tag-to-flag
rows to maintain, no entry in the private ops runbook, and the rule keeps
working across the `v1.0.0` flip with no edit, because at that flip the newest
tag stops being `0.x` and becomes Latest on its own.

**The consequence, stated plainly because it is what a reader sees.** Every
release this project has ever published is `0.x`. Under C, therefore, **no
Release is Latest** — the green badge disappears, the repo sidebar shows the
newest release labelled `Pre-release`, and `/releases/latest` returns 404. That
is the honest presentation of a channel whose own section already says *the tags
are preview-channel iteration artifacts, and a launch announcement is a separate,
later event*. A 404 is also the loud failure mode where the alternative is the
silent one: third-party tooling resolving that URL today gets a confidently
wrong answer, and after C it gets an error it can see.

**2. The one-time backfill flips the 21 historical Releases.** *(mechanical)*
`gh release edit <tag> --prerelease` over every published Release that carries
no flag — 21 of the 22 published, measured at authoring. It is all of them or
none: flagging only the newest unflagged Release walks Latest backward to the
next one. This asserts nothing revisionist — every one of those tags is `0.x`,
so the flag states what was true at each Release's own publication.

**The entry's stated blocker does not hold and must not be inherited.** It
records the write as blocked, on the grounds that a `gh release edit` 404s on
this machine per `release-runbook-identity-diagnosis`. Scope's survey falsified
that at HEAD: the active host-CLI account is the namespace-owning one, so the
plural-identity fault is **latent rather than armed** and the write is
available. RELEASING.md's clause that a 404 on a write is a permission signature
stays exactly as written — it is the diagnosis to reach for if this backfill
*does* 404, and the resolution stays *fix the permission, never switch
identity*.

**3. Invariant C is held by the monitor, not by the battery.** *(design-bearing)*
A seventh arm on `.github/workflows/site-health.yml`: read the Release list from
the API (`gh api --paginate`, streamed, for the reason the release-body arm
already documents), and report every Release whose flag disagrees with its own
version line. A disagreement opens or updates the single `site-health` issue and
reds the run, like every other arm; recovery self-clears.

This settles the entry's `[design-pending]` question — the three candidate homes
were the private ops runbook as recorded desired state, a scheduled probe, or a
backfill step in the release procedure — and the choice is forced rather than
preferred:

- The **battery cannot hold it**. Host state is out of a precommit gate's reach
  by construction, and RELEASING.md step 6 already rules this exact class: *the
  body lives on the host, out of the battery's reach, so its backstop is a
  monitor rather than a gate*. C is the same class of fact about the same
  objects.
- The **ops runbook cannot hold it**. Desired state that no automation reads is
  a note, and this desired state is not an operator preference to record — it is
  derived from a tracked declaration, so writing it down anywhere would be a
  second owner for a fact `docs/install.md` already owns.
- A **release-procedure step cannot hold it**. It would fire only at release
  time and could never catch the drift that already happened, which is the whole
  instance in front of us.

The arm needs no new knob and reads no file: it derives each expectation from
the tag it is looking at. It also needs no `permissions:` change — `contents:
read` is already declared for the release-body arm, and the SPEC's warning that
the block is an allowlist rather than an addition is why that is worth checking
rather than assuming.

**4. The arm lands in this repo's copy and *not* in site-kit's template.**
*(design-bearing)* This is the seam ruling. `check-release-channel-parity.sh`
lives in `scripts/` — a repo gate, not a kit gate — so the `Release channel:`
declaration and both invariants it holds are **repo-local mechanism, owned by no
kit**. Arm 7 implements a composition of those two invariants, so it cannot be
truer than they are: copied into a consumer tree it would assert a convention
the consumer never declared and no kit ships them, making it dead config in
every consumer that took the template verbatim. The widest tier true for every
reader of it is this repo.

The re-entry condition is named rather than left to be rediscovered: **if the
channel declaration ever becomes kit mechanism — a kit shipping
`check-release-channel-parity` and the declaration grammar it reads — arm 7
moves into the template with it**, as an optional arm on the existing
set-the-env-or-delete-the-arm pattern. Nothing in this amendment forecloses
that; the template is copied and edited rather than synced, and no gate couples
the two files.

**5. `docs/install.md` §The release channel is corrected, because this amendment
makes one of its sentences false.** *(design-bearing)* That section asserts
**Mechanized on exactly one surface** — the creating invocation. After C it is
mechanized on two, on two different tiers, and the section must say so or it
misleads exactly the reader who consults it to learn what the channel does. It
gains invariant C beside A and B, with C marked as monitor-held rather than
gate-held and pointed at its arm; the reader-facing consequence (no Release is
Latest while the line is `0.x`; use the releases list or an explicit version, not
`/releases/latest`); and the note that A remains the *creating* posture while C
is the *accumulated* one, which is why neither subsumes the other.

The same section's npm dist-tag paragraph gains the contrast, because that
paragraph is the design precedent for this identical trade and reading the two
side by side is what stops the next session re-deriving the argument. **The
precedent is applied here, not contradicted.** The dist-tag change was declined
to protect front-door time-to-first-value: a non-default tag makes §Quick start's
one-command install resolve to nothing until a reader learns to append the
channel. That cost is what decided it — and it is absent here, because no
documented install path resolves the Latest pointer at all (delta's premise, the
reader sweep above). Same preference, opposite outcome, because the front door
is not on this path. The dist-tag's own re-entry condition is untouched.

**6. `RELEASING.md` step 7 gains the presentation contract.** *(mechanical)* Step
7 is where a releaser looks at how the release presents, and after this change
they will see a repo with no Latest badge. Without a sentence there, the
helpful next action is to flip the newest Release — silently reverting this
unit with a UI click no gate and no monitor would attribute. The step states
that the absent Latest badge is the declared outcome of the `preview` channel,
names invariant C's owner, and says the monitor is what reports a flag that
drifted. It adds no hand-verification step: creation already carries the flag
and the monitor is the backstop, so a hand check would be a third owner.

**7. `site-kit/SPEC.md` §templates/site-health.yml records that the template is a
floor.** *(mechanical)* It says the workflow is *copied verbatim*, which now
reads as a prohibition on the thing delta 4 does. It gains the clause that a
consumer may add arms to its copy, and that an arm implementing a
consumer-local invariant belongs in the consumer's copy rather than in the
template — which is the general form of delta 4's seam ruling and the reason the
kit's own arm roster is a floor rather than a closed set.

**8. The projections the edited workflow stales are regenerated.**
*(mechanical)* The `# enforce:` marker on `.github/workflows/site-health.yml`
summarizes what the monitor covers, so arm 7 edits it — a class-registry change
that stales `docs/enforcement.md`. The file's line count changes, which stales
`docs/footprint.md` and `docs/value.md`'s rollup. Each gate prints its own regen
command on red; the roster and the ordering rule (regenerate the footprint after
`git add`, never before) are `docs/site-architecture.md` §Generated projections'
and are followed, not restated.

## Producers and consumers

The new interface is **invariant C** and the host-side state it asserts over:
the prerelease flag of every published Release. No new field, event, or message
is added to the tree, so the causal chain is a derived expectation and its
readers.

- **Producer (ongoing)** — `.github/workflows/publish.yml`'s `release` job, at
  `gh release create "$TAG" --title "$TAG" --prerelease --notes ""`. It is
  **unchanged by this amendment**: it already produces the state C demands for
  every `0.x` tag, and its enabling configuration is the tag push that fires the
  workflow, which is the shipped path rather than a test one. Delta 1 names what
  it was already producing.
- **Producer (one-time)** — delta 2's backfill, which brings the pre-mechanization
  Releases into the state the ongoing producer has produced since. After it, the
  producer set is closed: no path creates an unflagged `0.x` Release.
- **Consumer 1 — arm 7 of `.github/workflows/site-health.yml`**, by
  `gh api --paginate` over the Release list on its schedule. Read at the
  transition where the arm compares each Release's flag against its own version
  line, and consumed by opening or updating the `site-health` issue.
- **Consumer 2 — the GitHub host itself**, which computes the Latest pointer from
  exactly this flag. This is the consumer the unit exists for: it is what
  produces the front-page badge and the `/releases/latest` resolution, and it is
  named here rather than left implicit because every reader-facing effect of the
  change is its output.
- **Consumer 3 — `docs/install.md` §The release channel**, as the declared
  contract a human reads. Read at the transition where a reader asks what the
  channel means for the release they should take, and updated by delta 5 so the
  answer names the releases list and explicit versions rather than a pointer
  that will 404.
- **Consumer 4 — `RELEASING.md` step 7**, read by the releaser at the one moment
  the absent Latest badge is in front of someone with permission to remove it
  (delta 6).

**There is no fifth consumer, and that was checked rather than assumed.** The
reader sweep recorded above found no tracked file resolving `releases/latest`,
so no in-tree path changes behavior when it starts returning 404 — which is the
same fact that explains why nothing red before.

**Why the drift was silent, stated because the fix must close it.**
`check-release-channel-parity` reads two files off disk and one version line; it
never touches the network, correctly, since the battery is offline by tier. So
the surface that *declares* the channel was held against the surface that
*creates* Releases, and never against the Releases themselves — and the two
could agree perfectly while the accumulated history disagreed with both. Delta 3
closes that by putting the third assertion on the tier that can actually see the
host, rather than by weakening the gate's tier.

## Existing sections updated

- **`docs/install.md` §The release channel** — owns the declaration and the
  invariant list. Gains invariant C, its monitor-held tier, the reader-facing
  consequence, and the correction to *Mechanized on exactly one surface*; its
  npm dist-tag paragraph gains the front-door asymmetry that makes this the same
  preference rather than a reversal. *(Owns deltas 1, 5.)*
- **`RELEASING.md` step 7** — owns what a releaser verifies about presentation.
  Gains the statement that no Latest badge is the declared outcome, and that the
  monitor rather than a hand check reports a drifted flag. *(Owns delta 6.)*
- **`site-kit/SPEC.md` §templates/site-health.yml** — owns the template's
  distribution model and its arm roster. Gains the floor-not-ceiling clause and
  the rule that a consumer-local invariant's arm belongs in the consumer's copy.
  *(Owns deltas 4, 7.)*
- **`.github/workflows/site-health.yml`'s `# enforce:` marker** — the class
  registry's source for this monitor. Its summary gains the release-channel
  assertion. *(Owns deltas 3, 8.)*

The **queue entry itself** carries a stated ground that delta 2 falsifies (the
write is available, not blocked); build corrects it when the entry is closed
rather than leaving a false blocker in the record.

No section is listed here that no delta claims. In particular
`scripts/check-release-channel-parity.sh` is **not** updated: invariants A and B
are unchanged, and adding C to a gate that cannot reach the host would be the
one change this amendment exists to refuse.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The monitor is red before it is green** — arm 7's comparison is run
      against the host **before** delta 2's backfill and confirmed to report the
      21 offenders, then run after and confirmed clean. An arm that never saw
      the drift is an arm that cannot hold it out, and this is the one case
      where the defect is still standing when the check is written.
- [ ] **The host's response is observed, not predicted** — after the backfill,
      confirm by oracle that `gh api repos/{owner}/{repo}/releases/latest` 404s
      and that the repo page shows the newest release without a Latest badge.
      Delta 1 asserts a behavior of a system outside this tree; if the host does
      something else, the amendment's premise is wrong and that is an escalation
      rather than a fix-up.
- [ ] **The backfill is complete or absent** — every unflagged Release flipped,
      verified by re-listing. A partial backfill leaves Latest pointing at an
      even older release than it does today, which is strictly worse than not
      starting.
