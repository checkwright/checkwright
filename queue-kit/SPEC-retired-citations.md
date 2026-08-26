# SPEC amendment: retired-citations

Closes `dead-queue-citation-report`. An in-body citation that resolves to no live
entry reads exactly like one that does, and `bin/queue-edges.sh` — the one tool
that already performs the resolution — drops the miss on the floor at
`bin/queue-edges.sh:37` without naming it. A live entry then goes on instructing
a future promoting session to sequence against a sibling that is not there, which
is a false premise in a survey input at exactly the moment a scope decides what
to promote.

**The entry's cheapness claim is falsified, and the falsification is the design
content.** It reads "naming them costs one output section and no new scan".
Measured at this stage over `## Deferred` (`.workflow/survey-record.md` carries
the survey and its witness): **517** distinct backticked slug-grammar tokens,
**395** of them unresolved. A listing of those 395 is unusable — it is dominated
by commit SHAs, content digests, and ordinary backticked words (`awk`, `bash`,
`a`, `an`, `2`). The token grammar `[a-z0-9][a-z0-9-]*` is not a slug detector,
and treating it as one is what makes the naive report worthless.

**The discriminator that works is derivable and it is not a vocabulary.** A
*dead* citation is, by definition, a citation of a slug that **was** live. Split
the 395 against the set of slugs that ever held a lead line in this file's
history and the result is **71 retired-slug citations** and **324 tokens that
were never slugs**. The never-live remainder is the noisy half — iteration names,
KPI names, kit-side gate names, runner labels, CI job names, doctrine rule names
— and it stays on the floor. Both attested defect instances,
`installer-lifecycle-verbs` and `port-corpus-grouping-census-unbought`, fall in
the retired half. That is the inverse of the reading the entry started from and
it is what makes the report worth having.

## What changes

### (1) `bin/queue-edges.sh` resolves against retired targets as well as live ones

The tool's resolution set widens from `queue_live_slugs` to that set **plus** the
queue file's **retired** slugs: slugs that held a top-level entry lead line in
some earlier revision of the file and hold none now. A citation resolving to a
retired slug becomes an edge, marked as such; a citation resolving to neither
stays what it is today — not an edge, and not a complaint. **{design-bearing}**

One tool, one job, one output grammar. §bin/queue-counts.sh's refusal — folding
jobs together gives one tool two output grammars, "the same refusal that keeps
`bin/queue-edges.sh` separate" — is honoured rather than worked around: this is
not a second job. The tool's job is *aggregate in-body citations by target*, and
a retired target is a target whose entry has been disposed of, not a different
question over the same file.

**Output.** Live targets print exactly as today, in queue order. Retired targets
print after them, in a trailing block, each as `<slug> (<N> inbound, retired)`
followed by its edges in the existing two-column shape. They sort
**alphabetically** because a retired slug has no queue position to order by, and
inventing one (last-seen revision, say) would be a datum with no reader.

**`--inbound <slug>` widens with it.** It accepts a live **or** retired slug and
prints that slug's inbound set; a slug that is neither still exits 1 on stderr.
The contract §bin/queue-edges.sh states — "silence from this tool has to mean
*no inbound edges* and nothing else" — is preserved exactly, because the set of
addressable slugs grew and the meaning of silence did not.

### (2) The retired set is derived from the file's own history, and the derivation is bounded

The retired set is `{ slugs that ever held a top-level entry lead line in the
queue file } \ { queue_live_slugs }`, read with one
`git log -p --format= -- <queue-file>` pass whose added and removed lines are
matched against the same lead-line grammar `lib/queue.sh` already owns.
**{design-bearing}**

**Measured, not estimated:** 1436 revisions of this repo's queue file, 725 slugs
ever live, **0.72s** wall clock. That is a tool's budget, not a gate's, and this
is a tool.

**Two degradations are declared rather than discovered.** A queue file that is
**not in a git repository**, or a `git` that is absent, yields an **empty**
retired set and the tool prints its live block alone — the same output it prints
today, which is why the degradation is silent-safe rather than misleading. And
the derivation sees only history **this clone has**: a shallow clone or a queue
file whose history was rewritten under it reports fewer retired slugs, never
more, so the report **under-**claims and never invents. Both are stated because
the natural reading of "derived from history" is that history is total, and here
it is whatever the clone holds.

