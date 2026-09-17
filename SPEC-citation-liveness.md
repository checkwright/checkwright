# SPEC amendment: citation-liveness

Queue entry: `citation-liveness-family-convergence`, halves (A) and (B), with its members
`unqualified-section-citation-liveness`, `link-wrapped-section-citation-liveness`,
`spec-pointer-self-section-citation`, `spec-section-title-collision`,
`qualified-pointer-section-ownership`, `prose-filename-citation-liveness`,
`retired-slug-live-pointer-citation`, `queue-status-parenthetical-liveness` and
`done-slug-ownership-citation-report`, the citation half of `enum-and-citation-parity`.

A root-level amendment, because it spans two components:

- canon-kit owns `check-spec-pointer` (deltas 1 to 4) and `check-docs-cmd` (delta 5).
- queue-kit owns `check-queue-slug-liveness` and the shared queue adapters (deltas 6 to 8).

**What the surveys found.** Three read-only surveys ran at spec over this tree (the survey record's
2026-09-17 spec blocks carry corpus, oracle and witness). The family is one predicate, "a citation
resolves to nothing", read through four windows, and the measurements move where two members land:

- **Unqualified `§Heading` citations.** 2028 carry no adjacent `<path>.md`. Resolving each against
  its own file reds 193; adding the nearest path in the paragraph reds 107. Of the 107, 62 were
  hand-verified and **none is a dangling pointer**. Each is a citation shape the window misses:
  1. a placeholder `§<heading>` in prose about the grammar (18);
  2. a long sentence-shaped heading cited by its lead clause (12);
  3. a README citing its sibling SPEC's heading (11);
  4. a bold paragraph lead-in cited as though it were a heading (9);
  5. a cross-kit heading with no path anywhere in the paragraph (6);
  6. a path named after the heading, in a trailing parenthetical (4);
  7. a path spelled without `.md` (`lifecycle-kit SPEC §templates`, 1).

  So the intended file of an unqualified citation is not syntactic, and an antecedent rule cannot
  be made precise. Delta 1 resolves the form against the whole governed set instead.
- **Link-wrapped citations** (`[x](path.md) §Heading`): 45. 43 resolve when the link target is
  taken as the path. The other 2 link to a blob URL.
- **Title collisions:** one in the governed set, `guard-kit/SPEC.md`'s `## Consumer rules` and
  `### Consumer rules`. Four `spec:` directives and two prose citations name that title, and every
  one binds to the first heading.
- **Section ownership:** a token-owned-elsewhere predicate over 50 qualified citations flagged 15,
  and none was a wrong-section citation. That is the honest "not buildable" the entry allows.
- **Headingless filenames.** 68 `.md` tokens name no tracked file, and 1 is a real defect
  (`docs/evidence.md` in drift-kit/SPEC.md). The rest are harness files like `AGENTS.md`,
  `*.local.md`, illustrative amendment names, example paths and placeholders. **The owner has
  moved since the hub was costed.** check-docs-cmd assertion (C) landed later, and it owns path
  liveness in inline code spans. It admits a never-tracked path on a recorded ground (a hypothetical
  or consumer-side file) and reds a retired one. The filed instance, a deleted amendment's bare
  basename, is retired, and (C) misses it only because its path shape needs two segments. So delta 5
  widens (C)'s shape and adds no predicate to `check-spec-pointer`. `docs/evidence.md` was never
  tracked, so (C)'s ground admits it, and build repoints it as a found instance.
- **Retired queue slugs** as single-backtick tokens in the governed set: 35 hits. 33 are
  `check-spec-pointer`, a slug whose name lives on as a tracked gate file. The other 2 are
  `installer/SPEC.md`'s present-tense "owns" of `init-vendor-staging-argv-overflow`, a slug cleared
  from the queue before that prose was written. No hit was prose already marking the slug as past.
  `## Done` is empty at this rev, and a Done slug is already in the retired set, so the
  done-slug report's subject is a subset of delta 6's.
- **Status parentheticals** after a backticked slug: 3, all in the queue file, all accurate today
  (`(retired)` twice, `(icebox)` once). There are none in governed prose.

