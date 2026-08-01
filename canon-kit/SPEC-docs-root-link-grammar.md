# SPEC amendment: docs-root-link-grammar

A hand-authored docs-site page that links a target *outside* the site root with a
bare relative link resolves on disk and 404s for every reader. `check-md-refs`
resolves the target, finds it tracked, and stays green; `jekyll-relative-links`
cannot rewrite it, because the target is outside the Jekyll source and so has no
built URL to rewrite to — confirmed against `docs/_config.yml`, which enables that
plugin, so the defect stands under the actually-configured renderer rather than a
hypothetical one.

**This amendment invents no doctrine. It gates doctrine this kit already
states.** canon-kit/SPEC.md §The reference-link grammar already rules the case
verbatim: a source reference *"cites the tree with an absolute GitHub blob link …
because a relative link into the unrendered surrounding tree would 404 on a site
that serves `docs/` alone."* The prescription exists, the correct form is
established and hand-followed elsewhere, and nothing checks it. That is the whole
gap.

## The placement call, which the queue entry framed with one option missing

The entry posed the open call as *"a new gate, or an assertion inside
`check-md-refs`"*, and worried correctly that folding it into `check-md-refs`
*"gives a disk-resolution gate a site-topology opinion"*.

**Neither. It is a third rule on `check-docs-link-convention`** — and that option
was absent from the entry because the unit was scoped as a site-kit surface, which
it is not: site-kit owns two gates (`check-docs-cname-parity`,
`check-docs-render-fidelity`) and no docs-link gate at all.

`check-docs-link-convention` is already the right gate on every axis. It is
already scoped to the docs root by `CANON_KIT_LINK_ROOT`. Its SPEC already draws
exactly the seam the entry was reaching for — *"the resolution of those links is
`check-md-refs`' charge, and this gate owns shape alone"* — and *outside the site
root* is a shape claim, decidable without resolving anything a sibling gate has
not already resolved. And it already carries the two rules of the same family
(no directory-target link; anchored kit back-links), so this is that gate's own
sentence finished rather than a new opinion lodged anywhere.

The consequence worth stating: `check-md-refs` is **not modified**. It keeps its
one job and gains no site-topology opinion, which is precisely the outcome the
entry's stated worry asked for.

## What changes

### Delta 1 — a third rule on `check-docs-link-convention` *{design-bearing}*

**No off-root relative link.** A relative markdown link on a docs-root page whose
target resolves *outside* `CANON_KIT_LINK_ROOT` is a violation; the citation must
use the absolute self-repo reference form §The reference-link grammar specifies.

The boundary predicate is where this unit's `[design-pending]` lived, so each edge
is ruled rather than left to the implementer:

- **Links that stay inside the root are silent.** The rule turns on the *resolved*
  path, not on the link text — a `../`-prefixed link is perfectly correct when it
  resolves back under the root, which is the common case across the mirror's
  cross-references. Keying on `../` instead would red the majority of correct
  links in the corpus.
- **Anchors and absolute URLs never fire.** A pure `#anchor` has no path to
  resolve. `scheme://` and `mailto:` targets are already outside this gate's
  scope by its existing sentence, and the blob form is itself a `scheme://` link,
  so a correctly-cited off-root target cannot red the rule that demands it.
- **Only resolving targets are classified**, and this is the anti-double-report
  rule. A relative target that resolves to nothing is `check-md-refs`' finding and
  must stay only its finding; this rule fires solely where the target resolves to
  an **existing** path that happens to sit outside the root. The two gates therefore
  partition the failure space rather than overlapping it — one red per defect.
  *Existence, not tracked-ness*: the gate already performs both halves of this read
  for its directory-target rule — a pure-path `realpath -m` resolution against the
  source file's directory, then a filesystem test on the result — so the rule needs
  no capability the gate lacks. Keying on tracked-ness instead would add a git query
  to a gate that makes none, and would buy nothing, because the partition being drawn
  is between targets that resolve and targets that do not — exactly what the
  existence test already decides.
- **Directory targets stay the first rule's finding.** A relative link naming a
  directory outside the root satisfies the no-directory-target rule's predicate
  first; it is reported there, once, and this rule does not double-report it.
- **Generated pages are in scope, deliberately.** The mirror's pages
  (`generated: true` front matter, emitted by `scripts/gen-docs-mirror.sh`) conform
  by construction — the generator rewrites off-root source targets to the blob
  form and leaves a link relative only when both endpoints are mirrored. Exempting
  them would be the obvious move and it is **rejected**: their conformance is a
  property of the generator, and a generator is a thing that can regress. In scope,
  the rule is a standing regression test on the rewrite for free. Exempting them
  would buy nothing and would blind the corpus's larger half.
- **The existing valve is reused, not duplicated.** The gate's
  `docs-link-exempt: <reason>` comment suppresses this rule as it does the other
  two. No second valve, no rule-specific escape.

Fixtures: the existing `good/`+`bad/` pair grows a case per direction — a
`docs/` page citing an off-root target in blob form (good) and the same page citing
it relatively (bad) — so the rule's two states are proven and not merely the
failure.

### Delta 2 — the sweep, in the same unit *{mechanical}*

Enforcement-first: the rule and the violations it names land together. The census
across all 69 markdown files under `docs/` — 23 generated mirror pages, 46
hand-authored — found **exactly two violations, both in one file**:

- `docs/orchestration.md`:90 — `[the lead template](../lifecycle-kit/templates/lead.md)`
- `docs/orchestration.md`:108 — the same target, second occurrence

Both become the self-repo blob form, matching the hand-followed precedent at
`docs/index.md`:84, which already reaches the off-root `ROADMAP.md` that way.

