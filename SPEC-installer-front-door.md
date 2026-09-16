# SPEC amendment: installer front door

Pairs the queue entry `installer-readme-usage-tier-split`. `installer/README.md`
is at once the npm-visible package README and a 3,201-line, 216,970-byte design
record, so the package page and the tarball lead with mechanism grounds where
usage belongs. The design record becomes the installer's own SPEC and the README
is re-authored as the activation surface.

Sited at the repository root: the sweep the split costs spans `native/`,
`gate-sdk/`, `installer/`, `docs/`, `.github/`, `.workflow/` and the root
manifests, so no single component owns it.

**The seam this unit crosses, ruled.** Nothing here is kit mechanism: every
surface it touches — `installer/`, the package roster, this repository's own
docs and layout pointers — is repo-root-governed consumer content, and no kit
SPEC, knob, gate or template acquires a literal from it. The one place the seam
is *tested* is delta 5: naming a non-kit component's record `SPEC.md` puts it in
the governed spec glob, and the ruling there is that the glob's reach is a
mechanical fact while the provenance seam's subject stays a **kit** SPEC — so the
record keeps the dated operator stamps it legitimately carries. No consumer
config is minted and no knob is added.

**Ordering.** This amendment and `SPEC-payload-withholding.md` edit the same
passages — that amendment rewrites five sections this one relocates. Landing this
one **first** costs one uniform sweep; landing it second costs authoring those
passages twice. The dependency is stated here and on that amendment, and it is
the only coupling between them.

## What changes

### (1) The design record becomes `installer/SPEC.md`, section for section

Every `##` section of today's `installer/README.md` moves to a new
`installer/SPEC.md` keeping its heading verbatim, with one exception named in
delta 2. {mechanical}

Verbatim and whole is the ruling, not an economy. A carve that decided
section-by-section which prose is *usage* would make the citation sweep of delta
3 a judgment at every one of its sites; a whole move makes it one token swap,
which is what lets the sweep be delegated and re-run. The design record is not
edited by this amendment — it is relocated. Any edit it wants is a different
unit's, and `SPEC-payload-withholding.md`'s deltas are that unit.

The file is **not** added to `installer/package.json`'s `files` roster, which is
`bin/`, `payload/`, `profiles.list`, `README.md` and stays so. §Layout already
rules that the roster and not the directory listing decides what is reachable at
an installed `PKG_ROOT`, so the design record ships in neither the npm tarball
nor the Release tarball without a further edit — the same rule the payload
applies to a kit's SPEC, reaching the package one layer out.

### (2) `§What this package is` stays in the README, and the dependency contract gets a SPEC home

`## What this package is` is the one heading the README keeps, and the
no-dependency contract it currently also carries moves to a new
`installer/SPEC.md §The dependency boundary`. {design-bearing}

It stays because it is genuinely the usage tier: 931 bytes stating what an
adopter is about to install and what governs their tree afterwards, which is the
first thing a package-page reader needs and the last thing to hide behind a
pointer. It stays under its own name because three surfaces cite it for that
claim, one of them the override ledger, and a heading that survives is a
citation that needs no repair.

The two citers that want the *contract* rather than the claim —
`native/src/gates/installer_no_deps.rs` and `scripts/check-installer-no-deps.gate`
— re-target to §The dependency boundary, which is where the mechanism they
enforce now lives. Those two sites are the whole of delta 3's non-uniformity.

### (3) Every citation into the moved sections is re-targeted in the same commit

`installer/README.md §<section>` becomes `installer/SPEC.md §<section>` at every
site, in the commit that performs the move. {mechanical}

**This is not a `mv`, and the measurement is why.** The corpus is 43 tracked
files carrying roughly 459 citation instances into 16 distinct sections. Probe: a
recursive grep for the literal `installer/README.md` over the tracked tree,
then, for each occurrence, the text immediately following it — through a
backtick or possessive suffix and across a line wrap — matched against the
file's own `##` heading list. The queue entry's *seventeen files into fourteen
sections* is superseded by that probe and is not a differently-scoped count; the
corpus grew.

`check-spec-pointer` is the reason the sweep is same-commit rather than
follow-on: it holds that every `spec:`/`contract:` directive and every free-prose
`<path>.md §<heading>` citation resolves to a tracked file carrying that heading,
so a move that left the citations behind reds at roughly 459 sites at once.