**The one-adapter rule is respected and the scan stays in the tool.** The
lead-line grammar comes from `lib/queue.sh`; the history walk lives in
`bin/queue-edges.sh` beside the body-citation scan it already owns, on that
scan's own stated ground — one reader, and the library's rule is shared adapters
(§lib/queue.sh). A second reader of the retired set is what would move it.

### (3) The never-live remainder stays on the floor, and the silence stops being unexplained

§The tag algebra's bullet "**An unresolved token is not an error; it is simply
not an edge**" keeps its rule and gains its measurement: the unresolved
remainder is **not** a set of dead citations, it is overwhelmingly not citations
at all, and this is why no report over it and no gate on it can exist.
**{design-bearing}**

The entry's other genuinely-open half — *whether a citation of landed work should
be distinguishable in prose at all* — is **answered no**, and the answer is the
one the surface already gives twice. §The tag algebra refuses a relational
vocabulary because a kit literal spelling one project's relational verbs ships
that project's vocabulary as everyone's, and it refuses a `relates:` declaration
because every citation a corpus has already written would need hand-authoring,
"the maintained-roster anti-pattern derivation-first forbids". A prose marker
distinguishing landed from live is that same maintained roster with a different
spelling: it would need writing on every existing citation and re-writing on
every disposition. Delta 1 makes the distinction **derived** instead, which is
the form both refusals leave open, and no author writes anything.

### (4) The report has two named readers at two named transitions

The listing's readers are declared where each stage's obligation lives, because a
report with no reader is the failure this entry's own cost field describes.
**{design-bearing}**

- **scope, at its ranking survey** — the primary, because it is the attested
  harm's moment: the third instance was found "first-hand by the very scope
  survey it was predicted to mislead". A scope reading the retired block sees
  which of a candidate entry's citations point at disposed work before it ranks
  the entry.
- **close, at the gap-inbox drain** — the corrective transition, where an
  instance found is fixed inline on its owning entry, exactly as the 2026-08-25
  and 2026-08-06 instances were.

Neither stage gains a gate and neither gains a refusal. The tool stays a tool:
§bin/queue-edges.sh's "**Not a gate, either**" is unchanged, and its reasoning is
strengthened rather than weakened here — an entry citing landed work is
legitimate prose, so a retired edge is a *finding to read*, never a violation.

## Producers and consumers

**The retired slug set (deltas 1 and 2)** — the one new derived value.

- *Producer:* `queue-kit/bin/queue-edges.sh`, at start-up, one `git log -p` pass
  over the configured queue file, in the same run that reads the live set.
  **Enabling config actually set:** none is added. It reads
  `QUEUE_KIT_QUEUE_FILE` (or the positional argument), which every existing
  invocation already resolves, and `git`, which this repo's toolchain floor
  already requires. It is live on the next invocation in this tree rather than
  behind a knob nothing sets.
- *Consumer:* the tool's own `awk` resolution at `emit()`, in the same run — the
  function that today returns early on `!(tgt in L)` instead classifies the
  target as retired and records the edge in a second ordered list.
- *Named readers for the output,* at named transitions: the **scope** stage at
  its ranking survey, and the **close** stage at the gap-inbox drain (delta 4).
  **Asymmetric today, re-verified against both templates rather than assumed
  even.** Scope already runs `bin/queue-edges.sh` at its ranking survey
  (`templates/stages/scope.md` §the ranking survey already cites it) and gains
  only the retired block as a further thing that call's output carries. Close's
  gap-inbox drain runs no queue-kit tool against `TASK-QUEUE.md` today — it
  dispositions bullets from `LIFECYCLE_KIT_GAP_INBOX_FILE`, a different file —
  so delta 4 is that stage's **first** touch of `bin/queue-edges.sh`, not a
  second read through an existing one. Still no new mechanism: the touch is one
  more command a session already capable of running shell tools invokes, not a
  new script or gate.

**The `retired` marker on a target line** is the only new field.