Two lines is a small sweep and the amendment says so plainly rather than inflating
it. **The gate is the deliverable; the sweep is its receipt.** Today's
near-zero-violation state across the other 68 files is incidental rather than
enforced — every hand-authored page added meanwhile can add another silently, and
the two that exist stood on the served site through several closes precisely
because nothing looked.

### Delta 3 — the two SPEC sections that must move with it *{mechanical}*

`canon-kit/SPEC.md` §check-docs-link-convention gains the third rule in the
existing bulleted form. §The reference-link grammar gains one clause noting that
its off-root prescription is now held by that gate — the prescription already
reads as a rule, and a reader currently has no way to learn it is unenforced.
Both are transcription of Delta 1's ruling.

## Producers and consumers

This amendment introduces **no new state, no new file, no new knob, and no new
field.** It adds one assertion to an existing gate over surfaces that already
exist. The causal-completeness pass is correspondingly short, and short because
the design is small rather than because the survey was.

**The off-root rule** (new assertion, Delta 1).
*Producer:* `canon-kit/checks/check-docs-link-convention.sh`, run by
`gate-sdk/bin/run-gates.sh` and by the generated pre-commit hook — the gate is
already registered in this repo's `scripts/gates.list` at `tier=precommit`, so the
producer's enabling configuration is already set in every clone and no new
registration is owed. A consumer that vendored canon-kit and registered the gate
gets the rule on re-vendor, with no config change, because the rule reads
`CANON_KIT_LINK_ROOT` — a knob that already exists and that every registering
consumer already set or defaulted.
*Consumers, both named:* (1) the committing agent or human, who receives the
finding naming the file, line, resolved off-root target, and the blob form to use
instead; (2) `gate-sdk/bin/run-gates.sh`'s battery result, which is what the
`gates` workflow watches on the master push.

*Inputs, each with its existing owner.* The rule reads the docs root
(`CANON_KIT_LINK_ROOT`, canon-kit/SPEC.md §Layout and configuration) and the
relative link's resolution against the source file's directory — the same
resolution `check-md-refs` performs, computed independently here because this gate
must not depend on the other's traversal order. It does **not** read
`CANON_KIT_DOCS_BLOB_REF`: the rule reports that a relative off-root link is
wrong, and the blob form's *validity* — identity derivation and the pinned ref —
stays `check-md-refs`' self-repo pass, unchanged. Naming what this gate declines
to read is the seam that keeps the two gates' claims separable.

**An adjacent gap this survey found, filed rather than fixed.**
`scripts/gen-docs-mirror.sh` emits a `/tree/` form for off-root *directory*
targets, and `check-md-refs`' self-repo pass recognizes only the `/blob/<ref>/`
prefix — so a `/tree/` link falls through to the external-URL skip and is
resolved by nothing. It is out of this unit's boundary (this rule fires on
relative links; a `/tree/` link is absolute and already-converted) and it is not
a live defect, since the generator's output is correct by construction. It is
recorded here and filed to the gap inbox rather than flagged-and-skipped, per the
gap-disposition rule.

## Existing sections updated

- **`canon-kit/SPEC.md` §check-docs-link-convention** — gains the third rule
  (Delta 1/3), stated in the same bulleted form as the two it joins, with the
  resolving-targets-only clause that partitions its scope from `check-md-refs`.
  The section's existing seam sentence — *"the resolution of those links is
  `check-md-refs`' charge, and this gate owns shape alone"* — stays true and is not
  weakened: off-root-ness is read off a resolution this gate performs for shape,
  and the target's *validity* remains the sibling's.
- **`canon-kit/SPEC.md` §The reference-link grammar** — gains the clause naming
  the gate that now holds its off-root prescription (Delta 3).
- **`docs/orchestration.md`** — the two links (Delta 2).
- **`canon-kit/gate-tests/check-docs-link-convention/{good,bad}/`** — the fixture
  pair grows the two cases (Delta 1).
- **No `check-md-refs` change**, and no gate registration change — the gate is
  already registered, so no generated projection stales from this unit. Stated
  because a new-gate unit normally would stale the hook and the graph, and a build
  session should not go looking for a regen this unit does not owe.

## The seam

Ruled explicitly, per CLAUDE.md §The provenance seam.

**This lands as kit mechanism, and it is clean to do so.** The rule carries no
project-specific content: it reads `CANON_KIT_LINK_ROOT` for the site root — a
knob that already exists at default `docs` — and it names no repository, owner,
host, branch, or page. The blob form it prescribes is derived at gate runtime from
`git remote get-url origin` through gate-sdk's `gate_self_repo_prefix` adapter, so
the kit ships no repo identity and the seam holds exactly as §The reference-link
grammar already documents. **No new knob is introduced**, which is the strongest
form of the config-via-env convention being satisfied: the generic knob the rule
needs was already there.

The rule is also true for any consumer, not only this one. Any project serving a
subdirectory as its Pages root has this defect available to it, and the two
existing rules on the same gate are generalizations of the same kind. Nothing here
is this repo's posture wearing a kit's name.

The one repo-specific artifact is Delta 2's sweep of `docs/orchestration.md`,
which is consumer content and stays consumer content — it is the fixture-of-record
for the rule, not part of what vendors.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      canon-kit (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles. (This amendment retires no name — the check is run and its
      empty result recorded, not skipped on the assumption.)
- [ ] **Gaps filed** — the `/tree/`-form resolution gap named in §Producers and
      consumers is filed to the gap inbox
      (`bash lifecycle-kit/bin/file-gap.sh …`); any further cross-component gap
      discovered during the work is resolved that session or filed.
