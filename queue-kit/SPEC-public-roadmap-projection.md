# SPEC amendment: public-roadmap-projection

A generated public roadmap projected off the queue, plus the issue forms that
give pre-launch usability signal an inlet. The queue is the single source: no
hand-maintained roadmap copy exists at any point, and staleness is a red commit
rather than a reader's discovery.

## Seam ruling

Kit mechanism: the `[roadmap:]` tag's **spelling and grammar**, the projection
emitter, the freshness gate, and the marker-block protocol. Consumer config: the
**horizon vocabulary**, the **track vocabulary**, and the projection's file and
marker names — a kit literal spelling `now`/`next`/`later` or
`adoption`/`reliability`/`ecosystem`/`commercial` would ship one project's
roadmap posture as everyone's, the same leak `QUEUE_KIT_LESSON_TAGS` and
`QUEUE_KIT_PROSE_SURFACE_GLOBS` already avoid by shipping empty. Private rule
content: none — nothing here needs a term list.

The issue forms are **consumer repo-meta**, not kit surface at all: they land in
`.github/` under this repo's own governance and no kit knows they exist.

## What changes

### 1. The `[roadmap: <horizon>/<track>]` tag — *design-bearing*

A new member of the tag algebra (queue-kit/SPEC.md §The tag algebra), fixed
spelling, on the entry's lead line like every other tag. It marks an entry as
**curated onto the public projection** and carries the two facts a projected
item needs: which horizon it sits in and which track labels it.

One tag with two slash-joined fields rather than two tags, and the reason is
that the fields are never independently meaningful: a track with no horizon has
no slot on the page, and a horizon with no track has no label in it. The pairing
is intrinsic, so splitting it would invent two governed names where the domain
has one. The lead-line budget seconds it — `check-queue-wrap`'s 100-column floor
over a bullet already carrying `[needs-spec]` leaves about 21 columns for the
opening words at `[roadmap: later/ecosystem]`, and a second bracketed tag would
spend the rest.

Both field values are drawn from consumer-configured ordered arrays. An entry
carries at most one `[roadmap:]` tag; an untagged entry is simply not projected,
which is the normal case for most of the queue.

Placement is unconstrained by section: an active-section entry and a deferred
entry may both be projected. The projection is about *public direction*, not
selection order, and an entry currently being built is exactly what belongs
under the nearest horizon.

### 2. Config knobs — *mechanical*

Four knobs on the `QUEUE_KIT_` shape (queue-kit/SPEC.md §Layout and
configuration), each defaulting so an unconfigured consumer gets a clean skip
rather than a kit-shaped roadmap:

- `QUEUE_KIT_HORIZONS` — ordered array of horizon names, default empty. Order is
  the emitted section order.
- `QUEUE_KIT_TRACKS` — array of track labels, default empty.
- `QUEUE_KIT_ROADMAP_FILE` — repo-relative path of the projection page, default
  empty (no page, gate skips).
- `QUEUE_KIT_ROADMAP_MARKER` — the marker-block token, default `roadmap`,
  delimiting `<!-- roadmap:begin -->` / `<!-- roadmap:end -->`.

`QUEUE_KIT_HORIZONS` or `QUEUE_KIT_TRACKS` set while the other is empty is
malformed config: `lib/queue.sh` exits 2, per the loader's existing
broken-grammar contract. A half-configured vocabulary would silently accept
every track value.

This repo's `scripts/queue-config.sh` sets the three that have no usable
default — `QUEUE_KIT_HORIZONS=(now next later)`,
`QUEUE_KIT_TRACKS=(adoption reliability ecosystem commercial)`,
`QUEUE_KIT_ROADMAP_FILE=ROADMAP.md` — so the tag's enabling config is live in a
real configuration, not test-only. `QUEUE_KIT_ROADMAP_MARKER` stays on its kit
default and is deliberately absent from the consumer config: a consumer line
restating a kit default is the de-literalization defect, not a completeness win.

### 3. `queue-kit/bin/roadmap.sh` — *design-bearing*

