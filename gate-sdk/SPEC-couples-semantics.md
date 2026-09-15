# SPEC amendment: couples-semantics

Queue entry: `couples-glob-semantics-unowned`, selected for `couples-field-semantics` as the ruling its
lead unit waits on (operator direction, 2026-09-15, lead-relayed).

**This amendment lands first.** `SPEC-fallback-roots.md`, `SPEC-packed-knob-projection.md` and
canon-kit's `SPEC-depth-bound-globs.md` each read the semantics ruled here and merge after it.

## The ruling

**`couples=` has one semantics, the trigger's, and `trigger=` shares it.** Each token, after
expansion, is a bash string pattern put to a repo-relative path as `[[ path == token ]]`: `*` and
`?` cross `/`, and `**` means no more than `*`. The coverage reader stops using its own matcher. The
entry offered two answers, one semantics with stated exceptions or a per-reader meaning declared per
reader, and this takes the first. Only one semantics is compatible with the tree:

- **The trigger's matcher is declared intent and cannot narrow.** gate-sdk/SPEC.md §Reading a
  `couples=` field's reach forbids quoting the hook's pattern operand, because doing so would
  break every trigger in the tree at once. A segment-wise semantics applied everywhere would turn
  every coupling that reaches below its glob's depth into a lost trigger. That is the fail-open
  direction.
- **Coverage only means something as a trigger question.** Couples exist to fire the hook on a
  tracked-path commit (§check-reads-couples). A read is covered exactly when an edit to it fires the
  gate. A narrower coverage matcher measures a property no consumer of the field has. It also
  demands an enumeration of tree depth that no kit literal can supply for an adopter's tree, which
  is the withdrawal `couples-dynamic-root-resolution` records.
- **A per-reader meaning keeps the hazard it declares.** The crate would still carry a
  component-wise couples matcher beside a slash-spanning one. A porting session reaching for "the"
  matcher would still flip a verdict with no gate saying which side it was on. Declaring the split
  documents that hazard and does not remove it.

**Probed, not reasoned.** A probe at this stage ran over this tree's tracked files (the survey
record carries its block for this iteration). It measured the fallback-branch walks of the 14 members `--emit reads-census` lists with `fallback`
grounds against each member's literal couples (knob tokens empty, `kit:` expanded over `gate-sdk`
and the `*-kit` roots, the prune set applied):

- **Under the segment-wise matcher**, 24 of 26 manifest reads and 395 of 403 comment-surface reads
  were uncovered.
- **Under the trigger matcher**, the manifest reads came to 0 uncovered and the comment-surface
  reads to 2: `installer/bin/checkwright.sh` and `installer/consumer-smoke/run-smoke.sh`. Those two
  are **genuine** under-couples. A `.sh` outside the kit roots and `scripts/` fires no trigger for
  `check-comment-tier`'s kit literals either, so the settled semantics finds a real miss the narrow
  matcher was hiding among 395 false ones.
- **The descriptors were already written for the trigger's semantics.** Tokens such as `*.rs`,
  `*.gate`, `*README.md` and `*SPEC*.md` mean one file at the root under the segment-wise reading and
  every depth under the trigger's.

**The one refused repair this ruling is not.** The queue records option (5), keying the coverage
matcher off the filter kind, as measured dead: it flipped `check-reads-couples`' `bad/` case and its
unit cases to exit 0. That option chose a matcher per walk while the field's meaning was open. This
ruling closes the meaning. The flipped case is an instance of a depth-only miss, which under the
settled semantics is not an under-couple, so asserting it would assert a false positive. The
central assertion, reads covered by couples, is unchanged, and delta 5 re-instances the case as a
miss the one semantics does red on.

## The seam

Kit mechanism only: the field's semantics, one crate matcher, the `knob:` expansion rule, the
coverage gate's text and fixtures, and gate-sdk prose. The consumer side is this repo's CLAUDE.md
resident line, which retires (delta 6). No consumer knob and no private rule content.

## What changes

### (1) The field's one semantics, stated where the grammar is {design-bearing}

gate-sdk/SPEC.md §The `# graph:` manifest, the `couples=` bullet. **Not yet applied.** The two
sentences from **The authoring rule is that globs never cross `/`** through *§Reading a `couples=`
field's reach.* are replaced by:

