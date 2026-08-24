# SPEC amendment: friction-key-shape

Gives `bin/scan-prompts.sh`'s ranking key a write-shape axis. Pairs with
`prompt-ranking-command-word-shape-blind`.

**The entry's fork is ruled: this unit is the instrument, not the instance.**
The entry left two deliverables open — decompose the command word by shape
inside the scanner, or give the write form its own disposition (a grant, a steer
to the Write tool, a stated habit). The instrument is taken. The instance is
filed to the gap inbox at authoring as its own costed candidate, because two of
its three shapes are unreachable from a stage session at all: a grant is a
settings edit and therefore operator-class (TRAJECTORY.md §The closed rulings,
2026-08-22), and a stated habit is not a mechanism. Only the steer is buildable,
and a steer refusing heredoc file authorship is a wide behavioral change that has
nothing to do with the instrument being wrong.

## What changes

### (1) The ranking key gains a write-shape suffix

`pattern_of()` appends the segment's **write-redirect operator** to the key when
the segment carries one, so `cat > <file> <<'EOF'` ranks as `cat >` and
`cat >> <file> <<'EOF'` ranks as `cat >>`, while a redirect-free segment is
keyed exactly as it is today. {design-bearing}

The operator is normalized to two values, `>` and `>>`. Which file descriptor is
being redirected is dropped — `2> f` keys the same as `> f` — because the axis
the row has to carry is *this segment writes a file*, and create-versus-append is
the finest distinction any reader of the ranking has ever wanted. Descriptor-dup
forms (`2>&1`) are not redirects to a file and do not qualify.

**The resulting key shape is one the ranking already has.** `git status` and
`python3 -` are two-token keys today, produced by the subcommand roster, so
`cat >` introduces no new output shape — only a new source for the second token.

### (2) The suffix comes from the same segment the word does

`pattern_of()` derives both the leading word and the redirect from the **first
segment** of the logged line, split with `guard_split_compound`, rather than the
leading word from the whole line and the redirect from anywhere in it.
{design-bearing}

**Without this the change would be worse than the defect.** The key's word today
is the first token of the whole line, so `mkdir -p .tmp && cat > x <<EOF` keys as
`mkdir`; pulling a redirect from anywhere in the line would produce `mkdir >` and
attribute a write to a command that performs none. Reading both from one segment
makes the key internally consistent by construction.

**It does not absorb the compound question, and that is deliberate.** *Which*
segment should be keyed when the friction-bearing one is not the first is a
separate axis — which segment, not which shape — and it is filed to the gap
inbox at authoring as a distinct candidate. This delta changes no key for any
non-compound line, and for a compound line it produces the same word it produces
today. So the two units compose rather than contend, and neither has to land
first.

### (3) The correction the entry's own framing needs

The section's prose states what the row actually is, which is not what the entry
predicted. {design-bearing}

**The read half is not in the row; it cannot be.** `guard_rule_cat_file` blocks
a lone single-operand `cat <file>` read, and §scan-prompts already records that
`guard_block` exits 2 before `guard_log_fallthrough` runs, so a blocked command
never reaches the log. Measured at authoring on the live 157-line log: of 19
`cat`-led lines, 14 are `cat >>` and 5 are `cat >`, and **none** is a read. So
the top row is not an answered steer and an unowned write sharing a row — it is
the unowned write alone, wearing the word that names the answered steer.

**Why the correction is worth the words rather than being quietly fixed.** It
changes what the fix has to achieve and gives it a checkable post-condition:
after this amendment every `cat` row in a tree running the shipped ruleset
carries a redirect suffix, and a bare `cat` row is then a real finding — either a
pipe-read or a multi-file read, the two shapes `guard_rule_cat_file` deliberately
does not block. Three consecutive closes triaged this row under the read-steer
heading; a section that fixed the key without saying why the row misled would let
the fourth do it again.

### (4) `cat` is the only word this bites today, and the section says so

The shape axis is added for one measured row, and the measurement that bounds it
is recorded with it. {design-bearing}

Decomposed at authoring across the whole live ranking: `awk` 13 of 13 reads,
`grep` 6 of 6 reads, `python3 -` 3 of 3 inline heredoc execution and already
two-token, `git` already subcommand-keyed so its reads and writes already occupy
separate rows, `sed`/`echo`/`printf` absent — `sed` because
`guard_rule_sed_file` blocks its read and in-place forms upstream of the log, the
other two because they are `GUARD_KIT_APPEND_BINS` and auto-allow for `>>` under
a declared scratch dir.

Recorded because the honest reading of that table is that the axis is **general
and currently single-instance**, and a later reader deciding whether to extend or
retire it needs to know the bite was measured rather than assumed.

### (5) No parser is minted

The redirect detection composes helpers already present in the library
`bin/scan-prompts.sh` sources: `_guard_redirect_pairs` for the operator/target
pairs, `_guard_redirect_targets` for the fd-dup exclusion, `guard_skeleton` for
the lexical view, `guard_split_compound` for delta 2's split. {mechanical}

They are `_`-prefixed internal helpers rather than the documented `guard_*`
framework surface, and this amendment calls them from a `bin/` tool inside the
same kit. That is a kit-internal call and no contract is being widened for a
consumer; whether a vendored library's `_`-prefixed identifiers read as public is
the open question `vendored-library-identifier-reach` owns, and this unit is
sized against that ruling rather than ahead of it — if that entry later rules the
prefix private, the fix is a `guard_*` front end, not a second parser here.

### (6) The KPI's numerator moves, and the discontinuity is recorded

`drift-kit/kpis/kpi-prompt-friction.sh` reads `--count` and reports
`<distinct>/<total>`. Splitting one key into two raises `distinct` while leaving
`total` unchanged, so the trend steps at the landing commit for a definitional
reason and not a behavioral one. {design-bearing}

