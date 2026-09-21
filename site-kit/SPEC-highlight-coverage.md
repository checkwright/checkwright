# SPEC amendment: highlight-coverage

A site that restyles code highlighting has to override every token class that
the theme's own highlight CSS colours. Any class it misses keeps the theme's
colour. On a dark background that can mean an unreadable token. This happened on
this site: Rouge marks shell `&&` with `.o`, and Primer colours `.o` `#000000`,
so every shell fence's operators rendered black on the dark code background
(operator report, 2026-09-21). The lead hotfixed the layout by remapping the
classes it found. Nothing gates the coverage, so a theme bump or a newly coloured
class brings the defect back. The unit's constraint, stated by the lead: **the
theme's highlight CSS is not in the tree**, so no gate can read it.

**This splits exactly along site-kit's monitor boundary** (site-kit/SPEC.md §The
monitor boundary: "the line is where the asserted object lives"). Two objects are
involved, and they live in two places:

- **The layout's overrides are in the tree.** That the layout covers a declared
  class list is a tree property, so a gate asserts it.
- **The theme's coloured class set lives on the host.** It is served from the
  deployment and set by a theme version the Pages platform chooses, so no commit
  contains it. That the declared list still matches the live theme is deployment
  truth. The scheduled monitor, `templates/site-health.yml`, asserts it, and it
  signals through an issue and a red run of its own, never a blocked merge.

The tracked **snapshot**, the declared class list, is the seam between the two.
The gate reads it as its oracle and the monitor holds it to the live CSS. It is
consumer config: which theme a site uses and which classes that theme colours is
one project's content. The kit ships only the mechanism.

**Measured at this spec (2026-09-21), and it changed the hotfix.**
`curl https://checkwright.dev/assets/css/style.css` returned 76,559 bytes. From
it, every rule whose declaration sets `color` or `background` was taken. Its
selector list was **split on commas**, and each `.highlight .<class>` was kept.
That gives **63** classes. The same extraction run over the first draft of the
hotfix layout, followed by `comm -23`, left **four uncovered**: `.cd`, `.kv`,
`.mb` and `.mx`. Each is the second or later member of a grouped rule:

- `.highlight .c,.highlight .cd{color:#999988;…}`
- `.highlight .k,.highlight .kv{color:#000000;…}` — `.kv` is black, the
  reported defect's own colour
- `.highlight .m,.highlight .mb,.highlight .mx{color:#009999}`

The lead's check, which found none uncovered, read only each rule's first
selector. So the extraction rule below is specified to the comma, because the
one-selector reading is the error it already produced once. Before the hotfix,
the same comm left 18 uncovered. The committed hotfix, `57761a4d`, covers all
four as well, and the comm over it comes back empty. The lead measured that, and
it was re-run here against HEAD. So the layout satisfies the snapshot on landing.

## What changes

**Batching.** Deltas 1 and 2 land together, the gate and its monitor twin, since
each is half of the one invariant. Delta 3 arms both for this repo and, in the
same commit, fixes whatever the gate reds on first.

### (1) `check-docs-highlight-coverage` asserts the layout overrides every declared class {design-bearing}

**Not yet applied.** This is a new site-kit gate on the native substrate, with a
`good/`+`bad/` fixture pair, `tier=precommit` and
`# armed-by: SITE_KIT_HIGHLIGHT_TOKENS`.

- **The snapshot** is the file `SITE_KIT_HIGHLIGHT_TOKENS` names. The knob's
  default is empty, which disarms the gate, and the gate still prints its clean
  line. The file holds a `# contract:` header, then one class per line, written
  as `.o`. Blank lines are ignored.
- **The overrides** are the tracked files matching `SITE_KIT_HIGHLIGHT_OVERRIDES`,
  an array of pathspecs whose default is `docs/_layouts/*.html`. A class is
  **covered** when some CSS rule in those files has a selector, after the
  selector list is split on commas and whitespace is collapsed, that ends in
  `<scope> .<class>`. `<scope>` is `SITE_KIT_HIGHLIGHT_SCOPE`, default
  `.highlight`, and any ancestor prefix before it is admitted. That admits this
  site's `.site-content .markdown-body .highlight .o`.
