# SPEC amendment: foreign-review

**A second vendor's reading of the iteration, as an audit-roster class.** The deterministic gates are today the only critique an iteration gets. A consumer holding another vendor's coding agent can have it read the iteration's work at close, through delegation-kit's foreign-vendor run in audit mode (delegation-kit/SPEC.md §The foreign-vendor run). A strategic consultation is the entry's other candidate use. It needs no mechanism: the operator runs the consultation, so each run's grant is given in that session.

**The cadence is the audit roster, and no new close step.** §The audit roster already rules this: a differential sweep of an un-gateable class is a roster block, never a new close step, because the roster is already the event-keyed cadence close reviews. A second-vendor review fits it. Whether a review's findings hold is a reading, its cadence is an event ("every close"), and the consumer chooses whether to run it at all. So the kit ships no class, as for every other roster class, and the one generic piece missing is a disposition. A new close-template slot was refused: it would red every adopter's bound close shim under `check-skill-binding` for a review most consumers never run.

**The missing disposition is the skip with notice.** Close step 8 performs a due audit or defers it as a costed filing. A review run on another vendor's window can be refused by that window: its budget is small beside the primary harness's, and it is not the iteration's budget. A costed filing per refused close would file the same unchanged class again at every close. A block would make an outside budget gate the iteration. The disposition the entry names, *skipped with notice*, leaves the class due: its `last:` does not move, so the next review finds it due again on its own.

**The spend's authority is the consumer's, stated on the block.** Each live foreign run spends a vendor's budget, and a recurring one spends it at every close without a fresh ask. §The steering vocabulary's grant is iteration-scoped, so it cannot carry a recurring spend. The class's `scope:` carries the authority instead, as the operator's direction, where the review that spends it reads it. A consumer whose operator gives no standing direction asks at each close.

**The review adapter is chosen by name, not by tier class.** The transport slice rules that foreign adapters bind to no tier class, since every reader of delegation-kit's tier binding reads the master harness's model ids (delegation-kit/SPEC-foreign-session.md). So the block names the overlay's review adapter, and the entry's open question, whether foreign adapters map onto the tier classes, is answered there.

## What changes

### (1) §The audit roster gains the skip with notice

§The audit roster gains a paragraph after the one opening "A differential sweep of an un-gateable class is such a roster block" {mechanical}. **Not yet applied.**

> **A class swept on an executor outside the session can be skipped with notice.** Such a class runs its sweep on another vendor's agent (delegation-kit/SPEC.md §The foreign-vendor run), whose window is not the iteration's budget. Where that executor reports itself unavailable, the review skips the class: the run fails without a shape refusal, or the spend its `scope:` requires was not authorized at this close. The block's lines stay as they are, so `last:` does not move and the class is due again at the next review. The commit message names the skip and the executor's verdict line. A skip is neither a costed filing, since nothing about the class changed, nor a block, since an outside budget does not gate the iteration. A class whose sweep spends such a budget states the spend's authority in its `scope:`, which is the consumer's operator's to give.

### (2) Close step 8 names the third disposition

lifecycle-kit/templates/stages/close.md step 8: "then perform each due audit or defer it as a costed filing." is replaced {mechanical}. **Not yet applied.**

> then perform each due audit, skip with notice one whose sweep runs on an executor outside this session that is unavailable or unauthorized (lifecycle-kit/SPEC.md §The audit roster), or defer it as a costed filing.

### (3) This repo's roster gains the second-vendor review

`.workflow/audit-roster.txt` gains a block after `close-surface-actually-read` {mechanical}. **Not yet applied.** Its spend line is the operator's standing direction, given 2026-10-03 in the lead session and relayed: a review at every close on the existing review adapter, with no per-close ask, and an exhausted foreign window skipping with notice.

```
class: second-vendor-review
scope: a second vendor's reading of the iteration's work beside the deterministic gates; un-gateable, since whether a reviewer's finding holds is a reading. Run in the close session, after the drain, as an audit-mode `--foreign-run` on the local overlay's review adapter, with a prompt naming the iteration's range and asking for defects, unstated assumptions and drift between an amendment and what landed, each with a path and line. Triage every candidate as a claim to check before building on it: one that holds is filed with `--emit file-gap`, and one that falls is named in the commit message. A FAILED run skips with notice. Standing spend: operator direction, 2026-10-03.
due: every close
last: never
```

A never-swept class carries `last: never` and stops there, so `check-audit-roster` grades the block as written. The scope names no vendor and no adapter literal, since the overlay holds both.

## Producers and consumers

- **The skip disposition (deltas 1 and 2).** Producer: the closing session at step 8, on an executor's `FAILED` or an unauthorized spend. Consumers: the next close's review, which finds the class still due from its unmoved `last:`, and a reader of the close commit, which names the skip. No new field: the disposition writes nothing on the roster.
- **The class block (delta 3).** Producer: this repo's tracked roster. Consumers: close step 8, which judges `due:` against `last:`; `check-audit-roster`, which grades the block's grammar; and the closing session, which runs the `scope:`.
- **The review's executor.** `--foreign-run` in audit mode, configured by the private overlay's review adapter (`scripts/delegation-config.local.knobs`, gitignored). This repo's overlay carries that adapter today, so the producer is reachable.
- **The review's findings.** Producer: the foreign report, triaged by the closing session. Consumer: the gap inbox, through `--emit file-gap`, carried to the next scope's intake, since step 8 runs after the drain (close step 2's own rule for a later finding).
- **The spend authority.** Producer: the operator's direction, landed in the block's `scope:`. Consumer: the closing session, before the run.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §The audit roster (delta 1).
- `lifecycle-kit/templates/stages/close.md`, step 8 (delta 2).
- `.workflow/audit-roster.txt` (delta 3).
- `docs/lifecycle-kit/SPEC.md`, the generated mirror (delta 1).

The roster came from `git grep -n 'costed filing' -- '*.md' ':!docs/'` for the review's disposition sites, and `grep -n '^class:' .workflow/audit-roster.txt` for the block order.

## Retired spellings

- None — the amendment adds a disposition and a roster block and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the skip disposition and the class block.
- [ ] **Instruction surfaces: instruction only** — close step 8 carries the disposition; its grounds sit in §The audit roster.
- [ ] **Merged with no information lost** — §The audit roster reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **No live run at landing** — the block lands with `last: never`, and its first sweep is the next close's, under the spend line the block carries.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **Done move** — `foreign-vendor-critique` moves to Done in the landing commit, a stage before the drain stage, with `--queue done`.