## The seam

- **Kit mechanism:** every predicate, window and valve below. None names a project, path or
  vocabulary. The status vocabulary comes from the queue's configured section names.
- **Consumer config:** one new knob, `QUEUE_KIT_CITATION_SURFACE_GLOBS` (delta 6). It ships empty,
  and this repo aliases it to `CANON_KIT_MANIFEST_FILES`.
- **Private rule content:** none in reach. The two retired-slug instances are this repo's prose,
  fixed in place and never cited from a kit SPEC.

## What changes

### (1) An unqualified section citation resolves against the governed set

`check-spec-pointer`'s prose pass also extracts a `§<heading>` citation that has no adjacent
`<path>.md`. That citation resolves when **any** manifest file carries the heading {design-bearing}.
**Not yet applied.**

- **Extraction.** A `§` not already consumed by the adjacent-path form starts a citation, and its
  fragment is the paragraph tail, as today. A fragment that does not open with a letter, a digit or
  a backtick is not a citation: `§<heading>` is a placeholder in prose about the grammar, and a `§`
  followed by space or punctuation names the mark itself.
- **Link-wrapped form.** A `§` whose only gap after a markdown link `[..](<target>)` is whitespace
  takes the link target as its path. That path is resolved from the citing file's directory the way
  `check-md-refs` resolves it, then the `#anchor` is stripped. It then resolves strictly against
  that file, like the adjacent form. A target that is not a tracked file (a URL, an untracked path)
  drops to the unqualified rule.
- **Resolution set.** The unqualified form resolves against the union of headings across the
  manifest set, the citing file included. This one rule takes in the self-citation, the README
  citing its sibling SPEC, the cross-kit heading with no antecedent and the trailing-parenthetical
  path. The pass states its predicate as **liveness, never provenance**: the heading exists
  somewhere in the governed set. A citation aimed at the wrong file that carries a live title is
  delta 4's residue.
- **Honest limit.** A short heading such as `## Use` is a prefix of many fragments, so the union
  under-reds, and never over-reds, around short titles.
- **Red condition.** The fragment is a boundary prefix of no heading's text, of no qualifier-stripped
  heading and of no **lead clause** (delta 2) in any manifest file. The finding names the citing
  file and line and `§<fragment, cut to 50 characters>`, like today's prose finding, and says
  "in no governed file".
- **The landing sweep.** The survey's class 4 (bold lead-ins cited as headings, 9) and class 7
  (the path with no `.md`, 1) red under this delta. About 45 unverified sites may add to them. Each
  is fixed at landing by citing the enclosing heading or qualifying the path. Build records the
  count it swept in the commit.

**Replacement text, canon-kit/SPEC.md §check-spec-pointer** (**Not yet applied**). The
prose-citation paragraph's sentence "A cited path that is not a tracked file is out of this pass's
scope …" and the carve-out sentence "a bare `§` with no tracked path before it (the deliberate
non-citation use) never fires" are replaced by:

> The pass reads three citation forms. A `<path>.md §` citation, and a markdown link followed by
> `§` (the link target is the path), resolve against that file, and a path that is not a tracked
> file is out of scope. A `§` with no path resolves against the headings of the whole manifest
> set. Its intended file is not syntactic, because prose puts the owning path before the heading,
> after it or nowhere. So this form asserts liveness and not provenance. A fragment that does not
> open with a letter, a digit or a backtick is a placeholder or the mark itself, and never fires.

### (2) A heading's lead clause is a citable name

Prefix resolution in both prose forms also matches a heading's **lead clause**: its text up to the
first `, `, ` — ` or `: `, when that clause is shorter than the heading {mechanical}.
**Not yet applied.**

- A sentence-shaped heading such as `The probe is asymmetric, and no reading may treat it otherwise`
  is cited by its lead clause, and the fragment runs on into prose, so neither whole-heading test
  can pass. The directive pass stays exact.
- The clause is computed once per heading in `HeadingCache` beside the qualifier-stripped text.

**Replacement text, canon-kit/SPEC.md §check-spec-pointer** (**Not yet applied**). The sentence
"Heading match tolerates a trailing `(qualifier)` on either side" is followed by: "and a prose
citation may name a heading by its lead clause, the text before its first comma, em dash or colon".

