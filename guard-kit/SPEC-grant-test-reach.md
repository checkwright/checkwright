# SPEC amendment: grant-test-reach

Queue entry: `scan-prompts-grant-test-redirect-blind`, the grant-test unit of `guard-friction-reach`
(operator direction, 2026-09-15, lead-relayed).

**The entry's premise, probed, and it is live rather than latent.** The entry costs the defect as
latent, since no committed glob spells a redirect (`Bash(cat >> .tmp/*)`). But any committed
`Bash(<cmd> *)` glob's trailing `*` absorbs a redirect as readily as a spelled one. The probe fed
`scan-prompts` a scratch log over this repository's committed settings:

- `echo hi > notes.md`, `grep foo a.txt > .tmp/out.txt` and `ls >> notes.md` all read granted
  (`PROMPT-FRICTION: clean`).
- `` echo `date` ``, `grep -n foo <(ls)` and `ls >(wc)` read granted too.

rule 17 states that no `Bash(…)` entry grants a redirect target. Rule 6 and §scan-prompts' own
reachability verdict state that the matcher refuses every expansion. So the one defect has two halves,
and the entry names only the first: the grant test (`granted()` → `segment_granted` →
`guard::allow_match`, a plain glob match on the raw segment) disagrees with the reachability verdict
(`allowlist_unreachable`) on **both** shapes the verdict reads.

**The ruling.** The grant test consumes the verdict: a logged call the verdict marks
allowlist-unreachable is never granted, by the committed file or by the overlay. There is one
predicate with two readers, so the two readers cannot disagree again when either is edited. Fixing the
redirect half alone was weighed and refused. It would leave a disagreement the probe had already
measured, which would have to be filed the moment it was left, and it would give the redirect shape a
second holder beside the verdict's own scan.

## The seam

- **Kit mechanism:** one conjunct in `granted()` (`native/src/emit/scan_prompts.rs`), its in-crate and
  front-end cases, and §scan-prompts' text.
- **Consumer config:** none. The arm's three declared knobs are unchanged.
- **Private rule content:** none in reach.

## What changes

### (1) `granted()` returns false for an allowlist-unreachable call {mechanical}

`native/src/emit/scan_prompts.rs`. `granted(cmd, allow, overlay)` gains a leading conjunct: when
`allowlist_unreachable(cmd)` holds, the call is not granted, and no segment is matched. Both calls in
`tally` go through `granted`, the committed pass and the overlay pass, so an unreachable call can no
longer land committed-covered or overlay-covered. It always lands prompting, and it is counted in the
unreachable section. So `unreachable ⊆ prompting` holds by construction, where before it held only
because the tally consults the verdict inside the prompting branch. The verdict function itself is not
changed. Its heredoc bound (only the text before the first opener is scanned) and its `/dev/null` and
fd-dup exemptions carry over to the grant test unchanged.

In-crate cases, beside the existing verdict tests:

- Under an allow list holding `Bash(echo *)`: `echo hi > notes.md` and `` echo `date` `` are not
  granted; `echo hi > /dev/null` and `echo hi 2>&1` are.
- Under an overlay holding `Bash(ls *)` and an empty committed list, `ls >> notes.md` tallies as
  prompting and unreachable, not overlay-covered.

### (2) The front-end case {mechanical}

`guard-kit/gate-tests/scan-prompts.test.sh` gains a sandbox pair and log of its own, so the existing
fixtures' exact counts (`2/2`, `4/4`) keep their corpora. The committed file holds `Bash(echo *)`, and
the log holds `echo hi > notes.md`, `` echo `date` `` and `echo hi`. The assertions: the headline reads
`2 prompting call(s)`; `2 of them allowlist-unreachable`; and `echo hi` appears on no list.

### (3) §scan-prompts: the grant test reads the verdict, and the step is recorded {design-bearing}

guard-kit/SPEC.md §scan-prompts. **Not yet applied:**

