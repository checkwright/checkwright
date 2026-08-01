# SPEC amendment: derived-roster-sets

Queue entry: **`spec-roster-enumeration-derivation`**.

Seven kit-SPEC rosters name a set the tree already owns, and each has drifted.
This amendment rules which of them a derived enum set can hold honest, reshapes
the two whose prose form the existing oracle cannot read, and dispositions the
one that is not a roster at all.

**No kit gains a gate, an assertion, or a knob.** canon-kit's
`check-prose-enum` already scans every `*/SPEC.md` (this repo's
`CANON_KIT_MANIFEST_FILES` carries the glob) and already accepts consumer-declared
sets through `CANON_KIT_ENUM_SETS_CMD`. The whole mechanism exists; what is
missing is that no declared set names these members. This follows the ordering
`core-files-kit-coverage-derived` set at its own spec stage — reach for the
token the tree already ships before minting a gate.

## What changes

### 1. `scripts/enum-sets.sh` gains four derived set families {design-bearing}

The emitter today declares two sets, both derived from queue-kit's own tag
parser. It gains four **families**, each emitting one set per kit root
discovered from the tree, with every member derived — never restated:

| Family | Set name | Members derived from |
|---|---|---|
| Kit libs | `<kit>-lib` | basenames of tracked `<kit>/lib/*.sh` |
| Kit unit tests | `<kit>-gate-test` | basenames of tracked `<kit>/gate-tests/*.test.sh` |
| Lib functions | `<kit>-<lib>-fn` | function definitions in `<kit>/lib/<lib>.sh` |
| Lib callers | `<kit>-<lib>-caller` | tracked `*.sh` files sourcing `<kit>/lib/<lib>.sh` |

Kit roots come from the tree the same way the emitter already reads
`check-tag-lead-line.sh` for its tag table: by parsing the source of record, not
by carrying a list. A family whose directory does not exist for a kit emits
nothing for that kit, so a kit without `lib/` or `gate-tests/` costs no line.

**Why these four and not a hand-picked set per instance.** Each of the seven
findings is a set-difference over one of exactly three tree shapes — a file
list, a function-definition list, a call-site grep. Declaring the families
generically means the eighth instance, in a kit nobody has audited, is caught
the day its paragraph is written, which is the property seven hand corrections
would not have.

**The provenance seam holds by construction.** Every set lives in this repo's
consumer emitter; canon-kit ships the `<set-name>`⇥`<member>` emit grammar and
nothing else. No kit literal names a kit root, a lib, or a function — the same
split `CANON_KIT_ENUM_SETS_CMD` was given when it landed.

**Two mechanics build verifies against the gate's source rather than assuming.**
First, whether `_sk_present` retries after a boundary-failed hit: a member like
`ek_parse` first occurring inside `ek_parser_for` must not read as absent, or
the family manufactures a false omission. Second, the added matching cost — the
member population grows from the current tag vocabulary to the tree's file,
function and caller names, and `kpi-gate-runtime` is the meter that says whether
that is free. If it is not, the families narrow to per-kit granularity before
they narrow to a curated subset: a curated subset is the drift this unit exists
to delete.

### 2. gate-sdk/SPEC.md — the `lib/declaration.sh` caller roster {mechanical}

Three claims in one passage are false and are corrected together: the caller
list is completed with `scripts/check-tightened-gates-note-parity.sh`; the
"record arm's only caller is the smoke's untagged branch" clause goes, since
that script calls `decl_record_tokens` too; and the "no caller fixture to ride"
justification goes, because `scripts/gate-tests/check-tightened-gates-note-parity`
ships a good/bad pair that exercises the record arm through its `DECL_FILE`.
The direct unit test keeps its place — it is still the arm's runtime lock-in —
but it is no longer justified by an absence that is not there.

Mechanical because the `<kit>-<lib>-caller` set is the oracle: run the gate,
name what it reports, stop when it is green.

### 3. canon-kit/SPEC.md and queue-kit/SPEC.md — the config-template claim {mechanical}

