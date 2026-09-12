# SPEC amendment: knob-token

A consumer-configured corpus reaches a gate's **runtime** through a knob and its
**trigger** through a hand-written literal, and nothing holds the two together.
This amendment gives the trigger the knob's name, so one roster feeds both.

## What changes

### (1) A `couples=` token may name a knob rather than a literal glob

The field gains a second special token, `knob:<NAME>`, which expands to the
members of the kit knob `<NAME>` {design-bearing}.

Today the field admits one special token, `kit:<glob>`, and everything else
"stays literal". A gate whose scanned corpus *is* a knob's value therefore has no
way to declare its trigger without transcribing the knob's value — and where that
value is the consumer's, transcribing it puts consumer content in a kit literal,
which the provenance seam refuses. `knob:<NAME>` is the missing spelling: the kit
descriptor names the **knob**, and the value stays the consumer's.

**The spelling is `knob:`, and the ground is the tree's own word.** The config
bridge already spells these `GATE_SDK_KNOB_<NAME>` on every emitted invocation, so
the prefix reuses the vocabulary the mechanism already uses. `env:` was refused:
the value arrives through the config bridge rather than the ambient environment,
and naming it `env:` would invite a reader to set it in a shell and expect a
verdict change.

**The token is added beside a descriptor's literal globs, never in place of
them**, and that is a correctness requirement rather than a courtesy. A knob's
value and the kit's default corpus are **alternatives** at runtime, not a union —
canon-kit's `manifest_files()` globs with the knob when it is non-empty and
derives its default set only when it is empty — so a descriptor that dropped its
literals would lose the trigger for the default branch. The union over-triggers by
exactly the branch not taken, which §The `# graph:` manifest already sanctions in
the existing token's own words: *"Expansion over-approximates by design — a kit is
coupled even where the gate's subject is narrower, so an extra trigger runs a
green gate while a missing one would skip a red."*

The union also keeps this amendment clear of an unsettled question. The kit's
literal `*SPEC*.md` is **wider** than the knob member `*/SPEC.md` under the
slash-spanning trigger matcher and **narrower** under the coverage reader's
segment-wise one, so a replacing token would narrow or widen depending on which
reader looked — which is exactly the open design
`couples-glob-semantics-unowned` owns. A union is monotone-widening under either
matcher, so this unit neighbours that one and settles nothing on its behalf.

**Admissibility is checkable, and the check is what makes the token honest.**
`knob:<NAME>` on a gate's descriptor is admissible only where `<NAME>` is one of
the knobs that gate declares — the set the binary answers `--knobs <name>` with.
Because the crate declares only the knobs its own code reads (§lib/gate.sh), an
admissible token is *provably* a corpus the gate actually reads, and the hard
authoring rule stops being an honour-system duty for this class. A token naming an
undeclared knob is a finding, not a wider trigger.

**What the token does NOT do is add a matcher.** Each reader expands the token to
the knob's members and then applies the matching discipline it already applies.
Stated because the opposite reading is available and would be wrong: the three
incompatible semantics this field already carries are untouched here.

### (2) The token resolves at bridge time, through a union sentinel

Resolution is the shell's, beside the existing expansion, and the knob values the
expanding readers need reach them through a derived sentinel {design-bearing}.

The obstacle, stated first because the obvious design founders on it: the readers
that expand a descriptor's `couples=` — `check-graph`, `run-gates --for`,
`check-reads-couples` — expand **another member's** field, and the bridge resolves
only the fixed knob set a member itself declares. `check-graph` does not declare
`CANON_KIT_MANIFEST_FILES`, so a naive `knob:` token is unresolvable exactly where
it must resolve.

That is the same obstacle §check-reads-couples already solved for its run-time-named
filter knobs, and the solution is reused rather than re-invented: a **union
sentinel** the registry expands to every knob name the corpus names, derived from
the surface the names are written on, so a newly written token cannot be forgotten.
Here the surface is the descriptor corpus itself, which `gate_command` already
reads. The sentinel stays a crate-internal carrier: `--knobs` still prints one knob
name per line and no descriptor field moves.

Resolution order, stated because an unstated order is a per-reader order:

1. **`knob:` expands first**, then `kit:` expands over the result, so a knob whose
   member is spelled `kit:<glob>` composes.
2. **One pass each, and a knob member that is itself a `knob:` token is a
   refusal** — which bounds expansion without a cycle detector.
3. **Fail-closed is inherited, not authored.** The bridge's existing contract
   governs: an absent bridge variable is an error rather than a fallback, and an
   empty one is a resolved-empty set. So a `knob:` naming a knob nothing bridged is
   exit 2, and a declared knob a consumer set empty expands to nothing — correct,
   because the gate then scans nothing either.