- **Assertion:** every snapshot class is covered.

**Point 5, the red condition.**

- Each uncovered class is a finding naming the class and the snapshot line.
- An armed snapshot holding **zero** classes is a finding. A capture that
  extracted nothing is the extraction's failure, not an empty theme, so this
  reader reds on finding none, by design.
- An override glob matching no file is a finding, since the covering file is
  gone.
- The clean line counts snapshot classes, override files and covering rules.
- Exit 2 applies when the snapshot is named but unreadable.

### (2) `templates/site-health.yml` gains the theme-drift arm {design-bearing}

**Not yet applied.** This is an optional arm on the template's existing
set-the-env-or-delete-the-arm pattern (site-kit/SPEC.md §templates/site-health.yml).
It takes two step-level env values:

- `HIGHLIGHT_CSS_PATH`: the site path of the theme stylesheet;
- `HIGHLIGHT_TOKENS_FILE`: the tracked snapshot, which must be the same file
  the gate's knob names.

It is step env and not a `SITE_KIT_*` knob, on that section's own ground that
the template never reads the gates directory.

The arm fetches `https://<apex><HIGHLIGHT_CSS_PATH>`, with the apex taken from
the CNAME file as the other arms take it, and extracts the class set by **the
extraction rule**:

- **Rules.** Split the stylesheet into rules at `}`.
- **Colour-bearing.** Keep a rule when its declaration block sets `color`,
  `background` or `background-color`.
- **Selectors.** Split its selector list on `,`.
- **Classes.** Keep each selector that is exactly `.highlight .<class>`, a single
  class after the scope.
- **Result.** Sort and de-duplicate.

It then compares that set with the snapshot's classes in **both directions**:

- A class the theme colours and the snapshot lacks is a finding, because the
  gate would never ask for its override.
- A class the snapshot lists and the theme no longer colours is a finding,
  because the snapshot is a copy and a copy must match.

The finding prints the added and removed classes **and the full new list**, so
the remedy is a paste into the snapshot, followed by whatever overrides the
gate then reds on.

The zero-cases, ruled on the arm's own pattern:

- a fetch failure is a finding, asserted on the call's exit status;
- a fetched stylesheet yielding **zero** classes is a finding, since the
  extraction or the theme's structure broke;
- with both env values unset, the arm is skipped with a census line saying so.

The census prints fetched bytes, colour-bearing rules and extracted classes.

**The honest limit, in the section.** The gate cannot see a theme bump. The
monitor sees it on its next scheduled run. So between a bump and that run, the
live site can show uncovered tokens. The daily schedule bounds how long that
lasts, and the pre-bump tree cannot prevent it.

### (3) This repo arms both halves {mechanical}

**Not yet applied.**

- A new `scripts/highlight-tokens.list` holds the 63 classes, captured with delta
  2's rule. Build re-captures them at build time, and the arm's extraction must
  reproduce the file.
- `scripts/site-config.knobs` sets `SITE_KIT_HIGHLIGHT_TOKENS =
  scripts/highlight-tokens.list`.
- `scripts/gates.list` registers the gate.
- `.github/workflows/site-health.yml`, this repo's copy of the template, gains
  the arm, with `HIGHLIGHT_CSS_PATH=/assets/css/style.css` and
  `HIGHLIGHT_TOKENS_FILE=scripts/highlight-tokens.list`.
- **`docs/_layouts/default.html` gains overrides for any class the gate reds on,
  in the same commit.** At `57761a4d` that set is empty, since the layout covers
  all 63 classes. A class that joins the theme between spec and build takes its
  family's palette variable, the way the hotfix mapped `.cd`, `.kv`, `.mb` and
  `.mx`.

Mechanical: every value is enumerated.

## Producers and consumers

- **The snapshot (deltas 1 to 3).** Producer: a maintainer pasting the monitor's
  list. Consumers: the gate, which reads the classes, and the monitor arm, which
  reads the same file. It has one field per line, the class, and both read it.
