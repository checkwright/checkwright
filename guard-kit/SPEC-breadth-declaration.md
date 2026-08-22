# SPEC amendment: intended-breadth declaration

Closes `settings-allow-intended-breadth-declaration`. `compare-settings-allow`
offers the operator two dispositions over an over-broad local glob — narrow it,
or record that the breadth is intended — and ships a mechanism for one. Only
narrowing is expressible, so a glob ruled intended re-reports at every close and
the ruling survives in a session's memory or a commit message, which
spec-over-precedent says is not ground truth.

The operator ruled the deliverable on 2026-08-20: **build it, glob-plus-reason,
the committed file only.** This amendment authors that ruling; it does not
re-decide it.

## What the ruling's two clauses mean, settled here so no reader re-derives them

The entry states both of its shape questions are answered and only the authoring
remained. The two answers bind different things and the reading is recorded
because the second sentence of the ruling admits a wider one.

- **glob-plus-reason** — a declaration is a pair, not a bare list. A bare list
  re-loses the reason it exists to keep.
- **the committed file only** — this fixes **where a declaration lives**: the
  tracked config the consumer commits, never a per-clone config overlay. It does
  **not** widen the breadth question's corpus. The corpus is owned by
  guard-kit/SPEC.md §compare-settings-allow, which fixes the report's subject as
  the **local** glob ("the report names the local glob and the one probe that
  witnesses its breadth"), and TRAJECTORY.md §The closed rulings hands the
  disposition pair to that section by name rather than restating it. So the
  reading that the declaration attaches to *committed* settings globs, and that
  the breadth question grows a second corpus, is **refused**: it is a wider
  assertion than the ruled one, and the owner doc already answers the corpus
  question.
- The ruling's trailing clause — *overlay globs stay per-clone and keep
  re-reporting, since they were never ruled* — is therefore about **undeclared**
  overlay globs: an over-broad local glob keeps re-reporting until a **committed**
  declaration rules it, and a per-clone declaration cannot silence one. That is
  the whole content of "the committed file only", and it is what makes the
  declaration durable rather than a local mute switch.

## What changes

### (1) `GUARD_KIT_BREADTH_DECLARED` — the declaration knob

A new consumer-config knob beside `GUARD_KIT_BREADTH_PROBES`, **default empty**,
an associative array whose **key is a permission-rule string** and whose **value
is the reason that breadth was ruled intended**. **Design-bearing** — the pair
shape, the key's exactness and the empty default are the amendment's load-bearing
choices.

```bash
declare -A GUARD_KIT_BREADTH_DECLARED=(
    ["<permission-rule string>"]="<why this breadth was ruled intended>"
)
```

The kit ships **no** default declarations, for the same reason it ships no
default probes: every string naming a command is the consumer's vocabulary, never
the kit's (CLAUDE.md §The provenance seam). The knob is the mechanism; the
declarations are consumer rule content.

**Associative rather than a delimited indexed array, and the choice is not
cosmetic.** A permission rule may contain any character a command may contain, so
every single-character separator an indexed array would need (`|`, `::`, a tab)
is a character a legitimate rule can carry, and the knob would ship a grammar
that cannot express part of its own subject. An associative key holds the rule
verbatim. The idiom is already precedented in this tree — `gate-sdk`'s config
bridge carries associative knobs and its probe test covers tab- and
newline-bearing keys and values — so it needs no new mechanism.

**Report order does not depend on the array's iteration order.** The tool walks
the settings file's own allow list, as it does today, and looks each entry up in
the map; the map is never iterated. So the report stays deterministic and the
associative array's unordered iteration never reaches output.

### (2) The declaration match is exact-string, never glob

An entry is declared when the **exact** local allow-rule string is a key of
`GUARD_KIT_BREADTH_DECLARED`. **Design-bearing**, and the alternative is
explicitly refused: routing the lookup through `guard_allow_match` — the
kit's one matcher, which the redundancy and breadth questions both use — would
let one declaration silence globs the operator's ruling never named, which is a
declaration widening itself. A durable ruling about intended breadth is a ruling
about **one** glob, so the lookup is `${GUARD_KIT_BREADTH_DECLARED[$entry]+set}`
and nothing else.

The consequence is stated rather than left to be discovered: narrowing a declared
glob, or re-spelling it, drops its declaration and the entry re-reports. That is
correct — the ruling was taken on the old string — and it is the mechanism that
keeps a declaration from outliving the glob it ruled.

### (3) The breadth report gains a declared section

**Design-bearing.** A declared glob is **not dropped** from the report. It moves
out of the narrowing-candidate set and into a **declared** subsection printed with
its reason, so the record has a reader at the close-triage step the tool exists to
serve. Dropping it would make the knob a silencer whose contents nobody ever
reads, and a field with no named reader is removed rather than shipped
(canon-kit/SPEC.md §The causal-completeness check, point 4).

The breadth pass therefore partitions the over-broad set in two:

- **narrowing candidates** — over-broad and undeclared. Printed as today, with
  the same two-disposition help text.
- **declared intended** — over-broad and declared. Printed as
  `<glob>  ⊇  <probe>  — <reason>`, under its own heading, with help text saying
  the breadth was ruled and naming the config file as where the ruling lives.

An over-broad set that is entirely declared prints the declared section and
**no** narrowing section, rather than the clean-report line: printing *no
over-broad local entries* there would be false. Conversely, with no over-broad
entries at all the report is unchanged from today.

### (4) `--count` keeps two numbers, and the second one narrows

**Design-bearing**, because it is a behavior change to a machine-read output.
`--count` emits redundancy first and breadth second; the breadth number becomes
the count of **narrowing candidates** — over-broad and undeclared — rather than
of all over-broad entries. The count's one purpose is *how much is outstanding*,
and a declared entry is not outstanding. A third number for the declared count is
**refused**: no reader needs it, and the two-number line is a shape a consumer may
already parse.

### (5) The empty-knob behavior is exactly today's

**Mechanical.** With `GUARD_KIT_BREADTH_DECLARED` empty — the default, and the
shipped state for any consumer who declares nothing — the partition is trivial,
the declared section is omitted entirely, and every byte of output matches the
pre-amendment tool. The empty default is not a special case in the code; it falls
out of the lookup.

Note the asymmetry with `GUARD_KIT_BREADTH_PROBES`, which is deliberate: an empty
probe set omits the whole breadth section, because silence there could be misread
as coverage. An empty declaration set omits only the declared subsection, and no
coverage claim is available to misread — an absent declared section says exactly
that nothing was declared.

### (6) This repo declares nothing, and that is the shipped state

**Mechanical.** `scripts/guard-config.sh` gains no declaration in this unit.
Nothing over-broad re-reports here today, so a declaration written now would be a
record with no subject. The knob is bought for the next intended-broad glob — and
for `guard-grant-review`, which is the first work that will have a ruling to
record.

**What this buys `guard-grant-review`, stated exactly and without overclaiming.**
That entry's sequencing note says the *keep-the-breadth* half has no durable home
until this knob ships. This knob is that home: a glob the re-derivation rules safe
and keeps broad is recorded here as glob-plus-reason, in the tracked config, where
spec-over-precedent can find it. What it does **not** do is make the tool report
on the committed settings file — that corpus question is answered above, and a
kept **committed** glob was never in the breadth report to begin with. A
declaration whose glob names a committed rule is therefore a durable record that
the report itself does not read; that is honest and is the limit named below.

## The honest limits

**A declaration can outlive its subject and nothing notices.** The knob records a
ruling; it does not verify that the glob it names is still in
`.claude/settings.local.json`, still over-broad, or still real. A stale
declaration is silent. A stale-declaration report was weighed and refused for this
unit: it would print every declaration naming a committed glob — the shape
`guard-grant-review` is about to produce — as stale, which is noise rather than a
finding. The gap is filed rather than flagged-and-skipped; see below.

**The tool stays advisory, and the placement ruling is untouched.** Nothing here
makes a gate of `compare-settings-allow`, and guard-kit/SPEC.md
§compare-settings-allow's ruling on why — a gitignored per-machine subject and an
operator-intent-dependent verdict — is unchanged and unweakened by a knob that
records exactly that intent.

**Two declarations cannot conflict.** A key is unique by construction, so the
"which reason wins" question the delimited-array shape would have raised does not
arise.

## Producers and consumers

**`GUARD_KIT_BREADTH_DECLARED`, the knob.**
*Producer:* the consumer's config file — `scripts/guard-config.sh` in this repo,
or wherever `GUARD_KIT_CONFIG_FILE` points — sourced by `guard-kit/lib/guard.sh`
at the top of the lib, before any default is applied. Its enabling config is the
existing config-file mechanism, which is live: the same file already supplies
`GUARD_KIT_BREADTH_PROBES` and the tool reads it on every close-triage run, so the
producer is the production path rather than a test-only one. The lib supplies the
empty default through the same `declare -p … || …` guarded-assignment idiom the
other array knobs use, so an unset knob is an empty map rather than a `set -u`
abort.
*Consumer:* `guard-kit/bin/compare-settings-allow.sh`, in the breadth pass, by
direct key lookup.
*Named reader for each half of the pair, at a named transition:* the **key** is
read at the partition transition — the point where an over-broad entry is sorted
into the narrowing set or the declared set; the **value** is read at the print
transition of the declared section, where it is emitted beside its glob and probe.
Neither half is read anywhere else, and neither is populated at any other
transition.

**The declared section.**
*Producer:* `compare-settings-allow.sh`'s breadth pass. *Consumer:* the operator
at guard-kit/SPEC.md §The close-stage triage step, which already reads this tool's
output and already carries the two-disposition instruction the section discharges.
*Red condition:* none — the tool is advisory and exits 0 on every path, as it does
today. It is not a gate and has no verdict to invert.

**The `--count` breadth number.**
*Producer:* the same pass. *Consumer:* any caller of `--count`; in this tree that
is the close-triage reader alone (no gate, no script, no generated projection
consumes it — verified at this stage by a tree-wide grep for
`compare-settings-allow` with no stderr silenced, which finds the tool's own
source, its bespoke unit test, guard-kit's SPEC, README and close-triage template,
and this repo's queue prose, and no programmatic caller).
*Red condition:* n/a — advisory output, no verdict.

**No corpus is narrowed by this amendment**, so causal-completeness point 5's
narrowing clause is inert here: the breadth pass reads exactly the same two files
over exactly the same entries, and only the *partition* of an already-computed set
changes. The one output that shrinks is the `--count` breadth number, whose sole
reader is a human and which has no red condition to be non-monotone in.

## Existing sections updated

Each names the delta that owns it.

- **guard-kit/SPEC.md §compare-settings-allow** — the two-disposition sentence
  ("the operator disposes — narrow the glob, or record that the breadth is
  intended") gains the mechanism for its second half, and the declared section is
  specified there beside the narrowing one (deltas 1, 3). The `--count` sentence
  states that the breadth number counts narrowing candidates (delta 4). The
  exact-string match and its refusal of `guard_allow_match` land here, beside the
  existing sentence that names `guard_allow_match` as the shared match core
  (delta 2). The honest limits above land here (delta 6 and §The honest limits).
- **guard-kit/SPEC.md §Layout and configuration** — the knob roster gains
  `GUARD_KIT_BREADTH_DECLARED` with its default stated (deltas 1, 5). The default
  is an array, which `check-knob-default-coupling` skips-and-counts rather than
  couples, so the SPEC statement is descriptive; `check-knob-citation` still
  requires the knob be cited in its owning SPEC, which this row satisfies.
- **guard-kit/SPEC.md §The close-stage triage step** — the step's "for each entry
  the breadth report names, either narrow the glob or record that its breadth is
  intended" sentence now names *where* the second disposition is recorded
  (delta 3).
- **guard-kit/SPEC.md §Testing** — the paragraph specifying
  `gate-tests/compare-settings-allow.test.sh`'s three cases gains the declared
  cases (see below) (deltas 1–5).
- **guard-kit/templates/guard-config.sh** — carries a commented example of every
  knob a consumer may set; it gains a commented `GUARD_KIT_BREADTH_DECLARED`
  example beside the existing `GUARD_KIT_BREADTH_PROBES` one. The example must be
  generic, not this repo's vocabulary (delta 1).
- **guard-kit/lib/guard.sh** — the guarded-assignment default block gains the
  knob (delta 1). This is the only lib change; no matcher and no primitive moves.
- **guard-kit/README.md** — re-read for any restatement of the breadth report's
  shape or of the knob roster, and updated with the owner if it carries one
  (deltas 1, 3).
- **scripts/guard-config.sh** — **unchanged** in this unit, and recorded so the
  merge does not go looking for an edit (delta 6).

## What the build owes beyond the deltas

- **`gate-tests/compare-settings-allow.test.sh`** gains cases: a declared
  over-broad entry printing in the declared section with its reason and **not** in
  the narrowing set; an all-declared over-broad set printing no narrowing section
  and no false clean line; `--count`'s breadth number excluding the declared
  entry; and an exactness case — a declaration that differs from the local entry
  by one character leaves it in the narrowing set. The existing empty-knob case
  stays and now also proves the declared section is omitted. The test drives the
  tool through `GUARD_KIT_CONFIG_FILE` pointed at a sandbox config, as it already
  does, so this repo's own knobs cannot leak into the fixture.
- **No fixture pair is owed** — the tool is advisory, not a gate
  (guard-kit/SPEC.md §Testing, the bespoke-unit-test lane).
- **No `guard-tests/cases.tsv` row is owed** — that table pairs a decision with a
  command and cannot express a `bin/` tool's report.
- **The stale-declaration gap is already filed** — 2026-08-22, at this stage, in
  the committed gap inbox: a declaration whose glob is absent from the local allow
  list, or no longer over-broad, is invisible, and the cheap form (a report, not a
  red) is the same shape `done-slug-ownership-citation-report` settled on for its
  own class. The build owes nothing further on it.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`), discharged at the iteration rather
      than at the commit.
- [ ] **Removals propagated** — grepped every spec, README and template for a
      statement that only narrowing is expressible; nothing dangles.
- [ ] **Gaps filed** — the stale-declaration gap above is filed; cross-component
      gaps discovered during the work are resolved that session, not deferred.
