# SPEC amendment: seam-reach

`check-provenance-seam` holds four lexical provenance shapes over the kit SPECs.
Two gaps sit beside it. It names no **commit or digest reference**, so a hex
object name in a kit SPEC is caught only by a reviewer. And its corpus is kit
SPECs alone, so a consumer's own published design record is outside every arm.
installer/SPEC.md is that case: it carries dated rulings and CI run, job, head and
digest identifiers, which this repository's public-repo rule bars from any
tracked file. The file is off the npm `files` roster, but it is public on the
repository and mirrored onto the docs site. This amendment adds the missing arm,
gives the consumer a corpus knob to hold its own surfaces to the provenance
class, and sweeps installer/SPEC.md so the widened gate is green over it.

**The corpus-reach ruling.** Both options the entry names were weighed. The gate
takes the reach, through a consumer knob. The audit class keeps the shapes the
gate cannot see. Covering the file by review alone was refused: the audit class
is where this residue was found, reported out of corpus. A review class widened
to one more file is a second hand sweep, and enforcement-first puts the gate that
catches a defect in the unit that fixes it. Hardcoding the file into the gate was
refused as well, because which of a consumer's documents are public is that
consumer's content.

**The measurements this amendment rests on (2026-09-22).**

- **Kit SPECs carry no hex object name today.** `git grep -h -o -w -E '[0-9a-f]{7,40}' -- '*/SPEC.md' '*/README.md' ':!docs/' ':!installer/' | grep '[a-f]' | grep '[0-9]'`
  prints nothing, backticked or bare. So the new arm reds nothing on the current
  corpus. The one hit a bare hex class would take is evidence-kit/SPEC.md's
  decimal `2147483646`, which is why the arm requires a letter as well as a digit.
- **Kit templates carry two hex tokens, both mechanism.** The same probe over
  `'*/templates/*'` finds a pinned action digest in `gate-sdk/templates/gates-workflow.yml`
  and in `site-kit/templates/site-health.yml`. A supply-chain pin is kit content.
  Templates are outside the gate's corpus, so the audit class says so (delta 5).
- **installer/SPEC.md is the only repo-root-governed SPEC.**
  `git ls-files '*.md' ':!scripts/gate-tests/' | grep -v -E '^(docs/|[a-z-]+-kit/|gate-sdk/)'` lists no
  other `SPEC.md` — dropping the fixture exclusion also matches thirteen `SPEC.md`
  specimens under `scripts/gate-tests/`, gate-test fixtures rather than governed
  documents. It has seven letter-and-digit hex tokens (six commit heads and
  one digest prefix) and fourteen `` `<9-12 digits>` `` run or job id lines.
  The entry's attribution probe, `git grep -n -i -E 'ruled 20[0-9]{2}-|by the (operator|lead)' -- installer/SPEC.md`,
  plus a date beside the default `lead, ` marker, finds six dated-attribution
  sentences, opening at lines 642, 669, 1795, 2092, 2111 and 2119 at authoring.
  Two more lines, 712 and 956, cite one of them as "the 2026-08-26 ruling". They
  sit under §The install boundary, §The gate binary, §The manifest, §The packer
  and §The consumer smoke.
- **The other three provenance arms find nothing in installer/SPEC.md.** A
  `grep -F -w` of all 317 lead-line slugs of 12 or more characters matches
  nothing. `TRAJECTORY.md`, `BRIEF.local.md` and a `CLAUDE.md` section or
  possessive each match nothing.

## What changes

### (1) §check-provenance-seam gains a hex-reference arm {mechanical}

**Not yet applied.** Add to the **Arms** list, after **queue-slug**:

> - **hex-reference** — a run of 7 to 40 lowercase hex characters with no letter,
>   digit or `_` on either side, carrying at least one digit and at least one
>   letter `a`–`f`. That is the shape of an abbreviated or full git object name
>   and of a digest prefix. Either one points into one publisher's history, so it
>   is dead in every vendored copy. Requiring both classes keeps a decimal literal
>   and an all-letter word such as `defaced` out, and the arm matches bare prose as
>   well as inline code, since the reference is as dead either way.

Add to **Honest limits**:

> - an all-digit abbreviated object name passes, which is about one in thirty at
>   seven characters and rarer beyond; so does an uppercase one, and a decimal CI
>   run or job id, which no shape tells from a measured number;
> - a UUID segment or an eight-digit hex colour in prose red as a hex reference;
>   fence the specimen, since fences are the gate's one escape;