Three classes inside the corpus take different handling, and each is named so the
sweep is uniform within a class rather than uniform by assumption:

- **§-bearing citations** — the bulk. One token swap. The five
  `docs/<kit>/SPEC.md` files in the corpus are generated mirrors and are **not**
  edited: they are regenerated (delta 6).
- **Bare-path mentions**, carrying no `§`. These resolve to whichever file
  exists, so each is judged once by what it means: a mention of the design record
  re-targets, a mention of the package README stays. The probe above separates
  them; `check-spec-pointer` does not, which is why they are called out rather
  than swept.
- **`.github/workflows/*.yml` comments.** Outside `check-comment-tier`'s governed
  surface, so these drift silently instead of redding. They are swept anyway —
  a citation nothing checks is still a citation a reader follows — and the fact
  that no gate holds them is stated so a later reader does not mistake their
  passing for their being checked.

`.workflow/audit-roster.txt` carries §-bearing citations among its rows. Those
are repaired as paths and nothing else: a historical row keeps saying what it
said, at the location the thing it named now occupies. The alternative is a
permanently red gate over an append-only record, which is not an alternative.

### (4) The README is re-authored as the activation surface

The new `installer/README.md` is written fresh — requirements, quick start, the
verbs, profiles, and where the design lives — under headings that collide with
none of the moved ones. {design-bearing}

