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

- **Presence** — a Release exists for the tag, and its body **contains** the
  note's derived canonical URL, derived **scheme-qualified**: `https://` + the
  apex host from the CNAME file + the path from the configured pattern. This is
  the assertion that kills the empty-body class.
- **Resolution** — every apex-hosted URL *as literally written in the body*
  answers 200. This is the assertion that kills the trailing-slash class.

**The two run over two different strings, and collapsing them re-opens the defect
the entry filed.** A body carrying `…/posts/<slug>/` *contains* `…/posts/<slug>`
as a substring, so a presence-only check passes the trailing-slash form; and a
probe that derived the URL itself and then resolved *that* would confirm its own
arithmetic while the body stayed wrong. Presence runs over the URL the arm
derives; resolution runs over the URLs the body actually carries.

Presence is therefore deliberately loose in one direction and strict in the
other, and the shipped corpus punishes inverting either. *Containment, never
equality*: `v0.1.0`'s body writes the `.html`-suffixed form of its URL, so a
token-equality test reds it on day one — and the looseness that saves `v0.1.0` is
the same looseness that lets the trailing-slash form through, which is why
resolution exists rather than a stricter presence. *Scheme-qualified*: two bodies
carry a markdown link whose visible label repeats the URL scheme-less, so a
presence test over a scheme-less string is satisfied by the label while the link
target goes unchecked — and a wrong-but-resolving target then passes both
assertions with nothing red.

URL extraction is **scheme-anchored over the whole body text**, never
markdown-link-aware: the shipped corpus carries both shapes — a markdown link
whose visible label repeats the URL scheme-less (2 bodies), and a bare URL
sitting in running prose (17) — and only the scheme reliably separates a target
from a label.

**A trailing punctuation run is stripped from an extracted URL before it is
resolved, and the punctuation that actually occurs is markdown-structural, not
sentential.** The corpus was measured rather than imagined: of 23 apex URLs
across 19 bodies, **zero** are followed by sentence punctuation and **six**, in
six different tags, are followed by a `)` closing a link target or a `)**`
closing a link target inside bold. A stripping set written for the period —
`[.,;:!?]` — captures `…-v0-19-0)**` and `…/install)` and reds six tags on the
arm's first run. The set must cover `)` and `*`, and must strip a *run* rather
than one character, or `)**` merely becomes `)*`.

Not every apex URL in a body is a note pointer: four bodies also link
`…/install` in a prose-labelled markdown link. Resolution covers those
deliberately — the assertion is over apex-hosted URLs, not over note URLs — so
the arm must not filter the extracted set down to the note pattern.

**The zero-cases, ruled exhaustively — this is where the arm is won or lost.**
Every count the arm ranges over can be zero, and each zero is either a finding or
a legitimate pass; leaving one unruled is how the arm becomes a green that means
nothing.

- **The glob matches no notes** — a finding. A misconfigured `RELEASE_NOTE_GLOB`
  is itself the defect, which is what makes "set these knobs or delete the block"
  an instruction rather than advice.
- **The tag-list call fails** — a finding, and it must be asserted on the call's
  own exit status and HTTP code, never inferred from the count that follows. The
  probe step runs under `set +e`, so a failed `gh` invocation yields an empty tag
  list and a zero exit; every note is then unreleased, and the arm reports a
  clean run forever. This is the same masking Delta 3 describes, one call
  earlier: on the per-Release lookup a 404 reads like "no such Release", but on
  the tag-list call it reads like "green".
- **The glob matches notes and none carries the tag key** — *not* fail-closed,
  and stated so a build session does not add a red here by symmetry with the
  first case. A consumer whose notes genuinely precede its first release is in
  this state legitimately. It is nonetheless the likelier of the two knob
  misconfigurations, precisely because the glob's failure is loud and the key's
  is silent, so it is the census line's primary job to make it readable.
- **Zero *released* notes** — not a finding, for the same reason: a repository
  whose notes all precede its first tag is a legitimate pre-release state.

**The census line makes vacuity readable; the rules above are what make it
impossible.** The arm prints notes found, of those released, of those checked on
every run — the same shape as gate-sdk's vacuous-pass tripwire (§run-gates),
where a "0 files scanned" clean line is a *reading* available to whoever opens
the banner, not an assertion. On a green run nobody opens it. So the census line
is the diagnostic that tells a reader which zero-case they are in, and it earns
its place for that; it is not itself the thing standing between this arm and a
vacuous pass. Against this repository today the line reads 20 notes found, 19
released, 19 checked.

### Delta 2 — three consumer knobs on the probe step's `env:` — **design-bearing**

The seam the entry ruled: generic probe mechanism in the kit, the apex host from
the CNAME file as the existing probe already reads it, and the note-URL pattern
as consumer config. Three knobs carry the consumer half, each shipped in the kit
template as an editable placeholder in the established `ALT_DOMAIN` shape:

