# SPEC amendment: queue-citations

Two queue entries share one shape. Prose in a queue body points at something outside the entry, the pointer's coordinate stops resolving, and nothing reads it.

- **`queue-recovery-pickaxe-wrong-oracle`.** Every surface telling a reader how to recover an evicted body names `git log -S`. `-S` lists a commit only when the literal's occurrence **count** changes, and an eviction that keeps the slug (a Done move to a bare slug line, an icebox one-liner) changes no count. `-G` matches diff content and reaches both.
- **`queue-citation-line-number-stales-within-its-own-session`.** A `path:line` cite in a queue body goes stale when the cited file changes above the line, and a session that edits the cited file stales its own cite.

**The rulings.**

- **Recovery: one owner, the right oracle, and a hygiene axis on the wrong one.** The entry left open whether a gate earns its slot against a four-line prose fix. Enforcement-first ranks removing the duplication above gating it, so the recipe gets **one owner**, §The icebox tier, and the queue's own preamble points at it rather than restating it. The duplication is only half the recurrence, though. The other half is sessions writing a per-entry recovery recipe into a body at filing, which is where the most recent wrong spelling landed (the `queue-citation-…` entry's own "Body before eviction" line). A body recipe has no owner to point at. So the wrong oracle is also held lexically. That costs no slot: it is one axis of `check-queue-hygiene`, which already reads the whole queue file line by line. The axis bars `-S` only over the queue file itself. A phrase pickaxe over the gap inbox is correct, because a drain removes the phrase and so changes its count, and it stays legal.
- **Citations: cite by an anchor that survives edits, never by line number, and a hygiene axis on the number.** The entry offered a gate reading history (the decidable half: a cite whose file changed above the line since the cite landed) or a citation rule. The rule is taken. A line number goes stale on any edit above it, so the history gate would red an ordinary edit to the cited file, and it would red it in whatever later commit touched the queue. The rule removes the class instead. Every file the queue cites has a coordinate that does not move with its neighbours: a SPEC or markdown `§Section`, a function or constant name beside its path, or a quoted literal. The lexical half is decidable and cheap, and it becomes a second hygiene axis. This iteration's unwrap amendment (`SPEC-md-unwrap.md`) renumbers every line of every tracked markdown file, which makes the case concrete. Each of today's markdown line cites goes stale at that commit whatever this amendment says. The operator's direction on this entry (2026-09-22, lead-relayed) is that heading anchors cover queue records only. So a citation of another queue entry becomes a heading link under `SPEC-queue-headings.md`, and this rule governs every other file.

**Measured at authoring (2026-09-22).**

- `git grep -n -E "git log[^\`]*-S" -- ':!*.rs'` finds the recipe at queue-kit/SPEC.md §The icebox tier, its docs mirror, the queue's `## Icebox` preamble and one body line (the `queue-citation-…` entry's "Body before eviction"). Two further hits are phrase pickaxes over `.workflow/gap-inbox.md` (the hard-wrap and entry-shape entries), which are correct. `check-queue-entry-budget`'s help text no longer carries `-S`, as scope re-verified.
- On `scratch-execution-allowlist-bar`, `git log --format=%h -S'…' -- TASK-QUEUE.md | wc -l` returns 6 and the same with `-G` returns 10. The entry recorded 3 and 5 at filing, and the gap has persisted.
- `grep -n -E "[A-Za-z0-9_./-]+\.[a-z]{1,4}:[0-9]+" TASK-QUEUE.md` finds eight line-number cites on six lines. Two are already stale: `scripts/session-context.sh` cites line 91 and its `tr '/.' '-'` fold is now at line 101, and `context-kit/templates/session-context.sh` cites 86 where the fold is at 95. Delta 3 names each cite's replacement.

## What changes

### (1) §The icebox tier owns the recovery recipe, spelled with `-G` {mechanical}

**Not yet applied.** In §The icebox tier's "narrative is recoverable" bullet, replace "recovery is `git log -p -S'<slug>' -- <queue-file>` — a pickaxe that depends on no commit-message convention" with:

> recovery is `git log -p -G'<slug>' -- <queue-file>`, which depends on no commit-message convention. It is `-G`, which matches diff lines, and never `-S`, which lists a commit only when the slug's occurrence count changes. An eviction or a Done move keeps the slug in the file, so its count is unchanged and `-S` does not list the evicting commit. `-G` also lists every earlier commit that touched a line naming the slug. The evicting commit is the newest of them that removes the body.

In `TASK-QUEUE.md`'s `## Icebox` preamble, the sentence "The removed body is recoverable from the evicting commit (`git log -p -S'<slug>' -- TASK-QUEUE.md`)." becomes "The removed body is recoverable from the evicting commit (queue-kit/SPEC.md §The icebox tier)." In the `queue-citation-…` entry, "Body before eviction: `git log -p -S'<slug>' -- TASK-QUEUE.md`." takes `-G`. The two gap-inbox phrase pickaxes stay as they are.

### (2) `check-queue-hygiene` gains two citation axes {mechanical}

**Not yet applied.** §check-queue-hygiene's invariant becomes:

> Invariant: the queue contains only tasks, tags, and section structure, and its prose points at nothing by a coordinate that goes stale. No HTML comments (provenance belongs in git history). No exact-duplicate non-blank non-`---` lines (copy-paste artifacts). No column-0 prose: every column-0 line is a heading, a bullet, `---`, or a configured `QUEUE_KIT_PROSE_LEADS` token, because the shape that carries protocol duplication is banned while semantic duplication is not mechanizable. **No line-number citation**: a `<path>:<n>` or `<path>:<n>-<m>` token, whose path ends in a dot-extension, is red, and a queue body cites another file by `§Section`, by a symbol beside its path, or by a quoted literal. **No count pickaxe over the queue file**: an inline code span holding a `git log` command with `-S` and the queue file as its path is red, since §The icebox tier's recovery is `-G`.