The projection emitter. A tool, not a gate — no `# graph:` manifest, following
`bin/queue-index.sh`.

`--emit` prints the generated block to stdout; `--write` splices it between the
markers in `QUEUE_KIT_ROADMAP_FILE`, leaving every byte outside them untouched.
It reads live task sections only (active plus deferred; never Done — a shipped
item is history, not direction).

Emit grammar, which is a contract the freshness gate byte-compares and readers
consume:

- One `### <horizon>` heading per configured horizon, in configured order, each
  emitted even when empty (an empty horizon is information: nothing is queued
  there).
- Under each, one bullet per tagged entry, in queue order:
  ``- **`<slug>`** *(<track>)* — <summary>``.
- The summary is the entry's `roadmap-summary:` declaration, verbatim
  (delta 14). *Superseded:* this clause read "the entry's lead-line text with
  every tag stripped, truncated at the first sentence boundary"; delta 14
  records why that rule was replaced and what replaced it.

Ordering inside a horizon is queue order, not a rank — the page states so in its
framing rather than implying a priority the queue does not carry.

The **three-to-five items per horizon** band is editorial posture, stated in the
page's framing prose and in this amendment, and deliberately **not gated**. A
horizon legitimately holding two items is a true state of the queue, and
reddening a commit over it would buy a curation opinion at the cost of the
low-false-positive contract every gate in the battery is held to.

### 4. `check-roadmap-fresh` — *design-bearing*

A new queue-kit gate, `precommit` tier, two assertions:

- **(A) Freshness.** The committed marker block in `QUEUE_KIT_ROADMAP_FILE`
  byte-matches `bin/roadmap.sh --emit`, on the `check-footprint-fresh` /
  `check-value-rollup-fresh` model: diff the emit against the block, name the
  regen command in the red output. Missing file, missing markers, or an
  unbalanced marker pair is fail-closed (exit 2) — an absent projection under a
  configured path is a broken install, not a clean skip.
- **(B) Tag-field validity.** Every `[roadmap:]` tag in the live task sections
  parses as `<horizon>/<track>` with both fields members of their configured
  arrays. An unknown horizon or track, a missing slash, or a second `[roadmap:]`
  tag on one entry is a violation naming the entry and the offending field.

Assertion B lives here rather than in `check-tag-lead-line` because this is the
gate that already loads the horizon and track vocabularies; putting it in the
lead-line gate would make a placement gate load a value roster it has no other
use for. `check-tag-lead-line` still gains the tag on its *placement* axis
(delta 5).

`QUEUE_KIT_ROADMAP_FILE` empty is a clean skip for both assertions — the correct
no-op for a consumer that publishes no roadmap, matching
`check-queue-slug-liveness`'s empty-globs behavior.

Ships with a `good/`+`bad/` fixture pair per gate-sdk's fixture-pair contract
and a `# graph:` manifest coupling the gate to the queue file, the config, and
the projection page.

The fixture pair needs an interface, and the cited models already carry it:
both `check-footprint-fresh` and `check-value-rollup-fresh` take
`[projection-file] [emit-file]`, and their fixtures are a pre-baked
`projection.txt` + `emit.txt` behind an `args` file. Assertion A takes the same
two-arg form for the same reason — a freshness gate offering only its bare form
has no fixture that does not read the live queue, so the pair's verdict would
turn on tree state outside the fixture directory. Verified at align against
`context-kit/gate-tests/check-footprint-fresh/`.

### 5. `check-tag-lead-line` gains the tag — *mechanical*

One `arr["roadmap"] = 1` row on `/\[roadmap:/` in the gate's `classes()`
function, so a `[roadmap:]` tag pushed onto a continuation line is caught like
every other tag. The gate's `# spec:` header line gains the tag name.

### 6. `README.md`'s queue-kit row — *mechanical, but load-bearing*

`scripts/enum-sets.sh` derives this repo's `queue-task-tag` set by grepping the
`arr["<tag>"]` rows out of `check-tag-lead-line.sh`. So delta 5 grows the set,
and `check-prose-enum` then reds every manifest paragraph hand-listing two or
more task tags without the new member.