> **Every token, once expanded, is a bash string pattern over the repo-relative path** — the test
> `[[ path == token ]]`, in which `*` and `?` cross `/` and `**` is no more than `*` — and every
> reader that asks what a token reaches asks it through that one matcher, `trigger=` included. So
> `native/src/*.rs` couples `native/src/emit/mod.rs`, both as a trigger and as coverage, and a
> `kit:<glob>` token is one prefix whose reach is the pattern's. A gate that walks a directory
> recursively couples that recursion with a pattern whose string reach contains every path the walk
> reads; a sibling glob per depth is never needed, and never sufficient where it stops short of a
> prefix the walk reaches. The readers and what each asks: §Reading a `couples=` field's reach.

In the `knob:` rule's paragraph, the sentence ending **so it settles nothing on the unowned
glob-semantics question** loses that clause and ends at *already sanctions*.

### (2) One matcher, and the reader roster read against it {design-bearing}

gate-sdk/SPEC.md §Reading a `couples=` field's reach, whose heading stays because CLAUDE.md, the
gate-sdk README and `check-reads-couples`' help cite it. **Not yet applied.** The body is replaced
by:

> **The field has one matcher, and every reader that asks what a token reaches uses it.** The
> generated hook's `staged_matches` is spliced from `gate_staged_matches` in `lib/gate.sh`, whose
> `[[ "$f" == $pat ]]` leaves the pattern operand unquoted under a standing
> `# shellcheck disable=SC2053`: bash *string* matching, in which `*` spans `/`. **Do not "fix" the
> unquoting**: it is the semantics, and quoting it would break every trigger in the tree at once. The
> crate's readers call its port, one function, rather than each carrying a matcher:
>
> - **`run-gates --for`** selects the members a path triggers.
> - **`check-reads-couples`** asks whether a walk's tracked reads are covered, so its answer is
>   exactly *would an edit to this read fire the gate*.
> - **`check-gate-substrate-parity` assertion C** asks whether a member's couples reach a gate
>   declaration path.
>
> Three readers ask something else, and none is a second semantics. A `mode=staged` member's hook
> branch selects its positional arguments by git pathspec, whose default (non-`:(glob)`) form also
> lets `*` cross `/` (§run-gates). `check-graph` assertion B asks whether one *pattern* is contained
> in another rather than whether a path matches, through a four-branch predicate that is sound for
> this semantics and deliberately incomplete (§check-graph). The graph emitter and `port-blockers`
> print the field.
>
> **The filter field is not this field.** A `--reads` filter names its walker's discipline through
> its mandatory kind (§check-reads-couples), because it stands for a walk the reader cannot see.
> Couples name what fires a trigger. So a `glob:` filter's `**` is `walk::glob_files`' component
> globstar and a couples token's `**` is not. A knob serving as both is converted once, at
> expansion (§The `# graph:` manifest, the `knob:` rule).
>
> **Read the reach through the oracle, never off the field**, because `kit:` and `knob:` tokens hide
> their expansion: `run-gates.sh --for <path>` answers what a path triggers, and
> `check-reads-couples` answers what a gate's couples cover. The two now differ only in the question,
> not in the matcher.

### (3) One crate matcher for the field {mechanical}

`native/src/registry.rs` gains the field's matcher beside `expand_couples`. It is a named function
taking a path and one expanded token, and it is `walk::pattern_match` under the field's name, so a
reader reaches the field's semantics by name rather than by knowing which general matcher happens to
implement it. Its callers are `runner::staged_matches`, `check-gate-substrate-parity` assertion C
(`gate_substrate_parity.rs`, the `pattern_match` over `declpaths`) and `check-reads-couples`'
`cover_root`. `reads_couples.rs`' `path_matches_glob` and its comment are **deleted**. The
component-wise `glob_path_match` / `glob_walk` stay, because they serve the filter and prune fields
(delta 2). `runner::pathspec_matches` is untouched: git pathspec is its own mechanism, and its
comment already says so. One unit test in `registry.rs` pins the semantics on its own examples:
`native/src/*.rs` matches `native/src/emit/mod.rs`, `a/**/b.rs` does not match `a/b.rs`, and `?`
matches `/`. The function's name is build's calibration.

### (4) A `knob:` member expands to the pattern that covers it {design-bearing}

