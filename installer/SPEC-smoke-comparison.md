# SPEC amendment: smoke-comparison

**The native Windows leg's manifest arm fails 476 of 476 entries while every value it prints
agrees, and the defect is in the instrument's own control flow rather than in any operand.** The
failing comparison has two operands; the report holds one of them and re-reads the other. This
amendment closes that asymmetry, repairs a verdict-class error the same arm makes, and carries
§The consumer smoke's record through round 14.

**What this amendment asserts, and what it deliberately does not.** Ruled `lead, own-authority`
2026-09-06 through this iteration's dispatch relay, splitting the envelope by what a validate
stage can check.

- **Asserted** — all four deltas below, every one tree-verifiable and checkable at validate.
- **Not asserted** — a **green** Windows leg, and the **content** of the next round's reading.
  Asserting either would assert a fact no stage of this iteration can check. The shape has a name
  and a home already: `observation-predicate-entry-cannot-drain-in-its-own-iteration`,
  *landed-but-unobservable, not unstarted*.

**The reserved question and its resolution, recorded because the ground outlived it.** The
2026-08-30 operator ruling's red-cause limb — file and defer without looping — is what the
one-to-two push budget protects, so a repair wanting *rounds* rather than one verification push
was reserved to the lead. It fired here and resolved to **proceed**, on a refuted premise rather
than a judgment call: `.github/workflows/gates.yml:9-13` triggers `gates` on every push to
master and `:202-208` carries `continue-on-error: true` on `install-smoke-windows`, so a Windows
round rides a push the iteration already makes. **Rounds here spend wall-clock and attention,
never the push budget the ruling protects**, so the constraint being guarded does not reach this
case. Verified independently by the lead at the ruling.

**One alternative is refused rather than left open.** Normalizing the two operands to their
40-hex core before comparing would turn the leg green without repairing anything: it makes the
instrument accept a mangled producer silently — enforcement-first inverted — and it makes a green
leg unreadable as evidence, which is worse than a red one. Refused `lead, own-authority`
2026-09-06 on that ground. Not an alternative worth revisiting.

## What changes

### (1) The report holds **both** operands of the failing comparison, not one

`installer/consumer-smoke/run-smoke.sh`'s manifest arm carries the `got` its loop held into the
failure report, beside the `want` it already carries, and the report prints it as a held value
distinct from a re-read of the same call {design-bearing}.

**The asymmetry, stated in the instrument's own prose on one operand only.** The comparison is
`:318`, `[[ "$got" == "$want" ]]` — two operands. `:319` carries `("$path"$'\t'"$want")` into
`bad_hash`, so `:229-231` prints `want` **as the loop held it**, and its own `call` line says so:
*"read off the arm own jq stream of .files, which is the value the failing comparison used and
not a second read of it."* `got` gets no such treatment — `:232` is
`hash_probe got git hash-object -- "$C/$p"`, which **re-runs the call**. That is the exact trap
§The consumer smoke already names, in the `want` bullet, as the defect the round-13 form had:
*"a value that reaches the comparison mangled and re-reads clean is invisible to a report that
asks again, and that is not a hypothetical: the first form of this report re-read it, and saw
nothing on a leg where every entry disagreed."* The correction was applied to one operand. This
delta applies it to the other, and there is no third.

**Consequently the section's stated locus is falsified and delta 3 corrects it.** Round 14 read
`want` as held and it agreed with everything, so `installer/README.md:1526` — *"That leaves
exactly one locus: the value the arm's loop held in `want`"* — is now false. The remaining locus
is `got`-as-held, and **after it nothing in the comparison is unread**: both operands are then
printed as the comparison received them. That is what makes the next round the *last* diagnostic
round by construction rather than one of an open series.

**Five values per sampled path, and the two new reading rules.** `bad_hash`'s element becomes
`<path><TAB><the want the loop held><TAB><the got the loop held>`, and the report prints:

- **`want`** — unchanged; the held value off the arm's own jq stream.
- **`got`** — **new meaning: the value `:317`'s command substitution actually assigned**, carried
  out of the loop. This is the operand, not a description of one.
- **`reread`** — `git hash-object -- "$C/P"` re-run at report time from the smoke's own current
  directory. This is what the label `got` used to print, renamed so the two are not confusable;
  the rename is the whole point, because a single label covering a held value and a re-read is
  how the asymmetry stayed invisible for two rounds.
- **`own`**, **`raw`** — unchanged in meaning, and both are re-reads by construction; they answer
  about the *hashing*, never about the comparison.

Each keeps its byte rendering, its `call` line and its captured standard error. The rendering
stays non-optional for the reason already stated — and it is load-bearing rather than assumed:
bash 5.3.15 `printf '%q'` renders a trailing carriage return as `$'…\r'`, so round 14's bare
renderings genuinely retire every stray-byte hypothesis **about the four values it printed**, and
say nothing about the one it did not.

Two rows join the truth table, and they are the rows that read the *comparison* rather than the
hashing:

| observation | reading |
| --- | --- |
| `got != reread` | the value the comparison used is not the value the same call yields now; the mangling is at capture time inside the loop, and `got`'s byte rendering names the stray byte |
| `want == got`, byte-equal, both held | the comparison received two equal values and reported disagreement, which bash cannot do — so the pairing is wrong and `bad_hash` associated a `want` with another entry's `got`; read the sampled path against the loop's own echo order |

The existing four rows are unchanged in substance; where a row's subject is the hashing rather
than the comparison, its `got` term reads `reread`. The fourth row's own count updates too — *"any
of the four is not 40 lowercase hex"* becomes *"any of the five"*, since `reread` is now a fifth
printed value carrying the same byte rendering and is no less able to show a stray byte than the
other four; nothing in delta 1 exempts it from that row's catch-all.

### (2) A malformed operand is a harness precondition at exit 2, never a manifest verdict at exit 1

The arm asserts each operand's **shape** before comparing, and a value that is not exactly 40
lowercase hexadecimal characters routes to `blocked` (exit 2) instead of `fail` (exit 1)
{design-bearing}.

