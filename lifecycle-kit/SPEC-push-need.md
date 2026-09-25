# SPEC amendment: push-need

Nothing counts how many pushes a unit set needs before the set is ruled. lifecycle-kit/SPEC.md §The state machine places an `[observed-by:]` iteration's first push and names no count; `templates/stages/scope.md`'s unit-set escalation carries a `Composition:` line and `Supersession:` lines and no push figure; the budget lives only in close's `push-budget` slot, whose one named reader is the pushing session. So an overrun surfaces at close, as an operator ask the ruling on the set could have answered. At hosted-install-path two units reached done only through a Windows run during build, build spent the ordinary budget, and close's own push needed a grant.

**The ruling: scope states the set's push need against the budget, and ruling the set rules its need.** The count is the closing push plus one mid-iteration push per unit whose completion reads a remote run. A ruled need is landed on each push-needing entry, which is what the landing session reads; the closing push needs no ask of its own, because every count includes it.

**Refused: the binding names a measurement-push class** (the entry's other limb). A class exempt from the budget spends pushes nobody ruled on, so the wall-clock the budget bounds grows unpriced. Counting folds the overrun into the ask the set's ruling already makes.

**It moots the widening residue of `instrument-leg-expiry-keyed-to-a-green-run-not-its-assertion-set`.** A unit adding assertions to a binding leg is a remote-first-run unit, so its rehearsal push is counted and ruled at scope rather than spent unpriced at close (that entry's third limb, now accounted).

**Measured at authoring (2026-09-25):**

- **No push figure in the escalation.** `grep -n "Composition:\|Supersession:\|Push need" lifecycle-kit/templates/stages/scope.md lifecycle-kit/templates/lead.md` finds the two lines at `scope.md:23,33` and the lead's presence check at `lead.md:86`, and no push line.
- **The budget's readers.** `grep -rln "push-budget" --include=*.md .` outside `.tmp/` and the queue: close's slot (`templates/stages/close.md`), this repo's fill (`.claude/commands/close.md`), CLAUDE.md's pointer, lifecycle-kit/SPEC.md and its mirror, and a past release declaration. No scope surface names it.
- **Every replaced passage exists once.** Each quoted "becomes" source below, fed as a fixed-string pattern to `grep -rhoF` over `lifecycle-kit/templates` and `lifecycle-kit/SPEC.md`, matched exactly once.
- **The attested overrun.** `git show 680d6db4` carries the filing: build pushes `ce35711c`, `00dba701`, hotfix `a9133bd8`, then the granted close push `d1fdb228`.

## What changes

### (1) Scope states the set's push need {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/scope.md`, the `[observed-by:]` paragraph's sentence "If a **mid-iteration push** produces that run, nothing further is owed here: the push-placement rule governs (lifecycle-kit/SPEC.md §The state machine) and the observation arrives in time to be drained." becomes:

> If a **mid-iteration push** produces that run, the push-placement rule governs (lifecycle-kit/SPEC.md §The state machine), the observation arrives in time to be drained, and the push counts toward the set's push need (below).

After the supersession paragraph ("**Then test each shortlisted candidate for supersession, …**"), add:

> **Count the set's pushes before it is ruled.** The unit-set escalation's Recommendation also carries one line, `Push need: <n> against <b> — <unit>: <why>; …`. `<b>` is the budget the consumer's closing-stage binding fills its `push-budget` slot with. `<n>` is the closing push plus one mid-iteration push per unit whose completion reads a remote run: each `[observed-by:]` entry, and each unit whose change a remote run executes before any local run can — code for a host no local run reaches, or new assertions on a binding leg. Name the units one batch's push could serve. The line is unconditional: a set needing only the closing push says so, and a binding that pushes nothing makes it `Push need: none — no remote`. Where `<n>` exceeds `<b>`, Options also offer the cut that fits. Ruling the set rules its need: on each push-needing entry, land `**Push need (<date>, <grant>):** <the push it spends and why>`, where `<grant>` is `inside the budget` or the party that ruled the overrun.

### (2) The lead's presence check covers the line {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md` §Opening an iteration, "A unit-set escalation whose Recommendation lacks its `Composition:` line or its `Supersession:` lines (lifecycle-kit/templates/stages/scope.md, the composition test and the supersession pass)" becomes:

> A unit-set escalation whose Recommendation lacks its `Composition:` line, its `Supersession:` lines or its `Push need:` line (lifecycle-kit/templates/stages/scope.md, the composition test, the supersession pass and the push count)

### (3) The landing session pushes under the entry's push line {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/build.md`, "An entry carrying `[observed-by:]` is complete only when its run is read: after its landing commit, run close's push precondition, push, wait in-turn for the run, and read it before the Done move (lifecycle-kit/SPEC.md §The state machine)." becomes:

> An entry carrying `[observed-by:]`, or whose oracle is a remote run, is complete only when its run is read: after its landing commit, run close's push precondition, push under the entry's `**Push need**` line, wait in-turn for the run, and read it before the Done move (lifecycle-kit/SPEC.md §The state machine). A push no such line names changes the set's push need: escalate it before spending it.

### (4) The closing push needs no second ask {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/close.md` step 13, after "**No gate substitutes for this.**", add:

> The closing push is counted in every push need the scope stage states, so it is spent without an ask of its own; the budget's hotfix allowance still governs a red push.

### (5) The contract text {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §The state machine, the paragraph opening "**The seam is held deliberately.**": "The count's reader is a session, so it is close's `push-budget` slot and not a knob:" becomes:

> The budget's readers are sessions — the scope stage stating a unit set's push need against it (§templates/stages/), and each pushing session — so it is close's `push-budget` slot and not a knob:

In the same paragraph, "it pushes after its landing commit, waits in-turn for the run, and reads it before the Done move." becomes:

> it pushes after its landing commit, under the push line scope landed on the entry, waits in-turn for the run, and reads it before the Done move.

In §templates/stages/, the close paragraph's last sentence, "The same step carries the `push-budget` slot, on the ground the `drain-inputs` paragraph states: the count's reader is the pushing session, so the value is a slot and not a knob." becomes:

> The same step carries the `push-budget` slot, on the ground the `drain-inputs` paragraph states: its readers are sessions — the pushing session, and the scope stage stating a set's push need — so the value is a slot and not a knob.

After the scope supersession paragraph ("The `scope` template tests its shortlist for **supersession** …"), add:

> The `scope` template counts a unit set's **push need** because a push is an iteration's scarcest spend and an overrun was visible only at close, as an operator ask the ruling on the set could have answered. The count is the closing push plus one per unit whose completion reads a remote run: an `[observed-by:]` entry, whose push the placement rule already places (§The state machine), and a unit whose change a remote run executes first, whose red otherwise spends a watched push on code no local run reached. Ruling the set rules the need, so an overrun costs no second interrupt. The ruling lands on each push-needing entry because the landing session reads its entry; the closing push takes none because every count includes it. A budget-exempt measurement-push class in the consumer's binding is refused: it spends pushes nobody ruled on. **The honest limit:** the line travels on the message channel, like the `Composition:` line, and no arm observes a push, so a count that misses a remote-only unit surfaces at that unit's build as an escalation, never as a gated red.

In §templates/lead.md, "once the set carries scope's composition verdict — a presence check rather than a grading" becomes:

> once the set carries scope's composition verdict and push need — a presence check rather than a grading

## Producers and consumers

- **The `Push need:` line.**
  - Producer: the scope session, on every unit-set escalation. Enabled wherever the roster runs scope; no config.
  - Consumer: the party ruling the set, over the message channel; the lead's presence check before routing (delta 2). No gate reads it.
  - Fields: `<n>` and `<b>` are read by that party to rule the overrun; the per-unit `<why>` names which entries take a push line.
  - Input: the `push-budget` fill of the consumer's close binding, an existing slot every binding already fills (`check-skill-binding`).
- **The entry's `**Push need (<date>, <grant>):**` line.**
  - Producer: scope, at promotion, on each push-needing entry — or, where a feature is promoted by the authoring stage, on the deferred entry before that stage pairs it.
  - Consumer: the landing build session (delta 3), which pushes under it; `<grant>` tells it whether the push was an overrun and who granted it.
  - Roster readers: no gate scans entry prose for bold lead-ins. `check-queue-entry-budget` measures the entry and prices the line.
- **The closing push's standing** (delta 4). Its reader is the close session at step 13.
- **Point 5.** No corpus narrows.
- **Point 6.** Not obliged: the line is a per-set statement, not an obligation over an enumerable corpus.

## Existing sections updated

Roster from `grep -rn "push-budget\|nothing further is owed here\|composition verdict\|is complete only when its run is read" --include=*.md lifecycle-kit/` plus the lead's presence-check line, run 2026-09-25.

- `lifecycle-kit/templates/stages/scope.md` (delta 1).
- `lifecycle-kit/templates/lead.md` (delta 2).
- `lifecycle-kit/templates/stages/build.md` (delta 3).
- `lifecycle-kit/templates/stages/close.md` (delta 4).
- `lifecycle-kit/SPEC.md` §The state machine, §templates/stages/ and §templates/lead.md (delta 5).
- `.workflow/surface-ceiling.txt` and `docs/footprint.md`, regenerated for the templates' growth by their gates' printed commands (deltas 1, 2, 3 and 4).
- `.workflow/release-declarations.md`, one Behavior changes bullet naming `lifecycle-kit/templates/stages/` and `lifecycle-kit/templates/lead.md` (deltas 1, 2, 3 and 4): scope's unit-set escalation states its push need against the close binding's budget, the ruling lands a push line on each push-needing entry, build pushes under it, and close's push needs no ask of its own. A copied template is re-taken.
- The on-site mirror of `lifecycle-kit/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (delta 5).

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the line and the entry's push line.
- [ ] **Instruction surfaces: instruction only.** The template edits carry the act; the grounds sit in delta 5's SPEC text.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines; the one added paragraph has no passage to rewrite.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entries moved.** `push-need-uncounted-at-scope` moves to Done in the merge commit, and `instrument-leg-expiry-keyed-to-a-green-run-not-its-assertion-set` with it as mooted by this landing, both at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
