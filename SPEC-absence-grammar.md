# SPEC amendment: absence-grammar

No surface rules when an absence is stated or how. Sessions land on both sides of a distinction the tree already makes in practice: a release note's reconcile sections say `None` so a reader knows each was considered, an amendment's retired-spellings block has a mandatory negative form, the queue's empty sections and the ruling record's empty sections are their headings alone — and the roadmap emitter prints a whole sentence under an empty horizon, a ruling record once carried a sentence per empty section restating what its staleness probe derives, and a queue section holding only a `none` bullet passes the queue gates.

**The ruling: a methodology rule, Absence statements, with two branches, a shipped grammar brought to it, and a cadence rather than a gate.** State an absence only where a reader must tell *considered, and none* from *not yet filled*, and there as a token; elsewhere omit it, because the structure already shows it. A generated section is always in the second branch, since an emitter cannot forget to fill it, so the roadmap arm's empty horizon becomes its heading alone. The rule is a methodology rule rather than a craft rule because it bears on every surface edit, and the digest is where a session meets it before it writes the sentence.

**Enforcement is a cadence, not a gate, and the branch is why.** Which branch a section is in is a judgment about its reader, which no scanner decides. The sentence shape alone is decidable, and two arms reaching it are costed and filed to the gap inbox rather than built here: a prose-tells arm redding a section whose whole body is one sentence of absence, over a consumer-configured surface set, off by default; and a placeholder-slug denylist in the queue's name gate. Neither has a live instance to catch (the queue's empty sections and the ruling record's are headings alone today), and each is its own unit with a knob and a fixture pair. The generated surface this amendment changes is held by its emitter's contract under `check-roadmap-fresh` and the arm's own tests, which is the rule's enforced half.

## What changes

### (1) DOCTRINE.md gains methodology rule 14, Absence statements, and the craft register renumbers {mechanical}

**Not yet applied.** In `doctrine-kit/DOCTRINE.md`, after rule 13 (Policy-as-choice) and its digest, insert as the last rule of `## Methodology-maintenance rules`:

> 14. **Absence statements.** State an absence only where a reader must tell *considered, and none* from *not yet filled*, as in a hand-authored checklist or a declaration read as the empty set. There it is a token (`None`, a dash), with a reason only where the grammar asks for one. Elsewhere omit it, since the structure shows it: an empty section is its heading alone. A generated section always is, since an emitter cannot forget to fill it. A sentence saying nothing stands somewhere is the defect on both sides. *Under agent work:* an agent fills an empty section by reflex so the page looks finished, and its sentence is a second source of what the structure carries, stale once the section fills and read on a ledger as a finding. The inverse reflex drops the token a reader needed, so the rule has two branches, not one ban. *Enforced by:* a surface's own grammar where one owns it: an emitter's contract under its freshness gate ([queue-kit/SPEC.md](../queue-kit/SPEC.md) §The roadmap arm), a mandatory negative form ([canon-kit/SPEC.md](../canon-kit/SPEC.md) §check-amendment-retired-spelling). On hand prose the branch is judgment (the Enforcement-first false-positive carve-out), with a consumer audit-roster class as its cadence ([lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) §The audit roster).
>
>     *Digest:* state an absence only where a reader must know it was considered, and then as a token; elsewhere omit it: an empty section is its heading alone.

The engineering-craft rules renumber from 14–27 to 15–28. No citation of a craft rule by number exists outside the doctrine to move (§Producers and consumers). The body is sized to `check-prose-bounds`: measured at authoring over a scratch tree holding it alone, it carries no finding, so the `doctrine-kit/DOCTRINE.md` ceiling row stays where it is.

### (2) The always-loaded digest gains the rule {mechanical}

**Not yet applied.** `CLAUDE.md` §Delivery doctrine gains, after the Policy-as-choice bullet, `- **Absence statements** — ` followed by delta 1's digest text verbatim (`check-doctrine-registration` assertions B, C and F). Running `--install-doctrine` produces the same line. The always-loaded file grows by one line, so `check-surface-ratchet` reds the growing commit and prints the ceiling re-stamp, which that commit carries (context-kit/SPEC.md §The surface ratchet).

### (3) queue-kit/SPEC.md §The roadmap arm: an empty horizon is its heading alone {mechanical}

**Not yet applied.** Three passages of §The roadmap arm:

- In the paragraph beginning "**Table membership is forced rather than chosen**", the grammar list "the headings, the bullet shape, the placeholder, the trailing blank" becomes "the headings, the bullet shape, the trailing blank".
- The first bullet of the emit grammar becomes:

  > - One `### <horizon>` heading per configured horizon, in the knob array's own order — never a sort the crate imposes — each emitted even when the queue puts nothing there, since a section that vanishes when it empties reads as a page that forgot it. An empty horizon is its heading alone: a generated section cannot have been forgotten, so its empty body already says nothing is queued there (doctrine-kit's Absence statements rule). One blank line separates consecutive horizons, filled or empty.

- In the paragraph beginning "The block ends on a blank line", that opening sentence becomes "The block ends on one blank line, whether or not its last horizon is filled."

### (4) The roadmap arm emits no placeholder {design-bearing}

**Not yet applied.** In `native/src/emit/roadmap.rs`, the arm prints nothing under an empty horizon's heading and drops the constant it printed there. Consecutive horizons are separated by exactly one blank line whether either is filled, and the block ends on exactly one blank line whether its last horizon is filled — the naive removal of the placeholder line leaves two blank lines after an empty horizon, in both positions. The two `spec:` comments that name the placeholder are rewritten to delta 3's grammar.

Tests, each named for the invariant it holds:

- The unit test pinning the whole body for a filled and an empty horizon expects `### someday\n\n` as the empty horizon's whole emission and the block's end.
- A unit test holds the empty-first case: an empty horizon followed by a filled one is its heading, one blank line, then the next heading.
- The whitelist test's `### soon\n\n` prefix assertion no longer expects the placeholder after it; it asserts the next heading follows.
- `queue-kit/gate-tests/roadmap.test.sh`'s empty-horizon case keeps its heading assertion and replaces its placeholder assertion with one that the line after the empty horizon's heading and its blank is the block's end or the next heading. Its closing summary line names the case for what it now holds, and its check count moves only if the case's assertion count does.

`ROADMAP.md` carries no empty horizon, so its block is byte-unchanged and `check-roadmap-fresh` stays green.

### (5) This repo's audit roster gains the `absence-statement` class {mechanical}

**Not yet applied.** `.workflow/audit-roster.txt` gains a block, never swept:

```
class: absence-statement
scope: a sentence stating that nothing stands in a section, or a placeholder bullet (none, a dash) under a heading whose empty body already says so, on governed prose, the queue or the ruling record, per doctrine-kit/DOCTRINE.md Absence statements; un-gateable, since whether a reader must know a question was considered is judgment. Corpus: the sections the iteration's range emptied or reduced to one line, from git diff <base>..HEAD over those surfaces, each read for a body that only re-says its heading's absence. A None token in a hand-authored checklist or a declaration grammar is clean; a generated block is its emitter's and out of reach. Fix by deleting the sentence, or reducing it to the token where the section is a checklist, never by rewording it.
due: an iteration that empties a section of governed prose, the queue or the ruling record; an emitter gaining an empty case
last: never
```

`check-audit-roster` holds the block's shape, and its `scope:` line is under `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP`.

### (6) The release declarations name both behavior changes {mechanical}

**Not yet applied.** `.workflow/release-declarations.md` `## Behavior changes` gains two bullets:

> - **`doctrine-kit/DOCTRINE.md` Absence statements** — gains methodology rule 14, *Absence statements*: state an absence only where a reader must know it was considered, and then as a token; omit it elsewhere, so an empty section is its heading alone. The engineering-craft rules renumber from 14–27 to 15–28. `check-doctrine-registration` reds your agent file until it carries the new digest bullet: re-run `--install-doctrine` after re-vendoring.
> - **`--emit roadmap` empty horizon** — an empty horizon is now its heading alone, with no placeholder line under it. `check-roadmap-fresh` reds a page whose committed block still carries the old line: regenerate it with `--emit roadmap --write`.

The pending Policy-as-choice bullet's "13–26 to 14–27" stays: it states what that change did, and the note publishes both.

## Producers and consumers