Add to its calibration paragraph:

> The citation axes are lexical and bounded. A line number spelled in prose ("lines 642 and 669") passes, and so does a `-S` pickaxe over another file, which is correct wherever the literal leaves that file. The queue file is matched by the configured name's basename, so a recipe written with the default path or with `<queue-file>` both read. There is no valve, which matches the other axes. A false positive is rephrased, since the axes read prose and every hit has a stable spelling available.

`native/src/gates/queue_hygiene.rs` implements both axes with one finding class each, and its help text names the stable-anchor forms and `-G`. The fixture pair gains a line for each axis in `bad/` and a `§`-cite, a symbol cite and a gap-inbox `-S` phrase pickaxe in `good/`. The descriptor's `# spec:` summary gains "no line-number cites, no -S pickaxe on the queue".

### (3) The live queue's line-number cites are rewritten to stable anchors {mechanical}

**Not yet applied.** This lands in delta 2's commit, so the axis is green when it arms. Each member of the probe above takes the named value:

| Entry line (at authoring) | Cite | Replacement |
|---|---|---|
| 412 | `native/src/gates/evidence_baseline.rs:314-327` | `native/src/gates/evidence_baseline.rs` `run()`, its loop over the configured suites |
| 534 | `canon-kit/SPEC.md:239-250` | canon-kit/SPEC.md §Merging an amendment (on task completion), step 4's corpus-versus-increment test |
| 1056 | `docs/orchestration.md:22-23` | docs/orchestration.md, its "complements your orchestration setup" sentence |
| 1683 | `native/src/gates/memory_off.rs:26-29` | `native/src/gates/memory_off.rs` `memory_dir_default()` |
| 1684 | `scripts/session-context.sh:91` | `scripts/session-context.sh`, its `tr '/.' '-'` fold |
| 1685 | `context-kit/templates/session-context.sh:86` | `context-kit/templates/session-context.sh`, the same fold |
| 1737 | `queue-kit/SPEC.md:440-442` | queue-kit/SPEC.md §The tag algebra, the `recurrence:` declaration paragraph |
| 1739 | `queue-kit/SPEC.md:449-456` | queue-kit/SPEC.md §The tag algebra, the self-naming slug paragraph |

The line numbers in the first column are the authoring coordinates this amendment itself forbids in the queue. They identify rows for the build session and are superseded by the probe it re-runs.

## Producers and consumers

- **The recovery recipe** (delta 1). Its producer is the one owner section. Its consumers are a session recovering an evicted body, which §The icebox tier obliges before any ruling on a dormant entry, and the closing stage's eviction worklist reader. The preamble reaches it through the pointer.
- **The two hygiene axes** (delta 2). They run on every commit touching the queue: the descriptor couples `knob:QUEUE_KIT_QUEUE_FILE` at `tier=precommit`, and the gate is registered here. Their consumer is the committing session, through the failure text. Each finding names the line and its axis, and each field has that reader.
- **Point 5.** No corpus narrows. The gate gains red conditions, and delta 3 brings the live queue to green on them in the same commit.
- **Point 6.** Delta 3 enumerates every member by the named probe and names each one's value. Delta 1's three live `-S` sites are named with theirs.
- **Other readers of the gate.** `git grep -n -c "check-queue-hygiene" -- ':!docs/' ':!TASK-QUEUE.md'` finds, outside queue-kit/SPEC.md and the member, `queue-kit/README.md` (a name in a registration list), `queue-kit/smoke/violation.sh` (it reddens the gate on column-0 prose, an axis kept as it is), and the registration, hook and timing or validate baselines. Each of these names the gate and not its axes, so none needs an edit.

## Existing sections updated

Rosters from `git grep -n -E "git log[^\`]*-S" -- ':!*.rs'`, `git grep -n "check-queue-hygiene\|queue_hygiene" -- ':!docs/'` and the line-cite probe above, run 2026-09-22.

- queue-kit/SPEC.md §The icebox tier, the recoverable-narrative bullet (delta 1).
- queue-kit/SPEC.md §check-queue-hygiene, the invariant and calibration (delta 2).
- `TASK-QUEUE.md`, the `## Icebox` preamble and the `queue-citation-…` body recipe (delta 1), and the eight cites (delta 3).
- `native/src/gates/queue_hygiene.rs`, `queue-kit/checks/check-queue-hygiene.gate` and `queue-kit/gate-tests/check-queue-hygiene/` (delta 2).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/queue-kit/SPEC.md`.

## Retired spellings

- None — no name is retired. Delta 1 re-spells a recipe, `-S` stays correct over other files, and delta 2's axis holds its one wrong use.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the owner recipe and both axes.
- [ ] **Instruction surfaces: instruction only.** The preamble is a pointer and the help text names forms, with no grounds.
- [ ] **Merged with no information lost.** The hygiene invariant is re-phrased, not appended to. Each rewritten cite keeps the location it named.
- [ ] **Order held.** Delta 3 lands in delta 2's commit.
- [ ] **Amendment deleted.** This file is removed on merge (`ls queue-kit/SPEC-*.md`).
- [ ] **Entries moved.** `queue-recovery-pickaxe-wrong-oracle` and `queue-citation-line-number-stales-within-its-own-session` move to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block above.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