- **`RELEASE_NOTE_GLOB`** — the tracked path glob enumerating release-note files.
- **`RELEASE_NOTE_TAG_KEY`** — the front-matter key whose value is the tag a note
  belongs to. Load-bearing rather than decorative: a notes directory holds posts
  that are not release notes — one of this repository's 20 today — and the key is
  what separates them. Here it is the front-matter key `release`, and the
  non-note post carries no front matter at all.
- **`RELEASE_NOTE_URL_PATH`** — the site path a note is published at, written
  with a `{slug}` token the arm substitutes with the note filename minus its
  extension. It holds the **path only**; the host is never repeated here, because
  the CNAME file stays the single source for it.

This is the one knob of the three with no prior art in the tree and no
cross-check anywhere — nothing in this repository derives a note's served URL
outside Liquid at render time, which no bash can reach — so it is also the one a
build session can set wrongly and see nothing. The trap is concrete: the site's
generator emits `/posts/<slug>.html` and the host *additionally* serves the
extensionless form, so `/posts/{slug}.html` is the more literally-correct-looking
value and it reds every body written per `RELEASING.md`, which pins the bare
form. **The value is `/posts/{slug}`.** The other two knobs are `docs/posts/*.md`
— already the coupling glob on five gates' `# graph:` manifests — and `release`.

**Holding the host out of the knobs is SSOT and cutover, and the gate reaches
only half of it.** `check-docs-cname-parity` reds a configured host *alias* in a
`://` URL and exempts the CNAME host itself even when that host is also listed
among the aliases — so hard-coding `https://<apex>/posts/…` would pass the
battery cleanly here, as this repository's own tracked prose citing its apex in a
URL already demonstrates. The gate constrains `ALT_DOMAIN`, which names an
alias; it does not constrain an apex-hosted note path. What holds the host out of
`RELEASE_NOTE_URL_PATH` is that the CNAME file is the single gated source for it
and a host rename must stay a one-file edit — the reason the existing probe reads
the host from that file rather than from its own `env:`. These knobs keep the
`ALT_DOMAIN` *shape* (a bare value in `env:`, the URL built at run time) for
consistency and for the alias case, not because a gate would otherwise fire.

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
a green battery meant a verified permission would be assuming wrong. The claim
was tested at align rather than taken on faith — a literal grep, a concept grep
across the `GITHUB_TOKEN` / `GH_TOKEN` / least-privilege / token-scope
spellings, and an inspection of all nine gates that read workflow YAML at all —
and it holds; gate-sdk/SPEC.md §check-action-run-shell declares workflow-security
linting an explicit non-goal, so the absence is a standing ruling rather than an
oversight. A narrower gate than that non-goal is buildable (a job whose `run:`
bodies invoke `gh` must declare the scopes those calls consume), and it is filed
as costed debt on the gap inbox rather than started here — the delta itself
cannot be reshaped to need less, because the permission is genuinely required.

### Delta 4 — the tag list comes from the API, never `git tag` — **design-bearing**

`actions/checkout` defaults to `fetch-depth: 1` and fetches no tags, so a
`git tag`-driven arm would find zero released notes and report green forever —
the exact vacuous-pass class the `v0.19.0` note closed elsewhere in this
repository. The arm reads the tag list over the API, paginated so a consumer past
the first page is not silently truncated.

`gh api --paginate` discharges the paging with no manual page loop. Two
measurements bound the claim honestly: the API's default page is 30 and this
repository has 19 tags, so the pagination is **forward-looking and unexercised
here** — it buys nothing today and everything at tag 31, which is also why no
run of the arm will ever tell us it works. And `--paginate` concatenates the
pages' arrays, so `--jq 'length'` returns the *last page's* length rather than
the total: the census number would understate exactly once pagination begins to
matter. Count by streaming the elements.

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
with this repository's knob values filled in (Delta 2 names all three). **No
parity or freshness gate exists between the two files** — not
`check-template-copy-parity`, which pairs `.sh` templates against the gates dir —
so the mirroring is done by hand and a missed half is caught by nothing. Checked
against the whole gate roster at align rather than assumed: no gate compares two
hand-maintained copies of anything outside that `.sh` pairing. That is why it is
its own delta rather than a clause on Delta 1. The missing oracle is pre-existing
debt this arm widens rather than creates — the two files already diverge — and it
is filed as costed debt on the gap inbox, shaped after
`check-template-copy-parity`'s declared-divergence contract rather than a byte
compare, since these two copies are *meant* to differ.

### Delta 7 — the site-kit spec and README — **design-bearing**

`site-kit/SPEC.md` §templates/site-health.yml gains the arm, its three knobs, the
zero-case rulings, and the coverage limit below. §Layout and configuration gains
the sentence separating the monitor's step env from the
`SITE_KIT_*` knobs `lib/site.sh` resolves. §The monitor boundary gains the
refinement below. `site-kit/README.md` updates its paragraph on the template and
its Install step 4 — which already carries the `ALT_DOMAIN` set-or-delete idiom
the three knobs join. `site-kit/SPEC.md` **and `site-kit/README.md`** are both
byte-mirrored under `docs/`, so both mirrors are regenerated in the same unit or
`check-docs-mirror-fresh` reds.