**The propagation and its blast radius were both verified at align**, by running
the real gate against a simulated post-delta-5 set rather than reading the two
scripts and inferring. The mechanism holds — `arr["roadmap"]` matches the
emitter's ERE, and the derived set grows from four members to five. The sweep
the align audit was asked to run returns **four** red sites, not the one this
delta originally named:

- `README.md`'s queue-kit table row, which enumerates the algebra
  (`blocked-by/needs-spec/spec/drain-exempt/precondition-ok`).
- `queue-kit/README.md`'s opening tag-algebra sentence — a **separate** update
  target from the gate/tool roster row deltas 3 and 4 own, on a different block
  of the same file.
- `queue-kit/SPEC.md` §check-tag-lead-line's governed-set list, already owned by
  delta 5.
- `docs/queue-kit/index.md`, the docs-site kit page.

Three hand edits, then, not one; the fourth rides delta 5.

**Correction, made at build against the generator rather than inferred.** The
fourth site was filed as "the generated mirror of `queue-kit/README.md`, carried
by delta 13's `gen-docs-mirror.sh --write`, never hand-edited". It is not
generated: `scripts/gen-docs-mirror.sh`'s `sources()` emits
`docs/<kit>/SPEC.md` and `docs/<kit>/README.md` only, so the mirror of the kit
README is `docs/queue-kit/README.md`. `docs/queue-kit/index.md` is a
hand-authored docs-site page (nav front matter, no `generated:` key) and takes a
hand edit like the other two. The count of four red sites was right; the
disposition of the fourth was wrong.

### 7. `ROADMAP.md` framing prose — *design-bearing*

A new root file: hand-authored framing around one generated marker block, the
`docs/value.md` shape rather than the whole-file generation of
`docs/footprint.md`, because the page needs prose the queue cannot derive —
what the horizons mean, that the deferred rungs are demand-gated rather than
promised, that ordering within a horizon is queue order and not a rank, and
where to send the signal that moves an item (the issue forms and Discussions).

This is positioning-adjacent prose. It consumes `docs/positioning.md`'s
vocabulary rather than forking it, on the same rule the
`front-door-outcome-rewrite` entry states for the front door.

### 8. Root registrations for `ROADMAP.md` — *mechanical*

`scripts/root-allowlist.list` (a new root surface is a deliberate edit),
`scripts/core-files.list` (a public front-door surface is pinned like the other
repo-meta), `scripts/canon-config.sh`'s `CANON_KIT_MANIFEST_FILES` (so its links
and commands resolve under the doc gates), and `scripts/gates.list`
(`check-roadmap-fresh`).

`ROADMAP.md` deliberately does **not** join `QUEUE_KIT_PROSE_SURFACE_GLOBS`.
Assertion A already makes a dangling slug impossible on a generated page, and
the enforcement-first rule ranks removing a duplicate check above adding one.

### 9. The curation pass — *design-bearing*

Applying `[roadmap:]` to the chosen entries in `TASK-QUEUE.md`. Which rungs
belong on a public page, under which horizon, and under which track is editorial
judgment against the demand-gating posture the deferred entries already carry —
not a mechanical sweep. The wrap budget bites here: a tagged lead line has about
21 columns left for its opening words, so several lead lines need rewrapping in
the same pass.

### 10. Three issue forms — *design-bearing*

`.github/ISSUE_TEMPLATE/install-failure.yml`, `doc-problem.yml`, and
`adoption-report.yml`, beside the existing fixture-first `gate-defect.yml`.
Design-bearing because the fields are the whole deliverable: an install-failure
form that does not elicit the platform, the transport, and the failing step
produces an issue nobody can act on, and the existing `gate-defect.yml` sets the
bar — it routes reproducible defects to a pull request in its own preamble
rather than accepting them as issues.

Each form carries the same provenance-seam warning `gate-defect.yml` already
does: report generic mechanism, never your own private rule content.

Every field must be one a triager acts on. A field nobody reads is dropped
before the form ships — the causal-completeness rule applied to a form rather
than a message.