- The three-way split's **Committed-covered** bullet becomes `every segment matches the committed
  allowlist or a harness built-in, and the call is not allowlist-unreachable (below)`. The
  **Overlay-covered** bullet's `granted` gains the same qualifier.
- A paragraph follows **What the section claims is disposition (a) only**:

  > **The grant test reads the verdict, so the two cannot disagree.** A glob match on a raw segment
  > has no model of a redirect or an expansion. A trailing `*` absorbs both, so `Bash(echo *)` would
  > otherwise read `echo x > notes.md` and ``echo `date` `` as granted, while the verdict above marks
  > both allowlist-unreachable. A call the verdict marks is therefore never granted, by the committed
  > file or the overlay, and it always lands on the headline and in the unreachable section. There is
  > one predicate and the grant test does not restate it. **The honest limit leans the verdict's
  > way:** a harness may grant a redirect target through a file-write permission rule, which this arm
  > does not read. Such a call reads prompting although it was granted. That over-count is visible on
  > the ranking, where an under-count would be silent. A write rule 17 grants never reaches the log,
  > so the common gitignored case is untouched.

- The sentence `Because the partition moves no call between prompting and granted and re-keys no row,
  it is **not** a definitional step of the kind recorded below.` gains a successor: `Reading the
  verdict in the grant test does move calls, from granted to prompting, and it is recorded as such a
  step below.`
- A paragraph follows **Changing the wrapper strip is a step of the same definitional kind**, with the
  readings filled in by the landing session on one log, immediately before and after:

  > **The grant test's reading of the verdict is a step of the same kind.** It moves every logged call
  > carrying a write redirect or an expansion that a committed or overlay glob matched from granted to
  > prompting, and re-keys nothing. On one log, `<p1> patterns across <n1> prompting calls`
  > immediately before and `<p2> across <n2>` immediately after. The KPI is left as it is, on the
  > ground above.

  The `<…>` slots are the landing session's `--emit scan-prompts --count` readings. The merge fails if
  a slot is left unfilled.

## Producers and consumers

- **The changed grant verdict** — producer: `granted()` inside `tally`, reached by `--emit
  scan-prompts` through the front-end at close (guard-kit/templates/close-triage.md step 1) and by
  `kpi-prompt-friction` through `scan_prompts::count`. Consumers: the ranking's headline, its
  actionable, unreachable and overlay sections, `--count`'s `<patterns>/<occurrences>` token, and the
  drift KPI that records it.
- **Point 5, since the change narrows the granted set.** The readers and their red conditions:
  - `scan-prompts.test.sh` asserts exact counts (`2/2`, `4/4`) over sandbox logs whose allow lists
    (`Bash(git status:*)`, `Bash(ls)`, `Bash(npm test)`) match no redirect-bearing or
    expansion-bearing line, so no line those counts read moves. It is not monotone and is cleared by
    that reading, not by inspection. The build re-runs it.
  - `kpi-prompt-friction` reds only on a token failing `^[0-9]+/[0-9]+$`, which a larger count
    satisfies. It is monotone.
  - The in-crate `tally` and verdict tests assert specific partitions. The battery's `check-crate-arms`
    run is their oracle.
- **No new field, knob, state or file.**

Derivation of the rosters here and below: `git grep -n "granted(\|scan_prompts::"` over `native/src`,
which found `tally` and `native/src/emit/kpi/prompt_friction.rs`; a read of
`guard-kit/gate-tests/scan-prompts.test.sh` for its allow lists and counts; and
`git grep -n "kpi-prompt-friction"` over the tracked SPECs. The build unit re-derives them.

## Existing sections updated

- `native/src/emit/scan_prompts.rs` — `granted()` and its in-crate cases (delta 1).
- `guard-kit/gate-tests/scan-prompts.test.sh` — the redirect-and-expansion case (delta 2).
- `guard-kit/SPEC.md` — §scan-prompts: the three-way split, the grant-test paragraph, the partition
  sentence and the definitional-step record (delta 3).
- `docs/guard-kit/SPEC.md` — the on-site SPEC mirror, regenerated with `--emit docs-mirror --write`
  (delta 3).
- `.workflow/release-declarations.md` — one guard-kit bullet: `scan-prompts` now counts a logged
  redirect or expansion call as prompting even where a `*` glob matched it, so its headline count and
  the prompt-friction KPI step up once. A consumer needs to do nothing (delta 1).

## Retired spellings

- None — no delta retires a spelling. The two bullets delta 3 qualifies are extended, not renamed.

## Definition of Done

- [ ] **Causal completeness** — the changed verdict's producer and every reader are named, and the
      non-monotone reader was cleared by reading its corpus.
- [ ] **Instruction surfaces: instruction only** — no instruction surface changes.
- [ ] **Merged with no information lost** — §scan-prompts integrated, not appended, and the step
      record's readings filled in.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — rosters re-derived against the tree before the merge counts as complete,
      with any missed site landed and named in the commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