**§The monitor boundary is edited in place, never renamed.**
`gate-sdk/bin/enforcement-map.sh` hard-codes that heading's title and slug as the
monitor class's owner reference, so a rename would break the emitter silently and
stale `docs/enforcement.md` by a second route — a cross-kit coupling that exists
nowhere in either kit's prose.

**The boundary refinement, which is the design content of this delta.** The
section carries two framings and leans on the weaker one: it opens "A gate
verifies the tree", then justifies the line by *whether a commit caused the
failure* and enumerates three causes — DNS, a Pages incident, a stalled renewal
— none of them commit-shaped. This arm does not fit that second framing: its
failure cause **is** commit-shaped in spirit, a session that skipped a step, and
it is still not gateable. The refinement promotes the framing already in the
section's first sentence to the one that governs: the line is **where the
asserted object lives**, a gate asserts over the tree, and the Release body is
host state no checkout contains. That keeps the section true of every arm
instead of true of the three causes it happens to list and awkward about this
one. (The probe's five numbered arms are enumerated in the template and
summarized in §templates/site-health.yml, not here — that is the separate
listed edit, and this section stays cause-free.)

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
  before the call, set `GH_REPO`, pass `--repo` everywhere, or declare a
  `# gh-repo-exempt:` valve; the disjunction is evaluated per job, not per call.
  The probe step's checkout is already the job's first step, so the arm's `gh`
  calls satisfy it as written, no redundant `--repo` is owed, and no valve is.
- `check-docs-cname-parity`, as Delta 2 states — which is to say it constrains
  `ALT_DOMAIN` and not this arm's knobs.

**Prior art the arm should copy rather than re-derive.** Note enumeration by
front-matter tag key already exists three times in this tree, in two different
spellings: `scripts/check-release-bump.sh` anchors the key *inside* the opening
`---` fence and builds the tag→file map this arm needs;
`scripts/check-tightened-gates-grammar.sh` and `gate-sdk/bin/upgrade-smoke.sh`
match `^release:` anywhere in the file. The arm cannot reuse any of them as code
— Delta 2's reason for not sourcing `lib/site.sh` applies to a copied workflow
just as hard — but it is writing a fourth spelling of a shipped grammar, and the
anchored one is the spelling to copy. The looser two are a latent
false-positive surface, not this unit's to fix.

**Cost per run.** The paginated tag list plus one Release lookup per released
note, and one resolution request per apex URL in each body — linear in release
count, on a daily schedule, well inside the authenticated rate limit. Measured
against this repository today: 19 tags on one page, 19 Release lookups, 23
resolution requests. Deliberately uncapped and unknobbed: the two shipped defects
were found in old releases, which is precisely the population a "newest N only"
bound would stop probing.

**The arm lands green.** All 19 tags carry a Release, every body carries at
least one apex URL, and all 23 resolve 200 as literally written — verified at
this align audit, not assumed — so no companion repair ships with the arm and a
red on its first dispatch is a defect in the arm, not in the corpus. That is
what makes the DoD's "exercised, not assumed" item a real test rather than a
formality.

## Existing sections updated

- `site-kit/SPEC.md` §templates/site-health.yml — the arm, its knobs, the
  zero-case rulings, the coverage limit (Delta 7, describing Deltas 1-2).
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
- `docs/site-kit/index.md` and root `README.md`'s site-kit registry row — both
  restate the probe's arm roster in hand-authored prose ("HTTPS, redirects,
  certificate expiry") and both go stale the moment the arm lands (Delta 7,
  describing Delta 1). **Neither is generated and neither is gated**:
  `check-docs-kit-parity` holds the `docs/kits.md` row and the nav block, never
  the prose, and root `README.md` carries no marker block at all. This is Delta
  6's uncaught-mirroring problem one level out — the same roster restated in four
  places, two of them with an oracle and two without.

**The projections, corrected by running the edit set rather than reasoning about
it.** Two *gated* projections go stale, producing three stale files, and all are
regenerated in the same unit rather than left to a later gate run:

- `docs/enforcement.md` — `check-enforcement-fresh` (Delta 5), from the
  `.github/workflows/` marker alone; the template's marker projects nothing.
- `docs/site-kit/SPEC.md` **and** `docs/site-kit/README.md` —
  `check-docs-mirror-fresh` (Delta 7). Both, not just the SPEC.

The value rollup does **not** go stale. `.github/workflows/*.yml` sits in its
`couples=` manifest, which decides when the gate *runs*, not what the projection
*contains*: the rollup carries per-class row counts, and this unit edits an
existing marker's text without adding a row. Re-firing is not staleness, and the
two are easy to conflate because the gate does execute.

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
- [ ] **The arm roster reads the same in all four places** — both workflow header
      comments, `docs/site-kit/index.md`, and root `README.md`'s registry row.
      Two of the four have an oracle; this checkbox is the oracle for the others.
- [ ] **The arm is exercised, not assumed** — dispatched once after landing, with
      the census line in the run log showing a non-zero checked count. An arm
      that has only ever been reasoned about is the vacuous pass this design
      spends most of its length avoiding.
