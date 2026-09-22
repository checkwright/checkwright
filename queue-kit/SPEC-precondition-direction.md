# SPEC amendment: precondition-direction

`check-queue-prose-precondition` reds an active entry whose prose matches the forward-precondition phrase set and whose extent carries no `[blocked-by:]` or `[precondition-ok:]` tag. Two sentence shapes match that set without stating a precondition the entry waits on. One is **negated** ("not gated on", "no longer waiting on"), which is the iceboxed `precondition-gate-negation-false-positive`. The other is **inverted**: the sentence names this entry as the upstream that another waits on ("UPSTREAM of <slug>", "two sibling amendments are blocked on" this one), which is `precondition-gate-direction-blindness`. For both, three of the four remedies the gate prints are false. A blocker tag asserts a blocker that does not exist, a past-tense rephrase falsifies a statement about work not yet done, and a move to Deferred undoes a promotion. Only the valve is true.

**The ruling: the gate does not read direction or negation, and the valve with a stated cause is the contract for both shapes.** The entry offered two horns. The first, an author-subject test, was refused. Who is blocked in an English sentence is a parse of its grammatical subject and object, and the gate's whole design is a narrow phrase set over flattened prose. Any direction read the gate could run would be a second phrase set of subject cues, which is the brittle-vocabulary shape §The tag algebra refuses for relational verbs, and it would still miss the next spelling. The second horn is taken, and it also carries the negated sibling's third candidate deliverable word for word, as the direction entry's own drain noted ("whichever is taken first should take both"). So this amendment names both shapes, and the closing stage's moot sweep judges the iceboxed sibling against it (delta 2).

**What the session is told changes, and that is the fix.** The failure the entry measured is the help text teaching three false repairs. After this amendment the finding prints the phrase that fired, so the blocked session can see which sentence tripped the gate and whether it is one of the two shapes. The help text then branches on the shape the session reads, and does not list all four remedies as equal alternatives.

**Measured at authoring (2026-09-22).** `native/src/gates/queue_prose_precondition.rs` prints four remedy lines as alternatives, and its finding prints the entry's lead line only, not the text that matched. The fixture pair's `bad/expect.txt` holds the substring `forward precondition in prose`, which the new output keeps. `git grep -n "precondition-ok" -- TASK-QUEUE.md` shows no active entry carrying the valve today, so no live entry's verdict moves.

## What changes

### (1) The finding names the phrase that fired, and the help text branches on the sentence's shape {mechanical}

**Not yet applied.** `native/src/gates/queue_prose_precondition.rs`: each finding row gains the matched span, taken from the trigger's `find` over the rewritten body and widened to its enclosing clause (up to the neighbouring `.`, `;` or line end, capped at 80 code points each side). The row reads `<file>:<line>: <lead line>` and then an indented `fired on: "<span>"`. The span comes from the lowercased, rewritten body, so it is quoted as matched text and never presented as the author's spelling. The help block is replaced by:

> help: read the fired-on clause, then pick by its shape.
>   It states a precondition THIS entry waits on: tag the real blocker '[blocked-by: <slug>]', move the entry to Deferred, or rephrase past-tense if the precondition is already met.
>   It is negated, or it names this entry as what something else waits on: the gate cannot read either shape, and '[precondition-ok: <cause>]' anywhere in the entry is the answer (queue-kit/SPEC.md §check-queue-prose-precondition).

A unit test pins the span extraction (a match mid-sentence yields its clause, and a match at body end yields the tail). `bad/expect.txt` gains `fired on:`. The descriptor's `# spec:` line is unchanged.

### (2) §check-queue-prose-precondition states the two shapes the gate cannot read, and the valve as their contract {design-bearing}

**Not yet applied.** Replace the section's resolution sentence and the last sentence of its calibration paragraph ("FP-bearing by construction … the bounded scope.") with:

> Resolution depends on what the fired clause says, which the finding prints. A precondition the entry waits on takes the real blocker as a tag, a move to the deferred section, or a past-tense rephrase if it is met. The `[precondition-ok: <reason>]` opt-out (a queue tag, not an HTML comment, so it survives the hygiene gate) is for everything else.
>
> **Two shapes match the trigger set without stating a precondition, and the valve is their contract rather than a workaround.** A **negated** clause ("not gated on", "no longer waiting on") and an **inverted** one, naming this entry as what another waits on ("upstream of", "blocked on this"), both fire. The gate reads neither negation nor direction, deliberately. Telling who is blocked means parsing a sentence's subject, and any read the gate could run is a second phrase set of cues that misses the next spelling, the brittle-vocabulary refusal §The tag algebra makes for relational verbs. For those shapes the valve with a stated cause is the whole answer, and its reason names which shape it is. Blocking grade still stands. A silent pick was attested in production use, and a false positive costs one red and one valve that the finding's own help routes to.

Close's moot sweep reads this delta against the iceboxed `precondition-gate-negation-false-positive`, whose third candidate deliverable it lands. Whether the sweep moots that entry is the sweep's judgment. This amendment does not move it.

## Producers and consumers

- **The fired-on span** (delta 1). Its producer is the gate's existing trigger match, on every run that finds a violation. Its consumer is the blocked session, which reads it to choose a branch of the help text, at the moment it is blocked. It has no other field and no other reader.
- **The branched help text** (delta 1). Its producer is the gate's failure path. Its consumer is the same session. The branches cite §check-queue-prose-precondition, which delta 2 makes the owner of the two shapes.
- **The SPEC contract** (delta 2). It is read by a session deciding whether a valve is honest, and by close's moot sweep for the iceboxed sibling. It mints no name. The valve tag, its lead-line exemption (§check-tag-lead-line keeps it outside the governed set) and its opaque reason are all unchanged.
- No corpus narrows and no member obligation is added, so points 5 and 6 are not reached. The verdict is unchanged: the same entries red, and only the printed text differs.

## Existing sections updated

Roster from `git grep -n "precondition-ok\|queue-prose-precondition\|queue_prose_precondition" -- ':!docs/' ':!TASK-QUEUE.md'`, run 2026-09-22.

- queue-kit/SPEC.md §check-queue-prose-precondition, its resolution sentence and calibration paragraph (delta 2).
- `native/src/gates/queue_prose_precondition.rs`, the finding row, the help block and a unit test (delta 1).
- `queue-kit/gate-tests/check-queue-prose-precondition/bad/expect.txt` (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/queue-kit/SPEC.md`.

## Retired spellings

- None — no delta retires a name; the gate, the valve and the knob keep theirs.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the span field and the branched help.
- [ ] **Instruction surfaces: instruction only.** The help text carries the branch and a pointer, with no grounds.
- [ ] **Merged with no information lost.** The calibration paragraph is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls queue-kit/SPEC-*.md`).
- [ ] **Entry moved.** `precondition-gate-direction-blindness` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