- **The three knobs (delta 1).** Producer: site-kit's static table
  (site-kit/SPEC.md §Knob defaults gains three rows), plus this repo's knob file.
  Consumer: the gate. Roster-holding readers:
  - `check-knob-citation`, satisfied by the §Knob defaults rows;
  - `check-install-disposition` assertion D, satisfied by
    `SITE_KIT_HIGHLIGHT_TOKENS` being a declared static knob;
  - doctor's disarmed-member line, which names the gate on a fresh install.

  The snapshot and override paths are the gate's corpus, so the descriptor
  carries `knob:SITE_KIT_HIGHLIGHT_TOKENS` and `knob:SITE_KIT_HIGHLIGHT_OVERRIDES`.
- **`check-docs-highlight-coverage` (delta 1).** Producer: its descriptor, with
  `# install: zero-config` (it is disarmed until the knob is set). Consumers:
  - the battery and the hook;
  - site-kit/README.md's gate roster, a roster-holding reader
    (`check-readme-roster`);
  - the fixture runner.
- **The monitor arm (delta 2).** Producer: the template step. Consumers: the
  `site-health` issue and the run's status, and a maintainer reading the printed
  list. The template's `permissions:` block needs no new scope, because the fetch
  is anonymous HTTPS. So `check-action-permissions` is unaffected.
  `check-tree-terms` and the `check-action-*` gates read the installed template
  in site-kit's smoke (§templates/site-health.yml, last paragraph), so a
  regression in the arm's bash reds the smoke.
- **Point 6.** The corpus is the 63 snapshot classes, and each one's satisfying
  value is a covering selector in `docs/_layouts/default.html`. All 63 are covered
  at `57761a4d`. No member is narrowed past.

## Existing sections updated

Rosters produced by the live-CSS extraction and the two `comm -23` runs in the
preamble, by `grep -n "highlight" docs/_layouts/default.html`, and by reading
site-kit/SPEC.md in full.

- site-kit/SPEC.md: §Knob defaults gains three rows, a new
  §check-docs-highlight-coverage is added, §templates/site-health.yml gains the
  arm, and §Layout and configuration's list of what registers where is updated
  (deltas 1 and 2).
- `native/src/knobs/site_kit.rs`, a new gate module and its `REGISTRY` row,
  `site-kit/checks/check-docs-highlight-coverage.gate`, its fixture pair, and
  site-kit/README.md's gate roster (delta 1).
- `site-kit/templates/site-health.yml` (delta 2).
- `scripts/highlight-tokens.list`, `scripts/site-config.knobs`,
  `scripts/gates.list`, `.github/workflows/site-health.yml` and
  `docs/_layouts/default.html` (delta 3).
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/site-kit/SPEC.md`, `docs/site-kit/README.md`, `docs/enforcement.md`,
  `docs/check-graph.html`, `scripts/git-hooks/pre-commit`.

## Retired spellings

- None — no name is removed. The monitor gains an arm and the gate is new.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only.** The template's arm comment
      names the remedy and not its grounds.
- [ ] **Merged with no information lost.** Each addition re-phrases the text it
      refines.
- [ ] **Amendment deleted.** This file is removed on merge
      (`ls site-kit/SPEC-*.md`).
- [ ] **Entry moved.** `site-dark-token-palette-coverage-ungated` moves to Done in
      the merge commit, at a stage before the drain stage.
- [ ] **The grouped-selector case is a fixture.** The gate's `good/` layout
      covers one class only as a non-first member of a grouped rule, and the gate
      stays green. The `bad/` layout omits one snapshot class, and the gate reds.
- [ ] **The monitor's extraction is exercised before landing.** Build runs the
      arm's extraction step body locally against the fetched live stylesheet. It
      must reproduce `scripts/highlight-tokens.list` exactly, including `.cd`,
      `.kv`, `.mb` and `.mx`. Nobody dispatches `site-health` by hand (CLAUDE.md);
      its next scheduled run is the live confirmation.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