`config.yml` needs no change: `blank_issues_enabled: false` and the Discussions
contact link are already the routing this completes.

### 11. `CONTRIBUTING.md` routing prose — *mechanical*

The guide currently routes everything that is not a reproducible gate defect to
Discussions. With three inlets open, it names them and says which goes where.

### 12. Front-door links — *mechanical*

`README.md` and `docs/index.md` link the roadmap. `README.md` links it
relatively; `docs/index.md` takes the self-repo blob grammar the generated docs
pages already use for a repo source, because the site is served from `docs/` as
its root and a relative `../ROADMAP.md` resolves on disk — leaving `check-md-refs`
green — while 404ing for the reader who follows it. Verified at align: the docs
home currently links nothing outside `docs/`, so this is its first such link.
**Sequencing note:** the
`front-door-outcome-rewrite` debt unit promoted this same iteration rewrites
both first screens; land that unit's rewrite first and add these links into the
rewritten prose, rather than adding links a rewrite then relocates.

### 13. Regeneration fan-out — *mechanical*

Every generated projection this touches, each with its own freshness gate:
`bash gate-sdk/bin/gen-pre-commit.sh --write` (new gate, new `# graph:`
manifest), `bash gate-sdk/checks/check-graph.sh --emit > docs/check-graph.html`,
`bash gate-sdk/bin/enforcement-map.sh --emit > docs/enforcement.md` (a new gate
joins the class registry), `bash scripts/gen-docs-mirror.sh --write` (queue-kit
SPEC and README both change), `bash context-kit/bin/footprint.sh --emit >
docs/footprint.md` (new files carry token cost), and
`bash scripts/gen-value-rollup.sh` (its per-kit counts derive from the
enforcement map). This is the KPI-roster-style fan-out
docs/site-architecture.md §Generated projections describes; it is listed here
because a build that regenerates four of the six discovers the other two as red
gates.

`ROADMAP.md` itself is **not** on that page's roster: site-architecture.md owns
docs-site projections, and a generated file at the repo root is CLAUDE.md
§Housekeeping's tier. One line there, pointing at queue-kit/SPEC.md
§check-roadmap-fresh for the mechanism.

### 14. The `roadmap-summary:` declaration — *design-bearing, envelope amendment*

**Operator ruling, 2026-07-28, widening the envelope delta 3 set.** The
lead-line-only summary rule shipped and was measured against the real queue: it
yields 20-35 character fragments, and an entry carrying a `[spec: <file>]`
pointer reaches 77 columns before any prose, so both active `New Features`
entries could not carry the tag at all and `now` shipped one item. The rule needs
replacing. The **replacement was ruled, and the rejected alternative matters more
than the accepted one.**

*Rejected:* widening the projection to the entry's first sentence while excluding
the rationale and cost blocks. That is a **blocklist, and a blocklist fails
open** — a cost note or a premise correction authored as an ordinary opening
sentence projects to a public page anyway. It contradicts this amendment's own
premise that entry bodies carry exactly what a public page must not inherit, and
while it is harmless for a repo whose queue is already public, the kit ships to
consumers whose queue is private and whose roadmap is not. The seam here is a
**privacy boundary before it is a design one**, and a mechanism whose failure
mode is "publishes internal prose" is the wrong default to ship.

*Accepted:* the projection may read beyond the lead line, but **only text the
author explicitly marked projectable** — a whitelist. Unmarked prose never
projects, so the failure mode is a thin page, never a leak.

**The mechanism.** A `roadmap-summary: <text>` **declaration line**, indented
inside the entry body like any continuation line, carrying the one sentence the
public page prints. The emitter reads that text verbatim; there is no sentence
heuristic left anywhere in the projection, because the author already decided
where the summary ends.