- *Reader:* the two stage readers above, at the transitions above. It is read to
  decide whether a citing entry's premise is still standing before that entry is
  ranked or corrected. It is not read by any gate, any projection, or any
  generated page — deliberately, and stated so a later reader does not add one.
- It is populated at exactly one place (the retired block's target line) and at
  no other, so the field is not read at one transition and written at others.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
No delta narrows a corpus; delta 1 **widens** the tool's resolution set. The
readers that move are enumerated by red condition anyway:

- `bin/queue-edges.sh`'s own fixture/behavioural coverage — reds when observed
  output differs from the expected. **Not monotone** (exact match), and it is the
  reader that must move in the same unit: today's expectations assert the live
  block alone and would fail on a trailing retired block. Listed as an update
  target.
- `check-queue-slug-liveness` — reds when a **bold-code** token on a configured
  prose surface names no live task. **Untouched and must stay so**: its subject
  is a membership claim on a prose *surface*, this is a reference inside the
  queue, and §The tag algebra's split between the two is exactly what makes the
  no-red posture here correct. Named because widening one resolution set invites
  widening the other, and widening the other would red on good prose.
- `check-queue-sections`, `check-queue-entry-budget`, `check-queue-wrap` — red on
  the queue **file's** shape. **Monotone under this change and cleared by
  inspection**: no delta writes to the queue file at all.
- `queue-lib-parity.test.sh` — reds when the shell and compiled classifications
  of one corpus differ. **Not monotone** (byte comparison). Cleared by
  inspection: the retired derivation lives in `bin/queue-edges.sh`, which the
  compiled side does not implement, and `lib/queue.sh`'s classification is
  untouched — the tool reuses the existing lead-line grammar rather than adding
  one.

## Existing sections updated

- `queue-kit/SPEC.md` §bin/queue-edges.sh — the usage block gains nothing but the
  widened `--inbound` domain; the ordering paragraph gains the retired block and
  its alphabetical order with the reason; the "**Inbound only**" paragraph is
  unchanged and re-read to confirm it still reads true of a retired target
  (deltas 1 and 2).
- `queue-kit/SPEC.md` §bin/queue-edges.sh — the git dependency, its measured
  cost, and the two declared degradations (no repository, partial history), which
  are new obligations on a tool whose section currently promises it "reads the
  queue, writes **stdout only**, and mutates nothing" — still true, and now with
  a second input (delta 2).
- `queue-kit/SPEC.md` §bin/queue-edges.sh, "**Not a gate, either**" — the
  paragraph that refuses redding on an unresolved token gains the split: the
  refusal stands, and the *reporting* of the retired subset is what it always
  left available (deltas 1 and 3).
- `queue-kit/SPEC.md` §The tag algebra — the "an unresolved token is not an
  error" bullet takes its measurement and the derived-not-authored answer to the
  prose-distinguishability question, stated beside the two refusals it follows
  from (delta 3).
- `queue-kit/SPEC.md` §lib/queue.sh — no content change; re-read at merge to
  confirm the lead-line grammar the history walk reuses is stated as reusable
  rather than as an internal of the live-slug reader (delta 2).
- `lifecycle-kit/templates/stages/scope.md` — the ranking survey names the
  retired block as an input it reads before ranking (delta 4).
- `lifecycle-kit/templates/stages/close.md` — the gap-inbox drain names the
  retired block as an input and gains its first invocation of
  `bin/queue-edges.sh`, unlike scope's edit, which extends an existing one
  (delta 4).
- `queue-kit/gate-tests/queue-edges.test.sh` — the tool's behavioural coverage;
  its expectations assert `queue-edges.sh` output and move with delta 1, gaining
  a retired-target arm, a never-live-token arm proving the floor, and a
  no-repository arm proving the declared degradation (deltas 1 and 2).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`), discharged at the iteration rather
      than at this commit if a sibling queue-kit amendment is in flight.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The counts are re-derived, not cited** — the 517/395/71/324 figures and
      the 0.72s are re-run against the tree at build; the survey record's witness
      is the cheap form of that re-run.
- [ ] **The two stage templates carry the reader, or the report has none** — a
      listing landed without delta 4's edits is a derivation nobody runs, which is
      the shape §bin/queue-edges.sh's no-tracked-projection ruling already refuses
      for a different reason.