**This is a verdict-class error and it is repairable from the tree today, with no mechanism
named.** Whatever produced round 14's disagreement, `starter: 476 of 476 manifest entries
disagree with the tree` at exit 1 is a statement **about the consumer** — that the tree init
wrote no longer matches what init recorded. If instead an operand reached the comparison
malformed, that is a statement about the *harness*, and the honest code is 2. §The consumer
smoke already draws exactly this line for the report's own construction (*"A regeneration step
the arm could not complete is exit 2, the harness-precondition code: it is the arm's own
construction and not a finding about the consumer"*); this delta extends the same line to the
arm's operands. The channel already exists — `run-smoke.sh:19`'s `blocked()` prints to standard
error and exits 2 — so no new mechanism is minted.

**It fires after the report, not before it.** The malformed operand is exactly the case whose
diagnosis the five-value block exists to serve, so the arm still prints the full report and only
then chooses its exit code by whether any operand was malformed. Refusing at the first bad
operand would trade the report for the verdict and lose the more valuable half.

**The fourth truth-table row is not made redundant by this.** That row (*any of the four is not
40 lowercase hex*) tells a **reader** what a bad rendering means; this delta makes the **arm**
act on it. A report a human must read and a verdict a suite can read are two consumers, and
deleting either for the other is the mistake.

### (3) §The consumer smoke's record carries through round 14, and the falsified locus is corrected

`installer/README.md` §The consumer smoke's *What the native Windows leg has measured so far*
gains round 14 and corrects the sentence round 14 falsified {mechanical}.

The record currently stops at rounds 12 and 13 and reads *"both fail at `starter: 477 of 477`"*.
Round 14 is recorded only in `TASK-QUEUE.md`, while that same entry says **§The consumer smoke
owns the record, cited and not restated** — so the owner doc is stale against its own citation.
What lands, read off the run's log rather than off the queue's relay
(`gh run view 34002192468 --log`, a free read of a finished run, no push bought):

- Round 14 — run `34002192468`, head `a5b6907b` — exits 1 at `starter: 476 of 476`. The count
  moved from 477 with the payload, not with the defect.
- Every one of the 476 lines reads `manifest hash disagrees with the tree`, with **nothing
  interleaved**: `git hash-object` emitted no standard error across all 476 calls, so no
  per-path refusal is open and the calls did not fail.
- For **both** samples — the `.md` and the `.exe` artifact row — `want`, `got`, `own` and `raw`
  came back the same 40-hex, every byte rendering bare.
- `starter` is the **first** profile the loop drives, so no profile passed this arm on that host
  before it; the consumer's `git status --porcelain` printed nothing; the artifact control read
  `recorded` and `recomputed` equal.

The correction to `:1523-1533`: *what round 14 establishes* is that the recorded value, the
tree's bytes, every git context **and the held `want`** all agree — so the disagreement is in the
comparison, in the table's second row in its strongest form. **The locus sentence is rewritten**:
the remaining locus is the `got` the loop held, the one operand the report re-read rather than
carried, and the round-13 correction is named as having been applied to one operand of two.

### (4) The next round's reading has a named owner and is recorded rather than asserted

§The consumer smoke states where the reading of the next Windows round lands, and states the
trap that would otherwise make it silently never happen {design-bearing}.

**The owner is close, at its own push, and it costs no push.** `install-smoke-windows` is a job
of the `gates` workflow, which triggers on every push to master, and CLAUDE.md's push discipline
already has close watch that workflow and hold its run id; a finished run is read for free with
`gh run view <id> --log`. So the observation has an owner inside the ordinary stage set and needs
no new mechanism.

**The trap, and it is why this delta is prose in the section rather than an assumption.** The job
is `continue-on-error: true`, so **the workflow's conclusion is `success` while the job's is
`failure`** — measured, not predicted: run `34002192468` concluded `success` with
`install-smoke-windows` at `failure`. A session that watched the run to green and inferred the
leg from that verdict would read a passing workflow as a passing leg. **The reading is therefore
an explicit, job-keyed act** — read the job's log, never the workflow's conclusion — and the
section says so in those terms. A completion clause resting on the watch alone is the
dead-observer shape: an observation nobody performs, recorded as though scheduled.

**What is recorded is what was observed, by whoever observed it.** This amendment asserts no
content for that reading, which is the envelope's own boundary: the next round either shows
`got != reread` and names the mangling, or shows both operands held and byte-equal and moves the
defect to the pairing. Both are terminal for the diagnosis and neither is asserted here.

## Producers and consumers

**New state: the held `got`, carried out of the comparison loop (delta 1).**

- **Producer.** `run-smoke.sh:317`'s command substitution, `got="$(git hash-object -- "$C/$path")"`
  — the same assignment the comparison at `:318` reads, with no second call and no re-read
  interposed. It is appended to `bad_hash`'s tuple at `:319`, on the failure branch only, so it
  is produced exactly where a disagreement is produced. No enabling configuration exists or is
  minted: the report is failure-path code with no knob, deliberately — *"a report enabled by a
  configuration no CI job sets is the dead-producer shape, and the one host that needs this one
  is the host nobody is standing at."* That standing rule is what forbids gating this behind a
  flag, and it is cited rather than re-argued.
- **Consumer.** `manifest_report()`'s per-sample block, which splits the tuple at `:227` and
  prints the value with its byte rendering. Its reader is **a human reading the failing job's
  log**, at the transition delta 4 names: close, reading `install-smoke-windows`'s log off the
  run its own push produced.
- **Every new field has a named reader.** The tuple gains exactly one element, `got`, read by the
  `got` line of the five-value block and by the two new truth-table rows. `reread` is not a new
  field — it is the existing `hash_probe` call under a name that no longer collides with the held
  value. No other field is added, so none lacks a reader.

**New state: the malformed-operand flag (delta 2).**

- **Producer.** A shape test inside the same loop, over both operands, on the same failure branch.
- **Consumer.** The arm's own exit-code selection after `manifest_report` returns: `blocked` at
  exit 2 when the flag is set, the existing `fail` at exit 1 otherwise.
- **Reader of the verdict, with its red condition.** The `installer_smoke` evidence-kit validate
  suite (`scripts/evidence-config.sh`), whose parser distinguishes 0 / 1 / 2 — 2 being the
  precondition code, which is what makes this delta a real change to what the suite is told
  rather than a cosmetic one. On the Windows leg the reader is the job, whose
  `continue-on-error` posture is untouched by this amendment (delta 4).

**Existing integration prose describing the prior flow, updated rather than left to drift.**
§The consumer smoke's four-hash block, its truth table, its round record and its locus sentence
all describe the arm as it stands; deltas 1-3 rewrite each in place. The paragraph naming
`want`'s deliberate non-re-read is the one a reader will check first, so it gains the sentence
saying the same treatment now covers `got` — without it, that paragraph reads as though the
asymmetry were intentional.

**No knob is minted by any delta**, so no `<KIT>_<KNOB>` roster moves and no config seam changes.
The provenance seam is not approached: every delta is inside this repo's own acceptance harness
and its owner doc, `installer/consumer-smoke/run-smoke.sh` being declared `no-port` precisely
because it rides no payload and reaches no adopter.

## Existing sections updated

- `installer/README.md` §The consumer smoke — the four-hash block's introduction and its four
  bullets, which become five (delta 1); the truth table, which gains two rows and rereads its
  `got` term (delta 1); the arm's failure-path description, which gains the exit-2 class (delta
  2); *What the native Windows leg has measured so far* and the `:1523-1533` locus paragraph
  (delta 3); the observation-owner statement (delta 4).
- `installer/consumer-smoke/run-smoke.sh` — the manifest arm at `:313-324`, `manifest_report()`
  at `:190-235`, and `hash_probe`'s call sites; the file's own `# spec:` header sentence
  describing the manifest post-condition, which now has a second exit code (deltas 1 and 2).
