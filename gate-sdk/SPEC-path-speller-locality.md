# SPEC amendment: path-speller-locality

Clause two of §The path-dialect contract is **unenforced and, as written,
false**. This amendment restates what the contract asserts so that a gate can
hold it, and the restatement is a **narrowing** — which is why the unit that
carries it stops being debt. Sited in `gate-sdk/` because §The path-dialect
contract and its §Porting to Rust clause are that SPEC's.

## What changes

### (1) The false universal is replaced by the property that actually holds

§Porting to Rust does not retire dialect exposure asserts, verbatim: *"No module
outside it composes a path, in either style."* That sentence is replaced.
**{design-bearing}**

**Measured, which is what makes this envelope work rather than calibration.**
`git grep -c '\.join(' -- native/src` returns **137 files**, and modules outside
`walk.rs` compose real filesystem paths — `proc.rs`'s program probe and its
capture files, `marker.rs`'s scratch directory, `installer/init.rs`'s
`root.join(p)`. One counter-example falsifies a universal, and there are many.
*Carried rather than hidden:* `.join(` also matches `slice::join`, so 137
overstates **path** composition; the falsification does not depend on the count,
only on the named modules.

**The narrowing, and why it is envelope work.** What the sentence should have
said is not a smaller version of itself — it is a different assertion, over
*which spellings escape into a compared or reported value* rather than over
*which modules call a constructor*. A contract that asserts less than it did is
an envelope change by definition, so restating it is not build's to do
(escalate-on-discovery fires there) and is recorded here.

### (2) The clause is stated over the three path primitives, not over one idiom

The replacement names `walk.rs` as the sole holder of the three operations that
decide a path question **from a path's text**: testing absoluteness, composing a
prefix to test containment, and joining a root onto a segment. A module outside
`walk.rs` reaches each through a named `walk` helper rather than re-spelling it.
**{design-bearing}**

**This form is chosen over the narrower "the prefix-composition idiom is local to
`walk.rs`", and the reason is a defect the narrower form cannot see.** Judged at
this stage, `native/src/spec.rs`:75 and :89 test absoluteness with
`p.starts_with('/')`. §The crate's crosser already rules that test wrong in its
own words — a separator-rooted path and a drive-rooted one are both absolute, and
a leading-slash test answers false on the second, sending an already-absolute root
down the join-onto-cwd arm — and `walk::path_root` is the crate's single owner of
the question, handling `\`-rooted and drive-rooted alike. So a drive-rooted kit
root reaches `spec.rs`:78 and is joined onto the cwd (`D:/w/repo/D:/w/kit`);
`_spec_prune_kit_roots` then matches no root and **prunes nothing, silently
over-including**. It is conditional — it needs an absolute or off-drive
`GATE_SDK_KIT_DIRS` — but it is the declared contract's own named failure, and
**neither the prefix idiom nor any 12-site roster of it contains those two
lines**. A clause stated over one idiom would have left them outside the contract
it is the instance of.

### (3) The contract binds a filesystem location, and another namespace is declared out at its site

A `/`-separated value that names something other than a filesystem location is
outside the contract, and says so at the site in the spelling delta 4 rules,
rather than leaving a gate to infer a namespace from a string.
**{design-bearing}**

**This is the corpus's finding, and it is why locality needs an exemption shape
at all.** The twelve sites of the prefix idiom, judged whole at this stage rather
than sampled — the roster is
`git grep -n 'starts_with(&format!("{}/"' -- native/src`, run rather than copied:

- **One is lawful** — `walk.rs`:25, inside the speller.
- **Two are not filesystem paths at all** — `emit/roadmap.rs`:53 compares a
  roadmap tag's `horizon/track` field, and `gates/packed_links.rs`:61 compares a
  markdown link target against a withhold-knob member. A locality assertion reds
  both, for no defect and with no fix available: routing a tag field through a
  path speller is worse code, not better.
- **Five compare git-relative operands** — `gates/docs_link_convention.rs`:195,
  `gates/door_binding.rs`:99, `gates/gate_substrate_parity.rs`:806,
  `gates/mod.rs`:2365, `installer/init.rs`:387. These are **not** false
  positives: routing each through a named `walk` helper is
  correct-by-construction and costs nothing, so locality converts five
  provenance arguments into five call sites.
- **Four are host-absolute** — `emit/scratch_run.rs`:38 (both operands crossed,
  self-attested at the site), `spec.rs`:109 and :120 (crossed once through
  `walk::cwd()`, never re-normalized), and `installer/uninstall.rs`:188, whose
  weak operand is a **raw `git config --get core.hooksPath` value that never
  crossed at all** — so an adopter who spelled that config with backslashes gets
  a silently-skipped `core.hooksPath` unset on uninstall.

So the exemption is needed for **two of twelve**, in one namespace class, and is
declared rather than inferred because the namespace a `/`-separated string names
is not decidable from its text.

### (4) The exemption's spelling and its mandatory reason

`// path-dialect-exempt: <reason>` on the site's line or the one above, the
shared exempt window every other valve in this tree rides; the reason is
mandatory and an empty one is malformed. **{mechanical}**

