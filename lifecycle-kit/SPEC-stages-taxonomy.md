# SPEC amendment: templates-stages-taxonomy-realignment

The tree scatters a class the SPEC names as one. `lead.md`, `release-sweep.md`,
and `upgrade.md` are each classified **"boundary skill, not a stage"**
(lifecycle-kit/SPEC.md:1375, :1391, :1417), yet the layout puts two of them
inside `templates/skills/` beside the stages and the third alone at `templates/`
root. This realigns the directory to the SPEC's own stage/boundary axis:
`release-sweep.md` and `upgrade.md` move up beside `lead.md`, and
`templates/skills/` becomes `templates/stages/`. The result is two adoption globs
that mean something — `templates/*.md` is exactly the boundary skills,
`templates/stages/*.md` is exactly the stage-class templates — in place of a
mixed bag plus a lone root file.

The payoff is legibility, not correctness; the cost is a breaking rename on a
published adoption path. The unit rides `pre-adoption-grammar-break` on the
operator's launch-timing ruling recorded in the queue entry, so the break
amortizes with the sibling queue-grammar break rather than standing as its own.

Measured at authoring and re-verified at align: **65 references across 33 files**
carrying the plain literal (the authoring census said 66/31 and did not
reproduce), **plus one occurrence no plain grep finds** — an escaped-slash regex,
delta 5. Of those, 9 are gate-visible couplings that red or silently degrade,
**15** are `§templates/skills/` SPEC cross-references, 2 are markdown anchors
into a renamed heading, 14 are regenerated docs-mirror lines, 5 are regenerated
projection literals, and 5 occurrences across 4 files are immutable published
release notes that must not be touched.

Two occurrences are stale-but-unenforced and are left to their owners:
`TASK-QUEUE.md:1352` (an unrelated queue entry citing a stage-template path) and
the sibling amendment's `:233`/`:308`. `scripts/check-kit-ref-liveness.sh:58,:60`
valves both file classes, so neither reds; the sibling's own body names its path
dependency.

## Seam ruling

**Kit mechanism:** the directory layout and both adoption globs. A template
directory's name is kit structure a consumer copies, exactly like the `checks/`
and `gate-tests/` conventions gate-sdk fixes.

**Consumer config:** none added, and — importantly — **none removed**. The path
is a hardcoded literal in every reader today and stays one. No
`LIFECYCLE_KIT_TEMPLATE_DIR` knob is minted: a configurable template path would
let two consumers spell one kit's layout two ways while every SPEC
cross-reference still names one of them, and the gates that resolve template
paths read them out of the *shim's own directive line*, not from config.
`LIFECYCLE_KIT_SKILLS_DIR` is unrelated despite the name — it points at the
consumer's `.claude/commands/`, and this amendment does not touch it.

**Private rule content:** none. `stages` is the SPEC's own word for the class.

## What changes

### 1. The two moves — *design-bearing*

```
templates/lead.md            templates/lead.md
templates/skills/            templates/release-sweep.md      (moved up)
  release-sweep.md    ──▶    templates/upgrade.md            (moved up)
  upgrade.md                 templates/lifecycle-config.sh
  scope.md                   templates/stages/scope.md
  spec.md                    templates/stages/spec.md
  align.md                   templates/stages/align.md
  build.md                   templates/stages/build.md
  validate.md                templates/stages/validate.md
  close.md                   templates/stages/close.md
```

`templates/*.md` then enumerates the boundary skills exactly — three files, no
exceptions — and `templates/stages/*.md` the stage-class templates exactly.
`lifecycle-config.sh` is not markdown and so does not pollute the first glob.

