# SPEC amendment: witness-fidelity

**The consumer smoke's manifest report and its verdict decompose one recorded string and print
different answers, and the report asserts they are the same row.** This amendment makes that
assertion a *check* the report performs and prints, rather than a claim it makes. It does not
repair the carriage return the verdict reports, which is
`smoke-manifest-read-appends-carriage-return`'s and stays there.

**The attested instance, re-read at source this session rather than taken from the entry.** Run
`34142337941`, job `install-smoke-windows`, head `1369bf21` — and
`installer/consumer-smoke/run-smoke.sh` is **byte-identical** between that head and this one, so
the code read is the code that ran. On the deciding entry `gate-sdk/README.md`:

- the report's block prints six values — `want`, `got`, `reread`, `own`, `raw`, `wantalt` — and
  every one renders `len 40`, forty octets, `printf '%q'` unquoted, all six the same hash;
- the verdict prints, of that same path, `want (len40=no class=dirty[residue=$'\r' first=40]
  shape=fail)`;
- a census over the whole job log finds `len    41` zero times, an octets line carrying `0d` zero
  times, and a `%q` rendering opening `$'` zero times, against twelve `len    40` lines.

So the report's row says the two operands are one identical, well-formed hash — which would make
the comparison that failed impossible — while the verdict says one of them is forty-one bytes.
One of the two printed decompositions is unsound, and nothing the harness prints says which.

**The suspect the queue entry named is exonerated, and that is a deliverable of this stage rather
than a caveat on it.** The entry nominated the `malformed_first` tuple's round trip — built at
`run-smoke.sh`:433, split again inside `manifest_report` — and prescribed a local harness. The
harness was built and run: it constructs the tuple exactly as the loop does with a CR-suffixed
`want`, crosses the same guarded `${bad_hash[@]+"${bad_hash[@]}"}` argv expansion, and applies
both re-splits. **Both decompositions agree** (want `len=41`, `dirty[residue=$'\r' first=40]`,
through the report's path and the verdict's alike), and an argv census shows the tuple crossing
the boundary at its full length with its `0d` intact. The round trip does not reproduce the
divergence.

**What this amendment therefore asserts, and what it deliberately does not.**

- **Asserted** — the four deltas below, each tree-verifiable and checkable at validate: the
  verdict's own witness row is printed rather than deduplicated away, the coincidence claim
  becomes a performed check, the `call` field stops asserting provenance it cannot verify, and
  the record carries the exoneration.
- **Not asserted** — a mechanism for the divergence. It is unexplained on the code as written,
  and this amendment is shaped so that the *next* red round names it in-band instead of leaving
  a reader to reconstruct it from two contradictory blocks.
- **Not asserted** — a green `install-smoke-windows`. That leg stays red on the carriage return
  until its own entry lands.

**The decider does not move, and the disagreement is not collapsed.** The obvious repair — one
decomposition helper both the report and the verdict call — is **refused**, and the ground is
this smoke's own standing rule: two independent implementations reading one variable make their
disagreement *observable instead of absorbed* (`run-smoke.sh`:166, the glob-bracket class test
kept deliberately distinct from the ERE the composite uses). Unifying the two splits here would
delete the only signal that anything is wrong while changing nothing about the value that
reaches the comparison. The two splits stay two; what changes is that their disagreement is
printed and named rather than silently resolved in favour of one of them.

## What changes

### (1) The verdict's own witness row is printed, never deduplicated away

`manifest_report`'s sample selection stops dropping the `mal_first` tuple when some earlier
sample shares its path {design-bearing}.

Today (`run-smoke.sh`:269-279) the witness row joins the sample set only when no already-chosen
sample carries the same path; the match is `${entry%%$'\t'*}` against `${mal_first%%$'\t'*}`, a
comparison of **paths**. On a match the report prints the earlier sample's tuple and states that
the verdict row and that sample coincide.

The predicate becomes the **whole tuple**. `mal_first` is added to the sample set unless some
chosen sample is byte-equal to it, so the row the exit-2 verdict is computed from is always one
of the rows the report decomposed. The existing guarantee this section already carries — that no
run can exit 2 on an entry this report did not print — is unchanged in words and strengthened in
fact: it now holds of the entry's *bytes* and not only of its path.

The artifact-row dedup immediately above it (`:256-266`) takes the same tightening for the same
reason, and its two stated outcomes keep their present spellings.

### (2) A path-equal, tuple-unequal pair is a printed finding, not a collapse

The line that today claims two rows coincide becomes conditional on a check the report performs,
and the disagreeing case gets its own sentence {design-bearing}.

Three outcomes replace today's two:

- **byte-equal** — the witness row is one of the samples already chosen. Printed as it is today;
  the claim is now earned.
- **path-equal, bytes differ** — both blocks are printed, the witness block labelled as the row
  the verdict below is computed from, and the report says so plainly: the two decompositions of
  one recorded entry disagree, which is a statement about this harness and not about the
  consumer's tree. This is the outcome the attested run silently took.