The token is minted here rather than reusing `comment-tier-exempt:`, because the
two answer different questions — one says *this comment earns its place*, the
other says *this comparison is not about a filesystem* — and a reader who finds
one token doing both jobs cannot tell which claim a site is making.

### (5) The gap bullet's original discriminator does not survive, and its refusal is recorded

The filed premise offered a **host-absolute versus git-relative** discriminator.
It is dropped, not adjudicated. **{design-bearing}**

**Measured, rather than relayed:** it returns the same verdict for a shipped
defect and for correct code. The `toolfloor.rs` failure that broke both Windows
install-smoke legs was host-absolute; `emit/scratch_run.rs`:38 is host-absolute,
correct, and self-attested at the site. A predicate that cannot separate those
two is not a discriminator, so there is nothing for a later session to weigh and
the entry's remaining design question is answered by deletion rather than by
choice. Recorded because the bullet is on the record and a silent disappearance
would read as an oversight.

### (6) What the contract does **not** assert, stated so the next reader does not re-derive it

`Path::join` for a filesystem *operation* is lawful and ubiquitous, and the
clause says so. **{mechanical}**

The predicate is escape into a **reported or compared** value, never
construction. `root.join(p)` handed straight to `std::fs::read` crosses no
reader and is untouched; the same value rendered with `.display()` into a string
a gate prints or prefix-tests is the contract's subject. Stated because the
sentence being replaced said the opposite, and the first reader of the new clause
will arrive carrying the old one.

## Producers and consumers

**One name is minted: the comment directive `path-dialect-exempt:`.** Producer:
the author of a comparison whose operands are not filesystem paths. Consumer: the
gate delta 2's clause makes assertable — which this amendment **does not
author**, the entry's deliverable being the assertion and this amendment's being
the contract it asserts. **Roster-holding reader:**
`check-comment-tier` (canon-kit/SPEC.md §check-comment-tier) reds at an obliged
site on a directive its roster lacks, so the roster row and the directive are one
change. That row is an update target below; without it the two exempt sites red
the moment they are written.

**Producer and consumer of the clause itself.** The clause is read at authoring
and review time and by the assertion the paired entry builds; its enabling
configuration is the one already deployed, since `check-path-dialect` is
registered in `scripts/gates.list` today and the entry adds an assertion to it
rather than a new member.

**Point 5 — each reader's red condition, because delta 1 narrows a corpus.**
Replacing a universal with a narrower clause shrinks what the contract covers,
and the point binds rather than clearing by inspection:

- **`check-path-dialect`** reds on *finding a violating form*, never on finding
  none, and holds no count, floor or coverage arm. Monotone; a narrower contract
  can only remove findings. Probe:
  `native/target/release/checkwright-gates --reads check-path-dialect` and its
  `GIT_FLAGS` / `RUST_FORMS` vocabulary in `native/src/gates/path_dialect.rs`,
  which is a form scan over declared literals and carries no `pwd` form — so the
  narrowing touches no vocabulary member it currently matches.
- **`check-spec-pointer` and `check-md-refs`** read the *section name*, which
  does not change, so every `# spec:` directive and `§` citation aimed at
  §Porting to Rust does not retire dialect exposure keeps resolving.
- **No gate asserts a minimum, an exact count or a coverage floor over this
  section.** Probe: `git grep -n "path-dialect\|path_dialect" -- native/src scripts`.

**Point 6 — every member's satisfying value.** Delta 3's corpus is enumerable at
authoring time and is enumerated above, twelve of twelve. Each member's
satisfying value: `walk.rs`:25 is already satisfying; the two namespace sites
take a `path-dialect-exempt:` declaration; the five git-relative and the four
host-absolute sites take a named `walk` helper call. **No member has none**, so
the assertion is not narrowed past anything. `spec.rs`:75 and :89 are named as a
thirteenth and fourteenth site *outside* the idiom roster, and their satisfying
value is `walk::path_root`; they are listed because delta 2 rests on them.

