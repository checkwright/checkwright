# SPEC amendment: tracking-claim

Adds one gate, `check-tracking-claim`, that verifies a prose claim about a
path's **version-control tracking status** against git itself. It exists because
the defect class it catches — an always-loaded surface asserting a directory is
committed when part of it is gitignored — is invisible to every gate the kit
ships: `check-md-refs` resolves paths, `check-spec-pointer` resolves headings,
and neither reads what the sentence *claims* about the path.

## What changes

### New gate: `check-tracking-claim`

Invariant: every **tracking claim** on a governed manifest surface agrees with
git. A tracking claim is a fixed-vocabulary predicate bound to the backticked
repo-relative path token that immediately precedes it in the same sentence:

| Predicate | Holds when |
| --- | --- |
| `is committed`, `is tracked` | every existing member of the path is tracked and none is ignored |
| `is gitignored`, `is local-only` | every existing member of the path is ignored |
| `is two-tier` | both classes are non-empty — the path holds tracked and ignored members |

Resolution: a path naming a directory expands to its members (`git ls-files`
for the tracked side, `git check-ignore` for the ignored side); a path naming a
file is its own single member. `is two-tier` is the form this repo's `.workflow/`
claim takes, and it is the reason the predicate set carries a mixed member
rather than only the two pure ones — without it, an honestly-mixed directory has
no true sentence a reader of the always-loaded tier can be given.

Reddens on a predicate whose verification fails, and — fail-closed — on a claim
whose bound path exists in neither index nor working tree, since an unresolvable
path makes the claim unverifiable rather than true.

Surface: the manifest set (`spec_manifest_files`, the same surface the
manifest-narration gate family reads — `CANON_KIT_MANIFEST_FILES` in the
consumer's canon config). **No new knob**: the predicate vocabulary is kit-owned
generic English, and which surfaces are governed is a knob that already exists.
Fenced code blocks are skipped in both directions, matching `check-spec-pointer`
— a quoted example is not a claim.

Tier: `precommit`. `# graph:` manifest couples the gate to the manifest set and
`.gitignore`.

Calibration, stated as the gate's honest limit: **forward direction only, fixed
vocabulary only.** The gate fires on the listed predicates immediately following
a backticked path and rules on nothing else. A claim phrased any other way ("the
workflow directory ships in the repo") is out of scope and stays a review
concern — the same forward-only bar `check-spec-pointer` holds, and for the same
reason: widening to "any sentence asserting tracking" needs a notion of
assertion that cannot hold the false-positive floor. The binding is the
**nearest preceding** backticked path in the sentence, so a list-then-predicate
construction ("`A`, `B`, and `C` are gitignored") is verified for `C` alone;
that is deliberate under-detection, not a bug, and the fixture pair covers it.

**First run over this repo, before the gate exists** — the align audit ran the
vocabulary across the manifest set by hand, since a new gate's first honest test
is the drift it finds. Result: **no backfill**. Three sites bind a predicate to a
backticked path — the two `.tmp/` and `.metric/` claims in this repo's
always-loaded tier, both verified true, and the `.workflow/` claim this unit
corrects. Three near-misses (a kit SPEC's "their log is gitignored", a README's
"the file is local-only", a gate's prose about tracked targets) carry a predicate
with no backticked path immediately preceding it and are correctly *not* claims;
they are the false-positive floor the binding rule buys, observed rather than
predicted. A gate that lands green is the expected shape here — this one is a
regression gate for a defect that already shipped on the always-loaded tier, not
a discovery tool, and gate-sdk/SPEC.md §When a gate earns its place governs that
class.

### The consumer-side correction this gate makes checkable

The claim that motivated the gate is this repo's always-loaded
`.workflow/` line, which asserts the directory is committed while three of its
members are gitignored. Corrected, it reads as a `is two-tier` claim citing the
surface contract that owns the partition (gate-sdk/SPEC.md §The workflow
directory, landed by the sibling `workflow-file-format-convention` amendment):

> `.workflow/` is two-tier — tracked checked projections beside gitignored local
> capture (gate-sdk/SPEC.md §The workflow directory).

That phrasing is not incidental. The line sits in a one-line-per-rule
always-loaded tier under widest-true-tier placement, so it may carry a **rule**
and must not carry an **extensional fact**; the original was false precisely
because it stated membership rather than the partition. The gate keeps the
replacement honest by construction, which is what makes the correction a durable
fix rather than a one-time edit.

## Producers and consumers

**New state/interface: the tracking-claim predicate vocabulary.**

- *Producer* — prose authored on any surface in the manifest set. Enabling
  config: none to set; the manifest knob (`CANON_KIT_MANIFEST_FILES`) is already
  populated in every consumer that runs the manifest gate family, so the gate is
  live wherever it is registered rather than dependent on new configuration.
- *Consumer* — `check-tracking-claim`, run by `run-gates.sh` and the generated
  pre-commit hook once registered in the consumer's `gates.list`.

**Fields and their named readers.** The claim has two fields:

- `<path>` — read by the gate at the verification step, to expand into members
  via `git ls-files` / `git check-ignore`. Also read by a human at the sentence,
  which is why it stays backticked rather than becoming a marker.
- `<predicate>` — read by the gate at the same step, selecting which of the
  three verifications runs.

No third field is introduced. In particular the gate takes **no per-site valve**:
a claim that cannot be made true is a claim that must be reworded, and a valve
would restore exactly the unverified-prose state the gate exists to end.

## Existing sections updated

- **canon-kit/SPEC.md §Per-component contracts** gains `### check-tracking-claim`
  carrying the invariant, surface, calibration, and fixture-pair note above.
- **canon-kit/SPEC.md §Content tiering — the star topology** gains one clause on
  the **Quantitative literals are code-owned** bullet: a tracking status is a
  code-owned fact in the same sense a count is — git owns it — so prose states
  the rule and the gate verifies it, rather than prose transcribing membership.
- **canon-kit/README.md** gate roster gains the entry (`check-readme-roster`
  reads it).
- **The consumer's `gates.list`** registers the gate; the generated pre-commit
  hook and `docs/check-graph.html` regenerate, and `docs/enforcement.md`
  regenerates for the new `tier=` row.
- **This repo's CLAUDE.md §Housekeeping** takes the corrected `.workflow/`
  sentence above. No other always-loaded claim changes: `.tmp/` and `.metric/`
  are verified gitignored as written.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