The `^[0-9]+/[0-9]+$` contract the KPI asserts is unbroken, so nothing reds; that
is exactly what makes the step silent and worth stating. The section records the
pre-change reading — 27 patterns across 70 prompting calls on the authoring-time
log — so a later trend read can attribute the step rather than re-derive it.
drift-kit reads *trend, not level* and carries no annotation affordance, so the
SPEC sentence is the annotation.

**The KPI is not changed to compensate.** A key change that makes the metric
finer is the metric getting better, and rebasing it to hide the step would trade
a legible one-time discontinuity for a permanent lie about granularity.

### (7) The pinned test is re-derived, not assumed

`gate-tests/scan-prompts.test.sh` pins behavior by **substring** (`grep -qF` on
`'git status'`, `'make'`, `'npm test'`) plus one exact `--count` comparison
against `"2/2"`. {mechanical}

An additive suffix leaves every substring assertion true, and the `--count`
expectation moves only if a fixture line carries a redirect. The build re-derives
both by running the test rather than reasoning about it — the entry's own concern
was that this change "mints an output contract the gate-tests pin", and the
answer is that the contract they pin is the three-way split and the count
semantics, not the key's granularity. Cases are added for the new axis: a
redirect-bearing segment, an append-bearing segment, an fd-dup that must **not**
produce a suffix, and a compound whose later segment carries the only redirect
(delta 2's guard against mis-attribution).

## The provenance seam

**All kit mechanism, and the seam is what makes this unit safe to ship at
all.** The key is *derived* from the logged command's own lexical shape — a
redirect operator the segment carries — and never from any roster of command
names. There is no term list, no vocabulary, and nothing about which commands a
consumer runs enters the kit. That is a stronger position than the ranking's
existing subcommand roster (`git gh cargo docker npm …`), which this amendment
leaves untouched and does not extend: extending *that* is where a consumer's
tooling vocabulary would start leaking into a kit literal, and the shape axis
gets its bite without going there.

**No knob is added**, because there is no value a consumer would set: the two
operator tokens are shell grammar, not configuration. `GUARD_KIT_LOG` and the
two settings knobs the scanner already reads keep this repo's layout as their
defaults.

## Producers and consumers

**Changed interface — the ranking key.**
- *Producer:* `pattern_of()` in `guard-kit/bin/scan-prompts.sh`, at every ranked
  line, on both the full report and `--count`. Its enabling configuration is
  `GUARD_KIT_LOG` naming a readable log — already the default and already
  populated in every tree that vendors the guard.
- *Consumers, all three named:* the **close-stage triage**, at the
  `templates/close-triage.md` step that runs the scanner, which is the reader the
  new axis exists for; `drift-kit/kpis/kpi-prompt-friction.sh`, at its `--count`
  call, which reads only the two integers and is contract-unaffected (delta 6);
  and `gate-tests/scan-prompts.test.sh`, at the gate-test transition, which pins
  the split and the count (delta 7).

**New value class — the two-token `<word> <op>` key.**
- *Producer:* the same function, from the first segment's own redirect.
- *Named reader:* the close-stage triage, at the transition where it decides
  which row to give a disposition. That is the reader whose misreading the entry
  documents three times over, and it is the only reader that distinguishes the
  values at all — the KPI counts keys without inspecting them.

**No new knob, no new field, and no new output line.** The report's header,
footer and `%5dx  %s` line format are untouched; only the value of the second
`%s` widens. That is what keeps the change reachable from a substring-pinned test
without a fixture rewrite.

**No reader is added for the descriptor number**, which is why delta 1 drops it
rather than carrying it: a `2>` key would be a field with no reader, and the
template's own rule is that such a field is removed.

## Existing sections updated

- guard-kit/SPEC.md §scan-prompts — the stated granularity rule ("leading binary,
  plus subcommand for the common multi-command binaries") gains the write-shape
  clause and the first-segment scoping; the section also takes delta 3's
  correction, delta 4's bounding measurement and delta 6's recorded pre-change
  reading (deltas 1, 2, 3, 4 and 6).
- guard-kit/SPEC.md §scan-prompts, its output-contract paragraph — it currently
  says its behavior "is pinned by `gate-tests/scan-prompts.test.sh`"; what that
  test pins is the three-way split and the count semantics and not the key
  granularity, and the sentence now says which (delta 7).
- `guard-kit/bin/scan-prompts.sh` — `pattern_of()` and the helper calls it grows
  (deltas 1, 2 and 5).
- `guard-kit/gate-tests/scan-prompts.test.sh` — the new cases and any moved
  `--count` expectation (delta 7).
- `drift-kit/SPEC.md` — the KPI row that documents reading `scan-prompts.sh
  --count`; the contract is unchanged and the definitional step is what the row
  now also names, so a trend reader meets it where the KPI is defined rather than
  only in guard-kit (delta 6).
- `guard-kit/README.md` — the two usage lines describing the ranking and the
  `--count` token, if either states the key's granularity (delta 1).
<!-- update-target-exempt: a generated mirror is stale the moment any delta lands and is regenerated rather than authored -->
- `docs/guard-kit/SPEC.md`, `docs/guard-kit/README.md`, `docs/drift-kit/SPEC.md`
  — regenerated, never hand-edited.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Post-condition run, not reasoned** — the scanner is run against the live
      log and every `cat` row carries a redirect suffix (delta 3); a bare `cat`
      row surviving the change is a finding, not a pass.
- [ ] **Sibling-unit interaction checked** — `scratch-bash-only`'s new rule 22
      blocks a shape that reaches this log today, so a `python3 .tmp/…` line
      stops falling through once that unit lands. Whichever lands second re-runs
      this scanner rather than citing the other's numbers.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