- **absent** — the witness row is not among the samples; it joins them, as today.

The report stays a straight-line sequence of prints with no branch that can change the verdict
the caller goes on to fail with — the property `:229` states — so this adds an observation and
never a decider.

### (3) The `call` field stops asserting provenance it cannot verify

`held_probe`'s call string, which today asserts of `want` that it is "the value the failing
comparison used and not a second read of it", is replaced by a statement of **where the value was
read from** plus, for the witness row, a printed identity check against the verdict's own copy
{design-bearing}.

This is the load-bearing half. The `call` field is what licenses a reader to treat the report as
adjudicating the verdict; an unverifiable assertion there converts a live defect into positive
evidence of health, which is what let the series read clean. The replacement makes the same
information available without the claim: the field names the variable and the line the value came
off, and delta 2's outcome line carries the identity claim, where it is checked.

`hash_probe`'s call field is untouched — it prints the command that produced the value, which a
reader can re-run, and it asserts nothing beyond that.

### (4) The record carries the exoneration and the new truth-table row

`installer/README.md` §The consumer smoke gains the round record for run `34142337941` and the
truth-table row delta 2 mints {mechanical}.

Three facts land, none of them restated anywhere else: the report and the verdict disagreed on
one entry's `want` while the report asserted they were one row; the tuple round trip the finding
first suspected does **not** reproduce under a local harness, so the mechanism is open; and the
`path-equal, bytes differ` outcome, with what a reader should conclude from it — that the
harness's own witness is in question and no reading of the consumer's tree may be taken from that
run's manifest arm.

## Producers and consumers

**New state — the witness-row block and the three-outcome coincidence verdict (deltas 1, 2).**

- **Producer:** `manifest_report`, in `installer/consumer-smoke/run-smoke.sh`, on the manifest
  arm's **failure branch only** (`:436-437`, guarded by `[[ "$mismatch" -eq 0 ]] ||`). Its
  enabling configuration is nothing — the arm runs on every profile of every install-smoke leg
  and the branch is taken exactly when the manifest disagrees with the tree. It is reached today
  on `install-smoke-windows` (493 of 493, attested above) and on no other leg.
- **Consumer:** a reader of the leg's log, and `installer/README.md` §The consumer smoke's truth
  table, which is where every value the report prints is given its reading (delta 4).
- **Fields and their named readers.** The witness block carries the same three fields every
  sample block carries — `path`, `want`, `got` — each already having a truth-table row, and each
  rendered through the existing `held_probe`/`value_probe` pair with no new field. The one new
  field is the coincidence verdict's **outcome word**, read by delta 4's new truth-table row at
  the transition where a reader decides whether the manifest arm's finding is about the consumer
  or about the harness. No field is added that delta 4 does not give a reader.

**Changed interface — `held_probe`'s third operand (delta 3).** Producer: the two `held_probe`
call sites at `:284-285`. Consumer: the same log reader, at the same transition. The operand's
type is unchanged (a free-text provenance line); what changes is what it is permitted to say, and
that permission is stated in §The consumer smoke rather than at the call site.

**No corpus is narrowed by any delta**, so the red-condition enumeration of the
causal-completeness check's point 5 does not bind. The one predicate that moves — the sample
dedup, deltas 1 and 2 — **widens** the printed set: every tuple printed today is still printed,
and a tuple that was dropped may now be added. The readers whose verdicts could be affected are
the two binding legs, and their red conditions are unchanged by construction: `install-smoke` and
`install-smoke-macos` are **not** `continue-on-error`, every delta adds failure-branch-only code,
and neither leg reaches that branch on a tree whose manifest agrees — verified against
`.github/workflows/gates.yml`, not assumed.

## Existing sections updated

- `installer/README.md` §The consumer smoke — the manifest report's composition and its sample
  selection, which today states the dedup and the coincidence claim in the shape deltas 1 and 2
  change (deltas 1 and 2).
- `installer/README.md` §The consumer smoke, its instrument-reading truth table — the new
  outcome word and the reading a `path-equal, bytes differ` run licenses (delta 2), and the round
  record for run `34142337941` with the round-trip exoneration (delta 4).
- `installer/README.md` §The consumer smoke, its anti-normalization rule — extended in place to
  cover the *decompositions* of one recorded value and not only its two renderings, since delta
  2's refusal to unify the two splits is that rule reached from a third direction (delta 2).
- `installer/consumer-smoke/run-smoke.sh`'s `# spec:` pointers at `:229`, `:268` and `:202` — each
  binds to §The consumer smoke and each states a rule the deltas above move, so each is rewritten
  in the same unit rather than left pointing at prose that no longer describes the code
  (deltas 1, 2 and 3).

<!-- update-target-exempt: gates.yml is deliberately untouched — no delta changes a leg's arm header, its continue-on-error disposition or its steering, and the amendment's whole surface is the smoke's failure branch and the section that documents it -->
- `.github/workflows/gates.yml` — untouched.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