It is a **declaration, not a tag**, and the distinction is load-bearing rather
than cosmetic. Every member of the tag algebra is bracketed and lead-line-scoped,
and `check-tag-lead-line` exists to keep it that way; a bracketed marker living
on a continuation line by design would contradict the one rule that gate teaches.
So this takes the bare `<token>: <value>` shape the tree already uses for a
declaration a scanner reads off a line of its own — the `close-surface:`
declaration in queue-kit/SPEC.md §The tag algebra is the precedent, and
`QUEUE_KIT_PROSE_LEADS`' `Protocol:` token is the same shape at column 0.
`roadmap-summary:` also cannot collide with the `[roadmap:` tag scan, which keys
on the bracket.

Fixed spelling, kit mechanism: the seam ruling above puts the tag's spelling and
grammar on the kit side, and this declaration is the same class of thing. No new
consumer config.

**Why one line, not a marked block.** An indented declaration line has the whole
wrap budget behind it — about 81 columns after the indent and the token, roughly
triple what the lead line could ever spare, and comfortably a full sentence. A
multi-line form would need a block-end rule (deeper indent, a blank line, a
closing token), and every candidate either invites a reflow to silently truncate
the summary or adds a second grammar for the wrap gate to police. One line needs
no end rule at all. The cost is honest and bounded: an author who wants two
sentences on the public page cannot have them, which is the right constraint for
a bullet on a scannable roadmap.

**The wrap budget still applies**, unchanged — `check-queue-wrap` reads every
line in the file, and a declaration line is a line. This does not widen the
budget for anyone; it moves the summary off the one line that was already full.

**Delta 4 gains assertion C**, in both directions, because a whitelist with no
parity check degrades silently in the direction that matters:

- A `[roadmap:]`-tagged entry carrying **no** `roadmap-summary:` declaration, or
  more than one, is a violation naming the entry. Without this, a curated entry
  would project as a bullet with an empty summary — the thin-page failure mode
  reaching a reader instead of the author.
- A `roadmap-summary:` declaration on an entry whose lead line carries **no**
  `[roadmap:]` tag is a violation too: a dead marking is how a dropped or
  reflowed tag looks from the page's side, and the tag is what decides
  projection.

Both halves are mechanical over the shared parse, so the low-false-positive
contract holds. The parse itself stays single-sourced — `queue_roadmap_entries`
grows the declaration count and text, so the emitter and the gate still cannot
disagree about what an entry claims.

**Delta 9's curation is re-run under this grammar**: each curated entry gains a
declaration authored for a public reader rather than inherited from internal
prose, and the two `[spec:]`-tagged active features become projectable — their
lead line now needs room for the tag alone, not for the tag plus a summary.

## Producers and consumers

**The `[roadmap: <horizon>/<track>]` tag.**
*Producer:* a scope or close session curating the queue writes the tag on an
entry's lead line. Enabling config is `QUEUE_KIT_HORIZONS` + `QUEUE_KIT_TRACKS`
in `scripts/queue-config.sh` — set by this repo in a live tracked configuration,
not only in fixtures.
*Consumers:* (a) `bin/roadmap.sh`, through `lib/queue.sh`'s lead-line tag
extractor, at the emit walk; (b) `check-roadmap-fresh` assertion B, at the
tag-validity scan; (c) `check-tag-lead-line`'s `classes()`, at the placement
scan.
*Field readers:* `<horizon>` is read by `roadmap.sh` to select the item's
`###` section and by assertion B for membership; `<track>` is read by
`roadmap.sh` to print the item's `*(<track>)*` label and by assertion B for
membership. Both fields have a reader at a named transition; neither is
write-only.

**The generated marker block.**
*Producer:* `bash queue-kit/bin/roadmap.sh --write`, run by a maintainer after a
curation change; the red output of assertion A names it, so recovery does not
depend on the command staying resident.
*Consumers:* `check-roadmap-fresh` assertion A byte-compares it at commit time;
a reader of the rendered `ROADMAP.md` consumes it as the page's body.
*Field readers:* each emitted `### <horizon>` is read as the page's section
structure; each bullet's slug, track, and summary are read by the human reader.
Nothing is emitted that no reader consumes — the deliberate omission is the
entry body, which has no public reader and is therefore not projected.