4. **A knob member carrying a comma or whitespace is a refusal at expansion
   time.** The bridge joins members with a tab, while `couples=` is
   comma-separated and is read by a parser that splits the manifest line on
   unquoted whitespace. Such a member is unrepresentable after expansion, and the
   failure mode it would otherwise take is a silently truncated trigger.

**The refused alternative, recorded so a later reader does not re-propose it:**
pre-expanding the token in the shell so the binary never sees it. It fails because
`check-graph` parses the raw manifest line in the crate, so the binary *does* see
the raw token; the only way to hide it would be to generate the descriptor, making
a hand-authored field a generated surface with a second source.

**The freshness gate is inherited too.** The generated hooks already bake each
ported member's resolved knob values, and the projections roster already records
that a kit-config edit stales them. A knob edit therefore already stales the hook
that now also triggers on it, and `check-graph` already prints the regeneration
command on red. This amendment adds no freshness mechanism.

### (3) The amendment-body glob validator admits the new prefix

`valid_glob_token` strips at most one leading `kit:` and then rejects `:`, so it
must learn the second prefix or no amendment can propose one {mechanical}.

`check-graph` holds a `# graph:` manifest embedded in an amendment body to the glob
grammar, admitting only `[A-Za-z0-9._*?/-]` after one optional `kit:`. The
validator's prefix set becomes the same closed set the resolvers recognise, read
from one place rather than spelled twice.

**This amendment is itself the first case, and works around it deliberately.** No
fenced manifest line in this file carries the new token, because the validator
would red it at this amendment's own authoring commit — before delta 1 exists. The
token is shown in prose and in bare `couples=` values instead. Recorded so a later
reader does not read the absent example as an omission.

### (4) The eleven canon-kit manifest-corpus descriptors name their corpus knob

Each of the eleven gates whose scanned corpus is `CANON_KIT_MANIFEST_FILES` gains
`knob:CANON_KIT_MANIFEST_FILES` in its `couples=` {mechanical}.

The members, each keeping every literal token it carries today:
`check-manifest-count`, `check-prose-enum`, `check-manifest-temporal`,
`check-install-claim`, `check-payload-claim`, `check-docs-cmd`, `check-md-refs`,
`check-tracking-claim`, `check-knob-citation`, `check-spec-fence-balance`,
`check-spec-pointer`.

**The set's shape is more precise than the filing's summary, and the difference
matters to the edit.** Four of the eleven carry exactly
`couples=*SPEC*.md,*README.md,CLAUDE.md` (`check-manifest-count`,
`check-manifest-temporal`, `check-md-refs`, `check-knob-citation`); the other seven
are supersets of those three tokens, carrying further couples of their own —
`check-spec-fence-balance` adds `TASK-QUEUE.md`, `check-spec-pointer` adds five more.
So the edit appends one token per descriptor and rewrites none, and a sweep written
against "the three-token triple" would miss seven of its targets.

## Producers and consumers

- **The `knob:<NAME>` token** (delta 1).
  - *Producer:* a gate author writing it into a `.gate` descriptor's manifest —
    and delta 4 is the first deployed configuration that does, so the producer is
    reached in this tree rather than only in fixtures.
  - *Consumers:* the four readers that recognise a couples prefix today, each of
    which must learn the second one or diverge from the others —
    `gate_expand_couples_var` in `gate-sdk/lib/gate.sh` (feeding `gen-pre-commit`),
    `registry::expand_couples` (feeding `check-graph`, the graph emitter,
    `check-gate-substrate-parity` assertion G, `port-blockers`, and
    `run-gates --for`), `check-reads-couples`' own second copy of that expansion,
    and `check-graph`'s `valid_glob_token` (delta 3).
  - *Named reader of the admissibility rule:* `check-graph`'s live-registry
    manifest loop, at the per-member validation pass, against the binary's
    `--knobs <name>` answer.
- **The bridged knob-name union** (delta 2).
  - *Producer:* `gate_command`'s descriptor scan, at the point it already resolves
    a member's declared knobs into `GATE_SDK_KNOB_<NAME>` assignments.
  - *Consumer:* each expanding reader's `knob:` branch, at expansion time, reading
    `GATE_SDK_KNOB_<NAME>` by the existing array contract.
  - *Red condition of the producer's own failure:* an absent assignment is exit 2
    naming the knob, never an empty expansion — because an empty expansion is a
    lost trigger, the defect class this amendment closes.
- **The widened triggers** (delta 4).
  - *Producer:* `gen-pre-commit.sh`, emitting each member's `staged_matches` guard
    from the expanded trigger set.
  - *Consumer:* the generated `pre-commit` hook, at the staged-path test, which
    currently carries the consumer's widened corpus as a bridged knob value on the
    very invocation whose guard ignores it.
  - *Reader of the result:* `check-graph`'s couples-to-hook parity assertion, which
    compares both sides through the same expansion and so agrees by construction.