## Existing sections updated

- **gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure** — the
  sentence *"No module outside it composes a path, in either style."* is
  **replaced**; the surrounding two-horns argument, the `walk.rs` string-composition
  history and the 2026-08-30 Windows measurement are kept unchanged, the
  replacement re-phrasing the clause they lead into rather than appending a
  qualification after it. (deltas 1, 2, 3 and 6)
- **gate-sdk/SPEC.md §The crate's crosser** — the bullet roster gains the
  statement that `walk::path_root` is the absoluteness test every module uses,
  re-phrased into the existing `path_root` sentence rather than added beneath it;
  the section already names it "the crate's single owner of absoluteness", so the
  edit makes an existing claim binding rather than making a new one. (delta 2)
- **gate-sdk/SPEC.md §check-path-dialect** — the gate's own section records that
  the clause is now assertable and names the exemption token its future arm must
  honour, without authoring the arm. (deltas 3 and 4)
- **canon-kit/SPEC.md §check-comment-tier** — the comment-directive roster gains
  the `path-dialect-exempt:` row. (delta 4)
- **`docs/gate-sdk/SPEC.md` and `docs/canon-kit/SPEC.md`** — generated mirrors of
  the two edited SPECs, regenerated by their emitter rather than edited.
  (all deltas)

### Replacement text

**gate-sdk/SPEC.md §Porting to Rust does not retire dialect exposure — the
closing sentences of the two-horns paragraph. Not yet applied.** Replace

> The two horns together are the reason walk.rs owns a `child` speller: it is the
> crate's sole path **producer** (§The crate's crosser) and, by the same rule and
> for this second reason, its sole path **speller**. No module outside it
> composes a path, in either style.

with

> The two horns together are the reason walk.rs owns a `child` speller: it is the
> crate's sole path **producer** (§The crate's crosser) and, by the same rule and
> for this second reason, its sole path **speller**. **What that obliges is
> locality of the three text-level primitives, not abstinence from
> construction.** Testing whether a path is absolute, composing a prefix to test
> containment, and joining a root onto a segment are `walk.rs`'s; a module
> outside it reaches each through a named `walk` helper — `path_root`,
> `normalize_abs`, `abs_against`, `child` — rather than re-spelling it. Building
> a `PathBuf` to *operate* on is untouched and ubiquitous: the subject is a
> spelling that **escapes into a value a reader prints, matches or
> prefix-tests**, which is where a `\` separator turns a wrong answer into a
> confident one. And the contract binds a value naming a **filesystem location**
> — a `/`-separated string in another namespace, a queue tag's `horizon/track`
> field or a markdown link target, is outside it and declares so at its site with
> `// path-dialect-exempt: <reason>`, because which namespace a string names is
> not decidable from its text.

## Retired spellings

- None — no delta retires a spelling. Delta 1 deletes one sentence of prose and
  the others re-phrase around it; no token, knob, arm, path or function name is
  displaced, and the one name minted (`path-dialect-exempt:`) had no predecessor.
  The **host-absolute/git-relative discriminator** delta 5 drops was never a
  spelling on any surface — it lived in a gap bullet as a proposal and is
  answered here, so there is nothing for a survivor scan to find.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds; points 5 and 6 are discharged in
      §Producers and consumers, 5 by naming each reader's red condition against
      delta 1's narrowing and 6 by enumerating all twelve corpus members plus the
      two outside the idiom that delta 2 rests on.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition
      or shim is touched; recorded rather than dropped so the check reads as run.
- [ ] **Merged with no information lost** — the two-horns argument, the
      `check-install-disposition` measurement and §The crate's crosser's bullet
      roster all survive; only the false universal is replaced.
- [ ] **Amendment deleted** — this file removed on merge; **none remain for the
      component** (`ls gate-sdk/SPEC-*.md`) — and this component carries a
      sibling, `gate-sdk/SPEC-front-end-residency.md`, so that half is discharged
      at the iteration and not at this commit (canon-kit/SPEC.md §Merging an
      amendment step 3). Every sibling amendment's filename citation of this file
      is repointed at the canonical section it merged into, per
      §check-amendment-queue arm (e).
- [ ] **Removals propagated** — declared negative above, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps filed to the gap inbox. Two are
      already visible and are the build's to dispose rather than to absorb:
      `spec.rs`:75/:89's leading-slash absoluteness test, and
      `installer/uninstall.rs`:188's never-crossed `core.hooksPath` operand.