### (3) One file's headings carry distinct titles

`check-spec-pointer` reds a manifest file that carries two headings whose qualifier-stripped text
is equal, at any levels {design-bearing}.
**Not yet applied.**

- **Why a red and not a resolver rule.** Every section resolver takes the first match, so a pointer
  meaning the second one resolves elsewhere from the day it is written. No resolution rule can say
  which one an author meant, so the ambiguity is removed at the target.
- **Corpus.** The manifest set, which the prose pass already walks. A directive target outside it is
  unchanged. The finding names the file, both line numbers and the title.
- **The landing fix.** `guard-kit/SPEC.md`'s `### Consumer rules` (the ordering disciplines) is
  renamed. Build re-reads each of the four `spec:` directives in `scripts/bash-guard.sh` and
  `guard-kit/templates/bash-guard.sh`, the two `no-port:` mentions in
  `guard-kit/templates/bash-guard.sh` and `guard-kit/lib/guard.sh`, and the two prose citations in
  `guard-kit/SPEC.md`. Each is repointed at whichever section its sentence means.

**Replacement text, canon-kit/SPEC.md §check-spec-pointer** (**Not yet applied**). A paragraph after
the heading-match paragraph:

> **A title names one section per file.** Two headings in one manifest file whose qualifier-stripped
> text is equal are red. A resolver takes the first match, so a pointer meaning the second binds to
> the first from the day it is written, and no reading of the pointer can tell them apart.

### (4) Section ownership is recorded as not buildable

`check-spec-pointer` does not judge whether a resolving section owns the cited claim. The measured
ground is recorded, and the entry closes on the record {mechanical}.
**Not yet applied.**

**Replacement text, canon-kit/SPEC.md §check-spec-pointer** (**Not yet applied**). The calibration
paragraph opening "Calibration: forward direction only." gains:

> Nor does it judge ownership, meaning whether a resolving section is the one carrying the cited
> claim. That is comprehension. The one mechanical proxy measured (a backticked token in the citing
> paragraph that occurs in exactly one other section of the target) flagged 15 of 50 citations, and
> none was a wrong-section citation. A red there would train readers to skip the gate. Green means
> the section exists, and review owns the rest.

### (5) check-docs-cmd (C) reads an amendment basename as a path

Assertion (C)'s path shape admits a single-segment token matching `CANON_KIT_AMENDMENT_GLOB`. That
token resolves when any tracked file carries its basename, and reds when it names a retired path
{design-bearing}.
**Not yet applied.**

- **Why (C) and not a new predicate.** (C) already owns path liveness in code spans, including the
  retired-versus-never-tracked split and the history valves. A deleted amendment is retired by that
  definition. What misses it is the two-segment shape.
- **Resolution.** An amendment is sited in its component's directory and cited by bare basename,
  as `check-amendment-queue` resolves a `[spec:]` ref. So this shape resolves tree-wide by basename,
  not through (C)'s three roots. It is retired when a tracked path with that basename was deleted in
  history, and no tracked path carries the basename now.
- **Red condition.** Unchanged: retired, resolving nowhere, and not valved by the three
  `check-manifest-temporal` valves. The survey's illustrative names (`SPEC-sqlite.md`) were never
  tracked and stay admitted.
- **Found instance.** `drift-kit/SPEC.md` cites `docs/evidence.md`, which was never tracked. (C)'s
  recorded ground admits it, so build repoints it at `docs/evidence-data.md` or the page that owns
  the framing, and no predicate changes.

**Replacement text, canon-kit/SPEC.md §check-docs-cmd** (**Not yet applied**). In (C), after "and no
`..`, after the same quote and punctuation trims as (A) and a leading `./` stripped.":

> A single-segment token matching `CANON_KIT_AMENDMENT_GLOB` is path-shaped too, and it resolves by
> basename against the whole tracked tree, since an amendment is cited by bare name wherever its
> component sits.

### (6) A retired queue slug cited in governed prose is red