- **No delta narrows a corpus** (causal-completeness point 5), so no reader needs
  its red condition enumerated on that axis, and the claim is checked rather than
  assumed: delta 1 adds tokens to a trigger set, delta 3 admits a character a
  validator rejected, and delta 4 appends without rewriting. The one reader that
  could be thought to narrow is `check-kit-enum`, which reads the **raw**
  unexpanded field looking for the literal `kit:` spelling to catch a
  hand-enumerated per-kit list; a `knob:` token is neither that list nor that
  spelling, so its verdict is unchanged and it needs no edit.

## Existing sections updated

- `gate-sdk/SPEC.md` §The `# graph:` manifest, the manifest grammar block and the
  `couples=` bullet (deltas 1 and 2). The bullet gains the second special token,
  its union rule, its admissibility rule, and the resolution order; its sentence
  *"One token is special"* is re-phrased in place for two.
- `gate-sdk/SPEC.md` §The `# graph:` manifest, the same bullet's claim that the
  shared reader in `lib/gate.sh` *"feeds `gen-pre-commit`, `check-graph` and
  `run-gates --for`, so emitter, checker and selector cannot desync"` (delta 1).
  That sentence is **stale for the crate and this delta is where it is corrected**:
  the prefix is recognised by exact literal match in four independent places with
  no shared routine, and the token set is what they must agree on. Not yet applied.
- `gate-sdk/SPEC.md` §lib/gate.sh, the config-bridge passage owning
  `GATE_SDK_KNOB_<NAME>`, its absent-is-an-error / empty-is-resolved-empty rule, and
  the `--knobs` derivation (delta 2). It gains the knob-name union as a second
  sentinel case and cites §check-reads-couples as the precedent it reuses.
- `gate-sdk/SPEC.md` §check-graph (deltas 1, 2 and 3). Three additions: the
  amendment-body glob grammar's prefix set, the live-registry admissibility
  assertion against `--knobs`, and a note that couples-to-hook parity is unaffected
  because both operands pass through one expansion.
- `gate-sdk/SPEC.md` §check-reads-couples (delta 1). Its private copy of the
  expansion is a named consumer of the new token, and the section records that the
  copy must move with the shared one until the two are merged.
- `gate-sdk/SPEC.md` §gen-pre-commit (deltas 2 and 4). The emitted trigger set is
  now knob-derived for eleven members, so the section states that a knob edit
  stales the hook through the trigger as well as through the baked invocation.
- `canon-kit/SPEC.md` §Layout and configuration, the `CANON_KIT_MANIFEST_FILES`
  bullet (delta 4). It gains one clause: widening the array widens the eleven
  gates' **triggers** as well as their scanned corpus, which before this change it
  did not. Not yet applied.
- `scripts/git-hooks/pre-commit` and `docs/check-graph.html` (all deltas) — the
  generated projections. Regenerate with `bash gate-sdk/bin/gen-pre-commit.sh --write`,
  then `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`.
- `docs/gate-sdk/SPEC.md` and `docs/canon-kit/SPEC.md` (all deltas) — the generated
  on-site mirror; regenerate with
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Retired spellings

- None — no delta removes a name. `kit:` keeps its spelling and its meaning beside
  a sibling, `couples=` keeps its grammar with one more admitted token form, and
  delta 4 appends to eleven descriptors without rewriting any token they carry.

## Definition of Done

- [ ] **Causal completeness** — every new state, event and interface has a named,
      reachable producer and a named consumer; the token's one new field has a
      named reader at a named transition.
- [ ] **All four prefix readers move together** — the token is recognised by
      `gate_expand_couples_var`, `registry::expand_couples`,
      `check-reads-couples`' own copy, and `valid_glob_token`, in one commit.
      A partial landing is not a degraded feature: an unrecognised token falls
      through as an inert literal glob and the trigger is silently lost, which is
      the defect this unit closes.
- [ ] **The widened trigger is verified by oracle, not by reading the diff** —
      `bash gate-sdk/bin/run-gates.sh --for TRAJECTORY.md` selects the citation,
      temporal, link and count gates it selects none of today, and the same for
      `RELEASING.md` and `CONTRIBUTING.md`. The filing's measurement is the
      baseline: eight gates selected, of which two read the file.
- [ ] **Instruction surfaces: instruction only** — no grounds land in a template
      or descriptor; the grounds are this amendment's and the SPEC sections' above.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section, not appended, and the stale shared-reader
      sentence corrected rather than left beside its correction.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`). The none-remain half is discharged at
      the iteration, since a sibling amendment is in flight for this component.
- [ ] **Queue transition at the merging build batch, not at close** — the drain
      stage is `validate`, whose entry refuses a non-empty active queue, so the
      batch that merges this amendment moves its entry in the same commit.
- [ ] **Removals propagated** — `## Retired spellings` above holds, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — a cross-component gap found during the work is resolved
      that session, not deferred.