- **The rule and its digest** (deltas 1 and 2). Producer: `doctrine-kit/DOCTRINE.md`. Consumers: `--install-doctrine`, which derives the bullet from the `*Digest:*` trailer; `check-doctrine-registration`, whose assertion B reds `CLAUDE.md` until the bullet exists, C until it names a rule, E until the rule carries one trailer, F until the bullet text matches it; `check-prose-bounds` over the rule body under `.workflow/prose-bound-ceiling.txt`, whose row must not move; `--emit stage-rules`, which prints each craft rule's number read at run time, so the renumber moves its output with no code change; every session through the always-loaded digest.
- **The renumbered craft register** (delta 1). No gate reads a craft rule's number; `native/src/doctrine.rs` and the `check-doctrine-registration` fixtures carry no count of the real doctrine's rules (`git grep -n -E "\b(13|14|27|28)\b" -- doctrine-kit/smoke doctrine-kit/gate-tests`, and the doctrine module read at its section constants). Numbered citations were probed with `git grep -n -E "(craft|doctrine|DOCTRINE).{0,80}\b(1[4-9]|2[0-7])\b" -- ':!TASK-QUEUE.md' ':!docs/*'`, dates and versions filtered: one hit, the pending Policy-as-choice declaration's range, left standing by delta 6.
- **The empty-horizon grammar** (deltas 3 and 4). Producer: the roadmap arm. Consumers: `check-roadmap-fresh` assertion A, which byte-compares the arm's emission in process, so the arm and the gate cannot disagree; a consumer's committed page, which reds A until regenerated wherever it holds an empty horizon (delta 6 declares it); the page's own framing prose, which here already reads an empty horizon as "nothing is curated at that band" and needs no edit. No gate reds a heading followed by a heading: the gate sources carry no empty-section or empty-heading test (`git grep -n -i -E "empty (section|heading|body)|heading with no|no body|consecutive heading" -- native/src/gates native/src/section.rs native/src/spec.rs`, no hit), and this queue's `## New Features` and the ruling record's two sections are heading-alone today under a green battery.
- **The absence sentence elsewhere in shipped emitters.** Probed with `git grep -n -E '"_?(Nothing|No |None|none)[^"]*"' -- native/src/emit native/src/*.rs` and a negative-existential sentence pattern over the committed-projection emitters (`trajectory.rs`, `install_evidence.rs`, `value_rollup.rs`, `enforcement_map.rs`, `footprint.rs`, `docs_mirror.rs`, `graph.rs`, `git_hooks.rs`, `ruling_staleness.rs`): the roadmap placeholder is the one committed-block sentence. `md_index.rs` and `pub_index.rs` print a "No … found" line to stdout for a session, never to a committed surface, so they are outside the rule's reach. A keyword probe misses a paraphrase; the audit-roster class is where a missed one is found.
- **The roster class** (delta 5). Producer: this repo's tracked roster. Consumers: the close stage's roster review, which reads `due` and `last`; `check-audit-roster`, which holds the four-key never-swept shape; the next sweeping session, which reads `scope`.
- **The declarations** (delta 6). Consumer: the release note's composition, held by `check-release-declaration-parity`.
- **Point 5.** No delta narrows a corpus. **Point 6.** No delta obliges each member of a corpus.

## Existing sections updated

Rosters from the probes above, plus `git grep -n "Nothing is queued"` (the retired spelling's survivors) and `git grep -n -i "placeholder" -- queue-kit/ native/src/emit/roadmap.rs ROADMAP.md`, run 2026-09-26.

- `doctrine-kit/DOCTRINE.md` — the new rule 14 and the craft renumber (delta 1).
- `CLAUDE.md` §Delivery doctrine (delta 2).
- `.workflow/surface-ceiling.txt` — the `CLAUDE.md` row's re-stamp (delta 2).
- `queue-kit/SPEC.md` §The roadmap arm (delta 3).
- `native/src/emit/roadmap.rs` (delta 4).
- `queue-kit/gate-tests/roadmap.test.sh` (delta 4).
- `.workflow/audit-roster.txt` (delta 5).
- `.workflow/release-declarations.md` §Behavior changes (delta 6).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/doctrine-kit/DOCTRINE.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/queue-kit/SPEC.md`.

## Retired spellings

- `Nothing is queued under this horizon` — the roadmap arm's placeholder line (delta 4).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the rule, the empty-horizon grammar and the roster class.
- [ ] **Instruction surfaces: instruction only.** The `CLAUDE.md` bullet carries the digest and nothing else.
- [ ] **Merged with no information lost.** The queue-kit passages are re-phrased, not appended to; the rule's grounds live in its own body.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md` at the root).
- [ ] **Entry moved.** `absence-statement-grammar` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** The two gate arms named above are in the gap inbox; any gap found during the work is filed there too.