`check-queue-slug-liveness` gains assertion B. On every surface named by the new
`QUEUE_KIT_CITATION_SURFACE_GLOBS`, a single-backtick slug-shaped token that is in the queue's
retired set, and is no tracked file's stem, is red unless a history valve admits the line
{design-bearing}.
**Not yet applied.** Red, not report, by lead decision at spec (2026-09-17), informed by scope as
intent oracle: the operator's set direction fixed no report shape, and report-only was the hub's
filing-time assumption. The 35-hit count is the spec survey's and was not re-verified by the oracle.

- **Why a red is honest here when §The queue-edges arm refuses one.** That refusal is about the
  queue file's own bodies, where landed-work citations are ordinary. On the governed set, the rule
  is already written: prose about landed work drops the queue token and cites the owning SPEC
  (§check-queue-slug-liveness). The measurement agrees. With the live-name test applied, both
  survivors are false present-tense claims, and none was marked history.
- **The live-name test** is the queue-edges arm's `name live at` test: a tracked file whose basename,
  less its extension, equals the slug. It moves from `native/src/emit/queue_edges.rs` into the shared
  queue adapters, because a second reader now needs it.
- **The valve** is `<!-- retired-citation-exempt: <reason> -->` on the line or the one above, with a
  mandatory reason, on the line-or-one-above convention the canon-kit valves use. It is queue-kit's own marker, not canon-kit's
  `check-manifest-temporal` valves: borrowing those would make a queue-kit gate's verdict depend on
  another kit's knobs, and a consumer can vendor queue-kit alone.
- **The knob.** `QUEUE_KIT_CITATION_SURFACE_GLOBS` ships empty (clean skip). It is not
  `QUEUE_KIT_PROSE_SURFACE_GLOBS`: widening that knob to the SPECs would read about 45
  bold-code emphasis tokens (`sq`, `full`, `zero-config`) as membership claims. The descriptor
  couples `knob:QUEUE_KIT_CITATION_SURFACE_GLOBS`. This repo sets
  `QUEUE_KIT_CITATION_SURFACE_GLOBS[] <- CANON_KIT_MANIFEST_FILES`.
- **Degradation.** The retired set is `retired_set`'s, so a missing `git` or a shallow clone
  under-reds and never invents, the arm's declared degradation.
- **Landing fix.** `installer/SPEC.md`'s two citations of `init-vendor-staging-argv-overflow` name
  the defect or its owning section in place of the slug.

**Replacement text, queue-kit/SPEC.md §check-queue-slug-liveness** (**Not yet applied**). The
section's second paragraph is followed by:

> **Assertion B, the retired citation.** On every surface in `QUEUE_KIT_CITATION_SURFACE_GLOBS`, a
> single-backtick slug-shaped token naming a retired slug (§The queue-edges arm) whose name is no
> tracked file's stem is red. It is a pointer at disposed work, which this section already tells
> prose to replace with the owning SPEC. A line that is history takes a
> `retired-citation-exempt: <reason>` marker. The knob ships empty, and the membership surfaces stay a
> separate knob, because emphasis in bold code on a SPEC is not a queue claim.

### (7) A status parenthetical beside a queue citation matches the cited slug's section

`check-queue-slug-liveness` gains assertion C over the queue file. A single-backtick slug
immediately followed by a parenthetical that is exactly one status word must agree with where the
slug resolves {design-bearing}.
**Not yet applied.**

- **The vocabulary is derived, not declared.** It is the lowercased single-word section names the
  queue knobs configure (`QUEUE_KIT_DEFERRED_SECTION`, `QUEUE_KIT_ICEBOX_SECTION`,
  `QUEUE_KIT_DONE_SECTION`), plus `retired`. Here that is `(deferred)`, `(icebox)`, `(done)` and
  `(retired)`.
- **Agreement.** A section word agrees when the slug heads an entry in that section. `retired`
  agrees when the slug is in the retired set. A token that resolves to nothing is not a citation
  (§The tag algebra) and is skipped.
- **Honest limit.** A richer parenthetical (`(landed 2026-…)`, `(which account is active)`) and a
  multi-word active section name are outside the vocabulary and are never read. The entry's
  vocabulary-edges question is answered by exact match.