In the fixture paragraph, `bad/` trips the new arm. `good/` carries the decimal
`2147483646`, an all-letter hex word, and a fenced object name, and must add no
finding. The descriptor's `# spec:` line names the arm beside the four it lists.

The arm's text is fixed here, so executing it is a scanner plus fixtures. The
oracle is the fixture pair and the live battery, which the measurement above says
stays green.

### (2) A consumer seam-surface corpus: `CANON_KIT_SEAM_SURFACE_GLOBS` {mechanical}

**Not yet applied.** In §Layout and configuration, add to the
`CANON_KIT_SEAM_*` bullet:

> `CANON_KIT_SEAM_SURFACE_GLOBS` — array of repo-root-relative globs, default
> empty: the consumer's own published documents that `check-provenance-seam`
> holds to the provenance class. Which of a tree's documents are public is its
> own content, so no glob ships as a literal.

Replace §check-provenance-seam's **Corpus** paragraph with:

> **Corpus.** Two sets. The canonical spec (`CANON_KIT_SPEC_NAME`) at the root of
> every kit root (gate-sdk/SPEC.md §Layout and configuration), derived rather than
> listed, is scanned **only when `CANON_KIT_SCAN_KIT_ROOTS` is `1`**. That is
> the knob's existing meaning: the kit docs are the consumer's own first-party
> content. At the default `0` those files are a dependency's, since an adopter
> vendoring a SPEC-bearing tree would otherwise have its own queue slugs and
> private names red someone else's document. The files `CANON_KIT_SEAM_SURFACE_GLOBS`
> matches are the consumer's own by declaration, so they are scanned whatever the
> kit-roots knob says. They take the **provenance arms** (dated-attribution,
> agent-file-pointer, private-surface, queue-slug and hex-reference) and **not**
> the consumer-roster arm. That arm bars a kit from quoting a consumer's
> configuration, and a consumer's own record describing its own configuration is
> that consumer's content. A file both sets reach is scanned once, as a kit SPEC.
> With both sets empty the gate passes, and its clean line names which set was
> off. The gate ships in a kit rather than as a consumer gate because the seam is
> a rule for every kit publisher.

The wiring comes with the knob. The knob-table row goes in
`native/src/knobs/canon_kit.rs`. The registry member in `native/src/gates/mod.rs`
declares a `glob:knob:CANON_KIT_SEAM_SURFACE_GLOBS` walk root, and its comment
"declares no walk root" is rewritten to match. The descriptor's `couples=` gains
`knob:CANON_KIT_SEAM_SURFACE_GLOBS`, and the knob joins the corpus-knob list in
§Layout and configuration, whose "seven knobs" count becomes eight. The `.test.sh`
gains two cases a fixture pair cannot spell. First, a seam surface reds at
`CANON_KIT_SCAN_KIT_ROOTS=0`. Second, a seam surface quoting two configured
roster elements adds no consumer-roster finding.

`# armed-by: CANON_KIT_SCAN_KIT_ROOTS` stays as it is. That declaration is
already dead, because the knob never resolves empty. That is filed to the gap
inbox and is not repaired here.

### (3) installer/SPEC.md sheds its provenance {design-bearing}

**Not yet applied.** Sweep the file so every provenance arm of deltas 1 and 2 is
clean over it. Restate what binds and cut what is only history:

- **Each dated ruling** is restated as the rule plus its engineering grounds,
  undated and unattributed. This covers the six the probe above finds, plus the
  two later citations of an earlier one ("the … ruling"), which would dangle
  once that ruling's date is gone. A reversal becomes the standing rule, and the
  reversed alternative becomes a refused one on its grounds.
- **Each run id, job id, head and digest** is deleted. The finding it
  labelled keeps its round number, and its measurement date where the passage
  is a frozen attestation. gate-sdk/SPEC.md §The provenance seam says a dated
  measurement is neither class. A fact that needs the identifier to read true
  is rewritten so it does not.
- Nothing is compressed beyond that. Pruning §The consumer smoke's round record
  is a different unit.

The pass is judgment, one passage at a time, and delta 4's gate run is its
oracle.

### (4) This tree declares installer/SPEC.md a seam surface {mechanical}

**Not yet applied.** `scripts/canon-config.knobs` gains
`CANON_KIT_SEAM_SURFACE_GLOBS[] = installer/SPEC.md`, in the commit that lands
delta 3 or a later one, never earlier. The oracle is the battery: a green
`check-provenance-seam` over the widened corpus is the witness that delta 3
swept every shape a gate can see.

