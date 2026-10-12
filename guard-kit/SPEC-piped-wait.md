# SPEC amendment: piped-wait

Rule `bounded_wait`'s arm (A) grants a foreground wait loop piped into a reader, and neither a clause nor a row says so: clause (d) speaks of statements after the loop, and a pipe is none. Run through the hook member on this tree, in the decision-table sandbox:

- `until [ -f marker ]; do sleep 5; done | cat`, its unspaced `done|cat`, `done |& cat`, `done | grep -q x` and `done | head -3; echo waited` are granted;
- `done | tee out.txt`, `done|rm -rf x`, `done | cat > out.txt`, `done | xargs rm` and `done | sort -o out.txt` fall through;
- backgrounded, the file-test wait piped into `cat` takes rule `wait_no_producer`'s block in both spellings, and a `grep`-condition wait piped into `cat` is granted where the same wait piped into `tee out.txt` takes rule `background_no_record`'s block.

So the reader behind the pipe is already held to the test a statement behind `;` takes, by the compound split, and the grant is the tail clause's own reach under another separator. This amendment declares it.

## What changes

### (1) Clause (d) names the piped reader

Rule `bounded_wait`'s clause (d) holds every segment after the loop, a statement or a reader the loop is piped into, and the rows pin both spellings {mechanical} {user-facing: the entry's Deliverable — the piped tail declared in the rule's tail clause and pinned by a row for each spelling, or the grant withdrawn, which one being this stage's; declared, so no verdict moves}.

- **Declared, not withdrawn.** The reader passes the test a tail statement passes, read-only with inert redirect targets and under the declared-forms reader, and it reads the loop's output, which is `sleep`'s. `done | <reader>` therefore grants nothing `done; <reader>` does not. Withdrawing it would take a second split of the tail to tell `|` from `;`, and would remove the spaced spelling's grant, which the published release already gives.
- **No verdict moves**, so the build's oracle is the rows below staying as probed.
- **The clause's test is the one rule `background_no_record`'s exemption (2) and rule `wait_no_producer` reach by citation**, so the piped reader is a statement outside the loop for them too, and their items take no edit.
- **`guard-tests/cases.tsv`**, in rule `bounded_wait`'s section:
  - `until [ -f marker ]; do sleep 5; done | cat`, `until [ -f marker ]; do sleep 5; done|cat` and `until [ -f marker ]; do sleep 5; done |& cat` → allow;
  - `until [ -f marker ]; do sleep 5; done | tee out.txt`, `until [ -f marker ]; do sleep 5; done|rm -rf x` and `until [ -f marker ]; do sleep 5; done | cat > out.txt` → fallthrough.
- **`guard-tests/background-cases.tsv`**, each with the flag set:
  - in rule `wait_no_producer`'s section, `until [ -f .tmp/marker ]; do sleep 30; done | cat` and its unspaced spelling → block;
  - in rule `background_no_record`'s section, `until grep -q DONE .tmp/worker.log; do sleep 5; done | cat` → allow and the same wait piped into `tee out.txt` → block.
- **Where `guard-spelling-variant-oracle` has landed**, its `separator-tight` transform derives the unspaced spelling from the spaced row and skips it as equal to the authored one. The rows are authored in either case.
- **`.workflow/release-declarations.md`**: the shared-loop-walk bullet's sentence on the unblanked keywords gains the wait piped into a reader with no blank around the pipe, granted since that change as the spaced spelling was before it.

Replacement for the `bounded_wait` item's clause (d). **Not yet applied.**

> - **(d) Every segment after the loop passes rule `ro_pipeline`'s read-only-pipeline test**, with the banner tolerance: a statement behind `;`, `&&`, `||` or a newline, and a reader the loop is piped into, behind `|` or `|&`. Every measured call is a compound, the loop and then what the session does once the wait ends (*"report only results you hold"*), so a grant admitting only a lone loop would cover none of them. A piped reader is held as a statement is and reads only what `sleep` prints, so `done | cat` and `done|cat` are granted where `done | tee <file>` is not. Unlike rule `ro_pipeline`, the tail need carry no read, since the loop is the call's work. The declared-forms reader runs once over the whole call, so condition and tail are held to it alike.

Replacement for §Testing's third-table bullet *a wait loop in the background → the grant rule `bounded_wait`'s arm (A) earns it, alone or followed by a banner or a read*. **Not yet applied.**

> - a wait loop in the background → the grant rule `bounded_wait`'s arm (A) earns it, alone, followed by a banner or a read, or piped into a reader;

Replacement for §Testing's `wait_no_producer` bullet *a backgrounded file-test wait → block, in either polarity and over tests joined by `&&`*. **Not yet applied.**

> - a backgrounded file-test wait → block, in either polarity, over tests joined by `&&`, and piped into a reader;

## Producers and consumers

- **The declared reach.** Producer: rule `bounded_wait`'s arm (A), on every `Bash` payload whose first statement is a wait loop. Consumer: the session, through a grant it already receives.
- **Readers of clause (d)**, by `grep -n -e 'clause (d)' guard-kit/SPEC.md`: rule `background_no_record`'s exemption (2), which calls the test for every statement outside a loop. The other hits are rule `wait_no_producer`'s own clause (d) and §scan-prompts' citation of rule `append_scratch`'s, each a different clause of its own item.
- **The entry's premise, run.** The gate binary built at the commit before `loop-keyword-as-argument` falls through on the unspaced spelling and grants the spaced one, so the unspaced grant is no older than that unit and the spaced one predates it.
- **Knobs.** None is minted and none gains a reader.

## Existing sections updated

- `guard-kit/SPEC.md` §The rule roster — the `bounded_wait` item's clause (d) (delta 1).
- `guard-kit/SPEC.md` §Testing — the two third-table bullets (delta 1).
- `guard-kit/guard-tests/cases.tsv`, `guard-kit/guard-tests/background-cases.tsv` — the rows (delta 1).
- `.workflow/release-declarations.md` — the shared-loop-walk bullet (delta 1).
- `docs/guard-kit/SPEC.md` — generated, stale once the delta lands (all deltas).

## Retired spellings

- None — the amendment declares a reach the code has and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
