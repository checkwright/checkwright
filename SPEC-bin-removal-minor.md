# SPEC amendment: bin-removal-minor

Deleting a kit-shipped `bin/` tool does not reach the major-version criterion.
docs/install.md §Versioning defines a decommission as removing a
**deprecated** surface. No kit tool has ever carried a deprecation marker, and
`CANON_KIT_DEPRECATION_MARKERS` defaults to empty in canon-kit's table
(`native/src/knobs/canon_kit.rs`). So every tool removal so far has ridden a
minor by accident rather than by rule. This amendment makes that the rule. It
takes the third of the three shapes the paired entry weighed, which is the
shape the operator directed. Sited at the repo root because the rule's home is
docs/install.md, a repo-root-governed page, and not canon-kit, the entry's
filing surface. canon-kit's marker mechanism is unchanged.

## What changes

### (1) A kit `bin/` tool is not a deprecation-marked surface, and deleting one rides a minor

§Versioning's Major bullet states that deleting a kit `bin/` tool is not a
decommission. The removal rides a minor on either side of `v1.0.0` and is
declared by path under Behavior changes. **{mechanical}**

**Why this shape and not the other two.**

- **Widening the major criterion to cover any shipped-surface removal**
  collides with two rules in the same section. A decommission earns a major
  "even while 0.x", and `v1.0.0` is "cut deliberately, never earned
  mechanically". The next tool removal would force the first stability promise.
- **A gate requiring a marker in an earlier release** turns every removal into
  a two-release sequence. Its measured cost is high: 40 kit `bin/` paths were
  deleted in the span since `v0.25.0`, and that gate would have blocked all of
  them.
- **What an adopter is actually owed** is the declaration, because their
  residue is a script of their own that called the tool, and phase B's battery
  cannot see that script. `SPEC-release-change-declared.md` delta 3's class-R arm
  holds exactly that declaration in this repo. So the shape costs no new
  mechanism, and the entry's "accidental" behavior becomes stated behavior with
  an oracle behind it.

### (2) The release-sweep contract is unchanged, and says why in no new place

The sweep's "no marker rides into the next major undispositioned" constraint
stays anchored to the marker roster. Kit tools are outside that roster by delta
1, so an empty roster in this tree is no longer a gap for tools. No sweep text
moves. **{mechanical}**

## Producers and consumers

**The rule.** Producer: the §Versioning text. Consumers:
- the session deriving the bump at RELEASING.md §The procedure step 2, which
  reads §Versioning's criteria;
- an adopter reading the note, for whom the Behavior-changes bullet is the
  worklist item.

`check-release-bump` reads section presence rather than the major criteria,
which stay judgment ("a decommission is a semantic fact no section grammar
carries"). So no gate's verdict changes.

**The declaration it relies on.** Producer: the session that deletes the tool,
or the session that discovers the omission (`SPEC-release-change-declared.md`
delta 2). Its consumer and its red are that amendment's class R.

**Point 5.** No corpus is narrowed. **Point 6.** No corpus-wide obligation is
created: delta 1 is a rule, and the declarations it depends on are enumerated
by the sibling amendment's delta 4.

## Existing sections updated

- `docs/install.md` §Versioning, the Major bullet, with the replacement text
  below. (delta 1)
<!-- update-target-exempt: read at authoring, needs no edit -->
- `docs/install.md` §The upgrade contract's honest limit on Behavior changes:
  read and left unchanged. A removed tool is one more bullet reconciled by
  reading, which that paragraph already covers.

### Replacement text

**docs/install.md §Versioning, the Major bullet. Not yet applied.** Replace

> Majors are where the deprecation promises come due — the release-sweep
>   constraint that no marker rides into the next major undispositioned binds
>   here.

with

> Majors are where the deprecation promises come due — the release-sweep
>   constraint that no marker rides into the next major undispositioned binds
>   here. A kit `bin/` tool is not a deprecation-marked surface: deleting one is
>   not a decommission. It rides a minor and is declared by path under Behavior
>   changes, where a script of yours that called it finds its worklist item. In
>   this repository `check-release-change-declared` holds that declaration.

## Retired spellings

- None — no name, path or token is retired; delta 1 adds a sentence and delta 2
  moves nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the replacement sentence
      states the rule, and this amendment carries its grounds.
- [ ] **Merged with no information lost** — the Major bullet re-phrased in place.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap discovered during the work is filed.