### (5) The provenance-seam-residue audit class names the new shape and surfaces {mechanical}

**Not yet applied.** Re-phrase the `scope:` line of `.workflow/audit-roster.txt`'s
`provenance-seam-residue` block, keeping it under the line cap, so that:

- its honest-limit list gains delta 1's two limits (all-digit, uppercase and
  decimal ids pass; UUIDs and hex colours false-fire);
- its payload-surface pass runs the hex shape over templates and READMEs,
  where a pinned action or dependency digest is mechanism and clean;
- its corpus names the seam surfaces `CANON_KIT_SEAM_SURFACE_GLOBS` declares,
  in place of the standing "installer/ excluded as not a kit" reading.

## Producers and consumers

- **The hex-reference finding** (delta 1). Producer: `check-provenance-seam`'s
  scan, over the paragraph rejoin and fence skip the other arms use. Consumer:
  the committing session, through the output contract. Its fields are file, line,
  arm and span, which the remedy line reads. There is no persistent state.
- **`CANON_KIT_SEAM_SURFACE_GLOBS`** (delta 2). Producer: the consumer's knob
  file. It is reachable because this tree sets it (delta 4), and every adopter's
  default is empty, so their behavior does not change. Consumers: the gate's
  corpus walk. The `knob:` couples token fires the hook on a glob edit.
  `--emit knob-roster` lists it off the table row. `check-kit-ref-liveness` resolves the name
  once the table row exists, and until merge the amendment's own mention is
  valved as an amendment. Roster-holding readers get a line each: the knob
  table, the registry's knob list, the §Layout corpus-knob list and its count.
- **Point 5** does not bind. No corpus narrows: delta 2 widens one, and delta 1
  adds an arm.
- **Point 6.** The widened corpus has one member in this tree, installer/SPEC.md,
  found by the `git ls-files` probe above. Its satisfying value is clean, reached
  by delta 3. Its known red set at authoring is the six dated sentences and seven
  hex tokens measured above. The run and job ids fall under no arm and are swept
  on the entry's probe.

## Existing sections updated

Rosters from `git grep -n "check-provenance-seam\|provenance_seam" -- ':!docs/' ':!TASK-QUEUE.md'`
and `grep -n "provenance-seam-residue" .workflow/audit-roster.txt`, run 2026-09-22.

- canon-kit/SPEC.md §check-provenance-seam, the arms, honest limits, corpus and fixture paragraphs (deltas 1 and 2).
- canon-kit/SPEC.md §Layout and configuration, the `CANON_KIT_SEAM_*` bullet and the corpus-knob list with its count (delta 2).
- `canon-kit/checks/check-provenance-seam.gate`, its `# spec:` line and `couples=` (deltas 1 and 2).
- `native/src/gates/provenance_seam.rs` (deltas 1 and 2).
- `native/src/gates/mod.rs`, the registry member and its comment (delta 2).
- `native/src/knobs/canon_kit.rs` (delta 2).
- `canon-kit/gate-tests/check-provenance-seam/` and `canon-kit/gate-tests/check-provenance-seam.test.sh` (deltas 1 and 2).
- `canon-kit/README.md`, whose gate-roster comment reads "in a kit SPEC (kit authors only)" (delta 2).
- gate-sdk/SPEC.md §The provenance seam, the paragraph naming what `check-provenance-seam` holds and over which corpus (deltas 1 and 2).
- installer/SPEC.md, §The install boundary, §The gate binary, §The manifest, §The packer and §The consumer smoke (delta 3).
- `scripts/canon-config.knobs` (delta 4).
- `.workflow/audit-roster.txt`, the `provenance-seam-residue` scope line (delta 5).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md` and `docs/canon-kit/README.md`.

## Retired spellings

- None — no delta of this amendment removes a name; the arms, knobs and gate keep theirs.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the new arm, the new knob and the
      widened corpus.
- [ ] **Instruction surfaces: instruction only.** Not reached, since no template
      or shim changes.
- [ ] **Merged with no information lost.** The corpus paragraph is re-phrased,
      not appended to. installer/SPEC.md loses identifiers and dates of
      attribution, never a finding.
- [ ] **Order held.** Delta 4 lands in delta 3's commit or after it, and the
      battery is green on that commit.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved.** `provenance-seam-commit-sha-shape` and
      `installer-spec-provenance-residue` move to Done in the merge commit, at a
      stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