Both say their `templates/<kit>-config.sh` is "the consumer config template
documenting every knob". Every kit's template is a two-line `# spec:` pointer
stub assigning no knob — which is the **correct** design, not a defect: the knob
table has one home in the kit's §Layout section, and a template restating it
would be the parallel copy content tiering forbids. So the prose is what is
wrong, and it is corrected to say what the stub is and why: a pointer to the
knob table, so the table stays the one owner.

This instance is deliberately **not** given an oracle, and the reason is that it
has no growth vector once corrected. It is not a roster drifting behind a
growing set; it is a false description of a file whose shape is a fixed
convention across every kit. After the correction no SPEC claims it, and the
next kit's §templates/ line is written by copying a corrected neighbour. Filing
a gate for a class with no growth vector is the flagged-and-gated shape, not the
enforcement-first one.

### 4. delegation-kit/SPEC.md — `lib/delegation.sh` in the layout {mechanical}

The layout enumerates `bin`, `checks`, `gate-tests`, `templates` and `smoke` and
names no `lib/` at all; `lib/delegation.sh` appears **nowhere** in the SPEC,
which is wider than "absent from one list". It joins the layout with its
sourcers named — the two gates and two `bin/` tools that read it — so a reader
meets the file where its couplings are, not only where its path is.

### 5. evidence-kit/SPEC.md — the `lib/evidence.sh` adapter roster {mechanical}

The adapter list is short by **two**, not one: `ek_suite_cmd` (called at
`bin/run-validate.sh`) and `ek_parser_for` (called at `bin/run-validate.sh` and
exercised directly by `gate-tests/evidence-lib.test.sh`). Both join the list.
The second omission was not in the filed entry and is recorded here so the merge
does not close the instance on the filed count.

### 6. context-kit/SPEC.md — `check-brevity.test.sh` named {mechanical}

The kit ships two `gate-tests/*.test.sh` files and names one, twice, with a
justification; the other is never named. `check-brevity.test.sh` joins the
listing beside its sibling with the axis it holds, so the `context-kit-gate-test`
set sees a complete list rather than a lone member it must stay silent on.

### 7. canon-kit/SPEC.md — a literal unit-test roster {design-bearing}

Four of canon-kit's `gate-tests/*.test.sh` files are named nowhere in the SPEC
while six are named in their own gate's section. This is a **cross-document
absence**, not an incomplete paragraph, so `check-prose-enum` — which judges one
paragraph at a time — structurally cannot see it however the set is declared.

Rather than mint a whole-document arm for one instance, the prose is reshaped
into a form the existing oracle reads: canon-kit's §Per-component contracts
gains one sentence naming every `gate-tests/*.test.sh` file literally, while the
per-gate justification for each stays in that gate's own section, where it
belongs. That is deliberately a hand list held by a gate rather than a generated
one — the same trade gate-sdk/SPEC.md §check-readme-roster took and for the same
reason: the names are derivable, the reason beside each is hand prose, so the
list stays human-read and the `canon-kit-gate-test` set holds it honest.

**The ruled-out alternative, recorded.** Extending the emit grammar with a third
scope field, so a set could be checked over a whole file instead of a paragraph,
was weighed and refused. It changes a wire grammar whose current fail-closed
rule rejects a third field, for one instance, when reshaping the prose closes it
with no kit change at all — and a document-scoped completeness check has a much
worse false-positive profile than a paragraph-scoped one, since any mention
anywhere satisfies it.

### 8. site-kit/SPEC.md — the fixture-coverage paragraph made literal {design-bearing}

The paragraph enumerates three unit tests by narrative description — "the span
assertion also carries its own hermetic unit test", "the table assertion
likewise", "the batch contract carries a unit test of its own" — and names no
file. No literal member string is present, so the matcher has nothing to find
however the set is declared, and the fourth sibling
(`check-docs-render-fidelity-foreign.test.sh`, a real test of the SVG/MathML
exemption scoping) is simply missing.