- **Red condition.** Disagreement. The finding names the line, slug, stated status and actual
  section.

**Replacement text, queue-kit/SPEC.md §check-queue-slug-liveness** (**Not yet applied**), after
delta 6's paragraph:

> **Assertion C, the status parenthetical.** Inside the queue file, a cited slug followed by exactly
> `(<status>)` must be where the status says. The vocabulary is the configured single-word section
> names, lowercased, plus `retired`. A stale status inverts a reader's conclusion, and the queue's
> own sections are a total oracle for it.

### (8) The done-slug ownership report dissolves into assertion B

No report arm is minted. A slug in `## Done` is already in the retired set, so delta 6 reds a
governed-prose citation of it at the commit that moves it {mechanical}.
**Not yet applied.** Settled by delta 6's lead decision.

**Replacement text, queue-kit/SPEC.md §The queue-edges arm** (**Not yet applied**). The paragraph
opening "**Not a gate, either.**" ends its second sentence with: "Assertion B of
§check-queue-slug-liveness reds a retired citation on the governed citation surfaces, where the
same token is a false present-tense pointer, and never inside the queue file."

### (9) Fixtures, tests and mirrors

Every assertion lands with its executable statement {mechanical}.
**Not yet applied.**

- `canon-kit/gate-tests/check-spec-pointer/`: `bad/` adds an unqualified citation resolving
  nowhere, a link-wrapped citation to a missing heading and a duplicated title. `good/` adds a
  self-citation, a sibling-SPEC citation from a README, a lead-clause citation and a `§<heading>`
  placeholder.
- `canon-kit/gate-tests/check-docs-cmd.test.sh`: a retired `SPEC-*.md` basename reds, and a
  never-tracked one stays admitted. Both need built history, which is (C)'s existing reason for the
  bespoke test.
- `queue-kit/gate-tests/check-queue-slug-liveness/` and a bespoke test for assertions B and C: B needs
  queue history, and C needs a configured icebox. Cover the empty-knob clean skip, the live-name
  exemption and the valve.
- `.workflow/release-declarations.md` gains three **Tightened gates** bullets: `check-spec-pointer`
  (unqualified and link-wrapped citations, and title collisions), `check-docs-cmd` (amendment
  basenames) and `check-queue-slug-liveness` (assertions B and C, and the new knob).

## Producers and consumers

- **Delta 1's citation forms.** Producer: the manifest walk `check-spec-pointer` already runs.
  Consumer: its heading resolver, extended with a union index over the manifest set, built once.
  Red condition: named in the delta. The verdict is monotone in the violation set: widening
  extraction can only add findings, and the union can only resolve more.
- **Delta 2's lead clause.** Producer: `HeadingCache`. Consumer: the prefix mode of both prose
  forms. Never the directive pass.
- **Delta 3's collision finding.** Producer: the same heading read. Consumer: the committing session
  through the output contract.
- **Delta 5's shape.** Producer: (C)'s token scan. Consumers: (C)'s resolver (a basename lookup
  over the tracked set it already builds) and its retired set (basename match).
- **Delta 6's knob.** Producer: `scripts/queue-config.knobs`, set in this repo, so the path is live
  outside tests. Consumers: `check-queue-slug-liveness` and the descriptor's `knob:` couples token.
  Roster-holding readers of a minted knob name, by the causal-completeness point 2: the binary's
  static knob roster (`--emit knob-roster`), `check-knob-citation`, `check-knob-default-coupling`,
  and `queue-kit/templates/queue-config.knobs`. Probe:
  `git grep -n "QUEUE_KIT_PROSE_SURFACE_GLOBS"` over the tracked tree, for every surface a sibling
  knob is rostered on.
- **Delta 6's shared live-name test.** Producer: the shared queue adapters (`native/src/queue.rs`).
  Consumers: the queue-edges arm and assertion B. Probe:
  `git grep -n "tracked_stems\|name_live"` over `native/src`.
- **Delta 6's valve.** Producer: an author's `retired-citation-exempt:` marker. Consumer: assertion
  B, which reads the line and the one above. Probe: `git grep -n "exempt:" native/src/gates` for
  the existing readers of that convention.