<!-- update-target-exempt: the drop condition is a green observation this envelope explicitly does not assert, so touching it would assert what delta 4 records as unasserted -->
- `.github/workflows/gates.yml` — **deliberately untouched**. The `continue-on-error` line's own
  comment conditions its removal on the run the leg is first observed **green**, and green is
  exactly what this amendment does not assert, so the line and its comment stay.
- `TASK-QUEUE.md` — `platform-support-ci-matrix` promotes to New Features with this file's
  `[spec:]` ref, in the same commit as this file (all deltas).

## Definition of Done

- [ ] **Causal completeness** — the held `got` and the malformed-operand flag each have a named
      producer, a named consumer and a named reader at a named transition; the tuple's one new
      element is read by the block and by the two new truth-table rows.
- [ ] **Both operands are held** — no value the comparison used is printed by re-running its call;
      `reread`, `own` and `raw` are labelled as re-reads and the two held values are not.
- [ ] **The verdict class is correct** — a malformed operand exits 2 through `blocked` after the
      report prints, and a genuine hash disagreement still exits 1 through `fail`.
- [ ] **The record is current** — §The consumer smoke carries round 14 and the locus sentence is
      corrected; nothing about round 14 is left owned by `TASK-QUEUE.md` alone.
- [ ] **The observation has an owner and the trap is stated** — the reading is job-keyed, never
      inferred from the workflow's conclusion.
- [ ] **Nothing green is asserted** — no delta, and no line of the merged section, claims the
      Windows leg passes or predicts what the next round shows.
- [ ] **Battery and suite green** — `bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other, and the
      `installer_smoke` suite clean on this host (which does not exercise the failure-path report
      — that limit is §The consumer smoke's own, stated rather than gated).
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — grepped for the retired locus sentence and for the old single
      `got` label; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