`registry::expand_couples`. A knob member is written in its **walker's** discipline and relative to
its **walk's root**: a `glob_files` component glob with globstar, a `find_named` basename, or a
`find_files` extension. It is never a repo-path string pattern, and the expander can see neither the
kind nor the root. So each member expands to its **covering pattern**. Every `**/` is collapsed to
`*`, then a `*` is prefixed unless the result already begins with one:

- `**/*.sh` becomes `**.sh` and covers `x.sh` and `a/b/x.sh`.
- `SPEC.md` becomes `*SPEC.md` and covers the basename at any depth.
- `templates/*.md` under a non-`.` root becomes `*templates/*.md`.

This is a superset for all three disciplines and any root, and that is its whole ground. Over-trigger
is the direction §The `# graph:` manifest already sanctions (*an extra trigger runs a green gate
while a missing one would skip a red*). Literal tokens are **not** converted: their author writes
them in the field's semantics directly. The existing refusals (a nested `knob:` member, a comma or
whitespace member) run before the conversion. **Not yet applied**, in the `knob:` rule's paragraph
beginning **What the token does not do is add a matcher**, replace its first two sentences with:

> **What the token adds is a conversion, not a matcher.** A knob's member is a pattern in its
> walker's discipline, relative to its walk's root, so each member expands to its covering string
> pattern — every `**/` collapsed to `*`, and a leading `*` added unless present — which contains
> every path the member can select under any discipline and any root; each reader then applies the
> field's one matcher.

No `mode=staged` member carries a `knob:` token (probed at this stage: the tracked descriptors
carrying one all omit `mode=`), so the widening changes triggers only and never a staged member's
positional arguments. Unit tests in `registry.rs` cover each example above, and cover a knob whose
member already begins with `*` expanding unchanged.

### (5) `check-reads-couples` reads the one semantics {design-bearing}