**The usage tier's owner is the README, and this is the ruling the entry left
open.** The discriminator is which surface is true for every reader of the usage
text. Three read it: the npm package page, the extracted tarball, and the site.
The README is present for all three; the site is present for one. Widest-true-tier
placement therefore names the README as owner, and the objection it has to answer
— content-tiering's *point, never restate* — is answered by where the site
already stands: `docs/install.md` §Managing an install **already** defers to the
installer's own record by name rather than restating it, and §Requirements on the
two surfaces are different subjects under one word (the consumer's platform and
toolchain floor there, the delivery path's tool requirement here). What is left
overlapping is the quick start, and it overlaps a page a reader at the npm front
door does not have.

**The alternative is recorded with its grounds rather than left to be
re-proposed.** Pointing the README at the site for quick start costs the
evaluator a click at precisely the moment time-to-first-value is measured, on the
one surface that reaches a reader who has not yet decided to install. A front
door is not a secondary tier, and treating it as one spends the objective the
split exists to serve.

**No generated projection is minted for the overlap.** A marker block projecting
the site's quick start into the README would buy a freshness gate and a roster
row to hold perhaps thirty lines whose two copies address different audiences at
different lengths. The overlap is named here so the next reader finds it costed
rather than unnoticed, and the standing check on it is the gate that already
holds install claims consistent across surfaces.

The README states where the design lives, in one line, pointing at
`installer/SPEC.md` and at its published mirror.

### (5) `installer/` becomes a governed spec directory, and what that admits is measured

The new file joins the governed spec set by name, and the gates that newly reach
it were run down rather than assumed. {design-bearing}

**Already scanning `installer/README.md` today**, through the bare `*README.md`
glob and therefore unchanged in kind by the split: `check-install-claim`,
`check-knob-citation`, `check-payload-claim`, `check-prose-enum`,
`check-md-refs`, `check-spec-pointer`, `check-docs-cmd`, `check-manifest-temporal`,
`check-spec-fence-balance`, `check-manifest-count`, `check-tracking-claim`. The
content moves out from under them onto the `*SPEC*.md` half, and the short README
stays in their corpus.

**Newly reaching the design record once it is named `SPEC.md`**, with each
member's satisfying value named:

- `check-spec-dod-singleton` — this repo runs it in `at-most-one` mode and the
  record carries no `Definition of Done` heading, so zero is a passing count.
- `check-spec-derivable-section` — its banned headings are the configured
  default set, none of which the record's eighteen headings match, so it has no
  section to inspect.
- `check-spec-embedded-source` — the record carries one fenced block in 216,970
  bytes, far under any density the gate reads.
- `check-value-rollup-fresh` — an edit to the file becomes a freshness trigger
  where it was not one. The rollup's own emitter joins the enforcement map and
  the footprint and reads no SPEC prose, and the footprint's axis is the kit
  roster, which `installer/` does not join; so the trigger fires and finds
  nothing changed.
- `check-surface-duplication` — **unregistered in this tree**, this tree
  declaring no glossary, so it is not reached at all. Named because its `couples=`
  lists `*SPEC*.md` and a reader checking the corpus would otherwise stop at it.

**A bounded non-target, stated so a later session does not re-derive it.** The
provenance-seam rule's subject is a *kit* SPEC, and `installer/` is not a kit —
it has no `checks/` and no `smoke/`, and the repo's own housekeeping says so. The
design record carries dated operator stamps today and keeps them; nothing in the
split makes them a seam violation, and the sweep the seam entry owns does not
acquire this file. Whether a gate later built for that rule draws its corpus by
kit root or by spec glob is that unit's ruling to make, not a debt this one
leaves.

### (6) The site publishes the installer's SPEC as a reference tier

`docs/installer/SPEC.md` and `docs/installer/README.md` appear, the mirror is
regenerated, and the freshness gate's coupling is corrected to the corpus the
emitter actually reads. {design-bearing}

**The mirror takes the new file for free, and that is a measured property of the
emitter rather than a hope.** `--emit docs-mirror`'s source set is every
top-level directory holding a `SPEC.md`, plus that directory's `README.md` where
it has one, plus the doctrine deliverable — a list-and-test over the tree, not a
kit-root derivation, so a directory joins the mirror by acquiring a `SPEC.md`.
`installer/` is skipped today only because it has none. The whole delta on the
publication side is therefore the regeneration.

**The gate's coupling is not free, and that is the defect the delta repairs.**
`check-docs-mirror-fresh` declares its sources as `kit:SPEC.md,kit:README.md`,
which expands per kit root and so under-declares an emitter whose corpus is every
SPEC-holding directory. Its `trigger=` already carries the bare `*/SPEC.md` and
`*/README.md` globs, so the gate fires correctly today and would fire on this
file; what is wrong is the declared coupling, and a coupling that disagrees with
the reader it describes is exactly what the coupling manifest exists to prevent.
The `couples=` widens to the emitter's own corpus.

`docs/installer/` gets **no** `index.md`. The mirror pages are a reference tier
reached from a pointer — from `docs/install.md` §Managing an install, which
already names the installer's record, and from the README's one line — and a
page with no `nav_parent` is not offered in the nav, which is what a reference
tier should be. This is the same disposition `SPEC-payload-withholding.md` delta
11 gives the kit SPEC mirror, arrived at from the other side.

### (7) The layout pointer and the install page follow the file

`CLAUDE.md` §Housekeeping's installer sentence and `docs/install.md`'s references
name `installer/SPEC.md` for the design record and the README for the activation
surface. {mechanical}

## Producers and consumers

**`installer/SPEC.md` (deltas 1 and 5).** Producer: the move itself, one commit,
no generator — the file exists because the record was relocated into it, which is
a producer that runs once and is reachable by inspection thereafter. Consumers,
each named with the mechanism that reaches it: the eleven `*README.md`-keyed
canon-kit gates lose it and the four `*SPEC*.md`-keyed gates of delta 5 gain it;
the docs-mirror emitter gains it through its list-and-test corpus (delta 6); the
lifecycle roster-dir test gains `installer/` as a roster dir, because that test is
a directory holding `LIFECYCLE_KIT_ROSTER_BASENAME` and its default is `SPEC.md`.
**That last consumer is the one a reader will not predict**, so it is named: an
amendment sited in `installer/` will from now on reach a second component through
its own directory, which changes when the audit stage's trigger fires for a unit
sited there. Nothing in this amendment depends on it; it is a consequence of the
name and is stated because a later session meeting it at a stage entry would
otherwise re-derive it.

**The short `installer/README.md` (delta 4).** Producer: authored once, rewritten
by nothing — it carries no marker block and no generated span, which delta 4
rules deliberately. Consumers: the npm package page and the extracted tarball,
both reached through `package.json`'s `files` roster on which `README.md` is
already named; the eleven `*README.md`-keyed gates above; and `docs/install.md`
§Managing an install, which points at the record rather than at the README and so
is unaffected by the README shrinking.

**The citation corpus (delta 3).** Producer: the sweep, once. Consumer:
`check-spec-pointer`, whose red condition is a citation naming a heading its
target file does not carry. **The red condition under this corpus change, named
because the change both narrows and widens:** every site that stops resolving
does so at the same commit, so the gate's verdict is all-or-nothing rather than
gradual, and a partial sweep is not a partially-green tree. The two classes the
gate does not see — bare-path mentions and workflow-YAML comments — are the
classes that fail *silently*, which is why delta 3 enumerates them rather than
trusting the gate to find them.

**Enumerable corpus, each member's satisfying value (delta 1).** The corpus is
the eighteen `##` sections of `installer/README.md`. Probe: `grep -n '^## '` over
the file, cross-checked against a byte accounting whose section ranges plus the
315-byte preamble sum to the file's own 216,970 bytes. Seventeen members take the
same value — moved verbatim to `installer/SPEC.md` under the same heading — and
one, §What this package is, takes delta 2's value. No member lacks one. Two
members are cited by nothing (§Implementation at 1,100 bytes and §Docs at 98) and
move with the rest: uncited is not unwanted, and deciding otherwise is the
section-by-section carve delta 1 refuses.

**Enumerable corpus, each member's satisfying value (delta 5).** The corpus is
the canon-kit gates whose `couples=` reaches a spec or README glob. Probe: a grep
of `couples=` across `canon-kit/checks/*.gate` and `scripts/*.gate`, partitioned
by whether the glob matches `README.md`, `SPEC.md`, or both. Each member's value
is stated in delta 5; the one member with no value under this amendment is
`check-surface-duplication`, which is unregistered here, and that is a narrowing
rather than an unmet obligation.

## Existing sections updated

Each bullet names the delta that owns it. The rosters name the probe that
produced them and are floors the merging session re-derives, not completeness
claims.

- `installer/README.md` — all eighteen sections (deltas 1, 2 and 4). Probe:
  `grep -n '^## '` over the file.
- `installer/package.json` — unchanged, and stated as an update target because
  the `files` roster is what makes delta 1's non-shipping true and a reader will
  look for it here (delta 1).
- `CLAUDE.md` §Housekeeping — the installer's layout pointer (delta 7).
- `docs/install.md` §Managing an install and §Vendoring the kits — the named
  references (delta 7).
- `scripts/check-docs-mirror-fresh.gate` — the `couples=` correction (delta 6).
- `canon-kit/SPEC.md` §The reference-link grammar — the mirror's corpus is every
  SPEC-holding directory rather than every kit root, which the section states as
  the gate's subject (delta 6).
- The 43 files of the citation corpus (delta 3). Probe: as stated in delta 3.
  Named rather than listed here, because the roster is the probe's output and a
  copy of it in this file is a second thing to stale before the sweep runs.
- The on-site SPEC mirror — regenerated, and gaining `docs/installer/SPEC.md` and
  `docs/installer/README.md` (delta 6). Probe: `docs/site-architecture.md`
  §Generated projections and their freshness gates.
- `TASK-QUEUE.md` — the entry `installer-readme-usage-tier-split` carries
  `[spec: SPEC-installer-front-door.md]` and leaves the design-pending set (all
  deltas).

## Retired spellings

- None — this amendment retires no name. `installer/README.md` keeps its path and
  its `files`-roster entry, and every moved section keeps its heading verbatim;
  what changes is which file a heading lives in, which is a relocation and not a
  removal. The composite citation token `installer/README.md §<section>` does
  cease to be valid for sixteen sections, and its survivors are held by
  `check-spec-pointer` re-run over the whole tree rather than by a declaration
  here — a declared spelling would have to be enumerated sixteen times to say
  what one green gate run says once, and the two classes that gate cannot see are
  enumerated in delta 3 instead.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them.
- [ ] **Merged with no information lost** — the relocated record reads as one
      document; the new README reads as one document a reader who never saw the
      old one can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through the gap inbox.
- [ ] **The sweep is complete by its own probe** — the recursive grep for
      `installer/README.md` returns only live uses of the package README, and
      `check-spec-pointer` is green over the whole tree.
- [ ] **The front door is measured, not asserted** — the new README's byte count
      is recorded in the landing commit beside the old one's 216,970.