The paragraph is rewritten to name each of the four files literally beside the
assertion it isolates. This is the general rule the instance teaches, and it is
worth stating once in the merge: **a roster written as narrative is a roster no
oracle can hold** — where a SPEC enumerates a governed set, it names the members,
and the derived set does the rest.

## Producers and consumers

**The four set families (new state).**
Producer: `scripts/enum-sets.sh`, invoked through `CANON_KIT_ENUM_SETS_CMD`
(`scripts/canon-config.sh`) — already set to that script, so the enabling
configuration is emitted in this repo's real configuration and not only under
test. The families are computed at emitter runtime from tracked files, so a new
kit, a new lib, a new unit test or a new caller enrols with no edit.
Consumer: `canon-kit/checks/check-prose-enum.sh`, which loads the emitted lines
through `spec_enum_sets` (`canon-kit/lib/spec.sh`) at startup and reads **both**
fields of each line at match time — the set name into the finding, the member
into the paragraph matcher. Neither field is unread, and no third field is
introduced (delta 7's ruled-out alternative is exactly the field this amendment
declines to add).

**Fail-closed path, unchanged and inherited.** `spec_enum_sets` treats a
non-zero emitter exit or an unparsable line as exit 2. The emitter already
follows that contract for its tag derivation — it exits 2 when its anchor parse
finds other than the one class table it expects — and each new family follows
it: a family whose derivation returns nothing for a kit that has the directory
is a broken derivation, not an empty set, and exits 2 rather than reporting a
silently empty roster. A silently empty set is the one failure mode that would
turn this whole unit into a false clean, which is why it is named here rather
than left to build.

**The corrected prose (existing state, new obligation).**
Producer: a maintainer editing a kit SPEC. Consumer: `check-prose-enum` at the
pre-commit transition via the generated hook, plus the human reader the roster
exists for. `check-md-refs` and `check-docs-link-convention` are second consumers
of any section name these edits touch — a renamed heading breaks a citation
elsewhere in the tree, so the merge greps for citations to each edited section
rather than trusting that a prose edit is local.

**Whole-component-set reader survey.** The enum-set surface has exactly one
consumer in the tree (`check-prose-enum.sh`, through `spec_enum_sets`); the
seven edited passages are read by `check-prose-enum`, `check-manifest-count`
(delta 7's sentence must name members, never a cardinal, or it trades this
gate's finding for that one's), `check-md-refs`, `check-surface-duplication` and
`check-comment-tier` on their surrounding surfaces. Build re-runs that survey
against the tree, with no `2>/dev/null` on any path probe — a silenced stderr on
a mistyped path reads a live reader as absent, which is the exact false negative
this unit is about.

## Existing sections updated

- **canon-kit/SPEC.md §check-prose-enum** — the closing paragraph names this
  repo's emitter and the two sets it derives today; it gains the four families
  (delta 1), so the gate's own documentation of its consumer config does not
  become the eighth stale roster this unit exists to remove.
- **canon-kit/SPEC.md §templates/** — the config-template claim (delta 3).
- **canon-kit/SPEC.md §Per-component contracts** — the literal unit-test roster
  sentence (delta 7).
- **queue-kit/SPEC.md §templates/** — the config-template claim (delta 3).
- **gate-sdk/SPEC.md §lib/declaration.sh** — the caller roster and the two
  justifications built on its absence (delta 2).
- **delegation-kit/SPEC.md §Layout and configuration** — the layout listing
  (delta 4).
- **evidence-kit/SPEC.md §lib/evidence.sh** — the adapter roster (delta 5).
- **context-kit/SPEC.md** — the two passages naming the kit's unit tests
  (delta 6).
- **site-kit/SPEC.md** — the fixture-coverage paragraph (delta 8).
- **The generated docs mirror** — `docs/<kit>/SPEC.md` is a generated projection
  of each kit SPEC and carries copies of two of the corrected sentences today.
  It is regenerated rather than edited; the trigger and the regen command are
  rostered in docs/site-architecture.md §Generated projections, which is the
  owner of that fan-out for every delta above.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