- **SPEC text.** gate-sdk/SPEC.md §check-reads-couples. **Not yet applied.**
  - In the shell-arm paragraph, *asserts every one matches at least one expanded couple under the
    manifest's own glob semantics — segments never cross `/`, so path and glob must share a segment
    count (a shallow one-level couple misses a file one level down, the check-shim-restatement
    bug)* becomes *asserts every one matches at least one expanded couple under the field's one
    matcher (§Reading a `couples=` field's reach)*.
  - The paragraph beginning **Over-demand is absorbed one of two ways** keeps its two absorptions
    but drops *never by weakening the glob semantics to pass a near-miss*. That clause presumes a
    narrower semantics a near-miss could be widened out of, and there is none left.
- **Help line.** `reads_couples.rs`' help line is replaced. It names the covering pattern: a glob
  whose string reach contains the uncovered path, such as `<dir>/*.ext` for a walk under `<dir>`.
  It keeps the regeneration and marker clauses and the §Reading a `couples=` field's reach
  pointer, and drops *globs never cross '/'* and *Never widen a glob to cross '/'*.
- **Fixture pair.** `bad/` is re-instanced as a miss the one semantics reds. Its manifest couples
  `corpus/sub/*.md` while its walk over `corpus` reads `corpus/top.md`, so `expect.txt` names
  `corpus/top.md`, and its header comment says so. `good/` couples `corpus/*.md` alone, and its
  comment states that one pattern covers both depths.
- **The test file.** In `gate-sdk/gate-tests/check-reads-couples.test.sh`, `MANIFEST_NARROW` /
  `MANIFEST_WIDE` and case B are re-instanced the same way. Case B becomes the prefix miss. A new
  case pins that a one-level pattern covers the deeper tracked file, which is the settled
  semantics' positive statement, so a later narrowing reds a named case rather than nothing.

### (6) The resident caution retires {mechanical}

CLAUDE.md's always-loaded line **Never read a `couples=` field's reach off the field** states that
two readers' glob semantics disagree, which delta 3 makes false. It is deleted, not reworded. The
residual advice (read a reach through `--for`, because tokens hide their expansion) is already
carried by the gate-sdk README pointer and by §Reading a `couples=` field's reach. Under
Load-trigger residency a descriptor author reaches it there. The gate-sdk README pointer
(`gate-sdk/README.md`, *A manifest's trigger reach is read with `--for`, never off the field*) stays
true and is unchanged.

### (7) `check-graph`'s section stops calling the question open {mechanical}

gate-sdk/SPEC.md §check-graph, the paragraph beginning **Assertion B's coverage predicate**.
**Not yet applied.**

- *it is the `couples=` field's third reader and it invokes no glob matcher at all* becomes *it
  asks whether a couple pattern is contained in a trigger pattern, which no path matcher answers*.
- *Neither of the crate's two matchers is this predicate — the component-wise one requires equal
  segment counts and the slash-spanning one is branch three alone —* becomes *The field's matcher
  is branch three alone, and the component-wise filter matcher is not this field's*.
- The closing sentences from *This closes the **port's** exposure* are replaced by *The predicate
  is sound for the field's one semantics (§Reading a `couples=` field's reach): a `*.<ext>` trigger
  contains every pattern ending in that suffix only because `*` crosses `/`.*

## Producers and consumers

- **The field's matcher (delta 3).** Its producer is the new registry function. Its consumers are
  `runner::staged_matches` (`run-gates --for`), `check-gate-substrate-parity` assertion C and
  `check-reads-couples`' `cover_root`, each at its per-token test. The generated hook consumes the
  same semantics through the spliced `gate_staged_matches`, which already implements it, and whose
  splice §gen-pre-commit already holds.
- **The covering conversion (delta 4).** Its producer is `registry::expand_couples`, on every
  `knob:` member. It is consumed by every `expand_couples` caller: hook emission, `check-graph`'s
  manifest loop and assertion D, the graph emitter, `run-gates --for`, assertion C, `port-blockers`
  and `check-reads-couples`. The conversion adds no field and no knob.
- **Readers' red conditions (point 5).** Delta 3 lets the coverage reader accept more, so its own
  finding set shrinks monotonically. Delta 4 widens triggers. Neither is a narrowing, but four
  readers turn red by construction and must be regenerated or re-instanced in the same commit:
  - `check-graph` assertion D reds on a committed hook whose `knob:` expansions predate delta 4.
  - Assertion E reds on the stale coupling-graph artifact.
  - `check-reads-couples`' `bad/` case and test-file case B red because they expect a finding that
    no longer exists (delta 5).
  - `check-gate-substrate-parity` assertion C reds on any member a converted `knob:` member newly
    makes substrate-sensitive without a recorded disposition. None was found by reasoning, and build
    reads the gate's verdict rather than trusting that.
  - Docs-mirror freshness reds on `docs/gate-sdk/SPEC.md` until regenerated.

## Existing sections updated

Roster probe: `git grep -n "field's reach"` and
`git grep -n "never cross\|segment-wise\|segment count\|path_matches_glob"` over the tracked tree,
plus a read of every `"couples"` reader the scope survey block names (its witness re-run at this
stage).

- `gate-sdk/SPEC.md` — §The `# graph:` manifest's `couples=` bullet and `knob:` paragraph (deltas 1
  and 4), §Reading a `couples=` field's reach (delta 2), §check-reads-couples (delta 5), §check-graph
  (delta 7).
- `native/src/registry.rs` — the field's matcher and the covering conversion, with their tests
  (deltas 3 and 4).
- `native/src/gates/reads_couples.rs` — `path_matches_glob` deleted, `cover_root` calls the field's
  matcher, help line (deltas 3 and 5).
- `native/src/runner.rs` — `staged_matches` calls the field's matcher (delta 3).
- `native/src/gates/gate_substrate_parity.rs` — assertion C calls the field's matcher (delta 3).
- `gate-sdk/gate-tests/check-reads-couples/bad/sandbox-gate.sh` — re-instanced miss (delta 5).
- `gate-sdk/gate-tests/check-reads-couples/bad/expect.txt` — names `corpus/top.md` (delta 5).
- `gate-sdk/gate-tests/check-reads-couples/good/sandbox-gate.sh` — one covering pattern (delta 5).
- `gate-sdk/gate-tests/check-reads-couples.test.sh` — case B re-instanced, slash-span case added
  (delta 5).
- `CLAUDE.md` — the couples caution line deleted (delta 6).
- `scripts/git-hooks/pre-commit` — regenerated for the converted `knob:` expansions (delta 4).
- `docs/check-graph.html` — the coupling-graph artifact assertion E compares, regenerated if any
  edge it draws carries a converted `knob:` member (delta 4).
- `docs/gate-sdk/SPEC.md` — the generated mirror, regenerated (all deltas).

## Retired spellings

- `path_matches_glob` — the coverage reader's own couples matcher, deleted (delta 3).
- `globs never cross` — the authoring rule's spelling, which the one semantics makes false (deltas 1
  and 5).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition
      or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