**One honesty note the new name must carry:** `templates/stages/` is the
*stage-class* template set, not any one consumer's roster.
`LIFECYCLE_KIT_STAGES` defaults to `(scope align build validate close)` — five —
while the directory ships six, because `spec.md` serves the roster that splits
authoring out (this repo's). The directory is a superset of a default adoption
and **the roster is still not derivable from it**, which is what the queue entry
observed and what keeps `check-stage-skill-coverage` reading the configured
roster rather than a listing. The realignment buys a legible glob, and claiming
a derivation win it does not deliver would be the drift this repo gates against.

### 2. The SPEC's own section structure follows the move — *design-bearing*

`### templates/skills/` (lifecycle-kit/SPEC.md:1236) currently hosts **three
distinct contracts** — the stage-skill grammar, release-sweep, and upgrade — and
the latter two paragraphs cite *their own containing section* for the
binding-shim grammar (`:1386`, `:1406`). Leaving that intact under a
`templates/stages/` heading would put both boundary skills inside the stages
section, re-creating in prose exactly the miscategorization the directory move
fixes.

So the section splits, following the **`### templates/lead.md` precedent**
(:1412) — a *file-level* heading for each boundary skill:

- `### templates/stages/` — the stage-skill grammar and the two adoption modes.
- `### templates/release-sweep.md` — the paragraph at :1375-:1389, with "Beside
  the stage skills sits `release-sweep.md`" reworded: it now sits beside
  `lead.md` at root, and the sentence's whole point is that it is *not* beside
  the stages.
- `### templates/upgrade.md` — the paragraph at :1391-:1410, whose "Beside it
  sits `upgrade.md`" chains off release-sweep's antecedent and is rewritten to
  stand on its own section.

Both new sections cite `§templates/stages/` for the binding-shim grammar from
outside it — the same direction `### templates/lead.md` already cites it, which
is why the precedent transfers cleanly rather than needing a new grammar.

### 3. The anchor break is real and is caught — *mechanical*

Renaming the heading changes its anchor from `#templatesskills`, and two
hand-authored links target it: `docs/install.md:232` ("stage-skill modes") and
`docs/install.md:389` ("The upgrade skill"). `check-md-refs`' `anchor_ok()`
resolves both, and `docs/install.md` is in the manifest set via
`CANON_KIT_MANIFEST_FILES`, so **both reds fire in the same commit** — no
specification is owed beyond retargeting them. Note they retarget *differently*:
:232 to `#templatesstages`, :389 to the new `#templatesupgrademd`. The upgrade
link pointing at the stages section would resolve and be wrong, which is the one
failure the gate cannot see.

### 4. `check-skill-binding`: seven directives, and a couples gap — *design-bearing*

This gate is the migration's forcing function and its blind spot in one file.

**The forcing function.** `check-skill-binding.sh:25-31` extracts the template
path from each shim's line-1 directive and **stats it** — `[[ ! -f "$tmpl" ]]`
reds with "binding directive names template '<path>' — no such file". So the
seven shims naming a moved template (`align`, `build`, `close`, `scope`, `spec`,
`validate`, `release-sweep`) must be rewritten in the same commit as the move, or
the gate reds. It also reads each template's slot set to assert bidirectional
slot/binding equality, so shim and template are already atomically coupled. This
is the "gate that catches a botched migration" the unit owes, and it exists.

**The blind spot, and the delta that closes it.** `check-skill-binding.sh:2`'s
`couples=` names `lifecycle-kit/templates/skills/*.md` and
`lifecycle-kit/templates/lead.md` **file-by-file — it carries no
`kit:templates/*.md` glob**. Retargeting the first literal to
`templates/stages/*.md` therefore leaves `release-sweep.md` and `upgrade.md`
covered by *nothing*: the gate would keep passing while an edit to either
template stopped triggering it. Both are added explicitly beside `lead.md`,
matching the precedent's file-level entry. `upgrade.md` gets one even though this
repo binds no `upgrade` shim — the coupling is about the template's edits
triggering the gate, and a consumer that does bind it inherits the manifest.

### 5. The trigger literals, and the silent-enforcement loss — *design-bearing*

Four gates carry `lifecycle-kit/templates/skills/*.md` in their `# graph:`
manifests. **Exactly one carries it as a `trigger=`** —
`context-kit/checks/check-footprint-fresh.sh:2`, which carries it in both fields.
The other three carry it as `couples=` only. (The authoring draft claimed two
`trigger=` bearers and named `scripts/check-value-rollup-fresh.sh`; that file has
no `trigger=` field at all.)

**The correction makes the hazard wider, not narrower.**
`gate-sdk/bin/gen-pre-commit.sh:71` reads
`trigger="$(manifest_field "$gate" trigger)"; trigger="${trigger:-$couples}"` —
**`couples=` is the trigger fallback.** So every one of the four gates' staged-
path conditions in the generated hook derives from this literal, whichever field
holds it. A manifest left pointing at a directory that no longer exists does not
red — **it stops firing.** The freshness gates over `docs/footprint.md` and
`docs/value.md` would silently stop reacting to stage-template edits, and both
artifacts would drift with nothing to catch them. This is the same failure class
the sibling amendment's derivation delta closes on the tag side, and it is the
reason this unit is not the cosmetic no-risk change its own queue entry called
it. All four manifests retarget in the same commit, and all four matter equally:

- `lifecycle-kit/checks/check-shim-restatement.sh:2` — couples
- `lifecycle-kit/checks/check-skill-binding.sh:2` — couples (plus delta 4)
- `context-kit/checks/check-footprint-fresh.sh:2` — couples **and** trigger
- `scripts/check-value-rollup-fresh.sh:2` — couples

**A fifth reader, invisible to a path sweep.** `drift-kit/bin/overhead-meter.sh:71`
carries the path as an *escaped-slash awk regex* inside the overhead meter's
fixed classifier table:
`/lifecycle-kit\/templates\/skills|enter-stage|WORKFLOW-STATE|Execute the template at/`.
A `grep templates/skills` does not find it, which is why the authoring census
missed it. The alternation's other arms keep matching stage *invocations*, so the
classifier does not break wholesale — what silently reclassifies is the residue
this arm uniquely covered: bare path mentions with no invocation verb (a grep
over the stage templates, a file read, a SPEC citation), which fall out of the
`stage` bucket into `govdoc` or task. It retargets with the rest.
`drift-kit/SPEC.md:324-346` documents the marker table without restating the
path, so the script is the sole owner and no doc edit is owed.

`check-reads-couples` cannot help here: `check-shim-restatement`'s walk passes a
bare variable to `gate_find`, so its root is undecidable and the case is
skipped-and-counted. The retargeting is asserted by review and by the regenerated
hook (delta 8) agreeing with the manifests, not by a coverage gate.

### 6. `check-shim-restatement` needs no code change — *mechanical*

Its corpus walk (`:31-35`) does `gate_find "$root/templates" -name '*.md'`, and
`gate_find` recurses. The corpus therefore picks up `templates/stages/*.md` and
the two promoted files automatically, with **no gate-body edit** — the same
"widen with no gate edit" property the sibling amendment's icebox tier relies on,
and for the same reason: a recursive shared adapter rather than a per-directory
literal.

What does change is the explanatory sentence at lifecycle-kit/SPEC.md:1154-1158,
which justifies the explicit couple by naming "the one kit template
*sub*directory that holds bound templates — `lifecycle-kit/templates/skills/*.md`".
After the move that subdirectory is `templates/stages/`, and the two promoted
files fall under the plain `kit:templates/*.md` couple the sentence contrasts
against — so the sentence gets simpler, not just retargeted.

### 7. The promoted files enter a prose-surface glob, and self-exclude — *design-bearing*

`scripts/canon-config.sh:59-62` sets `CANON_KIT_PROSE_SURFACE_GLOBS=("*/templates/*.md" ".claude/agents/*.md")`
— a **single-level** glob that does not reach `templates/skills/` today. After
the move, `templates/release-sweep.md` and `templates/upgrade.md` land inside its
range and become candidates for the narration-gate family (`check-md-refs`,
`check-manifest-count`, `check-manifest-temporal`, `check-spec-pointer`,
`check-comment-tier`, `check-knob-citation`).

They **self-exclude**, and the amendment states why rather than leaving it to be
rediscovered when a gate unexpectedly does or does not fire: `_spec_slot_free`
(`canon-kit/lib/spec.sh:195`) drops slot-bearing templates from the surface, and
both files carry binding slots — `release-sweep.md` has `*<inventory-command:`
and `*<evidence-gate:`, `upgrade.md` has `*<gates-list:` and
`*<disposition-evidence:`. This is precisely the `lead.md` precedent, which has
sat in that glob's range since it was placed at root. **The promotion is safe
because both files are slot-bearing**, and that is a property of the boundary-
skill class rather than a coincidence of these two files: a boundary skill
carries named slots by definition (it is adopted by copy-and-specialize or by a
thin shim), which is what makes root placement correct for the class.

### 8. Projections and the docs mirror — *mechanical*

- `scripts/git-hooks/pre-commit` — 4 embedded literals (`:130`, `:142`, `:192`,
  `:196`). **Generated**; regenerate with `gate-sdk/bin/gen-pre-commit.sh --write`
  after the manifest edits, never hand-edit. `check-graph` byte-compares it.
- `docs/check-graph.html` — node `n192` plus four edges. Regenerate via
  `check-graph --emit`.
- `docs/lifecycle-kit/{SPEC,README}.md`, `docs/canon-kit/SPEC.md`,
  `docs/gate-sdk/SPEC.md` — 14 mirrored hits. Regenerate with
  `scripts/gen-docs-mirror.sh`; `check-docs-mirror-fresh` byte-compares.
- `docs/enforcement.md` — **no path hit** (gate-name rows only), so no
  regeneration is owed on this axis.

### 9. Hand-edited prose references — *mechanical*

Fifteen `§templates/skills/` SPEC cross-references retarget to
`§templates/stages/` **except where the referent is a boundary skill**, which is
the judgment a blind sweep gets wrong: `gate-sdk/SPEC.md:684` cites the section
for the *upgrade* skill and retargets to `§templates/upgrade.md`, and
`RELEASING.md:78` cites it for the *release-sweep* contract and retargets to
`§templates/release-sweep.md`. The remainder — `lifecycle-kit/SPEC.md` (:493,
:764, :1061, :1156), `canon-kit/SPEC.md:351`, `canon-kit/lib/spec.sh:194`,
`scripts/check-release-bump.sh:27,:36`, `docs/install.md:359`,
`lifecycle-kit/README.md:70` — are about the binding-shim or stage-skill grammar
and retarget to `§templates/stages/`.

Three more the authoring count missed, and one of them dangles:
`lifecycle-kit/SPEC.md:1386` and `:1406` are the two boundary paragraphs' self-
citations, absorbed by delta 2's section split. **`lifecycle-kit/SPEC.md:1420` is
not absorbed by anything** — it sits inside `### templates/lead.md` ("it carries
named slots, so it adopts the binding-shim grammar (§templates/skills/)") and
retargets to `§templates/stages/`, which is also the direction delta 2 cites as
the precedent the two new sections follow. (`:225` and `:228` are plain path
literals, not `§`-form; they are covered by the adoption-mode prose below.)

Plain-path prose: `lifecycle-kit/README.md:68` (the adoption line),
`docs/install.md:307`, `RELEASING.md:4`, `.claude/agents/stage-session.md:16`,
and `templates/stages/scope.md:80` — a self-reference *inside* the moving tree,
which a `git mv` carries along unchanged and therefore leaves stale.

### 10. Frozen occurrences that must not migrate — *design-bearing*

Four published release notes carry the old path:
`docs/posts/2026-07-19-checkwright-v0-8-0.md:28,:39`,
`docs/posts/2026-07-19-checkwright-v0-9-0.md:52`,
`docs/posts/2026-07-25-checkwright-v0-14-0.md:26`,
`docs/posts/2026-07-25-checkwright-v0-15-0.md:54`.

**These are immutable and are left exactly as they are.** A published note
describes the tree as it stood at that tag; rewriting it would falsify the
record, not update a path. The tree already protects them —
`CANON_KIT_TEMPORAL_EXEMPT_PATHS=("docs/posts/*")` and
`check-kit-ref-liveness.sh:55` valves the same glob — so the exemption is
mechanical rather than a convention the sweep has to remember. It is stated
because a tree-wide path sweep is precisely the operation that would ignore it.
This is the same guard the sibling amendment states for the token's frozen
attestations, and the two units share the rule: **a reference whose subject is
the historical name is preserved verbatim.**

### 11. Release-note declaration — the only compat obligation — *mechanical*

A moved adoption path is not a knob and not a gate, so it declares under
**Behavior changes** in the release note, per the three-section note structure
docs/install.md owns. That section is explicitly *not* smoke-asserted —
`upgrade-smoke` proves phase-A determinism and that the red set is declared, but
the binding shims that break here live in the **consumer's** tree, which phase A
never touches. The consumer-visible migration is therefore: retarget your shim
directives from `templates/skills/<stage>.md` to `templates/stages/<stage>.md`,
and from `templates/skills/{release-sweep,upgrade}.md` to
`templates/{release-sweep,upgrade}.md`. The note states that verbatim, because
nothing mechanical will state it for them.

With no deprecation window owed (§Ruled out), **that note is the entire compat
obligation this move carries**, which raises its weight rather than lowering it.
The release class is a **minor**: docs/install.md:349-351's pre-1.0 qualifier
rides a non-decommission break on a minor while the line is 0.x, and :341-345
reserves major for a release that *removes* a deprecated surface. Nothing is
removed here — a path moves — so no major is earned. This unit and **both**
siblings — `needs-spec-tag-rename` and `deferred-queue-carry-cost`, whose new
gate declares under Tightened gates — ride **one** minor and **one** note, which
is what delivers the one-re-bind outcome the launch-timing ruling weighed. No
unit owns the note; each declares its own bullets into it.

## Producers and consumers

- **`templates/stages/*.md`.** Producer: the kit, as shipped source. Consumers:
  `check-skill-binding` (stats each path out of a shim directive, reads its slot
  set), `check-shim-restatement`'s recursive corpus walk, the `couples=`/
  `trigger=` manifests of four gates → the generated pre-commit hook → the
  committing session, and every consumer's shim directive.
- **`templates/{release-sweep,upgrade}.md`.** Producer: the kit. Consumers:
  `check-skill-binding` via its new file-level couples entries (delta 4) and, for
  release-sweep, this repo's `.claude/commands/release-sweep.md` shim;
  `check-shim-restatement`'s corpus; `CANON_KIT_PROSE_SURFACE_GLOBS` →
  `_spec_slot_free`, which drops them (delta 7).
- **The `### templates/stages/`, `### templates/release-sweep.md`, and
  `### templates/upgrade.md` headings.** Producer: lifecycle-kit/SPEC.md.
  Consumers: 13 `§`-form cross-references across four kits plus `RELEASING.md`
  and `scripts/check-release-bump.sh`; the two `docs/install.md` markdown anchors
  resolved by `check-md-refs`; the docs mirror.
- **The retargeted `trigger=` literals.** Producer: the `# graph:` manifests.
  Consumers: `gen-pre-commit.sh` → `scripts/git-hooks/pre-commit` → the staged-
  path match that decides whether `check-footprint-fresh` and
  `check-value-rollup-fresh` run at all.

No new state, knob, or interface is introduced. Every new *name* is a directory
or a SPEC heading, and each is named above with the readers that resolve it.

## Existing sections updated

lifecycle-kit/SPEC.md — `### templates/skills/` at :1236 becomes
`### templates/stages/`; the release-sweep paragraph (:1375-1389) and the upgrade
paragraph (:1391-1410) become their own file-level sections per delta 2, with
their "beside the stage skills" / "beside it" framings rewritten; the
`check-shim-restatement` rationale at :1154-1158; the adoption-mode prose at :225
and :228; and the `§`-form refs at :493, :764, :1061. §check-skill-binding gains
the two file-level couples (delta 4).

canon-kit/SPEC.md:351, gate-sdk/SPEC.md:684, canon-kit/lib/spec.sh:194 — the
cross-kit `§` refs, retargeted per delta 9.

lifecycle-kit/SPEC.md:1420 — the `§` ref inside `### templates/lead.md`
(delta 9).

lifecycle-kit/README.md:68,:70 — the adoption glob line and its `§` ref
(delta 9).

docs/install.md:232, :389 — the two anchors (delta 3); :307, :359 — the two
prose paths (delta 9).

RELEASING.md:4, :78; scripts/check-release-bump.sh:27,:36;
`.claude/agents/stage-session.md:16`; `templates/stages/scope.md:80` — delta 9.

`.claude/commands/{align,build,close,scope,spec,validate,release-sweep}.md:1` —
the seven binding directives (delta 4).

`context-kit/checks/check-footprint-fresh.sh:2`,
`scripts/check-value-rollup-fresh.sh:2`,
`lifecycle-kit/checks/check-shim-restatement.sh:2`,
`lifecycle-kit/checks/check-skill-binding.sh:2` — the manifests (delta 5);
`lifecycle-kit/checks/check-skill-binding.sh:2` also gains the two file-level
couples (delta 4).

`drift-kit/bin/overhead-meter.sh:71` — the classifier-table regex (delta 5).

docs/posts/ — the release note's Behavior-changes bullet (delta 11).

**Deliberately not updated:** the four `docs/posts/` notes (delta 10); every
`gate-tests/` fixture (all use bare relative `templates/<name>.md` sandbox paths
and carry no lifecycle directory literal — verified tree-wide); every `*.list`
manifest; `installer/`, `demo/`, `.github/`, `.claude/settings.json`,
`CLAUDE.md`, `scripts/lifecycle-config.sh`, and `lifecycle-kit/lib/stages.sh`,
none of which names a template directory.

## Ruled out

- **A `LIFECYCLE_KIT_TEMPLATE_DIR` knob.** Seam ruling above — the readers
  resolve template paths out of shim directives, not config, so a knob would add
  a name nothing reads while every `§` cross-reference still hardcoded one
  spelling.
- **Renaming the directory without promoting the two boundary skills.** This is
  the cheaper half and it is refused: `templates/stages/` containing
  `release-sweep.md` and `upgrade.md` would be a *worse* name than `skills/`,
  because it would assert the taxonomy the contents contradict. The two moves are
  one change.
- **Promoting the boundary skills without renaming the directory.** Leaves
  `skills/` meaning "stages" with nothing saying so, and spends the shim break
  for one of the two payoffs.
- **Deriving `LIFECYCLE_KIT_STAGES` from the new directory listing.** Delta 1 —
  the roster is ordered and consumer-configurable and the directory ships a
  superset. There is no derivation win here and the amendment declines to claim
  one.
- **A compat directory or symlink at `templates/skills/`.** Ruled 2026-07-31 at
  spec: **no window is owed, because no clause reaches this move.**
  lifecycle-kit/SPEC.md:239-244 is the nearest-looking rule and does not apply —
  it is headed *"Knob-rename compat precedent"*, sits inside §Layout and
  configuration immediately above the `LIFECYCLE_KIT_*` roster it governs, and is
  cited nowhere else in the tree; its bare "a rename" is loose drafting inside a
  knob-scoped clause. This unit renames a directory, not a knob. **A scoping
  finding, not an exception taken.** (The clause's own threshold has separately
  drifted from its premise; that defect is filed for close, not fixed here.)

  The queue entry asserted this unit "needs a deprecation marker and a release
  note, exactly the machinery `upgrade.md` narrates". That is **factually off on
  the first half, and the correction is not a narrowing of the deliverable**:
  `upgrade.md` contains no occurrence of "deprecat" at all — it narrates the
  release-note-plus-gate-registration half — so the entry asserted a fact about a
  file's contents the file does not contain. The actual marker machinery
  (`check-deprecation-task` + `release-sweep.md`) resolves `task: <slug>`
  bindings on a *source-comment* surface via `CANON_KIT_DEPRECATION_MARKERS`, a
  roster this repo does not set. There is no directory-level marker, no alias
  convention, and no shim precedent anywhere in the tree. Scope's real deliverable
  — the rename plus a declared release note — is intact and is delta 11.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The only surviving `templates/skills/` mentions
      are delta 10's immutable published notes.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Enforcement retargeted, not just renamed** — all four manifest literals
      and `overhead-meter.sh`'s classifier regex point at the new directory, and
      both freshness gates are observed firing on a stage-template edit (they
      trigger off `couples=` by fallback, so all four matter, not just the one
      `trigger=`); the two promoted files carry file-level couples;
      no `--no-verify`.