**The `roadmap-summary: <text>` declaration** (delta 14).
*Producer:* the same scope or close session that writes the `[roadmap:]` tag
authors the declaration in the entry body, in the same edit — assertion C makes
the pair mandatory, so the tag cannot land without it.
*Consumers:* (a) `bin/roadmap.sh`, through `lib/queue.sh`'s shared
`queue_roadmap_entries` parse, at the emit walk, where the text becomes the
bullet's summary verbatim; (b) `check-roadmap-fresh` assertion C, which reads the
declaration *count* per entry and pairs it against the tag's presence in both
directions; (c) a reader of the rendered page, for whom this text is the only
prose the entry contributes.
*Field readers:* `<text>` is read by `roadmap.sh` at the emit walk and by the
human reader of the page; its presence-and-count is read by assertion C. The
declaration has no other reader by construction, and that is the point — it is
the whole of what the entry publishes, so nothing else in the body needs a
publication decision.

**The four config knobs.**
*Producer:* the consumer's `queue-config.sh`, sourced by `lib/queue.sh`'s
loader. *Consumers:* `roadmap.sh` (all four), `check-roadmap-fresh` (all four).
`QUEUE_KIT_ROADMAP_MARKER` is read by both the splice in `--write` and the block
locator in assertion A — the one knob with two readers, and they must agree,
which is why it resolves through the shared loader rather than being
re-defaulted in each.

**The issue forms.**
*Producer:* a GitHub user opening an issue through a form; the enabling
configuration is the form file present on `master` plus
`config.yml`'s `blank_issues_enabled: false`, both already live.
*Consumer:* the maintainer's triage, which is a standing close-stage duty over
this repo's inbound surfaces. Each form's `labels:` key is read by GitHub at
issue creation and by the triager filtering; each body field is read by the
triager deciding whether the report is actionable. A field with no triage reader
does not ship.

## Existing sections updated

- **queue-kit/SPEC.md §The tag algebra** — the `[roadmap:]` entry, its
  fixed-spelling/configured-values split (the same split `[needs-spec]` and
  `[spec:]` already carry), and the one-tag-two-fields ruling. Owned by delta 1.
- **queue-kit/SPEC.md §Layout and configuration** — the four knobs with their
  defaults and the half-configured-vocabulary fail-closed rule. Owned by
  delta 2.
- **queue-kit/SPEC.md §lib/queue.sh** — the loader's new validation of the two
  vocabulary arrays. Owned by delta 2.
- **queue-kit/SPEC.md §Per-component contracts** — two new sections,
  `bin/roadmap.sh` (delta 3) and `check-roadmap-fresh` (delta 4), the second
  stating the ungated count band and why. Both are amended by delta 14: the emit
  grammar's summary rule and the gate's assertion C.
- **queue-kit/SPEC.md §The tag algebra** — a second target: the
  `roadmap-summary:` declaration beside the `close-surface:` precedent, stated as
  a declaration rather than a tag. Owned by delta 14.
- **queue-kit/SPEC.md §lib/queue.sh** — a second target: `queue_roadmap_entries`
  grows the declaration count and text. Owned by delta 14.
- **queue-kit/SPEC.md §check-tag-lead-line** — the governed-tag list in its
  invariant statement. Owned by delta 5.
- **queue-kit/README.md** — two separate blocks: the gate/tool roster row, held
  by `check-readme-roster` (owned by deltas 3 and 4), and the opening
  tag-algebra sentence, which `check-prose-enum` reds once the set grows (owned
  by delta 6). Added at align, which found the second uncovered.
- **README.md** — the queue-kit table row's tag enumeration (delta 6) and the
  roadmap link (delta 12).
- **CLAUDE.md §Housekeeping** — one line placing `ROADMAP.md` as a generated
  root projection, pointing at queue-kit/SPEC.md §check-roadmap-fresh. Owned by
  delta 13.
- **CONTRIBUTING.md** — the issue-routing paragraph. Owned by delta 11.
- **docs/index.md** — the roadmap link. Owned by delta 12.

No update target here is unclaimed: every section above names the delta that
owns it.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