- **Delta 7's vocabulary.** Producer: the three section knobs plus the literal `retired`. Consumer:
  assertion C at match time. Every field of the finding (line, slug, stated, actual) is read by
  the committing session.
- **Point 6, members enumerable at authoring.** Delta 7's corpus is 3 parentheticals, and each one's
  satisfying value is its current accurate status. Delta 6's corpus is 35 hits: 33 are satisfied
  by the live-name test, and 2 are satisfied by the landing fix. Delta 3's corpus is one collision,
  satisfied by the rename. Delta 1's is named by class, with the landing sweep's rule for each.

Roster derivations, re-derived by the build:

- `git grep -n "§Consumer rules"` over the tracked tree (delta 3).
- `git grep -n "init-vendor-staging-argv-overflow" -- '*.md'` (delta 6).
- `bash gate-sdk/bin/run-gates.sh check-spec-pointer` after delta 1, for the sweep's actual count.

## Existing sections updated

- `canon-kit/SPEC.md` — §check-spec-pointer: the prose-citation paragraph, heading match, the
  title paragraph and calibration (deltas 1, 2, 3 and 4).
- `canon-kit/SPEC.md` — §check-docs-cmd assertion (C) (delta 5).
- `queue-kit/SPEC.md` — §check-queue-slug-liveness (deltas 6 and 7), §The queue-edges arm's
  "Not a gate" paragraph (delta 8), §Layout and configuration's knob list (delta 6), and
  §The shared queue adapters for the moved live-name test (delta 6).
- `native/src/gates/spec_pointer.rs` (deltas 1, 2 and 3), `native/src/gates/docs_cmd.rs` (delta 5),
  `native/src/gates/queue_slug_liveness.rs` (deltas 6 and 7), `native/src/queue.rs` and
  `native/src/emit/queue_edges.rs` (delta 6).
- `queue-kit/checks/check-queue-slug-liveness.gate` — the `knob:` couples token, and the
  `# spec:` gloss naming the three assertions (deltas 6 and 7).
- `queue-kit/templates/queue-config.knobs` and `scripts/queue-config.knobs` (delta 6).
- `guard-kit/SPEC.md`, `scripts/bash-guard.sh`, `guard-kit/templates/bash-guard.sh`,
  `guard-kit/lib/guard.sh` — the collision rename and repoints (delta 3).
- `installer/SPEC.md` — the two retired-slug citations (delta 6); `drift-kit/SPEC.md` — the
  `docs/evidence.md` citation (delta 5); the delta 1 sweep's citing files, per its run.
- The gate tests and fixtures, and `.workflow/release-declarations.md` (delta 9).
- `docs/canon-kit/SPEC.md`, `docs/queue-kit/SPEC.md`, `docs/guard-kit/SPEC.md`,
  `docs/installer/SPEC.md`, `docs/drift-kit/SPEC.md` — the on-site SPEC mirrors (all deltas).

## Retired spellings

- None — no delta retires a spelling. The live-name helpers move modules under their own names,
  and the renamed guard-kit heading keeps the `## Consumer rules` spelling live.

## Definition of Done

- [ ] **Causal completeness** — the three citation forms, the lead clause, the collision finding,
      the amendment shape, the new knob, assertions B and C and the moved live-name test each have a
      named producer and named consumers.
- [x] **Escalation answered** — red, not report, for deltas 6 and 8 (lead decision, 2026-09-17),
      recorded at delta 6.
- [ ] **Instruction surfaces: instruction only** — the three gates' `help:` lines carry the fix,
      not grounds.
- [ ] **Merged with no information lost** — each SPEC passage re-phrased, not appended.
- [ ] **Entries moved before the drain stage** — the hub and its nine members move to Done at a stage
      before `LIFECYCLE_KIT_DRAIN_STAGE`, with the merge that deletes this file.
- [ ] **Amendment deleted** — this file removed on merge, and none remain at the root for this unit.
- [ ] **Roster re-derived** — the probes above re-run against the tree before the merge counts.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — a cross-component gap found while landing filed through the gap inbox.
